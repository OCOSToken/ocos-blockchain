//! OCOS Node Runtime Loop
//! Runs core services such as sync, mining, API and monitors node lifecycle.

use crate::node::node::Node;
use crate::api::routes::start_api_server;
use std::thread;
use std::time::Duration;

/// Starts the OCOS node runtime:
/// - Launches REST API server
/// - Begins mining loop (if enabled)
/// - Periodically syncs blockchain from peers
pub fn run_node(mut node: Node) {
    println!("🎛️  OCOS node runtime started...");

    // 🛰 Start API server in a separate thread
    let api_config = node.config.clone();
    let api_handle = thread::spawn(move || {
        start_api_server(api_config);
    });

    // ⛏ Start mining loop in main thread (basic PoW simulator)
    if node.config.enable_mining {
        println!("⛏️  Mining enabled. Starting mining loop...");
        loop {
            thread::sleep(Duration::from_secs(10)); // simulate block time

            // TODO: generate block from mempool and add to chain
            println!("🔨 Mining simulated block... (placeholder)");

            // broadcast mined block to peers (future)
        }
    }

    // 🔁 Periodic peer sync (every 30 seconds)
    let sync_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));
            node.sync_chain();
        }
    });

    // Join API and sync threads (if needed)
    api_handle.join().unwrap();
    sync_handle.join().unwrap();
}
