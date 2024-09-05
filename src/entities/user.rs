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

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
}

#[cfg(feature = "ssr")]
impl User {
    pub async fn get(id: Uuid, pool: &MySqlPool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_from_username(
        username: String,
        pool: &MySqlPool,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await
    }

    pub async fn create(
        username: String,
        password: String,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
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
