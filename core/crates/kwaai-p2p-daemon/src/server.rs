//! Node-hosted p2pd control-socket **server**.
//!
//! This is the mirror image of [`crate::client`]: the same protobuf control
//! protocol, served by the node itself and translated into
//! [`kwaai_p2p::NetworkHandle`] calls instead of into a Go process. External
//! processes (the map's DHT crawler, `shard serve`, `storage serve`, `rag`,
//! `p2p`/`status`, inference-mux, …) attach to the socket and keep acting as
//! the node's peer identity.
//!
//! # Why byte-for-byte compatibility, not a nicer protocol
//!
//! The socket is a multi-process service bus with ~15 client call sites across
//! several binaries, some of which (`shard serve`, `storage serve`) register
//! *inbound* handlers and therefore serve network traffic on the node's behalf.
//! Changing the protocol would mean changing all of them atomically. Keeping it
//! means the daemon can be removed underneath them without a single client edit.
//!
//! The authoritative reference for every response shape here is the Go daemon
//! (`conn.go` / `persistent_stream.go` in `go-libp2p-daemon@v0.5.0.hivemind1`),
//! not the proto file: several behaviours are load-bearing but invisible in the
//! schema. In particular:
//!
//! - **An `AddUnaryHandler`/`RemoveUnaryHandler` ACK is a
//!   `PersistentConnectionResponse` carrying only a `callId` and *no* `message`**
//!   (Go's `okUnaryCallResponse`). `persistent.rs` decodes that `None` arm as the
//!   success ACK, so emitting a populated message here would break registration.
//! - **`IdentifyResponse.id` is raw peer-ID bytes** (`[]byte(d.ID())`), which the
//!   client hex-encodes; `addrs` are **binary multiaddrs** (`addr.Bytes()`), not
//!   strings. The harness round-trips these through `PeerId::from_bytes` and
//!   `Multiaddr::try_from`.
//! - **Errors are typed by verb**: the simple request/response verbs answer
//!   `Response{type: ERROR, error: {msg}}`, while persistent-connection verbs
//!   answer `PersistentConnectionResponse{callId, daemonError: {message}}`.
//!   Clients discriminate on exactly that.
//! - **On the inbound-dispatch path the `callUnary.peer` field is rewritten to
//!   the *caller's* peer ID** (`persistent_stream.go:298`). Handlers that echo
//!   identity depend on it.
//!
//! # Concurrency model
//!
//! One tokio task per accepted connection. A connection is in one of three
//! modes, and the last two are **terminal** — once entered, the connection never
//! returns to framing:
//!
//! - **request/response** — read a `Request`, write a `Response`, repeat. Simple
//!   verbs are handled here.
//! - **persistent** — entered by `PERSISTENT_CONN_UPGRADE`. Frames flow in both
//!   directions concurrently and are correlated by `callId`, so the read loop
//!   must never block on a call: each request is spawned onto its own task
//!   (matching Go's `go d.handlePersistentConnRequest(...)`) and all writes
//!   funnel through a shared `Mutex<writer>` so frames cannot interleave
//!   mid-message.
//! - **pipe** — entered by a successful `STREAM_OPEN`. The socket stops being a
//!   frame channel and becomes the raw data channel for a libp2p stream: its two
//!   halves are reunited and handed to `copy_bidirectional`. This is why the
//!   connection owns a *concrete* socket type rather than boxed trait objects —
//!   see [`ClientSocket`].
//!
//! ## Handler ownership and call-ID isolation
//!
//! Two independent clients may pick the same UUID for concurrent calls — UUIDv4
//! collisions are unlikely but the failure mode (a response delivered to the
//! wrong process) is silent and severe, so correlation state is **per
//! connection**, never global. Only the protocol → owning-connection map is
//! global, because a libp2p protocol can only be served by one handler.
//!
//! Every handler a connection registers is tracked — unary ones in
//! [`ConnState::handlers`], raw-stream ones in [`ConnState::stream_handlers`] —
//! and deregistered from both the global map and the swarm when the connection
//! ends, for any reason including a client crash — otherwise a crashed
//! `storage serve` leaves a protocol advertised that nothing will answer,
//! turning a clean negotiation refusal into a hang. For raw-stream handlers
//! this is a deliberate divergence from Go, whose `d.handlers` map is
//! process-global; see [`ConnState::do_stream_handler`].
//!
//! # Scope
//!
//! Served: IDENTIFY, CONNECT, DISCONNECT, LIST_PEERS, DHT FIND_PEER, the full
//! persistent-connection unary sub-protocol, and pipe mode (STREAM_OPEN,
//! STREAM_HANDLER, REMOVE_STREAM_HANDLER). Stubbed with the Go daemon's error
//! shape: the remaining DHT verbs, PUBSUB and CONNMANAGER (never implemented by
//! this codebase's client either).

use std::collections::HashMap;
#[cfg(unix)]
use std::path::PathBuf;
use std::sync::Arc;

use prost::Message as ProstMessage;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::sync::{oneshot, Mutex};
use tracing::{debug, info, trace, warn};
use uuid::Uuid;

use kwaai_p2p::{Multiaddr, NetworkHandle, PeerId};

use crate::error::{Error, Result};
use crate::protocol::p2pd::{
    dht_request, persistent_connection_request, persistent_connection_response, request, response,
    CallUnaryResponse, DhtResponse, ErrorResponse, IdentifyResponse, PeerInfo,
    PersistentConnectionRequest, PersistentConnectionResponse, Request, Response,
};

/// Inbound frame cap, matching the Go daemon's `persistentConnMsgMaxSize`.
const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// Error text for verbs this server does not implement.
///
/// The Go daemon answers unknown DHT verbs with `"not supported"`
/// (`dht.go:81`), so anything a client might plausibly probe reuses that
/// wording rather than inventing a new string clients cannot match on.
const NOT_SUPPORTED: &str = "not supported";

// ============================================================================
// Server
// ============================================================================

/// Serves the p2pd control protocol for a running node.
///
/// Construct with [`ControlServer::bind`], then drive with
/// [`ControlServer::run`] (usually on its own task). Dropping the server closes
/// the listener; in-flight connection tasks finish on their own.
pub struct ControlServer {
    listener: Listener,
    shared: Arc<Shared>,
}

/// State shared by every connection task.
struct Shared {
    handle: NetworkHandle,
    /// Protocol → the connection currently serving it.
    ///
    /// Global because libp2p allows exactly one handler per protocol. The Go
    /// daemon keeps a round-robin list here to support `balanced` handlers; we
    /// keep a single owner and refuse a second registration, which is what the
    /// Go daemon also does whenever `balanced` is false — the only mode any
    /// call site in this codebase uses. See [`ConnState::add_unary_handler`].
    handler_owners: Mutex<HashMap<String, u64>>,
    /// Raw-stream protocol → the connection currently serving it.
    ///
    /// A second map rather than a shared one because the two namespaces are
    /// independent on the swarm: `unary` and `raw_stream` keep separate inbound
    /// protocol sets, so the same name could in principle be a unary handler for
    /// one client and a stream handler for another. Merging the maps would
    /// impose a coupling the layer below does not have.
    stream_handler_owners: Mutex<HashMap<String, u64>>,
}

/// The bound socket. Unix-domain today; a TCP arm mirrors the Windows path the
/// client already speaks (`/ip4/127.0.0.1/tcp/5005`).
enum Listener {
    #[cfg(unix)]
    Unix {
        listener: tokio::net::UnixListener,
        /// Removed on drop so a restart is not blocked by a stale socket file.
        path: PathBuf,
    },
    Tcp(tokio::net::TcpListener),
}

impl ControlServer {
    /// Bind the control socket at a p2pd-style multiaddr.
    ///
    /// Accepts the two forms [`crate::client::P2PClient::connect`] parses:
    /// `/unix/<path>` and `/ip4/<addr>/tcp/<port>`. A stale socket file at
    /// `path` is removed first — the node owns this path, and a leftover file
    /// from a hard kill would otherwise make every restart fail with
    /// `EADDRINUSE`.
    pub async fn bind(addr: &str, handle: NetworkHandle) -> Result<Self> {
        let listener = Self::bind_listener(addr).await?;
        info!(%addr, "control socket listening");
        Ok(Self {
            listener,
            shared: Arc::new(Shared {
                handle,
                handler_owners: Mutex::new(HashMap::new()),
                stream_handler_owners: Mutex::new(HashMap::new()),
            }),
        })
    }

    async fn bind_listener(addr: &str) -> Result<Listener> {
        if let Some(path) = addr.strip_prefix("/unix/") {
            #[cfg(unix)]
            {
                let path = PathBuf::from(path);
                if path.exists() {
                    let _ = std::fs::remove_file(&path);
                }
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let listener = tokio::net::UnixListener::bind(&path).map_err(Error::Io)?;
                return Ok(Listener::Unix { listener, path });
            }
            #[cfg(not(unix))]
            {
                let _ = path;
                return Err(Error::Connection(
                    "Unix sockets not supported on this platform".to_string(),
                ));
            }
        }

        if addr.starts_with("/ip4/") || addr.starts_with("/ip6/") {
            let parts: Vec<&str> = addr.split('/').collect();
            if parts.len() < 5 || parts[3] != "tcp" {
                return Err(Error::Connection(format!("Invalid multiaddr: {addr}")));
            }
            let socket_addr = format!("{}:{}", parts[2], parts[4]);
            let listener = tokio::net::TcpListener::bind(&socket_addr)
                .await
                .map_err(Error::Io)?;
            return Ok(Listener::Tcp(listener));
        }

        Err(Error::Connection(format!(
            "Unsupported multiaddr format: {addr}"
        )))
    }

    /// The address the server actually bound, with the ephemeral port resolved.
    ///
    /// Tests bind `/ip4/127.0.0.1/tcp/0` and need the real port back.
    pub fn local_addr(&self) -> Result<String> {
        match &self.listener {
            #[cfg(unix)]
            Listener::Unix { path, .. } => Ok(format!("/unix/{}", path.display())),
            Listener::Tcp(l) => {
                let a = l.local_addr().map_err(Error::Io)?;
                Ok(format!("/ip4/{}/tcp/{}", a.ip(), a.port()))
            }
        }
    }

    /// Accept connections until the task is cancelled or the listener fails.
    ///
    /// Each connection is served on its own task, so a client that stalls
    /// mid-frame delays only itself. Accept errors are logged and retried
    /// rather than fatal: a single `EMFILE` must not take the node's IPC
    /// surface down permanently.
    pub async fn run(self) {
        let shared = self.shared;
        let mut next_conn_id: u64 = 0;

        loop {
            next_conn_id += 1;
            let conn_id = next_conn_id;
            let shared = Arc::clone(&shared);

            match &self.listener {
                #[cfg(unix)]
                Listener::Unix { listener, .. } => match listener.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(serve_connection(
                            conn_id,
                            shared,
                            ClientSocket::Unix(stream),
                        ));
                    }
                    Err(e) => {
                        warn!(error = %e, "control socket accept failed");
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                },
                Listener::Tcp(listener) => match listener.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(serve_connection(conn_id, shared, ClientSocket::Tcp(stream)));
                    }
                    Err(e) => {
                        warn!(error = %e, "control socket accept failed");
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                },
            }
        }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Listener::Unix { path, .. } = self {
            let _ = std::fs::remove_file(path);
        }
    }
}

// ============================================================================
// The client socket
// ============================================================================

/// An accepted control-socket connection, before it is split for framing.
///
/// This type exists for **pipe mode**. In request/response and persistent modes
/// the connection is read on one task and written from several, so it is split
/// into halves and the write half is shared behind a mutex. `STREAM_OPEN` then
/// needs the opposite: the two halves rejoined into one duplex stream that
/// `copy_bidirectional` can own outright.
///
/// Both `tokio::net::UnixStream` and `TcpStream` support `reunite`, but only on
/// their *concrete* half types — a `Box<dyn AsyncWrite>` cannot be rejoined with
/// anything. Keeping the concrete type here, and splitting only inside
/// [`ConnState`], is what makes the handoff possible without a second socket or
/// an intermediate copy.
enum ClientSocket {
    #[cfg(unix)]
    Unix(tokio::net::UnixStream),
    Tcp(tokio::net::TcpStream),
}

/// The read half of a [`ClientSocket`], kept concrete so it can be reunited.
enum SocketReader {
    #[cfg(unix)]
    Unix(tokio::net::unix::OwnedReadHalf),
    Tcp(tokio::net::tcp::OwnedReadHalf),
}

/// The write half of a [`ClientSocket`], kept concrete for the same reason.
enum SocketWriter {
    #[cfg(unix)]
    Unix(tokio::net::unix::OwnedWriteHalf),
    Tcp(tokio::net::tcp::OwnedWriteHalf),
}

impl ClientSocket {
    fn split(self) -> (SocketReader, SocketWriter) {
        match self {
            #[cfg(unix)]
            ClientSocket::Unix(s) => {
                let (r, w) = s.into_split();
                (SocketReader::Unix(r), SocketWriter::Unix(w))
            }
            ClientSocket::Tcp(s) => {
                let (r, w) = s.into_split();
                (SocketReader::Tcp(r), SocketWriter::Tcp(w))
            }
        }
    }
}

impl SocketReader {
    /// Rejoin this reader with `writer` into the original duplex stream.
    ///
    /// The halves always came from the same socket — they are only ever split
    /// by [`ClientSocket::split`] and stored together in one [`ConnState`] — so
    /// a mismatch is a bug in this file rather than a runtime condition, and the
    /// `ReuniteError` is reported as a protocol error rather than propagated.
    fn reunite(self, writer: SocketWriter) -> Result<ClientSocket> {
        match (self, writer) {
            #[cfg(unix)]
            (SocketReader::Unix(r), SocketWriter::Unix(w)) => r
                .reunite(w)
                .map(ClientSocket::Unix)
                .map_err(|e| Error::Protocol(format!("socket halves do not match: {e}"))),
            (SocketReader::Tcp(r), SocketWriter::Tcp(w)) => r
                .reunite(w)
                .map(ClientSocket::Tcp)
                .map_err(|e| Error::Protocol(format!("socket halves do not match: {e}"))),
            #[cfg(unix)]
            _ => Err(Error::Protocol(
                "socket halves are of different transports".to_string(),
            )),
        }
    }
}

impl AsyncRead for ClientSocket {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            ClientSocket::Unix(s) => std::pin::Pin::new(s).poll_read(cx, buf),
            ClientSocket::Tcp(s) => std::pin::Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for ClientSocket {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match self.get_mut() {
            #[cfg(unix)]
            ClientSocket::Unix(s) => std::pin::Pin::new(s).poll_write(cx, buf),
            ClientSocket::Tcp(s) => std::pin::Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            ClientSocket::Unix(s) => std::pin::Pin::new(s).poll_flush(cx),
            ClientSocket::Tcp(s) => std::pin::Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            ClientSocket::Unix(s) => std::pin::Pin::new(s).poll_shutdown(cx),
            ClientSocket::Tcp(s) => std::pin::Pin::new(s).poll_shutdown(cx),
        }
    }
}

impl AsyncRead for SocketReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            SocketReader::Unix(r) => std::pin::Pin::new(r).poll_read(cx, buf),
            SocketReader::Tcp(r) => std::pin::Pin::new(r).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for SocketWriter {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        match self.get_mut() {
            #[cfg(unix)]
            SocketWriter::Unix(w) => std::pin::Pin::new(w).poll_write(cx, buf),
            SocketWriter::Tcp(w) => std::pin::Pin::new(w).poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            SocketWriter::Unix(w) => std::pin::Pin::new(w).poll_flush(cx),
            SocketWriter::Tcp(w) => std::pin::Pin::new(w).poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            #[cfg(unix)]
            SocketWriter::Unix(w) => std::pin::Pin::new(w).poll_shutdown(cx),
            SocketWriter::Tcp(w) => std::pin::Pin::new(w).poll_shutdown(cx),
        }
    }
}

// ============================================================================
// Per-connection state
// ============================================================================

/// One socket client.
struct ConnState {
    /// Distinguishes connections in the global handler map and in logs. Also
    /// what makes call-ID correlation per-connection: `waiters` lives here, so
    /// two clients using the same UUID never collide.
    id: u64,
    shared: Arc<Shared>,
    /// Serialises writes so concurrently-spawned request tasks cannot interleave
    /// halves of two frames on the wire.
    ///
    /// `Option` because pipe mode *takes* the writer back out: once
    /// `STREAM_OPEN` succeeds the connection stops being a frame channel
    /// entirely, and the write half must be reunited with the read half to
    /// become one duplex stream. After the take, any late frame write finds
    /// `None` and fails — which is correct, since there is no longer a framed
    /// protocol on this socket to write into.
    writer: Arc<Mutex<Option<SocketWriter>>>,
    /// Unary protocols this connection registered, for teardown on disconnect.
    handlers: Mutex<Vec<String>>,
    /// Raw-stream protocols this connection registered, for the same teardown.
    ///
    /// Tracked separately from `handlers` because they live in a different
    /// namespace on the swarm (`raw_stream` vs `unary`) and are released
    /// through different handle calls.
    stream_handlers: Mutex<Vec<String>>,
    /// Inbound calls awaiting this client's `unaryResponse`, keyed by call ID.
    ///
    /// Per connection by construction — see the module docs on call-ID
    /// isolation. `Arc` because each registered handler's dispatcher closure
    /// outlives the borrow that created it.
    waiters: Arc<Mutex<HashMap<Uuid, oneshot::Sender<CallUnaryResponse>>>>,
}

/// Serve one accepted connection to completion, then clean up its handlers.
async fn serve_connection(id: u64, shared: Arc<Shared>, socket: ClientSocket) {
    let (reader, writer) = socket.split();
    let state = Arc::new(ConnState {
        id,
        shared,
        writer: Arc::new(Mutex::new(Some(writer))),
        handlers: Mutex::new(Vec::new()),
        stream_handlers: Mutex::new(Vec::new()),
        waiters: Arc::new(Mutex::new(HashMap::new())),
    });

    debug!(conn = id, "control connection opened");
    if let Err(e) = run_connection(Arc::clone(&state), reader).await {
        debug!(conn = id, error = %e, "control connection ended");
    }

    // Deregistration runs on *every* exit path — clean close, decode error,
    // client crash, and the end of a pipe-mode relay — which is the whole
    // point: a dead client must not leave the node advertising a protocol it
    // can no longer serve.
    state.deregister_all().await;
    debug!(conn = id, "control connection closed");
}

/// Read frames until EOF. Returns `Ok(())` on a clean client close.
///
/// Two verbs are **terminal**: after them the connection never returns to
/// request/response framing, so both `return` out of the loop rather than
/// continuing it.
///
/// - `PERSISTENT_CONN_UPGRADE` switches to the persistent frame protocol
///   (Go's `handleConn` does the same),
/// - `STREAM_OPEN`, on success, switches to **pipe mode**: the socket stops
///   carrying frames at all and becomes the raw data channel for a libp2p
///   stream (`conn.go:59-73` — write the response, `doStreamPipe`, `return`).
async fn run_connection(state: Arc<ConnState>, mut reader: SocketReader) -> Result<()> {
    loop {
        let bytes = match read_frame(&mut reader).await {
            Ok(b) => b,
            // EOF is how a well-behaved client disconnects.
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(e),
        };

        let request = Request::decode(&bytes[..])
            .map_err(|e| Error::Protocol(format!("Failed to decode request: {e}")))?;

        trace!(conn = state.id, r#type = request.r#type, "control request");

        if request.r#type == request::Type::PersistentConnUpgrade as i32 {
            state.write_response(ok_response()).await?;
            return run_persistent(state, reader).await;
        }

        if request.r#type == request::Type::StreamOpen as i32 {
            // The stream must be opened *before* the response is written: the
            // response carries the negotiated protocol and the remote's
            // address, and a failure has to surface as `Response{ERROR}` on a
            // socket that stays in framing mode.
            match state.do_stream_open(request).await {
                Ok((response, stream)) => {
                    // Go resets the stream if the response cannot be written —
                    // the client will never know the stream exists, so leaving
                    // it open would leak it on both ends.
                    if let Err(e) = state.write_response(response).await {
                        debug!(conn = state.id, error = %e, "stream open response failed; resetting");
                        drop(stream);
                        return Err(e);
                    }
                    return state.enter_pipe_mode(reader, stream).await;
                }
                Err(response) => {
                    state.write_response(response).await?;
                    continue;
                }
            }
        }

        let response = state.handle_simple_request(request).await;
        state.write_response(response).await?;
    }
}

// ============================================================================
// Simple request/response verbs
// ============================================================================

impl ConnState {
    /// Dispatch one non-persistent verb. Never returns `Err` for a
    /// protocol-level failure — those become `Response{ERROR}`, matching Go,
    /// which only drops the connection on I/O trouble.
    async fn handle_simple_request(&self, request: Request) -> Response {
        let Some(kind) = request::Type::try_from(request.r#type).ok() else {
            // Go closes here; answering leaves a buggy client informed rather
            // than hanging.
            return error_response(format!("Unexpected request type {}", request.r#type));
        };

        match kind {
            request::Type::Identify => self.do_identify().await,
            request::Type::Connect => self.do_connect(request).await,
            request::Type::Disconnect => self.do_disconnect(request).await,
            request::Type::ListPeers => self.do_list_peers().await,
            request::Type::Dht => self.do_dht(request).await,

            request::Type::StreamHandler => self.do_stream_handler(request).await,
            request::Type::RemoveStreamHandler => self.do_remove_stream_handler(request).await,

            // Handled in `run_connection`, because a successful STREAM_OPEN is
            // terminal for the connection and this method can only return a
            // response. Unreachable in practice.
            request::Type::StreamOpen => error_response("Unexpected request type".to_string()),

            // Never called by this codebase's client.
            request::Type::Pubsub | request::Type::Connmanager => {
                error_response(NOT_SUPPORTED.to_string())
            }

            // Handled before dispatch; unreachable in practice.
            request::Type::PersistentConnUpgrade => {
                error_response("Unexpected request type".to_string())
            }
        }
    }

    /// `IDENTIFY` — our peer ID and listen addresses.
    ///
    /// Go returns `d.Addrs()`, the host's full advertised address set. We return
    /// the swarm's listen addresses, the same set pre-NAT-traversal. Byte
    /// format matters more than membership here: raw peer
    /// ID bytes and binary multiaddrs, because `node.rs`'s self-discovery path
    /// parses both.
    async fn do_identify(&self) -> Response {
        let peer_id = self.shared.handle.peer_id();
        let addrs = match self.shared.handle.listen_addrs().await {
            Ok(addrs) => addrs,
            Err(e) => return error_response(e.to_string()),
        };

        let mut res = ok_response();
        res.identify = Some(IdentifyResponse {
            id: peer_id.to_bytes(),
            addrs: addrs.into_iter().map(|a| a.to_vec()).collect(),
        });
        res
    }

    /// `CONNECT` — dial a peer at the supplied addresses.
    ///
    /// The client sends one multiaddr that already carries `/p2p/<id>`;
    /// `NetworkHandle::connect_peer` requires that component, so an address
    /// without it is completed from the request's `peer` field rather than
    /// rejected — Go accepts the split form (`peer.AddrInfo{ID, Addrs}`) and
    /// some call sites rely on it.
    async fn do_connect(&self, request: Request) -> Response {
        let Some(connect) = request.connect else {
            return error_response("Malformed request; missing parameters".to_string());
        };

        let peer = match PeerId::from_bytes(&connect.peer) {
            Ok(p) => p,
            Err(e) => return error_response(format!("invalid peer id: {e}")),
        };

        if connect.addrs.is_empty() {
            return error_response("Malformed request; missing parameters".to_string());
        }

        let mut last_error = String::from("no addresses could be dialed");
        for raw in &connect.addrs {
            let addr = match Multiaddr::try_from(raw.clone()) {
                Ok(a) => a,
                Err(e) => {
                    last_error = format!("invalid multiaddr: {e}");
                    continue;
                }
            };
            // Ensure the /p2p/ component the handle needs is present.
            let dial = if addr
                .iter()
                .any(|p| matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
            {
                addr
            } else {
                addr.with(libp2p::multiaddr::Protocol::P2p(peer))
            };

            match self.shared.handle.connect_peer(&dial.to_string()).await {
                Ok(_) => return ok_response(),
                Err(e) => last_error = e.to_string(),
            }
        }

        error_response(last_error)
    }

    /// `DISCONNECT` — close all connections to a peer.
    async fn do_disconnect(&self, request: Request) -> Response {
        let Some(disconnect) = request.disconnect else {
            return error_response("Malformed request; missing parameters".to_string());
        };
        let peer = match PeerId::from_bytes(&disconnect.peer) {
            Ok(p) => p,
            Err(e) => return error_response(format!("invalid peer id: {e}")),
        };
        match self.shared.handle.disconnect_peer(peer).await {
            Ok(()) => ok_response(),
            Err(e) => error_response(e.to_string()),
        }
    }

    /// `LIST_PEERS` — one entry per live connection.
    ///
    /// Go emits one `PeerInfo` **per connection**, not per peer, each carrying
    /// exactly one address (`conn.go:333-340`); a peer with two connections
    /// appears twice. `NetworkHandle::list_peers` has the same per-connection
    /// shape, so this is a direct mapping — no deduplication, which would change
    /// what `kwaainet status` reports.
    async fn do_list_peers(&self) -> Response {
        let peers = match self.shared.handle.list_peers().await {
            Ok(p) => p,
            Err(e) => return error_response(e.to_string()),
        };

        let mut res = ok_response();
        res.peers = peers
            .into_iter()
            .map(|p| PeerInfo {
                id: p.peer_id.to_bytes(),
                addrs: vec![p.addr.to_vec()],
            })
            .collect();
        res
    }

    /// `DHT` — only `FIND_PEER` is served.
    ///
    /// `dht.rs` also exposes put/get/provide/find_providers. Those are
    /// libp2p-Kademlia record and provider operations, distinct from the
    /// hivemind DHT this node actually speaks (`DHTProtocol.rpc_*`, served
    /// natively by `kwaai_p2p::dht_service`); the node's own announce path uses
    /// the hivemind verbs, and the remaining callers are integration tiers
    /// pointed at Go daemons. They get Go's own `"not supported"` rather than a
    /// silent success that would look like a working store.
    /// TODO(phase3-dht): decide per verb whether to back it with
    /// `kad::Behaviour` record/provider APIs or delete the client methods at
    /// cutover.
    async fn do_dht(&self, request: Request) -> Response {
        let Some(dht) = request.dht else {
            return error_response("Malformed request; missing parameters".to_string());
        };
        let Some(kind) = dht_request::Type::try_from(dht.r#type).ok() else {
            return error_response("Unexpected request".to_string());
        };

        match kind {
            dht_request::Type::FindPeer => {
                let Some(peer_bytes) = dht.peer else {
                    return error_response("Malformed request; missing peer parameter".to_string());
                };
                let peer = match PeerId::from_bytes(&peer_bytes) {
                    Ok(p) => p,
                    Err(e) => return error_response(format!("invalid peer id: {e}")),
                };

                match self.shared.handle.dht_find_peer(peer).await {
                    // An empty address list means the walk finished without
                    // locating the peer. Go's `dht.FindPeer` errors in that
                    // case and `dht.rs` maps any error to "Peer not found", so
                    // erroring keeps the client-visible outcome identical.
                    Ok(addrs) if addrs.is_empty() => {
                        error_response("routing: not found".to_string())
                    }
                    Ok(addrs) => {
                        let mut res = ok_response();
                        res.dht = Some(DhtResponse {
                            r#type: crate::protocol::p2pd::dht_response::Type::Value as i32,
                            peer: Some(PeerInfo {
                                id: peer.to_bytes(),
                                addrs: addrs.into_iter().map(|a| a.to_vec()).collect(),
                            }),
                            value: None,
                        });
                        res
                    }
                    Err(e) => error_response(e.to_string()),
                }
            }
            _ => error_response(NOT_SUPPORTED.to_string()),
        }
    }
}

// ============================================================================
// Pipe mode: raw byte relay between the socket and a libp2p stream
// ============================================================================
//
// Two verbs put a socket into raw-relay mode, and they are mirror images.
//
// **`STREAM_OPEN` (outbound).** The client asks for a stream to a peer; we open
// it, answer with `StreamInfo`, and then *this very socket* becomes the data
// channel — every subsequent byte in either direction is the libp2p stream's.
// Go does exactly this (`conn.go:59-73`): write the response, `doStreamPipe(c,
// s)`, `return`. The client side is `P2PClient::stream_open_raw`, which consumes
// itself and hands the socket back as a `P2PStream`; there is no second
// connection and no further framing.
//
// **`STREAM_HANDLER` (inbound).** The client registers a listener address for a
// set of protocols. For each inbound libp2p stream on one of them we dial that
// address, write a length-delimited `StreamInfo` prologue, and relay. Consumers
// read the prologue and then their own protocol
// (`inference_mux.rs::read_p2pd_stream_info`).
//
// ## Backpressure
//
// Both directions are copied by `tokio::io::copy_bidirectional`, whose flow
// control is that it *awaits* each write before reading more. A slow consumer
// therefore stops the producer at the socket rather than accumulating in our
// process — which is the whole reason not to hand-roll this with channels. The
// libp2p stream contributes its own yamux window on top.
//
// ## Termination
//
// `copy_bidirectional` finishes when both directions have seen EOF and been
// shut down, propagating each half-close as it happens: a client that signals
// "request complete" by closing its write half gets that FIN forwarded to the
// remote and still receives the reply. Either side erroring ends the whole
// relay and drops both stream and socket, so no task or fd outlives it.

/// Cap on how long `STREAM_OPEN` waits for the stream to be established.
///
/// Go's `DefaultTimeout` when `StreamOpenRequest.timeout` is unset or
/// non-positive (`conn.go:21`, `requestContext`). The client's
/// `stream_open`/`stream_open_raw` both send 60 explicitly, but the field is
/// optional, so the default has to exist.
const DEFAULT_STREAM_OPEN_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

impl ConnState {
    /// `STREAM_OPEN` — open a raw stream, ready to hand the socket to the relay.
    ///
    /// Returns `Ok((response, stream))` when the stream is live and the caller
    /// should enter pipe mode, or `Err(response)` when it should write the error
    /// and keep framing. Splitting it this way keeps the terminal transition in
    /// `run_connection`, where the reader half still lives.
    async fn do_stream_open(
        &self,
        request: Request,
    ) -> std::result::Result<(Response, kwaai_p2p::RawStream), Response> {
        let Some(open) = request.stream_open else {
            return Err(error_response(
                "Malformed request; missing parameters".to_string(),
            ));
        };

        let peer = match PeerId::from_bytes(&open.peer) {
            Ok(p) => p,
            Err(e) => return Err(error_response(format!("invalid peer id: {e}"))),
        };

        if open.proto.is_empty() {
            return Err(error_response(
                "Malformed request; missing parameters".to_string(),
            ));
        }

        // `timeout` is seconds, and Go treats anything non-positive as "use the
        // default" rather than "fail immediately".
        let timeout = match open.timeout {
            Some(secs) if secs > 0 => std::time::Duration::from_secs(secs as u64),
            _ => DEFAULT_STREAM_OPEN_TIMEOUT,
        };

        let opened = tokio::time::timeout(
            timeout,
            self.shared.handle.open_raw_stream(peer, open.proto.clone()),
        )
        .await;

        let (proto, stream) = match opened {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => return Err(error_response(e.to_string())),
            Err(_) => {
                return Err(error_response(format!(
                    "opening stream to {peer}: context deadline exceeded"
                )))
            }
        };

        // `StreamInfo` describes the *remote* end, as Go's `makeStreamInfo`
        // does: raw peer-ID bytes and a binary multiaddr. The address is the
        // connection's remote address; `stream_open_raw` deliberately ignores it
        // (connecting to it would reach a relay, not a local proxy), but
        // `stream_open` parses it and clients may log it, so it must be a real
        // multiaddr rather than empty.
        let addr = self
            .shared
            .handle
            .list_peers()
            .await
            .ok()
            .and_then(|peers| peers.into_iter().find(|p| p.peer_id == peer))
            .map(|p| p.addr.to_vec())
            .unwrap_or_default();

        debug!(conn = self.id, %peer, %proto, "stream open; entering pipe mode");

        let mut response = ok_response();
        response.stream_info = Some(crate::protocol::p2pd::StreamInfo {
            peer: peer.to_bytes(),
            addr,
            proto,
        });
        Ok((response, stream))
    }

    /// Hand this connection's socket to the relay loop and run it to completion.
    ///
    /// Takes the write half back out from under the mutex and reunites it with
    /// `reader`. Any concurrently-spawned frame writer that runs after this
    /// point finds `None` and errors, which is correct: the socket is no longer
    /// a frame channel. In practice there are none — pipe mode is only reachable
    /// from the request/response loop, which dispatches inline.
    async fn enter_pipe_mode(
        &self,
        reader: SocketReader,
        stream: kwaai_p2p::RawStream,
    ) -> Result<()> {
        let Some(writer) = self.writer.lock().await.take() else {
            return Err(Error::Protocol(
                "control socket already handed to a relay".to_string(),
            ));
        };
        let socket = reader.reunite(writer)?;

        let relayed = relay(socket, stream).await;
        debug!(conn = self.id, ?relayed, "pipe mode ended");
        // The relay ending is a normal end of connection, not a failure: the
        // client closed, the remote closed, or the stream reset. All three mean
        // this task is done.
        Ok(())
    }

    /// `STREAM_HANDLER` — accept inbound raw streams and forward them to the
    /// client's listener address.
    ///
    /// The registration is **owned by this connection** and released when it
    /// ends, which is a deliberate divergence from Go: `d.handlers` there is
    /// process-global and survives the registering client, so a crashed
    /// `shard serve` leaves the daemon advertising a protocol whose forwarding
    /// address refuses connections — every inbound stream then costs a dial
    /// timeout instead of a negotiation refusal. The unary path already made
    /// this choice in slice 1; this keeps the two consistent.
    async fn do_stream_handler(&self, request: Request) -> Response {
        let Some(req) = request.stream_handler else {
            return error_response("Malformed request; missing parameters".to_string());
        };

        let addr = match Multiaddr::try_from(req.addr.clone()) {
            Ok(a) => a,
            Err(e) => return error_response(format!("invalid multiaddr: {e}")),
        };
        // The forwarding target must be somewhere we can actually open a TCP
        // connection; anything else would fail per-stream, long after the
        // client believed registration succeeded.
        let target = match dial_target(&addr) {
            Some(t) => t,
            None => {
                return error_response(format!(
                    "handler address {addr} is not a dialable /ip4|/ip6 + /tcp address"
                ))
            }
        };

        if req.proto.is_empty() {
            return error_response("Malformed request; missing parameters".to_string());
        }

        // Claim ownership first, so two clients racing on the same protocol
        // cannot both reach the swarm.
        {
            let mut owners = self.shared.stream_handler_owners.lock().await;
            for proto in &req.proto {
                match owners.get(proto) {
                    Some(&owner) if owner != self.id => {
                        return error_response(format!("handler for protocol {proto} already set"));
                    }
                    // Re-registering our own protocol is idempotent, matching
                    // the unary path.
                    Some(_) => {}
                    None => {
                        owners.insert(proto.clone(), self.id);
                    }
                }
            }
        }

        let (mut inbound, refused) =
            match self.shared.handle.accept_streams(req.proto.clone()).await {
                Ok(v) => v,
                Err(e) => {
                    self.release_stream_owners(&req.proto).await;
                    return error_response(e.to_string());
                }
            };

        if !refused.is_empty() {
            // The swarm already serves one of these under a different owner —
            // our own map said otherwise, so this is a race we lost. Release
            // everything and report Go's wording for the first casualty.
            self.release_stream_owners(&req.proto).await;
            let _ = self
                .shared
                .handle
                .remove_stream_handler(
                    req.proto
                        .iter()
                        .filter(|p| !refused.contains(p))
                        .cloned()
                        .collect(),
                )
                .await;
            return error_response(format!("handler for protocol {} already set", refused[0]));
        }

        self.stream_handlers.lock().await.extend(req.proto.clone());

        let conn_id = self.id;
        // One task per registration, ending when the service drops its sender
        // (i.e. when `remove_stream_handler` runs, on explicit removal or on
        // this connection's teardown). Each accepted stream gets its own task so
        // a slow dial-back cannot stall the next stream.
        tokio::spawn(async move {
            while let Some(stream) = inbound.recv().await {
                tokio::spawn(forward_inbound_stream(conn_id, target, stream));
            }
            trace!(conn = conn_id, "stream handler accept loop ended");
        });

        debug!(conn = self.id, protos = ?req.proto, %addr, "stream handler registered");
        ok_response()
    }

    /// `REMOVE_STREAM_HANDLER` — stop accepting inbound streams on `proto`.
    ///
    /// Go matches on the (protocol, address) pair because one protocol may have
    /// several forwarding addresses under `balanced`. We keep one owner per
    /// protocol (`balanced` is unused here, as on the unary path), so the
    /// address is validated but ownership is what decides — otherwise a client
    /// that re-bound its listener on a new port could not remove its own
    /// handler.
    async fn do_remove_stream_handler(&self, request: Request) -> Response {
        let Some(req) = request.remove_stream_handler else {
            return error_response("Malformed request; missing parameters".to_string());
        };

        if let Err(e) = Multiaddr::try_from(req.addr.clone()) {
            return error_response(format!("invalid multiaddr: {e}"));
        }

        {
            let owners = self.shared.stream_handler_owners.lock().await;
            for proto in &req.proto {
                match owners.get(proto) {
                    None => {
                        return error_response(format!(
                            "handler for protocol {proto} does not exist"
                        ))
                    }
                    Some(&owner) if owner != self.id => {
                        return error_response(format!(
                            "handler for protocol {proto} was not created in this connection"
                        ))
                    }
                    Some(_) => {}
                }
            }
        }

        self.release_stream_owners(&req.proto).await;
        let _ = self
            .shared
            .handle
            .remove_stream_handler(req.proto.clone())
            .await;
        self.stream_handlers
            .lock()
            .await
            .retain(|p| !req.proto.contains(p));

        debug!(conn = self.id, protos = ?req.proto, "stream handler removed");
        ok_response()
    }

    /// Drop this connection's claim on `protos` in the global owner map.
    async fn release_stream_owners(&self, protos: &[String]) {
        let mut owners = self.shared.stream_handler_owners.lock().await;
        for proto in protos {
            if owners.get(proto) == Some(&self.id) {
                owners.remove(proto);
            }
        }
    }
}

/// Serve one inbound libp2p stream by dialling the client's listener and
/// relaying.
///
/// A dial-back failure **resets the stream**, matching Go's `handleStream`: the
/// remote must learn immediately that nothing is going to answer, rather than
/// holding an open stream to a black hole until its own timeout.
async fn forward_inbound_stream(
    conn_id: u64,
    target: std::net::SocketAddr,
    inbound: kwaai_p2p::InboundStream,
) {
    let kwaai_p2p::InboundStream {
        peer,
        proto,
        stream,
    } = inbound;

    let socket = match tokio::net::TcpStream::connect(target).await {
        Ok(s) => s,
        Err(e) => {
            warn!(
                conn = conn_id, %peer, %proto, %target, error = %e,
                "dialling the stream handler failed; resetting the inbound stream"
            );
            // Dropping a libp2p `Stream` without closing it resets it, which is
            // the signal Go sends here.
            drop(stream);
            return;
        }
    };
    // The relay is raw after the prologue; Nagle would add latency to small
    // request/response protocols like the mux for no benefit.
    let _ = socket.set_nodelay(true);

    // The `StreamInfo` prologue, length-delimited exactly as gogo's
    // `DelimitedWriter` writes it and `stream.rs::parse_stream_info` reads it.
    // It describes the *caller*, so a handler can attribute the stream without
    // trusting anything inside the protocol that follows.
    let info = crate::protocol::p2pd::StreamInfo {
        peer: peer.to_bytes(),
        addr: Vec::new(),
        proto: proto.as_ref().to_string(),
    };
    let mut socket = socket;
    if let Err(e) = write_delimited(&mut socket, &info).await {
        warn!(
            conn = conn_id, %peer, %proto, error = %e,
            "writing the StreamInfo prologue failed; resetting the inbound stream"
        );
        drop(stream);
        return;
    }

    trace!(conn = conn_id, %peer, %proto, "relaying inbound stream");
    let relayed = relay(socket, stream).await;
    trace!(conn = conn_id, %peer, %proto, ?relayed, "inbound stream relay ended");
}

/// Copy bytes both ways between a socket and a libp2p stream until both
/// directions close.
///
/// The libp2p stream is `futures::io`; the socket is `tokio::io`. `compat()`
/// bridges them so a single [`tokio::io::copy_bidirectional`] can own both —
/// which is what supplies the backpressure: it awaits each write before reading
/// more, so neither side can outrun the other and nothing is buffered beyond one
/// in-flight chunk per direction.
///
/// Returns the (socket → stream, stream → socket) byte counts, for logging.
async fn relay<S>(socket: S, stream: kwaai_p2p::RawStream) -> Result<(u64, u64)>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    use tokio_util::compat::FuturesAsyncReadCompatExt;

    let mut socket = socket;
    let mut stream = stream.compat();

    tokio::io::copy_bidirectional(&mut socket, &mut stream)
        .await
        .map_err(Error::Io)
}

/// Write one length-delimited protobuf message, gogo `DelimitedWriter` style.
async fn write_delimited<W, M>(writer: &mut W, msg: &M) -> Result<()>
where
    W: AsyncWrite + Unpin,
    M: ProstMessage,
{
    let mut buf = Vec::with_capacity(msg.encoded_len() + 10);
    msg.encode(&mut buf)
        .map_err(|e| Error::Protocol(format!("Failed to encode message: {e}")))?;

    let mut len_buf = unsigned_varint::encode::usize_buffer();
    let len_bytes = unsigned_varint::encode::usize(buf.len(), &mut len_buf);

    let mut frame = Vec::with_capacity(len_bytes.len() + buf.len());
    frame.extend_from_slice(len_bytes);
    frame.extend_from_slice(&buf);

    writer.write_all(&frame).await.map_err(Error::Io)?;
    writer.flush().await.map_err(Error::Io)?;
    Ok(())
}

/// Extract a dialable `SocketAddr` from a `/ip4|/ip6/…/tcp/<port>` multiaddr.
///
/// Stream handler addresses are always loopback TCP listeners in this codebase
/// (`inference_mux.rs`, `node.rs`), and Go dials them with `manet.Dial`. Returns
/// `None` for anything without both an IP and a TCP port, so a malformed
/// registration fails at registration time rather than per inbound stream.
fn dial_target(addr: &Multiaddr) -> Option<std::net::SocketAddr> {
    use libp2p::multiaddr::Protocol;

    let mut ip = None;
    let mut port = None;
    for component in addr.iter() {
        match component {
            Protocol::Ip4(a) => ip = Some(std::net::IpAddr::V4(a)),
            Protocol::Ip6(a) => ip = Some(std::net::IpAddr::V6(a)),
            Protocol::Tcp(p) => port = Some(p),
            _ => {}
        }
    }
    Some(std::net::SocketAddr::new(ip?, port?))
}

// ============================================================================
// Persistent connection (unary handlers)
// ============================================================================

/// Read `PersistentConnectionRequest` frames until the client disconnects.
///
/// Each frame is dispatched on its own task so a slow `callUnary` cannot block
/// the arrival of the `unaryResponse` that some *other* in-flight call is
/// waiting for — the deadlock this sub-protocol invites if handled inline.
async fn run_persistent(state: Arc<ConnState>, mut reader: SocketReader) -> Result<()> {
    loop {
        let bytes = match read_frame(&mut reader).await {
            Ok(b) => b,
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(e),
        };

        let request = match PersistentConnectionRequest::decode(&bytes[..]) {
            Ok(r) => r,
            Err(e) => {
                warn!(conn = state.id, error = %e, "bad persistent frame");
                continue;
            }
        };

        // Go drops frames with an unparseable call ID without replying — there
        // is nothing to correlate a reply to.
        let Ok(call_id) = Uuid::from_slice(&request.call_id) else {
            warn!(conn = state.id, "bad call id in persistent frame");
            continue;
        };

        let state = Arc::clone(&state);
        tokio::spawn(async move {
            state.handle_persistent_request(call_id, request).await;
        });
    }
}

impl ConnState {
    async fn handle_persistent_request(&self, call_id: Uuid, request: PersistentConnectionRequest) {
        use persistent_connection_request::Message;

        match request.message {
            Some(Message::AddUnaryHandler(req)) => {
                let response = self.add_unary_handler(call_id, &req.proto).await;
                let _ = self.write_persistent(response).await;
            }

            Some(Message::RemoveUnaryHandler(req)) => {
                let response = self.remove_unary_handler(call_id, &req.proto).await;
                let _ = self.write_persistent(response).await;
            }

            Some(Message::CallUnary(req)) => {
                let response = self.call_unary(call_id, req).await;
                let _ = self.write_persistent(response).await;
            }

            // The client answering an inbound call we dispatched to it.
            Some(Message::UnaryResponse(resp)) => {
                let waiter = self.waiters.lock().await.remove(&call_id);
                match waiter {
                    Some(tx) => {
                        let _ = tx.send(resp);
                    }
                    // Late or duplicate response — the caller already gave up.
                    None => trace!(conn = self.id, %call_id, "response for unknown call"),
                }
            }

            // Cancellation of an in-flight outbound call. Our unary calls are
            // bounded by `NetworkConfig::request_timeout` and there is no way to
            // abort one mid-flight through `NetworkHandle`, so this is accepted
            // and ignored rather than errored — the caller has stopped waiting
            // either way.
            Some(Message::Cancel(_)) => {
                trace!(conn = self.id, %call_id, "cancel (no-op)");
            }

            None => warn!(conn = self.id, %call_id, "persistent frame with no message"),
        }
    }

    /// Register `proto` and start serving it on the swarm.
    ///
    /// Refuses a protocol another connection already owns, with Go's exact
    /// wording (`persistent_stream.go:146`) — `hivemind` and our own call sites
    /// treat that message as "someone else got there first" rather than a fault.
    ///
    /// The `balanced` flag is deliberately ignored: it selects Go's round-robin
    /// fan-out across several client connections, and no call site in this
    /// codebase passes `true`. Honouring it would mean a second owner map and a
    /// fairness policy for a mode nothing uses.
    async fn add_unary_handler(&self, call_id: Uuid, proto: &str) -> PersistentConnectionResponse {
        {
            let mut owners = self.shared.handler_owners.lock().await;
            match owners.get(proto) {
                Some(&owner) if owner != self.id => {
                    return daemon_error(
                        call_id,
                        format!("handler for protocol {proto} already set"),
                    );
                }
                // Re-registering our own protocol is idempotent.
                Some(_) => return ok_persistent(call_id),
                None => {
                    owners.insert(proto.to_string(), self.id);
                }
            }
        }

        // Route inbound libp2p calls for this protocol back to this socket.
        let dispatcher = self.inbound_dispatcher(proto.to_string());
        if let Err(e) = self
            .shared
            .handle
            .add_unary_handler_boxed(proto, dispatcher)
            .await
        {
            self.shared.handler_owners.lock().await.remove(proto);
            return daemon_error(call_id, e.to_string());
        }

        self.handlers.lock().await.push(proto.to_string());
        debug!(conn = self.id, proto, "unary handler registered");
        ok_persistent(call_id)
    }

    /// Build the closure the swarm calls for each inbound request on `proto`.
    ///
    /// It forwards the call to this socket as a `requestHandling` frame and
    /// waits for the client's `unaryResponse`, correlating on a **freshly
    /// generated** call ID. The ID is ours, not the remote caller's: two remote
    /// peers calling us concurrently could otherwise present the same ID, and
    /// the resulting cross-talk would be invisible.
    fn inbound_dispatcher(&self, proto: String) -> kwaai_p2p::UnaryHandler {
        let writer = Arc::clone(&self.writer);
        let waiters = Arc::clone(&self.waiters);
        let conn_id = self.id;

        Box::new(move |data: Vec<u8>| {
            let writer = Arc::clone(&writer);
            let waiters = Arc::clone(&waiters);
            let proto = proto.clone();
            Box::pin(async move {
                let call_id = Uuid::new_v4();
                let (tx, rx) = oneshot::channel();
                waiters.lock().await.insert(call_id, tx);

                // `peer` carries the *caller's* ID on this path, matching Go's
                // rewrite in `persistent_stream.go:298`. We do not have it here
                // — `UnaryHandler` receives only the payload — so it is left
                // empty; proto2 `required` means the field is still encoded.
                // TODO(phase3-caller-id): thread `InboundUnaryCall::peer`
                // through `add_unary_handler_boxed` so handlers that
                // authenticate the caller can see it.
                let frame = PersistentConnectionResponse {
                    call_id: call_id.as_bytes().to_vec(),
                    message: Some(persistent_connection_response::Message::RequestHandling(
                        crate::protocol::p2pd::CallUnaryRequest {
                            peer: Vec::new(),
                            proto: proto.clone(),
                            data,
                        },
                    )),
                };

                if let Err(e) = write_persistent_frame(&writer, &frame).await {
                    waiters.lock().await.remove(&call_id);
                    return Err(format!("control client unreachable: {e}"));
                }

                match rx.await {
                    Ok(resp) => match resp.result {
                        Some(crate::protocol::p2pd::call_unary_response::Result::Response(d)) => {
                            Ok(d)
                        }
                        Some(crate::protocol::p2pd::call_unary_response::Result::Error(e)) => {
                            Err(String::from_utf8_lossy(&e).into_owned())
                        }
                        None => Err("empty unary response".to_string()),
                    },
                    // The connection died with the call in flight. The remote
                    // caller gets an error arm instead of hanging to its own
                    // timeout.
                    Err(_) => {
                        trace!(conn = conn_id, %call_id, "control client dropped mid-call");
                        Err("control client disconnected".to_string())
                    }
                }
            })
        })
    }

    /// Stop serving `proto`.
    ///
    /// Refusing a protocol this connection does not own is Go's behaviour and
    /// matters for safety: without the ownership check one client could
    /// deregister another's handler.
    async fn remove_unary_handler(
        &self,
        call_id: Uuid,
        proto: &str,
    ) -> PersistentConnectionResponse {
        {
            let mut owners = self.shared.handler_owners.lock().await;
            match owners.get(proto) {
                None => {
                    return daemon_error(
                        call_id,
                        format!("handler for protocol {proto} does not exist"),
                    )
                }
                Some(&owner) if owner != self.id => {
                    return daemon_error(
                        call_id,
                        format!(
                            "handler for protocol {proto} was not created in this persistent connection"
                        ),
                    )
                }
                Some(_) => {
                    owners.remove(proto);
                }
            }
        }

        let _ = self.shared.handle.remove_unary_handler(proto).await;
        self.handlers.lock().await.retain(|p| p != proto);
        debug!(conn = self.id, proto, "unary handler removed");
        ok_persistent(call_id)
    }

    /// Translate `callUnary` into [`NetworkHandle::call_unary_handler`].
    ///
    /// Go returns the remote's `CallUnaryResponse` verbatim in a
    /// `callUnaryResponse` arm, and any *local* failure (bad peer ID, dial
    /// failure, stream reset) as a `daemonError`. `persistent.rs` distinguishes
    /// the two, so the split is preserved: a remote handler's error arm is not
    /// a daemon error.
    async fn call_unary(
        &self,
        call_id: Uuid,
        req: crate::protocol::p2pd::CallUnaryRequest,
    ) -> PersistentConnectionResponse {
        let peer = match PeerId::from_bytes(&req.peer) {
            Ok(p) => p,
            Err(e) => return daemon_error(call_id, format!("invalid peer id: {e}")),
        };

        match self
            .shared
            .handle
            .call_unary_handler(peer, &req.proto, &req.data)
            .await
        {
            Ok(data) => PersistentConnectionResponse {
                call_id: call_id.as_bytes().to_vec(),
                message: Some(persistent_connection_response::Message::CallUnaryResponse(
                    CallUnaryResponse {
                        result: Some(
                            crate::protocol::p2pd::call_unary_response::Result::Response(data),
                        ),
                    },
                )),
            },
            Err(e) => daemon_error(call_id, e.to_string()),
        }
    }

    /// Release every handler this connection owns.
    ///
    /// The ownership check makes this safe under races: if the protocol was
    /// already taken over by another connection, we must not remove it from the
    /// swarm.
    async fn deregister_all(&self) {
        let protos = std::mem::take(&mut *self.handlers.lock().await);
        if !protos.is_empty() {
            // Ownership is decided under the global lock, but the swarm
            // round-trips happen after it is dropped — `NetworkHandle` calls
            // have no timeout, and holding `handler_owners` across them would
            // let one dead client stall every other client's registration
            // behind a busy swarm.
            let mine: Vec<String> = {
                let mut owners = self.shared.handler_owners.lock().await;
                protos
                    .into_iter()
                    .filter(|p| {
                        if owners.get(p) == Some(&self.id) {
                            owners.remove(p);
                            true
                        } else {
                            false
                        }
                    })
                    .collect()
            };
            for proto in mine {
                let _ = self.shared.handle.remove_unary_handler(&proto).await;
                info!(
                    conn = self.id,
                    proto, "unary handler released on client disconnect"
                );
            }
        }

        // Raw-stream handlers get the same treatment, and for the same reason:
        // the forwarding address is this client's TCP listener, so once it is
        // gone every inbound stream on that protocol would dial a closed port.
        // Releasing turns that into a clean negotiation refusal. (Go leaves the
        // registration in place — see `do_stream_handler`.)
        let stream_protos = std::mem::take(&mut *self.stream_handlers.lock().await);
        if !stream_protos.is_empty() {
            let mut owners = self.shared.stream_handler_owners.lock().await;
            let mine: Vec<String> = stream_protos
                .into_iter()
                .filter(|p| owners.get(p) == Some(&self.id))
                .collect();
            for proto in &mine {
                owners.remove(proto);
            }
            drop(owners);
            if !mine.is_empty() {
                let _ = self.shared.handle.remove_stream_handler(mine.clone()).await;
                info!(
                    conn = self.id,
                    protos = ?mine,
                    "stream handlers released on client disconnect"
                );
            }
        }

        // Unblock anything still waiting on this client — dropping the senders
        // resolves each pending inbound call with an error arm.
        self.waiters.lock().await.clear();
    }
}

// ============================================================================
// Framing helpers
// ============================================================================

/// Read one uvarint-length-delimited frame.
///
/// Matches gogo-protobuf's `DelimitedReader`, which is what the Go daemon and
/// therefore every existing client speaks.
async fn read_frame<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Vec<u8>> {
    let mut len_bytes = Vec::with_capacity(10);
    let mut byte = [0u8; 1];

    loop {
        reader.read_exact(&mut byte).await.map_err(Error::Io)?;
        len_bytes.push(byte[0]);
        if byte[0] & 0x80 == 0 {
            break;
        }
        if len_bytes.len() == 10 {
            return Err(Error::Protocol("varint length too long".to_string()));
        }
    }

    let (len, _) = unsigned_varint::decode::usize(&len_bytes)
        .map_err(|e| Error::Protocol(format!("Invalid varint: {e}")))?;

    if len > MAX_MESSAGE_SIZE {
        return Err(Error::Protocol(format!("Message too large: {len} bytes")));
    }

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload).await.map_err(Error::Io)?;
    Ok(payload)
}

/// Encode and write one uvarint-delimited frame under the write lock.
///
/// Fails if the writer has been taken by pipe mode — the socket is a raw data
/// channel from that point on, and injecting a frame into it would corrupt the
/// relayed stream.
async fn write_frame<M: ProstMessage>(
    writer: &Arc<Mutex<Option<SocketWriter>>>,
    msg: &M,
) -> Result<()> {
    let mut buf = Vec::with_capacity(msg.encoded_len() + 10);
    msg.encode(&mut buf)
        .map_err(|e| Error::Protocol(format!("Failed to encode message: {e}")))?;

    let mut len_buf = unsigned_varint::encode::usize_buffer();
    let len_bytes = unsigned_varint::encode::usize(buf.len(), &mut len_buf);

    // One `write_all` per frame under the lock: two concurrent responders must
    // not interleave a length prefix with another frame's body.
    let mut frame = Vec::with_capacity(len_bytes.len() + buf.len());
    frame.extend_from_slice(len_bytes);
    frame.extend_from_slice(&buf);

    let mut guard = writer.lock().await;
    let Some(w) = guard.as_mut() else {
        return Err(Error::Protocol(
            "control socket is in pipe mode; no frames can be written".to_string(),
        ));
    };
    w.write_all(&frame).await.map_err(Error::Io)?;
    w.flush().await.map_err(Error::Io)?;
    Ok(())
}

async fn write_persistent_frame(
    writer: &Arc<Mutex<Option<SocketWriter>>>,
    frame: &PersistentConnectionResponse,
) -> Result<()> {
    write_frame(writer, frame).await
}

impl ConnState {
    async fn write_response(&self, response: Response) -> Result<()> {
        write_frame(&self.writer, &response).await
    }

    async fn write_persistent(&self, response: PersistentConnectionResponse) -> Result<()> {
        write_frame(&self.writer, &response).await
    }
}

// ============================================================================
// Response constructors — the Go daemon's exact shapes
// ============================================================================

/// `Response{type: OK}`, Go's `okResponse`.
fn ok_response() -> Response {
    Response {
        r#type: response::Type::Ok as i32,
        error: None,
        stream_info: None,
        identify: None,
        dht: None,
        peers: Vec::new(),
        pubsub: None,
    }
}

/// `Response{type: ERROR, error{msg}}`, Go's `errorResponseString`.
fn error_response(msg: String) -> Response {
    Response {
        r#type: response::Type::Error as i32,
        error: Some(ErrorResponse { msg }),
        stream_info: None,
        identify: None,
        dht: None,
        peers: Vec::new(),
        pubsub: None,
    }
}

/// `PersistentConnectionResponse{callId}` with **no** message arm — Go's
/// `okUnaryCallResponse`, and the ACK shape `persistent.rs` expects.
fn ok_persistent(call_id: Uuid) -> PersistentConnectionResponse {
    PersistentConnectionResponse {
        call_id: call_id.as_bytes().to_vec(),
        message: None,
    }
}

/// Go's `errorUnaryCallString`.
fn daemon_error(call_id: Uuid, message: String) -> PersistentConnectionResponse {
    PersistentConnectionResponse {
        call_id: call_id.as_bytes().to_vec(),
        message: Some(persistent_connection_response::Message::DaemonError(
            crate::protocol::p2pd::DaemonError {
                message: Some(message),
            },
        )),
    }
}

/// Default control-socket path, mirroring [`crate::DEFAULT_SOCKET_NAME`] in the
/// multiaddr form both [`ControlServer::bind`] and
/// [`crate::client::P2PClient::connect`] accept.
pub fn default_socket_addr() -> String {
    #[cfg(unix)]
    {
        format!("/unix/{}", crate::DEFAULT_SOCKET_NAME)
    }
    #[cfg(not(unix))]
    {
        "/ip4/127.0.0.1/tcp/5005".to_string()
    }
}
