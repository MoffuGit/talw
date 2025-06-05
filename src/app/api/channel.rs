use crate::app::stores::ChannelStoreSync;
use crate::entities::channel::Channel;
use crate::entities::channel::ChannelType;
use crate::sync::SubscriptionMode;
use crate::sync::SyncRequest;
use cfg_if::cfg_if;
use leptos::prelude::*;
use serde_json::json;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::sync;
        use crate::entities::server::Server;
        use super::user_can_edit;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ChannelContext {
    pub create_channel: ServerAction<CreateChannel>,
    pub delete_channel: ServerAction<DeleteChannel>,
    pub update_channel: ServerAction<UpdateChannel>,
}

pub fn use_channel() -> ChannelContext {
    use_context::<ChannelContext>().expect("have channel context")
}

pub fn provide_channel_context() {
    let create_channel = ServerAction::<CreateChannel>::new();
    let delete_channel = ServerAction::<DeleteChannel>::new();
    let update_channel = ServerAction::<UpdateChannel>::new();

    provide_context(ChannelContext {
        create_channel,
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

        let _ = sync()?
            .broadcast(SyncRequest::Mutation {
                key: format!("channelStore:channel:{channel_id}"),
                data: json!(ChannelStoreSync::Updated { id: channel_id }),
            })
            .await;
        Ok(())
    } else {
        Err(ServerFnError::new("You cant updatge this"))
    }
}

#[server(GetAllChannels)]
pub async fn get_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    let user = auth_user()?;
    let pool = pool()?;

    let channels = Server::get_channels(server_id, &pool).await?;

    let sender = sync()?;

    sender
        .broadcast(SyncRequest::Subscription {
            keys: vec![format!("channelStore:server:{server_id}")],
            client: user.id,
            action: SubscriptionMode::ReplacePrefix("channelsStore:server:".into()),
        })
        .await;
    sender
        .broadcast(SyncRequest::Subscription {
            keys: channels
                .iter()
                .map(|channel| format!("channelStore:channel:{}", channel.id))
                .collect(),
            client: user.id,
            action: SubscriptionMode::ReplacePrefix("channelsStore:channel:".into()),
        })
        .await;

    Ok(channels)
}

#[server(CreateChannel)]
pub async fn create_channel(
    name: String,
    channel_type: ChannelType,
    server_id: Uuid,
    category_id: Option<Uuid>,
) -> Result<Uuid, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    if user_can_edit(server_id, user.id, &pool).await? {
        if name.len() <= 1 {
            return Err(ServerFnError::new("the name have a min len of 1 char"));
        }

        let channel_id = if let Some(category_id) = category_id {
            Channel::create_with_category(&name, channel_type, server_id, category_id, &pool)
                .await?
        } else {
            Channel::create(&name, channel_type, server_id, &pool).await?
        };

        sync()?
            .broadcast(SyncRequest::Mutation {
                key: format!("channelStore:server:{server_id}"),
                data: json!(ChannelStoreSync::Created {
                    channel: Channel {
                        id: channel_id,
                        name,
                        channel_type,
                        server_id,
                        category_id,
                        topic: None,
                    }
                }),
            })
            .await;

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
        sync()?
            .broadcast(SyncRequest::Mutation {
                key: format!("channelStore:server:{server_id}"),
                data: json!(ChannelStoreSync::Deleted { id: channel_id }),
            })
            .await;
        return Ok(());
    }

    Err(ServerFnError::new("You cant create a channel"))
}
