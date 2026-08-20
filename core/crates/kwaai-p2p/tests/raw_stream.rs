//! Raw libp2p streams through the full [`NetworkService`] stack.
//!
//! This is the substrate the control socket's pipe mode relays bytes over, so
//! what matters here is not "a stream opened" but the properties the relay loop
//! will depend on:
//!
//! - the negotiated protocol is reported back (it becomes `StreamInfo.proto`),
//! - **arbitrary slash-less names negotiate**, because hivemind's do and
//!   `libp2p_stream` cannot express them,
//! - bytes cross verbatim in both directions, including payloads far larger than
//!   any single buffer — a stream that silently truncates at a window boundary
//!   would break inference-mux long after the tests passed,
//! - a half-close is observable as EOF on the far end while the *other*
//!   direction still works, which is what `io::Copy`'s termination condition
//!   relies on,
//! - an unregistered protocol is refused cleanly rather than hanging, since the
//!   socket client blocks on the `STREAM_OPEN` reply,
//! - a handler removed mid-life stops negotiating.
//!
//! Both swarms are loopback-only with freshly generated keys, so nothing here
//! can collide with a node running on the same machine.

use std::time::Duration;

use futures::io::{AsyncReadExt, AsyncWriteExt};
use kwaai_p2p::{InboundStream, NetworkConfig, NetworkHandle, NetworkService, P2PError, PeerId};
use libp2p::identity::Keypair;

/// A per-interaction cap so a lost stream fails the test rather than hanging it.
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

/// A slash-less name, deliberately: this is the shape hivemind uses and the one
/// `StreamProtocol` refuses to construct.
const PROTO: &str = "kwaai.inference_mux";

/// Spawn a loopback service, keeping the task handle alive for the test.
fn spawn_service() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service should start");
    (handle, task, peer_id)
}

/// The first listen address of `handle`, with `/p2p/<peer-id>` appended.
async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> String {
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    loop {
        if let Some(addr) = handle
            .listen_addrs()
            .await
            .ok()
            .and_then(|a| a.into_iter().next())
        {
            return format!("{addr}/p2p/{peer_id}");
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for a listen address"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// Two connected services: `caller` has dialed `responder`.
async fn connected_pair() -> (
    NetworkHandle,
    NetworkHandle,
    PeerId,
    Vec<tokio::task::JoinHandle<()>>,
) {
    let (caller, caller_task, _) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();

    let addr = dialable_addr(&responder, responder_id).await;
    let connected = caller
        .connect_peer(&addr)
        .await
        .expect("loopback dial should succeed");
    assert_eq!(connected, responder_id);

    (
        caller,
        responder,
        responder_id,
        vec![caller_task, responder_task],
    )
}

/// Run `f`, failing the test rather than hanging if it stalls.
async fn within<T>(what: &str, f: impl std::future::Future<Output = T>) -> T {
    tokio::time::timeout(TEST_TIMEOUT, f)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for: {what}"))
}

// ============================================================================
// Negotiation
// ============================================================================

/// The base case, and the one that proves slash-less negotiation works for raw
/// streams as it already does for unary: open a stream, get the protocol back,
/// exchange a byte in each direction.
#[tokio::test]
async fn opens_a_raw_stream_on_a_slashless_protocol() {
    assert!(
        !PROTO.starts_with('/'),
        "this test is meaningless if the protocol is slashed"
    );

    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, refused) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("registering a stream handler must succeed");
    assert!(refused.is_empty(), "a fresh protocol must not be refused");

    // Echo one byte back, upper-cased, so both directions are exercised.
    tokio::spawn(async move {
        let InboundStream {
            mut stream, proto, ..
        } = inbound.recv().await.expect("an inbound stream arrives");
        assert_eq!(
            proto.as_ref(),
            PROTO,
            "the negotiated protocol must be reported to the accepting side"
        );
        let mut byte = [0u8; 1];
        stream
            .read_exact(&mut byte)
            .await
            .expect("read the request");
        stream
            .write_all(&[byte[0].to_ascii_uppercase()])
            .await
            .expect("write the reply");
        stream.close().await.expect("close");
    });

    let (proto, mut stream) = within(
        "open_raw_stream",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect("opening a served protocol must succeed");
    assert_eq!(
        proto, PROTO,
        "the caller must learn which protocol won negotiation"
    );

    stream.write_all(b"x").await.expect("write");
    stream.flush().await.expect("flush");
    let mut reply = [0u8; 1];
    within("reply", stream.read_exact(&mut reply))
        .await
        .expect("read");
    assert_eq!(&reply, b"X", "bytes must cross verbatim in both directions");
}

/// The protocol list is a *preference order* handed to multistream-select, as
/// Go's `doStreamOpen` passes it to `host.NewStream`. The remote picks the one
/// it serves, and that choice is what the caller is told.
#[tokio::test]
async fn negotiates_the_supported_protocol_from_a_list() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");

    tokio::spawn(async move {
        while let Some(InboundStream { mut stream, .. }) = inbound.recv().await {
            let _ = stream.close().await;
        }
    });

    let (proto, _stream) = within(
        "open_raw_stream with a candidate list",
        caller.open_raw_stream(
            responder_id,
            vec!["not.served".to_string(), PROTO.to_string()],
        ),
    )
    .await
    .expect("a list containing one served protocol must negotiate");
    assert_eq!(
        proto, PROTO,
        "the reported protocol must be the one actually negotiated, not the first requested"
    );
}

/// An unserved protocol must fail fast with a refusal, never hang. The socket
/// client on the other side of pipe mode is blocked on the `STREAM_OPEN` reply,
/// so a hang here is a hung external process.
#[tokio::test]
async fn an_unserved_protocol_is_refused_not_hung() {
    let (caller, _responder, responder_id, _tasks) = connected_pair().await;

    let error = within(
        "refusal",
        caller.open_raw_stream(responder_id, vec!["nobody.serves.this".to_string()]),
    )
    .await
    .expect_err("an unserved protocol must not yield a stream");

    assert!(
        matches!(error, P2PError::Protocol(_)),
        "a clean negotiation refusal is a protocol-level answer, got: {error:?}"
    );
    assert!(
        error.to_string().contains("nobody.serves.this"),
        "the error must name the protocol that was refused, got: {error}"
    );
}

/// A whole candidate list can be refused, and the refusal still arrives before
/// the caller is handed anything.
///
/// This is the path the negotiation sentinel exists for. Outbound negotiation
/// offers the real protocols and then one entry nobody serves, because
/// multistream-select only takes its lazy shortcut on the *last* protocol it
/// has to offer. Every real name here is therefore refused eagerly, one after
/// another, and only the sentinel is reached lazily — where the handler turns
/// it back into a refusal rather than surfacing it. A regression drops the
/// caller into a stream for a protocol the remote never agreed to, and the
/// error stops naming what was actually asked for.
#[tokio::test]
async fn a_candidate_list_that_is_entirely_unserved_is_refused() {
    let (caller, _responder, responder_id, _tasks) = connected_pair().await;

    let error = within(
        "refusal of a full list",
        caller.open_raw_stream(
            responder_id,
            vec![
                "nobody.serves.this".to_string(),
                "nor.this.one".to_string(),
                "nor.this.either".to_string(),
            ],
        ),
    )
    .await
    .expect_err("a list with nothing served must not yield a stream");

    assert!(
        matches!(error, P2PError::Protocol(_)),
        "a clean negotiation refusal is a protocol-level answer, got: {error:?}"
    );
    for name in ["nobody.serves.this", "nor.this.one", "nor.this.either"] {
        assert!(
            error.to_string().contains(name),
            "the error must name every protocol that was refused, missing {name}: {error}"
        );
    }
    assert!(
        !error.to_string().contains("__negotiation_probe__"),
        "the sentinel is an implementation detail and must never reach a caller: {error}"
    );
}

/// The negotiation sentinel is reserved and a caller cannot reach it.
///
/// Without this guard, a caller asking for `kwaai.__negotiation_probe__`
/// produced an offered list of `[sentinel, sentinel]`: the first entry
/// negotiates *eagerly and successfully* against a remote that registered the
/// name, the handler's guard then fires on the name, and a live stream is reset
/// under the remote while the caller is told it was unsupported — leaking the
/// sentinel that `a_candidate_list_that_is_entirely_unserved_is_refused`
/// asserts can never reach a caller. Found in review of #107.
#[tokio::test]
async fn the_negotiation_sentinel_is_reserved() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    // Even with the remote advertising it, the name must not be openable.
    let _ = responder
        .accept_streams(vec!["kwaai.__negotiation_probe__".to_string()])
        .await;

    let error = within(
        "reserved sentinel",
        caller.open_raw_stream(
            responder_id,
            vec!["kwaai.__negotiation_probe__".to_string()],
        ),
    )
    .await
    .expect_err("the sentinel must never be openable by a caller");

    assert!(
        error.to_string().contains("reserved"),
        "the caller should be told the name is reserved, got: {error}"
    );
}

/// Removing a handler stops negotiation, observable to the remote as the same
/// clean refusal an unregistered protocol produces.
#[tokio::test]
async fn removing_a_handler_stops_negotiation() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");
    tokio::spawn(async move {
        while let Some(InboundStream { mut stream, .. }) = inbound.recv().await {
            let _ = stream.close().await;
        }
    });

    within(
        "first open",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect("served while registered");

    let removed = responder
        .remove_stream_handler(vec![PROTO.to_string()])
        .await
        .expect("removal must succeed");
    assert_eq!(
        removed,
        vec![PROTO.to_string()],
        "removal must report the protocol it actually released"
    );

    let error = within(
        "refusal after removal",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect_err("a removed protocol must be refused");
    assert!(
        matches!(error, P2PError::Protocol(_)),
        "expected a negotiation refusal, got: {error:?}"
    );
}

/// A second registration of a protocol another handler already owns must be
/// refused rather than silently rebinding it — the first owner's accept loop
/// would otherwise stop receiving with nothing to indicate why.
#[tokio::test]
async fn a_second_handler_cannot_take_over_a_registered_protocol() {
    let (_caller, responder, _responder_id, _tasks) = connected_pair().await;

    let (_first, refused) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("first registration");
    assert!(refused.is_empty());

    let (_second, refused) = responder
        .accept_streams(vec![PROTO.to_string(), "other.proto".to_string()])
        .await
        .expect("the call itself succeeds");
    assert_eq!(
        refused,
        vec![PROTO.to_string()],
        "the taken protocol is refused and the rest of the list still registers"
    );
}

// ============================================================================
// Byte relay properties
// ============================================================================

/// **Backpressure and volume.** 4 MiB in each direction, well past any single
/// yamux window, with the reader deliberately consuming in small chunks.
///
/// A stream that buffered without bound would pass a small test and fail here or
/// in production; one that truncated at a window boundary would fail here too.
/// The relay loop in the control server copies with `copy_bidirectional`, whose
/// flow control *is* awaiting the write — so the property this asserts is
/// exactly the one that loop inherits.
#[tokio::test]
async fn relays_multi_megabyte_payloads_in_both_directions() {
    const SIZE: usize = 4 * 1024 * 1024;

    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");

    // Responder: read SIZE bytes, then write SIZE bytes of its own pattern.
    let echo = tokio::spawn(async move {
        let InboundStream { mut stream, .. } = inbound.recv().await.expect("inbound stream");

        let mut received = vec![0u8; SIZE];
        stream
            .read_exact(&mut received)
            .await
            .expect("read the full payload");

        let out: Vec<u8> = (0..SIZE).map(|i| (i % 251) as u8).collect();
        stream.write_all(&out).await.expect("write the full reply");
        stream.flush().await.expect("flush");
        stream.close().await.expect("close");
        received
    });

    let (_proto, stream) = within(
        "open",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect("open");

    let sent: Vec<u8> = (0..SIZE).map(|i| (i % 253) as u8).collect();
    let expected_back: Vec<u8> = (0..SIZE).map(|i| (i % 251) as u8).collect();

    // Write and read concurrently: writing 4 MiB before reading anything would
    // deadlock against the remote's window once both directions are full, which
    // is precisely the backpressure being asserted.
    let sent_for_writer = sent.clone();
    let (mut reader, mut writer) = stream.split();
    let write_side = async move {
        writer.write_all(&sent_for_writer).await.expect("write all");
        writer.flush().await.expect("flush");
        writer.close().await.expect("half-close after writing");
    };
    let read_side = async move {
        let mut got = Vec::with_capacity(SIZE);
        // Small chunks so the reader drains slower than the writer fills,
        // exercising the window rather than a single large copy.
        let mut buf = vec![0u8; 8 * 1024];
        loop {
            let n = reader.read(&mut buf).await.expect("read");
            if n == 0 {
                break;
            }
            got.extend_from_slice(&buf[..n]);
        }
        got
    };

    let (_, received_back) = within("bulk exchange", async move {
        futures::future::join(write_side, read_side).await
    })
    .await;

    let received_by_responder = within("responder finishes", echo)
        .await
        .expect("responder task must not panic");

    assert_eq!(
        received_by_responder.len(),
        SIZE,
        "the responder must receive every byte, not a window's worth"
    );
    assert_eq!(
        received_by_responder, sent,
        "caller → responder bytes must arrive verbatim and in order"
    );
    assert_eq!(
        received_back.len(),
        SIZE,
        "the caller must receive every byte back"
    );
    assert_eq!(
        received_back, expected_back,
        "responder → caller bytes must arrive verbatim and in order"
    );
}

/// **Half-close.** Closing the write half must surface as EOF on the far end
/// while the *other* direction still carries data.
///
/// This is the termination condition the pipe loop is built on: Go's
/// `doStreamPipe` ends each direction when its source reads EOF and only then
/// closes the matching destination. Without a real half-close, a client that
/// signals "request complete" by closing its write side would tear down the
/// reply it is waiting for.
#[tokio::test]
async fn a_half_close_is_eof_on_the_far_end_and_the_reverse_direction_survives() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");

    let responder_side = tokio::spawn(async move {
        let InboundStream { mut stream, .. } = inbound.recv().await.expect("inbound stream");

        // Read to EOF — which only arrives because the caller half-closed.
        let mut request = Vec::new();
        stream
            .read_to_end(&mut request)
            .await
            .expect("read to EOF after the caller's half-close");

        // The reverse direction must still be writable *after* observing EOF.
        stream
            .write_all(b"answered-after-eof")
            .await
            .expect("the write half survives the peer's half-close");
        stream.flush().await.expect("flush");
        stream.close().await.expect("close");
        request
    });

    let (_proto, mut stream) = within(
        "open",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect("open");

    stream.write_all(b"request-body").await.expect("write");
    stream.flush().await.expect("flush");
    // Half-close: FIN out, read half still live.
    stream.close().await.expect("half-close");

    let mut reply = Vec::new();
    within("reply after half-close", stream.read_to_end(&mut reply))
        .await
        .expect("read the reply");
    assert_eq!(
        reply, b"answered-after-eof",
        "the reply direction must survive our half-close"
    );

    let request = within("responder finishes", responder_side)
        .await
        .expect("responder task must not panic");
    assert_eq!(
        request, b"request-body",
        "the responder must see exactly the bytes written before the half-close"
    );
}

/// Several concurrent streams on one connection must stay independent — yamux
/// multiplexes them, and pipe mode runs one per socket client.
#[tokio::test]
async fn concurrent_streams_on_one_connection_do_not_cross_talk() {
    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");

    // Echo server: each stream gets its own task.
    tokio::spawn(async move {
        while let Some(InboundStream { mut stream, .. }) = inbound.recv().await {
            tokio::spawn(async move {
                let mut body = Vec::new();
                let _ = stream.read_to_end(&mut body).await;
                let _ = stream.write_all(&body).await;
                let _ = stream.flush().await;
                let _ = stream.close().await;
            });
        }
    });

    let mut opens = Vec::new();
    for i in 0..8u8 {
        let caller = caller.clone();
        opens.push(tokio::spawn(async move {
            let (_proto, mut stream) = caller
                .open_raw_stream(responder_id, vec![PROTO.to_string()])
                .await
                .expect("open");
            // A payload unique per stream, so a mix-up is detectable.
            let payload = vec![i; 1024];
            stream.write_all(&payload).await.expect("write");
            stream.flush().await.expect("flush");
            stream.close().await.expect("half-close");

            let mut echoed = Vec::new();
            stream.read_to_end(&mut echoed).await.expect("read");
            (i, payload, echoed)
        }));
    }

    for open in opens {
        let (i, payload, echoed) = within("concurrent stream", open)
            .await
            .expect("no task may panic");
        assert_eq!(
            echoed, payload,
            "stream {i} must receive its own bytes, not another stream's"
        );
    }
}

// ============================================================================
// Lifecycle
// ============================================================================

/// `open_raw_stream` dials on demand, matching Go's `host.NewStream`. A
/// `STREAM_OPEN` to a known-but-unconnected peer is the common case: the socket
/// client knows a peer ID from the DHT and has never dialed it.
#[tokio::test]
async fn opening_a_stream_dials_an_unconnected_peer() {
    let (caller, caller_task, _) = spawn_service();
    let (responder, responder_task, responder_id) = spawn_service();
    let _tasks = (caller_task, responder_task);

    let (mut inbound, _) = responder
        .accept_streams(vec![PROTO.to_string()])
        .await
        .expect("register");
    tokio::spawn(async move {
        while let Some(InboundStream { mut stream, .. }) = inbound.recv().await {
            let _ = stream.write_all(b"dialed").await;
            let _ = stream.flush().await;
            let _ = stream.close().await;
        }
    });

    // Seed only the routing table — never dial. This is what production does
    // after a DHT lookup.
    let addr = dialable_addr(&responder, responder_id).await;
    let addr: kwaai_p2p::Multiaddr = addr.parse().expect("valid multiaddr");
    caller
        .add_kad_address(responder_id, addr)
        .await
        .expect("seed the routing table");

    assert!(
        caller
            .list_peers()
            .await
            .expect("list_peers")
            .iter()
            .all(|p| p.peer_id != responder_id),
        "the test is meaningless if the peer is already connected"
    );

    let (_proto, mut stream) = within(
        "dial-on-demand open",
        caller.open_raw_stream(responder_id, vec![PROTO.to_string()]),
    )
    .await
    .expect("opening a stream must dial the peer on demand");

    let mut got = Vec::new();
    within("payload", stream.read_to_end(&mut got))
        .await
        .expect("read");
    assert_eq!(got, b"dialed");
}

/// A peer we cannot reach at all must fail as a dial failure, distinct from a
/// protocol refusal — the control server maps the two to different client-facing
/// errors.
#[tokio::test]
async fn an_unreachable_peer_fails_as_a_dial_failure() {
    let (caller, _caller_task, _) = spawn_service();
    let unknown = Keypair::generate_ed25519().public().to_peer_id();

    let error = within(
        "dial failure",
        caller.open_raw_stream(unknown, vec![PROTO.to_string()]),
    )
    .await
    .expect_err("a peer with no known address cannot be reached");

    assert!(
        matches!(error, P2PError::DialFailed(_)),
        "an unreachable peer must be a dial failure, not a protocol refusal, got: {error:?}"
    );
}

/// An empty protocol list is a client bug (`STREAM_OPEN` with no `proto`), and
/// must be refused rather than reaching `UnaryProtocol::new`, which panics on an
/// empty name.
#[tokio::test]
async fn an_empty_protocol_list_is_refused() {
    let (caller, _responder, responder_id, _tasks) = connected_pair().await;

    let error = within("empty list", caller.open_raw_stream(responder_id, vec![]))
        .await
        .expect_err("an empty protocol list cannot negotiate anything");
    assert!(
        matches!(error, P2PError::Protocol(_)),
        "expected a protocol-level refusal, got: {error:?}"
    );
}

/// Registering several protocols in one call routes all of them to the same
/// receiver — `node.rs` registers the three `DHTProtocol.rpc_*` names together
/// and expects one accept loop to serve them.
#[tokio::test]
async fn one_handler_can_serve_several_protocols() {
    const A: &str = "DHTProtocol.rpc_ping";
    const B: &str = "DHTProtocol.rpc_store";

    let (caller, responder, responder_id, _tasks) = connected_pair().await;

    let (mut inbound, refused) = responder
        .accept_streams(vec![A.to_string(), B.to_string()])
        .await
        .expect("register both");
    assert!(refused.is_empty());

    // Echo the negotiated protocol name back, so the caller can prove which
    // registration served it.
    tokio::spawn(async move {
        while let Some(InboundStream {
            mut stream, proto, ..
        }) = inbound.recv().await
        {
            tokio::spawn(async move {
                let _ = stream.write_all(proto.as_ref().as_bytes()).await;
                let _ = stream.flush().await;
                let _ = stream.close().await;
            });
        }
    });

    for expected in [A, B] {
        let (proto, mut stream) = within(
            "open",
            caller.open_raw_stream(responder_id, vec![expected.to_string()]),
        )
        .await
        .expect("both registered protocols must negotiate");
        assert_eq!(proto, expected);

        let mut got = Vec::new();
        within("payload", stream.read_to_end(&mut got))
            .await
            .expect("read");
        assert_eq!(
            String::from_utf8(got).expect("utf8"),
            expected,
            "the accepting side must see the protocol that was actually negotiated"
        );
    }
}
