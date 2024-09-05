use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::server::Server;
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "ssr", derive(Decode, Encode))]
pub enum Role {
    ADMIN,
    GUEST,
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::MySql> for Role {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        <str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        <str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub user_id: Uuid,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn get_member_role(
        server_id: Uuid,
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Role, sqlx::Error> {
        Ok(sqlx::query_as::<_, (Role,)>(
            "SELECT members.role FROM members WHERE members.user_id = ? AND members.server_id = ?",
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_one(pool)
        .await?
        .0)
    }
    pub async fn get_user_members(
        user_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Vec<Member>, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT members.id, members.role, members.user_id, members.server_id, members.name FROM members WHERE members.user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }
    pub async fn get_user_member(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Result<Member, sqlx::Error> {
        sqlx::query_as::<_, Member>("SELECT members.id, members.role, members.user_id, members.server_id, members.name FROM members WHERE members.user_id = ? AND members.server_id = ?")
            .bind(user_id)
            .bind(server_id)
            .fetch_one(pool)
            .await
    }
    pub async fn create(
        role: Role,
        user: Uuid,
        server: Uuid,
        name: String,
        pool: &MySqlPool,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO members (id, role, user_id, server_id, name) VALUES (?,?,?,?,?)")
            .bind(id)
            .bind(role)
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
        Ok(sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await?.0)
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
