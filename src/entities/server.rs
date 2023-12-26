use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::channel::Channel;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySqlPool};
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub invite_code: Uuid,
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

    pub async fn check_member_from_invitation(
        user_id: Uuid,
        invitation: Uuid,
        pool: &MySqlPool,
    ) -> Option<Uuid> {
        let id = sqlx::query_as::<_, (Uuid,)>("SELECT id FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.invite_code = ?").bind(user_id).bind(invitation).fetch_one(pool).await.ok()?;
        Some(id.0)
    }

    pub async fn check_member(server_id: Uuid, user_id: Uuid, pool: &MySqlPool) -> Option<Server> {
        let server = sqlx::query_as::<_, Server>("SELECT servers.id, servers.name, servers.invite_code FROM servers LEFT JOIN members ON servers.id = members.server_id WHERE members.user_id = ? AND servers.id = ?").bind(user_id).bind(server_id).fetch_one(pool).await.ok()?;
        Some(server)
    }

    pub async fn get_channels(server_id: Uuid, pool: &MySqlPool) -> Option<Vec<Channel>> {
        let channels = sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE server_id = ?")
            .bind(server_id)
            .fetch_all(pool)
            .await;
        println!("channels db result: {:?}", channels);
        channels.ok()
    }
}
