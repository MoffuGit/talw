pub mod split;

use crate::app::api::channel::get_channel;
use crate::app::api::member::get_thread_members;
use crate::app::api::thread::get_thread;
use crate::app::components::channel::header::ChannelHeader;
use crate::app::components::channel::sidebars::{MemberSideBar, SideBarContext};
use crate::app::components::chat::Chat;
use crate::app::components::navigation::server::{use_current_channel, use_current_thread};
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::MemberStore;
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
    let current_channel = use_current_channel();
    let current_thread = use_current_thread();
    let channel = Resource::new(
        move || (server_id.get(), current_channel.get()),
        move |(server_id, channel_id)| get_channel(channel_id.unwrap_or_default(), server_id),
    );
    let thread = Resource::new(
        move || (current_thread.get(), current_channel.get()),
        move |(thread_id, channel_id)| {
            get_thread(
                thread_id.unwrap_or_default(),
                channel_id.unwrap_or_default(),
            )
        },
    );
    let members = Resource::new(
        move || (use_current_thread().get()),
        move |thread_id| get_thread_members(thread_id.unwrap_or_default()),
    );
    view! {
        <div class="w-full h-full flex relative overflow-hidden">
            <div class="grow min-w-[400px] shrink-0 flex flex-col">
                <Transition>
                    {
                        move || Suspend::new(async move {
                            match (channel.await, thread.await, members.await) {
                                (Ok(channel), Ok(thread ), Ok(members)) => {
                                    let channel = Store::new(channel);
                                    let thread = Store::new(thread);
                                    let members = Store::new(MemberStore {members});
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
                                            <Chat channel_id=channel.id() thread_id=thread.id() name=thread.name() />
                                            <MemberSideBar members=members />
                                        </div>

                                    }.into_any()
                                }
                                _ => ().into_any()
                            }
                        })
                    }
                </Transition>
            </div>
        </div>
    }
}
