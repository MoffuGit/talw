use leptos::*;
use leptos_router::A;

use crate::entities::channel::Channel;

#[component]
pub fn Channel(channel: Channel) -> impl IntoView {
    view! {
        <div class="relative py-[1px] ml-2 transition duration-200 ease-in-out delay-0 hover:bg-primary/60 rounded">
            <A href=move || channel.id.simple().to_string() class="relative box-border py-[6px] px-2 flex flex-col cursor-pointer">
                <div class="relative flex flex-row justify-center items-center">
                    <div class="relative w-[20px] h-[20px] mr-[6px]"/>
                    <div class="whitespace-nowrap overflow-hidden text-ellipsis text-[16px] leading-5 flex-auto relative">
                        {move || channel.name.clone()}
                    </div>
                    // <div class="w-[40px] h-[16px]"/>
                </div>
            </A>
        </div>
    }
}
