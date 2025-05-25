mod channel;
mod server;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::server::*;
use crate::entities::member::Member;
use crate::entities::server::Server;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AppMessage {
    ClientMessage(ClientMessage),
    //ServerMessage(ServerMessage)
    //ChannelMessage(ChannelMessage)
    //ThreadMessage(ThreadMessage)
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

impl From<ClientMessage> for AppMessage {
    fn from(val: ClientMessage) -> Self {
        AppMessage::ClientMessage(val)
    }
}
