use cfg_if::cfg_if;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::member::Member;
        use super::{category::Category, channel::Channel};
        use sqlx::{FromRow, MySqlPool};
        use super::Error;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Store)]
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
    pub async fn set_server_name(
        new_name: &str,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE servers SET servers.name = ? WHERE servers.id = ?")
            .bind(new_name)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn member_exist(
        server_id: Uuid,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<bool, Error> {
        Ok(sqlx::query_as::<_, (bool,)>("SELECT EXIST(SELECT * FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.id = ?)")
                    .bind(user_id)
                    .bind(server_id)
                    .fetch_one(pool)
                    .await?.0)
    }
    pub async fn get_mutual_servers_image_url(
        user1: Uuid,
        user2: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Option<String>>, Error> {
        Ok(sqlx::query_as::<_, (Option<String>,)>(
            r#"
                    WITH user_servers AS (
                        SELECT u.id AS user_id, s.id AS server_id, s.image_url AS image_url
                        FROM users u
                        JOIN members m ON u.id = m.user_id
                        JOIN servers s ON m.server_id = s.id
                        WHERE u.id IN (?, ?)
                    )
                    SELECT image_url
                    FROM user_servers
                    GROUP BY server_id
                    HAVING COUNT(DISTINCT user_id) = 2;
                "#,
        )
        .bind(user1)
        .bind(user2)
        .fetch_all(pool)
        .await?
        .iter()
        .map(|(image_url,)| image_url.clone())
        .collect::<Vec<Option<String>>>())
    }
    pub async fn get_user_servers(user_id: Uuid, pool: &MySqlPool) -> Result<Vec<Server>, Error> {
        Ok(sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code, servers.image_url, servers.owner_id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ?")
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?)
    }
    pub async fn get_server_members(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(
            sqlx::query_as::<_, Member>("SELECT * FROM members WHERE server_id = ?")
                .bind(server_id)
                .fetch_all(pool)
                .await?,
        )
    }
    pub async fn create(name: &str, user_id: Uuid, pool: &MySqlPool) -> Result<Server, Error> {
        let id = Uuid::new_v4();
        let invite_code = Uuid::new_v4();
        sqlx::query("INSERT INTO servers (id, name, invite_code, owner_id) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(invite_code)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(Server {
            id,
            name: name.to_string(),
            invite_code,
            image_url: None,
            owner_id: id,
        })
    }

    pub async fn get_server_image_key(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Option<String>, Error> {
        Ok(sqlx::query_as::<_, (Option<String>,)>(
            "SELECT servers.image_key FROM servers WHERE servers.id = ?",
        )
        .bind(server_id)
        .fetch_one(pool)
        .await?
        .0)
    }

    pub async fn set_image_url(
        url: &str,
        key: &str,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query(
            "UPDATE servers SET servers.image_url = ?, servers.image_key = ? WHERE servers.id = ?",
        )
        .bind(url)
        .bind(key)
        .bind(server_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_from_invitation(invitation: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
        Ok(
            sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers WHERE invite_code = ?")
                .bind(invitation)
                .fetch_one(pool)
                .await?
                .0,
        )
    }

    pub async fn get_server(server_id: Uuid, pool: &MySqlPool) -> Result<Server, Error> {
        Ok(sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code, servers.image_url, servers.owner_id FROM servers WHERE servers.id = ?").bind(server_id).fetch_one(pool).await?)
    }

    pub async fn get_server_owner(server_id: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
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
    ) -> Result<Vec<Channel>, Error> {
        Ok(sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id IS NULL",
        )
        .bind(server_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_channels_with_category(
        server_id: Uuid,
        category_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Channel>, Error> {
        Ok(sqlx::query_as::<_, Channel>(
            "SELECT * FROM channels WHERE server_id = ? AND category_id = ?",
        )
        .bind(server_id)
        .bind(category_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_server_categories(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Category>, Error> {
        Ok(
            sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE server_id = ?")
                .bind(server_id)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_channels(server_id: Uuid, pool: &MySqlPool) -> Result<Vec<Channel>, Error> {
        Ok(
            sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE server_id = ?")
                .bind(server_id)
                .fetch_all(pool)
                .await?,
        )
    }
}
