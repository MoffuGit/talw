use cfg_if::cfg_if;
use emojis::Emoji;
use leptos::prelude::*;
use log::debug;
use server_fn::ServerFnError;
use uuid::Uuid;

use crate::entities::message::{ChannelMessage, Reaction};
use crate::messages::{Message, ServerMessage};

cfg_if! {
    if #[cfg(feature = "ssr")] {
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

#[server(SendMessage)]
pub async fn send_message(
    server_id: Uuid,
    channel_id: Uuid,
    message: String,
    member_id: Uuid,
    msg_reference: Option<Uuid>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;

    if message.is_empty() {
        return Ok(());
    }

    match ChannelMessage::add_channel_message(channel_id, member_id, message, msg_reference, &pool)
        .await
    {
        Ok(content) => msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ChannelMessage {
                channel_id,
                content: Box::new(content),
            },
        }),
        Err(err) => debug!("{err:?}"),
    }

    Ok(())
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
                    member_id: member_id,
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
