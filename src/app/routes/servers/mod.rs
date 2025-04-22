pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;
use crate::app::api::auth::use_auth;
use crate::app::api::category::provide_category_context;
use crate::app::api::channel::provide_channel_context;
use crate::app::api::server::provide_server_context;
use crate::app::api::server::use_server;
use crate::app::api::thread::provide_thread_context;
use crate::app::api::user::provide_user_context;
use crate::app::components::navigation::sidebar::SideBar;
use crate::app::components::overview::server::provide_server_overview_context;
use crate::app::components::overview::server::ServerOverview;
use crate::app::components::overview::user::provide_user_overview_context;
use crate::app::components::overview::user::UserOverview;
use crate::entities::server::Server;
use crate::ws::client::provide_ws_context;
use crate::ws::client::use_ws;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use log::debug;
use reactive_stores::Store;
use uuid::Uuid;

#[derive(Debug, Clone, Store)]
pub struct ServersStore {
    #[store(key: Uuid = |server| server.id)]
    servers: Vec<Server>,
}

#[component]
pub fn Servers() -> impl IntoView {
    provide_server_context();
    let auth = use_auth().auth;
    let outer_owner = Owner::current().expect("should return teh current owner");
    let servers = use_server().get_servers;

    let inner_view = Suspend::new(async move {
        let user = auth.await.unwrap().unwrap();
        servers.await.map(|servers| {
            let server_store = Store::new(ServersStore {
                servers: servers.clone(),
            });
            outer_owner.with(|| {
                provide_user_context(user.id);
                provide_user_overview_context();
                provide_server_overview_context();
                provide_ws_context();
                provide_channel_context();
                provide_category_context();
                provide_thread_context();
                provide_context(server_store);
            });

            //[1] susbcribe and create the broadcast receiver
            //[2] let other users know that i connected
            //[3] when user left, join or create a server:
            //      uppdate the servers store
            //      update the broadcast receiver list
            //[4] when user edit the server, update the server store
            //[5] when ws receive a msg about a server:
            //      update the servers store, server store or/and broadcast receiver list

            //the server ws should seend an AppMessage
            //if is ServerMessage
            //send it to server broadcast
            //if not send it to app broadcast
            //now i can handler all msg from the server
            //move this to his own handler or function
            //should depend of the ServerStore
            //if ServerStore grow, sub to news,
            //if shrink, then unsub
            //can grow or shrink from AppMessages
            //can update from ServerMessages
            let ws = use_ws();
            Effect::new(move |_| {
                let servers = server_store
                    .servers()
                    .get()
                    .iter()
                    .map(|server| server.id)
                    .collect::<Vec<_>>();
                ws.sync_channels(servers.clone(), user.id);
                for server in servers {
                    use_ws().on_server_msg(server);
                }
            });

            view! {
                <UserOverview />
                <ServerOverview />
                <div class="h-full w-full relative z-40 flex">
                    <div class="flex w-12 h-full z-30 fixed inset-y-0">
                        <SideBar />
                    </div>
                    <div class="h-full w-full relative overflow-hidden md:pl-12">
                        <Outlet />
                    </div>
                </div>
            }
        })
    });

    view! { <Transition>{inner_view}</Transition> }
}
