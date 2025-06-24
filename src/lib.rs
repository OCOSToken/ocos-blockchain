//! 🔷 OCOS Blockchain Core Library
//!
//! This is the main library module for the OCOS blockchain node, written in Rust.
//! It provides modular access to all critical components required to run a secure,
//! performant, and extensible decentralized ledger.
//!
//! ## Modules
//! - [`blockchain`] — Core chain and block structures, hashing, and validation
//! - [`transaction`] — Transaction structure, validation, and mempool management
//! - [`consensus`] — Pluggable consensus mechanisms (e.g. PoW, PoS, DAO-based)
//! - [`network`] — P2P networking logic and peer message protocols
//! - [`storage`] — Persistent storage layer (e.g. RocksDB, snapshots)
//! - [`crypto`] — Hashing, Merkle tree, signature, keypair generation
//! - [`wallet`] — Address and key management, signing utilities
//! - [`api`] — REST API (Actix-web) for external access and control
//! - [`node`] — Node orchestration and runtime management
//! - [`utils`] — General-purpose utilities (e.g. config, logging, time handling)
//!
//! This file acts as the entry point for crates depending on this library,
//! and provides simplified exports for easy integration in `main.rs`, tests, and CLI tools.

pub mod api;
pub mod blockchain;
pub mod consensus;
pub mod crypto;
pub mod network;
pub mod node;
pub mod storage;
pub mod transaction;
pub mod utils;
pub mod wallet;

// Re-export commonly used components for external use
pub use api::handlers::init_routes;
pub use blockchain::block::Block;
pub use blockchain::chain::Blockchain;
pub use transaction::tx::Transaction;
pub use node::node::OcosNode;
pub use utils::config::Config;
pub use crypto::keypair::{generate_keypair, Address};

/// OCOS client version
pub const OCOS_VERSION: &str = "0.1.0";

/// Genesis block philosophy message
pub const GENESIS_MESSAGE: &str = "OCOS Genesis Block. 11 July 2025 — The New Era of Decentralization. Not your keys, not your mind.";

/// Displays an ASCII banner for console initialization
pub fn display_banner() {
    println!(
        r#"
   ____   ____   ____   _____   _____ 
  / __ \ / __ \ / __ \ / ____| |  __ \
 | |  | | |  | | |  | | (___   | |__) |   v{}
 | |  | | |  | | |  | |\___ \  |  _  /    
 | |__| | |__| | |__| |____) | | | \ \    
  \____/ \____/ \____/|_____/  |_|  \_\

{}
"#,
        OCOS_VERSION, GENESIS_MESSAGE
    );
}
