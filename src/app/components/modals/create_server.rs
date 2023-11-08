pub mod join_with_invitation;
pub mod select_name;

use crate::app::server::ServerTemplate;
use join_with_invitation::Join_with_invitation;
use leptos::*;
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
    // let selected_server_options = create_rw_signal::<ServerOptions>();
    // Server Option {type: Default | other options, name: string, for: friends | other}
    //
    provide_context(slides);
    provide_context(template);
    view! {
        <dialog id="create_server" class="modal">
            <div class="modal-box w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden">
                <div class=move || format!("transition-height duration-400 ease-out overflow-hidden {}", match slides.get().last() {
                    Some(0) => "h-[558px]",
                    Some(1) => "h-[436px]",
                    _ => "h-[404px]"
                })>
                    <Modal_Slide slide=0>
                        <div class="pt-6 px-4">
                            <h1 class="leading-[30px] font-bold text-[24px] text-center">Create a server</h1>
                            <p class="text-center leading-[20px] mt-2 text-[16px] text-base-content">Your server is where you and your friends hang out. Make yours and start talking.</p>
                            <form method="dialog">
                                <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">"✕"</button>
                            </form>
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
            </div>
            <form method="dialog" class="modal-backdrop">
                <button/>
            </form>
        </dialog>
    }
}
