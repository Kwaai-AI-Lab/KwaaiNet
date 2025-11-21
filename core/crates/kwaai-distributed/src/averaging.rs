//! Decentralized parameter averaging (Hivemind pattern)
//!
//! Implements gradient/parameter averaging without a central server.

use crate::error::{DistributedError, DistributedResult};
use async_trait::async_trait;
use candle_core::Tensor;
use kwaai_compression::{BlockwiseQuantizer, Compressor, QuantizedTensor};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Result of an averaging step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AveragingResult {
    /// Averaging successful
    Success {
        /// Number of peers that participated
        peers_count: usize,
        /// Compression ratio achieved
        compression_ratio: f32,
    },
    /// No peers available for averaging
    NoPeersAvailable,
    /// Averaging in progress (not enough peers yet)
    InProgress {
        /// Current number of ready peers
        ready_peers: usize,
        /// Target group size
        target_size: usize,
    },
    /// Averaging failed
    Failed(String),
}

/// Trait for parameter averaging
///
/// Implementors provide decentralized gradient/parameter averaging
/// capabilities without requiring a central server.
#[async_trait]
pub trait ParameterAverager: Send + Sync {
    /// Accumulate local gradients
    fn accumulate(&mut self, gradients: &[Tensor]) -> DistributedResult<()>;

    /// Attempt an averaging step
    ///
    /// This is non-blocking - returns immediately if no peers are available.
    async fn step(&mut self) -> DistributedResult<AveragingResult>;

    /// Get current accumulated gradients
    fn get_accumulated(&self) -> &[Tensor];

    /// Clear accumulated gradients
    fn clear(&mut self);
}

/// Configuration for decentralized averaging
#[derive(Debug, Clone)]
pub struct AveragingConfig {
    /// Target group size for averaging
    pub group_size: usize,
    /// Timeout for peer matching
    pub match_timeout: Duration,
    /// Timeout for gradient exchange
    pub exchange_timeout: Duration,
    /// Block size for quantization
    pub quantization_block_size: usize,
    /// Enable compression
    pub enable_compression: bool,
}

impl Default for AveragingConfig {
    fn default() -> Self {
        Self {
            group_size: 4,
            match_timeout: Duration::from_secs(30),
            exchange_timeout: Duration::from_secs(60),
            quantization_block_size: 64,
            enable_compression: true,
        }
    }
}

/// Decentralized parameter averager
///
/// Implements Hivemind-style gradient averaging:
/// 1. Accumulate local gradients
/// 2. Find peers ready to average
/// 3. Exchange compressed gradients
/// 4. Average and apply
pub struct DecentralizedAverager {
    /// Configuration
    config: AveragingConfig,
    /// Accumulated gradients
    accumulated: Vec<Tensor>,
    /// Compressor for bandwidth efficiency
    compressor: BlockwiseQuantizer,
    /// Number of accumulation steps
    accumulation_count: usize,
}

impl DecentralizedAverager {
    /// Create a new averager
    pub fn new(config: AveragingConfig) -> Self {
        let compressor = BlockwiseQuantizer::new(config.quantization_block_size);
        Self {
            config,
            accumulated: Vec::new(),
            compressor,
            accumulation_count: 0,
        }
    }

    /// Compress gradients for transmission
    pub fn compress_gradients(&self, gradients: &[Tensor]) -> DistributedResult<Vec<QuantizedTensor>> {
        gradients
            .iter()
            .map(|g| self.compressor.compress(g).map_err(DistributedError::from))
            .collect()
    }

    /// Decompress received gradients
    pub fn decompress_gradients(&self, compressed: &[QuantizedTensor]) -> DistributedResult<Vec<Tensor>> {
        compressed
            .iter()
            .map(|c| self.compressor.decompress(c).map_err(DistributedError::from))
            .collect()
    }

    /// Average multiple gradient sets
    pub fn average_gradients(&self, gradient_sets: &[Vec<Tensor>]) -> DistributedResult<Vec<Tensor>> {
        if gradient_sets.is_empty() {
            return Err(DistributedError::AveragingFailed(
                "No gradients to average".to_string(),
            ));
        }

        let num_sets = gradient_sets.len() as f64;
        let first_set = &gradient_sets[0];

        first_set
            .iter()
            .enumerate()
            .map(|(i, _)| {
                // Sum gradients at position i from all sets
                let mut sum = gradient_sets[0][i].clone();
                for set in gradient_sets.iter().skip(1) {
                    sum = (&sum + &set[i])?;
                }
                // Divide by number of sets
                Ok((sum / num_sets)?)
            })
            .collect()
    }
}

#[async_trait]
impl ParameterAverager for DecentralizedAverager {
    fn accumulate(&mut self, gradients: &[Tensor]) -> DistributedResult<()> {
        if self.accumulated.is_empty() {
            // First accumulation - just clone
            self.accumulated = gradients.to_vec();
        } else {
            // Add to existing
            if self.accumulated.len() != gradients.len() {
                return Err(DistributedError::AveragingFailed(format!(
                    "Gradient count mismatch: {} vs {}",
                    self.accumulated.len(),
                    gradients.len()
                )));
            }
            for (acc, grad) in self.accumulated.iter_mut().zip(gradients.iter()) {
                *acc = (acc.clone() + grad)?;
            }
        }
        self.accumulation_count += 1;
        Ok(())
    }

    async fn step(&mut self) -> DistributedResult<AveragingResult> {
        if self.accumulated.is_empty() {
            return Ok(AveragingResult::NoPeersAvailable);
        }

        // TODO: Actual P2P implementation would:
        // 1. Advertise readiness in DHT
        // 2. Find other ready peers
        // 3. Form averaging group
        // 4. Exchange compressed gradients
        // 5. Average and apply

        // For now, just return success with local-only averaging
        if self.accumulation_count > 0 {
            let count = self.accumulation_count as f64;
            for acc in &mut self.accumulated {
                *acc = (acc.clone() / count)?;
            }
            self.accumulation_count = 0;

            Ok(AveragingResult::Success {
                peers_count: 1,
                compression_ratio: 1.0,
            })
        } else {
            Ok(AveragingResult::NoPeersAvailable)
        }
    }

    fn get_accumulated(&self) -> &[Tensor] {
        &self.accumulated
    }

    fn clear(&mut self) {
        self.accumulated.clear();
        self.accumulation_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    #[tokio::test]
    async fn test_averaging() {
        let config = AveragingConfig::default();
        let mut averager = DecentralizedAverager::new(config);

        // Create some test gradients
        let grad1 = Tensor::from_vec(vec![1.0f32, 2.0, 3.0], &[3], &Device::Cpu).unwrap();
        let grad2 = Tensor::from_vec(vec![4.0f32, 5.0, 6.0], &[3], &Device::Cpu).unwrap();

        // Accumulate
        averager.accumulate(&[grad1]).unwrap();
        averager.accumulate(&[grad2]).unwrap();

        // Average
        let result = averager.step().await.unwrap();

        match result {
            AveragingResult::Success { peers_count, .. } => {
                assert_eq!(peers_count, 1);

                // Check averaged values
                let averaged = averager.get_accumulated();
                let values: Vec<f32> = averaged[0].to_vec1().unwrap();
                assert!((values[0] - 2.5).abs() < 0.01);
                assert!((values[1] - 3.5).abs() < 0.01);
                assert!((values[2] - 4.5).abs() < 0.01);
            }
            _ => panic!("Expected success"),
        }
    }
}
