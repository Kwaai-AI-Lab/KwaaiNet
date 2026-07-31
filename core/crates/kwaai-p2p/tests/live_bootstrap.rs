//! Live smoke test against the public KwaaiNet bootstrap servers.
//!
//! `#[ignore]` because it needs internet; run it deliberately:
//!
//! ```sh
//! cargo test -p kwaai-p2p --test live_bootstrap -- --ignored --nocapture
//! ```
//!
//! What it proves that no in-process test can: the noise handshake against the
//! bootstraps' **RSA-2048** identities (`Qm…` peer IDs) works from rust-libp2p.
//! That path has never run in production here — the Go daemon did the dialing —
//! so it is the single biggest unknown in the migration.
//!
//! Strictly read-only: generate an ephemeral key, connect, identify, disconnect.
//! No announce, no DHT writes, no on-disk identity.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkService, KWAAI_BOOTSTRAP_SERVERS_DNS};
use libp2p::identity::Keypair;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
#[ignore = "requires internet access to the live KwaaiNet bootstraps"]
async fn connects_and_identifies_against_live_bootstraps() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("kwaai_p2p=debug")
        .try_init();

    // Ephemeral identity — never touches ~/.kwaainet.
    let keypair = Keypair::generate_ed25519();
    let local = keypair.public().to_peer_id();
    println!("local (ephemeral) peer id: {local}");

    let config = NetworkConfig {
        listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
        ..NetworkConfig::default()
    };
    let (handle, _task) = NetworkService::spawn(config, keypair).expect("swarm should start");

    let mut connected = Vec::new();
    for addr in KWAAI_BOOTSTRAP_SERVERS_DNS {
        println!("dialing {addr}");
        match tokio::time::timeout(CONNECT_TIMEOUT, handle.connect_peer(addr)).await {
            Ok(Ok(peer)) => {
                println!("  connected: {peer}");
                assert!(
                    peer.to_base58().starts_with("Qm"),
                    "bootstrap peer ids are RSA multihashes: {peer}"
                );
                connected.push(peer);
            }
            // A handshake failure here is the critical finding the migration
            // plan flags — surface the exact error text, do not summarize it.
            Ok(Err(e)) => panic!("DIAL/HANDSHAKE FAILED for {addr}\n  error: {e}"),
            Err(_) => panic!("dial to {addr} timed out after {CONNECT_TIMEOUT:?}"),
        }
    }

    assert_eq!(
        connected.len(),
        KWAAI_BOOTSTRAP_SERVERS_DNS.len(),
        "expected to connect to every bootstrap"
    );

    // Wait for identify to complete on *both* connections: each bootstrap
    // reports its own view of our source port, so a second distinct observed
    // address (or a second confirmation) means both exchanges landed.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
    loop {
        let peers = handle.list_peers().await.expect("list_peers");
        let observed = handle.observed_addrs().await.expect("observed_addrs");
        let confirmations: usize = observed.iter().map(|(_, n)| n).sum();
        if confirmations >= connected.len() || tokio::time::Instant::now() >= deadline {
            println!("\n--- connected peers ---");
            for p in &peers {
                println!("  {} via {} ({})", p.peer_id, p.addr, p.direction.as_str());
            }
            println!("\n--- observed addresses (identify) ---");
            for (addr, confirmations) in &observed {
                println!("  {addr}  (confirmed by {confirmations} peer(s))");
            }
            assert!(
                !observed.is_empty(),
                "identify never delivered an observed address — the exchange did not complete"
            );
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Read-only teardown.
    for peer in connected {
        handle.disconnect_peer(peer).await.expect("disconnect");
        println!("disconnected from {peer}");
    }
    handle.shutdown().await.expect("shutdown");
}

/// Protocols the AutoNAT / relay design needs to know about, by exact wire name.
const AUTONAT_PROTO: &str = "/libp2p/autonat/1.0.0";
const RELAY_HOP_PROTO: &str = "/libp2p/circuit/relay/0.2.0/hop";
const RELAY_STOP_PROTO: &str = "/libp2p/circuit/relay/0.2.0/stop";
const DCUTR_PROTO: &str = "/libp2p/dcutr";

/// Snapshot each bootstrap's identify protocol list.
///
/// The NAT-traversal design has exactly one empirical unknown: whether the
/// bootstraps (which run hivemind's own go-libp2p daemon, not our `p2pd` flags)
/// answer AutoNAT dialbacks. AutoNAT's `use_connected` makes *any* connected
/// peer speaking `/libp2p/autonat/1.0.0` a probe target, so the answer decides
/// whether a native node gets a real reachability verdict from the production
/// network or falls back to the identify-consensus rule.
///
/// Nothing here writes: dial, read the identify response, disconnect. No DHT
/// store, no announce, no on-disk identity. The assertions are deliberately
/// weak — this is a **measurement**, and a bootstrap that drops a protocol is a
/// finding to record, not a test failure.
///
/// # Measured 2026-07-31
///
/// Both bootstraps advertise **exactly eight** protocols, identically:
///
/// ```text
/// /ipfs/id/1.0.0   /ipfs/id/push/1.0.0   /ipfs/kad/1.0.0   /ipfs/ping/1.0.0
/// /libp2p/autonat/1.0.0
/// /libp2p/circuit/relay/0.2.0/hop   /libp2p/circuit/relay/0.2.0/stop
/// /libp2p/dcutr
/// ```
///
/// So `/libp2p/autonat/1.0.0` **is** present: the design's one empirical
/// unknown resolves the favourable way. A native node connected to the
/// production bootstraps has two AutoNAT servers from its first dial, which
/// means the identify-consensus rule is a genuine fallback rather than the
/// primary reachability source. The bootstraps also offer relay **hop**, so
/// identify-driven relay discovery finds candidates immediately — though a hop
/// advertisement is not a promise, and these particular ones have a documented
/// `RESERVATION_REFUSED` history, which is exactly why refusal must rotate
/// rather than fail.
#[tokio::test]
#[ignore = "requires internet access to the live KwaaiNet bootstraps"]
async fn bootstraps_protocol_list_snapshot() {
    let keypair = Keypair::generate_ed25519();
    println!(
        "local (ephemeral) peer id: {}",
        keypair.public().to_peer_id()
    );

    let config = NetworkConfig {
        listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
        ..NetworkConfig::default()
    };
    let (handle, _task) = NetworkService::spawn(config, keypair).expect("swarm should start");

    let mut any_autonat = false;
    let mut any_hop = false;

    for addr in KWAAI_BOOTSTRAP_SERVERS_DNS {
        let peer = match tokio::time::timeout(CONNECT_TIMEOUT, handle.connect_peer(addr)).await {
            Ok(Ok(peer)) => peer,
            Ok(Err(e)) => panic!("DIAL/HANDSHAKE FAILED for {addr}\n  error: {e}"),
            Err(_) => panic!("dial to {addr} timed out after {CONNECT_TIMEOUT:?}"),
        };

        // identify runs immediately after the handshake; give it a moment.
        let deadline = tokio::time::Instant::now() + Duration::from_secs(20);
        let protocols = loop {
            match handle.peer_protocols(peer).await.expect("peer_protocols") {
                Some(protocols) => break protocols,
                None if tokio::time::Instant::now() >= deadline => {
                    panic!("identify never completed for {addr}")
                }
                None => tokio::time::sleep(Duration::from_millis(200)).await,
            }
        };

        println!(
            "\n=== {addr}\n    peer id: {peer}\n    {} protocol(s):",
            protocols.len()
        );
        for proto in &protocols {
            println!("      {proto}");
        }
        let has = |name: &str| protocols.iter().any(|p| p == name);
        println!(
            "    autonat={}  relay-hop={}  relay-stop={}  dcutr={}",
            has(AUTONAT_PROTO),
            has(RELAY_HOP_PROTO),
            has(RELAY_STOP_PROTO),
            has(DCUTR_PROTO),
        );
        any_autonat |= has(AUTONAT_PROTO);
        any_hop |= has(RELAY_HOP_PROTO);

        handle.disconnect_peer(peer).await.expect("disconnect");
    }

    println!(
        "\n--- verdict ---\n  at least one bootstrap speaks AutoNAT: {any_autonat}\n  \
         at least one bootstrap offers relay hop: {any_hop}"
    );
    if !any_autonat {
        println!(
            "  → no AutoNAT server among the bootstraps: a native node's status stays \
             Unknown until some other peer answers, and the identify-consensus fallback \
             is what actually decides reachability on the production network."
        );
    }

    handle.shutdown().await.expect("shutdown");
}
