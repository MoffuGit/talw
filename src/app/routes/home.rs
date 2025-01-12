use crate::app::api::auth::use_auth;
use leptos::*;

use crate::app::components::navigation::navbar::Navbar;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar />
        <Suspense fallback=move || ()>
            {move || {
                use_auth()
                    .auth
                    .get()
                    .map(|user| match user {
                        Ok(Some(user)) => view! { <div>{user.name}</div> }.into_view(),
                        _ => view! { <div>"error with auth"</div> }.into_view(),
                    })
            }}
        </Suspense>
    }
}
