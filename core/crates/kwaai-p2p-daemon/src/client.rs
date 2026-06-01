//! Client for communicating with the p2p daemon via IPC
//!
//! This module provides async I/O over:
//! - Windows: TCP socket (since Go daemon doesn't support named pipes in multiaddr format)
//! - Unix: Unix domain sockets

use crate::error::{Error, Result};
use crate::persistent::PersistentConnection;
use crate::protocol::p2pd::{
    request, ConnectRequest, DisconnectRequest, PeerInfo, Request, Response, StreamOpenRequest,
};
use bytes::{BufMut, BytesMut};
use prost::Message;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, trace};
use unsigned_varint::encode as varint_encode;

#[cfg(unix)]
use tokio::net::UnixStream;

/// Client for communicating with the p2p daemon
pub struct P2PClient {
    stream: DaemonStream,
    persistent: Arc<Mutex<Option<Arc<PersistentConnection>>>>,
    daemon_addr: String,
}

/// Platform-specific stream abstraction (private implementation detail)
enum DaemonStream {
    Tcp(TcpStream),
    #[cfg(unix)]
    UnixSocket(UnixStream),
}

/// Raw bidirectional stream to the p2p daemon, usable as a data channel after `stream_open_raw`.
///
/// Implements `AsyncRead + AsyncWrite + Unpin` so callers can use `tokio::io::split` to get
/// independent send and receive halves for concurrent I/O.
pub struct P2PStream(DaemonStream);

impl AsyncRead for P2PStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match &mut self.0 {
            DaemonStream::Tcp(tcp) => Pin::new(tcp).poll_read(cx, buf),
            #[cfg(unix)]
            DaemonStream::UnixSocket(sock) => Pin::new(sock).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for P2PStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match &mut self.0 {
            DaemonStream::Tcp(tcp) => Pin::new(tcp).poll_write(cx, buf),
            #[cfg(unix)]
            DaemonStream::UnixSocket(sock) => Pin::new(sock).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match &mut self.0 {
            DaemonStream::Tcp(tcp) => Pin::new(tcp).poll_flush(cx),
            #[cfg(unix)]
            DaemonStream::UnixSocket(sock) => Pin::new(sock).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match &mut self.0 {
            DaemonStream::Tcp(tcp) => Pin::new(tcp).poll_shutdown(cx),
            #[cfg(unix)]
            DaemonStream::UnixSocket(sock) => Pin::new(sock).poll_shutdown(cx),
        }
    }
}

impl P2PClient {
    /// Connect to the daemon at the given address
    ///
    /// - **Windows**: `addr` should be a multiaddr like `/ip4/127.0.0.1/tcp/5005`
    /// - **Unix**: `addr` should be a multiaddr like `/unix/tmp/p2pd.sock`
    pub async fn connect(addr: &str) -> Result<Self> {
        debug!("Connecting to daemon at: {}", addr);

        let stream = Self::connect_stream(addr).await?;

        debug!("Connected to daemon");

        Ok(Self {
            stream,
            persistent: Arc::new(Mutex::new(None)),
            daemon_addr: addr.to_string(),
        })
    }

    async fn connect_stream(addr: &str) -> Result<DaemonStream> {
        // Parse multiaddr to get the actual address
        // For simplicity, we'll support:
        // - /ip4/127.0.0.1/tcp/PORT -> TCP connection
        // - /unix/path -> Unix socket connection

        // Retry connection in case daemon is still starting
        let mut attempts = 0;
        let max_attempts = 10;

        if addr.starts_with("/ip4/") || addr.starts_with("/ip6/") {
            // Parse TCP address
            let parts: Vec<&str> = addr.split('/').collect();
            if parts.len() < 5 || parts[3] != "tcp" {
                return Err(Error::Connection(format!("Invalid multiaddr: {}", addr)));
            }
            let ip = parts[2];
            let port = parts[4];
            let tcp_addr = format!("{}:{}", ip, port);

            loop {
                match TcpStream::connect(&tcp_addr).await {
                    Ok(stream) => {
                        return Ok(DaemonStream::Tcp(stream));
                    }
                    Err(e) if attempts < max_attempts => {
                        attempts += 1;
                        debug!(
                            "TCP connection attempt {} failed: {}, retrying...",
                            attempts, e
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                    Err(e) => {
                        return Err(Error::Connection(format!(
                            "Failed to connect to TCP {}: {}",
                            tcp_addr, e
                        )));
                    }
                }
            }
        } else if addr.starts_with("/unix/") {
            #[cfg(unix)]
            {
                let socket_path = addr.strip_prefix("/unix/").unwrap(); // starts_with checked above

                loop {
                    match UnixStream::connect(socket_path).await {
                        Ok(stream) => {
                            return Ok(DaemonStream::UnixSocket(stream));
                        }
                        Err(e) if attempts < max_attempts => {
                            attempts += 1;
                            debug!(
                                "Unix socket connection attempt {} failed: {}, retrying...",
                                attempts, e
                            );
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                        Err(e) => {
                            return Err(Error::Connection(format!(
                                "Failed to connect to Unix socket {}: {}",
                                socket_path, e
                            )));
                        }
                    }
                }
            }
            #[cfg(not(unix))]
            {
                return Err(Error::Connection(
                    "Unix sockets not supported on this platform".to_string(),
                ));
            }
        } else {
            Err(Error::Connection(format!(
                "Unsupported multiaddr format: {}",
                addr
            )))
        }
    }

    /// Send a request to the daemon and receive a response
    pub async fn send_request(&mut self, request: Request) -> Result<Response> {
        // Debug: print request type
        debug!("Request type field = {}", request.r#type);

        // Serialize request
        let mut buf = BytesMut::new();
        request
            .encode(&mut buf)
            .map_err(|e| Error::Protocol(format!("Failed to encode request: {}", e)))?;

        debug!(
            "Encoded {} bytes - hex: {}",
            buf.len(),
            buf.iter()
                .take(20)
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );

        // Write framed message: [8-byte length][protobuf]
        self.write_framed(&buf).await?;

        // Read response
        let response_bytes = self.read_framed().await?;

        trace!("Received response ({} bytes)", response_bytes.len());

        // Decode response
        let response = Response::decode(&response_bytes[..])
            .map_err(|e| Error::Protocol(format!("Failed to decode response: {}", e)))?;

        // Check for error response
        if let Some(err) = &response.error {
            return Err(Error::Protocol(format!("Daemon error: {}", err.msg)));
        }

        Ok(response)
    }

    /// Write a framed message (varint length + payload)
    async fn write_framed(&mut self, payload: &[u8]) -> Result<()> {
        let len = payload.len();

        // Encode length as varint
        let mut len_buf = varint_encode::u64_buffer();
        let len_bytes = varint_encode::u64(len as u64, &mut len_buf);

        // Build frame: varint length + payload
        let mut frame = BytesMut::with_capacity(len_bytes.len() + payload.len());
        frame.put_slice(len_bytes);
        frame.put_slice(payload);

        match &mut self.stream {
            DaemonStream::Tcp(tcp) => {
                tcp.write_all(&frame).await?;
                tcp.flush().await?;
            }
            #[cfg(unix)]
            DaemonStream::UnixSocket(socket) => {
                socket.write_all(&frame).await?;
                socket.flush().await?;
            }
        }

        Ok(())
    }

    /// Read a framed message (varint length + payload)
    async fn read_framed(&mut self) -> Result<Vec<u8>> {
        // Read varint length prefix (up to 10 bytes for u64)
        let mut len_bytes = Vec::new();
        let mut byte = [0u8; 1];

        // Read varint byte by byte
        for _ in 0..10 {
            match &mut self.stream {
                DaemonStream::Tcp(tcp) => {
                    tcp.read_exact(&mut byte).await?;
                }
                #[cfg(unix)]
                DaemonStream::UnixSocket(socket) => {
                    socket.read_exact(&mut byte).await?;
                }
            }

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

        if len > 10 * 1024 * 1024 {
            // 10MB sanity check
            return Err(Error::Protocol(format!("Message too large: {} bytes", len)));
        }

        // Read payload
        let mut payload = vec![0u8; len];

        match &mut self.stream {
            DaemonStream::Tcp(tcp) => {
                tcp.read_exact(&mut payload).await?;
            }
            #[cfg(unix)]
            DaemonStream::UnixSocket(socket) => {
                socket.read_exact(&mut payload).await?;
            }
        }

        Ok(payload)
    }

    /// Send an IDENTIFY request to get our peer ID
    ///
    /// Returns the peer ID as hex-encoded string
    pub async fn identify(&mut self) -> Result<String> {
        let (peer_id, _) = self.identify_with_addrs().await?;
        Ok(peer_id)
    }

    /// Send an IDENTIFY request and return both the peer ID and observed multiaddrs.
    ///
    /// Returns `(peer_id_hex, addrs)` where `addrs` is the list of multiaddr bytes
    /// that p2pd has observed for itself (populated by the libp2p IDENTIFY protocol
    /// as peers connect and report our external address).
    pub async fn identify_with_addrs(&mut self) -> Result<(String, Vec<Vec<u8>>)> {
        let request = Request {
            r#type: request::Type::Identify as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if let Some(id) = response.identify {
            let peer_id = hex::encode(&id.id);
            Ok((peer_id, id.addrs))
        } else {
            Err(Error::InvalidResponse(
                "Expected IDENTIFY response".to_string(),
            ))
        }
    }

    /// Send an IDENTIFY request and return both the peer ID (hex) and the
    /// observed multiaddrs the daemon is reporting.
    ///
    /// Distinct from `identify()` which discards the addresses. Named
    /// `identify_full` rather than the more obvious `identify_with_addrs` to
    /// sidestep a duplicate-definition collision when this branch is merged
    /// alongside another in-flight branch (feature/public-ip) that
    /// independently added the latter name. Rename to `identify_with_addrs`
    /// once whichever branch lands second is updated.
    pub async fn identify_full(&mut self) -> Result<(String, Vec<Vec<u8>>)> {
        let request = Request {
            r#type: request::Type::Identify as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if let Some(id) = response.identify {
            let peer_id = hex::encode(&id.id);
            Ok((peer_id, id.addrs))
        } else {
            Err(Error::InvalidResponse(
                "Expected IDENTIFY response".to_string(),
            ))
        }
    }

    /// Connect to a peer using a multiaddr
    ///
    /// The multiaddr should be in the format: /ip4/1.2.3.4/tcp/1234/p2p/QmPeerID
    pub async fn connect_peer(&mut self, peer_multiaddr: &str) -> Result<()> {
        // Parse the multiaddr to extract peer ID and address
        let maddr: libp2p::Multiaddr = peer_multiaddr
            .parse()
            .map_err(|e| Error::Connection(format!("Invalid multiaddr: {}", e)))?;

        // Extract the destination peer ID from the multiaddr.
        // For a relay'd address like /ip4/.../p2p/<RELAY>/p2p-circuit/p2p/<DEST>,
        // the destination is the LAST /p2p/ component, not the first.
        let mut peer_id_bytes = None;
        for component in maddr.iter() {
            if let libp2p::multiaddr::Protocol::P2p(hash) = component {
                peer_id_bytes = Some(hash.to_bytes());
            }
        }

        let peer_id = peer_id_bytes
            .ok_or_else(|| Error::Connection("No peer ID found in multiaddr".to_string()))?;

        // Convert multiaddr to binary format for the daemon
        let addr_bytes = maddr.to_vec();

        let request = Request {
            r#type: request::Type::Connect as i32,
            connect: Some(ConnectRequest {
                peer: peer_id,
                addrs: vec![addr_bytes],
                timeout: Some(60),
            }),
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        debug!("Connecting to peer with multiaddr: {}", peer_multiaddr);
        let _response = self.send_request(request).await?;
        debug!("Connected successfully");
        Ok(())
    }

    /// Disconnect from a peer
    pub async fn disconnect_peer(&mut self, peer_id: &[u8]) -> Result<()> {
        let request = Request {
            r#type: request::Type::Disconnect as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: Some(DisconnectRequest {
                peer: peer_id.to_vec(),
            }),
            pubsub: None,
        };

        let _response = self.send_request(request).await?;
        Ok(())
    }

    /// List all currently connected peers
    ///
    /// Returns a list of PeerInfo containing peer IDs and their addresses.
    /// This is a fast local query to the daemon's connection table.
    pub async fn list_peers(&mut self) -> Result<Vec<PeerInfo>> {
        let request = Request {
            r#type: request::Type::ListPeers as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;
        Ok(response.peers)
    }

    /// Register a stream handler for the given protocols
    ///
    /// When a peer opens a stream with one of these protocols, the daemon will
    /// connect to our `listen_addr` and forward the stream.
    ///
    /// # Arguments
    /// * `listen_addr` - Local multiaddr where we'll accept connections (e.g., "/ip4/127.0.0.1/tcp/9000")
    /// * `protocols` - List of protocol names to handle (e.g., ["DHTProtocol.rpc_store"])
    pub async fn register_stream_handler(
        &mut self,
        listen_addr: &str,
        protocols: Vec<String>,
    ) -> Result<()> {
        use crate::protocol::p2pd::StreamHandlerRequest;

        // Parse multiaddr string and convert to binary format
        let maddr: libp2p::Multiaddr = listen_addr
            .parse()
            .map_err(|e| Error::Connection(format!("Invalid multiaddr: {}", e)))?;
        let addr_bytes = maddr.to_vec();

        let request = Request {
            r#type: 3, // STREAM_HANDLER = 3 from p2pd.proto
            stream_handler: Some(StreamHandlerRequest {
                addr: addr_bytes,
                proto: protocols.clone(),
                balanced: false, // Not using load balancing
            }),
            connect: None,
            stream_open: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
            remove_stream_handler: None,
        };

        debug!(
            "STREAM_HANDLER request: type={}, protocols={:?}",
            request.r#type, protocols
        );

        let response = self.send_request(request).await?;

        // Check for errors
        if let Some(error) = response.error {
            return Err(Error::Protocol(error.msg));
        }

        debug!("Stream handler registered for protocols: {:?}", protocols);
        Ok(())
    }

    /// Remove a previously registered stream handler
    pub async fn remove_stream_handler(
        &mut self,
        listen_addr: &str,
        protocols: Vec<String>,
    ) -> Result<()> {
        use crate::protocol::p2pd::{request, RemoveStreamHandlerRequest};

        // Parse multiaddr string and convert to binary format
        let maddr: libp2p::Multiaddr = listen_addr
            .parse()
            .map_err(|e| Error::Connection(format!("Invalid multiaddr: {}", e)))?;
        let addr_bytes = maddr.to_vec();

        let request = Request {
            r#type: request::Type::RemoveStreamHandler as i32,
            remove_stream_handler: Some(RemoveStreamHandlerRequest {
                addr: addr_bytes,
                proto: protocols.clone(),
            }),
            connect: None,
            stream_open: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
            stream_handler: None,
        };

        trace!("Removing stream handler for protocols: {:?}", protocols);

        let response = self.send_request(request).await?;

        if let Some(error) = response.error {
            return Err(Error::Protocol(error.msg));
        }

        debug!("Stream handler removed for protocols: {:?}", protocols);
        Ok(())
    }

    /// Open a new stream to a peer for a protocol
    ///
    /// Returns a TcpStream connected to the daemon-managed protocol stream
    /// that can be used to send requests and receive responses.
    pub async fn stream_open(
        &mut self,
        peer_id: &[u8],
        protocols: Vec<String>,
    ) -> Result<tokio::net::TcpStream> {
        let request = Request {
            r#type: request::Type::StreamOpen as i32,
            connect: None,
            stream_open: Some(StreamOpenRequest {
                peer: peer_id.to_vec(),
                proto: protocols.clone(),
                timeout: Some(60),
            }),
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        debug!("Opening stream for protocols: {:?}", protocols);
        let response = self.send_request(request).await?;

        // Extract StreamInfo from response
        let stream_info = response
            .stream_info
            .ok_or_else(|| Error::Protocol("No StreamInfo in response".to_string()))?;

        debug!(
            "StreamInfo received: proto={}, peer_len={}, addr_len={}",
            stream_info.proto,
            stream_info.peer.len(),
            stream_info.addr.len()
        );

        // Parse the multiaddr to extract TCP address
        // The addr field contains a binary multiaddr that we need to parse
        let maddr = libp2p::Multiaddr::try_from(stream_info.addr.clone())
            .map_err(|e| Error::Protocol(format!("Invalid multiaddr in StreamInfo: {}", e)))?;

        // Extract IP and port from multiaddr
        // Format is typically: /ip4/127.0.0.1/tcp/<port>
        let mut ip_addr = None;
        let mut port = None;

        for component in maddr.iter() {
            match component {
                libp2p::multiaddr::Protocol::Ip4(addr) => {
                    ip_addr = Some(std::net::IpAddr::V4(addr));
                }
                libp2p::multiaddr::Protocol::Ip6(addr) => {
                    ip_addr = Some(std::net::IpAddr::V6(addr));
                }
                libp2p::multiaddr::Protocol::Tcp(p) => {
                    port = Some(p);
                }
                _ => {}
            }
        }

        let ip =
            ip_addr.ok_or_else(|| Error::Protocol("No IP address in multiaddr".to_string()))?;
        let port = port.ok_or_else(|| Error::Protocol("No TCP port in multiaddr".to_string()))?;

        let socket_addr = std::net::SocketAddr::new(ip, port);
        debug!("Connecting to daemon stream at: {}", socket_addr);

        // Connect to the daemon's forwarded stream
        let stream = tokio::net::TcpStream::connect(socket_addr)
            .await
            .map_err(Error::Io)?;

        debug!("Connected to daemon stream");
        Ok(stream)
    }

    /// Open a stream to a peer and return the **daemon socket itself** as the data channel.
    ///
    /// The go-libp2p-daemon protocol works as follows for `STREAM_OPEN`:
    /// 1. Client sends `StreamOpen` request on the control socket.
    /// 2. Daemon opens the libp2p stream, sends back `StreamInfo` on the same socket.
    /// 3. Daemon calls `doStreamPipe(socket, libp2p_stream)` — raw bytes of the libp2p stream
    ///    now flow directly on the daemon socket (no separate TCP connection needed).
    ///
    /// This method consumes `self` and returns the socket as a [`P2PStream`].
    /// The `StreamInfo.addr` field (the remote peer's multiaddr) is intentionally ignored —
    /// connecting to it would reach the relay node, not a local proxy.
    pub async fn stream_open_raw(
        mut self,
        peer_id: &[u8],
        protocols: Vec<String>,
    ) -> Result<P2PStream> {
        let request = Request {
            r#type: request::Type::StreamOpen as i32,
            connect: None,
            stream_open: Some(StreamOpenRequest {
                peer: peer_id.to_vec(),
                proto: protocols.clone(),
                timeout: Some(60),
            }),
            stream_handler: None,
            remove_stream_handler: None,
            dht: None,
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        debug!("Opening raw stream for protocols: {:?}", protocols);
        let response = self.send_request(request).await?;

        let stream_info = response
            .stream_info
            .ok_or_else(|| Error::Protocol("No StreamInfo in response".to_string()))?;

        debug!(
            "StreamInfo received (raw mode): proto={}, peer_len={} — daemon now in pipe mode",
            stream_info.proto,
            stream_info.peer.len(),
        );

        // The daemon has set up doStreamPipe on its end: from this point raw libp2p stream
        // bytes flow directly on the daemon socket. Return it as the data channel.
        Ok(P2PStream(self.stream))
    }

    // ===== Persistent Connection / Unary Handler Support =====

    /// Get or create a persistent connection for unary RPC calls.
    /// If the cached connection's reader task has exited (stream reset / early eof),
    /// it is evicted and a fresh connection is established automatically.
    async fn get_persistent_connection(&self) -> Result<Arc<PersistentConnection>> {
        let mut guard = self.persistent.lock().await;

        if let Some(conn) = guard.as_ref() {
            if conn.is_alive() {
                return Ok(conn.clone());
            }
            // Reader task has exited — evict and reconnect.
            debug!("Persistent connection is dead, reconnecting");
            *guard = None;
        }

        // Need to create persistent connection
        debug!("Upgrading to persistent connection for unary handlers");

        // Open a new connection to the daemon
        let stream = Self::connect_stream(&self.daemon_addr).await?;

        // Send PERSISTENT_CONN_UPGRADE request
        let (mut reader, mut writer): (
            Box<dyn tokio::io::AsyncRead + Unpin + Send>,
            Box<dyn tokio::io::AsyncWrite + Unpin + Send>,
        ) = match stream {
            DaemonStream::Tcp(tcp) => {
                let (r, w) = tcp.into_split();
                (
                    Box::new(r) as Box<dyn tokio::io::AsyncRead + Unpin + Send>,
                    Box::new(w) as Box<dyn tokio::io::AsyncWrite + Unpin + Send>,
                )
            }
            #[cfg(unix)]
            DaemonStream::UnixSocket(sock) => {
                let (r, w) = sock.into_split();
                (
                    Box::new(r) as Box<dyn tokio::io::AsyncRead + Unpin + Send>,
                    Box::new(w) as Box<dyn tokio::io::AsyncWrite + Unpin + Send>,
                )
            }
        };

        // Build and send upgrade request
        let upgrade_request = Request {
            r#type: request::Type::PersistentConnUpgrade as i32,
            ..Default::default()
        };

        // Encode request
        let mut buf = BytesMut::new();
        upgrade_request
            .encode(&mut buf)
            .map_err(|e| Error::Protocol(format!("Failed to encode upgrade request: {}", e)))?;

        // Write varint-framed message
        let len = buf.len();
        let mut len_buf = varint_encode::u64_buffer();
        let len_bytes = varint_encode::u64(len as u64, &mut len_buf);

        let mut frame = BytesMut::with_capacity(len_bytes.len() + buf.len());
        frame.put_slice(len_bytes);
        frame.put_slice(&buf);

        writer.write_all(&frame).await.map_err(Error::Io)?;
        writer.flush().await.map_err(Error::Io)?;

        // Read OK response
        let response_bytes = Self::read_varint_framed_static(&mut reader).await?;
        let response = Response::decode(&response_bytes[..])
            .map_err(|e| Error::Protocol(format!("Failed to decode upgrade response: {}", e)))?;

        if let Some(err) = &response.error {
            return Err(Error::Protocol(format!(
                "Failed to upgrade connection: {}",
                err.msg
            )));
        }

        debug!("Successfully upgraded to persistent connection");

        // Create persistent connection
        let conn = Arc::new(PersistentConnection::new(reader, writer));
        *guard = Some(conn.clone());

        Ok(conn)
    }

    /// Helper to read varint-framed messages (static version for upgrading)
    async fn read_varint_framed_static<R: AsyncReadExt + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
        // Read varint length prefix
        let mut len_bytes = Vec::new();
        let mut byte = [0u8; 1];

        for _ in 0..10 {
            reader.read_exact(&mut byte).await.map_err(Error::Io)?;
            len_bytes.push(byte[0]);
            if byte[0] & 0x80 == 0 {
                break;
            }
        }

        let (len, _) = unsigned_varint::decode::u64(&len_bytes)
            .map_err(|e| Error::Protocol(format!("Failed to decode varint length: {:?}", e)))?;

        // Read message payload
        let mut payload = vec![0u8; len as usize];
        reader.read_exact(&mut payload).await.map_err(Error::Io)?;

        Ok(payload)
    }

    /// Call a unary handler on a remote peer
    ///
    /// This is the primary method for Hivemind DHT RPC calls.
    /// The daemon handles all protocol negotiation.
    ///
    /// # Arguments
    /// * `peer_id` - The peer ID bytes to call
    /// * `proto` - The protocol name (e.g., "DHTProtocol.rpc_store")
    /// * `data` - The request data (should be Hivemind-encoded: [8-byte len][marker][protobuf])
    ///
    /// # Returns
    /// The response data from the remote peer
    pub async fn call_unary_handler(
        &self,
        peer_id: &[u8],
        proto: &str,
        data: &[u8],
    ) -> Result<Vec<u8>> {
        let conn = self.get_persistent_connection().await?;
        conn.call_unary(peer_id, proto, data).await
    }

    /// Register a unary handler to receive incoming RPC requests
    ///
    /// # Arguments
    /// * `proto` - The protocol name to handle (e.g., "DHTProtocol.rpc_store")
    /// * `handler` - Async function that processes requests and returns responses
    /// * `balanced` - Whether to load-balance across multiple handlers
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
        let conn = self.get_persistent_connection().await?;
        conn.add_unary_handler(proto, handler, balanced).await
    }

    /// Remove a previously registered unary handler
    pub async fn remove_unary_handler(&self, proto: &str) -> Result<()> {
        let conn = self.get_persistent_connection().await?;
        conn.remove_unary_handler(proto).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Buf;

    #[test]
    fn test_frame_encoding() {
        let payload = b"test payload";
        let len = payload.len() as u64;

        let mut frame = BytesMut::new();
        frame.put_u64(len);
        frame.put_slice(payload);

        assert_eq!(frame.len(), 8 + payload.len());
        assert_eq!(&frame[8..], payload);

        let mut cursor = &frame[..];
        let decoded_len = cursor.get_u64();
        assert_eq!(decoded_len, len);
    }
}
