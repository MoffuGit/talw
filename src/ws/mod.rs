use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use log::{debug, error};

use crate::{
    entities::user::{AuthSession, User},
    state::AppState,
    sync::connections::{Connection, ConnectionMessage},
};

pub async fn ws_handler(
    auth_session: AuthSession,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    if let Some(user) = auth_session.current_user {
        return ws.on_upgrade(move |socket| handle_socket(socket, state, user));
    }
    (StatusCode::FORBIDDEN, "Unauthorized WebSocket connection").into_response()
}

async fn handle_socket(socket: WebSocket, state: AppState, user: User) {
    let (mut sender, mut receiver) = socket.split();

    let channels = state.user_connections.clone();

    let connection = state.connection_sender;

    let rx = {
        match channels.get(&user.id) {
            Some(channel) => channel.value().receiver(),
            None => {
                error!("The connection should exist by now, creating channel but its propably not up to date");
                let bad_connection = Connection::new();
                let receiver = bad_connection.receiver();
                channels.insert(user.id, bad_connection);
                receiver
            }
        }
    };

    let _ = connection
        .broadcast(ConnectionMessage::CompleteConnection { client: user.id })
        .await;

    let sync = state.sync_sender;

    let mut rx_clone = rx.clone();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx_clone.recv().await {
            if sender
                .send(WsMessage::Text(serde_json::to_string(&msg).unwrap().into()))
                .await
                .is_err()
            {
                debug!("we got an error when sending the message");
                break;
            }
        }
    });

    let mut recv_task: tokio::task::JoinHandle<Result<(), anyhow::Error>> =
        tokio::spawn(async move {
            let sync = sync.clone();
            while let Some(Ok(WsMessage::Text(msg))) = receiver.next().await {
                // if let Ok(msg) = serde_json::from_str(&msg) {
                //     debug!("got this msg from the client: {msg:?}");
                //     // let _ = sync.broadcast(msg).await;
                // } else {
                //     debug!("we got a msg but cant deserialize");
                // }
            }
            Ok(())
        });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    };
}
