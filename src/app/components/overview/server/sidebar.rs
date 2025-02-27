use leptos::prelude::*;

use crate::app::components::overview::server::ServerOverviewContext;

use super::ServerSettings;

#[component]
pub fn ServerSettingsSideBar() -> impl IntoView {
    view! {
        <div class="w-1/3 min-w-[240px] h-full flex bg-base-300 justify-end pt-8 pr-1">
            <div class="flex flex-col overflow-scroll min-w-[240px]">
                <div class="font-semibold text-sm mb-0.5">"Server Settings"</div>
                <ServerSettingsSelect select=ServerSettings::Overview />
                <div class="font-semibold text-sm mb-0.5">"Members"</div>
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
            class="w-full h-8 rounded-lg pl-3 mb-0.5 flex items-center hover:bg-base-100 select-none cursor-pointer"
            on:click=move |_| settings.set(select)
        >
            {select.to_string()}
        </div>
    }
}
