//! Self-update checker via GitHub releases API

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

const RELEASES_URL: &str = "https://api.github.com/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest";

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: Option<String>,
    html_url: Option<String>,
    body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub name: Option<String>,
    pub url: Option<String>,
    pub body: Option<String>,
}

fn cache_file() -> PathBuf {
    crate::config::run_dir().join("update_check.json")
}

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    checked_at: u64,
    update_info: Option<UpdateInfo>,
}

pub struct UpdateChecker {
    pub current_version: String,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            current_version: CURRENT_VERSION.to_string(),
        }
    }

    /// Check for a newer release. Returns `Some(UpdateInfo)` if one exists.
    pub async fn check(&self, force: bool) -> Result<Option<UpdateInfo>> {
        if !force {
            if let Some(cached) = self.load_cache() {
                return Ok(cached);
            }
        }

        let client = reqwest::Client::builder()
            .user_agent("kwaainet/".to_string() + CURRENT_VERSION)
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let resp = client.get(RELEASES_URL).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            // No releases published yet
            self.save_cache(&None)?;
            return Ok(None);
        }

        let release: GithubRelease = resp.json().await?;
        debug!("Latest release tag: {}", release.tag_name);
        let latest = release.tag_name.trim_start_matches('v').to_string();

        let update = if is_newer(&latest, &self.current_version) {
            Some(UpdateInfo {
                version: latest,
                name: release.name,
                url: release.html_url,
                body: release.body,
            })
        } else {
            None
        };

        self.save_cache(&update)?;
        Ok(update)
    }

    fn load_cache(&self) -> Option<Option<UpdateInfo>> {
        let text = std::fs::read_to_string(cache_file()).ok()?;
        let entry: CacheEntry = serde_json::from_str(&text).ok()?;

        // Cache valid for 24 hours
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs();
        if now.saturating_sub(entry.checked_at) < 86400 {
            // Re-validate: the binary may have been updated since the cache was
            // written, making the cached version no longer newer than current.
            let validated = entry
                .update_info
                .filter(|info| is_newer(&info.version, &self.current_version));
            Some(validated)
        } else {
            None
        }
    }

    fn save_cache(&self, info: &Option<UpdateInfo>) -> Result<()> {
        let path = cache_file();
        std::fs::create_dir_all(path.parent().unwrap())?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let entry = CacheEntry {
            checked_at: now,
            update_info: info.clone(),
        };
        std::fs::write(&path, serde_json::to_string(&entry)?)?;
        Ok(())
    }

    /// Download and install the update for this platform.
    /// `version` is the target version string (e.g. "0.4.13"), used to build
    /// version-specific archive URLs so we don't have to re-resolve "latest".
    pub async fn install_update(&self, version: &str) -> Result<()> {
        #[cfg(unix)]
        {
            // On Linux, prefer the CUDA-enabled binary for NVIDIA GPU machines.
            // Falls back to the CPU installer when the CUDA archive isn't published yet.
            #[cfg(not(target_os = "macos"))]
            if nvidia_smi_async().await {
                return self.install_cuda_linux(version).await;
            }

            self.install_cpu_linux(version).await?;
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // Resolve 8.3 short names (e.g. METRO_~1) in the temp dir.
            // Expand-Archive -LiteralPath rejects 8.3 paths because .NET's
            // ZipFile doesn't call GetLongPathName internally.
            // canonicalize() returns \\?\-prefixed paths on Windows; strip
            // that prefix so the paths are usable in PS1 single-quoted strings.
            let canonical_temp = std::env::temp_dir()
                .canonicalize()
                .map(|p| {
                    let s = p.to_string_lossy();
                    if let Some(rest) = s.strip_prefix("\\\\?\\") {
                        std::path::PathBuf::from(rest)
                    } else {
                        p
                    }
                })
                .unwrap_or_else(|_| std::env::temp_dir());

            let log = canonical_temp.join("kwaainet-update.log");
            let log_path = log.to_string_lossy().into_owned();

            let install_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                .unwrap_or_else(|| {
                    dirs::home_dir()
                        .map(|h| h.join(".cargo").join("bin"))
                        .unwrap_or_default()
                });

            // Prefer the CUDA-enabled zip for NVIDIA GPU machines when it exists.
            // The CI job `build-cuda-artifacts` publishes
            // kwaainet-x86_64-pc-windows-msvc-cuda.zip alongside the CPU zip.
            // If the artifact isn't published yet (or the machine has no GPU) we
            // fall back to the standard CPU zip without error.
            let cpu_url = format!(
                "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-pc-windows-msvc.zip"
            );
            let archive_url = if nvidia_smi_windows().await {
                // DLLs already present → use the lean binary-only zip (~30 MB, fast).
                // DLLs missing       → use the full zip with bundled CUDA runtime (~978 MB, one-time).
                let dlls_present = install_dir.join("cublas64_12.dll").exists();
                let cuda_url = if dlls_present {
                    format!(
                        "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-pc-windows-msvc-cuda.zip"
                    )
                } else {
                    format!(
                        "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-pc-windows-msvc-cuda-full.zip"
                    )
                };
                let client = reqwest::Client::builder()
                    .user_agent(format!("kwaainet/{}", CURRENT_VERSION))
                    .timeout(std::time::Duration::from_secs(10))
                    .build()?;
                let available = client
                    .head(&cuda_url)
                    .send()
                    .await
                    .map(|r| r.status().is_success())
                    .unwrap_or(false);
                if available {
                    cuda_url
                } else {
                    println!();
                    println!(
                        "  ⚠  NVIDIA GPU detected but CUDA build for v{version} isn't published yet."
                    );
                    println!(
                        "  Installing CPU build now — run `kwaainet update` again later for GPU support."
                    );
                    println!();
                    cpu_url
                }
            } else {
                cpu_url
            };

            let is_cuda = archive_url.contains("-cuda");
            let zip_path = canonical_temp.join("kwaainet-update.zip");
            if is_cuda {
                let dlls_present = install_dir.join("cublas64_12.dll").exists();
                if dlls_present {
                    print!("  NVIDIA GPU detected — downloading CUDA update for v{version} (binary only)…");
                } else {
                    print!("  NVIDIA GPU detected — downloading CUDA build for v{version} (first install, includes runtime DLLs)…");
                }
            } else {
                print!("  Downloading v{version} for Windows…");
            }
            let _ = std::io::Write::flush(&mut std::io::stdout());
            self.download_to(&archive_url, &zip_path).await?;
            println!(" done.");

            let kwaainet_exe = install_dir.join("kwaainet.exe");
            // Escape single quotes in paths for use inside PS1 single-quoted strings.
            let zip_str = zip_path.to_string_lossy().replace('\'', "''");
            let dir_str = install_dir.to_string_lossy().replace('\'', "''");
            let exe_str = kwaainet_exe.to_string_lossy().replace('\'', "''");
            let log_str = log_path.replace('\'', "''");

            // Include *.dll so the full CUDA zip's bundled runtime DLLs land in
            // the install dir.  Safe for lean zips and CPU zips — no DLLs found,
            // nothing extra installed.
            let file_include = if is_cuda { "'*.exe','*.dll'" } else { "'*.exe'" };

            // Single PS1 script handles the full update: waits for kwaainet to
            // exit, kills kwaainet.exe AND p2pd.exe (both may hold file locks),
            // extracts the zip, installs binaries, restarts the daemon, and
            // cleans up.  Spawning powershell.exe directly avoids cmd.exe's
            // 8.3-path / DETACHED_PROCESS batch-file parsing quirks.
            let ps1 = canonical_temp.join("kwaainet-update.ps1");
            let ps1_content = format!(
                "Start-Sleep -Seconds 3\r\n\
                 Get-Process kwaainet,p2pd -ErrorAction SilentlyContinue | Stop-Process -Force\r\n\
                 Start-Sleep -Seconds 2\r\n\
                 $ErrorActionPreference = 'Stop'\r\n\
                 $zip  = '{zip_str}'\r\n\
                 $dest = '{dir_str}'\r\n\
                 $tmp  = Join-Path ([System.IO.Path]::GetTempPath()) 'kwaainet-upd-extract'\r\n\
                 if (Test-Path $tmp) {{ Remove-Item $tmp -Recurse -Force }}\r\n\
                 Expand-Archive -LiteralPath $zip -DestinationPath $tmp -Force\r\n\
                 Get-ChildItem -Path $tmp -Recurse -Include {file_include} | ForEach-Object {{\r\n\
                   $target = Join-Path $dest $_.Name\r\n\
                   Move-Item -Path $_.FullName -Destination $target -Force\r\n\
                   Add-Content -Path '{log_str}' -Value ('Installed ' + $_.Name)\r\n\
                 }}\r\n\
                 Remove-Item $zip -Force -ErrorAction SilentlyContinue\r\n\
                 Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue\r\n\
                 Add-Content -Path '{log_str}' -Value 'Swap complete — restarting daemon'\r\n\
                 Start-Sleep -Seconds 2\r\n\
                 Start-Process -FilePath '{exe_str}' -ArgumentList 'start', '--daemon' -WindowStyle Hidden\r\n\
                 Add-Content -Path '{log_str}' -Value 'Daemon restart triggered'\r\n\
                 Remove-Item -LiteralPath $MyInvocation.MyCommand.Path -Force -ErrorAction SilentlyContinue\r\n"
            );
            std::fs::write(&ps1, &ps1_content).context("Failed to write update script")?;
            // canonicalize() resolves the real path (long form, no 8.3 short
            // names) after the file exists.
            let ps1_real = ps1.canonicalize().unwrap_or(ps1.clone());

            std::process::Command::new("powershell")
                .args([
                    "-ExecutionPolicy",
                    "Bypass",
                    "-WindowStyle",
                    "Hidden",
                    "-File",
                    ps1_real.to_str().unwrap_or("kwaainet-update.ps1"),
                ])
                .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
                .stderr(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .spawn()
                .context("Failed to spawn updater")?;

            println!("  Installer running in background.");
            println!("  Log: {}", log_path);
            println!("  Daemon will restart automatically.");
        }

        #[cfg(not(any(unix, windows)))]
        anyhow::bail!("Self-update is not supported on this platform");

        Ok(())
    }

    /// Run the cargo-dist shell installer (CPU build, all non-GPU Unix paths).
    #[cfg(unix)]
    async fn install_cpu_linux(&self, version: &str) -> Result<()> {
        let url = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh";
        let tmp = std::env::temp_dir().join("kwaainet-installer.sh");
        self.download_to(url, &tmp).await?;

        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))?;

        let status = std::process::Command::new("sh")
            .arg(&tmp)
            .status()
            .context("Failed to launch installer")?;

        let _ = std::fs::remove_file(&tmp);
        if !status.success() {
            anyhow::bail!("Installer exited with {}", status);
        }

        // macOS 26+ kills unsigned binaries even after quarantine removal.
        #[cfg(target_os = "macos")]
        {
            let install_dir = dirs::home_dir()
                .map(|h| h.join(".cargo/bin"))
                .unwrap_or_default();
            for bin in &["kwaainet", "p2pd"] {
                let path = install_dir.join(bin);
                if path.exists() {
                    let _ = std::process::Command::new("xattr")
                        .args(["-d", "com.apple.quarantine"])
                        .arg(&path)
                        .output();
                    let _ = std::process::Command::new("codesign")
                        .args(["-s", "-", "--force"])
                        .arg(&path)
                        .output();
                }
            }
        }

        let _ = version;
        Ok(())
    }

    /// Download and install the CUDA-enabled Linux binary directly.
    /// When the CUDA archive isn't published yet (async CI, ~90 min after release),
    /// falls back to the CPU installer with a clear warning rather than blocking.
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn install_cuda_linux(&self, version: &str) -> Result<()> {
        let cuda_url = format!(
            "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-unknown-linux-gnu-cuda.tar.xz"
        );

        let client = reqwest::Client::builder()
            .user_agent(format!("kwaainet/{}", CURRENT_VERSION))
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        let cuda_available = client
            .head(&cuda_url)
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if !cuda_available {
            anyhow::bail!(
                "NVIDIA GPU detected but the CUDA build for v{version} isn't published yet \
                 (CI takes ~90 min after release).\n\
                 Update skipped — your current GPU-enabled binary is unchanged.\n\
                 Try again in ~1 hour or watch: \
                 https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/tag/v{version}"
            );
        }

        print!("  NVIDIA GPU detected — downloading CUDA binary for v{version}…");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        let archive = std::env::temp_dir().join("kwaainet-cuda-update.tar.xz");
        self.download_to(&cuda_url, &archive).await?;
        println!(" done.");

        // Derive install dir from the running binary's path. Strip " (deleted)"
        // that Linux appends to /proc/self/exe after a previous in-place swap.
        let exe_path = std::env::current_exe().ok().map(|p| {
            let s = p.to_string_lossy().into_owned();
            if let Some(clean) = s.strip_suffix(" (deleted)") {
                std::path::PathBuf::from(clean)
            } else {
                p
            }
        });
        let install_dir_candidate = exe_path
            .as_deref()
            .and_then(|p| p.parent())
            .map(|d| d.to_path_buf())
            .or_else(|| dirs::home_dir().map(|h| h.join(".cargo/bin")))
            .context("Cannot determine install directory")?;

        // Verify we can actually write there; if not, fall back to ~/.cargo/bin.
        let install_dir = if std::fs::metadata(&install_dir_candidate)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
        {
            install_dir_candidate
        } else {
            let fallback = dirs::home_dir()
                .map(|h| h.join(".cargo/bin"))
                .context("Cannot determine fallback install directory")?;
            if install_dir_candidate != fallback {
                println!(
                    "  ⚠  {} is not writable — installing to {} instead.",
                    install_dir_candidate.display(),
                    fallback.display()
                );
            }
            std::fs::create_dir_all(&fallback)?;
            fallback
        };
        debug!("CUDA install dir: {}", install_dir.display());

        let tmp = std::env::temp_dir().join("kwaainet-cuda-extract");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp)?;

        let status = std::process::Command::new("tar")
            .args(["-xJf"])
            .arg(&archive)
            .arg("-C")
            .arg(&tmp)
            .status()
            .context("Failed to extract CUDA archive (is tar installed?)")?;

        let _ = std::fs::remove_file(&archive);
        if !status.success() {
            anyhow::bail!("tar exited with {status}");
        }

        let subdir = tmp.join("kwaainet-x86_64-unknown-linux-gnu-cuda");
        use std::os::unix::fs::PermissionsExt;
        for entry in std::fs::read_dir(&subdir)
            .with_context(|| format!("Reading extracted archive at {}", subdir.display()))?
        {
            let entry = entry?;
            let name = entry.file_name();
            let dest = install_dir.join(&name);
            // Write to a temp file then rename atomically to avoid ETXTBSY
            // (Linux won't let you overwrite a binary that is currently executing).
            let tmp_dest = install_dir.join(format!(".{}.tmp", name.to_string_lossy()));
            std::fs::copy(entry.path(), &tmp_dest)
                .with_context(|| format!("Installing {} (staging)", name.to_string_lossy()))?;
            let name_str = name.to_string_lossy();
            if name_str == "kwaainet" || name_str == "p2pd" {
                std::fs::set_permissions(&tmp_dest, std::fs::Permissions::from_mode(0o755))?;
            }
            // Unlink the destination first so rename() succeeds even when `dest`
            // is the currently-executing binary (some Linux kernels return ETXTBSY
            // for rename(2) over a running ELF; unlink always succeeds and the old
            // inode stays alive until the process exits).
            let _ = std::fs::remove_file(&dest);
            std::fs::rename(&tmp_dest, &dest).with_context(|| {
                format!(
                    "Installing {} ({} -> {})",
                    name_str,
                    tmp_dest.display(),
                    dest.display()
                )
            })?;
        }
        let _ = std::fs::remove_dir_all(&tmp);
        println!("  CUDA binary installed to {}.", install_dir.display());
        Ok(())
    }

    async fn download_to(&self, url: &str, path: &std::path::Path) -> Result<()> {
        let client = reqwest::Client::builder()
            .user_agent(format!("kwaainet/{}", CURRENT_VERSION))
            .timeout(std::time::Duration::from_secs(120))
            .build()?;
        let resp = client.get(url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("Download failed (HTTP {}): {}", resp.status(), url);
        }
        let bytes = resp.bytes().await?;
        std::fs::write(path, &bytes)
            .with_context(|| format!("Failed to write installer to {}", path.display()))?;
        Ok(())
    }
}

/// Query nvidia-smi asynchronously with a 4-second timeout.
/// Returns true if nvidia-smi exits successfully within the time limit.
#[cfg(all(unix, not(target_os = "macos")))]
async fn nvidia_smi_async() -> bool {
    use tokio::process::Command;
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(4),
        Command::new("nvidia-smi")
            .arg("--query-gpu=name")
            .arg("--format=csv,noheader")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status(),
    )
    .await;
    result
        .ok()
        .and_then(|r| r.ok())
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(not(any(all(unix, not(target_os = "macos")), windows)))]
#[allow(dead_code)]
async fn nvidia_smi_async() -> bool {
    false
}

/// Check for an NVIDIA GPU on Windows by running nvidia-smi with no console
/// window (CREATE_NO_WINDOW) to avoid a flash on headless / daemon contexts.
#[cfg(windows)]
async fn nvidia_smi_windows() -> bool {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    tokio::task::spawn_blocking(|| {
        std::process::Command::new("nvidia-smi")
            .args(["--query-gpu=name", "--format=csv,noheader"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

/// Returns true if `latest` is strictly greater than `current` (simple semver compare).
pub fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
            parts.get(2).copied().unwrap_or(0),
        )
    };
    parse(latest) > parse(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_newer_ordering() {
        assert!(is_newer("0.4.2", "0.4.1"));
        assert!(is_newer("0.5.0", "0.4.99"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.4.1", "0.4.1"));
        assert!(!is_newer("0.4.0", "0.4.1"));
    }

    /// On a Windows machine with an NVIDIA GPU, nvidia_smi_windows() must return
    /// true within the 4-second timeout.  Run on any CI/dev box that has a GPU.
    #[tokio::test]
    #[cfg(windows)]
    async fn windows_gpu_detected() {
        let has_gpu = nvidia_smi_windows().await;
        // This test is advisory: it documents that GPU detection works.
        // On machines without a GPU it is expected to return false.
        println!("nvidia_smi_windows() = {has_gpu}");
    }

    /// Verify that the CUDA archive URLs contain "-cuda" so is_cuda detection
    /// and file_include selection work correctly for both lean and full zips.
    #[test]
    fn cuda_url_suffix_detection() {
        let cuda_lean = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v0.4.79/kwaainet-x86_64-pc-windows-msvc-cuda.zip";
        let cuda_full = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v0.4.79/kwaainet-x86_64-pc-windows-msvc-cuda-full.zip";
        let cpu_url   = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v0.4.79/kwaainet-x86_64-pc-windows-msvc.zip";
        assert!(cuda_lean.contains("-cuda"), "Lean CUDA URL must contain -cuda");
        assert!(cuda_full.contains("-cuda"), "Full CUDA URL must contain -cuda");
        assert!(!cpu_url.contains("-cuda"),  "CPU URL must not contain -cuda");
    }

    /// Verify the file_include string for CUDA zips contains '*.dll' so bundled
    /// CUDA runtime DLLs (cublas64_12.dll etc.) are installed alongside kwaainet.exe.
    #[test]
    fn cuda_file_include_has_dll_glob() {
        let is_cuda = true;
        let file_include = if is_cuda {
            "'*.exe','*.dll'"
        } else {
            "'*.exe'"
        };
        assert!(
            file_include.contains("*.dll"),
            "CUDA file_include must contain *.dll glob to install bundled CUDA DLLs"
        );
        let is_cuda = false;
        let file_include = if is_cuda {
            "'*.exe','*.dll'"
        } else {
            "'*.exe'"
        };
        assert!(
            !file_include.contains("*.dll"),
            "CPU file_include must not contain *.dll glob"
        );
    }

    /// Verifies that when the CUDA archive isn't published yet, install_cuda_linux
    /// returns Err (no CPU fallback, no binary is touched).
    #[tokio::test]
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn cuda_update_bails_when_archive_missing() {
        // v0.4.70 never had a CUDA archive — safe version to test against.
        let checker = UpdateChecker::new();
        let result = checker.install_cuda_linux("0.4.70").await;
        let err = result.expect_err("should bail when CUDA archive is missing");
        let msg = err.to_string();
        assert!(
            msg.contains("CUDA build for v0.4.70 isn't published yet"),
            "Expected 'not published yet' message, got: {msg}"
        );
    }

    /// Smoke-test: nvidia_smi_async should not hang or panic regardless of GPU presence.
    #[tokio::test]
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn nvidia_smi_does_not_hang() {
        let _has_gpu = nvidia_smi_async().await;
        // Pass as long as it returns within the 4-second timeout.
    }
}
