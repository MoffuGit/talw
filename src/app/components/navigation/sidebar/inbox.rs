use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::*;
use std::time::Duration;

#[component]
pub fn Inbox() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0, 0)>
            <TooltipTrigger class="relative my-0.5">
                <div class="flex relative items-center">
                    <div class="flex transition-all items-center justify-center text-base-content w-8 h-8">
                        <Icon icon=icondata::LuInbox class="h-5 w-5 stroke-base-content" />
                    </div>
                </div>
            </TooltipTrigger>
            <TooltipContent
                tooltip_of_side=10.0
                tip="Inbox"
                arrow=true
                class="rounded-lg w-auto h-auto py-1.5 px-2.5 text-sm bg-base-400 border-base-400"
            />
        </TooltipProvider>
    }
}
