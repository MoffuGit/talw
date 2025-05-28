use crate::app::components::chat::ChatContext;
use crate::app::components::uploadthings::drop::DropZone;
use leptos::portal::Portal;
use leptos::prelude::*;

#[component]
pub fn ChatDropZone() -> impl IntoView {
    let active = RwSignal::new(false);
    let on_zone = RwSignal::new(false);
    let ChatContext { attachments, .. } =
        use_context::<ChatContext>().expect("should acces to the chat context");
    view! {
        <Portal mount=document().get_element_by_id("app").expect("acces to the app")>
            <DropZone
                files=attachments
                active=active
                on_zone=on_zone
                class=Signal::derive(
                    move || {
                        format!("absolute inset-0 z-100 bg-base-300/40 flex items-center justify-center {}", {
                            if active.get() {
                                "visible"
                            } else {
                                "invisible"
                            }
                        })
                    }
                )
            >
                <div class="text-2xl font-base">
                    "Drop to upload file"
                </div>
            </DropZone>
        </Portal>
    }
}
