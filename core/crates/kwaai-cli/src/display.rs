//! Console formatting helpers

use unicode_width::UnicodeWidthStr;

const WIDTH: usize = 69;

/// Returns the display width of a string in terminal columns, accounting for
/// East Asian Width and emoji presentation flags via `unicode-width`.
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

pub fn print_box_header(title: &str) {
    println!();
    println!("╭{}╮", "─".repeat(WIDTH));
    let title_w = display_width(title);
    let pad = (WIDTH.saturating_sub(title_w)) / 2;
    let right_pad = WIDTH.saturating_sub(pad + title_w);
    println!("│{}{}{}│", " ".repeat(pad), title, " ".repeat(right_pad));
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
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if mins > 0 {
        parts.push(format!("{}m", mins));
    }
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

// ── width helpers ─────────────────────────────────────────────────────────────

/// Usable terminal width in columns.
///
/// `COLUMNS` is a shell variable and is not exported to child processes, and we take
/// no dependency on a terminal-size crate, so this returns the house width of 69 and
/// offers `KWAAINET_WIDTH` as an explicit override.
pub fn term_width() -> usize {
    std::env::var("KWAAINET_WIDTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|w| (20..=400).contains(w))
        .unwrap_or(WIDTH)
}

/// Truncate to at most `cols` display columns, appending `…` when shortened.
pub fn truncate_to_width(s: &str, cols: usize) -> String {
    if display_width(s) <= cols {
        return s.to_string();
    }
    if cols == 0 {
        return String::new();
    }
    let budget = cols.saturating_sub(1);
    let mut out = String::new();
    let mut w = 0usize;
    for ch in s.chars() {
        let cw = UnicodeWidthStr::width(ch.to_string().as_str());
        if w + cw > budget {
            break;
        }
        out.push(ch);
        w += cw;
    }
    out.push('…');
    out
}

/// Flatten a value for use inside a table cell.
///
/// Chunk text routinely contains newlines and tabs; a single unsanitized cell
/// destroys the alignment of the whole table.
pub fn sanitize_cell(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_space = false;
    for ch in s.chars() {
        let c = if ch == '\n' || ch == '\r' || ch == '\t' {
            ' '
        } else if ch.is_control() {
            continue;
        } else {
            ch
        };
        if c == ' ' {
            if last_space {
                continue;
            }
            last_space = true;
        } else {
            last_space = false;
        }
        out.push(c);
    }
    out.trim().to_string()
}

// ── Table ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
}

/// A box-drawing table that **renders to lines rather than printing**.
///
/// Returning `Vec<String>` is deliberate: the caller decides which stream the lines
/// go to, which is what keeps `rag chat`'s stdout invariant enforceable, and it makes
/// the renderer unit-testable without a terminal.
pub struct Table {
    headers: Vec<String>,
    aligns: Vec<Align>,
    rows: Vec<Vec<String>>,
    /// Column allowed to absorb the shrink when the table is too wide.
    flex: usize,
}

impl Table {
    pub fn new(headers: &[&str]) -> Self {
        Self {
            headers: headers.iter().map(|h| h.to_string()).collect(),
            aligns: vec![Align::Left; headers.len()],
            rows: Vec::new(),
            flex: headers.len().saturating_sub(1),
        }
    }

    pub fn align(mut self, i: usize, a: Align) -> Self {
        if i < self.aligns.len() {
            self.aligns[i] = a;
        }
        self
    }

    /// Nominate the column that shrinks first when the table exceeds `max_width`.
    pub fn flex(mut self, i: usize) -> Self {
        self.flex = i;
        self
    }

    pub fn push(&mut self, cells: Vec<String>) {
        self.rows
            .push(cells.iter().map(|c| sanitize_cell(c)).collect());
    }

    /// Render to lines, capping the widest columns until the table fits `max_width`.
    ///
    /// Sizing is water-filling: find the level `L` at which capping every column to
    /// `min(natural, L)` fits, so the columns that give ground are the widest ones.
    /// Shrinking a single nominated column instead would starve exactly the column
    /// worth reading — the evidence quote is usually both the widest and the point.
    pub fn render(&self, max_width: usize) -> Vec<String> {
        const FLOOR: usize = 8;
        let n = self.headers.len();
        if n == 0 {
            return Vec::new();
        }
        let mut w: Vec<usize> = self.headers.iter().map(|h| display_width(h)).collect();
        for row in &self.rows {
            for (i, c) in row.iter().take(n).enumerate() {
                w[i] = w[i].max(display_width(c));
            }
        }
        // Frame overhead: "│ " before each cell and " │" after the last.
        let overhead = n * 3 + 1;
        let budget = max_width.saturating_sub(overhead);
        if w.iter().sum::<usize>() > budget {
            let mut lo = FLOOR.min(*w.iter().min().unwrap_or(&FLOOR));
            let mut hi = *w.iter().max().unwrap_or(&0);
            while lo < hi {
                let mid = (lo + hi).div_ceil(2);
                let total: usize = w.iter().map(|x| (*x).min(mid)).sum();
                if total <= budget {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            for x in w.iter_mut() {
                *x = (*x).min(lo.max(FLOOR));
            }
            // Hand any rounding slack to the nominated column.
            let used: usize = w.iter().sum();
            if used < budget && self.flex < n {
                w[self.flex] += budget - used;
            }
        }

        let bar = |l: &str, m: &str, r: &str| -> String {
            let mut s = String::from(l);
            for (i, wi) in w.iter().enumerate() {
                s.push_str(&"─".repeat(wi + 2));
                s.push_str(if i + 1 == n { r } else { m });
            }
            s
        };
        let row_line = |cells: &[String]| -> String {
            let mut s = String::from("│");
            for (i, wi) in w.iter().enumerate() {
                let raw = cells.get(i).map(|c| c.as_str()).unwrap_or("");
                let cell = truncate_to_width(raw, *wi);
                let pad = wi.saturating_sub(display_width(&cell));
                s.push(' ');
                if self.aligns[i] == Align::Right {
                    s.push_str(&" ".repeat(pad));
                    s.push_str(&cell);
                } else {
                    s.push_str(&cell);
                    s.push_str(&" ".repeat(pad));
                }
                s.push_str(" │");
            }
            s
        };

        let mut out = Vec::with_capacity(self.rows.len() + 4);
        out.push(bar("┌", "┬", "┐"));
        out.push(row_line(&self.headers));
        out.push(bar("├", "┼", "┤"));
        for r in &self.rows {
            out.push(row_line(r));
        }
        out.push(bar("└", "┴", "┘"));
        out
    }
}
