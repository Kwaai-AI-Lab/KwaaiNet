//! Unit tests for kwaai-p2p: NetworkConfig and NodeCapabilities.
//! No network, no daemon required.
//!
//! The ServerInfo / hivemind-framing cases that used to live here were removed
//! with the Phase 1 rewrite: `kwaai_p2p::hivemind` was a prototype on a wire
//! format that does not match hivemind's, and the real codec lands in
//! `kwaai-hivemind-dht` (Phase 0/2). Swarm behaviour is covered by
//! `kwaai-p2p`'s own `tests/swarm.rs`.

use kwaai_network_tests::metrics::MetricsRecorder;
use kwaai_p2p::{
    config::{NetworkConfig, KWAAI_BOOTSTRAP_SERVERS},
    NodeCapabilities, PETALS_BOOTSTRAP_SERVERS,
};
use std::time::Duration;

// ============================================================================
// NetworkConfig builder
// ============================================================================

#[test]
fn config_default_has_sane_values() {
    let rec = MetricsRecorder::start("unit::p2p::config_default_sane_values", "unit");
    let cfg = NetworkConfig::default();
    assert!(cfg.enable_dht);
    assert!(cfg.enable_relay_client);
    assert!(cfg.enable_nat_traversal);
    assert_eq!(cfg.max_connections, 100);
    assert_eq!(cfg.dht_replication, 20);
    assert!(!cfg.listen_addrs.is_empty());
    rec.finish(true);
}

#[test]
fn config_with_kwaai_bootstrap_includes_bootstrap_addrs() {
    let mut rec = MetricsRecorder::start("unit::p2p::config_with_kwaai_bootstrap", "unit");
    let cfg = NetworkConfig::with_kwaai_bootstrap();
    assert!(!cfg.bootstrap_peers.is_empty());
    for addr in &cfg.bootstrap_peers {
        assert!(
            addr.starts_with("/ip4/"),
            "bootstrap addr must be a valid multiaddr: {addr}"
        );
    }
    rec.metric("bootstrap_count", cfg.bootstrap_peers.len());
    rec.finish(true);
}

#[test]
fn config_builder_overrides() {
    let rec = MetricsRecorder::start("unit::p2p::config_builder_overrides", "unit");
    let cfg = NetworkConfig::builder()
        .max_connections(50)
        .connection_timeout(Duration::from_secs(10))
        .request_timeout(Duration::from_secs(5))
        .listen_addrs(vec!["/ip4/0.0.0.0/tcp/9000".to_string()])
        .bootstrap_peers(vec!["/ip4/1.2.3.4/tcp/8000".to_string()])
        .build();

    assert_eq!(cfg.max_connections, 50);
    assert_eq!(cfg.connection_timeout, Duration::from_secs(10));
    assert_eq!(cfg.listen_addrs, vec!["/ip4/0.0.0.0/tcp/9000"]);
    assert_eq!(cfg.bootstrap_peers.len(), 1);
    rec.finish(true);
}

#[test]
fn petals_bootstrap_servers_are_well_formed() {
    let mut rec = MetricsRecorder::start("unit::p2p::petals_bootstrap_servers_well_formed", "unit");
    for addr in PETALS_BOOTSTRAP_SERVERS {
        assert!(addr.starts_with("/ip4/"), "bad multiaddr: {addr}");
        assert!(addr.contains("/tcp/"), "missing tcp component: {addr}");
        assert!(
            addr.contains("/p2p/"),
            "missing p2p/peer_id component: {addr}"
        );
    }
    for addr in KWAAI_BOOTSTRAP_SERVERS {
        assert!(addr.starts_with("/ip4/"), "bad multiaddr: {addr}");
        assert!(addr.contains("/tcp/"), "missing tcp component: {addr}");
        assert!(
            addr.contains("/p2p/"),
            "missing p2p/peer_id component: {addr}"
        );
    }
    rec.metric("petals_count", PETALS_BOOTSTRAP_SERVERS.len());
    rec.metric("kwaai_count", KWAAI_BOOTSTRAP_SERVERS.len());
    rec.finish(true);
}

// ============================================================================
// NodeCapabilities encode / decode
// ============================================================================

#[test]
fn node_capabilities_encode_decode() {
    let mut rec = MetricsRecorder::start("unit::p2p::node_capabilities_encode_decode", "unit");
    let mut caps = NodeCapabilities::new("12D3KooWTest".to_string());
    caps.can_inference = true;
    caps.model_ids = vec!["llama3.2:3b".to_string(), "llama3.2:1b".to_string()];
    caps.compute_power = 38.4;
    caps.available_memory = 16384;

    let encoded = caps.encode().unwrap();
    let decoded = NodeCapabilities::decode(&encoded).unwrap();

    assert_eq!(decoded.peer_id, caps.peer_id);
    assert!(decoded.can_inference);
    assert_eq!(decoded.model_ids, caps.model_ids);
    assert!((decoded.compute_power - 38.4).abs() < 0.001);
    assert_eq!(decoded.available_memory, 16384);
    rec.metric("encoded_bytes", encoded.len());
    rec.finish(true);
}

#[test]
fn node_capabilities_default_values() {
    let rec = MetricsRecorder::start("unit::p2p::node_capabilities_defaults", "unit");
    let caps = NodeCapabilities::new("peer-id".to_string());
    assert!(!caps.can_inference);
    assert!(!caps.can_train);
    assert!(caps.model_ids.is_empty());
    assert_eq!(caps.compute_power, 0.0);
    rec.finish(true);
}

#[test]
fn node_capabilities_decode_bad_bytes_returns_error() {
    let rec = MetricsRecorder::start("unit::p2p::node_capabilities_decode_bad_bytes", "unit");
    let result = NodeCapabilities::decode(&[0xFF, 0x00, 0x01]);
    assert!(result.is_err());
    rec.finish(true);
}
