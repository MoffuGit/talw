pub mod user;

use crate::app::components::ui::overview::*;
use leptos::*;

#[derive(Clone)]
struct ServerOverviewContext {
    open: RwSignal<bool>,
}

#[component]
pub fn ServerOverview(children: Children) -> impl IntoView {
    let open = create_rw_signal(false);
    create_effect(move |_| {
        if open.get() {
            log::info!("you open the server overview")
        } else {
            log::info!("you close the server overview")
        }
    });
    provide_context(ServerOverviewContext { open });
    view! {
        {children()}
        <OverviewContent open=open class="bg-base-200">
            <div>"Overview of server"</div>
        </OverviewContent>
    }
}

#[component]
pub fn ServerOverviewTrigger(children: Children, class: &'static str) -> impl IntoView {
    let open = use_context::<ServerOverviewContext>()
        .expect("should acces to the server overview context")
        .open;
    view! {
        <OverviewTrigger open=open class=class>
            {children()}
        </OverviewTrigger>
    }
}
