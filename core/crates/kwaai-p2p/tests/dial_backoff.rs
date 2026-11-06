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
