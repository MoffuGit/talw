use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::{BlockQuoteKind, HeadingLevel};

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::context_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::markdown::{
    MarkdownElement, MarkdownNode, MarkdownTree,
};
use crate::entities::member::Member;
use crate::entities::role::Role;

#[component]
pub fn Markdown(
    markdown: Signal<MarkdownTree>,
    mentions: Signal<Vec<Member>>,
    role_mentions: Signal<Vec<Role>>,
    block_kind: RwSignal<Option<BlockQuoteKind>>,
) -> impl IntoView {
    view! {
        {
            move || {
                view!{
                    <MarkdownParagraph node=markdown.get().root block_kind=block_kind mentions=mentions role_mentions=role_mentions/>
                }
            }
        }
    }
}

#[component]
pub fn MarkdownParagraph(
    node: MarkdownNode,
    block_kind: RwSignal<Option<BlockQuoteKind>>,
    mentions: Signal<Vec<Member>>,
    role_mentions: Signal<Vec<Role>>,
) -> impl IntoView {
    let childrens = node
        .childrens
        .iter()
        .map(|node| {
            view! {<MarkdownParagraph node=node.clone() block_kind=block_kind mentions=mentions role_mentions=role_mentions/>}
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
            view! {<code class="font-jetbrains text-base-content text-sm font-thin bg-baes-100 rounded px-1">{code}</code>}.into_any()
        }
        MarkdownElement::CodeBlock(_lang) => {
            view! {
                <pre 
                    class="bg-base-100 text-base-content rounded-lg p-1"
                >
                    <code class="font-jetbrains text-sm font-thin">{childrens}</code>
                </pre>
            }
            .into_any()
        }
        MarkdownElement::Link { url } => view! {
            <a href=url class="text-blue-400 hover:underline">{url.clone()}</a>
        }
        .into_any(),
        MarkdownElement::Role(id) => view! {
            {
                move || {
                    if let Some(mention) = role_mentions.get().iter().find(|mention| mention.id == id).cloned() {
                        Either::Left(view!{
                            <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">@{mention.name}</span>
                        })
                    } else {
                        Either::Right(view!{
                            <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">"Unknown"</span>
                        })
                    }
                }
            }
        }
        .into_any(),
        MarkdownElement::Mention(id) => view! {
            {
                move || {
                    if let Some(mention) = mentions.get().iter().find(|mention| mention.id == id).cloned() {
                        let name = mention.name.clone();
                        Either::Left(
                            view!{
                                <MemberBanner side=MenuSide::Right align=MenuAlign::Start member=mention class="cursor-pointer inline select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5" >
                                    {format!("@{}", name)}
                                </MemberBanner >
                            }
                        )
                    } else {
                        Either::Right(view!{
                            <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">@"Unknown"</span>
                        })
                    }
                }
            }
        }
        .into_any(),
        MarkdownElement::Everyone => view! {
            <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">@"Everyone"</span>
        }
        .into_any(),
    }
}
