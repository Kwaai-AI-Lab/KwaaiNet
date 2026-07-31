//! Hivemind unary-RPC wire codec (peer-to-peer, on the libp2p stream).
//!
//! Hivemind's "unary handler" abstraction is not a daemon-local convention: the
//! wrapper travels **on the libp2p wire between peers**. A caller opens a stream
//! whose protocol ID is the bare handler name — *no leading slash*, e.g.
//! `DHTProtocol.rpc_store` — and then:
//!
//! ```text
//! caller  ──▶  uvarint(len) ++ PersistentConnectionRequest{callId, callUnary{peer, proto, data}}
//! caller  ◀──  uvarint(len) ++ PersistentConnectionRequest{callId, unaryResponse{response|error}}
//! (stream closes)
//! ```
//!
//! `data` is the raw application protobuf (`StoreRequest`, `FindRequest`,
//! `PingRequest` from [`crate::protocol`]) — there is no extra framing around it.
//!
//! **The reply is a `PersistentConnectionRequest`, not a
//! `PersistentConnectionResponse`.** `PersistentConnectionResponse` only exists
//! on the daemon's local control socket. This is verified against
//! go-libp2p-daemon `persistent_stream.go`: `Daemon.exchangeMessages` reads the
//! reply into `&pb.PersistentConnectionRequest{}` and calls `GetUnaryResponse()`
//! (field 4). Writing a `PersistentConnectionResponse` instead makes Go callers
//! fail to unmarshal — its field 2 is `CallUnaryResponse`, whereas field 2 of
//! `PersistentConnectionRequest` is `AddUnaryHandlerRequest`, a proto2 message
//! with `required` fields that are then missing.
//!
//! Two further details copied from the Go responder
//! (`Daemon.persistentStreamHandler`):
//!
//! - It overwrites `callUnary.peer` with the *caller's* peer ID before
//!   dispatching. Whatever a caller puts there is ignored (hivemind's Python
//!   client sets it; the Go daemon sets it to `[]byte{}`). We match the Go
//!   caller and send an empty `peer`.
//! - Inbound frames are capped by `persistentConnMsgMaxSize`; we mirror the
//!   10 MiB cap in [`MAX_FRAME_LEN`].
//!
//! The schema of record is `proto/persistent_conn.proto`. As elsewhere in this
//! crate (see [`crate::protocol`] vs `proto/dht.proto`) the Rust types are
//! hand-written `prost` derives rather than generated — the crate has no
//! build.rs and pulls no `prost-build`.
//!
//! This replaces the `[8-byte BE len][1-byte marker][protobuf]` framing in
//! [`crate::codec`], which is a kwaai invention and matches nothing on the
//! hivemind wire.

use prost::Message;
use std::io;
use tokio::io::{AsyncRead, AsyncReadExt};

/// Maximum accepted frame payload, matching the Go daemon's
/// `persistentConnMsgMaxSize` (10 MiB). Frames declaring more are rejected
/// without allocating.
pub const MAX_FRAME_LEN: usize = 10 * 1024 * 1024;

/// Longest uvarint that can encode a `u64` (10 groups of 7 bits).
const MAX_VARINT_LEN: usize = 10;

// ============================================================================
// Wire messages (subset of p2pd.pb, see proto/persistent_conn.proto)
// ============================================================================

/// `p2pd.pb.CallUnaryRequest` — the outbound half of a unary call.
#[derive(Clone, PartialEq, Message)]
pub struct CallUnaryRequest {
    /// Callee peer ID on the way out; the responder overwrites it with the
    /// caller's peer ID before dispatch, so the received value is the *caller*.
    #[prost(bytes = "vec", tag = "1")]
    pub peer: Vec<u8>,
    /// Bare handler name, no leading slash (e.g. `DHTProtocol.rpc_store`).
    #[prost(string, tag = "2")]
    pub proto: String,
    /// Raw application protobuf.
    #[prost(bytes = "vec", tag = "3")]
    pub data: Vec<u8>,
}

/// `p2pd.pb.CallUnaryResponse` — the inbound half, a `response`/`error` oneof.
#[derive(Clone, PartialEq, Message)]
pub struct CallUnaryResponse {
    #[prost(oneof = "call_unary_response::Result", tags = "1, 2")]
    pub result: Option<call_unary_response::Result>,
}

pub mod call_unary_response {
    /// `oneof result` of [`super::CallUnaryResponse`].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Successful payload: raw application protobuf.
        #[prost(bytes, tag = "1")]
        Response(Vec<u8>),
        /// Handler error. Go encodes `err.Error()` as UTF-8 bytes.
        #[prost(bytes, tag = "2")]
        Error(Vec<u8>),
    }
}

/// `p2pd.pb.AddUnaryHandlerRequest`.
///
/// Present so that the oneof tag numbering matches the daemon's exactly (field
/// 2). This arm never appears on the libp2p wire — it is a control-socket-only
/// message — but its presence at tag 2 is precisely what makes the historical
/// `PersistentConnectionResponse` reply undecodable to Go callers.
#[derive(Clone, PartialEq, Message)]
pub struct AddUnaryHandlerRequest {
    #[prost(string, tag = "1")]
    pub proto: String,
    #[prost(bool, tag = "2")]
    pub balanced: bool,
}

/// `p2pd.pb.RemoveUnaryHandlerRequest`. Control-socket only; see above.
#[derive(Clone, PartialEq, Message)]
pub struct RemoveUnaryHandlerRequest {
    #[prost(string, tag = "1")]
    pub proto: String,
}

/// `p2pd.pb.Cancel` — empty marker message.
#[derive(Clone, PartialEq, Message)]
pub struct Cancel {}

/// `p2pd.pb.DaemonError`.
#[derive(Clone, PartialEq, Message)]
pub struct DaemonError {
    #[prost(string, optional, tag = "1")]
    pub message: Option<String>,
}

/// `p2pd.pb.PersistentConnectionRequest` — the envelope in **both** directions
/// on the libp2p wire.
#[derive(Clone, PartialEq, Message)]
pub struct PersistentConnectionRequest {
    /// 16 raw UUID bytes in practice; Go does `uuid.FromBytes` and drops the
    /// message when that fails.
    #[prost(bytes = "vec", tag = "1")]
    pub call_id: Vec<u8>,
    #[prost(
        oneof = "persistent_connection_request::Message",
        tags = "2, 3, 4, 5, 6"
    )]
    pub message: Option<persistent_connection_request::Message>,
}

pub mod persistent_connection_request {
    /// `oneof message` of [`super::PersistentConnectionRequest`].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        AddUnaryHandler(super::AddUnaryHandlerRequest),
        #[prost(message, tag = "3")]
        CallUnary(super::CallUnaryRequest),
        #[prost(message, tag = "4")]
        UnaryResponse(super::CallUnaryResponse),
        #[prost(message, tag = "5")]
        Cancel(super::Cancel),
        #[prost(message, tag = "6")]
        RemoveUnaryHandler(super::RemoveUnaryHandlerRequest),
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Failure decoding or reading a hivemind unary frame.
#[derive(Debug, thiserror::Error)]
pub enum WireError {
    /// The uvarint length prefix was malformed or longer than 10 bytes.
    #[error("malformed uvarint length prefix")]
    BadVarint,
    /// The frame declared a length above [`MAX_FRAME_LEN`].
    #[error("frame too large: {len} bytes (max {MAX_FRAME_LEN})")]
    FrameTooLarge {
        /// Declared payload length.
        len: usize,
    },
    /// The payload was not a decodable `PersistentConnectionRequest`.
    #[error("protobuf decode failed: {0}")]
    Decode(#[from] prost::DecodeError),
    /// A well-formed envelope carrying the wrong oneof arm (or none).
    #[error("unexpected message: {0}")]
    UnexpectedMessage(String),
    /// Underlying I/O failure while reading a frame.
    #[error("io: {0}")]
    Io(#[from] io::Error),
}

/// Result alias for wire codec operations.
pub type WireResult<T> = std::result::Result<T, WireError>;

// ============================================================================
// Framing
// ============================================================================

/// Prefix `payload` with its uvarint length.
pub fn frame(payload: &[u8]) -> Vec<u8> {
    let mut buf = unsigned_varint::encode::usize_buffer();
    let prefix = unsigned_varint::encode::usize(payload.len(), &mut buf);
    let mut out = Vec::with_capacity(prefix.len() + payload.len());
    out.extend_from_slice(prefix);
    out.extend_from_slice(payload);
    out
}

/// Split one uvarint-framed message off the front of `bytes`.
///
/// Returns the payload and the total number of bytes consumed (prefix +
/// payload), so callers can advance through a buffer holding several frames.
pub fn unframe(bytes: &[u8]) -> WireResult<(&[u8], usize)> {
    let (len, rest) = unsigned_varint::decode::usize(bytes).map_err(|_| WireError::BadVarint)?;
    if len > MAX_FRAME_LEN {
        return Err(WireError::FrameTooLarge { len });
    }
    let prefix_len = bytes.len() - rest.len();
    if rest.len() < len {
        return Err(WireError::Io(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("frame declares {} bytes, only {} available", len, rest.len()),
        )));
    }
    Ok((&rest[..len], prefix_len + len))
}

/// Read exactly one uvarint-framed message from an async reader.
///
/// Reads the length prefix a byte at a time (it is 1–10 bytes and we must not
/// over-consume into the payload), rejects lengths above [`MAX_FRAME_LEN`]
/// *before* allocating, then `read_exact`s the payload — so partial reads of a
/// large frame are handled by tokio rather than by us.
pub async fn read_framed<R>(reader: &mut R) -> WireResult<Vec<u8>>
where
    R: AsyncRead + Unpin,
{
    let mut varint_buf = [0u8; MAX_VARINT_LEN];
    let mut n = 0usize;

    loop {
        if n == MAX_VARINT_LEN {
            return Err(WireError::BadVarint);
        }
        reader.read_exact(&mut varint_buf[n..n + 1]).await?;
        let is_last = varint_buf[n] & 0x80 == 0;
        n += 1;
        if is_last {
            break;
        }
    }

    let (len, _) =
        unsigned_varint::decode::usize(&varint_buf[..n]).map_err(|_| WireError::BadVarint)?;
    if len > MAX_FRAME_LEN {
        return Err(WireError::FrameTooLarge { len });
    }

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload).await?;
    Ok(payload)
}

// ============================================================================
// Caller side
// ============================================================================

/// Build the caller's frame: `uvarint(len) ++ PersistentConnectionRequest{callId, callUnary}`.
///
/// `peer` is what Go puts in `CallUnaryRequest.peer` on the way out; the
/// responder overwrites it, so an empty slice is fine and is what a Go-daemon
/// caller effectively transmits after its own local dispatch.
pub fn encode_unary_request(call_id: &[u8], peer: &[u8], proto: &str, data: &[u8]) -> Vec<u8> {
    let req = PersistentConnectionRequest {
        call_id: call_id.to_vec(),
        message: Some(persistent_connection_request::Message::CallUnary(
            CallUnaryRequest {
                peer: peer.to_vec(),
                proto: proto.to_string(),
                data: data.to_vec(),
            },
        )),
    };
    frame(&req.encode_to_vec())
}

/// Decode an *unframed* caller payload into `(call_id, peer, proto, data)`.
///
/// Use [`read_framed`] or [`unframe`] first. Errors when the envelope carries
/// any oneof arm other than `callUnary`.
pub fn decode_unary_request(payload: &[u8]) -> WireResult<(Vec<u8>, Vec<u8>, String, Vec<u8>)> {
    let req = PersistentConnectionRequest::decode(payload)?;
    match req.message {
        Some(persistent_connection_request::Message::CallUnary(cu)) => {
            Ok((req.call_id, cu.peer, cu.proto, cu.data))
        }
        other => Err(WireError::UnexpectedMessage(format!(
            "expected callUnary, got {}",
            describe_arm(other.as_ref())
        ))),
    }
}

// ============================================================================
// Responder side
// ============================================================================

/// Build the responder's frame:
/// `uvarint(len) ++ PersistentConnectionRequest{callId, unaryResponse}`.
///
/// `Ok(data)` becomes `CallUnaryResponse.response`; `Err(msg)` becomes
/// `CallUnaryResponse.error` with the message as UTF-8 bytes (Go writes
/// `err.Error()` the same way).
pub fn encode_unary_response(
    call_id: &[u8],
    result: std::result::Result<Vec<u8>, String>,
) -> Vec<u8> {
    let inner = match result {
        Ok(data) => call_unary_response::Result::Response(data),
        Err(msg) => call_unary_response::Result::Error(msg.into_bytes()),
    };
    let resp = PersistentConnectionRequest {
        call_id: call_id.to_vec(),
        message: Some(persistent_connection_request::Message::UnaryResponse(
            CallUnaryResponse {
                result: Some(inner),
            },
        )),
    };
    frame(&resp.encode_to_vec())
}

/// Decode an *unframed* responder payload into `(call_id, Ok(data) | Err(msg))`.
///
/// A `CallUnaryResponse` with neither arm set (an empty message) decodes to
/// `Err` describing the empty response rather than silently yielding no bytes.
#[allow(clippy::type_complexity)]
pub fn decode_unary_response(
    payload: &[u8],
) -> WireResult<(Vec<u8>, std::result::Result<Vec<u8>, String>)> {
    let req = PersistentConnectionRequest::decode(payload)?;
    match req.message {
        Some(persistent_connection_request::Message::UnaryResponse(r)) => {
            let out = match r.result {
                Some(call_unary_response::Result::Response(data)) => Ok(data),
                Some(call_unary_response::Result::Error(err)) => {
                    Err(String::from_utf8_lossy(&err).into_owned())
                }
                None => Err("empty CallUnaryResponse (neither response nor error set)".to_string()),
            };
            Ok((req.call_id, out))
        }
        other => Err(WireError::UnexpectedMessage(format!(
            "expected unaryResponse, got {}",
            describe_arm(other.as_ref())
        ))),
    }
}

fn describe_arm(arm: Option<&persistent_connection_request::Message>) -> &'static str {
    use persistent_connection_request::Message as M;
    match arm {
        None => "no message",
        Some(M::AddUnaryHandler(_)) => "addUnaryHandler",
        Some(M::CallUnary(_)) => "callUnary",
        Some(M::UnaryResponse(_)) => "unaryResponse",
        Some(M::Cancel(_)) => "cancel",
        Some(M::RemoveUnaryHandler(_)) => "removeUnaryHandler",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CALL_ID: [u8; 16] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00,
    ];

    // ---------------------------------------------------------------- golden

    /// Byte-for-byte layout of a request frame, checked against the proto2
    /// encoding rules by hand rather than against our own encoder.
    #[test]
    fn golden_request_bytes() {
        let framed = encode_unary_request(&CALL_ID, b"", "DHTProtocol.rpc_ping", b"\x08\x2a");

        let mut expect_inner = Vec::new();
        // field 1 (callId), wire type 2 → tag byte 0x0a, len 16
        expect_inner.push(0x0a);
        expect_inner.push(16);
        expect_inner.extend_from_slice(&CALL_ID);
        // field 3 (callUnary), wire type 2 → tag byte 0x1a
        let mut cu = Vec::new();
        // CallUnaryRequest field 1 (peer) is empty → prost omits it entirely.
        // field 2 (proto), wire type 2 → 0x12
        cu.push(0x12);
        cu.push(20); // len("DHTProtocol.rpc_ping")
        cu.extend_from_slice(b"DHTProtocol.rpc_ping");
        // field 3 (data), wire type 2 → 0x1a
        cu.push(0x1a);
        cu.push(2);
        cu.extend_from_slice(b"\x08\x2a");
        expect_inner.push(0x1a);
        expect_inner.push(cu.len() as u8);
        expect_inner.extend_from_slice(&cu);

        let mut expect = vec![expect_inner.len() as u8];
        expect.extend_from_slice(&expect_inner);

        assert_eq!(
            framed,
            expect,
            "request frame layout drifted\n got: {:02x?}\nwant: {:02x?}",
            framed,
            expect
        );
    }

    /// The reply envelope MUST be a `PersistentConnectionRequest` with the
    /// oneof arm at field **4** (`unaryResponse`) — not field 2.
    #[test]
    fn golden_response_bytes_use_field_4() {
        let framed = encode_unary_response(&CALL_ID, Ok(b"pong".to_vec()));
        let (payload, consumed) = unframe(&framed).unwrap();
        assert_eq!(consumed, framed.len());

        // callId (tag 0x0a) then unaryResponse (field 4, wire type 2 → 0x22).
        assert_eq!(payload[0], 0x0a);
        assert_eq!(payload[1], 16);
        assert_eq!(&payload[2..18], &CALL_ID);
        assert_eq!(
            payload[18], 0x22,
            "unaryResponse must be field 4; 0x12 (field 2) is the historical bug"
        );
        // CallUnaryResponse{response: "pong"} → 0x0a 0x04 "pong"
        assert_eq!(&payload[19..], &[0x06, 0x0a, 0x04, b'p', b'o', b'n', b'g']);
    }

    // ------------------------------------------------------------ round trip

    #[test]
    fn request_round_trip() {
        let data = b"\x0a\x14some-dht-payload----";
        let framed = encode_unary_request(&CALL_ID, b"\x00\x24peer", "DHTProtocol.rpc_store", data);
        let (payload, consumed) = unframe(&framed).unwrap();
        assert_eq!(consumed, framed.len());

        let (call_id, peer, proto, out) = decode_unary_request(payload).unwrap();
        assert_eq!(call_id, CALL_ID);
        assert_eq!(peer, b"\x00\x24peer");
        assert_eq!(proto, "DHTProtocol.rpc_store");
        assert_eq!(out, data);
    }

    #[test]
    fn response_ok_round_trip() {
        let framed = encode_unary_response(&CALL_ID, Ok(b"response-bytes".to_vec()));
        let (payload, _) = unframe(&framed).unwrap();
        let (call_id, result) = decode_unary_response(payload).unwrap();
        assert_eq!(call_id, CALL_ID);
        assert_eq!(result.unwrap(), b"response-bytes");
    }

    #[test]
    fn response_error_round_trip() {
        let framed = encode_unary_response(&CALL_ID, Err("handler exploded".to_string()));
        let (payload, _) = unframe(&framed).unwrap();
        let (call_id, result) = decode_unary_response(payload).unwrap();
        assert_eq!(call_id, CALL_ID);
        assert_eq!(result.unwrap_err(), "handler exploded");
    }

    /// An empty `data` must survive: proto3-style omission of an empty bytes
    /// field is fine because the responder only ever reads `data` as "the
    /// payload", and absent == empty.
    #[test]
    fn empty_payload_round_trip() {
        let framed = encode_unary_request(&CALL_ID, b"", "DHTProtocol.rpc_ping", b"");
        let (payload, _) = unframe(&framed).unwrap();
        let (_, _, proto, data) = decode_unary_request(payload).unwrap();
        assert_eq!(proto, "DHTProtocol.rpc_ping");
        assert!(data.is_empty());
    }

    // -------------------------------------------------------- varint framing

    /// Payload lengths straddling every uvarint width boundary.
    #[test]
    fn varint_length_edges() {
        for &len in &[0usize, 1, 127, 128, 16_383, 16_384, 2_097_151, 2_097_152] {
            let payload = vec![0xABu8; len];
            let framed = frame(&payload);

            let expected_prefix = match len {
                0..=127 => 1,
                128..=16_383 => 2,
                16_384..=2_097_151 => 3,
                _ => 4,
            };
            assert_eq!(
                framed.len() - len,
                expected_prefix,
                "len {} should use a {}-byte uvarint prefix",
                len,
                expected_prefix
            );

            let (out, consumed) = unframe(&framed).unwrap();
            assert_eq!(out, &payload[..], "unframe mismatch at len {}", len);
            assert_eq!(consumed, framed.len());
        }
    }

    /// `read_framed` must reassemble a multi-byte-varint frame delivered in
    /// dribbles — the partial-read path.
    #[tokio::test]
    async fn read_framed_handles_multibyte_varint_and_partial_reads() {
        for &len in &[0usize, 127, 128, 16_383, 16_384] {
            let payload: Vec<u8> = (0..len).map(|i| (i % 251) as u8).collect();
            let framed = frame(&payload);

            // duplex with a tiny buffer forces the writer to trickle.
            let (client, mut server) = tokio::io::duplex(8);
            let writer = tokio::spawn(async move {
                use tokio::io::AsyncWriteExt as _;
                for chunk in framed.chunks(3) {
                    server.write_all(chunk).await.unwrap();
                    tokio::task::yield_now().await;
                }
                server.flush().await.unwrap();
            });

            let mut client = client;
            let got = read_framed(&mut client).await.unwrap();
            writer.await.unwrap();
            assert_eq!(got, payload, "read_framed mismatch at len {}", len);
        }
    }

    /// Two frames back to back on one stream are read independently — the
    /// varint reader must not over-consume into the second frame.
    #[tokio::test]
    async fn read_framed_reads_consecutive_frames() {
        let a = encode_unary_request(&CALL_ID, b"", "DHTProtocol.rpc_ping", b"first");
        let b = encode_unary_response(&CALL_ID, Ok(b"second".to_vec()));

        let (mut client, mut server) = tokio::io::duplex(4096);
        {
            use tokio::io::AsyncWriteExt as _;
            server.write_all(&a).await.unwrap();
            server.write_all(&b).await.unwrap();
            server.flush().await.unwrap();
        }

        let f1 = read_framed(&mut client).await.unwrap();
        let f2 = read_framed(&mut client).await.unwrap();
        assert_eq!(decode_unary_request(&f1).unwrap().3, b"first");
        assert_eq!(decode_unary_response(&f2).unwrap().1.unwrap(), b"second");
    }

    // ------------------------------------------------------------ rejections

    #[test]
    fn unframe_rejects_oversized_declaration() {
        // Declare MAX_FRAME_LEN + 1 with no payload behind it: must be rejected
        // on the declared length alone, never allocated.
        let mut buf = unsigned_varint::encode::usize_buffer();
        let prefix = unsigned_varint::encode::usize(MAX_FRAME_LEN + 1, &mut buf).to_vec();
        match unframe(&prefix) {
            Err(WireError::FrameTooLarge { len }) => assert_eq!(len, MAX_FRAME_LEN + 1),
            other => panic!("expected FrameTooLarge, got {:?}", other.map(|(p, _)| p.len())),
        }
    }

    #[tokio::test]
    async fn read_framed_rejects_oversized_declaration() {
        let mut buf = unsigned_varint::encode::usize_buffer();
        let prefix = unsigned_varint::encode::usize(MAX_FRAME_LEN + 1, &mut buf).to_vec();

        let (mut client, mut server) = tokio::io::duplex(64);
        {
            use tokio::io::AsyncWriteExt as _;
            server.write_all(&prefix).await.unwrap();
            server.flush().await.unwrap();
        }
        match read_framed(&mut client).await {
            Err(WireError::FrameTooLarge { len }) => assert_eq!(len, MAX_FRAME_LEN + 1),
            other => panic!("expected FrameTooLarge, got {:?}", other.map(|v| v.len())),
        }
    }

    /// Exactly at the cap is legal (the cap is inclusive), so the guard must
    /// not be off by one. Checked on the declaration only — no 10 MiB alloc.
    #[test]
    fn unframe_accepts_exactly_max_len_declaration() {
        let mut buf = unsigned_varint::encode::usize_buffer();
        let prefix = unsigned_varint::encode::usize(MAX_FRAME_LEN, &mut buf).to_vec();
        // Payload is absent → UnexpectedEof, NOT FrameTooLarge.
        match unframe(&prefix) {
            Err(WireError::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof),
            other => panic!("expected UnexpectedEof, got {:?}", other.map(|(p, _)| p.len())),
        }
    }

    #[tokio::test]
    async fn read_framed_rejects_runaway_varint() {
        // 11 continuation bytes: longer than any valid u64 uvarint.
        let (mut client, mut server) = tokio::io::duplex(64);
        {
            use tokio::io::AsyncWriteExt as _;
            server.write_all(&[0xFFu8; 11]).await.unwrap();
            server.flush().await.unwrap();
        }
        assert!(matches!(
            read_framed(&mut client).await,
            Err(WireError::BadVarint)
        ));
    }

    #[test]
    fn unframe_rejects_truncated_payload() {
        let framed = encode_unary_response(&CALL_ID, Ok(b"abcdefgh".to_vec()));
        let truncated = &framed[..framed.len() - 3];
        assert!(matches!(unframe(truncated), Err(WireError::Io(_))));
    }

    // ------------------------------------------------- wrong-envelope decode

    /// The live-bug shape: a reply whose oneof arm sits at field 2. Decoded as
    /// a `PersistentConnectionRequest` (which is what Go does) field 2 is
    /// `AddUnaryHandlerRequest`, so this must NOT surface as a unary response.
    #[test]
    fn decode_rejects_response_with_arm_at_field_2() {
        // Hand-build PersistentConnectionResponse{callId, callUnaryResponse=field 2}.
        let mut payload = vec![0x0a, 16];
        payload.extend_from_slice(&CALL_ID);
        let inner = CallUnaryResponse {
            result: Some(call_unary_response::Result::Response(b"pong".to_vec())),
        }
        .encode_to_vec();
        payload.push(0x12); // field 2, wire type 2
        payload.push(inner.len() as u8);
        payload.extend_from_slice(&inner);

        match decode_unary_response(&payload) {
            // Either the bytes fail to parse as AddUnaryHandlerRequest, or they
            // parse into the wrong arm — both are a hard failure for callers.
            Err(WireError::Decode(_)) => {}
            Err(WireError::UnexpectedMessage(m)) => {
                assert!(m.contains("addUnaryHandler"), "unexpected arm: {m}");
            }
            other => panic!("field-2 reply must not decode as a unary response: {other:?}"),
        }
    }

    /// A `DaemonError` payload is not a valid unary reply on the libp2p wire —
    /// its field 1 is a string where `callId` expects bytes, and it carries no
    /// oneof arm.
    #[test]
    fn decode_response_that_is_a_daemon_error() {
        let err_bytes = DaemonError {
            message: Some("stream reset".to_string()),
        }
        .encode_to_vec();

        // Field 1 string vs bytes are the same wire type, so the envelope parses;
        // the message text lands in call_id and no oneof arm is present.
        let decoded = PersistentConnectionRequest::decode(err_bytes.as_slice()).unwrap();
        assert_eq!(decoded.call_id, b"stream reset");
        assert!(decoded.message.is_none());

        match decode_unary_response(&err_bytes) {
            Err(WireError::UnexpectedMessage(m)) => {
                assert!(m.contains("no message"), "unexpected description: {m}");
            }
            other => panic!("expected UnexpectedMessage, got {other:?}"),
        }
    }

    #[test]
    fn decode_request_rejects_response_envelope() {
        let framed = encode_unary_response(&CALL_ID, Ok(b"x".to_vec()));
        let (payload, _) = unframe(&framed).unwrap();
        match decode_unary_request(payload) {
            Err(WireError::UnexpectedMessage(m)) => assert!(m.contains("unaryResponse")),
            other => panic!("expected UnexpectedMessage, got {other:?}"),
        }
    }

    #[test]
    fn decode_response_rejects_request_envelope() {
        let framed = encode_unary_request(&CALL_ID, b"", "DHTProtocol.rpc_ping", b"x");
        let (payload, _) = unframe(&framed).unwrap();
        match decode_unary_response(payload) {
            Err(WireError::UnexpectedMessage(m)) => assert!(m.contains("callUnary")),
            other => panic!("expected UnexpectedMessage, got {other:?}"),
        }
    }

    /// The 10 MiB cap is a real ceiling, not a hard limit on realistic traffic:
    /// a ~1 MiB store payload must round-trip through the framing intact.
    #[test]
    fn large_but_legal_payload_round_trips() {
        let data: Vec<u8> = (0..1_000_000u32).map(|i| (i % 251) as u8).collect();
        let framed = encode_unary_request(&CALL_ID, b"", "DHTProtocol.rpc_store", &data);
        let (payload, _) = unframe(&framed).unwrap();
        let (_, _, _, out) = decode_unary_request(payload).unwrap();
        assert_eq!(out, data);
    }
}
