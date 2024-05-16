use super::use_current_channel;
use crate::entities::channel::Channel;
use crate::entities::channel::ChannelType;
use icondata;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn Channel(channel: Channel) -> impl IntoView {
    let use_current_channel = use_current_channel();

    let is_current_channel = move || {
        use_current_channel.with(|current| current.is_some_and(|current| current == channel.id))
    };

    view! {
        <div class=move || format!("relative py-[1px] ml-2 transition duration-200 ease-in-out delay-0 group rounded hover:bg-primary/75 mt-0.5 {}", match is_current_channel() {
            true => "bg-primary/50",
            false => "",
        })>
            <A href=move || channel.id.simple().to_string() class="relative box-border py-[6px] px-2 flex flex-col cursor-pointer">
                <div class="relative flex flex-row justify-center items-center">
                    // <Icon icon=MaybeSignal::derive()
                    {
                        match channel.channel_type {
                            ChannelType::TEXT => view! {<Icon icon=icondata::RiHashtagEditor class="relative w-[18px] h-[18px] mr-[6px] fill-base-content"/>},
                            ChannelType::VOICE => view! {<Icon icon=icondata::RiVolumeUpMediaFill class="relative w-[18px] h-[18px] mr-[6px] fill-base-content"/>},
                            ChannelType::ANNOUNCEMENTS => view! {<Icon icon=icondata::RiMegaphoneBusinessFill class="relative w-[18px] h-[18px] mr-[6px] fill-base-content"/>},
                            ChannelType::RULES => view! {<Icon icon=icondata::RiBookMarkDocumentFill class="relative w-[18px] h-[18px] mr-[6px] fill-base-content"/>},
                        }
                    }
                    <div class=move || format!("whitespace-nowrap overflow-hidden text-ellipsis text-[16px] font-bold text-base-content/50 leading-5 flex-auto relative group-hover:text-base-content/75 {}", match is_current_channel() {
                        true => "text-base-content/60",
                        false => ""
                    })>
                        {move || channel.name.clone()}
                    </div>
                </div>
            </A>
        </div>
    }
}
