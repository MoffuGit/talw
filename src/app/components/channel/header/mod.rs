pub mod thread_menu;
pub mod title;

use crate::app::components::channel::header::title::HeaderTitle;
use crate::app::components::channel::sidebars::MemberSideBarTrigger;
use crate::app::components::navigation::server::sidebar::ServerSideBarTrigger;
use crate::entities::channel::Channel;
use crate::entities::thread::Thread;
use icondata;
use leptos::*;
use leptos_icons::Icon;

use self::thread_menu::ThreadMenu;

#[allow(non_snake_case)]
#[component]
pub fn ChannelHeader(channel: Channel, #[prop(optional)] thread: Option<Thread>) -> impl IntoView {
    view! {
        <div class="relative bg-base-300 shadow shadow-base-300/80 min-h-[48px] max-h-[48px] w-full flex justify-between align-middle">
            <div class="relative flex-auto flex items-center overflow-hidden p-2">
                <ServerSideBarTrigger class="relative flex items-center mr-2">
                    <Icon icon=icondata::BsLayoutSidebar class="w-6 h-6 ml-1 fill-base-content/40"/>
                </ServerSideBarTrigger>
                {
                    match &thread {
                        Some(thread) => view!{<HeaderTitle channel=channel.clone() thread=thread.clone()/>},
                        None => view!{<HeaderTitle channel=channel.clone() />}
                    }
                }

            </div>
            <div class="h-auto relative flex items-center p-2 space-x-3 mr-3">
                {
                    if thread.is_none() {
                        view!{<ThreadMenu channel_id=channel.id server_id=channel.server_id/>}
                    } else {
                        ().into_view()
                    }
                }
                <Icon icon=icondata::RiPushpinMapFill class="w-6 h-6 fill-base-content/40"/>
                <MemberSideBarTrigger/>
                <Icon icon=icondata::RiSearchSystemLine class="w-6 h-6 fill-base-content/40 mr-0.5" />
            </div>
        </div>
    }
}
