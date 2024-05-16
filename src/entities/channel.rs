use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum ChannelType {
    TEXT,
    VOICE,
    ANNOUNCEMENTS,
    RULES,
}

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

#[cfg(feature = "ssr")]
impl Channel {
    pub async fn create(
        name: String,
        channel_type: ChannelType,
        server: Uuid,
        pool: &MySqlPool,
    ) -> Option<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO channels (id, name, channel_type, server_id) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_type)
            .bind(server)
            .execute(pool)
            .await
            .ok()?;
        Some(id)
    }

    pub async fn create_with_category(
        name: String,
        channel_type: ChannelType,
        server: Uuid,
        category: Uuid,
        pool: &MySqlPool,
    ) -> Option<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO channels (id, name, channel_type, server_id, category_id) VALUES (?, ?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_type)
            .bind(server)
            .bind(category)
            .execute(pool)
            .await
            .ok()?;
        Some(id)
    }
}
