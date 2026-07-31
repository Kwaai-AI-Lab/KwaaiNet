//! Node-hosted p2pd control-socket **server** — Phase 3 of the native-p2p migration.
//!
//! This is the mirror image of [`crate::client`]: the same protobuf control
//! protocol, served by the node itself and translated into
//! [`kwaai_p2p::NetworkHandle`] calls instead of into a Go process. External
//! processes (`map-server` crawler, `shard serve`, `storage serve`, `rag`,
//! `p2p`/`status`, inference-mux, …) attach to the socket exactly as they do
//! today — `client.rs`, `persistent.rs` and `dht.rs` are unchanged — and keep
//! acting as the node's peer identity.
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
//! One tokio task per accepted connection. A connection is in one of two modes:
//!
//! - **request/response** — read a `Request`, write a `Response`, repeat. Simple
//!   verbs are handled here.
//! - **persistent** — entered by `PERSISTENT_CONN_UPGRADE` and never left. Frames
//!   flow in both directions concurrently and are correlated by `callId`, so the
//!   read loop must never block on a call: each request is spawned onto its own
//!   task (matching Go's `go d.handlePersistentConnRequest(...)`) and all writes
//!   funnel through a shared `Mutex<writer>` so frames cannot interleave
//!   mid-message.
//!
//! ## Handler ownership and call-ID isolation
//!
//! Two independent clients may pick the same UUID for concurrent calls — UUIDv4
//! collisions are unlikely but the failure mode (a response delivered to the
//! wrong process) is silent and severe, so correlation state is **per
//! connection**, never global. Only the protocol → owning-connection map is
//! global, because a libp2p protocol can only be served by one handler.
//!
//! Every handler a connection registers is tracked in [`ConnState::handlers`]
//! and deregistered from both the global map and the swarm when the connection
//! ends — for any reason, including a client crash. That closes today's
//! stale-handler bug, where a crashed `storage serve` left the daemon
//! advertising a protocol nothing would answer, turning a clean negotiation
//! refusal into a hang.
//!
//! # Scope of this slice
//!
//! Served: IDENTIFY, CONNECT, DISCONNECT, LIST_PEERS, DHT FIND_PEER, and the
//! full persistent-connection unary sub-protocol. Stubbed with the Go daemon's
//! error shape: STREAM_OPEN, STREAM_HANDLER, REMOVE_STREAM_HANDLER (pipe-mode
//! raw byte relay — Phase 3 continuation), the remaining DHT verbs, PUBSUB and
//! CONNMANAGER (never implemented by this codebase's client either).

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
            }),
        })
    }

    async fn bind_listener(addr: &str) -> Result<Listener> {
        if let Some(path) = addr.strip_prefix("/unix/") {
            #[cfg(unix)]
            {
                let path = PathBuf::from(path);
                // A socket file left by a previous process is not a live
                // listener; binding over it is correct and required.
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
                        let (r, w) = stream.into_split();
                        tokio::spawn(serve_connection(conn_id, shared, r, w));
                    }
                    Err(e) => {
                        warn!(error = %e, "control socket accept failed");
                        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    }
                },
                Listener::Tcp(listener) => match listener.accept().await {
                    Ok((stream, _)) => {
                        let (r, w) = stream.into_split();
                        tokio::spawn(serve_connection(conn_id, shared, r, w));
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
    writer: Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
    /// Protocols this connection registered, for teardown on disconnect.
    handlers: Mutex<Vec<String>>,
    /// Inbound calls awaiting this client's `unaryResponse`, keyed by call ID.
    ///
    /// Per connection by construction — see the module docs on call-ID
    /// isolation. `Arc` because each registered handler's dispatcher closure
    /// outlives the borrow that created it.
    waiters: Arc<Mutex<HashMap<Uuid, oneshot::Sender<CallUnaryResponse>>>>,
}

/// Serve one accepted connection to completion, then clean up its handlers.
async fn serve_connection<R, W>(id: u64, shared: Arc<Shared>, reader: R, writer: W)
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let state = Arc::new(ConnState {
        id,
        shared,
        writer: Arc::new(Mutex::new(
            Box::new(writer) as Box<dyn AsyncWrite + Unpin + Send>
        )),
        handlers: Mutex::new(Vec::new()),
        waiters: Arc::new(Mutex::new(HashMap::new())),
    });

    debug!(conn = id, "control connection opened");
    if let Err(e) = run_connection(Arc::clone(&state), reader).await {
        debug!(conn = id, error = %e, "control connection ended");
    }

    // Deregistration runs on *every* exit path — clean close, decode error,
    // client crash — which is the whole point: a dead client must not leave the
    // node advertising a protocol it can no longer serve.
    state.deregister_all().await;
    debug!(conn = id, "control connection closed");
}

/// Read frames until EOF. Returns `Ok(())` on a clean client close.
async fn run_connection<R>(state: Arc<ConnState>, mut reader: R) -> Result<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
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

        // PERSISTENT_CONN_UPGRADE is terminal: the connection never returns to
        // request/response framing, exactly as in Go's `handleConn`.
        if request.r#type == request::Type::PersistentConnUpgrade as i32 {
            state.write_response(ok_response()).await?;
            return run_persistent(state, reader).await;
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
            // Go logs "unexpected request type" and closes; answering keeps a
            // buggy client informed instead of leaving it waiting on a socket
            // that just went away.
            return error_response(format!("Unexpected request type {}", request.r#type));
        };

        match kind {
            request::Type::Identify => self.do_identify().await,
            request::Type::Connect => self.do_connect(request).await,
            request::Type::Disconnect => self.do_disconnect(request).await,
            request::Type::ListPeers => self.do_list_peers().await,
            request::Type::Dht => self.do_dht(request).await,

            // Pipe mode: raw byte relay between this socket and a libp2p
            // stream, in both directions with backpressure. Deferred to the
            // Phase 3 continuation (see docs/NATIVE_P2P_MIGRATION.md); the
            // in-tree consumers are inference-mux and block_rpc.
            // TODO(phase3-pipe): implement STREAM_OPEN / STREAM_HANDLER /
            // REMOVE_STREAM_HANDLER over `libp2p_stream`.
            request::Type::StreamOpen
            | request::Type::StreamHandler
            | request::Type::RemoveStreamHandler => error_response(NOT_SUPPORTED.to_string()),

            // Never implemented by this codebase's client either; the Go daemon
            // supports them but nothing here has ever called them.
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
    /// the swarm's listen addresses, which is the same set pre-NAT-traversal;
    /// once Phase 4 lands external-address confirmation those become part of
    /// this answer too. Byte format matters more than membership here: raw peer
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
// Persistent connection (unary handlers)
// ============================================================================

/// Read `PersistentConnectionRequest` frames until the client disconnects.
///
/// Each frame is dispatched on its own task so a slow `callUnary` cannot block
/// the arrival of the `unaryResponse` that some *other* in-flight call is
/// waiting for — the deadlock this sub-protocol invites if handled inline.
async fn run_persistent<R>(state: Arc<ConnState>, mut reader: R) -> Result<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
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
        if protos.is_empty() {
            return;
        }

        let mut owners = self.shared.handler_owners.lock().await;
        for proto in protos {
            if owners.get(&proto) == Some(&self.id) {
                owners.remove(&proto);
                let _ = self.shared.handle.remove_unary_handler(&proto).await;
                info!(
                    conn = self.id,
                    proto, "unary handler released on client disconnect"
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
async fn write_frame<M: ProstMessage>(
    writer: &Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
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

    let mut w = writer.lock().await;
    w.write_all(&frame).await.map_err(Error::Io)?;
    w.flush().await.map_err(Error::Io)?;
    Ok(())
}

async fn write_persistent_frame(
    writer: &Arc<Mutex<Box<dyn AsyncWrite + Unpin + Send>>>,
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
