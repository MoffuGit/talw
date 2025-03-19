use log::debug;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::spawn;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{self, Receiver};
use tokio::task::JoinHandle;

use crate::topic::Topic;
use crate::ws::server::WsChannels;

use super::messages::Message;
use super::subs::Subscriptions;

#[derive(Clone)]
pub struct MsgSender {
    sender: Sender<Message>,
}

impl MsgSender {
    pub fn send(&self, msg: Message) {
        self.sender.send(msg);
    }
}

impl Debug for MsgSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MsgBroker").finish()
    }
}

impl MsgSender {
    pub async fn new(channels: WsChannels) -> Self {
        let (sender, mut receiver) = broadcast::channel(1000);
        let mut msg_receiver = MsgReceiver::new(receiver, channels);
        spawn(async move {
            msg_receiver.run().await;
        });
        MsgSender { sender }
    }
}

struct MsgReceiver {
    receiver: Receiver<Message>,
    subscriptions: Subscriptions,
    channels: WsChannels,
}

impl MsgReceiver {
    pub fn new(receiver: Receiver<Message>, channels: WsChannels) -> Self {
        MsgReceiver {
            receiver,
            channels,
            subscriptions: Subscriptions::default(),
        }
    }

    pub async fn run(&mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            self.handle_msg(msg);
        }
    }

    pub fn handle_msg(&mut self, msg: Message) {
        match msg {
            Message::Batch(msgs) => {
                for msg in msgs {
                    self.handle_msg(msg);
                }
            }
            Message::Subscribe { topic, user_id } => {
                self.subscriptions.subscribe(user_id, topic);
            }
            Message::Unsubscribe { topic, user_id } => {
                self.subscriptions.unsubscribe(user_id, topic);
            }
            Message::UserJoinedServer { user_id, server_id } => {
                self.send_msg_to_topic(msg, Topic::Server(server_id));
            }
            Message::UserConnected { user_id, server_id } => {
                self.send_msg_to_topic(msg, Topic::Server(server_id));
            }
            msg => {
                debug!("got this msg on the msg receiver {:?}", msg);
            }
        }
    }

    pub fn send_msg_to_topic(&mut self, msg: Message, topic: Topic) {
        self.subscriptions
            .topic_subscriptions
            .get(&topic)
            .map(|users| {
                for user in users {
                    if let Some(channel) = self.channels.get(user) {
                        let sender = channel.0.clone();
                        sender.send(msg.clone());
                    }
                }
            });
    }
}
