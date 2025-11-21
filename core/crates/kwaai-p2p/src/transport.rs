//! Transport layer abstractions
//!
//! This module provides transport configurations for different environments:
//! - TCP for native applications
//! - WebRTC for browser environments (WASM)

/// Transport type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    /// TCP transport for native applications
    Tcp,
    /// WebRTC transport for browsers
    WebRTC,
    /// QUIC transport (experimental)
    Quic,
}

impl Default for TransportType {
    fn default() -> Self {
        Self::Tcp
    }
}

/// Transport configuration
#[derive(Debug, Clone)]
pub struct TransportConfig {
    /// Type of transport to use
    pub transport_type: TransportType,
    /// Enable port reuse
    pub port_reuse: bool,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            transport_type: TransportType::Tcp,
            port_reuse: true,
            timeout_secs: 30,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl TransportConfig {
    /// Create default config for WASM environment
    pub fn wasm_default() -> Self {
        Self {
            transport_type: TransportType::WebRTC,
            port_reuse: false,
            timeout_secs: 60,
        }
    }
}
