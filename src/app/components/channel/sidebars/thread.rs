use std::collections::HashMap;

use crate::app::api::member::{get_members, get_thread_filtered_members};
use crate::app::api::server::get_server_roles;
use crate::app::components::channel::sidebars::group::{Group, Groups, Pagination};
use crate::app::components::channel::sidebars::profile::Profile;
use crate::app::components::channel::sidebars::server::{RoleStore, RoleStoreStoreFields};
use crate::app::components::channel::sidebars::SideBarContext;
use crate::entities::member::{Member, Status};
use crate::entities::role::RoleStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
use leptos::prelude::*;
use reactive_stores::Store;
//use leptos_icons::Icon;
use uuid::Uuid;

#[component]
pub fn ThreadMemberSideBar(server_id: Uuid, thread_id: Uuid) -> impl IntoView {
    let open = use_context::<SideBarContext>()
        .expect("should acces to the SideBarContext")
        .0;
    let roles = Resource::new(|| (), move |_| get_server_roles(server_id));
    view! {
        <div
            class="h-full shrink-0 bg-base-300 flex flex-col items-stretch justify-between ease-linear duration-200 transition-[width]"
            style=move || if open.get() { "width: 240px" } else { "width: 0px" }
        >
            <Transition>
                <div class="flex flex-col overflow-y-scroll overflow-x-hidden items-stretch">
                    {
                        Suspend::new(async move {
                            let roles = roles.await;
                            roles.map(|roles| {
                                let roles = Store::new(RoleStore{ roles });
                                let members: RwSignal<HashMap<Uuid, Member>> = RwSignal::new(HashMap::new());
                                use_ws().on_server_msg(server_id, move |msg| {
                                    match msg {
                                        Message::MemberConnected { member } => {
                                            members.update(|hash| {
                                                if let Some(exist) = hash.get_mut(&member.user_id) {
                                                    *exist = member;
                                                }
                                            });
                                        },
                                        Message::MemberDisconnected { user_id } => {
                                            members.update(|hash| {
                                                if let Some(member) = hash.get_mut(&user_id) {
                                                    member.status = Status::OFFLINE
                                                }
                                            });
                                        },
                                        Message::MemberLeftServer { user_id } => {
                                            members.update(|hash| {
                                                hash.remove(&user_id);
                                            });
                                        },
                                        Message::MemberJoinThread { member, thread_id: id } => {
                                            if id == thread_id {
                                                members.update(|hash| {
                                                    hash.insert(member.user_id, member);
                                                });
                                            }
                                        },
                                        Message::MemberLeaveThread { user_id, thread_id: id} => {
                                            if id == thread_id {
                                                members.update(|hash| {
                                                    hash.remove(&user_id);
                                                });

                                            }
                                        },
                                        Message::MemberUpdated { member_id, name, image_url } => {
                                            members.update(|hash| {
                                                if let Some(member) = hash.get_mut(&member_id) {
                                                    if let Some(name) = name {
                                                        member.name = name;
                                                    }
                                                    if let Some(image_url) = image_url {
                                                        member.image_url = Some(image_url);
                                                    }
                                                }
                                            });
                                        },
                                        _ => {}
                                    }

                                });
                                view!{
                                    <For
                                        each=move || roles.roles()
                                        key=|role| role.id().get()
                                        let:role
                                    >
                                    <Pagination members=members pagination=Resource::new(move || (), move |_| get_thread_filtered_members(thread_id, Some(role.id().get()), Some(Status::ONLINE)))>
                                            <Group members=members name=role.name() group=Groups::Online(Some(role.id().get())) />
                                        </Pagination >
                                    </For>
                                    <Pagination members=members pagination=Resource::new(move || (), move |_| get_thread_filtered_members(thread_id, None, Some(Status::ONLINE)))>
                                        <Group members=members name="Online" group=Groups::Online(None)/>
                                    </Pagination>
                                    <Pagination members=members pagination=Resource::new(move || (), move |_| get_thread_filtered_members(thread_id, None, Some(Status::OFFLINE)))>
                                        <Group members=members name="Offline" group=Groups::Offline/>
                                    </Pagination >
                                }
                            })
                        })
                    }
                </div>
            </Transition>
            <Profile />
        </div>
    }
}
