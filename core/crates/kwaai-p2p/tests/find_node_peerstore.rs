//! FIND_NODE answers from the peerstore (KWAAI PATCH in libp2p-kad): a peer
//! that is connected to a node but holds no k-bucket slot there is still
//! returned by that node, which is what go-libp2p's `handleFindPeer` did and
//! what made NATed peers findable through the Go bootstraps.
//!
//! Loopback swarms, fresh keys, ephemeral ports — see `swarm.rs`.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, Multiaddr, PeerId, StreamProtocol};

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

struct Rig {
    bootstrap: NetworkHandle,
    bootstrap_id: PeerId,
    relay_id: PeerId,
    relay_listen: String,
    target_id: PeerId,
    dialer: NetworkHandle,
    _handles: Vec<NetworkHandle>,
    _tasks: Vec<tokio::task::JoinHandle<()>>,
}

/// Where the NATed target holds its reservation.
enum Relay {
    /// On the bootstrap itself — the answer must synthesise the circuit.
    TheBootstrap,
    /// On a third node — the circuit reaches the bootstrap only via identify.
    Elsewhere,
}

/// A NATed target speaking `kad` connected to a bootstrap, and a dialer that
/// knows only the bootstrap. Loopback listeners are never vouched for, so
/// the target's circuit is the one address the walk's answer can carry.
async fn natted_target(relay: Relay, kad: Vec<StreamProtocol>) -> Rig {
    // A relay needs a declared external address for its hop server to hand
    // out circuits (see `dcutr.rs`).
    let relay_listen = format!(
        "/ip4/127.0.0.1/tcp/{}",
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    );
    let (relay_handle, rt, relay_id) = spawn_with(NetworkConfig {
        listen_addrs: vec![relay_listen.clone()],
        relay_server: true,
        external_addr: Some(relay_listen.clone()),
        ..NetworkConfig::for_tests()
    });
    let relay_addr = dialable_addr(&relay_handle, relay_id).await;
    let (target, tt, target_id) = spawn_with(NetworkConfig {
        force_private: true,
        trusted_relays: vec![relay_addr.to_string()],
        kad_protocols: kad,
        ..NetworkConfig::for_tests()
    });
    let (dialer, dt, _) = spawn();
    let mut handles = vec![target];
    // `handles[0]` is the target throughout.
    let mut tasks = vec![rt, tt, dt];

    // The target reserves on the relay: its listen set gains a circuit.
    eventually("the target's reservation on the relay", || async {
        handles[0]
            .listen_addrs()
            .await
            .ok()?
            .iter()
            .any(|a| a.to_string().contains("p2p-circuit"))
            .then_some(())
    })
    .await;

    let (bootstrap, bootstrap_id) = match relay {
        Relay::TheBootstrap => (relay_handle, relay_id),
        Relay::Elsewhere => {
            let (bootstrap, bt, bootstrap_id) = spawn();
            tasks.push(bt);
            let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
            handles[0]
                .connect_peer(&bootstrap_addr.to_string())
                .await
                .expect("target → bootstrap");
            handles.push(relay_handle);
            (bootstrap, bootstrap_id)
        }
    };
    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
    dialer
        .connect_peer(&bootstrap_addr.to_string())
        .await
        .expect("dialer → bootstrap");
    Rig {
        bootstrap,
        bootstrap_id,
        relay_id,
        relay_listen,
        target_id,
        dialer,
        _handles: handles,
        _tasks: tasks,
    }
}

impl Rig {
    /// Evict the target from the bootstrap's routing table — what a bucket
    /// full of foreign peers does to an inbound peer.
    async fn evict_target(&self) {
        eventually("the bootstrap to table the target", || async {
            self.bootstrap
                .routing_peers()
                .await
                .ok()?
                .contains(&self.target_id)
                .then_some(())
        })
        .await;
        assert!(self
            .bootstrap
            .drop_routing_entry(self.target_id)
            .await
            .unwrap());
    }
}

/// An ordinary Kademlia walk from the dialer — no new protocol, the same
/// request a 0.6.8 peer sends.
async fn walk_for(rig: &Rig) -> Vec<Multiaddr> {
    tokio::time::timeout(SETTLE_TIMEOUT, rig.dialer.dht_find_peer(rig.target_id))
        .await
        .expect("walk should finish")
        .expect("the evicted peer is answered from the bootstrap's peerstore")
}

/// The identify branch: what the target told the bootstrap it listens on is
/// vouched for although no bucket holds the target any more.
#[tokio::test]
async fn a_walk_finds_a_connected_peer_the_buckets_dropped() {
    let rig = natted_target(Relay::Elsewhere, kwaai_p2p::config::kad_protocols()).await;
    rig.evict_target().await;
    let addrs = walk_for(&rig).await;
    let circuit = format!("{}/p2p/{}/p2p-circuit", rig.relay_listen, rig.relay_id);
    assert!(
        addrs.iter().any(|a| a.to_string() == circuit),
        "the address the bootstrap vouched for: {addrs:?}"
    );
    let resolved = format!("{circuit}/p2p/{}", rig.target_id);
    let connected = rig
        .dialer
        .connect_peer(&resolved)
        .await
        .expect("dial with the walk's answer");
    assert_eq!(connected, rig.target_id);
}

/// The reservation branch: the answer carries a circuit through the
/// bootstrap, synthesised from its own reservation state. The circuit the
/// target's identify reports is dropped from the feed (it would outlive the
/// reservation), so only that branch can produce this address.
#[tokio::test]
async fn the_answer_carries_a_circuit_through_the_bootstrap_it_reserved_on() {
    let rig = natted_target(Relay::TheBootstrap, kwaai_p2p::config::kad_protocols()).await;
    rig.evict_target().await;
    let addrs = walk_for(&rig).await;
    let circuit = format!("{}/p2p/{}/p2p-circuit", rig.relay_listen, rig.bootstrap_id);
    assert!(
        addrs.iter().any(|a| a.to_string() == circuit),
        "a circuit on the bootstrap's external address: {addrs:?}"
    );
}

/// A peer that does not speak our kad is never vouched for, reachable or
/// not: the peerstore is what FIND_NODE hands out, and a bootstrap answering
/// for foreign peers is how it re-enters the public DHT's serving path.
#[tokio::test]
async fn a_peer_that_does_not_speak_our_kad_is_not_answered() {
    let rig = natted_target(
        Relay::Elsewhere,
        vec![StreamProtocol::new("/foreign/kad/1.0.0")],
    )
    .await;
    // Give identify time to land on the bootstrap, which is when a kad
    // speaker would be fed to the peerstore.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let addrs = tokio::time::timeout(SETTLE_TIMEOUT, rig.dialer.dht_find_peer(rig.target_id))
        .await
        .expect("walk should finish")
        .unwrap_or_default();
    assert!(
        addrs.is_empty(),
        "a foreign peer was vouched for: {addrs:?}"
    );
}
