//! Distributed operations coordinator

use crate::error::DistributedResult;
use crate::moe::{DistributedMoE, MoEConfig};
use crate::averaging::{AveragingConfig, DecentralizedAverager};
use crate::DistributedConfig;

/// Coordinator for all distributed ML operations
pub struct DistributedCoordinator {
    /// Configuration
    config: DistributedConfig,
    /// MoE layer (if enabled)
    moe: Option<DistributedMoE>,
    /// Parameter averager (if enabled)
    averager: Option<DecentralizedAverager>,
    /// Whether coordinator is running
    is_running: bool,
}

impl DistributedCoordinator {
    /// Create a new coordinator
    pub fn new(config: DistributedConfig) -> Self {
        Self {
            config,
            moe: None,
            averager: None,
            is_running: false,
        }
    }

    /// Initialize the coordinator
    pub fn initialize(&mut self) -> DistributedResult<()> {
        if self.config.enable_averaging {
            let averaging_config = AveragingConfig {
                group_size: self.config.averaging_group_size,
                ..Default::default()
            };
            self.averager = Some(DecentralizedAverager::new(averaging_config));
        }

        // MoE initialization would require router weights
        // Left as None for now, to be initialized when model is loaded

        self.is_running = true;
        Ok(())
    }

    /// Check if distributed mode is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enable_moe || self.config.enable_averaging
    }

    /// Get the MoE layer
    pub fn moe(&self) -> Option<&DistributedMoE> {
        self.moe.as_ref()
    }

    /// Get the MoE layer mutably
    pub fn moe_mut(&mut self) -> Option<&mut DistributedMoE> {
        self.moe.as_mut()
    }

    /// Get the averager
    pub fn averager(&self) -> Option<&DecentralizedAverager> {
        self.averager.as_ref()
    }

    /// Get the averager mutably
    pub fn averager_mut(&mut self) -> Option<&mut DecentralizedAverager> {
        self.averager.as_mut()
    }

    /// Check if coordinator is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Stop the coordinator
    pub fn stop(&mut self) {
        self.is_running = false;
    }
}

impl Default for DistributedCoordinator {
    fn default() -> Self {
        Self::new(DistributedConfig::default())
    }
}
