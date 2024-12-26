use leptos::*;

use crate::app::api::user::use_user;
use crate::app::components::overview::user::content::account::profile::ProfilesSettingsContext;
use wasm_bindgen::JsCast;
use web_sys::{Blob, FormData, HtmlFormElement, HtmlInputElement, Url};

use self::ev::SubmitEvent;
use self::html::Form;
#[component]
pub fn ImageBanner(primary_color_preview: RwSignal<Option<String>>) -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let image_url = context.banner.image_url;
    let image_preview_url = create_rw_signal(image_url);
    let edit_banner_image = use_user().edit_banner_image;
    let form_ref = create_node_ref::<Form>();
    view! {
        <form on:submit=move |evt: SubmitEvent| {
            evt.prevent_default();
            let target = evt.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            edit_banner_image.dispatch(form_data);
        }
        on:reset=move |_| {
            image_preview_url.set(None);
        }
        node_ref=form_ref
        >
            <input type="file" name="banner_image" accept="image/*" class="w-full h-36 absolute opacity-0 top-0 z-50 rounded-t" on:change=move |evt| {
                form_ref.get().map(|form| form.request_submit());
                let target = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                if let Some(file) = target.files().and_then(|files| files.item(0)) {
                    image_preview_url.set(Url::create_object_url_with_blob(&Blob::from(file)).ok());
                }
            } />
            {
                move || match image_preview_url.get() {
                    Some(url) => {
                        let url = store_value(url);
                        view!{
                            <img class="w-full h-36 absolute top-0 object-cover z-30 rounded-t" src=url.get_value() on:load=move |_| {
                                let _ = Url::revoke_object_url(&url.get_value());
                            }/>
                        }.into_view()
                    },
                    None => view!{
                        <div class="w-full h-36 absolute top-0 object-cover z-30 rounded-t bg-primary"/>
                    }.into_view()
                }
            }
            <div class="w-full h-36 absolute top-0 z-50 rounded-t transition-opacity bg-base-100/30 opacity-0 hover:opacity-100 flex items-center justify-center z-40">
                <p class="text-base-content text-xl">"Change Banner"</p>
            </div>
        </form>
    }
}
