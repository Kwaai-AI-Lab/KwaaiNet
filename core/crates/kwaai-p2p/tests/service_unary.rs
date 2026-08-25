//! Unary RPC through the full [`NetworkService`] stack.
//!
//! `tests/unary.rs` drives `unary::Behaviour` standalone; this tier exercises
//! the same protocol through the composed `KwaaiBehaviour` and the service's
//! select loop, which is where handler dispatch actually lives. What it proves
//! that the behaviour-level tests cannot:
//!
//! - `NetworkHandle::add_unary_handler` registers the protocol *and* the
//!   dispatch route in one round trip,
//! - the service maps `unary::UnaryError` onto `P2PError` as documented,
//! - dispatch survives the interesting failure modes: no handler registered, a
//!   handler that returns its error arm, a handler removed mid-life,
//! - a slow handler blocks neither the event loop nor other calls.
//!
//! Both swarms are loopback-only with freshly generated keys, so nothing here
//! can collide with a node running on the same machine.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService, P2PError, PeerId};
use libp2p::identity::Keypair;

const PROTO: &str = "DHTProtocol.rpc_ping";

/// A per-call cap so a lost reply fails the test rather than hanging it.
const TEST_TIMEOUT: Duration = Duration::from_secs(20);

/// Spawn a loopback service, keeping the task handle alive for the test.
fn spawn_service() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service should start");
    (handle, task, peer_id)
}

/// The first listen address of `handle`, with `/p2p/<peer-id>` appended so it is
/// directly dialable.
async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> String {
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    loop {
        if let Some(addr) = handle
            .listen_addrs()
            .await
            .ok()
            .and_then(|a| a.into_iter().next())
        {
            return format!("{addr}/p2p/{peer_id}");
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for a listen address"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Two connected services: `caller` has dialed `responder`.
async fn connected_pair() -> (
    NetworkHandle,
    NetworkHandle,
    PeerId,
    Vec<tokio::task::JoinHandle<()>>,
) {
    let (caller, caller_task, _caller_id) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();

    let addr = dialable_addr(&responder, responder_id).await;
    let connected = caller
        .connect_peer(&addr)
        .await
        .expect("loopback dial should succeed");
    assert_eq!(connected, responder_id);

    (
        caller,
        responder,
        responder_id,
        vec![caller_task, responder_task],
    )
}

/// Run `f`, failing the test rather than hanging if it stalls.
async fn within<T>(what: &str, f: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(TEST_TIMEOUT, f)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for: {what}"))
}

// ---------------------------------------------------------------------------
// Round trip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn handle_round_trips_a_unary_call() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move {
            let mut out = b"echo:".to_vec();
            out.extend_from_slice(&data);
            Ok(out)
        })
        .await
        .expect("registering a handler should succeed");

    let response = within(
        "the echo round trip",
        caller.call_unary_handler(responder_id, PROTO, b"hello"),
    )
    .await
    .expect("a registered handler must answer");
    assert_eq!(response, b"echo:hello");
}

/// The caller never dialed the responder: the behaviour must dial on demand
/// using the address Kademlia was seeded with, matching Go's `host.NewStream`.
#[tokio::test]
async fn call_dials_on_demand_when_not_connected() {
    let (caller, _caller_task, _caller_id) = spawn_service();
    let (responder, _responder_task, responder_id) = spawn_service();

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    // Seed only the routing table — no `connect_peer`.
    let addr = dialable_addr(&responder, responder_id).await;
    let stripped: libp2p::Multiaddr = addr
        .parse::<libp2p::Multiaddr>()
        .expect("valid multiaddr")
        .iter()
        .filter(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
        .collect();
    caller
        .add_kad_address(responder_id, stripped)
        .await
        .expect("seed the routing table");

    assert!(
        caller
            .list_peers()
            .await
            .expect("list_peers")
            .iter()
            .all(|p| p.peer_id != responder_id),
        "precondition: the caller must not already be connected"
    );

    let response = within(
        "a dial-on-demand unary call",
        caller.call_unary_handler(responder_id, PROTO, b"cold"),
    )
    .await
    .expect("the call must dial the peer itself");
    assert_eq!(response, b"cold");
}

// ---------------------------------------------------------------------------
// Error mapping
// ---------------------------------------------------------------------------

/// A handler's error arm must arrive as `P2PError::Protocol` carrying the
/// remote's text — callers key their retry logic on that string.
#[tokio::test]
async fn remote_handler_error_maps_to_protocol_error() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    responder
        .add_unary_handler(PROTO, |_data: Vec<u8>| async move {
            Err("expert not loaded".to_string())
        })
        .await
        .expect("register handler");

    let error = within(
        "the remote error arm",
        caller.call_unary_handler(responder_id, PROTO, b"x"),
    )
    .await
    .expect_err("a handler returning Err must surface as an error");

    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("expert not loaded"),
            "the remote's text must be preserved, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }
}

/// Calling a protocol nobody registered is refused during negotiation, so it
/// surfaces as `UnsupportedProtocol` → `P2PError::Protocol` — never as a hang.
#[tokio::test]
async fn calling_an_unregistered_protocol_is_refused() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    // A different protocol is served, proving the refusal is per-protocol.
    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let error = within(
        "the refusal of an unserved protocol",
        caller.call_unary_handler(responder_id, "DHTProtocol.rpc_nonexistent", b"x"),
    )
    .await
    .expect_err("an unserved protocol must be refused");

    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("does not support"),
            "expected an unsupported-protocol message, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }

    // The connection must survive the refusal.
    let response = within(
        "a call after the refusal",
        caller.call_unary_handler(responder_id, PROTO, b"still-alive"),
    )
    .await
    .expect("the connection must remain usable");
    assert_eq!(response, b"still-alive");
}

/// Dialing a peer with no known address fails as `DialFailed` rather than
/// waiting out the request timeout.
#[tokio::test]
async fn calling_an_unreachable_peer_maps_to_dial_failed() {
    let (caller, _task, _id) = spawn_service();
    let stranger = Keypair::generate_ed25519().public().to_peer_id();

    let error = within(
        "the dial failure",
        caller.call_unary_handler(stranger, PROTO, b"x"),
    )
    .await
    .expect_err("a peer with no address must not resolve");
    assert!(
        matches!(error, P2PError::DialFailed(_)),
        "expected P2PError::DialFailed, got {error:?}"
    );
}

// ---------------------------------------------------------------------------
// Registration lifecycle
// ---------------------------------------------------------------------------

#[tokio::test]
async fn removing_a_handler_causes_a_clean_refusal() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let response = within(
        "the call before removal",
        caller.call_unary_handler(responder_id, PROTO, b"before"),
    )
    .await
    .expect("the handler is registered");
    assert_eq!(response, b"before");

    assert!(
        responder
            .remove_unary_handler(PROTO)
            .await
            .expect("remove should reach the service"),
        "removing a registered handler reports true"
    );

    let error = within(
        "the refusal after removal",
        caller.call_unary_handler(responder_id, PROTO, b"after"),
    )
    .await
    .expect_err("a removed handler must no longer answer");
    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("does not support"),
            "removal must produce a negotiation refusal, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }

    // Idempotent: removing again is not an error, just `false`.
    assert!(
        !responder
            .remove_unary_handler(PROTO)
            .await
            .expect("remove should reach the service"),
        "removing an unregistered handler reports false"
    );
}

/// Re-registering a protocol swaps the handler rather than stacking two.
#[tokio::test]
async fn re_registering_replaces_the_handler() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    responder
        .add_unary_handler(PROTO, |_: Vec<u8>| async move { Ok(b"first".to_vec()) })
        .await
        .expect("register the first handler");
    responder
        .add_unary_handler(PROTO, |_: Vec<u8>| async move { Ok(b"second".to_vec()) })
        .await
        .expect("register the replacement");

    let response = within(
        "the replacement handler's answer",
        caller.call_unary_handler(responder_id, PROTO, b"x"),
    )
    .await
    .expect("the protocol is still served");
    assert_eq!(response, b"second", "the most recent registration must win");
}

// ---------------------------------------------------------------------------
// Concurrency
// ---------------------------------------------------------------------------

/// One task per call, not one per handler: a slow call must not delay calls
/// that arrive behind it, and must not stall the event loop.
#[tokio::test]
async fn a_slow_handler_does_not_serialize_other_calls() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let in_flight = Arc::new(AtomicUsize::new(0));
    let max_in_flight = Arc::new(AtomicUsize::new(0));
    let (seen, max_seen) = (Arc::clone(&in_flight), Arc::clone(&max_in_flight));

    responder
        .add_unary_handler(PROTO, move |data: Vec<u8>| {
            let seen = Arc::clone(&seen);
            let max_seen = Arc::clone(&max_seen);
            async move {
                let now = seen.fetch_add(1, Ordering::SeqCst) + 1;
                max_seen.fetch_max(now, Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(300)).await;
                seen.fetch_sub(1, Ordering::SeqCst);
                Ok(data)
            }
        })
        .await
        .expect("register handler");

    const CALLS: usize = 5;
    let calls = (0..CALLS).map(|i| {
        let caller = caller.clone();
        async move {
            caller
                .call_unary_handler(responder_id, PROTO, &[i as u8])
                .await
        }
    });

    let results = within("all concurrent calls", futures::future::join_all(calls)).await;
    for (i, result) in results.into_iter().enumerate() {
        assert_eq!(
            result.expect("every concurrent call must succeed"),
            vec![i as u8]
        );
    }

    assert!(
        max_in_flight.load(Ordering::SeqCst) > 1,
        "calls were serialized: peak concurrency was {}",
        max_in_flight.load(Ordering::SeqCst)
    );

    // The event loop is still responsive.
    assert!(responder.list_peers().await.is_ok());
}

/// Concurrent calls to **different** protocols on one connection must each get
/// their own answer.
///
/// Regression test: `FullyNegotiatedOutbound` events arrive in completion
/// order, not emission order, so replies must be correlated by request id.
/// Calls sharing one protocol cannot catch this — two protocols in flight can,
/// which is the `shard serve` + `storage serve` deployment.
#[tokio::test]
async fn concurrent_calls_to_different_protocols_do_not_cross_talk() {
    const PROTO_A: &str = "DHTProtocol.rpc_store";
    const PROTO_B: &str = "DHTProtocol.rpc_find";

    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    for (proto, tag) in [(PROTO_A, &b"A:"[..]), (PROTO_B, &b"B:"[..])] {
        let tag = tag.to_vec();
        responder
            .add_unary_handler(proto, move |data: Vec<u8>| {
                let tag = tag.clone();
                async move {
                    let mut out = tag.clone();
                    out.extend_from_slice(&data);
                    Ok(out)
                }
            })
            .await
            .expect("register handler");
    }

    let (a, b) = tokio::join!(
        caller.call_unary_handler(responder_id, PROTO_A, b"one"),
        caller.call_unary_handler(responder_id, PROTO_B, b"two"),
    );

    assert_eq!(
        a.expect("the rpc_store call must succeed"),
        b"A:one",
        "each concurrent call must receive its own protocol's response"
    );
    assert_eq!(b.expect("the rpc_find call must succeed"), b"B:two");
}

// ---------------------------------------------------------------------------
// Shutdown
// ---------------------------------------------------------------------------

#[tokio::test]
async fn calls_after_shutdown_error_rather_than_hang() {
    let (caller, task, _id) = spawn_service();
    let stranger = Keypair::generate_ed25519().public().to_peer_id();

    caller.shutdown().await.expect("shutdown ack");
    within("the event loop to exit", task)
        .await
        .expect("the event loop must not panic");

    let error = within(
        "a call against a stopped service",
        caller.call_unary_handler(stranger, PROTO, b"x"),
    )
    .await
    .expect_err("a stopped service cannot serve calls");
    assert!(matches!(error, P2PError::NotInitialized));

    assert!(caller
        .add_unary_handler(PROTO, |d: Vec<u8>| async move { Ok(d) })
        .await
        .is_err());
    assert!(caller.remove_unary_handler(PROTO).await.is_err());
}

// ---------------------------------------------------------------------------
// Routed dial: calling a peer we have no addresses for at all
// ---------------------------------------------------------------------------

/// Poll `handle.routing_peers()` until it contains `peer`.
async fn wait_in_routing_table(handle: &NetworkHandle, peer: PeerId) {
    loop {
        if handle
            .routing_peers()
            .await
            .expect("routing_peers")
            .contains(&peer)
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// The caller knows nothing about the responder — no connection, no seeded
/// address — only a shared bootstrap. The service must resolve the peer through
/// a DHT walk before dialing (Go's *routed host* semantics), because a petals
/// `ServerInfo` record carries a peer ID and nothing else.
#[tokio::test]
async fn call_to_unconnected_peer_resolves_through_the_dht() {
    let (bootstrap, bootstrap_task, bootstrap_id) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();
    let (caller, caller_task, _caller_id) = spawn_service();
    let _tasks = [bootstrap_task, responder_task, caller_task];

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
    responder
        .connect_peer(&bootstrap_addr)
        .await
        .expect("responder dials bootstrap");
    caller
        .connect_peer(&bootstrap_addr)
        .await
        .expect("caller dials bootstrap");

    // The walk can only find the responder once the bootstrap's routing table
    // has it (fed by identify) and the caller's has the bootstrap.
    within(
        "the bootstrap to learn the responder",
        wait_in_routing_table(&bootstrap, responder_id),
    )
    .await;
    within(
        "the caller to learn the bootstrap",
        wait_in_routing_table(&caller, bootstrap_id),
    )
    .await;

    let response = within(
        "the routed round trip",
        caller.call_unary_handler(responder_id, PROTO, b"hello"),
    )
    .await
    .expect("a call to an unconnected peer must resolve through the DHT");
    assert_eq!(response, b"hello");
}

/// A peer nobody has ever heard of: the walk completes empty and the call must
/// fail with a clear error, not hang or surface a bare-`/p2p/` transport error.
#[tokio::test]
async fn call_to_unknown_peer_fails_cleanly_after_the_walk() {
    let (bootstrap, bootstrap_task, bootstrap_id) = spawn_service();
    let (caller, caller_task, _caller_id) = spawn_service();
    let _tasks = [bootstrap_task, caller_task];

    caller
        .connect_peer(&dialable_addr(&bootstrap, bootstrap_id).await)
        .await
        .expect("caller dials bootstrap");
    within(
        "the caller to learn the bootstrap",
        wait_in_routing_table(&caller, bootstrap_id),
    )
    .await;

    let ghost = Keypair::generate_ed25519().public().to_peer_id();
    let error = within(
        "the failed walk",
        caller.call_unary_handler(ghost, PROTO, b"x"),
    )
    .await
    .expect_err("an unknown peer cannot be called");
    match error {
        P2PError::DialFailed(text) => assert!(
            text.contains("peer not found in DHT"),
            "unexpected error text: {text}"
        ),
        other => panic!("expected DialFailed, got {other:?}"),
    }
}

/// Same, from a completely isolated node: the walk has no peers to ask and
/// completes immediately — the call must still resolve, not hang.
#[tokio::test]
async fn call_from_isolated_node_fails_cleanly() {
    let (caller, _caller_task, _caller_id) = spawn_service();

    let ghost = Keypair::generate_ed25519().public().to_peer_id();
    let error = within(
        "the empty walk",
        caller.call_unary_handler(ghost, PROTO, b"x"),
    )
    .await
    .expect_err("an isolated node cannot reach anyone");
    assert!(
        matches!(error, P2PError::DialFailed(_)),
        "expected DialFailed, got {error:?}"
    );
}

/// `shard run` pre-connects each chain entry with a bare `/p2p/<id>` address —
/// no transport component at all. Go's daemon resolved that through the DHT;
/// the native service must do the same rather than dial an empty address (or,
/// worse, poison the routing table with one).
#[tokio::test]
async fn connect_by_bare_peer_id_resolves_through_the_dht() {
    let (bootstrap, bootstrap_task, bootstrap_id) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();
    let (caller, caller_task, _caller_id) = spawn_service();
    let _tasks = [bootstrap_task, responder_task, caller_task];

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
    responder
        .connect_peer(&bootstrap_addr)
        .await
        .expect("responder dials bootstrap");
    caller
        .connect_peer(&bootstrap_addr)
        .await
        .expect("caller dials bootstrap");

    within(
        "the bootstrap to learn the responder",
        wait_in_routing_table(&bootstrap, responder_id),
    )
    .await;
    within(
        "the caller to learn the bootstrap",
        wait_in_routing_table(&caller, bootstrap_id),
    )
    .await;

    let connected = within(
        "the routed connect",
        caller.connect_peer(&format!("/p2p/{}", responder_id.to_base58())),
    )
    .await
    .expect("a bare /p2p/ connect must resolve through the DHT");
    assert_eq!(connected, responder_id);

    // The connection is real: a call over it round-trips.
    let response = within(
        "a call over the routed connection",
        caller.call_unary_handler(responder_id, PROTO, b"ping"),
    )
    .await
    .expect("the routed connection must carry calls");
    assert_eq!(response, b"ping");
}

// ---------------------------------------------------------------------------
// Routed dial: bad addresses must not end the attempt
// ---------------------------------------------------------------------------

/// Strip `/p2p/<id>` so the address is shaped the way kad stores it.
fn stripped(addr: &str) -> libp2p::Multiaddr {
    addr.parse::<libp2p::Multiaddr>()
        .expect("valid multiaddr")
        .iter()
        .filter(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
        .collect()
}

/// A dead table entry is the normal case for a NATed peer: kad holds its
/// listen addresses, and only its DHT record has the relay circuit that works.
#[tokio::test]
async fn a_routed_call_falls_back_to_the_dht_when_the_known_address_is_dead() {
    let (bootstrap, bootstrap_task, bootstrap_id) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();
    let (caller, caller_task, _caller_id) = spawn_service();
    let _tasks = [bootstrap_task, responder_task, caller_task];

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;
    responder
        .connect_peer(&bootstrap_addr)
        .await
        .expect("responder dials bootstrap");
    caller
        .connect_peer(&bootstrap_addr)
        .await
        .expect("caller dials bootstrap");

    within(
        "the bootstrap to learn the responder",
        wait_in_routing_table(&bootstrap, responder_id),
    )
    .await;
    within(
        "the caller to learn the bootstrap",
        wait_in_routing_table(&caller, bootstrap_id),
    )
    .await;

    // Seeded last so the bootstrap handshake above cannot overwrite it.
    let dead = format!("/ip4/127.0.0.1/tcp/{}", free_port());
    caller
        .add_kad_address(responder_id, stripped(&dead))
        .await
        .expect("seed the dead address");
    assert!(
        caller
            .list_peers()
            .await
            .expect("list_peers")
            .iter()
            .all(|p| p.peer_id != responder_id),
        "precondition: the caller must not already be connected to the responder",
    );

    let response = within(
        "a routed call that must survive a dead address",
        caller.call_unary_handler(responder_id, PROTO, b"fallback"),
    )
    .await
    .expect("the dead address must not end the attempt");
    assert_eq!(response, b"fallback");
}

/// One of *our own* addresses, filed in kad under someone else's peer id, must
/// never be offered as a way to reach them: dialing it fails `WrongPeerId`.
#[tokio::test]
async fn our_own_address_filed_under_another_peer_is_not_a_route_to_them() {
    let (caller, _caller_task, caller_id) = spawn_service();
    let ghost_id = Keypair::generate_ed25519().public().to_peer_id();

    let own = dialable_addr(&caller, caller_id).await;
    caller
        .add_kad_address(ghost_id, stripped(&own))
        .await
        .expect("seed our own address under the ghost's id");

    let routes = caller
        .dht_find_peer(ghost_id)
        .await
        .expect("dht_find_peer should answer");
    assert!(
        routes.is_empty(),
        "our own address is not a route to anyone else, but was offered: {routes:?}",
    );
}

/// The same invariant on the dial path: a peer filed under both our address
/// and its own must be dialled only at its own. No bootstrap here, so a
/// self-dial has no DHT walk to recover with and the call fails.
#[tokio::test]
async fn a_routed_call_never_dials_our_own_address_even_when_kad_holds_it() {
    let (caller, _caller_task, caller_id) = spawn_service();
    let (responder, _responder_task, responder_id) = spawn_service();

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let own = dialable_addr(&caller, caller_id).await;
    let real = dialable_addr(&responder, responder_id).await;
    // Ours first, so anything dialling the table in order tries it first.
    caller
        .add_kad_address(responder_id, stripped(&own))
        .await
        .expect("seed our own address under the responder's id");
    caller
        .add_kad_address(responder_id, stripped(&real))
        .await
        .expect("seed the responder's real address");

    let response = within(
        "a routed call with our own address filed under the callee",
        caller.call_unary_handler(responder_id, PROTO, b"not-us"),
    )
    .await
    .expect("the call must reach the responder without a self-dial");
    assert_eq!(response, b"not-us");
}

/// The complement: the filter is `is_ours`, not "is it loopback", so a sibling
/// node on the same interface stays reachable.
#[tokio::test]
async fn a_second_local_node_on_another_port_is_still_reachable() {
    let (caller, _caller_task, _caller_id) = spawn_service();
    let (responder, _responder_task, responder_id) = spawn_service();

    responder
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register handler");

    let responder_addr = dialable_addr(&responder, responder_id).await;
    caller
        .add_kad_address(responder_id, stripped(&responder_addr))
        .await
        .expect("seed the sibling's loopback address");

    let response = within(
        "a call to a second node on the same machine",
        caller.call_unary_handler(responder_id, PROTO, b"sibling"),
    )
    .await
    .expect("a sibling node's loopback address is a real route");
    assert_eq!(response, b"sibling");
}

/// A port nothing is listening on: bound and released, so a small race window.
fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .expect("binding an ephemeral port")
        .local_addr()
        .expect("local addr")
        .port()
}

/// A peer whose lookup finds nothing better keeps its routing-table entry: the
/// eviction that lets the walk re-dial it is provisional, not a forgetting.
#[tokio::test]
async fn an_evicted_entry_is_restored_when_the_lookup_finds_nothing_better() {
    let (caller, _caller_task, _caller_id) = spawn_service();
    let (bootstrap, _bootstrap_task, bootstrap_id) = spawn_service();
    let absent_id = Keypair::generate_ed25519().public().to_peer_id();

    // Somebody to ask, who has never heard of the target.
    caller
        .connect_peer(&dialable_addr(&bootstrap, bootstrap_id).await)
        .await
        .expect("caller dials bootstrap");
    within(
        "the caller to learn the bootstrap",
        wait_in_routing_table(&caller, bootstrap_id),
    )
    .await;

    let dead = format!("/ip4/127.0.0.1/tcp/{}", free_port());
    caller
        .add_kad_address(absent_id, stripped(&dead))
        .await
        .expect("seed the dead address");

    let err = within(
        "a routed call to an unreachable peer",
        caller.call_unary_handler(absent_id, PROTO, b"nobody"),
    )
    .await
    .expect_err("nothing answers at the dead address and nobody knows the peer");
    assert!(matches!(err, P2PError::DialFailed(_)), "{err:?}");

    assert!(
        caller
            .routing_peers()
            .await
            .expect("routing_peers")
            .contains(&absent_id),
        "the failed lookup must hand the old entry back, not leave the peer forgotten",
    );
}
