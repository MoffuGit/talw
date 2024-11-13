use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use async_trait::async_trait;
        use axum_session::SessionMySqlPool;
        use axum_session_auth::Authentication;
        use bcrypt::{hash, DEFAULT_COST};
        use sqlx::{FromRow, MySqlPool};
        pub type AuthSession = axum_session_auth::AuthSession<User, Uuid, SessionMySqlPool, MySqlPool>;
        use super::Error;

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct AboutUser(pub Option<String>);

#[cfg(feature = "ssr")]
impl User {
    pub async fn get_name_and_image_url(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(String, Option<String>), Error> {
        Ok(sqlx::query_as::<_, (String, Option<String>)>(
            "SELECT users.username, users.image_url FROM users WHERE users.id = ?",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_about(user_id: Uuid, pool: &MySqlPool) -> Result<AboutUser, Error> {
        Ok(
            sqlx::query_as::<_, AboutUser>("SELECT users.about FROM users WHERE users.id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?,
        )
    }
    pub async fn get_user_name(user_id: Uuid, pool: &MySqlPool) -> Result<String, Error> {
        Ok(
            sqlx::query_as::<_, (String,)>("SELECT users.username FROM users WHERE users.id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?
                .0,
        )
    }

    pub async fn get_image_url(user_id: Uuid, pool: &MySqlPool) -> Result<Option<String>, Error> {
        Ok(
            sqlx::query_as::<_, (Option<String>,)>("SELECT image_url FROM users WHERE id = ?")
                .bind(user_id)
                .fetch_one(pool)
                .await?
                .0,
        )
    }
    pub async fn get(id: Uuid, pool: &MySqlPool) -> Result<User, Error> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(id)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_from_username(username: String, pool: &MySqlPool) -> Result<User, Error> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
                .bind(username)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn create(
        username: String,
        password: String,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let password = hash(password, DEFAULT_COST).unwrap();
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO users (id, username, password) VALUES (?,?,?)")
            .bind(id)
            .bind(username)
            .bind(password)
            .execute(pool)
            .await?;
        Ok(id)
    }
}

#[async_trait]
#[cfg(feature = "ssr")]
impl Authentication<User, Uuid, MySqlPool> for User {
    async fn load_user(userid: Uuid, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get(userid, pool)
            .await
            .or(Err(anyhow::anyhow!("Cannot get user")))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
