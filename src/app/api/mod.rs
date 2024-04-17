pub mod auth;
pub mod server;
pub mod theme;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use bcrypt::verify;
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
    }
}
