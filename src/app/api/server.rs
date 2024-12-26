use crate::entities::role::Role;
use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
use web_sys::FormData;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::entities::member::Member;
        use multer::bytes::Bytes as MulterBytes;
        use futures::TryStreamExt;
        use crate::uploadthing::UploadThing;
        use crate::uploadthing::upload_file::FileData;
        use crate::entities::{category::Category, channel::Channel};
        use leptos_axum::redirect;
        use http::uri::Scheme;
        use http::Uri;
        use super::auth_user;
        use super::pool;

    }
}

#[derive(Clone, Copy)]
pub struct ServerContext {
    pub join_with_invitation: Action<JoinServerWithInvitation, Result<(), ServerFnError>>,
    pub create_server: Action<FormData, Result<String, ServerFnError>>,
    pub leave_server: Action<LeaveServer, Result<(), ServerFnError>>,
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
    let join_with_invitation = create_server_action::<JoinServerWithInvitation>();
    let create_server = create_action(|data: &FormData| {
        let data = data.clone();
        create_server(data.into())
    });
    let leave_server = create_server_action::<LeaveServer>();
    // let create_category = create_server_action::<CreateCategory>();
    // let rename_category = create_server_action::<RenameCategory>();
    // let delete_category = create_server_action::<DeleteCategory>();
    //NOTE: agregar mas razones de cambio para los resources, leave_server, server_settings...,
    //rename_member
    provide_context(ServerContext {
        leave_server,
        join_with_invitation,
        create_server,
    })
}

pub fn use_server() -> ServerContext {
    use_context::<ServerContext>().expect("have server context")
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

    Ok(Server::get_user_servers(user.id, &pool).await?)
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
    let invitation = validate_invitation(invitation)
        .ok_or_else(|| ServerFnError::new("Your invitation is invalid"))?;
    match Member::check_member_from_invitation(user.id, invitation, &pool).await {
        Ok(uuid) => redirect(&format!("/servers/{}", uuid)),
        Err(crate::entities::Error::NotFound) => {
            match Member::create_member_from_invitation(user.id, invitation, &pool).await {
                Ok(id) => redirect(&format!("/servers/{}", id)),
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
    let auth = auth_user()?;
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

    let server = Server::create(server_name, auth.id, &pool).await?;
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
    Member::create(auth.id, server, &pool).await?;
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
