use crate::app::api::server::check_server;
use crate::app::api::server::get_member;
use crate::app::api::server::use_server;
use crate::app::api::uploadthing::get_list_files;
use crate::app::api::uploadthing::upload_file;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use leptos::*;
use leptos_router::use_params_map;
use leptos_router::Outlet;
use leptos_router::Redirect;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::FormData;
use web_sys::HtmlFormElement;
use web_sys::SubmitEvent;

#[allow(non_snake_case)]
#[component]
pub fn Server() -> impl IntoView {
    let params = use_params_map();
    //NOTE: las acciones las vamos a crear en el contexto del server y ya luego subscribimos los
    //resources a esas, server_settings,
    let leave_server = use_server().leave_server;
    let server = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| check_server(server_id),
    );
    let member = create_resource(
        move || {
            (
                leave_server.version().get(),
                params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| get_member(server_id),
    );
    let list_files = create_resource(|| (), |_| get_list_files());
    let upload_action = create_action(|data: &FormData| {
        let data = data.clone();
        upload_file(data.into())
    });

    view! {
            <div class="h-full w-full relative z-40">
                <div class="flex w-[240px] h-full fixed inset-y-0 bg-base-200 z-40">
                    <Transition fallback=move || ()>
                    {
                        move || {
                            server.get().map(|server| {
                                member.get().map(|member| {
                                    if let (Ok(server), Ok(member)) = (server, member) {
                                        view! {
                                            <ServerSideBar server=server member=member/>
                                        }.into_view()
                                    } else {
                                        view!{<Redirect path="/servers/me"/>}.into_view()
                                    }
                                })
                            })
                        }
                    }
                    </Transition>
                </div>

                <div class="h-full relative overflow-hidden md:pl-[240px] z-30">
                    <Transition fallback=move || ()>
                    {
                        move || {
                            list_files.and_then(|files| files.files.iter().map(|file| view! {<div>{file.name.clone()}</div>} ).collect_view())
                        }
                    }
                    </Transition >
                  <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch(form_data);
        }>
            <input type="file" name="file_to_upload"/>
            <input type="submit"/>
        </form>
        <p>
            {move || if upload_action.input().get().is_none() && upload_action.value().get().is_none() {
                "Upload a file.".to_string()
            } else if upload_action.pending().get() {
                "Uploading...".to_string()
            } else if let Some(Ok(value)) = upload_action.value().get() {
                value.to_string()
            } else {
                format!("{:?}", upload_action.value().get())
            }}
        </p>
                    <Outlet/>
                </div>

            </div>
    }
}
