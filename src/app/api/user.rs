use crate::entities::user::{Banner, Profile, User};
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

#[derive(Clone, Copy)]
pub struct UserContext {
    pub banner: Resource<(), Result<Banner, ServerFnError>>,
    pub profile: Resource<(), Result<Profile, ServerFnError>>,
}

pub fn provide_user_context(user_id: Uuid) {
    let banner = create_resource(move || (), move |_| get_user_banner(user_id));
    let profile = create_resource(move || (), move |_| get_user_profile(user_id));

    provide_context(UserContext { banner, profile });
}

pub fn use_user() -> UserContext {
    use_context::<UserContext>()
        .expect("should return the user context, check if you really provided the context")
}

#[server(GetUserProfile)]
pub async fn get_user_profile(user_id: Uuid) -> Result<Profile, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_profile(user_id, &pool).await?)
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

#[server(GetUserBanner)]
pub async fn get_user_banner(user_id: Uuid) -> Result<Banner, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_banner(user_id, &pool).await?)
}
