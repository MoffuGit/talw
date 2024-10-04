use super::use_current_channel;
use crate::app::components::modal::delete_channel::DeleteChannel;
use crate::app::components::modal::edit_channel::EditChannelModal;
use crate::app::components::modal::invite_people::InvitePeopleModal;
use crate::app::components::ui::context_menu::*;
use crate::app::components::ui::tool_tip::*;
use crate::app::routes::servers::server::use_current_server_context;
use crate::app::routes::servers::server::CurrentServerContext;
use crate::entities::channel::Channel;
use icondata;
use icondata::Icon;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;
use std::time::Duration;

use super::thread::Thread;

#[allow(non_snake_case)]
#[component]
pub fn Channel(channel: Channel) -> impl IntoView {
    let Channel { id: channel_id, .. } = channel;
    view! {
        <ChannelMenu channel=channel/>
        <Thread channel_id=channel_id />
    }
}

#[component]
pub fn ChannelMenu(channel: Channel) -> impl IntoView {
    let CurrentServerContext {
        server,
        member_can_edit,
        ..
    } = use_current_server_context();
    let Channel {
        id,
        name,
        channel_type,
        ..
    } = channel.clone();
    let open = create_rw_signal(false);
    let use_current_channel = use_current_channel();
    let is_current_channel =
        move || use_current_channel.with(|current| current.is_some_and(|current| current == id));
    let stored_channel = store_value(channel);
    view! {
        <div class=move || format!("relative py-[1px] ml-2 transition duration-200 ease-in-out delay-0 group rounded hover:bg-primary/75 mt-0.5 {}", match is_current_channel() {
            true => "bg-primary/50",
            false => "",
        })>
            <A href=move || id.simple().to_string() class="relative box-border flex flex-col cursor-pointer">
                <ContextMenuProvider open=open modal=false>
                    <ContextMenuTrigger class="relative flex flex-row group items-center py-[6px] px-2">
                        <Icon icon=Icon::from(channel_type) class="relative w-[18px] h-[18px] shrink-0 mr-[6px] fill-base-content"/>
                        <div class=move || format!("whitespace-nowrap overflow-hidden text-ellipsis text-[16px] mr-auto font-bold text-base-content/50 leading-5 flex-auto relative group-hover:text-base-content/75 {}", match is_current_channel() {
                            true => "text-base-content/60",
                            false => ""
                        })>
                            {&name.clone()}
                        </div>
                        {
                            move || match (member_can_edit, is_current_channel()) {
                                (true, true) => view! {
                                <TooltipProvider delay_duration=Duration::new(0,0)>
                                    <TooltipTrigger class="w-auto h-auto mr-0.5">
                                        <EditChannelModal channel=stored_channel.get_value() class="w-auto h-auto">
                                            <Icon icon=icondata::RiSettings5SystemFill class="w-[18px] h-[18px] fill-base-content/50 hover:fill-base-content/75"/>
                                        </EditChannelModal>
                                    </TooltipTrigger>
                                    <TooltipContent tip="Rename Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                </TooltipProvider>
                                <TooltipProvider delay_duration=Duration::new(0,0)>
                                    <TooltipTrigger class="w-auto h-auto mr-0.5">
                                        <DeleteChannel channel=stored_channel.get_value() server_id=server.id class="w-auto h-auto">
                                            <Icon icon=icondata::RiDeleteBinSystemFill class="w-[18px] h-[18px] fill-base-content/50 hover:fill-base-content/75"/>
                                        </DeleteChannel>
                                    </TooltipTrigger>
                                    <TooltipContent tip="Delete Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                </TooltipProvider>
                                }.into_view(),
                                (true, false) => view! {
                                <TooltipProvider delay_duration=Duration::new(0,0)>
                                    <TooltipTrigger class="w-auto h-auto mr-0.5">
                                        <EditChannelModal channel=stored_channel.get_value() class="w-[18px] h-[18px] group-hover:flex hidden">
                                            <Icon icon=icondata::RiSettings5SystemFill class="w-[18px] h-[18px] group-hover:fill-base-content/50"/>
                                        </EditChannelModal>
                                    </TooltipTrigger>
                                    <TooltipContent tip="Rename Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                </TooltipProvider>
                                <TooltipProvider delay_duration=Duration::new(0,0)>
                                    <TooltipTrigger class="w-auto h-auto mr-0.5">
                                        <DeleteChannel channel=stored_channel.get_value() server_id=server.id class="w-auto h-auto">
                                            <Icon icon=icondata::RiDeleteBinSystemFill class="w-[18px] h-[18px] group-hover:block group-hover:fill-base-content/50 hidden"/>
                                        </DeleteChannel>
                                    </TooltipTrigger>
                                    <TooltipContent tip="Delete Channel".to_string() tooltip_side=ToolTipSide::Top tooltip_of_side=22.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]" />
                                </TooltipProvider>
                                }.into_view(),
                                _ => view! {}.into_view(),
                            }
                        }

                    </ContextMenuTrigger>

                    <ContextMenuContent class="transition-all ease-out w-[188px] flex flex-col h-auto py-[6px] px-2 bg-[#dfdfe2] dark:bg-[#0d0d0d] rounded z-40".to_string()>
                        <InvitePeopleModal invite_code=server.invite_code class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                            <div class="group-hover:text-primary-content">"Invite People"</div>
                        </InvitePeopleModal>
                        {
                            match member_can_edit {
                                true => view! {
                                    <EditChannelModal channel=stored_channel.get_value() class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded"  on_click=Signal::derive(move || open.set(false))>
                                        <div class="group-hover:text-primary-content">"Edit Channel"</div>
                                    </EditChannelModal>
                                    <DeleteChannel channel=stored_channel.get_value() server_id=server.id class="flex justify-between hover:bg-primary items-center w-full text-sm py-[6px] px-2 my-0.5 group rounded" on_click=Signal::derive(move || open.set(false))>
                                        <div class="group-hover:text-primary-content">"Delete Channel"</div>
                                    </DeleteChannel>
                                }.into_view(),
                                _ => view! {}.into_view(),
                            }
                        }
                    </ContextMenuContent>
                </ContextMenuProvider>
            </A>
        </div>
    }
}
