//! Pipe mode ↔ real p2pd — the Phase 3 slice-2 gate.
//!
//! Tier 11 proved the ControlServer's *unary* verbs are indistinguishable from a
//! Go daemon's. This tier does the same for **pipe mode**: the raw byte relay
//! behind `STREAM_OPEN` and `STREAM_HANDLER`, which is what inference-mux and
//! `node.rs`'s `rpc_*` forwarding run on.
//!
//! Unlike the unary path, pipe mode has no framing of its own to check — the
//! whole contract is "the bytes that go in come out". So each cell here opens a
//! stream across an implementation boundary and asserts payload fidelity in both
//! directions, plus the one piece of structure that does exist: the
//! length-delimited `StreamInfo` prologue every consumer parses before its own
//! protocol starts.
//!
//! # The matrix
//!
//! "via" is the implementation whose control socket the client is attached to;
//! the stream handler is always registered through the *other* implementation's
//! socket, so every relay crosses two socket boundaries and the libp2p wire.
//!
//! ```text
//!   [P2PClient N] ──socket──▶ [ControlServer + NetworkService] ◀──libp2p──▶ [p2pd] ◀──socket── [P2PClient G]
//!        stream_open_raw                                                              register_stream_handler
//! ```
//!
//! | # | caller (client via) | stream handler (registered via) | test |
//! | --- | --- | --- | --- |
//! | 1 | **native** `ControlServer` | p2pd | `native_client_streams_to_a_daemon_stream_handler` |
//! | 2 | p2pd | **native** `ControlServer` | `daemon_client_streams_to_a_native_stream_handler` |
//! | 3 | **native** | **native** (2nd node) | covered in-process by `control_server.rs` — not repeated |
//! | 4 | p2pd | p2pd | covered by tier 07 — the pre-existing baseline |
//!
//! Cells 1 and 2 are the mixed fleet during the cutover window: cell 1 is a
//! migrated node's `inference_mux` client reaching a GPU node that still runs
//! the Go daemon, and cell 2 is the reverse. Both must work byte-for-byte or the
//! fleet partitions along the migration boundary.
//!
//! Two further checks that are not simple round trips:
//!
//! - `stream_info_prologue_matches_between_implementations` — the prologue a
//!   handler reads is shaped identically by both daemons. `inference_mux.rs`
//!   consumes it blind (reads the varint, discards the body), so a divergence in
//!   framing would desynchronise the mux frame loop rather than fail loudly.
//! - `a_large_payload_survives_a_cross_implementation_relay` — 1 MiB each way
//!   across the boundary, which is where backpressure differences between
//!   `io.Copy` and `copy_bidirectional` would show up.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`.
//!
//! # Process hygiene
//!
//! Daemons come from `TestNode::new_wire_peer`, which kills its own child by PID
//! on drop — nothing kills by name or touches the default socket path. The
//! native side listens on an ephemeral loopback port with a fresh key and a
//! tmpdir socket, so it cannot collide with a node running on the same machine.

use std::time::Duration;

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use kwaai_p2p_daemon::{ControlServer, P2PClient};
use libp2p::identity::Keypair;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// A slash-less protocol name, as everything else in this codebase uses.
const PROTO: &str = "kwaai.pipe_interop";

/// Cap on any single daemon interaction, so a regression fails rather than
/// hanging the suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(30);

/// A native node: `NetworkService` behind a `ControlServer`.
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

/// Bind an ephemeral loopback listener for a stream handler, as every real
/// consumer does (`inference_mux.rs`, `node.rs`).
async fn handler_listener() -> (TcpListener, String) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a handler listener");
    let port = listener.local_addr().expect("local addr").port();
    (listener, format!("/ip4/127.0.0.1/tcp/{port}"))
}

/// Consume the `StreamInfo` prologue and return it.
///
/// Both implementations prologue a forwarded stream with one length-delimited
/// `StreamInfo`; `stream.rs::parse_stream_info` is the parser every consumer
/// already uses, so reading it through that function is what makes these tests
/// speak the consumers' wire rather than a re-derivation of it.
async fn read_prologue(socket: &mut tokio::net::TcpStream) -> kwaai_p2p_daemon::p2pd::StreamInfo {
    kwaai_p2p_daemon::stream::parse_stream_info(socket)
        .await
        .expect("every forwarded stream is prologued with a StreamInfo")
}

/// Serve one forwarded stream: read `expect_len` bytes, reply with `reply`.
///
/// Returns the request bytes and the prologue, so a caller can assert on both.
async fn serve_one(
    listener: TcpListener,
    expect_len: usize,
    reply: Vec<u8>,
) -> (Vec<u8>, kwaai_p2p_daemon::p2pd::StreamInfo) {
    let (mut socket, _) = tokio::time::timeout(CALL_TIMEOUT, listener.accept())
        .await
        .expect("a forwarded stream must arrive")
        .expect("accept");
    let info = read_prologue(&mut socket).await;

    let mut request = vec![0u8; expect_len];
    socket
        .read_exact(&mut request)
        .await
        .expect("read the request payload");
    socket.write_all(&reply).await.expect("write the reply");
    socket.flush().await.expect("flush");
    (request, info)
}

// ============================================================================
// Matrix cell 1: native client → p2pd stream handler
// ============================================================================

/// A socket client on the **native** node opens a raw stream to a handler
/// registered through a **Go daemon's** socket, and bytes cross both ways.
///
/// The cutover-window direction that matters most: a migrated node's
/// inference-mux client reaching a GPU node that still runs the Go daemon.
#[tokio::test]
async fn native_client_streams_to_a_daemon_stream_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::pipe_mode::native_client_to_daemon_handler",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let (listener, handler_addr) = handler_listener().await;
    daemon
        .client
        .register_stream_handler(&handler_addr, vec![PROTO.to_string()])
        .await
        .expect("register a stream handler over the daemon's socket");

    let native = NativeNode::spawn().await;
    let (daemon_id, daemon_addr) = daemon_peer(&daemon);

    // The native side must know how to reach the daemon; `open_raw_stream`
    // dials on demand from the routing table, as Go's `host.NewStream` does.
    native
        .handle
        .add_kad_address(daemon_id, daemon_addr)
        .await
        .expect("seed the routing table");

    let handler = tokio::spawn(serve_one(
        listener,
        b"native-to-daemon".len(),
        b"daemon-handler-reply".to_vec(),
    ));

    // The socket becomes the data channel; `stream_open_raw` consumes the client.
    let client = native.client().await;
    let mut stream = tokio::time::timeout(
        CALL_TIMEOUT,
        client.stream_open_raw(&daemon_id.to_bytes(), vec![PROTO.to_string()]),
    )
    .await
    .expect("STREAM_OPEN must resolve")
    .expect("a native ControlServer must open a raw stream to a Go daemon");

    stream.write_all(b"native-to-daemon").await.expect("write");
    stream.flush().await.expect("flush");

    let mut reply = [0u8; 20];
    tokio::time::timeout(CALL_TIMEOUT, stream.read_exact(&mut reply))
        .await
        .expect("the reply must arrive")
        .expect("read the reply");
    assert_eq!(
        &reply, b"daemon-handler-reply",
        "handler → caller bytes must cross the implementation boundary verbatim"
    );

    let (request, info) = tokio::time::timeout(CALL_TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("handler task");
    assert_eq!(
        request, b"native-to-daemon",
        "caller → handler bytes must cross verbatim"
    );
    assert_eq!(
        PeerId::from_bytes(&info.peer).expect("StreamInfo.peer must be raw peer-id bytes"),
        native.peer_id,
        "the daemon's prologue must identify the native node as the caller"
    );

    rec.metric("reply_len", reply.len());
    native.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Matrix cell 2: p2pd client → native stream handler
// ============================================================================

/// A socket client on a **Go daemon** opens a raw stream to a handler registered
/// through the **native** node's socket.
///
/// The reverse cutover direction, and the one that exercises the whole inbound
/// path: an inbound libp2p stream → `raw_stream::Behaviour` → the service's
/// dispatch map → the dial-back to the client's listener → the StreamInfo
/// prologue → the relay.
#[tokio::test]
async fn daemon_client_streams_to_a_native_stream_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::pipe_mode::daemon_client_to_native_handler",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let native = NativeNode::spawn().await;

    let (listener, handler_addr) = handler_listener().await;
    let mut native_client = native.client().await;
    native_client
        .register_stream_handler(&handler_addr, vec![PROTO.to_string()])
        .await
        .expect("register a stream handler over the native control socket");

    daemon
        .client
        .connect_peer(&native.addr)
        .await
        .expect("the daemon must dial the native node");

    let handler = tokio::spawn(serve_one(
        listener,
        b"daemon-to-native".len(),
        b"native-handler-reply".to_vec(),
    ));

    // A fresh daemon client, because `stream_open_raw` consumes it.
    let daemon_stream_client = P2PClient::connect(&daemon.socket_addr)
        .await
        .expect("a second client on the daemon socket");
    let mut stream = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon_stream_client.stream_open_raw(&native.peer_id.to_bytes(), vec![PROTO.to_string()]),
    )
    .await
    .expect("STREAM_OPEN must resolve")
    .expect("a Go daemon must open a raw stream to a native node's stream handler");

    stream.write_all(b"daemon-to-native").await.expect("write");
    stream.flush().await.expect("flush");

    let mut reply = [0u8; 20];
    tokio::time::timeout(CALL_TIMEOUT, stream.read_exact(&mut reply))
        .await
        .expect("the reply must arrive")
        .expect("read the reply");
    assert_eq!(
        &reply, b"native-handler-reply",
        "handler → caller bytes must cross the implementation boundary verbatim"
    );

    let (request, info) = tokio::time::timeout(CALL_TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("handler task");
    assert_eq!(
        request, b"daemon-to-native",
        "caller → handler bytes must cross verbatim"
    );

    let (daemon_id, _) = daemon_peer(&daemon);
    assert_eq!(
        PeerId::from_bytes(&info.peer).expect("StreamInfo.peer must be raw peer-id bytes"),
        daemon_id,
        "the native prologue must identify the *calling* daemon, taken from the connection"
    );
    assert_eq!(
        info.proto, PROTO,
        "the prologue must name the negotiated protocol"
    );

    rec.metric("reply_len", reply.len());
    native.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Prologue shape conformance
// ============================================================================

/// The `StreamInfo` prologue must be framed identically by both
/// implementations.
///
/// `inference_mux.rs::read_p2pd_stream_info` consumes it *blind* — it reads the
/// varint length and discards the body without decoding — so a divergence in
/// framing would not fail loudly; it would desynchronise the mux frame loop and
/// surface much later as corrupt inference responses. This test therefore
/// compares the two prologues field by field on the parts that are
/// implementation-independent.
#[tokio::test]
async fn stream_info_prologue_matches_between_implementations() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::pipe_mode::stream_info_shape_parity",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let native = NativeNode::spawn().await;
    let (daemon_id, daemon_addr) = daemon_peer(&daemon);

    // (a) The prologue a Go daemon writes, triggered by a native caller.
    let (daemon_listener, daemon_handler_addr) = handler_listener().await;
    daemon
        .client
        .register_stream_handler(&daemon_handler_addr, vec![PROTO.to_string()])
        .await
        .expect("register on the daemon");
    native
        .handle
        .add_kad_address(daemon_id, daemon_addr)
        .await
        .expect("seed the routing table");

    let from_daemon = tokio::spawn(async move {
        let (mut socket, _) = tokio::time::timeout(CALL_TIMEOUT, daemon_listener.accept())
            .await
            .expect("a stream must arrive")
            .expect("accept");
        read_prologue(&mut socket).await
    });
    let client = native.client().await;
    let _to_daemon = tokio::time::timeout(
        CALL_TIMEOUT,
        client.stream_open_raw(&daemon_id.to_bytes(), vec![PROTO.to_string()]),
    )
    .await
    .expect("resolves")
    .expect("open to the daemon");
    let daemon_info = tokio::time::timeout(CALL_TIMEOUT, from_daemon)
        .await
        .expect("the daemon prologue must arrive")
        .expect("task");

    // (b) The prologue the native node writes, triggered by a daemon caller.
    let (native_listener, native_handler_addr) = handler_listener().await;
    let mut native_client = native.client().await;
    native_client
        .register_stream_handler(&native_handler_addr, vec![PROTO.to_string()])
        .await
        .expect("register on the native node");
    daemon
        .client
        .connect_peer(&native.addr)
        .await
        .expect("the daemon dials the native node");

    let from_native = tokio::spawn(async move {
        let (mut socket, _) = tokio::time::timeout(CALL_TIMEOUT, native_listener.accept())
            .await
            .expect("a stream must arrive")
            .expect("accept");
        read_prologue(&mut socket).await
    });
    let daemon_stream_client = P2PClient::connect(&daemon.socket_addr)
        .await
        .expect("a second daemon client");
    let _to_native = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon_stream_client.stream_open_raw(&native.peer_id.to_bytes(), vec![PROTO.to_string()]),
    )
    .await
    .expect("resolves")
    .expect("open to the native node");
    let native_info = tokio::time::timeout(CALL_TIMEOUT, from_native)
        .await
        .expect("the native prologue must arrive")
        .expect("task");

    // Both prologues decoded through the *same* parser the consumers use, so if
    // either framing were wrong the parse above would already have failed. What
    // remains to check is the content contract.
    assert_eq!(
        daemon_info.proto, native_info.proto,
        "both implementations report the negotiated protocol in `proto`"
    );
    assert_eq!(daemon_info.proto, PROTO);

    // `peer` is raw peer-id bytes in both, and identifies the caller.
    assert_eq!(
        PeerId::from_bytes(&daemon_info.peer).expect("daemon peer bytes"),
        native.peer_id,
        "the daemon names the native node as the caller"
    );
    assert_eq!(
        PeerId::from_bytes(&native_info.peer).expect("native peer bytes"),
        daemon_id,
        "the native node names the daemon as the caller"
    );

    // `addr` is a binary multiaddr where present. The Go daemon fills it with
    // the caller's remote address; the native side leaves it empty on the
    // inbound path, which is why nothing in this codebase reads it — but if it
    // *is* present it must parse.
    if !daemon_info.addr.is_empty() {
        Multiaddr::try_from(daemon_info.addr.clone())
            .expect("the daemon's StreamInfo.addr must be a binary multiaddr");
    }
    if !native_info.addr.is_empty() {
        Multiaddr::try_from(native_info.addr.clone())
            .expect("the native StreamInfo.addr must be a binary multiaddr");
    }

    native.shutdown().await;
    rec.finish(true);
}

// ============================================================================
// Volume across the boundary
// ============================================================================

/// 1 MiB in each direction across the implementation boundary.
///
/// The native side relays with `tokio::io::copy_bidirectional` and the Go side
/// with two `io.Copy` goroutines; both are supposed to apply backpressure by
/// awaiting writes, but they are different implementations with different buffer
/// sizes, and this is the only place the two meet under load. A regression that
/// truncated at a window boundary, or that deadlocked because one side buffers
/// where the other does not, fails here.
#[tokio::test]
async fn a_large_payload_survives_a_cross_implementation_relay() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::pipe_mode::large_payload_cross_impl",
        "integration",
    );

    const SIZE: usize = 1024 * 1024;

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let (listener, handler_addr) = handler_listener().await;
    daemon
        .client
        .register_stream_handler(&handler_addr, vec![PROTO.to_string()])
        .await
        .expect("register on the daemon");

    let native = NativeNode::spawn().await;
    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    native
        .handle
        .add_kad_address(daemon_id, daemon_addr)
        .await
        .expect("seed the routing table");

    // The handler reads and writes concurrently: doing either to completion
    // first would deadlock once both directions fill, which is the property
    // under test.
    let handler = tokio::spawn(async move {
        let (mut socket, _) = tokio::time::timeout(CALL_TIMEOUT, listener.accept())
            .await
            .expect("a forwarded stream must arrive")
            .expect("accept");
        read_prologue(&mut socket).await;

        let (mut reader, mut writer) = socket.into_split();
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

    let client = native.client().await;
    let stream = tokio::time::timeout(
        CALL_TIMEOUT,
        client.stream_open_raw(&daemon_id.to_bytes(), vec![PROTO.to_string()]),
    )
    .await
    .expect("resolves")
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
        // Small chunks, so the reader drains slower than the writer fills.
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
    let received_back = tokio::time::timeout(CALL_TIMEOUT, read_side)
        .await
        .expect("the reply must arrive in full")
        .expect("read task");
    let received_by_handler = tokio::time::timeout(CALL_TIMEOUT, handler)
        .await
        .expect("the handler must finish")
        .expect("handler task");

    assert_eq!(
        received_by_handler.len(),
        SIZE,
        "the Go-side handler must receive every byte the native side wrote"
    );
    assert_eq!(
        received_by_handler, sent,
        "native → daemon bytes must arrive verbatim and in order"
    );
    assert_eq!(
        received_back, expected_back,
        "daemon → native bytes must arrive verbatim and in order"
    );

    rec.metric("bytes_each_way", SIZE);
    native.shutdown().await;
    rec.finish(true);
}
