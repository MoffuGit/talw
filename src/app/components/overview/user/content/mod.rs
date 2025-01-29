mod account;
mod appareance;

use leptos::*;

use crate::app::components::overview::user::{UserOverviewContext, UserSettings};

use self::account::AccountSettings;
use self::appareance::AppareanceSettings;
// use self::profiles::ProfilesSettings;

#[component]
pub fn UserSettigsContent() -> impl IntoView {
    let settings = use_context::<UserOverviewContext>()
        .expect("should acces to the user overview context")
        .settings;
    view! {
        <div class="relative h-full w-full bg-base-200 pt-8 pl-8">
            <div class="max-w-[740px] relative w-full h-full overflow-scroll flex flex-col items-start">
                {move || match settings.get() {
                    UserSettings::Account => AccountSettings.into_view(),
                    UserSettings::Appearance => AppareanceSettings.into_view(),
                }}
            </div>
        </div>
    }
}
