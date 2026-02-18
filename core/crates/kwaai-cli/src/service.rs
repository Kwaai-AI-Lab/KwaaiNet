//! Auto-start service management
//!
//! macOS: launchd plist at ~/Library/LaunchAgents/ai.kwaai.kwaainet.plist
//! Linux: systemd user unit at ~/.config/systemd/user/kwaainet.service

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::info;

pub trait ServiceManager {
    fn install(&self) -> Result<()>;
    fn uninstall(&self) -> Result<()>;
    fn status(&self) -> ServiceStatus;
    fn restart(&self) -> Result<()>;
}

#[derive(Debug)]
pub struct ServiceStatus {
    pub installed: bool,
    pub loaded: bool,
    pub running: bool,
    pub pid: Option<u32>,
}

pub fn get_service_manager() -> Box<dyn ServiceManager> {
    #[cfg(target_os = "macos")]
    return Box::new(LaunchdManager);

    #[cfg(target_os = "linux")]
    return Box::new(SystemdManager);

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    return Box::new(NoopManager);
}

// ---------------------------------------------------------------------------
// macOS – launchd
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
struct LaunchdManager;

#[cfg(target_os = "macos")]
impl LaunchdManager {
    fn plist_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home)
            .join("Library/LaunchAgents/ai.kwaai.kwaainet.plist")
    }

    fn plist_content() -> Result<String> {
        let exe = std::env::current_exe().context("finding own executable")?;
        let log_dir = crate::config::log_dir();
        Ok(format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ai.kwaai.kwaainet</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
        <string>run-node</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{log}</string>
    <key>StandardErrorPath</key>
    <string>{log}</string>
</dict>
</plist>"#,
            exe = exe.display(),
            log = log_dir.join("kwaainet.log").display(),
        ))
    }
}

#[cfg(target_os = "macos")]
impl ServiceManager for LaunchdManager {
    fn install(&self) -> Result<()> {
        let path = Self::plist_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, Self::plist_content()?)?;
        std::process::Command::new("launchctl")
            .args(["load", &path.to_string_lossy()])
            .status()
            .context("launchctl load")?;
        info!("Installed launchd service at {}", path.display());
        Ok(())
    }

    fn uninstall(&self) -> Result<()> {
        let path = Self::plist_path();
        if path.exists() {
            std::process::Command::new("launchctl")
                .args(["unload", &path.to_string_lossy()])
                .status()
                .context("launchctl unload")?;
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }

    fn status(&self) -> ServiceStatus {
        let path = Self::plist_path();
        let installed = path.exists();
        if !installed {
            return ServiceStatus { installed: false, loaded: false, running: false, pid: None };
        }
        let out = std::process::Command::new("launchctl")
            .args(["list", "ai.kwaai.kwaainet"])
            .output()
            .ok();
        let running = out.as_ref().map(|o| o.status.success()).unwrap_or(false);
        ServiceStatus { installed, loaded: running, running, pid: None }
    }

    fn restart(&self) -> Result<()> {
        let path = Self::plist_path();
        std::process::Command::new("launchctl")
            .args(["unload", &path.to_string_lossy()])
            .status()?;
        std::process::Command::new("launchctl")
            .args(["load", &path.to_string_lossy()])
            .status()?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Linux – systemd (user)
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
struct SystemdManager;

#[cfg(target_os = "linux")]
impl SystemdManager {
    fn unit_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".config/systemd/user/kwaainet.service")
    }

    fn unit_content() -> Result<String> {
        let exe = std::env::current_exe().context("finding own executable")?;
        let log = crate::config::log_dir().join("kwaainet.log");
        Ok(format!(
            "[Unit]\nDescription=KwaaiNet Node\nAfter=network.target\n\n\
             [Service]\nExecStart={exe} run-node\nRestart=always\nRestartSec=10\n\
             StandardOutput=append:{log}\nStandardError=append:{log}\n\n\
             [Install]\nWantedBy=default.target\n",
            exe = exe.display(),
            log = log.display(),
        ))
    }
}

#[cfg(target_os = "linux")]
impl ServiceManager for SystemdManager {
    fn install(&self) -> Result<()> {
        let path = Self::unit_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, Self::unit_content()?)?;
        std::process::Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .status()?;
        std::process::Command::new("systemctl")
            .args(["--user", "enable", "--now", "kwaainet"])
            .status()?;
        info!("Installed systemd user service");
        Ok(())
    }

    fn uninstall(&self) -> Result<()> {
        std::process::Command::new("systemctl")
            .args(["--user", "disable", "--now", "kwaainet"])
            .status()?;
        let path = Self::unit_path();
        if path.exists() { std::fs::remove_file(&path)?; }
        std::process::Command::new("systemctl")
            .args(["--user", "daemon-reload"])
            .status()?;
        Ok(())
    }

    fn status(&self) -> ServiceStatus {
        let installed = Self::unit_path().exists();
        let out = std::process::Command::new("systemctl")
            .args(["--user", "is-active", "kwaainet"])
            .output()
            .ok();
        let running = out.map(|o| o.status.success()).unwrap_or(false);
        ServiceStatus { installed, loaded: installed, running, pid: None }
    }

    fn restart(&self) -> Result<()> {
        std::process::Command::new("systemctl")
            .args(["--user", "restart", "kwaainet"])
            .status()?;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Fallback no-op
// ---------------------------------------------------------------------------

struct NoopManager;
impl ServiceManager for NoopManager {
    fn install(&self) -> Result<()> { anyhow::bail!("Service management not supported on this platform") }
    fn uninstall(&self) -> Result<()> { anyhow::bail!("Service management not supported on this platform") }
    fn status(&self) -> ServiceStatus { ServiceStatus { installed: false, loaded: false, running: false, pid: None } }
    fn restart(&self) -> Result<()> { anyhow::bail!("Service management not supported on this platform") }
}
