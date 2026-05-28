//! Unit tests for kwaai-p2p: NetworkConfig, ServerInfo, Hivemind framing,
//! NodeCapabilities. No network, no daemon required.

use kwaai_network_tests::metrics::MetricsRecorder;
use kwaai_p2p::{
    NodeCapabilities, PETALS_BOOTSTRAP_SERVERS,
    config::{NetworkConfig, KWAAI_BOOTSTRAP_SERVERS},
    hivemind::{
        ExpertUID, ServerInfo, decode_message, encode_error, encode_message,
    },
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
        assert!(addr.contains("/p2p/"), "missing p2p/peer_id component: {addr}");
    }
    for addr in KWAAI_BOOTSTRAP_SERVERS {
        assert!(addr.starts_with("/ip4/"), "bad multiaddr: {addr}");
        assert!(addr.contains("/tcp/"), "missing tcp component: {addr}");
        assert!(addr.contains("/p2p/"), "missing p2p/peer_id component: {addr}");
    }
    rec.metric("petals_count", PETALS_BOOTSTRAP_SERVERS.len());
    rec.metric("kwaai_count", KWAAI_BOOTSTRAP_SERVERS.len());
    rec.finish(true);
}

// ============================================================================
// ServerInfo — msgpack roundtrip and Hivemind compatibility
// ============================================================================

#[test]
fn server_info_default_roundtrip() {
    let mut rec = MetricsRecorder::start("unit::p2p::server_info_default_roundtrip", "unit");
    let info = ServerInfo::default();
    let bytes = info.to_msgpack().unwrap();
    let decoded = ServerInfo::from_msgpack(&bytes).unwrap();

    assert_eq!(decoded.state, "online");
    assert_eq!(decoded.start_block, 0);
    assert_eq!(decoded.end_block, 1);
    assert_eq!(decoded.torch_dtype, "float16");
    assert_eq!(decoded.quant_type, "none");
    rec.metric("msgpack_bytes", bytes.len());
    rec.finish(true);
}

#[test]
fn server_info_builder_methods() {
    let rec = MetricsRecorder::start("unit::p2p::server_info_builder_methods", "unit");
    let info = ServerInfo::new("kwaai-test-node")
        .with_span(4, 16)
        .with_throughput(32.5)
        .with_cache_tokens(65536)
        .with_dtype("bfloat16")
        .with_relay(true);

    assert_eq!(info.public_name.as_deref(), Some("kwaai-test-node"));
    assert_eq!(info.start_block, 4);
    assert_eq!(info.end_block, 16);
    assert_eq!(info.throughput, 32.5);
    assert_eq!(info.cache_tokens_left, 65536);
    assert_eq!(info.torch_dtype, "bfloat16");
    assert!(info.using_relay);
    assert_eq!(info.spans, Some(vec![(4, 16)]));
    rec.finish(true);
}

#[test]
fn server_info_msgpack_roundtrip_with_extras() {
    let mut rec = MetricsRecorder::start("unit::p2p::server_info_msgpack_roundtrip_extras", "unit");
    let original = ServerInfo::new("node-a")
        .with_span(0, 8)
        .with_throughput(15.0)
        .with_cache_tokens(1024);

    let bytes = original.to_msgpack().unwrap();
    let decoded = ServerInfo::from_msgpack(&bytes).unwrap();

    assert_eq!(decoded.public_name, original.public_name);
    assert_eq!(decoded.start_block, original.start_block);
    assert_eq!(decoded.end_block, original.end_block);
    assert!((decoded.throughput - original.throughput).abs() < 0.001);
    assert_eq!(decoded.cache_tokens_left, original.cache_tokens_left);
    assert_eq!(decoded.version, original.version);

    rec.metric("msgpack_bytes", bytes.len());
    rec.finish(true);
}

#[test]
fn server_info_to_expert_info() {
    let rec = MetricsRecorder::start("unit::p2p::server_info_to_expert_info", "unit");
    let info = ServerInfo::new("kwaai-node").with_span(0, 4);
    let expert_info = info.to_expert_info().unwrap();
    assert!(!expert_info.serialized_info.is_empty());
    // Verify roundtrip through expert_info
    let decoded = ServerInfo::from_msgpack(&expert_info.serialized_info).unwrap();
    assert_eq!(decoded.start_block, 0);
    assert_eq!(decoded.end_block, 4);
    rec.finish(true);
}

// ============================================================================
// Hivemind message framing
// ============================================================================

#[test]
fn framing_encode_decode_message() {
    let mut rec = MetricsRecorder::start("unit::p2p::framing_encode_decode_message", "unit");
    use prost::Message as _;
    let uid = ExpertUID { uid: "llama3.2:3b.block.0".to_string() };

    let framed = encode_message(&uid);
    let (is_error, payload) = decode_message(&framed).expect("decode should succeed");
    assert!(!is_error);

    let decoded = ExpertUID::decode(payload).unwrap();
    assert_eq!(decoded.uid, uid.uid);
    rec.metric("framed_bytes", framed.len());
    rec.finish(true);
}

#[test]
fn framing_encode_decode_error() {
    let rec = MetricsRecorder::start("unit::p2p::framing_encode_decode_error", "unit");
    let framed = encode_error("inference backend unavailable");
    let (is_error, payload) = decode_message(&framed).expect("decode should succeed");
    assert!(is_error);
    assert_eq!(std::str::from_utf8(payload).unwrap(), "inference backend unavailable");
    rec.finish(true);
}

#[test]
fn framing_length_prefix_matches_body() {
    let rec = MetricsRecorder::start("unit::p2p::framing_length_prefix_matches_body", "unit");
    let uid = ExpertUID { uid: "test".to_string() };
    let framed = encode_message(&uid);

    // First 8 bytes declare the body length (marker + protobuf)
    let declared = u64::from_be_bytes(framed[0..8].try_into().unwrap()) as usize;
    let actual = framed.len() - 8;
    assert_eq!(declared, actual);
    rec.finish(true);
}

#[test]
fn framing_decode_too_short_returns_none() {
    let rec = MetricsRecorder::start("unit::p2p::framing_decode_too_short_returns_none", "unit");
    assert!(decode_message(&[]).is_none());
    assert!(decode_message(&[0u8; 4]).is_none());
    assert!(decode_message(&[0u8; 8]).is_none()); // exactly 8 bytes — no marker
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
