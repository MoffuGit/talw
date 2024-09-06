use crate::entities::channel::Channel;
use crate::entities::channel::ChannelType;
use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

use super::server;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::user_can_edit;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ChannelContext {
    pub create_channel: Action<CreateChannel, Result<Uuid, ServerFnError>>,
    pub create_channel_with_category:
        Action<CreateChannelWithCategory, Result<Uuid, ServerFnError>>,
    pub rename_channel: Action<RenameChannel, Result<(), ServerFnError>>,
    pub delete_channel: Action<DeleteChannel, Result<(), ServerFnError>>,
}

pub fn use_channel() -> ChannelContext {
    use_context::<ChannelContext>().expect("have channel context")
}

pub fn provide_channel_context() {
    let create_channel = create_server_action::<CreateChannel>();
    let rename_channel = create_server_action::<RenameChannel>();
    let create_channel_with_category = create_server_action::<CreateChannelWithCategory>();
    let delete_channel = create_server_action::<DeleteChannel>();

    provide_context(ChannelContext {
        create_channel,
        create_channel_with_category,
        rename_channel,
        delete_channel,
    })
}

#[server(GetChannel, "/api")]
pub async fn get_channel(channel_id: Uuid, server_id: Uuid) -> Result<Channel, ServerFnError> {
    auth_user()?;
    let pool = pool()?;
    let channel = Channel::get_channel(channel_id, server_id, &pool)
        .await
        .or(Err(ServerFnError::new("cant get the channel")))?;
    Ok(channel)
}

#[server(GetAllChannels, "/api")]
pub async fn get_all_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    let channels = Server::get_channels(server_id, &pool)
        .await
        .or(Err(ServerFnError::new("cant get channels server, sorry")))?;

    Ok(channels)
}

#[server(GetGeneralChannels, "/api")]
pub async fn get_general_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    let channels = Server::get_general_channels(server_id, &pool)
        .await
        .or(Err(ServerFnError::new("cant get channels server, sorry")))?;

    Ok(channels)
}

#[server(GetChannelsWithCategory, "/api")]
pub async fn get_channels_with_category(
    server_id: Uuid,
    category_id: Uuid,
) -> Result<Vec<Channel>, ServerFnError> {
    auth_user()?;
    let pool = pool()?;

    let channels = Server::get_channels_with_category(server_id, category_id, &pool)
        .await
        .or(Err(ServerFnError::new(
            "cant get this channels with category",
        )))?;

    Ok(channels)
}

#[server(CreateChannel, "/api")]
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

        return Channel::create(name, channel_type, server_id, &pool)
            .await
            .or(Err(ServerFnError::new("We cant create the new channel")));
    }
    Err(ServerFnError::new("You cant create a channel"))
}

#[server(CreateChannelWithCategory, "/api")]
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

        return Channel::create_with_category(name, channel_type, server_id, category_id, &pool)
            .await
            .or(Err(ServerFnError::new("cant create the new channel")));
    }

    Err(ServerFnError::new("You cant create a channel"))
}

#[server(RenameChannel, "/api")]
pub async fn rename_channel(
    server_id: Uuid,
    channel_id: Uuid,
    new_name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if new_name.len() <= 1 {
            return Err(ServerFnError::new("min len is 1"));
        }

        return Channel::rename(new_name, channel_id, server_id, &pool)
            .await
            .or(Err(ServerFnError::new("cant change the name")));
    }
    Err(ServerFnError::new("You cant create a channel"))
}

#[server(DeleteChannel, "/api")]
pub async fn delete_channel(server_id: Uuid, channel_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        return Channel::delete(channel_id, server_id, &pool)
            .await
            .or(Err(ServerFnError::new("cant delte the channel")));
    }

    Err(ServerFnError::new("You cant create a channel"))
}
