use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::server::Server;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppMessage {
    ClientMessage(ClientMessage),
    ClosedConnection { user_id: Uuid },
    Subscribe { user_id: Uuid, server_id: Uuid },
    Unsubscribe { user_id: Uuid, server_id: Uuid },
    Batch(Vec<AppMessage>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    ServerMessage(ServerMessage),
    ServerDeleted { server_id: Uuid },
    JoinedToServer { server: Server, user_id: Uuid },
    LeavedServer { server_id: Uuid, user_id: Uuid },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServerMessage {
    pub server_id: Uuid,
    pub msg: Message,
}

impl From<ClientMessage> for AppMessage {
    fn from(val: ClientMessage) -> Self {
        AppMessage::ClientMessage(val)
    }
}

impl From<ServerMessage> for AppMessage {
    fn from(val: ServerMessage) -> Self {
        AppMessage::ClientMessage(ClientMessage::ServerMessage(val))
    }
}

impl From<ServerMessage> for ClientMessage {
    fn from(value: ServerMessage) -> Self {
        ClientMessage::ServerMessage(value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Message {
    UserConnected {
        user_id: Uuid,
    },
    UserDisconnected {
        user_id: Uuid,
    },
    ChannelMessage {
        channel_id: Uuid,
        content: String,
        //content: Message
    },
    ThreadMessage {
        thread_id: Uuid,
        content: String,
        //content: Message
    },
    MemberJoinedServer {
        user_id: Uuid,
    },
    MemberLeftServer {
        user_id: Uuid,
    },
    ServerUpdated {
        name: Option<String>,
        image: Option<String>,
    },
    ThreadCreated {
        thread_id: Uuid,
    },
    ThreadDeleted {
        thread_id: Uuid,
    },
    ChannelCreated {
        new_channel: Channel,
    },
    ChannelDeleted {
        channel_id: Uuid,
    },
    ChannelUpdated {
        channel_id: Uuid,
        topic: Option<String>,
        name: Option<String>,
    },
    Typing {
        user_id: Uuid,
        chat_id: Uuid,
        is_typing: bool,
    },
    CategoryCreated {
        new_category: Category,
    },
    CategoryUpdated {
        category_id: Uuid,
        new_name: String,
    },
    CategoryDeleted {
        category_id: Uuid,
    },
}
