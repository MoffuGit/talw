use std::collections::{HashMap, HashSet};

use uuid::Uuid;

type ServerId = Uuid;

//change name to server
#[derive(Default, Debug)]
pub struct Subscriptions {
    pub topic_subscriptions: HashMap<ServerId, HashSet<Uuid>>,
    pub user_subscriptions: HashMap<Uuid, HashSet<ServerId>>,
}

impl Subscriptions {
    pub fn unsubscribe_all(&mut self, user_id: Uuid) -> Option<Vec<Uuid>> {
        let servers = self.user_subscriptions.get_mut(&user_id).map(|subs| {
            let servers = subs.clone().into_iter().collect::<Vec<_>>();
            subs.clear();
            servers
        });
        if let Some(servers) = &servers {
            for server in servers {
                self.topic_subscriptions.entry(*server).and_modify(|users| {
                    users.remove(&user_id);
                });
            }
        }
        servers
    }
    pub fn subscribe(&mut self, user: Uuid, server_id: ServerId) {
        self.user_subscriptions
            .entry(user)
            .and_modify(|topics| {
                topics.insert(server_id);
            })
            .or_default();
        self.topic_subscriptions
            .entry(server_id)
            .and_modify(|users| {
                users.insert(user);
            })
            .or_default();
    }

    pub fn unsubscribe(&mut self, user: Uuid, server_id: ServerId) {
        self.user_subscriptions.entry(user).and_modify(|topics| {
            topics.remove(&server_id);
        });
        self.topic_subscriptions
            .entry(server_id)
            .and_modify(|users| {
                users.remove(&user);
            });
    }
}
