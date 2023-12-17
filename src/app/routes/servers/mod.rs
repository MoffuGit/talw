pub mod server;

use crate::app::components::navigation::sidebar::SideBar;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Servers() -> impl IntoView {
    view! {
        <div class="h-full w-full">
            <div class="flex w-[72px] h-full z-30 fixed inset-y-0">
                <SideBar/>
            </div>
            <div  id="tooltip_layer" class="h-full relative overflow-hidden md:pl-[72px]">
                <Outlet/>
            </div>
        </div>
    }
}
