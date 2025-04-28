use crate::app::components::menu::thread::ThreadMenuContent;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::components::ui::tool_tip::*;
use crate::entities::thread::Thread;
use leptos::html;
use leptos::prelude::*;
use reactive_stores::Store;
use std::time::Duration;

#[component]
pub fn ThreadMenu() -> impl IntoView {
    let current_thread =
        use_context::<Store<Thread>>().expect("SHould return the current thrread context");
    let open = RwSignal::new(false);
    let delete_thread_modal_ref = NodeRef::<html::Div>::new();
    view! {
        <DropdownProvider modal=false open=open>
            <TooltipProvider delay_duration=Duration::new(0, 0)>
                <TooltipTrigger>
                    <DropdownTrigger class="p-1 rounded-lg hover:bg-base-content/5">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width="20"
                            height="20"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="lucide lucide-ellipsis"
                        >
                            <circle cx="12" cy="12" r="1" />
                            <circle cx="19" cy="12" r="1" />
                            <circle cx="5" cy="12" r="1" />
                        </svg>
                    </DropdownTrigger>
                </TooltipTrigger>
                <TooltipContent
                    tip="Options"
                    tooltip_side=ToolTipSide::Bottom
                    tooltip_of_side=10.0
                    class="rounded-md w-auto h-auto py-1.5 px-2.5 text-sm text-base-100 bg-base-content border-base-content"
                />
            </TooltipProvider>
            <DropdownContent
                ignore=vec![delete_thread_modal_ref]
                side=MenuSide::Bottom
                align=MenuAlign::End
                class="z-40"
            >
                <ThreadMenuContent
                    delete_thread_modal_ref=delete_thread_modal_ref
                    open=open
                    thread=current_thread
                />
            </DropdownContent>
        </DropdownProvider>
    }
}
