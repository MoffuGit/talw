use std::str::FromStr;

use crate::app::api::channel::get_channel;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::chat::Chat;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::{use_navigate, use_params_map};
use reactive_stores::Store;
use uuid::Uuid;

#[component]
pub fn ChannelView() -> impl IntoView {
    let server_id = use_current_server_context().server.id();
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
    //NOTE: Move the roles and member into here

    provide_context(SideBarContext(RwSignal::new(false)));
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="min-w-[400px] flex-auto flex flex-col">
                <Transition>
                {
                    move || Suspend::new(async move {
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
                                        if channel_id == channel.id().get() {
                                            if let Some(topic) = topic {
                                                *channel.topic().write() = Some(topic)
                                            }

                                            if let Some(name) = name {
                                                *channel.name().write() = name
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
                                <div class="relative overflow-auto flex shrink grow bg-base-200">
                                    <Chat channel_id=channel.id() name=channel.name()/>
                                    <MemberSideBar />
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
