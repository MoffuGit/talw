pub mod file_status;
pub mod list_files;
pub mod upload_file;

use anyhow::{anyhow, Error};
use futures::{future, Future};
use http::HeaderValue;
use reqwest::{header, multipart, Client, Response};
use serde::Serialize;
use serde_json::json;
use std::env;
use std::time::Duration;
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

pub const VERSION: &str = "6.13.2";

impl Default for UploadThing {
    fn default() -> Self {
        let api_key = env::var("UPLOADTHING_SECRET").expect("API key isnt found");
        let client = Client::new();
        UploadThing {
            host: "https://api.uploadthing.com".to_string(),
            user_agent: format!("talw/{}/rust", VERSION),
            api_key,
            version: VERSION.to_string(),
            client,
        }
    }
}

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use tokio::task::JoinError;
        use tokio_util::sync::CancellationToken;
    }
}

#[cfg(feature = "ssr")]
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
            Err(anyhow!(serde_json::to_string_pretty(
                &response.json::<serde_json::Value>().await?,
            )?))
        }
    }

    pub async fn list_files(&self) -> Result<ListFiles, Error> {
        self.list_files_with_options(ListFilesOpts::default()).await
    }

    pub async fn list_files_with_options(&self, opts: ListFilesOpts) -> Result<ListFiles, Error> {
        Ok(self
            .send_request("v6/listFiles", &opts)
            .await?
            .json::<ListFiles>()
            .await?)
    }

    pub async fn upload_file(
        &self,
        chunks: Vec<u8>,
        file_data: FileData,
        wait_until_done: bool,
    ) -> Result<UploadFileResponse, Error> {
        self.upload_file_with_options(
            UploadFileOpts::default(),
            chunks,
            file_data,
            wait_until_done,
        )
        .await
    }

    pub async fn upload_file_with_options(
        &self,
        opts: UploadFileOpts,
        chunks: Vec<u8>,
        file_data: FileData,
        wait_until_done: bool,
    ) -> Result<UploadFileResponse, Error> {
        let presigned_url_data = self.get_presigned_url(&opts, &file_data).await?;
        let presigned_url = presigned_url_data
            .data
            .first()
            .expect("acces to the first value");
        println!("this is the data: {:?}", presigned_url);
        if let (Some(urls), Some(chunk_size)) = (&presigned_url.urls, &presigned_url.chunk_size) {
            self.upload_multipart(&opts, &chunks, presigned_url, urls, chunk_size, &file_data)
                .await?;
        } else {
            let url = presigned_url.url.clone().expect("acces to the url");
            self.upload_presigned_post(presigned_url, &url, &file_data, &chunks)
                .await?;
        }
        if wait_until_done {
            let pool_url = format!("{}/v6/pollUpload/{}", self.host, &presigned_url.key);
            retry_async_with_time(|| self.poll_file_data(&pool_url), 60).await?;
        }
        println!("finished the upload of the file");
        Ok(UploadFileResponse {
            key: presigned_url.key.clone(),
            url: presigned_url.file_url.clone(),
            name: file_data.name.clone(),
            size: file_data.size,
        })
    }

    async fn upload_presigned_post(
        &self,
        presigned: &PresignedUrlResponseData,
        url: &str,
        file_data: &FileData,
        chunk: &[u8],
    ) -> Result<Response, Error> {
        println!("Upload from presigned post");
        let mut form = multipart::Form::new();
        for (k, v) in presigned
            .fields
            .clone()
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
        {
            let value = v.clone().to_owned().as_str().unwrap().to_owned();
            form = form.text(k.clone(), value);
        }

        let file_part = multipart::Part::bytes(chunk.to_vec()).file_name(file_data.name.clone());
        form = form.part("file", file_part);

        match self
            .client
            .post(url)
            .header("x-uploadthing-api-key", &self.api_key)
            .multipart(form)
            .send()
            .await
        {
            Ok(res) => match res.status().is_success() {
                true => Ok(res),
                false => {
                    eprintln!("Failed to upload file: {}", res.text().await?);
                    Err(anyhow!("status code is not good"))
                }
            },
            Err(err) => {
                self.client
                    .post("v6/failureCallback")
                    .json(&json!({
                        "fileKey": &presigned.key,
                        "uploadId": null
                    }))
                    .header(header::CONTENT_TYPE, "application/json")
                    .send()
                    .await?;
                Err(anyhow!(err))
            }
        }
    }

    async fn poll_file_data(&self, url: &str) -> Result<(), Error> {
        println!("poll url: {url}");
        match self.client.get(url).send().await {
            Ok(res) => match res.json::<serde_json::Value>().await {
                Err(err) => Err(err.into()),
                Ok(json) => {
                    println!("poll json: {:?}", json);
                    if json["status"] == "done" {
                        return Ok(());
                    }
                    Err(anyhow!("The status is not done"))
                }
            },
            Err(err) => {
                eprintln!("[UT] Error polling for file data for {}: {}", url, err);
                Err(anyhow!(err))
            }
        }
    }

    async fn upload_multipart(
        &self,
        opts: &UploadFileOpts,
        chunks: &[u8],
        presigned: &PresignedUrlResponseData,
        urls: &[String],
        chunk_size: &usize,
        file_data: &FileData,
    ) -> Result<Response, Error> {
        println!(
            "Uploading file {} with {} chunks of size {} bytes each",
            file_data.name,
            urls.len(),
            chunk_size
        );

        let cancel_token = CancellationToken::new();

        let upload_parts = future::join_all(
            chunks
                .chunks(*chunk_size)
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
                            res = retry_async_with_tries(|| client.upload_part(&url,&chunk , &file_data.file_type, &file_data.name, &content_disposition), 10) => {
                                match res {
                                    Ok(res) => {Ok(Etag {
                                        tag: res.to_str()?.to_string(),
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
                self.complete_multipart_upload(etags, &presigned.key, &presigned.upload_id)
                    .await
            }
            _ => {
                match self
                    .abort_multipart_upload(&presigned.key, &presigned.upload_id)
                    .await
                {
                    Ok(_) => Err(anyhow!("Aborted the multipartUpload")),
                    Err(err) => Err(err),
                }
            }
        }
    }

    async fn abort_multipart_upload(
        &self,
        key: &str,
        upload_id: &Option<String>,
    ) -> Result<Response, Error> {
        self.send_request(
            "v6/failureCallback",
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
        upload_id: &Option<String>,
    ) -> Result<Response, Error> {
        self.send_request(
            "v6/completeMultipart",
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
        chunk: &[u8],
        content_type: &str,
        file_name: &str,
        content_disposition: &ContentDisposition,
    ) -> Result<HeaderValue, Error> {
        match self
            .client
            .put(url)
            .body(chunk.to_vec())
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
                "v6/uploadFiles",
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

// Ok(response) => match response.json::<Value>().await {
//     Err(err) => {
//         eprintln!("Error parsign the response: {err}");
//         Err(err.into())
//     }
//     Ok(presigned_url) => {
//         println!("{presigned_url}");
//         Err(anyhow!("some"))
//     }
// },
#[cfg(feature = "ssr")]
async fn retry_async_with_tries<T, E, Fut, F: Fn() -> Fut>(
    f: F,
    max_retries: usize,
) -> Result<T, Error>
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

        match f().await {
            Ok(res) => return Ok(res),
            Err(err) => {
                retries += 1;
                backoff_ms *= 4;
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

#[cfg(feature = "ssr")]
async fn retry_async_with_time<T, E, Fut, F: Fn() -> Fut>(f: F, max_time: u64) -> Result<T, Error>
where
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut retries = 0;
    let mut backoff_ms = 10;
    loop {
        match f().await {
            Ok(res) => return Ok(res),
            Err(err) => {
                if backoff_ms / 1000 > max_time {
                    return Err(anyhow!("Operation cancelled after {} retries", retries));
                }
                retries += 1;
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
                backoff_ms *= 4;
            }
        }
    }
}
