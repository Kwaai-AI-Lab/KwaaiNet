//! Hivemind DHT protocol messages
//!
//! This module defines the protocol messages for Hivemind DHT.
//! Instead of using protobuf code generation, we manually implement
//! the structures and serialization to avoid build-time dependencies.
//!
//! **IMPORTANT**: This schema matches Hivemind commit 213bff98a62accb91f254e2afdccbf1d69ebdea9
//! used by Petals. See PROTOCOL.md for details.

use libp2p::PeerId;
use prost::Message;

// ============================================================================
// Enums
// ============================================================================

/// Result type for DHT find operations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum ResultType {
    NotFound = 0,
    FoundRegular = 1,
    FoundDictionary = 2,
}

impl ResultType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::NotFound),
            1 => Some(Self::FoundRegular),
            2 => Some(Self::FoundDictionary),
            _ => None,
        }
    }
}

impl From<ResultType> for i32 {
    fn from(val: ResultType) -> Self {
        val as i32
    }
}

impl TryFrom<i32> for ResultType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_i32(value).ok_or(())
    }
}

impl Default for ResultType {
    fn default() -> Self {
        Self::NotFound
    }
}

// ============================================================================
// Authentication Messages (from auth.proto)
// ============================================================================

/// Access token for authentication
#[derive(Clone, PartialEq, Message)]
pub struct AccessToken {
    #[prost(string, tag = "1")]
    pub username: String,
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: Vec<u8>,
    #[prost(string, tag = "3")]
    pub expiration_time: String,
    #[prost(bytes = "vec", tag = "4")]
    pub signature: Vec<u8>,
}

/// Authentication info for requests
#[derive(Clone, PartialEq, Message)]
pub struct RequestAuthInfo {
    #[prost(message, optional, tag = "1")]
    pub client_access_token: Option<AccessToken>,
    #[prost(bytes = "vec", tag = "2")]
    pub service_public_key: Vec<u8>,
    #[prost(double, tag = "3")]
    pub time: f64,
    #[prost(bytes = "vec", tag = "4")]
    pub nonce: Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub signature: Vec<u8>,
}

/// Authentication info for responses
#[derive(Clone, PartialEq, Message)]
pub struct ResponseAuthInfo {
    #[prost(message, optional, tag = "1")]
    pub service_access_token: Option<AccessToken>,
    #[prost(bytes = "vec", tag = "2")]
    pub nonce: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: Vec<u8>,
}

// ============================================================================
// DHT Messages (from dht.proto)
// ============================================================================

/// Node identification information
#[derive(Clone, PartialEq, Message)]
pub struct NodeInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub node_id: Vec<u8>,
}

/// Ping request - validate connectivity
#[derive(Clone, PartialEq, Message)]
pub struct PingRequest {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<RequestAuthInfo>,
    #[prost(message, optional, tag = "2")]
    pub peer: Option<NodeInfo>,
    #[prost(bool, tag = "3")]
    pub validate: bool,
}

/// Ping response
#[derive(Clone, PartialEq, Message)]
pub struct PingResponse {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<ResponseAuthInfo>,
    #[prost(message, optional, tag = "2")]
    pub peer: Option<NodeInfo>,
    #[prost(double, tag = "4")]  // NOTE: Field 4, not 3!
    pub dht_time: f64,
    #[prost(bool, tag = "5")]    // NOTE: Field 5, not 4!
    pub available: bool,
}

/// Store request - persist key-value pairs
#[derive(Clone, PartialEq, Message)]
pub struct StoreRequest {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<RequestAuthInfo>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: Vec<Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub subkeys: Vec<Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub values: Vec<Vec<u8>>,
    #[prost(double, repeated, tag = "5")]
    pub expiration_time: Vec<f64>,
    #[prost(bool, repeated, tag = "6")]
    pub in_cache: Vec<bool>,
    #[prost(message, optional, tag = "7")]
    pub peer: Option<NodeInfo>,
}

/// Store response
#[derive(Clone, PartialEq, Message)]
pub struct StoreResponse {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<ResponseAuthInfo>,
    #[prost(bool, repeated, tag = "2")]
    pub store_ok: Vec<bool>,
    #[prost(message, optional, tag = "3")]
    pub peer: Option<NodeInfo>,
}

/// Find request - retrieve values and nearest neighbors
#[derive(Clone, PartialEq, Message)]
pub struct FindRequest {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<RequestAuthInfo>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: Vec<Vec<u8>>,
    #[prost(message, optional, tag = "3")]
    pub peer: Option<NodeInfo>,
}

/// Find result - nested message containing value and nearest peers
#[derive(Clone, PartialEq, Message)]
pub struct FindResult {
    #[prost(enumeration = "ResultType", tag = "1")]
    pub result_type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub value: Vec<u8>,
    #[prost(double, tag = "3")]
    pub expiration_time: f64,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub nearest_node_ids: Vec<Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub nearest_peer_ids: Vec<Vec<u8>>,
}

/// Find response
#[derive(Clone, PartialEq, Message)]
pub struct FindResponse {
    #[prost(message, optional, tag = "1")]
    pub auth: Option<ResponseAuthInfo>,
    #[prost(message, repeated, tag = "2")]
    pub results: Vec<FindResult>,
    #[prost(message, optional, tag = "3")]
    pub peer: Option<NodeInfo>,
}

// ============================================================================
// Helper Implementations
// ============================================================================

impl NodeInfo {
    /// Create NodeInfo from libp2p PeerId
    /// NOTE: Hivemind's NodeInfo only has node_id (no separate peer_id field)
    pub fn from_peer_id(peer_id: PeerId) -> Self {
        Self {
            node_id: peer_id.to_bytes(),
        }
    }

    /// Try to parse PeerId from NodeInfo
    pub fn to_peer_id(&self) -> Option<PeerId> {
        PeerId::from_bytes(&self.node_id).ok()
    }
}

impl RequestAuthInfo {
    /// Create minimal auth info for public networks (like Petals)
    /// Most fields are left empty - only time is populated
    pub fn minimal(time: f64) -> Self {
        Self {
            client_access_token: None,
            service_public_key: vec![],
            time,
            nonce: vec![],
            signature: vec![],
        }
    }

    /// Create auth info with current DHT time (minimal version)
    pub fn new() -> Self {
        Self::minimal(crate::value::get_dht_time())
    }
}

impl ResponseAuthInfo {
    /// Create minimal response auth info
    pub fn minimal() -> Self {
        Self {
            service_access_token: None,
            nonce: vec![],
            signature: vec![],
        }
    }

    /// Create default response auth info
    pub fn new() -> Self {
        Self::minimal()
    }
}

impl PingRequest {
    /// Create a new ping request
    pub fn new(peer: NodeInfo, validate: bool) -> Self {
        Self {
            auth: Some(RequestAuthInfo::new()),
            peer: Some(peer),
            validate,
        }
    }
}

impl PingResponse {
    /// Create a new ping response
    pub fn new(peer: NodeInfo, dht_time: f64, available: bool) -> Self {
        Self {
            auth: Some(ResponseAuthInfo::new()),
            peer: Some(peer),
            dht_time,
            available,
        }
    }
}

impl StoreRequest {
    /// Create a new store request with subkeys
    pub fn new(
        peer: NodeInfo,
        keys: Vec<Vec<u8>>,
        subkeys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
        expiration_time: Vec<f64>,
        in_cache: Vec<bool>,
    ) -> Self {
        Self {
            auth: Some(RequestAuthInfo::new()),
            keys,
            subkeys,
            values,
            expiration_time,
            in_cache,
            peer: Some(peer),
        }
    }
}

impl StoreResponse {
    /// Create a new store response
    pub fn new(peer: NodeInfo, store_ok: Vec<bool>) -> Self {
        Self {
            auth: Some(ResponseAuthInfo::new()),
            store_ok,
            peer: Some(peer),
        }
    }
}

impl FindRequest {
    /// Create a new find request
    pub fn new(peer: NodeInfo, keys: Vec<Vec<u8>>) -> Self {
        Self {
            auth: Some(RequestAuthInfo::new()),
            keys,
            peer: Some(peer),
        }
    }
}

impl FindResult {
    /// Create a "not found" result with nearest peers
    pub fn not_found(nearest_node_ids: Vec<Vec<u8>>, nearest_peer_ids: Vec<Vec<u8>>) -> Self {
        Self {
            result_type: ResultType::NotFound as i32,
            value: vec![],
            expiration_time: 0.0,
            nearest_node_ids,
            nearest_peer_ids,
        }
    }

    /// Create a "found regular" result
    pub fn found_regular(value: Vec<u8>, expiration_time: f64, nearest_node_ids: Vec<Vec<u8>>, nearest_peer_ids: Vec<Vec<u8>>) -> Self {
        Self {
            result_type: ResultType::FoundRegular as i32,
            value,
            expiration_time,
            nearest_node_ids,
            nearest_peer_ids,
        }
    }

    /// Create a "found dictionary" result
    pub fn found_dictionary(value: Vec<u8>, expiration_time: f64, nearest_node_ids: Vec<Vec<u8>>, nearest_peer_ids: Vec<Vec<u8>>) -> Self {
        Self {
            result_type: ResultType::FoundDictionary as i32,
            value,
            expiration_time,
            nearest_node_ids,
            nearest_peer_ids,
        }
    }
}

impl FindResponse {
    /// Create a new find response
    pub fn new(peer: NodeInfo, results: Vec<FindResult>) -> Self {
        Self {
            auth: Some(ResponseAuthInfo::new()),
            results,
            peer: Some(peer),
        }
    }

    /// Create a "not found" response for all keys with nearest peers
    pub fn not_found(peer: NodeInfo, count: usize, nearest_node_ids: Vec<Vec<u8>>, nearest_peer_ids: Vec<Vec<u8>>) -> Self {
        let results = vec![FindResult::not_found(nearest_node_ids.clone(), nearest_peer_ids.clone()); count];
        Self {
            auth: Some(ResponseAuthInfo::new()),
            results,
            peer: Some(peer),
        }
    }
}
