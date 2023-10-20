use crate::app::components::navigation::sidebar::SideBar;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Server() -> impl IntoView {
    view! {
        <div class="h-full w-full">
            <div class="flex w-[72px] h-full z-30 fixed inset-y-0">
                <SideBar/>
            </div>
            <div class="h-full relative md:pl-[72px]">
                <Outlet/>
            </div>
        </div>
    }
}
