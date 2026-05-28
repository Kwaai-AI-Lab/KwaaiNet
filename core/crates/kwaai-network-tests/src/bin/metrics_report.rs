//! metrics-report — print a trend table from the JSONL metrics file.
//!
//! Usage:
//!   cargo run -p kwaai-network-tests --bin metrics-report
//!   cargo run -p kwaai-network-tests --bin metrics-report -- --tier integration
//!   cargo run -p kwaai-network-tests --bin metrics-report -- --last 10

use kwaai_network_tests::metrics::{load_records, TestRecord};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let tier_filter = find_flag(&args, "--tier");
    let last_n: Option<usize> = find_flag(&args, "--last").and_then(|s| s.parse().ok());

    let mut records = load_records()?;
    if records.is_empty() {
        println!("No metrics recorded yet.");
        println!("Run: KWAAI_INTEGRATION_TESTS=1 KWAAI_NETWORK_TESTS=1 cargo test -p kwaai-network-tests");
        return Ok(());
    }

    // Filter by tier if requested
    if let Some(ref tier) = tier_filter {
        records.retain(|r| &r.tier == tier);
    }

    // Group by test name
    let mut by_test: HashMap<String, Vec<&TestRecord>> = HashMap::new();
    for r in &records {
        by_test.entry(r.test.clone()).or_default().push(r);
    }

    // Sort test names alphabetically for stable output
    let mut test_names: Vec<String> = by_test.keys().cloned().collect();
    test_names.sort();

    // Column widths
    let name_w = test_names
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(20)
        .max(40);

    println!();
    println!("KwaaiNet Network Test Metrics");
    println!(
        "Metrics file: {}",
        kwaai_network_tests::metrics::metrics_path().display()
    );
    println!();
    println!(
        "{:<name_w$}  {:>5}  {:>6}  {:>9}  {:>8}  {}",
        "Test", "Runs", "Pass%", "Avg(ms)", "Last(ms)", "Trend"
    );
    println!("{}", "─".repeat(name_w + 40));

    for name in &test_names {
        let runs = &by_test[name];
        let mut all_runs = runs.clone();
        all_runs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Apply --last N
        let window: Vec<&TestRecord> = if let Some(n) = last_n {
            all_runs.iter().rev().take(n).rev().copied().collect()
        } else {
            all_runs.clone()
        };

        let total = window.len();
        let passed = window.iter().filter(|r| r.passed).count();
        let pass_pct = if total > 0 {
            (passed as f64 / total as f64 * 100.0).round() as u32
        } else {
            0
        };

        let durations: Vec<u64> = window.iter().map(|r| r.duration_ms).collect();
        let avg_ms = if !durations.is_empty() {
            durations.iter().sum::<u64>() / durations.len() as u64
        } else {
            0
        };
        let last_ms = durations.last().copied().unwrap_or(0);

        // Trend: compare last 3 runs vs prior 3 runs
        let trend = compute_trend(&window);

        println!(
            "{:<name_w$}  {:>5}  {:>5}%  {:>9}  {:>8}  {}",
            name, total, pass_pct, avg_ms, last_ms, trend
        );
    }

    println!();

    // Summary by tier
    println!("Summary by tier:");
    let mut tier_stats: HashMap<&str, (usize, usize)> = HashMap::new();
    for r in &records {
        let e = tier_stats.entry(r.tier.as_str()).or_default();
        e.0 += 1;
        if r.passed {
            e.1 += 1;
        }
    }
    let mut tiers: Vec<&&str> = tier_stats.keys().collect();
    tiers.sort();
    for tier in tiers {
        let (total, passed) = tier_stats[tier];
        let pct = if total > 0 { passed * 100 / total } else { 0 };
        println!("  {tier:<12}  {passed}/{total} passed ({pct}%)");
    }

    // Notable metrics — print the latest value of any non-trivial metric
    println!();
    println!("Latest notable metrics:");
    for name in &test_names {
        let runs = &by_test[name];
        if let Some(latest) = runs.iter().max_by(|a, b| a.timestamp.cmp(&b.timestamp)) {
            for (k, v) in &latest.metrics {
                if [
                    "peer_count",
                    "put_ms",
                    "get_ms",
                    "find_ms",
                    "bootstrap_ms",
                    "connect_ms",
                    "cross_get_ms",
                    "success_rate",
                    "vpk_nodes_found",
                ]
                .contains(&k.as_str())
                {
                    println!("  {name}  {k}={v}");
                }
            }
        }
    }

    println!();
    Ok(())
}

fn find_flag(args: &[String], flag: &str) -> Option<String> {
    let pos = args.iter().position(|a| a == flag)?;
    args.get(pos + 1).cloned()
}

/// Returns a one-character trend indicator based on recent pass rates.
fn compute_trend(runs: &[&TestRecord]) -> &'static str {
    if runs.len() < 2 {
        return "·";
    }

    let half = runs.len() / 2;
    let (earlier, recent) = runs.split_at(half);

    let pass_rate = |slice: &[&TestRecord]| -> f64 {
        if slice.is_empty() {
            return 0.0;
        }
        slice.iter().filter(|r| r.passed).count() as f64 / slice.len() as f64
    };

    let avg_dur = |slice: &[&TestRecord]| -> f64 {
        if slice.is_empty() {
            return 0.0;
        }
        slice.iter().map(|r| r.duration_ms as f64).sum::<f64>() / slice.len() as f64
    };

    let pass_delta = pass_rate(recent) - pass_rate(earlier);
    let dur_delta = avg_dur(recent) - avg_dur(earlier);

    if pass_delta > 0.1 {
        "↑ improving"
    } else if pass_delta < -0.1 {
        "↓ regressing"
    } else if dur_delta < -50.0 {
        "→ faster"
    } else if dur_delta > 100.0 {
        "→ slower"
    } else {
        "→ stable"
    }
}
