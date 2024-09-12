pub mod title;

use crate::app::components::channel::header::title::HeaderTitle;
use crate::app::components::channel::sidebars::members::MemberSideBarTrigger;
use icondata;
use leptos::*;
use leptos_icons::Icon;

use crate::entities::channel::Channel;

#[allow(non_snake_case)]
#[component]
pub fn ChannelHeader(channel: Channel, #[prop(optional)] thread: Option<String>) -> impl IntoView {
    view! {
        <div class="relative shadow shadow-base-300/80 min-h-[48px] max-h-[48px] w-full flex justify-between align-middle">
            <div class="relative flex-auto flex items-center overflow-hidden p-2">
                <HeaderTitle channel=channel thread=thread/>
            </div>
            <div class="h-auto relative flex items-center p-2 space-x-2">
                <Icon icon=icondata::RiDiscussCommunicationFill class="w-7 h-7 fill-base-content/40" />
                // <Icon icon=icondata::RiNotificationOffMediaFill class="w-6 h-6 fill-base-content/40" />
                <Icon icon=icondata::RiPushpinMapFill class="w-7 h-7 fill-base-content/40"/>
                <MemberSideBarTrigger/>
                <div class="mx-2 flex items-center justify-between h-7 w-[144px] rounded px-0.5 bg-base-300">
                    <div class="text-sm ml-0.5 text-base-content/40">"Search"</div>
                    <Icon icon=icondata::RiSearchSystemFill class="w-5 h-5 fill-base-content/40 mr-0.5" />
                </div>
            </div>
        </div>
    }
}
