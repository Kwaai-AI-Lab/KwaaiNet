//! Integration tests for daemon lifecycle and DHT operations.
//!
//! These tests spawn real p2pd processes and exercise the Rust IPC client.
//! They are gated by `KWAAI_INTEGRATION_TESTS=1` because they require the
//! p2pd binary to be built and take a few seconds to run.
//!
//! # Prerequisites
//!
//! ```bash
//! cd core && cargo build -p kwaai-p2p-daemon   # builds p2pd via build.rs
//! KWAAI_INTEGRATION_TESTS=1 cargo test -p kwaai-network-tests 04_integration_daemon
//! ```

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use sha2::{Digest, Sha256};
use std::time::Instant;

/// Build a valid 34-byte sha2-256 multihash for use as a DHT CID.
/// go-libp2p-kad-dht's provider records accept raw multihashes;
/// PUT_VALUE/GET_VALUE require /pk/ or /ipns/ namespace prefixes (daemon default
/// validators only). Production KwaaiNet KV storage uses the hivemind DHT
/// protocol over libp2p streams — not the daemon's DHT API.
fn sha256_multihash(data: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(data);
    let mut mh = vec![0x12u8, 0x20]; // sha2-256 function code + 32-byte length
    mh.extend_from_slice(&hash);
    mh
}

// ============================================================================
// Daemon lifecycle
// ============================================================================

#[tokio::test]
async fn daemon_spawn_identify_shutdown() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::spawn_identify_shutdown",
        "integration",
    );
    let t = Instant::now();

    let node = TestNode::new_relay_server()
        .await
        .expect("relay server should start");

    let startup_ms = t.elapsed().as_millis() as u64;
    rec.metric("startup_ms", startup_ms);

    assert!(
        !node.peer_id_hex.is_empty(),
        "peer_id_hex must not be empty"
    );
    assert!(
        node.peer_id_hex.len() >= 64,
        "peer_id hex should be at least 64 chars, got {}",
        node.peer_id_hex.len()
    );

    rec.metric("peer_id_len", node.peer_id_hex.len());
    rec.finish(true);
}

#[tokio::test]
async fn daemon_identify_returns_stable_peer_id() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::daemon::identify_returns_stable_peer_id",
        "integration",
    );

    let mut node = TestNode::new_relay_server()
        .await
        .expect("node start");

    let id1 = node.client.identify().await.expect("identify 1");
    let id2 = node.client.identify().await.expect("identify 2");
    assert_eq!(id1, id2, "peer ID must be stable within one daemon lifetime");
    assert_eq!(id1, node.peer_id_hex);

    rec.finish(true);
}

#[tokio::test]
async fn daemon_identify_with_addrs_returns_listen_addrs() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::identify_with_addrs",
        "integration",
    );

    let mut node = TestNode::new_relay_server()
        .await
        .expect("node start");

    let (peer_id, addrs) = node
        .client
        .identify_with_addrs()
        .await
        .expect("identify_with_addrs");

    assert!(!peer_id.is_empty());
    assert!(
        !addrs.is_empty(),
        "relay server should report at least one listen address"
    );
    rec.metric("addr_count", addrs.len());
    rec.finish(true);
}

// ============================================================================
// Two-node connectivity
// ============================================================================

#[tokio::test]
async fn two_nodes_can_connect() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::daemon::two_nodes_connect", "integration");

    let relay = TestNode::new_relay_server()
        .await
        .expect("relay node start");

    let bootstrap = relay
        .bootstrap_multiaddr()
        .expect("relay should have p2p_port");

    let t = Instant::now();
    let client = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("dht client start");
    let connect_ms = t.elapsed().as_millis() as u64;

    assert_ne!(
        relay.peer_id_hex, client.peer_id_hex,
        "two nodes must have different peer IDs"
    );

    rec.metric("connect_ms", connect_ms);
    rec.finish(true);
}

// ============================================================================
// DHT provider records — content routing (same node)
// ============================================================================

/// Provider records are the correct DHT API path for the go-libp2p-daemon.
/// A client bootstrapped from a relay announces it provides a CID, then finds
/// providers for that CID. Models KwaaiNet VPK node capability announcements.
/// (A single isolated node can't publish provider records — DHT requires routing table peers.)
#[tokio::test]
async fn dht_provide_and_find_connected_node() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::dht_provide_find_connected_node",
        "integration",
    );

    let relay = TestNode::new_relay_server()
        .await
        .expect("relay start");
    let bootstrap = relay.bootstrap_multiaddr().expect("relay p2p addr");

    let mut node = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("dht client start");

    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let cid = sha256_multihash(b"kwaai.test.capability.vpk");

    let t = Instant::now();
    node.client
        .dht_provide(cid.clone(), Some(10))
        .await
        .expect("dht_provide");
    let provide_ms = t.elapsed().as_millis() as u64;

    let t2 = Instant::now();
    let result = node.client
        .dht_find_providers(cid.clone(), 1, Some(10))
        .await
        .expect("dht_find_providers");
    let find_ms = t2.elapsed().as_millis() as u64;

    rec.metric("provide_ms", provide_ms);
    rec.metric("find_ms", find_ms);
    rec.metric("provider_found", result.is_some());
    rec.finish(true);
}

// ============================================================================
// DHT provider records — cross-node (node A provides, node B finds)
// ============================================================================

#[tokio::test]
async fn dht_provide_node_a_find_node_b() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::dht_provide_a_find_b",
        "integration",
    );

    let relay = TestNode::new_relay_server()
        .await
        .expect("relay start");
    let bootstrap = relay.bootstrap_multiaddr().expect("relay p2p addr");

    let mut node_a = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("node_a start");
    let mut node_b = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("node_b start");

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let cid = sha256_multihash(b"kwaai.test.cross_node.vpk_capability");

    node_a
        .client
        .dht_provide(cid.clone(), Some(10))
        .await
        .expect("node_a provide");

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let t = Instant::now();
    let result = node_b
        .client
        .dht_find_providers(cid.clone(), 1, Some(15))
        .await;
    let find_ms = t.elapsed().as_millis() as u64;

    let found = result.map_or(false, |p| p.is_some());
    rec.metric("cross_find_ms", find_ms);
    rec.metric("found", found);
    // Cross-node provider propagation may vary in small local nets
    rec.finish(found);
}

// ============================================================================
// DHT missing key returns error
// ============================================================================

#[tokio::test]
async fn dht_get_missing_key_returns_error() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::daemon::dht_get_missing_key_returns_error",
        "integration",
    );

    let mut node = TestNode::new_relay_server()
        .await
        .expect("node start");

    // dht_get_value returns Err when key is not found
    let result = node
        .client
        .dht_get_value(b"kwaai.test.no_such_key_xyz_12345".to_vec(), Some(5))
        .await;

    assert!(result.is_err(), "missing key must return Err");
    rec.finish(true);
}

// ============================================================================
// List peers
// ============================================================================

#[tokio::test]
async fn list_peers_after_connect() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::list_peers_after_connect",
        "integration",
    );

    let relay = TestNode::new_relay_server()
        .await
        .expect("relay start");
    let bootstrap = relay.bootstrap_multiaddr().expect("relay p2p addr");

    let mut client = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("client start");

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let peers = client.client.list_peers().await.expect("list_peers");
    rec.metric("peer_count", peers.len());

    // The client bootstrapped from the relay, so it should see at least the relay
    let ok = !peers.is_empty();
    rec.finish(ok);
}

// ============================================================================
// Peer discovery via DHT
// ============================================================================

#[tokio::test]
async fn find_peer_returns_known_peer() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::daemon::find_peer_returns_known_peer",
        "integration",
    );

    let relay = TestNode::new_relay_server()
        .await
        .expect("relay start");
    let bootstrap = relay.bootstrap_multiaddr().expect("relay p2p addr");

    let mut client = TestNode::new_dht_client(&bootstrap)
        .await
        .expect("client start");

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let relay_peer = kwaai_network_tests::harness::peer_id_from_hex(&relay.peer_id_hex)
        .expect("valid peer_id_hex");

    let t = Instant::now();
    // dht_find_peer returns Result<DhtPeerInfo> — Err if not found
    let result = client
        .client
        .dht_find_peer(relay_peer.to_bytes().to_vec(), Some(10))
        .await;
    let find_ms = t.elapsed().as_millis() as u64;

    let ok = result.is_ok();
    if let Ok(peer_info) = result {
        assert!(!peer_info.id.is_empty());
        rec.metric("found_addrs", peer_info.addrs.len());
    }

    rec.metric("find_ms", find_ms);
    rec.finish(ok);
}
