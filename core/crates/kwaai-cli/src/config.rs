//! Configuration management for KwaaiNet
//!
//! Config file lives at `~/.kwaainet/config.yaml`.
//! On first run a default config is written and returned.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Directory helpers
// ---------------------------------------------------------------------------

pub fn kwaainet_dir() -> PathBuf {
    dirs_home().join(".kwaainet")
}

pub fn config_file() -> PathBuf {
    kwaainet_dir().join("config.yaml")
}

pub fn run_dir() -> PathBuf {
    kwaainet_dir().join("run")
}

pub fn log_dir() -> PathBuf {
    kwaainet_dir().join("logs")
}

pub fn log_file() -> PathBuf {
    log_dir().join("kwaainet.log")
}

fn dirs_home() -> PathBuf {
    dirs_sys::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

// ---------------------------------------------------------------------------
// Config structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KwaaiNetConfig {
    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default = "default_blocks")]
    pub blocks: u32,

    /// First transformer block this node serves (0-indexed).
    /// The node serves blocks [start_block .. start_block + blocks).
    /// Defaults to 0 (start of model); set this on non-first nodes.
    #[serde(default)]
    pub start_block: u32,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_true")]
    pub use_gpu: bool,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default)]
    pub public_name: Option<String>,

    #[serde(default)]
    pub public_ip: Option<String>,

    #[serde(default)]
    pub announce_addr: Option<String>,

    #[serde(default)]
    pub no_relay: bool,

    #[serde(default = "default_peers")]
    pub initial_peers: Vec<String>,

    #[serde(default)]
    pub health_monitoring: HealthConfig,

    /// Canonical Hivemind DHT prefix for the selected model
    /// (e.g. "Llama-3-1-8B-Instruct-hf"), set from the network map.
    /// Used as the DHT key prefix when announcing blocks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_dht_prefix: Option<String>,

    /// HuggingFace repository URL for the selected model, set from the network map.
    /// Used in the _petals.models DHT registry entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_repository: Option<String>,

    // ── VPK (Virtual Private Knowledge) integration ──────────────────────────
    /// Whether this node hosts a local VPK service.
    /// When true, KwaaiNet polls the VPK health endpoint and advertises
    /// capability on the DHT. Defaults to false (opt-in).
    #[serde(default)]
    pub vpk_enabled: bool,

    /// VPK operating mode: "bob" (query-only), "eve" (storage), or "both".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpk_mode: Option<String>,

    /// HTTP endpoint to advertise to peers in the DHT record.
    /// When None the field is omitted from the DHT advertisement (local-only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpk_endpoint: Option<String>,

    /// Local port for the VPK health-check and REST API (default: 7432).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpk_local_port: Option<u16>,

    // ── Block rebalancing ─────────────────────────────────────────────────────
    /// Enable periodic block rebalancing (only active with `shard serve --auto`).
    /// When true, the shard server periodically checks DHT coverage and moves
    /// its blocks to fill gaps if its current range is well-covered by others.
    #[serde(default)]
    pub auto_rebalance: bool,

    /// How often to check coverage and potentially rebalance (seconds).
    #[serde(default = "default_rebalance_interval")]
    pub rebalance_interval_secs: u64,

    /// Minimum number of OTHER nodes that must cover our range before we will
    /// consider moving. Prevents moving when we are the sole coverage.
    #[serde(default = "default_rebalance_min_redundancy")]
    pub rebalance_min_redundancy: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: String,

    #[serde(default = "default_check_interval")]
    pub check_interval: u64,

    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,

    #[serde(default)]
    pub reconnection: ReconnectionConfig,

    #[serde(default)]
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconnectionConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,

    #[serde(default = "default_backoff_strategy")]
    pub backoff_strategy: String,

    #[serde(default = "default_initial_delay")]
    pub initial_delay: u64,

    #[serde(default = "default_max_delay")]
    pub max_delay: u64,

    #[serde(default = "default_backoff_multiplier")]
    pub backoff_multiplier: f64,

    #[serde(default = "default_true")]
    pub jitter: bool,

    #[serde(default = "default_jitter_factor")]
    pub jitter_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub on_disconnect: bool,

    #[serde(default = "default_true")]
    pub on_reconnect: bool,

    #[serde(default = "default_true")]
    pub on_critical: bool,

    #[serde(default)]
    pub webhook_url: Option<String>,

    #[serde(default)]
    pub email: Option<String>,
}

// ---------------------------------------------------------------------------
// Default value functions (required by serde)
// ---------------------------------------------------------------------------

fn default_model() -> String {
    std::env::var("KWAAINET_MODEL").unwrap_or_else(|_| "unsloth/Llama-3.1-8B-Instruct".to_string())
}
fn default_blocks() -> u32 {
    std::env::var("KWAAINET_BLOCKS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8)
}
fn default_port() -> u16 {
    std::env::var("KWAAINET_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}
fn default_true() -> bool {
    true
}
fn default_log_level() -> String {
    std::env::var("KWAAINET_LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
}
fn default_peers() -> Vec<String> {
    vec![
        "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
            .to_string(),
        "/dns/bootstrap-2.kwaai.ai/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY"
            .to_string(),
    ]
}
fn default_api_endpoint() -> String {
    "https://map.kwaai.ai/api/v1/state".to_string()
}
fn default_check_interval() -> u64 {
    60
}
fn default_request_timeout() -> u64 {
    10
}
fn default_failure_threshold() -> u32 {
    3
}
fn default_max_attempts() -> u32 {
    10
}
fn default_backoff_strategy() -> String {
    "exponential".to_string()
}
fn default_initial_delay() -> u64 {
    30
}
fn default_max_delay() -> u64 {
    1800
}
fn default_backoff_multiplier() -> f64 {
    2.0
}
fn default_jitter_factor() -> f64 {
    0.5
}
fn default_rebalance_interval() -> u64 {
    300
}
fn default_rebalance_min_redundancy() -> usize {
    1
}

impl Default for KwaaiNetConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            blocks: default_blocks(),
            start_block: 0,
            port: default_port(),
            use_gpu: true,
            log_level: default_log_level(),
            public_name: Some(format!(
                "{}-{}-{}",
                std::env::var("USER").unwrap_or_else(|_| "anonymous".to_string()),
                std::env::consts::OS,
                std::env::consts::ARCH,
            )),
            public_ip: None,
            announce_addr: None,
            no_relay: false,
            initial_peers: default_peers(),
            health_monitoring: HealthConfig::default(),
            model_dht_prefix: None,
            model_repository: None,
            vpk_enabled: false,
            vpk_mode: None,
            vpk_endpoint: None,
            vpk_local_port: None,
            auto_rebalance: false,
            rebalance_interval_secs: default_rebalance_interval(),
            rebalance_min_redundancy: default_rebalance_min_redundancy(),
        }
    }
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_endpoint: default_api_endpoint(),
            check_interval: default_check_interval(),
            request_timeout: default_request_timeout(),
            failure_threshold: default_failure_threshold(),
            reconnection: ReconnectionConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for ReconnectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: default_max_attempts(),
            backoff_strategy: default_backoff_strategy(),
            initial_delay: default_initial_delay(),
            max_delay: default_max_delay(),
            backoff_multiplier: default_backoff_multiplier(),
            jitter: true,
            jitter_factor: default_jitter_factor(),
        }
    }
}

// ---------------------------------------------------------------------------
// Load / save
// ---------------------------------------------------------------------------

impl KwaaiNetConfig {
    /// Load config from `~/.kwaainet/config.yaml`, creating it with defaults if absent.
    pub fn load_or_create() -> Result<Self> {
        let cfg_file = config_file();
        std::fs::create_dir_all(cfg_file.parent().unwrap())?;

        if cfg_file.exists() {
            let text = std::fs::read_to_string(&cfg_file)
                .with_context(|| format!("reading {}", cfg_file.display()))?;
            let cfg: KwaaiNetConfig = serde_yaml::from_str(&text)
                .with_context(|| format!("parsing {}", cfg_file.display()))?;
            debug!("Loaded config from {}", cfg_file.display());
            Ok(cfg)
        } else {
            let cfg = KwaaiNetConfig::default();
            cfg.save()?;
            info!("Created default config at {}", cfg_file.display());
            Ok(cfg)
        }
    }

    /// Persist the current config to disk.
    pub fn save(&self) -> Result<()> {
        let cfg_file = config_file();
        std::fs::create_dir_all(cfg_file.parent().unwrap())?;
        let text = serde_yaml::to_string(self).context("serializing config")?;
        std::fs::write(&cfg_file, text)
            .with_context(|| format!("writing {}", cfg_file.display()))?;
        debug!("Saved config to {}", cfg_file.display());
        Ok(())
    }

    /// Return the effective DHT prefix for this node's model.
    ///
    /// Uses the canonical prefix set by the map API when available.
    /// Falls back to deriving it from the model name using Petals conventions:
    /// `"org/Model-Name.1B"` → `"Model-Name-1B"` (basename only, dots to dashes).
    ///
    /// This is the single source of truth — both `node.rs` and `shard_cmd.rs`
    /// call this so they always agree on the DHT key.
    pub fn effective_dht_prefix(&self) -> String {
        if let Some(ref p) = self.model_dht_prefix {
            return p.clone();
        }
        let base = self.model.split('/').next_back().unwrap_or(&self.model);
        base.replace('.', "-")
    }

    /// Total transformer blocks in the full model.
    ///
    /// Reads `num_hidden_layers` from the model's `config.json` when the
    /// snapshot is available locally. Falls back to a name-based heuristic
    /// (32 / 40 / 80) when the model has not been downloaded yet.
    pub fn model_total_blocks(&self) -> i32 {
        if let Ok(model_dir) = crate::hf::resolve_snapshot(&self.model) {
            let config_path = model_dir.join("config.json");
            if let Ok(s) = std::fs::read_to_string(&config_path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                    if let Some(n) = v["num_hidden_layers"].as_i64() {
                        return n as i32;
                    }
                }
            }
        }
        // Fallback: name heuristic when model is not yet downloaded.
        let m = self.model.to_lowercase();
        if m.contains("70b") {
            80
        } else if m.contains("13b") {
            40
        } else {
            32
        }
    }

    /// Effective last block (exclusive) this node serves, clamped to the
    /// total number of transformer blocks in the model.
    ///
    /// Prevents `end_block = start_block + blocks` from exceeding the model
    /// size when the operator sets a large `blocks` value.
    pub fn effective_end_block(&self) -> u32 {
        let total = self.model_total_blocks() as u32;
        (self.start_block + self.blocks).min(total)
    }

    /// Set a top-level key by name (string value coerced to the right type).
    pub fn set_key(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "model" => self.model = value.to_string(),
            "blocks" => self.blocks = value.parse().context("blocks must be a number")?,
            "port" => self.port = value.parse().context("port must be a number")?,
            "use_gpu" => self.use_gpu = parse_bool(value)?,
            "log_level" => self.log_level = value.to_string(),
            "public_name" => self.public_name = Some(value.to_string()),
            "public_ip" => self.public_ip = Some(value.to_string()),
            "announce_addr" => self.announce_addr = Some(value.to_string()),
            "no_relay" => self.no_relay = parse_bool(value)?,
            "start_block" => {
                self.start_block = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("start_block must be a non-negative integer"))?
            }
            "auto_rebalance" => self.auto_rebalance = parse_bool(value)?,
            "rebalance_interval_secs" => {
                self.rebalance_interval_secs = value.parse().map_err(|_| {
                    anyhow::anyhow!("rebalance_interval_secs must be a positive integer")
                })?
            }
            "rebalance_min_redundancy" => {
                self.rebalance_min_redundancy = value.parse().map_err(|_| {
                    anyhow::anyhow!("rebalance_min_redundancy must be a positive integer")
                })?
            }
            _ => anyhow::bail!(
                "Unknown config key '{}'. Run `kwaainet config set --help` to see valid keys.",
                key
            ),
        }
        self.save()
    }
}

fn parse_bool(s: &str) -> Result<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => anyhow::bail!("Expected true/false, got {}", s),
    }
}

/// Detect public IP via ipify.org (async).
pub async fn detect_public_ip() -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;
    let ip = client
        .get("https://api.ipify.org")
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()?
        .trim()
        .to_string();
    if ip.is_empty() {
        None
    } else {
        Some(ip)
    }
}

mod dirs_sys {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        dirs::home_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(start: u32, blocks: u32, total_hint: &str) -> KwaaiNetConfig {
        KwaaiNetConfig {
            model: total_hint.to_string(), // name heuristic drives model_total_blocks()
            start_block: start,
            blocks,
            ..KwaaiNetConfig::default()
        }
    }

    #[test]
    fn effective_end_block_no_clamp() {
        // 0 + 8 = 8 < 32 — no clamping needed
        let c = cfg(0, 8, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 8);
    }

    #[test]
    fn effective_end_block_clamps_to_model_total() {
        // 8 + 32 = 40, but model has 32 blocks → clamped to 32
        let c = cfg(8, 32, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 32);
    }

    #[test]
    fn effective_end_block_exact_fit() {
        // 0 + 32 = 32 == total — no clamping
        let c = cfg(0, 32, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 32);
    }

    #[test]
    fn effective_end_block_70b_model() {
        // 70B has 80 blocks; 72 + 32 = 104 → clamped to 80
        let c = cfg(72, 32, "meta/Llama-2-70B");
        assert_eq!(c.effective_end_block(), 80);
    }
}
