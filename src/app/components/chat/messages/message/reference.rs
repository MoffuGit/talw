use leptos::either::Either;
use leptos::prelude::*;
use pulldown_cmark::HeadingLevel;

use crate::app::components::channel::member::banner::MemberBanner;
use crate::app::components::ui::context_menu::{MenuAlign, MenuSide};
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::components::ui::markdown::{MarkdownElement, MarkdownNode, MarkdownParser, MarkdownTree};
use crate::entities::member::Member;
use crate::entities::message::ChannelMessage;
use crate::entities::role::Role;

#[component]
pub fn Reference(message: Signal<ChannelMessage>) -> impl IntoView {
    let markdown = Signal::derive(move || MarkdownParser::new(&message.get().content).parse_tree());
    view! {
        <div class="w-full h-7 pl-14 pr-8 overflow-hidden flex items-center justify-start">
            <div class="absolute h-4 w-7 bg-transparent border-l-2 border-l-neutral/30 border-t-2 border-t-neutral/30 rounded-tl-md left-6.5 top-4"/>
            <MemberBanner
                side=MenuSide::Right
                align=MenuAlign::Start
                member=Signal::derive(move || message.get().sender)
                class="w-auto h-full flex mr-2 items-center"
            >
                {move || if let Some(url) = message.get().sender.image_url {
                    Either::Left(
                        view! {
                            <img
                                class="rounded-full object-cover w-5 h-5"
                                src=url
                            />
                        },
                    )
                } else {
                    Either::Right(
                        view! {
                            <div class="rounded-full bg-base-content/10 w-5 h-5" />
                        },
                    )
                }}
                <div class="ml-1 text-md font-normal">
                    {
                        move || {
                            message.get().sender.name
                        }
                    }
                </div>
            </MemberBanner>
            <div class="w-auto h-full flex items-center min-w-0 mr-1">
                <Markdown role_mentions=Signal::derive(move || message.get().mentions_roles) mentions=Signal::derive(move || message.get().mentions) markdown=markdown/>
            </div>
            <Show when=move || !message.get().attachments.is_empty()>
                <Icon icon=IconData::PaperClip class="w-4 h-4"/>
            </Show>
        </div>
    }
}

#[component]
fn Markdown(
    markdown: Signal<MarkdownTree>,
    mentions: Signal<Vec<Member>>,
    role_mentions: Signal<Vec<Role>>,
) -> impl IntoView {
    view! {
        {
            move || {
                view!{
                    <MarkdownParagraph node=markdown.get().root mentions=mentions role_mentions=role_mentions/>
                }
            }
        }
    }
}

#[component]
fn MarkdownParagraph(
    node: MarkdownNode,
    mentions: Signal<Vec<Member>>,
    role_mentions: Signal<Vec<Role>>,
) -> impl IntoView {
    let childrens = node
        .childrens
        .iter()
        .map(|node| {
            view! { <MarkdownParagraph node=node.clone() mentions=mentions role_mentions=role_mentions/>}
        })
        .collect_view();

    match node.element {
        MarkdownElement::Paragraph => {
            view! {<div class="text-xs font-light inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{childrens}</div>}.into_any()
        }
        MarkdownElement::Text(text) => view! {{text}}.into_any(),
        MarkdownElement::LineBreak => view! {<br/>}.into_any(),
        MarkdownElement::Bold => view! {<div class="font-medium inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{childrens}</div>}.into_any(),
        MarkdownElement::Italic => view! {<div class="italic inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{childrens}</div>}.into_any(),
        MarkdownElement::Heading(level) => match level {
            HeadingLevel::H1 => {
                view! {<div class="font-medium text-base inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{childrens}</div>}.into_any()
            }
            HeadingLevel::H2 => {
                view! {<div class="font-medium text-xl inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{childrens}</div>}.into_any()
            }
            _ => view! {{childrens}}.into_any(),
        },
        MarkdownElement::Blockquotes(_) => {
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
            view! {<div class="font-jetbrains text-base-content text-xs font-thin bg-baes-100 rounded px-0.5 inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{code}</div>}.into_any()
        }
        MarkdownElement::CodeBlock(_lang) => {
            view! {
                <div 
                    class="bg-base-100 font-jetbrains text-xs font-thin text-base-content rounded-md px-0.5 inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full"
                >
                    {childrens}
                </div>
            }
            .into_any()
        }
        MarkdownElement::Link { url } => view! {
            <a href=url class="text-blue-400 hover:underline inline-block whitespace-nowrap text-ellipsis overflow-hidden w-full">{url.clone()}</a>
        }
        .into_any(),
        MarkdownElement::Role(id) => view! {
            <div class="text-red-500">{id.to_string()}</div>
        }
        .into_any(),
        MarkdownElement::Mention(id) => view! {
            {
                move || {
                    if let Some(mention) = mentions.get().iter().find(|mention| mention.id == id).cloned() {
                        Either::Left(view!{
                            <span class="cursor-pointer select-none bg-indigo-500/20 color-indigo-100 font-base hover:color-base-content hover:bg-indigo-500/80 hover:underline rounded-sm px-0.5">{mention.name}</span>
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
        MarkdownElement::Everyone => view! {
            <div class="text-green-500">"Everyone"</div>
        }
        .into_any(),
    }
}
