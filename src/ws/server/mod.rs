use std::sync::Arc;
use zeromq::SocketSend;
use log::debug;
use async_broadcast::{broadcast, Sender, Receiver};
use futures::{SinkExt, StreamExt};
use uuid::Uuid;
use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use std::collections::HashMap;

use crate::state::AppState;

#[cfg(feature = "ssr")]
pub type WsChannels = HashMap<Uuid, (Sender<String>, Receiver<String>)>;

#[cfg(feature = "ssr")]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path(user): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, user))
}

#[cfg(feature = "ssr")]
async fn handle_socket(socket: WebSocket, state: AppState, user: Uuid) {

    let (mut sender, mut receiver) = socket.split();
    let publisher = state.msg_broker.publisher.clone();
    let (tx, mut rx) = {
        let mut channels = state.ws_channels;
        match channels.get(&user) {
            Some(s) => s.clone(),
            None => {
                let (tx, rx) = broadcast(1000);
                //the tx get used in the aggregation service for receiving data from the message
                //broker and send the important data into the rx sub
                channels.insert(user, (tx.clone(), rx.clone()));
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
        tokio::spawn(
            async move {
                while let Some(Ok(WsMessage::Text(msg))) = receiver.next().await {
                    let mut publisher = publisher.lock().await;
                    debug!("received ws message: {}", msg);
                    //send data to the message broker
                    match publisher.send(msg.into()).await  {
                        Ok(_) => debug!("the data got sended into the msg_broker"),
                        Err(_) => debug!("something go wrong when sending the data to the msg_broker"),
                    }
                }
                Ok(())
            }
        );

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    }
}
