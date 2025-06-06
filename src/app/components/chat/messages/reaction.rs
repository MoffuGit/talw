use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_document;
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
    let react = Action::new(move |name: &String| {
        react(
            name.to_string(),
            message.get().id,
            member.id().get(),
            message.get().channel_id,
        )
    });
    view! {
        <SubContextMenuProvider content_ref=content_ref>
            <SubContextMenuTrigger
                class="flex justify-between cursor-pointer hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
            >
                "Add Reaction"
                <Icon icon=IconData::ChevronRight class="absolute right-2 h-4 w-4"/>
            </SubContextMenuTrigger>
            <SubContextMenuContent side=menu_side align=MenuAlign::Center
                class="z-100 flex flex-col h-auto p-1 bg-base-300 rounded-lg border border-base-100">
                <div class="starting:opacity-0 starting:-translate-x-2 starting:scale-95 transition-all">
                    <div
                        class="grid grid-cols-8 gap-1 text-base overflow-x-hidden overflow-y-scroll max-h-28"
                    >
                        {emojis::iter()
                            .map(|emoji| {
                                let emoji = StoredValue::new(emoji.to_string());
                                view!{
                                    <button on:click=move |_| {
                                        react.dispatch(emoji.get_value());
                                    }
                                    disabled=move ||{
                                        react.pending()
                                    }
                                    class="p-0.5 rounded select-none cursor-pointer hover:bg-base-100">{move || emoji.get_value()}</button>
                                }
                            }).collect_view()}
                    </div>
                </div>
            </SubContextMenuContent>
        </SubContextMenuProvider>
    }
}
