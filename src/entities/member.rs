use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool, Type};
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Type))]
pub enum Role {
    ADMIN,
    GUEST,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Member {
    pub id: Uuid,
    pub role: Role,
    pub user_id: u64,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn create(role: Role, user: Uuid, server: Uuid, pool: &MySqlPool) -> Option<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO members (id, role, user_id, server_id) VALUES (?, ?, ?,?)")
            .bind(id)
            .bind(role)
            .bind(user)
            .bind(server)
            .execute(pool)
            .await
            .ok()?;
        Some(id)
    }
}
