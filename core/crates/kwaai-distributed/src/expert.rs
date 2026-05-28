//! Expert management for Mixture of Experts

use crate::error::DistributedResult;
use async_trait::async_trait;
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Unique identifier for an expert
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpertId(pub u64);

impl ExpertId {
    /// Create a new expert ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for ExpertId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expert-{}", self.0)
    }
}

/// Trait for expert implementations
///
/// Experts are the distributed "layers" in a Mixture of Experts model.
/// Each expert specializes in different aspects of the input.
#[async_trait]
pub trait Expert: Send + Sync {
    /// Get the expert's ID
    fn id(&self) -> ExpertId;

    /// Run forward pass through this expert
    async fn forward(&self, input: &Tensor) -> DistributedResult<Tensor>;

    /// Get expert's hidden dimension
    fn hidden_dim(&self) -> usize;

    /// Check if expert is ready for inference
    fn is_ready(&self) -> bool;
}

/// Registry for tracking available experts (local and remote)
pub struct ExpertRegistry {
    /// Local experts (hosted on this node)
    local_experts: HashMap<ExpertId, Box<dyn Expert>>,

    /// Remote expert locations (expert_id -> peer_id)
    remote_experts: HashMap<ExpertId, String>,

    /// Fallback experts for fault tolerance
    fallbacks: HashMap<ExpertId, Vec<ExpertId>>,
}

impl ExpertRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            local_experts: HashMap::new(),
            remote_experts: HashMap::new(),
            fallbacks: HashMap::new(),
        }
    }

    /// Register a local expert
    pub fn register_local(&mut self, expert: Box<dyn Expert>) {
        let id = expert.id();
        info!("Registering local expert {}", id);
        self.local_experts.insert(id, expert);
    }

    /// Register a remote expert location
    pub fn register_remote(&mut self, expert_id: ExpertId, peer_id: String) {
        info!(
            "Registering remote expert {} at peer {}",
            expert_id, peer_id
        );
        self.remote_experts.insert(expert_id, peer_id);
    }

    /// Check if an expert is local
    pub fn is_local(&self, expert_id: ExpertId) -> bool {
        self.local_experts.contains_key(&expert_id)
    }

    /// Get a local expert
    pub fn get_local(&self, expert_id: ExpertId) -> Option<&dyn Expert> {
        self.local_experts.get(&expert_id).map(|e| e.as_ref())
    }

    /// Get the peer ID for a remote expert
    pub fn get_remote_peer(&self, expert_id: ExpertId) -> Option<&String> {
        self.remote_experts.get(&expert_id)
    }

    /// Register fallback experts
    pub fn register_fallback(&mut self, expert_id: ExpertId, fallbacks: Vec<ExpertId>) {
        self.fallbacks.insert(expert_id, fallbacks);
    }

    /// Get fallback experts for fault tolerance
    pub fn get_fallbacks(&self, expert_id: ExpertId) -> Option<&Vec<ExpertId>> {
        self.fallbacks.get(&expert_id)
    }

    /// Report a failure for health tracking
    pub fn report_failure(&mut self, _expert_id: ExpertId) {
        warn!("Expert {} reported failure", _expert_id);
        // TODO: Track failures and update health scores
    }

    /// List all available experts
    pub fn list_experts(&self) -> Vec<ExpertId> {
        let mut experts: Vec<_> = self.local_experts.keys().copied().collect();
        experts.extend(self.remote_experts.keys().copied());
        experts.sort_by_key(|e| e.0);
        experts.dedup();
        experts
    }
}

impl Default for ExpertRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Placeholder local expert for testing
pub struct LocalExpert {
    id: ExpertId,
    hidden_dim: usize,
}

impl LocalExpert {
    /// Create a new local expert
    pub fn new(id: u64, hidden_dim: usize) -> Self {
        Self {
            id: ExpertId::new(id),
            hidden_dim,
        }
    }
}

#[async_trait]
impl Expert for LocalExpert {
    fn id(&self) -> ExpertId {
        self.id
    }

    async fn forward(&self, input: &Tensor) -> DistributedResult<Tensor> {
        debug!(
            "LocalExpert {} forward pass, input shape: {:?}",
            self.id,
            input.dims()
        );
        // Placeholder: return input as-is
        Ok(input.clone())
    }

    fn hidden_dim(&self) -> usize {
        self.hidden_dim
    }

    fn is_ready(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expert_registry() {
        let mut registry = ExpertRegistry::new();

        // Register a local expert
        let expert = LocalExpert::new(1, 4096);
        registry.register_local(Box::new(expert));

        assert!(registry.is_local(ExpertId::new(1)));
        assert!(!registry.is_local(ExpertId::new(2)));

        // Register a remote expert
        registry.register_remote(ExpertId::new(2), "peer-123".to_string());
        assert!(!registry.is_local(ExpertId::new(2)));
        assert_eq!(
            registry.get_remote_peer(ExpertId::new(2)),
            Some(&"peer-123".to_string())
        );
    }

    #[test]
    fn test_expert_id_display() {
        let id = ExpertId::new(7);
        assert_eq!(id.to_string(), "expert-7");
    }

    #[test]
    fn test_expert_id_equality_and_hash() {
        use std::collections::HashSet;
        let a = ExpertId::new(3);
        let b = ExpertId::new(3);
        let c = ExpertId::new(4);
        assert_eq!(a, b);
        assert_ne!(a, c);
        let mut s = HashSet::new();
        s.insert(a);
        s.insert(b);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_list_experts_sorted() {
        let mut registry = ExpertRegistry::new();
        registry.register_local(Box::new(LocalExpert::new(3, 64)));
        registry.register_local(Box::new(LocalExpert::new(1, 64)));
        registry.register_remote(ExpertId::new(2), "peer-x".to_string());
        let list = registry.list_experts();
        assert_eq!(list, vec![ExpertId::new(1), ExpertId::new(2), ExpertId::new(3)]);
    }

    #[test]
    fn test_list_experts_deduplicates() {
        let mut registry = ExpertRegistry::new();
        registry.register_local(Box::new(LocalExpert::new(5, 64)));
        // Registering same ID as remote too — list_experts deduplicates
        registry.register_remote(ExpertId::new(5), "peer-y".to_string());
        let list = registry.list_experts();
        assert_eq!(list.iter().filter(|&&id| id == ExpertId::new(5)).count(), 1);
    }

    #[test]
    fn test_fallback_registration_and_retrieval() {
        let mut registry = ExpertRegistry::new();
        let fallbacks = vec![ExpertId::new(10), ExpertId::new(11)];
        registry.register_fallback(ExpertId::new(5), fallbacks.clone());
        assert_eq!(registry.get_fallbacks(ExpertId::new(5)), Some(&fallbacks));
        assert_eq!(registry.get_fallbacks(ExpertId::new(99)), None);
    }

    #[test]
    fn test_report_failure_does_not_panic() {
        let mut registry = ExpertRegistry::new();
        // Reporting failure on an unknown expert should not panic
        registry.report_failure(ExpertId::new(999));
    }

    #[test]
    fn test_local_expert_properties() {
        let expert = LocalExpert::new(42, 2048);
        assert_eq!(expert.id(), ExpertId::new(42));
        assert_eq!(expert.hidden_dim(), 2048);
        assert!(expert.is_ready());
    }

    #[tokio::test]
    async fn test_local_expert_forward_passthrough() {
        use candle_core::{DType, Device, Tensor};
        let expert = LocalExpert::new(0, 4);
        let input = Tensor::zeros((2usize, 4usize), DType::F32, &Device::Cpu).unwrap();
        let output = expert.forward(&input).await.unwrap();
        assert_eq!(output.dims(), input.dims());
    }
}
