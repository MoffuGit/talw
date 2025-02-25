use crate::app::api::thread::get_thread;
use crate::app::api::thread::get_threads_for_member;
use crate::app::api::thread::use_thread;
use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::routes::servers::server::use_current_server_context;
use leptos::html;
use leptos::prelude::*;
use leptos_router::components::A;
use uuid::Uuid;

use crate::app::components::navigation::server::use_current_thread;
use crate::app::components::ui::context_menu::*;
use crate::entities::thread::Thread as EntThread;

#[component]
pub fn Thread(channel_id: Uuid) -> impl IntoView {
    let use_threads = use_thread();
    let member = use_current_server_context().member;
    let threads = Resource::new(
        move || {
            (
                use_threads.leave_thread.version().get(),
                use_threads.create_thread.version().get(),
                use_threads.join_thread.version().get(),
                use_threads.delete_thread.version().get(),
            )
        },
        move |_| get_threads_for_member(channel_id, member.id),
    );
    view! {
        <Transition fallback=move || ()>
            <div class="ml-6 border-0 border-l border-l-base-100">
                <For
                    each=move || threads.get().and_then(Result::ok).unwrap_or_default()
                    key=|thread| thread.id
                    children=move |thread: EntThread| {
                        view! {
                            <div class="flex h-auto ml-2 items-center">
                                <ThreadMenu thread=thread.clone() />
                            </div>
                        }
                    }
                />
            </div>
        </Transition>
    }
}

#[component]
pub fn ThreadMenu(thread: EntThread) -> impl IntoView {
    let open = RwSignal::new(false);
    let use_current_thread = use_current_thread();
    let is_current_thread = move || {
        use_current_thread.with(|url| url.is_some_and(|(thread_url)| thread_url == thread.id))
    };
    let name = StoredValue::new(thread.name.clone());
    let delete_thread_modal_ref = NodeRef::<html::Div>::new();
    view! {
        <ContextMenuProvider modal=false open=open>
            <ContextMenuTrigger class="w-full h-auto">
                <div class="relative py-[1px] mt-0.5 w-full max-h-[32px] group">
                    <A
                        href=move || {
                            format!("thread/{}/{}", thread.channel_id.simple(), thread.id.simple())
                        }
                        {..}
                        class="relative box-border flex flex-col cursor-pointer hover:bg-base-100 rounded-lg"
                    >
                        <div class="relative flex flex-row group items-center py-[6px] px-2">
                            <div class="whitespace-nowrap overflow-hidden text-ellipsis mr-auto leading-5 flex-auto relative text-sm">
                                {name.get_value()}
                            </div>
                        </div>
                    </A>
                    <div
                        on:click=move |_| {
                            open.set(true);
                        }
                        class=move || {
                            format!(
                                "absolute right-1 top-1.5 p-0.5 hover:bg-base-100 rounded {}",
                                if is_current_thread() {
                                    "opacity-100"
                                } else {
                                    "opacity-0 group-hover:opacity-100"
                                },
                            )
                        }
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-ellipsis"
                        >
                            <circle cx="12" cy="12" r="1" />
                            <circle cx="19" cy="12" r="1" />
                            <circle cx="5" cy="12" r="1" />
                        </svg>
                    </div>
                </div>
            </ContextMenuTrigger>
            <ContextMenuContent ignore=vec![delete_thread_modal_ref] class="z-40".into()>
                <ThreadMenuContent
                    delete_thread_modal_ref=delete_thread_modal_ref
                    thread=thread.clone()
                    open=open
                />
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}
