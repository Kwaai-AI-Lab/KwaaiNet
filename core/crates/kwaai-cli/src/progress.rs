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
    #[allow(dead_code)]
    pub fn finish(&self) {}
}

// ── stream predicates ─────────────────────────────────────────────────────────

/// Returns `true` when stderr is connected to a real terminal.
///
/// [`is_tty`] checks *stdout*, which is the wrong gate for anything written to
/// stderr. `rag chat` reserves stdout for the `You:` / `Assistant:` protocol and
/// sends every human-facing decoration to stderr, so it gates on this instead.
pub fn is_stderr_tty() -> bool {
    use std::io::IsTerminal as _;
    std::io::stderr().is_terminal()
}

/// Returns `true` when stdin is connected to a real terminal.
///
/// Interactive prompts must be gated on this: when stdin is a pipe, a prompt that
/// consumes a line would swallow the next scripted command.
pub fn is_stdin_tty() -> bool {
    use std::io::IsTerminal as _;
    std::io::stdin().is_terminal()
}

// ── StatusLine ────────────────────────────────────────────────────────────────

/// A single re-paintable status line on **stderr**, plus the scrolling writes above it.
///
/// This is the one owner of the terminal during a chat turn. The lesson recorded on
/// [`GenBar`] is that a redrawing bar cannot share a stream with another writer, so
/// every scrolling line goes through [`StatusLine::emit`], which clears the status,
/// writes the line, and repaints. There is no second writer and no drawing task.
///
/// When stderr is not a TTY, the status line is suppressed entirely and `emit` is a
/// plain `eprintln!` — no ANSI is ever emitted.
pub struct StatusLine {
    tty: bool,
    status: Option<String>,
    painted: bool,
    start: Instant,
    frame: usize,
}

impl StatusLine {
    pub fn stderr() -> Self {
        Self {
            tty: is_stderr_tty(),
            status: None,
            painted: false,
            start: Instant::now(),
            frame: 0,
        }
    }

    fn wipe(&mut self) {
        if self.tty && self.painted {
            eprint!("\r\x1b[K");
            self.painted = false;
        }
    }

    fn paint(&mut self) {
        if !self.tty {
            return;
        }
        if let Some(ref s) = self.status {
            eprint!("\r\x1b[K{s}");
            std::io::stderr().flush().ok();
            self.painted = true;
        }
    }

    /// Remove the status line and forget it.
    pub fn clear_status(&mut self) {
        self.wipe();
        self.status = None;
        std::io::stderr().flush().ok();
    }

    /// Write one scrolling line above the status, then repaint the status.
    ///
    /// Stdout is flushed first so that interleaving on a shared terminal preserves
    /// causal order between the two streams.
    pub fn emit(&mut self, line: &str) {
        std::io::stdout().flush().ok();
        self.wipe();
        eprintln!("{line}");
        self.paint();
        std::io::stderr().flush().ok();
    }

    /// Type `line` out one batch of characters at a time.
    ///
    /// `fast_fwd` is polled between batches; the moment it returns `true` the rest of
    /// the line is written with no further delay. Pacing exists to fill dead time and
    /// must never become dead time.
    pub async fn emit_paced(
        &mut self,
        line: &str,
        per_char: Duration,
        fast_fwd: &dyn Fn() -> bool,
    ) {
        if !self.tty || per_char.is_zero() {
            self.emit(line);
            return;
        }
        // Batch characters so a long line costs ~14 wakeups/sec, not one per char.
        const BATCH: usize = 4;
        std::io::stdout().flush().ok();
        self.wipe();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0usize;
        let mut ff = false;
        while i < chars.len() {
            if !ff && fast_fwd() {
                ff = true;
            }
            let end = (i + BATCH).min(chars.len());
            let batch: String = chars[i..end].iter().collect();
            eprint!("{batch}");
            std::io::stderr().flush().ok();
            i = end;
            if !ff && i < chars.len() {
                tokio::time::sleep(per_char * BATCH as u32).await;
            }
        }
        eprintln!();
        self.paint();
        std::io::stderr().flush().ok();
    }

    /// Advance the spinner and repaint as `⠹  label  1.2s`.
    pub fn tick_label(&mut self, label: &str) {
        self.frame = self.frame.wrapping_add(1);
        let f = FRAMES[self.frame % FRAMES.len()];
        let e = self.start.elapsed().as_secs_f64();
        self.status = Some(format!("  {f}  {label}  {e:.1}s"));
        self.paint();
    }
}

// ── LiveNarrator ──────────────────────────────────────────────────────────────

/// Owns stderr for the duration of a slow operation, animating a status line and
/// printing narration lines above it as they arrive.
///
/// Retrieval narrates through an `impl Fn(&str)` callback, which cannot mutate captured
/// state. A channel sidesteps that: the sender is `Clone + Sync` and `send` takes
/// `&self`, so the callback stays `Fn` with no interior mutability, and the receiving
/// task is the single writer to the stream.
pub struct LiveNarrator {
    tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    stop_tx: Option<oneshot::Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

impl LiveNarrator {
    pub fn start(label: impl Into<String>) -> Self {
        let label = label.into();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let (stop_tx, mut stop_rx) = oneshot::channel::<()>();
        let tty = is_stderr_tty();
        let handle = tokio::spawn(async move {
            let mut st = StatusLine::stderr();
            loop {
                tokio::select! {
                    biased;
                    _ = &mut stop_rx => break,
                    Some(line) = rx.recv() => st.emit(&line),
                    _ = tokio::time::sleep(Duration::from_millis(80)) => {
                        if tty {
                            st.tick_label(&label);
                        }
                    }
                }
            }
            // Drain anything still queued so no narration is lost on a fast finish.
            while let Ok(line) = rx.try_recv() {
                st.emit(&line);
            }
            st.clear_status();
        });
        Self {
            tx: Some(tx),
            stop_tx: Some(stop_tx),
            handle: Some(handle),
        }
    }

    /// A sender for the narration callback. Cheap to clone.
    pub fn sender(&self) -> tokio::sync::mpsc::UnboundedSender<String> {
        self.tx.as_ref().expect("narrator live").clone()
    }

    /// Stop animating and replace the status line with `msg`.
    pub async fn finish(mut self, msg: impl Into<String>) {
        // Drop the sender first so the task's drain loop terminates.
        self.tx.take();
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
        if let Some(h) = self.handle.take() {
            let _ = h.await;
        }
        eprintln!("{}", msg.into());
        std::io::stderr().flush().ok();
    }
}

impl Drop for LiveNarrator {
    fn drop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
    }
}

impl Drop for StatusLine {
    fn drop(&mut self) {
        // Never leave the cursor parked mid-line (e.g. on Ctrl-C).
        self.wipe();
        std::io::stderr().flush().ok();
    }
}
