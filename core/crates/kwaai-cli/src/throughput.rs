//! Throughput cache — persists measured tok/s values between CLI invocations.
//!
//! `kwaainet generate` writes here after each successful generation.
//! `kwaainet start` reads here to populate the DHT announcement.
//!
//! Cache file: `~/.kwaainet/throughput_cache.json`
//! Format:     `{ "model-name": <tok/s as f64>, … }`

use crate::config::kwaainet_dir;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn cache_file() -> PathBuf {
    kwaainet_dir().join("throughput_cache.json")
}

/// Persist a decode throughput measurement for `model`.
pub fn save(model: &str, tps: f64) -> Result<()> {
    let path = cache_file();
    std::fs::create_dir_all(path.parent().expect("cache_file has a parent"))?;

    let mut cache: HashMap<String, f64> = if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        HashMap::new()
    };

    cache.insert(model.to_string(), tps);
    std::fs::write(&path, serde_json::to_string_pretty(&cache)?)?;
    Ok(())
}

/// Load the cached decode throughput for `model`.
/// Returns `None` if the cache file doesn't exist or the model has no entry.
pub fn load(model: &str) -> Option<f64> {
    let text = std::fs::read_to_string(cache_file()).ok()?;
    let cache: HashMap<String, f64> = serde_json::from_str(&text).ok()?;
    cache.get(model).copied()
}
