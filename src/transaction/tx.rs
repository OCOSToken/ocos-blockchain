use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use hex::encode;
use chrono::{Utc, DateTime};

/// The core transaction structure used in the OCOS blockchain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub from: String,           // Sender public key or address
    pub to: String,             // Recipient public key or address
    pub amount: u64,            // Token or coin amount to transfer
    pub message: String,        // Optional message or metadata
    pub timestamp: String,      // RFC3339 timestamp
    pub signature: Option<String>, // Digital signature (optional)
    pub hash: String,           // Unique hash of the transaction
}

impl Transaction {
    /// Creates a new unsigned transaction
    pub fn new(from: &str, to: &str, amount: u64, message: &str) -> Self {
        let now: DateTime<Utc> = Utc::now();
        let timestamp = now.to_rfc3339();

        let raw = format!("{}{}{}{}{}", from, to, amount, message, &timestamp);
        let hash = Self::calculate_hash(&raw);

        Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            message: message.to_string(),
            timestamp,
            signature: None,
            hash,
        }
    }

    /// Hash function using SHA256
    pub fn calculate_hash(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        encode(hasher.finalize())
    }

    /// Signs the transaction with a given signature
    pub fn sign(&mut self, signature: &str) {
        self.signature = Some(signature.to_string());
    }

    /// Checks if transaction has a signature
    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}
