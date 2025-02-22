use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::prelude::*;
use leptos_icons::*;
use std::time::Duration;

#[component]
pub fn Search() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger class="relative my-1">
                <div class="flex relative items-center">
                    <div class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer">
                        <Icon icon=icondata::LuSearch /* class="h-5 w-5 stroke-base-content" */ />
                    </div>
                </div>
            </TooltipTrigger>
            <TooltipContent
                tooltip_of_side=10.0
                tip="Search"
                arrow=true
                class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
            />
        </TooltipProvider>
    }
}
