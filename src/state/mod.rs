use crate::sync::connections::{ConnectionMessage, UserConnections};
use crate::sync::SyncRequest;
use crate::uploadthing::server::UploadThing;
use async_broadcast::Sender;
use leptos::config::LeptosOptions;
use leptos_axum::AxumRouteListing;
use sqlx::MySqlPool;

#[derive(axum::extract::FromRef, Debug, Clone)]
pub struct AppState {
    pub connection_sender: Sender<ConnectionMessage>,
    pub sync_sender: Sender<SyncRequest>,
    pub leptos_options: LeptosOptions,
    pub pool: MySqlPool,
    pub user_connections: UserConnections,
    pub uploadthing: UploadThing,
    pub routes: Vec<AxumRouteListing>,
}
