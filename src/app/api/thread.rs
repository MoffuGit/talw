use core::f64;

use cfg_if::cfg_if;
use leptos::*;
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
