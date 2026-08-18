//! `unary::Behaviour` ↔ real p2pd interop — the Phase 2 swarm gate.
//!
//! Where `07_wire_interop` proved the *frame bytes* against a Go daemon over
//! p2pd-managed streams, this tier proves the whole native stack: a rust-libp2p
//! swarm running `kwaai_p2p::unary::Behaviour` negotiates TCP + noise + yamux +
//! slash-less multistream-select against go-libp2p and exchanges hivemind
//! unary calls in both directions.
//!
//! This is also the live check on the vendored `multistream-select` patch
//! (`core/patches/multistream-select`): a Go peer must accept our bare-name
//! proposal, and our listener must parse Go's.
//!
//! | test | caller | responder |
//! | --- | --- | --- |
//! | `swarm_calls_daemon_handler` | **swarm** | p2pd |
//! | `daemon_calls_swarm_handler` | p2pd | **swarm** |
//! | `swarm_gets_clean_refusal_from_daemon` | **swarm** | p2pd (no handler) |
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like the other integration tiers.

use std::time::Duration;

use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use kwaai_p2p::unary::{self, UnaryError, UnaryProtocol, UnaryResult};
use libp2p::futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId, Swarm, SwarmBuilder};
use tokio::sync::{mpsc, oneshot};

const PROTO: &str = "DHTProtocol.rpc_ping";
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

// ============================================================================
// Swarm harness
// ============================================================================

/// A driven swarm node: commands in via channels, its event loop on a task.
struct SwarmNode {
    peer_id: PeerId,
    /// Listen address including `/p2p/<id>`, dialable by a p2pd.
    addr: String,
    calls: mpsc::UnboundedSender<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>,
}

impl SwarmNode {
    /// Spawn a swarm serving `PROTO` with `handler`, and knowing `known_peer`'s
    /// address for dial-on-demand.
    async fn spawn(
        known_peer: Option<(PeerId, Multiaddr)>,
        handler: impl Fn(Vec<u8>) -> Result<Vec<u8>, String> + Send + 'static,
    ) -> Self {
        let mut swarm: Swarm<unary::Behaviour> = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default().nodelay(true),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )
            .expect("tcp transport")
            .with_behaviour(|_| unary::Behaviour::new(unary::Config::default()))
            .expect("behaviour")
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();

        swarm
            .behaviour_mut()
            .register_protocol(UnaryProtocol::new(PROTO));
        if let Some((peer, addr)) = known_peer {
            swarm.add_peer_address(peer, addr);
        }

        let peer_id = *swarm.local_peer_id();
        swarm
            .listen_on("/ip4/127.0.0.1/tcp/0".parse().expect("valid multiaddr"))
            .expect("listen");
        let listen_addr = loop {
            if let SwarmEvent::NewListenAddr { address, .. } = swarm.select_next_some().await {
                break address;
            }
        };

        let (calls_tx, mut calls_rx) =
            mpsc::unbounded_channel::<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    call = calls_rx.recv() => match call {
                        Some((peer, proto, data, reply)) => swarm.behaviour_mut().send_request(
                            peer,
                            UnaryProtocol::new(proto),
                            data,
                            reply,
                        ),
                        None => break,
                    },
                    event = swarm.select_next_some() => {
                        if let SwarmEvent::Behaviour(unary::Event::InboundRequest { request, .. }) = event {
                            let _ = request.responder.send(handler(request.data));
                        }
                    }
                }
            }
        });

        Self {
            peer_id,
            addr: format!("{listen_addr}/p2p/{peer_id}"),
            calls: calls_tx,
        }
    }

    async fn call(&self, peer: PeerId, proto: &str, data: &[u8]) -> UnaryResult {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.calls
            .send((peer, proto.to_string(), data.to_vec(), reply_tx))
            .expect("swarm task alive");
        tokio::time::timeout(CALL_TIMEOUT, reply_rx)
            .await
            .expect("call must resolve within the timeout")
            .expect("reply oneshot must not be dropped")
    }
}

/// The daemon's peer identity as swarm-side types.
fn daemon_peer(node: &TestNode) -> (PeerId, Multiaddr) {
    let peer_id = PeerId::from_bytes(&node.peer_id_bytes()).expect("valid peer id bytes");
    let port = node.p2p_port.expect("wire peer has a fixed port");
    let addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{port}")
        .parse()
        .expect("valid multiaddr");
    (peer_id, addr)
}

// ============================================================================
// Tests
// ============================================================================

/// Native swarm → Go daemon: our dialer negotiates noise, yamux and the bare
/// protocol name against go-libp2p, and the daemon-served handler answers.
#[tokio::test]
async fn swarm_calls_daemon_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::unary_swarm::swarm_calls_daemon",
        "integration",
    );

    let daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    daemon
        .client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"daemon:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("register unary handler on daemon");

    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    let swarm = SwarmNode::spawn(Some((daemon_id, daemon_addr)), Ok).await;

    let response = swarm
        .call(daemon_id, PROTO, b"from-swarm")
        .await
        .expect("a Go daemon must accept a native swarm's unary call");
    assert_eq!(response, b"daemon:from-swarm");

    rec.metric("response_len", response.len());
    rec.finish(true);
}

/// Go daemon → native swarm: go-libp2p dials us, proposes the bare name, and
/// our behaviour serves the call.
#[tokio::test]
async fn daemon_calls_swarm_handler() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::unary_swarm::daemon_calls_swarm",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let swarm = SwarmNode::spawn(None, |data| {
        let mut out = b"swarm:".to_vec();
        out.extend_from_slice(&data);
        Ok(out)
    })
    .await;

    daemon
        .client
        .connect_peer(&swarm.addr)
        .await
        .expect("daemon must dial the native swarm");

    let response = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon
            .client
            .call_unary_handler(&swarm.peer_id.to_bytes(), PROTO, b"from-daemon"),
    )
    .await
    .expect("call did not time out")
    .expect("a native swarm must serve a Go daemon's unary call");
    assert_eq!(response, b"swarm:from-daemon");

    rec.metric("response_len", response.len());
    rec.finish(true);
}

/// Calling a protocol the daemon does not serve must come back as the clean
/// `UnsupportedProtocol` refusal (go-libp2p answering `na`), and the
/// connection must remain usable.
#[tokio::test]
async fn swarm_gets_clean_refusal_from_daemon() {
    require_integration!();
    let rec = MetricsRecorder::start("integration::unary_swarm::clean_refusal", "integration");

    let daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    daemon
        .client
        .add_unary_handler(PROTO, |req: Vec<u8>| async move { Ok(req) }, false)
        .await
        .expect("register unary handler on daemon");

    let (daemon_id, daemon_addr) = daemon_peer(&daemon);
    let swarm = SwarmNode::spawn(Some((daemon_id, daemon_addr)), Ok).await;

    let error = swarm
        .call(daemon_id, "DHTProtocol.rpc_nonexistent", b"x")
        .await
        .expect_err("unserved protocol must be refused");
    assert!(
        matches!(error, UnaryError::UnsupportedProtocol(_)),
        "expected UnsupportedProtocol, got {error:?}"
    );

    let response = swarm
        .call(daemon_id, PROTO, b"still-alive")
        .await
        .expect("the connection must survive the refusal");
    assert_eq!(response, b"still-alive");

    rec.finish(true);
}
