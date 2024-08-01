use leptos::*;
use leptos_router::Outlet;

async fn fet_cat(cat: i32) -> Result<i32, ServerFnError> {
    Ok(cat)
}

#[component]
pub fn ChannelView() -> impl IntoView {
    let cat = create_rw_signal(1);
    let some = create_resource(move || cat.get(), move |cat| fet_cat(cat));
    view! {
        <div class="w-full h-full flex relative items-stretch">
            <div class="grow min-w-[400px] shrink-0" >
            </div>
            <Outlet/>
        </div>
    }
}
