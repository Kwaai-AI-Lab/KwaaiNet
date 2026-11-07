//! A relay introduces two peers; it does not carry their session.
//!
//! - Requests to a connected peer reuse its connection: no new dial, no new
//!   circuit.
//! - Once a direct connection to a peer is confirmed, the relayed one closes —
//!   but only after the requests already on it finish.
//! - A failed punch leaves the relayed connection up and working.
//! - A peer we exchange requests with is not idled out between them.
//!
//! Loopback has no NAT, so a punch always succeeds unless the peers have no
//! listener for it to reach; the "failed punch" rig below uses exactly that.

use std::time::Duration;

use kwaai_p2p::{is_circuit, NetworkConfig, NetworkHandle, NetworkService, PeerInfo};
use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr, PeerId};

const SETTLE_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(20);
const APP_PROTO: &str = "kwaai.test.echo";

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

fn spawn(config: NetworkConfig) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(config, keypair).expect("node should start");
    (handle, task, peer_id)
}

/// A relay with a declared external address, so reservations carry one.
async fn spawn_relay() -> (NetworkHandle, tokio::task::JoinHandle<()>, Multiaddr) {
    let listen = format!("/ip4/127.0.0.1/tcp/{}", free_port());
    let (handle, task, peer_id) = spawn(NetworkConfig {
        listen_addrs: vec![listen.clone()],
        relay_server: true,
        external_addr: Some(listen.clone()),
        ..NetworkConfig::for_tests()
    });
    let addr: Multiaddr = listen.parse().expect("listen addr");
    (handle, task, addr.with(Protocol::P2p(peer_id)))
}

/// A node with no TCP listener at all, so no punch dial can reach it. The
/// listen address is QUIC on a build without QUIC, which the swarm skips.
fn unreachable_config() -> NetworkConfig {
    NetworkConfig {
        listen_addrs: vec!["/ip4/127.0.0.1/udp/0/quic-v1".to_string()],
        enable_quic: false,
        ..NetworkConfig::for_tests()
    }
}

async fn circuit_of(handle: &NetworkHandle) -> Multiaddr {
    eventually("a circuit address", || async {
        handle
            .listen_addrs()
            .await
            .ok()?
            .into_iter()
            .find(is_circuit)
    })
    .await
}

async fn conns_to(handle: &NetworkHandle, peer: PeerId) -> Vec<PeerInfo> {
    handle
        .list_peers()
        .await
        .expect("list_peers")
        .into_iter()
        .filter(|p| p.peer_id == peer)
        .collect()
}

fn relayed(p: &PeerInfo) -> bool {
    is_circuit(&p.addr) || p.via.is_some()
}

async fn serve_echo(handle: &NetworkHandle, delay: Duration) {
    handle
        .add_unary_handler(APP_PROTO, move |data: Vec<u8>| async move {
            tokio::time::sleep(delay).await;
            Ok(data)
        })
        .await
        .expect("register handler");
}

/// Alice holds a reservation on a relay; Bob reaches her only through it.
struct Rig {
    _relay: (NetworkHandle, tokio::task::JoinHandle<()>),
    alice: NetworkHandle,
    alice_id: PeerId,
    _alice_task: tokio::task::JoinHandle<()>,
    bob: NetworkHandle,
    _bob_task: tokio::task::JoinHandle<()>,
    circuit: Multiaddr,
}

async fn rig(peer_config: impl Fn() -> NetworkConfig) -> Rig {
    let (relay, relay_task, relay_addr) = spawn_relay().await;
    let (alice, alice_task, alice_id) = spawn(NetworkConfig {
        force_private: true,
        trusted_relays: vec![relay_addr.to_string()],
        ..peer_config()
    });
    let circuit = circuit_of(&alice).await;
    let (bob, bob_task, _) = spawn(peer_config());
    bob.connect_peer(&relay_addr.to_string())
        .await
        .expect("bob → relay");
    Rig {
        _relay: (relay, relay_task),
        alice,
        alice_id,
        _alice_task: alice_task,
        bob,
        _bob_task: bob_task,
        circuit,
    }
}

#[tokio::test]
async fn the_relayed_connection_closes_after_the_direct_one_is_confirmed() {
    let r = rig(NetworkConfig::for_tests).await;
    let circuit = r.circuit.to_string();
    eventually("bob to reach alice over the circuit", || async {
        r.bob.connect_peer(&circuit).await.ok()
    })
    .await;

    eventually("a direct connection and no relayed one", || async {
        let conns = conns_to(&r.bob, r.alice_id).await;
        (conns.iter().any(|p| !relayed(p)) && !conns.iter().any(relayed)).then_some(())
    })
    .await;

    let bob = r.bob.network_snapshot().await.expect("snapshot").hole_punch;
    let alice = r
        .alice
        .network_snapshot()
        .await
        .expect("snapshot")
        .hole_punch;
    assert!(
        bob.succeeded + alice.succeeded >= 1,
        "a punch should be counted: {bob:?} {alice:?}"
    );
    assert!(
        bob.relayed_released + alice.relayed_released >= 1,
        "one side should report releasing the relay: {bob:?} {alice:?}"
    );
    assert_eq!(bob.attempted, bob.succeeded + bob.failed);
}

#[tokio::test]
async fn a_call_in_flight_on_the_relay_survives_the_upgrade() {
    let r = rig(NetworkConfig::for_tests).await;
    serve_echo(&r.alice, Duration::from_secs(5)).await;
    // Bob knows only the circuit, and the call itself dials it: the first
    // connection, and so the one the call rides, is relayed.
    r.bob
        .add_kad_address(r.alice_id, r.circuit.clone())
        .await
        .expect("seed the circuit");
    let bob = r.bob.clone();
    let alice_id = r.alice_id;
    let call =
        tokio::spawn(async move { bob.call_unary_handler(alice_id, APP_PROTO, b"slow").await });

    // The punch completes while the call is still in flight on the relay,
    // and the relay is held for it rather than closed under it.
    let mut overlapped = false;
    while !call.is_finished() {
        let conns = conns_to(&r.bob, r.alice_id).await;
        overlapped |= conns.iter().any(|p| !relayed(p)) && conns.iter().any(relayed);
        tokio::time::sleep(POLL_INTERVAL).await;
    }
    assert!(
        overlapped,
        "the direct connection should have opened while the call was in flight"
    );

    let reply = call.await.expect("join");
    assert_eq!(
        reply.expect("the call on the retiring relay must complete"),
        b"slow"
    );

    eventually("the relayed connection to close once idle", || async {
        let conns = conns_to(&r.bob, r.alice_id).await;
        (!conns.is_empty() && !conns.iter().any(relayed)).then_some(())
    })
    .await;
}

#[tokio::test]
async fn requests_to_a_relayed_peer_reuse_its_connection_and_a_failed_punch_keeps_it() {
    let r = rig(unreachable_config).await;
    serve_echo(&r.alice, Duration::ZERO).await;
    let circuit = r.circuit.to_string();
    eventually("bob to reach alice over the circuit", || async {
        r.bob.connect_peer(&circuit).await.ok()
    })
    .await;

    // Neither side listens, so every punch dial fails.
    eventually("the punch to fail", || async {
        let s = r.bob.network_snapshot().await.ok()?.hole_punch;
        let a = r.alice.network_snapshot().await.ok()?.hole_punch;
        (s.failed + a.failed >= 1).then_some(())
    })
    .await;

    for round in 0..2 {
        r.bob
            .connect_peer(&circuit)
            .await
            .unwrap_or_else(|e| panic!("connect {round}: {e}"));
        let reply = r
            .bob
            .call_unary_handler(r.alice_id, APP_PROTO, b"again")
            .await
            .unwrap_or_else(|e| panic!("call {round}: {e}"));
        assert_eq!(reply, b"again");
    }

    // One relayed connection the whole time: the repeat connect and the calls
    // rode it, and the failed punch did not close it.
    // Longer than a retiring connection's quiet period plus a check interval.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while tokio::time::Instant::now() < deadline {
        let conns = conns_to(&r.bob, r.alice_id).await;
        assert_eq!(conns.len(), 1, "exactly one connection to alice: {conns:?}");
        assert!(relayed(&conns[0]), "and it is the relayed one: {conns:?}");
        tokio::time::sleep(POLL_INTERVAL).await;
    }
    let s = r.bob.network_snapshot().await.expect("snapshot").hole_punch;
    assert_eq!(s.succeeded, 0, "{s:?}");
    assert_eq!(s.relayed_released, 0, "{s:?}");
    assert_eq!(s.relayed_connections, 1, "{s:?}");
}

/// Two directly connected nodes with a one-second idle timeout.
async fn idle_pair(
    lease: Duration,
) -> (
    NetworkHandle,
    NetworkHandle,
    PeerId,
    Vec<tokio::task::JoinHandle<()>>,
) {
    let config = || NetworkConfig {
        idle_connection_timeout: Duration::from_secs(1),
        active_peer_keep_alive: lease,
        ..NetworkConfig::for_tests()
    };
    let (alice, alice_task, alice_id) = spawn(config());
    let (bob, bob_task, _) = spawn(config());
    serve_echo(&alice, Duration::ZERO).await;
    alice
        .add_unary_handler(
            "DHTProtocol.rpc_test",
            |data: Vec<u8>| async move { Ok(data) },
        )
        .await
        .expect("register dht handler");
    let addr = eventually("alice's listen address", || async {
        alice.listen_addrs().await.ok()?.into_iter().next()
    })
    .await
    .with(Protocol::P2p(alice_id));
    bob.connect_peer(&addr.to_string()).await.expect("connect");
    (alice, bob, alice_id, vec![alice_task, bob_task])
}

async fn connected_after(bob: &NetworkHandle, peer: PeerId, wait: Duration) -> bool {
    tokio::time::sleep(wait).await;
    !conns_to(bob, peer).await.is_empty()
}

#[tokio::test]
async fn a_peer_in_active_use_is_not_idled_out() {
    let (_alice, bob, alice_id, _tasks) = idle_pair(Duration::from_secs(30)).await;
    bob.call_unary_handler(alice_id, APP_PROTO, b"x")
        .await
        .expect("call");
    assert!(
        connected_after(&bob, alice_id, Duration::from_secs(5)).await,
        "an application request should hold the connection past the idle timeout"
    );
}

#[tokio::test]
async fn dht_traffic_alone_does_not_hold_a_connection_open() {
    let (_alice, bob, alice_id, _tasks) = idle_pair(Duration::from_secs(30)).await;
    bob.call_unary_handler(alice_id, "DHTProtocol.rpc_test", b"x")
        .await
        .expect("call");
    assert!(
        !connected_after(&bob, alice_id, Duration::from_secs(5)).await,
        "a DHT call should leave the connection to the idle timeout"
    );
}

#[tokio::test]
async fn the_hold_ends_when_the_lease_does() {
    let (_alice, bob, alice_id, _tasks) = idle_pair(Duration::from_secs(2)).await;
    bob.call_unary_handler(alice_id, APP_PROTO, b"x")
        .await
        .expect("call");
    assert!(
        !connected_after(&bob, alice_id, Duration::from_secs(6)).await,
        "the connection should idle out once the lease expires"
    );
}

/// The relay rig with no punch possible, a two-second hold on circuits and an
/// idle timeout long enough that only the hold can close them.
fn short_relay_hold() -> NetworkConfig {
    NetworkConfig {
        idle_connection_timeout: Duration::from_secs(60),
        active_peer_keep_alive: Duration::from_secs(60),
        relayed_peer_keep_alive: Duration::from_secs(2),
        ..unreachable_config()
    }
}

#[tokio::test]
async fn a_relayed_session_is_held_only_briefly_once_it_goes_quiet() {
    let r = rig(short_relay_hold).await;
    serve_echo(&r.alice, Duration::from_secs(4)).await;
    r.bob
        .add_kad_address(r.alice_id, r.circuit.clone())
        .await
        .expect("seed the circuit");

    // A call longer than the hold is in flight, not idle.
    let reply = r
        .bob
        .call_unary_handler(r.alice_id, APP_PROTO, b"slow")
        .await
        .expect("a call outlasting the hold must complete");
    assert_eq!(reply, b"slow");
    assert_eq!(conns_to(&r.bob, r.alice_id).await.len(), 1);

    // Then quiet: the circuit goes after the short hold, not the 60 s lease.
    eventually("the quiet circuit to close", || async {
        conns_to(&r.bob, r.alice_id).await.is_empty().then_some(())
    })
    .await;
    let bob = r.bob.network_snapshot().await.expect("snapshot").hole_punch;
    let alice = r
        .alice
        .network_snapshot()
        .await
        .expect("snapshot")
        .hole_punch;
    assert!(
        bob.relayed_idled + alice.relayed_idled >= 1,
        "{bob:?} {alice:?}"
    );
}

#[tokio::test]
async fn steady_steps_over_a_relay_keep_one_circuit() {
    let r = rig(short_relay_hold).await;
    serve_echo(&r.alice, Duration::ZERO).await;
    r.bob
        .add_kad_address(r.alice_id, r.circuit.clone())
        .await
        .expect("seed the circuit");
    // Steps a second apart, each well inside the two-second hold.
    for step in 0..6 {
        r.bob
            .call_unary_handler(r.alice_id, APP_PROTO, b"step")
            .await
            .unwrap_or_else(|e| panic!("step {step}: {e}"));
        let conns = conns_to(&r.bob, r.alice_id).await;
        assert_eq!(conns.len(), 1, "one circuit throughout: {conns:?}");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    let s = r.bob.network_snapshot().await.expect("snapshot").hole_punch;
    assert_eq!(s.relayed_idled, 0, "{s:?}");
}

#[tokio::test]
async fn a_direct_active_peer_keeps_the_full_lease() {
    let config = || NetworkConfig {
        idle_connection_timeout: Duration::from_secs(1),
        ..short_relay_hold()
    };
    let (alice, _at, alice_id) = spawn(NetworkConfig {
        listen_addrs: vec!["/ip4/127.0.0.1/tcp/0".to_string()],
        ..config()
    });
    let (bob, _bt, _) = spawn(config());
    serve_echo(&alice, Duration::ZERO).await;
    let addr = eventually("alice's listen address", || async {
        alice.listen_addrs().await.ok()?.into_iter().next()
    })
    .await
    .with(Protocol::P2p(alice_id));
    bob.connect_peer(&addr.to_string()).await.expect("connect");
    bob.call_unary_handler(alice_id, APP_PROTO, b"x")
        .await
        .expect("call");
    assert!(
        connected_after(&bob, alice_id, Duration::from_secs(5)).await,
        "a direct connection is held for the full lease, not the relay hold"
    );
}
