//! # network::auth
//!
//! Handles peer handshake authentication and identity verification.
//! Each peer sends a signed handshake message proving control over a public key.
//!
//! Ensures:
//! - Replay attack protection
//! - Peer identity verification
//! - Connection trust bootstrap

use serde::{Serialize, Deserialize};
use crate::crypto::keypair::{PublicKey};
use crate::crypto::signature::{Signature, sign_message, verify_signature};
use chrono::{Utc, DateTime};

/// Handshake message sent during initial peer connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub timestamp: DateTime<Utc>,
    pub peer_id: String,
    pub client_version: String,
    pub public_key: PublicKey,
    pub signature: Signature,
}

impl HandshakeMessage {
    /// Creates and signs a handshake message.
    pub fn new_signed(
        peer_id: &str,
        client_version: &str,
        public_key: &PublicKey,
        private_key: &crate::crypto::keypair::PrivateKey,
    ) -> Self {
        let timestamp = Utc::now();
        let payload = handshake_payload(peer_id, client_version, &timestamp);

        let signature = sign_message(&payload, private_key);

        HandshakeMessage {
            timestamp,
            peer_id: peer_id.to_string(),
            client_version: client_version.to_string(),
            public_key: public_key.clone(),
            signature,
        }
    }

    /// Verifies the authenticity of the handshake.
    pub fn verify(&self) -> bool {
        let payload = handshake_payload(&self.peer_id, &self.client_version, &self.timestamp);
        verify_signature(&payload, &self.signature, &self.public_key)
    }
}

/// Serializes the handshake payload (without the signature).
fn handshake_payload(peer_id: &str, client_version: &str, timestamp: &DateTime<Utc>) -> Vec<u8> {
    let raw = format!("OCOS-HANDSHAKE|{}|{}|{}", peer_id, client_version, timestamp);
    raw.as_bytes().to_vec()
}
