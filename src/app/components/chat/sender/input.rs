use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;

use crate::app::components::chat::ChatContext;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::uploadthings::input::FileInput;

#[component]
pub fn Input(
    on_click: Signal<()>,
    name: Field<String>,
    message: RwSignal<String>,
    height: RwSignal<i32>,
) -> impl IntoView {
    let content_ref: NodeRef<Div> = NodeRef::new();

    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
            height.set(div.offset_height());
        }
    };

    let ChatContext { attachments, .. } =
        use_context::<ChatContext>().expect("should acces to the chat context");

    view! {
        <div class="relative w-full h-auto bg-base-300 rounded-b-lg px-4 only:rounded-lg border border-base-100 flex items-center">
            <FileInput files=attachments class="w-a h-auto">
                <Icon icon=IconData::PaperClip class="w-4 h-4 stroke-base-content/40 hover:stroke-base-content cursor-pointer"/>
            </FileInput>
                <div class="relative self-center h-fit w-full" /* style=move || format!("height: {}px", height.get()) */>
                    <div class="text-sm font-normal relative mx-4 py-4">
                        <div>
                            <Show when=move || message.get().is_empty()>
                                <div class="absolute left-0 select-none text-base-content/40">
                                    {move || format!("Message #{}", name.get())}
                                </div>
                            </Show>
                        </div>
                        <div
                            on:input=on_input
                            node_ref=content_ref
                            class="relative outline-0 wrap-break-word text-left whitespace-break-spaces"
                            contenteditable="true"
                            aria-multiline="true"
                            spellcheck="true"
                            aria-invalid="false">
                        </div>
                    </div>
                </div>
            <Icon icon=IconData::Sticker on:click=move |_| {
                    on_click.get();
                }
                class="w-4 h-4 stroke-base-content/40 hover:stroke-base-content cursor-pointer"/>
        </div>
    }
}
