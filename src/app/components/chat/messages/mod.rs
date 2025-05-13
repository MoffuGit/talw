mod message;

use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::{Field, Store};
use uuid::Uuid;

use crate::app::api::messages::get_messages;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::message::{ChannelMessage, ChannelMessageStoreFields};
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;

use self::message::ChatMessage;

#[derive(Debug, Store)]
struct MessagesStore {
    #[store(key: Uuid = |messages| messages.id)]
    messages: Vec<ChannelMessage>,
}

#[component]
pub fn ChatMessages(
    channel_id: Signal<Uuid>,
    #[prop(into)] member_id: Field<Uuid>,
) -> impl IntoView {
    let messages = Resource::new(
        move || (channel_id.get(), member_id.get()),
        move |(channel_id, member_id)| get_messages(channel_id, member_id),
    );
    let server = use_current_server_context().server;
    let node: NodeRef<Div> = NodeRef::new();
    view! {
        <div class="relative flex flex-col overflow-y-scroll overflow-x-hidden flex-auto pb-1" node_ref=node >
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
                                if id == channel_id.get() {
                                    messages.messages().update(|messages| messages.push(content));
                                }
                            }
                        });
                        Effect::new(move |_| {
                            if let Some(node) = node.get() {
                                node.scroll_with_x_and_y(0.0, node.scroll_height().into());
                            }
                        });
                        view!{
                            <For
                                each=move || messages.messages()
                                key=|message| message.id().get()
                                let:message
                            >
                                <ChatMessage message=message/>
                            </For>
                        }
                    })
                })}
            </Transition>
        </div>
    }
}
