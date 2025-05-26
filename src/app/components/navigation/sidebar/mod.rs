mod inbox;
mod profile;
mod search;
mod servers;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::tool_tip::*;
use leptos::prelude::*;
use leptos_router::components::A;
use std::time::Duration;

use self::inbox::Inbox;
use self::profile::Profile;
use self::search::Search;
use self::servers::Servers;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="w-full h-full flex flex-col items-center py-1 bg-base-300 justify-between">
            <div class="flex w-full grow flex-col items-center scrollbar-none overflow-y-scroll overflow-x-hidden space-y-1">
                <Search />
                <TooltipProvider delay_duration=Duration::new(0, 0)>
                    <TooltipTrigger class="relative">
                        <A href="me" {..} class=" flex relative items-center">
                            <div class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer">
                                <Icon icon=IconData::MessageCircle class="h-4 w-4" />
                            </div>
                        </A>
                    </TooltipTrigger>
                    <TooltipContent
                        tooltip_of_side=10.0
                        tip="Direct Messages"
                        arrow=true
                        class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                    />
                </TooltipProvider>
                <Inbox />
                <Servers />
            </div>
            <Profile />
        </div>
    }
}
