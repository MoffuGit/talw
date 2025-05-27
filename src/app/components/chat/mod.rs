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
            <ChatMessages channel_id=channel_id member_id=id  thread_id=thread_id/>
            <Sender channel_id=channel_id thread_id=thread_id name=name/>
        </div>
    }
}
