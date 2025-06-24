use std::collections::{HashMap, VecDeque};
use crate::transaction::Transaction;

/// Maximum number of transactions allowed in the mempool
const MAX_MEMPOOL_SIZE: usize = 10_000;

/// Mempool structure to hold unconfirmed/pending transactions.
#[derive(Debug)]
pub struct Mempool {
    pool: VecDeque<Transaction>,               // FIFO queue for processing
    index: HashMap<String, usize>,             // Fast lookup by transaction hash
}

impl Mempool {
    /// Creates a new empty mempool
    pub fn new() -> Self {
        Self {
            pool: VecDeque::new(),
            index: HashMap::new(),
        }
    }

    /// Adds a transaction to the mempool if it's not already present
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        if self.index.contains_key(&tx.hash) {
            return Err("Transaction already exists in the mempool.".into());
        }

        if self.pool.len() >= MAX_MEMPOOL_SIZE {
            return Err("Mempool is full. Try again later.".into());
        }

        self.index.insert(tx.hash.clone(), self.pool.len());
        self.pool.push_back(tx);
        Ok(())
    }

    /// Returns all pending transactions (cloned)
    pub fn all(&self) -> Vec<Transaction> {
        self.pool.iter().cloned().collect()
    }

    /// Clears the mempool completely (e.g., after mining a block)
    pub fn clear(&mut self) {
        self.pool.clear();
        self.index.clear();
    }

    /// Returns the current number of transactions
    pub fn size(&self) -> usize {
        self.pool.len()
    }

    /// Pops N transactions for mining/processing
    pub fn pop_n(&mut self, count: usize) -> Vec<Transaction> {
        let mut selected = Vec::new();
        for _ in 0..count.min(self.pool.len()) {
            if let Some(tx) = self.pool.pop_front() {
                self.index.remove(&tx.hash);
                selected.push(tx);
            }
        }
        selected
    }

    /// Check if a transaction exists by its hash
    pub fn contains(&self, tx_hash: &str) -> bool {
        self.index.contains_key(tx_hash)
    }
}
