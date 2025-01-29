use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, FormData, HtmlFormElement, HtmlInputElement, Url};

use crate::app::api::server::use_server;
use crate::app::components::overview::server::ServerSettingsData;

use self::ev::SubmitEvent;
use self::html::Form;

#[component]
pub fn ServerImage() -> impl IntoView {
    let server = use_context::<ServerSettingsData>()
        .expect("should acces to the user overview context")
        .server;
    let edit_server_image = use_server().edit_server_image;

    let image_preview_url = create_rw_signal(server.image_url);

    let form_ref = create_node_ref::<Form>();

    view! {
        <form
            on:submit=move |evt: SubmitEvent| {
                evt.prevent_default();
                let target = evt.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&target).unwrap();
                edit_server_image.dispatch(form_data);
            }
            class="group"
            node_ref=form_ref
        >
        <input
            type="file"
            name="server_image"
            accept="image/*"
            class="w-36 h-36 absolute opacity-0 top-0 z-50 rounded-full cursor-pointer"
            on:change=move |evt| {
                form_ref.get().map(|form| form.request_submit());
                let target = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                if let Some(file) = target.files().and_then(|files| files.item(0)) {
                    image_preview_url
                        .set(Url::create_object_url_with_blob(&Blob::from(file)).ok());
                }
            }
        />
        <input name="server_id" type="hidden" value=server.id.to_string()/>
        {move || match image_preview_url.get() {
            Some(url) => {
                let url = store_value(url);
                view! {
                    <img
                        class="w-36 h-36 absolute top-0 object-cover z-30 rounded-full shadow-xl"
                        src=url.get_value()
                        on:load=move |_| {
                            let _ = Url::revoke_object_url(&url.get_value());
                        }
                    />
                }.into_view()
            }
            None => {
                view! {
                    <div class="w-36 h-36 absolute top-0 object-cover z-30 rounded-full bg-base-content/5 shadow-xl"/>
                }.into_view()
            }
        }}
        <div class="w-36 h-36 absolute top-0 rounded-full transition-opacity bg-base-100/30 opacity-0 group-hover:opacity-100 flex items-center justify-center z-40">
            <p class="text-base-content">"Change Image"</p>
        </div>
        </form>
    }
}
