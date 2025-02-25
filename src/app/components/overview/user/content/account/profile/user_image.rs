use leptos::either::Either;
use leptos::prelude::*;
use leptos_icons::Icon;
use wasm_bindgen::JsCast;

use crate::app::api::user::use_user;
use crate::app::components::overview::user::content::account::profile::ProfilesSettingsContext;

use web_sys::{Blob, FormData, HtmlFormElement, HtmlInputElement, Url};

use leptos::ev::SubmitEvent;
use leptos::html::Form;

#[component]
pub fn UserImage() -> impl IntoView {
    let context = use_context::<ProfilesSettingsContext>()
        .expect("should acces to the user overview context");
    let image_url = context.profile.image_url;
    let image_preview_url = RwSignal::new(image_url);
    let edit_user_image = use_user().edit_profile_image;
    let form_ref = NodeRef::<Form>::new();
    view! {
        <form
            on:submit=move |evt: SubmitEvent| {
                evt.prevent_default();
                let target = evt.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&target).unwrap();
                edit_user_image.dispatch_local(form_data);
            }
            on:reset=move |_| {
                image_preview_url.set(None);
            }
            node_ref=form_ref
            class="group"
        >
            <input
                type="file"
                name="user_image"
                accept="image/*"
                class="w-36 h-36 absolute top-16 left-6 rounded-full opacity-0 z-[60] cursor-pointer"
                on:change=move |evt| {
                    form_ref.get().map(|form| form.request_submit());
                    let target = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if let Some(file) = target.files().and_then(|files| files.item(0)) {
                        image_preview_url
                            .set(Url::create_object_url_with_blob(&Blob::from(file)).ok())
                    }
                }
            />
            {move || match image_preview_url.get() {
                Some(url) => {
                    let url = StoredValue::new(url);
                    Either::Left(
                        view! {
                            <img
                                class="w-36 h-36 object-cover absolute top-16 left-6 rounded-full border-8 border-base-300 z-40"
                                src=url.get_value()
                                on:load=move |_| {
                                    let _ = Url::revoke_object_url(&url.get_value());
                                }
                            />
                        },
                    )
                }
                None => {
                    Either::Right(
                        view! {
                            <div class="w-36 h-36 absolute top-16 left-6 rounded-full border-8 bg-base-content/5 border-base-300 z-40" />
                        },
                    )
                }
            }}
            <div class="w-32 h-32 absolute top-[136px] left-8 rounded-full border-8 border-transparent transition opacity-0 group-hover:opacity-100 bg-base-content/10 z-50 flex items-center justify-center">
                <Icon icon=icondata::RiPencilDesignFill />
            </div>
        </form>
    }
}
