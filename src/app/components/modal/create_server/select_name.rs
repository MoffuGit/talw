use super::use_create_server;
use crate::app::api::auth::use_auth;
use crate::app::api::server::use_server;
use crate::app::components::ui::modal::slide_modal::SlideBack;
use crate::app::components::ui::modal::ModalClose;
use icondata;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;
use leptos_icons::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, FormData, HtmlFormElement, Url};

#[allow(non_snake_case)]
#[component]
pub fn SelectName() -> impl IntoView {
    let create_server = use_server().create_server;
    let select_name_ref = use_create_server().select_name_ref;
    let user = move || {
        use_auth().auth.get().map(|user| match user {
            Ok(Some(user)) => format!("{}'s server", user.username),
            _ => "User".to_string(),
        })
    };
    let image_input_ref = create_node_ref::<Input>();
    let image_preview_url = create_rw_signal::<String>("".into());

    view! {
            <form node_ref=select_name_ref on:submit=move |ev: SubmitEvent| {
                ev.prevent_default();
                let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
                let form_data = FormData::new_with_form(&target).unwrap();
                create_server.dispatch(form_data);
            }
            on:reset=move |_| {
                image_preview_url.set("".to_string());
            }
            >
                <div class="px-6 pt-6 relative flex flex-col justify-start items-center ">
                    <div class="text-center font-bold text-2xl leading-[30px]">Customize your server</div>
                    <div class="mt-2 text-center text-base leading-5 font-normal">Give your new server a personality with a name and an icon. You can always change it later.</div>
                    <ModalClose attr:type="reset" class="absolute right-2 top-2 flex items-center group bg-none">
                        <Icon icon=icondata::RiCloseSystemLine class="group-hover:fill-neutral fill-neutral-content w-8 h-8 transition-all"/>
                    </ModalClose>
                </div>
                <div class="px-2 my-4">
                    <div class="pt-1 flex justify-center">
                        <div class="w-20 h-20 flex-justify-center relative" >
                            <input type="file" name="image" accept="image/*" class="absolute top-0 left-0 w-full h-20 opacity-0 z-50" node_ref=image_input_ref on:change=move |_| {
                                if let Some(node) = image_input_ref.get() {
                                    if let Some(file) = node.files().and_then(|files| files.item(0)) {
                                        image_preview_url.set(Url::create_object_url_with_blob(&Blob::from(file)).unwrap_or_default())
                                    }
                                }
                            } />
                            {
                                move || match image_preview_url.get().len() {
                                    0 => view!{
                                        <div class="indicator w-20 h-20 flex flex-col justify-center items-center rounded-full border-2 border-base-content border-dashed relative z-30">
                                            <span class="indicator-item badge badge-primary translate-x-[11%] translate-y-[20%] w-[20px] h-[20px] p-0">
                                                <Icon icon=icondata::RiAddSystemFill class="fill-primary-content w-4 h-4"/>
                                            </span>
                                            <div class="flex flex-col items-center">
                                                <Icon icon=icondata::RiCameraMediaFill class="fill-base-content w-6 h-6 "/>
                                                <p class="uppercase fill-base-content text-xs font-bold">upload</p>
                                            </div>
                                        </div>
                                    }.into_view(),
                                    _ => view!{
                                        <img class="w-20 h-20 object-cover absolute top-0 left-0 z-40 rounded-full" src=move || image_preview_url.get() on:load=move |_| {
                                            let _ = Url::revoke_object_url(&image_preview_url.get());
                                        }/>
                                    }.into_view()
                                }
                            }
                        </div>
                    </div>
                    <div class="mt-6">
                        {
                            move || {
                                create_server.value().get().map(|res| match res {
                                    Err(ServerFnError::ServerError(err)) => view! { <p class="text-error mb-2 text-xs italic">{err}</p>},
                                    _ => view! { <p/>},
                                })
                            }
                        }
                        <label class="mb-2 text-xs leading-4 font-bold">SERVER NAME</label>
                        <div class="flex flex-col">
                            <Transition fallback=move || ()>
                                <input name="name" class="input input-secondary font-medium p-[10px] h-10 text-base w-full rounded-[3px]" type="text" placeholder=user />
                            </Transition>
                        </div>
                    </div>
                    <div class="mt-2 pb-1 text-xs leading-4 font-normal">"By creating a server, you agree to Discord's Community Guidelines."</div>
                </div>
                <div class="relative p-4 overflow-x-auto flex justify-between items-center bg-base-300/50">
                    <SlideBack attr:type="reset" class="w-auto min-h-min h-[38px] py-0.5 px-1 leading-[16px] hover:underline text-base-content text-[14px]">
                            Back
                    </SlideBack>
                    <button type="submit" class="bg-primary hover:bg-primary-focus text-neutral py-0.5 px-4 font-medium text-[14px] leading-[16px] align-middle rounded-[3px] h-[38px] no-animation" disabled=move || create_server.pending().get()>Create Server</button>
                </div>
            </form>
    }
}
