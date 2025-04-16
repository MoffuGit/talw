use crate::msg_sender::MsgSender;
use crate::uploadthing::UploadThing;
use crate::ws::server::WsChannels;
use leptos::config::LeptosOptions;
use leptos_axum::AxumRouteListing;
use sqlx::MySqlPool;

#[derive(axum::extract::FromRef, Debug, Clone)]
pub struct AppState {
    pub msg_sender: MsgSender,
    pub leptos_options: LeptosOptions,
    pub pool: MySqlPool,
    pub ws_channels: WsChannels,
    pub uploadthing: UploadThing,
    pub routes: Vec<AxumRouteListing>,
}
