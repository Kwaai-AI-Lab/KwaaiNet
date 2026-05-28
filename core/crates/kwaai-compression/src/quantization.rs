//! Blockwise 8-bit quantization
//!
//! Implements Hivemind-style blockwise quantization for efficient
//! gradient and tensor compression.

use crate::{CompressedData, CompressionError, CompressionResult, Compressor};
use candle_core::{Device, Tensor};
use half::f16;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Blockwise 8-bit quantizer
///
/// Quantizes tensors in blocks, with per-block scaling factors.
/// Achieves ~4x compression with minimal accuracy loss.
pub struct BlockwiseQuantizer {
    /// Block size for quantization
    block_size: usize,
}

impl BlockwiseQuantizer {
    /// Create a new quantizer with the given block size
    pub fn new(block_size: usize) -> Self {
        Self { block_size }
    }

    /// Get the block size
    pub fn block_size(&self) -> usize {
        self.block_size
    }
}

impl Compressor for BlockwiseQuantizer {
    type Compressed = QuantizedTensor;

    fn compress(&self, tensor: &Tensor) -> CompressionResult<QuantizedTensor> {
        debug!(
            "Quantizing tensor shape={:?} block_size={}",
            tensor.dims(),
            self.block_size
        );
        let data = tensor
            .flatten_all()?
            .to_vec1::<f32>()
            .map_err(|e| CompressionError::TensorError(e.to_string()))?;

        let mut quantized = Vec::with_capacity(data.len());
        let mut scales = Vec::with_capacity(data.len() / self.block_size + 1);

        for block in data.chunks(self.block_size) {
            // Find max absolute value in block
            let max_abs = block.iter().map(|x| x.abs()).fold(0.0f32, f32::max);

            let scale = if max_abs > 0.0 { max_abs / 127.0 } else { 1.0 };
            scales.push(f16::from_f32(scale));

            // Quantize block
            for &val in block {
                let q = if scale > 0.0 {
                    (val / scale).round().clamp(-127.0, 127.0) as i8
                } else {
                    0i8
                };
                quantized.push(q);
            }
        }

        let qt = QuantizedTensor {
            data: quantized,
            scales,
            shape: tensor.dims().to_vec(),
            block_size: self.block_size,
        };
        debug!(
            "Quantized tensor: ratio={:.2}x, {} bytes",
            qt.compression_ratio(),
            qt.size_bytes()
        );
        Ok(qt)
    }

    fn decompress(&self, compressed: &QuantizedTensor) -> CompressionResult<Tensor> {
        debug!("Dequantizing tensor shape={:?}", compressed.shape);
        let mut data = Vec::with_capacity(compressed.data.len());

        for (block_idx, block) in compressed.data.chunks(compressed.block_size).enumerate() {
            let scale = compressed
                .scales
                .get(block_idx)
                .map(|s| s.to_f32())
                .unwrap_or(1.0);
            for &q in block {
                data.push(q as f32 * scale);
            }
        }

        Tensor::from_vec(data, compressed.shape.as_slice(), &Device::Cpu)
            .map_err(|e| CompressionError::TensorError(e.to_string()))
    }
}

/// Quantized tensor representation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuantizedTensor {
    /// Quantized values (int8)
    pub data: Vec<i8>,
    /// Per-block scaling factors (f16)
    pub scales: Vec<f16>,
    /// Original tensor shape
    pub shape: Vec<usize>,
    /// Block size used for quantization
    pub block_size: usize,
}

impl CompressedData for QuantizedTensor {
    fn compression_ratio(&self) -> f32 {
        let original = self.original_size_bytes();
        let compressed = self.size_bytes();
        if compressed > 0 {
            original as f32 / compressed as f32
        } else {
            1.0
        }
    }

    fn size_bytes(&self) -> usize {
        // int8 data + f16 scales
        self.data.len() + self.scales.len() * 2
    }

    fn original_size_bytes(&self) -> usize {
        // f32 original
        self.data.len() * 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantization_roundtrip() {
        let quantizer = BlockwiseQuantizer::new(64);

        // Create a test tensor
        let data: Vec<f32> = (0..256).map(|i| i as f32 * 0.1).collect();
        let tensor = Tensor::from_vec(data.clone(), &[256], &Device::Cpu).unwrap();

        // Compress
        let compressed = quantizer.compress(&tensor).unwrap();
        assert!(compressed.compression_ratio() > 3.0);

        // Decompress
        let decompressed = quantizer.decompress(&compressed).unwrap();
        let decompressed_data: Vec<f32> = decompressed.to_vec1().unwrap();

        // Check approximate equality (quantization introduces some error)
        for (orig, decomp) in data.iter().zip(decompressed_data.iter()) {
            assert!((orig - decomp).abs() < 0.5, "Quantization error too large");
        }
    }

    #[test]
    fn test_block_size_getter() {
        let q = BlockwiseQuantizer::new(32);
        assert_eq!(q.block_size(), 32);
        let q2 = BlockwiseQuantizer::new(128);
        assert_eq!(q2.block_size(), 128);
    }

    #[test]
    fn test_shape_preserved_after_roundtrip() {
        let quantizer = BlockwiseQuantizer::new(16);
        let data: Vec<f32> = (0..48).map(|i| i as f32).collect();
        let tensor = Tensor::from_vec(data, &[3, 16], &Device::Cpu).unwrap();
        let compressed = quantizer.compress(&tensor).unwrap();
        assert_eq!(compressed.shape, vec![3, 16]);
        let decompressed = quantizer.decompress(&compressed).unwrap();
        assert_eq!(decompressed.dims(), &[3, 16]);
    }

    #[test]
    fn test_multi_block_tensor() {
        let block_size = 8;
        let quantizer = BlockwiseQuantizer::new(block_size);
        // 40 elements → 5 blocks of 8
        let data: Vec<f32> = (0..40).map(|i| (i as f32 - 20.0) * 0.5).collect();
        let tensor = Tensor::from_vec(data.clone(), &[40], &Device::Cpu).unwrap();
        let compressed = quantizer.compress(&tensor).unwrap();
        // Each block gets one scale entry
        assert_eq!(compressed.scales.len(), 5);
        assert_eq!(compressed.data.len(), 40);
        let recovered: Vec<f32> = quantizer
            .decompress(&compressed)
            .unwrap()
            .to_vec1()
            .unwrap();
        for (orig, got) in data.iter().zip(recovered.iter()) {
            assert!((orig - got).abs() < 0.2, "orig={orig} got={got}");
        }
    }

    #[test]
    fn test_all_zeros_tensor() {
        let quantizer = BlockwiseQuantizer::new(64);
        let data = vec![0.0f32; 64];
        let tensor = Tensor::from_vec(data.clone(), &[64], &Device::Cpu).unwrap();
        let compressed = quantizer.compress(&tensor).unwrap();
        let recovered: Vec<f32> = quantizer
            .decompress(&compressed)
            .unwrap()
            .to_vec1()
            .unwrap();
        for v in recovered {
            assert_eq!(v, 0.0);
        }
    }

    #[test]
    fn test_size_bytes_less_than_original() {
        let quantizer = BlockwiseQuantizer::new(64);
        let data: Vec<f32> = (0..256).map(|i| i as f32).collect();
        let tensor = Tensor::from_vec(data, &[256], &Device::Cpu).unwrap();
        let compressed = quantizer.compress(&tensor).unwrap();
        assert!(compressed.size_bytes() < compressed.original_size_bytes());
    }
}
