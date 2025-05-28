use std::str::FromStr;

use leptos::prelude::*;

use crate::app::components::chat::ChatContext;
use crate::app::components::uploadthings::FileType;
use crate::uploadthing::FileData;

#[component]
pub fn Attachments() -> impl IntoView {
    let ChatContext { attachments, .. } =
        use_context::<ChatContext>().expect("should acces to the chat context");
    let file_data = Signal::derive(move || {
        attachments
            .get()
            .iter()
            .map(|attach| attach.data.clone())
            .collect::<Vec<_>>()
    });
    view! {
        <Show when=move || !file_data.get().is_empty()>
            <div class="relative w-full h-fit bg-base-300 border first:rounded-t-lg border-b-0 border-base-100 flex items-center p-2 text-sm">
                <For
                    each=move || file_data.get()
                    key=|data| data.name.clone()
                    let:data
                >
                    <div class="w-20 h-10 flex flex-col items-center">
                        <div>
                            {data.name}
                        </div>
                        <div>
                            {data.file_type}
                        </div>
                        <div>
                            {data.size}
                        </div>
                    </div>
                </For>
            </div>
        </Show>
    }
}

#[component]
pub fn Attachment(attachment: FileData) -> impl IntoView {
    let file_type = FileType::from_str(&attachment.file_type).unwrap();
    //NOTE: all are going ot have the same size, width and height but the content its going to be
    //different, right now, i only care about image because its the more easy, the other types are
    //going to share the same look for now, later i add more views
    view! {}
}
