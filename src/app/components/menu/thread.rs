use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_router::hooks::use_params_map;

use crate::app::api::thread::{check_member_on_thread, use_thread};
use crate::app::components::modal::delete_thread::DeleteThreadModal;
use crate::app::components::navigation::server::use_current_thread;
use crate::app::components::thread::{JoinThread, LeaveThread};
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::server::ServerStoreFields;
use crate::entities::thread::Thread;
use leptos_router::components::A;

#[component]
pub fn ThreadMenuContent(
    thread: Thread,
    open: RwSignal<bool>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let thread_context = use_thread();
    let join_thread = thread_context.join_thread;
    let leave_thread = thread_context.leave_thread;
    let CurrentServerContext {
        server,
        member_can_edit,
        member,
    } = use_current_server_context();
    let thread_name = StoredValue::new(thread.name.clone());
    let check_member_on_thread = Resource::new(
        move || (join_thread.version().get(), leave_thread.version().get()),
        move |_| check_member_on_thread(thread.id),
    );

    let current_thread = use_current_thread();
    let is_split = Memo::new(move |_| use_params_map().with(|map| map.get("thread").is_none()));
    view! {
        <Transition>
            <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                {move || {
                    check_member_on_thread
                        .and_then(|exist| {
                            if *exist {
                                Either::Left(
                                    view! {
                                        <LeaveThread
                                            thread_id=thread.id
                                            server_id=server.id().get()
                                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        />
                                    },
                                )
                            } else {
                                Either::Right(
                                    view! {
                                        <JoinThread
                                            thread_id=thread.id
                                            server_id=server.id().get()
                                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        />
                                    },
                                )
                            }
                        })
                }}
                {move || {
                    current_thread
                        .with(|current| {
                            current
                                .is_none_or(|current| current != thread.id)
                                .then(|| {
                                    view! {
                                        <A
                                            href=move || {
                                                format!(
                                                    "/servers/{}/thread/{}/{}",
                                                    server.id().get().simple(),
                                                    thread.channel_id.simple(),
                                                    thread.id.simple(),
                                                )
                                            }
                                            on:click=move |_| open.set(false)
                                            {..}
                                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        >
                                            "Open Full View"
                                        </A>
                                        <A
                                            href=move || {
                                                format!(
                                                    "/servers/{}/{}/{}",
                                                    server.id().get().simple(),
                                                    thread.channel_id.simple(),
                                                    thread.id.simple(),
                                                )
                                            }
                                            on:click=move |_| open.set(false)
                                            {..}
                                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        >
                                            "Open Split View"
                                        </A>
                                    }
                                })
                        })
                }}
                {move || {
                    current_thread
                        .with(|current| {
                            current
                                .is_some_and(|current| current == thread.id)
                                .then(|| {
                                    if is_split.get() {
                                        Either::Left(
                                            view! {
                                                <A
                                                    href=move || {
                                                        format!(
                                                            "/servers/{}/thread/{}/{}",
                                                            server.id().get().simple(),
                                                            thread.channel_id.simple(),
                                                            thread.id.simple(),
                                                        )
                                                    }
                                                    on:click=move |_| open.set(false)
                                                    {..}
                                                    class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                                >
                                                    "Open Full View"
                                                </A>
                                            },
                                        )
                                    } else {
                                        Either::Right(
                                            view! {
                                                <A
                                                    href=move || {
                                                        format!(
                                                            "/servers/{}/{}/{}",
                                                            server.id().get().simple(),
                                                            thread.channel_id.simple(),
                                                            thread.id.simple(),
                                                        )
                                                    }
                                                    on:click=move |_| open.set(false)
                                                    {..}
                                                    class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                                >
                                                    "Open Split View"
                                                </A>
                                            },
                                        )
                                    }
                                })
                        })
                }}
                {(member_can_edit
                    || member.id == thread.created_by)
                    .then(|| {
                        view! {
                            <DeleteThreadModal
                                content_ref=delete_thread_modal_ref
                                thread_id=thread.id
                                thread_name=thread_name.get_value()
                                server_id=server.id().get()
                                class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                            >
                                "Delete Thread"
                            </DeleteThreadModal>
                        }
                    })}
            </div>
        </Transition>
    }
}
