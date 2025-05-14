pub mod split;

use crate::app::api::channel::get_channel;
use crate::app::api::thread::get_thread;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::navigation::server::{use_current_channel, use_current_thread};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use crate::entities::thread::ThreadStoreFields;
use crate::ws::client::use_ws;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;
//use leptos_icons::Icon;

#[component]
pub fn Thread() -> impl IntoView {
    let server_id = use_current_server_context().server.id();
    provide_context(SideBarContext(RwSignal::new(false)));
    let current_thread = use_current_thread();
    let current_channel = use_current_channel();
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="grow min-w-[400px] shrink-0 flex flex-col">
                {move || {
                    current_channel
                        .get()
                        .zip(current_thread.get())
                        .map(|(channel_id, thread_id)| {
                            let channel = Resource::new(
                                move || {
                                    server_id.get()
                                },
                                move |server_id| get_channel(channel_id, server_id),
                            );
                            let thread = Resource::new(
                                move || (),
                                move |_| get_thread(thread_id, channel_id),
                            );
                            view! {
                                <Transition>
                                    {
                                        Suspend::new(async move {
                                            match (channel.await, thread.await) {
                                                (Ok(channel), Ok(thread )) => {
                                                    let channel = Store::new(channel);
                                                    let thread = Store::new(thread);
                                                    use_ws().on_server_msg(server_id.get(), move |msg| {
                                                        match msg {
                                                            crate::messages::Message::ThreadDeleted { thread_id } => {
                                                                if thread_id == thread.id().get() {
                                                                    use_navigate()("/", Default::default())
                                                                }
                                                            },
                                                            crate::messages::Message::ChannelDeleted { channel_id } => {
                                                                if channel_id == channel.id().get() {
                                                                    use_navigate()("/", Default::default())
                                                                }

                                                            },
                                                            crate::messages::Message::ChannelUpdated { channel_id, topic, name } => {
                                                                if channel_id == channel.id().get() {
                                                                    if let Some(topic) = topic {
                                                                        *channel.topic().write() = Some(topic)
                                                                    }

                                                                    if let Some(name) = name {
                                                                        *channel.name().write() = name
                                                                    }
                                                                }
                                                            },
                                                            _ => {}
                                                        }
                                                    });
                                                    view!{
                                                        <ChannelHeader
                                                            channel=channel
                                                            thread=thread
                                                        />
                                                        <div class="w-full h-full flex bg-base-200">
                                                            // NOTE:
                                                            // this is the future chat
                                                            // NOTE: move this to his own component,
                                                            <div class="flex flex-col h-auto w-full">
                                                                <div class="grow overflow-auto" />
                                                                <div class="h-20 shrink-0 flex">
                                                                    <div class="m-4 w-full grow bg-base-300/60 rounded-lg flex items-center px-4">
                                                                        // <Icon icon=icondata::RiAddCircleSystemFill />
                                                                        // class="w-7 h-7 fill-base-content/40 grow-0 mr-4"
                                                                        <div class="grow text-base-content/60">
                                                                            {move || format!("Message #{}", thread.name().get())}
                                                                        </div>
                                                                        // <Icon icon=icondata::RiEmojiStickerCommunicationFill />
                                                                    // class="w-7 h-7 fill-base-content/40"
                                                                    </div>
                                                                </div>
                                                            </div>
                                                            <MemberSideBar thread_id=thread_id />
                                                        </div>

                                                    }.into_any()
                                                }
                                                _ => ().into_any()
                                            }
                                        })
                                    }
                                </Transition>
                            }
                        })
                }}
            </div>
        </div>
    }
}
