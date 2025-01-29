mod content;
mod sidebar;
use self::content::ServerSettingsContent;
use self::sidebar::ServerSettingsSideBar;
use crate::app::api::server::get_server;
use crate::app::components::ui::overview::*;
use crate::entities::server::Server;
use leptos::*;
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

#[component]
pub fn ServerOverview(children: Children) -> impl IntoView {
    let open = create_rw_signal(false);
    let server = create_rw_signal(None);
    let settings = create_rw_signal(ServerSettings::Overview);
    provide_context(ServerOverviewContext {
        open,
        server,
        settings,
    });
    view! {
        {children()}
        <OverviewContent open=open class="w-full h-full flex items-center">
            {
                move || {
                    if let Some(server_id) = server.get() {
                        let get_server = create_resource(move || (), move |_| get_server(server_id));
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
                        }.into_view()
                    } else {
                        ().into_view()
                    }
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
        <OverviewTrigger on_click=Signal::derive(move ||  {
            overview_context.server.set(Some(server_id));
            if let Some(select_setting) = select_setting {
                overview_context.settings.set(select_setting)
            }
        }) open=open class=class>
            {children()}
        </OverviewTrigger>
    }
}
