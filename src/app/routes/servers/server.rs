use std::str::FromStr;

use crate::app::api::member::get_member;
use crate::app::api::member::get_members;
use crate::app::api::member::member_can_edit;
use crate::app::api::server::get_server_roles;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use crate::app::components::navigation::server::sidebar::ServerSideBarContext;
use crate::app::routes::servers::MemberStore;
use crate::app::routes::servers::ServersStore;
use crate::app::routes::servers::ServersStoreStoreFields;
use crate::entities::member::Member;
use crate::entities::role::Role;
use crate::entities::server::Server as ServerEnt;
use crate::entities::server::ServerStoreFields;
use futures::try_join;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_params_map;
use reactive_stores::Field;
use reactive_stores::Store;
use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentServerContext {
    pub server: Field<ServerEnt>,
    pub member_can_edit: bool,
    pub member: Store<Member>,
    pub members: Store<MemberStore>,
    pub roles: Store<RoleStore>,
}

pub fn use_current_server_context() -> CurrentServerContext {
    use_context::<CurrentServerContext>().expect("Should acces to current server context")
}

#[derive(Store, Debug)]
pub struct RoleStore {
    #[store(key: Uuid = |role| role.id)]
    pub roles: Vec<Role>,
}

#[component]
pub fn Server() -> impl IntoView {
    let params_map = use_params_map();
    let server_id = move || {
        params_map.with(|map| {
            map.get("id")
                .and_then(|id| Uuid::from_str(&id).ok())
                .expect("should get the server id from the params")
        })
    };
    let servers = use_context::<Store<ServersStore>>().expect("Should acces the servers store");

    let open = RwSignal::new(true);
    let outer_owner = Owner::current().unwrap();
    view! {
        <div class="h-full w-full relative z-40 flex">
            {
                move || {
                    let outer_owner = outer_owner.clone();
                    servers.servers().into_iter().find(|server| server.id().get() == server_id()).map(|server| {
                        let server_data = Resource::new(move || server.id().get(), move |server_id| async move {
                            let member = get_member(server_id);
                            let can_edit = member_can_edit(server_id);
                            let members = get_members(server_id);
                            let roles = get_server_roles(server_id);
                            try_join!(member, can_edit, members, roles)
                        });


                        let inner_view = Suspend::new(async move {
                            server_data
                                .await
                                .map(|(member, can_edit, members, roles)| {
                                    let member = Store::new(member);
                                    let members = Store::new(MemberStore { members });
                                    let roles = Store::new(RoleStore { roles });

                                    outer_owner.with(|| {
                                        provide_context(ServerSideBarContext { open });
                                        provide_context(CurrentServerContext {
                                            server: server.into(),
                                            member_can_edit: can_edit,
                                            member,
                                            members,
                                            roles,
                                        })
                                    });
                                    view! {
                                        <ServerSideBar />
                                        <div class="h-full grow relative overflow-hidden z-30">
                                            <Outlet />
                                        </div>
                                    }
                                })
                        });
                        view!{
                            <Transition>{inner_view}</Transition>
                        }
                    })
                }
            }
        </div>
    }
}

// ws.on_server_msg(server.id().get(), move |msg| match msg {
//     Message::MemberUpdated {
//         member_id,
//         name,
//         image_url,
//     } => {
//         if member.id().get() == member_id {
//             if let Some(name) = name.clone() {
//                 *member.name().write() = name
//             }
//             if let Some(image_url) = image_url.clone() {
//                 *member.image_url().write() = Some(image_url);
//             }
//         }
//         members.members().update(|members| {
//             if let Some(position) =
//                 members.iter().position(|member| member.id == member_id)
//             {
//                 if let Some(member) = members.get_mut(position) {
//                     if let Some(name) = name {
//                         member.name = name
//                     }
//                     if let Some(image_url) = image_url {
//                         member.image_url = Some(image_url);
//                     }
//                 }
//             }
//         });
//     }
//     Message::ServerUpdated { name, image } => {
//         if let Some(name) = name {
//             *server.name().write() = name;
//         }
//         if let Some(image) = image {
//             *server.image_url().write() = Some(image);
//         }
//     }
//     Message::MemberConnected { member_id } => {
//         members.members().update(|members| {
//             if let Some(position) =
//                 members.iter().position(|member| member.id == member_id)
//             {
//                 if let Some(member) = members.get_mut(position) {
//                     member.status = Status::ONLINE;
//                 }
//             }
//         });
//     }
//     Message::MemberDisconnected { member_id } => {
//         members.members().update(|members| {
//             if let Some(position) =
//                 members.iter().position(|member| member.id == member_id)
//             {
//                 if let Some(member) = members.get_mut(position) {
//                     member.status = Status::OFFLINE;
//                 }
//             }
//         });
//     }
//     Message::MemberJoinedServer { member } => {
//         members.members().write().push(member);
//     }
//     Message::MemberLeftServer { member_id } => {
//         if let Some(position) = members
//             .members()
//             .read()
//             .iter()
//             .position(|member| member.id == member_id)
//         {
//             members.members().write().swap_remove(position);
//         }
//     }
//     _ => {}
// });
