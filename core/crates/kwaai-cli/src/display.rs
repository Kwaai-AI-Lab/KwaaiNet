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

/// The explicit `KWAAINET_WIDTH` override, if set and in range.
///
/// `COLUMNS` is a shell variable and is not exported to child processes, and we take no
/// dependency on a terminal-size crate, so this env var is the only signal available.
///
/// Returning an `Option` rather than a resolved width is deliberate: a caller that
/// widens past the house width (the findings table wants 80) must tell "no override,
/// use my default" apart from "the user asked for 70". Collapsing the two and writing
/// `.max(80)` silently ignored every narrower override and wrapped every table row.
pub fn width_override() -> Option<usize> {
    std::env::var("KWAAINET_WIDTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|w| (20..=400).contains(w))
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

/// Round a byte index down to the nearest `char` boundary.
///
/// `str::floor_char_boundary` is still unstable, and the alternative — slicing and
/// hoping — panics the whole process on any multi-byte character, which in this
/// corpus means every curly quote, en dash, and accented name.
fn floor_char_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Truncate to `cols` display columns, keeping the byte range `start..end` visible.
///
/// Used to centre an evidence quote on the mention the extractor actually matched,
/// rather than on the head of the passage — the trigger is frequently past the point
/// where a plain head-truncation would cut.
pub fn truncate_around(s: &str, start: usize, end: usize, cols: usize) -> String {
    if display_width(s) <= cols {
        return s.to_string();
    }
    // Callers derive these offsets from separate passes over the text — a sanitized
    // haystack and an unsanitized span — so an index can land mid-character. Clamping
    // to `s.len()` bounds it but does not make it sliceable; snapping to a boundary does.
    let start = floor_char_boundary(s, start.min(s.len()));
    let end = floor_char_boundary(s, end.clamp(start, s.len()));
    let trig_w = display_width(&s[start..end]);
    // Leave room for a leading and trailing ellipsis.
    let room = cols.saturating_sub(2);
    if trig_w >= room {
        return truncate_to_width(&s[start..], cols);
    }
    let before_budget = (room - trig_w) / 2;

    // Walk back from `start` over whole characters, preferring a word boundary.
    let mut lo = start;
    let mut w = 0usize;
    for (i, ch) in s[..start].char_indices().rev() {
        let cw = display_width(&ch.to_string());
        if w + cw > before_budget {
            break;
        }
        w += cw;
        lo = i;
    }
    if lo > 0 {
        if let Some(sp) = s[lo..start].find(' ') {
            lo += sp + 1;
        }
    }
    let head = if lo > 0 { "…" } else { "" };
    let tail_budget =
        cols.saturating_sub(display_width(head) + display_width(&s[lo..start]) + trig_w);
    let rest = truncate_to_width(&s[end..], tail_budget);
    format!("{head}{}{}", &s[lo..end], rest)
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

#[cfg(test)]
mod truncate_around_tests {
    use super::*;

    /// Regression: `end` arriving mid-character must not panic.
    ///
    /// Callers find the trigger in a sanitized copy but historically carried the
    /// *unsanitized* span length across, so `end` could land inside a multi-byte
    /// character. `&s[start..end]` then panicked and took the whole chat turn with it.
    #[test]
    fn mid_character_end_does_not_panic() {
        // Curly quotes and en dashes are three bytes each, so almost any off-by-one
        // lands inside one of them.
        let s = "a trigger word \u{201c}quoted\u{201d} and then a long tail \u{2014} more text here to overflow";
        let start = s.find("trigger").unwrap();
        for off in 0..12 {
            let end = start + "trigger".len() + off;
            // Must not panic for any end, on or off a boundary.
            let out = truncate_around(s, start, end, 40);
            assert!(!out.is_empty());
        }
    }

    /// A `start`/`end` past the end of the string is clamped, not panicked on.
    #[test]
    fn out_of_range_offsets_are_clamped() {
        let s = "short \u{2014} but multi-byte, and long enough to need truncating somewhere";
        assert!(!truncate_around(s, 9999, 10001, 20).is_empty());
        assert!(!truncate_around(s, 0, 9999, 20).is_empty());
    }

    /// The trigger stays visible when the string is truncated around it.
    #[test]
    fn keeps_the_trigger_visible() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa NEEDLE bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        let start = s.find("NEEDLE").unwrap();
        let out = truncate_around(s, start, start + "NEEDLE".len(), 30);
        assert!(out.contains("NEEDLE"), "trigger dropped: {out}");
    }
}

#[cfg(test)]
mod width_tests {
    use super::*;

    /// Regression: an explicit override must win in *both* directions.
    ///
    /// The findings table wanted 80 columns and wrote `term_width().max(80)`, which
    /// silently discarded a *narrower* KWAAINET_WIDTH and wrapped every row on an
    /// 80-column terminal. Callers now distinguish "no override" from "the user asked
    /// for 70" via `width_override()`.
    #[test]
    fn width_override_reports_only_an_explicit_in_range_value() {
        // The env var is process-global; this test owns it for its duration.
        std::env::set_var("KWAAINET_WIDTH", "70");
        assert_eq!(width_override(), Some(70));
        assert_eq!(
            width_override().unwrap_or(80),
            70,
            "narrow override ignored"
        );

        std::env::set_var("KWAAINET_WIDTH", "120");
        assert_eq!(width_override().unwrap_or(80), 120);

        // Out of range and non-numeric are not overrides.
        std::env::set_var("KWAAINET_WIDTH", "5");
        assert_eq!(width_override(), None);
        std::env::set_var("KWAAINET_WIDTH", "wide");
        assert_eq!(width_override(), None);
        assert_eq!(width_override().unwrap_or(80), 80);

        std::env::remove_var("KWAAINET_WIDTH");
        assert_eq!(width_override(), None);
    }
}
