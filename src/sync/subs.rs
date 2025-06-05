use std::collections::HashSet;
use std::sync::Arc;

use dashmap::DashMap;
use log::info;
use uuid::Uuid;

type UserSubscriptions = Arc<DashMap<Uuid, HashSet<String>>>;

#[derive(Debug, Clone)]
pub struct SubscriptionManager {
    subscriptions: Arc<dashmap::DashMap<String, HashSet<Uuid>>>,
    user_subscriptions: UserSubscriptions,
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(DashMap::new()),
            user_subscriptions: Arc::new(DashMap::new()),
        }
    }

    pub fn get_subscriptors(
        &self,
        key: &str,
    ) -> Option<dashmap::mapref::one::Ref<'_, String, HashSet<Uuid>>> {
        self.subscriptions.get(key)
    }

    pub fn subscribe(&self, keys: Vec<String>, client: Uuid) {
        for key in keys {
            info!("Client '{client}' subscribed to key '{key}'.");
            self.subscriptions
                .entry(key.clone())
                .or_default()
                .insert(client);

            self.user_subscriptions
                .entry(client)
                .or_default()
                .insert(key);
        }
    }

    pub fn unsubscribe(&self, keys: Vec<String>, client: &Uuid) {
        for key in keys {
            if let Some(mut entry) = self.subscriptions.get_mut(&key) {
                let removed = entry.remove(client);
                if removed {
                    info!("Client '{client}' unsubscribed from key '{key}'.");
                }
                if entry.is_empty() {
                    drop(entry);
                    self.subscriptions.remove(&key);
                    info!("Key '{key}' has no more subscribers, removing.");
                }
            }

            if let Some(mut sub_keys) = self.user_subscriptions.get_mut(client) {
                sub_keys.remove(&key);
            }
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
                        info!("Key '{key}' has no more subscribers, removing.");
                    }
                }
            }
        }
    }

    pub fn unsubscribe_group(&self, prefix: &str, client: &Uuid) {
        if let Some(mut user_keys) = self.user_subscriptions.get_mut(client) {
            let mut to_remove = Vec::new();
            for key in user_keys.iter() {
                if key.starts_with(prefix) {
                    to_remove.push(key.clone());
                }
            }
            for key in to_remove {
                if let Some(mut subs) = self.subscriptions.get_mut(&key) {
                    subs.remove(client);
                    if subs.is_empty() {
                        self.subscriptions.remove(&key);
                        info!("Key '{key}' has no more subscribers, removing.");
                    }
                }

                user_keys.remove(&key);
                info!("Client '{client}' unsubscribed from group key '{key}'.");
            }
        }
    }
}
