use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;
use log::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct SubscriptionManager {
    subscriptions: Arc<dashmap::DashMap<String, HashSet<Uuid>>>,
    user_subscriptions: Arc<DashMap<Uuid, HashSet<String>>>,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        SubscriptionManager {
            subscriptions: Arc::new(DashMap::new()),
            user_subscriptions: Arc::new(DashMap::new()),
        }
    }

    pub fn get_subscriptors(
        &self,
        key: &str,
    ) -> std::option::Option<
        dashmap::mapref::one::Ref<'_, std::string::String, std::collections::HashSet<uuid::Uuid>>,
    > {
        self.subscriptions.get(key)
    }

    pub fn subscribe(&self, key: &str, client: Uuid) {
        self.subscriptions
            .entry(key.to_string())
            .or_default()
            .insert(client);
        self.user_subscriptions
            .entry(client)
            .or_default()
            .insert(key.to_string());
        info!("Client '{client}' subscribed to key '{key}'.");
    }

    pub fn clear_prefix(&self, client: &Uuid, prefix: &str) {
        if let Some(mut keys) = self.user_subscriptions.get_mut(client) {
            for key in keys.iter().filter(|key| key.starts_with(prefix)) {
                if let Some(mut entry) = self.subscriptions.get_mut(key) {
                    let removed = entry.remove(client);
                    if removed {
                        info!("Client '{client}' unsubscribed from key '{key}'.");
                    }
                    if entry.is_empty() {
                        drop(entry);
                        self.subscriptions.remove(key);
                        info!("Key '{key}' has no more client subscribers, removing entry.");
                    }
                }
            }
            keys.retain(|keys| !keys.contains(prefix));
        }
    }

    pub fn clear_subscriptions(&self, client: &Uuid) {
        if let Some(mut keys) = self.user_subscriptions.get_mut(client) {
            for key in keys.drain() {
                if let Some(mut entry) = self.subscriptions.get_mut(&key) {
                    let removed = entry.remove(client);
                    if removed {
                        info!("Client '{client}' unsubscribed from key '{key}'.");
                    }
                    if entry.is_empty() {
                        drop(entry);
                        self.subscriptions.remove(&key);
                        info!("Key '{key}' has no more client subscribers, removing entry.");
                    }
                }
            }
        }
    }

    pub fn unsubscribe(&self, key: &str, client: &Uuid) {
        if let Some(mut entry) = self.user_subscriptions.get_mut(client) {
            entry.remove(key);
        }
        if let Some(mut entry) = self.subscriptions.get_mut(key) {
            let removed = entry.remove(client);
            if removed {
                info!("Client '{client}' unsubscribed from key '{key}'.");
            }
            if entry.is_empty() {
                drop(entry);
                self.subscriptions.remove(key);
                info!("Key '{key}' has no more client subscribers, removing entry.");
            }
        }
    }

    pub fn get_client_subscriptions(
        &self,
        client: &Uuid,
    ) -> std::option::Option<
        dashmap::mapref::one::Ref<'_, uuid::Uuid, std::collections::HashSet<std::string::String>>,
    > {
        self.user_subscriptions.get(client)
    }
}
