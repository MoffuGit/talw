use async_broadcast::{broadcast, Receiver, Sender};
use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use http::StatusCode;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use zeromq::SocketSend;

use crate::{
    entities::user::{AuthSession, User},
    state::AppState,
};

pub type WsChannels = HashMap<Uuid, (Sender<String>, Receiver<String>)>;

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

async fn handle_socket(socket: WebSocket, mut state: AppState, user: User) {
    use crate::messages::Messages;

    let (mut sender, mut receiver) = socket.split();
    let publisher = state.msg_broker.publisher.clone();
    let (tx, mut rx) = {
        let mut channels = state.ws_channels;
        match channels.get(&user.id) {
            Some(s) => s.clone(),
            None => {
                let (tx, rx) = broadcast(1000);
                //the tx get used in the aggregation service for receiving data from the message
                //broker and send the important data into the rx sub
                channels.insert(user.id, (tx.clone(), rx.clone()));
                (tx, rx)
            }
        }
    };

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
                if let Ok(msg) = serde_json::from_str::<Messages>(&msg) {
                    match msg {
                        Messages::Subscribe { topic } => {
                            debug!("Subscriptions to topic: {:?}", topic);
                            state.subscriptions.subscribe(user.id, topic)
                        }
                        Messages::Unsubscribe { topic } => {
                            debug!("Unsubscriptions to topic: {:?}", topic);
                            state.subscriptions.unsubscribe(user.id, topic)
                        }
                        _ => debug!("not impl"), // Messages::Unsubscribe { topic } => todo!(),
                                                 // Messages::ChatMessage { sender_id, chat_id, content, timestamp } => todo!(),
                                                 // Messages::Typing { user_id, chat_id, is_typing } => todo!(),
                    }
                }
            }
            Ok(())
        });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    }
}
