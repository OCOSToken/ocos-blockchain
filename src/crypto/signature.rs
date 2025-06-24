//! # crypto::signature
//!
//! Handles cryptographic signing and verification of messages using ECDSA
//! over the secp256k1 curve (via `k256` crate).
//!
//! Used for:
//! - Authenticating OCOS transactions
//! - Securing DAO voting
//! - Verifying consensus messages
//!
//! ✅ Secure, deterministic, and battle-tested.

use k256::{
    ecdsa::{Signature as K256Signature, SigningKey, VerifyingKey, signature::{Signer, Verifier}},
    elliptic_curve::sec1::ToEncodedPoint,
};
use crate::crypto::keypair::{PrivateKey, PublicKey};

/// A digital signature (64 bytes, secp256k1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    inner: K256Signature,
}

impl Signature {
    /// Returns the byte representation of the signature.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_der().as_bytes().to_vec()
    }

    /// Restores a signature from raw DER bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        K256Signature::from_der(bytes).ok().map(|sig| Signature { inner: sig })
    }
}

/// Signs arbitrary message data using a given private key.
pub fn sign_message(message: &[u8], private_key: &PrivateKey) -> Signature {
    let signing_key = SigningKey::from(private_key.to_bytes());
    let signature = signing_key.sign(message);
    Signature { inner: signature }
}

/// Verifies that a signature is valid for the given message and public key.
pub fn verify_signature(message: &[u8], signature: &Signature, public_key: &PublicKey) -> bool {
    match k256::PublicKey::from_sec1_bytes(public_key.to_bytes().as_slice()) {
        Ok(pk) => {
            let verifying_key = VerifyingKey::from(pk);
            verifying_key.verify(message, &signature.inner).is_ok()
        }
        Err(_) => false,
    }
}
