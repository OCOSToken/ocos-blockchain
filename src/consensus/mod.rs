//! # Consensus Module
//!
//! This module defines the interface and pluggable structure for various consensus algorithms
//! such as Proof of Work (PoW), Proof of Stake (PoS), Delegated PoS, and DAO-based consensus.
//!
//! ## Goals
//! - Decouple consensus logic from the core blockchain
//! - Allow easy swapping of consensus mechanisms
//! - Standardize interaction with mining, validation and block proposal
//!
//! ## Implementations (in this project)
//! - `pow::ProofOfWork`
//! - `pos::ProofOfStake`
//! - Custom DAO governance model (planned)

pub mod pow;
pub mod pos;

use crate::blockchain::block::Block;
use crate::transaction::tx::Transaction;

/// Represents the result of a consensus validation operation
#[derive(Debug, PartialEq, Eq)]
pub enum ConsensusResult {
    Valid,
    Invalid(String),
}

/// Trait to define a consensus algorithm’s interface
pub trait ConsensusEngine {
    /// Returns the name/type of the consensus algorithm
    fn name(&self) -> &'static str;

    /// Validates a block according to consensus rules
    fn validate_block(&self, block: &Block) -> ConsensusResult;

    /// Determines if a given set of transactions can be included in the next block
    fn validate_transactions(&self, txs: &[Transaction]) -> ConsensusResult;

    /// Calculates the next proposer or miner, depending on the consensus algorithm
    fn select_proposer(&self, previous_block: &Block) -> String;

    /// Optionally adjusts difficulty or state (used in PoW or PoS)
    fn update_state(&mut self, previous_block: &Block);
}

/// Default placeholder engine for testing or no-consensus mode
pub struct DummyConsensus;

impl ConsensusEngine for DummyConsensus {
    fn name(&self) -> &'static str {
        "DummyConsensus"
    }

    fn validate_block(&self, _block: &Block) -> ConsensusResult {
        ConsensusResult::Valid
    }

    fn validate_transactions(&self, _txs: &[Transaction]) -> ConsensusResult {
        ConsensusResult::Valid
    }

    fn select_proposer(&self, _previous_block: &Block) -> String {
        "0x0000000000000000".to_string()
    }

    fn update_state(&mut self, _previous_block: &Block) {
        // No-op
    }
}
