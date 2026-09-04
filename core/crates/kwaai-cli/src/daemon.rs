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
                info!(
                    "Sent SIGHUP to daemon PID {} — re-announce will follow",
                    pid
                );
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
///
/// Scoped to this instance's control socket when `KWAAINET_SOCKET` names one:
/// p2pd carries it in `-listen`, and without the filter a second node's stop
/// SIGKILLs the first node's p2pd. With no override there is only one socket
/// on the machine, so every p2pd is ours and the name alone is enough.
pub fn kill_orphaned_p2pd() {
    use sysinfo::ProcessRefreshKind;

    let mut sys = System::new();
    sys.refresh_processes_specifics(ProcessRefreshKind::new());

    let socket = std::env::var("KWAAINET_SOCKET")
        .ok()
        .filter(|s| !s.is_empty());

    let mut found = false;
    for (pid, process) in sys.processes() {
        let name = process.name();
        if name == "p2pd" || name == "p2pd.exe" {
            if let Some(ref sock) = socket {
                if !process.cmd().iter().any(|a| a.contains(sock.as_str())) {
                    debug!("Leaving p2pd PID {} alone — not on {}", pid, sock);
                    continue;
                }
            }
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

    pub fn ready_file() -> PathBuf {
        run_dir().join("shard.ready")
    }

    /// Sentinel for the *other* way a node serves inference: whole-model, over
    /// `/kwaai/ollama-proxy/1.0.0`.
    ///
    /// Deliberately not `shard.ready`. That one means "a block shard is loaded",
    /// and callers act on the block range it implies; a Mac on the Ollama path
    /// has no block range at all. Two sentinels keep both claims truthful, and
    /// keep `shard_is_ready()` meaning what its callers already assume.
    pub fn whole_model_ready_file() -> PathBuf {
        run_dir().join("whole_model.ready")
    }

    /// Returns true only when the shard process is alive AND the model is fully
    /// loaded (i.e. shard_cmd has written the `shard.ready` sentinel file).
    pub fn shard_is_ready() -> bool {
        Self::ready_file().exists() && Self::new().is_running()
    }

    /// Whether this node is serving whole-model inference.
    ///
    /// No liveness check, unlike [`shard_is_ready`]: the block shard runs as a
    /// separate process that can die behind its pid file, whereas whole-model
    /// serving is registered by the announcing process itself — so reaching
    /// this code is the liveness proof. A sentinel left by a killed process is
    /// rewritten on the next registration.
    pub fn whole_model_is_ready() -> bool {
        Self::whole_model_ready_file().exists()
    }

    /// Record that this node is serving whole-model inference.
    pub fn mark_whole_model_ready() {
        let path = Self::whole_model_ready_file();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Err(e) = std::fs::write(&path, b"") {
            tracing::warn!(path = %path.display(), "could not write whole-model sentinel: {e}");
        }
    }

    /// Clear the whole-model sentinel — the node is no longer serving.
    pub fn clear_whole_model_ready() {
        let _ = std::fs::remove_file(Self::whole_model_ready_file());
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
    ///
    /// Sends SIGTERM and waits up to 5 s for a clean exit, then sends SIGKILL
    /// so the CUDA context (and VRAM) is always freed before returning.
    pub fn stop_process(&self) {
        let Some(pid) = self.read_pid() else { return };
        info!("Stopping shard server PID {}", pid);

        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
            use nix::unistd::Pid as NixPid;
            let nix_pid = NixPid::from_raw(pid as i32);
            let _ = kill(nix_pid, Signal::SIGTERM);
            for _ in 0..10 {
                std::thread::sleep(Duration::from_millis(500));
                // Use waitpid(WNOHANG) rather than sysinfo — sysinfo sees zombies as
                // still-running, causing the loop to exhaust and SIGKILL a dead process.
                match waitpid(nix_pid, Some(WaitPidFlag::WNOHANG)) {
                    Ok(WaitStatus::StillAlive) => {} // still running, keep waiting
                    _ => {
                        // Exited cleanly (or ECHILD — already reaped).
                        self.remove_pid();
                        return;
                    }
                }
            }
            warn!(
                "Shard process {} did not exit after SIGTERM — sending SIGKILL",
                pid
            );
            let _ = kill(nix_pid, Signal::SIGKILL);
            // Reap the zombie; without this the child stays defunct until run-node exits.
            let _ = waitpid(nix_pid, None);
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
    ///
    /// Kills any already-running shard child first so its CUDA context is freed
    /// before the new process allocates GPU memory.
    pub fn spawn_shard_child() -> Result<u32> {
        let mgr = Self::new();
        if mgr.is_running() {
            info!("Existing shard child running — stopping it before respawn");
            mgr.stop_process();
        }
        // Clear stale ready sentinel so callers don't see the old state.
        let _ = std::fs::remove_file(Self::ready_file());

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

// ---------------------------------------------------------------------------
// StorageApiManager — manages the background `storage serve` child process
// ---------------------------------------------------------------------------

pub struct StorageApiManager {
    pub pid_file: PathBuf,
}

impl StorageApiManager {
    pub fn new() -> Self {
        let run = run_dir();
        std::fs::create_dir_all(&run).ok();
        Self {
            pid_file: run.join("storage_serve.pid"),
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
                let alive = sys.process(Pid::from_u32(pid)).is_some();
                // Scrub a stale PID file so the next spawn doesn't false-positive.
                if !alive {
                    self.remove_pid();
                }
                alive
            }
            None => false,
        }
    }

    /// Stop the storage serve child, if running.
    pub fn stop_process(&self) {
        let Some(pid) = self.read_pid() else { return };
        info!("Stopping storage API server PID {}", pid);

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

    /// Spawn `kwaainet storage serve` as a detached background process,
    /// appending output to `storage_serve.log`.
    #[cfg(feature = "storage")]
    pub fn spawn_storage_child() -> Result<u32> {
        let exe = std::env::current_exe().context("finding own executable")?;
        let log = log_dir().join("storage_serve.log");
        std::fs::create_dir_all(log.parent().unwrap()).ok();
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log)
            .with_context(|| format!("opening storage log {}", log.display()))?;

        let mut cmd = std::process::Command::new(&exe);
        cmd.args(["storage", "serve"]);

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

        let child = cmd.spawn().context("spawning storage serve child")?;
        let pid = child.id();
        debug!("Spawned storage serve child PID {}", pid);
        std::mem::forget(child);
        Ok(pid)
    }
}

/// Returns true if something is already listening on `<port>`, on either family.
/// Use this before binding to give a friendly error instead of an OS crash.
pub fn port_in_use(port: u16) -> bool {
    !crate::net::port_is_free(port, kwaai_p2p::Ipv6Mode::Auto)
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

#[cfg(test)]
mod whole_model_readiness_tests {
    use super::*;
    use crate::config::KwaaiNetConfig;

    /// Regression for #175: a node serving whole models over the Ollama proxy
    /// must announce ONLINE (2), not JOINING (1).
    ///
    /// `announce_state` gated only on `shard.ready`, which the block-sharding
    /// path writes and the macOS whole-model path deliberately does not — so a
    /// Mac that was serving perfectly well announced JOINING for as long as it
    /// ran. That put it off the map and, worse, out of `shard run`'s candidate
    /// set, which filters on `state == 2`.
    ///
    /// Serialised and env-scoped: these tests move a real sentinel under
    /// `KWAAINET_HOME`, so they must not run against a live node's run dir.
    #[test]
    fn whole_model_serving_announces_online() {
        let _env_lock = crate::config::HOME_ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let tmp = std::env::temp_dir().join(format!("kwaai-wm-ready-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        std::env::set_var("KWAAINET_HOME", &tmp);

        // Neither mode ready: JOINING. This is the state the bug left a Mac in.
        ShardManager::clear_whole_model_ready();
        let _ = std::fs::remove_file(ShardManager::ready_file());
        assert!(!ShardManager::whole_model_is_ready());
        assert_eq!(
            KwaaiNetConfig::announce_state(),
            1,
            "nothing to serve must stay JOINING"
        );

        // Whole-model serving, no block shard — the macOS stopgap.
        ShardManager::mark_whole_model_ready();
        assert!(ShardManager::whole_model_is_ready());
        assert_eq!(
            KwaaiNetConfig::announce_state(),
            2,
            "a node serving whole models over the Ollama proxy is ONLINE"
        );

        // Losing Ollama must retract the claim rather than advertise a dead path.
        ShardManager::clear_whole_model_ready();
        assert_eq!(
            KwaaiNetConfig::announce_state(),
            1,
            "clearing the sentinel must drop back to JOINING"
        );

        std::env::remove_var("KWAAINET_HOME");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// JOINING covers two different situations and the map has to tell them
    /// apart: a shard part-way through loading, and a node that will never load
    /// one. The live pid is this test process, which is certainly running.
    #[test]
    fn shard_loading_is_only_true_while_a_shard_process_loads() {
        let _env_lock = crate::config::HOME_ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let tmp = std::env::temp_dir().join(format!("kwaai-loading-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        std::env::set_var("KWAAINET_HOME", &tmp);

        let shard = ShardManager::new();
        let _ = std::fs::remove_file(ShardManager::ready_file());
        shard.remove_pid();
        assert!(
            !KwaaiNetConfig::announce_shard_loading(),
            "no shard process means nothing is loading"
        );

        shard.write_pid(std::process::id());
        assert!(
            KwaaiNetConfig::announce_shard_loading(),
            "a live shard process with no ready sentinel is loading"
        );

        std::fs::write(ShardManager::ready_file(), "").unwrap();
        assert!(
            !KwaaiNetConfig::announce_shard_loading(),
            "a loaded shard is serving, not loading"
        );
        assert_eq!(KwaaiNetConfig::announce_state(), 2);

        std::env::remove_var("KWAAINET_HOME");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// Live check against this machine's actual Ollama, following the
    /// `live_*` convention: `#[ignore]` alone, because it is read-only against
    /// the network (localhost only) and writes its sentinel into an isolated
    /// `KWAAINET_HOME` rather than the running node's run dir.
    ///
    /// ```text
    /// cargo test -p kwaainet --bin kwaainet live_whole_model -- --ignored --nocapture
    /// ```
    ///
    /// This is the half the unit tests cannot cover: that the real
    /// `ollama::readiness` probe agrees with the sentinel, and that losing
    /// Ollama retracts the claim rather than leaving a dead path advertised.
    #[tokio::test]
    #[ignore = "probes the local Ollama on 11434"]
    // The guard is held across awaits deliberately: it serialises `KWAAINET_HOME`
    // for the whole test, and a single-threaded test runtime cannot deadlock on it.
    #[allow(clippy::await_holding_lock)]
    async fn live_whole_model_readiness_tracks_real_ollama() {
        let _env_lock = crate::config::HOME_ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let tmp = std::env::temp_dir().join(format!("kwaai-wm-live-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        std::env::set_var("KWAAINET_HOME", &tmp);

        let port: u16 = std::env::var("KWAAI_OLLAMA_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(11434);

        match crate::ollama::readiness(port).await {
            Ok(models) => eprintln!("Ollama on {port} is serving {} model(s)", models.len()),
            Err(why) => {
                eprintln!("SKIPPED: Ollama on {port} is not ready ({why})");
                std::env::remove_var("KWAAINET_HOME");
                return;
            }
        }

        // Ollama up: the probe must mark us ready and the announcement must say ONLINE.
        ShardManager::clear_whole_model_ready();
        crate::ollama::refresh_whole_model_ready(port).await;
        assert!(
            ShardManager::whole_model_is_ready(),
            "a live Ollama must produce the whole-model sentinel"
        );
        assert_eq!(
            KwaaiNetConfig::announce_state(),
            2,
            "with Ollama serving, this node announces ONLINE"
        );
        eprintln!("announce_state() = 2 (ONLINE) with Ollama up");

        // Ollama gone: the claim must be retracted, not left standing.
        crate::ollama::refresh_whole_model_ready(1).await;
        assert!(
            !ShardManager::whole_model_is_ready(),
            "an unreachable Ollama must clear the sentinel"
        );
        assert_eq!(
            KwaaiNetConfig::announce_state(),
            1,
            "with Ollama gone, the node must stop claiming it can serve"
        );
        eprintln!("announce_state() = 1 (JOINING) with Ollama unreachable");

        std::env::remove_var("KWAAINET_HOME");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// The two sentinels stay distinct: whole-model readiness must not make
    /// `shard_is_ready()` true, because its callers act on the block range that
    /// implies, and a whole-model node has none.
    #[test]
    fn whole_model_readiness_does_not_imply_a_block_shard() {
        let _env_lock = crate::config::HOME_ENV_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let tmp = std::env::temp_dir().join(format!("kwaai-wm-distinct-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        std::env::set_var("KWAAINET_HOME", &tmp);

        let _ = std::fs::remove_file(ShardManager::ready_file());
        ShardManager::mark_whole_model_ready();

        assert!(ShardManager::whole_model_is_ready());
        assert!(
            !ShardManager::shard_is_ready(),
            "the whole-model sentinel must not be mistaken for a loaded block shard"
        );
        assert_ne!(
            ShardManager::ready_file(),
            ShardManager::whole_model_ready_file(),
            "the two sentinels must be separate files"
        );

        ShardManager::clear_whole_model_ready();
        std::env::remove_var("KWAAINET_HOME");
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
