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
use crate::app::stores::ServersStoreSync;
use crate::app::sync::provide_sync_context;
use crate::app::sync::use_sync;
use crate::entities::member::Member;
use crate::entities::server::Server;
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

#[derive(Debug, Clone, Store)]
pub struct MemberStore {
    #[store(key: Uuid = |member| member.id)]
    members: Vec<Member>,
}

#[component]
pub fn Servers() -> impl IntoView {
    provide_server_context();
    let auth = use_auth().auth;
    let outer_owner = Owner::current().expect("should return the current owner");
    let servers = use_server().get_servers;

    let inner_view = Suspend::new(async move {
        let user = auth.await.unwrap().unwrap();
        outer_owner.with(|| {
            provide_sync_context();
            provide_user_context(user.id);
            provide_user_overview_context();
            provide_server_overview_context();
            provide_channel_context();
            provide_category_context();
            provide_thread_context();
        });
        let servers = servers.await;
        let sync = use_sync();
        servers.map(|servers| {
            let server_store = Store::new(ServersStore { servers });
            if let Some(sync) = sync {
                sync.message_router
                    .on_module_msg("ServersStore", move |sync: ServersStoreSync| match sync {
                        ServersStoreSync::Updated { id } => {
                            debug!("server {id} updated");
                        }
                        ServersStoreSync::Join { server } => {
                            server_store.servers().write().push(server);
                        }
                        ServersStoreSync::Leave { id } => {
                            server_store
                                .servers()
                                .write()
                                .retain(|server| server.id != id);
                        }
                    });
            }
            outer_owner.with(|| {
                provide_context(server_store);
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
