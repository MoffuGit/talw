use crate::app::api::server::check_memeber;
use crate::app::components::navigation::server::sidebar::ServerSideBar;
// use crate::app::components::ui::portal::*;
// use crate::app::components::ui::dropdown_menu::*;
use leptos::*;
use leptos_router::use_params_map;
use leptos_router::Outlet;
use leptos_router::Redirect;
use uuid::Uuid;

#[component]
pub fn Server() -> impl IntoView {
    let params = use_params_map();
    //NOTE: las acciones las vamos a crear en el contexto del server y ya luego subscribimos los
    //resources a esas server_settings
    ////NOTE: cambiar check_memeber to check_server
    let server = create_resource(
        move || params.with(|p| Uuid::parse_str(p.get("id").unwrap()).unwrap_or_default()),
        check_memeber,
    );
    // let one = create_rw_signal(0);
    // let two = create_rw_signal(0);
    // let three = create_rw_signal(0);
    // let four = create_rw_signal(0);
    //
    // create_effect(move |_| log::info!("signal one: {}", one.get()));
    // create_effect(move |_| log::info!("signal two: {}", two.get()));
    // create_effect(move |_| log::info!("signal three: {}", one.get()));
    // create_effect(move |_| log::info!("signal four: {}", two.get()));

    view! {
            <Transition fallback=move || ()>
                <div class="h-full w-full relative z-40">
                    <div class="flex w-[240px] h-full fixed inset-y-0 bg-base-200 z-40">
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

                    <div class="h-full relative overflow-hidden md:pl-[240px] z-30">
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
                        // <div class="pl-[600px] flex pt-20 z-40">
                        //     <DropdownProvider>
                        //         <DropdownTrigger class="bg-blue-500 w-8 h-4 mr-30"/>
                        //         <DropdownContent class="bg-red-500 w-20 h-10 z-50".to_string() side=MenuSide::Bottom >
                        //             <div class="bg-white w-2 h-2" on:click=move |_| one.update(|value| *value += 1)/>
                        //         </DropdownContent>
                        //     </DropdownProvider>
                        //
                        //     <DropdownProvider>
                        //         <DropdownTrigger class="bg-emerald-500 w-8 h-4 mr-30"/>
                        //         <DropdownContent class="bg-pink-500 w-20 h-10 z-50".to_string() side=MenuSide::Top>
                        //             <div class="bg-blue-500 w-2 h-2" on:click=move |_| two.update(|value| *value += 1)>
                        //                 "two"
                        //             </div>
                        //         </DropdownContent>
                        //     </DropdownProvider>
                        //
                        //     <DropdownProvider>
                        //         <DropdownTrigger class="bg-gray-500 w-8 h-4 mr-30"/>
                        //         <DropdownContent class="bg-blue-500 w-20 h-10 z-50".to_string() side=MenuSide::Left>
                        //             <div class="bg-blue-500 w-2 h-2" on:click=move |_| three.update(|value| *value += 1)>
                        //                 "two"
                        //             </div>
                        //         </DropdownContent>
                        //     </DropdownProvider>
                        //
                        //     <DropdownProvider>
                        //         <DropdownTrigger class="bg-orange-500 w-8 h-4 mr-30"/>
                        //         <DropdownContent class="bg-indigo-500 w-20 h-10 z-50".to_string() side=MenuSide::Right>
                        //             <div class="bg-blue-500 w-2 h-2" on:click=move |_| four.update(|value| *value += 1)>
                        //                 "two"
                        //             </div>
                        //         </DropdownContent>
                        //     </DropdownProvider>
                        // </div>
                        <Outlet/>
                    </div>
                </div>
            </Transition>
        }
}
