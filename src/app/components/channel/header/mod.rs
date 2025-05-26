mod pinned;
pub mod thread_menu;
pub mod title;

use crate::app::components::channel::header::title::HeaderTitle;
use crate::app::components::channel::sidebars::MemberSideBarTrigger;
use crate::app::components::navigation::server::sidebar::ServerSideBarTrigger;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::tool_tip::*;
use crate::entities::channel::{Channel, ChannelStoreFields};
use crate::entities::thread::Thread;
use leptos::prelude::*;
use reactive_stores::Field;

use self::pinned::Pinned;
use self::thread_menu::ThreadMenu;

#[component]
pub fn ChannelHeader(
    #[prop(into)] channel: Field<Channel>,
    #[prop(optional, into)] thread: Option<Field<Thread>>,
) -> impl IntoView {
    view! {
        <div class="relative bg-base-300 shadow shadow-base-300/80 h-11 w-full flex justify-between align-middle">
            <div class="relative flex-auto flex items-center overflow-hidden py-2">
                <ServerSideBarTrigger class="relative flex items-center justify-center ml-1 mr-1 h-7 w-7 hover:bg-base-100 rounded-md cursor-pointer">
                    <Icon icon=IconData::PanelLeft class="w-4 h-4"/>
                </ServerSideBarTrigger>
                <div class="w-0.5 h-5 mx-0.5 bg-base-content"/>
                <HeaderTitle channel=channel thread=thread />

            </div>
            <div class="h-auto relative flex items-center px-2 space-x-1">
                {thread
                    .is_none()
                    .then(|| {
                        view! {
                            <TooltipProvider>
                                <TooltipTrigger close_on_click=true>
                                    <ThreadMenu channel_id=channel.id().get() server_id=channel.server_id().get() />
                                </TooltipTrigger>
                                <TooltipContent
                                    tooltip_of_side=10.0
                                    tip="Threads"
                                    arrow=true
                                    class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                                    tooltip_side=ToolTipSide::Bottom
                                />
                            </TooltipProvider>
                        }
                    })
                }
                <TooltipProvider>
                    <TooltipTrigger
                        close_on_click=true
                    >
                        <Pinned channel_id=channel.id()/>
                    </TooltipTrigger>
                    <TooltipContent
                        tip="Pinned Messages"
                        tooltip_of_side=10.0
                        arrow=true
                        class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                        tooltip_side=ToolTipSide::Bottom
                    />
                </TooltipProvider>
                <MemberSideBarTrigger />
                <div class="relative flex rounded-md p-1 text-center cursor-pointer h-7 border border-base-100 bg-base-200 w-42">
                    <span class="text-sm">"Search"</span>
                    <Icon icon=IconData::Search class="w-4 h-4 absolute right-1" />
                </div>
            </div>
        </div>
    }
}
