use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Messages {
    ChatMessage {
        sender_id: Uuid,
        chat_id: Uuid,
        content: String,
        timestamp: i64, 
    },
    AggregationRequest,
    AggregationResponse,
    CloseConnection {
        code: Option<u16>,
        reason: Option<String>, 
    },
}
