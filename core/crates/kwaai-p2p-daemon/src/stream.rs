//! Stream handling utilities for daemon-forwarded streams
//!
//! When the daemon forwards a stream to our handler, it sends:
//! 1. StreamInfo (varint-framed protobuf) - peer_id, addr, protocol
//! 2. The actual protocol stream data
//!
//! This module provides helpers to parse StreamInfo and handle the stream.

use crate::error::{Error, Result};
use crate::protocol::p2pd::StreamInfo;
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, trace};

/// Parse StreamInfo from the beginning of a daemon-forwarded stream
///
/// The daemon sends StreamInfo as a varint-framed protobuf message before
/// forwarding the actual protocol stream.
pub async fn parse_stream_info(stream: &mut TcpStream) -> Result<StreamInfo> {
    // Read varint length prefix
    let mut len_bytes = Vec::new();
    let mut byte = [0u8; 1];

    // Read varint byte by byte (max 10 bytes for u64)
    for _ in 0..10 {
        stream.read_exact(&mut byte).await?;
        len_bytes.push(byte[0]);

        // Check if this is the last byte (MSB is 0)
        if byte[0] & 0x80 == 0 {
            break;
        }
    }

    // Decode varint
    let mut cursor = &len_bytes[..];
    let len = match unsigned_varint::io::read_u64(&mut cursor) {
        Ok(l) => l as usize,
        Err(e) => return Err(Error::Protocol(format!("Failed to decode varint: {}", e))),
    };

    // Sanity check
    if len > 10 * 1024 * 1024 {
        // 10MB max
        return Err(Error::Protocol(format!(
            "StreamInfo too large: {} bytes",
            len
        )));
    }

    trace!("Reading StreamInfo ({} bytes)", len);

    // Read StreamInfo payload
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload).await?;

    // Decode protobuf
    let stream_info = StreamInfo::decode(&payload[..])
        .map_err(|e| Error::Protocol(format!("Failed to decode StreamInfo: {}", e)))?;

    debug!(
        "StreamInfo: proto={}, peer_len={}, addr_len={}",
        stream_info.proto,
        stream_info.peer.len(),
        stream_info.addr.len()
    );

    Ok(stream_info)
}

/// Write a varint-framed message to the stream
pub async fn write_varint_framed(stream: &mut TcpStream, payload: &[u8]) -> Result<()> {
    let len = payload.len();

    // Encode length as varint
    let mut len_buf = unsigned_varint::encode::u64_buffer();
    let len_bytes = unsigned_varint::encode::u64(len as u64, &mut len_buf);

    // Write varint length + payload
    stream.write_all(len_bytes).await?;
    stream.write_all(payload).await?;
    stream.flush().await?;

    Ok(())
}

/// Read a varint-framed message from the stream
pub async fn read_varint_framed(stream: &mut TcpStream) -> Result<Vec<u8>> {
    // Read varint length prefix
    let mut len_bytes = Vec::new();
    let mut byte = [0u8; 1];

    for _ in 0..10 {
        stream.read_exact(&mut byte).await?;
        len_bytes.push(byte[0]);

        if byte[0] & 0x80 == 0 {
            break;
        }
    }

    // Decode varint
    let mut cursor = &len_bytes[..];
    let len = match unsigned_varint::io::read_u64(&mut cursor) {
        Ok(l) => l as usize,
        Err(e) => return Err(Error::Protocol(format!("Failed to decode varint: {}", e))),
    };

    // Sanity check
    if len > 100 * 1024 * 1024 {
        // 100MB max for protocol messages
        return Err(Error::Protocol(format!("Message too large: {} bytes", len)));
    }

    // Read payload
    let mut payload = vec![0u8; len];
    stream.read_exact(&mut payload).await?;

    Ok(payload)
}

/// Decode a varint-framed `PersistentConnectionRequest` sent by a remote peer
/// over a stream-handler connection.
///
/// Returns `(call_id_bytes, dht_payload_bytes)`.  The `dht_payload_bytes` is
/// the raw protobuf of the actual DHT request (StoreRequest / FindRequest /
/// PingRequest) and is decoded by the caller using the kwaai-hivemind-dht
/// prost types (which share the same prost version as the workspace).
///
/// This function lives in kwaai-p2p-daemon so it can use prost 0.13 (the same
/// version as the p2pd protobuf types) without causing a version conflict in
/// kwaai-cli which uses prost 0.12 via the workspace.
pub fn unwrap_stream_handler_request(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    use crate::protocol::p2pd::{persistent_connection_request, PersistentConnectionRequest};
    use prost::Message as _;

    let outer = PersistentConnectionRequest::decode(bytes)
        .map_err(|e| Error::Protocol(format!("decode PersistentConnectionRequest: {}", e)))?;

    let call_id = outer.call_id.clone();

    let dht_data = match &outer.message {
        Some(persistent_connection_request::Message::CallUnary(cu)) => cu.data.clone(),
        other => {
            return Err(Error::Protocol(format!(
                "expected CallUnary in PersistentConnectionRequest, got: {:?}",
                other.as_ref().map(|_| "other variant")
            )))
        }
    };

    Ok((call_id, dht_data))
}

/// Encode a DHT response as a varint-framed `PersistentConnectionRequest`
/// carrying the `unaryResponse` oneof arm (field 4).
///
/// `call_id` must be the bytes extracted by `unwrap_stream_handler_request`.
/// `response_data` is the raw protobuf of the DHT response.
///
/// Returns the varint-framed bytes ready to write back to the TCP stream.
///
/// # Why `PersistentConnectionRequest` and not `...Response`
///
/// `PersistentConnectionResponse` exists only on the daemon's *local control
/// socket*. On the libp2p wire between peers both directions are
/// `PersistentConnectionRequest`: go-libp2p-daemon's `Daemon.exchangeMessages`
/// (persistent_stream.go) reads the reply into `&pb.PersistentConnectionRequest{}`
/// and takes `GetUnaryResponse()` — field 4.
pub fn wrap_stream_handler_response(call_id: Vec<u8>, response_data: Vec<u8>) -> Vec<u8> {
    use crate::protocol::p2pd::{
        call_unary_response, persistent_connection_request, CallUnaryResponse,
        PersistentConnectionRequest,
    };
    use prost::Message as _;
    use unsigned_varint::encode as varint_encode;

    let wrapper = PersistentConnectionRequest {
        call_id,
        message: Some(persistent_connection_request::Message::UnaryResponse(
            CallUnaryResponse {
                result: Some(call_unary_response::Result::Response(response_data)),
            },
        )),
    };

    let wrapper_bytes = wrapper.encode_to_vec();
    let mut vbuf = varint_encode::usize_buffer();
    let prefix = varint_encode::usize(wrapper_bytes.len(), &mut vbuf);
    let mut framed = Vec::with_capacity(prefix.len() + wrapper_bytes.len());
    framed.extend_from_slice(prefix);
    framed.extend_from_slice(&wrapper_bytes);
    framed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::p2pd::{
        call_unary_response, persistent_connection_request, persistent_connection_response,
        CallUnaryResponse, PersistentConnectionRequest, PersistentConnectionResponse,
    };

    fn call_id() -> Vec<u8> {
        (0u8..16).collect()
    }

    /// Strip the uvarint prefix off a framed message.
    fn unframe(framed: &[u8]) -> &[u8] {
        let (len, rest) = unsigned_varint::decode::usize(framed).expect("varint prefix");
        assert_eq!(rest.len(), len, "frame length prefix must match payload");
        rest
    }

    /// The reply a Go caller sees must decode as `PersistentConnectionRequest`
    /// with the `unaryResponse` arm — this is what `Daemon.exchangeMessages`
    /// does (`ReadMsg(&pb.PersistentConnectionRequest{})` + `GetUnaryResponse()`).
    #[test]
    fn wrapped_response_decodes_as_persistent_connection_request() {
        let framed = wrap_stream_handler_response(call_id(), b"dht-response".to_vec());
        let payload = unframe(&framed);

        let decoded =
            PersistentConnectionRequest::decode(payload).expect("must decode as ...Request");

        assert_eq!(decoded.call_id, call_id(), "callId must be echoed verbatim");
        match decoded.message {
            Some(persistent_connection_request::Message::UnaryResponse(r)) => {
                match r.result {
                    Some(call_unary_response::Result::Response(data)) => {
                        assert_eq!(data, b"dht-response");
                    }
                    other => panic!("expected Response arm, got {other:?}"),
                }
            }
            other => panic!("expected unaryResponse arm, got {other:?}"),
        }
    }

    /// The oneof arm must land on field 4.
    #[test]
    fn wrapped_response_uses_field_4() {
        let framed = wrap_stream_handler_response(call_id(), b"pong".to_vec());
        let payload = unframe(&framed);

        // callId: field 1, wire type 2.
        assert_eq!(payload[0], 0x0a);
        assert_eq!(payload[1], 16);
        assert_eq!(&payload[2..18], &call_id()[..]);
        // unaryResponse: field 4, wire type 2 → (4 << 3) | 2 == 0x22.
        assert_eq!(
            payload[18], 0x22,
            "reply oneof must be field 4 (unaryResponse), not field 2"
        );
    }

    /// A field-2 arm decodes as `AddUnaryHandlerRequest`: Go's gogo (proto2)
    /// rejects it for the absent `required bool balanced`, while prost accepts
    /// it and mis-decodes the payload as a handler name. `GetUnaryResponse()`
    /// is nil either way.
    #[test]
    fn response_shaped_reply_does_not_yield_a_unary_response() {
        let old = PersistentConnectionResponse {
            call_id: call_id(),
            message: Some(persistent_connection_response::Message::CallUnaryResponse(
                CallUnaryResponse {
                    result: Some(call_unary_response::Result::Response(b"pong".to_vec())),
                },
            )),
        };
        let bytes = old.encode_to_vec();

        // Field 2 on the wire, exactly where AddUnaryHandlerRequest lives.
        assert_eq!(bytes[18], 0x12, "old shape put its arm at field 2");

        match PersistentConnectionRequest::decode(bytes.as_slice()) {
            // Go's gogo unmarshal fails outright here (required fields absent).
            Err(_) => {}
            Ok(decoded) => {
                // prost's observed behaviour: the response payload "pong" lands
                // in AddUnaryHandlerRequest.proto. Nonsense, but not a
                // unaryResponse — which is the point.
                assert!(
                    matches!(
                        decoded.message,
                        Some(persistent_connection_request::Message::AddUnaryHandler(ref h))
                            if h.proto == "pong" && !h.balanced
                    ),
                    "expected the payload to be mis-read as AddUnaryHandlerRequest, got {:?}",
                    decoded.message
                );
            }
        }
    }

    /// Round trip through the pair of helpers a stream handler actually uses.
    #[test]
    fn unwrap_request_then_wrap_response_round_trip() {
        use crate::protocol::p2pd::CallUnaryRequest;

        let inbound = PersistentConnectionRequest {
            call_id: call_id(),
            message: Some(persistent_connection_request::Message::CallUnary(
                CallUnaryRequest {
                    peer: b"caller-peer".to_vec(),
                    proto: "DHTProtocol.rpc_ping".to_string(),
                    data: b"ping-payload".to_vec(),
                },
            )),
        }
        .encode_to_vec();

        let (id, data) = unwrap_stream_handler_request(&inbound).expect("unwrap");
        assert_eq!(id, call_id());
        assert_eq!(data, b"ping-payload");

        let framed = wrap_stream_handler_response(id.clone(), b"pong-payload".to_vec());
        let decoded = PersistentConnectionRequest::decode(unframe(&framed)).expect("decode");
        assert_eq!(decoded.call_id, id);
    }

    #[test]
    fn test_varint_encoding() {
        let payload = b"test payload";
        let len = payload.len() as u64;

        let mut len_buf = unsigned_varint::encode::u64_buffer();
        let len_bytes = unsigned_varint::encode::u64(len, &mut len_buf);

        // Decode it back
        let mut cursor = len_bytes;
        let decoded_len = unsigned_varint::io::read_u64(&mut cursor).unwrap();

        assert_eq!(decoded_len, len);
    }
}

