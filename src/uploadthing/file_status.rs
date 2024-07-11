use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone)]
pub enum FileStatus {
    Failed,
    Uploaded,
    Uploading,
    DeletionPending,
}
