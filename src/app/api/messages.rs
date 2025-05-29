use std::str::FromStr;

use cfg_if::cfg_if;
use leptos::prelude::*;
use log::debug;
use server_fn::codec::{MultipartData, MultipartFormData};
use server_fn::ServerFnError;
use uuid::Uuid;

use crate::app::components::uploadthings::{FileType, UploadthingFile};
use crate::entities::message::ChannelMessage;
use crate::messages::{Message, ServerMessage};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::uploadthing::{FileData, UploadThing};
        use multer::bytes::Bytes as MulterBytes;
        use futures::TryStreamExt;
        use super::{auth_user, user_can_edit};
        use super::auth;
        use super::msg_sender;
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
    server_id: Uuid,
    pinned: bool,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    if user_can_edit(server_id, user.id, &pool).await? {
        ChannelMessage::pin(message_id, pinned, &pool).await?;
        msg_sender()?.send(ServerMessage {
            server_id,
            msg: if pinned {
                Message::PinMessage { message_id }
            } else {
                Message::UnpinMessage { message_id }
            },
        });
    }
    Ok(())
}

#[server(name = SendMessageAttachments, prefix = "/api", input = MultipartFormData)]
pub async fn send_message_attachments(data: MultipartData) -> Result<(), ServerFnError> {
    auth()?;
    let mut data = data.into_inner().unwrap();
    let mut message_id: Option<Uuid> = None;
    let mut server_id: Option<Uuid> = None;
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
            "server_id" => {
                if let Ok(Some(chunk)) = field.chunk().await {
                    if let Ok(id) = String::from_utf8(chunk.to_vec()) {
                        server_id = Uuid::from_str(&id).ok();
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
    let server_id =
        server_id.ok_or_else(|| ServerFnError::new("Something go wrong in our servers"))?;
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
    msg_sender()?.send(ServerMessage {
        server_id,
        msg: Message::MessageAttachments {
            message_id,
            content: attachments,
        },
    });

    Ok(())
}

#[server(SendMessage)]
pub async fn send_message(
    server_id: Uuid,
    channel_id: Uuid,
    message: String,
    member_id: Uuid,
    msg_reference: Option<Uuid>,
    attachments: bool,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    auth()?;

    if message.is_empty() {
        return Err(ServerFnError::new("The message is empty"));
    }

    let message =
        ChannelMessage::add_channel_message(channel_id, member_id, &message, msg_reference, &pool)
            .await?;
    let id = message.id;
    msg_sender()?.send(ServerMessage {
        server_id,
        msg: Message::ChannelMessage {
            attachments,
            channel_id,
            content: Box::new(message),
        },
    });

    Ok(id)
}

#[server(React)]
pub async fn react(
    name: String,
    message_id: Uuid,
    member_id: Uuid,
    server_id: Uuid,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;
    if let Ok(reaction) = ChannelMessage::select_reaction(message_id, member_id, &name, &pool).await
    {
        debug!("{reaction:?}");
        if !reaction.me {
            ChannelMessage::inc_reaction_counter(reaction.id, &pool).await?;
            ChannelMessage::add_member_to_reaction(reaction.id, member_id, &pool).await?;
            msg_sender()?.send(ServerMessage {
                server_id,
                msg: Message::MemberReact {
                    react_id: reaction.id,
                    message_id,
                    member_id,
                },
            });
        }
    } else {
        let mut reaction = ChannelMessage::create_reaction(message_id, &name, &pool).await?;
        ChannelMessage::inc_reaction_counter(reaction.id, &pool).await?;
        ChannelMessage::add_member_to_reaction(reaction.id, member_id, &pool).await?;
        reaction.me = true;
        let reaction_id = reaction.id;
        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ReactionCreated {
                reaction,
                message_id,
            },
        });
        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::MemberReact {
                react_id: reaction_id,
                message_id,
                member_id,
            },
        });
    }

    Ok(())
}

#[server(Unreact)]
pub async fn unreact(
    name: String,
    message_id: Uuid,
    member_id: Uuid,
    server_id: Uuid,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;

    if let Ok(reaction) = ChannelMessage::select_reaction(message_id, member_id, &name, &pool).await
    {
        if reaction.me {
            ChannelMessage::remove_member_to_reaction(reaction.id, member_id, &pool).await?;
            msg_sender()?.send(ServerMessage {
                server_id,
                msg: Message::MemberUnreact {
                    react_id: reaction.id,
                    message_id: reaction.message_id,
                    member_id,
                },
            });
            if ChannelMessage::dec_reaction_counter(reaction.id, &pool).await? == 0 {
                ChannelMessage::delete_reaction(reaction.id, &pool).await?;
                msg_sender()?.send(ServerMessage {
                    server_id,
                    msg: Message::ReactionDeleted {
                        reaction_id: reaction.id,
                        message_id,
                    },
                });
            }
        }
    }
    Ok(())
}
