use crate::app::api::member::{get_member_profile, get_thread_members};
use crate::app::api::thread::{get_threads_from_channel, use_thread};
use crate::app::api::user::get_user_profile;
use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::components::modal::create_thread::CreatethreadModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::dropdown_menu::*;
use crate::entities::thread::Thread;
use crate::entities::user::Profile;
use icondata;
use leptos::{html, prelude::*};
use leptos_icons::Icon;
use leptos_router::components::A;
use uuid::Uuid;

#[component]
pub fn ThreadMenu(channel_id: Uuid, server_id: Uuid) -> impl IntoView {
    let open = RwSignal::new(false);
    let create_thread_node = NodeRef::<html::Div>::new();
    let context_menu_ref = NodeRef::<html::Div>::new();
    let delete_thread_modal_ref = NodeRef::<html::Div>::new();
    view! {
        <DropdownProvider open=open modal=false>
            <DropdownTrigger class="hover:bg-base-100 rounded-md p-1 cursor-pointer select-none p-1">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="lucide lucide-list-tree"
                >
                    <path d="M21 12h-8" />
                    <path d="M21 6H8" />
                    <path d="M21 18h-8" />
                    <path d="M3 6v4c0 1.1.9 2 2 2h3" />
                    <path d="M3 10v6c0 1.1.9 2 2 2h3" />
                </svg>
            </DropdownTrigger>
            <DropdownContent
                ignore=vec![context_menu_ref, create_thread_node, delete_thread_modal_ref]
                side=MenuSide::Bottom
                align=MenuAlign::End
                class="w-auto h-auto z-40"
            >
                <div class="w-[510px] h-auto bg-base-300 flex flex-col rounded-md border border-base-100 p-2 origin-top-right starting:opacity-0 starting:translate-x-2 starting:-translate-y-2 starting:scale-95 transition-all">
                    <div class="w-full h-auto flex items-center rounded-t-md mb-2">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-list-tree mr-2"
                        >
                            <path d="M21 12h-8" />
                            <path d="M21 6H8" />
                            <path d="M21 18h-8" />
                            <path d="M3 6v4c0 1.1.9 2 2 2h3" />
                            <path d="M3 10v6c0 1.1.9 2 2 2h3" />
                        </svg>
                        <div class="font-semibold text-lg mr-auto">"Threads"</div>
                        <div class="hover:bg-base-content/10 rounded-md p-1 fill-base-content ml-[10px]">
                            <Icon
                                icon=icondata::LuX
                                // class="w-5 h-5"
                                on:click=move |_| open.set(false)
                            />
                        </div>
                    </div>
                    <div class="bg-base-100 h-px -mx-1" />
                    <ActiveThreads
                        channel_id=channel_id
                        server_id=server_id
                        open=open
                        context_menu_ref=context_menu_ref
                        delete_thread_modal_ref=delete_thread_modal_ref
                        create_thread_node=create_thread_node
                    />
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn ActiveThreads(
    channel_id: Uuid,
    open: RwSignal<bool>,
    context_menu_ref: NodeRef<html::Div>,
    delete_thread_modal_ref: NodeRef<html::Div>,
    create_thread_node: NodeRef<html::Div>,
    server_id: Uuid,
) -> impl IntoView {
    let thread_context = use_thread();
    let delete_thread = thread_context.delete_thread;
    let get_threads = Resource::new(
        move || {
            (
                delete_thread.version().get(),
                thread_context.create_thread.version().get(),
            )
        },
        move |_| get_threads_from_channel(channel_id),
    );
    view! {
        <div class="w-full h-auto relative">
            <div class="flex w-full justify-between items-center my-2">
                <div class="h-6 w-[200px] border border-base-100 rounded p-1.5 flex items-center justify-between">
                    "Search" <Icon icon=icondata::LuSearch />
                </div>
                <CreatethreadModal
                    content_ref=create_thread_node
                    channel_id=channel_id
                    server_id=server_id
                    class="w-auto h-6 rounded bg-base-content/10 hover:bg-base-content/20 px-2 flex items-center"
                >
                    "Create"
                </CreatethreadModal>
            </div>
            <div class="w-full min-h-[342px] w-full max-h-[720px] overflow-y-scroll overflow-x-hidden space-y-2">
                <Transition fallback=move || ()>
                    {move || {
                        get_threads
                            .and_then(|threads| {
                                threads
                                    .iter()
                                    .map(|thread| {
                                        view! {
                                            <ThreadLink
                                                thread=thread.clone()
                                                open=open
                                                context_menu_ref=context_menu_ref
                                                delete_thread_modal_ref=delete_thread_modal_ref
                                            />
                                        }
                                    })
                                    .collect_view()
                            })
                    }}
                </Transition>
            </div>
        </div>
    }
}

#[component]
pub fn ThreadLink(
    thread: Thread,
    open: RwSignal<bool>,
    context_menu_ref: NodeRef<html::Div>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let created_by_profile = Resource::new(|| (), move |_| get_member_profile(thread.created_by));
    let name = StoredValue::new(thread.name.to_string());
    let thread_id = thread.id;
    let thread = StoredValue::new(thread);
    view! {
        <Transition fallback=move || ()>
            {move || {
                created_by_profile
                    .and_then(|profile| {
                        let profile = profile.as_ref().expect("should have a profile");
                        let created_by = StoredValue::new(profile.name.clone());
                        let member_url = StoredValue::new(profile.image_url.clone());
                        view! {
                            <ContextMenuProvider modal=false content_ref=context_menu_ref>
                                <ContextMenuTrigger class="w-full h-auto">
                                    <A
                                        on:click=move |_| open.set(false)
                                        href=format!("{}", thread_id.simple())
                                        {..}
                                        class="w-full h-20 hover:bg-base-content/10 rounded-md inline-block flex justify-between items-center px-2"
                                    >
                                        <div class="flex flex-col w-auto h-auto">
                                            <div class="font-semibold text-lg">{name.get_value()}</div>
                                            <div class="flex items-center text-sm">
                                                {if let Some(url) = member_url.get_value() {
                                                    view! {
                                                        <img
                                                            class="w-5 h-5 object-cover rounded-full mr-1"
                                                            src=url
                                                        />
                                                    }
                                                        .into_any()
                                                } else {
                                                    view! {
                                                        <div class="w-5 h-5 rounded-full bg-white border-base-200 mr-1" />
                                                    }
                                                        .into_any()
                                                }} {format!("Started by {}", created_by.get_value())}
                                            </div>
                                        </div>
                                        <ThreadMembers thread_id=thread_id />
                                    </A>
                                </ContextMenuTrigger>
                                <ContextMenuContent
                                    ignore=vec![delete_thread_modal_ref]
                                    class="z-50 select-none"
                                >
                                    <ThreadMenuContent
                                        delete_thread_modal_ref=delete_thread_modal_ref
                                        thread=thread.get_value()
                                        open=open
                                    />
                                </ContextMenuContent>
                            </ContextMenuProvider>
                        }
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn ThreadMembers(thread_id: Uuid) -> impl IntoView {
    let thread_members = Resource::new(|| (), move |_| get_thread_members(thread_id));

    view! {
        <Transition fallback=move || ()>
            <div class="-space-x-2">
                {move || {
                    thread_members
                        .and_then(|thread_members| {
                            thread_members
                                .clone()
                                .into_iter()
                                .map(|member| {
                                    let member_profile = Resource::new(
                                        || (),
                                        move |_| get_user_profile(member.user_id),
                                    );
                                    view! {
                                        {move || {
                                            if let Some(Ok(Profile { image_url, .. })) = member_profile
                                                .get()
                                            {
                                                view! {
                                                    <img
                                                        class="w-4 h-4 object-cover rounded-full mr-1"
                                                        src=image_url
                                                    />
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="w-4 h-4 rounded-full bg-white border-base-200 mr-1" />
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    }
                                })
                                .collect_view()
                        })
                }}
            </div>
        </Transition>
    }
}
