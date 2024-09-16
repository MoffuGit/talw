use crate::app::api::channel::{get_channel_topic, use_channel};
use crate::app::api::thread::get_thread;
use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::routes::servers::server::{use_current_server_context, CurrentServerContext};
use crate::entities::channel::Channel;
use icondata::Icon;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_router;
use leptos_router::Redirect;
use std::str::FromStr;
use uuid::Uuid;

#[component]
#[allow(non_snake_case)]
pub fn HeaderTitle(channel: Channel) -> impl IntoView {
    let open = create_rw_signal(false);
    let CurrentServerContext {
        server,
        member_can_edit,
    } = use_current_server_context();
    let channel_name = channel.name.clone();
    view! {
        <ContextMenuProvider modal=false open=open>
            <ContextMenuTrigger class="relative flex flex-row group items-center py-[6px] px-2 text-base">
                <Icon icon=Icon::from(channel.channel_type) class="w-6 h-6 mx-2"/>
                <div>
                    {channel_name}
                </div>
                <ChannelTopic channel_id=channel.id/>
                <ChannelThread channel_id=channel.id server_id=channel.server_id/>
            </ContextMenuTrigger>

            <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                <InvitePeopleModal invite_code=server.invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                    <div class="group-hover:text-primary-content">"Invite People"</div>
                </InvitePeopleModal>
                {
                    match member_can_edit {
                        true => view! {
                            <EditChannelModal channel=channel.clone() class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"  on_click=Signal::derive(move || open.set(false))>
                                <div class="group-hover:text-primary-content">"Edit Channel"</div>
                            </EditChannelModal>
                            <DeleteChannel channel=channel.clone() server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                                <div class="group-hover:text-primary-content">"Delete Channel"</div>
                            </DeleteChannel>
                        }.into_view(),
                        false => view! {}.into_view(),
                    }
                }
            </ContextMenuContent>
        </ContextMenuProvider>
    }
}

#[component]
pub fn ChannelTopic(channel_id: Uuid) -> impl IntoView {
    let channel_topic = create_resource(
        move || (use_channel().update_channel.version().get()),
        move |_| get_channel_topic(channel_id),
    );
    view! {
        <Transition fallback=move || ()>
            {
                move || {
                    channel_topic.and_then(|topic| topic.clone().map(|topic| {
                        view!{
                            <div class="divider divider-horizontal h-auto mx-0.5"/>
                            <div>{topic}</div>
                        }
                    }))
                }
            }
        </Transition>
    }
}

#[component]
pub fn ChannelThread(channel_id: Uuid, server_id: Uuid) -> impl IntoView {
    let current_thread = move || {
        use_router()
            .pathname()
            .with(|path| path.split('/').nth(4).map(Uuid::from_str))
    };
    view! {
        {
            move || {
                match current_thread() {
                    Some(Ok(thread_id)) => {
                        let current_thread = create_resource(move || (), move |_| get_thread(thread_id));
                        view!{
                            <Transition fallback=move || ()>
                                {
                                    move || {
                                        match current_thread.get() {
                                            None => ().into_view(),
                                            Some(Err(_)) => view!{<Redirect path=format!("/servers/{}/{}", server_id, channel_id)/>}.into_view(),
                                            Some(Ok(thread)) => view!{
                                                <Icon icon=icondata::RiArrowRightSArrowsLine class="w-4 h-4 mx-2"/>
                                                <div>{thread.name}</div>
                                            }.into_view()
                                        }
                                    }
                                }
                            </Transition>
                       }.into_view()
                    },
                    Some(Err(_)) => view!{<Redirect path=format!("/servers/{}/{}", server_id, channel_id)/>}.into_view(),
                    None => ().into_view()
                }
            }
        }
    }
}
