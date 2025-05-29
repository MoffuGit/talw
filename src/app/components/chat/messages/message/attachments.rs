use leptos::prelude::*;

use crate::entities::message::ChannelMessage;

#[component]
pub fn Attachments(message: RwSignal<ChannelMessage>) -> impl IntoView {
    view! {
        <Show when=move || !message.get().attachments.is_empty()>
            <div class="max-w-136 h-auto flex">
                <For
                    each=move || message.get().attachments
                    key=|attachment| attachment.id
                    let:attachment
                >
                    <img
                        src=attachment.url
                        alt=attachment.filename
                        class="w-auto h-auto block rounded-lg"
                    />
                </For>
            </div>
        </Show>
    }
}
