#[cfg(feature = "ssr")]
pub mod connections;
#[cfg(feature = "ssr")]
pub mod router;
#[cfg(feature = "ssr")]
pub mod subs;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncRequest {
    Mutation {
        key: String,
        data: Value,
    },
    Subscription {
        keys: Vec<String>,
        client: Uuid,
        action: SubscriptionMode,
    },
    Unsubscription {
        keys: Vec<String>,
        prefix: Option<String>,
        client: Uuid,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutation {
    pub module: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionMode {
    Add,
    ReplacePrefix(String),
}
