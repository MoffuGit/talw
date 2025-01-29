use leptos::*;
mod members;
mod overview;

use self::members::MembersSettings;
use self::overview::OverviewSettings;

use crate::app::components::overview::server::{
    ServerOverviewContext, ServerSettings, ServerSettingsData,
};

#[component]
pub fn ServerSettingsContent() -> impl IntoView {
    let settings = use_context::<ServerOverviewContext>()
        .expect("should acces to the user overview context")
        .settings;
    view! {
        <div class="relative h-full w-full bg-base-200 pt-8 pl-8">
            <div class="max-w-[740px] relative w-full h-full overflow-scroll flex flex-col items-start">
                {
                    move || match settings.get() {
                        ServerSettings::Overview => OverviewSettings.into_view(),
                        ServerSettings::Members => MembersSettings.into_view()
                    }
                }
            </div>
        </div>
    }
}
