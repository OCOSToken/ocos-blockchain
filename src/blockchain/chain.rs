//! # Chain Module
//! This module manages the blockchain structure, including block sequencing,
//! hash validation, block addition logic, and chain integrity checks.

use chrono::prelude::*;
use crate::blockchain::block::Block;
use crate::transaction::tx::Transaction;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Initializes a new blockchain instance with the Genesis block.
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::genesis(difficulty);
        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    /// Returns the latest block in the chain.
    pub fn latest_block(&self) -> &Block {
        self.blocks.last().expect("Blockchain must contain at least one block")
    }

    /// Adds a new block to the chain using the given transactions.
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.latest_block().hash.clone();
        let index = self.blocks.len() as u64;
        let timestamp = Utc::now().to_rfc3339();

        let mut new_block = Block::new(
            index,
            timestamp,
            previous_hash,
            transactions,
            self.difficulty,
        );

        new_block.mine();
        self.blocks.push(new_block);
    }

    /// Validates the entire chain's integrity by checking hashes and links.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Validate hash
            if current.hash != current.calculate_hash() {
                eprintln!("Invalid hash at block {}", i);
                return false;
            }

            // Validate previous hash linkage
            if current.previous_hash != previous.hash {
                eprintln!("Invalid previous hash link at block {}", i);
                return false;
            }
        }
        true
    }

    /// Returns the number of blocks in the chain.
    pub fn length(&self) -> usize {
        self.blocks.len()
    }

    /// Exports the full chain as a JSON string (for explorer or archive).
    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.blocks).unwrap_or_else(|_| "[]".to_string())
    }
}
