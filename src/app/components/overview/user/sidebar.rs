use leptos::*;

use crate::app::components::overview::user::UserOverviewContext;

use super::UserSettings;

#[component]
pub fn UserSettingsSideBar() -> impl IntoView {
    view! {
        <div class="w-3/5 min-w-[218px] h-full flex bg-base-200 justify-end pt-8 pr-1">
            <div class="flex flex-col overflow-scroll">
                <div class="font-bold mb-0.5 text-base-content/70">"User Settings"</div>
                <UserSettingsSelect select=UserSettings::Account />
                <div class="font-bold mb-0.5 text-base-content/70">"App Settings"</div>
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
            class="w-48 h-8 rounded pl-3 mb-0.5 flex items-center hover:bg-base-content/10 text-base-content/70"
            on:click=move |_| settings.set(select)
        >
            {select.to_string()}
        </div>
    }
}
