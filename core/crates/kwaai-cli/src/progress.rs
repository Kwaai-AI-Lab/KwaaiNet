//! Terminal progress indicators for long-running operations.
//!
//! All output is suppressed when stdout is not a TTY (piped / CI).

use std::io::Write as _;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

const FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// Returns `true` when stdout is connected to a real terminal.
pub fn is_tty() -> bool {
    use std::io::IsTerminal as _;
    std::io::stdout().is_terminal()
}

// ── Spinner ───────────────────────────────────────────────────────────────────

/// Background spinner that overwrites one terminal line every 80 ms.
///
/// Call [`Spinner::start`] before a slow operation and [`Spinner::finish`] when done.
/// When stdout is not a TTY, `start` prints the label once and `finish` prints the
/// final message; no ANSI escape codes are emitted.
pub struct Spinner {
    stop_tx: Option<oneshot::Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

impl Spinner {
    pub fn start(label: impl Into<String>) -> Self {
        if !is_tty() {
            println!("  {}…", label.into());
            return Self {
                stop_tx: None,
                handle: None,
            };
        }
        let (stop_tx, mut stop_rx) = oneshot::channel::<()>();
        let label = label.into();
        let handle = tokio::spawn(async move {
            let start = Instant::now();
            let mut i = 0usize;
            loop {
                tokio::select! {
                    biased;
                    _ = &mut stop_rx => break,
                    _ = tokio::time::sleep(Duration::from_millis(80)) => {}
                }
                let elapsed = start.elapsed().as_secs_f64();
                print!(
                    "\r  {}  {}  {:.1}s  ",
                    FRAMES[i % FRAMES.len()],
                    label,
                    elapsed,
                );
                std::io::stdout().flush().ok();
                i += 1;
            }
        });
        Self {
            stop_tx: Some(stop_tx),
            handle: Some(handle),
        }
    }

    /// Stop the spinner and replace its line with `msg`.
    pub async fn finish(mut self, msg: impl Into<String>) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
        if let Some(h) = self.handle.take() {
            let _ = h.await;
        }
        if is_tty() {
            print!("\r\x1b[K  {}\n", msg.into());
        } else {
            println!("  {}", msg.into());
        }
        std::io::stdout().flush().ok();
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
        // Clear the spinner line when dropped without finish().
        if is_tty() {
            print!("\r\x1b[K");
            std::io::stdout().flush().ok();
        }
    }
}

// ── GenBar ────────────────────────────────────────────────────────────────────

fn bar_str(done: usize, total: usize, width: usize) -> String {
    let filled = if total == 0 {
        0
    } else {
        (done.min(total) * width) / total
    };
    format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(width.saturating_sub(filled))
    )
}

fn fmt_eta(secs: f64) -> String {
    let s = secs.max(0.0) as u64;
    if s < 60 {
        format!("~{}s", s)
    } else {
        format!("~{}m{}s", s / 60, s % 60)
    }
}

/// Live progress bar for token generation.
///
/// After each token is printed to stdout, call [`GenBar::tick`].  The bar is
/// rendered on the line *below* the streaming output using ANSI cursor control
/// and is updated in-place so it never scrolls off screen.
///
/// Only active when stdout is a TTY; silently no-ops otherwise.
pub struct GenBar {
    max: usize,
    recent: Vec<f64>,
    tty: bool,
    ever_ticked: bool,
}

impl GenBar {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max: max_tokens,
            recent: Vec::with_capacity(8),
            tty: is_tty(),
            ever_ticked: false,
        }
    }

    /// Update the status bar. `done` = tokens generated so far, `tok_ms` = wall
    /// time for the most recent token.
    ///
    /// Uses ANSI sequences to keep the bar on the line below the token stream:
    /// `\n` → next line, clear, print bar, `\x1b[1A\x1b[999C` → back to end of output.
    pub fn tick(&mut self, done: usize, tok_ms: f64) {
        const WINDOW: usize = 8;
        if self.recent.len() == WINDOW {
            self.recent.remove(0);
        }
        self.recent.push(tok_ms);
        self.ever_ticked = true;

        if !self.tty {
            return;
        }

        let avg_ms = self.recent.iter().sum::<f64>() / self.recent.len() as f64;
        let tps = if avg_ms > 0.0 { 1000.0 / avg_ms } else { 0.0 };
        let remaining = self.max.saturating_sub(done);
        let eta = if tps > 0.0 && done < self.max {
            format!(" • {}", fmt_eta(remaining as f64 / tps))
        } else {
            String::new()
        };

        let status = format!(
            "  {} {}/{} • {:.1} tok/s{}",
            bar_str(done, self.max, 20),
            done,
            self.max,
            tps,
            eta
        );

        // Render on the line below the output, then restore cursor position.
        print!("\n\r\x1b[K{}\x1b[1A\x1b[999C", status);
        std::io::stdout().flush().ok();
    }

    /// Clear the status bar and leave the cursor at the end of the output text.
    pub fn finish(&self) {
        if !self.tty || !self.ever_ticked {
            return;
        }
        print!("\n\r\x1b[K\x1b[1A\x1b[999C");
        std::io::stdout().flush().ok();
    }
}
