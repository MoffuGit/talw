use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Category {
    pub async fn create(name: String, server: Uuid, pool: &MySqlPool) -> Option<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO categories (id, name, server_id) VALUES (?,?,?)")
            .bind(id)
            .bind(name)
            .bind(server)
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
        sqlx::query("UPDATE category SET category.name = ? WHERE category.server_id = ? AND category.id = ?")
            .bind(new_name)
            .bind(
                server
            ).bind(channel_id)
            .execute(pool)
            .await
            .ok()?;
        Some(())
    }
    pub async fn delete(id: Uuid, server_id: Uuid, pool: &MySqlPool) -> Option<()> {
        sqlx::query("DELETE FROM category WHERE server_id = ? AND id = ?")
            .bind(server_id)
            .bind(id)
            .execute(pool)
            .await
            .ok()?;
        Some(())
    }
}
