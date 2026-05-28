//! Metrics persistence for the kwaai-network test suite.
//!
//! Each test run appends one JSON line to `tests/kwaai-network/results/metrics.jsonl`.
//! The `metrics-report` binary reads that file and prints a trend table showing
//! pass rate, average latency, and whether things are getting better or worse.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

/// Path to the JSONL metrics file, relative to the workspace root.
///
/// Resolved at compile time from CARGO_MANIFEST_DIR so the location is
/// stable regardless of the current working directory at test time.
pub fn metrics_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/kwaai-network/results/metrics.jsonl")
}

/// A single recorded test result.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestRecord {
    /// RFC 3339 wall-clock timestamp of the run
    pub timestamp: String,
    /// Short git revision (`git rev-parse --short HEAD`), or "unknown"
    pub git_rev: String,
    /// Full test name, e.g. `integration::daemon::two_node_connect`
    pub test: String,
    /// `"unit"`, `"integration"`, or `"network"`
    pub tier: String,
    /// Whether the test passed
    pub passed: bool,
    /// Wall-clock duration of the test body
    pub duration_ms: u64,
    /// Arbitrary named metrics, e.g. `{"peer_count": 12, "dht_lookup_ms": 340}`
    #[serde(default)]
    pub metrics: HashMap<String, serde_json::Value>,
}

/// Builder / recorder for a single test.
///
/// # Example
/// ```ignore
/// let mut rec = MetricsRecorder::start("integration::daemon::identify");
/// let peer_id = client.identify().await?;
/// rec.metric("peer_id_len", peer_id.len());
/// rec.finish(true);
/// ```
pub struct MetricsRecorder {
    test: String,
    tier: String,
    started: Instant,
    metrics: HashMap<String, serde_json::Value>,
}

impl MetricsRecorder {
    pub fn start(test: impl Into<String>, tier: impl Into<String>) -> Self {
        Self {
            test: test.into(),
            tier: tier.into(),
            started: Instant::now(),
            metrics: HashMap::new(),
        }
    }

    /// Record a named metric value (any JSON-serialisable type).
    pub fn metric(&mut self, key: impl Into<String>, value: impl Serialize) {
        if let Ok(v) = serde_json::to_value(value) {
            self.metrics.insert(key.into(), v);
        }
    }

    /// Finish recording and append the result to the JSONL file.
    ///
    /// Failures to write are printed to stderr but never panic — the test
    /// result is what matters.
    pub fn finish(self, passed: bool) {
        let duration_ms = self.started.elapsed().as_millis() as u64;
        let git_rev = git_short_rev();
        let timestamp = chrono::Utc::now().to_rfc3339();

        let record = TestRecord {
            timestamp,
            git_rev,
            test: self.test,
            tier: self.tier,
            passed,
            duration_ms,
            metrics: self.metrics,
        };

        if let Err(e) = append_record(&record) {
            eprintln!("[metrics] write failed: {e}");
        }

        // Also print a one-liner so CI logs show the metrics inline.
        let status = if passed { "PASS" } else { "FAIL" };
        let extras: Vec<String> = record
            .metrics
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        if extras.is_empty() {
            eprintln!("[metrics] {status} {} ({duration_ms}ms)", record.test);
        } else {
            eprintln!(
                "[metrics] {status} {} ({duration_ms}ms) — {}",
                record.test,
                extras.join(", ")
            );
        }
    }
}

fn append_record(record: &TestRecord) -> anyhow::Result<()> {
    let path = metrics_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let line = serde_json::to_string(record)?;
    writeln!(file, "{line}")?;
    Ok(())
}

fn git_short_rev() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Load all records from the metrics file (for use by the report binary).
pub fn load_records() -> anyhow::Result<Vec<TestRecord>> {
    let path = metrics_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path)?;
    let records = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    Ok(records)
}
