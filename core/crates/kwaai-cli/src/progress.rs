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

/// Stat tracker for token generation.
///
/// Tokens stream to stdout uninterrupted.  Call [`GenBar::tick`] after each
/// token to record timing; call [`GenBar::tps`] at the end to get the
/// decode-phase throughput for the summary line.
pub struct GenBar {
    recent: Vec<f64>,
}

impl GenBar {
    pub fn new(_max_tokens: usize) -> Self {
        Self {
            recent: Vec::with_capacity(8),
        }
    }

    /// Record the wall-time for the most recent token (milliseconds).
    pub fn tick(&mut self, _done: usize, tok_ms: f64) {
        const WINDOW: usize = 8;
        if self.recent.len() == WINDOW {
            self.recent.remove(0);
        }
        self.recent.push(tok_ms);
    }

    /// Rolling-window decode throughput in tokens/second.
    pub fn tps(&self) -> f64 {
        if self.recent.is_empty() {
            return 0.0;
        }
        let avg_ms = self.recent.iter().sum::<f64>() / self.recent.len() as f64;
        if avg_ms > 0.0 {
            1000.0 / avg_ms
        } else {
            0.0
        }
    }

    /// No-op — no bar was drawn, nothing to clear.
    pub fn finish(&self) {}
}
