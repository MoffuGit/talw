use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::member::Member;
use crate::entities::message::ChannelMessage;
use crate::entities::role::Role;
use crate::entities::server::Server;
use crate::entities::thread::Thread;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppMessage {
    ClientMessage(ClientMessage),
    ClosedConnection {
        user_id: Uuid,
    },
    Subscribe {
        user_id: Uuid,
        server_id: Uuid,
        member_id: Uuid,
    },
    Unsubscribe {
        user_id: Uuid,
        server_id: Uuid,
        member_id: Uuid,
    },
    Batch(Vec<AppMessage>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientMessage {
    ServerMessage(ServerMessage),
    ServerDeleted {
        server_id: Uuid,
    },
    JoinedToServer {
        server: Server,
        user_id: Uuid,
        member: Member,
    },
    LeavedServer {
        server_id: Uuid,
        user_id: Uuid,
    },
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
    RoleCreated {
        role: Role,
    },
    RoleDeleted {
        role_id: Uuid,
    },
    RoleUpdated {
        role_id: Uuid,
        name: Option<String>,
        priority: Option<u8>,
        can_edit: Option<bool>,
    },
    MemberUpdated {
        member_id: Uuid,
        name: Option<String>,
        image_url: Option<String>,
    },
    MemberConnected {
        member_id: Uuid,
    },
    MemberDisconnected {
        member_id: Uuid,
    },
    ChannelMessage {
        channel_id: Uuid,
        content: Box<ChannelMessage>,
    },
    ThreadMessage {
        thread_id: Uuid,
        content: String,
        //content: Message
    },
    MemberJoinedServer {
        member: Member,
    },
    MemberLeftServer {
        member_id: Uuid,
    },
    ServerUpdated {
        name: Option<String>,
        image: Option<String>,
    },
    ThreadCreated {
        thread: Thread,
    },
    MemberJoinThread {
        thread_id: Uuid,
        member_id: Uuid,
    },
    MemberLeaveThread {
        thread_id: Uuid,
        member_id: Uuid,
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
    UnpinMessage {
        message_id: Uuid,
    },
    PinMessage {
        message_id: Uuid,
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
