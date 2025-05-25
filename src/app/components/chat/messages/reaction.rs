use emojis::Emoji;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_document;
use log::debug;
use reactive_stores::Store;

use crate::app::api::messages::react;
use crate::app::components::ui::context_menu::{
    MenuAlign, MenuSide, SubContextMenuContent, SubContextMenuProvider, SubContextMenuTrigger,
};
use crate::app::components::ui::icons::{Icon, IconData};
use crate::entities::member::{Member, MemberStoreFields};
use crate::entities::message::ChannelMessage;

#[component]
pub fn Reaction(
    parent_ref: NodeRef<Div>,
    message: RwSignal<ChannelMessage>,
    content_ref: NodeRef<Div>,
    member: Store<Member>,
) -> impl IntoView {
    let menu_side = Signal::derive(move || {
        let parent_width = parent_ref
            .get()
            .map(|node| node.offset_width())
            .unwrap_or_default();
        let parent_position = parent_ref
            .get()
            .map(|node| node.get_bounding_client_rect().x())
            .unwrap_or_default() as i32;
        let content_width = content_ref
            .get()
            .map(|node| node.offset_width())
            .unwrap_or_default();
        use_document()
            .body()
            .map(|body| {
                if body.offset_width() - (parent_position + parent_width) < content_width {
                    MenuSide::Left
                } else {
                    MenuSide::Right
                }
            })
            .unwrap_or(MenuSide::Right)
    });
    let base_emojis = ["🧋", "📹", "🎇", "🌊", "🪴", "☕", "🕹️"];
    let react = Action::new(move |name: &String| {
        debug!("{name}");
        react(
            name.to_string(),
            message.get().id,
            member.id().get(),
            member.server_id().get(),
        )
    });
    view! {
        <SubContextMenuProvider content_ref=content_ref>
            <SubContextMenuTrigger
                class="flex justify-between cursor-pointer hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
            >
                "Add Reaction"
                <Icon icon=IconData::ChevronRight class="absolute right-4 h-4 w-4"/>
            </SubContextMenuTrigger>
            <SubContextMenuContent side=menu_side align=MenuAlign::Center
                class="z-100 w-64 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100">
                <div class="starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <div
                        class="flex justify-between items-center w-full text-base py-1 px-1.5"
                    >
                        {base_emojis
                            .iter()
                            .map(|some| some.to_string())
                            .map(|fruit| {
                                let fruit = StoredValue::new(fruit);
                                view!{
                                    <button on:click=move |_| {
                                        react.dispatch(fruit.get_value());
                                    }
                                    disabled=move ||{
                                        react.pending()
                                    }
                                    class="p-0.5 rounded select-none cursor-pointer hover:bg-base-100">{move || fruit.get_value()}</button>
                                }
                            }).collect_view()}
                    </div>
                </div>
            </SubContextMenuContent>
        </SubContextMenuProvider>
    }
}
