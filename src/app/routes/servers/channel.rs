use std::str::FromStr;

use crate::app::api::channel::get_channel;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::server::ServerStoreFields;
use leptos::prelude::*;
//use leptos_icons::Icon;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_params_map;
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
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="grow min-w-[400px] shrink-0 flex flex-col">
                <Transition>
                    {move || {
                        channel
                            .and_then(|channel| {
                                let name = channel.name.clone();
                                view! {
                                    <ChannelHeader channel=channel.clone() />
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
                                                        {format!("Message #{}", name)}
                                                    </div>
                                                    <Icon icon=IconData::Sticker class="w-7 h-7 stroke-base-content/40"/>

                                                </div>
                                            </div>
                                        </div>
                                        <MemberSideBar server_id=server_id.get() />
                                    </div>
                                }
                            })
                    }}
                </Transition>
            </div>
            <Outlet />
        </div>
    }
}
