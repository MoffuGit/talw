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
    pub owner_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Server {
    pub async fn get_user_servers(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Server>, sqlx::Error> {
        sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code, servers.image_url, servers.owner_id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }
    pub async fn get_server_members(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT * FROM members WHERE server_id = ?")
            .bind(server_id)
            .fetch_all(pool)
            .await
    }
    pub async fn create(
        name: String,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO servers (id, name, invite_code, owner_id) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(Uuid::new_v4())
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn set_image_url(
        url: String,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE servers SET servers.image_url = ? WHERE servers.id = ?")
            .bind(url)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_from_invitation(
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        Ok(
            sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers WHERE invite_code = ?")
                .bind(invitation)
                .fetch_one(pool)
                .await?
                .0,
        )
    }

    pub async fn get_server(server_id: Uuid, pool: &MySqlPool) -> Result<Server, sqlx::Error> {
        sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code, servers.image_url, servers.owner_id FROM servers WHERE servers.id = ?").bind(server_id).fetch_one(pool).await
    }

    pub async fn get_server_owner(server_id: Uuid, pool: &MySqlPool) -> Result<Uuid, sqlx::Error> {
        Ok(sqlx::query_as::<_, (Uuid,)>(
            "SELECT servers.owner_id FROM servers WHERE servers.id = ?",
        )
        .bind(server_id)
        .fetch_one(pool)
        .await?
        .0)
    }

    pub async fn get_general_channels(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id IS NULL",
        )
        .bind(server_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_channels_with_category(
        server_id: Uuid,
        category_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id = ?",
        )
        .bind(server_id)
        .bind(category_id)
        .fetch_all(pool)
        .await
    }

    pub async fn get_server_categories(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE server_id = ?")
            .bind(server_id)
            .fetch_all(pool)
            .await
    }

    pub async fn get_channels(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE server_id = ?")
            .bind(server_id)
            .fetch_all(pool)
            .await
    }
}
