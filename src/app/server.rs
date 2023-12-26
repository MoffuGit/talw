use crate::entities::{channel::Channel, member::Member, server::Server};
use cfg_if::cfg_if;
use leptos::*;
use strum_macros::{Display, EnumIter};
use uuid::Uuid;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_axum::redirect;
        use http::uri::Scheme;
        use http::Uri;
        use super::auth::auth_user;
        use super::auth::pool;

        pub fn validate_invitation(invitation: String) -> Result<Uuid, ServerFnError> {
            match Uuid::parse_str(&invitation) {
                Ok(uuid) => Ok(uuid),
                Err(_) => {
                    let uri = invitation.parse::<Uri>()?;
                    if uri.host().is_some_and(|host| host == "discord.gg")
                        && uri.scheme().is_some_and(|scheme| scheme == &Scheme::HTTPS)
                    {
                        Uuid::parse_str(uri.path().split('/').last().ok_or_else(|| {
                            ServerFnError::ServerError("Error with invitation".to_string())
                        })?)
                        .map_err(|_| ServerFnError::ServerError("Error with the invitation".to_string()))
                    } else {
                        Err(ServerFnError::ServerError(
                            "Error with the invitation".to_string(),
                        ))
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ServerContext {
    pub servers: Resource<(usize, usize), Result<Vec<Server>, ServerFnError>>,
    pub join_with_invitation: Action<JoinServerWithInvitation, Result<(), ServerFnError>>,
    pub create_server: Action<CreateServer, Result<String, ServerFnError>>,
}

#[derive(Clone, Copy, EnumIter, Display)]
pub enum ServerTemplate {
    #[strum(serialize = "Create my own")]
    Default,
    #[strum(serialize = "Gaming")]
    Gaming,
}

pub fn provide_server_context() {
    let join_with_invitation = create_server_action::<JoinServerWithInvitation>();
    let create_server = create_server_action::<CreateServer>();
    let servers = create_resource(
        move || {
            (
                join_with_invitation.version().get(),
                create_server.version().get(),
            )
        },
        move |_| get_user_servers(),
    );
    provide_context(ServerContext {
        servers,
        join_with_invitation,
        create_server,
    })
}

pub fn use_server() -> ServerContext {
    use_context::<ServerContext>().expect("have server context")
}

pub fn user_servers() -> Resource<(usize, usize), Result<Vec<Server>, ServerFnError>> {
    use_server().servers
}

#[server(GetUserServers, "/api")]
pub async fn get_user_servers() -> Result<Vec<Server>, ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;

    user.get_servers(&pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("cant get servers".to_string()))
}

#[server(JoinServerWithInvitation, "/api")]
pub async fn join_server_with_invitation(invitation: String) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let user = auth_user()?;
    let invitation = validate_invitation(invitation)?;
    let existing_member = Server::check_member_from_invitation(user.id, invitation, &pool).await;
    match existing_member {
        Some(uuid) => redirect(&format!("/servers/{}", uuid)),
        None => {
            let server = Member::create_member_from_invitation(user.id, invitation, &pool)
                .await
                .ok_or_else(|| {
                    ServerFnError::ServerError("your invitation is incorrect".to_string())
                })?;
            redirect(&format!("/servers/{}", server))
        }
    };
    Ok(())
}

#[server(CreateServer, "/api")]
pub async fn create_server(name: String) -> Result<String, ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    let server = Server::create(name, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("Cant create server".to_string()))?;
    Member::create(crate::entities::member::Role::ADMIN, auth.id, server, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("Error".to_string()))?;
    //NOTE:
    //aqui crear dos canales por defecto
    redirect(&format!("/servers/{}", server));
    Ok(server.to_string())
}

#[server(CheckMember, "/api")]
pub async fn check_memeber(server_id: Uuid) -> Result<Server, ServerFnError> {
    let pool = pool()?;
    let auth = auth_user()?;

    let server = Server::check_member(server_id, auth.id, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("you cant acces here".to_string()))?;
    Ok(server)
}

#[server(GetChannels, "/api")]
pub async fn get_channels(server_id: Uuid) -> Result<Vec<Channel>, ServerFnError> {
    let _ = auth_user()?;
    let pool = pool()?;

    let channels = Server::get_channels(server_id, &pool)
        .await
        .ok_or_else(|| ServerFnError::ServerError("cant get channels server, sorry".to_string()))?;

    Ok(channels)
}
