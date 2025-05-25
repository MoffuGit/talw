use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::member::Member;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AppMessage, ClientMessage};
use crate::entities::message::{ChannelMessage, Reaction};
use crate::entities::role::Role;
use crate::entities::thread::Thread;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServerMessage {
    pub server_id: Uuid,
    pub msg: Message,
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
    ReactionCreated {
        reaction: Reaction,
        message_id: Uuid,
    },
    ReactionDeleted {
        reaction_id: Uuid,
        message_id: Uuid,
    },
    MemberReact {
        react_id: Uuid,
        message_id: Uuid,
        member_id: Uuid,
    },
    MemberUnreact {
        react_id: Uuid,
        message_id: Uuid,
        member_id: Uuid,
    },
}
