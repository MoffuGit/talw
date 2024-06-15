use cfg_if::cfg_if;
use leptos::IntoAttribute;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum ChannelType {
    TEXT,
    VOICE,
    ANNOUNCEMENTS,
    RULES,
}

impl IntoAttribute for ChannelType {
    fn into_attribute(self) -> leptos::Attribute {
        match self {
            ChannelType::TEXT => leptos::Attribute::String("TEXT".into()),
            ChannelType::VOICE => leptos::Attribute::String("VOICE".into()),
            ChannelType::ANNOUNCEMENTS => leptos::Attribute::String("ANNOUNCEMENTS".into()),
            ChannelType::RULES => leptos::Attribute::String("RULES".into()),
        }
    }

    fn into_attribute_boxed(self: Box<Self>) -> leptos::Attribute {
        self.into_attribute()
    }
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

    pub async fn rename(
        new_name: String,
        channel_id: Uuid,
        server: Uuid,
        pool: &MySqlPool,
    ) -> Option<()> {
        sqlx::query("UPDATE channels SET channels.name = ? WHERE channels.server_id = ? AND channels.id = ?")
            .bind(new_name)
            .bind(
                server
            ).bind(channel_id)
            .execute(pool)
            .await
            .ok()?;
        Some(())
    }
}
