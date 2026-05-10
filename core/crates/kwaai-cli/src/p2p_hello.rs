//! Direct-message ("hello") protocol over the libp2p fabric.
//!
//! Protocol ID: `/kwaai/p2p/hello/1.0.0`
//!
//! A minimal request/response RPC that lets one node send a short message to
//! another and surfaces the message in the recipient's logs. Doubles as the
//! example we point at when explaining how to plug a custom protocol into the
//! KwaaiNet p2p fabric — see [`make_handler`] for the server side and
//! [`send`] for the client side.
//!
//! Wire format: [`HelloRequest`] / [`HelloResponse`] serialised with msgpack
//! (matches the convention established by `/kwaai/inference/1.0.0` in
//! `block_rpc.rs`).

use anyhow::{Context, Result};
use kwaai_p2p_daemon::{error::Result as DaemonResult, P2PClient};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tracing::info;

/// libp2p protocol string registered with the p2p daemon.
pub const HELLO_PROTO: &str = "/kwaai/p2p/hello/1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloRequest {
    /// Sender's peer ID, base58-encoded (e.g. `12D3KooW…`). Filled in by the
    /// caller from the local node's identity.
    pub from: String,
    /// The message body. Plain UTF-8 — no length cap is enforced here, but
    /// the unary-RPC wire imposes its own framing limits.
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloResponse {
    /// Always `true` for now — kept as a struct (rather than a unit type) so
    /// future versions can add fields without breaking the wire format.
    pub ok: bool,
}

/// Send a hello message to `peer_id` over an already-established connection.
///
/// Returns `Ok(())` once the recipient has acknowledged the message.
pub async fn send(client: &P2PClient, peer_id: &PeerId, from: &PeerId, msg: &str) -> Result<()> {
    let req = HelloRequest {
        from: from.to_base58(),
        msg: msg.to_string(),
    };
    let req_bytes = rmp_serde::to_vec_named(&req).context("serialise HelloRequest")?;
    let resp_bytes = client
        .call_unary_handler(&peer_id.to_bytes(), HELLO_PROTO, &req_bytes)
        .await
        .context("call_unary_handler")?;
    let _resp: HelloResponse =
        rmp_serde::from_slice(&resp_bytes).context("deserialise HelloResponse")?;
    Ok(())
}

/// Build a unary handler suitable for [`P2PClient::add_unary_handler`].
///
/// On every inbound message, prints the message to stdout (so it's visible in
/// `docker logs` even with default tracing filters) and emits a `tracing::info!`
/// event for structured-log consumers.
#[allow(clippy::type_complexity)]
pub fn make_handler(
) -> impl Fn(Vec<u8>) -> Pin<Box<dyn std::future::Future<Output = DaemonResult<Vec<u8>>> + Send>>
       + Send
       + Sync
       + 'static {
    |data: Vec<u8>| {
        Box::pin(async move {
            let req: HelloRequest = match rmp_serde::from_slice(&data) {
                Ok(req) => req,
                Err(e) => {
                    let err = format!("hello: bad msgpack: {e}");
                    println!("⚠️  {err}");
                    return Err(kwaai_p2p_daemon::error::Error::Protocol(err));
                }
            };
            // Belt-and-braces: stdout for demo visibility, tracing for structured
            // log consumers. The leading emoji makes the line easy to spot in a
            // wall of debug output.
            println!("💬 [p2p hello] from {}: {}", req.from, req.msg);
            info!(from = %req.from, msg = %req.msg, "p2p hello received");

            let resp = HelloResponse { ok: true };
            rmp_serde::to_vec_named(&resp).map_err(|e| {
                kwaai_p2p_daemon::error::Error::Protocol(format!(
                    "hello: serialise HelloResponse: {e}"
                ))
            })
        })
    }
}
