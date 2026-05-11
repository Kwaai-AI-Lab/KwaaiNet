//! Plain-text messaging protocol over the libp2p fabric.
//!
//! Protocol ID: `/kwaai/p2p/hello/1.0.0`
//!
//! A trivial unary RPC: the request payload is a UTF-8 string, the recipient
//! logs it, and replies with `b"ok"`. No wrapper, no schema. Doubles as the
//! canonical example of how to plug a custom protocol into the KwaaiNet p2p
//! fabric — see [`make_handler`] for the server side.
//!
//! Sending side has no helper here on purpose: callers go through
//! [`crate::P2PClient::call_unary_handler`] directly with the bytes they
//! want to send. `peers send --message <text>` and
//! `peers send --payload-hex <hex>` (in `kwaainet`) produce identical wire
//! output for the same bytes — that's the no-magic property we want from a
//! diagnostic tool.
//!
//! Lives in `kwaai-p2p-daemon` rather than `kwaai-cli` because the protocol
//! is daemon-shaped infrastructure (handler registered via
//! [`crate::P2PClient::add_unary_handler`], called via
//! [`crate::P2PClient::call_unary_handler`]) and the CLI is just one
//! caller. Future code that wants to participate in the hello protocol —
//! whether to register the handler, to invoke it, or to reference its
//! protocol ID — imports from here.

use crate::error::Result;
use std::pin::Pin;
use tracing::info;

/// libp2p protocol string registered with the p2p daemon.
pub const HELLO_PROTO: &str = "/kwaai/p2p/hello/1.0.0";

/// Build a unary handler suitable for [`crate::P2PClient::add_unary_handler`].
///
/// Decodes the request payload as UTF-8, prints it to stdout (so it's
/// visible in `docker logs` even with default tracing filters), and emits a
/// `tracing::info!` event for structured-log consumers. Replies with
/// `b"ok"`.
///
/// Non-UTF-8 payloads are decoded with `String::from_utf8_lossy` (replacement
/// chars for invalid sequences) and logged with a leading marker so the
/// operator can see the protocol was misused.
#[allow(clippy::type_complexity)]
pub fn make_handler(
) -> impl Fn(Vec<u8>) -> Pin<Box<dyn std::future::Future<Output = Result<Vec<u8>>> + Send>>
       + Send
       + Sync
       + 'static {
    |data: Vec<u8>| {
        Box::pin(async move {
            match std::str::from_utf8(&data) {
                Ok(msg) => {
                    println!("💬 [p2p hello] {}", msg);
                    info!(msg = %msg, "p2p hello received");
                }
                Err(_) => {
                    let lossy = String::from_utf8_lossy(&data);
                    println!("💬 [p2p hello, non-utf8] {}", lossy);
                    info!(msg = %lossy, "p2p hello received (non-utf8 payload)");
                }
            }
            Ok(b"ok".to_vec())
        })
    }
}
