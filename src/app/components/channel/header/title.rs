use crate::app::api::channel::{get_channel_topic, use_channel};
use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::channel::Channel;
use crate::entities::thread::Thread;
use icondata::Icon;
use leptos::{html, prelude::*};
use leptos_icons::Icon;
use leptos_router::components::A;
use uuid::Uuid;

#[component]
pub fn HeaderTitle(channel: Channel, #[prop(optional)] thread: Option<Thread>) -> impl IntoView {
    let hidden = RwSignal::new(false);
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();
    let channel_name = channel.name.clone();
    let thread = thread.map(StoredValue::new);
    let edit_channel_node = NodeRef::<html::Div>::new();
    let delete_channel_node = NodeRef::<html::Div>::new();
    view! {
        <ContextMenuProvider modal=false hidden=hidden>
            <ContextMenuTrigger class="relative flex items-center p-1.5 text-base select-none">
                <Icon icon=Icon::from(channel.channel_type) />
                <div>{channel_name}</div>
                <ChannelTopic channel_id=channel.id />
                {thread.map(|thread| view! { <ChannelThread thread=thread.get_value() /> })}
            </ContextMenuTrigger>

            <ContextMenuContent
                ignore=vec![edit_channel_node, delete_channel_node]
                class="z-40 select-none"
            >
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <InvitePeopleModal
                        invite_code=server.invite_code
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on_click=Signal::derive(move || hidden.set(false))
                    >
                        <div>"Invite People"</div>
                    </InvitePeopleModal>
                    {
                        member_can_edit.then(||
                            view! {
                                <EditChannelModal
                                    channel=channel.clone()
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                    on_click=Signal::derive(move || hidden.set(false))
                                >
                                    <div>"Edit Channel"</div>
                                </EditChannelModal>
                                <DeleteChannel
                                    channel=channel.clone()
                                    server_id=server.id
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                    on_click=Signal::derive(move || hidden.set(false))
                                >
                                    <div>"Delete Channel"</div>
                                </DeleteChannel>
                            }
                        )
                    }
                    {thread
                        .map(|thread| {
                            let thread = thread.get_value();
                            view! {
                                <A
                                    href=move || {
                                        format!(
                                            "/servers/{}/{}/{}",
                                            server.id.simple(),
                                            thread.channel_id.simple(),
                                            thread.id.simple(),
                                        )
                                    }
                                    on:click=move |_| hidden.set(false)
                                    {..}
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                >
                                    "Open Split View"
                                </A>
                            }
                        })}
                </div>
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}

#[component]
pub fn ChannelTopic(channel_id: Uuid) -> impl IntoView {
    let channel_topic = Resource::new(
        move || (use_channel().update_channel.version().get()),
        move |_| get_channel_topic(channel_id),
    );
    view! {
        <Transition fallback=move || ()>
            {move || {
                channel_topic
                    .and_then(|topic| {
                        topic
                            .clone()
                            .map(|topic| {
                                view! {
                                    <div class="divider divider-horizontal h-auto mx-0.5" />
                                    <div>{topic}</div>
                                }
                            })
                    })
            }}
        </Transition>
    }
}

#[component]
pub fn ChannelThread(thread: Thread) -> impl IntoView {
    view! {
        <Icon icon=icondata::LuChevronRight />
        <div>{thread.name}</div>
    }
}
