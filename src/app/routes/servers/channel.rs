use std::str::FromStr;

use crate::app::api::channel::get_channel;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::chat::Chat;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::stores::ChannelStoreSync;
use crate::app::sync::use_sync;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_navigate, use_params_map};
use log::debug;
use reactive_stores::Store;
use uuid::Uuid;

#[component]
pub fn ChannelView() -> impl IntoView {
    let server_context = use_current_server_context();
    let server_id = server_context.server.id();
    let params_map = use_params_map();
    let channel_id = Signal::derive(move || {
        params_map
            .with(|map| {
                map.get("channel_id")
                    .and_then(|id| Uuid::from_str(&id).ok())
            })
            .unwrap_or_default()
    });

    let channel = Resource::new(
        move || (channel_id.get(), server_id.get()),
        move |(channel_id, server_id)| get_channel(channel_id, server_id),
    );

    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="min-w-[400px] flex-auto flex flex-col">
                <Transition>
                {
                    move || Suspend::new(async move {
                        channel.await.map(|channel| {
                            let channel = Store::new(channel);
                            let navigate = use_navigate();
                            let sync =use_sync();
                            if let Some(sync) = sync {
                                sync.message_router.on_module_msg("ChannelStore", move |msg: ChannelStoreSync| {
                                    match msg {
                                        ChannelStoreSync::Deleted { id } => {
                                            if channel.id().get() == id {
                                                navigate("/", Default::default())
                                            }
                                        },
                                        ChannelStoreSync::Updated { id } => {
                                            debug!("Update for channel: {id}");
                                        },
                                        _ => {}
                                    }
                                });
                            }
                            view! {
                                <ChannelHeader channel=channel />
                                <div class="relative flex w-full h-full min-w-0 min-h-0 overflow-hidden bg-base-200">
                                    <Chat channel_id=channel.id() name=channel.name()/>
                                    <MemberSideBar members=server_context.members />
                                </div>
                            }
                        })
                    })
                }
                </Transition>
            </div>
            <Outlet />
        </div>
    }
}
