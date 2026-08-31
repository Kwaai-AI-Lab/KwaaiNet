//! Which local port a dial leaves from.
//!
//! DCUtR is a property of NATs and cannot be demonstrated on loopback (see
//! `tests/dcutr.rs`). Its *precondition* can be, because the source port of a
//! dial is visible to the peer being dialed: it arrives as the connection's
//! send-back address.
//!
//! That precondition is the whole of the fix. `DialOpts` defaults to
//! `PortUse::Reuse` so that a dial leaves from the listen port, which is what
//! puts that port's NAT binding into the address peers observe for us — and
//! that observed address is the only thing DCUtR has to punch at. Override it
//! with `allocate_new_port()` and libp2p-identify rewrites the observation to
//! our listen port (`address_translation`), naming a port the NAT never
//! mapped. Every hole punch then aims at a closed door.
//!
//! So these tests exist to catch a re-introduced `allocate_new_port()`.

use std::time::Duration;

use kwaai_p2p::{Direction, NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, Multiaddr, PeerId};

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

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("binding an ephemeral port")
        .local_addr()
        .expect("local addr")
        .port()
}

/// A node on a port we choose, so the test can compare a dial's source port
/// against it.
fn spawn_on(port: u16) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            listen_addrs: vec![format!("/ip4/127.0.0.1/tcp/{port}")],
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("node should start");
    (handle, task, peer_id)
}

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

/// The port `observer` saw an inbound connection from `peer` arrive on.
async fn inbound_source_port(observer: &NetworkHandle, peer: PeerId) -> u16 {
    let info = eventually("an inbound connection from the dialer", || async {
        observer
            .list_peers()
            .await
            .ok()?
            .into_iter()
            .find(|p| p.peer_id == peer && p.direction == Direction::Inbound)
    })
    .await;
    info.addr
        .iter()
        .find_map(|p| match p {
            libp2p::multiaddr::Protocol::Tcp(port) => Some(port),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no tcp port in the send-back address: {}", info.addr))
}

#[tokio::test]
async fn a_bootstrap_dial_leaves_from_our_listen_port() {
    let (bootstrap, _bootstrap_task, bootstrap_id) = spawn_on(free_port());
    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;

    let listen_port = free_port();
    let (node, _node_task, node_id) = spawn_on(listen_port);
    node.bootstrap(vec![bootstrap_addr])
        .await
        .expect("bootstrap dial");

    assert_eq!(
        inbound_source_port(&bootstrap, node_id).await,
        listen_port,
        "a bootstrap dial must leave from our listen port, or identify teaches \
         the network a mapping no hole punch can use"
    );
}

#[tokio::test]
async fn an_explicit_connect_leaves_from_our_listen_port() {
    // The same rule for the other dial path. Both are `PortUse::Reuse` only
    // because neither overrides the default, so this fails the moment someone
    // reaches for `allocate_new_port()` again.
    let (peer, _peer_task, peer_id) = spawn_on(free_port());
    let peer_addr = dialable_addr(&peer, peer_id).await;

    let listen_port = free_port();
    let (node, _node_task, node_id) = spawn_on(listen_port);
    node.connect_peer(&peer_addr.to_string())
        .await
        .expect("connect_peer");

    assert_eq!(
        inbound_source_port(&peer, node_id).await,
        listen_port,
        "an explicit connect must leave from our listen port too"
    );
}
