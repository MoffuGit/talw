use crate::app::api::member::{get_five_thread_members, get_member_profile};
use crate::app::api::thread::get_threads_from_channel;
use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::components::modal::create_thread::CreatethreadModal;
use crate::app::components::navigation::server::thread::{ThreadStore, ThreadStoreStoreFields};
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::entities::thread::{Thread, ThreadStoreFields};
use crate::messages::Message;
// use crate::ws::client::use_ws;
//use icondata;
use leptos::{html, prelude::*};
//use leptos_icons::Icon;
use leptos_router::components::A;
use reactive_stores::{Field, Store};
use uuid::Uuid;

#[component]
pub fn ThreadMenu(channel_id: Uuid, server_id: Uuid) -> impl IntoView {
    let open = RwSignal::new(false);
    let create_thread_node = NodeRef::<html::Div>::new();
    let context_menu_ref = NodeRef::<html::Div>::new();
    let delete_thread_modal_ref = NodeRef::<html::Div>::new();
    view! {
        <DropdownProvider open=open modal=false>
            <DropdownTrigger class="hover:bg-base-100 rounded-md h-7 w-7 cursor-pointer select-none flex items-center justify-center">
                <Icon icon=IconData::ListTree class="h-4 w-4"/>
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
                            // <Icon
                            //     icon=icondata::LuX
                            //     on:click=move |_| open.set(false)
                            // />
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
    let get_threads = Resource::new(move || (), move |_| get_threads_from_channel(channel_id));
    view! {
        <div class="w-full h-auto relative">
            <div class="flex w-full justify-between items-center my-2">
                <div class="h-6 w-[200px] border border-base-100 rounded p-1.5 flex items-center justify-between">
                    "Search"
                    <Icon icon=IconData::Search />
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
                <Transition>
                    {
                        Suspend::new(async move {
                            get_threads.await.map(|threads| {
                                let thread_store = Store::new(ThreadStore { threads });
                                // use_ws().on_server_msg(server_id, move |msg| {
                                //     match msg {
                                //         Message::ThreadCreated { thread } => {
                                //             thread_store.threads().update(|threads| {
                                //                 threads.push(thread);
                                //             });
                                //         },
                                //         Message::ThreadDeleted { thread_id } => {
                                //             thread_store.threads().update(|threads| {
                                //                 threads.retain(|thread| thread.id != thread_id);
                                //             });
                                //         },
                                //         _ => {}
                                //     }
                                // });
                                view!{
                                    <For
                                        each=move || thread_store.threads()
                                        key=|thread| thread.id().get()
                                        let:thread
                                    >
                                        <ThreadLink
                                            thread=thread
                                            open=open
                                            context_menu_ref=context_menu_ref
                                            delete_thread_modal_ref=delete_thread_modal_ref
                                        />
                                    </For>
                                }
                            })
                        })
                    }
                </Transition>
            </div>
        </div>
    }
}

#[component]
pub fn ThreadLink(
    #[prop(into)] thread: Field<Thread>,
    open: RwSignal<bool>,
    context_menu_ref: NodeRef<html::Div>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let created_by_profile = Resource::new(move || thread.created_by().get(), get_member_profile);
    let name = thread.name();
    let id = thread.id();
    view! {
        <Transition >
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
                                        href=format!("{}", id.get().simple())
                                        {..}
                                        class="w-full h-20 hover:bg-base-content/10 rounded-md inline-block flex justify-between items-center px-2"
                                    >
                                        <div class="flex flex-col w-auto h-auto">
                                            <div class="font-semibold text-lg">{move || name.get()}</div>
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
                                        <ThreadMembers thread_id=id.get() />
                                    </A>
                                </ContextMenuTrigger>
                                <ContextMenuContent
                                    ignore=vec![delete_thread_modal_ref]
                                    class="z-50 select-none"
                                >
                                    <ThreadMenuContent
                                        delete_thread_modal_ref=delete_thread_modal_ref
                                        thread=thread
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
    let thread_members = Resource::new(|| (), move |_| get_five_thread_members(thread_id));

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
                                    if let Some(image_url) = member.image_url
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
                                })
                                .collect_view()
                        })
                }}
            </div>
        </Transition>
    }
}
