//! Day 9: Local Averaging Example
//!
//! Demonstrates decentralized parameter averaging (Hivemind pattern):
//! - Accumulate gradients from multiple steps
//! - Average gradients locally
//! - Compress gradients for network transfer
//! - Simulate multi-peer averaging
//!
//! Run with: cargo run --example local_averaging

use candle_core::{Device, Tensor};
use kwaai_compression::{BlockwiseQuantizer, CompressedData, Compressor};
use kwaai_distributed::{
    averaging::{AveragingConfig, AveragingResult, DecentralizedAverager, ParameterAverager},
};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet Local Averaging Demo\n");
    println!("==============================\n");

    let device = Device::Cpu;

    // 1. Basic Gradient Accumulation
    println!("1. Gradient Accumulation");
    println!("------------------------");

    let config = AveragingConfig::default();
    let mut averager = DecentralizedAverager::new(config);

    println!("Simulating 4 training steps with gradient accumulation...\n");

    // Simulate model with 3 parameter tensors
    for step in 0..4 {
        // Generate "gradients" for this step (in reality, from backprop)
        let g1 = Tensor::from_vec(
            vec![(step + 1) as f32; 8],
            &[2, 4],
            &device,
        )?;
        let g2 = Tensor::from_vec(
            vec![(step + 1) as f32 * 0.5; 16],
            &[4, 4],
            &device,
        )?;
        let g3 = Tensor::from_vec(
            vec![(step + 1) as f32 * 0.1; 4],
            &[4],
            &device,
        )?;

        averager.accumulate(&[g1, g2, g3])?;
        println!("  Step {}: accumulated gradients", step + 1);
    }

    // Perform averaging step
    let result = averager.step().await?;
    match &result {
        AveragingResult::Success { peers_count, compression_ratio } => {
            println!("\nAveraging complete:");
            println!("  Peers participated: {}", peers_count);
            println!("  Compression ratio:  {:.2}x", compression_ratio);
        }
        _ => println!("  Result: {:?}", result),
    }

    // Check averaged values
    let averaged = averager.get_accumulated();
    println!("\nAveraged gradients:");
    for (i, tensor) in averaged.iter().enumerate() {
        let data: Vec<f32> = tensor.flatten_all()?.to_vec1()?;
        let mean: f32 = data.iter().sum::<f32>() / data.len() as f32;
        println!(
            "  Tensor {}: shape {:?}, mean {:.4}",
            i,
            tensor.dims(),
            mean
        );
    }

    println!();

    // 2. Compression for Network Transfer
    println!("2. Gradient Compression");
    println!("-----------------------");

    let config = AveragingConfig {
        quantization_block_size: 64,
        enable_compression: true,
        ..Default::default()
    };
    let averager = DecentralizedAverager::new(config);

    // Create larger gradients to compress
    let gradients = vec![
        Tensor::randn(0f32, 1.0, &[1024, 256], &device)?,  // 256K params
        Tensor::randn(0f32, 1.0, &[256, 256], &device)?,   // 64K params
        Tensor::randn(0f32, 1.0, &[256], &device)?,        // 256 params
    ];

    let total_params: usize = gradients.iter()
        .map(|t| t.dims().iter().product::<usize>())
        .sum();
    println!("Total parameters: {}", total_params);

    // Compress
    let compressed = averager.compress_gradients(&gradients)?;

    let original_bytes: usize = total_params * 4; // f32
    let compressed_bytes: usize = compressed.iter().map(|c| c.size_bytes()).sum();
    let ratio = original_bytes as f32 / compressed_bytes as f32;

    println!("Compression:");
    println!("  Original:   {} KB", original_bytes / 1024);
    println!("  Compressed: {} KB", compressed_bytes / 1024);
    println!("  Ratio:      {:.2}x", ratio);

    // Decompress and verify
    let decompressed = averager.decompress_gradients(&compressed)?;
    println!("\nVerifying roundtrip:");
    for (i, (orig, decomp)) in gradients.iter().zip(decompressed.iter()).enumerate() {
        let orig_data: Vec<f32> = orig.flatten_all()?.to_vec1()?;
        let decomp_data: Vec<f32> = decomp.flatten_all()?.to_vec1()?;

        let mse: f32 = orig_data
            .iter()
            .zip(decomp_data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            / orig_data.len() as f32;

        println!("  Tensor {}: MSE {:.6}", i, mse);
    }

    println!();

    // 3. Simulating Multi-Peer Averaging
    println!("3. Multi-Peer Averaging Simulation");
    println!("-----------------------------------");

    // Simulate 4 peers, each with their own gradients
    let num_peers = 4;
    let param_shape = [256, 128];

    println!("Simulating {} peers, each with gradients for {} parameters",
             num_peers, param_shape[0] * param_shape[1]);

    // Generate gradients from each peer (different random values)
    let mut peer_gradients = Vec::new();
    for peer in 0..num_peers {
        // Each peer has slightly different gradients (simulating different data)
        let g = Tensor::randn(peer as f32, 1.0, &param_shape, &device)?;
        let mean: f32 = g.mean_all()?.to_scalar()?;
        println!("  Peer {}: gradient mean {:.4}", peer, mean);
        peer_gradients.push(vec![g]);
    }

    // Average across peers
    let config = AveragingConfig::default();
    let averager = DecentralizedAverager::new(config);
    let averaged = averager.average_gradients(&peer_gradients)?;

    let avg_mean: f32 = averaged[0].mean_all()?.to_scalar()?;
    println!("\nAveraged gradient mean: {:.4}", avg_mean);
    println!("Expected mean (sum of peer ids / num_peers): {:.4}",
             (0..num_peers).sum::<i32>() as f32 / num_peers as f32);

    println!();

    // 4. Averaging Configuration
    println!("4. Averaging Configuration");
    println!("--------------------------");

    let configs = [
        ("Default", AveragingConfig::default()),
        ("Fast", AveragingConfig {
            group_size: 2,
            match_timeout: Duration::from_secs(10),
            exchange_timeout: Duration::from_secs(30),
            quantization_block_size: 32,
            enable_compression: true,
        }),
        ("High Quality", AveragingConfig {
            group_size: 8,
            match_timeout: Duration::from_secs(60),
            exchange_timeout: Duration::from_secs(120),
            quantization_block_size: 128,
            enable_compression: true,
        }),
    ];

    for (name, config) in configs {
        println!("{}:", name);
        println!("  Group size:       {}", config.group_size);
        println!("  Match timeout:    {:?}", config.match_timeout);
        println!("  Exchange timeout: {:?}", config.exchange_timeout);
        println!("  Quantization:     {} block size", config.quantization_block_size);
        println!("  Compression:      {}", config.enable_compression);
        println!();
    }

    // 5. Training Loop Simulation
    println!("5. Training Loop Simulation");
    println!("---------------------------");

    let config = AveragingConfig::default();
    let mut averager = DecentralizedAverager::new(config);
    let accumulation_steps = 4;
    let num_epochs = 3;

    println!("Simulating {} epochs with {} accumulation steps each\n",
             num_epochs, accumulation_steps);

    for epoch in 0..num_epochs {
        println!("Epoch {}:", epoch + 1);

        // Accumulate gradients
        for step in 0..accumulation_steps {
            let gradients = vec![
                Tensor::randn(0f32, 1.0, &[128, 64], &device)?,
                Tensor::randn(0f32, 1.0, &[64], &device)?,
            ];
            averager.accumulate(&gradients)?;
            print!("  Step {}: accumulated", step + 1);

            // Every accumulation_steps, attempt averaging
            if (step + 1) % accumulation_steps == 0 {
                match averager.step().await? {
                    AveragingResult::Success { peers_count, compression_ratio } => {
                        println!(" -> averaged ({} peers, {:.1}x compression)",
                                 peers_count, compression_ratio);
                    }
                    AveragingResult::InProgress { ready_peers, target_size } => {
                        println!(" -> waiting ({}/{} peers ready)", ready_peers, target_size);
                    }
                    AveragingResult::NoPeersAvailable => {
                        println!(" -> no peers (using local only)");
                    }
                    AveragingResult::Failed(e) => {
                        println!(" -> failed: {}", e);
                    }
                }
                averager.clear();
            } else {
                println!();
            }
        }
        println!();
    }

    // 6. Bandwidth Analysis
    println!("6. Bandwidth Analysis");
    println!("---------------------");

    let model_sizes = [
        ("Small (10M)", 10_000_000),
        ("Medium (100M)", 100_000_000),
        ("Large (1B)", 1_000_000_000),
    ];

    let compressor = BlockwiseQuantizer::new(64);
    // Sample for compression ratio
    let sample = Tensor::randn(0f32, 1.0, &[10000], &device)?;
    let compressed = compressor.compress(&sample)?;
    let ratio = compressed.compression_ratio();

    println!("With {:.2}x compression (block size 64):\n", ratio);

    for (name, params) in model_sizes {
        let original_mb = params as f64 * 4.0 / 1024.0 / 1024.0;
        let compressed_mb = original_mb / ratio as f64;

        // Assume 4 peers, all-reduce pattern
        let peers = 4;
        let total_per_step_mb = compressed_mb * (peers - 1) as f64 * 2.0; // send + receive

        println!("{} params:", name);
        println!("  Per-peer gradient: {:.1} MB -> {:.1} MB compressed",
                 original_mb, compressed_mb);
        println!("  Total network per step ({} peers): {:.1} MB", peers, total_per_step_mb);
        println!();
    }

    println!("==============================");
    println!("Local averaging demo complete!");

    Ok(())
}
