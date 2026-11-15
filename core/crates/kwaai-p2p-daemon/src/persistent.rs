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
    CallUnaryRequest, CallUnaryResponse, PersistentConnectionRequest, PersistentConnectionResponse,
    RemoveUnaryHandlerRequest,
};
// use bytes for potential future needs
use prost::Message as ProstMessage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{oneshot, watch, Mutex};
use tokio::task::JoinHandle;
use tracing::{debug, error, trace, warn};
use unsigned_varint::encode as varint_encode;
use uuid::Uuid;

/// Largest frame either daemon reads on a control connection: the Go daemon's
/// `persistentConnMsgMaxSize`, and the native server's inbound cap. A larger
/// frame is not an error the daemon reports per call — it closes the whole
/// connection, and with it every handler registered on it — so nothing larger
/// may be written.
pub const MAX_FRAME_SIZE: usize = 10 * 1024 * 1024;

/// Type alias for unary handler functions (use Arc for cloning)
pub type UnaryHandlerFn = Arc<
    dyn Fn(Vec<u8>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>>> + Send>>
        + Send
        + Sync,
>;

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
    /// Flips to `true` when the reader task exits, i.e. the connection is gone.
    closed: watch::Receiver<bool>,
}

impl PersistentConnection {
    /// Create a new persistent connection from an upgraded stream
    pub fn new<R, W>(reader: R, writer: W) -> Self
    where
        R: AsyncReadExt + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let writer = Arc::new(Mutex::new(
            Box::new(writer) as Box<dyn AsyncWrite + Unpin + Send>
        ));
        let pending_calls = Arc::new(Mutex::new(HashMap::new()));
        let unary_handlers = Arc::new(Mutex::new(HashMap::new()));
        let (closed_tx, closed) = watch::channel(false);

        // Spawn background task to read responses
        let reader_task = Some(Self::spawn_reader(
            reader,
            pending_calls.clone(),
            unary_handlers.clone(),
            writer.clone(),
            closed_tx,
        ));

        Self {
            writer,
            reader_task,
            pending_calls,
            unary_handlers,
            closed,
        }
    }

    /// Spawn background task to read and dispatch responses
    fn spawn_reader<R>(
        mut reader: R,
        pending_calls: Arc<Mutex<HashMap<Uuid, ResponseFuture>>>,
        unary_handlers: Arc<Mutex<HashMap<String, UnaryHandlerFn>>>,
        writer: Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
        closed_tx: watch::Sender<bool>,
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
                        // EOF and stream-reset are normal when the remote peer closes the
                        // connection.  Log at warn so they're visible but not alarming.
                        let e_str = e.to_string();
                        if e_str.contains("early eof")
                            || e_str.contains("unexpected eof")
                            || e_str.contains("stream reset")
                        {
                            warn!("Persistent connection closed by remote: {}", e);
                        } else {
                            error!("Failed to read persistent connection message: {}", e);
                        }
                        // Drain all pending calls so their awaiters unblock immediately
                        // rather than hanging until the oneshot sender is dropped.
                        let mut calls = pending_calls.lock().await;
                        for (_, tx) in calls.drain() {
                            let _ =
                                tx.send(Err(Error::Protocol(format!("connection closed: {e}"))));
                        }
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

                                let msg = Self::unary_response_within_limit(
                                    call_id_bytes,
                                    response,
                                    &proto,
                                );

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
                                    persistent_connection_request::Message::UnaryResponse(
                                        error_response,
                                    ),
                                ),
                            };

                            let mut w = writer.lock().await;
                            if let Err(e) = Self::write_varint_message(&mut *w, &msg).await {
                                error!("Failed to send error response: {}", e);
                            }
                        }
                    }

                    None => {
                        // p2pd sends an empty response (message=None) as an ACK for
                        // AddUnaryHandler / RemoveUnaryHandler — route it to the pending
                        // call so the awaiter can unblock.
                        let mut calls = pending_calls.lock().await;
                        if let Some(tx) = calls.remove(&call_id) {
                            let _ = tx.send(Ok(response));
                        } else {
                            warn!("Received persistent response with no message for unknown call ID: {}", call_id);
                        }
                    }
                }
            }

            debug!("Persistent connection reader task exiting");
            let _ = closed_tx.send(true);
        })
    }

    /// Wrap a handler's result for sending, replacing it with an error if the
    /// frame would exceed [`MAX_FRAME_SIZE`]. Writing it anyway would make the
    /// daemon close this connection and drop every handler on it; refusing
    /// fails only this call, and tells the caller why.
    fn unary_response_within_limit(
        call_id: Vec<u8>,
        response: CallUnaryResponse,
        proto: &str,
    ) -> PersistentConnectionRequest {
        let wrap = |response| PersistentConnectionRequest {
            call_id: call_id.clone(),
            message: Some(persistent_connection_request::Message::UnaryResponse(
                response,
            )),
        };
        let msg = wrap(response);
        let len = msg.encoded_len();
        if len <= MAX_FRAME_SIZE {
            return msg;
        }
        warn!(
            proto,
            bytes = len,
            limit = MAX_FRAME_SIZE,
            "unary response exceeds the control-socket frame limit — answering with an error instead"
        );
        wrap(CallUnaryResponse {
            result: Some(crate::protocol::p2pd::call_unary_response::Result::Error(
                format!(
                    "response too large: {len} bytes exceeds the {MAX_FRAME_SIZE}-byte \
                     control-socket limit ({proto})"
                )
                .into_bytes(),
            )),
        })
    }

    /// Resolves once the connection has closed: the daemon went away or hung
    /// up, and every handler registered on this connection went with it.
    pub async fn closed(&self) {
        let mut rx = self.closed.clone();
        // A dropped sender (reader task aborted) also means closed.
        let _ = rx.wait_for(|closed| *closed).await;
    }

    /// Call a unary handler on a remote peer
    pub async fn call_unary(&self, peer_id: &[u8], proto: &str, data: &[u8]) -> Result<Vec<u8>> {
        let call_id = Uuid::new_v4();
        debug!(
            "Calling unary handler {} on peer (call_id: {})",
            proto, call_id
        );

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

        // Send request. An oversize frame is refused before it is written
        // (see `write_varint_message`); drop the pending entry with it.
        {
            let mut writer = self.writer.lock().await;
            if let Err(e) = Self::write_varint_message(&mut *writer, &request).await {
                self.pending_calls.lock().await.remove(&call_id);
                return Err(e);
            }
        }

        // Wait for response
        let response = rx
            .await
            .map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

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
                        Err(Error::Protocol(format!(
                            "Remote handler error: {}",
                            err_msg
                        )))
                    }
                    None => Err(Error::Protocol("Empty unary response".to_string())),
                }
            }
            Some(persistent_connection_response::Message::DaemonError(err)) => {
                let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
                // "stream reset" is a transient network event, not a bug — log at debug.
                // "protocols not supported" likewise says something true about the
                // peer rather than wrong about us: it is the ordinary answer when
                // asking whether a peer speaks a protocol, which is exactly what
                // `p2p probe` does on every call.
                if err_msg.contains("stream reset")
                    || err_msg.contains("early eof")
                    || err_msg.contains("protocols not supported")
                {
                    debug!("Daemon transient error for call {}: {}", call_id, err_msg);
                } else {
                    warn!("Daemon error for call {}: {}", call_id, err_msg);
                }
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
        let response = rx
            .await
            .map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

        // Check for errors
        if let Some(persistent_connection_response::Message::DaemonError(err)) = response.message {
            let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            return Err(Error::Protocol(format!(
                "Failed to add handler: {}",
                err_msg
            )));
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
        let response = rx
            .await
            .map_err(|_| Error::Protocol("Response channel closed".to_string()))??;

        // Check for errors
        if let Some(persistent_connection_response::Message::DaemonError(err)) = response.message {
            let err_msg = err.message.unwrap_or_else(|| "Unknown error".to_string());
            return Err(Error::Protocol(format!(
                "Failed to remove handler: {}",
                err_msg
            )));
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
                .map_err(Error::Io)?;

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
        reader.read_exact(&mut data).await.map_err(Error::Io)?;

        Ok(data)
    }

    /// Write a varint-length-prefixed message
    async fn write_varint_message<W: AsyncWrite + Unpin, M: ProstMessage>(
        writer: &mut W,
        msg: &M,
    ) -> Result<()> {
        // Refuse a frame the daemon would answer by closing the connection.
        let len = msg.encoded_len();
        if len > MAX_FRAME_SIZE {
            return Err(Error::Protocol(format!(
                "Message too large: {len} bytes exceeds the {MAX_FRAME_SIZE}-byte control-socket limit"
            )));
        }

        // Encode message
        let mut msg_buf = Vec::new();
        msg.encode(&mut msg_buf)
            .map_err(|e| Error::Protocol(format!("Failed to encode message: {}", e)))?;

        // Encode length as varint
        let mut length_buf = varint_encode::usize_buffer();
        let length_bytes = varint_encode::usize(msg_buf.len(), &mut length_buf);

        // Write length + message
        writer.write_all(length_bytes).await.map_err(Error::Io)?;
        writer.write_all(&msg_buf).await.map_err(Error::Io)?;
        writer.flush().await.map_err(Error::Io)?;

        Ok(())
    }

    /// Returns false if the background reader task has exited (connection is dead).
    pub fn is_alive(&self) -> bool {
        self.reader_task
            .as_ref()
            .map(|t| !t.is_finished())
            .unwrap_or(false)
    }
}

impl Drop for PersistentConnection {
    fn drop(&mut self) {
        if let Some(task) = self.reader_task.take() {
            task.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::p2pd::call_unary_response::Result as UnaryResult;
    use std::time::Duration;

    fn reply(data: Vec<u8>) -> CallUnaryResponse {
        CallUnaryResponse {
            result: Some(UnaryResult::Response(data)),
        }
    }

    fn unwrap_reply(msg: PersistentConnectionRequest) -> UnaryResult {
        match msg.message {
            Some(persistent_connection_request::Message::UnaryResponse(r)) => {
                r.result.expect("a result")
            }
            other => panic!("expected a unary response, got {other:?}"),
        }
    }

    /// #296: a reply the daemon would refuse is turned into an error for that
    /// one call. Written as is, it made the daemon close the connection and
    /// release every handler the shard had registered.
    #[test]
    fn an_oversize_reply_becomes_an_error_for_that_call() {
        // The payload alone fits; the framing around it does not.
        let msg = PersistentConnection::unary_response_within_limit(
            vec![7; 16],
            reply(vec![0; MAX_FRAME_SIZE]),
            "/kwaai/inference/1.0.0",
        );
        assert!(msg.encoded_len() <= MAX_FRAME_SIZE);
        assert_eq!(msg.call_id, vec![7; 16], "the error answers the same call");
        match unwrap_reply(msg) {
            UnaryResult::Error(e) => {
                let e = String::from_utf8(e).unwrap();
                assert!(e.contains("response too large"), "got: {e}");
                assert!(
                    e.contains("/kwaai/inference/1.0.0"),
                    "names the protocol: {e}"
                );
            }
            UnaryResult::Response(_) => panic!("an oversize reply must not be sent"),
        }
    }

    #[test]
    fn a_reply_within_the_limit_is_sent_unchanged() {
        let msg = PersistentConnection::unary_response_within_limit(
            vec![1; 16],
            reply(b"hidden states".to_vec()),
            "/kwaai/inference/1.0.0",
        );
        match unwrap_reply(msg) {
            UnaryResult::Response(d) => assert_eq!(d, b"hidden states"),
            UnaryResult::Error(e) => panic!("unexpected error: {}", String::from_utf8_lossy(&e)),
        }
    }

    /// Nothing over the limit reaches the socket, whichever path writes it.
    #[tokio::test]
    async fn an_oversize_frame_is_refused_before_anything_is_written() {
        let msg = PersistentConnectionRequest {
            call_id: vec![0; 16],
            message: Some(persistent_connection_request::Message::CallUnary(
                CallUnaryRequest {
                    peer: vec![],
                    proto: "p".into(),
                    data: vec![0; MAX_FRAME_SIZE],
                },
            )),
        };
        let mut sink: Vec<u8> = Vec::new();
        let err = PersistentConnection::write_varint_message(&mut sink, &msg)
            .await
            .expect_err("an oversize frame must be refused");
        assert!(err.to_string().contains("Message too large"), "got: {err}");
        assert!(sink.is_empty(), "no partial frame may be written");
    }

    /// `closed()` is what lets a handler-serving process notice that the
    /// daemon dropped it (#296).
    #[tokio::test]
    async fn closed_resolves_when_the_daemon_hangs_up() {
        let (ours, theirs) = tokio::io::duplex(1024);
        let (reader, writer) = tokio::io::split(ours);
        let conn = PersistentConnection::new(reader, writer);

        assert!(
            tokio::time::timeout(Duration::from_millis(100), conn.closed())
                .await
                .is_err(),
            "an open connection must not read as closed"
        );

        drop(theirs);
        tokio::time::timeout(Duration::from_secs(5), conn.closed())
            .await
            .expect("closed() must resolve once the daemon end goes away");
        assert!(!conn.is_alive());
    }
}
