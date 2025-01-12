use leptos::*;

use crate::app::api::thread::{check_member_on_thread, use_thread};
use crate::app::components::modal::delete_thread::DeleteThreadModal;
use crate::app::components::navigation::server::{
    use_current_channel, use_current_split_thread, use_current_thread,
};
use crate::app::components::thread::{JoinThread, LeaveThread};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::thread::Thread;
use leptos_router::A;

#[component]
pub fn ThreadMenuContent(
    thread: Thread,
    open: RwSignal<bool>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let thread_context = use_thread();
    let join_thread = thread_context.join_thread;
    let leave_thread = thread_context.leave_thread;
    let current_server_context = use_current_server_context();
    let thread_name = store_value(thread.name.clone());
    let check_member_on_thread = create_resource(
        move || (join_thread.version().get(), leave_thread.version().get()),
        move |_| check_member_on_thread(thread.id),
    );
    view! {
        <div class="w-[188px] h-auto bg-base-200 rounded-md flex flex-col items-center space-y-1 p-2">
            <Transition fallback=move || ()>
                {move || {
                    check_member_on_thread
                        .and_then(|exist| {
                            if *exist {
                                view! {
                                    <LeaveThread
                                        thread_id=thread.id
                                        server_id=current_server_context.server.id
                                        class="flex justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                                    />
                                }
                                    .into_view()
                            } else {
                                view! {
                                    <JoinThread
                                        thread_id=thread.id
                                        server_id=current_server_context.server.id
                                        class="flex justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                                    />
                                }
                            }
                        })
                }}
            </Transition>
            {move || {
                if let Some((channel_url, thread_url)) = use_current_thread().get() {
                    if channel_url != thread.channel_id && thread_url != thread.id {
                        view! {
                            <A
                                href=move || {
                                    format!(
                                        "/servers/{}/thread/{}/{}",
                                        current_server_context.server.id.simple(),
                                        thread.channel_id.simple(),
                                        thread.id.simple(),
                                    )
                                }
                                on:click=move |_| open.set(false)
                                class="flex inline-block justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                            >
                                "Open Full View"
                            </A>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                } else {
                    view! {
                        <A
                            href=move || {
                                format!(
                                    "/servers/{}/thread/{}/{}",
                                    current_server_context.server.id.simple(),
                                    thread.channel_id.simple(),
                                    thread.id.simple(),
                                )
                            }
                            on:click=move |_| open.set(false)
                            class="flex inline-block justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                        >
                            "Open Full View"
                        </A>
                    }
                        .into_view()
                }
            }}
            {move || {
                if let (Some(channel_url), Some(thread_url)) = (
                    use_current_channel().get(),
                    use_current_split_thread().get(),
                ) {
                    if channel_url != thread.channel_id && thread_url != thread.id {
                        view! {
                            <A
                                href=move || {
                                    format!(
                                        "/servers/{}/{}/{}",
                                        current_server_context.server.id.simple(),
                                        thread.channel_id.simple(),
                                        thread.id.simple(),
                                    )
                                }
                                on:click=move |_| open.set(false)
                                class="flex inline-block justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                            >
                                "Open Split View"
                            </A>
                        }
                            .into_view()
                    } else {
                        ().into_view()
                    }
                } else {
                    view! {
                        <A
                            href=move || {
                                format!(
                                    "/servers/{}/{}/{}",
                                    current_server_context.server.id.simple(),
                                    thread.channel_id.simple(),
                                    thread.id.simple(),
                                )
                            }
                            on:click=move |_| open.set(false)
                            class="flex inline-block justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                        >
                            "Open Split View"
                        </A>
                    }
                        .into_view()
                }
            }}
            <Transition fallback=move || ()>
                {move || {
                    if current_server_context.member_can_edit
                        || current_server_context.member.id == thread.created_by
                    {
                        view! {
                            <DeleteThreadModal
                                content_ref=delete_thread_modal_ref
                                thread_id=thread.id
                                thread_name=thread_name.get_value()
                                server_id=current_server_context.server.id
                                class="flex justify-between hover:bg-primary items-center w-full text-sm rounded py-[6px] px-2"
                            >
                                "Delete Thread"
                            </DeleteThreadModal>
                        }
                    } else {
                        ().into_view()
                    }
                }}
            </Transition>
        </div>
    }
}
