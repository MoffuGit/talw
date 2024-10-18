use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::*;
use std::time::Duration;

#[component]
pub fn Inbox() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0,0)>
            <TooltipTrigger class="relative my-0.5 mb-3">
                <div class="flex relative items-center">
                    <div class="flex mx-3 my-1 transition-all items-center justify-center text-base-content w-[48px]">
                        <Icon icon=icondata::RiInboxBusinessFill class="h-6 w-6 fill-primary"/>
                    </div>
                </div>
            </TooltipTrigger>
            <TooltipContent tip="Inbox" class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
        </TooltipProvider>
    }
}
