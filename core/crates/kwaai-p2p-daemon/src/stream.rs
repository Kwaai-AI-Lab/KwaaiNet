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
    use crate::protocol::p2pd::{
        persistent_connection_request, PersistentConnectionRequest,
    };
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

/// Encode a DHT response as a varint-framed `PersistentConnectionResponse`.
///
/// `call_id` must be the bytes extracted by `unwrap_stream_handler_request`.
/// `response_data` is the raw protobuf of the DHT response.
///
/// Returns the varint-framed bytes ready to write back to the TCP stream.
pub fn wrap_stream_handler_response(call_id: Vec<u8>, response_data: Vec<u8>) -> Vec<u8> {
    use crate::protocol::p2pd::{
        call_unary_response, persistent_connection_response, CallUnaryResponse,
        PersistentConnectionResponse,
    };
    use prost::Message as _;
    use unsigned_varint::encode as varint_encode;

    let wrapper = PersistentConnectionResponse {
        call_id,
        message: Some(persistent_connection_response::Message::CallUnaryResponse(
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
