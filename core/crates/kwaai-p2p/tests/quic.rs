//! QUIC alongside TCP, and the switch that turns it off.
//!
//! Loopback cannot show QUIC's latency or loss behaviour. It can show that the
//! transport is wired up: a peer reaches us over QUIC rather than silently
//! falling back to TCP, and `enable_quic: false` removes it.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr, PeerId};

const SETTLE_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

async fn eventually<T, F, Fut>(what: &str, mut f: F) -> T
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<T>>,
{
    let deadline = tokio::time::Instant::now() + SETTLE_TIMEOUT;
    loop {
        if let Some(value) = f().await {
            return value;
        }
        if tokio::time::Instant::now() >= deadline {
            panic!("timed out after {SETTLE_TIMEOUT:?} waiting for: {what}");
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

fn is_quic(addr: &Multiaddr) -> bool {
    addr.iter().any(|p| matches!(p, Protocol::QuicV1))
}

/// QUIC-only, so a connection to it cannot quietly be TCP.
fn spawn_quic_only() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            listen_addrs: vec!["/ip4/127.0.0.1/udp/0/quic-v1".to_string()],
            enable_quic: true,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("node should start");
    (handle, task, peer_id)
}

#[tokio::test]
async fn two_nodes_connect_over_quic() {
    let (alice, _alice_task, alice_id) = spawn_quic_only();
    let alice_addr = eventually("alice to report a quic listen address", || async {
        alice.listen_addrs().await.ok()?.into_iter().find(is_quic)
    })
    .await
    .with(Protocol::P2p(alice_id));

    let (bob, _bob_task, _bob_id) = spawn_quic_only();
    let connected = bob
        .connect_peer(&alice_addr.to_string())
        .await
        .expect("bob should reach alice over quic");
    assert_eq!(connected, alice_id);

    let peer = eventually("bob to list alice", || async {
        bob.list_peers()
            .await
            .ok()?
            .into_iter()
            .find(|p| p.peer_id == alice_id)
    })
    .await;
    assert!(
        is_quic(&peer.addr),
        "the connection should be QUIC, not a TCP fallback: {}",
        peer.addr
    );
}

/// The nat-test nodes and production configure `port`, not `listen_addrs`, so
/// this fallback is the real listen set.
#[test]
fn enable_quic_decides_whether_quic_is_listened_on() {
    let with_quic = NetworkConfig {
        port: 8080,
        listen_addrs: Vec::new(),
        enable_quic: true,
        ..NetworkConfig::default()
    }
    .swarm_listen_addrs();
    assert!(
        with_quic.iter().any(|a| a == "/ip4/0.0.0.0/tcp/8080"),
        "tcp must survive: {with_quic:?}"
    );
    assert!(
        with_quic
            .iter()
            .any(|a| a == "/ip4/0.0.0.0/udp/8080/quic-v1"),
        "quic must be listened on: {with_quic:?}"
    );

    let without = NetworkConfig {
        port: 8080,
        listen_addrs: Vec::new(),
        enable_quic: false,
        ..NetworkConfig::default()
    }
    .swarm_listen_addrs();
    assert!(
        without.iter().any(|a| a == "/ip4/0.0.0.0/tcp/8080"),
        "tcp is unaffected by the switch: {without:?}"
    );
    assert!(
        !without.iter().any(|a| a.contains("quic")),
        "enable_quic: false must leave no quic listener: {without:?}"
    );
}

/// The switch gates the transport, not just the listen set.
#[tokio::test]
async fn a_node_with_quic_disabled_cannot_dial_quic() {
    let (alice, _alice_task, alice_id) = spawn_quic_only();
    let alice_addr = eventually("alice to report a quic listen address", || async {
        alice.listen_addrs().await.ok()?.into_iter().find(is_quic)
    })
    .await
    .with(Protocol::P2p(alice_id));

    let keypair = Keypair::generate_ed25519();
    let (bob, _bob_task) = NetworkService::spawn(
        NetworkConfig {
            enable_quic: false,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("node should start");

    assert!(
        bob.connect_peer(&alice_addr.to_string()).await.is_err(),
        "a node built without the QUIC transport should not reach a QUIC address"
    );
}
