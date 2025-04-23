use async_broadcast::{broadcast, Receiver, Sender};
use dashmap::DashMap;
use futures::lock::Mutex;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket as GlooWs;
use gloo_net::websocket::Message as GlooMsg;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::debug;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use crate::messages::{AppMessage, ClientMessage, Message, ServerMessage};

#[derive(PartialEq, Debug, Clone)]
pub enum WsState {
    Open,
    Closed,
    Stoped,
}

type Channels = Arc<DashMap<Uuid, (Sender<Message>, Arc<Mutex<Receiver<Message>>>)>>;

#[derive(Clone)]
pub struct WsContext {
    sender: Sender<WsMessage>,
    servers_channels: Channels,
    app_channel: (Sender<ClientMessage>, Arc<Mutex<Receiver<ClientMessage>>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsMessage {
    AppMessage(AppMessage),
    Close,
}

impl WsContext {
    pub fn send(&self, msg: AppMessage) {
        #[cfg(feature = "hydrate")]
        {
            let sb = self.sender.clone();
            spawn_local(async move {
                let _ = sb.broadcast(WsMessage::AppMessage(msg)).await;
            });
        }
    }

    pub fn sync_channels(&self, servers: Vec<Uuid>, user_id: Uuid) {
        debug!("Syncing channels for servers: {servers:?}");

        let new_servers: HashSet<_> = servers.iter().cloned().collect();
        let current_servers: HashSet<_> = self.servers_channels.iter().map(|e| *e.key()).collect();

        let mut subscribe_msgs = vec![];

        for server_id in current_servers.difference(&new_servers) {
            self.servers_channels.remove(server_id);
        }

        for server_id in new_servers.difference(&current_servers) {
            let (sender, receiver) = broadcast::<Message>(1000);
            self.servers_channels
                .insert(*server_id, (sender, Arc::new(Mutex::new(receiver))));
            subscribe_msgs.push(AppMessage::Subscribe {
                user_id,
                server_id: *server_id,
            });
        }

        if !subscribe_msgs.is_empty() {
            debug!("we sub to: {subscribe_msgs:?}");
            self.send(AppMessage::Batch(subscribe_msgs));
        }
    }

    pub fn on_server_msg(&self, server_id: Uuid, on_msg: impl Fn(Message) + Send + Sync + 'static) {
        #[cfg(feature = "hydrate")]
        {
            if let Some(broadcast) = self.servers_channels.get(&server_id) {
                let rx = broadcast.1.clone();
                spawn_local(async move {
                    while let Ok(msg) = rx.lock().await.recv().await {
                        on_msg(msg)
                    }
                });
            }
        }
    }

    pub fn on_app_msg(&self, on_msg: impl Fn(ClientMessage) + Send + Sync + 'static) {
        #[cfg(feature = "hydrate")]
        {
            let rx = self.app_channel.1.clone();
            spawn_local(async move {
                while let Ok(msg) = rx.lock().await.recv().await {
                    on_msg(msg)
                }
            });
        }
    }
}

pub fn use_ws() -> WsContext {
    use_context::<WsContext>().expect("should acces to the Ws context")
}

pub fn provide_ws_context() {
    let ws_state = RwSignal::new(WsState::Closed);
    let (sender_sb, sender_rb) = broadcast::<WsMessage>(1000);
    let servers_channels: Channels = Arc::new(DashMap::new());
    let (app_sender, app_receiver) = broadcast::<ClientMessage>(1000);
    let app_receiver = Arc::new(Mutex::new(app_receiver));

    let connect = StoredValue::new({
        let channels = servers_channels.clone();
        let app_sender = app_sender.clone();
        move || {
            let ws = match GlooWs::open("ws://localhost:3000/ws") {
                Ok(ws) => {
                    ws_state.set(WsState::Open);
                    ws
                }
                Err(err) => {
                    debug!("error: {:?}", err);
                    return;
                }
            };

            let (mut sender_ws, mut receiver_ws) = ws.split();

            let mut sender_rb = sender_rb.clone();

            spawn_local(async move {
                while let Some(message) = receiver_ws.next().await {
                    match message {
                        Ok(GlooMsg::Text(msg)) => {
                            debug!("received msg from ws: {msg:?}");
                            let message = serde_json::from_str::<ClientMessage>(&msg)
                                .expect("should receive an ClientMessage");
                            match message {
                                ClientMessage::ServerMessage(ServerMessage { server_id, msg }) => {
                                    if let Some(broadcast) = channels.get(&server_id) {
                                        let _ = broadcast.0.clone().broadcast(msg).await;
                                    } else {
                                        debug!("Got a msg to server_id: {}, but we don't have a broadcast for this id", server_id)
                                    }
                                }
                                msg => {
                                    let _ = app_sender.broadcast(msg).await;
                                }
                            }
                        }
                        Ok(_) => {
                            debug!("not impl yet")
                        }
                        Err(err) => {
                            debug!("message in the ws receiver: {err:?}");
                            break;
                        }
                    }
                }
            });

            spawn_local(async move {
                while let Ok(msg) = sender_rb.recv().await {
                    match msg {
                        WsMessage::Close => {
                            let _ = sender_ws.close().await;
                            ws_state.set(WsState::Closed);
                        }
                        WsMessage::AppMessage(msg) => {
                            if (sender_ws
                                .send(GlooMsg::Text(serde_json::to_string(&msg).unwrap()))
                                .await)
                                .is_err()
                            {
                                ws_state.set(WsState::Closed);
                                break;
                            }
                        }
                    }
                }
            });
        }
    });

    let sb_clean = sender_sb.clone();
    #[cfg(feature = "hydrate")]
    {
        on_cleanup(move || {
            let sb = sb_clean;
            spawn_local(async move {
                let _ = sb.broadcast(WsMessage::Close).await;
            });
        });
    }

    Effect::new(move |_| {
        if ws_state.get() == WsState::Closed {
            spawn_local(async move {
                connect.get_value()();
            });
        }
    });

    provide_context(WsContext {
        sender: sender_sb.clone(),
        servers_channels,
        app_channel: (app_sender, app_receiver),
    });
}
