#[cfg(feature = "ssr")]
pub mod connections;
#[cfg(feature = "ssr")]
pub mod router;
#[cfg(feature = "ssr")]
pub mod subs;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncRequest {
    Mutation(MutationRequest),
    Subscription(SubscriptionRequest),
    Unsubscription(UnsubscriptionRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRequest {
    pub key: String,
    pub module: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutation {
    pub module: String,
    pub data: Value,
}

impl From<MutationRequest> for Mutation {
    fn from(value: MutationRequest) -> Self {
        Self {
            module: value.module,
            data: value.data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    pub keys: HashSet<String>,
    pub client: Uuid,
    pub action: SubscriptionMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionMode {
    Add,
    ReplaceAll,
    ReplaceByPrefix(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscriptionRequest {
    pub keys: HashSet<String>,
    pub prefix: Option<String>,
    pub client: Uuid,
}
