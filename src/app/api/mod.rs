pub mod auth;
pub mod category;
pub mod channel;
pub mod member;
pub mod server;
pub mod theme;
pub mod thread;

use cfg_if::cfg_if;
use leptos::*;

#[cfg(feature = "ssr")]
pub const SERVER_ERROR: &str = "Something go wrong in our servers";

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::entities::user::AuthSession;
        use crate::entities::user::User;
        use crate::entities::member::Member;
        use crate::entities::server::Server;
        use sqlx::MySqlPool;
        use uuid::Uuid;

        pub fn pool() -> Result<MySqlPool, ServerFnError> {
            use_context::<MySqlPool>().ok_or_else(|| ServerFnError::new(SERVER_ERROR))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::new(SERVER_ERROR.to_string()))
        }

        pub fn auth_user() -> Result<User, ServerFnError> {
            auth()?.current_user.ok_or_else(|| ServerFnError::new("You arent' authenticated, please log in or sign in"))
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
