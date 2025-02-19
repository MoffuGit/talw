use crate::app::api::auth::use_auth;
use leptos::prelude::*;

use crate::app::components::navigation::navbar::Navbar;

 
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar />
        <Transition>
            {move || {
                Suspend::new(async move {
                    use_auth().auth.await.map(|user| {
                        match user {
                            Some(user) => view! { <div>{user.name}</div> }.into_any(),
                            _ => view! { <div>"error with auth"</div> }.into_any(),
                        }
                    }).into_any()
                })
            }}
        </Transition>
    }
}
