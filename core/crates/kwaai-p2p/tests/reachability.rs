//! Reachability as the service actually applies it.
//!
//! The rule ladder itself is unit-tested in `src/reachability.rs`, where every
//! transition is reachable without a network. What can only be checked here is
//! the wiring: that a declared address really reaches the swarm's external
//! address set, that the grace timer really fires, and that a disconnecting
//! peer really stops counting as an observer.

use std::time::Duration;

use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService, Reachability, ReachabilitySource};
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

fn spawn(config: NetworkConfig) -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) = NetworkService::spawn(config, keypair).expect("swarm should start");
    (handle, task, peer_id)
}

async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> Multiaddr {
    let addr = eventually("swarm to report a listen address", || async {
        handle.listen_addrs().await.ok()?.into_iter().next()
    })
    .await;
    addr.with(libp2p::multiaddr::Protocol::P2p(peer_id))
}

#[tokio::test]
async fn a_declared_external_addr_is_confirmed_at_startup() {
    let declared = "/ip4/203.0.113.7/tcp/8080";
    let (handle, _task, _id) = spawn(NetworkConfig {
        external_addr: Some(declared.to_string()),
        ..NetworkConfig::for_tests()
    });

    // Before any peer has been dialed and before any probe has run.
    let reachability = handle.reachability().await.expect("reachability");
    assert_eq!(
        reachability,
        Reachability::Public {
            addr: declared.parse().unwrap(),
            source: ReachabilitySource::Declared,
        }
    );

    // And it reached the swarm, not just the state machine: `listen_addrs`
    // reports listeners, so the proof that `add_external_address` ran is that
    // the declared address is advertised *without* being listened on.
    let listeners = handle.listen_addrs().await.expect("listen_addrs");
    assert!(
        !listeners.iter().any(|a| a.to_string() == declared),
        "the declared address is external, not a listener: {listeners:?}"
    );
}

#[tokio::test]
async fn a_malformed_external_addr_fails_the_spawn() {
    // An external_addr is an instruction, not a guess. Ignoring a typo would
    // leave the node quietly unreachable in exactly the deployment that took
    // the trouble to configure it.
    let result = NetworkService::spawn(
        NetworkConfig {
            external_addr: Some("192.0.2.1:8080".to_string()), // not a multiaddr
            ..NetworkConfig::for_tests()
        },
        Keypair::generate_ed25519(),
    );
    assert!(
        result.is_err(),
        "a malformed external_addr must not be ignored"
    );
}

#[tokio::test]
async fn force_private_is_private_from_the_first_moment() {
    // Not Unknown-then-Private after the grace period: relay reservations are
    // supposed to start at t=0, which is the whole reason the flag exists.
    let (handle, _task, _id) = spawn(NetworkConfig {
        force_private: true,
        ..NetworkConfig::for_tests()
    });
    assert_eq!(
        handle.reachability().await.expect("reachability"),
        Reachability::Private
    );
}

#[tokio::test]
async fn a_declared_addr_wins_over_force_private() {
    let (handle, _task, _id) = spawn(NetworkConfig {
        force_private: true,
        external_addr: Some("/ip4/203.0.113.7/tcp/8080".to_string()),
        ..NetworkConfig::for_tests()
    });
    assert_eq!(
        handle.reachability().await.expect("reachability").source(),
        Some(ReachabilitySource::Declared),
        "the operator's declaration outranks force_private"
    );
}

#[tokio::test]
async fn an_isolated_node_starts_unknown_rather_than_guessing() {
    // Before the grace period elapses, "no evidence" means "not yet". A node
    // that announced itself Private on its first millisecond would go looking
    // for a relay before its first bootstrap dial had even completed.
    let (handle, _task, _id) = spawn(NetworkConfig::for_tests());
    assert_eq!(
        handle.reachability().await.expect("reachability"),
        Reachability::Unknown
    );
}

#[tokio::test]
async fn a_disconnecting_peer_stops_being_an_observer() {
    // The identify-consensus fallback counts distinct observers. If observers
    // were never pruned the count could only ever rise, and a node that moved
    // networks would keep being promoted on the strength of peers that saw it
    // at its old address.
    let (alice, _alice_task, _alice_id) = spawn(NetworkConfig::for_tests());
    let (bob, _bob_task, bob_id) = spawn(NetworkConfig::for_tests());

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");

    eventually("alice to record bob's observation", || async {
        let observed = alice.observed_addrs().await.ok()?;
        (!observed.is_empty()).then_some(())
    })
    .await;

    alice.disconnect_peer(bob_id).await.expect("disconnect");

    eventually("bob's observation to be forgotten", || async {
        alice.observed_addrs().await.ok()?.is_empty().then_some(())
    })
    .await;
}

// ---------------------------------------------------------------------------
// The announce watch channel
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_declared_node_is_announceable_immediately() {
    // Before this channel existed the announce loop had no way to learn about
    // an address change at all; a node either announced what it was configured
    // with or nothing. Here the state is right from the first read, with no
    // round trip to the event loop.
    let (handle, _task, _id) = spawn(NetworkConfig {
        external_addr: Some("/ip4/203.0.113.7/tcp/8080".to_string()),
        ..NetworkConfig::for_tests()
    });

    let state = handle.current_announce_state();
    assert_eq!(state.reachability, kwaai_p2p::ReachabilityKind::Public);
    assert!(state.announceable);
    assert!(!state.using_relay, "a public node is not using a relay");
}

#[tokio::test]
async fn a_private_node_is_announceable_but_not_yet_relaying() {
    // `force_private` is a statement about reachability, not about having a
    // circuit — `using_relay` stays false until one is actually confirmed,
    // because announcing it earlier would tell the map a node is reachable
    // through a relay that has not agreed to carry it.
    let (handle, _task, _id) = spawn(NetworkConfig {
        force_private: true,
        ..NetworkConfig::for_tests()
    });

    let state = handle.current_announce_state();
    assert_eq!(state.reachability, kwaai_p2p::ReachabilityKind::Private);
    assert!(state.announceable);
    assert!(!state.using_relay);
}

#[tokio::test]
async fn an_unknown_node_is_not_announceable() {
    // A node that does not know where it stands should not be telling the
    // network it is Direct.
    let (handle, _task, _id) = spawn(NetworkConfig::for_tests());
    let state = handle.current_announce_state();
    assert_eq!(state.reachability, kwaai_p2p::ReachabilityKind::Unknown);
    assert!(!state.announceable);
}

#[tokio::test]
async fn address_churn_does_not_wake_the_announce_loop() {
    // The regression this channel exists to prevent. A node's addresses change
    // constantly — identify pushes, connections coming and going — and the DHT
    // record carries no addresses at all, so none of it should cause a
    // re-announce. Only reachability or `using_relay` changing should.
    let (alice, _alice_task, _alice_id) = spawn(NetworkConfig {
        force_private: true,
        ..NetworkConfig::for_tests()
    });

    let mut announce = alice.announce_state();
    let before = *announce.borrow();

    // Plenty of address activity: three peers connect, identify, disconnect.
    for _ in 0..3 {
        let (bob, _bob_task, bob_id) = spawn(NetworkConfig::for_tests());
        let bob_addr = dialable_addr(&bob, bob_id).await;
        alice
            .connect_peer(&bob_addr.to_string())
            .await
            .expect("dial");
        eventually("alice to observe an address", || async {
            let observed = alice.observed_addrs().await.ok()?;
            (!observed.is_empty()).then_some(())
        })
        .await;
        alice.disconnect_peer(bob_id).await.expect("disconnect");
    }

    // Nothing about reachability changed, so nothing was published.
    assert!(
        tokio::time::timeout(Duration::from_millis(500), announce.changed())
            .await
            .is_err(),
        "address churn must not wake the announce loop"
    );
    assert_eq!(*announce.borrow(), before, "including the epoch");
}

#[tokio::test]
async fn each_receiver_waits_for_the_next_real_change() {
    // `announce_state()` marks the current value as seen, so a consumer that
    // loops on `changed()` does not immediately re-announce what it already
    // announced.
    let (handle, _task, _id) = spawn(NetworkConfig {
        force_private: true,
        ..NetworkConfig::for_tests()
    });

    let mut announce = handle.announce_state();
    assert!(
        tokio::time::timeout(Duration::from_millis(300), announce.changed())
            .await
            .is_err(),
        "a fresh receiver should not fire on the value it was created with"
    );
}

#[tokio::test]
async fn loopback_observations_never_promote_a_node() {
    // Two loopback swarms observe each other at 127.0.0.1. That clears the
    // two-observer threshold on count alone, so the only thing stopping a
    // promotion is the address classifier — which is exactly what this pins.
    let config = NetworkConfig {
        identify_min_confirmations: 1,
        ..NetworkConfig::for_tests()
    };
    let (alice, _alice_task, _alice_id) = spawn(config.clone());
    let (bob, _bob_task, bob_id) = spawn(config);

    let bob_addr = dialable_addr(&bob, bob_id).await;
    alice
        .connect_peer(&bob_addr.to_string())
        .await
        .expect("dial");

    eventually("alice to record an observed address", || async {
        let observed = alice.observed_addrs().await.ok()?;
        (!observed.is_empty()).then_some(())
    })
    .await;

    // Even with the threshold at one and an observation in hand, a loopback
    // address is not evidence of anything.
    assert!(
        !alice
            .reachability()
            .await
            .expect("reachability")
            .is_public(),
        "a loopback observation must never promote a node to public"
    );
}
