use leptos::prelude::*;
use pulldown_cmark::{BlockQuoteKind, HeadingLevel};

use crate::app::components::ui::markdown::{
    MarkdownElement, MarkdownNode, MarkdownParser, MarkdownTree,
};

#[component]
pub fn Markdown(
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
            view! {<MarkdownParagraph node=node.clone() block_kind=block_kind />}
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
