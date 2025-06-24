//! OCOS Node Structure
//! Defines the internal state and components of a single node instance.

use crate::blockchain::chain::Blockchain;
use crate::transaction::mempool::Mempool;
use crate::config::config::NodeConfig;
use crate::network::peer::PeerManager;
use crate::wallet::wallet::Wallet;

#[derive(Debug)]
pub struct Node {
    pub config: NodeConfig,
    pub blockchain: Blockchain,
    pub mempool: Mempool,
    pub peer_manager: PeerManager,
    pub wallet: Wallet,
}

impl Node {
    /// Creates a new OCOS Node instance from the given configuration
    pub fn new(config: NodeConfig) -> Self {
        let blockchain = Blockchain::new_genesis(&config);
        let mempool = Mempool::default();
        let peer_manager = PeerManager::from_config(&config);
        let wallet = Wallet::load_or_generate(&config);

        println!("🧠 OCOS Node initialized: network = {}", config.network_name);

        Node {
            config,
            blockchain,
            mempool,
            peer_manager,
            wallet,
        }
    }

    /// Syncs chain with connected peers
    pub fn sync_chain(&mut self) {
        println!("🔄 Syncing chain with peers...");
        // TODO: implement peer-to-peer block sync
    }

    /// Broadcasts new transactions to peer network
    pub fn broadcast_transaction(&self) {
        println!("📡 Broadcasting transaction...");
        // TODO: implement tx broadcast via PeerManager
    }

    /// Validates current state of the node
    pub fn validate(&self) -> bool {
        self.blockchain.validate_chain()
    }
}
