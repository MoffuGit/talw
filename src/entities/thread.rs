use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::Error;
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Thread {
    pub id: Uuid,
    pub name: String,
    pub channel_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Thread {
    pub async fn get_threads_from_channel(
        channel_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Thread>, Error> {
        Ok(sqlx::query_as::<_, Thread>(
            "SELECT threads.id, threads.name, threads.channel_id FROM threads WHERE threads.channel_id = ?",
        )
        .bind(channel_id)
        .fetch_all(pool)
        .await?)
    }
    pub async fn get_from_id(thread_id: Uuid, pool: &MySqlPool) -> Result<Thread, Error> {
        Ok(sqlx::query_as::<_, Thread>(
            "SELECT id, name, channel_id FROM threads WHERE threads.id = ?",
        )
        .bind(thread_id)
        .fetch_one(pool)
        .await?)
    }
    pub async fn create(name: String, channel_id: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO threads (id, name, channel_id) VALUES ( ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn rename(new_name: String, channel_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("UPDATE threads SET threads.name = ? WHERE threads.id = ?")
            .bind(new_name)
            .bind(channel_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(thread_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("DELETE FROM threads WHERE id = ?")
            .bind(thread_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
