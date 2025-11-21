//! Day 4: Tensor Operations Example
//!
//! Demonstrates basic Candle tensor operations:
//! - Tensor creation
//! - Matrix multiplication
//! - Softmax
//! - Device detection
//!
//! Run with: cargo run --example tensor_ops

use candle_core::{Device, Tensor, DType};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("KwaaiNet Tensor Operations Demo\n");
    println!("================================\n");

    // Detect device
    let device = detect_device();
    println!("Device: {:?}\n", device);

    // 1. Basic tensor creation
    println!("1. Tensor Creation");
    println!("------------------");

    let data = vec![1.0f32, 2.0, 3.0, 4.0];
    let tensor = Tensor::from_vec(data.clone(), &[4], &device)?;
    println!("Created tensor from vec: {:?}", tensor.to_vec1::<f32>()?);

    let zeros = Tensor::zeros(&[2, 3], DType::F32, &device)?;
    println!("Zeros [2,3]: {:?}", zeros.to_vec2::<f32>()?);

    let ones = Tensor::ones(&[2, 3], DType::F32, &device)?;
    println!("Ones [2,3]: {:?}", ones.to_vec2::<f32>()?);

    let range = Tensor::arange(0f32, 6.0, &device)?.reshape(&[2, 3])?;
    println!("Range [2,3]: {:?}", range.to_vec2::<f32>()?);

    println!();

    // 2. Matrix multiplication
    println!("2. Matrix Multiplication");
    println!("------------------------");

    // [2, 3] x [3, 2] = [2, 2]
    let a = Tensor::from_vec(
        vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[2, 3],
        &device,
    )?;
    let b = Tensor::from_vec(
        vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[3, 2],
        &device,
    )?;

    println!("A [2,3]: {:?}", a.to_vec2::<f32>()?);
    println!("B [3,2]: {:?}", b.to_vec2::<f32>()?);

    let start = Instant::now();
    let c = a.matmul(&b)?;
    let elapsed = start.elapsed();

    println!("A @ B = {:?}", c.to_vec2::<f32>()?);
    println!("Time: {:?}", elapsed);

    println!();

    // 3. Softmax
    println!("3. Softmax Operation");
    println!("--------------------");

    let logits = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], &[4], &device)?;
    println!("Logits: {:?}", logits.to_vec1::<f32>()?);

    let probs = softmax(&logits)?;
    println!("Softmax: {:?}", probs.to_vec1::<f32>()?);

    let sum: f32 = probs.to_vec1::<f32>()?.iter().sum();
    println!("Sum (should be 1.0): {:.6}", sum);

    println!();

    // 4. Element-wise operations
    println!("4. Element-wise Operations");
    println!("--------------------------");

    let x = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], &[4], &device)?;
    let y = Tensor::from_vec(vec![2.0f32, 2.0, 2.0, 2.0], &[4], &device)?;

    println!("x: {:?}", x.to_vec1::<f32>()?);
    println!("y: {:?}", y.to_vec1::<f32>()?);
    println!("x + y: {:?}", (&x + &y)?.to_vec1::<f32>()?);
    println!("x * y: {:?}", (&x * &y)?.to_vec1::<f32>()?);
    println!("x - y: {:?}", (&x - &y)?.to_vec1::<f32>()?);
    println!("x / y: {:?}", (&x / &y)?.to_vec1::<f32>()?);

    println!();

    // 5. Reductions
    println!("5. Reductions");
    println!("-------------");

    let data = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3], &device)?;
    println!("Data [2,3]: {:?}", data.to_vec2::<f32>()?);
    println!("Sum all: {}", data.sum_all()?.to_scalar::<f32>()?);
    println!("Mean all: {}", data.mean_all()?.to_scalar::<f32>()?);
    println!("Max all: {}", data.max_all()?.to_scalar::<f32>()?);
    println!("Min all: {}", data.min_all()?.to_scalar::<f32>()?);

    println!();

    // 6. Benchmark larger matmul
    println!("6. Performance Benchmark");
    println!("------------------------");

    let sizes = [64, 128, 256, 512];
    for size in sizes {
        let a = Tensor::randn(0f32, 1.0, &[size, size], &device)?;
        let b = Tensor::randn(0f32, 1.0, &[size, size], &device)?;

        let start = Instant::now();
        let iterations = 10;
        for _ in 0..iterations {
            let _ = a.matmul(&b)?;
        }
        let elapsed = start.elapsed();
        let avg_ms = elapsed.as_secs_f64() * 1000.0 / iterations as f64;

        println!(
            "MatMul [{:4}x{:4}] x [{:4}x{:4}]: {:.3} ms/op",
            size, size, size, size, avg_ms
        );
    }

    println!("\n================================");
    println!("All tensor operations successful!");

    Ok(())
}

/// Detect best available device
fn detect_device() -> Device {
    #[cfg(feature = "cuda")]
    {
        if candle_core::utils::cuda_is_available() {
            println!("CUDA available!");
            return Device::new_cuda(0).unwrap_or(Device::Cpu);
        }
    }

    #[cfg(feature = "metal")]
    {
        if candle_core::utils::metal_is_available() {
            println!("Metal available!");
            return Device::new_metal(0).unwrap_or(Device::Cpu);
        }
    }

    Device::Cpu
}

/// Compute softmax
fn softmax(x: &Tensor) -> candle_core::Result<Tensor> {
    let max = x.max_keepdim(0)?;
    let exp = (x.broadcast_sub(&max)?)?.exp()?;
    let sum = exp.sum_keepdim(0)?;
    exp.broadcast_div(&sum)
}
