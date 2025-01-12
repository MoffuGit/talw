use super::use_create_server;
use crate::app::api::server::ServerTemplate;
use crate::app::components::ui::modal::slide_modal::SlideForward;
use crate::app::components::ui::modal::ModalClose;
use icondata;
use leptos::*;
use leptos_icons::*;
use strum::IntoEnumIterator;

#[allow(non_snake_case)]
#[component]
pub fn SelectTemplate() -> impl IntoView {
    view! {
        <div class="pt-6 px-4">
            <h1 class="leading-[30px] font-bold text-[24px] text-center">Create a server</h1>
            <p class="text-center leading-[20px] mt-2 text-[16px] text-base-content">
                Your server is where you and your friends hang out. Make yours and start talking.
            </p>
            <ModalClose
                attr:type="reset"
                class="absolute right-2 top-2 flex items-center group bg-none"
            >
                <Icon
                    icon=icondata::RiCloseSystemLine
                    class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"
                />
            </ModalClose>
        </div>
        <div class="overflow-x-hidden overflow-y-scroll mt-6 h-[330px] px-4 pb-2 relative">
            <Templates template=ServerTemplate::Default>
                {ServerTemplate::Default.to_string()}
            </Templates>
            <div class="mt-[12px] mb-[8px] uppercase font-bold text-[12px]">
                "start from a template"
            </div>
            {ServerTemplate::iter()
                .filter(|&template| template != ServerTemplate::Default)
                .map(|template| {
                    view! { <Templates template=template>{template.to_string()}</Templates> }
                })
                .collect_view()}
        </div>
        <div class="relative p-4 overflow-x-auto flex-col items-start bg-base-300/50">
            <h2 class="mb-2 leading-[24px] text-[20px] font-bold text-center">
                Have an invite already?
            </h2>
            <SlideForward
                value=1
                class="bg-accent hover:bg-accent-focus text-accent-content leading-[16px] font-medium no-animation w-full rounded-[3px] h-[38px] text-[14px]"
            >
                Join a Server
            </SlideForward>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
fn Templates(template: ServerTemplate, children: Children) -> impl IntoView {
    let selected_template = use_create_server().selected_template;
    view! {
        <SlideForward
            value=2
            class="rounded-lg border-secondary border-[1px] border-solid mb-2  w-full p-0"
        >
            <div
                class="flex items-center"
                on:click=move |_| selected_template.update(|value| *value = template)
            >
                <div class="w-[66px] h-[66px]" />
                <div class="text-base leading-[20px] font-bold">{children()}</div>
                <Icon icon=icondata::RiArrowRightSArrowsLine class="ml-auto mr-3 h-6 w-6" />
            </div>
        </SlideForward>
    }
}
