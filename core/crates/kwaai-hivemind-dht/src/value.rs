//! DHT value types with expiration handling

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

/// DHT expiration time (seconds since UNIX epoch as float)
pub type DHTExpiration = f64;

/// Get current DHT time (seconds since UNIX epoch)
pub fn get_dht_time() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// Value with expiration metadata
///
/// Hivemind DHT stores all values with expiration timestamps.
/// Nodes prefer values with higher expiration times and may delete
/// any value past its expiration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHTValue {
    /// The actual value (MessagePack-serializable)
    pub value: Vec<u8>,

    /// Expiration time (seconds since UNIX epoch)
    pub expiration_time: DHTExpiration,

    /// Whether this value is in cache tier (subject to LRU eviction)
    pub in_cache: bool,
}

impl DHTValue {
    /// Create a new DHT value with expiration
    pub fn new(value: Vec<u8>, expiration_time: DHTExpiration) -> Self {
        debug!("Creating DHTValue: {} bytes, expires at {:.0}", value.len(), expiration_time);
        Self {
            value,
            expiration_time,
            in_cache: false,
        }
    }

    /// Create a cached DHT value (subject to eviction)
    pub fn new_cached(value: Vec<u8>, expiration_time: DHTExpiration) -> Self {
        Self {
            value,
            expiration_time,
            in_cache: true,
        }
    }

    /// Create a value with TTL (time-to-live) in seconds
    pub fn with_ttl(value: Vec<u8>, ttl_seconds: f64) -> Self {
        let expiration_time = get_dht_time() + ttl_seconds;
        Self::new(value, expiration_time)
    }

    /// Check if the value has expired
    pub fn is_expired(&self) -> bool {
        self.expiration_time < get_dht_time()
    }

    /// Check if the value is still valid
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Get remaining time until expiration (seconds)
    pub fn time_until_expiration(&self) -> f64 {
        self.expiration_time - get_dht_time()
    }

    /// Serialize value to MessagePack
    pub fn to_msgpack(&self) -> Result<Vec<u8>> {
        Ok(rmp_serde::to_vec(&self.value)?)
    }

    /// Deserialize value from MessagePack
    pub fn from_msgpack(bytes: &[u8]) -> Result<Vec<u8>> {
        Ok(rmp_serde::from_slice(bytes)?)
    }

    /// Serialize a Rust type to MessagePack for DHT storage
    pub fn serialize<T: Serialize>(value: &T, ttl_seconds: f64) -> Result<Self> {
        let bytes = rmp_serde::to_vec(value)?;
        Ok(Self::with_ttl(bytes, ttl_seconds))
    }

    /// Deserialize a Rust type from DHT value
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        if self.is_expired() {
            warn!("Attempt to deserialize expired DHT value (expired {:.1}s ago)", -self.time_until_expiration());
            return Err(Error::Expired(self.expiration_time));
        }
        Ok(rmp_serde::from_slice(&self.value)?)
    }
}

/// Builder for creating DHT values with options
pub struct DHTValueBuilder {
    value: Vec<u8>,
    expiration_time: Option<DHTExpiration>,
    ttl_seconds: Option<f64>,
    in_cache: bool,
}

impl DHTValueBuilder {
    /// Create a new builder with raw bytes
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            expiration_time: None,
            ttl_seconds: None,
            in_cache: false,
        }
    }

    /// Create a new builder from a serializable type
    pub fn from_type<T: Serialize>(value: &T) -> Result<Self> {
        let bytes = rmp_serde::to_vec(value)?;
        Ok(Self::new(bytes))
    }

    /// Set absolute expiration time
    pub fn expiration_time(mut self, time: DHTExpiration) -> Self {
        self.expiration_time = Some(time);
        self.ttl_seconds = None;
        self
    }

    /// Set time-to-live in seconds
    pub fn ttl_seconds(mut self, ttl: f64) -> Self {
        self.ttl_seconds = Some(ttl);
        self.expiration_time = None;
        self
    }

    /// Mark as cached value (subject to LRU eviction)
    pub fn cached(mut self) -> Self {
        self.in_cache = true;
        self
    }

    /// Build the DHT value
    pub fn build(self) -> DHTValue {
        let expiration_time = if let Some(exp) = self.expiration_time {
            exp
        } else if let Some(ttl) = self.ttl_seconds {
            get_dht_time() + ttl
        } else {
            // Default: 1 hour TTL
            get_dht_time() + 3600.0
        };

        DHTValue {
            value: self.value,
            expiration_time,
            in_cache: self.in_cache,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dht_value_expiration() {
        let value = DHTValue::with_ttl(vec![1, 2, 3], 10.0);
        assert!(value.is_valid());
        assert!(!value.is_expired());
        assert!(value.time_until_expiration() > 0.0);
    }

    #[test]
    fn test_expired_value() {
        let value = DHTValue::new(vec![1, 2, 3], 0.0);
        assert!(!value.is_valid());
        assert!(value.is_expired());
    }

    #[test]
    fn test_serialize_deserialize() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestData {
            name: String,
            count: u32,
        }

        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        let dht_value = DHTValue::serialize(&data, 3600.0).unwrap();
        assert!(dht_value.is_valid());

        let decoded: TestData = dht_value.deserialize().unwrap();
        assert_eq!(decoded, data);
    }
}
