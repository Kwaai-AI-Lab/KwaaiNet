//! Day 6: 8-bit Quantization Example
//!
//! Demonstrates blockwise 8-bit quantization for tensor compression:
//! - Compress tensors to ~4x smaller size
//! - Decompress with minimal accuracy loss
//! - Measure compression ratio and error
//!
//! Run with: cargo run --example quantization

use candle_core::{Device, DType, Tensor};
use kwaai_compression::{BlockwiseQuantizer, CompressedData, Compressor};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet 8-bit Quantization Demo\n");
    println!("=================================\n");

    let device = Device::Cpu;

    // 1. Basic Quantization Demo
    println!("1. Basic Quantization");
    println!("---------------------");

    // Create a simple test tensor
    let data: Vec<f32> = (0..16).map(|i| i as f32 * 0.5 - 4.0).collect();
    let tensor = Tensor::from_vec(data.clone(), &[16], &device)?;
    println!("Original tensor: {:?}", data);

    let quantizer = BlockwiseQuantizer::new(8);
    let compressed = quantizer.compress(&tensor)?;
    println!("Compressed size: {} bytes", compressed.size_bytes());
    println!("Original size:   {} bytes", compressed.original_size_bytes());
    println!("Compression ratio: {:.2}x", compressed.compression_ratio());

    let decompressed = quantizer.decompress(&compressed)?;
    let decompressed_data: Vec<f32> = decompressed.to_vec1()?;
    println!("Decompressed tensor: {:?}", decompressed_data);

    // Calculate error
    let mse: f32 = data
        .iter()
        .zip(decompressed_data.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        / data.len() as f32;
    println!("Mean Squared Error: {:.6}", mse);

    println!();

    // 2. Different Block Sizes
    println!("2. Block Size Comparison");
    println!("------------------------");

    let sizes = [16, 32, 64, 128, 256];
    let test_tensor = Tensor::randn(0f32, 1.0, &[4096], &device)?;
    let original_data: Vec<f32> = test_tensor.to_vec1()?;

    for block_size in sizes {
        let q = BlockwiseQuantizer::new(block_size);
        let comp = q.compress(&test_tensor)?;
        let decomp = q.decompress(&comp)?;
        let decomp_data: Vec<f32> = decomp.to_vec1()?;

        let mse: f32 = original_data
            .iter()
            .zip(decomp_data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            / original_data.len() as f32;

        let max_err: f32 = original_data
            .iter()
            .zip(decomp_data.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);

        println!(
            "Block size {:4}: ratio {:.2}x, MSE {:.6}, max_err {:.4}",
            block_size,
            comp.compression_ratio(),
            mse,
            max_err
        );
    }

    println!();

    // 3. Gradient-like Tensor Compression
    println!("3. Gradient Compression Simulation");
    println!("-----------------------------------");

    // Simulate gradient tensor (most values near zero, some outliers)
    let gradient_data: Vec<f32> = (0..8192)
        .map(|i| {
            if i % 100 == 0 {
                (i as f32 * 0.01).sin() * 10.0 // Occasional large values
            } else {
                (i as f32 * 0.001).sin() * 0.1 // Most values small
            }
        })
        .collect();

    let gradient_tensor = Tensor::from_vec(gradient_data.clone(), &[8192], &device)?;

    println!("Gradient tensor stats:");
    println!("  Elements: {}", gradient_data.len());
    println!("  Mean: {:.6}", gradient_data.iter().sum::<f32>() / gradient_data.len() as f32);
    println!(
        "  Max abs: {:.6}",
        gradient_data.iter().map(|x| x.abs()).fold(0.0f32, f32::max)
    );

    let quantizer = BlockwiseQuantizer::new(64);
    let compressed = quantizer.compress(&gradient_tensor)?;

    println!("\nCompression results:");
    println!("  Original:   {} bytes", compressed.original_size_bytes());
    println!("  Compressed: {} bytes", compressed.size_bytes());
    println!("  Ratio:      {:.2}x", compressed.compression_ratio());

    let decompressed = quantizer.decompress(&compressed)?;
    let decomp_data: Vec<f32> = decompressed.to_vec1()?;

    let mse: f32 = gradient_data
        .iter()
        .zip(decomp_data.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        / gradient_data.len() as f32;

    println!("  MSE:        {:.8}", mse);
    println!("  RMSE:       {:.8}", mse.sqrt());

    println!();

    // 4. Performance Benchmark
    println!("4. Performance Benchmark");
    println!("------------------------");

    let tensor_sizes = [1024, 4096, 16384, 65536, 262144];
    let quantizer = BlockwiseQuantizer::new(64);

    for size in tensor_sizes {
        let tensor = Tensor::randn(0f32, 1.0, &[size], &device)?;

        // Benchmark compression
        let iterations = 100;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = quantizer.compress(&tensor)?;
        }
        let compress_time = start.elapsed().as_secs_f64() * 1000.0 / iterations as f64;

        // Benchmark decompression
        let compressed = quantizer.compress(&tensor)?;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = quantizer.decompress(&compressed)?;
        }
        let decompress_time = start.elapsed().as_secs_f64() * 1000.0 / iterations as f64;

        // Throughput calculation
        let mb = size as f64 * 4.0 / 1024.0 / 1024.0; // Size in MB (f32)
        let compress_throughput = mb / (compress_time / 1000.0);
        let decompress_throughput = mb / (decompress_time / 1000.0);

        println!(
            "Size {:7}: compress {:.3}ms ({:.0} MB/s), decompress {:.3}ms ({:.0} MB/s)",
            size, compress_time, compress_throughput, decompress_time, decompress_throughput
        );
    }

    println!();

    // 5. Bandwidth Savings Calculation
    println!("5. Network Bandwidth Savings");
    println!("----------------------------");

    // Simulate model gradients (typical sizes)
    let model_sizes = [
        ("Small model (10M params)", 10_000_000),
        ("Medium model (100M params)", 100_000_000),
        ("Large model (1B params)", 1_000_000_000),
    ];

    let quantizer = BlockwiseQuantizer::new(64);
    // Use a smaller tensor for actual computation, scale the results
    let sample_tensor = Tensor::randn(0f32, 1.0, &[10000], &device)?;
    let sample_compressed = quantizer.compress(&sample_tensor)?;
    let compression_ratio = sample_compressed.compression_ratio();

    for (name, params) in model_sizes {
        let original_bytes = params * 4; // f32 = 4 bytes
        let compressed_bytes = (original_bytes as f32 / compression_ratio) as usize;
        let savings = original_bytes - compressed_bytes;

        println!("{}:", name);
        println!("  Original:    {:>10.2} MB", original_bytes as f64 / 1024.0 / 1024.0);
        println!("  Compressed:  {:>10.2} MB", compressed_bytes as f64 / 1024.0 / 1024.0);
        println!("  Savings:     {:>10.2} MB ({:.1}%)",
            savings as f64 / 1024.0 / 1024.0,
            (savings as f64 / original_bytes as f64) * 100.0
        );
        println!();
    }

    println!("=================================");
    println!("Quantization demo complete!");

    Ok(())
}
