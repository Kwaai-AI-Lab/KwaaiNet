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

pub mod client;
pub mod codec;
pub mod error;
pub mod protocol;
pub mod server;
pub mod storage;
pub mod value;
pub mod wire;

pub use client::HivemindDHT;
pub use error::{Error, Result};
pub use protocol::{
    dht_id_from_peer_id, AccessToken, FindResult, NodeInfo, RequestAuthInfo, ResponseAuthInfo,
    ResultType,
};
pub use server::{DHTStorage, RoutingPeer, IS_DICTIONARY, IS_REGULAR_VALUE};
pub use storage::{parse_dictionary, serialize_dictionary, LocalStorage, ParsedDictionary, Stored};
pub use value::{DHTExpiration, DHTValue};

/// Hivemind DHT protocol IDs, as they appear on the libp2p wire.
///
/// Hivemind derives these from the servicer class and method name —
/// `{ClassName}.rpc_{method}` — with **no leading slash**. That is what a real
/// p2pd negotiates, proven in `07_wire_interop`'s `slashless_protocol_negotiates`
/// and recorded under "Resolved verification items" in
/// `docs/NATIVE_P2P_MIGRATION.md`; using a slashed name here would fail to
/// negotiate against every Python peer on the network.
///
/// Note `libp2p::StreamProtocol` refuses slash-less names locally, so these
/// cannot be fed to it directly; `kwaai-p2p`'s `UnaryProtocol` (plus the vendored
/// `multistream-select` patch in `core/patches/`) is the path that carries them.
pub const PROTOCOL_PING: &str = "DHTProtocol.rpc_ping";
pub const PROTOCOL_STORE: &str = "DHTProtocol.rpc_store";
pub const PROTOCOL_FIND: &str = "DHTProtocol.rpc_find";
