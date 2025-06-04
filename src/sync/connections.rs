use std::sync::Arc;
use std::time::Duration;

use async_broadcast::{broadcast, InactiveReceiver, Receiver, Sender};
use dashmap::DashMap;
use log::{debug, error, info};
use serde_json::Value;
use tokio::spawn;
use tokio::time::{interval, Instant};
use uuid::Uuid;

pub type UserConnections = Arc<DashMap<Uuid, Connection>>;

#[derive(Debug, Clone)]
pub struct Connection {
    sender: Sender<Value>,
    receiver: InactiveReceiver<Value>,
    created: Instant,
    confirmed: bool,
}

impl Connection {
    pub fn sender(&self) -> Sender<Value> {
        self.sender.clone()
    }
    pub fn receiver(&self) -> Receiver<Value> {
        self.receiver.clone().activate()
    }
}

impl Connection {
    pub fn new() -> Self {
        let (sender, receiver) = broadcast(1000);
        Connection {
            sender,
            receiver: receiver.deactivate(),
            created: Instant::now(),
            confirmed: false,
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct UserConnectionsManager {
    connections: UserConnections,
}

#[derive(Debug, Clone)]
pub enum ConnectionMessage {
    InitConnection { client: Uuid },
    CompleteConnection { client: Uuid },
    DeleteConnection { client: Uuid },
}

impl UserConnectionsManager {
    pub fn new(connections: UserConnections) -> Self {
        Self { connections }
    }

    pub async fn connection_cleanup(self) {
        spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            debug!("Timed Dead connection checker started.");
            loop {
                interval.tick().await;
                let mut timed_out_connections = Vec::new();
                let now = Instant::now();

                for entry in self.connections.iter() {
                    let user_id = *entry.key();
                    let creation_time = entry.value().created;
                    let confirmed = entry.value().confirmed;

                    if now.duration_since(creation_time) > Duration::from_secs(5) && !confirmed {
                        println!(
                            "User {} exceeded connection timeout ({}sec). Removing.",
                            user_id, 5
                        );
                        timed_out_connections.push(user_id);
                    }
                }

                for user_id in timed_out_connections {
                    self.connections.remove(&user_id);
                    debug!(" Removed timed-out connection for user: {user_id}");
                }
            }
        });
    }

    pub async fn start_receiving(self, mut recevier: Receiver<ConnectionMessage>) {
        info!("Connection Manager: Starting message manager loop...");
        spawn(async move {
            while let Ok(message) = recevier.recv().await {
                match message {
                    ConnectionMessage::InitConnection { client } => {
                        debug!("Connection Manager: Starting connection for {client}");
                        if self.connections.contains_key(&client) {
                            error!("Connection already exist, missing confirmation");
                        } else {
                            self.connections.insert(client, Connection::new());
                            debug!("Connection started");
                        }
                    }
                    ConnectionMessage::CompleteConnection { client } => {
                        if let Some(mut connection) = self.connections.get_mut(&client) {
                            connection.confirmed = true;
                            debug!("Connection Manager: Connection for {client} completed");
                        } else {
                            error!("Connection Manager: Connection for {client} don't exist");
                        }
                    }
                    ConnectionMessage::DeleteConnection { client } => {
                        self.connections.remove(&client);
                    }
                }
            }
        });
    }
}
