use std::str::FromStr;

use gloo_file::Blob;
use leptos::prelude::*;
use web_sys::Url;

use crate::app::components::chat::ChatContext;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::uploadthings::{FileType, UploadthingFile};
use crate::uploadthing::FileData;

#[component]
pub fn Attachments() -> impl IntoView {
    let ChatContext { attachments, .. } =
        use_context::<ChatContext>().expect("should acces to the chat context");
    view! {
        <Show when=move || !attachments.get().is_empty()>
            <div class="relative w-full h-auto bg-base-300 border first:rounded-t-lg border-b-0 border-base-100 flex items-center p-2 text-sm">
                <For
                    each=move || attachments.get().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    let:((idx, attachment))
                >
                    <Attachment attachment=attachment idx=idx attachments=attachments/>
                </For>
            </div>
        </Show>
    }
}

#[component]
pub fn Attachment(
    attachment: UploadthingFile,
    idx: usize,
    attachments: RwSignal<Vec<UploadthingFile>>,
) -> impl IntoView {
    let FileData {
        name, file_type, ..
    } = attachment.data;
    let file_type = FileType::from_str(&file_type).unwrap();
    let url: RwSignal<Option<String>> = RwSignal::new(match file_type {
        FileType::Jpeg | FileType::Png => {
            Url::create_object_url_with_blob(&Blob::new(&*attachment.chunks).into()).ok()
        }
        _ => None,
    });
    view! {
        <div class="relative w-40 h-40 p-2 rounded-lg border border-base-100 flex flex-col items-center justify-around">
            <div
                class="bg-base-300 hover:bg-red-700 rounded-md flex items-center justify-center w-7 h-7 cursor-pointer select-none absolute top-1 right-1"
            >
                <Icon
                    icon=IconData::Trash
                    on:click=move |_| {
                        attachments.update(|attachments| {
                            attachments.remove(idx);
                        });
                    }
                    class="h-4 w-4"
                />
            </div>
            {
                match file_type {
                    FileType::Jpeg | FileType::Png =>  {
                        view! {
                            <img
                                class="w-full mx-1 h-28 object-cover rounded-md"
                                src=move || url.get().unwrap()
                                on:load=move |_| {
                                    let _ = Url::revoke_object_url(&url.get().unwrap());
                                }
                            />
                        }.into_any()
                    },
                    _ => {
                        ().into_any()
                    }
                }
            }
            <div class="w-full text-start max-h-4 text-xs text-nowrap truncate inline-block">
                {name}
            </div>
        </div>

    }
}
