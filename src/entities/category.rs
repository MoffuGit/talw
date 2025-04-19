use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::Error;
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Category {
    pub async fn create(name: &str, server: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO categories (id, name, server_id) VALUES (?,?,?)")
            .bind(id)
            .bind(name)
            .bind(server)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn rename(
        new_name: &str,
        channel_id: Uuid,
        server: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("UPDATE categories SET categories.name = ? WHERE categories.server_id = ? AND categories.id = ?")
            .bind(new_name)
            .bind(
                server
            ).bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }
    pub async fn delete(id: Uuid, server_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM categories WHERE server_id = ? AND id = ?")
            .bind(server_id)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
