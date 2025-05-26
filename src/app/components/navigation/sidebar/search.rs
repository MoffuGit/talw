use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::tool_tip::*;
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Search() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger class="relative mt-1.5">
                <div class="flex items-center justify-center text-base-content w-7 h-7 fill-base-content stroke-base-content relative hover:bg-base-100 rounded-md cursor-pointer">
                    <Icon icon=IconData::Search class="w-4 h-4"/>
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
