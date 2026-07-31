//! In-process conformance suite for [`ControlServer`] — old client, new server.
//!
//! Every test here drives the **unmodified** [`P2PClient`] against a
//! [`ControlServer`] backed by a real [`NetworkService`]. That is the whole
//! premise of Phase 3: if these pass, the ~15 external client call sites work
//! against a node with no Go daemon behind it. Nothing in this file constructs a
//! protobuf frame by hand — doing so would test the server against my reading of
//! the protocol rather than against the code that actually ships.
//!
//! Topology, unless a test says otherwise:
//!
//! ```text
//!   [P2PClient A] ─socket─┐
//!                          ├─▶ [ControlServer] ─▶ [NetworkService]  ◀──libp2p──▶  [NetworkService #2]
//!   [P2PClient B] ─socket─┘
//! ```
//!
//! The two-swarm tests are what make handler assertions meaningful: a handler's
//! registration is only observable to a **remote** caller, so "deregistered on
//! disconnect" is checked by calling in from the second node, not by inspecting
//! server state.
//!
//! These run in the default `cargo test` tier — no daemon binary, no network,
//! no `KWAAI_INTEGRATION_TESTS` gate. Cross-implementation coverage against a
//! real p2pd lives in `kwaai-network-tests/tests/11_control_server_interop.rs`.

use std::time::Duration;

use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use kwaai_p2p_daemon::{ControlServer, P2PClient};
use libp2p::identity::Keypair;
use tempfile::TempDir;

/// Cap on any single interaction, so a regression fails instead of hanging.
const TIMEOUT: Duration = Duration::from_secs(20);

const PROTO: &str = "DHTProtocol.rpc_ping";

/// A node: swarm + control socket, torn down together.
struct TestNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// Control-socket multiaddr, for `P2PClient::connect`.
    socket: String,
    /// libp2p listen address including `/p2p/<id>`.
    addr: String,
    service_task: tokio::task::JoinHandle<()>,
    server_task: tokio::task::JoinHandle<()>,
    _tmpdir: TempDir,
}

impl TestNode {
    async fn spawn() -> Self {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");

        let listen: Multiaddr = tokio::time::timeout(TIMEOUT, async {
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
            .expect("client connects to the control socket")
    }

    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        self.server_task.abort();
        let _ = tokio::time::timeout(Duration::from_secs(5), self.service_task).await;
    }
}

// ============================================================================
// Simple verbs
// ============================================================================

/// IDENTIFY must return the node's real peer ID and parseable binary
/// multiaddrs. The byte formats are the contract: `node.rs` hex-decodes the ID
/// and `Multiaddr::try_from`s each address during self-discovery.
#[tokio::test]
async fn identify_returns_peer_id_and_listen_addrs() {
    let node = TestNode::spawn().await;
    let mut client = node.client().await;

    let (peer_hex, addrs) = client.identify_with_addrs().await.expect("identify");

    let decoded = PeerId::from_bytes(&hex::decode(&peer_hex).expect("hex peer id"))
        .expect("peer id bytes must parse");
    assert_eq!(
        decoded, node.peer_id,
        "IDENTIFY must report our own peer id"
    );

    assert!(!addrs.is_empty(), "a listening node must report addresses");
    for raw in &addrs {
        Multiaddr::try_from(raw.clone()).expect("addrs must be binary multiaddrs, not strings");
    }

    node.shutdown().await;
}

/// CONNECT dials a second native node, and LIST_PEERS then shows it. Together
/// these are what `kwaainet status` and `p2p peers list` depend on.
#[tokio::test]
async fn connect_then_list_peers_shows_the_remote() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;
    let mut client = a.client().await;

    tokio::time::timeout(TIMEOUT, client.connect_peer(&b.addr))
        .await
        .expect("connect must not hang")
        .expect("dialing a live node must succeed");

    let peers = client.list_peers().await.expect("list_peers");
    assert!(
        peers
            .iter()
            .any(|p| PeerId::from_bytes(&p.id).ok() == Some(b.peer_id)),
        "the peer we just dialed must appear in LIST_PEERS"
    );
    for p in &peers {
        assert!(
            !p.addrs.is_empty() && Multiaddr::try_from(p.addrs[0].clone()).is_ok(),
            "each PeerInfo carries one binary multiaddr, as the Go daemon emits"
        );
    }

    // DISCONNECT is the inverse and must clear the entry.
    client
        .disconnect_peer(&b.peer_id.to_bytes())
        .await
        .expect("disconnect");
    let peers = client
        .list_peers()
        .await
        .expect("list_peers after disconnect");
    assert!(
        !peers
            .iter()
            .any(|p| PeerId::from_bytes(&p.id).ok() == Some(b.peer_id)),
        "a disconnected peer must not be listed"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// A malformed CONNECT must come back as a protocol error on the socket, not
/// close the connection — the client keeps using it afterwards.
#[tokio::test]
async fn errors_are_reported_without_killing_the_connection() {
    let node = TestNode::spawn().await;
    let mut client = node.client().await;

    let err = client
        .connect_peer(
            "/ip4/127.0.0.1/tcp/1/p2p/12D3KooWA8EXV3KjBxEU9NMLgvcnpNHfkFhtomdNGCjnwBmU8YZQ",
        )
        .await
        .expect_err("dialling a dead port must fail");
    assert!(
        err.to_string().contains("Daemon error"),
        "a failed dial surfaces as an ERROR response, got: {err}"
    );

    // The same client must still work.
    client
        .identify()
        .await
        .expect("the connection survives an error response");

    node.shutdown().await;
}

/// Verbs this slice does not serve must be refused cleanly with the Go daemon's
/// wording, never hang and never half-succeed.
#[tokio::test]
async fn unsupported_verbs_are_refused_with_the_go_error_shape() {
    let node = TestNode::spawn().await;
    let mut client = node.client().await;

    let err = client
        .register_stream_handler("/ip4/127.0.0.1/tcp/9999", vec![PROTO.to_string()])
        .await
        .expect_err("stream handlers are deferred to the pipe-mode continuation");
    assert!(
        err.to_string().contains("not supported"),
        "expected the Go daemon's 'not supported', got: {err}"
    );

    // And the socket is still usable, so a client that probes an unsupported
    // verb is not left with a dead connection.
    client.identify().await.expect("connection still usable");

    node.shutdown().await;
}

// ============================================================================
// Persistent connection: unary handlers
// ============================================================================

/// The core Phase 3 path: a socket client registers a handler, and a **remote**
/// node's call is routed through the swarm, out to that client, and its answer
/// back. This is what `shard serve` and `storage serve` do.
#[tokio::test]
async fn a_socket_client_serves_a_remote_callers_unary_call() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let client = a.client().await;
    client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"client:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register a handler over the control socket");

    let response = tokio::time::timeout(
        TIMEOUT,
        call_from(&b, a.peer_id, &a.addr, PROTO, b"from-remote"),
    )
    .await
    .expect("the call must resolve")
    .expect("a socket client's handler must answer a remote caller");
    assert_eq!(response, b"client:from-remote");

    a.shutdown().await;
    b.shutdown().await;
}

/// `call_unary_handler` in the other direction: a socket client calls out
/// through its node to a handler on the remote node.
#[tokio::test]
async fn a_socket_client_calls_a_remote_handler() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    b.handle
        .add_unary_handler(PROTO, |data: Vec<u8>| async move {
            let mut out = b"remote:".to_vec();
            out.extend_from_slice(&data);
            Ok(out)
        })
        .await
        .expect("register a handler on the remote node");

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial remote");

    let client = a.client().await;
    let response = tokio::time::timeout(
        TIMEOUT,
        client.call_unary_handler(&b.peer_id.to_bytes(), PROTO, b"from-socket"),
    )
    .await
    .expect("the call must resolve")
    .expect("the server must translate callUnary into a swarm call");
    assert_eq!(response, b"remote:from-socket");

    a.shutdown().await;
    b.shutdown().await;
}

/// **The stale-handler fix.** When a client dies, its handlers must leave the
/// swarm — otherwise the node keeps advertising a protocol nothing will answer
/// and remote callers hang until their own timeout instead of getting a clean
/// refusal.
///
/// Deregistration is only observable remotely, so that is how it is asserted.
#[tokio::test]
async fn handlers_are_released_when_their_client_disconnects() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    {
        let client = a.client().await;
        client
            .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
            .await
            .expect("register a handler");

        let response =
            tokio::time::timeout(TIMEOUT, call_from(&b, a.peer_id, &a.addr, PROTO, b"alive"))
                .await
                .expect("the call must resolve")
                .expect("the handler answers while its client is connected");
        assert_eq!(response, b"alive");
        // Dropping the client closes the socket — the crash case.
    }

    // Deregistration is asynchronous with respect to the drop; allow the
    // server's cleanup task to run.
    let refused = tokio::time::timeout(TIMEOUT, async {
        loop {
            match call_from(&b, a.peer_id, &a.addr, PROTO, b"after").await {
                Err(e) => return e,
                Ok(_) => tokio::time::sleep(Duration::from_millis(50)).await,
            }
        }
    })
    .await
    .expect("the protocol must stop being served after the client disconnects");

    let text = refused.to_string();
    assert!(
        text.contains("does not support") || text.contains("not support"),
        "a released protocol must be refused during negotiation, got: {text}"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// A protocol another connection already owns must be refused, with the Go
/// daemon's exact wording — and the first client must keep serving it.
#[tokio::test]
async fn a_second_client_cannot_steal_a_registered_protocol() {
    let a = TestNode::spawn().await;

    let first = a.client().await;
    first
        .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
        .await
        .expect("first registration wins");

    let second = a.client().await;
    let err = second
        .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
        .await
        .expect_err("a second client must not take over a served protocol");
    assert!(
        err.to_string().contains("already set"),
        "expected Go's 'handler for protocol X already set', got: {err}"
    );

    // Removing someone else's handler is refused too — otherwise any client
    // could silently unregister another process's protocol.
    let err = second
        .remove_unary_handler(PROTO)
        .await
        .expect_err("removing a protocol we do not own must fail");
    assert!(
        err.to_string()
            .contains("not created in this persistent connection"),
        "expected Go's ownership error, got: {err}"
    );

    a.shutdown().await;
}

/// Two clients, each serving their own protocol, answering concurrent inbound
/// calls. Correlation state is per connection, so the responses must not
/// cross-talk even though both connections number their calls independently.
#[tokio::test]
async fn two_clients_with_handlers_stay_isolated() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    const PROTO_ONE: &str = "DHTProtocol.rpc_store";
    const PROTO_TWO: &str = "DHTProtocol.rpc_find";

    let one = a.client().await;
    one.add_unary_handler(
        PROTO_ONE,
        |req: Vec<u8>| async move {
            let mut out = b"one:".to_vec();
            out.extend_from_slice(&req);
            Ok(out)
        },
        false,
    )
    .await
    .expect("client one registers");

    let two = a.client().await;
    two.add_unary_handler(
        PROTO_TWO,
        |req: Vec<u8>| async move {
            let mut out = b"two:".to_vec();
            out.extend_from_slice(&req);
            Ok(out)
        },
        false,
    )
    .await
    .expect("client two registers");

    // Concurrent calls to both protocols from the remote node.
    let (r1, r2) = tokio::join!(
        call_from(&b, a.peer_id, &a.addr, PROTO_ONE, b"x"),
        call_from(&b, a.peer_id, &a.addr, PROTO_TWO, b"y"),
    );

    assert_eq!(
        r1.expect("protocol one answers"),
        b"one:x",
        "each protocol's answer must come from its own client"
    );
    assert_eq!(r2.expect("protocol two answers"), b"two:y");

    a.shutdown().await;
    b.shutdown().await;
}

/// A handler that returns an error must reach the remote caller as an error
/// arm, not as a daemon-level failure and not as a hang.
#[tokio::test]
async fn handler_errors_propagate_to_the_remote_caller() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let client = a.client().await;
    client
        .add_unary_handler(
            PROTO,
            |_req: Vec<u8>| async move {
                Err(kwaai_p2p_daemon::Error::Protocol("handler said no".into()))
            },
            false,
        )
        .await
        .expect("register a failing handler");

    let err = tokio::time::timeout(TIMEOUT, call_from(&b, a.peer_id, &a.addr, PROTO, b"x"))
        .await
        .expect("the call must resolve, not hang")
        .expect_err("a handler error must surface as an error");
    assert!(
        err.to_string().contains("handler said no"),
        "the handler's own message must be preserved, got: {err}"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// `remove_unary_handler` over the socket must take the protocol off the swarm,
/// observable to a remote caller as a clean refusal.
#[tokio::test]
async fn explicit_removal_stops_serving_the_protocol() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let client = a.client().await;
    client
        .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
        .await
        .expect("register");

    let ok = tokio::time::timeout(TIMEOUT, call_from(&b, a.peer_id, &a.addr, PROTO, b"before"))
        .await
        .expect("resolves")
        .expect("served while registered");
    assert_eq!(ok, b"before");

    client
        .remove_unary_handler(PROTO)
        .await
        .expect("removing our own handler succeeds");

    let err = tokio::time::timeout(TIMEOUT, call_from(&b, a.peer_id, &a.addr, PROTO, b"after"))
        .await
        .expect("resolves")
        .expect_err("a removed protocol must be refused");
    assert!(
        err.to_string().contains("does not support") || err.to_string().contains("not support"),
        "expected a negotiation refusal, got: {err}"
    );

    a.shutdown().await;
    b.shutdown().await;
}

// ============================================================================
// Helpers
// ============================================================================

/// Call `proto` on `target` from node `from`'s swarm, dialling if needed.
///
/// Uses the *handle* rather than a socket client so that a failure is
/// unambiguously about the server under test, not about a second control
/// socket.
async fn call_from(
    from: &TestNode,
    target: PeerId,
    target_addr: &str,
    proto: &str,
    data: &[u8],
) -> Result<Vec<u8>, kwaai_p2p::P2PError> {
    let addr: Multiaddr = target_addr.parse().expect("valid multiaddr");
    // Seed the routing table so the unary behaviour can dial on demand, which
    // is how production reaches a peer it is not yet connected to.
    let _ = from.handle.add_kad_address(target, addr).await;
    from.handle.call_unary_handler(target, proto, data).await
}
