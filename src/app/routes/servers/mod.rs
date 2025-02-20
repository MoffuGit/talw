pub mod channel;
pub mod empty_server;
pub mod server;
pub mod thread;
use crate::app::api::auth::use_auth;
use crate::app::api::user::provide_user_context;
use crate::app::components::navigation::sidebar::SideBar;
use crate::app::components::overview::server::provide_server_overview_context;
use crate::app::components::overview::server::ServerOverview;
use crate::app::components::overview::user::provide_user_overview_context;
use crate::app::components::overview::user::UserOverview;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::components::Redirect;
use leptos_router::components::A;

#[component]
pub fn Servers() -> impl IntoView {
    let auth = use_auth().auth;
    let outer_owner = Owner::current().unwrap();

    let inner_view = move || {
        auth.and_then(|user| {
            user.clone().map(|user| {
                outer_owner.with(|| {
                    provide_user_context(user.id);
                    provide_user_overview_context();
                    provide_server_overview_context();
                });
                view! {
                    <UserOverview/>
                    <ServerOverview/>
                    //NOTE:
                    <div class="h-full w-full relative z-40 flex">
                        <div class="flex w-12 h-full z-30 fixed inset-y-0">
                            <SideBar />
                        </div>
                        <div class="h-full w-full relative overflow-hidden md:pl-12">
                            <Outlet />
                        </div>
                    </div>
                }
            })
        })
    };

    view! {
        <Transition>
            {inner_view}
        </Transition>
    }
}
