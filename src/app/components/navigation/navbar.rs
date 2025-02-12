use crate::app::api::auth::use_auth;
use crate::app::components::theme::Toggle_Theme;
use leptos::prelude::*;
use leptos_router::components::A;

#[allow(non_snake_case)]
#[component]
pub fn Logout() -> impl IntoView {
    let logout = use_auth().logout;
    view! {
        <ActionForm action=logout /* class="w-auto h-auto" */>
            <button type="submit" class="btn btn-sm">
                "Logout"
            </button>
        </ActionForm>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <Transition>
            <div class="navbar w-auto">
                <div class="navbar-start" />
                <div class="navbar-end">
                    {move || {
                        Suspend::new(async move {
                            use_auth()
                                .auth.await.map(|user| {
                                    match user {
                                        Some(_) => view! { <Logout /> }.into_any(),
                                        _ => view! {
                                            <A href="/login" {..} class="btn btn-ghost btn-sm m-1">
                                                "Login"
                                            </A>
                                            <A href="/signup" {..} class="btn btn-neutral btn-sm">
                                                "Signup"
                                            </A>
                                        }
                                        .into_any()
                                    }
                            }).into_any()
                        })
                    }} <div class="divider divider-horizontal m-0 self-center h-6" />
                    // <Toggle_Theme class="btn btn-sm btn-square btn-accent" />
                </div>
            </div>
        </Transition>
    }
}
