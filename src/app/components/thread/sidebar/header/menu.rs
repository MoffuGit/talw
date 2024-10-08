use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::components::thread::sidebar::CurrentThreadContext;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::components::ui::tool_tip::*;
use leptos::*;
use leptos_icons::Icon;
use std::time::Duration;

#[component]
pub fn ThreadMenu() -> impl IntoView {
    let current_thread = use_context::<CurrentThreadContext>()
        .expect("SHould return the current thrread context")
        .thread;
    let open = create_rw_signal(false);
    view! {
        <DropdownProvider modal=false open=open>
                <TooltipProvider delay_duration=Duration::new(0, 0)>
                    <TooltipTrigger>
                        <DropdownTrigger>
                            <Icon icon=icondata::RiMoreSystemLine class="w-6 h-6 "/>
                        </DropdownTrigger>
                    </TooltipTrigger>
                    <TooltipContent tip="More" tooltip_side=ToolTipSide::Bottom tooltip_of_side=10.0 class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:bottom-[100%] after:left-[50%] after:ml-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-b-[#dfdfe2] dark:after:border-b-[#0d0d0d]"/>
                </TooltipProvider>
            <DropdownContent side=MenuSide::Bottom align=MenuAlign::End class="z-40">
                <ThreadMenuContent open=open thread=current_thread.clone()/>
            </DropdownContent>
        </DropdownProvider>
    }
}
