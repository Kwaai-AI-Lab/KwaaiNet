//! IPv6 listeners, and the three things `ipv6:` can mean.
//!
//! Loopback cannot show anything about v6 routing. It can show that the
//! transport is wired up, that the mode decides the listen set, that `off`
//! keeps v6 addresses out of the dial set, and that `true` refuses to fall
//! back to IPv4-only in silence.

use kwaai_p2p::{Ipv6Mode, NetworkConfig, NetworkService};
use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr, PeerId};

fn is_v6(addr: &Multiaddr) -> bool {
    addr.iter().any(|p| matches!(p, Protocol::Ip6(_)))
}

/// The nat-test nodes and production configure `port`, not `listen_addrs`, so
/// this fallback is the real listen set.
#[test]
fn mode_decides_whether_v6_is_listened_on() {
    let listen = |ipv6, enable_quic| {
        NetworkConfig {
            port: 8080,
            listen_addrs: Vec::new(),
            ipv6,
            enable_quic,
            ..NetworkConfig::default()
        }
        .swarm_listen_addrs()
    };

    for mode in [Ipv6Mode::Auto, Ipv6Mode::On] {
        let addrs = listen(mode, false);
        assert!(addrs.iter().any(|a| a == "/ip4/0.0.0.0/tcp/8080"));
        assert_eq!(
            addrs.iter().any(|a| a == "/ip6/::/tcp/8080"),
            kwaai_p2p::IPV6_BUILD,
            "{mode:?} should listen on v6 iff the build supports it: {addrs:?}"
        );

        let with_quic = listen(mode, true);
        assert!(with_quic
            .iter()
            .any(|a| a == "/ip4/0.0.0.0/udp/8080/quic-v1"));
        assert_eq!(
            with_quic.iter().any(|a| a == "/ip6/::/udp/8080/quic-v1"),
            kwaai_p2p::IPV6_BUILD,
            "{mode:?} quic: {with_quic:?}"
        );
    }

    for enable_quic in [false, true] {
        let addrs = listen(Ipv6Mode::Off, enable_quic);
        assert!(
            !addrs.iter().any(|a| a.contains("/ip6/")),
            "ipv6: false must leave no v6 listener: {addrs:?}"
        );
        assert!(addrs.iter().any(|a| a == "/ip4/0.0.0.0/tcp/8080"));
    }
}

/// `off` is not just about listeners: a v6 address learned from the DHT or a
/// peer is unusable on a node with no v6 stack, and keeping it would crowd out
/// the v4 address that works.
#[tokio::test]
async fn off_filters_v6_dial_candidates() {
    let peer = Keypair::generate_ed25519().public().to_peer_id();
    let v4: Multiaddr = "/ip4/198.18.0.20/tcp/8080".parse().unwrap();
    let v6: Multiaddr = "/ip6/2606:4700::1111/tcp/8080".parse().unwrap();

    async fn addrs_known_for(mode: Ipv6Mode, peer: PeerId, seed: &[Multiaddr]) -> Vec<Multiaddr> {
        let (handle, _task) = NetworkService::spawn(
            NetworkConfig {
                ipv6: mode,
                ..NetworkConfig::for_tests()
            },
            Keypair::generate_ed25519(),
        )
        .expect("node should start");
        for addr in seed {
            handle.add_kad_address(peer, addr.clone()).await.unwrap();
        }
        handle
            .known_peers()
            .await
            .expect("known peers")
            .into_iter()
            .find(|k| k.peer_id == peer)
            .map(|k| k.addrs)
            .unwrap_or_default()
    }

    let seed = [v4.clone(), v6.clone()];
    let off = addrs_known_for(Ipv6Mode::Off, peer, &seed).await;
    assert!(off.contains(&v4), "v4 must survive: {off:?}");
    assert!(
        !off.iter().any(is_v6),
        "ipv6: false must drop v6 candidates: {off:?}"
    );

    if kwaai_p2p::IPV6_BUILD {
        let on = addrs_known_for(Ipv6Mode::On, peer, &seed).await;
        assert!(on.iter().any(is_v6), "v6 must survive when on: {on:?}");
    }
}

/// Without the feature the key still parses; it just cannot mean anything.
#[cfg(not(feature = "ipv6"))]
#[test]
fn feature_off_reads_as_off() {
    assert!(!kwaai_p2p::IPV6_BUILD);
    for mode in [Ipv6Mode::Auto, Ipv6Mode::On, Ipv6Mode::Off] {
        assert_eq!(mode.effective(), Ipv6Mode::Off);
        assert!(mode.is_off());
    }
}

/// Everything that needs a real v6 listener. Skipped wholesale on a build
/// without the feature, where there is nothing to listen with.
#[cfg(feature = "ipv6")]
mod with_v6_listeners {
    use std::net::TcpListener;
    use std::time::Duration;

    use kwaai_p2p::{Ipv6Mode, Ipv6Status, NetworkConfig, NetworkHandle, NetworkService};
    use libp2p::{identity::Keypair, multiaddr::Protocol, Multiaddr, PeerId};

    const SETTLE_TIMEOUT: Duration = Duration::from_secs(30);
    const POLL_INTERVAL: Duration = Duration::from_millis(50);

    fn is_v6(addr: &Multiaddr) -> bool {
        addr.iter().any(|p| matches!(p, Protocol::Ip6(_)))
    }

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

    /// CI hosts without an IPv6 stack must skip rather than fail.
    fn v6_loopback_works() -> bool {
        TcpListener::bind("[::1]:0").is_ok()
    }

    fn spawn_v6_only(mode: Ipv6Mode) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, task) = NetworkService::spawn(
            NetworkConfig {
                listen_addrs: vec!["/ip6/::1/tcp/0".to_string()],
                ipv6: mode,
                ..NetworkConfig::for_tests()
            },
            keypair,
        )
        .expect("node should start");
        (handle, task, peer_id)
    }

    #[tokio::test]
    async fn two_nodes_connect_over_ip6_loopback() {
        if !v6_loopback_works() {
            println!("skipping: no IPv6 loopback on this host");
            return;
        }

        let (alice, _alice_task, alice_id) = spawn_v6_only(Ipv6Mode::On);
        let alice_addr = eventually("alice to report a v6 listen address", || async {
            alice.listen_addrs().await.ok()?.into_iter().find(is_v6)
        })
        .await
        .with(Protocol::P2p(alice_id));

        let (bob, _bob_task, _bob_id) = spawn_v6_only(Ipv6Mode::On);
        let connected = bob
            .connect_peer(&alice_addr.to_string())
            .await
            .expect("bob should reach alice over ipv6");
        assert_eq!(connected, alice_id);

        let peer = eventually("bob to list alice", || async {
            bob.list_peers()
                .await
                .ok()?
                .into_iter()
                .find(|p| p.peer_id == alice_id)
        })
        .await;
        assert!(
            is_v6(&peer.addr),
            "the connection should be IPv6: {}",
            peer.addr
        );

        let snapshot = bob.network_snapshot().await.expect("snapshot");
        assert_eq!(snapshot.ipv6, Ipv6Status::Active);
    }

    /// `auto` and `true` differ only when the bind fails, which is the whole
    /// reason `true` exists.
    #[tokio::test]
    async fn on_is_a_hard_error_when_v6_bind_fails() {
        // Port 1 is privileged. If we can take it we are root and the test has no
        // failure to observe.
        if TcpListener::bind("[::1]:1").is_ok() {
            println!("skipping: privileged enough to bind [::1]:1");
            return;
        }

        let config = |ipv6| NetworkConfig {
            listen_addrs: vec!["/ip6/::1/tcp/1".to_string()],
            ipv6,
            ..NetworkConfig::for_tests()
        };

        assert!(
            NetworkService::spawn(config(Ipv6Mode::On), Keypair::generate_ed25519()).is_err(),
            "ipv6: true must refuse to start rather than run v4-only"
        );

        let (handle, _task) =
            NetworkService::spawn(config(Ipv6Mode::Auto), Keypair::generate_ed25519())
                .expect("auto should fall back rather than fail");
        let snapshot = handle.network_snapshot().await.expect("snapshot");
        assert_eq!(snapshot.ipv6, Ipv6Status::Unavailable);
    }
}
