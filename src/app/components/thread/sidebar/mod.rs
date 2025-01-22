pub mod header;
use self::header::ThreadHeader;
use crate::app::api::thread::{get_thread, use_thread};
use crate::app::components::navigation::server::use_current_channel;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::thread::Thread;
use leptos_icons::Icon;
use leptos_router::{use_router, Redirect};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone)]
struct CurrentThreadContext {
    pub thread: Thread,
}

use leptos::*;
#[component]
pub fn ThreadSideBar() -> impl IntoView {
    let current_thread = move || {
        use_router()
            .pathname()
            .with(|path| path.split('/').nth(4).map(Uuid::from_str))
    };
    let server_id = use_current_server_context().server.id.simple();
    let channel_id = move || use_current_channel().get().unwrap().simple();
    view! {
        {move || {
            match current_thread() {
                Some(Ok(thread_id)) => {
                    let current_thread = create_resource(
                        move || (use_thread().delete_thread.version().get()),
                        move |_| get_thread(thread_id, channel_id().into_uuid()),
                    );
                    view! {
                        <Transition fallback=move || ()>
                            {move || {
                                match current_thread.get() {
                                    None => ().into_view(),
                                    Some(Err(_)) => {
                                        view! {
                                            <Redirect path=format!(
                                                "/servers/{}/{}",
                                                server_id,
                                                channel_id(),
                                            ) />
                                        }
                                            .into_view()
                                    }
                                    Some(Ok(thread)) => {
                                        let name = thread.name.clone();
                                        provide_context(CurrentThreadContext { thread });
                                        view! {
                                            <div class=" flex flex-col flex-1">
                                                <ThreadHeader />
                                                <div class="flex-grow overflow-auto bg-base-200" />
                                                <div class="h-20 flex-shrink-0 flex">
                                                    // NOTE: move this to his own component,
                                                    <div class="m-4 w-full flex-grow bg-base-300/60 rounded-lg flex items-center px-4">
                                                        <Icon
                                                            icon=icondata::RiAddCircleSystemFill
                                                            class="w-7 h-7 fill-base-content/40 grow-0 mr-4"
                                                        />
                                                        <div class="grow text-base-content/60">
                                                            {format!("Message #{}", name)}
                                                        </div>
                                                        <Icon
                                                            icon=icondata::RiEmojiStickerCommunicationFill
                                                            class="w-7 h-7 fill-base-content/40"
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                            .into_view()
                                    }
                                }
                            }}
                        </Transition>
                    }
                        .into_view()
                }
                Some(Err(_)) | None => {
                    view! { <Redirect path=format!("/servers/{}/{}", server_id, channel_id()) /> }
                        .into_view()
                }
            }
        }}
    }
}
