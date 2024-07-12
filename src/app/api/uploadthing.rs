use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};

use crate::uploadthing::{list_files::ListFiles, UploadThing};

#[server(GetListFiles, "/api")]
pub async fn get_list_files() -> Result<ListFiles, ServerFnError> {
    let uploadthing = use_context::<UploadThing>().expect("acced to uploadthing");
    match uploadthing.list_files().await {
        Err(err) => Err(ServerFnError::new(format!("got this error: {}", err))),
        Ok(files) => Ok(files),
    }
}

#[server(name = UploadFile, prefix = "/api", input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<usize, ServerFnError> {
    let uploadthing = use_context::<UploadThing>().expect("acced to uploadthing");
    let mut data = data.into_inner().unwrap();

    if let Ok(Some(field)) = data.next_field().await {
        match uploadthing.upload_file(field, true).await {
            Ok(res) => println!("{res:?}"),
            Err(err) => println!("{err}"),
        }
        Ok(0)
    } else {
        Err(ServerFnError::new("cant get the len of the file"))
    }
}
