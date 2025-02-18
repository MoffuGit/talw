pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;
use crate::app::api::auth::use_auth;
use crate::app::api::user::provide_user_context;
use crate::app::components::navigation::sidebar::SideBar;
use crate::app::components::overview::server::ServerOverview;
use crate::app::components::overview::user::UserOverview;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::components::Redirect;
use leptos_router::components::A;

#[allow(non_snake_case)]
#[component]
pub fn Servers() -> impl IntoView {
    view! {
        <Transition>
            {move || {
                use_auth().auth.and_then(|user| {
                    user.clone().map(|user| {
                        provide_user_context(user.id);
                        view!{
                            <UserOverview>
                                <ServerOverview>
                                    <div class="h-full w-full">
                                        <div class="flex w-12 h-full z-30 fixed inset-y-0">
                                            <SideBar />
                                        </div>
                                        <div class="h-full relative overflow-hidden md:pl-12">
                                            <Outlet />
                                        </div>
                                    </div>
                                </ServerOverview>
                            </UserOverview>
                        }.into_any()
                    })
                })
            }}
        </Transition>
    }
}
