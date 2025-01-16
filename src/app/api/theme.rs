use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::{Body, Html, Script};

use self::html::Head;

#[server(ToggleTheme, "/api")]
pub async fn toggle_theme(theme: bool) -> Result<bool, ServerFnError> {
    use tower_cookies::cookie::SameSite;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    let cookies = use_context::<Cookies>().expect("Have tower_cookies::Cookies provided");

    cookies.add(
        Cookie::build(("theme", theme.to_string()))
            .path("/")
            .same_site(SameSite::Strict)
            .into(),
    );

    Ok(theme)
}

cfg_if! {
    if  #[cfg(not(feature = "ssr"))] {

fn initial_theme() -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("theme=true")
}
    } else {

fn initial_theme() -> bool {
    use tower_cookies::Cookies;

    use_context::<Cookies>()
        .map(|cookies| {
            cookies
                .get("theme")
                .is_some_and(|theme| theme.value() == "true")
        })
        .unwrap_or(false)
}
    }
}

// #[cfg(feature = "ssr")]

#[derive(Clone)]
pub struct ThemeContext {
    pub toggle_theme: Action<ToggleTheme, Result<bool, ServerFnError>>,
    pub prefers_theme: Signal<bool>,
}

pub fn provide_theme_context() {
    let initial = initial_theme();
    let toggle_theme = create_server_action::<ToggleTheme>();
    let input = toggle_theme.input();
    let value = toggle_theme.value();
    let prefers_theme = Signal::derive(move || match (input.get(), value.get()) {
        (Some(submission), _) => submission.theme,
        (_, Some(Ok(value))) => value,
        _ => initial,
    });

    provide_context(ThemeContext {
        toggle_theme,
        prefers_theme,
    });
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("theme context")
}

#[allow(non_snake_case)]
#[component]
pub fn Theme() -> impl IntoView {
    let theme_context = use_theme();
    let prefers_theme = theme_context.prefers_theme;
    let theme = move || {
        if prefers_theme.get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    view! {
        <Html attr:data-theme=theme />
        <head>
            <link rel="preconnect" href="https://fonts.googleapis.com"/>
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
            <link href="https://fonts.googleapis.com/css2?family=Geist:wght@100..900&display=swap" rel="stylesheet"/>
        </head>
        <Body class=move || format!("w-full h-screen font-geist {}", theme()) />
    }
}
