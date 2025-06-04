pub mod auth;
pub mod category;
pub mod channel;
pub mod member;
pub mod messages;
pub mod server;
pub mod theme;
pub mod thread;
pub mod user;

use cfg_if::cfg_if;
use leptos::prelude::*;

#[cfg(feature = "ssr")]
pub const SERVER_ERROR: &str = "Something go wrong in our servers";

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::sync::connections::ConnectionMessage;
        use crate::sync::SyncRequest;
        use crate::entities::user::AuthSession;
        use crate::entities::user::User;
        use crate::entities::member::Member;
        use crate::entities::server::Server;
        use async_broadcast::Sender;
        use sqlx::MySqlPool;
        use uuid::Uuid;

        pub fn pool() -> Result<MySqlPool, ServerFnError> {
            use_context().ok_or_else(|| ServerFnError::new(SERVER_ERROR))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context()
                .ok_or_else(|| ServerFnError::new(SERVER_ERROR.to_string()))
        }

        pub fn auth_user() -> Result<User, ServerFnError> {
            auth()?.current_user.ok_or_else(|| ServerFnError::new("You arent' authenticated, please log in or sign in"))
        }

        pub fn sync() -> Result<Sender<SyncRequest>, ServerFnError> {
            use_context()
                .ok_or_else(|| ServerFnError::new(SERVER_ERROR.to_string()))
        }

        pub fn connection() -> Result<Sender<ConnectionMessage>, ServerFnError> {
            use_context()
                .ok_or_else(|| ServerFnError::new(SERVER_ERROR.to_string()))
        }

        pub async fn user_can_edit (server: Uuid, user: Uuid, pool: &MySqlPool) -> Result<bool, ServerFnError> {
            if Server::get_server_owner(server, pool)
                .await?
                == user
            {
                return Ok(true);
            };

            if Member::member_can_edit(user, pool).await? {
                return Ok(true);

            }
                 Ok(false)
        }

    }
}
