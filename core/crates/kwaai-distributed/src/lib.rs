//! # kwaai-distributed
//!
//! Distributed ML operations for KwaaiNet, implementing Hivemind patterns.
//!
//! This crate provides:
//!
//! - **Mixture of Experts (MoE)**: Distributed model layers across network
//! - **Decentralized Averaging**: Parameter sync without master node
//! - **Fault Tolerance**: Graceful handling of node failures
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                  kwaai-distributed                   │
//! ├─────────────────┬─────────────────┬─────────────────┤
//! │   MoE Layer     │   Averaging     │  Fault Tolerance │
//! │  (Expert Routing)│ (Gradient Sync) │  (Retry/Fallback)│
//! ├─────────────────┴─────────────────┴─────────────────┤
//! │                    kwaai-p2p                         │
//! │               (P2P Networking / DHT)                 │
//! └─────────────────────────────────────────────────────┘
//! ```

pub mod averaging;
pub mod coordinator;
pub mod error;
pub mod expert;
pub mod moe;

pub use averaging::{AveragingResult, DecentralizedAverager, ParameterAverager};
pub use coordinator::DistributedCoordinator;
pub use error::{DistributedError, DistributedResult};
pub use expert::{Expert, ExpertId, ExpertRegistry};
pub use moe::{ExpertRouter, MixtureOfExperts, Routing};

use async_trait::async_trait;
use candle_core::Tensor;

/// Configuration for distributed operations
#[derive(Debug, Clone)]
pub struct DistributedConfig {
    /// Enable MoE distributed layers
    pub enable_moe: bool,
    /// Enable parameter averaging
    pub enable_averaging: bool,
    /// Number of experts to route to (top-k)
    pub moe_top_k: usize,
    /// Target averaging group size
    pub averaging_group_size: usize,
    /// Timeout for remote operations (ms)
    pub timeout_ms: u64,
    /// Maximum retry attempts
    pub max_retries: usize,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self {
            enable_moe: true,
            enable_averaging: true,
            moe_top_k: 2,
            averaging_group_size: 4,
            timeout_ms: 5000,
            max_retries: 3,
        }
    }
}
