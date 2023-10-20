use leptos::*;
use leptos_meta::{Body, Html};

#[server(ToggleTheme, "/api")]
pub async fn toggle_theme(theme: bool) -> Result<bool, ServerFnError> {
    use tower_cookies::cookie::SameSite;
    use tower_cookies::Cookie;
    use tower_cookies::Cookies;
    let cookies = use_context::<Cookies>().expect("Have tower_cookies::Cookies provided");

    cookies.add(
        Cookie::build("theme", theme.to_string())
            .path("/")
            .same_site(SameSite::Strict)
            .finish(),
    );

    Ok(theme)
}

#[cfg(not(feature = "ssr"))]
fn initial_theme() -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("theme=true")
}

#[cfg(feature = "ssr")]
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

pub type ThemeContext = (Action<ToggleTheme, Result<bool, ServerFnError>>, bool);

pub fn provide_theme_context() {
    let initial = initial_theme();
    let toggle_theme_action = create_server_action::<ToggleTheme>();

    provide_context((toggle_theme_action, initial));
}

pub fn use_theme() -> (Action<ToggleTheme, Result<bool, ServerFnError>>, bool) {
    use_context::<ThemeContext>().expect("theme context")
}

pub fn prefers_theme() -> Signal<bool> {
    let (toggle_theme_action, initial) = use_theme();
    let input = toggle_theme_action.input();
    let value = toggle_theme_action.value();
    Signal::derive(move || match (input.get(), value.get()) {
        (Some(submission), _) => submission.theme,
        (_, Some(Ok(value))) => value,
        _ => initial,
    })
}

#[component]
pub fn Theme() -> impl IntoView {
    let theme = move || {
        if prefers_theme().get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };

    view! {
        <Body class={move || format!("w-full h-screen {}", theme())}/>
        <Html attr:data-theme={move || theme()} />
    }
}
