//! Hivemind DHT server for responding to FIND and STORE requests

use crate::codec::{DHTRequest, DHTResponse};
use crate::protocol::*;
use crate::value::{DHTValue, get_dht_time};
use crate::{Error, Result};
use libp2p::PeerId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info, warn};

/// DHT storage backend
#[derive(Debug, Clone)]
pub struct DHTStorage {
    /// Stored key-value pairs with expiration
    storage: Arc<RwLock<HashMap<Vec<u8>, StoredValue>>>,

    /// Local peer ID
    local_peer_id: PeerId,

    /// Known peers (for nearest peer queries)
    peers: Arc<RwLock<Vec<PeerId>>>,
}

#[derive(Debug, Clone)]
struct StoredValue {
    value: Vec<u8>,
    expiration_time: f64,
    in_cache: bool,
}

impl DHTStorage {
    /// Create a new DHT storage backend
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            local_peer_id,
            peers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Update known peers (from Kademlia routing table)
    pub fn update_peers(&self, peers: Vec<PeerId>) {
        if let Ok(mut peer_list) = self.peers.write() {
            *peer_list = peers;
        }
    }

    /// Clean up expired values
    pub fn cleanup_expired(&self) {
        let now = get_dht_time();
        if let Ok(mut storage) = self.storage.write() {
            storage.retain(|_, v| v.expiration_time > now);
        }
    }

    /// Handle a STORE request
    pub fn handle_store(&self, request: StoreRequest) -> StoreResponse {
        debug!("Handling STORE request with {} keys", request.keys.len());

        let mut store_ok = Vec::new();

        if let Ok(mut storage) = self.storage.write() {
            for (i, key) in request.keys.iter().enumerate() {
                let value = request.values.get(i).cloned().unwrap_or_default();
                let expiration_time = request.expiration_time.get(i).copied().unwrap_or(get_dht_time() + 3600.0);
                let in_cache = request.in_cache.get(i).copied().unwrap_or(false);

                // Only store if not expired
                if expiration_time > get_dht_time() {
                    storage.insert(
                        key.clone(),
                        StoredValue {
                            value,
                            expiration_time,
                            in_cache,
                        },
                    );
                    store_ok.push(true);

                    // // Verbose logging commented - key is SHA1 hash (gibberish)
                    // let key_str = String::from_utf8_lossy(key);
                    // info!("Stored key: {} (expires: {:.0}s)", key_str, expiration_time - get_dht_time());
                } else {
                    warn!("Rejected expired value");
                    store_ok.push(false);
                }
            }
        } else {
            // Storage lock failed
            store_ok = vec![false; request.keys.len()];
        }

        StoreResponse {
            auth: Some(ResponseAuthInfo::new()),
            store_ok,
            peer: Some(NodeInfo::from_peer_id(self.local_peer_id)),
        }
    }

    /// Handle a FIND request
    pub fn handle_find(&self, request: FindRequest) -> FindResponse {
        debug!("Handling FIND request with {} keys", request.keys.len());

        // Get nearest peers once (for all results)
        let (nearest_node_ids, nearest_peer_ids): (Vec<Vec<u8>>, Vec<Vec<u8>>) =
            if let Ok(peers) = self.peers.read() {
                let node_peer_pairs: Vec<_> = peers.iter()
                    .take(20)  // Return up to 20 nearest peers
                    .map(|p| {
                        let node_info = NodeInfo::from_peer_id(*p);
                        let peer_bytes = p.to_bytes();
                        (node_info.node_id, peer_bytes)
                    })
                    .collect();

                node_peer_pairs.into_iter().unzip()
            } else {
                (vec![], vec![])
            };

        let mut results = Vec::new();

        if let Ok(storage) = self.storage.read() {
            for key in &request.keys {
                let find_result = if let Some(stored_value) = storage.get(key) {
                    // Check if still valid
                    if stored_value.expiration_time > get_dht_time() {
                        // // Verbose logging commented - key is SHA1 hash (gibberish)
                        // let key_str = String::from_utf8_lossy(key);
                        // info!("Found key: {} (expires in {:.0}s)", key_str, stored_value.expiration_time - get_dht_time());

                        FindResult {
                            result_type: ResultType::FoundRegular as i32,
                            value: stored_value.value.clone(),
                            expiration_time: stored_value.expiration_time,
                            nearest_node_ids: nearest_node_ids.clone(),
                            nearest_peer_ids: nearest_peer_ids.clone(),
                        }
                    } else {
                        // Expired
                        FindResult {
                            result_type: ResultType::NotFound as i32,
                            value: vec![],
                            expiration_time: 0.0,
                            nearest_node_ids: nearest_node_ids.clone(),
                            nearest_peer_ids: nearest_peer_ids.clone(),
                        }
                    }
                } else {
                    FindResult {
                        result_type: ResultType::NotFound as i32,
                        value: vec![],
                        expiration_time: 0.0,
                        nearest_node_ids: nearest_node_ids.clone(),
                        nearest_peer_ids: nearest_peer_ids.clone(),
                    }
                };

                results.push(find_result);
            }
        }

        FindResponse {
            auth: Some(ResponseAuthInfo::new()),
            results,
            peer: Some(NodeInfo::from_peer_id(self.local_peer_id)),
        }
    }

    /// Handle any DHT request
    pub fn handle_request(&self, request: DHTRequest) -> Result<DHTResponse> {
        match request {
            DHTRequest::Store(store_req) => {
                Ok(DHTResponse::Store(self.handle_store(store_req)))
            }
            DHTRequest::Find(find_req) => {
                Ok(DHTResponse::Find(self.handle_find(find_req)))
            }
            DHTRequest::Ping(_ping_req) => {
                // Simple ping response
                Ok(DHTResponse::Ping(PingResponse {
                    auth: Some(ResponseAuthInfo::new()),
                    peer: Some(NodeInfo::from_peer_id(self.local_peer_id)),
                    dht_time: get_dht_time(),
                    available: true,
                }))
            }
        }
    }

    /// Get statistics about stored data
    pub fn stats(&self) -> (usize, usize) {
        if let Ok(storage) = self.storage.read() {
            let now = get_dht_time();
            let total = storage.len();
            let valid = storage.values().filter(|v| v.expiration_time > now).count();
            (total, valid)
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_find() {
        let peer_id = PeerId::random();
        let storage = DHTStorage::new(peer_id);

        // Store a value
        let node_info = NodeInfo::from_peer_id(peer_id);
        let store_req = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![b"test_key".to_vec()],
            subkeys: vec![vec![]],
            values: vec![b"test_value".to_vec()],
            expiration_time: vec![get_dht_time() + 3600.0],
            in_cache: vec![false],
            peer: Some(node_info),
        };

        let store_res = storage.handle_store(store_req);
        assert_eq!(store_res.store_ok.len(), 1);
        assert!(store_res.store_ok[0]);

        // Find the value
        let find_req = FindRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![b"test_key".to_vec()],
            peer: None,
        };

        let find_res = storage.handle_find(find_req);
        assert_eq!(find_res.results.len(), 1);
        assert_eq!(find_res.results[0].result_type, ResultType::FoundRegular as i32);
        assert_eq!(find_res.results[0].value, b"test_value");
    }

    #[test]
    fn test_expired_value() {
        let peer_id = PeerId::random();
        let storage = DHTStorage::new(peer_id);

        // Try to store an expired value
        let node_info = NodeInfo::from_peer_id(peer_id);
        let store_req = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![b"expired_key".to_vec()],
            subkeys: vec![vec![]],
            values: vec![b"test".to_vec()],
            expiration_time: vec![0.0],  // Already expired
            in_cache: vec![false],
            peer: Some(node_info),
        };

        let store_res = storage.handle_store(store_req);
        assert!(!store_res.store_ok[0]);  // Should not be stored
    }
}
