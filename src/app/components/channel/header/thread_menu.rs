use crate::app::api::member::{get_member_name_and_url, get_thread_members};
use crate::app::api::thread::{get_threads_from_channel, use_thread};
use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::components::modal::create_thread::CreatethreadModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::dropdown_menu::*;
use crate::entities::thread::Thread;
use icondata;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;
use uuid::Uuid;

#[component]
pub fn ThreadMenu(channel_id: Uuid, server_id: Uuid) -> impl IntoView {
    let open = create_rw_signal(false);
    view! {
        <DropdownProvider open=open modal=false >
            <DropdownTrigger>
                <Icon icon=icondata::RiDiscussCommunicationFill class="w-6 h-6 fill-base-content/40" />
            </DropdownTrigger>
            <DropdownContent side=MenuSide::Bottom align=MenuAlign::End class="w-auto h-auto z-40">
                <div class="w-[510px] h-auto bg-base-200 flex flex-col rounded-md">
                    <div class="w-full h-[48px] bg-base-300 flex items-center px-2 rounded-t-md">
                        <Icon icon=icondata::RiDiscussCommunicationFill class="w-7 h-7 fill-base-content/40 mr-2" />
                        <div class="font-bold test-base-content/40">"Threads"</div>
                        <div class="divider divider-horizontal h-6 mx-2 my-auto"></div>
                        <div class="h-6 w-[200px] mr-auto bg-base-100/60 rounded-md flex items-center">
                            <div class="ml-1 text-base-content/60">"Search For Thread Name"</div>
                        </div>
                        <CreatethreadModal channel_id=channel_id server_id=server_id class="w-auto h-6 rounded-md bg-primary px-2 flex items-center">
                            <div class="my-auto text-primary-content">"Create"</div>
                        </CreatethreadModal>
                        <Icon icon=icondata::RiCloseSystemLine class="w-7 h-7 fill-base-content ml-[10px]" on:click=move|_|open.set(false)/>
                    </div>
                    <ActiveThreads channel_id=channel_id open=open/>
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn ActiveThreads(channel_id: Uuid, open: RwSignal<bool>) -> impl IntoView {
    let thread_context = use_thread();
    let delete_thread = thread_context.delete_thread;
    let get_threads = create_resource(
        move || {
            (
                delete_thread.version().get(),
                thread_context.create_thread.version().get(),
            )
        },
        move |_| get_threads_from_channel(channel_id),
    );
    view! {
        <div class="w-full h-auto px-4">
            <div class="font-bold text my-3 text-base-content/80">"More Active Threads"</div>
            <div class="w-full min-h-[342px] w-full max-h-[720px] overflow-y-scroll overflow-x-hidden space-y-2">
                <Transition fallback=move || ()>
                    {
                        move || {
                            get_threads.and_then(|threads|
                                threads.iter().map(|thread| {
                                    view!{<ThreadLink thread=thread.clone() open=open/>}
                                }).collect_view()
                            )
                        }
                    }
                </Transition>
            </div>
        </div>
    }
}

#[component]
pub fn ThreadLink(thread: Thread, open: RwSignal<bool>) -> impl IntoView {
    let created_by_data =
        create_resource(|| (), move |_| get_member_name_and_url(thread.created_by));
    let name = store_value(thread.name.to_string());
    let thread_id = thread.id;
    let thread = store_value(thread);
    view! {
        <Transition fallback=move || ()>
            {
                move || {
                    created_by_data.and_then(|(created_by, member_url)| {
                        let created_by = store_value(created_by.clone());
                        let member_url = store_value(member_url.clone());
                        view!{
                            <ContextMenuProvider modal=false >
                                <ContextMenuTrigger class="w-full h-auto">
                                    <A on:click=move |_| open.set(false) href=format!("{}",thread_id.simple()) class="w-full h-20 bg-base-300 rounded-md inline-block flex justify-between items-center px-2">
                                        <div class="flex flex-col w-auto h-auto">
                                            <div class="font-bold">{name.get_value()}</div>
                                            <div class="flex items-center">
                                                {
                                                    if let Some(url) = member_url.get_value() {
                                                        view! {
                                                            <img class="w-4 h-4 object-cover rounded-full mr-1" src=url/>
                                                        }.into_view()
                                                    } else {
                                                        view! {
                                                            <div class="w-4 h-4 rounded-full bg-white border-base-200 mr-1"/>
                                                        }.into_view()
                                                    }
                                                }
                                                {format!("Started by {}", created_by.get_value())}
                                            </div>
                                        </div>
                                        <ThreadMembers thread_id=thread_id/>
                                    </A>
                                </ContextMenuTrigger>
                                <ContextMenuContent  class="z-50".into()>
                                    <ThreadMenuContent thread=thread.get_value() open=open />
                                </ContextMenuContent>
                            </ContextMenuProvider>
                        }
                    })
                }
            }
        </Transition>
    }
}

#[component]
pub fn ThreadMembers(thread_id: Uuid) -> impl IntoView {
    let thread_members = create_resource(|| (), move |_| get_thread_members(thread_id));

    view! {
        <Transition fallback=move || ()>
            <div class="-space-x-2">
                {
                    move || {
                        thread_members.and_then(|thread_members| {
                            thread_members.iter().map(|member| {
                                view!{
                                    {
                                        if let Some(url) = &member.image_url {
                                            view! {
                                                <img class="w-4 h-4 object-cover rounded-full mr-1" src=url/>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <div class="w-4 h-4 rounded-full bg-white border-base-200 mr-1"/>
                                            }.into_view()
                                        }
                                    }
                                }
                            }).collect_view()
                        })
                    }
                }
            </div>
        </Transition>
    }
}
