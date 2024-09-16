use uuid::Uuid;

use crate::entities::thread::Thread;
use cfg_if::cfg_if;
use core::f64;
use leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ThreadContext {
    pub create_thread: Action<CreateThread, Result<(), ServerFnError>>,
}

pub fn use_thread() -> ThreadContext {
    use_context::<ThreadContext>().expect("have thread context")
}

pub fn provide_thread_context() {
    let create_thread = create_server_action::<CreateThread>();

    provide_context(ThreadContext { create_thread })
}

#[server(CreateThread)]
pub async fn create_thread(
    channel_id: Uuid,
    server_id: Uuid,
    name: String,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    let id = Thread::create(name, channel_id, &pool).await?;
    leptos_axum::redirect(&format!(
        "/servers/{}/{}/{}",
        server_id.simple(),
        channel_id.simple(),
        id.simple()
    ));
    Ok(())
}

#[server(GetThread)]
pub async fn get_thread(thread_id: Uuid) -> Result<Thread, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    let res = Thread::get_from_id(thread_id, &pool).await;
    log::info!("{:?}", res);
    Ok(res?)
}

#[server(ToggleThreadWidth, "/api")]
pub async fn toggle_thread_width(width: f64) -> Result<f64, ServerFnError> {
    use tower_cookies::cookie::SameSite;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    let cookies = use_context::<Cookies>().expect("Have tower_cookies::Cookies provided");

    cookies.add(
        Cookie::build(("thread_width", width.to_string()))
            .path("/")
            .same_site(SameSite::Strict)
            .into(),
    );

    Ok(width)
}
cfg_if! {
    if  #[cfg(not(feature = "ssr"))] {
        pub fn initial_width() -> f64 {
            use wasm_bindgen::JsCast;

            let doc = document().unchecked_into::<web_sys::HtmlDocument>();
            let cookie = doc.cookie().unwrap_or_default();
            cookie
                .split(';')
                .find(|cookie| cookie.contains("thread_width"))
                .and_then(|cookie| {
                    cookie
                        .split('=')
                        .last()
                        .and_then(|width| width.parse::<f64>().ok())
                })
                .unwrap_or(400.0)
        }
    } else {
        pub fn initial_width() -> f64 {
            use tower_cookies::Cookies;

            use_context::<Cookies>()
                .map(|cookies| {
                    cookies
                        .get("thread_width")
                        .and_then(|width| width.value().parse::<f64>().ok()).unwrap_or(400.0)
                })
            .unwrap_or(400.0)
        }
    }
}
