use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::BlockQuoteKind;
use reactive_stores::{Field, Store};
use uuid::Uuid;

use crate::app::api::messages::get_pinned_messages;
use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::*;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::markdown::styled::Markdown;
use crate::app::components::ui::markdown::MarkdownParser;
use crate::entities::member::MemberStoreFields;
use crate::entities::message::{ChannelMessage, ChannelMessageStoreFields};

#[derive(Debug, Store)]
struct MessageStore {
    #[store(key: Uuid = |category| category.id)]
    messages: Vec<ChannelMessage>,
}

#[component]
pub fn Pinned(#[prop(into)] channel_id: Field<Uuid>) -> impl IntoView {
    let pinned = Resource::new(move || (channel_id.get()), get_pinned_messages);
    let open = RwSignal::new(false);
    view! {
        <DropdownProvider open=open modal=false>
            <DropdownTrigger class="hover:bg-base-100 rounded-md flex items-center justify-center w-7 h-7 cursor-pointer select-none">
                <Icon icon=IconData::Pin class="h-4 w-4"/>
            </DropdownTrigger>
            <DropdownContent
                side=MenuSide::Bottom
                align=MenuAlign::End
                class="w-auto h-auto z-40"
            >
                <div class="relative w-[510px] min-h-[342px] h-auto bg-base-300 flex flex-col overflow-x-hidden overflow-y-scroll rounded-md border border-base-100 p-2 origin-top-right starting:opacity-0 starting:translate-x-2 starting:-translate-y-2 starting:scale-95 transition-all">
                    <Transition>
                        {
                            move || {
                                pinned.and_then(|messages| {
                                    let messages = Store::new(MessageStore {messages: messages.clone()});
                                    view!{
                                        {
                                            move || {
                                                view!{
                                                    <Show when=move || messages.messages().get().is_empty()>
                                                        <div class="absolute inset-0 flex items-center justify-center text-sm">
                                                            "This channel doesn't have any pinned messages.."
                                                        </div>
                                                    </Show>
                                                }
                                            }
                                        }
                                        <For
                                            each=move || messages.messages()
                                            key=|message| message.id().get()
                                            let:message
                                        >
                                            <div class="relative py-1 w-full flex items-start isolate rounded-lg border-base-content">
                                                <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=message.sender() class="w-auto h-auto absolute left-2 top-2 z-10" >
                                                    {if let Some(url) = message.sender().image_url().get() {
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
                                                <div class="relative w-full flex flex-col text-wrap whitespace-break-spaces">
                                                    <ChatMessage message=message />
                                                </div>
                                            </div>
                                        </For>
                                    }
                                })
                            }
                        }
                    </Transition>
                </div>
            </DropdownContent>
        </DropdownProvider>
    }
}

#[component]
pub fn ChatMessage(#[prop(into)] message: Field<ChannelMessage>) -> impl IntoView {
    let markdown = Signal::derive(move || MarkdownParser::new(&message.get().content).parse_tree());
    let block_kind: RwSignal<Option<BlockQuoteKind>> = RwSignal::new(None);
    view! {
        <div class="relative py-0.5 w-full pl-14 pr-4 flex items-start text-wrap whitespace-break-spaces">
            {
                move || {
                    block_kind.get().map(|kind| {
                        view!{<div class=format!("absolute border-l-2 inset-0 {}", match kind {
                            BlockQuoteKind::Note => "bg-note/5 border-l-note/60",
                            BlockQuoteKind::Tip => "bg-tip/5 border-l-tip/60",
                            BlockQuoteKind::Important => "bg-important/5 border-l-important/60",
                            BlockQuoteKind::Warning => "bg-warn/5 border-l-warn/60",
                            BlockQuoteKind::Caution => "bg-caution/5 border-l-caution/60",

                        })/>}
                    })
                }
            }
            <div class="flex flex-col items-start">
                <div class="flex items-center mb-1">
                    <div class="font-medium mr-2">
                        {move || message.sender().get().name}
                    </div>
                    <div class="text-[11px] text-base-content/50 self-end mb-0.5">
                        {move || message.get().timestamp.format("%d/%m/%y, %H:%M").to_string()}
                    </div>
                </div>
                <Markdown role_mentions=Signal::derive(move || message.get().mentions_roles) mentions=Signal::derive(move || message.get().mentions) markdown=markdown block_kind=block_kind/>
            </div>
        </div>
    }
}
