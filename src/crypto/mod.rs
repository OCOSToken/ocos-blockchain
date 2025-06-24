//! # crypto
//!
//! The `crypto` module contains all cryptographic primitives used in the OCOS blockchain.
//! This includes hashing algorithms, digital signatures, public/private key generation,
//! address derivation, and cryptographic utilities.
//!
//! ## Submodules:
//! - `hash`: SHA-256, Blake2b, Merkle Root, etc.
//! - `keypair`: ECDSA key generation and management.
//! - `signature`: Message signing and verification.
//!
//! This module ensures all cryptographic operations are deterministic, secure, and auditable.

pub mod hash;
pub mod keypair;
pub mod signature;

pub use hash::{
    sha256_hash,
    double_sha256,
    blake2b_hash,
    merkle_root,
};

pub use keypair::{
    KeyPair,
    PublicKey,
    PrivateKey,
    generate_keypair,
};

pub use signature::{
    sign_message,
    verify_signature,
    Signature,
};

/// Re-exports for simplified usage from external modules.
pub mod prelude {
    pub use super::{
        sha256_hash,
        double_sha256,
        merkle_root,
        generate_keypair,
        sign_message,
        verify_signature,
        PublicKey,
        PrivateKey,
        Signature,
    };
}
