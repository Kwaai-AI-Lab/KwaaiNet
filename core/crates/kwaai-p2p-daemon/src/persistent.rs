//! Persistent connection support for unary RPC handlers
//!
//! This module implements Hivemind's unary handler pattern via go-libp2p-daemon.
//! Unlike regular stream-based protocols, unary handlers use:
//! - A single persistent connection for all RPC calls
//! - UUID-based call tracking for concurrent requests
//! - Daemon-side protocol negotiation (no multistream in client code)
//! - Bidirectional RPC (can both send and receive requests)

use crate::error::{Error, Result};
use crate::protocol::p2pd::{
    persistent_connection_request, persistent_connection_response, AddUnaryHandlerRequest,
    CallUnaryRequest, CallUnaryResponse, PersistentConnectionRequest,
    PersistentConnectionResponse, RemoveUnaryHandlerRequest,
};
// use bytes for potential future needs
use prost::Message as ProstMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;
use tracing::{debug, error, trace, warn};
use unsigned_varint::encode as varint_encode;
use uuid::Uuid;

/// Type alias for unary handler functions (use Arc for cloning)
pub type UnaryHandlerFn =
    Arc<dyn Fn(Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>>> + Send>> + Send + Sync>;

/// Response future for pending RPC calls
type ResponseFuture = oneshot::Sender<Result<PersistentConnectionResponse>>;

/// Persistent connection for unary RPC handlers
pub struct PersistentConnection {
    /// Shared writer for sending requests
    writer: Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
    /// Background task reading responses
    reader_task: Option<JoinHandle<()>>,
    /// Pending RPC calls waiting for responses
    pending_calls: Arc<Mutex<HashMap<Uuid, ResponseFuture>>>,
    /// Registered unary handlers
    unary_handlers: Arc<Mutex<HashMap<String, UnaryHandlerFn>>>,
}

impl PersistentConnection {
    /// Create a new persistent connection from an upgraded stream
    pub fn new<R, W>(reader: R, writer: W) -> Self
    where
        R: AsyncReadExt + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let writer = Arc::new(Mutex::new(Box::new(writer) as Box<dyn AsyncWrite + Unpin + Send>));
        let pending_calls = Arc::new(Mutex::new(HashMap::new()));
        let unary_handlers = Arc::new(Mutex::new(HashMap::new()));

        // Spawn background task to read responses
        let reader_task = Some(Self::spawn_reader(
            reader,
            pending_calls.clone(),
            unary_handlers.clone(),
            writer.clone(),
        ));

        Self {
            writer,
            reader_task,
            pending_calls,
            unary_handlers,
        }
    }

    /// Spawn background task to read and dispatch responses
    fn spawn_reader<R>(
        mut reader: R,
        pending_calls: Arc<Mutex<HashMap<Uuid, ResponseFuture>>>,
        unary_handlers: Arc<Mutex<HashMap<String, UnaryHandlerFn>>>,
        writer: Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
    ) -> JoinHandle<()>
    where
        R: AsyncReadExt + Unpin + Send + 'static,
    {
        tokio::spawn(async move {
            loop {
                // Read varint-framed message
                let msg_bytes = match Self::read_varint_message(&mut reader).await {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!("Failed to read persistent connection message: {}", e);
                        break;
                    }
                };

                // Decode PersistentConnectionResponse
                let response = match PersistentConnectionResponse::decode(&msg_bytes[..]) {
                    Ok(resp) => resp,
                    Err(e) => {
                        error!("Failed to decode PersistentConnectionResponse: {}", e);
                        continue;
                    }
                };

                // Extract call ID
                let call_id = match Uuid::from_slice(&response.call_id) {
                    Ok(id) => id,
                    Err(e) => {
                        warn!("Received response with invalid call ID: {}", e);
                        continue;
                    }
                };

                trace!("Received persistent response for call {}", call_id);

                // Dispatch based on message type
                match response.message {
                    Some(persistent_connection_response::Message::CallUnaryResponse(_))
                    | Some(persistent_connection_response::Message::DaemonError(_))
                    | Some(persistent_connection_response::Message::Cancel(_)) => {
                        // Complete pending call
                        let mut calls = pending_calls.lock().await;
                        if let Some(tx) = calls.remove(&call_id) {
                            let _ = tx.send(Ok(response));
                        } else {
                            warn!("Received response for unknown call ID: {}", call_id);
                        }
                    }

                    Some(persistent_connection_response::Message::RequestHandling(req)) => {
                        // Incoming RPC request from remote peer
                        debug!(
                            "Handling incoming unary request for protocol: {}",
                            &req.proto
                        );

                        let proto = req.proto.clone();
                        let data = req.data.clone();
                        let call_id_bytes = response.call_id.clone();

                        // Look up handler
                        let handlers = unary_handlers.lock().await;
                        if let Some(handler) = handlers.get(&proto) {
                            let handler = handler.clone();
                            drop(handlers); // Release lock before async work

                            let writer = writer.clone();

                            // Execute handler in background
                            tokio::spawn(async move {
                                let result = handler(data).await;

                                // Send response back
                                let response = match result {
                                    Ok(response_data) => CallUnaryResponse {
                                        result: Some(
                                            crate::protocol::p2pd::call_unary_response::Result::Response(
                                                response_data,
                                            ),
                                        ),
                                    },
                                    Err(e) => CallUnaryResponse {
                                        result: Some(
                                            crate::protocol::p2pd::call_unary_response::Result::Error(
                                                e.to_string().into_bytes(),
                                            ),
                                        ),
                                    },
                                };

                                let msg = PersistentConnectionRequest {
                                    call_id: call_id_bytes,
                                    message: Some(
                                        persistent_connection_request::Message::UnaryResponse(response),
                                    ),
                                };

                                let mut w = writer.lock().await;
                                if let Err(e) = Self::write_varint_message(&mut *w, &msg).await {
                                    error!("Failed to send unary response: {}", e);
                                }
                            });
                        } else {
                            warn!("No handler registered for protocol: {}", proto);

                            // Send error response
                            let error_response = CallUnaryResponse {
                                result: Some(
                                    crate::protocol::p2pd::call_unary_response::Result::Error(
                                        format!("handler for protocol {} not found", proto)
                                            .into_bytes(),
                                    ),
                                ),
                            };

                            let msg = PersistentConnectionRequest {
                                call_id: call_id_bytes,
                                message: Some(
                                    persistent_connection_request::Message::UnaryResponse(error_response),
                                ),
                            };

                            let mut w = writer.lock().await;
                            if let Err(e) = Self::write_varint_message(&mut *w, &msg).await {
                                error!("Failed to send error response: {}", e);
                            }
                        }
                    }

                    None => {
                        warn!("Received persistent response with no message");
                    }
                }
            }

            debug!("Persistent connection reader task exiting");
        })
    }

    /// Call a unary handler on a remote peer
    pub async fn call_unary(
        &self,
        peer_id: &[u8],
        proto: &str,
        data: &[u8],
    ) -> Result<Vec<u8>> {
        let call_id = Uuid::new_v4();
        debug!("Calling unary handler {} on peer (call_id: {})", proto, call_id);

        // Create response channel
        let (tx, rx) = oneshot::channel();

        // Register pending call
        {
            let mut calls = self.pending_calls.lock().await;
            calls.insert(call_id, tx);
        }

        // Build request message
        let request = PersistentConnectionRequest {
            call_id: call_id.as_bytes().to_vec(),
            message: Some(persistent_connection_request::Message::CallUnary(
                CallUnaryRequest {
                    peer: peer_id.to_vec(),
                    proto: proto.to_string(),
                    data: data.to_vec(),
                },
            )),
        };

        // Send request
        {
            let mut writer = self.writer.lock().await;
            Self::write_varint_message(&mut *writer, &request).await?;
        }

        // Wait for response
        let response = rx.await.map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

        // Extract result
        match response.message {
            Some(persistent_connection_response::Message::CallUnaryResponse(resp)) => {
                match resp.result {
                    Some(crate::protocol::p2pd::call_unary_response::Result::Response(data)) => {
                        debug!("Unary call {} succeeded ({} bytes)", call_id, data.len());
                        Ok(data)
                    }
                    Some(crate::protocol::p2pd::call_unary_response::Result::Error(err)) => {
                        let err_msg = String::from_utf8_lossy(&err);
                        error!("Unary call {} failed: {}", call_id, err_msg);
                        Err(Error::Protocol(format!("Remote handler error: {}", err_msg)))
                    }
                    None => Err(Error::Protocol("Empty unary response".to_string())),
                }
            }
            Some(persistent_connection_response::Message::DaemonError(err)) => {
                let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
                error!("Daemon error for call {}: {}", call_id, err_msg);
                Err(Error::Protocol(format!("Daemon error: {}", err_msg)))
            }
            _ => Err(Error::Protocol("Unexpected response type".to_string())),
        }
    }

    /// Register a unary handler for incoming requests
    pub async fn add_unary_handler<F, Fut>(
        &self,
        proto: &str,
        handler: F,
        balanced: bool,
    ) -> Result<()>
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Vec<u8>>> + Send + 'static,
    {
        let call_id = Uuid::new_v4();
        debug!("Adding unary handler for protocol: {}", proto);

        // Create response channel
        let (tx, rx) = oneshot::channel();

        // Register pending call
        {
            let mut calls = self.pending_calls.lock().await;
            calls.insert(call_id, tx);
        }

        // Build request message
        let request = PersistentConnectionRequest {
            call_id: call_id.as_bytes().to_vec(),
            message: Some(persistent_connection_request::Message::AddUnaryHandler(
                AddUnaryHandlerRequest {
                    proto: proto.to_string(),
                    balanced,
                },
            )),
        };

        // Send request
        {
            let mut writer = self.writer.lock().await;
            Self::write_varint_message(&mut *writer, &request).await?;
        }

        // Wait for response
        let response = rx.await.map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

        // Check for errors
        if let Some(persistent_connection_response::Message::DaemonError(err)) = response.message {
            let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            return Err(Error::Protocol(format!("Failed to add handler: {}", err_msg)));
        }

        // Store handler
        {
            let mut handlers = self.unary_handlers.lock().await;
            handlers.insert(
                proto.to_string(),
                Arc::new(move |data| Box::pin(handler(data))),
            );
        }

        debug!("Successfully registered unary handler for: {}", proto);
        Ok(())
    }

    /// Remove a unary handler
    pub async fn remove_unary_handler(&self, proto: &str) -> Result<()> {
        let call_id = Uuid::new_v4();
        debug!("Removing unary handler for protocol: {}", proto);

        // Create response channel
        let (tx, rx) = oneshot::channel();

        // Register pending call
        {
            let mut calls = self.pending_calls.lock().await;
            calls.insert(call_id, tx);
        }

        // Build request message
        let request = PersistentConnectionRequest {
            call_id: call_id.as_bytes().to_vec(),
            message: Some(persistent_connection_request::Message::RemoveUnaryHandler(
                RemoveUnaryHandlerRequest {
                    proto: proto.to_string(),
                },
            )),
        };

        // Send request
        {
            let mut writer = self.writer.lock().await;
            Self::write_varint_message(&mut *writer, &request).await?;
        }

        // Wait for response
        let response = rx.await.map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

        // Check for errors
        if let Some(persistent_connection_response::Message::DaemonError(err)) = response.message {
            let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            return Err(Error::Protocol(format!("Failed to remove handler: {}", err_msg)));
        }

        // Remove handler from map
        {
            let mut handlers = self.unary_handlers.lock().await;
            handlers.remove(proto);
        }

        debug!("Successfully removed unary handler for: {}", proto);
        Ok(())
    }

    /// Read a varint-length-prefixed message
    async fn read_varint_message<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
        // Read varint length
        let mut length_buf = [0u8; 10]; // Max varint size
        let mut length = 0usize;

        for i in 0..10 {
            reader
                .read_exact(&mut length_buf[i..=i])
                .await
                .map_err(|e| Error::Io(e))?;

            if length_buf[i] & 0x80 == 0 {
                // Last byte of varint
                let (len, _) = unsigned_varint::decode::usize(&length_buf[..=i])
                    .map_err(|e| Error::Protocol(format!("Invalid varint: {}", e)))?;
                length = len;
                break;
            }
        }

        if length == 0 {
            return Err(Error::Protocol("Invalid varint length".to_string()));
        }

        // Read message data
        let mut data = vec![0u8; length];
        reader
            .read_exact(&mut data)
            .await
            .map_err(|e| Error::Io(e))?;

        Ok(data)
    }

    /// Write a varint-length-prefixed message
    async fn write_varint_message<W: AsyncWrite + Unpin, M: ProstMessage>(
        writer: &mut W,
        msg: &M,
    ) -> Result<()> {
        // Encode message
        let mut msg_buf = Vec::new();
        msg.encode(&mut msg_buf)
            .map_err(|e| Error::Protocol(format!("Failed to encode message: {}", e)))?;

        // Encode length as varint
        let mut length_buf = varint_encode::usize_buffer();
        let length_bytes = varint_encode::usize(msg_buf.len(), &mut length_buf);

        // Write length + message
        writer
            .write_all(length_bytes)
            .await
            .map_err(|e| Error::Io(e))?;
        writer.write_all(&msg_buf).await.map_err(|e| Error::Io(e))?;
        writer.flush().await.map_err(|e| Error::Io(e))?;

        Ok(())
    }
}

impl Drop for PersistentConnection {
    fn drop(&mut self) {
        if let Some(task) = self.reader_task.take() {
            task.abort();
        }
    }
}
