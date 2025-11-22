//! Day 7: Sparse Gradients Example
//!
//! Demonstrates Top-K gradient compression for bandwidth efficiency:
//! - Keep only the largest gradients by magnitude
//! - Achieve high compression ratios (10-100x)
//! - Accumulate residuals for error correction
//!
//! Run with: cargo run --example sparse_gradients

use candle_core::{Device, Tensor};
use kwaai_compression::{CompressedData, Compressor, TopKCompressor};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet Sparse Gradients Demo\n");
    println!("===============================\n");

    let device = Device::Cpu;

    // 1. Basic Top-K Compression
    println!("1. Basic Top-K Compression");
    println!("--------------------------");

    // Create a gradient-like tensor (few large values, many small)
    let mut data: Vec<f32> = (0..100).map(|i| (i as f32 * 0.01).sin() * 0.01).collect();
    // Add some significant gradients
    data[10] = 1.0;
    data[25] = -0.8;
    data[50] = 0.9;
    data[75] = -0.7;
    data[90] = 0.6;

    let tensor = Tensor::from_vec(data.clone(), &[100], &device)?;

    println!("Original gradient:");
    println!("  Size: {} elements", data.len());
    println!("  Top 5 values: {:.3}, {:.3}, {:.3}, {:.3}, {:.3}",
        data[10], data[50], data[25], data[75], data[90]);

    // Compress with Top-10%
    let compressor = TopKCompressor::new(0.1);
    let compressed = compressor.compress(&tensor)?;

    println!("\nTop-10% compression:");
    println!("  Kept elements: {}", compressed.indices.len());
    println!("  Compression ratio: {:.1}x", compressed.compression_ratio());
    println!("  Original size: {} bytes", compressed.original_size_bytes());
    println!("  Compressed size: {} bytes", compressed.size_bytes());

    // Show which indices were kept
    let mut sorted_indices: Vec<_> = compressed.indices.iter().zip(compressed.values.iter()).collect();
    sorted_indices.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
    println!("  Preserved (idx, value):");
    for (idx, val) in sorted_indices.iter().take(5) {
        println!("    [{:3}] = {:.4}", idx, val);
    }

    // Decompress and check
    let decompressed = compressor.decompress(&compressed)?;
    let decomp_data: Vec<f32> = decompressed.to_vec1()?;

    // Verify large values preserved
    println!("\nVerification (large values preserved):");
    println!("  data[10]: orig {:.4}, decomp {:.4}", data[10], decomp_data[10]);
    println!("  data[50]: orig {:.4}, decomp {:.4}", data[50], decomp_data[50]);

    println!();

    // 2. K-Fraction Comparison
    println!("2. K-Fraction Comparison");
    println!("------------------------");

    let tensor = Tensor::randn(0f32, 1.0, &[10000], &device)?;
    let orig_data: Vec<f32> = tensor.to_vec1()?;

    let fractions = [0.001, 0.01, 0.05, 0.1, 0.2];

    for frac in fractions {
        let comp = TopKCompressor::new(frac);
        let sparse = comp.compress(&tensor)?;
        let decomp = comp.decompress(&sparse)?;
        let decomp_data: Vec<f32> = decomp.to_vec1()?;

        // Calculate reconstruction error
        let mse: f32 = orig_data
            .iter()
            .zip(decomp_data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            / orig_data.len() as f32;

        // Calculate preserved energy (sum of squares)
        let orig_energy: f32 = orig_data.iter().map(|x| x.powi(2)).sum();
        let preserved_energy: f32 = decomp_data.iter().map(|x| x.powi(2)).sum();
        let energy_ratio = preserved_energy / orig_energy;

        println!(
            "Top-{:5.1}%: ratio {:5.1}x, MSE {:.4}, energy preserved {:.1}%",
            frac * 100.0,
            sparse.compression_ratio(),
            mse,
            energy_ratio * 100.0
        );
    }

    println!();

    // 3. Error Feedback (Residual Accumulation)
    println!("3. Error Feedback Simulation");
    println!("----------------------------");

    // Simulate multiple gradient updates with error feedback
    let compressor = TopKCompressor::new(0.1); // Top 10%
    let mut residual = vec![0.0f32; 1000];

    println!("Simulating 5 gradient updates with error feedback:");

    for round in 0..5 {
        // Generate new gradients
        let new_grad: Vec<f32> = (0..1000)
            .map(|i| ((round * 1000 + i) as f32 * 0.001).sin() * 0.1)
            .collect();

        // Add residual from previous round
        let combined: Vec<f32> = new_grad
            .iter()
            .zip(residual.iter())
            .map(|(g, r)| g + r)
            .collect();

        let tensor = Tensor::from_vec(combined.clone(), &[1000], &device)?;
        let compressed = compressor.compress(&tensor)?;
        let decompressed = compressor.decompress(&compressed)?;
        let decomp_data: Vec<f32> = decompressed.to_vec1()?;

        // Calculate new residual (what wasn't transmitted)
        residual = combined
            .iter()
            .zip(decomp_data.iter())
            .map(|(c, d)| c - d)
            .collect();

        let residual_norm: f32 = residual.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();

        println!(
            "  Round {}: transmitted {} values, residual norm {:.4}",
            round + 1,
            compressed.indices.len(),
            residual_norm
        );
    }

    println!();

    // 4. Performance Benchmark
    println!("4. Performance Benchmark");
    println!("------------------------");

    let tensor_sizes = [1_000, 10_000, 100_000, 1_000_000];
    let compressor = TopKCompressor::new(0.01); // Top 1%

    for size in tensor_sizes {
        let tensor = Tensor::randn(0f32, 1.0, &[size], &device)?;

        // Benchmark compression
        let iterations = if size > 100_000 { 10 } else { 50 };
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = compressor.compress(&tensor)?;
        }
        let compress_time = start.elapsed().as_secs_f64() * 1000.0 / iterations as f64;

        // Benchmark decompression
        let compressed = compressor.compress(&tensor)?;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = compressor.decompress(&compressed)?;
        }
        let decompress_time = start.elapsed().as_secs_f64() * 1000.0 / iterations as f64;

        let mb = size as f64 * 4.0 / 1024.0 / 1024.0;

        println!(
            "Size {:>9}: compress {:>7.2}ms ({:>5.0} MB/s), decompress {:>6.2}ms ({:>6.0} MB/s), ratio {:>5.1}x",
            size,
            compress_time,
            mb / (compress_time / 1000.0),
            decompress_time,
            mb / (decompress_time / 1000.0),
            compressed.compression_ratio()
        );
    }

    println!();

    // 5. Use Case: Distributed Training
    println!("5. Distributed Training Scenario");
    println!("---------------------------------");

    // Simulate 8 workers each with 100M parameter gradients
    let workers = 8;
    let params_per_worker = 100_000_000;
    let compressor = TopKCompressor::new(0.01); // Top 1%

    // Calculate with sample tensor
    let sample = Tensor::randn(0f32, 1.0, &[10000], &device)?;
    let sample_compressed = compressor.compress(&sample)?;
    let ratio = sample_compressed.compression_ratio();

    let original_bytes = params_per_worker * 4; // f32
    let compressed_bytes = (original_bytes as f32 / ratio) as usize;

    println!("Scenario: {} workers, each with {} parameters", workers, params_per_worker);
    println!("\nAll-reduce communication per step:");
    println!("  Without compression: {} MB per worker", original_bytes / 1024 / 1024);
    println!("  With Top-1%:         {} MB per worker", compressed_bytes / 1024 / 1024);
    println!("  Compression ratio:   {:.1}x", ratio);

    let total_without = original_bytes * workers * (workers - 1);
    let total_with = compressed_bytes * workers * (workers - 1);

    println!("\nTotal network traffic (ring all-reduce):");
    println!("  Without compression: {:.2} GB", total_without as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("  With compression:    {:.2} GB", total_with as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("  Bandwidth saved:     {:.2} GB per step", (total_without - total_with) as f64 / 1024.0 / 1024.0 / 1024.0);

    println!("\n===============================");
    println!("Sparse gradients demo complete!");

    Ok(())
}
