use crate::entities::server::Server;
use cfg_if::cfg_if;
use leptos::*;
use strum_macros::{Display, EnumIter};
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use http::uri::Scheme;
        use http::Uri;
        use uuid::Uuid;
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
    pub servers: Resource<usize, Result<Vec<Server>, ServerFnError>>,
    pub join_with_invitation: Action<JoinServerWithInvitation, Result<(), ServerFnError>>,
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
    let servers = create_resource(
        move || (join_with_invitation.version().get()),
        move |_| get_user_servers(),
    );
    provide_context(ServerContext {
        servers,
        join_with_invitation,
    })
}

pub fn use_server() -> ServerContext {
    use_context::<ServerContext>().expect("have server context")
}

pub fn user_servers() -> Resource<usize, Result<Vec<Server>, ServerFnError>> {
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
    // let user = auth_user()?;
    let invitation = validate_invitation(invitation)?;
    let server_id = Server::get_from_invitation(invitation, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("probablemente no este bien la invitacion".into())
        })?;
    //NOTE: hay que revisar si el usuario no esta unido ya a este server
    Ok(())
}

// #[server(CreateServer, "api")]
// pub async fn create_server(name: String) -> Result<(), ServerFnError> {
//     let pool = pool()?;
//     let auth = auth()?;
//
//     let server = Server::create(name, &pool);
//     return match auth.current_user {};
// }
