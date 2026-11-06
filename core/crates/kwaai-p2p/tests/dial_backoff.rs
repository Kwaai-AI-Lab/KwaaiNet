//! Dial backoff through the full service: a failed routed dial suppresses the
//! next DHT lookup, but an explicit request still gets one dial.

use std::time::Duration;

use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use libp2p::identity::Keypair;

const TEST_TIMEOUT: Duration = Duration::from_secs(20);

/// A loopback service whose only bootstrap is a port nothing listens on.
fn spawn_isolated() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let mut config = NetworkConfig::for_tests();
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
        assert!(tokio::time::Instant::now() < deadline, "no listen address");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn within<T>(what: &str, f: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(TEST_TIMEOUT, f)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for: {what}"))
}

#[tokio::test]
async fn a_peer_the_dht_could_not_find_is_not_looked_up_again() {
    let (a, _task, _) = spawn_isolated();
    let ghost = PeerId::random();
    let bare = format!("/p2p/{ghost}");

    let first = within("the first lookup", a.connect_peer(&bare))
        .await
        .expect_err("nothing can reach a random peer");
    assert!(
        !first.to_string().contains("suppressed"),
        "the first request must run the lookup: {first}"
    );

    let second = within("the backed-off request", a.connect_peer(&bare))
        .await
        .expect_err("still unreachable");
    assert!(
        second.to_string().contains("suppressed"),
        "a peer in backoff must not buy another lookup: {second}"
    );
}

#[tokio::test]
async fn an_explicit_request_still_dials_a_backed_off_peer_once() {
    let (a, _a_task, _) = spawn_isolated();
    let (b, _b_task, b_id) = spawn_isolated();
    let b_addr = first_listen_addr(&b).await;

    // A dead address puts B in backoff.
    within(
        "the failing connect",
        a.connect_peer_with_addrs(b_id, vec!["/ip4/127.0.0.1/tcp/1".parse().unwrap()]),
    )
    .await
    .expect_err("nothing listens on port 1");

    let connected = within(
        "the connect through the backoff",
        a.connect_peer_with_addrs(b_id, vec![b_addr]),
    )
    .await
    .expect("an explicit request with a good address must get its dial");
    assert_eq!(connected, b_id);
}

async fn connected(handle: &NetworkHandle, peer: PeerId) -> bool {
    handle
        .list_peers()
        .await
        .expect("list_peers")
        .iter()
        .any(|p| p.peer_id == peer)
}

/// Whether `handle` holds a connection *it* dialed to `peer`.
async fn dialed_out(handle: &NetworkHandle, peer: PeerId) -> bool {
    handle
        .list_peers()
        .await
        .expect("list_peers")
        .iter()
        .any(|p| p.peer_id == peer && p.direction == kwaai_p2p::Direction::Outbound)
}

/// Put `peer` in A's backoff with a dial to a port nothing listens on, then
/// file its real address in A's routing table for kad to find.
async fn back_off_then_table(a: &NetworkHandle, peer: PeerId, real: Multiaddr) {
    within(
        "the failing connect",
        a.connect_peer_with_addrs(peer, vec!["/ip4/127.0.0.1/tcp/1".parse().unwrap()]),
    )
    .await
    .expect_err("nothing listens on port 1");
    a.add_kad_address(peer, real)
        .await
        .expect("seed the real address");
}

#[tokio::test]
async fn a_walk_may_not_dial_a_backed_off_peer_but_the_service_may() {
    let (a, _a_task, _) = spawn_isolated();
    let (b, _b_task, b_id) = spawn_isolated();
    let b_addr = first_listen_addr(&b).await;
    back_off_then_table(&a, b_id, b_addr.clone()).await;

    // kad's walk would dial B from the routing table; the gate must refuse it.
    a.bootstrap(vec![]).await.expect("bootstrap starts");
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(
        !dialed_out(&a, b_id).await,
        "a walk-originated dial reached a peer in backoff"
    );

    let explicit = format!("{b_addr}/p2p/{b_id}");
    within("the explicit connect", a.connect_peer(&explicit))
        .await
        .expect("a service dial is exempt from the backoff");
}

#[tokio::test]
async fn an_inbound_connection_clears_the_backoff() {
    let (a, _a_task, a_id) = spawn_isolated();
    let (b, _b_task, b_id) = spawn_isolated();
    let a_addr = first_listen_addr(&a).await;
    let b_addr = first_listen_addr(&b).await;
    back_off_then_table(&a, b_id, b_addr).await;

    // B reaches A on its own, then leaves.
    within("B dials A", b.connect_peer(&format!("{a_addr}/p2p/{a_id}")))
        .await
        .expect("B reaches A");
    b.disconnect_peer(a_id).await.expect("disconnect");
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    while connected(&a, b_id).await {
        assert!(tokio::time::Instant::now() < deadline, "close never landed");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    // Proven alive, so kad's walk may dial B again.
    a.bootstrap(vec![]).await.expect("bootstrap starts");
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while !dialed_out(&a, b_id).await {
        assert!(
            tokio::time::Instant::now() < deadline,
            "the walk was still refused after B connected in"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
