//! # network::peer
//!
//! Provides peer connection structures and utilities for OCOS P2P networking.
//!
//! This includes peer identification, connection metadata, status tracking,
//! and helper methods to initiate or close peer links.

use std::net::SocketAddr;
use std::time::{Instant};

use uuid::Uuid;

/// Represents a unique peer ID (non-cryptographic).
pub type PeerId = Uuid;

/// Current connection status of a peer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerStatus {
    Connected,
    Connecting,
    Disconnected,
    Banned,
}

/// Metadata about a connected or known peer.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: PeerId,
    pub address: SocketAddr,
    pub status: PeerStatus,
    pub last_seen: Option<Instant>,
    pub latency_ms: Option<u128>,
    pub client_version: Option<String>,
}

impl PeerInfo {
    /// Create a new peer with default status (Disconnected).
    pub fn new(address: SocketAddr) -> Self {
        PeerInfo {
            id: Uuid::new_v4(),
            address,
            status: PeerStatus::Disconnected,
            last_seen: None,
            latency_ms: None,
            client_version: None,
        }
    }

    /// Marks peer as connected and resets last_seen timestamp.
    pub fn mark_connected(&mut self) {
        self.status = PeerStatus::Connected;
        self.last_seen = Some(Instant::now());
    }

    /// Marks peer as disconnected.
    pub fn mark_disconnected(&mut self) {
        self.status = PeerStatus::Disconnected;
    }

    /// Updates latency in milliseconds.
    pub fn update_latency(&mut self, latency: u128) {
        self.latency_ms = Some(latency);
    }

    /// Updates client version info (if provided).
    pub fn update_client_version(&mut self, version: &str) {
        self.client_version = Some(version.to_string());
    }
}

/// Attempts to establish a connection with a peer (simulated).
pub fn connect_to_peer(address: &str) -> Result<PeerInfo, String> {
    let socket_addr: SocketAddr = address.parse().map_err(|_| "Invalid address")?;
    let mut peer = PeerInfo::new(socket_addr);
    peer.mark_connected();
    peer.update_client_version("OCOS-Node/0.1.0");
    Ok(peer)
}

/// Closes the connection with a peer.
pub fn disconnect_peer(peer: &mut PeerInfo) {
    peer.mark_disconnected();
}
