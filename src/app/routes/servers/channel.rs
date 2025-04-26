use std::str::FromStr;

use crate::app::api::channel::get_channel;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
use leptos::prelude::*;
//use leptos_icons::Icon;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_navigate, use_params_map};
use reactive_stores::Store;
use uuid::Uuid;

#[component]
pub fn ChannelView() -> impl IntoView {
    let server_id = use_current_server_context().server.id();
    let params_map = use_params_map();
    let channel_id = move || {
        params_map
            .with(|map| {
                map.get("channel_id")
                    .and_then(|id| Uuid::from_str(&id).ok())
            })
            .unwrap_or_default()
    };
    let channel = Resource::new(
        move || (channel_id(), server_id.get()),
        move |(channel_id, server_id)| get_channel(channel_id, server_id),
    );

    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <Transition>
            <div class="w-full h-full flex relative overflow-hidden">
                <div class="grow min-w-[400px] shrink-0 flex flex-col">
                    {
                        Suspend::new(async move {
                            channel.await.map(|channel| {
                                let channel = Store::new(channel);
                                let ws = use_ws();
                                let navigate = use_navigate();
                                ws.on_server_msg(server_id.get(), move |msg| {
                                    match msg {
                                        Message::ChannelDeleted { channel_id } => {
                                            if channel.id().get() == channel_id {
                                                navigate("/", Default::default())
                                            }
                                        },
                                        Message::ChannelUpdated { topic, name, channel_id } => {
                                            if channel.id().get() == channel_id {
                                                *channel.topic().write() = topic;
                                                if let Some(name) = name {
                                                    *channel.name().write() = name;
                                                }
                                            }
                                        },
                                        Message::CategoryDeleted { category_id } => {
                                            if channel.category_id().get().is_some_and(|category| category == category_id)  {
                                                *channel.category_id().write() = None;
                                            }
                                        },
                                        _ => {}
                                    }
                                });
                                view! {
                                    <ChannelHeader channel=channel />
                                    <div class="w-full h-full flex bg-base-200">
                                        // NOTE:
                                        // this is the future chat
                                        // NOTE: move this to his own component,
                                        <div class="flex flex-col h-auto w-full">
                                            <div class="grow overflow-auto" />
                                            <div class="h-20 shrink-0 flex">
                                                <div class="m-4 w-full grow bg-base-300/60 rounded-lg flex items-center px-4">
                                                    <Icon icon=IconData::CirclePlus class="w-7 h-7 stroke-base-content/40 grow-0 mr-4"  />
                                                    <div class="grow text-base-content/60">
                                                        {move || format!("Message #{}", channel.name().get())}
                                                    </div>
                                                    <Icon icon=IconData::Sticker class="w-7 h-7 stroke-base-content/40"/>

                                                </div>
                                            </div>
                                        </div>
                                        <MemberSideBar server_id=server_id.get() />
                                    </div>
                                }
                            })
                        })
                    }
                </div>
                <Outlet />
            </div>
        </Transition>
    }
}
