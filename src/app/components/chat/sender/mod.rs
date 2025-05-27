mod input;
mod reference;

use crate::app::api::messages::{send_message, SendMessage};
use crate::app::components::chat::ChatContext;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::MemberStoreFields;
use crate::entities::server::ServerStoreFields;
use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;
use web_sys::{window, HtmlDivElement, Node, Range};

use self::input::Input;
use self::reference::Reference;

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
pub fn Sender(
    channel_id: Signal<Uuid>,
    thread_id: Option<Signal<Uuid>>,
    #[prop(into)] name: Field<String>,
) -> impl IntoView {
    let message = RwSignal::new(String::default());
    let height = RwSignal::new(56);

    let server = use_current_server_context().server;
    let member = use_current_server_context().member;

    let send_msg = ServerAction::<SendMessage>::new();

    let ChatContext { msg_reference } =
        use_context::<ChatContext>().expect("should acces to the chat context");

    let on_click = Signal::derive(move || {
        let channel_id = channel_id.get();
        send_msg.dispatch(SendMessage {
            server_id: server.id().get(),
            channel_id,
            message: message.get(),
            member_id: member.id().get(),
            msg_reference: msg_reference.get().map(|reference| reference.id),
        });
    });

    view! {
        <div class="shrink-0 relative mb-4 px-4 w-full h-auto">
            <div class="relative w-full flex flex-col px-4">
                <Reference/>
                <Input name=name message=message height=height on_click=on_click/>
            </div>
        </div>
    }
}
