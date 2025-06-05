use std::str::FromStr;

use cfg_if::cfg_if;
use leptos::prelude::*;
use log::debug;
use regex::Regex;
use serde_json::json;
use server_fn::codec::{MultipartData, MultipartFormData};
use server_fn::ServerFnError;
use uuid::Uuid;

use crate::app::components::uploadthings::{FileType, UploadthingFile};
use crate::app::stores::{MessageStoreSync, MessageSync};
use crate::entities::member::Member;
use crate::entities::message::ChannelMessage;
use crate::entities::role::Role;
use crate::sync::SyncRequest;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use reqwest::Url;
        use crate::uploadthing::{FileData, UploadThing};
        use crate::open_graph::fetch_op_data;
        use multer::bytes::Bytes as MulterBytes;
        use futures::TryStreamExt;
        use super::{auth_user, user_can_edit};
        use super::auth;
        use super::sync;
        use super::pool;
    }
}

#[server(GetMessages)]
pub async fn get_messages(
    channel_id: Uuid,
    member_id: Uuid,
) -> Result<Vec<ChannelMessage>, ServerFnError> {
    let pool = pool()?;
    auth()?;

    Ok(ChannelMessage::get_channel_messages(channel_id, member_id, &pool).await?)
}

#[server(GetPinnedMessages)]
pub async fn get_pinned_messages(channel_id: Uuid) -> Result<Vec<ChannelMessage>, ServerFnError> {
    let pool = pool()?;
    auth()?;

    Ok(ChannelMessage::get_pinned(channel_id, &pool).await?)
}

#[server(GetThreadMessages)]
pub async fn get_thread_messages(
    thread_id: Uuid,
    member_id: Uuid,
) -> Result<Vec<ChannelMessage>, ServerFnError> {
    let pool = pool()?;
    auth()?;

    Ok(ChannelMessage::get_thread_messages(thread_id, member_id, &pool).await?)
}

#[server(UpdatePinned)]
pub async fn update_pinned(
    message_id: Uuid,
    channel_id: Uuid,
    server_id: Uuid,
    pinned: bool,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    if user_can_edit(server_id, user.id, &pool).await? {
        ChannelMessage::pin(message_id, pinned, &pool).await?;
        // let _ = sync()?
        //     .broadcast(SyncRequest::Mutation(MutationRequest {
        //         key: format!("channel:{channel_id}"),
        //         module: "ChannelStore".into(),
        //         data: if pinned {
        //             json!(MessageSync::Pin { id: message_id })
        //         } else {
        //             json!(MessageSync::Unpin { id: message_id })
        //         },
        //     }))
        //     .await;
    }
    Ok(())
}

#[server(name = SendMessageAttachments, prefix = "/api", input = MultipartFormData)]
pub async fn send_message_attachments(data: MultipartData) -> Result<(), ServerFnError> {
    auth()?;
    let mut data = data.into_inner().unwrap();
    let mut message_id: Option<Uuid> = None;
    let mut channel_id: Option<Uuid> = None;
    let mut files: Vec<UploadthingFile> = vec![];
    while let Ok(Some(mut field)) = data.next_field().await {
        match field.name().unwrap_or_default() {
            "message_id" => {
                if let Ok(Some(chunk)) = field.chunk().await {
                    if let Ok(id) = String::from_utf8(chunk.to_vec()) {
                        message_id = Uuid::from_str(&id).ok();
                    }
                }
            }
            "channel_id" => {
                if let Ok(Some(chunk)) = field.chunk().await {
                    if let Ok(id) = String::from_utf8(chunk.to_vec()) {
                        channel_id = Uuid::from_str(&id).ok();
                    }
                }
            }
            _ => {
                let content_type = field.content_type().expect("mime type").as_ref();
                let file_type = if let Ok(file_type) = FileType::from_str(content_type) {
                    file_type
                } else {
                    continue;
                };

                let file_name = field.file_name().expect("file name").to_string();
                if file_type != FileType::Unknown {
                    let chunks = field
                        .try_collect::<Vec<MulterBytes>>()
                        .await
                        .or(Err(ServerFnError::new("Something go wrong in our servers")))?
                        .concat();
                    files.push(UploadthingFile {
                        data: FileData {
                            name: file_name,
                            file_type: file_type.to_string(),
                            size: chunks.len(),
                        },
                        chunks,
                    });
                }
            }
        }
    }
    let message_id =
        message_id.ok_or_else(|| ServerFnError::new("Something go wrong in our servers"))?;
    let channel_id =
        channel_id.ok_or_else(|| ServerFnError::new("Something go wrong in our servers"))?;
    let pool = pool()?;

    let uploadthing = use_context::<UploadThing>().expect("acces to upload thing");
    let mut attachments = vec![];

    for file in files {
        if file.data.size != 0 {
            if let Ok(res) = uploadthing.upload_file(file.chunks, file.data, true).await {
                attachments.push(
                    ChannelMessage::add_attachment(message_id, &res.name, &res.url, &pool).await?,
                );
            }
        }
    }
    // let _ = sync()?
    //     .broadcast(SyncRequest::Mutation(MutationRequest {
    //         key: format!("channel:{channel_id}"),
    //         module: "ChannelStore".into(),
    //         data: json!(MessageSync::Attachments {
    //             id: message_id,
    //             attachments,
    //         }),
    //     }))
    //     .await;

    Ok(())
}

#[cfg(feature = "ssr")]
#[derive(Debug, PartialEq)]
enum MessageElement {
    Member(Uuid),
    Role(Uuid),
    Everyone,
    Url(Url),
}

#[cfg(feature = "ssr")]
fn extract_message_elements(message: &str) -> Vec<MessageElement> {
    let mention_regex =
        Regex::new(r"<@(?:(?P<type>role):)?(?P<id>[0-9a-f]{32})>|<@everyone>").unwrap();

    let mut data = vec![];
    let mut current_index = 0;

    for capture in mention_regex.captures_iter(message) {
        if let Some(match_range) = capture.get(0) {
            if match_range.start() > current_index {
                let text = &message[current_index..match_range.start()];
                process_urls(text, &mut data);
            }

            if capture.get(0).is_some_and(|m| m.as_str() == "<@everyone>") {
                data.push(MessageElement::Everyone);
            } else if let Some(id_match) = capture.name("id") {
                if let Ok(id) = Uuid::from_str(id_match.as_str()) {
                    if capture.name("type").is_some() {
                        data.push(MessageElement::Role(id));
                    } else {
                        data.push(MessageElement::Member(id));
                    }
                }
            }

            current_index = match_range.end();
        }
    }

    if current_index < message.len() {
        let text = &message[current_index..];
        process_urls(text, &mut data);
    }

    data
}

#[cfg(feature = "ssr")]
fn process_urls(text: &str, data: &mut Vec<MessageElement>) {
    for word in text.split_whitespace() {
        if word.starts_with("<") && word.ends_with(">") && word.len() > 2 {
            let url_string = &word[1..word.len() - 1];
            if let Ok(url) = Url::parse(url_string) {
                data.push(MessageElement::Url(url));
            }
        }
    }
}

#[server(SendMessage)]
pub async fn send_message(
    server_id: Uuid,
    channel_id: Uuid,
    message: String,
    member_id: Uuid,
    msg_reference: Option<Uuid>,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    auth()?;

    if message.is_empty() {
        return Err(ServerFnError::new("The message is empty"));
    }

    let mut message =
        ChannelMessage::add_channel_message(channel_id, member_id, &message, msg_reference, &pool)
            .await?;

    let elements = extract_message_elements(&message.content);
    let mut urls = vec![];

    for element in elements {
        match element {
            MessageElement::Member(id) => {
                if let Ok(member) = Member::check_member_on_server(id, server_id, &pool).await {
                    ChannelMessage::add_mention(message.id, member.id, &pool).await?;
                    message.mentions.push(member);
                }
            }
            MessageElement::Role(id) => {
                if let Ok(role) = Role::check_role_on_server(id, server_id, &pool).await {
                    ChannelMessage::add_role_mention(message.id, role.id, &pool).await?;
                    message.mentions_roles.push(role);
                }
            }
            MessageElement::Everyone => {
                ChannelMessage::mention_everyone(message.id, &pool).await?;
                message.mention_everyone = true;
            }
            MessageElement::Url(url) => {
                urls.push(url);
            }
        }
    }

    let id = message.id;
    // let _ = sync()?
    //     .broadcast(crate::sync::SyncRequest::Mutation(MutationRequest {
    //         key: format!("channe:{channel_id}"),
    //         module: "MessageStore".into(),
    //         data: json!(MessageStoreSync::Created {
    //             message: Box::new(message)
    //         }),
    //     }))
    //     .await;

    let mut embeds = vec![];

    for url in urls {
        if let Ok(op) = fetch_op_data(url.clone()).await {
            if let Ok(embed) = ChannelMessage::add_embed(id, op, url.to_string(), &pool).await {
                embeds.push(embed);
            }
        }
    }
    // let _ = sync()?
    //     .broadcast(SyncRequest::Mutation(MutationRequest {
    //         key: format!("channel:{channel_id}"),
    //         module: "ChannelStore".into(),
    //         data: json!(MessageSync::Embeds { id, embeds }),
    //     }))
    //     .await;

    Ok(id)
}

#[server(React)]
pub async fn react(
    name: String,
    message_id: Uuid,
    member_id: Uuid,
    channel_id: Uuid,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;
    if let Ok(reaction) = ChannelMessage::select_reaction(message_id, member_id, &name, &pool).await
    {
        debug!("{reaction:?}");
        if !reaction.me {
            ChannelMessage::inc_reaction_counter(reaction.id, &pool).await?;
            ChannelMessage::add_member_to_reaction(reaction.id, member_id, &pool).await?;
            // let _ = sync()?
            //     .broadcast(SyncRequest::Mutation(MutationRequest {
            //         key: format!("channel:{channel_id}"),
            //         module: "ChannelStore".into(),
            //         data: json!(MessageSync::MemberReact {
            //             member: member_id,
            //             id: message_id,
            //             reaction: reaction.id
            //         }),
            //     }))
            //     .await;
        }
    } else {
        let mut reaction = ChannelMessage::create_reaction(message_id, &name, &pool).await?;
        ChannelMessage::inc_reaction_counter(reaction.id, &pool).await?;
        ChannelMessage::add_member_to_reaction(reaction.id, member_id, &pool).await?;
        reaction.me = true;
        let reaction_id = reaction.id;
        // let _ = sync()?
        //     .broadcast(SyncRequest::Mutation(MutationRequest {
        //         key: format!("channel:{channel_id}"),
        //         module: "ChannelStore".into(),
        //         data: json!(MessageSync::NewReaction {
        //             id: message_id,
        //             reaction
        //         }),
        //     }))
        //     .await;
        // let _ = sync()?
        //     .broadcast(SyncRequest::Mutation(MutationRequest {
        //         key: format!("channel:{channel_id}"),
        //         module: "ChannelStore".into(),
        //         data: json!(MessageSync::MemberReact {
        //             member: member_id,
        //             id: message_id,
        //             reaction: reaction_id
        //         }),
        //     }))
        //     .await;
    }

    Ok(())
}

#[server(Unreact)]
pub async fn unreact(
    name: String,
    message_id: Uuid,
    member_id: Uuid,
    channel_id: Uuid,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;

    if let Ok(reaction) = ChannelMessage::select_reaction(message_id, member_id, &name, &pool).await
    {
        if reaction.me {
            ChannelMessage::remove_member_to_reaction(reaction.id, member_id, &pool).await?;
            // let _ = sync()?
            //     .broadcast(SyncRequest::Mutation(MutationRequest {
            //         key: format!("channel:{channel_id}"),
            //         module: "ChannelStore".into(),
            //         data: json!(MessageSync::MemberUnreact {
            //             member: member_id,
            //             id: message_id,
            //             reaction: reaction.id
            //         }),
            //     }))
            //     .await;
            if ChannelMessage::dec_reaction_counter(reaction.id, &pool).await? == 0 {
                ChannelMessage::delete_reaction(reaction.id, &pool).await?;
                // let _ = sync()?
                //     .broadcast(SyncRequest::Mutation(MutationRequest {
                //         key: format!("channel:{channel_id}"),
                //         module: "ChannelStore".into(),
                //         data: json!(MessageSync::DeletedReaction {
                //             id: message_id,
                //             reaction: reaction.id
                //         }),
                //     }))
                //     .await;
            }
        }
    }
    Ok(())
}
