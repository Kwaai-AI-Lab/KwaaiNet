//! `max_connections`, which was configurable and enforced nowhere.
//!
//! It matters now that idle connections are kept for ten minutes rather than
//! thirty seconds: nothing else bounds growth, and a bootstrap — which mostly
//! receives connections rather than making them — is where that bites first.

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

fn spawn(max_connections: usize) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(
        NetworkConfig {
            max_connections,
            ..NetworkConfig::for_tests()
        },
        keypair,
    )
    .expect("node should start");
    (handle, task, peer_id)
}

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

/// `max_connections: 2` leaves an inbound cap of one, so the second dialer is
/// refused rather than accepted and remembered.
#[tokio::test]
async fn inbound_connections_are_capped() {
    let (limited, _limited_task, limited_id) = spawn(2);
    let addr = dialable_addr(&limited, limited_id).await.to_string();

    let (first, _first_task, _) = spawn(100);
    first.connect_peer(&addr).await.expect("first dialer");
    eventually("the first connection to be established", || async {
        limited.list_peers().await.ok()?.first().map(|_| ())
    })
    .await;

    // Assert on the *limited* node's view, not the dialer's: the denial happens
    // in `handle_established_inbound_connection`, after the transport upgrade,
    // so the dialer can see its own connection established and only then be
    // closed. What matters is that the limited node never keeps the second.
    let (second, _second_task, second_id) = spawn(100);
    let _ = second.connect_peer(&addr).await;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let peers = limited.list_peers().await.expect("list_peers");
    assert_eq!(
        peers.len(),
        1,
        "inbound is capped at one, so the second dialer must not be kept: {peers:?}"
    );
    assert!(
        peers.iter().all(|p| p.peer_id != second_id),
        "the kept connection should be the first dialer, not the refused one"
    );
}

/// The inbound cap sits below the total on purpose. A node whose inbound slots
/// are full must still be able to dial: if inbound could consume every slot the
/// node would go deaf, unable to re-dial the bootstraps and relays that keep it
/// reachable.
#[tokio::test]
async fn a_full_inbound_cap_still_leaves_room_to_dial_out() {
    let (limited, _limited_task, limited_id) = spawn(2);
    let addr = dialable_addr(&limited, limited_id).await.to_string();

    let (inbound, _inbound_task, _) = spawn(100);
    inbound.connect_peer(&addr).await.expect("inbound dialer");
    eventually("the inbound connection to be established", || async {
        limited.list_peers().await.ok()?.first().map(|_| ())
    })
    .await;

    // Inbound is now at its cap of one. The outbound slot must remain.
    let (peer, _peer_task, peer_id) = spawn(100);
    let peer_addr = dialable_addr(&peer, peer_id).await.to_string();
    limited
        .connect_peer(&peer_addr)
        .await
        .expect("a node at its inbound cap must still be able to dial out");
}
