//! Health monitoring with exponential backoff reconnection

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::config::HealthConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthMetrics {
    pub checks_total: u64,
    pub checks_healthy: u64,
    pub checks_degraded: u64,
    pub checks_unhealthy: u64,
    pub checks_critical: u64,
    pub reconnections_triggered: u64,
    pub reconnections_successful: u64,
    pub consecutive_failures: u32,
    pub reconnection_attempts: u32,
    pub is_running: bool,
    pub enabled: bool,
    pub check_interval: u64,
    pub failure_threshold: u32,
    pub last_check: Option<LastCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastCheck {
    pub time: String,
    pub status: String,
}

pub struct HealthMonitor {
    config: HealthConfig,
    metrics: HealthMetrics,
    client: reqwest::Client,
}

impl HealthMonitor {
    pub fn new(config: HealthConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout))
            .build()
            .unwrap_or_default();

        let metrics = HealthMetrics {
            enabled: config.enabled,
            check_interval: config.check_interval,
            failure_threshold: config.failure_threshold,
            is_running: true,
            ..Default::default()
        };

        Self { config, metrics, client }
    }

    /// Run the health monitoring loop indefinitely.
    pub async fn run(mut self) {
        if !self.config.enabled {
            info!("Health monitoring disabled");
            return;
        }
        info!("Health monitor starting (interval={}s)", self.config.check_interval);

        loop {
            let status = self.check_once().await;
            self.metrics.checks_total += 1;

            match status.as_str() {
                "healthy" => {
                    self.metrics.checks_healthy += 1;
                    self.metrics.consecutive_failures = 0;
                }
                "degraded" => {
                    self.metrics.checks_degraded += 1;
                    self.metrics.consecutive_failures += 1;
                }
                _ => {
                    self.metrics.checks_unhealthy += 1;
                    self.metrics.consecutive_failures += 1;
                }
            }

            self.metrics.last_check = Some(LastCheck {
                time: chrono::Utc::now().to_rfc3339(),
                status: status.clone(),
            });

            // Trigger reconnection if failure threshold exceeded
            if self.metrics.consecutive_failures >= self.config.failure_threshold
                && self.config.reconnection.enabled
                && self.metrics.reconnection_attempts < self.config.reconnection.max_attempts
            {
                self.metrics.reconnections_triggered += 1;
                self.metrics.reconnection_attempts += 1;
                warn!(
                    "Health failure threshold reached ({} consecutive failures). Triggering reconnection attempt {}/{}",
                    self.metrics.consecutive_failures,
                    self.metrics.reconnection_attempts,
                    self.config.reconnection.max_attempts
                );
                // In a full implementation we'd signal the node to reconnect its DHT peers.
                // For now we log and reset.
                self.metrics.consecutive_failures = 0;
                if let Some(ref url) = self.config.alerting.webhook_url {
                    let _ = self.send_webhook(url, "reconnection_triggered").await;
                }
            }

            tokio::time::sleep(Duration::from_secs(self.config.check_interval)).await;
        }
    }

    async fn check_once(&self) -> String {
        debug!("Health check: {}", self.config.api_endpoint);
        match self.client.get(&self.config.api_endpoint).send().await {
            Ok(resp) if resp.status().is_success() => "healthy".to_string(),
            Ok(resp) => {
                warn!("Health check returned status {}", resp.status());
                "degraded".to_string()
            }
            Err(e) => {
                warn!("Health check failed: {}", e);
                "unhealthy".to_string()
            }
        }
    }

    async fn send_webhook(&self, url: &str, event: &str) -> Result<()> {
        let payload = serde_json::json!({
            "event": event,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        self.client.post(url).json(&payload).send().await?;
        Ok(())
    }

    pub fn metrics(&self) -> &HealthMetrics {
        &self.metrics
    }
}

/// Compute backoff delay for attempt N (1-based).
pub fn backoff_delay(attempt: u32, initial_delay: u64, max_delay: u64, multiplier: f64, jitter: bool, jitter_factor: f64) -> Duration {
    let delay = (initial_delay as f64 * multiplier.powi(attempt as i32 - 1)) as u64;
    let delay = delay.min(max_delay);
    let delay = if jitter {
        let jitter_amount = (delay as f64 * jitter_factor * rand_f64()) as u64;
        delay.saturating_sub(jitter_amount / 2) + jitter_amount / 2
    } else {
        delay
    };
    Duration::from_secs(delay)
}

fn rand_f64() -> f64 {
    // Simple LCG for jitter without pulling in the rand crate
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (seed as f64) / (u32::MAX as f64)
}
