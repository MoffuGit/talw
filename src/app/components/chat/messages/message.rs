use std::ops::Not;

use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::{BlockQuoteKind, HeadingLevel};

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::dropdown_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::markdown::{
    MarkdownElement, MarkdownNode, MarkdownParser, MarkdownTree,
};
use crate::entities::member::Member;
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
    view! {
        //MessageContextMenu
        <div class="relative py-0.5 w-full pl-14 pr-4 group hover:bg-base-content/5 flex items-start text-wrap whitespace-break-spaces">
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
            {
                is_first.not().then(|| view!{
                    <div class="text-[11px] text-base-content/50 absolute left-4 top-2 opacity-0 group-hover:opacity-100 flex items-center">
                        {move || message.get().timestamp.format("%H:%M").to_string()}
                    </div>
                })
            }
            <div class="flex flex-col items-start">
                {
                    is_first.then(||
                        view!{
                            <div class="flex items-center mb-1">
                                <div class="font-medium mr-2">
                                    {move || member.get().name}
                                </div>
                                <div class="text-[11px] text-base-content/50 self-end mb-0.5">
                                    {move || message.get().timestamp.format("%d/%m/%y, %H:%M").to_string()}
                                </div>
                            </div>
                        }
                    )
                }
                <Markdown markdown=markdown block_kind=block_kind/>
            </div>
        </div>
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
            view! {<code class="font-jetbrains text-sm font-light">{code}</code>}.into_any()
        }
        MarkdownElement::CodeBlock(_lang) => {
            view! {<pre><code class="font-jetbrains text-sm font-light">{childrens}</code></pre>}
                .into_any()
        }
        MarkdownElement::Link { url } => view! {
            <a href=url class="text-note">{url.clone()}</a>
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
