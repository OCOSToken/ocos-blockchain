//! # wallet::wallet
//!
//! Provides functionality for generating, managing, and serializing OCOS wallet accounts.
//! A wallet consists of a private key, derived public key, and associated address.
//!
//! ⚠️ Private keys are kept in memory only; secure storage integration (e.g., keystore or HSM)
//! is recommended for production use.

use crate::crypto::keypair::{PrivateKey, PublicKey, generate_keypair};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use hex;

/// Represents a locally held OCOS wallet/account.
#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub address: String,
}

impl Wallet {
    /// Generates a new OCOS wallet from a fresh key pair.
    pub fn new() -> Self {
        let (private_key, public_key) = generate_keypair();
        let address = derive_address(&public_key);
        Wallet {
            private_key,
            public_key,
            address,
        }
    }

    /// Returns the wallet address as a string.
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// Exports the private key in hex format (NOT SAFE — only for dev/testing).
    pub fn export_private_key_hex(&self) -> String {
        hex::encode(self.private_key.to_bytes())
    }
}

/// Derives a wallet address from a public key using:
/// SHA256 -> RIPEMD160 -> Hex (OCOS prefixed)
pub fn derive_address(public_key: &PublicKey) -> String {
    let pubkey_bytes = public_key.to_bytes();
    let sha256 = Sha256::digest(&pubkey_bytes);
    let ripemd = Ripemd160::digest(&sha256);
    format!("OCOS{}", hex::encode(ripemd))
}
