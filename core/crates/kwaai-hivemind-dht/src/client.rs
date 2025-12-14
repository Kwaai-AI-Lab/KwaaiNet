//! Hivemind DHT client for get/store operations

use crate::codec::{DHTRequest, DHTResponse, HivemindCodec};
use crate::protocol::*;
use crate::value::{DHTExpiration, DHTValue};
use crate::{Error, Result, PROTOCOL_FIND, PROTOCOL_STORE};
use libp2p::request_response::{self, OutboundRequestId, ProtocolSupport};
use libp2p::{PeerId, StreamProtocol};
use std::collections::HashMap;
use tracing::{debug, warn};

/// Hivemind DHT client for storing and retrieving values
///
/// This client uses libp2p request-response to communicate with
/// DHT nodes using the Hivemind protocol.
pub struct HivemindDHT {
    /// Local peer ID
    local_peer_id: PeerId,

    /// Request-response behaviour
    behaviour: request_response::Behaviour<HivemindCodec>,

    /// Pending requests
    pending_requests: HashMap<OutboundRequestId, PendingRequest>,
}

#[derive(Debug)]
enum PendingRequest {
    Get { keys: Vec<Vec<u8>> },
    Store { keys: Vec<Vec<u8>> },
}

impl HivemindDHT {
    /// Create a new Hivemind DHT client with Petals-compatible protocol names
    pub fn new(local_peer_id: PeerId) -> Self {
        // Petals uses: DHTProtocol.rpc_store and DHTProtocol.rpc_find
        let protocols = vec![
            (StreamProtocol::new(PROTOCOL_STORE), ProtocolSupport::Full),
            (StreamProtocol::new(PROTOCOL_FIND), ProtocolSupport::Full),
        ];

        let cfg = request_response::Config::default();
        let behaviour = request_response::Behaviour::new(protocols, cfg);

        Self {
            local_peer_id,
            behaviour,
            pending_requests: HashMap::new(),
        }
    }

    /// Get the underlying request-response behaviour
    pub fn behaviour(&self) -> &request_response::Behaviour<HivemindCodec> {
        &self.behaviour
    }

    /// Get the underlying request-response behaviour (mutable)
    pub fn behaviour_mut(&mut self) -> &mut request_response::Behaviour<HivemindCodec> {
        &mut self.behaviour
    }

    /// Store a single key-value pair in the DHT
    ///
    /// # Arguments
    /// * `peer` - Target peer to store on
    /// * `key` - DHT key
    /// * `value` - DHT value with expiration
    pub fn store(
        &mut self,
        peer: PeerId,
        key: Vec<u8>,
        value: DHTValue,
    ) -> OutboundRequestId {
        self.store_many(peer, vec![(key, value)])
    }

    /// Store multiple key-value pairs in one request (batch operation)
    ///
    /// This is more efficient than multiple individual stores.
    pub fn store_many(
        &mut self,
        peer: PeerId,
        entries: Vec<(Vec<u8>, DHTValue)>,
    ) -> OutboundRequestId {
        let keys: Vec<Vec<u8>> = entries.iter().map(|(k, _)| k.clone()).collect();
        let values: Vec<Vec<u8>> = entries.iter().map(|(_, v)| v.value.clone()).collect();
        let expiration_times: Vec<DHTExpiration> =
            entries.iter().map(|(_, v)| v.expiration_time).collect();
        let in_cache: Vec<bool> = entries.iter().map(|(_, v)| v.in_cache).collect();

        let sender = NodeInfo::from_peer_id(self.local_peer_id);
        let subkeys = vec![vec![]; keys.len()]; // Empty subkeys for regular values
        let request = StoreRequest::new(sender, keys.clone(), subkeys, values, expiration_times, in_cache);

        debug!(
            "Storing {} keys to peer {}",
            keys.len(),
            peer.to_base58()
        );

        let req_id = self
            .behaviour
            .send_request(&peer, DHTRequest::Store(request));

        self.pending_requests
            .insert(req_id, PendingRequest::Store { keys });

        req_id
    }

    /// Get a single value from the DHT
    ///
    /// # Arguments
    /// * `peer` - Target peer to query
    /// * `key` - DHT key to retrieve
    pub fn get(&mut self, peer: PeerId, key: Vec<u8>) -> OutboundRequestId {
        self.get_many(peer, vec![key])
    }

    /// Get multiple values in one request (batch operation)
    ///
    /// This is the unified FIND operation that returns both values
    /// and nearest peers for routing.
    pub fn get_many(&mut self, peer: PeerId, keys: Vec<Vec<u8>>) -> OutboundRequestId {
        let sender = NodeInfo::from_peer_id(self.local_peer_id);
        let request = FindRequest::new(sender, keys.clone());

        debug!(
            "Finding {} keys from peer {}",
            keys.len(),
            peer.to_base58()
        );

        let req_id = self
            .behaviour
            .send_request(&peer, DHTRequest::Find(request));

        self.pending_requests
            .insert(req_id, PendingRequest::Get { keys });

        req_id
    }

    /// Handle a response from the DHT
    pub fn handle_response(
        &mut self,
        request_id: OutboundRequestId,
        response: DHTResponse,
    ) -> Result<ResponseData> {
        let pending = self
            .pending_requests
            .remove(&request_id)
            .ok_or_else(|| Error::Network("Unknown request ID".to_string()))?;

        match (pending, response) {
            (PendingRequest::Store { keys }, DHTResponse::Store(store_res)) => {
                debug!("Store response: {} results", store_res.store_ok.len());

                let results: Vec<StoreResult> = keys
                    .into_iter()
                    .zip(store_res.store_ok.into_iter())
                    .map(|(key, stored)| StoreResult { key, stored })
                    .collect();

                Ok(ResponseData::Store(results))
            }

            (PendingRequest::Get { keys }, DHTResponse::Find(find_res)) => {
                debug!(
                    "Find response: {} results",
                    find_res.results.len()
                );

                // Collect all nearest peers from all FindResult messages
                let mut all_nearest_peers = Vec::new();

                let results: Vec<GetResult> = keys
                    .into_iter()
                    .zip(find_res.results.into_iter())
                    .map(|(key, find_result)| {
                        let result_type = ResultType::try_from(find_result.result_type)
                            .unwrap_or(ResultType::NotFound);

                        // Collect nearest peers from this result
                        for peer_id_bytes in &find_result.nearest_peer_ids {
                            if let Ok(peer_id) = PeerId::from_bytes(peer_id_bytes) {
                                all_nearest_peers.push(peer_id);
                            }
                        }

                        match result_type {
                            ResultType::NotFound => GetResult {
                                key,
                                value: None,
                            },
                            ResultType::FoundRegular | ResultType::FoundDictionary => {
                                let dht_value = DHTValue {
                                    value: find_result.value,
                                    expiration_time: find_result.expiration_time,
                                    in_cache: false,
                                };

                                if dht_value.is_expired() {
                                    warn!("Received expired value");
                                    GetResult {
                                        key,
                                        value: None,
                                    }
                                } else {
                                    GetResult {
                                        key,
                                        value: Some(dht_value),
                                    }
                                }
                            }
                        }
                    })
                    .collect();

                Ok(ResponseData::Find(FindResponseData {
                    results,
                    nearest_peers: all_nearest_peers,
                }))
            }

            _ => Err(Error::Network(
                "Response type mismatch with request".to_string(),
            )),
        }
    }
}

/// Response data from DHT operations
#[derive(Debug)]
pub enum ResponseData {
    Store(Vec<StoreResult>),
    Find(FindResponseData),
}

/// Result of a store operation
#[derive(Debug)]
pub struct StoreResult {
    pub key: Vec<u8>,
    pub stored: bool,
}

/// Result of a get operation
#[derive(Debug)]
pub struct GetResult {
    pub key: Vec<u8>,
    pub value: Option<DHTValue>,
}

/// Find response with values and routing info
#[derive(Debug)]
pub struct FindResponseData {
    pub results: Vec<GetResult>,
    pub nearest_peers: Vec<PeerId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let peer_id = PeerId::random();
        let _client = HivemindDHT::new(peer_id);
    }
}
