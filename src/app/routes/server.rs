use crate::app::components::navigation::sidebar::SideBar;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Server() -> impl IntoView {
    view! {
        <div class="h-full w-full">
            //NOTE:
            //voy a intentar insertar este server modal al area de document.body() y de ahi sacar lo
            //demas para los tooltips y mas adelante lo que es mi menu de click derecho
            //la idea es poder crear distintos "portales" donde colocar nuevos elementos en la parte
            //del body, esto para luego poder insertar dentro de este nuevo espacio los componentes que
            //que se deben de colocar de manera absoluta pero sin sentirse restringidos por los
            //elementos relativos

            <div class="flex w-[72px] h-full z-30 fixed inset-y-0">
                <SideBar/>
            </div>
            <div class="h-full relative md:pl-[72px]">
                <Outlet/>
            </div>
        </div>
    }
}
