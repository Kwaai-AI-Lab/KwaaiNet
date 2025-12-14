//! DHT (Distributed Hash Table) operations

use crate::error::{P2PError, P2PResult};
use async_trait::async_trait;
use libp2p::PeerId;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Commands sent to the swarm to perform DHT operations
#[derive(Debug, Clone)]
pub enum DhtCommand {
    /// Put a record in the DHT
    PutRecord {
        key: String,
        value: Vec<u8>,
        publisher: Option<PeerId>,
    },
    /// Start providing a key
    StartProviding { key: String },
    /// Get a value from the DHT
    GetRecord { key: String },
    /// Find providers for a key
    GetProviders { key: String },
}

/// DHT manager for Kademlia operations
pub struct DhtManager {
    /// Local cache of DHT records
    local_cache: HashMap<String, Vec<u8>>,
    /// Providers cache
    providers_cache: HashMap<String, Vec<PeerId>>,
    /// Channel to send commands to swarm
    command_tx: Option<mpsc::UnboundedSender<DhtCommand>>,
}

impl DhtManager {
    /// Create a new DHT manager without channel (for testing)
    pub fn new() -> Self {
        Self {
            local_cache: HashMap::new(),
            providers_cache: HashMap::new(),
            command_tx: None,
        }
    }

    /// Create a new DHT manager with command channel
    pub fn with_channel(command_tx: mpsc::UnboundedSender<DhtCommand>) -> Self {
        Self {
            local_cache: HashMap::new(),
            providers_cache: HashMap::new(),
            command_tx: Some(command_tx),
        }
    }

    /// Store a value in the DHT
    pub async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()> {
        debug!("DHT put: {} ({} bytes)", key, value.len());

        // Keep local cache for quick access
        self.local_cache.insert(key.to_string(), value.clone());

        // Send command to swarm if channel available
        if let Some(tx) = &self.command_tx {
            tx.send(DhtCommand::PutRecord {
                key: key.to_string(),
                value,
                publisher: None,
            })
            .map_err(|e| P2PError::Internal(format!("Failed to send DHT command: {}", e)))?;

            info!("DHT put command sent: {}", key);
        } else {
            warn!("DHT command channel not available, operation only cached locally");
        }

        Ok(())
    }

    /// Get a value from the DHT
    pub async fn get(&self, key: &str) -> P2PResult<Option<Vec<u8>>> {
        debug!("DHT get: {}", key);

        // For now, return from local cache
        // TODO: Send GetRecord command and wait for response
        Ok(self.local_cache.get(key).cloned())
    }

    /// Announce as provider for a key
    pub async fn provide(&mut self, key: &str) -> P2PResult<()> {
        info!("DHT provide: {}", key);

        // Send command to swarm if channel available
        if let Some(tx) = &self.command_tx {
            tx.send(DhtCommand::StartProviding {
                key: key.to_string(),
            })
            .map_err(|e| P2PError::Internal(format!("Failed to send DHT command: {}", e)))?;

            info!("DHT provide command sent: {}", key);
        } else {
            warn!("DHT command channel not available");
        }

        Ok(())
    }

    /// Find providers for a key
    pub async fn find_providers(&self, key: &str) -> P2PResult<Vec<PeerId>> {
        debug!("DHT find providers: {}", key);

        // For now, return from local cache
        // TODO: Send GetProviders command and wait for response
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
