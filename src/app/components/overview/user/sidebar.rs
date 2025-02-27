use leptos::prelude::*;

use crate::app::components::overview::user::UserOverviewContext;

use super::UserSettings;

#[component]
pub fn UserSettingsSideBar() -> impl IntoView {
    view! {
        <div class="w-1/3 min-w-[218px] h-full flex bg-base-300 justify-end pt-8 pr-1">
            <div class="flex flex-col overflow-scroll">
                <div class="font-semibold mb-0.5">"User Settings"</div>
                <UserSettingsSelect select=UserSettings::Account />
                <div class="font-semibold mb-0.5">"App Settings"</div>
                <UserSettingsSelect select=UserSettings::Appearance />
            </div>
        </div>
    }
}

#[component]
pub fn UserSettingsSelect(select: UserSettings) -> impl IntoView {
    let settings = use_context::<UserOverviewContext>()
        .expect("should acces to the user overview context")
        .settings;
    view! {
        <div
            class="w-48 h-8 rounded-lg pl-3 mb-0.5 flex items-center hover:bg-base-content/10 select-none cursor-pointer"
            on:click=move |_| settings.set(select)
        >
            {select.to_string()}
        </div>
    }
}
