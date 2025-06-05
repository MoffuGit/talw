use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::category::Category;
use crate::entities::channel::Channel;
use crate::entities::message::{Attachment, ChannelMessage, Embed, Reaction};
use crate::entities::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServersStoreSync {
    Updated { id: Uuid },
    Join { server: Server },
    Leave { id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChannelStoreSync {
    Deleted { id: Uuid },
    Created { channel: Channel },
    Updated { id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CategoryStoreSync {
    Deleted { id: Uuid },
    Created { category: Category },
    Updated { id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageStoreSync {
    Created { message: Box<ChannelMessage> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageSync {
    Pin {
        id: Uuid,
    },
    Unpin {
        id: Uuid,
    },
    NewReaction {
        id: Uuid,
        reaction: Reaction,
    },
    DeletedReaction {
        id: Uuid,
        reaction: Uuid,
    },
    MemberReact {
        member: Uuid,
        id: Uuid,
        reaction: Uuid,
    },
    MemberUnreact {
        member: Uuid,
        id: Uuid,
        reaction: Uuid,
    },
    Attachments {
        id: Uuid,
        attachments: Vec<Attachment>,
    },
    Embeds {
        id: Uuid,
        embeds: Vec<Embed>,
    },
}
