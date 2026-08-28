//! Direct-connection upgrade through relay (dcutr), as far as loopback allows.
//!
//! # What these tests can and cannot prove
//!
//! Hole punching is a property of *NATs*: two peers behind different address
//! translators simultaneously dial each other so both NATs open a mapping for
//! traffic they believe is a reply. On loopback there is no NAT, no mapping,
//! and nothing to punch through — every peer can already reach every other
//! directly. So no in-process test can demonstrate a hole punch, and none here
//! claims to.
//!
//! What they do prove is the **plumbing**: that a relayed connection is
//! established at all, that dcutr is attached to it and attempts an upgrade,
//! and that the upgrade produces a genuinely non-relayed connection between two
//! peers that only ever exchanged a circuit address. Everything above that —
//! success rate against cone NATs, behaviour against symmetric ones — is
//! docker nat-test topology work and is honestly out of reach here.
//!
//! A useful accident of loopback: because the direct dial always succeeds, the
//! upgrade path runs *reliably* rather than probabilistically, which makes it a
//! much better regression test of the wiring than a real NAT would be.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
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

/// A relay with a declared external address — without one its hop server
/// accepts reservations but returns no address, and the client cannot build a
/// circuit address. See `tests/relay.rs`.
async fn spawn_relay() -> (NetworkHandle, tokio::task::JoinHandle<()>, Multiaddr) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let listen = format!("/ip4/127.0.0.1/tcp/{}", free_port());
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            listen_addrs: vec![listen.clone()],
            relay_server: true,
            external_addr: Some(listen),
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("relay should start");
    let addr = dialable_addr(&handle, peer_id).await;
    (handle, task, addr)
}

fn spawn_client(relays: Vec<Multiaddr>) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            force_private: true,
            trusted_relays: relays.iter().map(|a| a.to_string()).collect(),
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("client should start");
    (handle, task, peer_id)
}

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

async fn circuit_of(handle: &NetworkHandle) -> Multiaddr {
    eventually("a circuit address", || async {
        handle
            .listen_addrs()
            .await
            .ok()?
            .into_iter()
            .find(kwaai_p2p::is_circuit)
    })
    .await
}

#[tokio::test]
async fn a_relayed_connection_is_upgraded_to_a_direct_one() {
    // Alice holds a reservation. Bob knows nothing about her but the circuit
    // address, so the connection *must* start relayed — and dcutr should then
    // replace it with a direct one, which on loopback always succeeds.
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let (alice, _alice_task, alice_id) = spawn_client(vec![relay_addr.clone()]);
    let circuit = circuit_of(&alice).await;

    let (bob, _bob_task, bob_id) = spawn_client(vec![]);
    bob.connect_peer(&relay_addr.to_string())
        .await
        .expect("bob → relay");

    let connected = eventually("bob to reach alice over the circuit", || async {
        bob.connect_peer(&circuit.to_string()).await.ok()
    })
    .await;
    assert_eq!(connected, alice_id);

    // The upgrade: a connection to alice whose address carries no
    // `/p2p-circuit`. Bob was never told a direct address for her — dcutr
    // derived it from the relayed connection.
    let direct = eventually("the connection to become direct", || async {
        let peers = bob.list_peers().await.ok()?;
        peers
            .into_iter()
            .find(|p| p.peer_id == alice_id && !kwaai_p2p::is_circuit(&p.addr))
    })
    .await;
    assert!(
        !direct.addr.to_string().contains("/p2p-circuit"),
        "dcutr should have produced a non-relayed address: {}",
        direct.addr
    );

    // And it must be *reported* as punched, not merely be direct. This flag is
    // the only thing distinguishing "there was no NAT in the way" from "a NAT
    // was traversed", and it is what the GUI renders as `p2p` rather than
    // `direct`. The service sets it from the dcutr event, which arrives after
    // the `ConnectionEstablished` that registers the connection — assert it
    // rather than trusting that ordering to hold across a libp2p upgrade.
    let punched = eventually("the upgraded connection to be flagged dcutr", || async {
        let peers = bob.list_peers().await.ok()?;
        peers.into_iter().find(|p| p.peer_id == alice_id && p.dcutr)
    })
    .await;
    assert!(
        !kwaai_p2p::is_circuit(&punched.addr),
        "the dcutr-flagged connection should be the direct one: {}",
        punched.addr
    );

    // The circuit it replaced is retired. Leaving it up is what made hole
    // punching pointless in practice: new substreams keep landing on whichever
    // connection libp2p picks, so kad and identify carry on over the relay, the
    // punched path sits idle, and the idle timeout reaps it inside a minute.
    eventually("the relayed path to be closed", || async {
        let peers = alice.list_peers().await.ok()?;
        peers
            .iter()
            .all(|p| p.peer_id != bob_id || !kwaai_p2p::is_circuit(&p.addr))
            .then_some(())
    })
    .await;

    // Alice sees Bob too, and by a peer id she was never given out of band.
    eventually("alice to see bob", || async {
        let peers = alice.list_peers().await.ok()?;
        peers.iter().any(|p| p.peer_id == bob_id).then_some(())
    })
    .await;
}

#[tokio::test]
async fn the_relayed_path_still_works_when_the_upgrade_is_pointless() {
    // Two nodes that can already reach each other directly should not be broken
    // by dcutr trying: a failed or unnecessary upgrade must leave the original
    // connection intact rather than tearing it down.
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let (alice, _alice_task, alice_id) = spawn_client(vec![relay_addr.clone()]);
    let circuit = circuit_of(&alice).await;

    let (bob, _bob_task, _bob_id) = spawn_client(vec![]);
    bob.connect_peer(&relay_addr.to_string())
        .await
        .expect("bob → relay");
    eventually("bob to reach alice", || async {
        bob.connect_peer(&circuit.to_string()).await.ok()
    })
    .await;

    // Whatever dcutr decides, the peers stay connected and usable.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let peers = bob.list_peers().await.expect("list_peers");
    assert!(
        peers.iter().any(|p| p.peer_id == alice_id),
        "the connection must survive the upgrade attempt: {peers:?}"
    );
}

#[tokio::test]
async fn dcutr_does_not_disturb_an_ordinary_direct_connection() {
    // dcutr acts only on relayed connections. A plain dial must be unaffected —
    // this is the regression guard for the behaviour being composed in at all.
    let (alice, _alice_task, alice_id) = spawn_client(vec![]);
    let (bob, _bob_task, bob_id) = spawn_client(vec![]);

    let alice_addr = dialable_addr(&alice, alice_id).await;
    let connected = bob
        .connect_peer(&alice_addr.to_string())
        .await
        .expect("a plain dial should work");
    assert_eq!(connected, alice_id);

    tokio::time::sleep(Duration::from_secs(1)).await;
    let peers = bob.list_peers().await.expect("list_peers");
    assert!(peers.iter().any(|p| p.peer_id == alice_id));
    assert!(
        peers.iter().all(|p| !kwaai_p2p::is_circuit(&p.addr)),
        "no circuits are involved here at all: {peers:?}"
    );
    assert_ne!(alice_id, bob_id);
}
