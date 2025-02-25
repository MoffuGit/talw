use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::member::Member;
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
    pub created_by: Uuid,
}

#[cfg(feature = "ssr")]
impl Thread {
    pub async fn get_members_witout_role(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
            SELECT DISTINCT m.id,
                m.user_id,
                m.server_id
FROM members m
INNER JOIN threads_members tm ON m.id = tm.member_id
INNER JOIN threads t ON t.id = tm.thread_id
WHERE t.id = ?
  AND NOT EXISTS (
    SELECT 1
    FROM member_roles mr
    WHERE mr.member_id = m.id
  );
                    "#,
        )
        .bind(thread_id)
        .fetch_all(pool)
        .await?)
    }

    pub async fn get_members_with_role(
        role_id: Uuid,
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                    SELECT DISTINCT m.id, m.user_id, m.server_id 
                    FROM members m
                    JOIN threads_members ON m.id = threads_members.member_id
                    JOIN threads ON threads.id = threads_members.thread_id
                    JOIN member_roles mr ON m.id = mr.member_id
                    JOIN roles r ON mr.role_id = r.id
                    WHERE r.priority = ? AND threads.id = ?
                    AND NOT EXIST (
                        SELECT 1
                        FROM member_roles mr2
                        JOIN roles r2 ON mr2.role_id = r2.id
                        WHERE mr2.member_id = m.id
                        AND r2.priority > ? )
                    "#,
        )
        .bind(role_id)
        .bind(thread_id)
        .bind(role_id)
        .fetch_all(pool)
        .await?)
    }
    pub async fn delete_members(thread_id: Uuid, pool: &MySqlPool) -> Result<(), Error> {
        sqlx::query("Delete FROM threads_members WHERE threads_members.thread_id = ?")
            .bind(thread_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_threads_for_member(
        channel_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Thread>, Error> {
        Ok(sqlx::query_as::<_, Thread>(
            "SELECT threads.id, threads.name, threads.channel_id, threads.created_by FROM threads JOIN threads_members ON threads_members.thread_id = threads.id JOIN members ON threads_members.member_id = members.id WHERE threads.channel_id = ? AND members.id = ?",
        )
        .bind(channel_id)
        .bind(member_id)
        .fetch_all(pool)
        .await?)
    }
    pub async fn check_member(
        thread_id: Uuid,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<bool, Error> {
        match sqlx::query_as::<_, (Uuid,)>("SELECT members.id FROM members JOIN threads_members ON members.id = threads_members.member_id JOIN threads ON threads_members.thread_id = threads.id WHERE members.user_id = ? AND threads.id = ?")
                    .bind(user_id)
                    .bind(thread_id)
                    .fetch_one(pool)
                    .await {
            Ok(_) => Ok(true),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(err) => Err(err.into())
        }
    }
    pub async fn add_member(
        thread_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO threads_members (thread_id, member_id) VALUES ( ?, ?)")
            .bind(thread_id)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn remove_member(
        thread_id: Uuid,
        member_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM threads_members WHERE thread_id = ? AND member_id = ?")
            .bind(thread_id)
            .bind(member_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_created_by(thread_id: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
        Ok(sqlx::query_as::<_, (Uuid,)>(
            "SELECT threads.created_by FROM threads WHERE threads.id = ?",
        )
        .bind(thread_id)
        .fetch_one(pool)
        .await?
        .0)
    }
    pub async fn get_threads_from_channel(
        channel_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Thread>, Error> {
        Ok(sqlx::query_as::<_, Thread>(
                    "SELECT threads.id, threads.name, threads.channel_id, threads.created_by FROM threads WHERE threads.channel_id = ? LIMIT 8",
                )
                .bind(channel_id)
                .fetch_all(pool)
                .await?)
    }

    pub async fn get(thread_id: Uuid, channel_id: Uuid, pool: &MySqlPool) -> Result<Thread, Error> {
        Ok(sqlx::query_as::<_, Thread>(
            "SELECT id, name, channel_id, created_by FROM threads WHERE threads.id = ? AND threads.channel_id = ?",
        )
        .bind(thread_id)
        .bind(channel_id)
        .fetch_one(pool)
        .await?)
    }
    pub async fn create(
        name: String,
        channel_id: Uuid,
        created_by: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO threads (id, name, channel_id, created_by) VALUES ( ?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(channel_id)
            .bind(created_by)
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
