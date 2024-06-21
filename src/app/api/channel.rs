use crate::entities::channel::Channel;
use crate::entities::channel::ChannelType;
use crate::entities::member::Role;
use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::auth_member;
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

#[server(GetGeneralChannels, "/api")]
pub async fn get_general_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    let _ = auth_user()?;
    let pool = pool()?;

    let channels = Server::get_general_channels(server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant get channels server, sorry".to_string()))?;

    Ok(channels)
}

#[server(GetChannelsWithCategory, "/api")]
pub async fn get_channels_with_category(
    server_id: Uuid,
    category_id: Uuid,
) -> Result<Vec<Channel>, ServerFnError> {
    let _ = auth_user()?;
    let pool = pool()?;

    let channels = Server::get_channels_with_category(server_id, category_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant get this channels with category".to_string()))?;

    Ok(channels)
}

#[server(CreateChannel, "/api")]
pub async fn create_channel(
    name: String,
    channel_type: ChannelType,
    server_id: Uuid,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    if name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    Channel::create(name, channel_type, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant create the new channel"))
}

#[server(CreateChannelWithCategory, "/api")]
pub async fn create_channel_with_category(
    name: String,
    channel_type: ChannelType,
    server_id: Uuid,
    category_id: Uuid,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    if name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    Channel::create_with_category(name, channel_type, server_id, category_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant create the new channel"))
}

#[server(RenameChannel)]
pub async fn rename_channel(
    server_id: Uuid,
    channel_id: Uuid,
    new_name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;

    if new_name.len() <= 1 {
        return Err(ServerFnError::new("min len is 1"));
    }

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    Channel::rename(new_name, channel_id, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant change the name"))
}

#[server(DeleteChannel)]
pub async fn delete_channel(server_id: Uuid, channel_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;

    if auth_member(server_id).await?.role != Role::ADMIN {
        return Err(ServerFnError::new(
            "the user is not an admin of this server",
        ));
    }

    Channel::delete(channel_id, server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("cant delte the channel"))
}
