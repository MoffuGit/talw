mod messages;
mod sender;

use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;

use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::MemberStoreFields;
use crate::entities::message::ChannelMessage;

use self::messages::ChatMessages;
use self::sender::Sender;

#[derive(Debug, Clone, Default)]
pub struct ChatContext {
    msg_reference: RwSignal<Option<ChannelMessage>>,
}

#[component]
pub fn Chat(
    #[prop(into)] channel_id: Signal<Uuid>,
    #[prop(into, optional)] thread_id: Option<Signal<Uuid>>,
    #[prop(into)] name: Field<String>,
) -> impl IntoView {
    let id = use_current_server_context().member.id();
    provide_context(ChatContext::default());
    view! {
        <div class="relative flex flex-col h-full w-full min-w-0 overflow-hidden bg-base-200">
            <div
                on:dragenter=move |evt: DragEvent| {
                    evt.prevent_default();
                    evt.stop_propagation();
                    debug!("Enter the drag zone");
                }
                on:dragover= move |evt: DragEvent |{
                    evt.prevent_default();
                    evt.stop_propagation();
                    debug!("Over the drag zone");
                }
                on:dragleave= move |evt: DragEvent |{
                    evt.prevent_default();
                    evt.stop_propagation();
                    debug!("Leave the drag zone");
                }
                on:drop= move |evt: DragEvent |{
                    evt.prevent_default();
                    evt.stop_propagation();
                    if let Some(data) = evt.data_transfer() {
                        if let Some(files) = data.files() {
                            for idx in 0..files.length() {
                                if let Some(file) =     files.get(idx) {
                                    selected_files.update(|list| list.push[file]);
                                }
                            }
                            debug!("files len: {}", files.length());
                        }
                    }
                    debug!("Drop in the drag zone");
                }
                class="absolute inset-0 bg-red-500 z-100"
            />
            <ChatMessages channel_id=channel_id member_id=id  thread_id=thread_id/>
            <Sender channel_id=channel_id thread_id=thread_id name=name/>
        </div>
    }
}
