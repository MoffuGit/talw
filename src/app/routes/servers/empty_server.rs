use std::str::FromStr;

use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::api::channel::{get_all_channels, use_channel};

 
#[component]
pub fn EmptyServer() -> impl IntoView {
    let params = use_params_map();
    let channels = Resource::new(
        move || {
            (
                use_channel().create_channel.version(),
                params
                    .with(|p| Uuid::from_str(&p.get("id").unwrap_or_default()).unwrap_or_default()),
            )
        },
        move |(_, server_id)| get_all_channels(server_id),
    );
    view! {
        <Transition fallback=move || ()>
            {move || match channels.get() {
                None => view! {}.into_any(),
                Some(Ok(channels)) if !channels.is_empty() => {
                    view! { <Redirect path=format!("{}", channels.first().unwrap().id.simple()) /> }
                        .into_any()
                }
                Some(Ok(_)) => view! { <div>"Empty"</div> }.into_any(),
                Some(Err(_)) => view! { <Redirect path="/servers/me" /> }.into_any(),
            }}
        </Transition>
    }
}
