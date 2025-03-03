use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use async_broadcast::broadcast;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket as GlooWs;
use gloo_net::websocket::Message;
use gloo_timers::callback::Interval;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::debug;
use uuid::Uuid;

use crate::messages::Messages;

const CLOSE: &str = "CLOSE";

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

pub fn use_ws() -> WsContext {
    use_context::<WsContext>().expect("should acces to the Ws context")
}

pub fn provide_ws_context() {
    let ws_state = RwSignal::new(WsState::Closed);
    let (sb, rb) = broadcast::<String>(1000);
    let ws_signal = RwSignal::new_local(None::<GlooWs>);
    let attempts = StoredValue::new(0);

    let connect = StoredValue::new({
        let sb = sb.clone();

        move || {
            let ws = match GlooWs::open("ws://localhost:3000/ws") {
                Ok(ws) => ws,
                Err(_) => return,
            };

            ws_state.set(WsState::Open);
            attempts.set_value(0);

            let (mut ws_s, mut ws_r) = ws.split();

            let mut rb = rb.clone();

            spawn_local(async move {
                while let Some(message) = ws_r.next().await {
                    match message {
                        Ok(Message::Text(msg)) => {
                            debug!("got a message from the server: {}", msg)
                        }
                        Ok(_) => {
                            debug!("not impl yet")
                        }
                        Err(_) => {
                            ws_state.set(WsState::Closed);
                            break;
                        }
                    }
                }
            });

            spawn_local(async move {
                while let Ok(message) = rb.recv().await {
                    if &message == CLOSE {
                        ws_s.close().await;
                        rb.close();
                        ws_state.set(WsState::Closed);
                        break;
                    }
                    if let Err(_) = ws_s.send(Message::Text(message)).await {
                        ws_state.set(WsState::Closed);
                        break;
                    }
                }
            });
        }
    });
    let sb1 = sb.clone();

    on_cleanup(move || {
        let sb = sb1;
        spawn_local(async move {
            let _ = sb.broadcast(CLOSE.into()).await;
        });
    });

    Effect::new(move |_| {
        if ws_state.get() == WsState::Closed {
            while attempts.get_value() < 5 {
                spawn_local(async move {
                    connect.get_value()();
                });
                attempts.with_value(|count| count + 1);
            }
        }
    });

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
    });
}
