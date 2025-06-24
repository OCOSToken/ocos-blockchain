//! # network::client
//!
//! Manages outgoing TCP connections to remote OCOS peers.
//!
//! This module initiates connection, sends handshake/ping messages,
//! and prepares the socket for async communication (read/write).

use tokio::{
    net::TcpStream,
    io::{AsyncReadExt, AsyncWriteExt},
};
use crate::network::message::{NetworkMessage, serialize_message, deserialize_message};
use crate::network::peer::{PeerInfo, PeerStatus, connect_to_peer};
use std::time::Duration;

/// Connects to a remote peer via TCP and returns its `PeerInfo`.
pub async fn connect_to_remote(addr: &str) -> Result<PeerInfo, String> {
    match TcpStream::connect(addr).await {
        Ok(mut stream) => {
            println!("🌐 Connected to remote peer at {}", addr);

            // Send handshake or ping as initial message
            let ping = NetworkMessage::empty(crate::network::message::MessageType::Ping);
            let bytes = serialize_message(&ping).map_err(|e| e.to_string())?;

            stream.write_all(&bytes).await.map_err(|e| e.to_string())?;

            // Optionally read response
            let mut buffer = vec![0u8; 4096];
            let n = stream.read(&mut buffer).await.map_err(|e| e.to_string())?;
            if n == 0 {
                return Err("Connection closed by peer.".into());
            }

            if let Ok(msg) = deserialize_message(&buffer[..n]) {
                println!("✅ Received response from peer: {:?}", msg.msg_type);
            }

            let peer_info = connect_to_peer(addr).map_err(|e| e)?;
            Ok(peer_info)
        }

        Err(e) => Err(format!("❌ Failed to connect to {}: {}", addr, e)),
    }
}
