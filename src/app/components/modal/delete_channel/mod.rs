use crate::app::api::channel::use_channel;
use crate::app::components::ui::modal::*;
use crate::app::ActionForm;
use crate::entities::channel::{Channel, ChannelStoreFields};
use leptos::{html, prelude::*};
use uuid::Uuid;

#[component]
pub fn DeleteChannel(
    #[prop(into)] channel: reactive_stores::Field<Channel>,
    #[prop(into)] server_id: Signal<Uuid>,
    class: &'static str,
    #[prop(optional)] on_click: Signal<()>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] content_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let delete_channel = use_channel().delete_channel;
    Effect::new(move |_| {
        delete_channel.version().with(|_| {
            if let Some(Ok(_)) = delete_channel.value().get() {
                open.update(|value| *value = false);
            }
        });
    });
    let name = channel.name();
    view! {
        <ModalProvider content_ref=content_ref open=open>
            <ModalTrigger class=class on_click=on_click>
                {children.map(|children| children())}
            </ModalTrigger>
            <ModalContent class="w-[440px] rounded p-0 h-auto overflow-hidden flex flex-col items-center">
                <h2 class="p-4  leading-[24px] text-[20px] font-bold text-start w-full">
                    {move || format!("Delete '{}'", name.get())}
                </h2>
                <div class="px-4 pb-10 w-full">
                    {move || format!(
                        "Are you sure you want to delete {}? This cannot be undone",
                        name.get(),
                    )}
                </div>
                <div class="relative p-4 flex justify-end w-full bg-base-300/80">
                    <ModalClose class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 hover:underline">
                        "Cancel"
                    </ModalClose>
                    <ActionForm action=delete_channel>
                        <input value=channel.id().get().to_string() type="hidden" name="channel_id" />
                        <input value=move || server_id.get().to_string() type="hidden" name="server_id" />
                        <button
                            type="submit"
                            class="relative flex justify-center items-center text-sm font-medium h-[38px] px-4 rounded bg-error text-error-content"
                            disabled=move || delete_channel.pending().get()
                        >
                            "Delete Channel"
                        </button>
                    </ActionForm>
                </div>
            </ModalContent>
        </ModalProvider>
    }
}
