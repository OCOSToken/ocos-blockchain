//! # network::router
//!
//! The message router is responsible for handling incoming peer messages,
//! broadcasting messages to other peers, and delegating actions
//! to appropriate handlers based on message type.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::network::message::{NetworkMessage, MessageType};
use crate::network::peer::{PeerId, PeerInfo, PeerStatus};
use crate::network::message::{serialize_message};

/// A thread-safe, shareable peer list.
pub type PeerMap = Arc<Mutex<HashMap<PeerId, PeerInfo>>>;

/// Manages message propagation and peer communication.
pub struct MessageRouter {
    pub peers: PeerMap,
}

impl MessageRouter {
    /// Creates a new message router instance.
    pub fn new(peers: PeerMap) -> Self {
        MessageRouter { peers }
    }

    /// Broadcasts a message to all connected peers.
    pub fn broadcast(&self, message: &NetworkMessage) {
        let peers = self.peers.lock().unwrap();
        let bytes = match serialize_message(message) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to serialize message: {}", e);
                return;
            }
        };

        for (peer_id, peer) in peers.iter() {
            if peer.status == PeerStatus::Connected {
                println!("📡 Sending {:?} to peer [{}] at {}", message.msg_type, peer_id, peer.address);
                // In real implementation: send bytes via TCP or WebSocket
                // e.g., peer.send(bytes.clone());
            }
        }
    }

    /// Handles a message received from a specific peer.
    pub fn handle_incoming(&self, sender_id: PeerId, message: NetworkMessage) {
        match message.msg_type {
            MessageType::Ping => {
                println!("📥 Received Ping from {}", sender_id);
                // reply with Pong
                let pong = NetworkMessage::empty(MessageType::Pong);
                if let Some(peer) = self.peers.lock().unwrap().get(&sender_id) {
                    println!("↩️ Sending Pong to {}", peer.address);
                    // send(pong) to sender_id...
                }
            }

            MessageType::Transaction => {
                println!("📥 Received Transaction from {}", sender_id);
                // TODO: validate transaction, add to mempool, propagate to others
            }

            MessageType::Block => {
                println!("📥 Received Block from {}", sender_id);
                // TODO: validate block, check chain state, add to chain
            }

            _ => {
                println!("⚠️ Received unhandled message type: {:?}", message.msg_type);
            }
        }
    }
}
