mod attachments;
mod embeds;
mod reactions;
mod reference;

use crate::app::api::messages::{React, Unreact};
use crate::app::components::chat::messages::menu::MessageContextMenu;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::markdown::styled::Markdown;
use crate::app::components::ui::markdown::MarkdownParser;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
use std::ops::Not;

use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::BlockQuoteKind;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::entities::member::{Member, MemberStoreFields};
use crate::entities::message::ChannelMessage;

use self::attachments::Attachments;
use self::embeds::Embeds;
use self::reference::Reference;

use super::Group;

#[component]
pub fn ChatGroup(group: Group) -> impl IntoView {
    let sender = RwSignal::new(group.sender);
    let fist_message = RwSignal::new(group.messages.first().cloned().unwrap());
    let messages = RwSignal::new(group.messages);
    view! {
        <div class="relative py-1 w-full flex flex-col items-start isolate">
            <Show when=move || fist_message.get().message_reference.is_some()>
                <Reference message=Signal::derive(move || *fist_message.get().message_reference.unwrap())/>
            </Show>
            <div class="relative w-full flex flex-col">
                <ChatMessage message=fist_message sender=sender is_first=true/>
                <For
                    each=move || messages.get().into_iter().skip(1)
                    key=|message| message.id
                    let:message
                >
                    <ChatMessage message=RwSignal::new(message) sender=sender/>
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn ChatMessage(
    message: RwSignal<ChannelMessage>,
    sender: RwSignal<Member>,
    #[prop(default = false)] is_first: bool,
) -> impl IntoView {
    let markdown = Signal::derive(move || MarkdownParser::new(&message.get().content).parse_tree());
    let block_kind: RwSignal<Option<BlockQuoteKind>> = RwSignal::new(None);
    let current_server = use_current_server_context().server;
    let current_member = use_current_server_context().member;
    let ws = use_ws();
    view! {
        {
            move || {
                ws.on_server_msg(current_server.id().get_untracked(), move |msg| match msg {
                    Message::PinMessage { message_id } => {
                        if message.get().id == message_id {
                            message.update(|message| message.pinned = true);
                        }
                    }
                    Message::UnpinMessage { message_id } => {
                        if message.get().id == message_id {
                            message.update(|message| message.pinned = false);
                        }
                    }
                    Message::ReactionCreated {
                        reaction,
                        message_id,
                    } => {
                        if message.get().id == message_id {
                            message.update(|message| message.reactions.push(reaction));
                        }
                    }
                    Message::MessageAttachments {content , message_id} => {
                        if message.get().id == message_id {
                            message.update(|message| message.attachments = content);
                        }
                    },
                    Message::MessageEmbeds { message_id, embeds } => {
                        if message.get().id == message_id {
                            message.update(|message| message.embeds = embeds);
                        }
                    }
                    Message::ReactionDeleted {
                        reaction_id,
                        message_id,
                    } => {
                        if message.get().id == message_id {
                            message.update(|message| {
                                message
                                    .reactions
                                    .retain(|reaction| reaction.id != reaction_id)
                            });
                        }
                    }
                    Message::MemberReact {
                        react_id,
                        message_id,
                        member_id,
                    } => {
                        if message.get().id == message_id {
                            message.update(|message| {
                                if let Some(reaction) = message
                                    .reactions
                                    .iter_mut()
                                    .find(|reaction| reaction.id == react_id)
                                {
                                    reaction.counter += 1;
                                    if member_id == current_member.id().get() {
                                        reaction.me = true
                                    }
                                }
                            });
                        }
                    }
                    Message::MemberUnreact {
                        react_id,
                        message_id,
                        member_id,
                    } => {
                        if message.get().id == message_id {
                            message.update(|message| {
                                if let Some(reaction) = message
                                    .reactions
                                    .iter_mut()
                                    .find(|reaction| reaction.id == react_id)
                                {
                                    reaction.counter -= 1;
                                    if member_id == current_member.id().get() {
                                        reaction.me = false
                                    }
                                }
                            });
                        }
                    }
                    _ => {}
                });

            }
        }
        <MessageContextMenu message=message member_id=Signal::derive(move || sender.get().id)>
            <div class="relative py-0.5 w-full pl-14 pr-4 group hover:bg-neutral/10 flex flex-col items-start text-wrap whitespace-break-spaces">
                {
                    is_first.then(|| view! {
                        <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=sender class="w-auto h-auto absolute left-2 top-1 z-10" >
                            {move || if let Some(url) = sender.get().image_url {
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
                        <div class="flex items-center mb-1">
                            <div class="font-semibold text-base mr-2">
                                {move || sender.get().name}
                            </div>
                            <div class="text-xs text-base-content/50 self-end mb-0.5">
                                {move || message.get().timestamp.format("%d/%m/%y, %H:%M").to_string()}
                            </div>
                        </div>
                    })
                }
                {
                    move || {
                        block_kind.get().map(|kind| {
                            view!{<div class=format!("absolute border-l-2 inset-0 {}", match kind {
                                BlockQuoteKind::Note => "bg-blue-400/5 border-l-blue-400/60",
                                BlockQuoteKind::Tip => "bg-emerald-400/5 border-l-emerald-400/60",
                                BlockQuoteKind::Important => "bg-orange-400/5 border-l-orange-400/60",
                                BlockQuoteKind::Warning => "bg-yellow-400/5 border-l-yellow-400/60",
                                BlockQuoteKind::Caution => "bg-red-400/5 border-l-red-400/60",

                            })/>}
                        })
                    }
                }
                {
                    move || {
                        message.get().pinned.then_some(view!{
                            <div class="absolute text-sm select-none right-1 bottom-1 group-hover:opacity-100 opacity-0">
                                "📍"
                            </div>
                        })
                    }
                }
                {
                    is_first.not().then(|| view!{
                        <div class="text-[10px] text-base-content/50 absolute left-4 top-1 opacity-0 group-hover:opacity-100 flex items-center">
                            {move || message.get().timestamp.format("%H:%M").to_string()}
                        </div>
                    })
                }
                <div class="flex flex-col items-start">
                    <Markdown role_mentions=Signal::derive(move || message.get().mentions_roles) mentions=Signal::derive(move || message.get().mentions) markdown=markdown block_kind=block_kind/>
                    <Embeds message=message />
                    <Attachments message=message/>
                    <Show when=move || {
                        !message.get().reactions.is_empty()
                    }>
                        <div class="relative flex justify-start space-x-1 mt-1">
                            {
                                move || {
                                    let react = ServerAction::<React>::new();
                                    let unreact = ServerAction::<Unreact>::new();
                                    message.get().reactions.into_iter().map(|reaction| view!{
                                        <button
                                            disabled=move || react.pending().get() || unreact.pending().get()
                                            on:click=move |_| {
                                                if reaction.me {
                                                    unreact.dispatch(Unreact { name: reaction.name.clone(), message_id: reaction.message_id, member_id: current_member.id().get(), server_id: current_server.id().get() });
                                                } else {
                                                    react.dispatch(React { name: reaction.name.clone(), message_id: reaction.message_id, member_id: current_member.id().get(), server_id: current_server.id().get() });
                                                }
                                            }
                                            class=format!("flex items-center cursor-pointer pl-1 pr-1.5 h-6 w-auto text-center select-none rounded bg-neutral/10 hover:bg-neutral/20 {}", if reaction.me {
                                                "border border-indigo-400/40"
                                            } else {
                                                ""
                                            })>
                                            <span class="text-sm">
                                                {reaction.name.clone()}
                                            </span>
                                            <span class="text-sm">
                                                {format!(" {}", reaction.counter)}
                                            </span>
                                        </button>
                                    }).collect_view()
                                }
                            }
                            <Icon icon=IconData::Plus class="select-none cursor-pointer h-6 w-6 p-1 rounded bg-neutral/10 hover:bg-neutral/20" />
                        </div>
                    </Show>
                </div>
            </div>
        </MessageContextMenu>
    }
}
