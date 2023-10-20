use leptos::{server_fn::server, *};

use crate::user::Server;

#[cfg(feature = "ssr")]
use super::auth::pool;
#[cfg(feature = "ssr")]
use crate::app::auth::auth;

#[server(GetUserServers, "/api")]
pub async fn get_user_servers() -> Result<Vec<Server>, ServerFnError> {
    let auth = auth()?;
    let pool = pool()?;

    return match auth.current_user {
        Some(user) => user
            .get_servers(&pool)
            .await
            .ok_or_else(|| ServerFnError::ServerError("cant get servers".to_string())),
        None => Err(ServerFnError::ServerError("cant auth user".to_string())),
    };
}

// #[server(CreateServer, "api")]
// pub async fn create_server(name: String) -> Result<(), ServerFnError> {
//     let pool = pool()?;
//     let auth = auth()?;
//
//     let server = Server::create(name, &pool);
//     return match auth.current_user {};
// }
