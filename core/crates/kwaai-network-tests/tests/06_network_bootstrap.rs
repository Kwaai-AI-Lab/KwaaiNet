//! Network-tier tests — connectivity to the live KwaaiNet bootstrap servers.
//!
//! These tests measure real network performance: bootstrap latency, peer count,
//! DHT lookup time, VPK node discovery. Results are stored in metrics.jsonl
//! so trends can be tracked across runs.
//!
//! Gate: `KWAAI_NETWORK_TESTS=1` (requires internet access and p2pd binary)
//!
//! # Key metrics tracked
//!
//! - `bootstrap_ms` — time until first peer is seen in the routing table
//! - `peer_count` — peers visible after 10s
//! - `dht_put_ms`, `dht_get_ms` — round-trip latency for DHT operations
//! - `vpk_nodes_found` — number of VPK-capable peers discovered
//! - `relay_addr_count` — relay addresses available for this node

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_network};
use kwaai_p2p::config::KWAAI_BOOTSTRAP_SERVERS;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

fn sha256_multihash(data: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(data);
    let mut mh = vec![0x12u8, 0x20];
    mh.extend_from_slice(&hash);
    mh
}

// ============================================================================
// Bootstrap connectivity
// ============================================================================

#[tokio::test]
async fn bootstrap_server_reachable() {
    require_network!();
    let mut rec =
        MetricsRecorder::start("network::bootstrap::server_reachable", "network");

    // Start a node that bootstraps from the live KwaaiNet servers
    let t = Instant::now();
    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];
    let node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node with live bootstrap");
    let startup_ms = t.elapsed().as_millis() as u64;

    rec.metric("startup_ms", startup_ms);
    rec.metric("bootstrap_addr", bootstrap_addr);
    assert!(!node.peer_id_hex.is_empty());
    rec.finish(true);
}

// ============================================================================
// Peer count after settling
// ============================================================================

#[tokio::test]
async fn peer_count_after_settling() {
    require_network!();
    let mut rec = MetricsRecorder::start("network::bootstrap::peer_count", "network");

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];
    let mut node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node start");

    // Allow DHT routing table to fill
    tokio::time::sleep(Duration::from_secs(10)).await;

    let peers = node
        .client
        .list_peers()
        .await
        .expect("list_peers");
    let peer_count = peers.len();
    rec.metric("peer_count", peer_count);
    rec.metric("node_peer_id", node.peer_id_hex.as_str());

    // A healthy network should have at least 2 peers after 10s
    let ok = peer_count >= 2;
    if !ok {
        eprintln!("[network test] only {peer_count} peers after 10s — network may be degraded");
    }
    rec.finish(ok);
}

// ============================================================================
// DHT latency — provide and find-providers on the live network
// ============================================================================

/// Measures DHT content-routing round-trip on the live KwaaiNet network.
/// Uses provide + find_providers (the daemon's actual DHT API for arbitrary content).
/// PUT_VALUE/GET_VALUE require namespaced keys (/pk/, /ipns/) — not usable for
/// arbitrary KwaaiNet keys with the daemon's default validator configuration.
#[tokio::test]
async fn dht_roundtrip_latency() {
    require_network!();
    let mut rec = MetricsRecorder::start("network::dht::roundtrip_latency", "network");

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];
    let mut node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node start");

    // Wait for routing table
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Use a CID unique to this test runner to avoid collisions
    let cid_input = format!("kwaai.test.latency.{}", &node.peer_id_hex[..16]);
    let cid = sha256_multihash(cid_input.as_bytes());

    let t_provide = Instant::now();
    node.client
        .dht_provide(cid.clone(), Some(30))
        .await
        .expect("dht_provide");
    let provide_ms = t_provide.elapsed().as_millis() as u64;

    let t_find = Instant::now();
    let result = node
        .client
        .dht_find_providers(cid.clone(), 1, Some(30))
        .await;
    let find_ms = t_find.elapsed().as_millis() as u64;

    let found = result.map_or(false, |p| p.is_some());
    rec.metric("provide_ms", provide_ms);
    rec.metric("find_ms", find_ms);
    rec.metric("found", found);
    rec.finish(found);
}

// ============================================================================
// VPK node discovery
// ============================================================================

/// Checks whether any VPK nodes have announced themselves on the live network
/// using provider records for the well-known VPK service CID.
/// Not a hard failure — reports availability as a health metric.
#[tokio::test]
async fn vpk_nodes_discoverable() {
    require_network!();
    let mut rec = MetricsRecorder::start("network::dht::vpk_nodes_discoverable", "network");

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];
    let mut node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node start");

    // Allow the routing table to populate
    tokio::time::sleep(Duration::from_secs(8)).await;

    // VPK nodes announce themselves as providers for this well-known CID
    let vpk_cid = sha256_multihash(b"_kwaai.vpk.nodes");

    let t = Instant::now();
    let result = node
        .client
        .dht_find_providers(vpk_cid.clone(), 5, Some(20))
        .await;
    let lookup_ms = t.elapsed().as_millis() as u64;

    let found = result.map_or(false, |p| p.is_some());
    rec.metric("lookup_ms", lookup_ms);
    rec.metric("vpk_providers_found", found);
    rec.finish(true); // Not a failure if no VPK nodes online — just report
}

// ============================================================================
// Relay address availability on the live network
// ============================================================================

#[tokio::test]
async fn relay_address_available_on_live_network() {
    require_network!();
    let mut rec = MetricsRecorder::start(
        "network::relay::address_available_on_live_network",
        "network",
    );

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];

    // Start as a relay-seeking node (auto_relay = true)
    // Note: we use new_dht_client here; in production the node would have auto_relay.
    // For the network test we check that the bootstrap server's relay is discoverable.
    let mut node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node start");

    tokio::time::sleep(Duration::from_secs(8)).await;

    // Check if any of our observed addresses are relay addresses
    let has_relay = node.has_relay_addr().await.unwrap_or(false);
    let addrs = node.observed_addrs().await.unwrap_or_default();
    rec.metric("addr_count", addrs.len());
    rec.metric("has_relay_addr", has_relay);

    // Not a hard failure — relay availability depends on network state
    rec.finish(true);
}

// ============================================================================
// Second bootstrap server reachable
// ============================================================================

#[tokio::test]
async fn bootstrap_server_2_reachable() {
    require_network!();
    let mut rec =
        MetricsRecorder::start("network::bootstrap::server_2_reachable", "network");

    if KWAAI_BOOTSTRAP_SERVERS.len() < 2 {
        rec.metric("skipped", true);
        rec.finish(true);
        return;
    }

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[1];
    let t = Instant::now();
    let node = TestNode::new_dht_client(bootstrap_addr)
        .await
        .expect("node with bootstrap-2");
    let startup_ms = t.elapsed().as_millis() as u64;

    rec.metric("startup_ms", startup_ms);
    rec.metric("bootstrap_addr", bootstrap_addr);
    assert!(!node.peer_id_hex.is_empty());
    rec.finish(true);
}

// ============================================================================
// Uptime probe: repeated connectivity checks
// ============================================================================

/// This test runs 3 rapid connectivity probes and records the success rate.
/// Over time, the pass rate in metrics.jsonl shows network uptime trends.
#[tokio::test]
async fn connectivity_probe_3x() {
    require_network!();
    let mut rec = MetricsRecorder::start("network::uptime::connectivity_probe_3x", "network");

    let bootstrap_addr = KWAAI_BOOTSTRAP_SERVERS[0];
    let mut successes = 0u32;

    for i in 0..3 {
        match TestNode::new_dht_client(bootstrap_addr).await {
            Ok(n) if !n.peer_id_hex.is_empty() => {
                successes += 1;
                eprintln!("  probe {i}: OK (peer {}...)", &n.peer_id_hex[..12]);
            }
            Ok(_) => {
                eprintln!("  probe {i}: FAIL (empty peer_id)");
            }
            Err(e) => {
                eprintln!("  probe {i}: FAIL ({e})");
            }
        }
        // Brief pause between probes to avoid hammering bootstrap
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let success_rate = (successes as f64) / 3.0;
    rec.metric("successes", successes);
    rec.metric("success_rate", success_rate);
    rec.finish(successes == 3);
}
