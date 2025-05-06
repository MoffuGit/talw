pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;
use std::collections::HashMap;

use crate::app::api::auth::use_auth;
use crate::app::api::category::provide_category_context;
use crate::app::api::channel::provide_channel_context;
use crate::app::api::member::get_user_members;
use crate::app::api::member::get_user_members_and_connect;
use crate::app::api::member::update_member_status;
use crate::app::api::server::provide_server_context;
use crate::app::api::server::use_server;
use crate::app::api::thread::provide_thread_context;
use crate::app::api::user::provide_user_context;
use crate::app::components::navigation::sidebar::SideBar;
use crate::app::components::overview::server::provide_server_overview_context;
use crate::app::components::overview::server::ServerOverview;
use crate::app::components::overview::user::provide_user_overview_context;
use crate::app::components::overview::user::UserOverview;
use crate::entities::member::Member;
use crate::entities::member::Status;
use crate::entities::server::Server;
use crate::messages::ClientMessage;
use crate::ws::client::provide_ws_context;
use crate::ws::client::use_ws;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::Outlet;
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
    let outer_owner = Owner::current().expect("should return teh current owner");
    let servers = use_server().get_servers;
    let members = Resource::new(move || (), move |_| get_user_members_and_connect());

    let inner_view = Suspend::new(async move {
        let user = auth.await.unwrap().unwrap();
        outer_owner.with(|| {
            provide_user_context(user.id);
            provide_user_overview_context();
            provide_server_overview_context();
            provide_ws_context();
            provide_channel_context();
            provide_category_context();
            provide_thread_context();
        });
        let members = members.await;
        let servers = servers.await;
        members.map(|members| {
            servers.map(|servers| {
                let server_store = Store::new(ServersStore { servers });
                outer_owner.with(|| {
                    provide_context(server_store);
                });
                let members = Store::new(MemberStore { members });

                let ws = use_ws();

                ws.on_app_msg(move |msg| match msg {
                    ClientMessage::LeavedServer { server_id, .. }
                    | ClientMessage::ServerDeleted { server_id } => {
                        members.update(|store| {
                            store.members.retain(|member| member.server_id != server_id)
                        });
                        server_store
                            .update(|store| store.servers.retain(|server| server.id != server_id));
                    }
                    ClientMessage::JoinedToServer { server, member, .. } => {
                        spawn_local(async move {
                            if update_member_status(member.id, Status::ONLINE)
                                .await
                                .is_ok()
                            {
                                let mut member = member;
                                member.status = Status::ONLINE;
                                members.update(|store| store.members.push(member));
                                server_store.update(|store| store.servers.push(server));
                            }
                        });
                    }
                    _ => {}
                });
                Effect::new_isomorphic(move |_| {
                    let servers = server_store
                        .servers()
                        .get()
                        .iter()
                        .map(|server| server.id)
                        .collect::<Vec<_>>();
                    let members = members
                        .members()
                        .get_untracked()
                        .iter()
                        .map(|member| (member.server_id, member.clone()))
                        .collect::<HashMap<_, _>>();
                    ws.sync_channels(servers, members, user.id);
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
        })
    });

    view! { <Transition>{inner_view}</Transition> }
}
