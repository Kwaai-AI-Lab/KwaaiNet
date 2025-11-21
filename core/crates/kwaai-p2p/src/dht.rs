//! DHT (Distributed Hash Table) operations

use crate::error::{P2PError, P2PResult};
use async_trait::async_trait;
use libp2p::PeerId;
use std::collections::HashMap;
use tracing::{debug, info};

/// DHT manager for Kademlia operations
pub struct DhtManager {
    /// Local cache of DHT records
    local_cache: HashMap<String, Vec<u8>>,
    /// Providers cache
    providers_cache: HashMap<String, Vec<PeerId>>,
}

impl DhtManager {
    /// Create a new DHT manager
    pub fn new() -> Self {
        Self {
            local_cache: HashMap::new(),
            providers_cache: HashMap::new(),
        }
    }

    /// Store a value in the DHT
    pub async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()> {
        debug!("DHT put: {} ({} bytes)", key, value.len());
        self.local_cache.insert(key.to_string(), value);
        // TODO: Actually put to Kademlia DHT via swarm
        Ok(())
    }

    /// Get a value from the DHT
    pub async fn get(&self, key: &str) -> P2PResult<Option<Vec<u8>>> {
        debug!("DHT get: {}", key);
        // TODO: Actually get from Kademlia DHT via swarm
        Ok(self.local_cache.get(key).cloned())
    }

    /// Announce as provider for a key
    pub async fn provide(&mut self, key: &str) -> P2PResult<()> {
        info!("DHT provide: {}", key);
        // TODO: Actually provide via Kademlia
        Ok(())
    }

    /// Find providers for a key
    pub async fn find_providers(&self, key: &str) -> P2PResult<Vec<PeerId>> {
        debug!("DHT find providers: {}", key);
        // TODO: Actually query Kademlia for providers
        Ok(self.providers_cache.get(key).cloned().unwrap_or_default())
    }
}

impl Default for DhtManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dht_put_get() {
        let mut dht = DhtManager::new();
        let key = "test-key";
        let value = b"test-value".to_vec();

        dht.put(key, value.clone()).await.unwrap();
        let result = dht.get(key).await.unwrap();

        assert_eq!(result, Some(value));
    }
}
