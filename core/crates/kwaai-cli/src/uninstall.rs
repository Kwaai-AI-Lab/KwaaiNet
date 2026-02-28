//! `kwaainet uninstall` — remove all KwaaiNet artefacts
//!
//! Removal order:
//!   1. Stop the running daemon (SIGTERM → SIGKILL after timeout)
//!   2. Uninstall the auto-start service (launchd / systemd)
//!   3. Remove the data directory (~/.kwaainet/)
//!   4. Remove the kwaainet binary and the co-installed p2pd binary

use anyhow::Result;
use std::io::{self, BufRead, Write as _};
use std::path::Path;

use crate::cli::UninstallArgs;
use crate::config::kwaainet_dir;
use crate::daemon::DaemonManager;
use crate::display::*;
use crate::service::get_service_manager;

pub fn run_uninstall(args: &UninstallArgs) -> Result<()> {
    print_box_header("KwaaiNet Uninstall");
    println!();

    // ── Confirmation ────────────────────────────────────────────────────────
    if !args.yes {
        println!("  This will remove:");
        if !args.keep_data {
            println!("    • All KwaaiNet data and configuration (~/.kwaainet/)");
        }
        println!("    • Auto-start service (if installed)");
        println!("    • kwaainet binary (and p2pd if present in the same directory)");
        println!();
        print!("  Proceed? [y/N] ");
        io::stdout().flush().ok();

        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).ok();
        let answer = line.trim().to_lowercase();
        if answer != "y" && answer != "yes" {
            println!("  Aborted.");
            return Ok(());
        }
        println!();
    }

    // ── 1. Stop running daemon ───────────────────────────────────────────────
    let mgr = DaemonManager::new();
    if mgr.is_running() {
        print!("  Stopping KwaaiNet daemon ... ");
        io::stdout().flush().ok();
        match mgr.stop_process() {
            Ok(_) => println!("done"),
            Err(e) => println!("warning: {e}"),
        }
    }

    // ── 2. Uninstall auto-start service ─────────────────────────────────────
    {
        let svc = get_service_manager();
        if svc.status().installed {
            print!("  Uninstalling auto-start service ... ");
            io::stdout().flush().ok();
            match svc.uninstall() {
                Ok(_) => println!("done"),
                Err(e) => println!("warning: {e}"),
            }
        }
    }

    // ── 3. Remove data directory ─────────────────────────────────────────────
    if !args.keep_data {
        let dir = kwaainet_dir();
        if dir.exists() {
            print!("  Removing {} ... ", dir.display());
            io::stdout().flush().ok();
            match std::fs::remove_dir_all(&dir) {
                Ok(_) => println!("done"),
                Err(e) => println!("warning: {e}"),
            }
        }
    }

    // ── 4. Remove binaries ───────────────────────────────────────────────────
    remove_binaries();

    println!();
    print_success("KwaaiNet uninstalled successfully.");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// Binary removal helpers
// ---------------------------------------------------------------------------

fn remove_binaries() {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            print_warning(&format!("Could not determine binary path: {e}"));
            return;
        }
    };

    let bin_dir = match exe.parent() {
        Some(d) => d.to_path_buf(),
        None => {
            print_warning("Could not determine binary directory");
            return;
        }
    };

    #[cfg(windows)]
    let p2pd_name = "p2pd.exe";
    #[cfg(not(windows))]
    let p2pd_name = "p2pd";

    let p2pd = bin_dir.join(p2pd_name);

    #[cfg(windows)]
    {
        // On Windows a running .exe cannot be deleted in-place, but it CAN be
        // renamed (the OS tracks open executables by file ID, not name).
        // Strategy:
        //   1. Rename <binary> → <binary>.del  (frees the original name immediately)
        //   2. Schedule async deletion of the .del file via a batch script
        // The original path disappears synchronously; the .del file is a transient
        // leftover cleaned up once the process exits.
        remove_binary_windows(&exe);
        if p2pd.exists() {
            remove_binary_windows(&p2pd);
        }
    }

    #[cfg(not(windows))]
    {
        remove_binary_file(&exe);
        if p2pd.exists() {
            remove_binary_file(&p2pd);
        }
    }
}

/// On Unix: unlink the file at `path`.  If permission is denied, print the
/// `sudo rm` command the user needs to run manually.
#[cfg(not(windows))]
fn remove_binary_file(path: &Path) {
    print!("  Removing {} ... ", path.display());
    io::stdout().flush().ok();
    match std::fs::remove_file(path) {
        Ok(_) => println!("done"),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            println!("permission denied");
            println!("    Run manually: sudo rm \"{}\"", path.display());
        }
        Err(e) => println!("warning: {e}"),
    }
}

/// On Windows: rename the binary to `<path>.del` (freeing the original name
/// immediately), then launch a minimal batch script to delete the renamed
/// file once this process has exited.
///
/// Windows tracks open executables by file ID, not by name, so renaming a
/// running .exe always succeeds.  The original path disappears synchronously;
/// the .del file is a transient leftover cleaned up once the process exits.
#[cfg(windows)]
fn remove_binary_windows(path: &Path) {
    use std::fmt::Write as FmtWrite;

    print!("  Removing {} ... ", path.display());
    io::stdout().flush().ok();

    // Step 1: try direct deletion first (works when the file is not locked)
    if std::fs::remove_file(path).is_ok() {
        println!("done");
        return;
    }

    // Step 2: rename to <path>.del — frees the original name synchronously
    let del_path = {
        let mut p = path.to_path_buf();
        let mut ext = p.extension()
            .map(|e| e.to_string_lossy().into_owned())
            .unwrap_or_default();
        ext.push_str(".del");
        p.set_extension(&ext);
        p
    };

    if std::fs::rename(path, &del_path).is_err() {
        println!("permission denied");
        println!("    Run manually: del /F /Q \"{}\"", path.display());
        return;
    }

    // Original path is gone. Schedule async deletion of the .del file.
    let pid = std::process::id();
    let temp = std::env::var("TEMP")
        .or_else(|_| std::env::var("TMP"))
        .unwrap_or_else(|_| "C:\\Windows\\Temp".to_string());
    let script_path = format!(
        "{}\\kwaainet_cleanup_{}_{}.bat",
        temp,
        pid,
        del_path.file_name().unwrap_or_default().to_string_lossy()
    );

    let mut script = String::new();
    let _ = writeln!(script, "@echo off");
    let _ = writeln!(script, ":wait");
    let _ = writeln!(
        script,
        "tasklist /FI \"PID eq {}\" 2>nul | find /I \"kwaainet\" >nul 2>&1",
        pid
    );
    let _ = writeln!(script, "if not errorlevel 1 (");
    let _ = writeln!(script, "    ping -n 2 127.0.0.1 >nul");
    let _ = writeln!(script, "    goto wait");
    let _ = writeln!(script, ")");
    let _ = writeln!(script, "del /F /Q \"{}\"", del_path.display());
    let _ = writeln!(script, "del /F /Q \"%~f0\"");

    if std::fs::write(&script_path, &script).is_ok() {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "/min", "", &script_path])
            .spawn();
        println!("done");
    } else {
        println!("done (cleanup: delete {} manually)", del_path.display());
    }
}
