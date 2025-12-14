//! DHT operations for the p2p daemon
//!
//! This module provides high-level DHT operations:
//! - PUT_VALUE: Store a value in the DHT
//! - GET_VALUE: Retrieve a value from the DHT
//! - FIND_PEER: Find a peer's addresses
//! - FIND_PROVIDERS: Find providers for content
//! - PROVIDE: Announce that we provide content

use crate::client::P2PClient;
use crate::error::{Error, Result};
use crate::protocol::p2pd::{DhtRequest, Request, dht_request, request};
use tracing::{debug, trace};

/// DHT peer information
#[derive(Debug, Clone)]
pub struct DhtPeerInfo {
    /// Peer ID (binary)
    pub id: Vec<u8>,
    /// Multiaddrs the peer is listening on
    pub addrs: Vec<Vec<u8>>,
}

/// Result of a DHT GET_VALUE operation
#[derive(Debug, Clone)]
pub struct DhtValue {
    /// The value bytes
    pub value: Vec<u8>,
    /// Type of value (if specified)
    pub value_type: Option<i32>,
}

impl P2PClient {
    /// Put a value into the DHT
    ///
    /// # Arguments
    /// * `key` - The key to store the value under (binary)
    /// * `value` - The value to store (binary)
    /// * `timeout_secs` - Optional timeout in seconds
    pub async fn dht_put_value(
        &mut self,
        key: Vec<u8>,
        value: Vec<u8>,
        timeout_secs: Option<i64>,
    ) -> Result<()> {
        debug!("DHT PUT_VALUE: key len={}, value len={}", key.len(), value.len());

        let dht_request = DhtRequest {
            r#type: dht_request::Type::PutValue as i32,
            peer: None,
            cid: None,
            key: Some(key),
            value: Some(value),
            count: None,
            timeout: timeout_secs,
        };

        let request = Request {
            r#type: request::Type::Dht as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: Some(dht_request),
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if response.dht.is_none() {
            return Err(Error::InvalidResponse(
                "Expected DHT response".to_string(),
            ));
        }

        trace!("DHT PUT_VALUE successful");
        Ok(())
    }

    /// Get a value from the DHT
    ///
    /// # Arguments
    /// * `key` - The key to retrieve (binary)
    /// * `timeout_secs` - Optional timeout in seconds
    ///
    /// # Returns
    /// The value if found, or an error if not found
    pub async fn dht_get_value(
        &mut self,
        key: Vec<u8>,
        timeout_secs: Option<i64>,
    ) -> Result<DhtValue> {
        debug!("DHT GET_VALUE: key len={}", key.len());

        let dht_request = DhtRequest {
            r#type: dht_request::Type::GetValue as i32,
            peer: None,
            cid: None,
            key: Some(key),
            value: None,
            count: None,
            timeout: timeout_secs,
        };

        let request = Request {
            r#type: request::Type::Dht as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: Some(dht_request),
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if let Some(dht_response) = response.dht {
            if let Some(value) = dht_response.value {
                trace!("DHT GET_VALUE successful: value len={}", value.len());
                return Ok(DhtValue {
                    value,
                    value_type: Some(dht_response.r#type),
                });
            }
        }

        Err(Error::Dht("Value not found in DHT".to_string()))
    }

    /// Find peer addresses in the DHT
    ///
    /// # Arguments
    /// * `peer_id` - The peer ID to find (binary)
    /// * `timeout_secs` - Optional timeout in seconds
    ///
    /// # Returns
    /// Peer info with addresses
    pub async fn dht_find_peer(
        &mut self,
        peer_id: Vec<u8>,
        timeout_secs: Option<i64>,
    ) -> Result<DhtPeerInfo> {
        debug!("DHT FIND_PEER: peer_id len={}", peer_id.len());

        let dht_request = DhtRequest {
            r#type: dht_request::Type::FindPeer as i32,
            peer: Some(peer_id.clone()),
            cid: None,
            key: None,
            value: None,
            count: None,
            timeout: timeout_secs,
        };

        let request = Request {
            r#type: request::Type::Dht as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: Some(dht_request),
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if let Some(dht_response) = response.dht {
            if let Some(peer_info) = dht_response.peer {
                trace!("DHT FIND_PEER successful: {} addrs", peer_info.addrs.len());
                return Ok(DhtPeerInfo {
                    id: peer_info.id,
                    addrs: peer_info.addrs,
                });
            }
        }

        Err(Error::Dht("Peer not found in DHT".to_string()))
    }

    /// Find providers for content in the DHT
    ///
    /// # Arguments
    /// * `cid` - Content identifier (binary)
    /// * `count` - Number of providers to find
    /// * `timeout_secs` - Optional timeout in seconds
    ///
    /// # Returns
    /// Peer info if a provider is found (note: daemon returns one peer at a time)
    pub async fn dht_find_providers(
        &mut self,
        cid: Vec<u8>,
        count: i32,
        timeout_secs: Option<i64>,
    ) -> Result<Option<DhtPeerInfo>> {
        debug!("DHT FIND_PROVIDERS: cid len={}, count={}", cid.len(), count);

        let dht_request = DhtRequest {
            r#type: dht_request::Type::FindProviders as i32,
            peer: None,
            cid: Some(cid),
            key: None,
            value: None,
            count: Some(count),
            timeout: timeout_secs,
        };

        let request = Request {
            r#type: request::Type::Dht as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: Some(dht_request),
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let response = self.send_request(request).await?;

        if let Some(dht_response) = response.dht {
            if let Some(peer_info) = dht_response.peer {
                trace!("DHT FIND_PROVIDERS successful");
                return Ok(Some(DhtPeerInfo {
                    id: peer_info.id,
                    addrs: peer_info.addrs,
                }));
            }
        }

        Ok(None)
    }

    /// Announce that we provide content
    ///
    /// # Arguments
    /// * `cid` - Content identifier (binary)
    /// * `timeout_secs` - Optional timeout in seconds
    pub async fn dht_provide(
        &mut self,
        cid: Vec<u8>,
        timeout_secs: Option<i64>,
    ) -> Result<()> {
        debug!("DHT PROVIDE: cid len={}", cid.len());

        let dht_request = DhtRequest {
            r#type: dht_request::Type::Provide as i32,
            peer: None,
            cid: Some(cid),
            key: None,
            value: None,
            count: None,
            timeout: timeout_secs,
        };

        let request = Request {
            r#type: request::Type::Dht as i32,
            connect: None,
            stream_open: None,
            stream_handler: None,
            remove_stream_handler: None,
            dht: Some(dht_request),
            conn_manager: None,
            disconnect: None,
            pubsub: None,
        };

        let _response = self.send_request(request).await?;

        trace!("DHT PROVIDE successful");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dht_value_creation() {
        let value = DhtValue {
            value: vec![1, 2, 3, 4],
            value_type: Some(0),
        };
        assert_eq!(value.value.len(), 4);
    }

    #[test]
    fn test_dht_peer_info_creation() {
        let peer = DhtPeerInfo {
            id: vec![1, 2, 3],
            addrs: vec![vec![4, 5, 6]],
        };
        assert_eq!(peer.id.len(), 3);
        assert_eq!(peer.addrs.len(), 1);
    }
}
