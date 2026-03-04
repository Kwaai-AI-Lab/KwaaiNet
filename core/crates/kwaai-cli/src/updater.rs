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
            Some(entry.update_info)
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

    /// Download and run the cargo-dist installer for this platform.
    /// On Unix runs the shell installer via `sh`; on Windows runs the PowerShell installer.
    pub async fn install_update(&self) -> Result<()> {
        #[cfg(unix)]
        {
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
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const DETACHED_PROCESS: u32 = 0x00000008;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            let url = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1";
            let tmp = std::env::temp_dir().join("kwaainet-installer.ps1");
            self.download_to(url, &tmp).await?;

            // Windows locks running executables, so we can't overwrite kwaainet.exe
            // while this process is alive.  Write a batch script that sleeps until
            // we exit, then runs the PowerShell installer, then deletes itself.
            let bat = std::env::temp_dir().join("kwaainet-update.bat");
            let ps_path = tmp.to_string_lossy().replace('\'', "''");
            let bat_content = format!(
                "@echo off\r\n\
                 ping -n 3 127.0.0.1 > nul\r\n\
                 powershell -ExecutionPolicy Bypass -File \"{ps_path}\"\r\n\
                 del /f \"{ps_path}\"\r\n\
                 del /f \"%~f0\"\r\n"
            );
            std::fs::write(&bat, &bat_content).context("Failed to write updater batch script")?;

            // Launch the batch detached (no window) and return immediately.
            // kwaainet.exe will exit after this function returns, freeing the lock.
            std::process::Command::new("cmd")
                .args(["/c", bat.to_str().unwrap_or("kwaainet-update.bat")])
                .creation_flags(DETACHED_PROCESS | CREATE_NO_WINDOW)
                .spawn()
                .context("Failed to spawn updater batch")?;

            // Print notice — the actual install happens a few seconds after we exit.
            println!("  Update will complete in a moment (installer running in background).");
            println!("  Run  kwaainet start --daemon  once it finishes.");
        }

        #[cfg(not(any(unix, windows)))]
        anyhow::bail!("Self-update is not supported on this platform");

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
