use crate::entities::server::Server;
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
    pub async fn get(id: Uuid, pool: &MySqlPool) -> Option<Self> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;

        Some(user)
    }

    pub async fn get_from_username(username: String, pool: &MySqlPool) -> Option<Self> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(pool)
            .await
            .ok()?;
        Some(user)
    }

    pub async fn create(username: String, password: String, pool: &MySqlPool) -> Option<Uuid> {
        let password = hash(password, DEFAULT_COST).unwrap();
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO users (id, username, password) VALUES (?,?,?)")
            .bind(id)
            .bind(username)
            .bind(password)
            .execute(pool)
            .await
            .ok()?;
        Some(id)
    }

    pub async fn get_servers(self, pool: &MySqlPool) -> Option<Vec<Server>> {
        let servers = sqlx::query_as::<_, Server>("SELECT * FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ?")
            .bind(self.id)
            .fetch_all(pool)
            .await;
        log::info!("{:?}", servers);
        servers.ok()
    }
}

#[async_trait]
#[cfg(feature = "ssr")]
impl Authentication<User, Uuid, MySqlPool> for User {
    async fn load_user(userid: Uuid, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
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
