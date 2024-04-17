use crate::app::api::server::check_memeber;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use leptos::*;
use leptos_router::use_params_map;
use leptos_router::Outlet;
use leptos_router::Redirect;
use uuid::Uuid;

#[component]
pub fn Server() -> impl IntoView {
    let params = use_params_map();
    let server = create_resource(
        move || params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
        check_memeber,
    );

    view! {
        <Transition fallback=move || ()>
            <div class="h-full w-full">
                <div class="flex w-[240px] h-full z-30 fixed inset-y-0 bg-base-200">
                    <Suspense fallback=move || ()>
                    {
                        move || {
                            server.get().map(|result| {
                                if let Ok(server) = result {
                                    view! {
                                        <ServerSideBar server=server/>
                                    }.into_view()
                                } else {
                                    view!{<Redirect path="/servers/me"/>}.into_view()
                                }
                            })
                        }
                    }
                    </Suspense>
                </div>

                <div class="h-full relative overflow-hidden md:pl-[240px]">
                    <Outlet/>
                </div>
            </div>
        </Transition>
    }
}
