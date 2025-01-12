use std::str::FromStr;

use leptos::*;
use leptos_router::{use_params_map, Redirect};
use uuid::Uuid;

use crate::app::api::channel::{get_all_channels, use_channel};

#[allow(non_snake_case)]
#[component]
pub fn EmptyServer() -> impl IntoView {
    let params = use_params_map();
    let channels = create_resource(
        move || {
            (
                use_channel().create_channel.version(),
                params.with(|p| Uuid::from_str(p.get("id").unwrap()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| get_all_channels(server_id),
    );
    view! {
        <Transition fallback=move || ()>
            {move || match channels.get() {
                None => view! {}.into_view(),
                Some(Ok(channels)) if !channels.is_empty() => {
                    view! { <Redirect path=format!("{}", channels.first().unwrap().id.simple()) /> }
                        .into_view()
                }
                Some(Ok(_)) => view! { <div>"Empty"</div> }.into_view(),
                Some(Err(_)) => view! { <Redirect path="/servers/me" /> }.into_view(),
            }}
        </Transition>
    }
}
