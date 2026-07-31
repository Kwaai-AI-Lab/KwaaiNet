//! Circuit relay reservations, end to end over loopback.
//!
//! The state machine's rules are unit-tested in `src/relay_manager.rs`. What
//! only a real swarm can show is that the machine and libp2p agree: that
//! `listen_on` on a circuit address really produces a HOP RESERVE, that a
//! relay's acceptance really arrives as a `NewListenAddr`, and that losing the
//! relay really surfaces as `ListenerClosed` and triggers a replacement.
//!
//! Every node here is on 127.0.0.1 with `force_private` set, which is what
//! makes reservations start immediately instead of after the 45s grace period
//! — no in-process test could wait that out.

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

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "off".into()),
        )
        .with_test_writer()
        .try_init();
}

/// A node that serves circuits for others.
///
/// The relay needs a **confirmed external address** or it is useless as one:
/// libp2p's hop server accepts the reservation but answers with an empty
/// address list ("Accepting relay reservation without providing external
/// addresses of local node"), and the client cannot build a circuit address out
/// of nothing, so the reservation fails. A relay is by definition a node that
/// knows where it is reachable — here that is declared, exactly as a real relay
/// operator would configure it.
async fn spawn_relay() -> (NetworkHandle, tokio::task::JoinHandle<()>, Multiaddr) {
    init_tracing();
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();

    // Pick a free port up front so the declared address is the one it really
    // listens on; a relay advertising somewhere it does not answer would be a
    // different and much more confusing test.
    let port = free_port();
    let listen = format!("/ip4/127.0.0.1/tcp/{port}");
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

/// A port nothing is listening on. Bound and released, so there is a small race
/// window — acceptable here because the alternative (an ephemeral port) cannot
/// be declared before it is known.
fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("binding an ephemeral port")
        .local_addr()
        .expect("local addr")
        .port()
}

/// A node that wants a reservation on `relays`.
fn spawn_client(relays: Vec<Multiaddr>) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            // Private from t=0, so the reservation is requested immediately
            // rather than after the identify grace period.
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

/// The circuit addresses a node is currently listening on.
async fn circuits(handle: &NetworkHandle) -> Vec<Multiaddr> {
    handle
        .listen_addrs()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(kwaai_p2p::is_circuit)
        .collect()
}

#[tokio::test]
async fn a_reservation_is_acquired_on_a_trusted_relay() {
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let (client, _client_task, client_id) = spawn_client(vec![relay_addr.clone()]);

    let circuits = eventually("the client to hold a circuit address", || async {
        let addrs = circuits(&client).await;
        (!addrs.is_empty()).then_some(addrs)
    })
    .await;

    assert_eq!(
        circuits.len(),
        1,
        "one relay, one reservation: {circuits:?}"
    );
    let circuit = circuits[0].to_string();
    assert!(
        circuit.contains("/p2p-circuit"),
        "not a circuit address: {circuit}"
    );
    assert!(
        circuit.contains(&client_id.to_base58()),
        "the circuit address should name the client it reaches: {circuit}"
    );
}

#[tokio::test]
async fn a_second_peer_can_dial_us_through_the_circuit() {
    // The point of the whole exercise: a reservation is only worth holding if
    // it actually carries a connection.
    let (relay, _relay_task, relay_addr) = spawn_relay().await;
    let (client, _client_task, client_id) = spawn_client(vec![relay_addr.clone()]);

    let circuit = eventually("the client to hold a circuit address", || async {
        circuits(&client).await.into_iter().next()
    })
    .await;

    // The dialer needs to know the relay before it can route through it.
    let (dialer, _dialer_task, _dialer_id) = spawn_client(vec![]);
    dialer
        .connect_peer(&relay_addr.to_string())
        .await
        .expect("dialer → relay");
    // Let the relay learn the dialer, so the HOP CONNECT has somewhere to go.
    let _ = relay.list_peers().await;

    let connected = eventually(
        "the dialer to reach the client over the circuit",
        || async { dialer.connect_peer(&circuit.to_string()).await.ok() },
    )
    .await;
    assert_eq!(connected, client_id);

    // The client sees the dialer. Deliberately *not* asserted: that the
    // connection is still relayed. On loopback the client's real listen address
    // is directly dialable, so dcutr frequently upgrades the circuit to a
    // direct connection within milliseconds — which is the whole point of
    // dcutr, and asserting the relayed form would make a success look like a
    // failure. What matters here is that the circuit carried the introduction.
    let peers = eventually("the client to see the dialer", || async {
        let peers = client.list_peers().await.ok()?;
        peers
            .iter()
            .any(|p| p.peer_id != client_id)
            .then_some(peers)
    })
    .await;
    assert!(!peers.is_empty());
}

#[tokio::test]
async fn only_one_reservation_per_relay_when_it_is_listed_twice() {
    // A duplicated entry must not consume both slots on one relay — that is two
    // circuits sharing a single point of failure.
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let (client, _client_task, _id) = spawn_client(vec![relay_addr.clone(), relay_addr.clone()]);

    eventually("the client to hold a circuit address", || async {
        circuits(&client).await.into_iter().next()
    })
    .await;

    // Give a duplicate every chance to appear before concluding it did not.
    tokio::time::sleep(Duration::from_secs(2)).await;
    let held = circuits(&client).await;
    assert_eq!(held.len(), 1, "exactly one reservation: {held:?}");
}

#[tokio::test]
async fn reservations_are_bounded_by_max_relay_reservations() {
    let (_r1, _t1, addr1) = spawn_relay().await;
    let (_r2, _t2, addr2) = spawn_relay().await;
    let (_r3, _t3, addr3) = spawn_relay().await;

    let keypair = Keypair::generate_ed25519();
    let (client, _task) = NetworkService::spawn(
        NetworkConfig {
            force_private: true,
            trusted_relays: vec![addr1.to_string(), addr2.to_string(), addr3.to_string()],
            max_relay_reservations: 2,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("client should start");

    eventually("the client to fill both slots", || async {
        (circuits(&client).await.len() >= 2).then_some(())
    })
    .await;

    tokio::time::sleep(Duration::from_secs(2)).await;
    let held = circuits(&client).await;
    assert_eq!(held.len(), 2, "the cap is 2, not 3: {held:?}");
}

#[tokio::test]
async fn refusal_rotates_to_the_next_candidate() {
    // The first "relay" does not serve hop at all, so the reservation is
    // refused at negotiation. That is the production bootstraps' documented
    // behaviour, so it has to move us on rather than stall us.
    let keypair = Keypair::generate_ed25519();
    let not_a_relay_id = keypair.public().to_peer_id();
    let (not_a_relay, _nt) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("swarm should start");
    let not_a_relay_addr = dialable_addr(&not_a_relay, not_a_relay_id).await;

    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let relay_id = relay_addr
        .iter()
        .find_map(|p| match p {
            libp2p::multiaddr::Protocol::P2p(id) => Some(id),
            _ => None,
        })
        .expect("relay addr carries a peer id");

    // Refusing candidate first, so rotation is what gets us to the working one.
    let (client, _client_task, _id) = spawn_client(vec![not_a_relay_addr, relay_addr.clone()]);

    let circuit = eventually("the client to reserve on the working relay", || async {
        circuits(&client).await.into_iter().next()
    })
    .await;
    assert!(
        circuit.to_string().contains(&relay_id.to_base58()),
        "the reservation should be on the relay that serves hop: {circuit}"
    );
}

#[tokio::test]
async fn losing_a_relay_is_detected_and_a_replacement_is_sought() {
    let (relay_a, _ta, addr_a) = spawn_relay().await;
    let (_relay_b, _tb, addr_b) = spawn_relay().await;

    // One slot, two candidates: losing the first must produce a reservation on
    // the second rather than leaving the node with none.
    let keypair = Keypair::generate_ed25519();
    let (client, _task) = NetworkService::spawn(
        NetworkConfig {
            force_private: true,
            trusted_relays: vec![addr_a.to_string(), addr_b.to_string()],
            max_relay_reservations: 1,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("client should start");

    let first = eventually("the first reservation", || async {
        circuits(&client).await.into_iter().next()
    })
    .await;

    // Take the relay away. Its connection dies, which arrives at the client as
    // a ListenerClosed — the only signal there is, since relay::client::Event
    // has no failure variant.
    relay_a.shutdown().await.expect("relay shutdown");

    let replacement = eventually("a reservation on a different relay", || async {
        circuits(&client)
            .await
            .into_iter()
            .find(|addr| *addr != first)
    })
    .await;

    assert_ne!(replacement, first);
    assert!(replacement.to_string().contains("/p2p-circuit"));
}

#[tokio::test]
async fn a_public_node_holds_no_reservations() {
    // Holding a circuit while directly dialable costs the relay real resources
    // and routes peers to us the slow way.
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;

    let keypair = Keypair::generate_ed25519();
    let (client, _task) = NetworkService::spawn(
        NetworkConfig {
            external_addr: Some("/ip4/203.0.113.7/tcp/8080".to_string()),
            trusted_relays: vec![relay_addr.to_string()],
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("client should start");

    assert!(client
        .reachability()
        .await
        .expect("reachability")
        .is_public());
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        circuits(&client).await.is_empty(),
        "a declared-public node must not reserve circuits"
    );
}

#[tokio::test]
async fn a_node_with_no_relay_candidates_does_not_spin() {
    // No configured relays, nothing discovered: the node should sit quietly
    // Private rather than retry-looping against nothing.
    let (client, _task, _id) = spawn_client(vec![]);
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(circuits(&client).await.is_empty());
    // Still responsive — a stuck relay loop would starve the command channel.
    assert!(client.list_peers().await.is_ok());
}

#[tokio::test]
async fn a_loopback_relay_is_never_discovered_by_identify() {
    // Identify discovery only accepts relays at *announceable* addresses — a
    // relay we can reach only at 127.0.0.1 is no use to peers who are not on
    // this machine. Every in-process relay is exactly that, which is why the
    // discovery path cannot be exercised here at all and is unit-tested in
    // `relay_manager` instead. This pins the filter rather than leaving its
    // absence looking like a gap.
    let (_relay, _relay_task, relay_addr) = spawn_relay().await;
    let (client, _client_task, _id) = spawn_client(vec![]);

    client
        .connect_peer(&relay_addr.to_string())
        .await
        .expect("client → relay");

    tokio::time::sleep(Duration::from_secs(3)).await;
    assert!(
        circuits(&client).await.is_empty(),
        "a loopback-only relay must not be adopted as a candidate"
    );
}
