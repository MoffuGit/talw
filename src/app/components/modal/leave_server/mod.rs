use crate::app::api::server::use_server;
use crate::app::components::ui::modal::*;
use crate::app::ActionForm;
use crate::entities::server::Server;
use leptos::*;

#[component]
pub fn LeaveServer(
    server: Server,
    class: &'static str,
    on_click: Signal<()>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let leave_server = use_server().leave_server;
    view! {
        <ModalProvider content_ref=content_ref>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <h2 class="p-4  leading-[24px] text-[20px] font-bold text-start w-full">
                    {format!("Leave '{}'", server.name)}
                </h2>
                <div class="px-4 pb-10 w-full">
                    {format!(
                        "Are you sure you want to leave '{}'? You won't be able to rejoin this server unless you are re-invited.",
                        server.name,
                    )}
                </div>
                <div class="relative p-4 flex justify-end w-full bg-base-300/80">
                    <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                        "Cancel"
                    </ModalClose>
                    <ActionForm action=leave_server>
                        <input value=server.id.to_string() type="hidden" name="server_id" />
                        <button
                            type="submit"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-error text-error-content"
                            disabled=move || leave_server.pending().get()
                        >
                            "Leave Server"
                        </button>
                    </ActionForm>
                </div>
            </ModalContent>
        </ModalProvider>
    }
}
