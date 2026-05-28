//! Distributed operations coordinator

use crate::averaging::{AveragingConfig, DecentralizedAverager};
use crate::error::DistributedResult;
use crate::moe::DistributedMoE;
use crate::DistributedConfig;
use tracing::{debug, info};

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
        info!(
            moe = self.config.enable_moe,
            averaging = self.config.enable_averaging,
            "Initializing DistributedCoordinator"
        );
        if self.config.enable_averaging {
            let averaging_config = AveragingConfig {
                group_size: self.config.averaging_group_size,
                ..Default::default()
            };
            self.averager = Some(DecentralizedAverager::new(averaging_config));
            debug!(
                group_size = self.config.averaging_group_size,
                "Parameter averager initialized"
            );
        }

        // MoE initialization would require router weights
        // Left as None for now, to be initialized when model is loaded

        self.is_running = true;
        info!("DistributedCoordinator initialized");
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
        info!("DistributedCoordinator stopping");
        self.is_running = false;
    }
}

impl Default for DistributedCoordinator {
    fn default() -> Self {
        Self::new(DistributedConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disabled_config() -> DistributedConfig {
        DistributedConfig {
            enable_moe: false,
            enable_averaging: false,
            ..DistributedConfig::default()
        }
    }

    #[test]
    fn test_disabled_coordinator_not_enabled() {
        let coord = DistributedCoordinator::new(disabled_config());
        assert!(!coord.is_enabled());
    }

    #[test]
    fn test_not_running_before_init() {
        let coord = DistributedCoordinator::new(DistributedConfig::default());
        assert!(!coord.is_running());
    }

    #[test]
    fn test_initialize_sets_running() {
        let mut coord = DistributedCoordinator::new(DistributedConfig::default());
        coord.initialize().unwrap();
        assert!(coord.is_running());
    }

    #[test]
    fn test_stop_clears_running() {
        let mut coord = DistributedCoordinator::new(DistributedConfig::default());
        coord.initialize().unwrap();
        coord.stop();
        assert!(!coord.is_running());
    }

    #[test]
    fn test_averaging_enabled_creates_averager() {
        let cfg = DistributedConfig {
            enable_moe: false,
            enable_averaging: true,
            ..DistributedConfig::default()
        };
        let mut coord = DistributedCoordinator::new(cfg);
        coord.initialize().unwrap();
        assert!(coord.averager().is_some());
        assert!(coord.moe().is_none());
    }

    #[test]
    fn test_averaging_disabled_no_averager() {
        let mut coord = DistributedCoordinator::new(disabled_config());
        coord.initialize().unwrap();
        assert!(coord.averager().is_none());
    }

    #[test]
    fn test_is_enabled_with_averaging_only() {
        let cfg = DistributedConfig {
            enable_moe: false,
            enable_averaging: true,
            ..DistributedConfig::default()
        };
        let coord = DistributedCoordinator::new(cfg);
        assert!(coord.is_enabled());
    }

    #[test]
    fn test_default_coordinator() {
        let coord = DistributedCoordinator::default();
        // Default config has both enabled
        assert!(coord.is_enabled());
    }
}
