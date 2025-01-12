use crate::app::api::theme::use_theme;
use icondata::Icon;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[derive(Clone)]
pub struct ThemeIcons {
    pub dark: Icon,
    pub light: Icon,
    pub class: &'static str,
}

#[component]
pub fn Toggle_Theme(
    class: &'static str,
    #[prop(optional)] icons: Option<ThemeIcons>,
) -> impl IntoView {
    let theme_context = use_theme();
    let toggle_theme = theme_context.toggle_theme;
    let prefers_theme = theme_context.prefers_theme;

    view! {
        <ActionForm action=toggle_theme class=class>
            <input type="hidden" name="theme" value=move || (!prefers_theme.get()).to_string() />
            <button type="submit" class="w-full h-full flex items-center justify-center">
                {icons
                    .clone()
                    .map(|icons| move || match prefers_theme.get() {
                        true => view! { <Icon icon=icons.dark class=icons.class /> },
                        false => view! { <Icon icon=icons.light class=icons.class /> },
                    })}
            </button>
        </ActionForm>
    }
}
