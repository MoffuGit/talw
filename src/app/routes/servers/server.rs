use std::str::FromStr;

use crate::app::api::member::get_member;
use crate::app::api::member::member_can_edit;
use crate::app::api::server::get_server;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use crate::app::components::navigation::server::sidebar::ServerSideBarContext;
use crate::entities::member::Member;
use crate::entities::server::Server as ServerEntitie;
use crate::entities::server::ServerStoreFields;
use crate::messages::ClientMessage;
use crate::ws::client::use_ws;
use futures::try_join;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;
use leptos_router::hooks::use_params_map;
use reactive_stores::Store;
use uuid::Uuid;

#[derive(Clone)]
pub struct CurrentServerContext {
    pub server: Store<ServerEntitie>,
    pub member_can_edit: bool,
    pub member: Member,
}

pub fn use_current_server_context() -> CurrentServerContext {
    use_context::<CurrentServerContext>().expect("Should acces to current server context")
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
    let server_data = Resource::new(server_id, move |server_id| async move {
        let server = get_server(server_id);
        let member = get_member(server_id);
        let can_edit = member_can_edit(server_id);
        try_join!(server, member, can_edit)
    });

    let open = RwSignal::new(true);
    //INFO:
    //Solution for providing context to outlet
    //https://github.com/leptos-rs/leptos/issues/3042
    let outer_owner = Owner::current().unwrap();

    let inner_view = move || {
        server_data.and_then(|data| {
            let server = Store::new(data.0.clone());
            let ws = use_ws();
            Effect::new(move |_| {
                let navigate = use_navigate();
                ws.on_app_msg(move |msg| match msg {
                    ClientMessage::LeavedServer { server_id, .. }
                    | ClientMessage::ServerDeleted { server_id } => {
                        if server_id == server.id().get() {
                            navigate("/home", Default::default())
                        }
                    }
                    _ => {}
                });
                ws.on_server_msg(server.id().get(), move |msg| {
                    if let crate::messages::Message::ServerUpdated { name, image } = msg {
                        if let Some(name) = name {
                            *server.name().write() = name;
                        }
                        if let Some(image) = image {
                            *server.image_url().write() = Some(image);
                        }
                    }
                });
            });

            outer_owner.with(|| {
                provide_context(ServerSideBarContext { open });
                provide_context(CurrentServerContext {
                    server,
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

    //NOTE: handle the on error with a redirect
    view! {
        <div class="h-full w-full relative z-40 flex">
            <Transition>{inner_view}</Transition>
        </div>
    }
}
