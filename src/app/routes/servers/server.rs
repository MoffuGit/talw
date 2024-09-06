use crate::app::api::server::get_server;
use crate::app::api::server::member_can_edit;
use crate::app::api::server::use_server;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
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
}

pub fn use_current_server_context() -> CurrentServerContext {
    use_context::<CurrentServerContext>().expect("Should acces to current server context")
}

#[allow(non_snake_case)]
#[component]
pub fn Server() -> impl IntoView {
    let params = use_params_map();
    let leave_server = use_server().leave_server;
    let server = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| get_server(server_id),
    );

    let member_can_edit = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| member_can_edit(server_id),
    );

    view! {
        <div class="h-full w-full relative z-40">
            <Transition fallback=move || ()>
            {
                move || {
                    match (server.get(),  member_can_edit.get()) {
                        (Some(Ok(server)), Some(Ok(member_can_edit))) => {
                            provide_context(CurrentServerContext {
                                server,
                                member_can_edit
                            });
                            view! {
                                <div class="flex w-[240px] h-full fixed inset-y-0 bg-base-200 z-40">
                                    <ServerSideBar />
                                </div>
                                <div class="h-full relative overflow-hidden md:pl-[240px] z-30">
                                    <Outlet/>
                                </div>
                            }.into_view()
                        },
                        (None, _) | (_, None) => {
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
        </div>
    }
}
