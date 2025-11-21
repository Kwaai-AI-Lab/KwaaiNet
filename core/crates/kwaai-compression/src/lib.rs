//! # kwaai-compression
//!
//! Compression utilities for KwaaiNet distributed ML operations.
//!
//! This crate provides compression algorithms for efficient gradient
//! and tensor transfer, including:
//!
//! - **Blockwise 8-bit Quantization**: ~4x compression with minimal accuracy loss
//! - **Sparse Gradient Compression**: Top-K selection for bandwidth efficiency
//! - **Delta Encoding**: Only transfer changes
//!
//! ## Example
//!
//! ```rust,no_run
//! use kwaai_compression::{BlockwiseQuantizer, Compressor};
//! use candle_core::Tensor;
//!
//! let quantizer = BlockwiseQuantizer::new(64);
//! let tensor = Tensor::zeros(&[1024], candle_core::DType::F32, &candle_core::Device::Cpu)?;
//!
//! let compressed = quantizer.compress(&tensor)?;
//! println!("Compression ratio: {:.2}x", compressed.compression_ratio());
//!
//! let decompressed = quantizer.decompress(&compressed)?;
//! ```

pub mod error;
pub mod quantization;
pub mod sparse;

pub use error::{CompressionError, CompressionResult};
pub use quantization::{BlockwiseQuantizer, QuantizedTensor};
pub use sparse::{SparseGradient, TopKCompressor};

use candle_core::Tensor;

/// Core trait for compression operations
pub trait Compressor: Send + Sync {
    /// Compressed data type
    type Compressed;

    /// Compress a tensor
    fn compress(&self, tensor: &Tensor) -> CompressionResult<Self::Compressed>;

    /// Decompress back to tensor
    fn decompress(&self, compressed: &Self::Compressed) -> CompressionResult<Tensor>;
}

/// Trait for compressed data
pub trait CompressedData {
    /// Get compression ratio achieved
    fn compression_ratio(&self) -> f32;

    /// Get compressed size in bytes
    fn size_bytes(&self) -> usize;

    /// Get original size in bytes
    fn original_size_bytes(&self) -> usize;
}
