use crate::{app::server::get_channels, entities::server::Server};
use leptos::*;

#[component]
pub fn ServerSideBar(server: Server) -> impl IntoView {
    let channels = create_resource(|| (), move |_| get_channels(server.id));
    view! {
        <div class="w-full h-full flex flex-col items-center relative bg-base-200 scrollbar-none overflow-y-scroll overflow-x-hidden">
            <div class="relative w-full cursor-pointer">
                <div class="relative font-medium py-3 px-4 shadow shadow-base-300/80">
                    <div class="h-6 flex items-center">
                        <div class="mr-2"/>
                        <div class="flex-1 flex items-center text-base font-bold overflow-hidden text-ellipsis whitespace-nowrap min-w-0">
                            {server.name}
                        </div>
                        <div class="relative"/>
                    </div>
                </div>
            </div>
            <Transition fallback=move || ()>
                {
                    move || {
                        channels.and_then(|channels| {
                            channels.iter().map(|channel| {
                                view! {<div>{&channel.name}</div>}
                            }).collect_view()
                        })
                    }
                }
            </Transition>
            //NOTE:
            //escribir un prueba del portal como segun yo si logras hacer funcionar el componente
            //y que no se confunda con los contextos tomando el de la copia mas nueva en vez del
            //suyo
        </div>
    }
}
