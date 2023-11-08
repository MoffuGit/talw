use crate::app::auth::{current_user, use_auth};
use crate::app::components::theme::Toggle_Theme;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Logout() -> impl IntoView {
    let logout = use_auth().logout;
    view! {
        <ActionForm action=logout class="w-auto h-auto">
            <button type="submit" class="btn btn-sm">"Logout"</button>
        </ActionForm>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <Transition fallback=move || ()>
            <div class="navbar w-auto">
                <div class="navbar-start"/>
                <div class="navbar-end">
                    {
                        move || current_user().get().map(|user| match user {
                            Ok(Some(_)) => view! {<Logout/>}.into_view(),
                            _ => view!{<A href="/login" class="btn btn-ghost btn-sm m-1">"Login"</A> <A href="/signup" class="btn btn-neutral btn-sm">"Signup"</A>}.into_view()
                        })
                    }
                    <div class="divider divider-horizontal m-0 self-center h-6"/>

                    <Toggle_Theme class="btn btn-sm btn-square btn-accent" />
                </div>
            </div>
        </Transition>
    }
}
