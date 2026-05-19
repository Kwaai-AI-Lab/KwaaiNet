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
            // On Linux, check for an NVIDIA GPU before deciding what to install.
            // If one is present we must use the CUDA-enabled binary — silently
            // installing the CPU build would break GPU inference.
            #[cfg(not(target_os = "macos"))]
            {
                if nvidia_smi_async().await {
                    return self.install_cuda_linux(version).await;
                }
            }

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
            // Strip the quarantine xattr then apply an ad-hoc signature so
            // Gatekeeper accepts the new binary before we try to spawn it.
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

            let _ = version; // used on Windows only
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let log = std::env::temp_dir().join("kwaainet-update.log");
            let log_path = log.to_string_lossy().into_owned();
            let bat = std::env::temp_dir().join("kwaainet-update.bat");

            // Determine the install directory (same dir as the running kwaainet.exe)
            let install_dir = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                .unwrap_or_else(|| {
                    dirs::home_dir()
                        .map(|h| h.join(".cargo").join("bin"))
                        .unwrap_or_default()
                });

            // Check if CUDA is already present on this system.
            // We check five signals in order — the first hit short-circuits:
            //   1. %CUDA_PATH% env var (set by the CUDA toolkit installer)
            //   2. %CUDA_HOME% env var (common alternative)
            //   3. Standard install dir exists (reliable even when env vars aren't
            //      propagated, e.g. when kwaainet is run from Git Bash)
            //   4. nvidia-smi.exe on PATH (capped at 4 s — NVML init is slow on Windows)
            //   5. cublas*.dll in the kwaainet install dir (bundled by previous update)
            print!("  Detecting GPU…");
            let _ = std::io::Write::flush(&mut std::io::stdout());
            let cuda_installed = if std::env::var_os("CUDA_PATH").is_some() {
                println!(" CUDA_PATH set");
                true
            } else if std::env::var_os("CUDA_HOME").is_some() {
                println!(" CUDA_HOME set");
                true
            } else if std::path::Path::new(r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA")
                .exists()
            {
                println!(" CUDA toolkit dir found");
                true
            } else if nvidia_smi_async().await {
                println!(" nvidia-smi found");
                true
            } else if std::fs::read_dir(&install_dir)
                .ok()
                .map(|dir| {
                    dir.filter_map(|e| e.ok()).any(|e| {
                        let name = e.file_name().to_string_lossy().to_lowercase();
                        name.starts_with("cublas") && name.ends_with(".dll")
                    })
                })
                .unwrap_or(false)
            {
                println!(" cublas DLLs found");
                true
            } else {
                println!(" no GPU/CUDA detected");
                false
            };

            // For the full (non-CUDA) path we need the PS1 installer on disk before
            // writing the batch file; download it now while we're still async.
            let ps1_tmp: Option<PathBuf> = if !cuda_installed {
                let url = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1";
                let tmp = std::env::temp_dir().join("kwaainet-installer.ps1");
                self.download_to(url, &tmp).await?;
                Some(tmp)
            } else {
                None
            };

            // The kill-and-install header is the same regardless of CUDA vs CPU path:
            // wait for THIS process to exit, then force-kill every remaining
            // kwaainet.exe (daemon, storage serve, orphaned instances) so the
            // installer can overwrite the binary without a sharing violation.
            let kill_header = "\
                @echo off\r\n\
                ping -n 3 127.0.0.1 > nul\r\n\
                taskkill /IM kwaainet.exe /F /T > nul 2>&1\r\n\
                ping -n 2 127.0.0.1 > nul\r\n";

            // For the CUDA fast path, download the exe-only zip NOW (in-process,
            // foreground) so the user sees progress. The batch script only needs
            // to do the exe swap after kwaainet exits — that's near-instant.
            let cuda_zip_tmp: Option<PathBuf> = if cuda_installed {
                let archive_url = format!(
                    "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-pc-windows-msvc-cuda-exes.zip"
                );
                let zip_path = std::env::temp_dir().join("kwaainet-cuda-exes-update.zip");
                print!("  Downloading CUDA binary update (~30 MB)…");
                let _ = std::io::Write::flush(&mut std::io::stdout());
                self.download_to(&archive_url, &zip_path).await?;
                println!(" done.");
                Some(zip_path)
            } else {
                None
            };

            let kwaainet_exe = install_dir.join("kwaainet.exe");
            let respawn = format!(
                "ping -n 3 127.0.0.1 > nul\r\n\
                 start \"\" \"{exe}\" start --daemon\r\n",
                exe = kwaainet_exe.to_string_lossy()
            );

            let bat_content = if cuda_installed {
                // Zip is already on disk — write a .ps1 for the swap so paths
                // are never processed by cmd.exe's character expansion (%, ^, !).
                let zip_path = cuda_zip_tmp.as_ref().unwrap();
                let ps1 = std::env::temp_dir().join("kwaainet-cuda-swap.ps1");
                let ps1_content = format!(
                    "$zip = @'\r\n{zip}\r\n'@\r\n\
                     $dest_dir = @'\r\n{dir}\r\n'@\r\n\
                     $tmp = [System.IO.Path]::GetTempPath() + 'kwaainet-cuda-exes-extract'\r\n\
                     Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue\r\n\
                     Expand-Archive -Path $zip -DestinationPath $tmp -Force\r\n\
                     Get-ChildItem $tmp -Recurse -Filter '*.exe' | ForEach-Object {{\r\n\
                       $dest = Join-Path $dest_dir $_.Name\r\n\
                       Move-Item $_.FullName $dest -Force\r\n\
                       Write-Host ('Installed ' + $_.Name)\r\n\
                     }}\r\n\
                     Remove-Item $zip -Force -ErrorAction SilentlyContinue\r\n\
                     Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue\r\n\
                     Write-Host 'Update complete. CUDA DLLs preserved.'\r\n",
                    zip = zip_path.to_string_lossy(),
                    dir = install_dir.to_string_lossy(),
                );
                std::fs::write(&ps1, &ps1_content).context("Failed to write CUDA swap script")?;
                let ps1_str = ps1.to_string_lossy().into_owned();
                format!(
                    "{kill_header}\
                     powershell -ExecutionPolicy Bypass -File \"{ps1_str}\" >> \"{log_path}\" 2>&1\r\n\
                     del /f \"{ps1_str}\"\r\n\
                     {respawn}\
                     del /f \"%~f0\"\r\n"
                )
            } else {
                // Full path: use the cargo-dist PS1 installer (handles first-time
                // CUDA detection and DLL installation).
                let ps_path = ps1_tmp
                    .as_ref()
                    .unwrap()
                    .to_string_lossy()
                    .replace('\'', "''");
                format!(
                    "{kill_header}\
                     powershell -ExecutionPolicy Bypass -File \"{ps_path}\" >> \"{log_path}\" 2>&1\r\n\
                     del /f \"{ps_path}\"\r\n\
                     {respawn}\
                     del /f \"%~f0\"\r\n"
                )
            };

            if cuda_installed {
                let reason = if std::env::var_os("CUDA_PATH").is_some() {
                    "CUDA_PATH env var set"
                } else if std::env::var_os("CUDA_HOME").is_some() {
                    "CUDA_HOME env var set"
                } else if std::path::Path::new(
                    r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA",
                )
                .exists()
                {
                    "CUDA toolkit dir found"
                } else if which_nvidia_smi() {
                    "nvidia-smi found on PATH"
                } else {
                    "cublas DLLs in install dir"
                };
                println!("  CUDA detected ({reason}) — downloading CUDA binary (~30 MB, DLLs preserved).");
            }

            std::fs::write(&bat, &bat_content).context("Failed to write updater batch script")?;

            // Launch the batch detached. kwaainet.exe exits after this fn returns,
            // releasing its file lock so the batch can overwrite the binary.
            std::process::Command::new("cmd")
                .args(["/c", bat.to_str().unwrap_or("kwaainet-update.bat")])
                .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
                .spawn()
                .context("Failed to spawn updater batch")?;

            if cuda_installed {
                println!("  Swapping binaries in background (will complete in seconds).");
            } else {
                println!("  Installer running in background.");
            }
            println!("  Log: {}", log_path);
            println!("  Run  kwaainet start --daemon  once it finishes.");
        }

        #[cfg(not(any(unix, windows)))]
        anyhow::bail!("Self-update is not supported on this platform");

        Ok(())
    }

    /// Download the CUDA-enabled Linux binary for `version` and install it in-place.
    ///
    /// Bails with a clear error if the CUDA archive isn't published yet for this
    /// release rather than silently falling back to the CPU build.
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn install_cuda_linux(&self, version: &str) -> Result<()> {
        let cuda_url = format!(
            "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v{version}/kwaainet-x86_64-unknown-linux-gnu-cuda.tar.xz"
        );

        // HEAD check — don't download if the archive isn't there yet.
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
                "NVIDIA GPU detected but the CUDA build for v{version} isn't published yet.\n\
                 Update skipped — your current GPU-enabled binary is unchanged.\n\
                 Try again in a few minutes or watch: \
                 https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/tag/v{version}"
            );
        }

        print!("  NVIDIA GPU detected — downloading CUDA binary for v{version}…");
        let _ = std::io::Write::flush(&mut std::io::stdout());

        let archive = std::env::temp_dir().join("kwaainet-cuda-update.tar.xz");
        self.download_to(&cuda_url, &archive).await?;
        println!(" done.");

        let install_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .or_else(|| dirs::home_dir().map(|h| h.join(".cargo/bin")))
            .context("Cannot determine install directory")?;

        // Extract archive into a temp directory, then copy every file into
        // install_dir (binary + bundled CUDA .so runtime libs).
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

        // Archive layout: kwaainet-x86_64-unknown-linux-gnu-cuda/<files>
        let subdir = tmp.join("kwaainet-x86_64-unknown-linux-gnu-cuda");
        use std::os::unix::fs::PermissionsExt;
        for entry in std::fs::read_dir(&subdir)
            .with_context(|| format!("Reading extracted archive at {}", subdir.display()))?
        {
            let entry = entry?;
            let name = entry.file_name();
            let dest = install_dir.join(&name);
            std::fs::copy(entry.path(), &dest)
                .with_context(|| format!("Installing {}", name.to_string_lossy()))?;
            let name_str = name.to_string_lossy();
            if name_str == "kwaainet" || name_str == "p2pd" {
                std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
            }
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
        let bytes = client.get(url).send().await?.bytes().await?;
        std::fs::write(path, &bytes)
            .with_context(|| format!("Failed to write installer to {}", path.display()))?;
        Ok(())
    }
}

/// Query nvidia-smi asynchronously with a 4-second timeout.
/// Returns true if nvidia-smi exits successfully within the time limit.
#[cfg(any(windows, all(unix, not(target_os = "macos"))))]
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

#[cfg(not(any(windows, all(unix, not(target_os = "macos")))))]
#[allow(dead_code)]
async fn nvidia_smi_async() -> bool {
    false
}

/// Returns true if `nvidia-smi.exe` is reachable on the current PATH.
/// Synchronous fallback used only for the post-detection reason string.
#[cfg(windows)]
fn which_nvidia_smi() -> bool {
    std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=name")
        .arg("--format=csv,noheader")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(not(windows))]
#[allow(dead_code)]
fn which_nvidia_smi() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── nvidia_smi_async ────────────────────────────────────────────────────
    // Run with: cargo test -p kwaai-cli -- updater --nocapture
    #[tokio::test]
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn nvidia_smi_detects_gpu_on_linux() {
        // This test only passes on machines with an NVIDIA GPU.
        // On CI (no GPU) it will return false, which is also correct.
        let has_gpu = nvidia_smi_async().await;
        println!("nvidia_smi_async() = {has_gpu}");
        // Don't assert — just verify it doesn't hang or panic.
    }

    // ── install_cuda_linux: missing archive → clear error, no CPU fallback ──
    #[tokio::test]
    #[cfg(all(unix, not(target_os = "macos")))]
    async fn cuda_update_bails_when_archive_missing() {
        // v0.4.70 never had a CUDA archive published — safe version to test against.
        let checker = UpdateChecker::new();
        let result = checker.install_cuda_linux("0.4.70").await;
        let err = result.expect_err("should bail when CUDA archive is missing");
        let msg = err.to_string();
        println!("Error message: {msg}");
        assert!(
            msg.contains("CUDA build for v0.4.70 isn't published yet"),
            "Expected clear 'not published' message, got: {msg}"
        );
        // Crucially: no CPU binary was installed (install_cuda_linux returns Err,
        // so main.rs restarts the daemon with the existing binary untouched).
    }

    // ── version ordering ────────────────────────────────────────────────────
    #[test]
    fn is_newer_ordering() {
        assert!(is_newer("0.4.2", "0.4.1"));
        assert!(is_newer("0.5.0", "0.4.99"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.4.1", "0.4.1"));
        assert!(!is_newer("0.4.0", "0.4.1"));
    }
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
