use async_broadcast::Sender;
use futures::channel::mpsc;
use leptos::prelude::{on_cleanup, provide_context, use_context};
use leptos::task::spawn_local;
use log::debug;

use crate::sync::Mutation;

use self::subscriptions::SyncChannels;
use self::ws::{Ws, WsMessage};

mod subscriptions;
mod ws;

#[derive(Clone)]
pub struct SyncContext {
    pub ws_sender: Sender<WsMessage>,
    pub message_router: SyncChannels,
}

pub fn use_sync() -> Option<SyncContext> {
    use_context()
}

pub fn provide_sync_context() {
    #[cfg(feature = "hydrate")]
    {
        debug!("Providing sync context...");
        let (ws_to_router_tx, ws_to_router_rx) = mpsc::channel::<Mutation>(32);
        let ws_client = Ws::new(ws_to_router_tx);
        let ws_sender_for_app = ws_client.get_ws_sender();

        let message_router = SyncChannels::new();

        spawn_local(async move {
            ws_client.run().await;
            debug!("WebSocket client run loop exited.");
        });

        let msg_router_clone = message_router.clone();
        spawn_local(async move {
            msg_router_clone.start(ws_to_router_rx).await;
            debug!("MessageRouter processing loop exited.");
        });

        let ws_sender_clone = ws_sender_for_app.clone();
        on_cleanup(move || {
            spawn_local(async move {
                let _ = ws_sender_clone.broadcast(WsMessage::Close).await;
            });
        });
        provide_context(SyncContext {
            ws_sender: ws_sender_for_app,
            message_router,
        });

        debug!("Sync context provided.");
    }
}
