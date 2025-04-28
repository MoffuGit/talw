use leptos::either::Either;
use leptos::{html, prelude::*};
use leptos_router::hooks::use_params_map;
use reactive_stores::Field;

use crate::app::api::thread::check_member_on_thread;
use crate::app::components::modal::delete_thread::DeleteThreadModal;
use crate::app::components::navigation::server::use_current_thread;
use crate::app::components::thread::{JoinThread, LeaveThread};
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::server::ServerStoreFields;
use crate::entities::thread::{Thread, ThreadStoreFields};
use crate::ws::client::use_ws;
use leptos_router::components::A;

#[component]
pub fn ThreadMenuContent(
    #[prop(into)] thread: Field<Thread>,
    open: RwSignal<bool>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let CurrentServerContext {
        server,
        member_can_edit,
        member,
    } = use_current_server_context();
    let thread_name = thread.name();
    let check_member_on_thread = Resource::new(move || thread.id().get(), check_member_on_thread);

    let current_thread = use_current_thread();
    let is_split = Memo::new(move |_| use_params_map().with(|map| map.get("thread").is_none()));
    view! {
        <Transition>
            <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                {move || {
                    check_member_on_thread
                        .and_then(|exist| {
                            let exist = RwSignal::new(*exist);
                            use_ws().on_server_msg(server.id().get(), move |msg| {
                                match msg {
                                    crate::messages::Message::ThreadDeleted { thread_id } => {
                                        if thread_id == thread.id().get() {
                                            open.set(false);
                                        }
                                    },
                                    crate::messages::Message::ChannelDeleted {channel_id} => {
                                        if channel_id == thread.channel_id().get() {
                                            open.set(false);
                                        }
                                    },
                                    crate::messages::Message::MemberJoinThread { thread_id, user_id } => {
                                        if thread_id == thread.id().get() && user_id == member.user_id {
                                            exist.set(true);
                                        }
                                    },
                                    crate::messages::Message::MemberLeaveThread { thread_id, user_id } => {
                                        if thread_id == thread.id().get() && user_id == member.user_id {
                                            exist.set(false);
                                        }
                                    },
                                    _ => {},

                                }
                            });
                            view!{
                                move || {
                                    if exist.get() {
                                        Either::Left(
                                            view! {
                                                <LeaveThread
                                                    thread_id=thread.id().get()
                                                    server_id=server.id().get()
                                                    class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                                />
                                            },
                                        )
                                    } else {
                                        Either::Right(
                                            view! {
                                                <JoinThread
                                                    thread_id=thread.id().get()
                                                    server_id=server.id().get()
                                                    class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                                />
                                            },
                                        )
                                    }
                                }
                            }
                        })
                }}
                {move || {
                    current_thread
                        .with(|current| {
                            current
                                .is_none_or(|current| current != thread.id().get())
                                .then(|| {
                                    view! {
                                        <A
                                            href=move || {
                                                format!(
                                                    "/servers/{}/thread/{}/{}",
                                                    server.id().get().simple(),
                                                    thread.channel_id().get().simple(),
                                                    thread.id().get().simple(),
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
                                                    thread.channel_id().get().simple(),
                                                    thread.id().get().simple(),
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
                                .is_some_and(|current| current == thread.id().get())
                                .then(|| {
                                    if is_split.get() {
                                        Either::Left(
                                            view! {
                                                <A
                                                    href=move || {
                                                        format!(
                                                            "/servers/{}/thread/{}/{}",
                                                            server.id().get().simple(),
                                                            thread.channel_id().get().simple(),
                                                            thread.id().get().simple(),
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
                                                            thread.channel_id().get().simple(),
                                                            thread.id().get().simple(),
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
                    || member.id == thread.created_by().get())
                    .then(|| {
                        view! {
                            <DeleteThreadModal
                                content_ref=delete_thread_modal_ref
                                thread_id=thread.id().get()
                                thread_name=thread_name
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
