use cfg_if::cfg_if;
use leptos::prelude::*;
use log::debug;
use server_fn::ServerFnError;
use uuid::Uuid;

use crate::entities::message::ChannelMessage;
use crate::messages::{Message, ServerMessage};

cfg_if! {
    if #[cfg(feature = "ssr")] {
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
                content,
            },
        }),
        Err(err) => debug!("{err:?}"),
    }

    Ok(())
}
