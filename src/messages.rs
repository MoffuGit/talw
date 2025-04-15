use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::channel::Channel;
use crate::topic::Topic;

//NOTE:
//they are missing more messages, like, message read, user banned but at this point im ok with only
//more messages
//reaction
//category changes
//private messages
//allmost al the things that the user already can change should send a message to the server
//i will be thinking what more to add when i return to the project, at this point i can think well
//if i can see the state of the app
//impl this messages
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Message {
    Batch(Vec<Message>),
    Subscribe {
        topic: Topic,
        user_id: Uuid,
    },
    Unsubscribe {
        topic: Topic,
        user_id: Uuid,
    },
    ChatMessage {
        chat_id: Uuid,
        content: String,
        //content: Message
    },
    UserConnected {
        user_id: Uuid,
        server_id: Uuid,
    },
    UserDisconnected {
        user_id: Uuid,
        server_id: Uuid,
    },
    UserJoinedServer {
        user_id: Uuid,
        server_id: Uuid,
    },
    UserLeftServer {
        user_id: Uuid,
        server_id: Uuid,
    },
    ServerDeleted {
        server_id: Uuid,
    },
    ServerUpdated {
        server_id: Uuid,
        //server_update: name,settings...
    },
    ChannelCreated {
        server_id: Uuid,
        // channel: Channel,
    },
    ChannelDeleted {
        server_id: Uuid,
        channel_id: Uuid,
    },
    ChannelUpdated {
        server_id: Uuid,
        //channel_update: name, settings...
    },
    //UserStatus
    Typing {
        user_id: Uuid,
        chat_id: Uuid,
        is_typing: bool,
    },
    Close,
}
