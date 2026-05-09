//! Demos three candidate output layouts for shard inference progress.
//!
//! Run:  cargo run --example progress_demo -- [A|B|C]
//!
//!   A — bar BELOW output (current behaviour, breaks on wrap)
//!   B — bar ABOVE output, cursor save/restore (proposed)
//!   C — clean output only, summary stats at end (no live bar)

use std::io::Write as _;
use std::time::Duration;

const FAKE_TOKENS: &[&str] = &[
    "The",
    " capital",
    " of",
    " France",
    " is",
    " Paris",
    ".",
    " It",
    " is",
    " a",
    " beautiful",
    " city",
    " on",
    " the",
    " Seine",
    " river",
    " and",
    " one",
    " of",
    " the",
    " most",
    " visited",
    " cities",
    " in",
    " the",
    " world",
    ".",
    " The",
    " Eiffel",
    " Tower",
    " stands",
    " tall",
    " over",
    " the",
    " city",
    " skyline",
    ".",
    " Paris",
    " is",
    " also",
    " known",
    " for",
    " its",
    " world",
    "-",
    "class",
    " cuisine",
    ".",
];

const FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

fn bar_str(done: usize, total: usize, width: usize) -> String {
    let filled = (done.min(total) * width).checked_div(total).unwrap_or(0);
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

// ── Design A: bar below cursor (current) ─────────────────────────────────────

fn demo_a() {
    println!("  ⠋  Prefilling 4 input tokens…");
    std::thread::sleep(Duration::from_millis(600));
    print!("\r\x1b[K  ✓ Prefill  612 ms  (4 input tokens)\n");
    println!();
    print!("  Assistant: ");
    std::io::stdout().flush().ok();

    let total = FAKE_TOKENS.len();
    for (i, tok) in FAKE_TOKENS.iter().enumerate() {
        std::thread::sleep(Duration::from_millis(120));
        print!("{}", tok);
        std::io::stdout().flush().ok();

        // bar below: \n clear print bar \x1b[1A\x1b[999C
        if i > 0 {
            let tps = 8.3f64;
            let eta = fmt_eta((total - i) as f64 / tps);
            let status = format!(
                "  {} {}/{} • {:.1} tok/s • {}",
                bar_str(i + 1, total, 20),
                i + 1,
                total,
                tps,
                eta
            );
            print!("\n\r\x1b[K{}\x1b[1A\x1b[999C", status);
            std::io::stdout().flush().ok();
        }
    }

    println!("\n");
    println!("  ✓ Generated {} token(s)  •  8.3 tok/s  •  5.8s", total);
}

// ── Design B: bar above output, cursor save/restore ──────────────────────────

fn demo_b() {
    // Spinner simulation
    for frame in &FRAMES[..6] {
        print!("\r  {}  Prefilling 4 input tokens…  ", frame);
        std::io::stdout().flush().ok();
        std::thread::sleep(Duration::from_millis(80));
    }
    print!("\r\x1b[K  ✓ Prefill  612 ms  (4 input tokens)\n");

    let total = FAKE_TOKENS.len();
    let term_width: usize = {
        // probe terminal width, fall back to 80
        #[cfg(unix)]
        {
            let mut ws = libc_winsize();
            if unsafe { libc_ioctl(&mut ws) } == 0 {
                ws.ws_col as usize
            } else {
                80
            }
        }
        #[cfg(not(unix))]
        80
    };

    // Print bar placeholder, blank line, and "  Assistant:" header.
    // After this, cursor is 3 lines below the bar.
    let init_bar = format!("  {} 0/{} • -- tok/s", bar_str(0, total, 20), total);
    println!("{}", init_bar); // line B (bar slot)
    println!(); // line B+1 (blank separator)
    println!("  Assistant:"); // line B+2
    print!("  "); // indent; cursor at (B+3, col=2)
    std::io::stdout().flush().ok();

    let mut col: usize = 2;
    let mut lines_below: usize = 3;

    for (i, tok) in FAKE_TOKENS.iter().enumerate() {
        std::thread::sleep(Duration::from_millis(120));

        // Print token
        print!("{}", tok);
        std::io::stdout().flush().ok();

        // Track cursor column / line wraps
        for ch in tok.chars() {
            if ch == '\n' {
                col = 0;
                lines_below += 1;
            } else {
                col += 1;
                if col >= term_width {
                    col = 0;
                    lines_below += 1;
                }
            }
        }

        // Update bar: save cursor → go up to bar line → rewrite → restore
        let tps = 8.3f64;
        let eta = fmt_eta((total.saturating_sub(i + 1)) as f64 / tps);
        let bar_line = format!(
            "  {} {}/{} • {:.1} tok/s • {}",
            bar_str(i + 1, total, 20),
            i + 1,
            total,
            tps,
            eta
        );
        print!("\x1b[s\x1b[{}A\r\x1b[K{}\x1b[u", lines_below, bar_line);
        std::io::stdout().flush().ok();
    }

    println!("\n");
    println!("  ✓ Generated {} token(s)  •  8.3 tok/s  •  5.8s", total);
}

// ── Design C: no live bar, clean output, summary after ───────────────────────

fn demo_c() {
    for frame in &FRAMES[..6] {
        print!("\r  {}  Prefilling 4 input tokens…  ", frame);
        std::io::stdout().flush().ok();
        std::thread::sleep(Duration::from_millis(80));
    }
    print!("\r\x1b[K  ✓ Prefill  612 ms  (4 input tokens)\n");
    println!();
    println!("  Assistant:");
    print!("  ");
    std::io::stdout().flush().ok();

    let total = FAKE_TOKENS.len();
    for tok in FAKE_TOKENS {
        std::thread::sleep(Duration::from_millis(120));
        print!("{}", tok);
        std::io::stdout().flush().ok();
    }

    println!("\n");
    println!(
        "  ✓ Generated {} token(s)  •  8.3 tok/s  •  5.8s total",
        total
    );
}

// ── libc shim for terminal width ─────────────────────────────────────────────

#[cfg(unix)]
#[repr(C)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}
#[cfg(unix)]
fn libc_winsize() -> Winsize {
    Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    }
}
#[cfg(unix)]
unsafe fn libc_ioctl(ws: &mut Winsize) -> i32 {
    extern "C" {
        fn ioctl(fd: i32, request: u64, ...) -> i32;
    }
    ioctl(1, 0x5413, ws as *mut Winsize)
}

// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let design = std::env::args().nth(1).unwrap_or_else(|| "B".to_string());
    println!();
    println!("  ══ Design {} ══", design);
    println!();
    match design.to_uppercase().as_str() {
        "A" => demo_a(),
        "B" => demo_b(),
        "C" => demo_c(),
        other => {
            eprintln!("Unknown design '{}'. Use A, B, or C.", other);
            std::process::exit(1);
        }
    }
    println!();
}
