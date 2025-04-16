use log::debug;
use std::fmt::Debug;
use tokio::spawn;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{self, Receiver};
use uuid::Uuid;

use crate::ws::server::WsChannels;

use super::messages::Message;
use super::subs::Subscriptions;

#[derive(Clone)]
pub struct MsgSender {
    sender: Sender<Message>,
}

impl MsgSender {
    pub fn send(&self, msg: Message) {
        let _ = self.sender.send(msg);
    }
}

impl Debug for MsgSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MsgBroker").finish()
    }
}

impl MsgSender {
    pub async fn new(channels: WsChannels) -> Self {
        let (sender, receiver) = broadcast::channel(1000);
        let mut msg_receiver = MsgReceiver::new(receiver, channels);
        spawn(async move {
            msg_receiver.start().await;
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

    pub async fn start(&mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            self.handle_msg(msg);
        }
    }

    pub fn handle_msg(&mut self, msg: Message) {
        debug!("got this msg on the msg receiver {:?}", msg);
        match msg {
            Message::Batch(msgs) => {
                for msg in msgs {
                    self.handle_msg(msg);
                }
            }
            Message::Subscribe { user_id, server_id } => {
                self.subscriptions.subscribe(user_id, server_id);
            }
            Message::Unsubscribe { server_id, user_id } => {
                self.subscriptions.unsubscribe(user_id, server_id);
            }
            Message::MemberJoinedServer { server_id, .. } => {
                self.send_msg_to_topic(msg, server_id);
            }
            Message::UserConnected { server_id, .. } => {
                self.send_msg_to_topic(msg, server_id);
            }
            Message::UserDisconnected { user_id } => {
                let servers = self.subscriptions.unsubscribe_all(user_id);
                if let Some(servers) = servers {
                    servers.into_iter().for_each(|server_id| {
                        self.send_msg_to_topic(msg.clone(), server_id);
                    })
                }
            }
            _ => {}
        }
    }

    pub fn send_msg_to_topic(&mut self, msg: Message, server_id: Uuid) {
        if let Some(users) = self.subscriptions.topic_subscriptions.get(&server_id) {
            for user in users {
                if let Some(channel) = self.channels.get(user) {
                    let sender = channel.0.clone();
                    let _ = sender.send(msg.clone());
                }
            }
        }
    }
}
