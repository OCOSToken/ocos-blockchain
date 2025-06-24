//! # network::message
//!
//! Defines the protocol messages exchanged between OCOS peers.
//! This includes handshake, block and transaction propagation, pings, and errors.
//!
//! Messages are encoded via serde and transferred over TCP or WebSocket.

use serde::{Serialize, Deserialize};

/// The type of message being transmitted over the OCOS network.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Handshake,
    Ping,
    Pong,
    Transaction,
    Block,
    RequestBlock,
    ResponseBlock,
    Status,
    Error,
}

/// Main message wrapper structure for peer-to-peer communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub msg_type: MessageType,
    pub payload: Vec<u8>, // raw serialized message (e.g. tx, block, etc.)
}

impl NetworkMessage {
    /// Creates a new message with given type and raw payload.
    pub fn new(msg_type: MessageType, payload: Vec<u8>) -> Self {
        NetworkMessage { msg_type, payload }
    }

    /// Convenience method: empty payload (for Ping/Pong).
    pub fn empty(msg_type: MessageType) -> Self {
        Self::new(msg_type, vec![])
    }
}

/// Serializes a NetworkMessage into bytes (e.g., for sending over socket).
pub fn serialize_message(message: &NetworkMessage) -> Result<Vec<u8>, String> {
    bincode::serialize(message).map_err(|e| format!("Serialization error: {}", e))
}

/// Deserializes raw bytes into a NetworkMessage.
pub fn deserialize_message(data: &[u8]) -> Result<NetworkMessage, String> {
    bincode::deserialize(data).map_err(|e| format!("Deserialization error: {}", e))
}
