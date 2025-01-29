use leptos::*;

use crate::app::components::overview::server::ServerOverviewContext;

use super::ServerSettings;

#[component]
pub fn ServerSettingsSideBar() -> impl IntoView {
    view! {
        <div class="w-3/5 min-w-[218px] h-full flex bg-base-300 justify-end pt-8 pr-1">
            <div class="flex flex-col overflow-scroll">
                <div class="font-semibold mb-0.5">"Server Settings"</div>
                <ServerSettingsSelect select=ServerSettings::Overview />
                <div class="font-semibold mb-0.5">"Members"</div>
                <ServerSettingsSelect select=ServerSettings::Members />
            </div>
        </div>
    }
}

#[component]
pub fn ServerSettingsSelect(select: ServerSettings) -> impl IntoView {
    let settings = use_context::<ServerOverviewContext>()
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
