//! Daemon process lifecycle management
//!
//! Handles PID files, lock files, status files, start/stop/restart,
//! and process health queries via sysinfo.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use sysinfo::{Pid, System};
use tracing::{debug, info, warn};

use crate::config::{log_dir, run_dir};

// ---------------------------------------------------------------------------
// Status file
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub uptime_secs: Option<u64>,
    pub cpu_percent: Option<f32>,
    pub memory_mb: Option<f64>,
    pub memory_percent: Option<f32>,
    pub connections: Option<u32>,
    pub threads: Option<u32>,
    pub started_at: Option<u64>,
    pub health_monitoring: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// DaemonManager
// ---------------------------------------------------------------------------

pub struct DaemonManager {
    pub pid_file: PathBuf,
    pub lock_file: PathBuf,
    pub status_file: PathBuf,
}

impl DaemonManager {
    pub fn new() -> Self {
        let run = run_dir();
        std::fs::create_dir_all(&run).ok();
        std::fs::create_dir_all(log_dir()).ok();
        Self {
            pid_file: run.join("kwaainet.pid"),
            lock_file: run.join("kwaainet.lock"),
            status_file: run.join("kwaainet.status"),
        }
    }

    // -----------------------------------------------------------------------
    // PID helpers
    // -----------------------------------------------------------------------

    pub fn write_pid(&self, pid: u32) -> Result<()> {
        std::fs::write(&self.pid_file, pid.to_string())
            .with_context(|| format!("writing PID file {}", self.pid_file.display()))
    }

    pub fn read_pid(&self) -> Option<u32> {
        let text = std::fs::read_to_string(&self.pid_file).ok()?;
        text.trim().parse().ok()
    }

    pub fn remove_pid(&self) {
        let _ = std::fs::remove_file(&self.pid_file);
    }

    /// Signal the running daemon to re-read config and re-announce its block
    /// range to DHT.  Called by `shard serve` after updating config.yaml.
    ///
    /// Unix: sends SIGHUP — the daemon's event loop handles it immediately.
    /// Windows: writes a flag file that the re-announce tick polls (≤120 s).
    pub fn signal_reannounce(&self) {
        #[cfg(unix)]
        {
            if let Some(pid) = self.read_pid() {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid as NixPid;
                let _ = kill(NixPid::from_raw(pid as i32), Signal::SIGHUP);
                info!("Sent SIGHUP to daemon PID {} — re-announce will follow", pid);
            } else {
                warn!("signal_reannounce: no daemon PID found");
            }
        }
        #[cfg(not(unix))]
        {
            let flag = self.pid_file.with_file_name("reannounce.flag");
            let _ = std::fs::write(&flag, "1");
        }
    }

    // -----------------------------------------------------------------------
    // Lock helpers (Unix only)
    // -----------------------------------------------------------------------

    #[cfg(unix)]
    pub fn try_acquire_lock(&self) -> Result<bool> {
        use nix::fcntl::{flock, FlockArg};
        use std::os::unix::io::AsRawFd;

        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false) // lock file; content irrelevant, we only use flock
            .write(true)
            .open(&self.lock_file)
            .with_context(|| format!("opening lock file {}", self.lock_file.display()))?;

        match flock(file.as_raw_fd(), FlockArg::LockExclusiveNonblock) {
            Ok(()) => {
                // Keep the file open to hold the lock (leak fd intentionally for the process lifetime)
                std::mem::forget(file);
                Ok(true)
            }
            Err(nix::errno::Errno::EWOULDBLOCK) => Ok(false),
            Err(e) => bail!("flock: {}", e),
        }
    }

    #[cfg(not(unix))]
    pub fn try_acquire_lock(&self) -> Result<bool> {
        // On non-Unix, skip locking
        Ok(true)
    }

    // -----------------------------------------------------------------------
    // Process status
    // -----------------------------------------------------------------------

    pub fn is_running(&self) -> bool {
        match self.read_pid() {
            Some(pid) => {
                let mut sys = System::new();
                sys.refresh_process(Pid::from_u32(pid));
                sys.process(Pid::from_u32(pid)).is_some()
            }
            None => false,
        }
    }

    pub fn get_status(&self) -> NodeStatus {
        let pid = match self.read_pid() {
            Some(p) => p,
            None => return NodeStatus::default(),
        };

        let mut sys = System::new_all();
        sys.refresh_all();

        let sysinfo_pid = Pid::from_u32(pid);
        let proc = match sys.process(sysinfo_pid) {
            Some(p) => p,
            None => {
                self.remove_pid();
                return NodeStatus::default();
            }
        };

        let started_at = proc.start_time();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let uptime_secs = now.saturating_sub(started_at);

        NodeStatus {
            running: true,
            pid: Some(pid),
            uptime_secs: Some(uptime_secs),
            cpu_percent: Some(proc.cpu_usage()),
            memory_mb: Some(proc.memory() as f64 / 1_048_576.0),
            memory_percent: None,
            connections: None,
            threads: None,
            started_at: Some(started_at),
            health_monitoring: self.read_status().and_then(|s| s.health_monitoring),
        }
    }

    // -----------------------------------------------------------------------
    // Status file (JSON)
    // -----------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn write_status(&self, status: &NodeStatus) -> Result<()> {
        let text = serde_json::to_string_pretty(status).context("serializing status")?;
        std::fs::write(&self.status_file, text)
            .with_context(|| format!("writing status file {}", self.status_file.display()))
    }

    pub fn read_status(&self) -> Option<NodeStatus> {
        let text = std::fs::read_to_string(&self.status_file).ok()?;
        serde_json::from_str(&text).ok()
    }

    // -----------------------------------------------------------------------
    // Stop
    // -----------------------------------------------------------------------

    pub fn stop_process(&self) -> Result<()> {
        let pid = self.read_pid().context("No daemon is running")?;
        info!("Sending SIGTERM to PID {}", pid);

        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid as NixPid;

            kill(NixPid::from_raw(pid as i32), Signal::SIGTERM)
                .with_context(|| format!("SIGTERM to PID {}", pid))?;

            // Wait up to 10 seconds then SIGKILL
            for _ in 0..20 {
                std::thread::sleep(Duration::from_millis(500));
                let mut sys = System::new();
                sys.refresh_process(Pid::from_u32(pid));
                if sys.process(Pid::from_u32(pid)).is_none() {
                    info!("Process {} exited cleanly", pid);
                    self.remove_pid();
                    return Ok(());
                }
            }

            warn!("Process {} did not exit, sending SIGKILL", pid);
            let _ = kill(NixPid::from_raw(pid as i32), Signal::SIGKILL);
        }

        #[cfg(not(unix))]
        {
            // Windows: use taskkill
            let _ = std::process::Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }

        self.remove_pid();
        // Kill any orphaned p2pd processes so they don't hold the port for the next start.
        kill_orphaned_p2pd();
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Daemonize
    // -----------------------------------------------------------------------

    /// Re-launch the current binary with `run-node` as a detached child.
    /// Returns immediately in the parent; the child runs the node.
    pub fn spawn_daemon_child(extra_args: &[String]) -> Result<u32> {
        let exe = std::env::current_exe().context("finding own executable")?;
        let log = log_dir().join("kwaainet.log");
        std::fs::create_dir_all(log.parent().unwrap()).ok();
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
            .with_context(|| format!("opening log file {}", log.display()))?;

        let mut cmd = std::process::Command::new(&exe);
        cmd.arg("run-node");
        for a in extra_args {
            cmd.arg(a);
        }

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.stdout(log_file.try_clone()?);
            cmd.stderr(log_file);
            // Detach from terminal session
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
        }

        #[cfg(not(unix))]
        {
            use std::os::windows::process::CommandExt;
            cmd.stdout(log_file.try_clone()?);
            cmd.stderr(log_file);
            cmd.creation_flags(0x00000008); // DETACHED_PROCESS
        }

        let child = cmd.spawn().context("spawning daemon child")?;
        let pid = child.id();
        debug!("Spawned daemon child PID {}", pid);
        // Don't wait – let it run
        std::mem::forget(child);
        Ok(pid)
    }
}

// ---------------------------------------------------------------------------
// Orphan cleanup
// ---------------------------------------------------------------------------

/// Kill any p2pd processes that may have been left behind when the daemon
/// process was terminated by SIGTERM (which bypasses Rust destructors, so
/// the kwaai-p2p-daemon Drop impl never fires to clean them up).
/// Without this, a new daemon start fails because p2pd can't bind the port.
fn kill_orphaned_p2pd() {
    use sysinfo::ProcessRefreshKind;

    let mut sys = System::new();
    sys.refresh_processes_specifics(ProcessRefreshKind::new());

    let mut found = false;
    for (pid, process) in sys.processes() {
        let name = process.name();
        if name == "p2pd" || name == "p2pd.exe" {
            info!("Killing orphaned p2pd process (PID {})", pid);
            found = true;
            #[cfg(unix)]
            {
                use nix::sys::signal::{kill, Signal};
                use nix::unistd::Pid as NixPid;
                // SIGKILL — no grace period, port released immediately.
                let _ = kill(NixPid::from_raw(pid.as_u32() as i32), Signal::SIGKILL);
            }
            #[cfg(not(unix))]
            {
                let _ = std::process::Command::new("taskkill")
                    .args(["/PID", &pid.as_u32().to_string(), "/F"])
                    .output();
            }
        }
    }

    // Give the OS a moment to release the port before the next p2pd starts.
    if found {
        std::thread::sleep(Duration::from_millis(500));
    }
}

// ---------------------------------------------------------------------------
// ShardManager — manages the background `shard serve` child process
// ---------------------------------------------------------------------------

pub struct ShardManager {
    pub pid_file: PathBuf,
}

impl ShardManager {
    pub fn new() -> Self {
        let run = run_dir();
        std::fs::create_dir_all(&run).ok();
        Self {
            pid_file: run.join("shard.pid"),
        }
    }

    pub fn write_pid(&self, pid: u32) {
        let _ = std::fs::write(&self.pid_file, pid.to_string());
    }

    pub fn read_pid(&self) -> Option<u32> {
        std::fs::read_to_string(&self.pid_file)
            .ok()
            .and_then(|t| t.trim().parse().ok())
    }

    pub fn remove_pid(&self) {
        let _ = std::fs::remove_file(&self.pid_file);
    }

    pub fn is_running(&self) -> bool {
        match self.read_pid() {
            Some(pid) => {
                let mut sys = System::new();
                sys.refresh_process(Pid::from_u32(pid));
                sys.process(Pid::from_u32(pid)).is_some()
            }
            None => false,
        }
    }

    /// Stop the shard serve child, if running.
    pub fn stop_process(&self) {
        let Some(pid) = self.read_pid() else { return };
        info!("Stopping shard server PID {}", pid);

        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid as NixPid;
            let _ = kill(NixPid::from_raw(pid as i32), Signal::SIGTERM);
            for _ in 0..10 {
                std::thread::sleep(Duration::from_millis(500));
                let mut sys = System::new();
                sys.refresh_process(Pid::from_u32(pid));
                if sys.process(Pid::from_u32(pid)).is_none() {
                    break;
                }
            }
        }
        #[cfg(not(unix))]
        {
            let _ = std::process::Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output();
        }

        self.remove_pid();
    }

    /// Spawn `kwaainet shard serve --auto --auto-rebalance` as a detached
    /// background process, appending output to `shard.log`.
    pub fn spawn_shard_child() -> Result<u32> {
        let exe = std::env::current_exe().context("finding own executable")?;
        let log = log_dir().join("shard.log");
        std::fs::create_dir_all(log.parent().unwrap()).ok();
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
            .with_context(|| format!("opening shard log {}", log.display()))?;

        let mut cmd = std::process::Command::new(&exe);
        cmd.args(["shard", "serve", "--auto-rebalance"]);

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            cmd.stdout(log_file.try_clone()?);
            cmd.stderr(log_file);
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
        }
        #[cfg(not(unix))]
        {
            use std::os::windows::process::CommandExt;
            cmd.stdout(log_file.try_clone()?);
            cmd.stderr(log_file);
            cmd.creation_flags(0x00000008); // DETACHED_PROCESS
        }

        let child = cmd.spawn().context("spawning shard child")?;
        let pid = child.id();
        debug!("Spawned shard child PID {}", pid);
        std::mem::forget(child);
        Ok(pid)
    }
}

#[cfg(unix)]
extern "C" {
    #[allow(dead_code)]
    fn libc_setsid() -> i32;
}

// On Unix we need libc for setsid
#[cfg(unix)]
mod libc {
    extern "C" {
        pub fn setsid() -> i32;
    }
}
