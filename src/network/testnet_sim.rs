//! # network::testnet_sim
//!
//! Launches multiple OCOS nodes locally with in-memory peer routing.
//! Useful for development, integration testing, and message propagation simulation.

use crate::network::{
    peer::{PeerInfo, PeerStatus, PeerId},
    router::MessageRouter,
    message::{NetworkMessage, MessageType},
    server::start_server,
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    net::SocketAddr,
    thread,
    time::Duration,
};
use tokio::runtime::Runtime;

/// Represents a simulated node in testnet
pub struct SimulatedNode {
    pub peer_map: Arc<Mutex<HashMap<PeerId, PeerInfo>>>,
    pub router: Arc<MessageRouter>,
    pub port: u16,
}

impl SimulatedNode {
    /// Starts a node asynchronously on a new thread with its own Tokio runtime
    pub fn launch(port: u16) -> Self {
        let peer_map = Arc::new(Mutex::new(HashMap::new()));
        let router = Arc::new(MessageRouter::new(peer_map.clone()));
        let addr = format!("127.0.0.1:{}", port);

        // Launch async server in background thread
        thread::spawn({
            let router_clone = router.clone();
            move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(async {
                    start_server(&addr, router_clone).await.unwrap();
                });
            }
        });

        SimulatedNode {
            peer_map,
            router,
            port,
        }
    }

    /// Broadcasts a test message from this node
    pub fn broadcast_ping(&self) {
        let msg = NetworkMessage::empty(MessageType::Ping);
        self.router.broadcast(&msg);
    }
}
