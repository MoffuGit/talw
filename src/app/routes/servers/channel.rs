use leptos::*;
use leptos_router::Outlet;

#[allow(non_snake_case)]
#[component]
pub fn Channel() -> impl IntoView {
    view! {
        <div class="w-full h-full flex relative items-stretch">
            <div class="grow min-w-[400px] shrink-0" />
            <Outlet/>
        </div>
    }
}
