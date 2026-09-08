//! FIND_NODE answers from the peerstore (KWAAI PATCH in libp2p-kad): a peer
//! that is connected to a node but holds no k-bucket slot there is still
//! returned by that node, which is what go-libp2p's `handleFindPeer` did and
//! what made NATed peers findable through the Go bootstraps.
//!
//! Loopback swarms, fresh keys, ephemeral ports — see `swarm.rs`.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, Multiaddr, PeerId};

const SETTLE_TIMEOUT: Duration = Duration::from_secs(20);

fn spawn_with(config: NetworkConfig) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(config, keypair).expect("swarm should start");
    (handle, task, peer_id)
}

fn spawn() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    spawn_with(NetworkConfig::for_tests())
}

async fn eventually<T, F, Fut>(what: &str, mut f: F) -> T
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<T>>,
{
    let deadline = tokio::time::Instant::now() + SETTLE_TIMEOUT;
    loop {
        if let Some(v) = f().await {
            return v;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for: {what}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("a listen address", || async {
        handle
            .listen_addrs()
            .await
            .ok()?
            .into_iter()
            .find(|a| !a.to_string().contains("p2p-circuit"))
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

/// Connect `target` and `dialer` to `bootstrap`, then evict `target` from the
/// bootstrap's routing table — what a bucket full of foreign peers does to an
/// inbound peer.
async fn tabled_then_evicted(
    bootstrap: &NetworkHandle,
    bootstrap_id: PeerId,
    target: &NetworkHandle,
    target_id: PeerId,
    dialer: &NetworkHandle,
) {
    let bootstrap_addr = dialable_addr(bootstrap, bootstrap_id).await;
    target
        .connect_peer(&bootstrap_addr.to_string())
        .await
        .expect("target → bootstrap");
    dialer
        .connect_peer(&bootstrap_addr.to_string())
        .await
        .expect("dialer → bootstrap");
    eventually("the bootstrap to table the target", || async {
        bootstrap
            .routing_peers()
            .await
            .ok()?
            .contains(&target_id)
            .then_some(())
    })
    .await;
    assert!(bootstrap.drop_routing_entry(target_id).await.unwrap());
}

#[tokio::test]
async fn a_walk_finds_a_connected_peer_the_buckets_dropped() {
    let (bootstrap, _bt, bootstrap_id) = spawn();
    let (target, _tt, target_id) = spawn();
    let (dialer, _dt, _) = spawn();
    tabled_then_evicted(&bootstrap, bootstrap_id, &target, target_id, &dialer).await;

    // An ordinary Kademlia walk from the dialer — no new protocol, the same
    // request a 0.6.8 peer sends — comes back with the target's address.
    let addrs = tokio::time::timeout(SETTLE_TIMEOUT, dialer.dht_find_peer(target_id))
        .await
        .expect("walk should finish")
        .expect("the evicted peer is answered from the bootstrap's peerstore");
    assert!(
        addrs.iter().any(|a| a.to_string().contains("/tcp/")),
        "a dialable address: {addrs:?}"
    );
    let resolved = addrs[0]
        .clone()
        .with(libp2p::multiaddr::Protocol::P2p(target_id));
    let connected = dialer
        .connect_peer(&resolved.to_string())
        .await
        .expect("dial with the walk's answer");
    assert_eq!(connected, target_id);
}

/// The NATed case: the target's only way in is a reservation on the
/// bootstrap, and the answer carries a circuit through it.
#[tokio::test]
async fn the_answer_carries_a_circuit_through_the_bootstrap_it_reserved_on() {
    // A relay needs a declared external address for its hop server to hand
    // out circuits (see `dcutr.rs`).
    let listen = format!(
        "/ip4/127.0.0.1/tcp/{}",
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    );
    let (bootstrap, _bt, bootstrap_id) = spawn_with(NetworkConfig {
        listen_addrs: vec![listen.clone()],
        relay_server: true,
        external_addr: Some(listen),
        ..NetworkConfig::for_tests()
    });
    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
    let (target, _tt, target_id) = spawn_with(NetworkConfig {
        force_private: true,
        trusted_relays: vec![bootstrap_addr.to_string()],
        ..NetworkConfig::for_tests()
    });
    let (dialer, _dt, _) = spawn();

    // The target reserves on the bootstrap: its listen set gains a circuit.
    eventually("the target's reservation on the bootstrap", || async {
        target
            .listen_addrs()
            .await
            .ok()?
            .iter()
            .any(|a| a.to_string().contains("p2p-circuit"))
            .then_some(())
    })
    .await;
    dialer
        .connect_peer(&bootstrap_addr.to_string())
        .await
        .expect("dialer → bootstrap");
    eventually("the bootstrap to table the target", || async {
        bootstrap
            .routing_peers()
            .await
            .ok()?
            .contains(&target_id)
            .then_some(())
    })
    .await;
    assert!(bootstrap.drop_routing_entry(target_id).await.unwrap());

    let addrs = tokio::time::timeout(SETTLE_TIMEOUT, dialer.dht_find_peer(target_id))
        .await
        .expect("walk should finish")
        .expect("answered from the peerstore");
    let circuit = format!("/p2p/{bootstrap_id}/p2p-circuit");
    assert!(
        addrs.iter().any(|a| a.to_string().ends_with(&circuit)),
        "a circuit through the bootstrap: {addrs:?}"
    );
}
