mod content;
mod sidebar;
use std::fmt::Display;

use crate::app::components::ui::overview::*;
use leptos::prelude::*;

use self::content::UserSettigsContent;
use self::sidebar::UserSettingsSideBar;

#[derive(Clone)]
pub struct UserOverviewContext {
    open: RwSignal<bool>,
    settings: RwSignal<UserSettings>,
}

#[derive(Copy, Clone)]
pub enum UserSettings {
    Account,
    Appearance,
}

impl Display for UserSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSettings::Account => write!(f, "Account"),
            UserSettings::Appearance => write!(f, "Appearance"),
        }
    }
}

pub fn provide_user_overview_context() {
    let open = RwSignal::new(false);
    let settings = RwSignal::new(UserSettings::Account);
    provide_context(UserOverviewContext { open, settings });
}

pub fn use_user_overview() -> UserOverviewContext {
    use_context::<UserOverviewContext>().expect("should acces the user overview context")
}

#[component]
pub fn UserOverview() -> impl IntoView {
    let user_overview = use_user_overview();
    view! {
        <OverviewContent
            on_close=Signal::derive(move || {
                user_overview.settings.set(UserSettings::Account);
            })
            open=user_overview.open
            class="w-full h-full flex items-center"
        >
            <UserSettingsSideBar />
            <UserSettigsContent />
        </OverviewContent>
    }
}

#[component]
pub fn UserOverviewTrigger(
    children: Children,
    class: &'static str,
    #[prop(optional)] select_setting: Option<UserSettings>,
) -> impl IntoView {
    let context =
        use_context::<UserOverviewContext>().expect("should acces to the user overview context");
    let open = context.open;
    let settings = context.settings;
    view! {
        <OverviewTrigger
            on_click=Signal::derive(move || {
                if let Some(select_setting) = select_setting {
                    settings.set(select_setting)
                }
            })
            open=open
            class=class
        >
            {children()}
        </OverviewTrigger>
    }
}
