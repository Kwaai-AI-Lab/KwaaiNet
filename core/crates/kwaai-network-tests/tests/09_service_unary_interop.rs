//! `NetworkService` ↔ real p2pd unary interop — the Phase 2 service gate.
//!
//! `08_unary_swarm_interop` proved `unary::Behaviour` against a Go daemon with a
//! hand-driven bare swarm. This tier raises the left-hand side to the thing
//! production will actually run: a full [`NetworkService`] — the composed
//! `KwaaiBehaviour` (ping + identify + kad + unary) behind a `NetworkHandle` —
//! exchanging hivemind unary calls with a real p2pd in both directions.
//!
//! What that adds over 08:
//!
//! - the call path is `NetworkHandle::call_unary_handler`, including its
//!   `UnaryError` → `P2PError` mapping, rather than raw `send_request`,
//! - inbound calls go through the service's dispatch map and a handler
//!   registered with `NetworkHandle::add_unary_handler`, not an inline match on
//!   a swarm event,
//! - ping, identify and kad share the connection, so this also checks that the
//!   composed behaviour does not disturb unary negotiation against go-libp2p
//!   (identify runs on the same connection; a Go peer must tolerate it).
//!
//! ```text
//!   [NetworkService]  ◀──unary──▶  [p2pd]
//! ```
//!
//! | test | caller | responder |
//! | --- | --- | --- |
//! | `service_calls_daemon_handler` | **service** | p2pd |
//! | `daemon_calls_service_handler` | p2pd | **service** |
//! | `service_maps_daemon_refusal_to_protocol_error` | **service** | p2pd (no handler) |
//! | `removing_a_handler_makes_the_daemon_call_fail` | p2pd | **service** (handler removed) |
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like the other integration tiers.
//!
//! # Process hygiene
//!
//! Each daemon is spawned via `TestNode::new_wire_peer`, whose `P2PDaemon`
//! kills **its own child by PID** on drop — nothing here kills by process name
//! or touches the default socket path. The `NetworkService` side listens on an
//! ephemeral loopback port with a freshly generated key, so it cannot collide
//! with a node running on the same machine; its task is shut down explicitly at
//! the end of each test.

use std::time::Duration;

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService, P2PError, PeerId};
use libp2p::{identity::Keypair, Multiaddr};

const PROTO: &str = "DHTProtocol.rpc_ping";

/// Cap on any single daemon interaction, so a regression surfaces as a failure
/// rather than a hung suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

/// A `NetworkService` on loopback, kept alive with its task.
struct ServiceNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// Listen address including `/p2p/<id>`, dialable by a p2pd.
    addr: String,
    task: tokio::task::JoinHandle<()>,
}

impl ServiceNode {
    /// Spawn a loopback service. `kad_hint`, when given, seeds the routing
    /// table so a unary call can dial the peer on demand — the same route
    /// production takes, where Kademlia supplies the address.
    async fn spawn(kad_hint: Option<(PeerId, Multiaddr)>) -> Self {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, task) = NetworkService::spawn(NetworkConfig::for_tests(), keypair)
            .expect("network service should start");

        let listen_addr = tokio::time::timeout(CALL_TIMEOUT, async {
            loop {
                if let Some(addr) = handle
                    .listen_addrs()
                    .await
                    .ok()
                    .and_then(|a| a.into_iter().next())
                {
                    return addr;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
        .await
        .expect("the service must report a listen address");

        if let Some((peer, addr)) = kad_hint {
            handle
                .add_kad_address(peer, addr)
                .await
                .expect("seed the routing table");
        }

        Self {
            handle,
            peer_id,
            addr: format!("{listen_addr}/p2p/{peer_id}"),
            task,
        }
    }

    /// Stop the event loop and wait for the task to exit, so a test never
    /// leaves a listener behind.
    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        let _ = tokio::time::timeout(Duration::from_secs(5), self.task).await;
    }
}

/// The daemon's peer identity as swarm-side types.
fn daemon_peer(node: &TestNode) -> (PeerId, Multiaddr) {
    let peer_id = PeerId::from_bytes(&node.peer_id_bytes()).expect("valid peer id bytes");
    let port = node.p2p_port.expect("wire peer has a fixed port");
    let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{port}")
        .parse()
        .expect("valid multiaddr");
    (peer_id, addr)
}

// ============================================================================
// Tests
// ============================================================================

/// Service → Go daemon: `call_unary_handler` dials on demand off the Kademlia
/// hint, negotiates the bare protocol name against go-libp2p, and returns the
/// daemon-served handler's bytes.
#[tokio::test]
async fn service_calls_daemon_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::service_unary::service_calls_daemon",
        "integration",
    );

    let daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    daemon
        .client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"daemon:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register unary handler on daemon");

    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    let service = ServiceNode::spawn(Some((daemon_id, daemon_addr))).await;

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        service
            .handle
            .call_unary_handler(daemon_id, PROTO, b"from-service"),
    )
    .await
    .expect("the call must resolve within the timeout")
    .expect("a Go daemon must accept a NetworkService's unary call");
    assert_eq!(response, b"daemon:from-service");

    rec.metric("response_len", response.len());
    service.shutdown().await;
    rec.finish(true);
}

/// Go daemon → service: go-libp2p dials us, and the call is dispatched through
/// the service's handler map to a closure registered via `add_unary_handler`.
#[tokio::test]
async fn daemon_calls_service_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::service_unary::daemon_calls_service",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let service = ServiceNode::spawn(None).await;

    service
        .handle
        .add_unary_handler(PROTO, |data: Vec<u8>| async move {
            let mut out = b"service:".to_vec();
            out.extend_from_slice(&data);
            Ok(out)
        })
        .await
        .expect("register a handler on the service");

    daemon
        .client
        .connect_peer(&service.addr)
        .await
        .expect("daemon must dial the NetworkService");

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon
            .client
            .call_unary_handler(&service.peer_id.to_bytes(), PROTO, b"from-daemon"),
    )
    .await
    .expect("call did not time out")
    .expect("a NetworkService must serve a Go daemon's unary call");
    assert_eq!(response, b"service:from-daemon");

    rec.metric("response_len", response.len());
    service.shutdown().await;
    rec.finish(true);
}

/// A protocol the daemon does not serve comes back as go-libp2p's clean `na`
/// refusal, which the handle reports as `P2PError::Protocol` — and the
/// connection stays usable afterwards.
#[tokio::test]
async fn service_maps_daemon_refusal_to_protocol_error() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::service_unary::refusal_maps_to_protocol_error",
        "integration",
    );

    let daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    daemon
        .client
        .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
        .await
        .expect("register unary handler on daemon");

    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    let service = ServiceNode::spawn(Some((daemon_id, daemon_addr))).await;

    let error = tokio::time::timeout(
        CALL_TIMEOUT,
        service
            .handle
            .call_unary_handler(daemon_id, "DHTProtocol.rpc_nonexistent", b"x"),
    )
    .await
    .expect("the refusal must resolve within the timeout")
    .expect_err("an unserved protocol must be refused");

    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("does not support"),
            "a negotiation refusal must map to an unsupported-protocol message, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        service
            .handle
            .call_unary_handler(daemon_id, PROTO, b"still-alive"),
    )
    .await
    .expect("the follow-up call must resolve")
    .expect("the connection must survive the refusal");
    assert_eq!(response, b"still-alive");

    service.shutdown().await;
    rec.finish(true);
}

/// `remove_unary_handler` must be visible to a foreign stack: after removal the
/// daemon's call fails rather than hanging or being silently accepted. This is
/// the deregistration path Phase 3 relies on when an IPC client disconnects.
#[tokio::test]
async fn removing_a_handler_makes_the_daemon_call_fail() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::service_unary::removal_visible_to_daemon",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let service = ServiceNode::spawn(None).await;

    service
        .handle
        .add_unary_handler(PROTO, |data: Vec<u8>| async move { Ok(data) })
        .await
        .expect("register a handler on the service");

    daemon
        .client
        .connect_peer(&service.addr)
        .await
        .expect("daemon must dial the NetworkService");

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon
            .client
            .call_unary_handler(&service.peer_id.to_bytes(), PROTO, b"before"),
    )
    .await
    .expect("call did not time out")
    .expect("the handler is registered");
    assert_eq!(response, b"before");

    assert!(
        service
            .handle
            .remove_unary_handler(PROTO)
            .await
            .expect("remove should reach the service"),
        "removing a registered handler reports true"
    );

    let result = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon
            .client
            .call_unary_handler(&service.peer_id.to_bytes(), PROTO, b"after"),
    )
    .await
    .expect("the post-removal call must resolve, not hang");
    assert!(
        result.is_err(),
        "a removed handler must not answer, got {result:?}"
    );

    service.shutdown().await;
    rec.finish(true);
}
