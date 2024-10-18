use std::str::FromStr;

use crate::app::api::member::get_member;
use crate::app::api::member::member_can_edit;
use crate::app::api::server::get_server;
use crate::app::api::server::use_server;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use crate::entities::member::Member;
use leptos::*;
use leptos_router::use_params_map;
use leptos_router::Outlet;
use leptos_router::Redirect;
use uuid::Uuid;

use crate::entities::server::Server as ServerEntitie;

#[derive(Clone)]
pub struct CurrentServerContext {
    pub server: ServerEntitie,
    pub member_can_edit: bool,
    pub member: Member,
}

pub fn use_current_server_context() -> CurrentServerContext {
    use_context::<CurrentServerContext>().expect("Should acces to current server context")
}

#[allow(non_snake_case)]
#[component]
pub fn Server() -> impl IntoView {
    view! {
        <div class="h-full w-full relative z-40 flex">
            {
                move || {
                    match use_params_map().with(|params| Uuid::from_str(params.get("id").unwrap())) {
                        Err(_) => view!{<Redirect path="/servers/me"/>}.into_view(),
                        Ok(server_id) => {
                            let leave_server = use_server().leave_server;
                            let server = create_resource(
                                move || {
                                    (
                                        leave_server.version().get(),
                                    )
                                },
                                move |_| get_server(server_id),
                            );

                            let member_can_edit = create_resource(
                                move || {
                                    (
                                        leave_server.version().get(),
                                    )
                                },
                                move |_| member_can_edit(server_id),
                            );

                            let member = create_resource(move || (
                                leave_server.version().get()
                            ), move |_| get_member(server_id));

                            view!{
                                <Transition fallback=move || ()>
                                    {
                                        move || {
                                            match (server.get(),  member_can_edit.get(), member.get()) {
                                                (Some(Ok(server)), Some(Ok(member_can_edit)), Some(Ok(member))) => {
                                                    provide_context(CurrentServerContext {
                                                        server,
                                                        member_can_edit,
                                                        member
                                                    });
                                                    view! {
                                                        <ServerSideBar />
                                                        <div class="h-full grow relative overflow-hidden z-30">
                                                            <Outlet/>
                                                        </div>
                                                    }.into_view()
                                                },
                                                (None,_, _) | (_, None, _) | (_, _, None) => {
                                                    view! {
                                                        <div class="flex w-[240px] h-full fixed inset-y-0 bg-base-200 z-40">
                                                        </div>
                                                        <div class="h-full relative overflow-hidden md:pl-[240px] z-30">
                                                        </div>
                                                    }.into_view()
                                                },
                                                _ => view!{<Redirect path="/servers/me"/>}.into_view()
                                            }
                                        }
                                    }
                                </Transition>
                            }.into_view()
                        },
                    }
                }
            }
        </div>
    }
}
