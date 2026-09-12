//! In-process swarm tests.
//!
//! Every swarm here binds an ephemeral port on 127.0.0.1 with a freshly
//! generated key — no shared sockets, no on-disk identity, nothing that can
//! collide with a node running on the same machine.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use kwaai_p2p::{Direction, NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, Multiaddr, PeerId};

/// Ceiling on any "wait for the network to settle" loop. Loopback is fast; a
/// test that needs longer than this is failing, not slow.
const SETTLE_TIMEOUT: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Spin up a loopback swarm and return its handle plus the task, so the caller
/// can keep the task alive for the duration of the test.
fn spawn_test_swarm() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("swarm should start");
    (handle, task, peer_id)
}

/// Poll `f` until it yields `Some`, or fail the test after [`SETTLE_TIMEOUT`].
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

/// The first listen address a swarm reports, with `/p2p/<peer-id>` appended so
/// it is directly dialable.
async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

// ---------------------------------------------------------------------------
// Two-swarm: dial, identify, list_peers, disconnect
// ---------------------------------------------------------------------------

#[tokio::test]
async fn two_swarms_dial_identify_and_disconnect() {
    let (alice, _alice_task, alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    assert_ne!(alice_id, bob_id, "swarms must have distinct identities");
    assert_eq!(alice.peer_id(), alice_id);
    assert_eq!(alice.local_peer_id(), alice_id.to_base58());

    let bob_addr = dialable_addr(&bob, bob_id).await;

    // --- dial ---------------------------------------------------------
    let connected = alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial to a loopback peer should succeed");
    assert_eq!(connected, bob_id, "connect_peer returns the dialed peer id");

    // --- list_peers, both sides --------------------------------------
    let alice_view = eventually("alice to list bob", || async {
        let peers = alice.list_peers().await.ok()?;
        peers.into_iter().find(|p| p.peer_id == bob_id)
    })
    .await;
    assert_eq!(
        alice_view.direction,
        Direction::Outbound,
        "alice dialed, so her side is outbound"
    );

    let bob_view = eventually("bob to list alice", || async {
        let peers = bob.list_peers().await.ok()?;
        peers.into_iter().find(|p| p.peer_id == alice_id)
    })
    .await;
    assert_eq!(
        bob_view.direction,
        Direction::Inbound,
        "bob was dialed, so his side is inbound"
    );

    // --- identify: each side learns an observed address for itself ----
    // Bob observes Alice's ephemeral source port; Alice observes the address
    // Bob saw her connect from. Both are evidence the identify exchange ran.
    let alice_observed = eventually("alice to receive an observed address", || async {
        let addrs = alice.observed_addrs().await.ok()?;
        (!addrs.is_empty()).then_some(addrs)
    })
    .await;
    let (addr, observers) = &alice_observed[0];
    assert_eq!(*observers, 1, "exactly one peer has observed us so far");
    assert!(
        addr.to_string().contains("/ip4/127.0.0.1/"),
        "observed address should be loopback, got {addr}"
    );

    eventually("bob to receive an observed address", || async {
        let addrs = bob.observed_addrs().await.ok()?;
        (!addrs.is_empty()).then_some(())
    })
    .await;

    // --- disconnect ---------------------------------------------------
    alice
        .disconnect_peer(bob_id)
        .await
        .expect("disconnecting a connected peer should succeed");

    eventually("alice's peer list to drop bob", || async {
        let peers = alice.list_peers().await.ok()?;
        peers.iter().all(|p| p.peer_id != bob_id).then_some(())
    })
    .await;

    eventually("bob's peer list to drop alice", || async {
        let peers = bob.list_peers().await.ok()?;
        peers.iter().all(|p| p.peer_id != alice_id).then_some(())
    })
    .await;
}

// ---------------------------------------------------------------------------
// Three-swarm Kademlia: A knows B, B knows C, A resolves C
// ---------------------------------------------------------------------------

#[tokio::test]
async fn kad_resolves_a_peer_two_hops_away() {
    let (a, _a_task, a_id) = spawn_test_swarm();
    let (b, _b_task, b_id) = spawn_test_swarm();
    let (c, _c_task, c_id) = spawn_test_swarm();

    let b_addr = dialable_addr(&b, b_id).await;
    let c_addr = dialable_addr(&c, c_id).await;

    // A knows B.
    a.connect_peer(&b_addr.to_string()).await.expect("A → B");
    // B knows C.
    b.connect_peer(&c_addr.to_string()).await.expect("B → C");

    // Let identify populate each side's kad routing table with the peers'
    // *listen* addresses (not the ephemeral dial source ports).
    eventually("B's routing table to learn C", || async {
        let addrs = b.dht_find_peer(c_id).await.ok()?;
        (!addrs.is_empty()).then_some(())
    })
    .await;

    // A has never seen C. The lookup must walk to B and come back with C's
    // address before A can dial it.
    let c_addrs = eventually("A to resolve C through the DHT", || async {
        let addrs = a.dht_find_peer(c_id).await.ok()?;
        (!addrs.is_empty()).then_some(addrs)
    })
    .await;

    assert!(
        c_addrs
            .iter()
            .any(|addr| addr.to_string().contains("/tcp/")),
        "resolved addresses should be dialable TCP addrs: {c_addrs:?}"
    );

    // Prove the resolution is real: dial C using only what the DHT returned.
    let resolved = c_addrs[0]
        .clone()
        .with(libp2p::multiaddr::Protocol::P2p(c_id));
    let connected = a
        .connect_peer(&resolved.to_string())
        .await
        .expect("dialing the DHT-resolved address should work");
    assert_eq!(connected, c_id);
    assert_ne!(a_id, c_id);
}

// ---------------------------------------------------------------------------
// Kad protocol migration: legacy-only ↔ dual-default ↔ kwaai-only
// ---------------------------------------------------------------------------

/// Like [`spawn_test_swarm`] but with an explicit kad protocol list. A
/// single foreign name works on every build; the multi-name bridging node
/// needs the patched setter and the `kad-multi-protocol` feature.
fn spawn_swarm_with_kad_protocols(
    protocols: &[&str],
) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let config = NetworkConfig {
        kad_protocols: protocols
            .iter()
            .map(|s| libp2p::StreamProtocol::try_from_owned((*s).to_string()).expect("valid"))
            .collect(),
        ..NetworkConfig::for_tests()
    };
    let (handle, task) = NetworkService::spawn(config, keypair).expect("swarm should start");
    (handle, task, peer_id)
}

/// The cutover topology in miniature — with the probe placed where the
/// short-circuit cannot reach it. A speaks only the legacy `/ipfs/kad/1.0.0`
/// (a node predating the kwaai name), B runs this build's dual set (a
/// migration-window bootstrap), C and D speak only `/kwaai/kad/1.0.0`.
///
/// The previous version of this test asked B for C — but `dial()` seeds the
/// routing table for every dialled peer and `DhtFindPeer` short-circuits on
/// any known address, so that resolved from B's own dial records without a
/// kad substream ever opening; it passed with B legacy-only, i.e. with the
/// patched multi-name negotiation deleted (found in review). The probes are
/// now for peers the caller has *never dialled*, so each answer can only
/// come from a real FIND_NODE round trip:
///
/// - `b.dht_find_peer(d)` — B has no address for D; only C knows D, so the
///   walk must negotiate `/kwaai/kad/1.0.0` with C.
/// - `a.dht_find_peer(d)` — A must first negotiate the legacy name with B,
///   and the answer still only exists on the far side of B↔C.
#[cfg(feature = "kad-multi-protocol")]
#[tokio::test]
async fn kad_negotiates_across_the_protocol_migration() {
    let (a, _a_task, _a_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::LEGACY_KAD_PROTOCOL]);
    let (b, _b_task, b_id) = spawn_test_swarm();
    let (c, _c_task, c_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (d, _d_task, d_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);

    let b_addr = dialable_addr(&b, b_id).await;
    let c_addr = dialable_addr(&c, c_id).await;
    let d_addr = dialable_addr(&d, d_id).await;

    a.connect_peer(&b_addr.to_string()).await.expect("A → B");
    b.connect_peer(&c_addr.to_string()).await.expect("B → C");
    c.connect_peer(&d_addr.to_string()).await.expect("C → D");

    // The kwaai leg: B has never dialled D, so this is a genuine walk that
    // must open a kad substream to C and negotiate the kwaai name.
    let d_addrs = eventually("B to resolve D through C over /kwaai/kad", || async {
        let addrs = b.dht_find_peer(d_id).await.ok()?;
        (!addrs.is_empty()).then_some(addrs)
    })
    .await;
    assert!(
        d_addrs
            .iter()
            .any(|addr| addr.to_string().contains("/tcp/")),
        "resolved addresses should be dialable TCP addrs: {d_addrs:?}"
    );

    // The legacy leg, end to end: A's only kad peer is B (legacy name), and
    // D's address only exists beyond B's kwaai leg.
    eventually("A to resolve D through B on the legacy name", || async {
        let addrs = a.dht_find_peer(d_id).await.ok()?;
        (!addrs.is_empty()).then_some(())
    })
    .await;
}

/// The negative control that makes the test above falsifiable: identical
/// topology, but the bridge speaks only the legacy name — the state the
/// fleet is in when the patched `set_protocol_names` stops working. B and C
/// share no kad protocol, so B must NOT be able to resolve D. If this probe
/// ever yields addresses, the short-circuit is back and the migration test
/// proves nothing again.
#[cfg(feature = "kad-multi-protocol")]
#[tokio::test]
async fn a_legacy_only_bridge_cannot_resolve_across_the_protocol_boundary() {
    let (b, _b_task, _b_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::LEGACY_KAD_PROTOCOL]);
    let (c, _c_task, c_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (d, _d_task, d_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);

    let c_addr = dialable_addr(&c, c_id).await;
    let d_addr = dialable_addr(&d, d_id).await;

    b.connect_peer(&c_addr.to_string()).await.expect("B → C");
    c.connect_peer(&d_addr.to_string()).await.expect("C → D");

    // A failed or timed-out walk is the correct outcome; only a walk that
    // *yields addresses* means the boundary leaked.
    if let Ok(Ok(addrs)) =
        tokio::time::timeout(std::time::Duration::from_secs(8), b.dht_find_peer(d_id)).await
    {
        assert!(
            addrs.is_empty(),
            "B resolved D without a shared kad protocol — the walk was \
             short-circuited: {addrs:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Foreign kad names must never enter the routing table
// ---------------------------------------------------------------------------

/// Wait until A has completed identify with `peer`, so the protocol set is
/// known and any purge it triggers has run in the same handler.
async fn identified(a: &NetworkHandle, peer: PeerId) {
    eventually("identify to land", || async {
        a.list_peers()
            .await
            .ok()?
            .into_iter()
            .find(|p| p.peer_id == peer && p.agent_version.is_some())
            .map(|_| ())
    })
    .await;
}

/// `dial()` seeds the routing table for every dialled peer, before anything
/// is known about it. A peer that then turns out to speak only a foreign kad
/// name (`/ipfs/kad/1.0.0` here — kubo's) must be removed once identify says
/// so, or it sits there Disconnected forever and is served to every
/// FIND_NODE caller. Fails without the purge in `handle_identify_event`.
#[tokio::test]
async fn a_dialled_peer_with_a_foreign_kad_name_is_purged_on_identify() {
    // A is pinned to the kwaai name: the dual-default build would otherwise
    // share the legacy name with B, and B would be a legitimate kad peer.
    let (a, _a_task, _a_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (b, _b_task, b_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::LEGACY_KAD_PROTOCOL]);

    let b_addr = dialable_addr(&b, b_id).await;
    a.connect_peer(&b_addr.to_string()).await.expect("A → B");
    identified(&a, b_id).await;

    let table = a.routing_peers().await.expect("routing peers");
    assert!(
        !table.contains(&b_id),
        "B speaks only {} and must not be in A's routing table: {table:?}",
        kwaai_p2p::LEGACY_KAD_PROTOCOL
    );
}

/// The production shape (2026-09-06 probe): a neighbour serves an address for
/// a peer it never identified, the walk dials it, and V1Lazy reports the kad
/// substream negotiated before the remote has said anything — so kad tables
/// it. That false confirm tabled every kubo peer a bootstrap handed out, and
/// nothing removed them. Here C holds B by operator fiat and never connects
/// to it, so C cannot purge it; once A has identified B, B must be gone.
#[tokio::test]
async fn a_foreign_kad_peer_served_by_a_neighbour_is_never_tabled() {
    // Pinned to the kwaai name for the same reason as the test above.
    let (a, _a_task, _a_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (c, _c_task, c_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (b, _b_task, b_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::LEGACY_KAD_PROTOCOL]);

    let c_addr = dialable_addr(&c, c_id).await;
    let b_listen = eventually("B to report a listen address", || async {
        b.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;

    a.connect_peer(&c_addr.to_string()).await.expect("A → C");
    identified(&a, c_id).await;
    c.add_kad_address(b_id, b_listen)
        .await
        .expect("C learns B by fiat");

    // The walk reaches C, is handed B, and dials B to continue.
    let _ = tokio::time::timeout(SETTLE_TIMEOUT, a.dht_find_peer(b_id)).await;
    identified(&a, b_id).await;

    let table = a.routing_peers().await.expect("routing peers");
    assert!(
        table.contains(&c_id),
        "the kwaai neighbour C must still be tabled: {table:?}"
    );
    assert!(
        !table.contains(&b_id),
        "B was handed to A by C and dialled, but speaks only {} — it must not be tabled: {table:?}",
        kwaai_p2p::LEGACY_KAD_PROTOCOL
    );
}

/// The purge drops table membership, not the operator's address. `connect
/// --addr` is the documented way to reach a legacy-only peer; if the purge
/// discarded that address it would survive only as long as the live
/// connection, and every later call by PeerId would fall to a DHT lookup
/// that cannot find a peer nobody tables. Fails without `pinned_addrs`.
#[tokio::test]
async fn an_operator_supplied_address_outlives_the_purge() {
    const PROTO: &str = "/kwaai/test/echo/1.0.0";
    let (a, _a_task, a_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::KWAAI_KAD_PROTOCOL]);
    let (b, _b_task, b_id) = spawn_swarm_with_kad_protocols(&[kwaai_p2p::LEGACY_KAD_PROTOCOL]);
    b.add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("B serves echo");

    let b_addr = dialable_addr(&b, b_id).await;
    a.connect_peer(&b_addr.to_string()).await.expect("A → B");
    identified(&a, b_id).await;
    assert!(
        !a.routing_peers()
            .await
            .expect("routing peers")
            .contains(&b_id),
        "B must have been purged"
    );

    // Drop the connection so the only way back to B is an address book. B
    // hangs up, not A: the re-dial reuses A's listen port as its source, and
    // the closer's TIME_WAIT on that 4-tuple would refuse it (EADDRINUSE).
    b.disconnect_peer(a_id).await.expect("disconnect");
    eventually("B to leave A's peer list", || async {
        a.list_peers()
            .await
            .ok()?
            .iter()
            .all(|p| p.peer_id != b_id)
            .then_some(())
    })
    .await;

    let reply = tokio::time::timeout(SETTLE_TIMEOUT, a.call_unary_handler(b_id, PROTO, b"ping"))
        .await
        .expect("routed call to settle")
        .expect("B reachable by PeerId from the address the operator gave");
    assert_eq!(reply, b"ping");
}

// ---------------------------------------------------------------------------
// Handle semantics
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dialing_an_unreachable_address_returns_an_error_not_a_hang() {
    let (handle, _task, _id) = spawn_test_swarm();

    // Reserved-for-documentation address with a syntactically valid peer id:
    // the dial must fail, and crucially the pending-dial entry must be cleaned
    // up so the caller is not left awaiting a reply forever.
    let addr = "/ip4/192.0.2.1/tcp/1/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN";
    let result = tokio::time::timeout(SETTLE_TIMEOUT, handle.connect_peer(addr)).await;

    match result {
        Ok(Err(_)) => {} // expected: dial failed, reply delivered
        Ok(Ok(_)) => panic!("dial to 192.0.2.1 should not have succeeded"),
        Err(_) => panic!("connect_peer hung — a pending dial was leaked on the error path"),
    }
}

#[tokio::test]
async fn malformed_multiaddr_is_rejected_before_dialing() {
    let (handle, _task, _id) = spawn_test_swarm();
    assert!(handle.connect_peer("not-a-multiaddr").await.is_err());
    // Still usable afterwards — a bad input must not poison the event loop.
    assert!(handle.list_peers().await.is_ok());
}

#[tokio::test]
async fn disconnecting_an_unconnected_peer_errors() {
    let (handle, _task, _id) = spawn_test_swarm();
    let stranger = Keypair::generate_ed25519().public().to_peer_id();
    assert!(handle.disconnect_peer(stranger).await.is_err());
}

#[tokio::test]
async fn handle_is_clonable_and_shares_one_swarm() {
    let (handle, _task, peer_id) = spawn_test_swarm();
    let clone = handle.clone();
    assert_eq!(clone.peer_id(), peer_id);

    let addrs = dialable_addr(&handle, peer_id).await;
    let via_clone = clone.listen_addrs().await.unwrap();
    assert!(
        via_clone
            .iter()
            .any(|a| addrs.to_string().starts_with(&a.to_string())),
        "both handles must observe the same listeners"
    );
}

#[tokio::test]
async fn bootstrap_with_no_reachable_peers_reports_failure() {
    let (handle, _task, _id) = spawn_test_swarm();
    // Nothing dialable and no peers in the routing table → kad has no peers.
    let result = handle.bootstrap(vec![]).await;
    assert!(
        result.is_err(),
        "bootstrapping with an empty peer set should fail"
    );
}

#[tokio::test]
async fn bootstrap_dials_initial_peers() {
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();
    let bob_addr = dialable_addr(&bob, bob_id).await;

    alice
        .bootstrap(vec![bob_addr])
        .await
        .expect("bootstrap with one reachable peer should succeed");

    eventually("alice to connect to her bootstrap peer", || async {
        let peers = alice.list_peers().await.ok()?;
        peers.iter().any(|p| p.peer_id == bob_id).then_some(())
    })
    .await;
}

#[tokio::test]
async fn shutdown_stops_the_event_loop() {
    let (handle, task, _id) = spawn_test_swarm();
    handle.shutdown().await.expect("shutdown ack");

    tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .expect("event loop should exit promptly after shutdown")
        .expect("event loop should not panic");

    // Commands after shutdown fail rather than hang.
    assert!(handle.list_peers().await.is_err());
}

// ---------------------------------------------------------------------------
// NAT-traversal behaviours are present and do not disturb the base swarm
// ---------------------------------------------------------------------------

#[tokio::test]
async fn nat_behaviours_advertise_their_protocols_over_identify() {
    // The behaviours are inert at this point — nothing drives reservations or
    // acts on an AutoNAT verdict yet — but they are *composed in*, and identify
    // is where that becomes observable to another node. This is also the exact
    // signal `bootstraps_protocol_list_snapshot` reads off the live network, so
    // the two use one mechanism.
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();

    // Bob runs a hop server, which the default test config leaves off.
    let keypair = Keypair::generate_ed25519();
    let bob_id = keypair.public().to_peer_id();
    let (bob, _bob_task) = NetworkService::spawn(
        NetworkConfig {
            relay_server: true,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("relay-serving swarm should start");

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");

    let protocols = eventually("alice to receive bob's protocol list", || async {
        alice.peer_protocols(bob_id).await.ok().flatten()
    })
    .await;

    let has = |name: &str| protocols.iter().any(|p| p == name);
    assert!(has("/libp2p/autonat/1.0.0"), "autonat: {protocols:?}");
    // The client side of relay: this is what a relay needs to see before it
    // will forward a circuit to us.
    assert!(
        has("/libp2p/circuit/relay/0.2.0/stop"),
        "relay stop: {protocols:?}"
    );
    // `/libp2p/dcutr` is deliberately NOT asserted. rust-libp2p 0.53 installs
    // the dcutr handler only on *relayed* connections, so it never appears in
    // an identify exchange over a direct one — unlike go-libp2p, which
    // advertises it statically (the live bootstraps list it; see
    // `live_bootstrap::bootstraps_protocol_list_snapshot`). Nothing depends on
    // us advertising it: dcutr is initiated by the peer holding the relayed
    // *inbound* connection, which negotiates on that connection directly.
    assert!(
        !has("/libp2p/dcutr"),
        "if rust-libp2p starts advertising dcutr on direct connections this note \
         is stale and tests/dcutr.rs should assert it instead: {protocols:?}"
    );
    // And the server side, which only appears because bob toggled it on.
    assert!(
        has("/libp2p/circuit/relay/0.2.0/hop"),
        "relay hop should be advertised when relay_server is set: {protocols:?}"
    );
}

#[tokio::test]
async fn the_hop_protocol_is_absent_when_the_relay_server_is_off() {
    // `for_tests()` turns the hop server off, so the Toggle really has to be
    // toggling something — a `Toggle` that is always enabled would pass the
    // test above and be silently wrong.
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");

    let protocols = eventually("alice to receive bob's protocol list", || async {
        alice.peer_protocols(bob_id).await.ok().flatten()
    })
    .await;

    assert!(
        !protocols
            .iter()
            .any(|p| p == "/libp2p/circuit/relay/0.2.0/hop"),
        "hop must not be advertised with relay_server off: {protocols:?}"
    );
}

#[tokio::test]
async fn a_disconnected_peers_protocol_list_is_dropped() {
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");
    eventually("alice to learn bob's protocols", || async {
        alice.peer_protocols(bob_id).await.ok().flatten()
    })
    .await;

    alice.disconnect_peer(bob_id).await.expect("disconnect");

    // Relay-candidate selection reads this feed; a list left behind for a peer
    // we can no longer reach would send it dialing a ghost.
    eventually("bob's protocol list to be forgotten", || async {
        alice
            .peer_protocols(bob_id)
            .await
            .ok()?
            .is_none()
            .then_some(())
    })
    .await;
}

#[tokio::test]
async fn add_kad_address_makes_a_peer_resolvable_without_a_dial() {
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    let bob_addr = dialable_addr(&bob, bob_id).await;
    let stripped: Multiaddr = bob_addr
        .iter()
        .filter(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
        .collect();

    assert!(
        alice.dht_find_peer(bob_id).await.unwrap().is_empty(),
        "bob should be unknown before add_kad_address"
    );

    alice
        .add_kad_address(bob_id, stripped.clone())
        .await
        .unwrap();

    let addrs = alice.dht_find_peer(bob_id).await.unwrap();
    assert!(
        !addrs.is_empty(),
        "the manually-added address should be resolvable"
    );
}

// ---------------------------------------------------------------------------
// Routing-table poisoning: an address that belongs to somebody else
// ---------------------------------------------------------------------------

/// Regression cover for the 2026-08-10 finding.
///
/// A live address filed in kad under the *wrong* peer id makes every dial to
/// that peer land on whoever actually owns the address. Before the fix the
/// entry survived the failure, so kad handed the same wrong address back on the
/// next attempt and the peer stayed unreachable — in production a stale
/// `/ip4/127.0.0.1/tcp/8080` under metro-win's id meant calls to metro-win hit
/// a different local node forever, and the working circuit address was never
/// tried.
///
/// `known_addresses` cannot prevent this: the address is perfectly routable, and
/// a dial by PeerId never passes through that filter anyway.
#[tokio::test]
async fn an_address_that_answers_with_the_wrong_peer_id_is_evicted() {
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    // A peer that does not exist, pointed at Bob's real address. Dialing it
    // reaches Bob, who reports his own id — exactly the production shape.
    let ghost_id = Keypair::generate_ed25519().public().to_peer_id();
    let bob_addr = dialable_addr(&bob, bob_id).await;
    let stripped: Multiaddr = bob_addr
        .iter()
        .filter(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
        .collect();

    alice
        .add_kad_address(ghost_id, stripped.clone())
        .await
        .unwrap();
    assert!(
        alice.routing_peers().await.unwrap().contains(&ghost_id),
        "the poisoned entry should be in the routing table to begin with"
    );

    // A unary call is the production path: it dispatches by PeerId, so the
    // swarm resolves addresses from kad directly rather than through
    // `known_addresses`. The call is expected to fail — the point is what the
    // failure leaves behind.
    let _ = alice.call_unary_handler(ghost_id, "hello", b"ping").await;

    // The failure is evidence the address is not the ghost's, so it must go.
    // Its only address gone, kad drops the peer with it.
    eventually("the mis-filed address to be evicted", || async {
        let still_there = alice
            .routing_peers()
            .await
            .map(|peers| peers.contains(&ghost_id))
            .unwrap_or(true);
        (!still_there).then_some(())
    })
    .await;
}

// ---------------------------------------------------------------------------
// Redundant dials
// ---------------------------------------------------------------------------

/// Re-dialing a peer we are already connected to must not open a second
/// connection.
///
/// `DialOpts::from(Multiaddr)` builds the unknown-peer-id variant, which
/// hardcodes `PeerCondition::Always`. Against a live network that produced a
/// duplicate connection to every bootstrap on each re-dial — same peer,
/// byte-identical `/dns/` address, indistinguishable in `list_peers` — held
/// until libp2p reaped it on keepalive ~45s later.
#[tokio::test]
async fn re_dialing_a_connected_peer_does_not_open_a_second_connection() {
    let (alice, _alice_task, _alice_id) = spawn_test_swarm();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("first dial");

    let count = || async {
        alice
            .list_peers()
            .await
            .unwrap()
            .into_iter()
            .filter(|p| p.peer_id == bob_id)
            .count()
    };
    assert_eq!(count().await, 1, "one connection after the first dial");

    // Dial the same address again, several times. Each is a no-op.
    for _ in 0..3 {
        let again = alice.connect_peer(&bob_addr.to_string()).await;
        assert_eq!(
            again.expect("a redundant dial reports the peer, not an error"),
            bob_id,
            "the caller asked to be connected and is — that is success",
        );
    }

    // Give any dial that did slip through time to establish before counting.
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert_eq!(
        count().await,
        1,
        "re-dialing an already-connected peer must not add connections",
    );
}

// ---------------------------------------------------------------------------
// AutoNAT dial-back
// ---------------------------------------------------------------------------

/// `io::Write` into a shared buffer, for [`capture_logs`].
#[derive(Clone)]
struct LogSink(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for LogSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Capture this thread's `debug!` output. `#[tokio::test]` is a current-thread
/// runtime, so the swarms spawned on it log here too — and a thread-local
/// default cannot see, or be seen by, the other tests.
fn capture_logs() -> (tracing::subscriber::DefaultGuard, LogSink) {
    let sink = LogSink(Arc::new(Mutex::new(Vec::new())));
    let writer = sink.clone();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(move || writer.clone())
        .finish();
    (tracing::subscriber::set_default(subscriber), sink)
}

/// An AutoNAT dial-back must not be left open beside the connection it
/// duplicates.
///
/// The server side of a probe dials with `PeerCondition::Always` and
/// `allocate_new_port()` — a fresh dial is the only thing that proves
/// reachability — and then never closes the result. Nothing else does either:
/// identify refreshes every connection at half the idle timeout, so the
/// duplicate outlives any reaping. Measured on the nat-test bed, every node
/// held two byte-identical direct connections to each peer that had probed it.
#[tokio::test]
async fn an_autonat_dial_back_does_not_leave_a_duplicate_connection() {
    let (_guard, logs) = capture_logs();
    let (bob, _bob_task, bob_id) = spawn_test_swarm();
    // Stagger the two `boot_delay`s. Fired together, one side's probe request
    // can be routed over the other's millisecond-long dial-back and die with
    // it; autonat then retries only after 30 s, past `SETTLE_TIMEOUT`.
    tokio::time::sleep(Duration::from_secs(1)).await;
    let (alice, _alice_task, alice_id) = spawn_test_swarm();

    // Alice's listen address is what she asks bob to dial back, so she must be
    // listening before she probes.
    let _alice_addr = dialable_addr(&alice, alice_id).await;
    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");

    // A closed dial-back never reaches `list_peers`, so the count cannot be
    // seen rising; the close is logged instead, and that is the evidence the
    // probe ran at all. Both sides probe each other, and bob does hold alice's
    // dial-back to *him* until she closes it, so wait for both.
    let closed_dialback = |peer: PeerId| {
        let logs = logs.clone();
        move || {
            let logs = logs.clone();
            async move {
                let text = String::from_utf8_lossy(&logs.0.lock().unwrap()).into_owned();
                text.lines()
                    .any(|l| {
                        l.contains("closing autonat dial-back") && l.contains(&peer.to_string())
                    })
                    .then_some(())
            }
        }
    };
    eventually(
        "bob to close his dial-back to alice",
        closed_dialback(alice_id),
    )
    .await;
    eventually(
        "alice to close her dial-back to bob",
        closed_dialback(bob_id),
    )
    .await;

    let connections_to_alice = || async {
        let peers = bob.list_peers().await.ok()?;
        Some(peers.into_iter().filter(|p| p.peer_id == alice_id).count())
    };
    eventually("bob to hold exactly one connection to alice", || async {
        (connections_to_alice().await? == 1).then_some(())
    })
    .await;
    // Nothing else is in flight now, so a second sample must agree.
    tokio::time::sleep(POLL_INTERVAL * 4).await;
    assert_eq!(
        connections_to_alice().await,
        Some(1),
        "the dial-back must be closed, not held beside the connection it duplicates",
    );

    // The dial-back address is a remote claim like any other: loopback is not
    // announceable, so it must not have seeded bob's routing table.
    assert!(
        !bob.routing_peers().await.unwrap().contains(&alice_id),
        "a loopback dial-back address must not enter the routing table"
    );
}
