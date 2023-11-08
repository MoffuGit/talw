use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub invite_code: String,
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

    pub async fn get_from_invitation(invitation: Uuid, pool: &MySqlPool) -> Option<Uuid> {
        let id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers WHERE invite_code = ?")
            .bind(invitation)
            .fetch_one(pool)
            .await
            .ok()?;
        Some(id.0)
    }
}
