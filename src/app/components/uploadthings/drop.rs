use leptos::ev::dragenter;
use leptos::ev::dragleave;
use leptos::prelude::*;
use leptos_use::use_document;
use leptos_use::use_event_listener;
use log::debug;
use wasm_bindgen::JsCast as _;

use super::read_file;
use super::UploadthingFile;

#[component]
pub fn DropZone(
    #[prop(default = RwSignal::new(false), into)] active: RwSignal<bool>,
    #[prop(default = RwSignal::new(false), into)] on_zone: RwSignal<bool>,
    files: RwSignal<Vec<UploadthingFile>>,
    #[prop(into)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        let _ = use_event_listener(use_document(), dragenter, move |evt| {
            evt.prevent_default();
            active.set(true)
        });
        let _ = use_event_listener(use_document(), dragleave, move |evt| {
            evt.prevent_default();
            let document_element = use_document().body().unwrap();
            match evt.related_target() {
                Some(related_target) => {
                    if !document_element.contains(Some(&related_target.unchecked_into())) {
                        active.set(false);
                    }
                }
                None => {
                    active.set(false);
                }
            }
        });
    }

    view! {
        <div
            class=move || class.get()
            on:dragenter=move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                on_zone.set(true);
            }
            on:dragover=move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
            }
            on:dragleave=move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                on_zone.set(false);
            }
            on:drop=move |evt| {
                evt.prevent_default();
                evt.stop_propagation();
                if let Some(data) = evt.data_transfer() {
                    if let Some(file_list) = data.files() {
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
                }
                on_zone.set(false);
                active.set(false);
            }
        >
            {children.map(|children| children())}
        </div>
    }
}
