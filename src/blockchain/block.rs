//! Block structure definition for the OCOS blockchain.
//!
//! This module contains the core definition of a `Block`,
//! including its structure, constructor, and hash calculation.
//!
//! Inspired by Bitcoin and adapted for modular, future-proof Rust-based chains.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex::encode;

use crate::transaction::tx::Transaction;

/// Represents a single block in the OCOS blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Position of the block in the chain.
    pub index: u64,

    /// UTC timestamp when the block was created.
    pub timestamp: String,

    /// Hash of the previous block in the chain.
    pub previous_hash: String,

    /// Merkle root of all transactions in this block.
    pub merkle_root: String,

    /// SHA256 hash of the current block (after mining).
    pub hash: String,

    /// Nonce used to mine the block.
    pub nonce: u64,

    /// Difficulty level used during mining (number of leading zeros).
    pub difficulty: usize,

    /// All transactions included in the block.
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Creates a new (unmined) block.
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Self {
        let timestamp: DateTime<Utc> = Utc::now();
        let timestamp_str = timestamp.to_rfc3339();
        let merkle_root = Block::calculate_merkle_root(&transactions);

        Self {
            index,
            timestamp: timestamp_str,
            previous_hash,
            merkle_root,
            hash: String::new(), // Will be computed after mining
            nonce: 0,
            difficulty,
            transactions,
        }
    }

    /// Returns the block header string (used for hashing).
    pub fn header_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.merkle_root,
            self.nonce,
            self.difficulty
        )
    }

    /// Calculates the hash of the block using SHA256.
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.header_string());
        encode(hasher.finalize())
    }

    /// Calculates the Merkle root from the transactions.
    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.calculate_hash())
            .collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let a = &hashes[i];
                let b = if i + 1 < hashes.len() {
                    &hashes[i + 1]
                } else {
                    a // duplicate the last hash if odd number
                };

                let mut hasher = Sha256::new();
                hasher.update(a.as_bytes());
                hasher.update(b.as_bytes());
                new_hashes.push(encode(hasher.finalize()));
            }
            hashes = new_hashes;
        }

        hashes[0].clone()
    }

    /// Starts mining by iterating nonces until the hash meets the difficulty target.
    pub fn mine(&mut self) {
        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                self.hash = hash;
                break;
            } else {
                self.nonce += 1;
            }
        }
    }

    /// Validates the current block hash against expected difficulty.
    pub fn is_valid(&self) -> bool {
        self.hash == self.calculate_hash() && self.hash.starts_with(&"0".repeat(self.difficulty))
    }
}
