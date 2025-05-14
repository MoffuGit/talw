mod message;

use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Month, Utc};
use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;

use crate::app::api::messages::get_messages;
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::message::ChannelMessage;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
use crate::ws::client::use_ws;

use self::message::ChatMessage;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Date {
    month: u32,
    day: u32,
    year: i32,
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Date {
            month: value.month(),
            day: value.day(),
            year: value.year(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct MessageGroup {
    groups: BTreeMap<Date, RwSignal<Vec<ChannelMessage>>>,
}

impl From<Vec<ChannelMessage>> for MessageGroup {
    fn from(value: Vec<ChannelMessage>) -> Self {
        let mut groups = MessageGroup::new();
        groups.add_messages(value);
        groups
    }
}

impl MessageGroup {
    pub fn new() -> Self {
        MessageGroup::default()
    }

    pub fn add_messages(&mut self, messages: Vec<ChannelMessage>) {
        for message in messages {
            self.add(message);
        }
    }

    pub fn add(&mut self, message: ChannelMessage) {
        let date = Date::from(message.timestamp);
        self.groups
            .entry(date)
            .or_default()
            .update(|messages| messages.push(message));
    }
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
        <div class="relative flex flex-col-reverse overflow-y-scroll overflow-x-hidden flex-auto py-1" node_ref=node >
            <Transition>
                {move || Suspend::new(async move {
                    messages.await.map(|messages| {
                        let messages = RwSignal::new(MessageGroup::from(messages));

                        use_ws().on_server_msg(server.id().get(), move |msg| {
                            if let Message::ChannelMessage {
                                channel_id: id,
                                content,
                            } = msg
                            {
                                if id == channel_id.get() {
                                    messages.update(|messages| messages.add(*content));
                                }
                            }
                        });
                        view!{
                            <For
                                each=move || messages.get().groups.into_iter().rev()
                                key=|(date, _)| *date
                                let:((date, group))
                            >
                                <For
                                    each=move || group.get().into_iter().rev()
                                    key=|message| message.id
                                    let:message
                                >
                                    <ChatMessage message=message/>
                                </For>
                                <div
                                    class="isolate relative w-full flex items-center justify-center my-1"
                                >
                                    <div class="z-0 absolute right-0 left-0 border-t border-base-content/10"/>
                                    <div class="z-1 text-xs text-base-content/50 bg-base-200 mx-1"> {format!("{:#?} {}, {}", Month::try_from(date.month as u8).unwrap(), date.day, date.year)}</div>
                                </div>
                            </For>
                        }
                    })
                })}
            </Transition>
        </div>
    }
}
