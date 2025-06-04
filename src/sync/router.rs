use std::sync::Arc;

use dashmap::{DashMap, DashSet};
use futures::StreamExt as _;
use log::{debug, error, info, warn};
use serde_json::json;
use tokio::spawn;

use crate::sync::{
    Mutation, MutationRequest, SubscriptionMode, SubscriptionRequest, UnsubscriptionRequest,
};

const INTERNAL_BROADCAST_CHANNEL_CAPACITY: usize = 1024;

use async_broadcast::{InactiveReceiver, Receiver, Sender};

pub type SyncChannels =
    Arc<DashMap<String, (Sender<MutationRequest>, InactiveReceiver<MutationRequest>)>>;

use super::connections::UserConnections;
use super::subs::SubscriptionManager;
use super::SyncRequest;

type WorkerList = Arc<DashSet<String>>;

#[derive(Debug, Clone)]
pub struct SyncRouter {
    channels: SyncChannels,
    subcriptions: SubscriptionManager,
    workers: WorkerList,
    user_connections: UserConnections,
}

impl SyncRouter {
    pub fn new(subcriptions: SubscriptionManager, connections: UserConnections) -> Self {
        Self {
            channels: Arc::new(DashMap::new()),
            subcriptions,
            user_connections: connections,
            workers: Arc::new(DashSet::new()),
        }
    }

    fn spawn_worker(&self, key: String) {
        if let Some(mut receiver) = self
            .channels
            .get(&key)
            .map(|e| e.value().1.clone().activate())
        {
            self.workers.insert(key.clone());
            let client_subscriptions = self.subcriptions.clone();
            let user_connections = self.user_connections.clone();
            spawn(async move {
                info!("SyncRouter: Started forwarding worker for key '{key}'");
                while let Ok(MutationRequest { key, module, data }) = receiver.recv().await {
                    if let Some(subscribed_clients_entry) =
                        client_subscriptions.get_subscriptors(&key.clone())
                    {
                        for user_id in subscribed_clients_entry.iter() {
                            let data = data.clone();
                            let module = module.clone();
                            if let Some(connection) = user_connections
                                .get(user_id)
                                .map(|entry| entry.value().clone())
                            {
                                if let Err(e) = connection
                                    .sender()
                                    .broadcast(json!(Mutation { module, data }))
                                    .await
                                {
                                    log::error!(
                                        "SyncRouter: Failed to send message to client '{user_id}' for key '{key}': {e}"
                                    );
                                } else {
                                    debug!(
                                        "SyncRouter: Sent message to client '{user_id}' for key '{key}'"
                                    );
                                }
                            } else {
                                warn!(
                                    "SyncRouter: Client '{user_id}' subscribed to '{key}' \
                                     but no active WebSocket connection found. Removing subscriptions."
                                );
                                client_subscriptions.clear_subscriptions(user_id);
                            }
                        }
                    } else {
                        debug!("SubscriptionManager: No clients subscribed to key '{key}'. Message not forwarded to clients.");
                    }
                }
                info!("SyncRouter: Worker for key '{key}' exited.");
            });
        }
    }

    pub async fn start(&self, receiver: Receiver<SyncRequest>) {
        info!("SyncRouter: Starting message routing loop...");
        receiver
            .for_each_concurrent(None, |message| async move {
                let channels = self.channels.clone();
                let workers = self.workers.clone();
                match &message {
                    SyncRequest::Mutation(mutation) => {
                        let (sender, _) = channels
                            .entry(mutation.key.clone())
                            .or_insert_with(|| {
                                info!("SyncRouter: Creating channel for key '{}'", mutation.key);
                                let (sender, receiver) =
                                    async_broadcast::broadcast(INTERNAL_BROADCAST_CHANNEL_CAPACITY);
                                (sender, receiver.deactivate())
                            })
                            .clone();

                        if !workers.contains(&mutation.key) {
                            self.spawn_worker(mutation.key.clone());
                        }

                        if let Err(e) = sender.broadcast(mutation.clone()).await {
                            error!(
                                "SyncRouter: Failed to broadcast message for key '{}': {e}",
                                mutation.key
                            );
                        }
                    }
                    SyncRequest::Subscription(SubscriptionRequest {
                        keys,
                        client,
                        action,
                    }) => match action {
                        SubscriptionMode::Add => {
                            for key in keys {
                                debug!("Subscribing: {client} to {key}");
                                self.subcriptions.subscribe(key, *client);
                                if !workers.contains(key) {
                                    self.spawn_worker(key.clone());
                                }
                            }
                        }
                        SubscriptionMode::ReplaceAll => {
                            self.subcriptions.clear_subscriptions(client);
                            for key in keys {
                                debug!("Subscribing: {client} to {key}");
                                self.subcriptions.subscribe(key, *client);
                                if !workers.contains(key) {
                                    self.spawn_worker(key.clone());
                                }
                            }
                        }
                        SubscriptionMode::ReplaceByPrefix(prefix) => {
                            self.subcriptions.clear_prefix(client, prefix);
                            for key in keys {
                                debug!("Subscribing: {client} to {key}");
                                self.subcriptions.subscribe(key, *client);
                                if !workers.contains(key) {
                                    self.spawn_worker(key.clone());
                                }
                            }
                        }
                    },
                    SyncRequest::Unsubscription(UnsubscriptionRequest {
                        keys,
                        prefix,
                        client,
                    }) => {
                        for key in keys {
                            debug!("Unsubscribing: {client} to {key}");
                            self.subcriptions.unsubscribe(key, client);
                        }
                        if let Some(prefix) = prefix {
                            self.subcriptions.clear_prefix(client, prefix);
                        }
                    }
                }
            })
            .await;
        info!("SyncRouter: Message routing loop stopped.");
    }
}
