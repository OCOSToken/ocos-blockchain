//! Blockchain module
//!
//! This module defines the core structures and behaviors of the OCOS blockchain,
//! including blocks, the chain itself, and genesis initialization.
//!
//! Modules:
//! - `block`: Defines the Block structure, Merkle tree, and hashing logic
//! - `chain`: Manages the blockchain state, validation, and block insertion
//! - `genesis`: Builds and returns the initial genesis block

pub mod block;
pub mod chain;
pub mod genesis;

pub use block::{Block, BlockHeader, BlockHash};
pub use chain::{Blockchain, ChainError, ChainResult};
pub use genesis::create_genesis_block;
