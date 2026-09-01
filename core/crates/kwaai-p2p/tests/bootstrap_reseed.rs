//! A node must recover a bootstrap whose routing-table entry was evicted.
//!
//! The wedge this guards against: configured bootstrap addresses used to be
//! dialled exactly once, at startup. Announces and routed dials go by peer id,
//! the periodic kad refresh only walks peers still in the table, and identify
//! needs a live connection — so once a bootstrap's entry was evicted (dial
//! failures during its restart window, bucket replacement on a busy DHT),
//! nothing could ever reach it again. The node kept listening but stopped
//! announcing and reserving until *it* was restarted. Observed on the live
//! fleet with v0.6.3 daemons after a bootstrap restart.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, Multiaddr, PeerId};

const SETTLE_TIMEOUT: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(50);
const TICK: Duration = Duration::from_millis(500);

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

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

async fn tabled(handle: &NetworkHandle, peer: PeerId) -> bool {
    handle
        .routing_peers()
        .await
        .expect("routing_peers should answer")
        .contains(&peer)
}

#[tokio::test]
async fn an_evicted_bootstrap_is_reseeded_while_it_is_unreachable() {
    let bootstrap_key = Keypair::generate_ed25519();
    let bootstrap_id = bootstrap_key.public().to_peer_id();
    let (bootstrap, bootstrap_task) =
        NetworkService::spawn(NetworkConfig::for_tests(), bootstrap_key)
            .expect("bootstrap swarm should start");
    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;

    // `initial_peers`, not just the argument to `bootstrap()` below: the
    // re-seed list is built from the node's own config, so that a caller's
    // dial set — which on the native node also carries the peer cache's
    // remembered peers — cannot enrol a hundred strangers as bootstraps.
    let node_config = NetworkConfig {
        initial_peers: vec![bootstrap_addr.to_string()],
        kad_maintenance_interval: TICK,
        ..NetworkConfig::for_tests()
    };
    let (node, _node_task) = NetworkService::spawn(node_config, Keypair::generate_ed25519())
        .expect("node swarm should start");

    node.bootstrap(vec![bootstrap_addr])
        .await
        .expect("bootstrap dial should succeed");
    eventually("the bootstrap to enter the routing table", || async {
        tabled(&node, bootstrap_id).await.then_some(())
    })
    .await;

    // Take the bootstrap away *before* evicting it. This is what makes the
    // assertion below mean anything: with the bootstrap up, the live
    // connection's identify re-adds it to the routing table within
    // milliseconds, and the test then passes whether or not the re-seed
    // exists. Down, there is no connection, no identify and no inbound dial,
    // so the configured address is the only thing left that can supply it.
    bootstrap
        .shutdown()
        .await
        .expect("bootstrap should shut down");
    let _ = bootstrap_task.await;
    eventually(
        "the node to lose its connection to the bootstrap",
        || async { node.list_peers().await.ok()?.is_empty().then_some(()) },
    )
    .await;

    // Checked by return value rather than a follow-up `routing_peers()`: a
    // maintenance tick landing between the two calls would re-seed the entry
    // and fail the assertion against *correct* code.
    assert!(
        node.drop_routing_entry(bootstrap_id)
            .await
            .expect("eviction hook should reach the service"),
        "the bootstrap should have been in the routing table to evict"
    );

    eventually("the maintenance tick to re-seed the bootstrap", || async {
        tabled(&node, bootstrap_id).await.then_some(())
    })
    .await;
}

#[tokio::test]
async fn a_dial_set_peer_that_is_not_configured_is_never_reseeded() {
    // The scope guard. `NativeNode::start` hands `bootstrap()` its configured
    // peers *plus* up to `MAX_CACHED_PEERS` remembered ones; most of those are
    // NATed or long gone, so treating the dial set as the bootstrap list turns
    // every tick into a dial and log burst that also feeds unreachable peers
    // back into the routing table for this node to serve to others.
    let stranger_key = Keypair::generate_ed25519();
    let stranger_id = stranger_key.public().to_peer_id();
    let (stranger, stranger_task) = NetworkService::spawn(NetworkConfig::for_tests(), stranger_key)
        .expect("stranger swarm should start");
    let stranger_addr = dialable_addr(&stranger, stranger_id).await;

    // Configured with nothing; the stranger arrives only via the dial set.
    let node_config = NetworkConfig {
        kad_maintenance_interval: TICK,
        ..NetworkConfig::for_tests()
    };
    let (node, _node_task) = NetworkService::spawn(node_config, Keypair::generate_ed25519())
        .expect("node swarm should start");

    node.bootstrap(vec![stranger_addr])
        .await
        .expect("dial should succeed");
    eventually("the stranger to enter the routing table", || async {
        tabled(&node, stranger_id).await.then_some(())
    })
    .await;

    stranger
        .shutdown()
        .await
        .expect("stranger should shut down");
    let _ = stranger_task.await;
    eventually(
        "the node to lose its connection to the stranger",
        || async { node.list_peers().await.ok()?.is_empty().then_some(()) },
    )
    .await;
    assert!(
        node.drop_routing_entry(stranger_id)
            .await
            .expect("eviction hook should reach the service"),
        "the stranger should have been in the routing table to evict"
    );

    // Several ticks' worth. Nothing should bring it back.
    tokio::time::sleep(TICK * 4).await;
    assert!(
        !tabled(&node, stranger_id).await,
        "a peer that was only ever in the dial set must not be re-seeded as a bootstrap"
    );
}
