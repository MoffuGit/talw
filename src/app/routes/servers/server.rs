use std::str::FromStr;

use crate::app::api::member::get_member;
use crate::app::api::member::member_can_edit;
use crate::app::api::server::get_server;
use crate::app::api::server::use_server;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use crate::app::components::navigation::server::sidebar::ServerSideBarContext;
use crate::entities::member::Member;
use futures::try_join;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_params_map;
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

#[component]
pub fn Server() -> impl IntoView {
    let params_map = use_params_map();
    let leave_server = use_server().leave_server;
    let server_id = move || {
        params_map.with(|map| {
            map.get("id")
                .and_then(|id| Uuid::from_str(&id).ok())
                .expect("should get the server id from the params")
        })
    };
    let server_data = Resource::new(
        move || (leave_server.version().get(), server_id()),
        move |(_, server_id)| async move {
            let server = get_server(server_id);
            let member = get_member(server_id);
            let can_edit = member_can_edit(server_id);
            try_join!(server, member, can_edit)
        },
    );

    let open = RwSignal::new(true);
    //INFO:
    //Solution for providing context to outlet
    //https://github.com/leptos-rs/leptos/issues/3042
    let outer_owner = Owner::current().unwrap();

    let inner_view = move || {
        server_data.and_then(|data| {
            outer_owner.with(|| {
                provide_context(ServerSideBarContext { open });
                provide_context(CurrentServerContext {
                    server: data.0.clone(),
                    member_can_edit: data.2,
                    member: data.1.clone(),
                })
            });
            view! {
                <ServerSideBar />
                <div class="h-full grow relative overflow-hidden z-30">
                    <Outlet />
                </div>
            }
        })
    };

    view! {
        <div class="h-full w-full relative z-40 flex">
            <Transition>{inner_view}</Transition>
        </div>
    }
}
