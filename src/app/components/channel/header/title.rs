use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::channel::{Channel, ChannelStoreFields};
use crate::entities::server::ServerStoreFields;
use crate::entities::thread::{Thread, ThreadStoreFields};
//use icondata:Icon;
use leptos::{html, prelude::*};
//use leptos_icons::Icon;
use leptos_router::components::A;
use reactive_stores::Field;

#[component]
pub fn HeaderTitle(
    #[prop(into)] channel: Field<Channel>,
    #[prop(into)] thread: Option<Field<Thread>>,
) -> impl IntoView {
    let hidden = RwSignal::new(false);
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();
    let name = channel.name();
    let edit_channel_node = NodeRef::<html::Div>::new();
    let delete_channel_node = NodeRef::<html::Div>::new();
    view! {
        <ContextMenuProvider modal=false hidden=hidden>
            <ContextMenuTrigger class="relative flex items-center p-1.5 text-sm select-none">
                // <Icon icon=Icon::from(channel.channel_type) />
                <div>{move || name.get()}</div>
                <ChannelTopic topic=channel.topic() />
                {thread.map(|thread| view! { <ChannelThread thread=thread /> })}
            </ContextMenuTrigger>

            <ContextMenuContent
                ignore=vec![edit_channel_node, delete_channel_node]
                class="z-40 select-none"
            >
                <div class="w-56 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100 origin-left starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <InvitePeopleModal
                        invite_code=server.invite_code()
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on_click=Signal::derive(move || hidden.set(false))
                    >
                        <div>"Invite People"</div>
                    </InvitePeopleModal>
                    {
                        member_can_edit.then(||
                            view! {
                                <EditChannelModal
                                    channel=channel
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                    on_click=Signal::derive(move || hidden.set(false))
                                >
                                    <div>"Edit Channel"</div>
                                </EditChannelModal>
                                <DeleteChannel
                                    channel=channel
                                    server_id=server.id()
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
pub fn ChannelTopic(#[prop(into)] topic: Signal<Option<String>>) -> impl IntoView {
    view! {
        {move || {
            topic.get().map(|topic| {
                view! {
                    <div class="divider divider-horizontal h-auto mx-0.5" />
                    <div>{topic}</div>
                }

            })
        }}
    }
}

#[component]
pub fn ChannelThread(#[prop(into)] thread: Field<Thread>) -> impl IntoView {
    view! {
        // <Icon icon=icondata::LuChevronRight />
        <div>{move || thread.name().get()}</div>
    }
}
