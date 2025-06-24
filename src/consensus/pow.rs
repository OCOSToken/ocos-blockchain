use sha2::{Digest, Sha256};
use hex::encode;
use chrono::prelude::*;
use crate::blockchain::block::Block;

/// Trait for consensus mechanisms
pub trait ConsensusEngine {
    fn mine_block(&self, block: &mut Block);
    fn is_valid_proof(&self, block: &Block) -> bool;
}

/// Proof-of-Work implementation
pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        ProofOfWork { difficulty }
    }

    fn calculate_hash(&self, block: &Block) -> String {
        let input = format!(
            "{}{}{}{}{}",
            block.index,
            block.timestamp,
            block.previous_hash,
            block.merkle_root,
            block.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        encode(hasher.finalize())
    }
}

impl ConsensusEngine for ProofOfWork {
    fn mine_block(&self, block: &mut Block) {
        println!(
            "⛏️  Mining block #{} with difficulty {}...",
            block.index, self.difficulty
        );

        loop {
            let hash = self.calculate_hash(block);
            if hash.starts_with(&"0".repeat(self.difficulty)) {
                block.hash = hash;
                println!("✅ Block mined: {}", block.hash);
                break;
            } else {
                block.nonce += 1;
            }
        }
    }

    fn is_valid_proof(&self, block: &Block) -> bool {
        let hash = self.calculate_hash(block);
        hash.starts_with(&"0".repeat(self.difficulty)) && hash == block.hash
    }
}
