use crate::app::components::ui::modal::ModalTrigger;
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::Icon;
use std::time::Duration;

#[component]
pub fn CreateServerTrigger() -> impl IntoView {
    view! {
        <TooltipProvider delay_duration=Duration::new(0,500)>
            <TooltipTrigger class="group relative flex items-center mb-1" >
                <ModalTrigger class="flex items-center justify-center mx-3 transition-all h-[48px] w-[48px] bg-base-100 rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] overflow-hidden">
                    <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                    <Icon icon=icondata::RiAddSystemFill class="fill-primary w-7 h-7 group-hover:fill-base-100"/>
                    <TooltipContent tip="create a server".into() class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#c6d2d2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#c6d2d2] dark:after:border-r-[#0d0d0d]"/>
                </ModalTrigger>
            </TooltipTrigger>
        </TooltipProvider>
    }
}
