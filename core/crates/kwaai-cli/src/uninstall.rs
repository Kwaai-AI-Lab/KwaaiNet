//! `kwaainet uninstall` — remove all KwaaiNet artefacts
//!
//! Removal order:
//!   1. Stop the running daemon (SIGTERM → SIGKILL after timeout)
//!   2. Uninstall the auto-start service (launchd / systemd)
//!   3. Remove the data directory (~/.kwaainet/)
//!   4. Remove the kwaainet binary and a legacy p2pd binary, if present

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
        if let Some(cmd) = crate::updater::packaged_remove_command() {
            println!("    • stray kwaainet copies in ~/.cargo/bin and ~/.local/bin");
            println!("  The packaged binary itself is left for: {cmd}");
        } else {
            println!("    • kwaainet binary (and a legacy p2pd if present in the same directory)");
        }
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
    let exe = match crate::updater::current_exe_path() {
        Some(p) => p,
        None => {
            print_warning("Could not determine binary path");
            return;
        }
    };
    let home = std::env::var_os("HOME").map(std::path::PathBuf::from);
    remove_binaries_from(
        &exe,
        crate::updater::packaged_remove_command(),
        home.as_deref(),
    );
}

/// `exe` (and a legacy p2pd beside it) is left to the package manager when
/// `packaged` names one; stray ~/.cargo/bin and ~/.local/bin copies go either way.
fn remove_binaries_from(exe: &Path, packaged: Option<&str>, home: Option<&Path>) {
    #[cfg(windows)]
    let (kwaainet_name, p2pd_name) = ("kwaainet.exe", "p2pd.exe");
    #[cfg(not(windows))]
    let (kwaainet_name, p2pd_name) = ("kwaainet", "p2pd");

    if let Some(cmd) = packaged {
        println!();
        print_warning(&format!(
            "{} belongs to your system package manager — leaving it in place.",
            exe.display()
        ));
        println!("    Remove it with: {cmd}");
    } else {
        remove_binary(exe);
        if let Some(p2pd) = exe.parent().map(|d| d.join(p2pd_name)) {
            if p2pd.exists() {
                remove_binary(&p2pd);
            }
        }
    }

    // Also remove from other known install locations that differ from the
    // currently-running binary:
    //   ~/.cargo/bin/  — cargo-dist installer default
    //   ~/.local/bin/  — original pre-v0.1.5 install.sh
    if let Some(home) = home {
        let extra_locations = [
            home.join(".cargo").join("bin").join(kwaainet_name),
            home.join(".local").join("bin").join(kwaainet_name),
        ];
        for alt_bin in &extra_locations {
            if alt_bin.exists() && alt_bin != exe {
                remove_binary(alt_bin);
            }
        }
    }
}

fn remove_binary(path: &Path) {
    #[cfg(not(windows))]
    remove_binary_file(path);
    #[cfg(windows)]
    remove_binary_windows(path);
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
        let mut ext = p
            .extension()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The binary name `remove_binaries_from` actually looks for. The fixture
    /// has to match it: on Windows the cleanup scans for `kwaainet.exe`, so a
    /// stray written as `kwaainet` is not the file under test.
    #[cfg(windows)]
    const STRAY: &str = "kwaainet.exe";
    #[cfg(not(windows))]
    const STRAY: &str = "kwaainet";

    fn fake_install(home: &Path, exe: &Path) {
        let cargo = home.join(".cargo").join("bin");
        std::fs::create_dir_all(&cargo).unwrap();
        std::fs::write(cargo.join(STRAY), b"stale").unwrap();
        std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
        std::fs::write(exe, b"exe").unwrap();
    }

    /// A packaged uninstall keeps the package's binary but still clears the
    /// PATH-shadowing copies no package manager tracks.
    #[test]
    fn packaged_uninstall_keeps_exe_and_clears_stray_copies() {
        let tmp = tempfile::tempdir().unwrap();
        let exe = tmp.path().join("usr").join("bin").join("kwaainet");
        fake_install(tmp.path(), &exe);

        remove_binaries_from(&exe, Some("sudo apt remove kwaainet"), Some(tmp.path()));

        assert!(exe.exists(), "packaged exe must survive");
        assert!(!tmp.path().join(".cargo").join("bin").join(STRAY).exists());
    }

    #[test]
    fn unpackaged_uninstall_removes_everything() {
        let tmp = tempfile::tempdir().unwrap();
        let exe = tmp.path().join("opt").join("kwaainet");
        fake_install(tmp.path(), &exe);

        remove_binaries_from(&exe, None, Some(tmp.path()));

        assert!(!exe.exists());
        assert!(!tmp.path().join(".cargo").join("bin").join(STRAY).exists());
    }
}
