use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::ws::WsChannels;
        use axum::extract::FromRef;
        use leptos::LeptosOptions;
        use sqlx::MySqlPool;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: MySqlPool,
            pub ws_channels: WsChannels
        }
    }
}
