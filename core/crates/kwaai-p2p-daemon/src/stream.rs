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
        return Err(Error::Protocol(format!("StreamInfo too large: {} bytes", len)));
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
