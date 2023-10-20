use crate::app::{components::navigation::item::Item, server::get_user_servers};
use leptos::*;

#[component]
pub fn SideBar() -> impl IntoView {
    // let create_server = create_server_action();
    let get_servers = create_resource(move || (), move |_| get_user_servers());
    view! {
            <div class="w-full h-full flex flex-col items-center bg-base-200">
                <Transition fallback=move || ()>
                    {move || get_servers.and_then(|servers| servers.iter().map(|server| view! {
                        <div class="mb-4" >
                            <Item id=server.id name=server.name.clone()/>
                        </div>
                    }).collect_view())}
                </Transition>
            </div>
    }
}
