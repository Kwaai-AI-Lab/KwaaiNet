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
        debug!("Sparse compress tensor shape={:?} k_fraction={}", tensor.dims(), self.k_fraction);
        let data = tensor
            .flatten_all()?
            .to_vec1::<f32>()
            .map_err(|e| CompressionError::TensorError(e.to_string()))?;

        let k = ((data.len() as f32 * self.k_fraction) as usize).max(1);

        // Find indices of top-k by magnitude
        let mut indexed: Vec<(usize, f32)> = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
        indexed.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());

        let top_k: Vec<_> = indexed.into_iter().take(k).collect();

        let sg = SparseGradient {
            indices: top_k.iter().map(|(i, _)| *i as u32).collect(),
            values: top_k.iter().map(|(_, v)| *v).collect(),
            original_size: data.len(),
            shape: tensor.dims().to_vec(),
        };
        debug!("Sparse gradient: kept {}/{} values, ratio={:.2}x", sg.indices.len(), data.len(), sg.compression_ratio());
        Ok(sg)
    }

    fn decompress(&self, compressed: &SparseGradient) -> CompressionResult<Tensor> {
        debug!("Sparse decompress: {} non-zero values, shape={:?}", compressed.indices.len(), compressed.shape);
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
        assert!(compressed.compression_ratio() > 5.0);

        // The large values should be preserved
        assert!(compressed.values.iter().any(|&v| v.abs() > 0.5));
    }
}
