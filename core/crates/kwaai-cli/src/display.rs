//! Console formatting helpers

use std::fmt::Write as FmtWrite;

const WIDTH: usize = 69;

pub fn print_box_header(title: &str) {
    println!();
    println!("╭{}╮", "─".repeat(WIDTH));
    let pad = (WIDTH.saturating_sub(title.len())) / 2;
    let right_pad = WIDTH.saturating_sub(pad + title.len());
    println!("│{}{}{} │", " ".repeat(pad), title, " ".repeat(right_pad.saturating_sub(1)));
    println!("╰{}╯", "─".repeat(WIDTH));
    println!();
}

pub fn print_separator() {
    println!("{}", "─".repeat(WIDTH));
}

/// Format seconds into a human-readable uptime string.
pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let s = secs % 60;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if mins > 0 { parts.push(format!("{}m", mins)); }
    parts.push(format!("{}s", s));
    parts.join(" ")
}

/// Format bytes into a human-readable size string.
pub fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1_073_741_824;
    const MB: u64 = 1_048_576;
    const KB: u64 = 1_024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn print_success(msg: &str) {
    println!("  ✅ {}", msg);
}

pub fn print_error(msg: &str) {
    eprintln!("  ❌ {}", msg);
}

pub fn print_warning(msg: &str) {
    println!("  ⚠️  {}", msg);
}

pub fn print_info(msg: &str) {
    println!("  💡 {}", msg);
}
