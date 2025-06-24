//! # network::server
//!
//! Tokio-based TCP server for accepting incoming peer connections and processing messages.
//!
//! Each peer is handled on a separate async task. Messages are deserialized and passed to the
//! `MessageRouter` for further handling.

use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{UnboundedSender, UnboundedReceiver, unbounded_channel},
    task,
};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::network::peer::{PeerId, PeerInfo, PeerStatus};
use crate::network::router::MessageRouter;
use crate::network::message::{deserialize_message, serialize_message, NetworkMessage};

type SharedPeers = Arc<Mutex<HashMap<PeerId, PeerInfo>>>;

/// Starts the TCP server for incoming connections.
pub async fn start_server(bind_addr: &str, router: Arc<MessageRouter>) -> std::io::Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    println!("🔌 OCOS node listening on {}", bind_addr);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("✅ Incoming connection from {}", addr);

        let router_clone = Arc::clone(&router);
        task::spawn(async move {
            if let Err(e) = handle_peer(socket, router_clone).await {
                eprintln!("⚠️ Error handling peer {}: {}", addr, e);
            }
        });
    }
}

/// Handles a single peer TCP connection.
async fn handle_peer(mut socket: TcpStream, router: Arc<MessageRouter>) -> std::io::Result<()> {
    let mut buf = vec![0u8; 4096];

    loop {
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            println!("🔌 Peer disconnected");
            break;
        }

        let data = &buf[..n];
        match deserialize_message(data) {
            Ok(message) => {
                let fake_peer_id = Uuid::new_v4(); // Replace with real handshake later
                router.handle_incoming(fake_peer_id, message);
            }
            Err(e) => {
                eprintln!("❌ Failed to parse message: {}", e);
            }
        }
    }

    Ok(())
}
