mod inbox;
mod profile;
mod search;
mod servers;
use crate::app::components::theme::{ThemeIcons, Toggle_Theme};
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::prelude::*;
use leptos_icons::*;
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
            <div class="flex w-full grow flex-col items-center scrollbar-none overflow-y-scroll overflow-x-hidden">
                <Search />
                <TooltipProvider delay_duration=Duration::new(0, 0)>
                    <TooltipTrigger class="relative my-1">
                        <A href="me" {..} class=" flex relative items-center">
                            <div class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer">
                                <Icon
                                    icon=icondata::LuMessageCircle
                                    // class="h-5 w-5 stroke-base-content"
                                />
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
                <div class="divider bg-base-100 my-0.5 mx-[10px] h-0.5"></div>
                <Servers />
            </div>
            <Profile />
        </div>
    }
}
