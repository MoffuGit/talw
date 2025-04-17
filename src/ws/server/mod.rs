use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use log::debug;
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};
use uuid::Uuid;

use crate::{
    entities::user::{AuthSession, User},
    messages::Message,
    state::AppState,
};

pub type WsChannels = Arc<DashMap<Uuid, (Sender<Message>, Arc<Mutex<Receiver<Message>>>)>>;

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

    let channels = state.ws_channels.clone();

    let rx = {
        match channels.get(&user.id) {
            Some(channel) => channel.1.clone(),
            None => {
                let (tx, rx) = broadcast::channel::<Message>(1000);
                let rx = Arc::new(Mutex::new(rx));
                channels.insert(user.id, (tx.clone(), rx.clone()));
                rx
            }
        }
    };

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.lock().await.recv().await {
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
            let msg_sender = state.msg_sender;
            //if the receiver close connection send a UserDisconnectd to the msg_sender
            while let Some(Ok(WsMessage::Text(msg))) = receiver.next().await {
                if let Ok(msg) = serde_json::from_str::<Message>(&msg) {
                    msg_sender.send(msg);
                } else {
                    debug!("we got a msg but cant deserialize");
                }
            }

            msg_sender.send(Message::UserDisconnected { user_id: user.id });
            Ok(())
        });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    }
}
