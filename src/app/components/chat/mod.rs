mod content;
mod input;

use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;

use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::MemberStoreFields;

use self::content::ChatContent;
use self::input::Input;

#[component]
pub fn Chat(
    channel_id: Uuid,
    #[prop(into, optional)] thread_id: Option<Uuid>,
    #[prop(into)] name: Field<String>,
) -> impl IntoView {
    let id = use_current_server_context().member.id();
    view! {
        <div class="relative flex flex-col flex-auto h-full w-full">
            <ChatContent channel_id=channel_id member_id=id.get() />
            <Input channel_id=channel_id thread_id=thread_id name=name/>
        </div>
    }
}
