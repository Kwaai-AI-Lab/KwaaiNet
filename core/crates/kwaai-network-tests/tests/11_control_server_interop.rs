//! `ControlServer` end-to-end over the control socket.
//!
//! A [`ControlServer`] serves the p2pd control protocol for a native node, with
//! real [`P2PClient`]s attached. Two interfaces must hold at once:
//!
//! 1. the **IPC protocol** — a client drives the node over the socket,
//! 2. the **libp2p wire** — a handler registered over one node's socket answers
//!    a caller that arrived through another's.
//!
//! ```text
//!   [P2PClient] ──socket──▶ [ControlServer + NetworkService] ◀──libp2p──▶ [ControlServer + NetworkService] ◀──socket── [P2PClient]
//! ```
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`.
//!
//! # Process hygiene
//!
//! Each node listens on an ephemeral loopback port with a fresh key and a
//! tmpdir socket, so it cannot collide with a node running on the same machine.

use std::time::Duration;

use kwaai_network_tests::{metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use kwaai_p2p_daemon::{ControlServer, P2PClient};
use libp2p::identity::Keypair;
use tempfile::TempDir;

const PROTO: &str = "DHTProtocol.rpc_ping";

/// Cap on any single daemon interaction, so a regression fails rather than
/// hanging the suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

/// A native node: `NetworkService` behind a `ControlServer`.
struct NativeNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// Control-socket multiaddr, for `P2PClient::connect`.
    socket: String,
    /// libp2p listen address including `/p2p/<id>`, dialable by another node.
    addr: String,
    service_task: tokio::task::JoinHandle<()>,
    server_task: tokio::task::JoinHandle<()>,
    _tmpdir: TempDir,
}

impl NativeNode {
    async fn spawn() -> Self {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");

        let listen: Multiaddr = tokio::time::timeout(CALL_TIMEOUT, async {
            loop {
                if let Some(a) = handle
                    .listen_addrs()
                    .await
                    .ok()
                    .and_then(|a| a.into_iter().next())
                {
                    return a;
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .expect("the swarm must report a listen address");

        let tmpdir = TempDir::new().expect("tmpdir");
        let socket = format!("/unix/{}", tmpdir.path().join("kwaai.sock").display());
        let server = ControlServer::bind(&socket, handle.clone())
            .await
            .expect("control socket binds");
        let server_task = tokio::spawn(server.run());

        Self {
            handle,
            peer_id,
            socket,
            addr: format!("{listen}/p2p/{peer_id}"),
            service_task,
            server_task,
            _tmpdir: tmpdir,
        }
    }

    async fn client(&self) -> P2PClient {
        P2PClient::connect(&self.socket)
            .await
            .expect("a P2PClient must connect to the native control socket")
    }

    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        self.server_task.abort();
        let _ = tokio::time::timeout(Duration::from_secs(5), self.service_task).await;
    }
}

// ============================================================================
// Native node → native node
// ============================================================================

/// Two native nodes, each with its own `ControlServer`, and socket clients on
/// both ends.
#[tokio::test]
async fn native_socket_client_calls_another_native_node() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::control_server::native_client_calls_native",
        "integration",
    );

    let caller = NativeNode::spawn().await;
    let responder = NativeNode::spawn().await;

    let responder_client = responder.client().await;
    responder_client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"peer:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register a handler on the responder's socket");

    let mut connector = caller.client().await;
    connector
        .connect_peer(&responder.addr)
        .await
        .expect("CONNECT over the native control socket must dial the peer");

    let client = caller.client().await;
    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        client.call_unary_handler(&responder.peer_id.to_bytes(), PROTO, b"native-to-native"),
    )
    .await
    .expect("the call must resolve within the timeout")
    .expect("two native nodes must exchange a unary call end to end over their sockets");
    assert_eq!(response, b"peer:native-to-native");

    rec.metric("response_len", response.len());
    caller.shutdown().await;
    responder.shutdown().await;
    rec.finish(true);
}
