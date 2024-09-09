use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::server::Server;
        use super::role::Role;
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    pub server_id: Uuid,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct AboutMember(pub Option<String>);

#[cfg(feature = "ssr")]
impl Member {
    pub async fn get_members_without_role(
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, sqlx::Error> {
        sqlx::query_as::<_, Member>(
            r#"
            SELECT m.id, m.user_id, m.server_id, m.name, m.image_url
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
        .await
    }
    pub async fn get_member_from_role(
        role_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, sqlx::Error> {
        sqlx::query_as::<_, Member>(
            r#"
            SELECT DISTINCT m.id, m.user_id, m.server_id, m.name, m.image_url
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
        .await
    }
    pub async fn member_can_edit(
        user: Uuid,
        server: Uuid,
        pool: &MySqlPool,
    ) -> Result<bool, sqlx::Error> {
        match Role::get_member_roles(user, server, pool)
            .await?
            .iter()
            .find(|role| role.can_edit)
        {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn get_user_members(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT members.id, members.user_id, members.server_id, members.name FROM members WHERE members.user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }
    pub async fn get_user_member(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT members.id, members.user_id, members.server_id, members.name FROM members WHERE members.user_id = ? AND members.server_id = ?")
            .bind(user_id)
            .bind(server_id)
            .fetch_one(pool)
            .await
    }
    pub async fn create(
        user: Uuid,
        server: Uuid,
        name: String,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO members (id, user_id, server_id, name) VALUES (?,?,?,?)")
            .bind(id)
            .bind(user)
            .bind(server)
            .bind(name)
            .execute(pool)
            .await?;
        Ok(id)
    }

    pub async fn create_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        name: String,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let server_id = Server::get_from_invitation(invitation, pool).await?;
        sqlx::query("INSERT INTO members (id, user_id, server_id, name) VALUES(?, ?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(server_id)
            .bind(name)
            .execute(pool)
            .await?;
        Ok(server_id)
    }

    pub async fn get_member(
        server_id: Uuid,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT * FROM members WHERE server_id = ? AND user_id = ?")
            .bind(server_id)
            .bind(user_id)
            .fetch_one(pool)
            .await
        // log::info!("{member:?}");
    }

    pub async fn check_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        Ok(sqlx::query_as::<_, (Uuid,)>("SELECT servers.id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await?.0)
    }

    pub async fn delete_from_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM members WHERE user_id=? AND server_id=?")
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
