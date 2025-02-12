pub mod thread_menu;
pub mod title;

use crate::app::components::channel::header::title::HeaderTitle;
use crate::app::components::channel::sidebars::MemberSideBarTrigger;
use crate::app::components::navigation::server::sidebar::ServerSideBarTrigger;
use crate::app::components::ui::tool_tip::*;
use crate::entities::channel::Channel;
use crate::entities::thread::Thread;
use icondata;
use leptos::prelude::*;
use leptos_icons::Icon;

use self::thread_menu::ThreadMenu;

#[allow(non_snake_case)]
#[component]
pub fn ChannelHeader(channel: Channel, #[prop(optional)] thread: Option<Thread>) -> impl IntoView {
    view! {
        <div class="relative bg-base-300 shadow shadow-base-300/80 h-11 w-full flex justify-between align-middle">
            <div class="relative flex-auto flex items-center overflow-hidden py-2">
                <ServerSideBarTrigger class="relative flex items-center ml-3 mr-1 hover:bg-base-content/5 rounded-lg p-1 cursor-pointer">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-panel-left">
                        <rect width="18" height="18" x="3" y="3" rx="2"/>
                        <path d="M9 3v18"/>
                    </svg>
                </ServerSideBarTrigger>

                <div class="divider divider-horizontal h-auto m-0"/>

                {match &thread {
                    Some(thread) => {
                        view! { <HeaderTitle channel=channel.clone() thread=thread.clone() /> }
                    }
                    None => view! { <HeaderTitle channel=channel.clone() /> },
                }}

            </div>
            <div class="h-auto relative flex items-center px-4 space-x-2.5">
                {if thread.is_none() {
                    view! {
                        <TooltipProvider>
                            <TooltipTrigger
                                close_on_click=true
                            >
                                <ThreadMenu channel_id=channel.id server_id=channel.server_id />
                            </TooltipTrigger>
                            <TooltipContent
                                tooltip_of_side=10.0
                                tip="Threads"
                                arrow=true
                                class="rounded-lg w-auto h-auto py-1 px-2 text-sm bg-base-400 border-base-400"
                                tooltip_side=ToolTipSide::Bottom
                            />
                        </TooltipProvider>
                    }.into_any()
                } else {
                    ().into_any()
                }}
                <TooltipProvider>
                    <TooltipTrigger
                        close_on_click=true
                        class="hover:bg-base-content/5 rounded-lg p-1 cursor-pointer"
                    >
                         <Icon icon=icondata::LuPin /* class="w-5 h-5" */ />
                    </TooltipTrigger>
                    <TooltipContent
                        tip="Pinned Messages"
                        tooltip_of_side=10.0
                        arrow=true
                        class="rounded-lg w-auto h-auto py-1 px-2 text-sm bg-base-400 border-base-400"
                        tooltip_side=ToolTipSide::Bottom
                    />
                </TooltipProvider>
                <MemberSideBarTrigger />
                <TooltipProvider>
                    <TooltipTrigger
                        close_on_click=true
                        class="hover:bg-base-content/5 rounded-lg p-1 cursor-pointer"
                    >
                        <Icon
                            icon=icondata::LuSearch
                            // class="w-5 h-5"
                        />
                    </TooltipTrigger>
                    <TooltipContent
                        tooltip_of_side=10.0
                        tip="Search"
                        arrow=true
                        class="rounded-lg w-auto h-auto py-1 px-2 text-sm bg-base-400 border-base-400"
                        tooltip_side=ToolTipSide::Bottom
                    />
                </TooltipProvider>
            </div>
        </div>
    }
}
