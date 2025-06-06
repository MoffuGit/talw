use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::tool_tip::*;
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Inbox() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger class="relative">
                <div class="flex relative items-center">
                    <div class="flex items-center justify-center text-base-content w-7 h-7 relative hover:bg-base-100 rounded-md cursor-pointer">
                        <Icon icon=IconData::Inbox class="w-4 h-4"/>
                    </div>
                </div>
            </TooltipTrigger>
            <TooltipContent
                tooltip_of_side=10.0
                tip="Inbox"
                arrow=true
                class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
            />
        </TooltipProvider>
    }
}
