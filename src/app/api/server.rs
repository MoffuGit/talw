use crate::entities::role::Role;
use crate::entities::server::Server;
use crate::messages::Message;
use crate::topic::Topic;
use cfg_if::cfg_if;
use leptos::prelude::*;
use log::debug;
use server_fn::codec::{MultipartData, MultipartFormData};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
use web_sys::FormData;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::str::FromStr;
        use crate::entities::member::Member;
        use multer::bytes::Bytes as MulterBytes;
        use futures::TryStreamExt;
        use crate::uploadthing::UploadThing;
        use crate::uploadthing::upload_file::FileData;
        use crate::entities::{category::Category, channel::Channel};
        use leptos_axum::redirect;
        use http::uri::Scheme;
        use http::Uri;
        use super::msg_sender;
        use super::auth_user;
        use super::pool;
    }
}

#[derive(Clone, Copy)]
pub struct ServerContext {
    pub join_with_invitation: ServerAction<JoinServerWithInvitation>,
    pub create_server: Action<FormData, Result<String, ServerFnError>, LocalStorage>,
    pub leave_server: ServerAction<LeaveServer>,
    pub edit_server_image: Action<FormData, Result<(), ServerFnError>, LocalStorage>,
    pub edit_server_name: ServerAction<EditServerName>,
}

#[derive(Clone, Copy, EnumIter, Display, PartialEq)]
pub enum ServerTemplate {
    #[strum(serialize = "Create my own")]
    Default,
    #[strum(serialize = "Gaming")]
    Gaming,
    #[strum(serialize = "School Club")]
    SchoolClub,
    #[strum(serialize = "Study Group")]
    StudyGroup,
    #[strum(serialize = "Friends")]
    Friends,
    #[strum(serialize = "Artist & Creators")]
    ArtistCreators,
    #[strum(serialize = "Local Community")]
    LocalCommunity,
}

pub fn provide_server_context() {
    let join_with_invitation = ServerAction::<JoinServerWithInvitation>::new();
    let create_server = Action::new_local(|data: &FormData| {
        let data = data.clone();
        create_server(data.into())
    });
    let leave_server = ServerAction::<LeaveServer>::new();
    let edit_server_image = Action::new_local(|data: &FormData| {
        let data = data.clone();
        edit_server_image(data.into())
    });
    let edit_server_name = ServerAction::<EditServerName>::new();
    provide_context(ServerContext {
        edit_server_name,
        edit_server_image,
        leave_server,
        join_with_invitation,
        create_server,
    })
}

pub fn use_server() -> ServerContext {
    use_context::<ServerContext>().expect("have server context")
}

#[server(name = EditServerImage, prefix = "/api", input = MultipartFormData)]
pub async fn edit_server_image(data: MultipartData) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    let mut data = data.into_inner().unwrap();
    let mut server_id: Option<Uuid> = None;
    let mut chunks: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut file_type: Option<String> = None;

    while let Ok(Some(mut field)) = data.next_field().await {
        match field.name().unwrap_or_default() {
            "server_id" => {
                if let Ok(Some(chunk)) = field.chunk().await {
                    if let Ok(id) = String::from_utf8(chunk.to_vec()) {
                        server_id = Uuid::from_str(&id).ok();
                    }
                }
            }
            "server_image" => {
                file_name = Some(field.file_name().expect("file name").to_string());
                file_type = Some(field.content_type().expect("mime type").to_string());
                chunks = Some(
                    field
                        .try_collect::<Vec<MulterBytes>>()
                        .await
                        .or(Err(ServerFnError::new("Something go wrong in our servers")))?
                        .concat(),
                );
            }
            field => {
                return Err(ServerFnError::new(format!(
                    "Field {field} not should exist"
                )))
            }
        }
    }
    if let (Some(file_type), Some(chunks), Some(file_name), Some(server_id)) =
        (file_type, chunks, file_name, server_id)
    {
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
                    name: file_name,
                    file_type,
                    size,
                },
                true,
            )
            .await
        {
            if let Some(current_image_key) = Server::get_server_image_key(server_id, &pool).await? {
                println!("deleting the file with key: {}", current_image_key);
                uploadthing
                    .delete_files(vec![current_image_key])
                    .await
                    .map_err(|_| ServerFnError::new("We have problems deleting your file"))?;
            }
            return Ok(Server::set_image_url(res.url, res.key, server_id, &pool).await?);
        }
    }

    Ok(())
}

#[server(EditServerName)]
pub async fn edit_server_name(new_name: String, server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Server::set_server_name(new_name, server_id, &pool).await?)
}

#[server(GetServerRoles)]
pub async fn get_server_roles(server_id: Uuid) -> Result<Vec<Role>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Role::get_server_roles(server_id, &pool).await?)
}

#[server(GetUserServers, "/api")]
pub async fn get_user_servers() -> Result<Vec<Server>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    let msg_sender = msg_sender()?;
    let servers = Server::get_user_servers(user.id, &pool).await?;

    msg_sender.send(Message::Batch(
        servers
            .iter()
            .map(|server| Message::UserConnected {
                user_id: user.id,
                server_id: server.id,
            })
            .collect::<Vec<_>>(),
    ));

    Ok(servers)
}

#[server(JoinServerWithInvitation, "/api")]
pub async fn join_server_with_invitation(invitation: String) -> Result<(), ServerFnError> {
    fn validate_invitation(invitation: String) -> Option<Uuid> {
        match Uuid::parse_str(&invitation) {
            Ok(uuid) => Some(uuid),
            Err(_) => invitation.parse::<Uri>().ok().and_then(|uri| {
                if uri.host().is_some_and(|host| host == "discord.gg")
                    && uri.scheme().is_some_and(|scheme| scheme == &Scheme::HTTPS)
                {
                    Uuid::parse_str(uri.path().split('/').last()?).ok()
                } else {
                    None
                }
            }),
        }
    }
    let pool = pool()?;
    let user = auth_user()?;
    let msg_sender = msg_sender()?;
    let invitation = validate_invitation(invitation)
        .ok_or_else(|| ServerFnError::new("Your invitation is invalid"))?;
    match Member::check_member_from_invitation(user.id, invitation, &pool).await {
        Ok(uuid) => redirect(&format!("/servers/{}", uuid)),
        Err(crate::entities::Error::NotFound) => {
            match Member::create_member_from_invitation(user.id, invitation, &pool).await {
                Ok(id) => {
                    msg_sender.send(crate::messages::Message::Subscribe {
                        topic: Topic::Server(id),
                        user_id: user.id,
                    });
                    msg_sender.send(crate::messages::Message::UserJoinedServer {
                        user_id: user.id,
                        server_id: id,
                    });
                    redirect(&format!("/servers/{}", id))
                }
                Err(crate::entities::Error::NotFound) => {
                    return Err(ServerFnError::new("Your invitation is invalid"))
                }
                Err(_) => {
                    return Err(ServerFnError::new("We can't to this"));
                }
            };
        }
        Err(_) => {
            return Err(ServerFnError::new("We can't to this"));
        }
    };
    Ok(())
}

#[server(name = CreateServer, prefix = "/api", input = MultipartFormData)]
pub async fn create_server(data: MultipartData) -> Result<String, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    let mut data = data.into_inner().unwrap();
    let mut server_name: String = Default::default();
    let mut chunks: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;
    let mut file_type: Option<String> = None;

    while let Ok(Some(mut field)) = data.next_field().await {
        match field.name().unwrap_or_default() {
            "name" => {
                if let Ok(Some(chunk)) = field.chunk().await {
                    if let Ok(name) = String::from_utf8(chunk.to_vec()) {
                        server_name = name
                    }
                }
            }
            "image" => {
                file_name = Some(field.file_name().expect("file name").to_string());
                file_type = Some(field.content_type().expect("mime type").to_string());
                chunks = Some(
                    field
                        .try_collect::<Vec<MulterBytes>>()
                        .await
                        .or(Err(ServerFnError::new("Something go wrong in our servers")))?
                        .concat(),
                );
            }
            field => {
                return Err(ServerFnError::new(format!(
                    "Field {field} not should exist"
                )))
            }
        }
    }

    if server_name.len() < 2 || server_name.len() > 100 {
        return Err(ServerFnError::new(
            "The server name should be between 2 and 100 in length",
        ));
    }

    let server = Server::create(server_name, user.id, &pool).await?;
    if let (Some(chunks), Some(file_name), Some(file_type)) = (chunks, file_name, file_type) {
        let uploadthing = use_context::<UploadThing>().expect("acces to upload thing");
        let size = chunks.len();
        if size > 0 {
            if let Ok(res) = uploadthing
                .upload_file(
                    chunks,
                    FileData {
                        name: file_name.to_string(),
                        file_type,
                        size,
                    },
                    true,
                )
                .await
            {
                Server::set_image_url(res.url, res.key, server, &pool).await?;
            }
        }
    }
    Member::create(user.id, server, &pool).await?;
    Channel::create(
        "general".to_string(),
        crate::entities::channel::ChannelType::TEXT,
        server,
        &pool,
    )
    .await?;
    let text_category = Category::create("text".to_string(), server, &pool).await?;
    Channel::create_with_category(
        "text".to_string(),
        crate::entities::channel::ChannelType::TEXT,
        server,
        text_category,
        &pool,
    )
    .await?;
    let voice_category = Category::create("voice".to_string(), server, &pool).await?;
    Channel::create_with_category(
        "voice".to_string(),
        crate::entities::channel::ChannelType::VOICE,
        server,
        voice_category,
        &pool,
    )
    .await?;
    let msg_sender = msg_sender()?;
    msg_sender.send(crate::messages::Message::Subscribe {
        topic: Topic::Server(server),
        user_id: user.id,
    });
    msg_sender.send(crate::messages::Message::UserJoinedServer {
        user_id: user.id,
        server_id: server,
    });
    redirect(&format!("/servers/{}", server.simple()));
    Ok(server.to_string())
}

#[server(CheckServer, "/api")]
pub async fn get_server(server_id: Uuid) -> Result<Server, ServerFnError> {
    let pool = pool()?;
    auth_user()?;

    Ok(Server::get_server(server_id, &pool).await?)
}

#[server(LeaveServer, "/api")]
pub async fn leave_server(server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;
    Member::delete_from_server(auth.id, server_id, &pool).await?;
    redirect("/servers/me");
    Ok(())
}
