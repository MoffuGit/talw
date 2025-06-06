use crate::app::api::thread::use_thread;
use crate::app::components::ui::modal::*;
use crate::app::ActionForm;
use leptos::{html, prelude::*};
use uuid::Uuid;

#[component]
pub fn DeleteThreadModal(
    thread_id: Uuid,
    #[prop(into)] thread_name: Signal<String>,
    server_id: Uuid,
    class: &'static str,
    #[prop(optional)] children: Option<Children>,
    content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let delete_thread = use_thread().delete_thread;
    let open = RwSignal::new(false);
    Effect::new(move |_| delete_thread.version().with(|_| open.set(false)));
    view! {
        <ModalProvider content_ref=content_ref open=open>
            <ModalTrigger class=class>{children.map(|children| children())}</ModalTrigger>
            <ModalContent class="w-[440px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <h2 class="p-4  leading-[24px] text-[20px] font-bold text-start w-full">
                    {move || format!("Delete '{}'", thread_name.get())}
                </h2>
                <div class="px-4 pb-10 w-full">
                    {move || format!(
                        "Are you sure you want to delete {}? This cannnot be undone.",
                        thread_name.get(),
                    )}
                </div>
                <div class="relative p-4 flex justify-end w-full bg-base-300/80">
                    <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                        "Cancel"
                    </ModalClose>
                    <ActionForm action=delete_thread>
                        <input value=thread_id.to_string() type="hidden" name="thread_id" />
                        <input value=server_id.to_string() type="hidden" name="server_id" />
                        <button
                            type="submit"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-error text-error-content"
                            disabled=move || delete_thread.pending().get()
                        >
                            "Delete Thread"
                        </button>
                    </ActionForm>
                </div>
            </ModalContent>
        </ModalProvider>
    }
}
