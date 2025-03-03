use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::topic::Topic;

#[derive(Default, Debug, Clone)]
pub struct Subscriptions {
    topic_subscriptions: HashMap<Topic, HashSet<Uuid>>,
    user_subscriptions: HashMap<Uuid, HashSet<Topic>>,
}

impl Subscriptions {
    pub fn subscribe(&mut self, user: Uuid, topic: Topic) {
        self.user_subscriptions
            .entry(user.clone())
            .and_modify(|topics| {
                topics.insert(topic.clone());
            })
            .or_default();
        self.topic_subscriptions
            .entry(topic)
            .and_modify(|users| {
                users.insert(user);
            })
            .or_default();
    }

    pub fn unsubscribe(&mut self, user: Uuid, topic: Topic) {
        self.user_subscriptions.entry(user).and_modify(|topics| {
            topics.remove(&topic);
        });
        self.topic_subscriptions.entry(topic).and_modify(|users| {
            users.remove(&user);
        });
    }
}
