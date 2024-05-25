use crate::app::api::server::check_memeber;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
use crate::app::components::ui::portal::*;
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

                    <div class="h-full relative overflow-hidden md:pl-[240px] z-50">
    //                     <ProvidePortalContext name="one">
    //   <PortalTrigger class="bg-red-500 z-50"/>
    //   <PortalContent class=" ">
    //     <ClosePortal class=" absolute w-20 h-8 bg-pink-500 -translate-x-20 left-0 right-0 top-0 bottom-0 m-auto z-50"/>
    //   </PortalContent>
    // </ProvidePortalContext>
    //
    //         <ProvidePortalContext name="two">
    //   <PortalTrigger class="bg-blue-500 z-50"/>
    //   <PortalContent class="">
    //     <ClosePortal class="absolute w-20 h-8 bg-green-500 -translate-x-20 left-0 right-0 top-0 bottom-0 m-auto z-50"/>
    //   </PortalContent>
    // </ProvidePortalContext>
    //         <ProvidePortalContext name="three">
    //   <PortalTrigger class="bg-yellow-500 z-50"/>
    //   <PortalContent class="">
    //     <ClosePortal class="absolute w-20 h-8 bg-orange-500 -translate-x-20 left-0 right-0 top-0 bottom-0 m-auto z-50"/>
    //   </PortalContent>
    // </ProvidePortalContext>
    //
    //         <ProvidePortalContext name="four">
    //   <PortalTrigger class="bg-purple-500 z-50"/>
    //   <PortalContent class="">
    //     <ClosePortal class="absolute w-20 h-8 bg-lime-500 -translate-x-20 left-0 right-0 top-0 bottom-0 m-auto z-50"/>
    //   </PortalContent>
    // </ProvidePortalContext>

                        <Outlet/>
                    </div>
                </div>
            </Transition>
        }
}
