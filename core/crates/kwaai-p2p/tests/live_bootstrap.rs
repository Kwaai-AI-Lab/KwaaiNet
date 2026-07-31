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
