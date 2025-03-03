use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub enum Topic {
    Server(Uuid),
    Channel(Uuid),
}
