//! A NATed peer is findable by an ordinary walk, unpatched: identify puts its
//! circuit into the bootstrap's k-bucket, kad returns a tabled peer first in
//! a FIND_NODE for its own id, and the circuit leaves the table again when
//! the reservation behind it ends.
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
    target: NetworkHandle,
    dialer: NetworkHandle,
    _handles: Vec<NetworkHandle>,
    _tasks: Vec<tokio::task::JoinHandle<()>>,
}

/// Where the NATed target holds its reservation.
enum Relay {
    /// On the bootstrap itself.
    TheBootstrap,
    /// On a third node.
    Elsewhere,
}

/// A NATed target speaking `kad` connected to a bootstrap, and a dialer that
/// knows only the bootstrap. Loopback listeners are never tabled, so the
/// target's circuit is the one address the walk's answer can carry.
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
    let mut handles = Vec::new();
    let mut tasks = vec![rt, tt, dt];

    // The target reserves on the relay: its listen set gains a circuit.
    eventually("the target's reservation on the relay", || async {
        target
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
            target
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
        target,
        dialer,
        _handles: handles,
        _tasks: tasks,
    }
}

impl Rig {
    /// What the bootstrap's routing table holds for the target.
    async fn tabled_target_addrs(&self) -> Vec<Multiaddr> {
        self.bootstrap
            .network_snapshot()
            .await
            .expect("snapshot")
            .routing
            .into_iter()
            .find(|e| e.peer_id == self.target_id)
            .map(|e| e.addrs)
            .unwrap_or_default()
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

/// The reservation is on a third node: the circuit reaches the bootstrap
/// only through the target's identify, and a plain walk returns it.
#[tokio::test]
async fn a_walk_finds_a_natted_peer_by_its_circuit() {
    let rig = natted_target(Relay::Elsewhere, kwaai_p2p::config::kad_protocols()).await;
    let addrs = walk_for(&rig).await;
    let circuit = format!("{}/p2p/{}/p2p-circuit", rig.relay_listen, rig.relay_id);
    assert!(
        addrs.iter().any(|a| a.to_string() == circuit),
        "the circuit the bootstrap tabled: {addrs:?}"
    );
    let resolved = format!("{circuit}/p2p/{}", rig.target_id);
    let connected = rig
        .dialer
        .connect_peer(&resolved)
        .await
        .expect("dial with the walk's answer");
    assert_eq!(connected, rig.target_id);
}

/// The reservation is on the bootstrap itself: the answer carries a circuit
/// through the bootstrap's own external address.
#[tokio::test]
async fn the_answer_carries_a_circuit_through_the_bootstrap_it_reserved_on() {
    let rig = natted_target(Relay::TheBootstrap, kwaai_p2p::config::kad_protocols()).await;
    let addrs = walk_for(&rig).await;
    let circuit = format!("{}/p2p/{}/p2p-circuit", rig.relay_listen, rig.bootstrap_id);
    assert!(
        addrs.iter().any(|a| a.to_string() == circuit),
        "a circuit on the bootstrap's external address: {addrs:?}"
    );
}

/// A circuit through the bootstrap is only good while the reservation
/// stands. kad keeps a disconnected peer's entry, so without an explicit
/// removal the bootstrap would keep serving a dead circuit.
#[tokio::test]
async fn the_circuit_leaves_the_table_when_the_reservation_ends() {
    let rig = natted_target(Relay::TheBootstrap, kwaai_p2p::config::kad_protocols()).await;
    let circuit = format!("{}/p2p/{}/p2p-circuit", rig.relay_listen, rig.bootstrap_id);
    // kad stores the entry with the target's own `/p2p/<id>` appended.
    let holds_circuit =
        |addrs: &[Multiaddr]| addrs.iter().any(|a| a.to_string().starts_with(&circuit));
    eventually("the bootstrap to table the circuit", || async {
        holds_circuit(&rig.tabled_target_addrs().await).then_some(())
    })
    .await;
    // Shut down rather than disconnect: a live target would re-reserve on
    // its trusted relay and put the circuit straight back.
    rig.target.shutdown().await.expect("target shuts down");
    eventually("the circuit to leave the table", || async {
        (!holds_circuit(&rig.tabled_target_addrs().await)).then_some(())
    })
    .await;
}

/// A peer that does not speak our kad is never tabled, reachable or not:
/// the table is what FIND_NODE hands out, and a bootstrap answering for
/// foreign peers is how it re-enters the public DHT's serving path.
#[tokio::test]
async fn a_peer_that_does_not_speak_our_kad_is_not_answered() {
    let rig = natted_target(
        Relay::Elsewhere,
        vec![StreamProtocol::new("/foreign/kad/1.0.0")],
    )
    .await;
    // Give identify time to land on the bootstrap, which is when a kad
    // speaker would be tabled.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let addrs = tokio::time::timeout(SETTLE_TIMEOUT, rig.dialer.dht_find_peer(rig.target_id))
        .await
        .expect("walk should finish")
        .unwrap_or_default();
    assert!(
        addrs.is_empty(),
        "a foreign peer was answered for: {addrs:?}"
    );
}
