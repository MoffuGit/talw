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
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;
use uuid::Uuid;

#[component]
pub fn ThreadMenu(channel_id: Uuid, server_id: Uuid) -> impl IntoView {
    let open = create_rw_signal(false);
    let create_thread_node = create_node_ref::<html::Div>();
    let context_menu_ref = create_node_ref::<html::Div>();
    let delete_thread_modal_ref = create_node_ref::<html::Div>();
    view! {
        <DropdownProvider open=open modal=false >
            <DropdownTrigger>
                <Icon icon=icondata::RiDiscussCommunicationFill class="w-6 h-6 fill-base-content/40" />
            </DropdownTrigger>
            <DropdownContent ignore=vec![context_menu_ref, create_thread_node, delete_thread_modal_ref] side=MenuSide::Bottom align=MenuAlign::End class="w-auto h-auto z-40">
                <div class="w-[510px] h-auto bg-base-200 flex flex-col rounded-md">
                    <div class="w-full h-[48px] bg-base-300 flex items-center px-2 rounded-t-md">
                        <Icon icon=icondata::RiDiscussCommunicationFill class="w-7 h-7 fill-base-content/40 mr-2" />
                        <div class="font-bold test-base-content/40">"Threads"</div>
                        <div class="divider divider-horizontal h-6 mx-2 my-auto"></div>
                        <div class="h-6 w-[200px] mr-auto bg-base-100/60 rounded-md flex items-center">
                            <div class="ml-1 text-base-content/60">"Search For Thread Name"</div>
                        </div>
                        <CreatethreadModal content_ref=create_thread_node channel_id=channel_id server_id=server_id class="w-auto h-6 rounded-md bg-primary px-2 flex items-center">
                            <div class="my-auto text-primary-content">"Create"</div>
                        </CreatethreadModal>
                        <Icon icon=icondata::RiCloseSystemLine class="w-7 h-7 fill-base-content ml-[10px]" on:click=move|_|open.set(false)/>
                    </div>
                    <ActiveThreads channel_id=channel_id open=open context_menu_ref=context_menu_ref delete_thread_modal_ref=delete_thread_modal_ref/>
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
) -> impl IntoView {
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
                                    view!{<ThreadLink thread=thread.clone() open=open context_menu_ref=context_menu_ref delete_thread_modal_ref=delete_thread_modal_ref/>}
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
pub fn ThreadLink(
    thread: Thread,
    open: RwSignal<bool>,
    context_menu_ref: NodeRef<html::Div>,
    delete_thread_modal_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let created_by_profile = create_resource(|| (), move |_| get_member_profile(thread.created_by));
    let name = store_value(thread.name.to_string());
    let thread_id = thread.id;
    let thread = store_value(thread);
    view! {
        <Transition fallback=move || ()>
            {
                move || {
                    created_by_profile.and_then(|profile| {
                        //NOTE: only for now
                        let profile = profile.as_ref().expect("should have a profile");
                        let created_by = store_value(profile.name.clone());
                        let member_url = store_value(profile.image_url.clone());
                        view!{
                            <ContextMenuProvider modal=false content_ref=context_menu_ref>
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
                                <ContextMenuContent ignore=vec![delete_thread_modal_ref]  class="z-50".into()>
                                    <ThreadMenuContent delete_thread_modal_ref=delete_thread_modal_ref thread=thread.get_value() open=open />
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
                            thread_members.clone().into_iter().map(|member| {
                                let member_profile = create_resource(|| (), move |_| get_user_profile(member.user_id));
                                view!{
                                    {
                                        move || if let Some(Ok(Profile { image_url ,..})) = member_profile.get() {
                                            view! {
                                                <img class="w-4 h-4 object-cover rounded-full mr-1" src=image_url/>
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
