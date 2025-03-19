use async_broadcast::{broadcast, Receiver, Sender};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket as GlooWs;
use gloo_net::websocket::Message as GlooMsg;
use gloo_timers::callback::Interval;
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::debug;
use std::marker::Send;

use crate::messages::Message;

const CLOSE: &str = "CLOSE";

#[derive(PartialEq, Debug, Clone)]
pub enum WsState {
    Open,
    Closed,
    Stoped,
}

#[derive(Clone)]
pub struct WsContext {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl WsContext {
    pub fn send(&self, msg: Message) {
        let sb = self.sender.clone();
        spawn_local(async move {
            let _ = sb.broadcast(msg).await;
        });
    }

    // pub fn on_msg(&self) {
    //     let rb = self.receiver.clone();
    //     spawn_local(async move {
    //         while let Ok(msg) = rb.recv().await {
    //
    //         }
    //     });
    // }
}

pub fn use_ws() -> WsContext {
    use_context::<WsContext>().expect("should acces to the Ws context")
}

pub fn provide_ws_context() {
    let ws_state = RwSignal::new(WsState::Closed);
    let (sender_sb, sender_rb) = broadcast::<Message>(1000);
    let (receiver_sb, receiver_rb) = broadcast::<Message>(1000);

    let connect = StoredValue::new({
        let sender_sb = sender_sb.clone();
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

            let sender_sb = sender_sb.clone();
            let mut sender_rb = sender_rb.clone();

            let receiver_sb = receiver_sb.clone();

            spawn_local(async move {
                while let Some(message) = receiver_ws.next().await {
                    match message {
                        Ok(GlooMsg::Text(msg)) => {
                            let msg = serde_json::from_str(&msg).unwrap();
                            debug!("we got this msg from the server: {:?}", msg);
                            receiver_sb.broadcast(msg).await;
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
                while let Ok(message) = sender_rb.recv().await {
                    if &message == &Message::Close {
                        sender_ws.close().await;
                        ws_state.set(WsState::Closed);
                    } else {
                        if let Err(_) = sender_ws
                            .send(GlooMsg::Text(serde_json::to_string(&message).unwrap()))
                            .await
                        {
                            ws_state.set(WsState::Closed);
                            break;
                        }
                    }
                }
            });
        }
    });

    let sb_clean = sender_sb.clone();
    on_cleanup(move || {
        let sb = sb_clean;
        spawn_local(async move {
            let _ = sb.broadcast(Message::Close).await;
        });
    });

    Effect::new(move |_| {
        if ws_state.get() == WsState::Closed {
            spawn_local(async move {
                connect.get_value()();
            });
        }
    });

    provide_context(WsContext {
        sender: sender_sb.clone(),
        receiver: receiver_rb.clone(),
    });
}
