use cfg_if::cfg_if;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
        use super::Error;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Store, Eq, PartialEq)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub server_id: Uuid,
    pub can_edit: bool,
    pub priority: u8,
}

#[cfg(feature = "ssr")]
impl Role {
    pub async fn get_server_roles(server: Uuid, pool: &MySqlPool) -> Result<Vec<Role>, Error> {
        Ok(sqlx::query_as::<_, Role>(
            r#"
                    SELECT roles.id, roles.name,roles.server_id, roles.can_edit, roles.priority
                    FROM roles
                    WHERE roles.server_id = ?
                    ORDER BY priority DESC
                "#,
        )
        .bind(server)
        .fetch_all(pool)
        .await?)
    }
    pub async fn get_member_roles(member: Uuid, pool: &MySqlPool) -> Result<Vec<Role>, Error> {
        Ok(sqlx::query_as::<_, Role>("SELECT roles.id, roles.name,roles.server_id, roles.can_edit, roles.priority FROM roles LEFT JOIN member_roles ON roles.id = member_roles.role_id LEFT JOIN members ON member_roles.member_id = members.id WHERE members.id = ?  ORDER BY priority DESC").bind(member).fetch_all(pool).await?)
    }
}
