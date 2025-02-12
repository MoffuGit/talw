use leptos::prelude::*;

use crate::app::api::user::use_user;
use crate::app::components::overview::user::content::account::profile::ProfilesSettingsContext;
use wasm_bindgen::JsCast;
use web_sys::{Blob, FormData, HtmlFormElement, HtmlInputElement, Url};

use leptos::ev::SubmitEvent;
use leptos::html::Form;
#[component]
pub fn ImageBanner(primary_color_preview: RwSignal<Option<String>>) -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let image_url = context.banner.image_url;
    let image_preview_url = RwSignal::new(image_url.clone());
    let edit_banner_image = use_user().edit_banner_image;
    let form_ref = NodeRef::<Form>::new();
    view! {
        <form
            on:submit=move |evt: SubmitEvent| {
                evt.prevent_default();
                let target = evt.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&target).unwrap();
                edit_banner_image.dispatch_local(form_data);
            }
            on:reset=move |_| {
                image_preview_url.set(image_url.clone());
            }
            node_ref=form_ref
            class="group w-full relative"
        >
            <input
                type="file"
                name="banner_image"
                accept="image/*"
                class="w-full h-36 absolute opacity-0 top-0 z-50 rounded-t cursor-pointer"
                on:change=move |evt| {
                    form_ref.get().map(|form| form.request_submit());
                    let target = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if let Some(file) = target.files().and_then(|files| files.item(0)) {
                        image_preview_url
                            .set(Url::create_object_url_with_blob(&Blob::from(file)).ok());
                    }
                }
            />
            {move || match image_preview_url.get() {
                Some(url) => {
                    let url = StoredValue::new(url);
                    view! {
                        <img
                            class="w-full h-36 absolute top-0 object-cover z-30 rounded-t"
                            src=url.get_value()
                            on:load=move |_| {
                                let _ = Url::revoke_object_url(&url.get_value());
                            }
                        />
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <div class="w-full h-36 absolute top-0 object-cover z-30 rounded-t bg-primary" />
                    }
                        .into_any()
                }
            }}
            <div class="w-full h-36 absolute top-0 rounded-t transition-opacity bg-base-100/30 opacity-0 group-hover:opacity-100 flex items-center justify-center z-40">
                <p class="text-base-content text-xl">"Change Banner"</p>
            </div>
        </form>
    }
}
