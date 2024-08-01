use leptos::*;

use crate::entities::channel::Channel;

#[allow(non_snake_case)]
#[component]
pub fn ChannelHeader(channel: Channel, #[prop(optional)] thread: Option<String>) -> impl IntoView {
    view! {
        <div class="relative shadow shadow-base-300/80 min-h-[48px] w-full flex align-middle p-2">
            <div class="relative flex-auto flex items-center overflow-hidden">
                {channel.name}
            </div>
        </div>
    }
}
