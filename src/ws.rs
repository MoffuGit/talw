use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::{
            extract::{
                ws::{Message as WsMessage, WebSocket},
                Path, State, WebSocketUpgrade,
            },
            response::IntoResponse,
        };
        use tokio::sync::broadcast;
        use std::collections::HashMap;

        use futures::{SinkExt, StreamExt};

        use crate::state::AppState;
    }
}

#[cfg(feature = "ssr")]
pub type WsChannels = HashMap<usize, broadcast::Sender<String>>;

#[cfg(feature = "ssr")]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(channel): Path<usize>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, channel))
}

#[cfg(feature = "ssr")]
async fn handle_socket(socket: WebSocket, state: AppState, channel: usize) {
    let (mut sender, mut receiver) = socket.split();
    let tx = {
        let mut channels = state.ws_channels;
        match channels.get(&channel) {
            Some(s) => s.clone(),
            None => {
                let (tx, _) = broadcast::channel(1000);
                channels.insert(channel.clone(), tx.clone());
                tx
            }
        }
    };
    let mut rx = tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(WsMessage::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task: tokio::task::JoinHandle<Result<(), anyhow::Error>> =
        tokio::spawn(async move {
            while let Some(Ok(WsMessage::Text(msg))) = receiver.next().await {
                tx.send(msg)?;
            }
            Ok(())
        });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    }
}
