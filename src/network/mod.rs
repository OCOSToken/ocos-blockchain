//! # network
//!
//! The `network` module manages all peer-to-peer (P2P) networking logic for the OCOS blockchain.
//!
//! ## Responsibilities:
//! - Peer discovery and handshake
//! - Message serialization and dispatch
//! - Gossip protocol for transaction/block propagation
//! - Network health and liveness tracking
//! - Future support: DHT, NAT traversal, relay nodes
//!
//! ✅ Designed for modularity, real-time propagation, and secure peer communication

pub mod peer;
pub mod message;
pub mod router;
pub mod discovery;

pub use peer::{
    PeerId,
    PeerInfo,
    PeerStatus,
    connect_to_peer,
    disconnect_peer,
};

pub use message::{
    NetworkMessage,
    MessageType,
    serialize_message,
    deserialize_message,
};

pub use router::{
    MessageRouter,
    handle_incoming,
    broadcast_to_peers,
};

pub use discovery::{
    start_peer_discovery,
    seed_peers,
};

/// Constants related to OCOS network configuration.
pub mod config {
    /// Default port OCOS nodes listen on (for testnet: 47474)
    pub const DEFAULT_PORT: u16 = 47474;

    /// Maximum number of active peers
    pub const MAX_PEERS: usize = 50;

    /// Time between peer pings in seconds
    pub const PING_INTERVAL_SECS: u64 = 30;
}
