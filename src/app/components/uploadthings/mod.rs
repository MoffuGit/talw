use gloo_file::futures::read_as_bytes;
use gloo_file::{Blob, File, FileReadError};
use leptos::task::spawn_local_scoped_with_cancellation;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::uploadthing::FileData;

pub mod drop;
pub mod input;

#[derive(Serialize, Deserialize, Clone)]
pub struct UploadthingFile {
    pub data: FileData,
    pub chunks: Vec<u8>,
}

#[derive(Debug, Display, EnumString, PartialEq, Default)]
pub enum FileType {
    #[strum(serialize = "image/jpeg", serialize = "image/jpg")]
    Jpeg,
    #[strum(serialize = "image/png")]
    Png,
    #[strum(serialize = "image/gif")]
    Gif,
    #[strum(serialize = "image/webp")]
    Webp,
    #[strum(serialize = "application/pdf")]
    Pdf,
    #[strum(serialize = "text/plain")]
    Text,
    #[strum(serialize = "application/msword")]
    Doc,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    Docx,
    #[strum(serialize = "application/vnd.ms-excel")]
    Xls,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    Xlsx,
    #[strum(serialize = "application/zip")]
    Zip,
    #[strum(serialize = "audio/mpeg")]
    Mp3,
    #[strum(serialize = "audio/wav")]
    Wav,
    #[strum(serialize = "video/mp4")]
    Mp4,
    #[strum(serialize = "video/webm")]
    Webm,
    #[strum(serialize = "application/json")]
    Json,
    #[strum(serialize = "text/csv")]
    Csv,
    #[strum(serialize = "text/html")]
    Html,
    #[default]
    Unknown,
}

fn read_file<F>(file: File, callback: F)
where
    F: FnOnce(Result<UploadthingFile, FileReadError>) + 'static,
{
    let name = file.name();
    let file_type = file.raw_mime_type();
    let size = file.size() as usize;
    spawn_local_scoped_with_cancellation(async move {
        callback(
            read_as_bytes(&Blob::from(file))
                .await
                .map(|chunks| UploadthingFile {
                    data: FileData {
                        name,
                        file_type,
                        size,
                    },
                    chunks,
                }),
        )
    });
}
