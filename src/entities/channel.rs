use cfg_if::cfg_if;
use icondata::Icon;
use leptos::attr::IntoAttribute;
use leptos::prelude::IntoRender;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::Error;
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum ChannelType {
    TEXT,
    VOICE,
}

impl From<ChannelType> for Icon {
    fn from(value: ChannelType) -> Self {
        match value {
            ChannelType::TEXT => icondata::LuHash,
            ChannelType::VOICE => icondata::LuVolume2,
        }
    }
}

impl IntoRender for ChannelType {
    type Output = &'static str;

    fn into_render(self) -> Self::Output {
        match self {
            ChannelType::TEXT => "TEXT",
            ChannelType::VOICE => "VOICE",
        }
    }
}
// fn into_attribute(self) -> leptos::Attribute {
//     match self {
//         ChannelType::TEXT => leptos::attr::Attribute::String("TEXT".into()),
//         ChannelType::VOICE => leptos::attr::Attribute::String("VOICE".into()),
//     }
// }
//
// fn into_attribute_boxed(self: Box<Self>) -> leptos::Attribute {
//     self.into_attribute()
// }

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::MySql> for ChannelType {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub server_id: Uuid,
    pub category_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ChannelTopic(pub Option<String>);

#[cfg(feature = "ssr")]
impl Channel {
    pub async fn update_topic(
        channel_id: Uuid,
        topic: String,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE channels SET channels.topic = ? WHERE channels.id = ?")
            .bind(topic)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn get_channel_topic(
        channel_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<ChannelTopic, Error> {
        Ok(sqlx::query_as::<_, ChannelTopic>(
            "SELECT channels.topic FROM channels WHERE channels.id = ?",
        )
        .bind(channel_id)
        .fetch_one(pool)
        .await?)
    }
    pub async fn create(
        name: String,
        channel_type: ChannelType,
        server: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO channels (id, name, channel_type, server_id) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_type)
            .bind(server)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn create_with_category(
        name: String,
        channel_type: ChannelType,
        server: Uuid,
        category: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO channels (id, name, channel_type, server_id, category_id) VALUES (?, ?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_type)
            .bind(server)
            .bind(category)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn rename(new_name: String, channel_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE channels SET channels.name = ? WHERE channels.id = ?")
            .bind(new_name)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(channel_id: Uuid, server_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM channels WHERE server_id = ? AND id = ?")
            .bind(server_id)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn remove_all_from_category(
        server_id: Uuid,
        category_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE channels SET channels.category_id = NULL WHERE channels.category_id = ? AND channels.server_id = ?")
            .bind(
                category_id
            ).bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_channel(
        channel_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Channel, Error> {
        Ok(sqlx::query_as::<_, Channel>("SELECT channels.id,channels.name,channels.channel_type,channels.server_id,channels.category_id FROM channels LEFT JOIN servers ON servers.id = channels.server_id WHERE channels.id = ? AND servers.id = ?")
                    .bind(channel_id)
                    .bind(server_id)
                    .fetch_one(pool)
                    .await?)
    }
}
