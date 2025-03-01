use gloo_net::websocket::Message;
use leptos::prelude::*;
use leptos::task::spawn_local;
use async_broadcast::broadcast;
use gloo_net::websocket::futures::WebSocket as GlooWs;
use log::debug;
use uuid::Uuid;
use futures::{SinkExt, StreamExt};

use crate::messages::Messages;

#[derive(PartialEq, Debug, Clone)]
pub enum WsState {
    Open,
    Closed,
}

#[derive(Clone, Debug)]
pub struct WsContext {
    pub send_message: Callback<(Messages,)>,
    // pub messages: RwSignal<Vec<String>>,
}

pub fn use_ws_context() -> WsContext {
    use_context::<WsContext>().expect("should acces to the Ws context")
}

pub fn provide_ws_context(user_id: Uuid) {
    let ws_state = RwSignal::new(WsState::Closed);
    let (sb, rb) = broadcast::<String>(1000);

    let connect = StoredValue::new({
        let sb = sb.clone();

        move || {
            let ws = match GlooWs::open(&format!("ws://localhost:3000/ws/{}", user_id)) {
                Ok(ws) => ws,
                Err(_) => return,
            };

            ws_state.set(WsState::Open);

            let (mut ws_s, mut ws_r) = ws.split();

            let mut rb = rb.clone();

            spawn_local(async move {
                while let Some(message) = ws_r.next().await {
                    match message {
                        Ok(Message::Text(msg)) => {
                            debug!("got a message from the user: {}", msg)
                        },
                        Ok(_) => {
                            debug!("not impl yet")
                        },
                        Err(_) => {
                            ws_state.set(WsState::Closed);
                            break;
                        }
                    }
                }
            });

            spawn_local(async move {
                while let Ok(message) = rb.recv().await {
                    let _ = ws_s.send(Message::Text(serde_json::to_string(&message).expect("parse the WsMessage to string"))).await;
                }
            });
        }

    });

    Effect::new(move |_| {
        if ws_state.get() == WsState::Closed {
            spawn_local(async move {
                    connect.get_value()();
            });
        }

    });
    //impl the on_cleanup 
    //should send a close message from the broadcast to the ws

    provide_context(WsContext {
        send_message: Callback::from({
            move |msg: Messages| {
                if ws_state.get() == WsState::Open {
                    let sb = sb.clone();
                    spawn_local(async move {
                        let _ = sb.broadcast(serde_json::to_string(&msg).unwrap()).await;
                    });
                }
            }
        }),
        // messages,
    });
}

