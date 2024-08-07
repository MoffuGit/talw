use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::member::Member;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::{category::Category, channel::Channel};
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub invite_code: Uuid,
    pub image_url: Option<String>,
}

#[cfg(feature = "ssr")]
impl Server {
    pub async fn create(name: String, pool: &MySqlPool) -> Option<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO servers (id, name, invite_code) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(Uuid::new_v4())
            .execute(pool)
            .await
            .ok()?;
        Some(id)
    }

    pub async fn set_image_url(url: String, server_id: Uuid, pool: &MySqlPool) -> Option<()> {
        sqlx::query("UPDATE servers SET servers.image_url = ? WHERE servers.id = ?")
            .bind(url)
            .bind(server_id)
            .execute(pool)
            .await
            .ok()?;
        Some(())
    }

    pub async fn get_from_invitation(invitation: Uuid, pool: &MySqlPool) -> Option<Uuid> {
        let id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers WHERE invite_code = ?")
            .bind(invitation)
            .fetch_one(pool)
            .await
            .ok()?;
        Some(id.0)
    }

    pub async fn check_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Option<Uuid> {
        let id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await.ok()?;
        Some(id.0)
    }

    pub async fn check_server(server_id: Uuid, user_id: Uuid, pool: &MySqlPool) -> Option<Server> {
        let server = sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code, servers.image_url FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.id = ?").bind(user_id).bind(server_id).fetch_one(pool).await.ok()?;
        Some(server)
    }

    pub async fn get_general_channels(server_id: Uuid, pool: &MySqlPool) -> Option<Vec<Channel>> {
        sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id IS NULL",
        )
        .bind(server_id)
        .fetch_all(pool)
        .await
        .ok()
    }

    pub async fn get_channels_with_category(
        server_id: Uuid,
        category_id: Uuid,
        pool: &MySqlPool,
    ) -> Option<Vec<Channel>> {
        sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id = ?",
        )
        .bind(server_id)
        .bind(category_id)
        .fetch_all(pool)
        .await
        .ok()
    }

    pub async fn get_categories(server_id: Uuid, pool: &MySqlPool) -> Option<Vec<Category>> {
        let categories =
            sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE server_id = ?")
                .bind(server_id)
                .fetch_all(pool)
                .await;
        categories.ok()
    }

    pub async fn get_channels(server_id: Uuid, pool: &MySqlPool) -> Option<Vec<Channel>> {
        let channels = sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE server_id = ?")
            .bind(server_id)
            .fetch_all(pool)
            .await;
        channels.ok()
    }

    pub async fn get_member(server_id: Uuid, user_id: Uuid, pool: &MySqlPool) -> Option<Member> {
        let member = sqlx::query_as::<_, Member>(
            "SELECT * FROM members WHERE server_id = ? AND user_id = ?",
        )
        .bind(server_id)
        .bind(user_id)
        .fetch_one(pool)
        .await;
        // log::info!("{member:?}");
        member.ok()
    }
}
