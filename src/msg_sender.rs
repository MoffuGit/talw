use log::debug;
use std::fmt::Debug;
use tokio::spawn;
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::{self, Receiver};
use uuid::Uuid;

use crate::messages::{ClientMessage, Message, ServerMessage};
use crate::ws::server::WsChannels;

use super::messages::AppMessage;
use super::subs::Subscriptions;

#[derive(Clone)]
pub struct MsgSender {
    sender: Sender<AppMessage>,
}

impl MsgSender {
    pub fn send(&self, msg: impl Into<AppMessage>) {
        let _ = self.sender.send(msg.into());
    }
}

impl Debug for MsgSender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    receiver: Receiver<AppMessage>,
    subscriptions: Subscriptions,
    channels: WsChannels,
}

impl MsgReceiver {
    pub fn new(receiver: Receiver<AppMessage>, channels: WsChannels) -> Self {
        MsgReceiver {
            receiver,
            channels,
            subscriptions: Subscriptions::default(),
        }
    }

    pub async fn start(&mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            debug!("receive msg: {msg:?}");
            self.handle_app_msg(msg);
        }
    }

    pub fn handle_sub_msg(&mut self, server_id: Uuid, user_id: Uuid, member_id: Uuid) {
        self.subscriptions.subscribe(user_id, server_id, member_id);
    }

    pub fn handle_unsub_msg(&mut self, server_id: Uuid, user_id: Uuid, member_id: Uuid) {
        self.subscriptions
            .unsubscribe(user_id, server_id, member_id);
        self.send_msg_to_sever(
            server_id,
            ServerMessage {
                server_id,
                msg: Message::MemberDisconnected { member_id },
            },
        );
    }

    pub fn handle_client_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::ServerMessage(ref server_message) => {
                self.send_msg_to_sever(server_message.server_id, msg);
            }
            ClientMessage::ServerDeleted { server_id } => {
                self.send_msg_to_sever(server_id, msg);
            }
            ClientMessage::JoinedToServer { user_id, .. } => {
                self.send_msg_to_user(user_id, msg);
            }
            ClientMessage::LeavedServer { user_id, .. } => {
                self.send_msg_to_user(user_id, msg);
            }
        }
    }

    pub fn handle_user_disconnect(&mut self, user_id: Uuid) {
        let servers = self.subscriptions.unsubscribe_all(user_id);
        for (server_id, member_id) in servers {
            self.send_msg_to_sever(
                server_id,
                ServerMessage {
                    server_id,
                    msg: Message::MemberDisconnected { member_id },
                },
            );
        }
    }

    pub fn handle_app_msg(&mut self, message: AppMessage) {
        match message {
            AppMessage::ClientMessage(client_message) => {
                self.handle_client_message(client_message);
            }
            AppMessage::ClosedConnection { user_id } => {
                self.handle_user_disconnect(user_id);
            }
            AppMessage::Subscribe {
                user_id,
                server_id,
                member_id,
            } => {
                self.handle_sub_msg(server_id, user_id, member_id);
            }
            AppMessage::Unsubscribe {
                user_id,
                server_id,
                member_id,
            } => {
                self.handle_unsub_msg(server_id, user_id, member_id);
            }
            AppMessage::Batch(app_messages) => {
                for msg in app_messages {
                    self.handle_app_msg(msg);
                }
            }
        }
    }

    pub fn send_msg_to_sever(
        &mut self,
        server_id: Uuid,
        msg: impl Into<ClientMessage> + std::fmt::Debug,
    ) {
        if let Some(users) = self.subscriptions.server_subscriptions.get(&server_id) {
            debug!("sending msg: {msg:?} to users: {users:?}");
            let msg = std::convert::Into::<ClientMessage>::into(msg);
            for user in users {
                if let Some(sender) = self.channels.get(user) {
                    let _ = sender.send(AppMessage::ClientMessage(msg.clone()));
                }
            }
        }
    }

    pub fn send_msg_to_user(
        &mut self,
        user_id: Uuid,
        msg: impl Into<ClientMessage> + std::fmt::Debug,
    ) {
        if let Some(sender) = self.channels.get(&user_id) {
            let _ = sender.send(AppMessage::ClientMessage(msg.into()));
        }
    }
}
