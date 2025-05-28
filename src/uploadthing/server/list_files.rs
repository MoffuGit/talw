use crate::uploadthing::server::upload_file::FileStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ListFilesOpts {
    limit: Option<i32>,
    offset: Option<i32>,
}

impl Default for ListFilesOpts {
    fn default() -> Self {
        ListFilesOpts {
            limit: Some(10),
            offset: Some(0),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ListFile {
    key: String,
    id: String,
    status: FileStatus,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ListFiles {
    pub files: Vec<ListFile>,
}
