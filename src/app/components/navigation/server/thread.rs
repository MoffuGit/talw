use crate::app::api::thread::get_thread;
use crate::app::api::thread::get_threads_for_member;
use crate::app::api::thread::use_thread;
use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::routes::servers::server::use_current_server_context;
use leptos::*;
use leptos_router::A;
use uuid::Uuid;

use crate::app::components::navigation::server::use_current_thread;
use crate::app::components::ui::context_menu::*;
use crate::entities::thread::Thread;

#[component]
pub fn Thread(channel_id: Uuid) -> impl IntoView {
    let use_threads = use_thread();
    let member = use_current_server_context().member;
    let threads = create_resource(
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
            {move || {
                threads
                    .and_then(|threads| {
                        if let Some((channel_url, thread_url)) = use_current_thread().get() {
                            if !threads.iter().any(|thread| thread.id == thread_url)
                                && channel_id == channel_url
                            {
                                let threads_is_empty = threads.is_empty();
                                let thread = create_resource(
                                    move || (),
                                    move |_| get_thread(thread_url, channel_id),
                                );
                                view! {
                                    <Transition fallback=move || ()>
                                        {move || {
                                            thread
                                                .and_then(|thread| {
                                                    view! {
                                                        <div class="flex h-auto ml-6 items-center">
                                                            {if threads_is_empty {
                                                                view! {
                                                                    <svg viewBox="0 0 12 11" class="h-[12px] fill-base-content">
                                                                        <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                                    </svg>
                                                                }
                                                                    .into_view()
                                                            } else {
                                                                view! {
                                                                    <div class="flex flex-col -space-y-1">
                                                                        <svg viewBox="0 0 12 11" class="h-[12px] fill-base-content">
                                                                            <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                                        </svg>
                                                                        <svg
                                                                            viewBox="0 0 12 11"
                                                                            class="h-[12px] fill-base-content rotate-90"
                                                                        >
                                                                            <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                                        </svg>
                                                                    </div>
                                                                }
                                                                    .into_view()
                                                            }} <ThreadMenu thread=thread.clone() />
                                                        </div>
                                                    }
                                                })
                                        }}
                                    </Transition>
                                }
                                    .into_view()
                            } else {
                                ().into_view()
                            }
                        } else {
                            ().into_view()
                        }
                    })
            }}
        </Transition>
        <Transition fallback=move || ()>
            {move || {
                threads
                    .and_then(|threads| {
                        let len = threads.len();
                        threads
                            .iter()
                            .enumerate()
                            .map(|(idx, thread)| {
                                view! {
                                    <div class="flex h-auto ml-6 items-center">
                                        {if idx == len - 1 {
                                            view! {
                                                <svg viewBox="0 0 12 11" class="h-[12px] fill-base-content">
                                                    <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                </svg>
                                            }
                                                .into_view()
                                        } else {
                                            view! {
                                                <div class="flex flex-col -space-y-1">
                                                    <svg viewBox="0 0 12 11" class="h-[12px] fill-base-content">
                                                        <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                    </svg>
                                                    <svg
                                                        viewBox="0 0 12 11"
                                                        class="h-[12px] fill-base-content rotate-90"
                                                    >
                                                        <path d="M11 9H4C2.89543 9 2 8.10457 2 7V1C2 0.447715 1.55228 0 1 0C0.447715 0 0 0.447715 0 1V7C0 9.20914 1.79086 11 4 11H11C11.5523 11 12 10.5523 12 10C12 9.44771 11.5523 9 11 9Z" />
                                                    </svg>
                                                </div>
                                            }
                                                .into_view()
                                        }} <ThreadMenu thread=thread.clone() />
                                    </div>
                                }
                            })
                            .collect_view()
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn ThreadMenu(thread: Thread) -> impl IntoView {
    let open = create_rw_signal(false);
    let use_current_thread = use_current_thread();
    let is_current_thread = move || {
        use_current_thread.with(|url| url.is_some_and(|(_, thread_url)| thread_url == thread.id))
    };
    let name = store_value(thread.name.clone());
    let delete_thread_modal_ref = create_node_ref::<html::Div>();
    view! {
        <ContextMenuProvider modal=false open=open>
            <ContextMenuTrigger class="w-full h-auto">
                <div class="relative py-[1px] rounded mt-0.5 w-full max-h-[32px] group">
                    <A
                        href=move || {
                            format!("thread/{}/{}", thread.channel_id.simple(), thread.id.simple())
                        }
                        class="relative box-border flex flex-col cursor-pointer hover:bg-base-content/5"
                    >
                        <div class="relative flex flex-row group items-center py-[6px] px-2">
                            <div
                                class="whitespace-nowrap overflow-hidden text-ellipsis text-[16px] mr-auto font-bold text-base-content/50 leading-5 flex-auto relative"
                            >
                                {name.get_value()}
                            </div>
                        </div>
                    </A>
                    <div
                        on:click=move |_| {
                            open.set(true);
                        }
                        class=move || {
                            format!("absolute right-1 top-1.5 p-0.5 hover:bg-base-content/5 rounded {}", if is_current_thread() {
                                "opacity-100"
                            }else {
                                "opacity-0 group-hover:opacity-100"
                            })
                        }
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-ellipsis"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>
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
