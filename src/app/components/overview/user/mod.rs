mod content;
mod sidebar;
use std::fmt::Display;

use crate::app::components::ui::overview::*;
use leptos::prelude::*;

use self::content::UserSettigsContent;
use self::sidebar::UserSettingsSideBar;

#[derive(Clone)]
struct UserOverviewContext {
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

#[component]
pub fn UserOverview(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    let settings = RwSignal::new(UserSettings::Account);
    provide_context(UserOverviewContext { open, settings });
    view! {
        {children()}
        <OverviewContent
            on_close=Signal::derive(move || {
                settings.set(UserSettings::Account);
            })
            open=open
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
