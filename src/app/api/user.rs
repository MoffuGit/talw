use crate::entities::user::User;
use cfg_if::cfg_if;
use leptos::*;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::entities::server::Server;
        use super::auth_user;
        use super::pool;
        use super::auth;
    }
}

#[server(GetUserName)]
pub async fn get_user_name(user_id: Uuid) -> Result<String, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_user_name(user_id, &pool).await?)
}

#[server(GetUserImageUrl)]
pub async fn get_user_image_url(user_id: Uuid) -> Result<Option<String>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_image_url(user_id, &pool).await?)
}

#[server(GetUserImageUrlAndName)]
pub async fn get_user_name_and_image_url(
    user_id: Uuid,
) -> Result<(String, Option<String>), ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_name_and_image_url(user_id, &pool).await?)
}

#[server(GetMutualServers)]
pub async fn get_mutual_servers_image_url(
    user_id: Uuid,
) -> Result<Vec<Option<String>>, ServerFnError> {
    let pool = pool()?;
    let user1 = auth_user()?;

    let user2 = User::get(user_id, &pool).await?;

    let res = Server::get_mutual_servers_image_url(user1.id, user2.id, &pool).await;
    Ok(res?)
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;
    Ok(auth.current_user)
}

#[server(GetUserAbout, "/api")]
pub async fn get_user_about(user_id: Uuid) -> Result<Option<String>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_about(user_id, &pool).await?.0)
}
