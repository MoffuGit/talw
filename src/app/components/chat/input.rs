use crate::app::api::messages::{send_message, SendMessage};
use crate::app::components::ui::icons::{Icon, IconData};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::MemberStoreFields;
use crate::entities::server::ServerStoreFields;
use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;
use web_sys::{window, HtmlDivElement, Node, Range};

// fn get_caret_position(editable: &HtmlDivElement) -> usize {
//     if let Some(selection) = window().and_then(|w| w.get_selection().ok()).flatten() {
//         if let Ok(range) = selection.get_range_at(0) {
//             if let (Ok(start_container), Ok(start_offset)) =
//                 (range.start_container(), range.start_offset())
//             {
//                 let mut position = 0;
//                 let mut found = false;
//
//                 fn walk(
//                     node: &web_sys::Node,
//                     target: &web_sys::Node,
//                     offset: u32,
//                     pos: &mut usize,
//                     found: &mut bool,
//                 ) {
//                     if *found {
//                         return;
//                     }
//
//                     if node == target {
//                         *pos += offset as usize;
//                         *found = true;
//                         return;
//                     }
//
//                     match node.node_type() {
//                         3 => {
//                             // Text node
//                             if let Some(text) = node.node_value() {
//                                 *pos += text.chars().count();
//                             }
//                         }
//                         1 => {
//                             let tag = node.node_name().to_lowercase();
//                             if tag == "br" {
//                                 *pos += 1;
//                             } else {
//                                 let child_nodes = node.child_nodes();
//                                 for i in 0..child_nodes.length() {
//                                     if let Some(child) = child_nodes.item(i) {
//                                         walk(&child, target, offset, pos, found);
//                                         if *found {
//                                             return;
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//
//                 walk(
//                     editable,
//                     &start_container,
//                     start_offset,
//                     &mut position,
//                     &mut found,
//                 );
//
//                 return position;
//             }
//         }
//     }
//
//     0
// }
//
// pub fn set_caret_position(editable: &HtmlDivElement, target_offset: usize) {
//     let document = window().and_then(|w| w.document());
//     let selection = window().and_then(|w| w.get_selection().ok()).flatten();
//
//     if document.is_none() || selection.is_none() {
//         return;
//     }
//
//     let document = document.unwrap();
//     let selection = selection.unwrap();
//
//     let range = document.create_range().ok();
//     if range.is_none() {
//         return;
//     }
//
//     let range = range.unwrap();
//     let mut char_count = 0;
//     let mut found = false;
//
//     fn walk(node: &Node, target_offset: usize, count: &mut usize, range: &Range, found: &mut bool) {
//         if *found {
//             return;
//         }
//
//         match node.node_type() {
//             3 => {
//                 // Text node
//                 if let Some(text) = node.node_value() {
//                     let len = text.chars().count();
//                     if *count + len > target_offset {
//                         // Set the caret inside this text node
//                         let offset_in_node = target_offset - *count;
//                         let _ = range.set_start(node, offset_in_node as u32);
//                         *found = true;
//                         return;
//                     }
//                     *count += len;
//                 }
//             }
//             1 => {
//                 let tag = node.node_name().to_lowercase();
//                 if tag == "br" {
//                     if *count == target_offset {
//                         let _ = range.set_start(node, 0);
//                         *found = true;
//                         return;
//                     }
//                     *count += 1;
//                 } else {
//                     let children = node.child_nodes();
//                     for i in 0..children.length() {
//                         if let Some(child) = children.item(i) {
//                             walk(&child, target_offset, count, range, found);
//                             if *found {
//                                 return;
//                             }
//                         }
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
//
//     walk(editable, target_offset, &mut char_count, &range, &mut found);
//
//     if found {
//         range.collapse_with_to_start(true);
//         let _ = selection.remove_all_ranges();
//         let _ = selection.add_range(&range);
//     }
// }

#[component]
pub fn Input(
    channel_id: Uuid,
    thread_id: Option<Uuid>,
    #[prop(into)] name: Field<String>,
) -> impl IntoView {
    let message = RwSignal::new(String::default());
    let content_ref: NodeRef<Div> = NodeRef::new();
    let height = RwSignal::new(56);

    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
            height.set(div.offset_height());
        }
    };
    let server = use_current_server_context().server;
    let member = use_current_server_context().member;

    let send_msg = ServerAction::<SendMessage>::new();

    view! {
        <div class="shrink-0 relative flex px-4">
            <div class="mb-4 relative w-full bg-base-300/60 rounded-lg flex px-4">
                <Icon icon=IconData::CirclePlus class="w-7 h-7 stroke-base-300/60 fill-base-content/40 grow-0 mb-3.5 mt-auto"/>
                    <div class="relative self-center h-fit w-full" style=move || format!("height: {}px", height.get())>
                        <div class="h-14 text-base font-normal relative">
                            <div>
                                <Show when=move || message.get().is_empty()>
                                    <div class="mx-4 py-4 absolute left-0 select-none text-base-content/40">
                                        {move || format!("Message #{}", name.get())}
                                    </div>
                                </Show>
                            </div>
                            <div
                                on:input=on_input
                                node_ref=content_ref
                                class="relative mx-4 py-4 outline-0 wrap-break-word text-left whitespace-break-spaces"
                                contenteditable="true"
                                aria-multiline="true"
                                spellcheck="true"
                                aria-invalid="false">
                            </div>
                        </div>
                    </div>
                <Icon icon=IconData::Sticker on:click=move |_| {
                    send_msg.dispatch(SendMessage { server_id: server.id().get(), channel_id, message: message.get(), member_id: member.id().get() });
                } class="w-7 h-7 stroke-base-300/60 fill-base-content/40 mb-3.5 mt-auto"/>
            </div>
        </div>
    }
}
