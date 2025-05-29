use leptos::prelude::*;

use crate::entities::message::{ChannelMessage, Embed};
use crate::open_graph::OpenGraph;

#[component]
pub fn Embeds(message: RwSignal<ChannelMessage>) -> impl IntoView {
    view! {
        <Show when=move || !message.get().embeds.is_empty()>
            <div class="mt-1 max-w-136 h-auto flex flex-col items-start justify-center">
                <For
                    each=move || message.get().embeds
                    key=|embeds| embeds.id
                    let:embed
                >
                    <Embed embed=embed/>
                </For>
            </div>
        </Show>
    }
}

#[component]
pub fn embed(embed: Embed) -> impl IntoView {
    let OpenGraph {
        title,
        image,
        url,
        description,
        site_name,
        ..
    } = serde_json::from_value(embed.data).unwrap();
    view! {
        <div class="w-min h-auto p-2 bg-base-100 space-y-1 rounded-lg flex flex-col items-start justify-center">
            {site_name.map(|name| view!{<div class="text-xs text-base-content/80">{name}</div>})}
            <a href=url class="text-sm hover:underline text-blue-400">{title}</a>
            {description.map(|desc| view!{<div class="text-xs">{desc}</div>})}
            <img src=image class="w-auto max-w-100 h-auto rounded"/>
        </div>
    }
}
