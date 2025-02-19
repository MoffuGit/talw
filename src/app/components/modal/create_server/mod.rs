pub mod join_with_invitation;
pub mod select_name;
pub mod select_template;
use crate::app::api::server::use_server;
use crate::app::api::server::ServerTemplate;
use crate::app::components::modal::create_server::join_with_invitation::JoinWithInvitation;
use crate::app::components::ui::modal::slide_modal::*;
use crate::app::components::ui::modal::*;
use leptos::html;
use leptos::prelude::*;
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

 
#[component]
pub fn CreateServerModal(
    class: &'static str,
    children: Children,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
    #[prop(optional)] on_open: Signal<()>,
) -> impl IntoView {
    let use_server = use_server();

    let is_open = RwSignal::new(false);

    let selected_template = RwSignal::new(ServerTemplate::Default);
    let join_with_invitation_ref = NodeRef::<html::Form>::new();
    let select_name_ref = NodeRef::<html::Form>::new();

    let inital_value = 0;
    let slides = RwSignal::<Vec<u8>>::new(vec![]);

    let on_close = move || {
        if let Some(form) = join_with_invitation_ref.get() {
            form.reset()
        }
        if let Some(form) = select_name_ref.get() {
            form.reset()
        }
        use_server.create_server.clear();
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

    Effect::new(move |_| {
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
        <ModalProvider content_ref=content_ref open=is_open on_close=Signal::derive(on_close)>
            <ModalTrigger class=class on_click=on_open>
                {children()}
            </ModalTrigger>
            <ModalContent class="w-[440px] max-h-[720px] rounded p-0 bg-none min-h-[200px] h-auto overflow-hidden flex items-center">
                <SlideProvider
                    initial_value=inital_value
                    slides=slides
                    on_slide=Signal::derive(on_slide)
                >
                    <SlideViewport class="transition-height duration-400 ease-out overflow-hidden">
                        <SlideContent
                            value=0
                            class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 "
                        >
                            <SelectTemplate />
                        </SlideContent>
                        <SlideContent
                            value=1
                            class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 "
                        >
                            <JoinWithInvitation />
                        </SlideContent>
                        <SlideContent
                            value=2
                            class="absolute flex-col items-center h-min duration-400 ease-in transition w-[440px] inset-0 "
                        >
                            <SelectName />
                        </SlideContent>
                    </SlideViewport>
                </SlideProvider>
            </ModalContent>
        </ModalProvider>
    }
}
