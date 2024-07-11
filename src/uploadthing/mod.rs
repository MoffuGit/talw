pub mod file_status;
pub mod list_files;
pub mod upload_file;
use std::env;
use std::sync::Arc;

use anyhow::{anyhow, Error};
use futures::{future, Future, StreamExt, TryStreamExt};
use http::HeaderValue;
use multer::bytes::Bytes as MulterBytes;
use multer::Field;
use reqwest::{header, Client, Response};
use serde::Serialize;
use serde_json::json;
use std::time::Duration;
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;
use urlencoding::encode;

use crate::uploadthing::upload_file::UploadFileOpts;

use self::list_files::{ListFiles, ListFilesOpts};
use self::upload_file::PresignedUrlResponseData;
use self::upload_file::UploadFileResponse;
use self::upload_file::{ContentDisposition, FileData};
use self::upload_file::{Etag, PresignedUrlResponse};

#[derive(Clone, Debug)]
pub struct UploadThing {
    host: String,
    user_agent: String,
    api_key: String,
    version: String,
    client: Client,
}

pub const VERSION: &str = "6.12.0";

impl Default for UploadThing {
    fn default() -> Self {
        let api_key = env::var("UPLOADTHING_SECRET").expect("API key isnt found");
        let client = Client::new();
        UploadThing {
            host: "https://uploadthing.com".to_string(),
            user_agent: format!("talw/{}/rust", VERSION),
            api_key,
            version: VERSION.to_string(),
            client,
        }
    }
}

impl UploadThing {
    async fn send_request<T: Serialize>(
        &self,
        end_point: &str,
        payload: &T,
    ) -> Result<Response, Error> {
        let response = self
            .client
            .post(format!("{}/{}", self.host, end_point))
            .json(payload)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CACHE_CONTROL, "no-store")
            .header(header::USER_AGENT, &self.user_agent)
            .header("x-uploadthing-api-key", self.api_key.to_string())
            .header("x-uploadthing-version", self.version.to_string())
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            let response_data =
                serde_json::to_string_pretty(&response.json::<serde_json::Value>().await?)?.clone();
            Err(anyhow!(response_data))
        }
    }

    pub async fn list_files(&self) -> Result<ListFiles, Error> {
        self.list_files_with_options(ListFilesOpts::default()).await
    }

    pub async fn list_files_with_options(&self, opts: ListFilesOpts) -> Result<ListFiles, Error> {
        Ok(self
            .send_request("/api/listFiles", &opts)
            .await?
            .json::<ListFiles>()
            .await?)
    }

    pub async fn upload_file(
        &self,
        field: Field<'static>,
        wait_until_done: bool,
    ) -> Result<UploadFileResponse, Error> {
        self.upload_file_with_options(UploadFileOpts::default(), field, wait_until_done)
            .await
    }

    pub async fn upload_file_with_options(
        &self,
        opts: UploadFileOpts,
        field: Field<'static>,
        _wait_until_done: bool,
    ) -> Result<UploadFileResponse, Error> {
        let name = field.file_name().expect("no filename on field").to_string();
        let file_type = field.content_type().expect("mime type").to_string();
        // let some = field.map(|some| 1);
        let size = field
            .try_fold(0, |size, chunk| async move { Ok(size + chunk.len()) })
            .await?;

        let file_data = FileData {
            name,
            file_type,
            size,
        };

        // if let Some(presigned_url) = self
        //     .get_presigned_url(&opts, &file_data)
        //     .await?
        //     .data
        //     .first()
        // {
        //     if let Some(urls) = presigned_url.urls {
        //         self.upload_multipart(&opts, field, presigned_url, &file_data);
        //     }
        //     todo!()
        // };
        todo!()
    }

    async fn upload_multipart(
        &self,
        opts: &UploadFileOpts,
        field: Field<'static>,
        presigned: &PresignedUrlResponseData,
        file_data: &FileData,
    ) -> Result<Response, Error> {
        let urls = presigned.urls.clone().expect("acces to the urls");
        let chunk_size = presigned.chunk_size.expect("acces to the chunk size") as usize;
        println!(
            "Uploading file {} with {} chunks of size {} bytes each",
            file_data.name,
            urls.len(),
            chunk_size
        );

        let cancel_token = CancellationToken::new();

        let upload_parts = future::join_all(
            field
                .try_collect::<Vec<MulterBytes>>()
                .await?
                .concat()
                .chunks(chunk_size)
                .zip(urls.iter()).enumerate()
                .map(|(idx, (chunk, url))| {
                    let file_data = file_data.clone();
                    let content_disposition = opts.content_disposition.clone();
                    let cancel_token =cancel_token.clone();
                    let client = self.clone();
                    let chunk = chunk.to_vec();
                    let url = url.clone();
                    tokio::spawn(async move {
                        tokio::select! {
                            _ = cancel_token.cancelled() => {
                                Err(anyhow!("Part cancelled from cancellation token"))
                            }
                            res = retry_async(|| client.upload_part(&url,chunk.clone() , &file_data.file_type, &file_data.name, &content_disposition), 10) => {
                                match res {
                                    Ok(res) => {Ok(Etag {
                                        tag: res.to_str().expect("parse the header to a str").to_string(),
                                        part_number: idx + 1
                                    })},
                                    Err(err) => {
                                        cancel_token.cancel();
                                        Err(err)
                                    }
                                }
                            }
                        }
                    })
                }),
        )
        .await.into_iter().collect::<Result<Result<Vec<Etag>, Error>, JoinError>>();
        match upload_parts {
            Ok(Ok(etags)) => {
                self.complete_multipart_upload(etags, &presigned.key, &presigned.file_url)
                    .await
            }
            _ => {
                match self
                    .abort_multipart_upload(&presigned.key, &presigned.file_url)
                    .await
                {
                    Ok(_) => Err(anyhow!("Aborted the multipartUpload")),
                    Err(err) => Err(err),
                }
            }
        }
    }

    async fn abort_multipart_upload(&self, key: &str, upload_id: &str) -> Result<Response, Error> {
        self.send_request(
            "/api/failureCallback",
            &json!({
                "fileKey": key,
                "uploadId": upload_id
            }),
        )
        .await
    }

    async fn complete_multipart_upload(
        &self,
        etags: Vec<Etag>,
        key: &str,
        upload_id: &str,
    ) -> Result<Response, Error> {
        self.send_request(
            "/api/completeMultipart",
            &json!({
                "fileKey": key,
                "uploadId": upload_id,
                "etags": etags
            }),
        )
        .await
    }

    async fn upload_part(
        &self,
        url: &str,
        chunk: Vec<u8>,
        content_type: &str,
        file_name: &str,
        content_disposition: &ContentDisposition,
    ) -> Result<HeaderValue, Error> {
        match self
            .client
            .put(url)
            .body(chunk)
            .header(header::CONTENT_TYPE, content_type)
            .header(
                header::CONTENT_DISPOSITION,
                format!(
                    r#"{}; filename="{}"; filename*=UTF-8''{}"#,
                    content_disposition,
                    encode(file_name),
                    encode(file_name)
                ),
            )
            .send()
            .await
        {
            Ok(res) => match res.headers().get("Etag") {
                Some(value) => Ok(value.clone()),
                None => Err(anyhow!("Cant get the header value")),
            },
            Err(err) => Err(err.into()),
        }
    }

    async fn get_presigned_url(
        &self,
        opts: &UploadFileOpts,
        file_data: &FileData,
    ) -> Result<PresignedUrlResponse, Error> {
        match self
            .send_request(
                "/api/uploadFiles",
                &json!({
                    "files": vec![file_data],
                    "metadata": opts.metadata,
                    "contentDisposition": opts.content_disposition,
                    "acl": opts.acl
                }),
            )
            .await
        {
            Err(err) => {
                eprintln!("Error getting presigned urls: {err}");
                Err(err)
            }
            Ok(response) => match response.json::<PresignedUrlResponse>().await {
                Err(err) => {
                    eprintln!("Error parsign the response: {err}");
                    Err(err.into())
                }
                Ok(presigned_url) => Ok(presigned_url),
            },
        }
    }
}

async fn retry_async<T, E, Fut, F: Fn() -> Fut>(f: F, max_retries: usize) -> Result<T, Error>
where
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut retries = 0;
    let mut backoff_ms = 100;
    loop {
        if retries == max_retries {
            return Err(anyhow!("Operation cancelled after {} retries", retries));
        }

        let res = f().await;
        match res {
            Ok(res) => return Ok(res),
            Err(err) => {
                retries += 1;
                backoff_ms *= 2;
                println!(
                    "Retry {} fail with err {}. Retrying in {} seconds...",
                    retries,
                    err,
                    backoff_ms / 1000
                );

                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        return Err(anyhow::anyhow!("Operation cancelled"));
                    }
                    _ = tokio::time::sleep(Duration::from_millis(backoff_ms)) => {}
                }
            }
        }
    }
}
