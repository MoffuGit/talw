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
    //NOTE: probably the outlet should be outside the transition,
    //or use the new await suspend option
    view! {
        <Transition fallback=move || view!{<Outlet/>}>
            {move || {
                use_auth().auth.and_then(|user| {
                    user.clone().map(|auth_user| {
                            provide_user_context(auth_user.id);
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
                            }
                    })
                })
            }}
        </Transition>
    }
}
