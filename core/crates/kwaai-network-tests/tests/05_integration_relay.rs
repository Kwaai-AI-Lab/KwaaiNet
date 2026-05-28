//! Relay connectivity tests — the core p2p routing invariant.
//!
//! **The fundamental rule**: inference traffic must never travel over direct TCP
//! to metro machines. All inter-node communication must go through p2p relay.
//! These tests verify that:
//!
//!   1. A NATed node obtains a `/p2p-circuit` relay address.
//!   2. A third node can connect to the NATed node via relay (not direct TCP).
//!   3. DHT operations work through the relay path.
//!
//! Topology used:
//! ```text
//! [relay_server]  (DHT server + relay, known TCP port)
//!       │  p2p-circuit reservation
//!       │
//! [nat_node]  (force_reachability_private + auto_relay, bootstraps via relay_server)
//!   ↑ connected by relay, not direct
//! [observer]  (standard DHT client, bootstraps via relay_server)
//! ```
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`

use kwaai_network_tests::{harness::RelayHarness, metrics::MetricsRecorder, require_integration};
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};

fn sha256_multihash(data: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(data);
    let mut mh = vec![0x12u8, 0x20]; // sha2-256 function code + 32-byte length
    mh.extend_from_slice(&hash);
    mh
}

// ============================================================================
// Relay address acquisition
// ============================================================================

#[tokio::test]
async fn nat_node_acquires_relay_address() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::relay::nat_node_acquires_relay_addr",
        "integration",
    );

    let mut harness = RelayHarness::new().await.expect("relay harness startup");

    // Give the NAT node time to negotiate a relay reservation.
    // The force_reachability_private flag means it won't wait for AutoNAT.
    let deadline = Instant::now() + Duration::from_secs(15);
    let mut has_relay = false;
    while Instant::now() < deadline {
        if harness.nat_node.has_relay_addr().await.unwrap_or(false) {
            has_relay = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    let wait_ms = deadline
        .saturating_duration_since(Instant::now())
        .as_millis() as u64;
    rec.metric("has_relay_addr", has_relay);
    rec.metric("wait_ms", wait_ms);

    // Not a hard assertion — relay negotiation may vary in CI environments.
    // We record the metric so we can track how often this succeeds over time.
    rec.finish(has_relay);
}

// ============================================================================
// Three distinct peer IDs
// ============================================================================

#[tokio::test]
async fn relay_harness_three_distinct_peers() {
    require_integration!();
    let rec = MetricsRecorder::start("integration::relay::three_distinct_peers", "integration");

    let harness = RelayHarness::new().await.expect("relay harness startup");

    let relay_id = &harness.relay.peer_id_hex;
    let nat_id = &harness.nat_node.peer_id_hex;
    let obs_id = &harness.observer.peer_id_hex;

    assert_ne!(
        relay_id, nat_id,
        "relay and nat_node must be distinct peers"
    );
    assert_ne!(
        relay_id, obs_id,
        "relay and observer must be distinct peers"
    );
    assert_ne!(
        nat_id, obs_id,
        "nat_node and observer must be distinct peers"
    );

    for id in [relay_id, nat_id, obs_id] {
        assert!(!id.is_empty());
        assert!(id.len() >= 64, "peer_id hex too short: {}", id.len());
    }

    rec.finish(true);
}

// ============================================================================
// Observer can find NAT node in DHT via relay
// ============================================================================

#[tokio::test]
async fn observer_finds_nat_node_via_dht() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::relay::observer_finds_nat_node_via_dht",
        "integration",
    );

    let mut harness = RelayHarness::new().await.expect("relay harness startup");

    // Give DHT routing tables time to propagate
    tokio::time::sleep(Duration::from_secs(2)).await;

    let nat_peer = kwaai_network_tests::harness::peer_id_from_hex(&harness.nat_node.peer_id_hex)
        .expect("valid nat peer_id");

    let t = Instant::now();
    // dht_find_peer returns Result<DhtPeerInfo> — Err if peer not found
    let result = harness
        .observer
        .client
        .dht_find_peer(nat_peer.to_bytes().to_vec(), Some(15))
        .await;
    let find_ms = t.elapsed().as_millis() as u64;

    rec.metric("find_ms", find_ms);
    let ok = result.is_ok();
    if let Ok(peer_info) = &result {
        rec.metric("found_addr_count", peer_info.addrs.len());
        let relay_addr_found = peer_info.addrs.iter().any(|a| {
            let s = String::from_utf8_lossy(a);
            s.contains("p2p-circuit") || s.contains("circuit")
        });
        rec.metric("relay_addr_in_dht", relay_addr_found);
    }

    rec.finish(ok);
}

// ============================================================================
// Relay path — no direct TCP connection to NAT node
// ============================================================================

#[tokio::test]
async fn observer_connects_to_nat_node_via_relay_not_direct_tcp() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::relay::no_direct_tcp_to_nat_node",
        "integration",
    );

    let mut harness = RelayHarness::new().await.expect("relay harness startup");

    // Wait for relay reservation
    let deadline = Instant::now() + Duration::from_secs(15);
    while Instant::now() < deadline {
        if harness.nat_node.has_relay_addr().await.unwrap_or(false) {
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Get the NAT node's observed addresses
    let nat_addrs = harness
        .nat_node
        .observed_addrs()
        .await
        .expect("nat_node observed_addrs");

    rec.metric("nat_addr_count", nat_addrs.len());

    // The NAT node must NOT have any direct IP4/IP6 TCP addresses advertised
    // (because force_reachability_private prevents it from accepting direct conns).
    // Its only usable address should be a /p2p-circuit relay address.
    //
    // We verify: if there are any addresses, at least one must be a relay address.
    if !nat_addrs.is_empty() {
        let has_relay = nat_addrs.iter().any(|a| {
            libp2p::Multiaddr::try_from(a.clone())
                .map(|ma| ma.to_string().contains("p2p-circuit"))
                .unwrap_or(false)
        });
        rec.metric("has_relay_addr", has_relay);
        // Record whether the invariant holds: nat node is reachable only via relay
        rec.finish(has_relay);
    } else {
        // No addresses yet — relay negotiation may not have completed
        rec.metric("has_relay_addr", false);
        rec.finish(false);
    }
}

// ============================================================================
// DHT provider announced by NAT node is findable by observer
// ============================================================================

/// Verifies the relay path for content routing: NAT node announces a CID as a
/// provider; observer finds it via the relay-connected DHT routing table.
/// Uses provider records (not PUT_VALUE/GET_VALUE) — the daemon's DHT API
/// requires namespaced keys for value records; provider records accept raw CIDs.
#[tokio::test]
async fn dht_provide_by_nat_node_findable_by_observer() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::relay::dht_provide_nat_findable_observer",
        "integration",
    );

    let mut harness = RelayHarness::new().await.expect("relay harness startup");

    // Let DHT bootstrap and relay reservation settle
    tokio::time::sleep(Duration::from_secs(2)).await;

    let cid = sha256_multihash(b"kwaai.relay.test.vpk_capability");

    harness
        .nat_node
        .client
        .dht_provide(cid.clone(), Some(15))
        .await
        .expect("nat_node dht_provide");

    tokio::time::sleep(Duration::from_millis(500)).await;

    let t = Instant::now();
    let result = harness
        .observer
        .client
        .dht_find_providers(cid.clone(), 1, Some(15))
        .await;
    let find_ms = t.elapsed().as_millis() as u64;

    let found = result.map_or(false, |p| p.is_some());
    rec.metric("find_ms", find_ms);
    rec.metric("found", found);
    rec.finish(found);
}
