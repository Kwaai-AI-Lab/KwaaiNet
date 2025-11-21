//! # kwaai-wasm
//!
//! WASM bindings for KwaaiNet browser integration.
//!
//! This crate provides JavaScript bindings for running KwaaiNet
//! in web browsers via WebAssembly.
//!
//! ## Usage
//!
//! ```javascript
//! import init, { KwaaiNet } from './kwaai_wasm.js';
//!
//! async function main() {
//!     await init();
//!     const kwaainet = new KwaaiNet();
//!     await kwaainet.initialize({ services: ['compute'] });
//!
//!     const result = await kwaainet.generate("Hello, world!");
//!     console.log(result);
//! }
//! ```

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use tracing_wasm;

/// Initialize WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // Initialize tracing for browser console
    #[cfg(target_arch = "wasm32")]
    tracing_wasm::set_as_global_default();
}

/// Main KwaaiNet interface for browser
#[wasm_bindgen]
pub struct KwaaiNet {
    /// Whether initialized
    initialized: bool,
    /// Configuration
    config: KwaaiNetConfig,
}

/// Configuration for KwaaiNet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KwaaiNetConfig {
    /// Services to enable
    pub services: Vec<String>,
    /// Maximum memory usage (MB)
    pub max_memory_mb: usize,
    /// Enable distributed inference
    pub enable_distributed: bool,
}

impl Default for KwaaiNetConfig {
    fn default() -> Self {
        Self {
            services: vec!["compute".to_string()],
            max_memory_mb: 1024,
            enable_distributed: false,
        }
    }
}

#[wasm_bindgen]
impl KwaaiNet {
    /// Create a new KwaaiNet instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            initialized: false,
            config: KwaaiNetConfig::default(),
        }
    }

    /// Initialize with configuration
    #[wasm_bindgen]
    pub async fn initialize(&mut self, config: JsValue) -> Result<(), JsError> {
        // Parse configuration
        if !config.is_undefined() && !config.is_null() {
            self.config = serde_wasm_bindgen::from_value(config)
                .map_err(|e| JsError::new(&format!("Invalid config: {}", e)))?;
        }

        tracing::info!("Initializing KwaaiNet with config: {:?}", self.config);

        // TODO: Initialize P2P, inference engine, etc.

        self.initialized = true;
        Ok(())
    }

    /// Check if initialized
    #[wasm_bindgen]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Generate text from a prompt
    #[wasm_bindgen]
    pub async fn generate(&self, prompt: String) -> Result<String, JsError> {
        if !self.initialized {
            return Err(JsError::new("KwaaiNet not initialized"));
        }

        tracing::debug!("Generate called with prompt: {}", prompt);

        // TODO: Actual inference implementation
        Ok(format!("[Generated response for: {}]", prompt))
    }

    /// Connect to the P2P network
    #[wasm_bindgen]
    pub async fn connect(&mut self, bootstrap_peers: Vec<String>) -> Result<(), JsError> {
        if !self.initialized {
            return Err(JsError::new("KwaaiNet not initialized"));
        }

        tracing::info!("Connecting to {} bootstrap peers", bootstrap_peers.len());

        // TODO: P2P connection implementation
        Ok(())
    }

    /// Start contributing compute to the network
    #[wasm_bindgen]
    pub async fn start_contributing(&mut self) -> Result<(), JsError> {
        if !self.initialized {
            return Err(JsError::new("KwaaiNet not initialized"));
        }

        tracing::info!("Starting compute contribution");

        // TODO: Register as compute provider
        Ok(())
    }

    /// Stop contributing
    #[wasm_bindgen]
    pub fn stop_contributing(&mut self) {
        tracing::info!("Stopping compute contribution");
        // TODO: Unregister from network
    }

    /// Get current status
    #[wasm_bindgen]
    pub fn status(&self) -> JsValue {
        let status = Status {
            initialized: self.initialized,
            connected: false, // TODO: Check actual connection
            contributing: false,
            models_loaded: 0,
        };
        serde_wasm_bindgen::to_value(&status).unwrap_or(JsValue::NULL)
    }
}

impl Default for KwaaiNet {
    fn default() -> Self {
        Self::new()
    }
}

/// Status information
#[derive(Debug, Serialize, Deserialize)]
struct Status {
    initialized: bool,
    connected: bool,
    contributing: bool,
    models_loaded: usize,
}

/// Event callback type
#[wasm_bindgen]
extern "C" {
    /// JavaScript callback function type
    #[wasm_bindgen(typescript_type = "(event: string, data: any) => void")]
    pub type EventCallback;
}

/// Version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get supported features
#[wasm_bindgen]
pub fn supported_features() -> JsValue {
    let features = vec![
        "inference",
        "p2p",
        "compression",
        // "distributed", // TODO: Enable when ready
    ];
    serde_wasm_bindgen::to_value(&features).unwrap_or(JsValue::NULL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kwaainet_creation() {
        let kwaainet = KwaaiNet::new();
        assert!(!kwaainet.is_initialized());
    }

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty());
    }
}
