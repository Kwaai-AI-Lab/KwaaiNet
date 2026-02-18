//! Hardware calibration — estimate optimal block count from available RAM

use sysinfo::System;
use tracing::debug;

/// Known model block counts (total blocks in the full model)
fn model_total_blocks(model: &str) -> u32 {
    let model = model.to_lowercase();
    if model.contains("llama-3") && model.contains("8b") { 32 }
    else if model.contains("llama-3") && model.contains("70b") { 80 }
    else if model.contains("llama-2") && model.contains("7b") { 32 }
    else if model.contains("llama-2") && model.contains("13b") { 40 }
    else if model.contains("llama-2") && model.contains("70b") { 80 }
    else if model.contains("mistral") && model.contains("7b") { 32 }
    else { 32 } // safe default
}

/// Memory per block in bytes (float16)
fn bytes_per_block_f16(model: &str) -> u64 {
    let model = model.to_lowercase();
    if model.contains("70b") { 500 * 1024 * 1024 }        // ~500 MB
    else if model.contains("13b") { 312 * 1024 * 1024 }   // ~312 MB
    else { 250 * 1024 * 1024 }                             // ~250 MB (7-8B)
}

#[derive(Debug, Clone)]
pub struct HardwareInfo {
    pub total_memory: u64,
    pub available_memory: u64,
    pub cpu_cores: usize,
}

#[derive(Debug, Clone)]
pub struct CalibrationProfile {
    pub min_blocks: u32,
    pub recommended_blocks: u32,
    pub max_blocks: u32,
    pub total_blocks: u32,
}

impl CalibrationProfile {
    pub fn get_blocks(&self, profile: &str) -> Option<u32> {
        match profile {
            "min" => Some(self.min_blocks),
            "recommended" => Some(self.recommended_blocks),
            "max" => Some(self.max_blocks),
            _ => None,
        }
    }
}

pub struct CalibrationEngine {
    pub hardware: HardwareInfo,
}

impl CalibrationEngine {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let hardware = HardwareInfo {
            total_memory: sys.total_memory(),
            available_memory: sys.available_memory(),
            cpu_cores: sys.cpus().len(),
        };
        debug!(?hardware, "Hardware detected");
        Self { hardware }
    }

    pub fn calibrate(&self, model: &str) -> CalibrationProfile {
        let total_blocks = model_total_blocks(model);
        let bytes_per_block = bytes_per_block_f16(model);

        // Reserve 2 GB for OS + other processes
        let usable = self.hardware.available_memory.saturating_sub(2 * 1024 * 1024 * 1024);
        let max_blocks = ((usable as f64 / bytes_per_block as f64) as u32).min(total_blocks).max(1);

        // Recommended = 75% of max; min = 1 block or 25% of max
        let recommended_blocks = ((max_blocks as f64 * 0.75) as u32).max(1);
        let min_blocks = ((max_blocks as f64 * 0.25) as u32).max(1);

        CalibrationProfile {
            min_blocks,
            recommended_blocks,
            max_blocks,
            total_blocks,
        }
    }
}
