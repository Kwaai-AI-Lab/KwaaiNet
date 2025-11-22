//! Day 5: Simple Forward Pass Example
//!
//! Demonstrates a simple neural network forward pass:
//! - Create random weights
//! - Build a simple 2-layer network
//! - Run forward pass with timing
//!
//! Run with: cargo run --example forward_pass

use candle_core::{Device, Tensor, DType};
use std::error::Error;
use std::time::Instant;

/// Simple 2-layer MLP
struct SimpleMLP {
    w1: Tensor,
    b1: Tensor,
    w2: Tensor,
    b2: Tensor,
}

impl SimpleMLP {
    /// Create a new MLP with random weights
    fn new(input_dim: usize, hidden_dim: usize, output_dim: usize, device: &Device) -> candle_core::Result<Self> {
        // Xavier initialization scale
        let scale1 = (2.0 / (input_dim + hidden_dim) as f64).sqrt() as f32;
        let scale2 = (2.0 / (hidden_dim + output_dim) as f64).sqrt() as f32;

        let w1 = (Tensor::randn(0f32, 1.0, &[input_dim, hidden_dim], device)? * scale1 as f64)?;
        let b1 = Tensor::zeros(&[hidden_dim], DType::F32, device)?;

        let w2 = (Tensor::randn(0f32, 1.0, &[hidden_dim, output_dim], device)? * scale2 as f64)?;
        let b2 = Tensor::zeros(&[output_dim], DType::F32, device)?;

        Ok(Self { w1, b1, w2, b2 })
    }

    /// Forward pass
    fn forward(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        // Layer 1: Linear + ReLU
        let h = x.matmul(&self.w1)?;
        let h = h.broadcast_add(&self.b1)?;
        let h = relu(&h)?;

        // Layer 2: Linear
        let out = h.matmul(&self.w2)?;
        let out = out.broadcast_add(&self.b2)?;

        Ok(out)
    }

    /// Get parameter count
    fn param_count(&self) -> usize {
        let w1_size: usize = self.w1.dims().iter().product();
        let b1_size: usize = self.b1.dims().iter().product();
        let w2_size: usize = self.w2.dims().iter().product();
        let b2_size: usize = self.b2.dims().iter().product();
        w1_size + b1_size + w2_size + b2_size
    }

    /// Get memory usage in bytes (f32 = 4 bytes)
    fn memory_bytes(&self) -> usize {
        self.param_count() * 4
    }
}

/// ReLU activation
fn relu(x: &Tensor) -> candle_core::Result<Tensor> {
    x.maximum(&Tensor::zeros_like(x)?)
}

/// Softmax for final output
fn softmax(x: &Tensor) -> candle_core::Result<Tensor> {
    let max = x.max_keepdim(candle_core::D::Minus1)?;
    let exp = x.broadcast_sub(&max)?.exp()?;
    let sum = exp.sum_keepdim(candle_core::D::Minus1)?;
    exp.broadcast_div(&sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet Forward Pass Demo\n");
    println!("==========================\n");

    // Detect device
    let device = Device::Cpu;
    println!("Device: {:?}\n", device);

    // Network configuration
    let input_dim = 768;   // e.g., BERT hidden size
    let hidden_dim = 3072; // 4x expansion
    let output_dim = 768;
    let batch_size = 8;
    let seq_len = 128;

    println!("Network Configuration:");
    println!("  Input dim:  {}", input_dim);
    println!("  Hidden dim: {}", hidden_dim);
    println!("  Output dim: {}", output_dim);
    println!("  Batch size: {}", batch_size);
    println!("  Seq length: {}", seq_len);
    println!();

    // Create model
    println!("Creating model...");
    let start = Instant::now();
    let model = SimpleMLP::new(input_dim, hidden_dim, output_dim, &device)?;
    let init_time = start.elapsed();

    println!("  Parameters: {}", model.param_count());
    println!("  Memory: {} KB", model.memory_bytes() / 1024);
    println!("  Init time: {:?}", init_time);
    println!();

    // Create input
    println!("Creating input tensor...");
    let input = Tensor::randn(0f32, 1.0, &[batch_size, seq_len, input_dim], &device)?;
    println!("  Input shape: {:?}", input.dims());
    println!();

    // Warm-up forward pass
    println!("Warm-up forward pass...");
    let _ = model.forward(&input.reshape(&[batch_size * seq_len, input_dim])?)?;

    // Benchmark forward pass
    println!("Running forward pass benchmark...\n");
    let iterations = 10;
    let mut times = Vec::new();

    for i in 0..iterations {
        let flat_input = input.reshape(&[batch_size * seq_len, input_dim])?;

        let start = Instant::now();
        let output = model.forward(&flat_input)?;
        let elapsed = start.elapsed();

        times.push(elapsed.as_secs_f64() * 1000.0);

        if i == 0 {
            let output_reshaped = output.reshape(&[batch_size, seq_len, output_dim])?;
            println!("  Output shape: {:?}", output_reshaped.dims());

            // Apply softmax to get probabilities (last dim)
            let probs = softmax(&output)?;
            let sample_probs: Vec<f32> = probs.get(0)?.to_vec1()?;
            println!("  Sample output (first 5 values): {:?}", &sample_probs[..5.min(sample_probs.len())]);
        }
    }

    // Statistics
    let avg_ms: f64 = times.iter().sum::<f64>() / times.len() as f64;
    let min_ms: f64 = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_ms: f64 = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    println!("\nPerformance ({} iterations):", iterations);
    println!("  Average: {:.3} ms", avg_ms);
    println!("  Min:     {:.3} ms", min_ms);
    println!("  Max:     {:.3} ms", max_ms);

    // Throughput calculation
    let tokens_per_batch = batch_size * seq_len;
    let tokens_per_sec = tokens_per_batch as f64 / (avg_ms / 1000.0);
    println!("  Throughput: {:.0} tokens/sec", tokens_per_sec);

    println!("\n==========================");
    println!("Forward pass successful!");

    Ok(())
}
