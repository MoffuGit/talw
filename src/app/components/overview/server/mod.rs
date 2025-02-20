mod content;
mod sidebar;
use self::content::ServerSettingsContent;
use self::sidebar::ServerSettingsSideBar;
use crate::app::api::server::get_server;
use crate::app::components::ui::overview::*;
use crate::entities::server::Server;
use leptos::prelude::*;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone)]
pub struct ServerOverviewContext {
    open: RwSignal<bool>,
    server: RwSignal<Option<Uuid>>,
    settings: RwSignal<ServerSettings>,
}

#[derive(Copy, Clone)]
pub enum ServerSettings {
    Overview,
    Members,
}

impl Display for ServerSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerSettings::Overview => write!(f, "Overview"),
            ServerSettings::Members => write!(f, "Members"),
        }
    }
}

#[derive(Clone)]
pub struct ServerSettingsData {
    server: Server,
}

pub fn provide_server_overview_context() {
    let open = RwSignal::new(false);
    let server = RwSignal::new(None);
    let settings = RwSignal::new(ServerSettings::Overview);
    provide_context(ServerOverviewContext {
        open,
        server,
        settings,
    });
}

pub fn use_server_overview() -> ServerOverviewContext {
    use_context::<ServerOverviewContext>().expect("should get the server overview context")
}

#[component]
pub fn ServerOverview() -> impl IntoView {
    let ServerOverviewContext {
        open,
        server,
        settings,
    } = use_server_overview();
    view! {
        <OverviewContent open=open class="w-full h-full flex items-center">
            {
                move || {
                    server.get().map(|server|  {
                        let get_server = Resource::new(move || (), move |_| get_server(server));
                        view!{
                            <Transition>
                                {
                                    move || {
                                        get_server.and_then(|server| {
                                            provide_context(ServerSettingsData { server:server.clone() });
                                            view!{
                                                <ServerSettingsSideBar/>
                                                <ServerSettingsContent/>
                                            }
                                        })
                                    }
                                }
                            </Transition>
                        }
                    })
                }
            }
        </OverviewContent>
    }
}

#[component]
pub fn ServerOverviewTrigger(
    children: Children,
    class: &'static str,
    server_id: Uuid,
    #[prop(optional)] select_setting: Option<ServerSettings>,
) -> impl IntoView {
    let overview_context = use_context::<ServerOverviewContext>()
        .expect("should acces to the server overview context");
    let open = overview_context.open;
    view! {
        <OverviewTrigger
            on_click=Signal::derive(move ||  {
                overview_context.server.set(Some(server_id));
                if let Some(select_setting) = select_setting {
                    overview_context.settings.set(select_setting)
                }
            })
            open=open class=class>
            {children()}
        </OverviewTrigger>
    }
}
