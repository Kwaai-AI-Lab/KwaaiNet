//! Sparse gradient compression
//!
//! Implements top-K selection and other sparsification methods
//! for gradient compression.

use crate::{CompressedData, CompressionError, CompressionResult, Compressor};
use candle_core::{Device, Tensor};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Top-K gradient compressor
///
/// Keeps only the top-K largest gradients by magnitude,
/// dramatically reducing communication overhead.
pub struct TopKCompressor {
    /// Fraction of gradients to keep (0.0 - 1.0)
    k_fraction: f32,
}

impl TopKCompressor {
    /// Create a new top-K compressor
    ///
    /// # Arguments
    /// * `k_fraction` - Fraction of values to keep (0.0 - 1.0)
    pub fn new(k_fraction: f32) -> Self {
        Self {
            k_fraction: k_fraction.clamp(0.0, 1.0),
        }
    }

    /// Get the k fraction
    pub fn k_fraction(&self) -> f32 {
        self.k_fraction
    }
}

impl Compressor for TopKCompressor {
    type Compressed = SparseGradient;

    fn compress(&self, tensor: &Tensor) -> CompressionResult<SparseGradient> {
        debug!(
            "Sparse compress tensor shape={:?} k_fraction={}",
            tensor.dims(),
            self.k_fraction
        );
        let data = tensor
            .flatten_all()?
            .to_vec1::<f32>()
            .map_err(|e| CompressionError::TensorError(e.to_string()))?;

        let k = ((data.len() as f32 * self.k_fraction) as usize).max(1);

        // Find indices of top-k by magnitude
        let mut indexed: Vec<(usize, f32)> =
            data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());

        let top_k: Vec<_> = indexed.into_iter().take(k).collect();

        let sg = SparseGradient {
            indices: top_k.iter().map(|(i, _)| *i as u32).collect(),
            values: top_k.iter().map(|(_, v)| *v).collect(),
            original_size: data.len(),
            shape: tensor.dims().to_vec(),
        };
        debug!(
            "Sparse gradient: kept {}/{} values, ratio={:.2}x",
            sg.indices.len(),
            data.len(),
            sg.compression_ratio()
        );
        Ok(sg)
    }

    fn decompress(&self, compressed: &SparseGradient) -> CompressionResult<Tensor> {
        debug!(
            "Sparse decompress: {} non-zero values, shape={:?}",
            compressed.indices.len(),
            compressed.shape
        );
        let mut data = vec![0.0f32; compressed.original_size];

        for (&idx, &val) in compressed.indices.iter().zip(compressed.values.iter()) {
            if (idx as usize) < data.len() {
                data[idx as usize] = val;
            }
        }

        Tensor::from_vec(data, compressed.shape.as_slice(), &Device::Cpu)
            .map_err(|e| CompressionError::TensorError(e.to_string()))
    }
}

/// Sparse gradient representation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SparseGradient {
    /// Indices of non-zero values
    pub indices: Vec<u32>,
    /// Non-zero values
    pub values: Vec<f32>,
    /// Original tensor size
    pub original_size: usize,
    /// Original shape
    pub shape: Vec<usize>,
}

impl CompressedData for SparseGradient {
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
        // u32 indices + f32 values
        self.indices.len() * 4 + self.values.len() * 4
    }

    fn original_size_bytes(&self) -> usize {
        self.original_size * 4 // f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topk_compression() {
        let compressor = TopKCompressor::new(0.1); // Keep top 10%

        // Create a test tensor with mostly small values and a few large ones
        let mut data = vec![0.01f32; 100];
        data[10] = 1.0;
        data[50] = -2.0;
        data[90] = 1.5;

        let tensor = Tensor::from_vec(data.clone(), &[100], &Device::Cpu).unwrap();

        // Compress
        let compressed = compressor.compress(&tensor).unwrap();
        assert!(compressed.indices.len() <= 10);
        assert!(compressed.compression_ratio() >= 5.0);

        // The large values should be preserved
        assert!(compressed.values.iter().any(|&v| v.abs() > 0.5));
    }

    #[test]
    fn test_k_fraction_getter() {
        let c = TopKCompressor::new(0.25);
        assert!((c.k_fraction() - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_k_fraction_clamped() {
        let lo = TopKCompressor::new(-1.0);
        assert_eq!(lo.k_fraction(), 0.0);
        let hi = TopKCompressor::new(2.0);
        assert_eq!(hi.k_fraction(), 1.0);
    }

    #[test]
    fn test_decompress_roundtrip_preserves_large_values() {
        let compressor = TopKCompressor::new(0.1);
        let mut data = vec![0.0f32; 100];
        data[5] = 3.0;
        data[42] = -2.5;
        data[77] = 1.8;
        let tensor = Tensor::from_vec(data.clone(), &[100], &Device::Cpu).unwrap();
        let compressed = compressor.compress(&tensor).unwrap();
        let recovered: Vec<f32> = compressor
            .decompress(&compressed)
            .unwrap()
            .to_vec1()
            .unwrap();
        // Large values at their original positions must be preserved
        assert!((recovered[5] - 3.0).abs() < 1e-4);
        assert!((recovered[42] - (-2.5)).abs() < 1e-4);
        assert!((recovered[77] - 1.8).abs() < 1e-4);
    }

    #[test]
    fn test_k_fraction_one_keeps_all() {
        let compressor = TopKCompressor::new(1.0);
        let data: Vec<f32> = (0..20).map(|i| i as f32).collect();
        let tensor = Tensor::from_vec(data.clone(), &[20], &Device::Cpu).unwrap();
        let compressed = compressor.compress(&tensor).unwrap();
        assert_eq!(compressed.indices.len(), 20);
        let recovered: Vec<f32> = compressor
            .decompress(&compressed)
            .unwrap()
            .to_vec1()
            .unwrap();
        // Every original value should be recovered exactly
        for (orig, got) in data.iter().zip(recovered.iter()) {
            assert!((orig - got).abs() < 1e-4);
        }
    }

    #[test]
    fn test_shape_and_metadata() {
        let compressor = TopKCompressor::new(0.5);
        let data: Vec<f32> = (0..50).map(|i| i as f32).collect();
        let tensor = Tensor::from_vec(data, &[5, 10], &Device::Cpu).unwrap();
        let compressed = compressor.compress(&tensor).unwrap();
        assert_eq!(compressed.shape, vec![5, 10]);
        assert_eq!(compressed.original_size, 50);
        assert_eq!(
            compressed.original_size_bytes(),
            50 * 4,
            "f32 = 4 bytes each"
        );
    }

    #[test]
    fn test_decompress_restores_shape() {
        let compressor = TopKCompressor::new(0.5);
        let data: Vec<f32> = (0..30).map(|i| i as f32).collect();
        let tensor = Tensor::from_vec(data, &[3, 10], &Device::Cpu).unwrap();
        let compressed = compressor.compress(&tensor).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed.dims(), &[3, 10]);
    }
}
