use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServersStoreSync {
    Updated { id: Uuid },
    Join { server: Server },
    Leave { id: Uuid },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerStoreSync {}
