//! Self-update checker via GitHub releases API

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

const RELEASES_URL: &str =
    "https://api.github.com/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest";

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
        Self { current_version: CURRENT_VERSION.to_string() }
    }

    /// Check for a newer release. Returns `Some(UpdateInfo)` if one exists.
    pub async fn check(self: &Self, force: bool) -> Result<Option<UpdateInfo>> {
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
        let entry = CacheEntry { checked_at: now, update_info: info.clone() };
        std::fs::write(&path, serde_json::to_string(&entry)?)?;
        Ok(())
    }
}

/// Returns true if `latest` is strictly greater than `current` (simple semver compare).
fn is_newer(latest: &str, current: &str) -> bool {
    let parse = |s: &str| -> (u32, u32, u32) {
        let parts: Vec<u32> = s.split('.').filter_map(|p| p.parse().ok()).collect();
        (parts.get(0).copied().unwrap_or(0),
         parts.get(1).copied().unwrap_or(0),
         parts.get(2).copied().unwrap_or(0))
    };
    parse(latest) > parse(current)
}
