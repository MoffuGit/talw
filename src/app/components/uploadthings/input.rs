use leptos::html::Input as HtmlInput;
use leptos::prelude::*;
use log::debug;

use super::{read_file, UploadthingFile};

#[component]
pub fn FileInput(
    files: RwSignal<Vec<UploadthingFile>>,
    #[prop(into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let input_ref: NodeRef<HtmlInput> = NodeRef::new();

    let on_change = move |_| {
        let input = input_ref.get().expect("input ref to be valid");
        if let Some(file_list) = input.files() {
            for idx in 0..file_list.length() {
                if let Some(file) = file_list.get(idx) {
                    read_file(file.into(), move |file| {
                        if let Ok(file) = file {
                            files.update(|files| files.push(file));
                        } else {
                            debug!("Error on the read file");
                        }
                    });
                }
            }
        }
    };
    view! {
        <label class=move || format!("relative {}", class.get())>
            {children.map(|children| children())}
            <input
                type="file"
                class="absolute inset-0 opacity-0 pointer-events-none"
                node_ref=input_ref
                on:change=on_change
                multiple=true
            />
        </label>
    }
}
