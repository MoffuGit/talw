use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;

use crate::app::api::messages::update_pinned;
use crate::entities::message::ChannelMessage;

#[component]
pub fn Pin(
    message: RwSignal<ChannelMessage>,
    #[prop(into)] server_id: Field<Uuid>,
) -> impl IntoView {
    let pin =
        Action::new(move |pinned: &bool| update_pinned(message.get().id, server_id.get(), *pinned));
    view! {
        <button
        on:click=move |_| {pin.dispatch(!message.get().pinned);}
        class="flex justify-between hover:bg-base-100 items-center w-full text-sm py-1.5 px-2 group rounded-md"
        disabled=move || pin.pending().get()
        >
        {
            move || {
                if message.get().pinned {
                    "Unpin Message"
                } else {
                    "Pin Message"
                }
            }
        }
        </button>
    }
}
