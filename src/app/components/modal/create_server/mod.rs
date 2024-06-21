pub mod join_with_invitation;
pub mod select_name;
pub mod select_template;
use crate::app::api::server::use_server;
use crate::app::api::server::ServerTemplate;
use crate::app::components::modal::create_server::join_with_invitation::JoinWithInvitation;
use crate::app::components::ui::modal::slide_modal::*;
use crate::app::components::ui::modal::*;
use crate::app::components::ui::tool_tip::*;
use icondata;
use leptos::*;
use leptos_icons::Icon;
use select_name::SelectName;
use select_template::SelectTemplate;
use std::time::Duration;

#[derive(Clone)]
struct CreateServerContext {
    is_open: RwSignal<bool>,
    selected_template: RwSignal<ServerTemplate>,
    join_with_invitation_ref: NodeRef<html::Form>,
    select_name_ref: NodeRef<html::Form>,
}

fn use_create_server() -> CreateServerContext {
    use_context::<CreateServerContext>().expect("have create server context")
}

#[allow(non_snake_case)]
#[component]
pub fn CreateServerModal() -> impl IntoView {
    let use_server = use_server();

    let is_open = create_rw_signal(false);

    let selected_template = create_rw_signal::<ServerTemplate>(ServerTemplate::Default);
    let join_with_invitation_ref = create_node_ref::<html::Form>();
    let select_name_ref = create_node_ref::<html::Form>();

    let inital_value = 0;
    let slides = create_rw_signal::<Vec<u8>>(vec![]);

    let on_close = move || {
        if let Some(form) = join_with_invitation_ref.get() {
            form.reset()
        }
        if let Some(form) = select_name_ref.get() {
            form.reset()
        }
        use_server.create_server.value().set(None);
        set_timeout(
            move || {
                slides.update(|slides| {
                    slides.clear();
                    slides.push(inital_value);
                })
            },
            Duration::from_millis(250),
        );
    };
    let on_slide = move || {
        use_server.join_with_invitation.value().set(None);
        use_server.create_server.value().set(None);
    };

    create_effect(move |_| {
        use_server.create_server.version().with(|_| {
            if let Some(Ok(_)) = use_server.create_server.value().get() {
                is_open.update(|value| *value = false);
            }
        });
    });

    provide_context(CreateServerContext {
        is_open,
        selected_template,
        select_name_ref,
        join_with_invitation_ref,
    });
    view! {
        <ModalProvider open=is_open on_close=Signal::derive(on_close)>
            <TooltipProvider delay_duration=Duration::new(0,500)>
                <TooltipTrigger class="group relative flex items-center my-0.5" >
                    <ModalTrigger class="flex items-center justify-center mx-3 transition-all h-[48px] w-[48px] bg-base-100 rounded-[24px] group-hover:bg-primary group-hover:rounded-[16px] overflow-hidden">
                        <div class="absolute left-0 bg-primary rounded-r-full transition-all w-[4px] group-hover:h-[20px] h-[8px]"/>
                        <Icon icon=icondata::RiAddSystemFill class="fill-primary w-7 h-7 group-hover:fill-base-100"/>
                        <TooltipContent tip="create a server".into() class="rounded w-auto h-auto py-1 px-2 text-base font-bold bg-[#dfdfe2] dark:bg-[#0d0d0d] after:content-[' '] after:absolute after:top-[50%] after:right-[100%] after:mt-[-5px] after:border-[5px] after:border-solid after:border-transparent after:border-r-[#dfdfe2] dark:after:border-r-[#0d0d0d]"/>
                    </ModalTrigger>
                </TooltipTrigger>
            </TooltipProvider>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden flex items-center" >
                <SlideProvider initial_value=inital_value slides=slides on_slide=Signal::derive(on_slide)>
                    <SlideViewport class="transition-height duration-400 ease-out overflow-hidden">
                        <SlideContent value=0 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <SelectTemplate/>
                        </SlideContent>
                        <SlideContent value=1 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <JoinWithInvitation/>
                        </SlideContent>
                        <SlideContent value=2 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <SelectName/>
                        </SlideContent>
                    </SlideViewport>
                </SlideProvider>
            </ModalContent>
        </ModalProvider>
    }
}
