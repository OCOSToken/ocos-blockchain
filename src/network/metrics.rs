//! # network::metrics
//!
//! Provides Prometheus-compatible metrics collection for OCOS P2P network.
//!
//! Tracks peer count, message throughput, connection events, and block propagation latency.

use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use prometheus::{
    Encoder, TextEncoder, IntGauge, IntCounter, Histogram, HistogramOpts, Registry,
};

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // 🧩 Peer Metrics
    pub static ref ACTIVE_PEER_COUNT: IntGauge = register_gauge(
        "ocos_active_peers",
        "Number of currently connected peers"
    );

    pub static ref TOTAL_PEER_CONNECTIONS: IntCounter = register_counter(
        "ocos_total_peer_connections",
        "Total peer connection attempts"
    );

    pub static ref TOTAL_PEER_DISCONNECTIONS: IntCounter = register_counter(
        "ocos_total_peer_disconnections",
        "Total peer disconnections"
    );

    // ✉️ Message Metrics
    pub static ref MESSAGES_RECEIVED: IntCounter = register_counter(
        "ocos_messages_received",
        "Total network messages received"
    );

    pub static ref MESSAGES_SENT: IntCounter = register_counter(
        "ocos_messages_sent",
        "Total network messages sent"
    );

    // ⏱ Block Propagation
    pub static ref BLOCK_PROPAGATION_TIME: Histogram = register_histogram(
        HistogramOpts::new(
            "ocos_block_propagation_seconds",
            "Time from block origin to full propagation"
        )
        .buckets(vec![0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0])
    );
}

/// Increments peer connection count.
pub fn peer_connected() {
    ACTIVE_PEER_COUNT.inc();
    TOTAL_PEER_CONNECTIONS.inc();
}

/// Increments peer disconnection count.
pub fn peer_disconnected() {
    ACTIVE_PEER_COUNT.dec();
    TOTAL_PEER_DISCONNECTIONS.inc();
}

/// Increments received message count.
pub fn message_received() {
    MESSAGES_RECEIVED.inc();
}

/// Increments sent message count.
pub fn message_sent() {
    MESSAGES_SENT.inc();
}

/// Records block propagation delay.
pub fn record_block_propagation(start_time: Instant) {
    let elapsed = start_time.elapsed().as_secs_f64();
    BLOCK_PROPAGATION_TIME.observe(elapsed);
}

/// Exposes metrics as a Prometheus-formatted string.
pub fn export_prometheus_metrics() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

// Helper registration functions

fn register_counter(name: &str, help: &str) -> IntCounter {
    let counter = IntCounter::new(name, help).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
}

fn register_gauge(name: &str, help: &str) -> IntGauge {
    let gauge = IntGauge::new(name, help).unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
}

fn register_histogram(opts: HistogramOpts) -> Histogram {
    let histogram = Histogram::with_opts(opts).unwrap();
    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
}
