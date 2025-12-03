//! Hivemind RPC Protocol Handler
//!
//! Implements the Hivemind/Petals RPC protocol using libp2p request-response.
//! This allows KwaaiNet nodes to respond to health monitor queries and other
//! Hivemind protocol requests.

use crate::hivemind::{
    encode_error, encode_message, ExpertInfo, ExpertUID, ServerInfo, HIVEMIND_PROTOCOL,
};
use async_trait::async_trait;
use futures::prelude::*;
use libp2p::{
    request_response::{self, Codec, ProtocolSupport},
    StreamProtocol,
};
use prost::Message;
use std::io;
use tracing::{debug, error, info};

// =============================================================================
// Protocol Codec
// =============================================================================

/// Hivemind RPC protocol codec
///
/// Implements the Hivemind message framing:
/// - 8 bytes: length (big-endian)
/// - 1 byte: marker (0x00=message, 0x01=error)
/// - N bytes: protobuf payload
#[derive(Debug, Clone, Default)]
pub struct HivemindCodec;

#[async_trait]
impl Codec for HivemindCodec {
    type Protocol = StreamProtocol;
    type Request = RpcRequest;
    type Response = RpcResponse;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        // Read length prefix (8 bytes)
        let mut len_buf = [0u8; 8];
        io.read_exact(&mut len_buf).await?;
        let len = u64::from_be_bytes(len_buf) as usize;

        if len > 10_000_000 {
            // 10MB max
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message too large",
            ));
        }

        // Read marker + payload
        let mut buf = vec![0u8; len];
        io.read_exact(&mut buf).await?;

        if buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty message",
            ));
        }

        let marker = buf[0];
        let payload = &buf[1..];

        if marker == 0x01 {
            // Error marker
            return Err(io::Error::new(
                io::ErrorKind::Other,
                std::str::from_utf8(payload).unwrap_or("Unknown error"),
            ));
        }

        // Parse based on what we receive
        // For now, we only support rpc_info requests (ExpertUID)
        match ExpertUID::decode(payload) {
            Ok(expert_uid) => Ok(RpcRequest::Info(expert_uid)),
            Err(e) => {
                error!("Failed to decode ExpertUID: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to decode request: {}", e),
                ))
            }
        }
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        // Read length prefix (8 bytes)
        let mut len_buf = [0u8; 8];
        io.read_exact(&mut len_buf).await?;
        let len = u64::from_be_bytes(len_buf) as usize;

        if len > 10_000_000 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Response too large",
            ));
        }

        // Read marker + payload
        let mut buf = vec![0u8; len];
        io.read_exact(&mut buf).await?;

        if buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Empty response",
            ));
        }

        let marker = buf[0];
        let payload = &buf[1..];

        if marker == 0x01 {
            // Error response
            let error_msg = std::str::from_utf8(payload)
                .unwrap_or("Unknown error")
                .to_string();
            return Ok(RpcResponse::Error(error_msg));
        }

        // Parse ExpertInfo response
        match ExpertInfo::decode(payload) {
            Ok(expert_info) => Ok(RpcResponse::Info(expert_info)),
            Err(e) => {
                error!("Failed to decode ExpertInfo: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to decode response: {}", e),
                ))
            }
        }
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = match req {
            RpcRequest::Info(expert_uid) => encode_message(&expert_uid),
        };

        io.write_all(&data).await?;
        io.close().await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = match res {
            RpcResponse::Info(expert_info) => encode_message(&expert_info),
            RpcResponse::Error(error_msg) => encode_error(&error_msg),
        };

        io.write_all(&data).await?;
        io.close().await?;
        Ok(())
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

/// RPC request types
#[derive(Debug, Clone)]
pub enum RpcRequest {
    /// Get server info (rpc_info)
    Info(ExpertUID),
}

/// RPC response types
#[derive(Debug, Clone)]
pub enum RpcResponse {
    /// Server info response
    Info(ExpertInfo),
    /// Error response
    Error(String),
}

// =============================================================================
// RPC Handler
// =============================================================================

/// Handles incoming RPC requests
pub struct RpcHandler {
    server_info: ServerInfo,
}

impl RpcHandler {
    /// Create a new RPC handler with the given server info
    pub fn new(server_info: ServerInfo) -> Self {
        Self { server_info }
    }

    /// Update server info (e.g., when load changes)
    pub fn update_info(&mut self, server_info: ServerInfo) {
        self.server_info = server_info;
    }

    /// Handle an RPC request and generate a response
    pub fn handle_request(&self, request: RpcRequest) -> RpcResponse {
        match request {
            RpcRequest::Info(expert_uid) => {
                info!(
                    "Handling rpc_info request for UID: {}",
                    if expert_uid.uid.is_empty() {
                        "general"
                    } else {
                        &expert_uid.uid
                    }
                );

                match self.server_info.to_expert_info() {
                    Ok(info) => {
                        debug!("Responding with server info");
                        RpcResponse::Info(info)
                    }
                    Err(e) => {
                        error!("Failed to serialize server info: {}", e);
                        RpcResponse::Error(format!("Failed to serialize info: {}", e))
                    }
                }
            }
        }
    }
}

// =============================================================================
// Protocol Configuration
// =============================================================================

/// Create a Hivemind RPC protocol configuration
pub fn create_hivemind_protocol() -> (
    request_response::Behaviour<HivemindCodec>,
    StreamProtocol,
) {
    let protocol = StreamProtocol::new(HIVEMIND_PROTOCOL);
    let codec = HivemindCodec;

    let behaviour = request_response::Behaviour::with_codec(
        codec,
        [(protocol.clone(), ProtocolSupport::Full)],
        request_response::Config::default(),
    );

    (behaviour, protocol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_info_handling() {
        let info = ServerInfo::new("test-node")
            .with_span(0, 8)
            .with_cache_tokens(1000);

        let handler = RpcHandler::new(info);

        let request = RpcRequest::Info(ExpertUID {
            uid: String::new(),
        });

        match handler.handle_request(request) {
            RpcResponse::Info(expert_info) => {
                assert!(!expert_info.serialized_info.is_empty());

                // Decode and verify
                let decoded = ServerInfo::from_msgpack(&expert_info.serialized_info).unwrap();
                assert_eq!(decoded.public_name, Some("test-node".to_string()));
            }
            RpcResponse::Error(e) => panic!("Expected Info response, got Error: {}", e),
        }
    }
}
