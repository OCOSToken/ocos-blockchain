//! # crypto::keypair
//!
//! Provides functionality for key pair generation, public key derivation,
//! and serialization for cryptographic signing in the OCOS blockchain.
//!
//! Uses the `k256` crate for secp256k1 elliptic curve cryptography (same as Bitcoin/Ethereum).

use k256::{
    ecdsa::SigningKey,
    elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint},
    PublicKey as K256PublicKey,
    SecretKey,
};
use rand_core::OsRng;

/// Represents a private key (secp256k1 curve).
#[derive(Debug, Clone)]
pub struct PrivateKey {
    inner: SecretKey,
}

impl PrivateKey {
    /// Generates a new random private key using secure RNG.
    pub fn generate() -> Self {
        let inner = SecretKey::random(&mut OsRng);
        PrivateKey { inner }
    }

    /// Returns the raw 32-byte representation of the private key.
    pub fn to_bytes(&self) -> [u8; 32] {
        self.inner.to_bytes()
    }

    /// Derives the associated public key.
    pub fn public_key(&self) -> PublicKey {
        let signing_key = SigningKey::from(&self.inner);
        let verifying_key = signing_key.verifying_key();
        PublicKey {
            inner: verifying_key.to_encoded_point(false),
        }
    }
}

/// Represents a public key (uncompressed SEC1 format).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    inner: k256::EncodedPoint,
}

impl PublicKey {
    /// Returns the raw uncompressed byte form of the public key.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }

    /// Restores a public key from uncompressed bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let encoded = k256::EncodedPoint::from_bytes(bytes).ok()?;
        if K256PublicKey::from_encoded_point(&encoded).is_some().into() {
            Some(Self { inner: encoded })
        } else {
            None
        }
    }

    /// Returns the compressed (33-byte) form of the public key.
    pub fn to_compressed_bytes(&self) -> Vec<u8> {
        self.inner.compress().as_bytes().to_vec()
    }
}

/// Generates a fresh key pair (private and public).
pub fn generate_keypair() -> (PrivateKey, PublicKey) {
    let private = PrivateKey::generate();
    let public = private.public_key();
    (private, public)
}
