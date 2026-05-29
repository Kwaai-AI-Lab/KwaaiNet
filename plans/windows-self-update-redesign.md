# Plan: Windows Self-Update Redesign — Synchronous Rust

**Status:** Ready to implement  
**Branch:** `fix/updater-ps1-move-retry` (add to existing PR #65)  
**Files changed:** `core/crates/kwaai-cli/Cargo.toml`, `updater.rs`, `main.rs`

---

## Problem

The Windows updater has failed silently across a dozen-plus commits. The structure is the bug:

1. `install_update()` generates a PowerShell PS1 script and spawns it detached with `CREATE_NO_WINDOW`
2. The PS1 runs invisible — stdout/stderr piped to null
3. `Move-Item` errors are caught but not re-thrown; the catch block increments a counter and sleeps, then after exhausting retries the script **unconditionally logs "Installed"** and continues
4. The script deletes itself at the end, destroying the audit trail
5. The daemon restarts on the old binary; the user sees no error

Every fix since `0xff5abc` has patched symptoms of this design. Time to delete it.

---

## Why the New Approach Works

### The PS1 never needed to kill the daemon
`main.rs` lines 821–838 already call `DaemonManager::stop_process()` (taskkill /PID) **before** `install_update()` is called. The daemon is dead before any swap happens. The kill inside the PS1 was redundant — and the 8-second sleep before it was waiting for a process that was already gone.

### Renaming a running EXE on Windows Vista+ is legal
The OS loader opens EXE files with FILE_SHARE_DELETE at the kernel level. A process can rename its own executable while running — the memory mapping stays valid because it points to the file data, not the name. This is exactly what `self_update`, `self-replace`, Chrome, Firefox, and VS Code use. `std::fs::rename("kwaainet.exe", "kwaainet.exe.old")` succeeds from inside a running `kwaainet.exe`.

### A `zip` crate is already in the project
`kwaai-rag` uses `zip = { version = "2", default-features = false, features = ["deflate"] }`. Adding the same line to `kwaai-cli` adds no new external crates.

---

## Changes

### 1. `core/crates/kwaai-cli/Cargo.toml`

Add under `[dependencies]`:
```toml
zip = { version = "2", default-features = false, features = ["deflate"] }
```

---

### 2. `core/crates/kwaai-cli/src/updater.rs` — Windows path

Delete everything from `let ps1 = …` through the final `println!("  Daemon will restart automatically.")` (~55 lines of PS1 templating and spawning).

Replace with synchronous Rust:

```rust
// ── Extract zip ──────────────────────────────────────────────────────────────
let tmp = canonical_temp.join("kwaainet-upd-extract");
if tmp.exists() {
    std::fs::remove_dir_all(&tmp).context("Clearing old extract dir")?;
}
std::fs::create_dir_all(&tmp)?;

print!("  Extracting…");
let _ = std::io::Write::flush(&mut std::io::stdout());
{
    let zip_file = std::fs::File::open(&zip_path).context("Opening downloaded zip")?;
    let mut archive = zip::ZipArchive::new(zip_file).context("Reading zip")?;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let raw_name = entry.name().to_string();
        let fname = std::path::Path::new(&raw_name)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if fname.ends_with(".exe") || (is_cuda && fname.ends_with(".dll")) {
            let dest = tmp.join(&fname);
            let mut out = std::fs::File::create(&dest)
                .with_context(|| format!("Creating {fname}"))?;
            std::io::copy(&mut entry, &mut out)?;
        }
    }
}
println!(" done.");

// ── Swap binary ──────────────────────────────────────────────────────────────
// Rename old exe aside (works on Windows Vista+ while process is running —
// loader uses FILE_SHARE_DELETE), then copy new files in.
let exe_bak = install_dir.join("kwaainet.exe.old");
let _ = std::fs::remove_file(&exe_bak);
std::fs::rename(&kwaainet_exe, &exe_bak)
    .context("Could not rename kwaainet.exe — is another process holding it?")?;

for entry in std::fs::read_dir(&tmp).context("Reading extract dir")? {
    let entry = entry?;
    let name = entry.file_name();
    let dest = install_dir.join(&name);
    std::fs::copy(entry.path(), &dest)
        .with_context(|| format!("Installing {}", name.to_string_lossy()))?;
    println!("  Installed {}", name.to_string_lossy());
}

// ── Clean up ─────────────────────────────────────────────────────────────────
let _ = std::fs::remove_file(&zip_path);
let _ = std::fs::remove_dir_all(&tmp);
let _ = std::fs::remove_file(&exe_bak);
```

Also delete the now-unused variables: `log`, `log_path`, `log_str`, `exe_str`, `file_include`, `ps1`, `ps1_content`, `ps1_real`, `zip_str`, `dir_str`.
Keep: `canonical_temp`, `install_dir`, `zip_path`, `kwaainet_exe`, `is_cuda`, the download section, and the 8.3-path canonicalization (still needed for `zip_path`).

---

### 3. `core/crates/kwaai-cli/src/main.rs` — Windows update handler

**Step A**: Capture `daemon_was_running` in the existing stop block (lines 821–838):

```rust
#[cfg(windows)]
let daemon_was_running = {
    let shard_mgr = ShardManager::new();
    if shard_mgr.is_running() {
        shard_mgr.stop_process();
        print_info("Shard server stopping…");
    }
    let storage_mgr = StorageApiManager::new();
    if storage_mgr.is_running() {
        storage_mgr.stop_process();
        print_info("Storage API stopping…");
    }
    let node_mgr = DaemonManager::new();
    let was = node_mgr.is_running();
    if was {
        let _ = node_mgr.stop_process();
        print_info("Daemon stopping…");
    }
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    was
};
```

**Step B**: Replace the post-install Windows block (lines 871–877) with the same pattern Linux already uses (lines 854–917):

```rust
#[cfg(windows)]
if let Err(e) = checker.install_update(&info.version).await {
    if daemon_was_running {
        let _ = std::process::Command::new(
            std::env::current_exe().unwrap_or_else(|_| "kwaainet".into()),
        )
        .args(["start", "--daemon"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    }
    print_error(&format!("Update failed: {e:#}"));
    print_separator();
    return Ok(());
}
#[cfg(windows)]
{
    if daemon_was_running {
        let new_bin = install_dir.join("kwaainet.exe");
        match std::process::Command::new(&new_bin)
            .args(["start", "--daemon"])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            Ok(_) => print_success(&format!(
                "Updated to v{} — daemon restarted with new binary.",
                info.version
            )),
            Err(e) => print_warning(&format!(
                "Updated to v{} but daemon restart failed ({e}).\n  Run: kwaainet start --daemon",
                info.version
            )),
        }
    } else {
        print_success(&format!(
            "Updated to v{}. Run 'kwaainet start --daemon' to start the node.",
            info.version
        ));
    }
}
```

**Note**: `install_dir` needs to be in scope here. Extract it from the same logic `install_update()` uses (current exe → parent dir), or pass it back. Simplest: expose `install_dir` computation as a small helper in `updater.rs` or inline the same `current_exe().parent()` logic.

---

## Failure Modes — All Now Visible

| Scenario | Old | New |
|---|---|---|
| Zip corrupt or incomplete | PS1 exits silently | `zip::ZipArchive::new()` returns `Err`, printed |
| Rename of old exe fails (AV lock?) | Never attempted | Error: "Could not rename kwaainet.exe — is another process holding it?" |
| Copy of new exe fails | Silent after fake "Installed" log | Error with exact filename |
| Daemon restart fails | PS1 `Start-Process` silently errors | Warning with manual command shown |
| Success | Unclear; user checks log | "Installed kwaainet.exe", "Updated to v0.4.92 — daemon restarted" |

---

## What Gets Deleted

- PS1 template string (~35 lines of `\r\n\`-escaped PowerShell in `format!()`)
- `DETACHED_PROCESS | CREATE_NO_WINDOW` process spawn
- Background installer log file (`kwaainet-update.log`)
- `log`, `log_path`, `log_str`, `exe_str`, `dir_str`, `file_include`, `ps1*` variables
- `"  Installer running in background."` / `"  Log: …"` / `"  Daemon will restart automatically."` prints
- The `is_cuda` variable (still computed for zip extraction filter — keep it)

---

## Verification

```powershell
# Build
cargo build -p kwaainet --release

# With daemon running
kwaainet update
# Expected: prints each step synchronously, shows "daemon restarted"

# Verify
kwaainet --version   # must show new version number

# Test failure path: hold kwaainet.exe open in another process, then run update
# Expected: prints "Could not rename kwaainet.exe", NOT silent failure
```
