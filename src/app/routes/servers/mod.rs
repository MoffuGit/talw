pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;

use crate::app::api::auth::use_auth;
use crate::app::api::user::provide_user_context;
use crate::app::components::navigation::sidebar::SideBar;
use crate::app::components::overview::user::UserOverview;
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
                        provide_user_context(user.id);
                        view! {
                            <UserOverview >
                                <div class="h-full w-full">
                                    <div class="flex w-[72px] h-full z-30 fixed inset-y-0">
                                        <SideBar />
                                    </div>
                                    <div class="h-full relative overflow-hidden md:pl-[72px]">
                                        <Outlet/>
                                    </div>
                                </div>
                            </UserOverview>
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
