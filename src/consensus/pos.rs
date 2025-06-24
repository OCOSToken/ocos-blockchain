//! Proof-of-Stake (PoS) Consensus Mechanism
//!
//! This module implements a simplified Proof-of-Stake consensus logic,
//! including staking, validator management, and leader selection.

use std::collections::{HashMap, HashSet};
use rand::{seq::IteratorRandom, thread_rng};

/// Type alias for address
pub type Address = String;

/// Minimum stake required to become a validator
const MIN_STAKE: u64 = 100;

/// Represents a staking record for a given address
#[derive(Debug, Clone)]
pub struct Stake {
    pub staker: Address,
    pub amount: u64,
}

/// Maintains all active stakes in the system
#[derive(Default, Debug)]
pub struct StakingPool {
    stakes: HashMap<Address, u64>,
    validators: HashSet<Address>,
}

impl StakingPool {
    /// Create a new staking pool
    pub fn new() -> Self {
        Self {
            stakes: HashMap::new(),
            validators: HashSet::new(),
        }
    }

    /// Stake tokens to become eligible for validation
    pub fn stake(&mut self, address: Address, amount: u64) {
        let entry = self.stakes.entry(address.clone()).or_insert(0);
        *entry += amount;

        if *entry >= MIN_STAKE {
            self.validators.insert(address.clone());
        }
    }

    /// Unstake tokens (partial or full)
    pub fn unstake(&mut self, address: &Address, amount: u64) {
        if let Some(balance) = self.stakes.get_mut(address) {
            *balance = balance.saturating_sub(amount);
            if *balance < MIN_STAKE {
                self.validators.remove(address);
            }
        }
    }

    /// Retrieve the stake of a given address
    pub fn get_stake(&self, address: &Address) -> u64 {
        *self.stakes.get(address).unwrap_or(&0)
    }

    /// Get the list of current validators
    pub fn get_validators(&self) -> Vec<Address> {
        self.validators.iter().cloned().collect()
    }

    /// Randomly select a validator based on stake-weighted lottery
    pub fn select_leader(&self) -> Option<Address> {
        let mut weighted_pool = vec![];

        for (addr, amount) in &self.stakes {
            if *amount >= MIN_STAKE {
                // Add the address multiple times based on stake
                for _ in 0..*amount {
                    weighted_pool.push(addr.clone());
                }
            }
        }

        let mut rng = thread_rng();
        weighted_pool.into_iter().choose(&mut rng)
    }
}
