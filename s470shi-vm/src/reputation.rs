// src/reputation.rs – S470SHI VM Reputation Engine

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Identity {
    pub address: String,
    pub reputation: f64,
    pub dao_votes: u32,
    pub last_active_block: u64,
}

#[derive(Debug, PartialEq)]
pub enum ExecTier {
    Priority,
    Trusted,
    Limited,
}

pub struct ReputationEngine {
    pub identities: HashMap<String, Identity>,
}

impl ReputationEngine {
    pub fn new() -> Self {
        Self {
            identities: HashMap::new(),
        }
    }

    pub fn update_reputation(&mut self, addr: &str, vote_cast: bool, current_block: u64) {
        let identity = self.identities.entry(addr.to_string()).or_insert(Identity {
            address: addr.to_string(),
            reputation: 50.0,
            dao_votes: 0,
            last_active_block: current_block,
        });

        if vote_cast {
            identity.dao_votes += 1;
            identity.reputation *= 1.03;
        } else {
            identity.reputation *= 0.97;
        }

        identity.last_active_block = current_block;
        identity.reputation = identity.reputation.clamp(0.0, 100.0);
    }

    pub fn get_exec_tier(&self, addr: &str) -> ExecTier {
        match self.identities.get(addr) {
            Some(id) if id.reputation > 90.0 => ExecTier::Priority,
            Some(id) if id.reputation > 47.0 => ExecTier::Trusted,
            _ => ExecTier::Limited,
        }
    }

    pub fn get_identity(&self, addr: &str) -> Option<&Identity> {
        self.identities.get(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_growth_and_tier() {
        let mut engine = ReputationEngine::new();
        let addr = "0xAlice";

        for i in 0..10 {
            engine.update_reputation(addr, true, 1000 + i);
        }

        let tier = engine.get_exec_tier(addr);
        assert_eq!(tier, ExecTier::Priority);

        let id = engine.get_identity(addr).unwrap();
        assert!(id.reputation > 90.0);
        assert_eq!(id.dao_votes, 10);
    }

    #[test]
    fn test_reputation_decay() {
        let mut engine = ReputationEngine::new();
        let addr = "0xBob";

        engine.update_reputation(addr, false, 1050);
        let tier = engine.get_exec_tier(addr);
        assert_eq!(tier, ExecTier::Trusted);
    }
}
