// src/stream_tape.rs – S470SHI VM Stream-Based Ledger Layer

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub memo: String,
    pub zk_hash: String,
    pub timestamp: DateTime<Utc>,
    pub tag: Option<String>,
}

#[derive(Debug, Default)]
pub struct StreamTape {
    pub log: Vec<Transaction>,
    pub anchor: Option<DateTime<Utc>>,
}

impl StreamTape {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            anchor: None,
        }
    }

    pub fn commit(&mut self, tx: Transaction) -> Result<(), String> {
        if tx.zk_hash.len() != 66 || !tx.zk_hash.starts_with("zk-0x") {
            return Err("Invalid zk-proof hash format".into());
        }

        if tx.amount <= 0.0 {
            return Err("Zero or negative amount is not allowed".into());
        }

        self.log.push(tx);
        Ok(())
    }

    pub fn anchor_now(&mut self) {
        self.anchor = Some(Utc::now());
    }

    pub fn get_recent(&self, count: usize) -> Vec<&Transaction> {
        self.log.iter().rev().take(count).collect()
    }

    pub fn trace_summary(&self) {
        println!("📄 Stream Ledger Summary ({} entries)", self.log.len());
        for tx in &self.log {
            println!(
                "- {} → {} | {:.4} | {} | {}",
                tx.from,
                tx.to,
                tx.amount,
                tx.memo,
                tx.timestamp
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_commit_and_anchor() {
        let mut tape = StreamTape::new();

        let tx = Transaction {
            from: "0xAlice".into(),
            to: "0xVault".into(),
            amount: 0.047,
            memo: "DAO bootstrap",
            zk_hash: "zk-0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab",
            timestamp: Utc::now(),
            tag: Some("dao-genesis".into()),
        };

        let result = tape.commit(tx);
        assert!(result.is_ok());
        tape.anchor_now();
        assert!(tape.anchor.is_some());
    }
}
