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
use leptos::html;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;
use std::time::Duration;

use super::thread::Thread;

 
#[component]
pub fn Channel(channel: Channel) -> impl IntoView {
    let Channel { id: channel_id, .. } = channel;
    view! {
        <ChannelMenu channel=channel />
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
    let hidden = RwSignal::new(false);
    let use_current_channel = use_current_channel();
    let is_current_channel =
        move || use_current_channel.with(|current| current.is_some_and(|current| current == id));
    let stored_channel = StoredValue::new(channel);
    let invite_people_node = NodeRef::<html::Div>::new();
    let edit_channel_node = NodeRef::<html::Div>::new();
    let delete_channel_node = NodeRef::<html::Div>::new();
    let open = RwSignal::new(false);
    view! {
        <div class="relative py-px ml-2 group mt-0.5">
            <ContextMenuProvider hidden=hidden open=open modal=false>
                <ContextMenuTrigger class="relative box-border flex flex-col cursor-pointer">
                    <A
                        href=move || id.simple().to_string()
                        {..}
                        class=move || {
                            format!("relative flex group items-center py-1.5 px-2 rounded-lg {}", {
                                if is_current_channel() {
                                    "bg-base-content/5"
                                } else {
                                    "hover:bg-base-content/5"
                                }
                            })
                        }
                    >
                        <Icon
                            icon=Icon::from(channel_type)
                            // class="relative w-4 h-4 shrink-0 mr-1.5 fill-base-content"
                        />
                        <div
                            class="whitespace-nowrap overflow-hidden text-ellipsis mr-auto leading-5 flex-auto relative text-sm"
                        >
                            {name.clone()}
                        </div>
                    </A>
                    <div
                        on:click=move |_| {
                            open.set(true);
                        }
                        class=move || {
                            format!("absolute right-1 top-1.5 p-0.5 hover:bg-base-content/5 rounded {}", if is_current_channel() {
                                "opacity-100"
                            }else {
                                "opacity-0 group-hover:opacity-100"
                            })
                        }
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-ellipsis"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>
                    </div>
                </ContextMenuTrigger>

                <ContextMenuContent
                    ignore=vec![invite_people_node, edit_channel_node, delete_channel_node]
                    class="transition-all ease-out w-56 flex flex-col h-auto p-1 bg-base-400 z-40 rounded-md border border-base-100"
                >
                    <InvitePeopleModal
                        content_ref=invite_people_node
                        invite_code=server.invite_code
                        class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                        on_click=Signal::derive(move || hidden.set(false))
                    >
                        <div>"Invite People"</div>
                    </InvitePeopleModal>
                    {match member_can_edit {
                        true => {
                            view! {
                                <EditChannelModal
                                    content_ref=edit_channel_node
                                    channel=stored_channel.get_value()
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                    on_click=Signal::derive(move || hidden.set(false))
                                >
                                    <div>
                                        "Edit Channel"
                                    </div>
                                </EditChannelModal>
                                <DeleteChannel
                                    content_ref=delete_channel_node
                                    channel=stored_channel.get_value()
                                    server_id=server.id
                                    class="flex justify-between hover:bg-base-content/10 items-center w-full text-sm py-1.5 px-2 group rounded-sm"
                                    on_click=Signal::derive(move || hidden.set(false))
                                >
                                    <div>
                                        "Delete Channel"
                                    </div>
                                </DeleteChannel>
                            }
                                .into_any()
                        }
                        _ => view! {}.into_any(),
                    }}
                </ContextMenuContent>
            </ContextMenuProvider>
        </div>
    }
}

// {move || match (member_can_edit, is_current_channel()) {
//     (true, true) => {
//         view! {
//             <TooltipProvider delay_duration=Duration::new(0, 0)>
//                 <TooltipTrigger class="w-auto h-auto mr-0.5">
//                     <EditChannelModal
//                         channel=stored_channel.get_value()
//                         class="w-auto h-auto"
//                     >
//                         <Icon
//                             icon=icondata::RiSettings5SystemFill
//                             class="w-[18px] h-[18px] fill-base-content/50 hover:fill-base-content/75"
//                         />
//                     </EditChannelModal>
//                 </TooltipTrigger>
//                 <TooltipContent
//                     tip="Rename Channel".to_string()
//                     tooltip_side=ToolTipSide::Top
//                     tooltip_of_side=22.0
//                     class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]"
//                 />
//             </TooltipProvider>
//             <TooltipProvider delay_duration=Duration::new(0, 0)>
//                 <TooltipTrigger class="w-auto h-auto mr-0.5">
//                     <DeleteChannel
//                         channel=stored_channel.get_value()
//                         server_id=server.id
//                         class="w-auto h-auto"
//                     >
//                         <Icon
//                             icon=icondata::RiDeleteBinSystemFill
//                             class="w-[18px] h-[18px] fill-base-content/50 hover:fill-base-content/75"
//                         />
//                     </DeleteChannel>
//                 </TooltipTrigger>
//                 <TooltipContent
//                     tip="Delete Channel".to_string()
//                     tooltip_side=ToolTipSide::Top
//                     tooltip_of_side=22.0
//                     class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]"
//                 />
//             </TooltipProvider>
//         }
//             .into_view()
//     }
//     (true, false) => {
//         view! {
//             <TooltipProvider delay_duration=Duration::new(0, 0)>
//                 <TooltipTrigger class="w-auto h-auto mr-0.5">
//                     <EditChannelModal
//                         channel=stored_channel.get_value()
//                         class="w-[18px] h-[18px] group-hover:flex hidden"
//                     >
//                         <Icon
//                             icon=icondata::RiSettings5SystemFill
//                             class="w-[18px] h-[18px] group-hover:fill-base-content/50"
//                         />
//                     </EditChannelModal>
//                 </TooltipTrigger>
//                 <TooltipContent
//                     tip="Rename Channel".to_string()
//                     tooltip_side=ToolTipSide::Top
//                     tooltip_of_side=22.0
//                     class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]"
//                 />
//             </TooltipProvider>
//             <TooltipProvider delay_duration=Duration::new(0, 0)>
//                 <TooltipTrigger class="w-auto h-auto mr-0.5">
//                     <DeleteChannel
//                         channel=stored_channel.get_value()
//                         server_id=server.id
//                         class="w-auto h-auto"
//                     >
//                         <Icon
//                             icon=icondata::RiDeleteBinSystemFill
//                             class="w-[18px] h-[18px] group-hover:block group-hover:fill-base-content/50 hidden"
//                         />
//                     </DeleteChannel>
//                 </TooltipTrigger>
//                 <TooltipContent
//                     tip="Delete Channel".to_string()
//                     tooltip_side=ToolTipSide::Top
//                     tooltip_of_side=22.0
//                     class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-t-[#dfdfe2] dark:after:border-t-[#0d0d0d]"
//                 />
//             </TooltipProvider>
//         }
//             .into_view()
//     }
//     _ => view! {}.into_view(),
// }}
