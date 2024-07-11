use cfg_if::cfg_if;

use crate::uploadthing::UploadThing;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ws::WsChannels;
        use leptos::LeptosOptions;
        use sqlx::MySqlPool;

        #[derive(axum::extract::FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: MySqlPool,
            pub ws_channels: WsChannels,
            pub uploadthing: UploadThing
        }
    }
}
