pub mod header;
use self::header::ThreadHeader;
use crate::app::api::thread::get_thread;
use crate::app::components::navigation::server::{use_current_channel, use_current_thread};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::server::ServerStoreFields;
use crate::entities::thread::ThreadStoreFields;
use crate::ws::client::use_ws;
//use leptos_icons::Icon;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use reactive_stores::Store;
#[component]
pub fn ThreadSideBar() -> impl IntoView {
    let curent_server = use_current_server_context();
    let current_thread = use_current_thread();
    let channel_id = move || use_current_channel().with(|channel_id| channel_id.unwrap());
    view! {
        {move || {
            current_thread
                .get()
                .map(|current| {
                    let thread = Resource::new(
                        move || (),
                        move |_| get_thread(current, channel_id()),
                    );

                    view! {
                        <Transition>
                            {
                                Suspend::new(async move {
                                    thread.await.map(|thread| {
                                        let thread = Store::new(thread);
                                        use_ws().on_server_msg(curent_server.server.id().get(), move |msg| {
                                            match msg {
                                                crate::messages::Message::ThreadDeleted { thread_id } => {
                                                    if thread_id == thread.id().get() {
                                                        use_navigate()("/", Default::default())
                                                    }
                                                },
                                                crate::messages::Message::ChannelDeleted { channel_id } => {
                                                    if channel_id == current {
                                                        use_navigate()("/", Default::default())
                                                    }

                                                },
                                                _ => {}
                                            }
                                        });
                                        provide_context(thread);
                                        view! {
                                            <div class=" flex flex-col flex-1">
                                                <ThreadHeader />
                                                <div class="grow overflow-auto bg-base-200" />
                                                <div class="h-20 shrink-0 flex">
                                                    // NOTE: move this to his own component,
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
                                        }
                                    })
                                })
                            }
                        </Transition>
                    }
                })
        }}
    }
}
