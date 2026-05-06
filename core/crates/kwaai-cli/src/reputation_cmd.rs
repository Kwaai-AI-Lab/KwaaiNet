//! `kwaainet reputation` — display and manage the local peer trust store.

use anyhow::Result;

use crate::cli::{ReputationAction, ReputationArgs};
use crate::display::*;
use crate::reputation::{ReputationStore, TrustTier};

pub async fn run(args: ReputationArgs) -> Result<()> {
    match args.action {
        ReputationAction::List => list(),
        ReputationAction::Show { peer_id } => show(&peer_id),
        ReputationAction::Reset { peer_id } => reset_peer(&peer_id),
    }
}

// ---------------------------------------------------------------------------
// list
// ---------------------------------------------------------------------------

// Column widths (chars):  NAME=18  TIER=10  SCORE=7  N=6  LATENCY=9  PEER_ID=13
// Row total: 2 + 18 + 2 + 10 + 2 + 7 + 2 + 6 + 2 + 9 + 2 + 13 = 75
const TABLE_SEP: &str = "  ───────────────────────────────────────────────────────────────────────────";

fn list() -> Result<()> {
    let store = ReputationStore::load();
    let scored = store.all_scored();

    print_box_header("Peer Reputation — Local Trust View");

    if scored.is_empty() {
        print_info("No peer observations recorded yet.");
        print_info("Run `kwaainet shard run` to start collecting data.");
        return Ok(());
    }

    println!(
        "  {:<18}  {:<10}  {:<7}  {:<6}  {:<9}  {}",
        "PEER NAME", "TIER", "SCORE", "N", "LATENCY", "PEER ID"
    );
    println!("{TABLE_SEP}");

    for (peer_id, score) in &scored {
        let record = store.peer_record(peer_id);
        let name = record.map(|r| r.public_name.as_str()).unwrap_or("unknown");

        println!(
            "  {:<18}  {:<10}  {:<7.3}  {:<6}  {:<9}  {}",
            truncate(name, 18),
            tier_badge(score.tier),
            score.score,
            score.sample_count,
            format_latency(score.avg_latency_ms, score.sample_count),
            truncate(peer_id, 13),
        );
    }

    println!("{TABLE_SEP}");
    println!("  {} peer(s) observed.", scored.len());
    Ok(())
}

// ---------------------------------------------------------------------------
// show
// ---------------------------------------------------------------------------

fn show(peer_id: &str) -> Result<()> {
    let store = ReputationStore::load();

    // Accept prefix match.
    let matched = store
        .all_scored()
        .into_iter()
        .find(|(id, _)| id == peer_id || id.starts_with(peer_id));

    let Some((full_id, score)) = matched else {
        print_error(&format!("No reputation data for peer: {peer_id}"));
        print_info("Run `kwaainet reputation list` to see known peers.");
        return Ok(());
    };

    let record = store.peer_record(&full_id);
    let name = record.map(|r| r.public_name.as_str()).unwrap_or("unknown");

    // Box inner width = 69 display cols; "Peer: " = 6 cols.
    // "..." (U+2026) counts as 2 display cols, so max pure-ASCII name before ellipsis = 61.
    print_box_header(&format!("Peer: {}", truncate(name, 61)));
    println!("  ID:           {}", full_id);
    println!("  Trust tier:   {}", tier_badge(score.tier));
    println!("  Score:        {:.4}", score.score);
    println!("  Samples:      {}", score.sample_count);
    println!("  Availability: {:.1}%", score.availability * 100.0);
    println!("  Avg latency:  {}", format_latency(score.avg_latency_ms, score.sample_count));
    if let Some(ratio) = score.throughput_ratio {
        println!("  Throughput:   {:.1}% of claimed", ratio * 100.0);
    } else {
        println!("  Throughput:   — (no TPS measurements)");
    }
    if let Some(r) = record {
        println!("  First seen:   {} ago", format_age(r.first_seen_secs));
        println!("  Last seen:    {} ago", format_age(r.last_seen_secs));
    }

    println!();
    print_info("Tiers: Unknown (<5 samples) | Known (<0.40) | Verified (<0.70) | Trusted (>=0.70)");
    Ok(())
}

// ---------------------------------------------------------------------------
// reset
// ---------------------------------------------------------------------------

fn reset_peer(peer_id: &str) -> Result<()> {
    let mut store = ReputationStore::load();

    // Accept prefix match.
    let full_id = store
        .all_scored()
        .into_iter()
        .find(|(id, _)| id == peer_id || id.starts_with(peer_id))
        .map(|(id, _)| id);

    let Some(full_id) = full_id else {
        print_warning(&format!("No reputation data for peer: {peer_id}"));
        return Ok(());
    };

    store.reset(&full_id);
    print_success(&format!("Cleared reputation data for: {full_id}"));
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tier_badge(tier: TrustTier) -> &'static str {
    match tier {
        TrustTier::Unknown => "[UNKNOWN]",
        TrustTier::Known => "[KNOWN]",
        TrustTier::Verified => "[VERIFIED]",
        TrustTier::Trusted => "[TRUSTED]",
    }
}

/// Truncate a string to `max` Unicode scalar values, appending "..." if cut.
/// Uses char boundaries — never panics on multibyte input.
fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        // Reserve 1 char for the ellipsis.
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

/// Format a latency value into a bounded string (max 9 chars).
///
/// - <1 s   → "999 ms"
/// - <60 s  → "9.9 s"
/// - <3600 s → "99 min"
/// - else   → "99+ hr"
///
/// Returns "—" when sample_count is 0.
fn format_latency(avg_ms: f64, sample_count: usize) -> String {
    if sample_count == 0 {
        return "—".to_string();
    }
    if avg_ms < 1_000.0 {
        format!("{:.0} ms", avg_ms)
    } else if avg_ms < 60_000.0 {
        format!("{:.1} s", avg_ms / 1_000.0)
    } else if avg_ms < 3_600_000.0 {
        format!("{:.0} min", avg_ms / 60_000.0)
    } else {
        format!("{:.0}+ hr", avg_ms / 3_600_000.0)
    }
}

fn format_age(ts_secs: u64) -> String {
    let now = crate::reputation::now_secs();
    let elapsed = now.saturating_sub(ts_secs);
    format_uptime(elapsed)
}
