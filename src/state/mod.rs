use crate::uploadthing::UploadThing;
// use crate::ws::WsChannels;
use leptos::config::LeptosOptions;
use leptos_axum::AxumRouteListing;
use sqlx::MySqlPool;

#[derive(axum::extract::FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: MySqlPool,
    // pub ws_channels: WsChannels,
    pub uploadthing: UploadThing,
    pub routes: Vec<AxumRouteListing>,
}
