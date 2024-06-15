use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::server::Server;
        use sqlx::{FromRow, MySqlPool, Decode, Encode};
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    pub role: Role,
    pub user_id: Uuid,
    pub server_id: Uuid,
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn create(role: Role, user: Uuid, server: Uuid, pool: &MySqlPool) -> Option<Uuid> {
        let id = Uuid::new_v4();
        let error =
            sqlx::query("INSERT INTO members (id, role, user_id, server_id) VALUES (?,?,?,?)")
                .bind(id)
                .bind(role)
                .bind(user)
                .bind(server)
                .execute(pool)
                .await;
        log::info!("{:?}", error);
        error.ok()?;
        Some(id)
    }

    pub async fn create_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Option<Uuid> {
        let id = Uuid::new_v4();
        let server_id = Server::get_from_invitation(invitation, pool).await?;
        sqlx::query("INSERT INTO members (id, user_id, server_id) VALUES(?, ?, ?)")
            .bind(id)
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await
            .ok()?;
        Some(server_id)
    }

    pub async fn delete_from_server(
        user_id: Uuid,
        server_id: Uuid,
        pool: &MySqlPool,
    ) -> Option<()> {
        sqlx::query("DELETE FROM members WHERE user_id=? AND server_id=?")
            .bind(user_id)
            .bind(server_id)
            .execute(pool)
            .await
            .ok()?;
        Some(())
    }
}
