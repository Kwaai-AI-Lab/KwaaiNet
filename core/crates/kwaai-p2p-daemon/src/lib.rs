//! The p2pd control protocol — both halves.
//!
//! This crate speaks the go-libp2p-daemon protobuf control protocol, in two
//! directions:
//!
//! - **Client** ([`P2PClient`], [`persistent`], [`dht`]) — what every external
//!   process uses to drive the node: identify, connect, list_peers, DHT verbs,
//!   stream handlers, and the persistent-connection unary sub-protocol.
//! - **Server** ([`ControlServer`]) — the node's own implementation of that same
//!   protocol, translating it into `kwaai_p2p::NetworkHandle` calls. Added in
//!   Phase 3 of `docs/NATIVE_P2P_MIGRATION.md` so the Go daemon can be removed
//!   without touching a single client call site.
//!
//! Historically only the client existed and the server was the Go binary
//! ([`P2PDaemon`] spawns it). Both server implementations answer the same bytes;
//! `kwaai-network-tests` tiers 07/09/11 hold them to it.
//!
//! ## Architecture
//!
//! The control socket is IPC:
//! - **Windows**: TCP on loopback (the Go daemon never supported named pipes in
//!   multiaddr form)
//! - **Linux/macOS**: Unix domain sockets (`/tmp/name.sock`)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use kwaai_p2p_daemon::P2PDaemon;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Start the daemon
//!     let mut daemon = P2PDaemon::builder()
//!         .dht(true)
//!         .relay(true)
//!         .spawn()
//!         .await?;
//!
//!     // Get a client to communicate with it
//!     let mut client = daemon.client().await?;
//!
//!     // Use the client...
//!     let peer_id = client.identify().await?;
//!     println!("Our peer ID: {}", peer_id);
//!
//!     // Keep daemon running
//!     daemon.wait().await?;
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod daemon;
pub mod dht;
pub mod error;
pub mod hello;
pub mod persistent;
pub mod protocol;
pub mod server;
pub mod stream;

pub use client::{P2PClient, P2PStream};
pub use daemon::{DaemonBuilder, P2PDaemon};
pub use dht::{DhtPeerInfo, DhtValue};
pub use error::{Error, Result};
pub use server::{default_socket_addr, ControlServer};

// Re-export commonly used types
pub use protocol::p2pd;

/// Path to the compiled p2pd daemon binary
///
/// This is set at compile time by build.rs
pub const DAEMON_BINARY_PATH: &str = env!("P2PD_PATH");

/// Default socket name for IPC
#[cfg(windows)]
pub const DEFAULT_SOCKET_NAME: &str = "kwaai-p2pd";

#[cfg(unix)]
pub const DEFAULT_SOCKET_NAME: &str = "/tmp/kwaai-p2pd.sock";
