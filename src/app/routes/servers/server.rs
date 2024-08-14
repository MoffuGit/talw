use crate::app::api::server::check_server;
use crate::app::api::server::get_member;
use crate::app::api::server::use_server;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use leptos::*;
use leptos_router::use_params_map;
use leptos_router::Outlet;
use leptos_router::Redirect;
use uuid::Uuid;

#[allow(non_snake_case)]
#[component]
pub fn Server() -> impl IntoView {
    let params = use_params_map();
    //NOTE: las acciones las vamos a crear en el contexto del server y ya luego subscribimos los
    //resources a esas, server_settings,
    let leave_server = use_server().leave_server;
    let server = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| check_server(server_id),
    );
    let member = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| get_member(server_id),
    );

    view! {
        <div class="h-full w-full relative z-40">
            <Transition fallback=move || ()>
            {
                move || {
                    match (server.get(), member.get()) {
                        (Some(Ok(server)), Some(Ok(member))) => {
                            provide_context((server.clone(), member.clone()));
                            view! {
                                <div class="flex w-[240px] h-full fixed inset-y-0 bg-base-200 z-40">
                                    <ServerSideBar server=server member=member/>
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
