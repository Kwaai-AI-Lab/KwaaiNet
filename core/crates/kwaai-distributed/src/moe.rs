//! Mixture of Experts (MoE) implementation
//!
//! Enables arbitrarily large models by distributing "expert" sublayers
//! across network participants.

use crate::error::{DistributedError, DistributedResult};
use crate::expert::{ExpertId, ExpertRegistry};
use async_trait::async_trait;
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Routing information for MoE layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routing {
    /// Expert indices for each token [batch, seq_len, top_k]
    pub expert_indices: Vec<Vec<ExpertId>>,
    /// Expert weights for each token [batch, seq_len, top_k]
    pub expert_weights: Vec<Vec<f32>>,
    /// Auxiliary load balancing loss
    pub aux_loss: f32,
}

/// Trait for expert routing
///
/// The router determines which experts handle each token.
#[async_trait]
pub trait ExpertRouter: Send + Sync {
    /// Route tokens to experts
    ///
    /// Returns routing information including expert assignments
    /// and weights for each token.
    fn route(&self, hidden_states: &Tensor) -> DistributedResult<Routing>;

    /// Number of experts to route to per token
    fn top_k(&self) -> usize;

    /// Total number of experts
    fn num_experts(&self) -> usize;
}

/// Trait for Mixture of Experts layer
#[async_trait]
pub trait MixtureOfExperts: Send + Sync {
    /// Forward pass through MoE layer
    ///
    /// Routes tokens to experts and combines results.
    async fn forward(&mut self, input: &Tensor) -> DistributedResult<Tensor>;

    /// Get the expert registry
    fn registry(&self) -> &ExpertRegistry;

    /// Get the router
    fn router(&self) -> &dyn ExpertRouter;
}

/// Simple top-k router implementation
pub struct TopKRouter {
    /// Gating weights [hidden_size, num_experts]
    gate_weights: Tensor,
    /// Number of experts to select per token
    top_k: usize,
    /// Total number of experts
    num_experts: usize,
    /// Auxiliary loss coefficient
    #[allow(dead_code)]
    aux_loss_coef: f32,
}

impl TopKRouter {
    /// Create a new top-k router
    pub fn new(gate_weights: Tensor, top_k: usize, num_experts: usize, aux_loss_coef: f32) -> Self {
        Self {
            gate_weights,
            top_k,
            num_experts,
            aux_loss_coef,
        }
    }
}

#[async_trait]
impl ExpertRouter for TopKRouter {
    fn route(&self, hidden_states: &Tensor) -> DistributedResult<Routing> {
        // Compute gating scores
        let scores = hidden_states
            .matmul(&self.gate_weights)
            .map_err(|e| DistributedError::RoutingFailed(e.to_string()))?;

        // Get dimensions
        let dims = scores.dims();
        let _batch_size = if dims.len() > 2 { dims[0] } else { 1 };
        let seq_len = if dims.len() > 2 { dims[1] } else { dims[0] };

        // Placeholder routing - in real implementation, would do proper softmax + top-k
        // For now, return uniform routing to first top_k experts
        let expert_indices: Vec<Vec<ExpertId>> = (0..seq_len)
            .map(|_| (0..self.top_k).map(|i| ExpertId::new(i as u64)).collect())
            .collect();

        let expert_weights: Vec<Vec<f32>> = (0..seq_len)
            .map(|_| vec![1.0 / self.top_k as f32; self.top_k])
            .collect();

        Ok(Routing {
            expert_indices,
            expert_weights,
            aux_loss: 0.0,
        })
    }

    fn top_k(&self) -> usize {
        self.top_k
    }

    fn num_experts(&self) -> usize {
        self.num_experts
    }
}

/// Distributed MoE layer implementation
pub struct DistributedMoE {
    /// Expert router
    router: Box<dyn ExpertRouter>,
    /// Expert registry
    registry: ExpertRegistry,
    /// Configuration
    #[allow(dead_code)]
    config: MoEConfig,
}

/// Configuration for MoE layer
#[derive(Debug, Clone)]
pub struct MoEConfig {
    /// Hidden dimension
    pub hidden_dim: usize,
    /// Number of experts
    pub num_experts: usize,
    /// Top-k experts per token
    pub top_k: usize,
    /// Timeout for remote calls (ms)
    pub timeout_ms: u64,
}

impl Default for MoEConfig {
    fn default() -> Self {
        Self {
            hidden_dim: 4096,
            num_experts: 8,
            top_k: 2,
            timeout_ms: 5000,
        }
    }
}

impl DistributedMoE {
    /// Create a new distributed MoE layer
    pub fn new(router: Box<dyn ExpertRouter>, config: MoEConfig) -> Self {
        info!(
            num_experts = config.num_experts,
            top_k = config.top_k,
            hidden_dim = config.hidden_dim,
            "Creating DistributedMoE layer"
        );
        Self {
            router,
            registry: ExpertRegistry::new(),
            config,
        }
    }

    /// Register an expert (local or remote)
    pub fn register_expert(&mut self, expert: Box<dyn crate::expert::Expert>) {
        debug!("Registering local expert in MoE layer");
        self.registry.register_local(expert);
    }

    /// Register remote expert location
    pub fn register_remote_expert(&mut self, expert_id: ExpertId, peer_id: String) {
        debug!(
            "Registering remote expert {} in MoE layer at peer {}",
            expert_id, peer_id
        );
        self.registry.register_remote(expert_id, peer_id);
    }
}

#[async_trait]
impl MixtureOfExperts for DistributedMoE {
    async fn forward(&mut self, input: &Tensor) -> DistributedResult<Tensor> {
        debug!("MoE forward pass, input shape: {:?}", input.dims());
        // 1. Route tokens to experts
        let routing = self.router.route(input)?;
        debug!("Routing computed: aux_loss={:.4}", routing.aux_loss);

        // 2. For now, just return input (placeholder)
        // Real implementation would:
        // - Partition tokens by expert assignment
        // - Call local experts directly
        // - Call remote experts via P2P
        // - Combine results weighted by routing weights

        Ok(input.clone())
    }

    fn registry(&self) -> &ExpertRegistry {
        &self.registry
    }

    fn router(&self) -> &dyn ExpertRouter {
        self.router.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expert::LocalExpert;
    use candle_core::{DType, Device, Tensor};

    fn make_router(hidden: usize, num_experts: usize, top_k: usize) -> TopKRouter {
        let gate = Tensor::zeros((hidden, num_experts), DType::F32, &Device::Cpu).unwrap();
        TopKRouter::new(gate, top_k, num_experts, 0.01)
    }

    #[test]
    fn test_router_accessors() {
        let router = make_router(64, 8, 2);
        assert_eq!(router.top_k(), 2);
        assert_eq!(router.num_experts(), 8);
    }

    #[test]
    fn test_routing_output_matches_seq_len() {
        let router = make_router(16, 4, 2);
        let seq_len = 5usize;
        let input = Tensor::zeros((seq_len, 16usize), DType::F32, &Device::Cpu).unwrap();
        let routing = router.route(&input).unwrap();
        assert_eq!(routing.expert_indices.len(), seq_len);
        assert_eq!(routing.expert_weights.len(), seq_len);
        for row in &routing.expert_indices {
            assert_eq!(row.len(), 2, "top_k=2 → each token routed to 2 experts");
        }
    }

    #[test]
    fn test_routing_weights_sum_to_one() {
        let router = make_router(8, 4, 2);
        let input = Tensor::zeros((3usize, 8usize), DType::F32, &Device::Cpu).unwrap();
        let routing = router.route(&input).unwrap();
        for row in &routing.expert_weights {
            let sum: f32 = row.iter().sum();
            assert!((sum - 1.0).abs() < 1e-5, "weights sum={sum}");
        }
    }

    #[test]
    fn test_moe_register_local_expert() {
        let router = make_router(16, 4, 2);
        let cfg = MoEConfig {
            hidden_dim: 16,
            num_experts: 4,
            top_k: 2,
            timeout_ms: 1000,
        };
        let mut moe = DistributedMoE::new(Box::new(router), cfg);
        moe.register_expert(Box::new(LocalExpert::new(0, 16)));
        assert!(moe.registry().is_local(crate::expert::ExpertId::new(0)));
        assert!(!moe.registry().is_local(crate::expert::ExpertId::new(1)));
    }

    #[test]
    fn test_moe_register_remote_expert() {
        let router = make_router(16, 4, 2);
        let cfg = MoEConfig::default();
        let mut moe = DistributedMoE::new(Box::new(router), cfg);
        moe.register_remote_expert(crate::expert::ExpertId::new(5), "peer-abc".to_string());
        assert_eq!(
            moe.registry()
                .get_remote_peer(crate::expert::ExpertId::new(5)),
            Some(&"peer-abc".to_string())
        );
    }

    #[tokio::test]
    async fn test_moe_forward_returns_same_shape() {
        let router = make_router(8, 2, 1);
        let cfg = MoEConfig {
            hidden_dim: 8,
            num_experts: 2,
            top_k: 1,
            timeout_ms: 1000,
        };
        let mut moe = DistributedMoE::new(Box::new(router), cfg);
        let input = Tensor::zeros((2usize, 8usize), DType::F32, &Device::Cpu).unwrap();
        let output = moe.forward(&input).await.unwrap();
        assert_eq!(output.dims(), input.dims());
    }
}
