use super::server;
use crate::entities::channel::Channel;
use crate::entities::channel::ChannelType;
use crate::messages::Message;
use crate::messages::ServerMessage;
use cfg_if::cfg_if;
use leptos::prelude::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::msg_sender;
        use crate::entities::server::Server;
        use super::user_can_edit;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ChannelContext {
    pub create_channel: ServerAction<CreateChannel>,
    pub create_channel_with_category: ServerAction<CreateChannelWithCategory>,
    pub delete_channel: ServerAction<DeleteChannel>,
    pub update_channel: ServerAction<UpdateChannel>,
}

pub fn use_channel() -> ChannelContext {
    use_context::<ChannelContext>().expect("have channel context")
}

pub fn provide_channel_context() {
    let create_channel = ServerAction::<CreateChannel>::new();
    let create_channel_with_category = ServerAction::<CreateChannelWithCategory>::new();
    let delete_channel = ServerAction::<DeleteChannel>::new();
    let update_channel = ServerAction::<UpdateChannel>::new();

    provide_context(ChannelContext {
        create_channel,
        create_channel_with_category,
        delete_channel,
        update_channel,
    })
}

#[server(GetChannel)]
pub async fn get_channel(channel_id: Uuid, server_id: Uuid) -> Result<Channel, ServerFnError> {
    auth_user()?;
    let pool = pool()?;
    Ok(Channel::get_channel(channel_id, server_id, &pool).await?)
}

#[server(UpdateChannel)]
pub async fn update_channel(
    channel_id: Uuid,
    server_id: Uuid,
    topic: Option<String>,
    name: Option<String>,
) -> Result<(), ServerFnError> {
    let user = auth_user()?;
    let pool = pool()?;
    if user_can_edit(server_id, user.id, &pool).await? {
        if let Some(ref name) = name {
            Channel::rename(name, channel_id, &pool).await?;
        };

        if let Some(ref topic) = topic {
            Channel::update_topic(channel_id, topic, &pool).await?;
        }

        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ChannelUpdated {
                topic,
                name,
                channel_id,
            },
        });
        Ok(())
    } else {
        Err(ServerFnError::new("You cant updatge this"))
    }
}

#[server(GetAllChannels)]
pub async fn get_all_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    Ok(Server::get_channels(server_id, &pool).await?)
}

#[server(GetGeneralChannels)]
pub async fn get_general_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    Ok(Server::get_general_channels(server_id, &pool).await?)
}

#[server(GetChannelsWithCategory)]
pub async fn get_channels_with_category(
    server_id: Uuid,
    category_id: Uuid,
) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    Ok(Server::get_channels_with_category(server_id, category_id, &pool).await?)
}

#[server(CreateChannel)]
pub async fn create_channel(
    name: String,
    channel_type: ChannelType,
    server_id: Uuid,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if name.len() <= 1 {
            return Err(ServerFnError::new("the name have a min len of 1 char"));
        }

        let channel_id = Channel::create(&name, channel_type, server_id, &pool).await?;
        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ChannelCreated {
                new_channel: Channel {
                    id: channel_id,
                    name,
                    channel_type,
                    server_id,
                    category_id: None,
                    topic: None,
                },
            },
        });
        return Ok(channel_id);
    }
    Err(ServerFnError::new("You cant create a channel"))
}

#[server(CreateChannelWithCategory)]
pub async fn create_channel_with_category(
    name: String,
    channel_type: ChannelType,
    server_id: Uuid,
    category_id: Uuid,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if name.len() <= 1 {
            return Err(ServerFnError::new("min len is 1"));
        }

        let channel_id =
            Channel::create_with_category(&name, channel_type, server_id, category_id, &pool)
                .await?;

        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ChannelCreated {
                new_channel: Channel {
                    id: channel_id,
                    name,
                    channel_type,
                    server_id,
                    category_id: Some(category_id),
                    topic: None,
                },
            },
        });

        return Ok(channel_id);
    }

    Err(ServerFnError::new("You cant create a channel"))
}

#[server(DeleteChannel)]
pub async fn delete_channel(server_id: Uuid, channel_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        Channel::delete(channel_id, server_id, &pool).await?;
        msg_sender()?.send(ServerMessage {
            server_id,
            msg: Message::ChannelDeleted { channel_id },
        });
        return Ok(());
    }

    Err(ServerFnError::new("You cant create a channel"))
}
