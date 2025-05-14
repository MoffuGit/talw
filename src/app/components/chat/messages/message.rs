use leptos::either::Either;
use leptos::prelude::*;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::markdown::Markdown;
use crate::entities::message::ChannelMessage;

#[component]
pub fn ChatMessage(message: ChannelMessage) -> impl IntoView {
    let member = message.sender;
    let image_url = StoredValue::new(member.image_url.clone());
    view! {
        //MessageContextMenu
        <div class="relative px-4 py-2 hover:bg-base-content/5 flex items-start text-wrap whitespace-break-spaces">
            <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=member.clone() class="w-auto h-auto mr-4" >
                {move || if let Some(url) = image_url.get_value() {
                    Either::Left(
                        view! {
                            <img
                                class="rounded-full object-cover w-10 h-10"
                                src=url
                            />
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <div class="rounded-full bg-base-content/10 w-10 h-10" />
                        },
                    )
                }}
            </MemberBanner>
            <div class="flex flex-col items-start">
                <div class="flex items-center mb-1">
                    <div class="font-medium mr-2">
                        {member.name}
                    </div>
                    <div class="text-[11px] text-base-content/50 self-end mb-0.5">
                        {message.timestamp.format("%d/%m/%y, %H:%M").to_string()}
                    </div>
                </div>
                <Markdown text=message.content/>
            </div>
        </div>
    }
}
