use async_broadcast::Receiver;
use log::{debug, info, warn};
use serde_json::{json, Value};
use tokio::spawn;

use crate::sync::{Mutation, SubscriptionMode};

use super::connections::UserConnections;
use super::subs::SubscriptionManager;
use super::SyncRequest;

#[derive(Debug, Clone)]
pub struct SyncRouter {
    subcriptions: SubscriptionManager,
    user_connections: UserConnections,
}

impl SyncRouter {
    pub fn new(subcriptions: SubscriptionManager, connections: UserConnections) -> Self {
        Self {
            subcriptions,
            user_connections: connections,
        }
    }

    pub async fn send_mutation(&self, key: String, data: Value) {
        info!("SyncRouter: Started forwarding for key '{key}'");
        let client_subscriptions = self.subcriptions.clone();
        let user_connections = self.user_connections.clone();
        if let Some(subscribed_clients_entry) = client_subscriptions.get_subscriptors(&key) {
            for user_id in subscribed_clients_entry.iter() {
                let data = data.clone();
                if let Some(module) = key.split(':').next() {
                    let module = module.to_string();
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
            }
        };
    }

    pub async fn start(&self, mut receiver: Receiver<SyncRequest>) {
        info!("SyncRouter: Starting message routing loop...");

        while let Ok(receiver) = receiver.recv().await {
            let router = self.clone();
            spawn(async move {
                match receiver {
                    SyncRequest::Mutation { key, data } => {
                        router.send_mutation(key, data).await;
                    }
                    SyncRequest::Subscription {
                        keys,
                        client,
                        action,
                    } => match action {
                        SubscriptionMode::Add => {
                            router.subcriptions.subscribe(keys, client);
                        }
                        SubscriptionMode::ReplacePrefix(prefix) => {
                            router.subcriptions.unsubscribe_group(&prefix, &client);
                            router.subcriptions.subscribe(keys, client);
                        }
                    },
                    SyncRequest::Unsubscription {
                        keys,
                        client,
                        prefix,
                    } => {
                        router.subcriptions.unsubscribe(keys, &client);
                        if let Some(prefix) = prefix {
                            router.subcriptions.unsubscribe_group(&prefix, &client);
                        }
                    }
                }
            });
        }
        info!("SyncRouter: Message routing loop stopped.");
    }
}
