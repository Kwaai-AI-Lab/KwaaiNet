//! Day 8: Expert Registry Example
//!
//! Demonstrates Mixture of Experts (MoE) infrastructure:
//! - Register local and remote experts
//! - Set up fallback experts for fault tolerance
//! - Simulate expert routing and token distribution
//!
//! Run with: cargo run --example expert_registry

use candle_core::{Device, DType, Tensor};
use kwaai_distributed::{
    expert::{Expert, ExpertId, ExpertRegistry, LocalExpert},
    moe::{DistributedMoE, ExpertRouter, MoEConfig, Routing, TopKRouter},
    DistributedConfig,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet Expert Registry Demo\n");
    println!("==============================\n");

    let device = Device::Cpu;

    // 1. Basic Expert Registry
    println!("1. Expert Registry Basics");
    println!("-------------------------");

    let mut registry = ExpertRegistry::new();

    // Register local experts (running on this node)
    println!("Registering local experts...");
    for i in 0..4 {
        let expert = LocalExpert::new(i, 4096);
        println!(
            "  Local: {} (hidden_dim: {}, ready: {})",
            expert.id(),
            expert.hidden_dim(),
            expert.is_ready()
        );
        registry.register_local(Box::new(expert));
    }

    // Register remote experts (on other nodes)
    println!("\nRegistering remote experts...");
    let remote_peers = [
        ("expert-4", "12D3KooWA...abc123"),
        ("expert-5", "12D3KooWB...def456"),
        ("expert-6", "12D3KooWC...ghi789"),
        ("expert-7", "12D3KooWD...jkl012"),
    ];

    for (i, (name, peer_id)) in remote_peers.iter().enumerate() {
        let expert_id = ExpertId::new((i + 4) as u64);
        registry.register_remote(expert_id, peer_id.to_string());
        println!("  Remote: {} -> peer {}", name, peer_id);
    }

    // List all experts
    println!("\nAll registered experts:");
    for expert_id in registry.list_experts() {
        let location = if registry.is_local(expert_id) {
            "local".to_string()
        } else if let Some(peer) = registry.get_remote_peer(expert_id) {
            format!("remote@{}", &peer[..20.min(peer.len())])
        } else {
            "unknown".to_string()
        };
        println!("  {} -> {}", expert_id, location);
    }

    println!();

    // 2. Fault Tolerance with Fallbacks
    println!("2. Fault Tolerance Setup");
    println!("------------------------");

    let mut registry = ExpertRegistry::new();

    // Create expert groups with fallbacks
    // Group A: experts 0, 1, 2 (can substitute for each other)
    // Group B: experts 3, 4, 5 (can substitute for each other)

    println!("Setting up expert groups with fallbacks...");

    for i in 0..3 {
        registry.register_local(Box::new(LocalExpert::new(i, 4096)));
        // Each expert in group A can fall back to others in the group
        let fallbacks: Vec<ExpertId> = (0..3)
            .filter(|&j| j != i)
            .map(ExpertId::new)
            .collect();
        registry.register_fallback(ExpertId::new(i), fallbacks);
    }

    for i in 3..6 {
        registry.register_remote(ExpertId::new(i), format!("peer-{}", i));
        let fallbacks: Vec<ExpertId> = (3..6)
            .filter(|&j| j != i)
            .map(ExpertId::new)
            .collect();
        registry.register_fallback(ExpertId::new(i), fallbacks);
    }

    // Show fallback configuration
    for expert_id in registry.list_experts() {
        if let Some(fallbacks) = registry.get_fallbacks(expert_id) {
            let fallback_str: Vec<_> = fallbacks.iter().map(|f| f.to_string()).collect();
            println!(
                "  {} -> fallbacks: [{}]",
                expert_id,
                fallback_str.join(", ")
            );
        }
    }

    println!();

    // 3. MoE Configuration
    println!("3. Mixture of Experts Configuration");
    println!("------------------------------------");

    let config = MoEConfig {
        hidden_dim: 4096,
        num_experts: 8,
        top_k: 2,
        timeout_ms: 5000,
    };

    println!("MoE Config:");
    println!("  Hidden dim:   {}", config.hidden_dim);
    println!("  Num experts:  {}", config.num_experts);
    println!("  Top-K:        {}", config.top_k);
    println!("  Timeout:      {} ms", config.timeout_ms);

    let distributed_config = DistributedConfig::default();
    println!("\nDistributed Config:");
    println!("  MoE enabled:         {}", distributed_config.enable_moe);
    println!("  Averaging enabled:   {}", distributed_config.enable_averaging);
    println!("  MoE top-k:           {}", distributed_config.moe_top_k);
    println!("  Averaging group:     {}", distributed_config.averaging_group_size);
    println!("  Max retries:         {}", distributed_config.max_retries);

    println!();

    // 4. Expert Routing Simulation
    println!("4. Expert Routing Simulation");
    println!("----------------------------");

    // Create gate weights for routing
    let hidden_dim = 768;
    let num_experts = 8;
    let gate_weights = Tensor::randn(0f32, 0.02, &[hidden_dim, num_experts], &device)?;

    let router = TopKRouter::new(gate_weights, 2, num_experts, 0.01);

    println!("Router config:");
    println!("  Top-K:       {}", router.top_k());
    println!("  Num experts: {}", router.num_experts());

    // Simulate routing for a batch of tokens
    let batch_size = 2;
    let seq_len = 8;
    let hidden_states = Tensor::randn(0f32, 1.0, &[batch_size, seq_len, hidden_dim], &device)?;
    let flat_hidden = hidden_states.reshape(&[batch_size * seq_len, hidden_dim])?;

    println!("\nInput shape: {:?}", hidden_states.dims());
    println!("Routing {} tokens...", batch_size * seq_len);

    let routing = router.route(&flat_hidden)?;

    println!("\nRouting results:");
    for (i, (indices, weights)) in routing
        .expert_indices
        .iter()
        .zip(routing.expert_weights.iter())
        .enumerate()
        .take(4)
    {
        let indices_str: Vec<_> = indices.iter().map(|id| id.to_string()).collect();
        let weights_str: Vec<_> = weights.iter().map(|w| format!("{:.3}", w)).collect();
        println!(
            "  Token {:2}: experts [{}], weights [{}]",
            i,
            indices_str.join(", "),
            weights_str.join(", ")
        );
    }
    println!("  ... ({} more tokens)", batch_size * seq_len - 4);
    println!("  Auxiliary loss: {:.4}", routing.aux_loss);

    println!();

    // 5. Token Distribution Analysis
    println!("5. Token Distribution Analysis");
    println!("------------------------------");

    // Simulate routing 1000 tokens and analyze distribution
    let num_tokens = 1000;
    let hidden_states = Tensor::randn(0f32, 1.0, &[num_tokens, hidden_dim], &device)?;
    let routing = router.route(&hidden_states)?;

    // Count tokens per expert
    let mut expert_counts = vec![0usize; num_experts];
    for indices in &routing.expert_indices {
        for expert_id in indices {
            expert_counts[expert_id.0 as usize] += 1;
        }
    }

    println!("Token distribution across {} experts:", num_experts);
    let total_assignments: usize = expert_counts.iter().sum();
    for (i, count) in expert_counts.iter().enumerate() {
        let pct = *count as f32 / total_assignments as f32 * 100.0;
        let bar_len = (pct / 2.0) as usize;
        let bar: String = "=".repeat(bar_len);
        println!(
            "  Expert {}: {:4} tokens ({:5.1}%) |{}",
            i, count, pct, bar
        );
    }

    // Check load balance
    let avg = total_assignments as f32 / num_experts as f32;
    let variance: f32 = expert_counts
        .iter()
        .map(|&c| (c as f32 - avg).powi(2))
        .sum::<f32>()
        / num_experts as f32;
    let std_dev = variance.sqrt();

    println!("\nLoad balance metrics:");
    println!("  Total assignments: {}", total_assignments);
    println!("  Average per expert: {:.1}", avg);
    println!("  Std deviation: {:.2}", std_dev);
    println!("  Balance score: {:.1}% (lower is more balanced)", std_dev / avg * 100.0);

    println!();

    // 6. Distributed MoE Layer
    println!("6. Distributed MoE Layer");
    println!("------------------------");

    let config = MoEConfig {
        hidden_dim,
        num_experts,
        top_k: 2,
        timeout_ms: 5000,
    };

    let gate_weights = Tensor::randn(0f32, 0.02, &[hidden_dim, num_experts], &device)?;
    let router = TopKRouter::new(gate_weights, 2, num_experts, 0.01);
    let mut moe = DistributedMoE::new(Box::new(router), config);

    // Register experts
    for i in 0..4 {
        moe.register_expert(Box::new(LocalExpert::new(i, hidden_dim)));
    }
    for i in 4..8 {
        moe.register_remote_expert(ExpertId::new(i), format!("peer-{}", i));
    }

    println!("MoE layer initialized:");
    println!("  Local experts:  4");
    println!("  Remote experts: 4");
    println!("  Router top-k:   {}", moe.router().top_k());

    // Forward pass (placeholder implementation)
    let input = Tensor::randn(0f32, 1.0, &[4, 16, hidden_dim], &device)?;
    println!("\nForward pass:");
    println!("  Input shape: {:?}", input.dims());

    let output = moe.forward(&input).await?;
    println!("  Output shape: {:?}", output.dims());

    println!("\n==============================");
    println!("Expert registry demo complete!");

    Ok(())
}
