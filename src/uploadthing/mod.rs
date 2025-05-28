#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "ssr")]
pub use self::server::UploadThing;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileData {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: usize,
}
