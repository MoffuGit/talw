use super::use_current_channel;
use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use crate::entities::channel::Channel;
use crate::entities::channel::ChannelStoreFields;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
//use icondata;
//use icondata:Icon;
use leptos::html;
use leptos::prelude::*;
//use leptos_icons::Icon;
use leptos_router::components::A;
use reactive_stores::Field;

use super::thread::Thread;

#[component]
pub fn Channel(#[prop(into)] channel: Field<Channel>) -> impl IntoView {
    let ws = use_ws();
    ws.on_server_msg(channel.server_id().get(), move |msg| {
        if let Message::ChannelUpdated {
            topic,
            name,
            channel_id,
        } = msg
        {
            if channel_id == channel.id().get() {
                if let Some(topic) = topic {
                    *channel.topic().write() = Some(topic)
                }

                if let Some(name) = name {
                    *channel.name().write() = name
                }
            }
        }
    });
    view! {
        <ChannelMenu channel=channel />
        <Thread channel_id=channel.id().get() server_id=channel.server_id().get() />
    }
}

#[component]
pub fn ChannelMenu(#[prop(into)] channel: Field<Channel>) -> impl IntoView {
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();

    let id = channel.id();
    let name = channel.name();

    let hidden = RwSignal::new(false);
    let use_current_channel = use_current_channel();
    let is_current_channel = move || {
        use_current_channel.with(|current| current.is_some_and(|current| current == id.get()))
    };
    let invite_people_node = NodeRef::<html::Div>::new();
    let edit_channel_node = NodeRef::<html::Div>::new();
    let delete_channel_node = NodeRef::<html::Div>::new();
    let open = RwSignal::new(false);
    view! {
        <div class="relative py-px ml-2 group mt-0.5">
            <ContextMenuProvider hidden=hidden open=open modal=false>
                <ContextMenuTrigger class="relative box-border flex flex-col cursor-pointer">
                    <A
                        href=move || id.get().simple().to_string()
                        {..}
                        class=move || {
                            format!(
                                "relative flex group items-center py-1.5 px-2 rounded-md {}",
                                {
                                    if is_current_channel() {
                                        "bg-base-100"
                                    } else {
                                        "hover:bg-base-100"
                                    }
                                },
                            )
                        }
                    >
                        // <Icon icon=Icon::from(channel_type) />
                        // class="relative w-4 h-4 shrink-0 mr-1.5 fill-base-content"
                        <div class="whitespace-nowrap overflow-hidden text-ellipsis mr-auto leading-5 flex-auto relative text-sm">
                            {move || name.get()}
                        </div>
                    </A>
                    <div
                        on:click=move |_| {
                            open.set(true);
                        }
                        class=move || {
                            format!(
                                "absolute right-1 top-1.5 p-0.5 rounded {}",
                                if is_current_channel() {
                                    "opacity-100 hover:bg-base-content/5"
                                } else {
                                    "opacity-0 group-hover:opacity-100 hover:bg-base-100"
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
                </ContextMenuTrigger>

                <ContextMenuContent
                    ignore=vec![invite_people_node, edit_channel_node, delete_channel_node]
                    class="select-none z-40"
                >
                    <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                        <InvitePeopleModal
                            content_ref=invite_people_node
                            invite_code=server.invite_code()
                            class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                            on_click=Signal::derive(move || hidden.set(false))
                        >
                            <div>"Invite People"</div>
                        </InvitePeopleModal>
                        {member_can_edit
                            .then(|| {
                                view! {
                                    <EditChannelModal
                                        content_ref=edit_channel_node
                                        channel=channel
                                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        on_click=Signal::derive(move || hidden.set(false))
                                    >
                                        <div>"Edit Channel"</div>
                                    </EditChannelModal>
                                    <DeleteChannel
                                        content_ref=delete_channel_node
                                        channel=channel
                                        server_id=server.id()
                                        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
                                        on_click=Signal::derive(move || hidden.set(false))
                                    >
                                        <div>"Delete Channel"</div>
                                    </DeleteChannel>
                                }
                            })}
                    </div>
                </ContextMenuContent>
            </ContextMenuProvider>
        </div>
    }
}
