use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::category::Category;
use crate::entities::channel::Channel;

//NOTE:
//they are missing more messages, like, message read, user banned but at this point im ok with only
//more messages
//reaction
//category changes
//private messages
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Message {
    Batch(Vec<Message>),
    Subscribe {
        server_id: Uuid,
        user_id: Uuid,
    },
    Unsubscribe {
        server_id: Uuid,
        user_id: Uuid,
    },
    ChannelMessage {
        server_id: Uuid,
        channel_id: Uuid,
        content: String,
        //content: Message
    },
    ThreadMessage {
        server_id: Uuid,
        thread_id: Uuid,
        content: String,
        //content: Message
    },
    UserConnected {
        server_id: Uuid,
        user_id: Uuid,
    },
    UserDisconnected {
        user_id: Uuid,
    },
    MemberJoinedServer {
        member_id: Uuid,
        server_id: Uuid,
    },
    MemberLeftServer {
        member_id: Uuid,
        server_id: Uuid,
    },
    ServerDeleted {
        server_id: Uuid,
    },
    ServerUpdated {
        server_id: Uuid,
        name: Option<String>,
        image: Option<String>,
    },
    ThreadCreated {
        server_id: Uuid,
        thread_id: Uuid,
    },
    ThreadDeleted {
        server_id: Uuid,
        thread_id: Uuid,
    },
    ChannelCreated {
        server_id: Uuid,
        new_channel: Channel,
    },
    ChannelDeleted {
        server_id: Uuid,
        channel_id: Uuid,
    },
    ChannelUpdated {
        server_id: Uuid,
        topic: Option<String>,
        name: Option<String>,
    },
    Typing {
        user_id: Uuid,
        chat_id: Uuid,
        is_typing: bool,
    },
    CategoryCreated {
        server_id: Uuid,
        new_category: Category,
    },
    CategoryUpdated {
        server_id: Uuid,
        id: Uuid,
        new_name: String,
    },
    CategoryDeleted {
        server_id: Uuid,
        id: Uuid,
    },
    Close,
}
