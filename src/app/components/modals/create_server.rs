pub mod join_with_invitation;
pub mod select_name;

use crate::app::components::modals::slide_modal::*;
use crate::app::components::modals::*;
use crate::app::components::tool_tip::*;
use crate::app::server::ServerTemplate;
use join_with_invitation::Join_with_invitation;
use leptos::*;
use leptos_icons::RiIcon::*;
use leptos_icons::*;
use select_name::Select_Name;
use strum::IntoEnumIterator;

pub type TemplateContext = RwSignal<ServerTemplate>;

fn use_template() -> TemplateContext {
    use_context::<TemplateContext>().expect("have create server context")
}

#[component]
pub fn Select_Template(template: ServerTemplate, children: Children) -> impl IntoView {
    let templates = use_template();

    view! {
        <SlideForward value=2 class="rounded-lg border-secondary border-[1px] border-solid mb-2  w-full p-0">
            <div class="flex items-center" on:click=move |_| templates.update(|value| *value = template)>
                <div class="w-[66px] h-[66px]"/>
                <div class="text-base leading-[20px] font-bold">{children()}</div>
                <div class="w-4 h-4 ml-auto mr-4">">"</div>
            </div>
        </SlideForward>
    }
}

#[component]
pub fn Create_server_modal() -> impl IntoView {
    let template = create_rw_signal::<ServerTemplate>(ServerTemplate::Default);
    let inital_value = 0;
    let slides = create_rw_signal::<Vec<u8>>(vec![]);

    provide_context(template);
    view! {
        <ModalProvider on_close=Signal::derive(move || slides.update(move |slides| {
            slides.clear();
            slides.push(inital_value);
        }))>



            <ModalTrigger class="group relative flex items-center mb-1 ">
                <TooltipProvider>
                    <TooltipTrigger class="flex items-center justify-center mx-3 transition-all h-[48px] w-[48px] bg-base-100 rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] overflow-hidden">
                        <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                        <Icon icon=Icon::from(RiAddSystemFill) class="fill-primary w-7 h-7 group-hover:fill-base-100"/>
                    </TooltipTrigger>
                    <TooltipContent tip="create a server".into()/>
                </TooltipProvider>
            </ModalTrigger>




            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden" >
                <SlideProvider initial_value=inital_value slides=slides>
                    <SlideViewport class="transition-height duration-400 ease-out overflow-hidden">
                        <SlideContent value=0 class="absolute flex-col items-center h-auto duration-400 ease-in transition w-[440px] inset-0 ">
                            <div class="pt-6 px-4">
                                <h1 class="leading-[30px] font-bold text-[24px] text-center">Create a server</h1>
                                <p class="text-center leading-[20px] mt-2 text-[16px] text-base-content">Your server is where you and your friends hang out. Make yours and start talking.</p>
                                <ModalClose attr:type="reset" class="absolute right-2 top-2 flex items-center group bg-none">
                                    <Icon icon=Icon::from(RiCloseSystemLine) class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                                </ModalClose>
                            </div>
                            <div class="overflow-x-hidden overflow-y-scroll mt-6 h-[330px] px-4 pb-2 relative">
                                {ServerTemplate::iter().map(|template| view!{<Select_Template template=template>{template.to_string()}</Select_Template>}).collect_view()}
                            </div>
                            <div class="relative p-4 overflow-x-auto flex-col items-start bg-base-200">
                                <h2 class="mb-2 leading-[24px] text-[20px] font-bold text-center">Have an invite already?</h2>
                                <SlideForward value=1 class="bg-accent hover:bg-accent-focus text-accent-content leading-[16px] font-medium no-animation w-full rounded-[3px] h-[38px] text-[14px]">
                                    Join a Server
                                </SlideForward>
                            </div>
                        </SlideContent>
                        <SlideContent value=1 class="absolute flex-col items-center h-[436px] duration-400 ease-in transition w-[440px] inset-0 ">
                            <Join_with_invitation/>
                        </SlideContent>
                        <SlideContent value=2 class="absolute flex-col items-center h-[404px] duration-400 ease-in transition w-[440px] inset-0 ">
                            <Select_Name/>
                        </SlideContent>
                    </SlideViewport>
                </SlideProvider>
            </ModalContent>
        </ModalProvider>
    }
}
