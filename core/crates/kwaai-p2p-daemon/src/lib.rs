//! The p2pd control protocol — both halves.
//!
//! This crate speaks the go-libp2p-daemon protobuf control protocol, in two
//! directions:
//!
//! - **Client** ([`P2PClient`], [`persistent`], [`dht`]) — what every external
//!   process uses to drive the node: identify, connect, list_peers, DHT verbs,
//!   stream handlers, and the persistent-connection unary sub-protocol.
//! - **Server** ([`ControlServer`]) — the node's own implementation of that same
//!   protocol, translating it into `kwaai_p2p::NetworkHandle` calls.
//!
//! ## Architecture
//!
//! The control socket is IPC:
//! - **Windows**: TCP on loopback
//! - **Linux/macOS**: Unix domain sockets (`/tmp/name.sock`)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Attach to the socket a running node serves.
//!     let mut client = P2PClient::connect(DEFAULT_SOCKET_NAME).await?;
//!
//!     let peer_id = client.identify().await?;
//!     println!("Our peer ID: {}", peer_id);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod dht;
pub mod error;
pub mod hello;
pub mod persistent;
pub mod protocol;
pub mod server;
pub mod stream;

pub use client::{P2PClient, P2PStream};
pub use dht::{DhtPeerInfo, DhtValue};
pub use error::{Error, Result};
pub use server::{default_socket_addr, ControlServer};

// Re-export commonly used types
pub use protocol::p2pd;

/// Default socket name for IPC
#[cfg(windows)]
pub const DEFAULT_SOCKET_NAME: &str = "kwaai-p2pd";

#[cfg(unix)]
pub const DEFAULT_SOCKET_NAME: &str = "/tmp/kwaai-p2pd.sock";
