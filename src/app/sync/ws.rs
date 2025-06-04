use async_broadcast::{broadcast, Receiver, Sender};
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use log::debug;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use crate::sync::Mutation;

#[derive(PartialEq, Debug, Clone)]
pub enum WsState {
    Open,
    Closed,
    Stopped,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsMessage {
    Message(Value),
    Close,
    Stop,
}

pub struct Ws {
    ws_sender: Sender<WsMessage>,
    ws_receiver: Receiver<WsMessage>, // Associated receiver for the above sender
    channel_sender: futures::channel::mpsc::Sender<Mutation>,
    ws_state: Arc<Mutex<WsState>>,
}

impl Ws {
    pub fn new(channel_sender: mpsc::Sender<Mutation>) -> Self {
        let (ws_sender, ws_receiver) = broadcast::<WsMessage>(1000); // For messages FROM app TO WebSocket

        Ws {
            ws_sender,
            ws_receiver,
            channel_sender,
            ws_state: Arc::new(Mutex::new(WsState::Closed)),
        }
    }

    // You might want to expose the sender for the app to send messages
    pub fn get_ws_sender(&self) -> Sender<WsMessage> {
        self.ws_sender.clone()
    }

    pub async fn run(self) {
        const MAX_RECONNECT_TRIES: u32 = 5;
        const INITIAL_RECONNECT_DELAY_MS: u32 = 1000; // 1 second
        const MAX_RECONNECT_DELAY_MS: u32 = 8000; // Cap delay at 8 seconds

        let mut reconnect_tries: u32 = 0;
        let mut reconnect_delay_ms: u32 = INITIAL_RECONNECT_DELAY_MS;

        // Clone the sender to allow the main loop to send a `Stop` message if needed
        let ws_internal_sender = self.ws_sender.clone();

        loop {
            // Read the current state using the mutex
            let current_ws_state = {
                let state_lock = self.ws_state.lock().unwrap();
                state_lock.clone()
            };

            match current_ws_state {
                WsState::Closed => {
                    if reconnect_tries >= MAX_RECONNECT_TRIES {
                        debug!(
                            "WS: Max reconnect tries ({MAX_RECONNECT_TRIES}) reached. Stopping."
                        );
                        // Explicitly send a stop message to self to break the loop
                        let _ = ws_internal_sender.broadcast(WsMessage::Stop).await;
                        // Update state to stopped to ensure loop termination
                        *self.ws_state.lock().unwrap() = WsState::Stopped;
                        continue; // Go back to loop start and hit WsState::Stopped case
                    }

                    reconnect_tries += 1;

                    debug!(
                        "WS: Attempting to connect (try {reconnect_tries}/{MAX_RECONNECT_TRIES}) after {reconnect_delay_ms}ms delay..."
                    );

                    TimeoutFuture::new(reconnect_delay_ms).await; // Wait before retrying

                    match WebSocket::open("ws://localhost:3000/ws") {
                        Ok(ws) => {
                            // Reset reconnect attempts and delay on successful connection
                            reconnect_tries = 0;
                            reconnect_delay_ms = INITIAL_RECONNECT_DELAY_MS;

                            // Update state to Open
                            *self.ws_state.lock().unwrap() = WsState::Open;
                            debug!("WS: Connection opened.");

                            let (mut ws_sink, mut ws_stream) = ws.split();

                            // Clone necessary parts for the spawned tasks
                            let mut ws_local_sender_rb = self.ws_receiver.clone();
                            let mut app_router_tx_clone = self.channel_sender.clone();
                            let ws_state_clone_for_tasks = self.ws_state.clone(); // Arc clone

                            // Task for sending messages FROM app TO WebSocket
                            spawn_local(async move {
                                debug!("WS Send Task: Started");
                                while let Ok(msg) = ws_local_sender_rb.recv().await {
                                    match msg {
                                        WsMessage::Close => {
                                            debug!("WS Send Task: Received Close signal.");
                                            // Attempt to close the WebSocket cleanly
                                            let _ = ws_sink.close().await;
                                            // Signal main loop to reconnect
                                            *ws_state_clone_for_tasks.lock().unwrap() =
                                                WsState::Closed;
                                            break; // Exit send task
                                        }
                                        WsMessage::Stop => {
                                            debug!(
                                                "WS Send Task: Received Stop signal. Closing WS."
                                            );
                                            let _ = ws_sink.close().await;
                                            *ws_state_clone_for_tasks.lock().unwrap() =
                                                WsState::Stopped;
                                            break; // Exit send task
                                        }
                                        WsMessage::Message(json_value) => {
                                            let json_string =
                                                serde_json::to_string(&json_value).unwrap_or_else(|e| {
                                                    debug!(
                                                        "WS Send Task: Failed to serialize outgoing JSON: {e}"
                                                    );
                                                    "{}".to_string() // Send empty JSON or handle error
                                                });
                                            if (ws_sink.send(Message::Text(json_string)).await)
                                                .is_err()
                                            {
                                                debug!(
                                                    "WS Send Task: Failed to send message, connection likely closed."
                                                );
                                                // Signal main loop to reconnect
                                                *ws_state_clone_for_tasks.lock().unwrap() =
                                                    WsState::Closed;
                                                break; // Exit send task
                                            }
                                        }
                                    }
                                }
                                debug!("WS Send Task: Stopped.");
                            });

                            let ws_state_clone_for_tasks = self.ws_state.clone(); // Arc clone
                                                                                  // Task for receiving messages FROM WebSocket
                            spawn_local(async move {
                                debug!("WS Receive Task: Started");
                                while let Some(message) = ws_stream.next().await {
                                    match message {
                                        Ok(Message::Text(msg)) => {
                                            let parsed_value: Mutation =
                                                match serde_json::from_str(&msg) {
                                                    Ok(v) => v,
                                                    Err(e) => {
                                                        debug!("WS Receive Task: Failed to parse incoming WS message as JSON: {e}. Message: {msg}");
                                                        continue; // Skip if not valid JSON
                                                    }
                                                };
                                            // All incoming JSON messages go directly to the AppMessageRouter
                                            if let Err(e) = app_router_tx_clone.send(parsed_value).await
                                            {
                                                debug!("WS Receive Task: Failed to send message to AppRouter: {e}");
                                                // If we can't send to the app router, something is wrong,
                                                // potentially close the connection and try to reconnect.
                                                *ws_state_clone_for_tasks.lock().unwrap() =
                                                    WsState::Closed;
                                                break; // Exit receive task
                                            } else {
                                                debug!("WS Receive Task: Routed generic message to AppRouter");
                                            }
                                        }
                                        Ok(_) => debug!(
                                            "WS Receive Task: Received non-text message, not implemented yet"
                                        ),
                                        Err(err) => {
                                            debug!("WS Receive Task: Error in receiver: {err:?}");
                                            // Set state to closed on receive error
                                            *ws_state_clone_for_tasks.lock().unwrap() =
                                                WsState::Closed;
                                            break; // Exit receive task on error
                                        }
                                    }
                                }
                                debug!("WS Receive Task: Stopped.");
                            });
                        }
                        Err(err) => {
                            debug!("WS: Connection error: {err:?}");
                            reconnect_tries += 1;
                            // Exponential back-off with a cap
                            reconnect_delay_ms =
                                (reconnect_delay_ms * 2).min(MAX_RECONNECT_DELAY_MS);
                            // Ensure state is closed on error
                            *self.ws_state.lock().unwrap() = WsState::Closed;
                        }
                    }
                }
                WsState::Open => {
                    // If we're open, just briefly wait before re-checking the state.
                    // The actual connection management is handled by the spawned tasks.
                    TimeoutFuture::new(1_000).await; // Shorter timeout for quicker state checks
                }
                WsState::Stopped => {
                    debug!("WS: Run loop received Stop signal. Exiting.");
                    break; // Exit the main run loop
                }
            }
        }
        debug!("WS: Main run loop stopped.");
    }
}
