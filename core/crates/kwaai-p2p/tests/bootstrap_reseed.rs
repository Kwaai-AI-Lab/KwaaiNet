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

#[tokio::test]
async fn an_evicted_bootstrap_is_reseeded_by_the_maintenance_tick() {
    let bootstrap_key = Keypair::generate_ed25519();
    let bootstrap_id = bootstrap_key.public().to_peer_id();
    let (bootstrap, _bootstrap_task) =
        NetworkService::spawn(NetworkConfig::for_tests(), bootstrap_key)
            .expect("bootstrap swarm should start");
    let bootstrap_addr = dialable_addr(&bootstrap, bootstrap_id).await;

    // The node ticks fast so the test does not wait out the 5-minute default.
    let node_config = NetworkConfig {
        kad_maintenance_interval: Duration::from_millis(500),
        ..NetworkConfig::for_tests()
    };
    let (node, _node_task) = NetworkService::spawn(node_config, Keypair::generate_ed25519())
        .expect("node swarm should start");

    node.bootstrap(vec![bootstrap_addr])
        .await
        .expect("bootstrap dial should succeed");
    eventually("the bootstrap to enter the routing table", || async {
        node.routing_peers()
            .await
            .ok()?
            .contains(&bootstrap_id)
            .then_some(())
    })
    .await;

    // Evict it, standing in for what a restart window does on a busy DHT.
    node.drop_routing_entry(bootstrap_id)
        .await
        .expect("eviction hook should reach the service");
    assert!(
        !node
            .routing_peers()
            .await
            .expect("routing_peers should answer")
            .contains(&bootstrap_id),
        "eviction should have removed the bootstrap from the table"
    );

    // Nothing else knows this bootstrap: only the configured-address re-seed
    // can bring it back. Before the fix this timed out.
    eventually("the maintenance tick to re-seed the bootstrap", || async {
        node.routing_peers()
            .await
            .ok()?
            .contains(&bootstrap_id)
            .then_some(())
    })
    .await;
}
