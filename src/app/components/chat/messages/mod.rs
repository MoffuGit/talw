mod menu;
mod message;
mod pin;
mod reaction;

use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Month, Utc};
use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;
use uuid::Uuid;

use crate::app::api::messages::{get_messages, get_thread_messages};
use crate::app::routes::servers::server::use_current_server_context;
use crate::entities::member::Member;
use crate::entities::message::ChannelMessage;
use crate::entities::server::ServerStoreFields;
use crate::messages::Message;
// use crate::ws::client::use_ws;

use self::message::ChatGroup;

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
    groups: BTreeMap<Date, Vec<Group>>,
}

#[derive(Debug, PartialEq, Clone)]
struct Group {
    sender: Member,
    messages: Vec<ChannelMessage>,
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
        let entry = self.groups.entry(date).or_default();

        if message.message_reference.is_some() {
            entry.push(Group {
                sender: message.sender.clone(),
                messages: vec![message],
            });
            return;
        }

        if let Some(last_group) = entry.last_mut() {
            if last_group.sender == message.sender {
                last_group.messages.push(message);
                return;
            }
        }

        entry.push(Group {
            sender: message.sender.clone(),
            messages: vec![message],
        });
    }
}

#[component]
pub fn ChatMessages(
    channel_id: Signal<Uuid>,
    thread_id: Option<Signal<Uuid>>,
    #[prop(into)] member_id: Field<Uuid>,
) -> impl IntoView {
    let messages = Resource::new(
        move || (channel_id.get(), member_id.get(), thread_id.get()),
        move |(channel_id, member_id, thread_id)| async move {
            if let Some(thread_id) = thread_id {
                return get_thread_messages(thread_id, member_id).await;
            }
            get_messages(channel_id, member_id).await
        },
    );
    let server = use_current_server_context().server;
    let node: NodeRef<Div> = NodeRef::new();
    view! {
        <div class="relative min-h-0 h-full scrollbar-none flex flex-col-reverse overflow-y-scroll min-w-0 overflow-x-hidden py-1" node_ref=node >
            <Transition>
                {move || Suspend::new(async move {
                    messages.await.map(|messages| {
                        let groups = RwSignal::new(MessageGroup::from(messages));

                        // use_ws().on_server_msg(server.id().get(), move |msg| {
                        //     if let Message::ChannelMessage {
                        //         channel_id: id,
                        //         content,
                        //     } = msg
                        //     {
                        //         if id == channel_id.get() {
                        //             groups.write().add(*content);
                        //         }
                        //     }
                        // });
                        view!{
                            {
                                move || {
                                    groups.get().groups.into_iter().rev().map(|(date, groups)| {
                                        view!{
                                            {
                                                groups.into_iter().rev().map(|group| {
                                                    view!{<ChatGroup group=group/>}
                                                }).collect_view()
                                            }
                                            <div
                                                class="isolate relative w-full flex items-center justify-center my-1"
                                            >
                                                <div class="z-0 absolute right-0 left-0 border-t border-base-content/10"/>
                                                <div class="z-1 text-xs font-semibold text-base-content/50 bg-base-200 mx-2"> {format!("{:#?} {}, {}", Month::try_from(date.month as u8).unwrap(), date.day, date.year)}</div>
                                            </div>
                                        }
                                    }).collect_view()
                                }
                            }
                        }
                    })
                })}
            </Transition>
        </div>
    }
}
