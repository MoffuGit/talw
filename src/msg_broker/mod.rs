use std::fmt::Debug;
use std::sync::Arc;

use tokio::sync::Mutex;
use zeromq::{PubSocket, Socket, SubSocket};
use zeromq::prelude::*;

#[derive(Clone)]
pub struct MsgBroker {
    pub publisher: Arc<Mutex<PubSocket>>,
    pub subscriber: Arc<Mutex<SubSocket>>,
}

impl Debug for MsgBroker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MsgBroker").finish()
    }
}

impl MsgBroker {
    pub async fn new() -> Self  {
        let mut publisher = PubSocket::new();
        let bind_endpoint = publisher.bind("tcp://127.0.0.1:5556").await.expect("should bind the publisher");
        let mut subscriber = SubSocket::new();
        subscriber.connect(&bind_endpoint.to_string()).await.expect("should bind the subscriber");
        subscriber.subscribe("").await.expect("should subscribe to all topics");
        MsgBroker {
            publisher: Arc::new(Mutex::new(publisher)),
            subscriber: Arc::new(Mutex::new(subscriber)),
        }
    }
}
