use crate::entities::{category::Category, channel::Channel, member::Member, server::Server};
use crate::uploadthing::upload_file::FileData;
use crate::uploadthing::UploadThing;
use cfg_if::cfg_if;
use futures::TryStreamExt;
use leptos::*;
use multer::bytes::Bytes as MulterBytes;
use server_fn::codec::{MultipartData, MultipartFormData};
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
use web_sys::FormData;
cfg_if! {
    if #[cfg(feature = "ssr")] {
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

#[server(GetServerMembers)]
pub async fn get_server_members(server_id: Uuid) -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    auth_user()?;
    Server::get_server_members(server_id, &pool)
        .await
        .or(Err(ServerFnError::new("Something go wrong")))
}

#[server(GetUserServersWithMembers)]
pub async fn get_user_servers_with_members() -> Result<Vec<(Server, Member)>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    let servers = Server::get_user_servers(user.id, &pool)
        .await
        .or(Err(ServerFnError::new(
            "We can't get yours servers in this moment",
        )))?;
    let mut res = vec![];
    for server in servers {
        if let Ok(member) = Member::get_user_member(user.id, server.id, &pool).await {
            res.push((server, member.clone()))
        }
    }
    Ok(res)
}

#[server(GetUserServers, "/api")]
pub async fn get_user_servers() -> Result<Vec<Server>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Server::get_user_servers(user.id, &pool)
        .await
        .or(Err(ServerFnError::new(
            "We can't get yours servers in this moment",
        )))
}

#[server(GetUserMembers, "/api")]
pub async fn get_user_members() -> Result<Vec<Member>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    Member::get_user_members(user.id, &pool)
        .await
        .or(Err(ServerFnError::new(
            "cant get members from user".to_string(),
        )))
}

#[server(GetMember, "/api")]
pub async fn get_member(server_id: Uuid) -> Result<Member, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    match Member::get_member(server_id, user.id, &pool).await {
        Ok(member) => Ok(member),
        Err(sqlx::Error::RowNotFound) => Err(ServerFnError::new("Your user dont exist")),
        Err(_) => Err(ServerFnError::new("We cant get your member")),
    }
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
        Err(sqlx::Error::RowNotFound) => {
            match Member::create_member_from_invitation(user.id, invitation, user.username, &pool)
                .await
            {
                Ok(id) => redirect(&format!("/servers/{}", id)),
                Err(sqlx::Error::RowNotFound) => {
                    return Err(ServerFnError::new("Your invitation is invalid"))
                }
                Err(_) => return Err(ServerFnError::new("We can't to this")),
            };
        }
        Err(_) => return Err(ServerFnError::new("We can't to this")),
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

    let server = Server::create(server_name, &pool)
        .await
        .or(Err(ServerFnError::new("Cant create server".to_string())))?;
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
                Server::set_image_url(res.url, server, &pool)
                    .await
                    .or(Err(ServerFnError::new("Error".to_string())))?;
            }
        }
    }
    Member::create(
        crate::entities::member::Role::ADMIN,
        auth.id,
        server,
        auth.username,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new("Error".to_string())))?;
    Channel::create(
        "general".to_string(),
        crate::entities::channel::ChannelType::TEXT,
        server,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new(
        "cant create the channel for this server".to_string(),
    )))?;
    Channel::create(
        "announcement".to_string(),
        crate::entities::channel::ChannelType::ANNOUNCEMENTS,
        server,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new(
        "cant create the channel for this server".to_string(),
    )))?;
    Channel::create(
        "rules".to_string(),
        crate::entities::channel::ChannelType::RULES,
        server,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new(
        "cant create the channel for this server".to_string(),
    )))?;
    let text_category = Category::create("text".to_string(), server, &pool)
        .await
        .or(Err(ServerFnError::new(
            "cant create the category".to_string(),
        )))?;
    Channel::create_with_category(
        "text".to_string(),
        crate::entities::channel::ChannelType::TEXT,
        server,
        text_category,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new(
        "cant create the channel with category".to_string(),
    )))?;
    let voice_category = Category::create("voice".to_string(), server, &pool)
        .await
        .or(Err(ServerFnError::new(
            "cant create the category".to_string(),
        )))?;
    Channel::create_with_category(
        "voice".to_string(),
        crate::entities::channel::ChannelType::VOICE,
        server,
        voice_category,
        &pool,
    )
    .await
    .or(Err(ServerFnError::new(
        "cant create the channel with category".to_string(),
    )))?;
    redirect(&format!("/servers/{}", server.simple()));
    Ok(server.to_string())
}

#[server(CheckServer, "/api")]
pub async fn check_server(server_id: Uuid) -> Result<Server, ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    let server = Server::check_server(server_id, auth.id, &pool)
        .await
        .or(Err(ServerFnError::new("you cant acces here".to_string())))?;
    Ok(server)
}

#[server(LeaveServer, "/api")]
pub async fn leave_server(server_id: Uuid) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;
    Member::delete_from_server(auth.id, server_id, &pool)
        .await
        .or(Err(ServerFnError::new(
            "cant delte the member form the server",
        )))?;
    Ok(())
}
