mod menu;

use self::menu::ThreadMenu;
use crate::app::components::navigation::server::use_current_channel;
use crate::app::components::thread::sidebar::CurrentThreadContext;
use crate::app::routes::servers::server::use_current_server_context;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn ThreadHeader() -> impl IntoView {
    let server_id = use_current_server_context().server.id;
    let channel_id = move || use_current_channel().get().unwrap();
    view! {
        <div class="relative bg-base-300 shadow shadow-base-300/80 h-11 w-full flex justify-between align-middle p-2">
            <ThreadTitle />
            <div class=" flex items-center space-x-2">
                <ThreadMenu />
                <A
                    href=move || {
                        format!("/servers/{}/{}", server_id.simple(), channel_id().simple())
                    }
                    class="inline-block p-1 hover:bg-base-content/5 rounded-lg"
                >
                    <Icon icon=icondata::LuX class="w-5 h-5" />
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn ThreadTitle() -> impl IntoView {
    let current_thread =
        use_context::<CurrentThreadContext>().expect("SHould return the current thrread context");
    view! {
        <div class="relative flex items-center p-1.5 text-base select-none">
            <Icon icon=icondata::RiDiscussCommunicationFill class="w-5 h-5 mr-2" />
            <div>{current_thread.thread.name}</div>
        </div>
    }
}
