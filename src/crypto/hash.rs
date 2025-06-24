//! # crypto::hash
//!
//! Cryptographic hash functions used across the OCOS blockchain.
//!
//! Provides the following hash utilities:
//! - `sha256_hash`: single SHA-256 hash
//! - `double_sha256`: double SHA-256 (used in Bitcoin-like structures)
//! - `blake2b_hash`: Blake2b-256, fast and secure alternative
//! - `merkle_root`: calculate Merkle root from a list of data entries

use sha2::{Sha256, Digest};
use blake2::{Blake2b256, Digest as BlakeDigest};

/// Computes a single SHA-256 hash of the input data.
pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Computes a double SHA-256 hash (SHA256(SHA256(data))).
/// Common in Bitcoin block headers and transaction IDs.
pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = sha256_hash(data);
    sha256_hash(&first_hash)
}

/// Computes a BLAKE2b-256 hash of the input data.
/// Useful for fast hashing (used in Polkadot, Zcash, etc.).
pub fn blake2b_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Calculates a Merkle root from a list of byte-array leaves.
/// Pads with last element if odd count. Returns 32-byte root hash.
pub fn merkle_root(leaves: &[Vec<u8>]) -> Vec<u8> {
    if leaves.is_empty() {
        return sha256_hash(b"");
    }

    let mut hashes = leaves.to_vec();

    while hashes.len() > 1 {
        let mut next_level = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = if i + 1 < hashes.len() {
                &hashes[i + 1]
            } else {
                &hashes[i] // pad with last if odd number
            };

            let mut combined = Vec::with_capacity(left.len() + right.len());
            combined.extend_from_slice(left);
            combined.extend_from_slice(right);

            let new_hash = sha256_hash(&combined);
            next_level.push(new_hash);
        }

        hashes = next_level;
    }

    hashes[0].clone()
}
