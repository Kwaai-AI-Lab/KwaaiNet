//! Hivemind unary-RPC wire interop — Phase 0 gate for the native-p2p migration.
//!
//! These tests put a **real p2pd on the other end of the wire** and check the
//! codec in `kwaai-hivemind-dht::wire` against it in both directions. If the
//! wire assumptions in `docs/NATIVE_P2P_MIGRATION.md` are wrong, they fail here
//! rather than three phases later.
//!
//! The claim under test:
//!
//! ```text
//! caller ──▶ uvarint(len) ++ PersistentConnectionRequest{callId, callUnary{peer, proto, data}}
//! caller ◀── uvarint(len) ++ PersistentConnectionRequest{callId, unaryResponse{response|error}}
//! (stream closes)
//! ```
//!
//! on a libp2p stream whose protocol ID is the **bare handler name, no leading
//! slash** (`DHTProtocol.rpc_ping`).
//!
//! Topology — two independent p2pd instances, each with its own tmpdir socket,
//! its own throwaway identity (no `-id` flag) and its own ephemeral TCP port on
//! 127.0.0.1:
//!
//! ```text
//!   [daemon A]  ──dial──▶  [daemon B]
//!    caller                 responder
//! ```
//!
//! Coverage:
//!
//! | test | caller | responder | proves |
//! | --- | --- | --- | --- |
//! | `daemon_to_daemon_unary_round_trip` | p2pd A | p2pd B | the fixture + baseline |
//! | `raw_wire_responder_is_accepted_by_daemon_caller` | p2pd A | **our wire.rs** | the fix works on the real wire |
//! | `old_wrapper_shape_is_rejected_by_daemon_caller` | p2pd A | old encoding | the bug was real |
//! | `raw_wire_responder_error_arm` | p2pd A | **our wire.rs** | the error arm propagates |
//! | `raw_wire_caller_against_daemon_handler` | **our wire.rs** | p2pd B | our caller frame is accepted |
//! | `slashless_protocol_negotiates` | **our wire.rs** | p2pd B | multistream-select takes a slash-less ID |
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1` (spawns p2pd processes), matching the other
//! integration tiers in this crate.
//!
//! # Process hygiene
//!
//! Every daemon is spawned via `DaemonBuilder` and owned by a `TestNode`, whose
//! `P2PDaemon` kills **its own child by PID** on drop. Nothing here kills by
//! process name, and nothing touches the default socket path or `~/.kwaainet`.
//! [`WirePair`] additionally holds the nodes so they drop — and the daemons die
//! — even when a test panics mid-way.

use kwaai_hivemind_dht::wire;
use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

/// The protocol under test. **No leading slash** — this is the hivemind handler
/// name exactly as it appears as a libp2p protocol ID.
const PROTO: &str = "DHTProtocol.rpc_ping";

/// Cap on any single test's daemon interaction, so a wire regression surfaces as
/// a failure rather than a hung suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

// ============================================================================
// Fixture
// ============================================================================

/// Two connected p2pd instances: `a` calls, `b` responds.
///
/// Dropping this drops both [`TestNode`]s, each of which SIGTERMs/kills the
/// daemon child it spawned — by PID, never by name. Holding both in one struct
/// means a panic anywhere in a test still tears down both processes via unwind.
struct WirePair {
    a: TestNode,
    b: TestNode,
}

impl WirePair {
    async fn new() -> anyhow::Result<Self> {
        let (a, b) = tokio::try_join!(TestNode::new_wire_peer(), TestNode::new_wire_peer())?;
        let mut pair = Self { a, b };

        // A must know how to reach B before any stream can be opened. The Go
        // daemon's `host.NewStream` will not dial an unknown peer.
        let b_addr = pair
            .b
            .bootstrap_multiaddr()
            .ok_or_else(|| anyhow::anyhow!("peer B has no listen port"))?;
        pair.a.client.connect_peer(&b_addr).await?;

        Ok(pair)
    }
}

/// Bind an ephemeral localhost TCP listener for a p2pd stream handler to
/// forward inbound streams to. Returns the listener and its multiaddr.
async fn handler_listener() -> anyhow::Result<(TcpListener, String)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    Ok((listener, format!("/ip4/127.0.0.1/tcp/{port}")))
}

// ============================================================================
// (a) Baseline: daemon → daemon, both halves handled by p2pd
// ============================================================================

/// Register a unary handler on B via its own client, call it from A, and check
/// the payload survives. This validates the two-daemon fixture and gives the
/// later raw-wire tests a known-good reference.
#[tokio::test]
async fn daemon_to_daemon_unary_round_trip() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::wire::daemon_to_daemon", "integration");

    let pair = WirePair::new().await.expect("two-daemon fixture");

    pair.b
        .client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"echo:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register unary handler on B");

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        pair.a
            .client
            .call_unary_handler(&pair.b.peer_id_bytes(), PROTO, b"ping-payload"),
    )
    .await
    .expect("call did not time out")
    .expect("call_unary_handler should succeed");

    assert_eq!(response, b"echo:ping-payload");
    rec.metric("response_len", response.len());
    rec.finish(true);
}

// ============================================================================
// (b) THE KEY TEST: our raw-wire responder, a real p2pd caller
// ============================================================================

/// Serve one inbound hivemind unary call using **only** `wire.rs` for the
/// decode and the fixed `PersistentConnectionRequest{unaryResponse}` encode,
/// then have a real p2pd caller invoke it.
///
/// B registers a *stream* handler (not a unary handler), so p2pd forwards the
/// raw libp2p stream bytes to our TCP listener — exactly the shape `node.rs`
/// `handle_rpc_stream` sees in production. If the caller gets its payload back,
/// the wire format in `wire.rs` is correct against a real Go implementation.
#[tokio::test]
async fn raw_wire_responder_is_accepted_by_daemon_caller() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::wire::raw_responder", "integration");

    let mut pair = WirePair::new().await.expect("two-daemon fixture");
    let (listener, listen_addr) = handler_listener().await.expect("handler listener");

    pair.b
        .client
        .register_stream_handler(&listen_addr, vec![PROTO.to_string()])
        .await
        .expect("register stream handler on B");

    // Serve exactly one call, entirely through wire.rs.
    let served = tokio::spawn(async move {
        let (mut tcp, _) = listener.accept().await?;

        // p2pd prefixes the forwarded stream with a StreamInfo frame.
        let info = kwaai_p2p_daemon::stream::parse_stream_info(&mut tcp)
            .await
            .map_err(|e| anyhow::anyhow!("parse_stream_info: {e}"))?;
        anyhow::ensure!(
            info.proto == PROTO,
            "StreamInfo.proto should be the bare name, got {:?}",
            info.proto
        );

        // Read + decode the caller's frame with our codec.
        let frame = wire::read_framed(&mut tcp).await?;
        let (call_id, peer, proto, data) = wire::decode_unary_request(&frame)?;
        anyhow::ensure!(proto == PROTO, "proto mismatch: {proto}");

        // Reply with our codec.
        let mut payload = b"raw-wire:".to_vec();
        payload.extend_from_slice(&data);
        tcp.write_all(&wire::encode_unary_response(&call_id, Ok(payload)))
            .await?;
        tcp.flush().await?;

        Ok::<_, anyhow::Error>((call_id, peer, data))
    });

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        pair.a
            .client
            .call_unary_handler(&pair.b.peer_id_bytes(), PROTO, b"hello"),
    )
    .await
    .expect("call did not time out")
    .expect("a real p2pd caller must accept our wire.rs reply");

    assert_eq!(
        response, b"raw-wire:hello",
        "payload must survive the round trip unmodified"
    );

    let (call_id, peer, data) = served.await.expect("responder task").expect("responder");
    assert_eq!(data, b"hello", "request payload must arrive verbatim");
    assert_eq!(
        call_id.len(),
        16,
        "callId is a 16-byte UUID; Go drops the message if uuid.FromBytes fails"
    );
    // MEASURED, and contrary to what the migration doc implies: on a *stream*
    // handler the `peer` field is the CALLEE (B, ourselves) — the dial target
    // the calling daemon wrote — not the caller.
    //
    // `Daemon.persistentStreamHandler`'s `req.GetCallUnary().Peer =
    // s.Conn().RemotePeer()` rewrite only runs for handlers registered via
    // `add_unary_handler`. `register_stream_handler` puts p2pd in raw-pipe mode,
    // so nothing rewrites the field and it arrives exactly as the caller sent it.
    //
    // Consequence for Phase 2: a native responder must take the caller's
    // identity from the libp2p connection, NEVER from `callUnary.peer`. Trusting
    // this field would attribute every inbound RPC to ourselves.
    assert_eq!(
        peer,
        pair.b.peer_id_bytes(),
        "on a raw stream handler, callUnary.peer is the callee (unrewritten), \
         not the caller"
    );
    assert_ne!(
        peer,
        pair.a.peer_id_bytes(),
        "callUnary.peer is NOT the caller on this path — see comment above"
    );

    rec.metric("call_id_len", call_id.len());
    rec.finish(true);
}

/// Regression witness: serve the **old** wrapper shape
/// (`PersistentConnectionResponse{callId, callUnaryResponse}`, oneof arm at
/// field 2) and confirm a real p2pd caller cannot use it.
///
/// This is what shipped before the `stream.rs` fix, so every inbound
/// rpc_ping/store/find a Rust node served to a Go-daemon caller failed this way.
#[tokio::test]
async fn old_wrapper_shape_is_rejected_by_daemon_caller() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::wire::old_shape_rejected", "integration");

    let mut pair = WirePair::new().await.expect("two-daemon fixture");
    let (listener, listen_addr) = handler_listener().await.expect("handler listener");

    pair.b
        .client
        .register_stream_handler(&listen_addr, vec![PROTO.to_string()])
        .await
        .expect("register stream handler on B");

    tokio::spawn(async move {
        let (mut tcp, _) = listener.accept().await?;
        kwaai_p2p_daemon::stream::parse_stream_info(&mut tcp)
            .await
            .map_err(|e| anyhow::anyhow!("parse_stream_info: {e}"))?;
        let frame = wire::read_framed(&mut tcp).await?;
        let (call_id, _, _, _) = wire::decode_unary_request(&frame)?;

        // The pre-fix encoding, reconstructed byte-for-byte:
        // PersistentConnectionResponse{callId=1, callUnaryResponse=2}.
        tcp.write_all(&old_shape_response(&call_id, b"pong"))
            .await?;
        tcp.flush().await?;
        Ok::<_, anyhow::Error>(())
    });

    let result = tokio::time::timeout(
        CALL_TIMEOUT,
        pair.a
            .client
            .call_unary_handler(&pair.b.peer_id_bytes(), PROTO, b"hello"),
    )
    .await
    .expect("call did not time out");

    match result {
        Err(e) => {
            // Expected: the Go caller cannot unmarshal the reply.
            rec.metric("error_len", e.to_string().len());
            eprintln!("old wrapper shape rejected as expected: {e}");
        }
        Ok(payload) => {
            // Honest-reporting path: if the old shape somehow works, the bug
            // premise is wrong and that matters far more than a green test.
            panic!(
                "the OLD wrapper shape was ACCEPTED by a real p2pd caller, returning {:?} \
                 ({} bytes). The PersistentConnectionResponse-vs-Request premise in \
                 docs/NATIVE_P2P_MIGRATION.md is wrong — investigate before proceeding.",
                String::from_utf8_lossy(&payload),
                payload.len()
            );
        }
    }

    rec.finish(true);
}

/// Build the pre-fix reply: `PersistentConnectionResponse{callId, callUnaryResponse}`.
///
/// Hand-assembled rather than taken from the daemon crate's types, so this test
/// keeps witnessing the historical bytes even after `stream.rs` was corrected.
fn old_shape_response(call_id: &[u8], payload: &[u8]) -> Vec<u8> {
    // CallUnaryResponse{response = payload}: field 1, wire type 2.
    let mut inner = vec![0x0a, payload.len() as u8];
    inner.extend_from_slice(payload);

    let mut msg = Vec::new();
    // callId: field 1, wire type 2.
    msg.push(0x0a);
    msg.push(call_id.len() as u8);
    msg.extend_from_slice(call_id);
    // callUnaryResponse: field 2, wire type 2 → the bug. Field 2 of
    // PersistentConnectionRequest is AddUnaryHandlerRequest.
    msg.push(0x12);
    msg.push(inner.len() as u8);
    msg.extend_from_slice(&inner);

    let mut buf = unsigned_varint::encode::usize_buffer();
    let prefix = unsigned_varint::encode::usize(msg.len(), &mut buf);
    let mut framed = Vec::with_capacity(prefix.len() + msg.len());
    framed.extend_from_slice(prefix);
    framed.extend_from_slice(&msg);
    framed
}

/// The `error` arm of `CallUnaryResponse` must reach the caller as an error,
/// not as an empty success.
#[tokio::test]
async fn raw_wire_responder_error_arm() {
    require_integration!();
    let rec = MetricsRecorder::start("integration::wire::responder_error_arm", "integration");

    let mut pair = WirePair::new().await.expect("two-daemon fixture");
    let (listener, listen_addr) = handler_listener().await.expect("handler listener");

    pair.b
        .client
        .register_stream_handler(&listen_addr, vec![PROTO.to_string()])
        .await
        .expect("register stream handler on B");

    tokio::spawn(async move {
        let (mut tcp, _) = listener.accept().await?;
        kwaai_p2p_daemon::stream::parse_stream_info(&mut tcp)
            .await
            .map_err(|e| anyhow::anyhow!("parse_stream_info: {e}"))?;
        let frame = wire::read_framed(&mut tcp).await?;
        let (call_id, _, _, _) = wire::decode_unary_request(&frame)?;
        tcp.write_all(&wire::encode_unary_response(
            &call_id,
            Err("deliberate handler failure".to_string()),
        ))
        .await?;
        tcp.flush().await?;
        Ok::<_, anyhow::Error>(())
    });

    let result = tokio::time::timeout(
        CALL_TIMEOUT,
        pair.a
            .client
            .call_unary_handler(&pair.b.peer_id_bytes(), PROTO, b"hello"),
    )
    .await
    .expect("call did not time out");

    let err = result.expect_err("error arm must surface as an error");
    let msg = err.to_string();
    assert!(
        msg.contains("deliberate handler failure"),
        "the handler's message must reach the caller, got: {msg}"
    );

    rec.finish(true);
}

// ============================================================================
// (c) Raw-wire caller: our frames, a real p2pd responder
// ============================================================================

/// Drive the caller side entirely from `wire.rs`.
///
/// B registers a normal unary handler via `add_unary_handler` — so p2pd itself
/// is the responder. A opens a raw libp2p stream to it via `stream_open_raw`
/// (which puts the daemon socket into pipe mode, so we are writing directly onto
/// the libp2p stream), writes a frame built by `encode_unary_request`, and reads
/// the reply with `read_framed` + `decode_unary_response`.
///
/// A real p2pd accepting our caller frame proves `encode_unary_request` matches
/// what `Daemon.persistentStreamHandler` expects.
#[tokio::test]
async fn raw_wire_caller_against_daemon_handler() {
    require_integration!();
    let rec = MetricsRecorder::start("integration::wire::raw_caller", "integration");

    let pair = WirePair::new().await.expect("two-daemon fixture");

    pair.b
        .client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"daemon-served:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register unary handler on B");

    let call_id: Vec<u8> = (0u8..16).collect();
    let (call_id_out, result) = tokio::time::timeout(
        CALL_TIMEOUT,
        raw_unary_call(&pair, &call_id, PROTO, b"from-raw-caller"),
    )
    .await
    .expect("call did not time out")
    .expect("raw wire call");

    assert_eq!(call_id_out, call_id, "callId must be echoed verbatim");
    assert_eq!(
        result.expect("handler should succeed"),
        b"daemon-served:from-raw-caller"
    );

    rec.finish(true);
}

/// Empirical check that multistream-select accepts a protocol ID with **no
/// leading slash**, which is what hivemind uses and what rust-libp2p's
/// `StreamProtocol::new` refuses to construct (it panics on slash-less names).
///
/// This is an open verification item in `docs/NATIVE_P2P_MIGRATION.md`: Phase 2
/// plans a custom `UnaryProtocol(Arc<str>)` to sidestep that constructor, and it
/// is only worth building if the negotiation itself is legal. A successful call
/// here means the go-libp2p peer on the far end negotiated `DHTProtocol.rpc_ping`
/// verbatim.
#[tokio::test]
async fn slashless_protocol_negotiates() {
    require_integration!();
    let rec = MetricsRecorder::start("integration::wire::slashless_protocol", "integration");

    assert!(
        !PROTO.starts_with('/'),
        "this test is meaningless if the protocol ID is slashed"
    );

    let pair = WirePair::new().await.expect("two-daemon fixture");

    pair.b
        .client
        .add_unary_handler(
            PROTO,
            |_req: Vec<u8>| async move { Ok(b"ok".to_vec()) },
            false,
        )
        .await
        .expect("register unary handler on B");

    let call_id: Vec<u8> = (100u8..116).collect();
    let (_, result) =
        tokio::time::timeout(CALL_TIMEOUT, raw_unary_call(&pair, &call_id, PROTO, b""))
            .await
            .expect("call did not time out")
            .expect("slash-less protocol must negotiate on the libp2p wire");

    assert_eq!(result.expect("handler should succeed"), b"ok");
    rec.finish(true);
}

/// One full caller-side exchange over a raw libp2p stream, using only `wire.rs`
/// for encode/decode.
#[allow(clippy::type_complexity)]
async fn raw_unary_call(
    pair: &WirePair,
    call_id: &[u8],
    proto: &str,
    data: &[u8],
) -> anyhow::Result<(Vec<u8>, Result<Vec<u8>, String>)> {
    // A fresh control-socket client, because stream_open_raw consumes it: the
    // daemon socket becomes the raw libp2p stream.
    let client = kwaai_p2p_daemon::P2PClient::connect(&pair.a.socket_addr).await?;
    let mut stream = client
        .stream_open_raw(&pair.b.peer_id_bytes(), vec![proto.to_string()])
        .await?;

    // Go's caller (`Daemon.exchangeMessages`) sets `peer` to the dial target;
    // mirror that. The field is proto2 `required`, so it must be present either
    // way — an omitted field makes the responding daemon reset the stream.
    stream
        .write_all(&wire::encode_unary_request(
            call_id,
            &pair.b.peer_id_bytes(),
            proto,
            data,
        ))
        .await?;
    stream.flush().await?;

    let frame = wire::read_framed(&mut stream).await?;
    Ok(wire::decode_unary_response(&frame)?)
}

// ============================================================================
// Framing conformance against the real daemon
// ============================================================================

/// The daemon must accept a frame whose uvarint length prefix is multi-byte.
///
/// Anything over 127 bytes uses a 2-byte prefix, which is where a hand-rolled
/// single-byte framer would break. DHT store payloads routinely exceed this, so
/// the boundary is exercised in production traffic constantly.
#[tokio::test]
async fn multibyte_length_prefix_accepted_by_daemon() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::wire::multibyte_prefix", "integration");

    let pair = WirePair::new().await.expect("two-daemon fixture");

    pair.b
        .client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move { Ok(req.len().to_string().into_bytes()) },
            false,
        )
        .await
        .expect("register unary handler on B");

    // 4 KiB payload → 2-byte uvarint on the inner data field and a 2-byte frame
    // prefix on the envelope.
    let data = vec![0x5Au8; 4096];
    let call_id: Vec<u8> = (200u8..216).collect();

    let framed = wire::encode_unary_request(&call_id, b"", PROTO, &data);
    assert!(
        framed.len() - 4096 > 3,
        "envelope should need a multi-byte prefix"
    );

    let (_, result) =
        tokio::time::timeout(CALL_TIMEOUT, raw_unary_call(&pair, &call_id, PROTO, &data))
            .await
            .expect("call did not time out")
            .expect("multi-byte-prefixed frame must be accepted");

    assert_eq!(
        result.expect("handler should succeed"),
        b"4096",
        "the handler must see all 4096 bytes"
    );

    rec.metric("payload_bytes", data.len());
    rec.finish(true);
}
