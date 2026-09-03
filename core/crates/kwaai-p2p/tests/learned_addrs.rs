//! The learned-address map, through the full service.
//!
//! `src/learned_addrs.rs` unit-tests the container's bounds. What only a real
//! swarm can show is the property the map exists for: that an address supplied
//! with a connect request is **kept**, so a later dial to the same peer uses it
//! instead of asking a routing table that never held the peer.
//!
//! The two nodes here share no bootstrap and never meet through the DHT, so A's
//! only way to reach B is what the test hands it. That is deliberate: it is the
//! shape of the real failure — a NATed peer discovered as a bare PeerId, which
//! kad cannot resolve because it answers `FIND_NODE` from its k-buckets alone.
//!
//! Negative control, run: with the learned-address consult removed from
//! `candidate_addresses`, both `connect_with_addresses_reaches_a_peer_the_dht_cannot`
//! and `a_routed_request_redials_from_the_learned_map_after_a_disconnect` fail
//! with `peer not found in DHT (no addresses)`.

use std::time::Duration;

use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use libp2p::identity::Keypair;

const PROTO: &str = "DHTProtocol.rpc_ping";
const TEST_TIMEOUT: Duration = Duration::from_secs(20);

/// A loopback service with no bootstraps at all — nothing it can discover.
fn spawn_isolated() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "off".into()),
        )
        .with_test_writer()
        .try_init();

    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let mut config = NetworkConfig::for_tests();
    // Both lists empty would fall through to the compiled-in bootstraps; a
    // syntactically valid address on a port nothing listens on keeps this test
    // from dialing anything real.
    config.initial_peers = vec![
        "/ip4/127.0.0.1/tcp/1/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN".to_string(),
    ];
    let (handle, task) = NetworkService::spawn(config, keypair).expect("service should start");
    (handle, task, peer_id)
}

async fn first_listen_addr(handle: &NetworkHandle) -> Multiaddr {
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    loop {
        if let Some(addr) = handle
            .listen_addrs()
            .await
            .ok()
            .and_then(|a| a.into_iter().next())
        {
            return addr;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for a listen address"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn within<T>(what: &str, f: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(TEST_TIMEOUT, f)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for: {what}"))
}

/// The base case. B is in no routing table of A's, so a bare-PeerId connect
/// could only fail; supplying the address makes it work, and the routed unary
/// that follows proves the connection is a real one the behaviours can use.
#[tokio::test]
async fn connect_with_addresses_reaches_a_peer_the_dht_cannot() {
    let (a, _a_task, _a_id) = spawn_isolated();
    let (b, _b_task, b_id) = spawn_isolated();

    b.add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("registering a handler should succeed");

    let b_addr = first_listen_addr(&b).await;
    assert!(
        !a.routing_peers()
            .await
            .expect("routing table reads")
            .contains(&b_id),
        "the test is meaningless if B is already reachable through kad"
    );

    let connected = within(
        "the seeded connect",
        a.connect_peer_with_addrs(b_id, vec![b_addr]),
    )
    .await
    .expect("a supplied address must be enough to reach a peer kad cannot resolve");
    assert_eq!(connected, b_id);

    let echoed = within("a routed unary over the seeded connection", async {
        a.call_unary_handler(b_id, PROTO, b"hello").await
    })
    .await
    .expect("the connection must be usable by the behaviours");
    assert_eq!(echoed, b"hello");
}

/// The property a caller-side pre-connect could not give: the addresses
/// outlive the connection.
///
/// A pre-connect only helps while its connection lasts. The hop RPCs go through
/// `dispatch_routed`, which never saw the published addresses — so a dropped
/// connection put the next dial back at "no addresses". Here the connection is
/// dropped deliberately and the *routed request itself* re-dials from the map.
///
/// The re-dial must also be quick: a DHT walk would take the query timeout to
/// come back empty, so completing in a fraction of that is the evidence that no
/// walk was needed.
#[tokio::test]
async fn a_routed_request_redials_from_the_learned_map_after_a_disconnect() {
    let (a, _a_task, _a_id) = spawn_isolated();
    let (b, _b_task, b_id) = spawn_isolated();

    b.add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("registering a handler should succeed");

    let b_addr = first_listen_addr(&b).await;
    within(
        "the seeded connect",
        a.connect_peer_with_addrs(b_id, vec![b_addr]),
    )
    .await
    .expect("the seeded connect must succeed");

    a.disconnect_peer(b_id).await.expect("disconnect");
    // The close is asynchronous; wait for the swarm to agree it is gone, or
    // the request below would simply ride the old connection and prove nothing.
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    while a
        .list_peers()
        .await
        .expect("list_peers")
        .iter()
        .any(|p| p.peer_id == b_id)
    {
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for the connection to close"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // The map really is the only source: kad never took B in, so nothing else
    // in the node knows an address for it.
    assert!(
        !a.routing_peers()
            .await
            .expect("routing table reads")
            .contains(&b_id),
        "the re-dial must come from the learned map, not from a routing-table entry"
    );

    let started = std::time::Instant::now();
    let echoed = within("the re-dialed unary", async {
        a.call_unary_handler(b_id, PROTO, b"again").await
    })
    .await
    .expect("a routed request must re-dial from the learned addresses");
    assert_eq!(echoed, b"again");
    assert!(
        started.elapsed() < Duration::from_secs(5),
        "a re-dial from the learned map must not wait out a DHT walk (took {:?})",
        started.elapsed()
    );
}

/// Addresses supplied for a peer that cannot be reached at them must not
/// silently succeed, and must not wedge: the connect fails with the DHT
/// fallback's error, exactly as a bare-PeerId connect to an unknown peer does.
#[tokio::test]
async fn unreachable_supplied_addresses_still_fail() {
    let (a, _a_task, _a_id) = spawn_isolated();
    let unknown = PeerId::random();

    let result = within(
        "the doomed connect",
        a.connect_peer_with_addrs(
            unknown,
            vec!["/ip4/127.0.0.1/tcp/1".parse().expect("a valid addr")],
        ),
    )
    .await;
    assert!(
        result.is_err(),
        "a connect to a peer nothing can reach must report failure"
    );
}
