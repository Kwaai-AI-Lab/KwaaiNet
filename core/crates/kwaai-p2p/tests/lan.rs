//! Peers behind the same router dial each other on their LAN addresses, and
//! those addresses never reach what a node serves (see `src/lan.rs`).
//!
//! Loopback swarms with fresh keys, except where a test needs this host's own
//! private interface address; that one skips on a host without one.

use std::net::IpAddr;
use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr, PeerId};

const SETTLE_TIMEOUT: Duration = Duration::from_secs(20);

fn spawn_with(config: NetworkConfig) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(config, keypair).expect("swarm should start");
    (handle, task, peer_id)
}

fn listening_on(addrs: &[&str]) -> NetworkConfig {
    NetworkConfig {
        listen_addrs: addrs.iter().map(|a| a.to_string()).collect(),
        ..NetworkConfig::for_tests()
    }
}

async fn eventually<T, F, Fut>(what: &str, mut f: F) -> T
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Option<T>>,
{
    let deadline = tokio::time::Instant::now() + SETTLE_TIMEOUT;
    loop {
        if let Some(v) = f().await {
            return v;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for: {what}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn listen_addrs(handle: &NetworkHandle, at_least: usize) -> Vec<Multiaddr> {
    eventually("listen addresses", || async {
        let addrs = handle.listen_addrs().await.ok()?;
        (addrs.len() >= at_least).then_some(addrs)
    })
    .await
}

fn private_v4(addr: &Multiaddr) -> bool {
    matches!(addr.iter().next(), Some(Protocol::Ip4(ip)) if ip.is_private())
}

async fn tabled(handle: &NetworkHandle, peer: PeerId) -> Vec<Multiaddr> {
    handle
        .network_snapshot()
        .await
        .expect("snapshot")
        .routing
        .into_iter()
        .find(|e| e.peer_id == peer)
        .map(|e| e.addrs)
        .unwrap_or_default()
}

fn bare(addr: &Multiaddr) -> Multiaddr {
    addr.iter()
        .filter(|p| !matches!(p, Protocol::P2p(_)))
        .collect()
}

/// A peer on this host listens twice; we reach it on one and learn the other
/// from identify, as a dial candidate and nowhere else.
#[tokio::test]
async fn a_peer_on_this_host_is_dialable_on_the_address_we_did_not_dial() {
    let (bob, _bob_task, bob_id) = spawn_with(listening_on(&[
        "/ip4/127.0.0.1/tcp/0",
        "/ip4/127.0.0.1/tcp/0",
    ]));
    let (alice, _alice_task, _) = spawn_with(NetworkConfig::for_tests());

    let bob_addrs = listen_addrs(&bob, 2).await;
    let (dialed, other) = (bob_addrs[0].clone(), bob_addrs[1].clone());
    alice
        .connect_peer(&dialed.clone().with(Protocol::P2p(bob_id)).to_string())
        .await
        .expect("loopback dial");

    let candidates = eventually("bob's undialed listener as a candidate", || async {
        let c = alice.dial_candidates(bob_id).await.ok()?;
        c.contains(&other).then_some(c)
    })
    .await;
    assert!(candidates.contains(&dialed), "{candidates:?}");

    // Never served or stored: not in kad, not in what find_peer hands back.
    assert!(!tabled(&alice, bob_id)
        .await
        .iter()
        .any(|a| bare(a) == other));
    let found = alice.dht_find_peer(bob_id).await.expect("find_peer");
    assert!(!found.contains(&other), "find_peer leaked it: {found:?}");
    let known = alice.known_peers().await.expect("known peers");
    assert!(
        !known.iter().any(|k| k.addrs.contains(&other)),
        "known_peers leaked it"
    );
}

/// A dial on a LAN address puts it in kad's routing table, which serves it to
/// every FIND_NODE. It has to come back out once identify places the peer on
/// our LAN. Needs a private IPv4 interface on this host.
#[tokio::test]
async fn a_lan_address_kad_tabled_from_our_own_dial_is_taken_back_out() {
    let (bob, _bob_task, bob_id) = spawn_with(listening_on(&["/ip4/0.0.0.0/tcp/0"]));
    let (alice, _alice_task, _) = spawn_with(NetworkConfig::for_tests());

    // The wildcard expands to one listen address per interface, not all at once.
    let mut lan = None;
    for _ in 0..40 {
        lan = bob
            .listen_addrs()
            .await
            .ok()
            .and_then(|a| a.into_iter().find(private_v4));
        if lan.is_some() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let Some(lan) = lan else {
        eprintln!("skipping: this host has no private IPv4 interface");
        return;
    };
    alice
        .connect_peer(&lan.clone().with(Protocol::P2p(bob_id)).to_string())
        .await
        .expect("dial on our own LAN address");

    eventually("bob's LAN address as a candidate", || async {
        let c = alice.dial_candidates(bob_id).await.ok()?;
        c.contains(&lan).then_some(())
    })
    .await;
    // Past identify; give kad's own ProtocolConfirmed insert time to land too.
    tokio::time::sleep(Duration::from_millis(500)).await;
    let table = tabled(&alice, bob_id).await;
    assert!(
        !table.iter().any(|a| bare(a) == lan),
        "a LAN address left in kad: {table:?}"
    );
}

/// Only a connection we dialed is evidence: an inbound one's source could be a
/// local forwarder, so the dialer's other listeners stay unknown to us.
#[tokio::test]
async fn an_inbound_connection_is_not_evidence() {
    let (bob, _bob_task, _) = spawn_with(listening_on(&[
        "/ip4/127.0.0.1/tcp/0",
        "/ip4/127.0.0.1/tcp/0",
    ]));
    let (alice, _alice_task, alice_id) = spawn_with(NetworkConfig::for_tests());
    let bob_id = bob.peer_id();

    let bob_addrs = listen_addrs(&bob, 2).await;
    let alice_addr = listen_addrs(&alice, 1).await.remove(0);
    bob.connect_peer(&alice_addr.with(Protocol::P2p(alice_id)).to_string())
        .await
        .expect("loopback dial");
    eventually("identify from bob", || async {
        let p = alice.peer_protocols(bob_id).await.ok()??;
        (!p.is_empty()).then_some(())
    })
    .await;
    let candidates = alice.dial_candidates(bob_id).await.expect("candidates");
    // AutoNAT may dial one listener back; a live connection's own address is
    // always a candidate, so only the listeners alice never used are telling.
    let used: Vec<Multiaddr> = alice
        .list_peers()
        .await
        .expect("peers")
        .into_iter()
        .filter(|p| p.peer_id == bob_id)
        .map(|p| bare(&p.addr))
        .collect();
    let unused: Vec<&Multiaddr> = bob_addrs.iter().filter(|a| !used.contains(a)).collect();
    assert!(!unused.is_empty(), "both listeners in use: {used:?}");
    assert!(
        !unused.iter().any(|a| candidates.contains(a)),
        "inbound evidence admitted bob's listeners: {candidates:?}"
    );
}

/// Under `only_global_ips: true` even a loopback address our own dial put in
/// kad comes back out: served to a remote peer it reaches that peer itself.
#[tokio::test]
async fn strict_mode_keeps_a_dialed_loopback_address_out_of_kad() {
    let (bob, _bob_task, bob_id) = spawn_with(NetworkConfig::for_tests());
    let (alice, _alice_task, _) = spawn_with(NetworkConfig {
        require_global_ips: true,
        ..NetworkConfig::for_tests()
    });
    let dialed = listen_addrs(&bob, 1).await.remove(0);
    alice
        .connect_peer(&dialed.clone().with(Protocol::P2p(bob_id)).to_string())
        .await
        .expect("loopback dial");
    eventually("identify from bob", || async {
        let p = alice.peer_protocols(bob_id).await.ok()??;
        (!p.is_empty()).then_some(())
    })
    .await;
    tokio::time::sleep(Duration::from_millis(500)).await;
    let table = tabled(&alice, bob_id).await;
    assert!(
        !table.iter().any(|a| bare(a) == dialed),
        "loopback left in kad: {table:?}"
    );
}

/// A peer claiming a private address on a LAN we are not on — here a declared
/// external address, which identify advertises — never gets us to dial it.
#[tokio::test]
async fn a_claimed_private_address_outside_our_subnets_is_never_a_candidate() {
    let spoofed: Multiaddr = "/ip4/10.254.254.77/tcp/4001".parse().unwrap();
    let (bob, _bob_task, bob_id) = spawn_with(NetworkConfig {
        external_addr: Some(spoofed.to_string()),
        ..NetworkConfig::for_tests()
    });
    let (alice, _alice_task, _) = spawn_with(NetworkConfig::for_tests());
    // Identify advertises external addresses alongside listeners, so this is the claim.
    let claimed = eventually("bob to declare the address", || async {
        let ext = bob.external_addrs().await.ok()?;
        ext.contains(&spoofed).then_some(())
    });
    claimed.await;

    let dialed = listen_addrs(&bob, 1).await.remove(0);
    alice
        .connect_peer(&dialed.with(Protocol::P2p(bob_id)).to_string())
        .await
        .expect("loopback dial");
    eventually("identify to deliver bob's claim", || async {
        let protocols = alice.peer_protocols(bob_id).await.ok()??;
        (!protocols.is_empty()).then_some(())
    })
    .await;
    let candidates = alice.dial_candidates(bob_id).await.expect("candidates");
    let on_our_lan = candidates.iter().any(|c| {
        matches!(c.iter().next(), Some(Protocol::Ip4(ip)) if IpAddr::V4(ip) == IpAddr::from([10, 254, 254, 77]))
    });
    assert!(
        !on_our_lan,
        "dialing a LAN host the peer named: {candidates:?}"
    );
}

/// Two nodes on one LAN find each other by mDNS and connect with no dial and
/// no relay. Ignored by default: needs a multicast-capable interface, and it
/// will also see any other libp2p node on the network.
#[tokio::test]
#[ignore = "needs a multicast LAN; run by hand on each OS"]
async fn mdns_connects_two_nodes_on_one_lan_directly() {
    let mdns = || NetworkConfig {
        enable_mdns: true,
        ..listening_on(&["/ip4/0.0.0.0/tcp/0"])
    };
    let (alice, _alice_task, _) = spawn_with(mdns());
    let (_bob, _bob_task, bob_id) = spawn_with(mdns());

    let peer = tokio::time::timeout(Duration::from_secs(60), async {
        loop {
            if let Ok(peers) = alice.list_peers().await {
                if let Some(p) = peers.into_iter().find(|p| p.peer_id == bob_id) {
                    return p;
                }
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .expect("mDNS should connect the two nodes within a minute");
    assert!(
        private_v4(&peer.addr),
        "connected over a LAN address: {}",
        peer.addr
    );
}
