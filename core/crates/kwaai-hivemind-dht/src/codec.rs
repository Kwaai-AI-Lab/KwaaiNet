//! Codec for Hivemind DHT protocol over libp2p
//!
//! Implements the wire format for DHT RPC messages:
//! [8-byte length (big-endian)][1-byte marker][protobuf payload]

use crate::protocol::*;
use crate::{Error, Result};
use async_trait::async_trait;
use futures::prelude::*;
use libp2p::request_response::Codec;
use libp2p::StreamProtocol;
use prost::Message;
use std::io;

/// Maximum message size (10 MB)
const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// Protocol markers for different RPC types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ProtocolMarker {
    Ping = 0x01,
    Store = 0x02,
    Find = 0x03,
}

impl ProtocolMarker {
    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Self::Ping),
            0x02 => Some(Self::Store),
            0x03 => Some(Self::Find),
            _ => None,
        }
    }
}

/// DHT request message types
#[derive(Debug, Clone)]
pub enum DHTRequest {
    Ping(PingRequest),
    Store(StoreRequest),
    Find(FindRequest),
}

/// DHT response message types
#[derive(Debug, Clone)]
pub enum DHTResponse {
    Ping(PingResponse),
    Store(StoreResponse),
    Find(FindResponse),
}

impl DHTRequest {
    /// Get the protocol marker for this request
    pub fn marker(&self) -> ProtocolMarker {
        match self {
            Self::Ping(_) => ProtocolMarker::Ping,
            Self::Store(_) => ProtocolMarker::Store,
            Self::Find(_) => ProtocolMarker::Find,
        }
    }

    /// Encode to bytes with length prefix and marker
    pub fn encode(&self) -> Result<Vec<u8>> {
        let (marker, proto_bytes) = match self {
            Self::Ping(req) => (ProtocolMarker::Ping, req.encode_to_vec()),
            Self::Store(req) => (ProtocolMarker::Store, req.encode_to_vec()),
            Self::Find(req) => (ProtocolMarker::Find, req.encode_to_vec()),
        };

        let total_len = 1 + proto_bytes.len(); // marker + protobuf
        let mut result = Vec::with_capacity(8 + total_len);

        // 8-byte length prefix (big-endian)
        result.extend_from_slice(&(total_len as u64).to_be_bytes());
        // 1-byte marker
        result.push(marker as u8);
        // Protobuf payload
        result.extend_from_slice(&proto_bytes);

        Ok(result)
    }

    /// Decode from bytes
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(Error::ProtobufDecode(prost::DecodeError::new(
                "Empty message",
            )));
        }

        let marker = ProtocolMarker::from_u8(bytes[0]).ok_or_else(|| {
            Error::ProtobufDecode(prost::DecodeError::new("Invalid protocol marker"))
        })?;

        let proto_bytes = &bytes[1..];

        match marker {
            ProtocolMarker::Ping => {
                let req = PingRequest::decode(proto_bytes)?;
                Ok(Self::Ping(req))
            }
            ProtocolMarker::Store => {
                let req = StoreRequest::decode(proto_bytes)?;
                Ok(Self::Store(req))
            }
            ProtocolMarker::Find => {
                let req = FindRequest::decode(proto_bytes)?;
                Ok(Self::Find(req))
            }
        }
    }
}

impl DHTResponse {
    /// Get the protocol marker for this response
    pub fn marker(&self) -> ProtocolMarker {
        match self {
            Self::Ping(_) => ProtocolMarker::Ping,
            Self::Store(_) => ProtocolMarker::Store,
            Self::Find(_) => ProtocolMarker::Find,
        }
    }

    /// Encode to bytes with length prefix and marker
    pub fn encode(&self) -> Result<Vec<u8>> {
        let (marker, proto_bytes) = match self {
            Self::Ping(res) => (ProtocolMarker::Ping, res.encode_to_vec()),
            Self::Store(res) => (ProtocolMarker::Store, res.encode_to_vec()),
            Self::Find(res) => (ProtocolMarker::Find, res.encode_to_vec()),
        };

        let total_len = 1 + proto_bytes.len();
        let mut result = Vec::with_capacity(8 + total_len);

        result.extend_from_slice(&(total_len as u64).to_be_bytes());
        result.push(marker as u8);
        result.extend_from_slice(&proto_bytes);

        Ok(result)
    }

    /// Decode from bytes
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(Error::ProtobufDecode(prost::DecodeError::new(
                "Empty message",
            )));
        }

        let marker = ProtocolMarker::from_u8(bytes[0]).ok_or_else(|| {
            Error::ProtobufDecode(prost::DecodeError::new("Invalid protocol marker"))
        })?;

        let proto_bytes = &bytes[1..];

        match marker {
            ProtocolMarker::Ping => {
                let res = PingResponse::decode(proto_bytes)?;
                Ok(Self::Ping(res))
            }
            ProtocolMarker::Store => {
                let res = StoreResponse::decode(proto_bytes)?;
                Ok(Self::Store(res))
            }
            ProtocolMarker::Find => {
                let res = FindResponse::decode(proto_bytes)?;
                Ok(Self::Find(res))
            }
        }
    }
}

/// Codec for Hivemind DHT protocol
#[derive(Debug, Clone)]
pub struct HivemindCodec;

impl HivemindCodec {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HivemindCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Codec for HivemindCodec {
    type Protocol = StreamProtocol;
    type Request = DHTRequest;
    type Response = DHTResponse;

    async fn read_request<T>(&mut self, _: &Self::Protocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        // Read 8-byte length prefix
        let mut len_buf = [0u8; 8];
        io.read_exact(&mut len_buf).await?;
        let len = u64::from_be_bytes(len_buf) as usize;

        if len > MAX_MESSAGE_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Message too large: {} bytes", len),
            ));
        }

        // Read message body (marker + protobuf)
        let mut msg_buf = vec![0u8; len];
        io.read_exact(&mut msg_buf).await?;

        // Decode request
        DHTRequest::decode(&msg_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn read_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        // Read 8-byte length prefix
        let mut len_buf = [0u8; 8];
        io.read_exact(&mut len_buf).await?;
        let len = u64::from_be_bytes(len_buf) as usize;

        if len > MAX_MESSAGE_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Message too large: {} bytes", len),
            ));
        }

        // Read message body
        let mut msg_buf = vec![0u8; len];
        io.read_exact(&mut msg_buf).await?;

        // Decode response
        DHTResponse::decode(&msg_buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    async fn write_request<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let bytes = req
            .encode()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        io.write_all(&bytes).await?;
        io.flush().await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let bytes = res
            .encode()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        io.write_all(&bytes).await?;
        io.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_ping() {
        let node = NodeInfo::from_peer_id(libp2p::PeerId::random());
        let req = DHTRequest::Ping(PingRequest::new(node, true));

        let bytes = req.encode().unwrap();
        assert!(bytes.len() >= 9); // 8-byte length + 1-byte marker

        let decoded = DHTRequest::decode(&bytes[8..]).unwrap();
        matches!(decoded, DHTRequest::Ping(_));
    }

    #[test]
    fn test_encode_decode_find() {
        let node = NodeInfo::from_peer_id(libp2p::PeerId::random());
        let keys = vec![b"key1".to_vec(), b"key2".to_vec()];
        let req = DHTRequest::Find(FindRequest::new(node, keys));

        let bytes = req.encode().unwrap();
        let decoded = DHTRequest::decode(&bytes[8..]).unwrap();

        if let DHTRequest::Find(find_req) = decoded {
            assert_eq!(find_req.keys.len(), 2);
        } else {
            panic!("Expected Find request");
        }
    }
}
