use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::Error;
        use super::server::Server;
        use super::role::Role;
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Member {
    pub id: Uuid,
    pub user_id: Uuid,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn get(member_id: Uuid, pool: &MySqlPool) -> Result<Member, Error> {
        Ok(sqlx::query_as::<_, Member>(
        r#"
                SELECT members.id, members.server_id, members.user_id FROM members WHERE members.id = ?
            "#,
        )
        .bind(member_id)
        .fetch_one(pool)
        .await?)
    }
    pub async fn get_from_user_on_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        Ok(sqlx::query_as::<_, Member>(
        r#"
                SELECT members.id, members.server_id, members.user_id FROM members WHERE members.user_id = ? AND members.server_id = ?
            "#,
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_thread_members(
        thread_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
                    "SELECT members.id, members.server_id, members.user_id FROM members JOIN threads_members ON members.id = threads_members.member_id JOIN threads ON threads.id = threads_members.thread_id WHERE threads.id = ? LIMIT 5",
                )
                .bind(thread_id)
                .fetch_all(pool)
                .await?)
    }

    pub async fn get_members_without_role(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                    SELECT m.id, m.user_id, m.server_id
                    FROM members m
                    WHERE m.server_id = ?
                    AND NOT EXISTS (
                        SELECT 1
                        FROM member_roles mr
                        WHERE mr.member_id = m.id
                    )
                    "#,
        )
        .bind(server_id)
        .fetch_all(pool)
        .await?)
    }
    pub async fn get_member_from_role(
        role_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>(
            r#"
                    SELECT DISTINCT m.id, m.user_id, m.server_id
                    FROM members m
                    JOIN member_roles mr ON m.id = mr.member_id
                    JOIN roles r ON mr.role_id = r.id
                    WHERE r.priority = ?
                    AND NOT EXIST (
                        SELECT 1
                        FROM member_roles mr2
                        JOIN roles r2 ON mr2.role_id = r2.id
                        WHERE mr2.member_id = m.id
                        AND r2.priority > ? )
                    "#,
        )
        .bind(role_id)
        .bind(role_id)
        .fetch_all(pool)
        .await?)
    }
    pub async fn member_can_edit(user: Uuid, pool: &MySqlPool) -> Result<bool, Error> {
        match Role::get_member_roles(user, pool)
            .await?
            .iter()
            .find(|role| role.can_edit)
        {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn get_user_members(user_id: Uuid, pool: &MySqlPool) -> Result<Vec<Member>, Error> {
        Ok(sqlx::query_as::<_, Member>("SELECT members.id, members.user_id, members.server_id FROM members WHERE members.user_id = ?")
                    .bind(user_id)
                    .fetch_all(pool)
                    .await?)
    }

    pub async fn get_user_member(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, Error> {
        let res = sqlx::query_as::<_, Member>(
            "SELECT * FROM members WHERE members.user_id = ? AND members.server_id = ?",
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_one(pool)
        .await;
        Ok(res?)
    }
    pub async fn create(user: Uuid, server: Uuid, pool: &MySqlPool) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO members (id, user_id, server_id) VALUES (?,?,?)")
            .bind(id)
            .bind(user)
            .bind(server)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn create_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        let id = Uuid::new_v4();
        let server_id = Server::get_from_invitation(invitation, pool).await?;
        sqlx::query("INSERT INTO members (id, user_id, server_id, name) VALUES(?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(server_id)
    }

    pub async fn check_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, Error> {
        Ok(sqlx::query_as::<_, (Uuid,)>("SELECT servers.id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await?.0)
    }

    pub async fn delete_from_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), Error> {
        sqlx::query("DELETE FROM members WHERE user_id=? AND server_id=?")
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
