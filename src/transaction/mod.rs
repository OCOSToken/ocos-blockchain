//! Transaction Module
//!
//! This module handles all logic related to transactions within the OCOS blockchain.
//! It includes:
//! - Transaction structure and serialization
//! - Mempool (pending transactions buffer)
//! - Validation and verification
//! - Receipts and confirmations
//!
//! Designed to be modular, extensible, and secure for real blockchain applications.
//!
//! # Example
//! ```rust
//! use transaction::Transaction;
//! let tx = Transaction::new("Alice", "Bob", 100, "Payment");
//! ```

pub mod tx;
pub mod mempool;
pub mod validator;
pub mod receipt;

pub use tx::Transaction;
pub use mempool::Mempool;
pub use validator::TransactionValidator;
pub use receipt::TransactionReceipt;
