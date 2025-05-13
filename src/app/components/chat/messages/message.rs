use leptos::either::Either;
use leptos::prelude::*;
use reactive_stores::Field;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::markdown::Markdown;
use crate::entities::member::MemberStoreFields;
use crate::entities::message::{ChannelMessage, ChannelMessageStoreFields};

#[component]
pub fn ChatMessage(#[prop(into)] message: Field<ChannelMessage>) -> impl IntoView {
    let member = message.sender();
    view! {
        //MessageContextMenu
        <div class="relative px-4 py-2 hover:bg-base-content/5 flex items-start">
            {
                move || {
                    member.with(|member| {
                        let image_url = StoredValue::new(member.image_url.clone());
                        view!{
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

                        }
                    })
                }
            }
            <div class="flex flex-col items-start">
                <div class="flex items-center mb-1">
                    <div class="font-medium mr-2">
                        {move || member.name().get()}
                    </div>
                    <div class="text-xs text-base-content/50 self-end mb-1">
                        {move || message.timestamp().get().format("%y/%m/%d, %H:%M").to_string()}
                    </div>
                </div>
                <Markdown text=message.content()/>
            </div>
        </div>
    }
}
