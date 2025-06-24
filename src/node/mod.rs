//! OCOS Blockchain Node Module
//! This module defines the core structure and lifecycle of a node instance,
//! including initialization, configuration loading, and main loop execution.

pub mod node;
pub mod runner;

use crate::node::node::Node;
use crate::node::runner::run_node;

use crate::config::config::NodeConfig;

/// Initializes the OCOS blockchain node with the provided configuration.
/// This function is typically called from `main.rs` or API/CLI interface.
///
/// # Arguments
/// * `config` - NodeConfig struct containing RPC ports, peers, and environment settings
///
/// # Example
/// ```
/// let config = NodeConfig::load_from_file("config/devnet.toml");
/// start_node(config);
/// ```

pub fn start_node(config: NodeConfig) {
    println!("🚀 Starting OCOS Node [mode: {}]...", config.mode);
    
    let node = Node::new(config);
    run_node(node);
}
