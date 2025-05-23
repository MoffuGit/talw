use std::collections::{HashMap, HashSet};

use uuid::Uuid;

type ServerId = Uuid;
type MemberId = Uuid;

//change name to server
#[derive(Default, Debug)]
pub struct Subscriptions {
    pub server_subscriptions: HashMap<ServerId, HashSet<Uuid>>,
    pub user_subscriptions: HashMap<Uuid, HashSet<(ServerId, MemberId)>>,
}

impl Subscriptions {
    pub fn unsubscribe_all(&mut self, user_id: Uuid) -> Vec<(Uuid, Uuid)> {
        let servers = match self.user_subscriptions.remove(&user_id) {
            Some(subs) => subs.into_iter().collect::<Vec<_>>(),
            None => vec![],
        };
        for (server, _) in &servers {
            self.server_subscriptions
                .entry(*server)
                .and_modify(|users| {
                    users.remove(&user_id);
                });
        }
        servers
    }
    pub fn subscribe(&mut self, user: Uuid, server_id: ServerId, member_id: MemberId) {
        self.user_subscriptions
            .entry(user)
            .or_default()
            .insert((server_id, member_id));
        self.server_subscriptions
            .entry(server_id)
            .or_default()
            .insert(user);
    }

    pub fn unsubscribe(&mut self, user: Uuid, server_id: ServerId, member_id: MemberId) {
        self.user_subscriptions.entry(user).and_modify(|topics| {
            topics.remove(&(server_id, member_id));
        });
        self.server_subscriptions
            .entry(server_id)
            .and_modify(|users| {
                users.remove(&user);
            });
    }
}
