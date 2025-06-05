use async_broadcast::{broadcast, Receiver, Sender};
use dashmap::DashMap;
use futures::channel::mpsc;
use futures::StreamExt as _;
use leptos::task::spawn_local_scoped_with_cancellation;
use log::{debug, error};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

use crate::sync::Mutation;

type Channels = Arc<DashMap<String, (Sender<Value>, Receiver<Value>)>>;

const BROADCAST_CHANNEL_CAPACITY: usize = 1024;

#[derive(Debug, Clone)]
pub struct SyncChannels {
    channels: Channels,
}

impl SyncChannels {
    pub fn new() -> Self {
        SyncChannels {
            channels: Default::default(),
        }
    }

    pub async fn start(&self, receiver: mpsc::Receiver<Mutation>) {
        debug!("SyncChannels: Starting message routing loop...");

        receiver
            .for_each_concurrent(None, |mutation| async {
                let module = mutation.module;

                let (sender, _) = self
                    .channels
                    .entry(module.clone())
                    .or_insert_with(|| {
                        debug!(
                            "SyncChannels: Creating new broadcast channel for module '{module}'"
                        );
                        let (s, r) = broadcast(BROADCAST_CHANNEL_CAPACITY);
                        (s, r)
                    })
                    .value()
                    .clone();

                if let Err(e) = sender.broadcast(mutation.data).await {
                    error!("SyncChannels: Failed to broadcast message for module '{module}': {e}");
                } else {
                    debug!("SyncChannels: Message broadcasted for module '{module}'");
                }
            })
            .await;

        debug!("SyncChannels: Message routing loop stopped.");
    }

    pub fn subscribe(&self, module: &str) -> Receiver<Value> {
        self.channels
            .entry(module.to_string())
            .or_insert_with(|| {
                debug!("SyncChannels: Creating new broadcast channel for module '{module}'");
                let (s, r) = broadcast(BROADCAST_CHANNEL_CAPACITY);
                (s, r)
            })
            .value()
            .1
            .clone()
    }

    pub fn on_module_msg<T>(&self, module: &str, on_msg: impl Fn(T) + Send + Sync + 'static)
    where
        T: for<'de> Deserialize<'de> + Send + 'static,
    {
        #[cfg(feature = "hydrate")]
        {
            let mut rx = self.subscribe(module);
            let on_msg = Arc::new(on_msg);
            let key_clone = module.to_string();

            spawn_local_scoped_with_cancellation(async move {
                debug!("Started listener for key '{key_clone}'");
                while let Ok(msg_value) = rx.recv().await {
                    if let Ok(msg) = serde_json::from_value(msg_value) {
                        on_msg(msg);
                    }
                }
                debug!("Listener for key '{key_clone}' stopped.");
            });
        }
    }
}
