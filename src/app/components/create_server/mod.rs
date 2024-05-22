pub mod create_server_trigger;
pub mod join_with_invitation;
pub mod select_name;
pub mod select_template;

use crate::app::api::server::use_server;
use crate::app::api::server::ServerTemplate;
use crate::app::components::create_server::select_template::SelectTemplate;
use crate::app::components::ui::modal::slide_modal::*;
use crate::app::components::ui::modal::*;
use create_server_trigger::CreateServerTrigger;
use join_with_invitation::Join_with_invitation;
use leptos::*;
use select_name::Select_Name;
use std::time::Duration;

#[derive(Clone)]
pub struct CreateServerContext {
    is_open: RwSignal<bool>,
    selected_template: RwSignal<ServerTemplate>,
    join_with_invitation_ref: NodeRef<html::Form>,
    select_name_ref: NodeRef<html::Form>,
}

fn use_create_server() -> CreateServerContext {
    use_context::<CreateServerContext>().expect("have create server context")
}

#[component]
pub fn Create_server_modal() -> impl IntoView {
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
            <CreateServerTrigger/>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden flex items-center" >
                <SlideProvider initial_value=inital_value slides=slides on_slide=Signal::derive(on_slide)>
                    <SlideViewport class="transition-height duration-400 ease-out overflow-hidden">
                        <SlideContent value=0 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <SelectTemplate/>
                        </SlideContent>
                        <SlideContent value=1 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <Join_with_invitation/>
                        </SlideContent>
                        <SlideContent value=2 class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 ">
                            <Select_Name/>
                        </SlideContent>
                    </SlideViewport>
                </SlideProvider>
            </ModalContent>
        </ModalProvider>
    }
}
