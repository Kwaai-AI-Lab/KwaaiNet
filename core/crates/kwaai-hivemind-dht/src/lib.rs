//! Hivemind DHT protocol implementation for KwaaiNet
//!
//! This crate provides a Rust implementation of the Hivemind DHT protocol
//! used by the Petals distributed ML network. It implements the custom
//! DHT protocol with:
//!
//! - DHTValue wrapper with expiration timestamps
//! - Unified FIND RPC (value retrieval + routing)
//! - Batch STORE operations
//! - MessagePack serialization for values
//! - Protobuf wire format for RPC messages

pub mod error;
pub mod protocol;
pub mod value;
pub mod client;
pub mod codec;
pub mod server;

pub use error::{Error, Result};
pub use value::{DHTValue, DHTExpiration};
pub use client::HivemindDHT;
pub use server::DHTStorage;
pub use protocol::{ResultType, FindResult, NodeInfo, AccessToken, RequestAuthInfo, ResponseAuthInfo};

/// Hivemind DHT protocol handlers
/// These match the Python implementation's handler names: /{ClassName}.rpc_{method}
/// libp2p requires protocol names to start with /
pub const PROTOCOL_PING: &str = "/DHTProtocol.rpc_ping";
pub const PROTOCOL_STORE: &str = "/DHTProtocol.rpc_store";
pub const PROTOCOL_FIND: &str = "/DHTProtocol.rpc_find";
