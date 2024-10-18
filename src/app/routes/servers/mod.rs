pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;

use crate::app::api::auth::use_auth;
use crate::app::components::navigation::sidebar::SideBar;
use leptos::*;
use leptos_router::Outlet;
use leptos_router::Redirect;

#[allow(non_snake_case)]
#[component]
pub fn Servers() -> impl IntoView {
    view! {
        <Transition fallback=move || ()>
            {move || {
                use_auth().auth.get().map(|result| match result {
                    Ok(Some(user)) => {
                        view! {
                            <div class="h-full w-full">
                                <div class="flex w-[72px] h-full z-30 fixed inset-y-0">
                                    <SideBar user=user/>
                                </div>
                                <div class="h-full relative overflow-hidden md:pl-[72px]">
                                    <Outlet/>
                                </div>
                            </div>
                            // <div id="float_container" class="absolute bg-transparent top-0 left-0 right-0 bottom-0">
                            // </div>
                        }.into_view()
                    },
                    _ => {
                        view!{<Redirect path="/"/>}.into_view()
                    }
                })
            }}
        </Transition>
    }
}
