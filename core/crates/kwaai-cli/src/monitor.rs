//! P2P connection monitoring and alert configuration

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

use crate::config::run_dir;

fn monitor_file() -> PathBuf {
    run_dir().join("monitor.json")
}

fn alert_config_file() -> PathBuf {
    run_dir().join("alert_config.json")
}

// ---------------------------------------------------------------------------
// Monitor stats
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionStats {
    pub samples: u64,
    pub current_connections: u32,
    pub avg_connections: f64,
    pub min_connections: u32,
    pub max_connections: u32,
    pub uptime_percent: f64,
    pub disconnection_periods: Vec<DisconnectionPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectionPeriod {
    pub start: String,
    pub end: String,
    pub duration_seconds: f64,
}

pub fn load_stats() -> Option<ConnectionStats> {
    let text = std::fs::read_to_string(monitor_file()).ok()?;
    serde_json::from_str(&text).ok()
}

// ---------------------------------------------------------------------------
// Alert config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub disconnection_threshold_minutes: u32,
    pub min_connections: u32,
    pub webhook_url: Option<String>,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            disconnection_threshold_minutes: 5,
            min_connections: 1,
            webhook_url: None,
        }
    }
}

pub fn load_alert_config() -> AlertConfig {
    let path = alert_config_file();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

pub fn save_alert_config(cfg: &AlertConfig) -> Result<()> {
    let path = alert_config_file();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let text = serde_json::to_string_pretty(cfg)?;
    std::fs::write(&path, text)?;
    debug!("Saved alert config to {}", path.display());
    Ok(())
}
