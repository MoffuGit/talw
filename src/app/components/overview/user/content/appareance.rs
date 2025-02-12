use leptos::prelude::*;

use crate::app::components::theme::SelectTheme;

#[component]
pub fn AppareanceSettings() -> impl IntoView {
    view! {
        <div class="font-bold text-xl mb-2">"Theme"</div>
        <div class="relative flex items-center space-x-5">
            <SelectTheme dark_mode=true />
            <SelectTheme dark_mode=false />
        </div>
        <div>"Advance setting"</div>
    }
}
