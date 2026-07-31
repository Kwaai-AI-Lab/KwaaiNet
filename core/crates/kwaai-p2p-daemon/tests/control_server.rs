//! In-process conformance suite for [`ControlServer`] — old client, new server.
//!
//! Every test here drives the **unmodified** [`P2PClient`] against a
//! [`ControlServer`] backed by a real [`NetworkService`]: if these pass, the
//! ~15 external client call sites work against a node with no Go daemon behind
//! it. Nothing here constructs a protobuf frame by hand — that would test the
//! server against a re-reading of the protocol rather than the client that
//! ships.
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

/// Verbs this server does not serve must be refused cleanly with the Go
/// daemon's wording, never hang and never half-succeed.
#[tokio::test]
async fn unsupported_verbs_are_refused_with_the_go_error_shape() {
    let node = TestNode::spawn().await;
    let mut client = node.client().await;

    // A DHT verb backed by neither kad records nor the hivemind path.
    let err = client
        .dht_put_value(b"k".to_vec(), b"v".to_vec(), None)
        .await
        .expect_err("DHT put is not served");
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
// Pipe mode: raw byte relay
// ============================================================================
//
// The topology for every test here:
//
// ```text
//   [P2PClient A] ──STREAM_OPEN──▶ [ControlServer A] ─libp2p─▶ [ControlServer B] ──dial──▶ [TcpListener]
//                  (socket becomes                                                          registered by
//                   the data channel)                                                       [P2PClient B]
// ```
//
// Client B registers a stream handler pointing at a plain `TcpListener` it owns
// — exactly what `inference_mux.rs` and `node.rs` do — and client A reaches it
// with `stream_open_raw`. What is being proven is that bytes written at one end
// of that chain come out the other end unchanged, in both directions, and that
// the chain tears down without leaking when either end stops.

/// A stream-handler endpoint: a TCP listener registered with a node's control
/// socket, with the `P2PClient` that owns the registration held alive (dropping
/// it releases the handler — that is the disconnect fix, asserted below).
struct StreamEndpoint {
    listener: tokio::net::TcpListener,
    _client: P2PClient,
}

impl StreamEndpoint {
    /// Bind a loopback listener and register it for `protos` on `node`.
    async fn register(node: &TestNode, protos: Vec<String>) -> Self {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind a handler listener");
        let addr = format!(
            "/ip4/127.0.0.1/tcp/{}",
            listener.local_addr().unwrap().port()
        );

        let mut client = node.client().await;
        client
            .register_stream_handler(&addr, protos)
            .await
            .expect("STREAM_HANDLER must be served");

        Self {
            listener,
            _client: client,
        }
    }

    /// Accept one forwarded stream and consume its `StreamInfo` prologue,
    /// returning the socket positioned at the first application byte.
    ///
    /// Every real consumer does this (`inference_mux.rs::read_p2pd_stream_info`),
    /// so a test that skipped it would not be testing the wire they see.
    async fn accept(&self) -> (tokio::net::TcpStream, kwaai_p2p_daemon::p2pd::StreamInfo) {
        let (mut socket, _) = tokio::time::timeout(TIMEOUT, self.listener.accept())
            .await
            .expect("a forwarded stream must arrive")
            .expect("accept");
        let info = kwaai_p2p_daemon::stream::parse_stream_info(&mut socket)
            .await
            .expect("the daemon protocol prologues each forwarded stream with StreamInfo");
        (socket, info)
    }
}

/// The end-to-end pipe: `stream_open_raw` on one node reaches a stream handler
/// registered on another, and bytes cross verbatim in both directions.
///
/// This is the inference-mux path in miniature, and the single most important
/// assertion in this file: it proves the socket really did stop being a frame
/// channel and became the libp2p stream.
#[tokio::test]
async fn stream_open_raw_relays_bytes_to_a_registered_stream_handler() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    const STREAM_PROTO: &str = "kwaai.mux";

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let endpoint = StreamEndpoint::register(&b, vec![STREAM_PROTO.to_string()]).await;

    // The handler side: read a request, answer it.
    let handler = tokio::spawn(async move {
        let (mut socket, info) = endpoint.accept().await;
        assert_eq!(
            info.proto, STREAM_PROTO,
            "StreamInfo must name the negotiated protocol"
        );
        assert!(
            PeerId::from_bytes(&info.peer).is_ok(),
            "StreamInfo.peer must be raw peer-id bytes, as Go's makeStreamInfo emits"
        );

        let mut request = [0u8; 12];
        socket.read_exact(&mut request).await.expect("read request");
        socket
            .write_all(b"pong-from-handler")
            .await
            .expect("write reply");
        socket.flush().await.expect("flush");
        (request.to_vec(), info)
    });

    // Dial B so the stream open has a connection to use.
    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    // `stream_open_raw` consumes the client: the socket *is* the data channel.
    let client = a.client().await;
    let mut stream = tokio::time::timeout(
        TIMEOUT,
        client.stream_open_raw(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
    )
    .await
    .expect("STREAM_OPEN must resolve")
    .expect("STREAM_OPEN to a registered protocol must succeed");

    stream.write_all(b"ping-request").await.expect("write");
    stream.flush().await.expect("flush");

    let mut reply = [0u8; 17];
    tokio::time::timeout(TIMEOUT, stream.read_exact(&mut reply))
        .await
        .expect("the reply must arrive")
        .expect("read the reply");
    assert_eq!(
        &reply, b"pong-from-handler",
        "handler → caller bytes must cross the pipe verbatim"
    );

    let (request, info) = tokio::time::timeout(TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("the handler task must not panic");
    assert_eq!(
        request, b"ping-request",
        "caller → handler bytes must cross the pipe verbatim"
    );
    assert_eq!(
        PeerId::from_bytes(&info.peer).unwrap(),
        a.peer_id,
        "StreamInfo must identify the *calling* peer, taken from the connection"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// **Backpressure and volume through the full socket → libp2p → socket chain.**
///
/// 2 MiB each way, with the far end reading in small chunks. Two hops of
/// `copy_bidirectional` sit in this path (one per node), and each awaits its
/// write before reading more — so a regression that buffered without bound, or
/// truncated at a window boundary, fails here rather than in production under a
/// large inference response.
#[tokio::test]
async fn a_large_payload_survives_the_relay_in_both_directions() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    const STREAM_PROTO: &str = "kwaai.bulk";
    const SIZE: usize = 2 * 1024 * 1024;

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let endpoint = StreamEndpoint::register(&b, vec![STREAM_PROTO.to_string()]).await;

    let handler = tokio::spawn(async move {
        let (socket, _) = endpoint.accept().await;
        let (mut reader, mut writer) = socket.into_split();

        // Read and write concurrently: doing either to completion first would
        // deadlock once both directions fill, which is the backpressure being
        // asserted.
        let read_side = tokio::spawn(async move {
            let mut got = vec![0u8; SIZE];
            reader.read_exact(&mut got).await.expect("read the payload");
            got
        });
        let out: Vec<u8> = (0..SIZE).map(|i| (i % 251) as u8).collect();
        writer.write_all(&out).await.expect("write the reply");
        writer.flush().await.expect("flush");
        read_side.await.expect("read task")
    });

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    let client = a.client().await;
    let stream = tokio::time::timeout(
        TIMEOUT,
        client.stream_open_raw(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
    )
    .await
    .expect("STREAM_OPEN must resolve")
    .expect("open");

    let sent: Vec<u8> = (0..SIZE).map(|i| (i % 253) as u8).collect();
    let expected_back: Vec<u8> = (0..SIZE).map(|i| (i % 251) as u8).collect();

    let (mut reader, mut writer) = tokio::io::split(stream);
    let sent_for_writer = sent.clone();
    let write_side = tokio::spawn(async move {
        writer.write_all(&sent_for_writer).await.expect("write all");
        writer.flush().await.expect("flush");
    });
    let read_side = tokio::spawn(async move {
        let mut got = Vec::with_capacity(SIZE);
        // Small chunks so the reader drains slower than the writer fills.
        let mut buf = vec![0u8; 8 * 1024];
        while got.len() < SIZE {
            let n = reader.read(&mut buf).await.expect("read");
            if n == 0 {
                break;
            }
            got.extend_from_slice(&buf[..n]);
        }
        got
    });

    write_side.await.expect("the write side must not panic");
    let received_back = tokio::time::timeout(TIMEOUT, read_side)
        .await
        .expect("the reply must arrive in full")
        .expect("read task");
    let received_by_handler = tokio::time::timeout(TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("handler task");

    assert_eq!(
        received_by_handler.len(),
        SIZE,
        "the handler must receive every byte, not a window's worth"
    );
    assert_eq!(
        received_by_handler, sent,
        "caller → handler bytes must arrive verbatim and in order"
    );
    assert_eq!(
        received_back, expected_back,
        "handler → caller bytes must arrive verbatim and in order"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// A half-close by the caller must reach the handler as EOF while the reply
/// direction still works.
///
/// This is how a request/response protocol over a raw stream signals "request
/// complete", and `copy_bidirectional` must forward the FIN rather than tearing
/// the whole relay down — otherwise the reply the caller is waiting for never
/// gets written.
#[tokio::test]
async fn a_caller_half_close_reaches_the_handler_as_eof() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    const STREAM_PROTO: &str = "kwaai.halfclose";

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let endpoint = StreamEndpoint::register(&b, vec![STREAM_PROTO.to_string()]).await;

    let handler = tokio::spawn(async move {
        let (mut socket, _) = endpoint.accept().await;
        // Reads to EOF, which only arrives because the caller half-closed.
        let mut request = Vec::new();
        socket
            .read_to_end(&mut request)
            .await
            .expect("read to EOF after the caller's half-close");
        // The reply direction must still be alive after that EOF.
        socket
            .write_all(b"answered-after-eof")
            .await
            .expect("the reply direction survives the caller's half-close");
        socket.flush().await.expect("flush");
        drop(socket);
        request
    });

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    let client = a.client().await;
    let stream = tokio::time::timeout(
        TIMEOUT,
        client.stream_open_raw(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
    )
    .await
    .expect("STREAM_OPEN must resolve")
    .expect("open");

    let (mut reader, mut writer) = tokio::io::split(stream);
    writer.write_all(b"request-body").await.expect("write");
    writer.flush().await.expect("flush");
    writer.shutdown().await.expect("half-close the write side");

    let mut reply = Vec::new();
    tokio::time::timeout(TIMEOUT, reader.read_to_end(&mut reply))
        .await
        .expect("the reply must arrive")
        .expect("read");
    assert_eq!(
        reply, b"answered-after-eof",
        "the reply direction must survive the caller's half-close"
    );

    let request = tokio::time::timeout(TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("handler task");
    assert_eq!(
        request, b"request-body",
        "the handler must see exactly the bytes written before the half-close"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// `STREAM_OPEN` for a protocol nobody serves must come back as an `ERROR`
/// response on a socket that is **still framing** — the connection must not be
/// consumed by a pipe that was never established.
#[tokio::test]
async fn stream_open_for_an_unserved_protocol_errors_without_entering_pipe_mode() {
    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let mut client = a.client().await;
    client.connect_peer(&b.addr).await.expect("dial B");

    let err = tokio::time::timeout(
        TIMEOUT,
        client.stream_open(
            &b.peer_id.to_bytes(),
            vec!["nobody.serves.this".to_string()],
        ),
    )
    .await
    .expect("STREAM_OPEN must resolve, not hang")
    .expect_err("an unserved protocol must be refused");
    assert!(
        err.to_string().contains("does not support") || err.to_string().contains("not support"),
        "expected a negotiation refusal, got: {err}"
    );

    // The socket must still speak the framed protocol. If the server had
    // entered pipe mode on a failed open, this would hang or return garbage.
    client
        .identify()
        .await
        .expect("a failed STREAM_OPEN must leave the connection in framing mode");

    a.shutdown().await;
    b.shutdown().await;
}

/// **The stale-handler fix, for raw streams.** When the client that registered a
/// stream handler disconnects, the protocol must stop being served.
///
/// This is a deliberate divergence from Go, whose handler map is process-global:
/// there, a crashed `shard serve` leaves the daemon advertising a protocol whose
/// forwarding address refuses connections, so every inbound stream costs a dial
/// timeout instead of an immediate refusal. Observable only remotely, so that is
/// how it is asserted.
#[tokio::test]
async fn stream_handlers_are_released_when_their_client_disconnects() {
    const STREAM_PROTO: &str = "kwaai.released";

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    {
        let endpoint = StreamEndpoint::register(&b, vec![STREAM_PROTO.to_string()]).await;

        // Prove it is being served while the registering client is alive.
        let accepted = tokio::spawn(async move {
            let _ = endpoint.accept().await;
        });
        let client = a.client().await;
        let _stream = tokio::time::timeout(
            TIMEOUT,
            client.stream_open_raw(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
        )
        .await
        .expect("resolves")
        .expect("served while the registering client is connected");
        tokio::time::timeout(TIMEOUT, accepted)
            .await
            .expect("the handler must be reached")
            .expect("accept task");
        // Dropping `endpoint` closes the registering client's socket — the
        // crash case — and drops the listener with it.
    }

    // Deregistration is asynchronous with respect to the drop; poll until the
    // protocol stops negotiating.
    let refused = tokio::time::timeout(TIMEOUT, async {
        loop {
            let mut client = a.client().await;
            match client
                .stream_open(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()])
                .await
            {
                Err(e) => return e,
                Ok(_) => tokio::time::sleep(Duration::from_millis(50)).await,
            }
        }
    })
    .await
    .expect("a released stream protocol must stop being served");

    let text = refused.to_string();
    assert!(
        text.contains("does not support") || text.contains("not support"),
        "a released protocol must be refused during negotiation, got: {text}"
    );

    a.shutdown().await;
    b.shutdown().await;
}

/// Explicit `REMOVE_STREAM_HANDLER` must take the protocol off the swarm, and a
/// client must not be able to remove a protocol it does not own.
#[tokio::test]
async fn removing_a_stream_handler_stops_serving_it_and_respects_ownership() {
    const STREAM_PROTO: &str = "kwaai.removable";

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let handler_addr = format!(
        "/ip4/127.0.0.1/tcp/{}",
        listener.local_addr().unwrap().port()
    );

    let mut owner = b.client().await;
    owner
        .register_stream_handler(&handler_addr, vec![STREAM_PROTO.to_string()])
        .await
        .expect("register");

    // A second client must not be able to steal or remove it.
    let mut intruder = b.client().await;
    let err = intruder
        .register_stream_handler(&handler_addr, vec![STREAM_PROTO.to_string()])
        .await
        .expect_err("a second client must not take over a registered stream protocol");
    assert!(
        err.to_string().contains("already set"),
        "expected Go's 'handler for protocol X already set', got: {err}"
    );

    let err = intruder
        .remove_stream_handler(&handler_addr, vec![STREAM_PROTO.to_string()])
        .await
        .expect_err("removing another client's handler must be refused");
    assert!(
        err.to_string().contains("not created in this connection"),
        "expected an ownership error, got: {err}"
    );

    // The owner can remove it, and then the protocol stops negotiating.
    owner
        .remove_stream_handler(&handler_addr, vec![STREAM_PROTO.to_string()])
        .await
        .expect("removing our own handler succeeds");

    let mut client = a.client().await;
    let err = tokio::time::timeout(
        TIMEOUT,
        client.stream_open(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
    )
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

/// A stream handler whose listener has gone away must **reset** the inbound
/// stream rather than leave it open.
///
/// Go does this (`handleStream`: `s.Reset()` when `manet.Dial` fails), and it
/// matters because the caller is otherwise holding an established stream that
/// nothing will ever answer — it would wait out its own application timeout
/// instead of failing at once.
#[tokio::test]
async fn a_dial_back_failure_resets_the_inbound_stream() {
    use tokio::io::AsyncReadExt;

    const STREAM_PROTO: &str = "kwaai.deadlistener";

    let a = TestNode::spawn().await;
    let b = TestNode::spawn().await;

    // Register a handler, then drop the listener while keeping the registration
    // alive. The registering client stays connected, so the protocol is still
    // advertised — only the dial-back target is dead.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let handler_addr = format!(
        "/ip4/127.0.0.1/tcp/{}",
        listener.local_addr().unwrap().port()
    );
    let mut owner = b.client().await;
    owner
        .register_stream_handler(&handler_addr, vec![STREAM_PROTO.to_string()])
        .await
        .expect("register");
    drop(listener);

    let mut connector = a.client().await;
    connector.connect_peer(&b.addr).await.expect("dial B");

    // Negotiation still succeeds — the protocol is advertised.
    let client = a.client().await;
    let mut stream = tokio::time::timeout(
        TIMEOUT,
        client.stream_open_raw(&b.peer_id.to_bytes(), vec![STREAM_PROTO.to_string()]),
    )
    .await
    .expect("resolves")
    .expect("an advertised protocol negotiates even if the dial-back will fail");

    // But the stream must then die rather than hang: the reset propagates as
    // EOF or an error on the caller's socket.
    let mut buf = [0u8; 64];
    let outcome = tokio::time::timeout(TIMEOUT, stream.read(&mut buf))
        .await
        .expect("a reset stream must not hang the caller");
    match outcome {
        Ok(0) => {}
        Err(_) => {}
        Ok(n) => panic!("expected a reset, got {n} bytes of data"),
    }

    a.shutdown().await;
    b.shutdown().await;
}

/// Registering an address that is not a dialable TCP endpoint must fail at
/// registration, not silently per inbound stream.
#[tokio::test]
async fn a_stream_handler_address_must_be_dialable() {
    let node = TestNode::spawn().await;
    let mut client = node.client().await;

    let err = client
        .register_stream_handler("/ip4/127.0.0.1/udp/9999/quic-v1", vec!["x.y".to_string()])
        .await
        .expect_err("a non-TCP handler address cannot be dialled back");
    assert!(
        err.to_string().contains("dialable"),
        "the error must say why the address was rejected, got: {err}"
    );

    client.identify().await.expect("connection still usable");

    node.shutdown().await;
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
