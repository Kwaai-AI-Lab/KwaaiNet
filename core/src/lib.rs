//! # KwaaiNet Core
//!
//! Root crate that re-exports all KwaaiNet components.
//!
//! ## Crates
//!
//! - [`kwaai_p2p`]: P2P networking with libp2p and Kademlia DHT
//! - [`kwaai_inference`]: ML inference engine with Candle
//! - [`kwaai_distributed`]: Distributed ML operations (MoE, averaging)
//! - [`kwaai_compression`]: Gradient compression utilities

pub use kwaai_compression as compression;
pub use kwaai_distributed as distributed;
pub use kwaai_inference as inference;
pub use kwaai_p2p as p2p;

/// KwaaiNet version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
