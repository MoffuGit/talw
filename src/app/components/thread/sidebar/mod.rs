pub mod header;
use self::header::ThreadHeader;
use crate::app::api::thread::get_thread;
use crate::app::components::navigation::server::{use_current_channel, use_current_thread};
use crate::entities::thread::Thread;
//use leptos_icons::Icon;

#[derive(Clone)]
struct CurrentThreadContext {
    pub thread: Thread,
}

use leptos::prelude::*;
#[component]
pub fn ThreadSideBar() -> impl IntoView {
    let current_thread = use_current_thread();
    let channel_id = move || use_current_channel().with(|channel_id| channel_id.unwrap().simple());
    view! {
        {move || {
            current_thread
                .get()
                .map(|current| {
                    let thread = Resource::new(
                        move || (),
                        move |_| get_thread(current, channel_id().into_uuid()),
                    );

                    view! {
                        <Transition>
                            {move || {
                                thread
                                    .and_then(|thread| {
                                        let name = thread.name.clone();
                                        provide_context(CurrentThreadContext {
                                            thread: thread.clone(),
                                        });
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
                                                            {format!("Message #{}", name)}
                                                        </div>
                                                        // <Icon icon=icondata::RiEmojiStickerCommunicationFill />
                                                    // class="w-7 h-7 fill-base-content/40"
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    })
                            }}
                        </Transition>
                    }
                })
        }}
    }
}
