pub mod join_with_invitation;
pub mod select_name;

use crate::app::components::modals::slide_modal::*;
use crate::app::components::modals::*;
use crate::app::server::ServerTemplate;
use join_with_invitation::Join_with_invitation;
use leptos::*;
use leptos_icons::RiIcon::*;
use leptos_icons::*;
use select_name::Select_Name;
use strum::IntoEnumIterator;

pub type SlidesContext = RwSignal<Vec<u8>>;
pub type TemplateContext = RwSignal<ServerTemplate>;

fn use_slides() -> SlidesContext {
    use_context::<SlidesContext>().expect("have this context")
}

fn use_template() -> TemplateContext {
    use_context::<TemplateContext>().expect("have create server context")
}

#[component]
pub fn Modal_Slide(children: Children, slide: u8) -> impl IntoView {
    let slides = use_slides();
    let position = move || {
        if slides.get().last() == Some(&slide) {
            ""
        } else if slides.get().iter().any(|val| val == &slide) {
            "-translate-x-[440px]"
        } else {
            "translate-x-[440px]"
        }
    };
    view! {
        <div class=move || format!("absolute flex-col items-center duration-400 ease-in transition w-[440px] inset-0 {}", position())>
            {children()}
        </div>
    }
}

#[component]
pub fn Select_Template(template: ServerTemplate, children: Children) -> impl IntoView {
    let templates = use_template();
    let slides = use_slides();

    view! {
        <button on:click=move |_| { templates.update(|temp: &mut ServerTemplate| *temp = template); slides.update(move |slides| slides.push(2)); }
        class="rounded-lg border-secondary border-[1px] border-solid mb-2 flex items-center w-full p-0">
            <div class="w-[66px] h-[66px]"/>
            <div class="text-base leading-[20px] font-bold">{children()}</div>
            <div class="w-4 h-4 ml-auto mr-4">">"</div>
        </button>
    }
}

#[component]
pub fn Create_server_modal() -> impl IntoView {
    let slides = create_rw_signal::<Vec<u8>>(vec![0]);
    let template = create_rw_signal::<ServerTemplate>(ServerTemplate::Default);
    provide_context(slides);
    provide_context(template);
    view! {
        <ModalProvider>
            <ModalTrigger class="flex items-center justify-center mx-3 transition-all h-[48px] w-[48px] bg-base-100 rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] overflow-hidden">
                <Icon icon=Icon::from(RiAddSystemFill) class="fill-primary w-7 h-7 group-hover:fill-base-100"/>
            </ModalTrigger>
            <ModalPortal>
                <ModalContent class="w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden" >
                    <SlideProvider initial_value=0>
                        <SlideViewport class="transition-height duration-400 ease-out overflow-hidden">
                            <SlideContent value=0>
                                <div class="pt-6 px-4">
                                    <h1 class="leading-[30px] font-bold text-[24px] text-center">Create a server</h1>
                                    <p class="text-center leading-[20px] mt-2 text-[16px] text-base-content">Your server is where you and your friends hang out. Make yours and start talking.</p>
                                    <ModalClose class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">
                                        "x"
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
                        </SlideViewport>
                    </SlideProvider>
                    <div class=move || format!("transition-height duration-400 ease-out overflow-hidden {}", match slides.get().last() {
                        Some(0) => "h-[558px]",
                        Some(1) => "h-[436px]",
                        _ => "h-[404px]"
                    })>
                        <Modal_Slide slide=0>
                            <div class="pt-6 px-4">
                                <h1 class="leading-[30px] font-bold text-[24px] text-center">Create a server</h1>
                                <p class="text-center leading-[20px] mt-2 text-[16px] text-base-content">Your server is where you and your friends hang out. Make yours and start talking.</p>
                                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">"✕"</button>
                            </div>
                            <div class="overflow-x-hidden overflow-y-scroll mt-6 h-[330px] px-4 pb-2 relative">
                                {ServerTemplate::iter().map(|template| view!{<Select_Template template=template>{template.to_string()}</Select_Template>}).collect_view()}
                            </div>
                            <div class="relative p-4 overflow-x-auto flex-col items-start bg-base-200">
                                <h2 class="mb-2 leading-[24px] text-[20px] font-bold text-center">Have an invite already?</h2>
                                <button on:click=move |_| slides.update(move |slides| slides.push(1))  class="bg-accent hover:bg-accent-focus text-accent-content leading-[16px] font-medium no-animation w-full rounded-[3px] h-[38px] text-[14px]">Join a Server</button>
                            </div>
                        </Modal_Slide>

                        <Modal_Slide slide=1>
                            <Join_with_invitation/>
                        </Modal_Slide>
                        <Modal_Slide slide=2>
                            <Select_Name/>
                        </Modal_Slide>
                    </div>
                </ModalContent>
            </ModalPortal>
        </ModalProvider>
    }
}
