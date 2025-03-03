use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::topic::Topic;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Messages {
    Subscribe {
        topic: Topic,
    },
    Unsubscribe {
        topic: Topic,
    },
    ChatMessage {
        sender_id: Uuid,
        chat_id: Uuid,
        content: String,
        timestamp: i64,
    },
    Typing {
        user_id: Uuid,
        chat_id: Uuid,
        is_typing: bool,
    },
}
