//! genesis.rs — Genesis block creation logic for OCOS Blockchain
//!
//! This module defines the creation of the first block in the blockchain (Genesis Block).
//! It includes a hardcoded motivational message and initializes the chain with a predefined transaction.
//!
//! Genesis blocks are immutable and serve as the trust anchor for all future blocks.
//! © OCOS Blockchain, 2025 — "Not your keys, not your mind."

use chrono::{DateTime, Utc, TimeZone};
use crate::blockchain::block::Block;
use crate::transaction::tx::Transaction;
use crate::crypto::hash::calculate_merkle_root;

/// The default timestamp for OCOS Genesis Block
pub const GENESIS_TIMESTAMP: &str = "2025-07-11T00:47:00Z";

/// The default difficulty for the genesis block (can be updated)
pub const GENESIS_DIFFICULTY: usize = 4;

/// The symbolic OCOS Genesis reward
pub const GENESIS_REWARD: u64 = 47;

/// The initial recipient of the Genesis Block reward
pub const GENESIS_RECEIVER: &str = "OCOS_PUBLIC_KEY";

/// A unique, philosophical message encoded into the genesis block.
pub const GENESIS_MESSAGE: &str =
    "11 July 2025 — OCOS Genesis Block. The New Era of Decentralization. Not your keys, not your mind.";

/// Creates the Genesis Block with pre-defined metadata, transaction, and hardcoded timestamp.
///
/// # Returns
/// - [`Block`] — A fully mined and immutable genesis block.
///
/// # Example
/// ```
/// let genesis = create_genesis_block();
/// println!("Genesis block hash: {}", genesis.hash);
/// ```
pub fn create_genesis_block() -> Block {
    // Create the genesis transaction
    let tx = Transaction {
        from: String::from("0x0"),  // No sender
        to: GENESIS_RECEIVER.to_string(),
        amount: GENESIS_REWARD,
        message: GENESIS_MESSAGE.to_string(),
    };

    let transactions = vec![tx.clone()];
    let merkle_root = calculate_merkle_root(&transactions);

    let timestamp: DateTime<Utc> = Utc.datetime_from_str(GENESIS_TIMESTAMP, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Invalid GENESIS_TIMESTAMP format");

    let mut block = Block {
        index: 0,
        timestamp: timestamp.to_rfc3339(),
        previous_hash: "0".repeat(64),
        hash: String::new(),
        merkle_root,
        nonce: 0,
        difficulty: GENESIS_DIFFICULTY,
        transactions,
    };

    block.mine(); // Perform Proof-of-Work
    block
}
