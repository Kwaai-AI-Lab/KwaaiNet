//! Rust wrapper for go-libp2p-daemon
//!
//! This crate provides a Rust interface to the go-libp2p-daemon binary,
//! enabling full compatibility with the Hivemind/Petals DHT network.
//!
//! ## Architecture
//!
//! The go-libp2p-daemon runs as a separate process and communicates with
//! our Rust code via IPC:
//! - **Windows**: Named pipes (`//./pipe/name`)
//! - **Linux/macOS**: Unix domain sockets (`/tmp/name.sock`)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use kwaai_p2p_daemon::P2PDaemon;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Start the daemon
//!     let daemon = P2PDaemon::builder()
//!         .dht(true)
//!         .relay(true)
//!         .spawn()
//!         .await?;
//!
//!     // Get a client to communicate with it
//!     let client = daemon.client().await?;
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

pub mod error;
pub mod daemon;
pub mod client;
pub mod protocol;
pub mod dht;
pub mod stream;
pub mod persistent;

pub use error::{Error, Result};
pub use daemon::{P2PDaemon, DaemonBuilder};
pub use client::P2PClient;
pub use dht::{DhtPeerInfo, DhtValue};

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
