use std::collections::HashMap;

use serde::Serialize;
use strum::Display;

#[derive(Serialize, Clone)]
pub struct UploadFileOpts {
    pub metadata: HashMap<String, String>,
    #[serde(rename(serialize = "contentDisposition"))]
    pub content_disposition: ContentDisposition,
    pub acl: Acl,
}

impl Default for UploadFileOpts {
    fn default() -> Self {
        UploadFileOpts {
            metadata: HashMap::new(),
            content_disposition: ContentDisposition::Inline,
            acl: Acl::PublicRead,
        }
    }
}

#[derive(Clone)]
pub struct UploadPartOpts {
    pub url: String,
    pub chunk: Vec<u8>,
    pub max_retries: usize,
}

#[derive(Serialize, Clone)]
pub enum Acl {
    #[serde(rename(serialize = "public-read"))]
    PublicRead,
    #[serde(rename(serialize = "private"))]
    Private,
}

#[derive(Serialize, Clone, Debug, Display)]
pub enum ContentDisposition {
    #[serde(rename(serialize = "inline"))]
    Inline,
    #[serde(rename(serialize = "attachment"))]
    Attachment,
}

#[derive(Serialize, Clone)]
pub struct Etag {
    pub tag: String,
    #[serde(rename = "partNumber")]
    pub part_number: usize,
}

#[derive(Serialize, Clone)]
pub struct FileData {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub size: usize,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct PresignedUrlResponse {
    pub data: Vec<PresignedUrlResponseData>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct PresignedUrlResponseData {
    pub fields: Option<serde_json::Value>,
    #[serde(rename = "fileUrl")]
    pub file_url: String,
    pub key: String,
    #[serde(rename = "uploadId")]
    pub upload_id: Option<String>,
    pub url: Option<String>,
    pub urls: Option<Vec<String>>,
    #[serde(rename = "chunkSize")]
    pub chunk_size: Option<usize>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UploadFileResponse {
    pub key: String,
    pub url: String,
    pub name: String,
    pub size: usize,
}
