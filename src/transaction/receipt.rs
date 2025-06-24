use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};

/// A transaction receipt is generated after a transaction is
/// included in a block and executed successfully.
///
/// It serves as proof of inclusion and execution result.
///
/// Includes:
/// - Transaction hash
/// - Status (success/failure)
/// - Block index & timestamp
/// - Gas used / execution cost (future)
/// - Optional logs/events emitted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub tx_hash: String,              // Hash of the executed transaction
    pub block_index: u64,            // Block in which tx was included
    pub status: TxStatus,            // Success or Failure
    pub timestamp: String,           // Block timestamp
    pub logs: Vec<String>,           // Optional event logs
}

/// Status enum for a transaction execution result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TxStatus {
    Success,
    Failure(String), // Error reason
}

impl TransactionReceipt {
    /// Creates a new successful receipt
    pub fn success(tx_hash: &str, block_index: u64, logs: Vec<String>) -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            tx_hash: tx_hash.to_string(),
            block_index,
            status: TxStatus::Success,
            timestamp: now.to_rfc3339(),
            logs,
        }
    }

    /// Creates a new failed receipt
    pub fn failure(tx_hash: &str, block_index: u64, error: &str) -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            tx_hash: tx_hash.to_string(),
            block_index,
            status: TxStatus::Failure(error.to_string()),
            timestamp: now.to_rfc3339(),
            logs: vec![],
        }
    }

    /// Checks if the receipt indicates success
    pub fn is_success(&self) -> bool {
        self.status == TxStatus::Success
    }
}
