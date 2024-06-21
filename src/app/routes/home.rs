use leptos::*;

use crate::app::api::auth::current_user;
use crate::app::components::navigation::navbar::Navbar;

#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Navbar/>
        <Suspense fallback=move || ()>
            {
                move || {
                    current_user().get().map(|user| match user {
                        Ok(Some(user)) => view! {<div>{user.username}</div>}.into_view(),
                        _ => view! {<div>"error with auth"</div>}.into_view(),
                    })
                }
            }
        </Suspense>
    }
}
