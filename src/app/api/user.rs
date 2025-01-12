use crate::entities::user::{Banner, Profile, User};
use cfg_if::cfg_if;
use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use uuid::Uuid;
use web_sys::FormData;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use futures::TryStreamExt;
        use crate::uploadthing::upload_file::FileData;
        use crate::uploadthing::UploadThing;
        use multer::bytes::Bytes as MulterBytes;
        use crate::entities::server::Server;
        use super::auth_user;
        use super::pool;
        use super::auth;
    }
}

#[derive(Clone, Copy)]
pub struct UserContext {
    pub edit_banner_image: Action<FormData, Result<(), ServerFnError>>,
    pub edit_profile_image: Action<FormData, Result<(), ServerFnError>>,
    pub banner: Resource<(usize, usize, usize), Result<Banner, ServerFnError>>,
    pub profile: Resource<(usize, usize, usize), Result<Profile, ServerFnError>>,
    pub edit_profile_name: Action<EditUserName, Result<(), ServerFnError>>,
    pub edit_banner_about: Action<EditUserAbout, Result<(), ServerFnError>>,
    pub edit_user_data: Action<EditUserData, Result<(), ServerFnError>>,
}

pub fn provide_user_context(user_id: Uuid) {
    let edit_banner_image = create_action(|data: &FormData| {
        let data = data.clone();
        edit_image_banner(data.into())
    });
    let edit_banner_about = create_server_action::<EditUserAbout>();
    let edit_profile_image = create_action(|data: &FormData| {
        let data = data.clone();
        edit_profile_image(data.into())
    });
    let edit_profile_name = create_server_action::<EditUserName>();
    let edit_user_data = create_server_action::<EditUserData>();
    let banner = create_resource(
        move || {
            (
                edit_banner_image.version().get(),
                edit_banner_about.version().get(),
                edit_user_data.version().get(),
            )
        },
        move |_| get_user_banner(user_id),
    );
    let profile = create_resource(
        move || {
            (
                edit_profile_image.version().get(),
                edit_profile_name.version().get(),
                edit_user_data.version().get(),
            )
        },
        move |_| get_user_profile(user_id),
    );

    provide_context(UserContext {
        edit_banner_about,
        edit_profile_name,
        edit_profile_image,
        edit_banner_image,
        banner,
        profile,
        edit_user_data,
    });
}

#[server(EditUserName)]
pub async fn edit_user_name(new_name: String) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    if new_name.is_empty() {
        return Ok(());
    }

    Ok(User::set_profile_name(auth.id, new_name, &pool).await?)
}

#[server(EditUserAbout)]
pub async fn edit_user_about(new_about: String) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    if new_about.is_empty() {
        return Ok(());
    }

    Ok(User::set_banner_about(auth.id, new_about, &pool).await?)
}

#[server(EditUserData)]
pub async fn edit_user_data(new_name: String, new_about: String) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    if new_name.is_empty() {
        return Ok(());
    }
    User::set_profile_name(auth.id, new_name, &pool).await?;

    if new_about.is_empty() {
        return Ok(());
    }
    User::set_banner_about(auth.id, new_about, &pool).await?;
    Ok(())
}

#[server(name = EditImageBanner, prefix = "/api", input = MultipartFormData)]
pub async fn edit_image_banner(data: MultipartData) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;
    let mut data = data.into_inner().unwrap();

    while let Ok(Some(field)) = data.next_field().await {
        if field.name() == Some("banner_image") {
            let name = field.file_name().map(|name| name.to_string());
            let file_type = field.content_type().map(|name| name.to_string());
            let chunks = field
                .try_collect::<Vec<MulterBytes>>()
                .await
                .map(|bytes| bytes.concat())
                .or(Err(ServerFnError::new("Something go wrong in our servers")))?;
            if let (Some(name), Some(file_type)) = (name, file_type) {
                let uploadthing = use_context::<UploadThing>().expect("acces to upload thing");
                if chunks.is_empty() {
                    return Err(ServerFnError::new(
                        "Something go wrong, the chunks are empty",
                    ));
                }
                let size = chunks.len();

                if let Ok(res) = uploadthing
                    .upload_file(
                        chunks,
                        FileData {
                            name,
                            file_type,
                            size,
                        },
                        true,
                    )
                    .await
                {
                    if let Some(current_image_key) =
                        User::get_banner_image_key(auth.id, &pool).await?
                    {
                        println!("deleting the file with key: {}", current_image_key);
                        uploadthing
                            .delete_files(vec![current_image_key])
                            .await
                            .map_err(|_| {
                                ServerFnError::new("We have problems deleting your file")
                            })?;
                    }
                    return Ok(User::set_image_banner_url(res.url, res.key, auth.id, &pool).await?);
                }
            }
        }
    }
    Err(ServerFnError::new(
        "Something go wrong when uploading the file",
    ))
}

#[server(name = EditUserImage, prefix = "/api", input = MultipartFormData)]
pub async fn edit_profile_image(data: MultipartData) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;
    let mut data = data.into_inner().unwrap();

    while let Ok(Some(field)) = data.next_field().await {
        if field.name() == Some("user_image") {
            let name = field.file_name().map(|name| name.to_string());
            let file_type = field.content_type().map(|name| name.to_string());
            let chunks = field
                .try_collect::<Vec<MulterBytes>>()
                .await
                .map(|bytes| bytes.concat())
                .or(Err(ServerFnError::new("Something go wrong in our servers")))?;
            if let (Some(name), Some(file_type)) = (name, file_type) {
                let uploadthing = use_context::<UploadThing>().expect("acces to upload thing");
                if chunks.is_empty() {
                    return Err(ServerFnError::new(
                        "Something go wrong, the chunks are empty",
                    ));
                }
                let size = chunks.len();

                if let Ok(res) = uploadthing
                    .upload_file(
                        chunks,
                        FileData {
                            name,
                            file_type,
                            size,
                        },
                        true,
                    )
                    .await
                {
                    if let Some(current_image_key) =
                        User::get_profile_image_key(auth.id, &pool).await?
                    {
                        println!("deleting the file with key: {}", current_image_key);
                        uploadthing
                            .delete_files(vec![current_image_key])
                            .await
                            .map_err(|_| {
                                ServerFnError::new("We have problems deleting your file")
                            })?;
                    }
                    return Ok(
                        User::set_image_profile_url(res.url, res.key, auth.id, &pool).await?,
                    );
                }
            }
        }
    }
    Err(ServerFnError::new(
        "Something go wrong when uploading the file",
    ))
}

pub fn use_user() -> UserContext {
    use_context::<UserContext>()
        .expect("should return the user context, check if you really provided the context")
}

#[server(GetUserProfile)]
pub async fn get_user_profile(user_id: Uuid) -> Result<Profile, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_profile(user_id, &pool).await?)
}

#[server(GetMutualServers)]
pub async fn get_mutual_servers_image_url(
    user_id: Uuid,
) -> Result<Vec<Option<String>>, ServerFnError> {
    let pool = pool()?;
    let user1 = auth_user()?;

    let user2 = User::get(user_id, &pool).await?;

    let res = Server::get_mutual_servers_image_url(user1.id, user2.id, &pool).await;
    Ok(res?)
}

#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;
    Ok(auth.current_user)
}

#[server(GetUserBanner)]
pub async fn get_user_banner(user_id: Uuid) -> Result<Banner, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(User::get_banner(user_id, &pool).await?)
}
