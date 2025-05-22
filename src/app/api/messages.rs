use cfg_if::cfg_if;
use leptos::prelude::*;
use log::debug;
use server_fn::ServerFnError;
use uuid::Uuid;

use crate::entities::message::ChannelMessage;
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
pub async fn get_pinned_messages(
    channel_id: Uuid,
    member_id: Uuid,
) -> Result<Vec<ChannelMessage>, ServerFnError> {
    let pool = pool()?;
    auth()?;

    Ok(ChannelMessage::get_pinned(channel_id, member_id, &pool).await?)
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
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth()?;

    match ChannelMessage::add_channel_message(channel_id, member_id, message, &pool).await {
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
