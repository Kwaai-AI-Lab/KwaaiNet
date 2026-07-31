//! `ControlServer` ↔ real p2pd cross-implementation matrix — the Phase 3 gate.
//!
//! Tier 09 proved a `NetworkService` interoperates with a Go daemon over
//! **libp2p**. This tier adds the other socket: a [`ControlServer`] serving the
//! p2pd *control protocol* for that native node, with real [`P2PClient`]s
//! attached to both implementations. Two independent interfaces must hold at
//! once, and only this tier exercises both together:
//!
//! 1. the **IPC protocol** — the same client code drives a Go daemon and a
//!    native node and cannot tell them apart,
//! 2. the **libp2p wire** — a handler registered over one implementation's
//!    socket answers a caller that arrived through the other's.
//!
//! ```text
//!   [P2PClient N] ──socket──▶ [ControlServer + NetworkService] ◀──libp2p──▶ [p2pd] ◀──socket── [P2PClient G]
//! ```
//!
//! # The matrix
//!
//! Each cell is one unary call. "via" is the implementation whose control
//! socket the client is attached to; the handler is always registered by the
//! *other* side's socket client, so every call crosses both a socket boundary
//! and the libp2p wire.
//!
//! | # | caller (client via) | responder (handler registered via) | test |
//! | --- | --- | --- | --- |
//! | 1 | **native** `ControlServer` | p2pd | `native_socket_client_calls_a_daemon_socket_client` |
//! | 2 | p2pd | **native** `ControlServer` | `daemon_socket_client_calls_a_native_socket_client` |
//! | 3 | **native** `ControlServer` | **native** `ControlServer` (2nd node) | `native_socket_client_calls_another_native_node` |
//! | 4 | p2pd | p2pd | covered by tier 07 — the pre-existing baseline, not repeated |
//!
//! Cells 1 and 2 are the ones that matter: they are the mixed fleet during the
//! cutover window, when some nodes still run the Go daemon. Cell 3 is the
//! post-cutover steady state. Cell 4 is what we are replacing and already has
//! coverage, so re-testing it here would only add daemon startup time.
//!
//! Two further checks that are not calls:
//!
//! - `identify_matches_between_implementations` — the IDENTIFY response *shape*
//!   agrees byte-for-byte in the ways clients actually parse (raw peer-ID bytes,
//!   binary multiaddrs), which is what `node.rs`'s self-discovery depends on.
//! - `a_daemon_caller_gets_a_clean_refusal_after_the_client_disconnects` — the
//!   stale-handler fix, observed from a *foreign* stack: when the native node's
//!   socket client dies, a Go daemon's call must be refused rather than hang.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`.
//!
//! # Process hygiene
//!
//! Daemons come from `TestNode::new_wire_peer`, which kills its own child by
//! PID on drop — nothing kills by name or touches the default socket path. The
//! native side listens on an ephemeral loopback port with a fresh key and a
//! tmpdir socket, so it cannot collide with a node running on the same machine.

use std::time::Duration;

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use kwaai_p2p_daemon::{ControlServer, P2PClient};
use libp2p::identity::Keypair;
use tempfile::TempDir;

const PROTO: &str = "DHTProtocol.rpc_ping";

/// Cap on any single daemon interaction, so a regression fails rather than
/// hanging the suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

/// A native node: `NetworkService` behind a `ControlServer`, the thing this
/// tier is proving is indistinguishable from a p2pd.
struct NativeNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// Control-socket multiaddr, for `P2PClient::connect`.
    socket: String,
    /// libp2p listen address including `/p2p/<id>`, dialable by a p2pd.
    addr: String,
    service_task: tokio::task::JoinHandle<()>,
    server_task: tokio::task::JoinHandle<()>,
    _tmpdir: TempDir,
}

impl NativeNode {
    async fn spawn() -> Self {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");

        let listen: Multiaddr = tokio::time::timeout(CALL_TIMEOUT, async {
            loop {
                if let Some(a) = handle
                    .listen_addrs()
                    .await
                    .ok()
                    .and_then(|a| a.into_iter().next())
                {
                    return a;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("the swarm must report a listen address");

        let tmpdir = TempDir::new().expect("tmpdir");
        let socket = format!("/unix/{}", tmpdir.path().join("kwaai.sock").display());
        let server = ControlServer::bind(&socket, handle.clone())
            .await
            .expect("control socket binds");
        let server_task = tokio::spawn(server.run());

        Self {
            handle,
            peer_id,
            socket,
            addr: format!("{listen}/p2p/{peer_id}"),
            service_task,
            server_task,
            _tmpdir: tmpdir,
        }
    }

    async fn client(&self) -> P2PClient {
        P2PClient::connect(&self.socket)
            .await
            .expect("a P2PClient must connect to the native control socket")
    }

    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        self.server_task.abort();
        let _ = tokio::time::timeout(Duration::from_secs(5), self.service_task).await;
    }
}

/// The daemon's identity as swarm-side types.
fn daemon_peer(node: &TestNode) -> (PeerId, Multiaddr) {
    let peer_id = PeerId::from_bytes(&node.peer_id_bytes()).expect("valid peer id bytes");
    let port = node.p2p_port.expect("wire peer has a fixed port");
    let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{port}")
        .parse()
        .expect("valid multiaddr");
    (peer_id, addr)
}

// ============================================================================
// Matrix cell 1: client via native ControlServer → handler on p2pd
// ============================================================================

/// A socket client attached to the **native** node calls a handler registered
/// by a socket client attached to a **Go daemon**.
///
/// This is the cutover-window direction that matters most: a migrated node's
/// `shard serve` reaching a peer that has not been migrated yet.
#[tokio::test]
async fn native_socket_client_calls_a_daemon_socket_client() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::control_server::native_client_calls_daemon",
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
        .expect("register a handler over the daemon's socket");

    let native = NativeNode::spawn().await;
    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    native
        .handle
        .add_kad_address(daemon_id, daemon_addr)
        .await
        .expect("seed the routing table so the call can dial on demand");

    // The call goes: P2PClient → native control socket → ControlServer →
    // NetworkHandle → libp2p → p2pd → its socket client's handler.
    let client = native.client().await;
    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        client.call_unary_handler(&daemon_id.to_bytes(), PROTO, b"from-native-socket"),
    )
    .await
    .expect("the call must resolve within the timeout")
    .expect("a native ControlServer must route a call to a Go daemon's handler");
    assert_eq!(response, b"daemon:from-native-socket");

    rec.metric("response_len", response.len());
    native.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Matrix cell 2: client via p2pd → handler on native ControlServer
// ============================================================================

/// A socket client attached to a **Go daemon** calls a handler registered by a
/// socket client attached to the **native** node.
///
/// The reverse cutover direction, and the one that exercises the whole inbound
/// path: libp2p dispatch → `add_unary_handler_boxed` → the native control
/// socket → the client's handler → back out as a `unaryResponse` frame.
#[tokio::test]
async fn daemon_socket_client_calls_a_native_socket_client() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::control_server::daemon_client_calls_native",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let native = NativeNode::spawn().await;

    let client = native.client().await;
    client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"native:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register a handler over the native control socket");

    daemon
        .client
        .connect_peer(&native.addr)
        .await
        .expect("the daemon must dial the native node");

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon
            .client
            .call_unary_handler(&native.peer_id.to_bytes(), PROTO, b"from-daemon-socket"),
    )
    .await
    .expect("the call must resolve within the timeout")
    .expect("a Go daemon's client must reach a handler served over the native socket");
    assert_eq!(response, b"native:from-daemon-socket");

    rec.metric("response_len", response.len());
    native.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Matrix cell 3: native → native, both ends over control sockets
// ============================================================================

/// Two native nodes, each with its own `ControlServer`, and socket clients on
/// both ends — the post-cutover steady state with no Go daemon in the picture.
#[tokio::test]
async fn native_socket_client_calls_another_native_node() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::control_server::native_client_calls_native",
        "integration",
    );

    let caller = NativeNode::spawn().await;
    let responder = NativeNode::spawn().await;

    let responder_client = responder.client().await;
    responder_client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"peer:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register a handler on the responder's socket");

    let mut connector = caller.client().await;
    connector
        .connect_peer(&responder.addr)
        .await
        .expect("CONNECT over the native control socket must dial the peer");

    let client = caller.client().await;
    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        client.call_unary_handler(&responder.peer_id.to_bytes(), PROTO, b"native-to-native"),
    )
    .await
    .expect("the call must resolve within the timeout")
    .expect("two native nodes must exchange a unary call end to end over their sockets");
    assert_eq!(response, b"peer:native-to-native");

    rec.metric("response_len", response.len());
    caller.shutdown().await;
    responder.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Response-shape conformance
// ============================================================================

/// IDENTIFY must be shaped identically by both implementations in every way the
/// client code actually parses it: a hex-decodable raw peer ID, and addresses
/// that are binary multiaddrs rather than strings.
///
/// `node.rs` self-discovery reads exactly these two fields, so a divergence here
/// would break address announcement rather than fail loudly.
#[tokio::test]
async fn identify_matches_between_implementations() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::control_server::identify_shape_parity",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let native = NativeNode::spawn().await;
    let mut native_client = native.client().await;

    let (daemon_hex, daemon_addrs) = daemon
        .client
        .identify_with_addrs()
        .await
        .expect("daemon identify");
    let (native_hex, native_addrs) = native_client
        .identify_with_addrs()
        .await
        .expect("native identify");

    // Both peer IDs decode from hex to a valid PeerId, and the native one is
    // the key we actually generated.
    let daemon_id = PeerId::from_bytes(&hex::decode(&daemon_hex).expect("daemon hex"))
        .expect("the daemon's id must be raw peer-id bytes");
    let native_id = PeerId::from_bytes(&hex::decode(&native_hex).expect("native hex"))
        .expect("the native id must be raw peer-id bytes, in the same encoding");
    assert_eq!(
        native_id, native.peer_id,
        "IDENTIFY must report the node's own peer id"
    );
    assert_ne!(daemon_id, native_id, "the two nodes are distinct peers");

    // Both address lists are binary multiaddrs.
    assert!(
        !daemon_addrs.is_empty() && !native_addrs.is_empty(),
        "both implementations report at least one address"
    );
    for (label, addrs) in [("daemon", &daemon_addrs), ("native", &native_addrs)] {
        for raw in addrs {
            Multiaddr::try_from(raw.clone())
                .unwrap_or_else(|e| panic!("{label} addrs must be binary multiaddrs: {e}"));
        }
    }

    native.shutdown().await;
    rec.finish(true);
}

/// The stale-handler fix, proven against a foreign stack.
///
/// When the native node's socket client disconnects — the `storage serve` crash
/// case — the protocol must stop being served, so a **Go daemon's** call is
/// refused during negotiation instead of hanging until its own timeout. The
/// observation has to come from outside our own implementation for this to mean
/// anything, which is why it lives in the interop tier rather than the
/// in-process suite.
#[tokio::test]
async fn a_daemon_caller_gets_a_clean_refusal_after_the_client_disconnects() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::control_server::handler_released_on_disconnect",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let native = NativeNode::spawn().await;

    daemon
        .client
        .connect_peer(&native.addr)
        .await
        .expect("the daemon must dial the native node");

    {
        let client = native.client().await;
        client
            .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
            .await
            .expect("register a handler over the native socket");

        let response = tokio::time::timeout(
            CALL_TIMEOUT,
            daemon
                .client
                .call_unary_handler(&native.peer_id.to_bytes(), PROTO, b"alive"),
        )
        .await
        .expect("the call must resolve")
        .expect("the handler answers while its client is connected");
        assert_eq!(response, b"alive");
        // Dropping the client closes its socket — the crash case.
    }

    // Deregistration is asynchronous with respect to the drop; poll until the
    // daemon's call starts failing, bounded by the usual timeout.
    let error = tokio::time::timeout(CALL_TIMEOUT, async {
        loop {
            match daemon
                .client
                .call_unary_handler(&native.peer_id.to_bytes(), PROTO, b"after")
                .await
            {
                Err(e) => return e,
                Ok(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    })
    .await
    .expect("a released protocol must stop answering, not hang the Go caller");

    // The failure must be a refusal, not a timeout — that distinction is the
    // entire point of releasing the handler.
    let text = error.to_string();
    assert!(
        !text.contains("timed out"),
        "the daemon must get a refusal rather than time out, got: {text}"
    );

    native.shutdown().await;
    rec.finish(true);
}
