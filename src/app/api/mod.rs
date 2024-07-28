pub mod auth;
pub mod category;
pub mod channel;
pub mod server;
pub mod theme;
pub mod thread;

use crate::entities::member::Member;
use crate::entities::server::Server;
use cfg_if::cfg_if;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use crate::entities::user::AuthSession;
        use crate::entities::user::User;
        use sqlx::MySqlPool;

        pub fn pool() -> Result<MySqlPool, ServerFnError> {
            use_context::<MySqlPool>().ok_or_else(|| ServerFnError::new("Pool missing.".to_string()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::new("Auth session missing.".to_string()))
        }

        pub fn auth_user() -> Result<User, ServerFnError> {
            auth()?.current_user.ok_or_else(|| ServerFnError::new("cant auth user".to_string()))
        }

        pub async fn auth_member(server_id: Uuid) -> Result<Member, ServerFnError> {
            let pool = pool()?;
            let user = auth_user()?;
            Server::get_member(server_id, user.id, &pool).await.ok_or_else(|| ServerFnError::new("cant get the member"))
        }
    }
}
