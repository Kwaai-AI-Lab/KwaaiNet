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

/// Name of the stop-request file under `run/`. See
/// [`DaemonManager::stop_request_file`].
pub const STOP_REQUEST_FILE: &str = "stop.request";

/// How long `stop` gives the node's own shutdown before killing it. The node
/// sizes its last-moment work to fit (`UPDATE_STOP_GRACE` in `node_native`).
pub const DAEMON_STOP_GRACE: Duration = Duration::from_secs(10);

/// How often a Windows node checks for a stop request.
#[cfg(any(not(unix), test))]
pub const STOP_REQUEST_POLL: Duration = Duration::from_millis(250);

/// How long `stop` waits for a forced daemon to be gone, so a restart does
/// not race its predecessor for the instance lock and ports.
#[cfg(windows)]
const KILL_WAIT: Duration = Duration::from_secs(5);

/// Resolve once a stop request naming `pid` appears at `path`. The node's
/// Windows stand-in for waiting on SIGTERM; a file because the request has
/// to cross from an unrelated process to a detached one, and `run/` already
/// carries `reannounce.flag` the same way. Unix waits on SIGTERM itself, so
/// it has no caller there.
///
/// A request naming another PID is stale — `stop` read an old PID file —
/// and is removed rather than obeyed. An empty or unreadable one is left
/// alone and read again: the writer renames a finished file into place, so
/// that is a transient, not a stale request.
#[cfg(any(not(unix), test))]
pub async fn wait_for_stop_request(path: &std::path::Path, pid: u32) {
    loop {
        if let Ok(text) = std::fs::read_to_string(path) {
            match text.trim().parse::<u32>() {
                Ok(p) if p == pid => return,
                Ok(other) => {
                    warn!("Ignoring a stop request for PID {other} (this node is PID {pid})");
                    let _ = std::fs::remove_file(path);
                }
                Err(_) => {}
            }
        }
        tokio::time::sleep(STOP_REQUEST_POLL).await;
    }
}

/// Whether `pid` is running a `kwaainet` executable — the guard against
/// force-killing whatever reused the PID a stale PID file names.
#[cfg(windows)]
fn is_kwaainet_process(pid: u32) -> bool {
    let mut sys = System::new();
    let pid = Pid::from_u32(pid);
    sys.refresh_process(pid);
    // Exactly the CLI's executable: `kwaainet_gui.exe` shares the prefix.
    sys.process(pid)
        .is_some_and(|p| p.name().eq_ignore_ascii_case("kwaainet.exe"))
}

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

    // -----------------------------------------------------------------------
    // Stop request (Windows' SIGTERM)
    // -----------------------------------------------------------------------

    /// The file `stop` writes on Windows to ask the node to shut down
    /// cleanly. Windows has no SIGTERM to deliver to a detached process,
    /// and a hard kill skips the shutdown path, which is what writes the
    /// `state = -1` tombstone. Without it, the node's last record reads
    /// online. It sits beside the PID file, so it is per `KWAAINET_HOME`.
    pub fn stop_request_file(&self) -> PathBuf {
        self.pid_file.with_file_name(STOP_REQUEST_FILE)
    }

    /// Drop a stop request nobody consumed — one written to a daemon that
    /// predates it and was force-killed, say — so it cannot stop the next
    /// node the moment it starts. The node calls this before writing its PID
    /// file, which is what `stop` reads before writing a request, so a
    /// request aimed at *this* node cannot be the one cleared.
    pub fn clear_stop_request(&self) {
        let _ = std::fs::remove_file(self.stop_request_file());
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
    // Instance lock
    // -----------------------------------------------------------------------

    /// Take `run/kwaainet.lock` for as long as the returned file lives, or
    /// `None` if another node holds it. Everything under a `KWAAINET_HOME` is
    /// shared — identity, PID, gRPC token and socket — so it runs one node.
    #[cfg(unix)]
    pub fn acquire_instance_lock(&self) -> Result<Option<std::fs::File>> {
        use nix::fcntl::{flock, FlockArg};
        use std::os::unix::io::AsRawFd;

        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false) // lock file; content irrelevant, we only use flock
            .write(true)
            .open(&self.lock_file)
            .with_context(|| format!("opening lock file {}", self.lock_file.display()))?;

        match flock(file.as_raw_fd(), FlockArg::LockExclusiveNonblock) {
            Ok(()) => Ok(Some(file)),
            Err(nix::errno::Errno::EWOULDBLOCK) => Ok(None),
            Err(e) => bail!("flock: {}", e),
        }
    }

    /// No sharing: a second open fails until this handle closes.
    #[cfg(windows)]
    pub fn acquire_instance_lock(&self) -> Result<Option<std::fs::File>> {
        use std::os::windows::fs::OpenOptionsExt;
        const ERROR_SHARING_VIOLATION: i32 = 32;

        match std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .share_mode(0)
            .open(&self.lock_file)
        {
            Ok(file) => Ok(Some(file)),
            Err(e) if e.raw_os_error() == Some(ERROR_SHARING_VIOLATION) => Ok(None),
            Err(e) => {
                Err(e).with_context(|| format!("opening lock file {}", self.lock_file.display()))
            }
        }
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

    /// Ask the node to shut down, give it up to 10 s for its own shutdown
    /// (DHT unannounce, its children), then kill it. On Unix the request is
    /// SIGTERM; on Windows, the stop-request file (see
    /// [`Self::request_stop_and_wait`]).
    pub fn stop_process(&self) -> Result<()> {
        self.stop().map(|_| ())
    }

    /// [`Self::stop_process`], reporting whether a daemon was actually
    /// running: `Ok(false)` is a PID file left by a node that is gone, which
    /// is removed. Only Windows can tell; Unix reports `true` as before.
    pub fn stop(&self) -> Result<bool> {
        let pid = self.read_pid().context("No daemon is running")?;
        info!("Stopping daemon PID {}", pid);
        #[cfg(windows)]
        let was_running = self.request_stop_and_wait(pid, DAEMON_STOP_GRACE);
        #[cfg(not(windows))]
        let was_running = {
            stop_pid(pid, DAEMON_STOP_GRACE);
            true
        };
        self.remove_pid();
        Ok(was_running)
    }

    /// Windows: write the stop request, wait for the daemon to exit for up
    /// to `grace`, and force it only if it is still there. Returns whether a
    /// daemon was running at all.
    ///
    /// The wait is on a process handle (`WaitForSingleObject`), not a PID
    /// probe: a PID probe reads a dead process as alive while anyone holds a
    /// handle to it. When the handle cannot be opened — the daemon runs as
    /// another user, say — the request is still written, since it is only a
    /// file, and the wait falls back to the instance lock. The lock is not
    /// the first choice because a v0.7.3 daemon, which predates it, never
    /// takes it. Such a daemon also never answers the request, and is forced
    /// after `grace` as before.
    #[cfg(windows)]
    fn request_stop_and_wait(&self, pid: u32, grace: Duration) -> bool {
        let process = match win::Process::open(pid) {
            Ok(p) => Some(p),
            Err(win::OpenError::NoSuchProcess) => {
                info!("Daemon PID {pid} is not running — removing its PID file");
                self.clear_stop_request();
                return false;
            }
            Err(win::OpenError::Other(code)) => {
                warn!("Cannot open daemon PID {pid} (OpenProcess error {code}) — asking it to stop anyway");
                None
            }
        };
        // Taken before the request: a lock already free now proves nothing
        // about a daemon that never held it.
        let lock_was_held = process.is_none() && !self.instance_lock_free();

        let asked = match self.write_stop_request(pid) {
            Ok(()) => true,
            Err(e) => {
                warn!("Writing {}: {e:#}", self.stop_request_file().display());
                false
            }
        };
        let exited = asked
            && match &process {
                Some(p) => p.wait(grace),
                None => lock_was_held && self.wait_for_instance_lock(grace),
            };
        self.clear_stop_request();
        if exited {
            info!("Daemon PID {pid} shut down cleanly");
            return true;
        }

        if asked {
            warn!(
                "Daemon PID {pid} did not exit within {grace:?} of the stop request — forcing it. \
                 A daemon older than the request never answers it; a newer one may have \
                 unannounced and still be stopping its children"
            );
        }
        if !is_kwaainet_process(pid) {
            // It may simply have finished just after the deadline.
            if process.as_ref().is_some_and(|p| p.wait(Duration::ZERO)) {
                info!("Daemon PID {pid} exited just after the deadline");
                return true;
            }
            warn!("PID {pid} is not a kwaainet process — not killing it");
            return false;
        }
        match &process {
            Some(p) if p.terminate() => {
                if !p.wait(KILL_WAIT) {
                    warn!("Daemon PID {pid} still running {KILL_WAIT:?} after TerminateProcess");
                }
            }
            _ => stop_pid(pid, grace),
        }
        true
    }

    /// Write the request atomically, naming the PID it is for: the node
    /// ignores one meant for another process, and must never read a
    /// half-written file.
    #[cfg(windows)]
    fn write_stop_request(&self, pid: u32) -> Result<()> {
        let request = self.stop_request_file();
        let tmp = request.with_extension("request.tmp");
        std::fs::write(&tmp, pid.to_string())?;
        std::fs::rename(&tmp, &request)?;
        Ok(())
    }

    /// Whether nobody holds the instance lock. Takes it and lets go at once.
    #[cfg(windows)]
    fn instance_lock_free(&self) -> bool {
        matches!(self.acquire_instance_lock(), Ok(Some(_)))
    }

    /// Wait up to `grace` for the instance lock to come free, which the
    /// node lets go of only after unannouncing and stopping its children.
    #[cfg(windows)]
    fn wait_for_instance_lock(&self, grace: Duration) -> bool {
        let deadline = std::time::Instant::now() + grace;
        while std::time::Instant::now() < deadline {
            if self.instance_lock_free() {
                return true;
            }
            std::thread::sleep(STOP_REQUEST_POLL);
        }
        false
    }

    // -----------------------------------------------------------------------
    // Daemonize
    // -----------------------------------------------------------------------

    /// The node flags this daemon was started with, beside the PID file (the
    /// same idea as PostgreSQL's `postmaster.opts`): what `restart`,
    /// `reconnect` and `update` relaunch. Removed by `stop`, never by a
    /// restart, so the shape of the instance survives its own stop/start.
    fn start_args_file() -> PathBuf {
        run_dir().join("start-args.json")
    }

    pub fn write_start_args(overrides: &crate::cli::StartOverrides) {
        let path = Self::start_args_file();
        let _ = std::fs::create_dir_all(run_dir());
        let json = serde_json::to_string_pretty(overrides).expect("flags serialise");
        if let Err(e) = std::fs::write(&path, json) {
            warn!("Could not record start flags at {}: {e}", path.display());
        }
    }

    /// Flags recorded by the last `start --daemon`; none if never started or
    /// stopped since.
    pub fn read_start_args() -> crate::cli::StartOverrides {
        let path = Self::start_args_file();
        let Ok(json) = std::fs::read_to_string(&path) else {
            return Default::default();
        };
        serde_json::from_str(&json).unwrap_or_else(|e| {
            warn!("Ignoring unreadable start flags at {}: {e}", path.display());
            Default::default()
        })
    }

    pub fn remove_start_args() {
        let _ = std::fs::remove_file(Self::start_args_file());
    }

    /// `start --daemon` plus the recorded flags: the argv every relaunch uses.
    pub fn relaunch_args() -> Vec<String> {
        let mut v = vec!["start".to_string(), "--daemon".to_string()];
        v.extend(Self::read_start_args().to_argv());
        v
    }

    /// Re-launch the current binary as a detached `run-node` with the given
    /// node flags. Returns immediately in the parent.
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
        cmd.args(extra_args);

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

        #[cfg(windows)]
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

    /// Stop the shard serve child, if running. SIGKILL after the grace period
    /// so the CUDA context (and VRAM) is always freed before returning.
    pub fn stop_process(&self) {
        let Some(pid) = self.read_pid() else { return };
        info!("Stopping shard server PID {}", pid);
        stop_pid(pid, Duration::from_secs(5));
        self.remove_pid();
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
        stop_pid(pid, Duration::from_secs(5));
        self.remove_pid();
    }
}

/// SIGTERM, wait up to `grace` for the process to go, then SIGKILL. Liveness
/// is a PID probe that counts a zombie as gone, so it is right whether or not
/// the process is our child — the old `waitpid` version returned ECHILD for
/// a process another `kwaainet` had spawned and declared it stopped after
/// half a second. If it is our child, the exit is reaped as well.
fn stop_pid(pid: u32, grace: Duration) {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::sys::wait::{waitpid, WaitPidFlag};
        let nix_pid = nix::unistd::Pid::from_raw(pid as i32);
        let _ = kill(nix_pid, Signal::SIGTERM);
        let deadline = std::time::Instant::now() + grace;
        while std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(250));
            let _ = waitpid(nix_pid, Some(WaitPidFlag::WNOHANG));
            if !pid_alive(pid) {
                return;
            }
        }
        warn!("Process {pid} did not exit after SIGTERM — sending SIGKILL");
        let _ = kill(nix_pid, Signal::SIGKILL);
        let _ = waitpid(nix_pid, None);
    }
    // `taskkill` is a Windows program, so this is `windows`, not `not(unix)`.
    #[cfg(windows)]
    {
        // A hard kill: there is no signal to wait out. The daemon is asked
        // first, by `DaemonManager::request_stop_and_wait`; this is its
        // fallback, and the whole stop for the children's PID files.
        let _ = grace;
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output();
    }
}

/// Remove `path` if it records `me`. Returns whether it did.
fn remove_pid_file_if_ours(path: &std::path::Path, me: u32) -> bool {
    let ours = std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        == Some(me);
    ours && std::fs::remove_file(path).is_ok()
}

/// For a child about to exit without unwinding (the supervisor watchdog):
/// drop the PID file and ready sentinel this process wrote, so nothing
/// later mistakes a recycled PID for a live server.
pub fn remove_own_child_state() {
    let me = std::process::id();
    if remove_pid_file_if_ours(&ShardManager::new().pid_file, me) {
        let _ = std::fs::remove_file(ShardManager::ready_file());
    }
    remove_pid_file_if_ours(&StorageApiManager::new().pid_file, me);
}

/// Whether `pid` is a live, non-zombie process. Unix only: on Windows a PID
/// probe reads a dead process as alive while anyone holds a handle to it.
#[cfg(unix)]
fn pid_alive(pid: u32) -> bool {
    let mut sys = System::new();
    let pid = Pid::from_u32(pid);
    sys.refresh_process(pid);
    sys.process(pid)
        .is_some_and(|p| p.status() != sysinfo::ProcessStatus::Zombie)
}

/// A waitable handle on a Windows process: what tells `stop` the daemon has
/// really gone, rather than merely that its PID still resolves.
#[cfg(windows)]
mod win {
    use std::time::Duration;
    use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, ERROR_INVALID_PARAMETER, HANDLE, WAIT_OBJECT_0,
    };
    use windows_sys::Win32::System::Threading::{
        OpenProcess, TerminateProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
    };

    pub enum OpenError {
        /// No process has this PID (`ERROR_INVALID_PARAMETER`).
        NoSuchProcess,
        /// Anything else, such as access denied: the process may well be
        /// alive, so the caller must not treat this as stopped.
        Other(u32),
    }

    pub struct Process(HANDLE);

    impl Process {
        /// Open for waiting and, where allowed, terminating. Killing through
        /// this handle cannot hit a process that reused the PID since.
        pub fn open(pid: u32) -> Result<Self, OpenError> {
            for access in [PROCESS_SYNCHRONIZE | PROCESS_TERMINATE, PROCESS_SYNCHRONIZE] {
                // SAFETY: plain FFI call; a null return is the documented failure.
                let handle = unsafe { OpenProcess(access, 0, pid) };
                if !handle.is_null() {
                    return Ok(Self(handle));
                }
            }
            // SAFETY: reads this thread's last-error value.
            match unsafe { GetLastError() } {
                ERROR_INVALID_PARAMETER => Err(OpenError::NoSuchProcess),
                code => Err(OpenError::Other(code)),
            }
        }

        /// Whether the process exited within `timeout`. A process that has
        /// already exited is signalled even while handles to it stay open,
        /// which is exactly the case a PID probe gets wrong.
        pub fn wait(&self, timeout: Duration) -> bool {
            let ms = u32::try_from(timeout.as_millis()).unwrap_or(u32::MAX - 1);
            // SAFETY: `self.0` is a live handle opened with SYNCHRONIZE.
            unsafe { WaitForSingleObject(self.0, ms) == WAIT_OBJECT_0 }
        }

        /// Force the process down. False when the handle lacks
        /// PROCESS_TERMINATE, and the caller falls back to `taskkill`.
        pub fn terminate(&self) -> bool {
            // SAFETY: `self.0` is a live process handle.
            unsafe { TerminateProcess(self.0, 1) != 0 }
        }
    }

    impl Drop for Process {
        fn drop(&mut self) {
            // SAFETY: `self.0` came from OpenProcess and is closed once, here.
            unsafe { CloseHandle(self.0) };
        }
    }
}

/// Returns true if something is already listening on `<port>`, on every
/// family `mode` would bind. Use this before binding to give a friendly error
/// instead of an OS crash.
pub fn port_in_use(port: u16, mode: kwaai_p2p::Ipv6Mode) -> bool {
    !crate::net::port_is_free(port, mode)
}

// On Unix we need libc for setsid
#[cfg(unix)]
mod libc {
    extern "C" {
        pub fn setsid() -> i32;
    }
}

#[cfg(test)]
mod own_pid_file_tests {
    use super::remove_pid_file_if_ours;

    #[test]
    fn removes_only_a_file_that_records_this_pid() {
        let dir = tempfile::tempdir().unwrap();
        let ours = dir.path().join("ours.pid");
        let theirs = dir.path().join("theirs.pid");
        let junk = dir.path().join("junk.pid");
        std::fs::write(&ours, "4242\n").unwrap();
        std::fs::write(&theirs, "4243").unwrap();
        std::fs::write(&junk, "not a pid").unwrap();

        assert!(remove_pid_file_if_ours(&ours, 4242));
        assert!(!ours.exists());
        assert!(!remove_pid_file_if_ours(&theirs, 4242));
        assert!(theirs.exists(), "another process's file is left alone");
        assert!(!remove_pid_file_if_ours(&junk, 4242));
        assert!(!remove_pid_file_if_ours(
            &dir.path().join("missing.pid"),
            4242
        ));
    }
}

#[cfg(test)]
mod stop_request_tests {
    use super::*;

    fn mgr(dir: &std::path::Path) -> DaemonManager {
        DaemonManager {
            pid_file: dir.join("kwaainet.pid"),
            lock_file: dir.join("kwaainet.lock"),
            status_file: dir.join("kwaainet.status"),
        }
    }

    /// Per home: beside the PID file, under the name the node polls for.
    #[test]
    fn the_request_sits_beside_the_pid_file() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(
            mgr(tmp.path()).stop_request_file(),
            tmp.path().join(STOP_REQUEST_FILE)
        );
    }

    /// A request left by a force-killed old daemon must not stop the next
    /// node on arrival; clearing when there is none is not an error.
    #[test]
    fn a_leftover_request_is_cleared() {
        let tmp = tempfile::tempdir().unwrap();
        let m = mgr(tmp.path());
        m.clear_stop_request();
        std::fs::write(m.stop_request_file(), "4242").unwrap();
        m.clear_stop_request();
        assert!(!m.stop_request_file().exists());
    }

    const ME: u32 = 4242;

    async fn waits_out(request: &std::path::Path, polls: u32) -> bool {
        tokio::time::timeout(
            STOP_REQUEST_POLL * polls,
            wait_for_stop_request(request, ME),
        )
        .await
        .is_err()
    }

    #[tokio::test]
    async fn the_node_waits_until_a_request_for_it_appears() {
        let tmp = tempfile::tempdir().unwrap();
        let request = tmp.path().join(STOP_REQUEST_FILE);

        assert!(waits_out(&request, 3).await, "no request, no stop");

        let writer = {
            let request = request.clone();
            tokio::spawn(async move {
                tokio::time::sleep(STOP_REQUEST_POLL).await;
                std::fs::write(request, ME.to_string()).unwrap();
            })
        };
        let answered = !waits_out(&request, 20).await;
        writer.await.unwrap();
        assert!(
            answered,
            "a request naming this PID is seen within a few polls"
        );
    }

    /// `stop` read a stale PID file: its request is for some other PID. It
    /// must not stop this node, and is cleared so it cannot linger.
    #[tokio::test]
    async fn a_request_for_another_pid_is_ignored_and_removed() {
        let tmp = tempfile::tempdir().unwrap();
        let request = tmp.path().join(STOP_REQUEST_FILE);
        std::fs::write(&request, (ME + 4).to_string()).unwrap();

        assert!(waits_out(&request, 3).await, "not ours, no stop");
        assert!(!request.exists(), "the stale request is removed");
    }

    /// An empty file is a write in progress, not a stale request: left in
    /// place, and obeyed once it names this PID.
    #[tokio::test]
    async fn an_empty_request_is_read_again_not_removed() {
        let tmp = tempfile::tempdir().unwrap();
        let request = tmp.path().join(STOP_REQUEST_FILE);
        std::fs::write(&request, "").unwrap();

        assert!(waits_out(&request, 3).await, "empty, no stop");
        assert!(request.exists(), "an empty request is left alone");
        std::fs::write(&request, ME.to_string()).unwrap();
        assert!(!waits_out(&request, 20).await, "then obeyed");
    }

    /// The wait `stop` relies on: a live process is not signalled, and one
    /// that has exited is, even while a handle to it is still open — the
    /// case a PID probe reads as alive.
    #[cfg(windows)]
    #[test]
    fn a_process_handle_tells_running_from_exited() {
        use std::process::{Command, Stdio};

        // `pause` blocks on stdin, so the child lives until we say otherwise
        // and leaves nothing behind.
        let mut child = Command::new("cmd")
            .args(["/C", "pause"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        let Ok(process) = win::Process::open(child.id()) else {
            panic!("a live child can be opened");
        };
        assert!(!process.wait(Duration::from_millis(300)), "still running");

        child.kill().unwrap();
        // `child` still holds its own handle here.
        assert!(process.wait(Duration::from_secs(10)), "exited is signalled");
        child.wait().unwrap();
    }

    /// No process at that PID reads as gone, not as an error to force past.
    #[cfg(windows)]
    #[test]
    fn an_unused_pid_is_no_such_process() {
        // PIDs are multiples of four and nowhere near this high.
        assert!(matches!(
            win::Process::open(4_000_000_000),
            Err(win::OpenError::NoSuchProcess)
        ));
    }
}

#[cfg(test)]
mod instance_lock_tests {
    use super::*;

    /// One node per home: a second taker is refused until the first lets go.
    #[test]
    fn instance_lock_admits_one_holder_at_a_time() {
        let tmp = tempfile::tempdir().unwrap();
        let mgr = || DaemonManager {
            pid_file: tmp.path().join("kwaainet.pid"),
            lock_file: tmp.path().join("kwaainet.lock"),
            status_file: tmp.path().join("kwaainet.status"),
        };

        let first = mgr().acquire_instance_lock().unwrap();
        assert!(first.is_some(), "a free lock is taken");
        assert!(
            mgr().acquire_instance_lock().unwrap().is_none(),
            "a held lock refuses a second node"
        );
        drop(first);
        assert!(
            mgr().acquire_instance_lock().unwrap().is_some(),
            "dropping the holder frees it"
        );
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

        ShardManager::mark_whole_model_ready();
        assert!(
            !KwaaiNetConfig::announce_shard_loading(),
            "a whole-model server is serving, not loading (#287)"
        );
        ShardManager::clear_whole_model_ready();

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
