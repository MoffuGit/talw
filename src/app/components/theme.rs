use std::time::Duration;

use crate::app::api::theme::use_theme;
use crate::app::components::ui::tool_tip::*;
//use icondata:Icon;
use leptos::prelude::*;
//use leptos_icons::*;

// #[derive(Clone)]
// pub struct ThemeIcons {
//     pub dark: Icon,
//     pub light: Icon,
//     pub class: &'static str,
//

#[component]
pub fn Toggle_Theme(
    class: &'static str,
    // #[prop(optional)] icons: Option<ThemeIcons>,
) -> impl IntoView {
    let theme_context = use_theme();
    let toggle_theme = theme_context.toggle_theme;
    let prefers_theme = theme_context.prefers_theme;

    view! {
        <ActionForm action=toggle_theme>
            <input type="hidden" name="theme" value=move || (!prefers_theme.get()).to_string() />
            <button type="submit" class="w-full h-full flex items-center justify-center">
                // {icons
                //     .clone()
                //     .map(|icons| move || match prefers_theme.get() {
                //         true => view! { <Icon icon=icons.dark /> },
                //         false => view! { <Icon icon=icons.light /> },
                //     })}
            </button>
        </ActionForm>
    }
}

#[component]
pub fn SelectTheme(dark_mode: bool) -> impl IntoView {
    let theme_context = use_theme();
    let tip = if dark_mode { "Dark" } else { "Light" };
    view! {
        <ActionForm action=theme_context.toggle_theme>
            // class=move || {
            // if theme_context.prefers_theme.get() == dark_mode {
            // "rounded-full w-16 h-16 border-2 border-primary"
            // } else {
            // "rounded-full w-16 h-16 "
            // }
            // }
            <input type="hidden" name="theme" value=dark_mode.to_string() />
            <TooltipProvider delay_duration=Duration::new(0, 0)>
                <TooltipTrigger close_on_click=true class="w-full h-full">
                    <button
                        type="submit"
                        class=format!(
                            "w-full h-full flex items-center rounded-full justify-center {}",
                            { if dark_mode { "bg-[#2c2d31]" } else { "bg-[#f6f6f8]" } },
                        )
                    />
                </TooltipTrigger>
                <TooltipContent
                    tip=tip
                    arrow=true
                    class="z-[1000] rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                    tooltip_side=ToolTipSide::Top
                />
            </TooltipProvider>
        </ActionForm>
    }
}
