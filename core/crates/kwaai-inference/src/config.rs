//! Configuration for the inference engine

use crate::DeviceType;
use serde::{Deserialize, Serialize};

/// Configuration for the inference engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Device to use for inference
    pub device: DeviceType,

    /// Maximum memory to use (in bytes)
    pub max_memory: usize,

    /// Model cache size (number of models to keep loaded)
    pub model_cache_size: usize,

    /// Prefer quantized models for lower memory usage
    pub prefer_quantized: bool,

    /// Maximum batch size
    pub max_batch_size: usize,

    /// Maximum sequence length
    pub max_seq_len: usize,

    /// Use flash attention if available
    pub use_flash_attention: bool,

    /// Number of threads for CPU inference
    pub num_threads: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            device: DeviceType::detect_best(),
            max_memory: 4 * 1024 * 1024 * 1024, // 4GB default
            model_cache_size: 3,
            prefer_quantized: true,
            max_batch_size: 8,
            max_seq_len: 4096,
            use_flash_attention: true,
            num_threads: num_cpus::get(),
        }
    }
}

impl EngineConfig {
    /// Create configuration optimized for browser (WASM)
    pub fn browser_optimized() -> Self {
        Self {
            device: DeviceType::Cpu,
            max_memory: 1024 * 1024 * 1024, // 1GB
            model_cache_size: 1,
            prefer_quantized: true,
            max_batch_size: 1,
            max_seq_len: 2048,
            use_flash_attention: false,
            num_threads: 4,
        }
    }

    /// Create configuration optimized for mobile
    pub fn mobile_optimized() -> Self {
        Self {
            device: DeviceType::Cpu,
            max_memory: 512 * 1024 * 1024, // 512MB
            model_cache_size: 1,
            prefer_quantized: true,
            max_batch_size: 1,
            max_seq_len: 1024,
            use_flash_attention: false,
            num_threads: 2,
        }
    }

    /// Create configuration for server deployment
    pub fn server_optimized() -> Self {
        Self {
            device: DeviceType::detect_best(),
            max_memory: 32 * 1024 * 1024 * 1024, // 32GB
            model_cache_size: 10,
            prefer_quantized: false,
            max_batch_size: 32,
            max_seq_len: 8192,
            use_flash_attention: true,
            num_threads: num_cpus::get(),
        }
    }
}

fn num_cpus_get() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}

mod num_cpus {
    pub fn get() -> usize {
        super::num_cpus_get()
    }
}
