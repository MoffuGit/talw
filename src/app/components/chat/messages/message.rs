use crate::app::api::messages::{React, Unreact};
use crate::app::components::chat::messages::menu::MessageContextMenu;
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;
use std::ops::Not;

use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::{BlockQuoteKind, HeadingLevel};

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::markdown::{
    MarkdownElement, MarkdownNode, MarkdownParser, MarkdownTree,
};
use crate::entities::member::{Member, MemberStoreFields};
use crate::entities::message::ChannelMessage;

#[component]
pub fn ChatGroup(messages: Vec<ChannelMessage>) -> impl IntoView {
    let first = RwSignal::new(messages.first().cloned().unwrap());
    let member = Signal::derive(move || first.get().sender.clone());
    view! {
        <div class="relative py-1 w-full flex items-start isolate">
            {
                move || {
                    let member = member.get();
                    view!{
                        <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=member.clone() class="w-auto h-auto absolute left-2 top-2 z-10" >
                            {if let Some(url) = member.image_url {
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
                }
            }
            <div class="relative w-full flex flex-col text-wrap whitespace-break-spaces">
                <ChatMessage message=first member=member is_first=true/>
                {
                    messages.into_iter().skip(1).map(|message| {
                        view!{<ChatMessage message=RwSignal::new(message) member=member/>}
                    }).collect_view()
                }
            </div>
        </div>
    }
}

#[component]
pub fn ChatMessage(
    message: RwSignal<ChannelMessage>,
    member: Signal<Member>,
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
        <MessageContextMenu message=message member_id=Signal::derive(move || member.get().id)>
            <div class="relative py-0.5 w-full pl-14 pr-4 group hover:bg-neutral/10 flex items-start text-wrap whitespace-break-spaces">
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
                    {
                        is_first.then(||
                            view!{
                                <div class="flex items-center mb-1">
                                    <div class="font-base mr-2">
                                        {move || member.get().name}
                                    </div>
                                    <div class="text-xs text-base-content/50 self-end mb-0.5">
                                        {move || message.get().timestamp.format("%d/%m/%y, %H:%M").to_string()}
                                    </div>
                                </div>
                            }
                        )
                    }
                    <Markdown markdown=markdown block_kind=block_kind/>
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
                                                "border border-indigo-400/30"
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

#[component]
fn Markdown(
    markdown: Signal<MarkdownTree>,
    block_kind: RwSignal<Option<BlockQuoteKind>>,
) -> impl IntoView {
    view! {
        {
            move || {
                view!{
                    <MarkdownParagraph node=markdown.get().root block_kind=block_kind/>
                }
            }
        }
    }
}

#[component]
pub fn MarkdownParagraph(
    node: MarkdownNode,
    block_kind: RwSignal<Option<BlockQuoteKind>>,
) -> impl IntoView {
    let childrens = node
        .childrens
        .iter()
        .map(|node| {
            view! {<MarkdownParagraph node=node.clone() block_kind=block_kind/>}
        })
        .collect_view();

    match node.element {
        MarkdownElement::Paragraph => {
            view! {<span class="text-sm font-light">{childrens}</span>}.into_any()
        }
        MarkdownElement::Text(text) => view! {{text}}.into_any(),
        MarkdownElement::LineBreak => view! {<br/>}.into_any(),
        MarkdownElement::Bold => view! {<span class="font-medium">{childrens}</span>}.into_any(),
        MarkdownElement::Italic => view! {<span class="italic">{childrens}</span>}.into_any(),
        MarkdownElement::Heading(level) => match level {
            HeadingLevel::H1 => {
                view! {<span class="font-medium text-xl ">{childrens}</span>}.into_any()
            }
            HeadingLevel::H2 => {
                view! {<span class="font-medium text-lg ">{childrens}</span>}.into_any()
            }
            _ => view! {{childrens}}.into_any(),
        },
        MarkdownElement::Blockquotes(kind) => {
            if kind.is_some() {
                block_kind.set(kind);
            }
            view! {{childrens}}.into_any()
        }
        MarkdownElement::ListItem => view! {<li>{childrens}</li>}.into_any(),
        MarkdownElement::List { order } => {
            if order {
                view! {<ol class="list-decimal pl-4">{childrens}</ol>}.into_any()
            } else {
                view! {<ul class="list-disc pl-4">{childrens}</ul>}.into_any()
            }
        }
        MarkdownElement::Code(code) => {
            view! {<code class="font-jetbrains text-base-content text-sm font-light bg-baes-100 rounded px-1">{code}</code>}.into_any()
        }
        MarkdownElement::CodeBlock(_lang) => {
            view! {<pre class="bg-base-100 text-base-content rounded-lg p-2"><code class="font-jetbrains text-sm font-light">{childrens}</code></pre>}
                .into_any()
        }
        MarkdownElement::Link { url } => view! {
            <a href=url class="text-blue-500">{url.clone()}</a>
        }
        .into_any(),
        MarkdownElement::Role(id) => view! {
            <span class="text-red-500">{id}</span>
        }
        .into_any(),
        MarkdownElement::Mention(id) => view! {
            <span class="text-red-500">{id}</span>
        }
        .into_any(),
        MarkdownElement::Everyone => view! {
            <span class="text-green-500">"Everyone"</span>
        }
        .into_any(),
    }
}
