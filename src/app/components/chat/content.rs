use leptos::prelude::*;
use log::debug;
use reactive_stores::Store;
use uuid::Uuid;

use crate::app::api::messages::get_messages;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::message::{ChannelMessage, ChannelMessageStoreFields};
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;

#[derive(Debug, Store)]
struct MessagesStore {
    #[store(key: Uuid = |messages| messages.id)]
    messages: Vec<ChannelMessage>,
}

#[component]
pub fn ChatContent(channel_id: Uuid, member_id: Uuid) -> impl IntoView {
    let messages = Resource::new(move || (), move |_| get_messages(channel_id, member_id));
    let server = use_current_server_context().server;
    view! {
        <div class="relative flex flex-auto flex-col" >
            <Transition>
                {Suspend::new(async move {
                    messages.await.map(|messages| {
                        let messages = Store::new(MessagesStore {
                            messages
                        });

                        use_ws().on_server_msg(server.id().get(), move |msg| {
                            if let Message::ChannelMessage {
                                channel_id: id,
                                content,
                            } = msg
                            {
                                if id == channel_id {
                                    messages.messages().update(|messages| messages.push(content));
                                }
                            }
                        });
                        view!{
                            <For
                                each=move || messages.messages()
                                key=|message| message.id().get()
                                let:messages
                            >
                                <div>
                                    {move || messages.content().get()}
                                </div>
                            </For>
                        }
                    })
                })}
            </Transition>
        </div>
    }
}
