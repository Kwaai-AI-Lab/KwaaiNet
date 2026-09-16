//! `run-node` owns its `shard serve` and `storage serve` children.
//!
//! The children used to be spawned detached by the short-lived `kwaainet
//! start` process and tracked by PID file only: nothing waited on them, a
//! crashed shard stayed crashed, and a killed daemon left them serving into a
//! socket that no longer existed. Here the node is their parent: it decides
//! from the contribution policy which to run ([`plan`]), spawns them once the
//! control socket answers, restarts them with backoff when they exit, and
//! terminates them on shutdown. A child watches its parent's PID and exits
//! when it disappears ([`stop_requested`]), so a SIGKILLed daemon leaves no
//! orphans either. Same mechanism on every platform; no process groups, no
//! job objects.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::cli::StartOverrides;
use crate::config::{log_dir, KwaaiNetConfig};

/// Set on every child so it can watch for its parent going away.
pub const SUPERVISOR_PID_ENV: &str = "KWAAINET_SUPERVISOR_PID";

const SHARD_MIN_RAM_BYTES: u64 = 10 * 1024 * 1024 * 1024;
const BACKOFF_MIN: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(60);
/// A child that lived this long before exiting gets a fresh backoff.
const STABLE_AFTER: Duration = Duration::from_secs(60);
const SOCKET_WAIT: Duration = Duration::from_secs(60);
const TERM_GRACE: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Start,
    Skip(String),
}

/// Which children this instance runs, with the reason for each skip so
/// `start` can print it and `run-node` can log it.
#[derive(Debug, Clone)]
pub struct ChildPlan {
    pub shard: Decision,
    pub storage: Decision,
}

pub fn model_is_locally_available(model: &str) -> bool {
    crate::hf::resolve_snapshot(model).is_ok() || crate::ollama::resolve_model_blob(model).is_ok()
}

fn system_total_ram_bytes() -> u64 {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    sys.total_memory()
}

/// `overrides` must already be applied to `cfg`.
pub fn plan(cfg: &KwaaiNetConfig, overrides: &StartOverrides) -> ChildPlan {
    plan_with(
        cfg,
        overrides,
        model_is_locally_available(&cfg.model),
        system_total_ram_bytes(),
    )
}

/// The decision given the probes' answers. `--shard` waives the model-present
/// and RAM gates: the operator asked, so the node tries.
fn plan_with(
    cfg: &KwaaiNetConfig,
    overrides: &StartOverrides,
    model_available: bool,
    total_ram: u64,
) -> ChildPlan {
    let policy = cfg.contribute_policy();
    let explicit = overrides.shard;
    let available = explicit || model_available;
    let enough_ram = explicit || total_ram >= SHARD_MIN_RAM_BYTES;

    let shard = if policy.shards && available && enough_ram {
        Decision::Start
    } else if policy.shards && available {
        Decision::Skip(
            "Low memory (< 10 GB) — skipping shard serving to prevent OOM. \
             Override: kwaainet start --daemon --shard"
                .into(),
        )
    } else if policy.shards {
        Decision::Skip(format!(
            "No local model found for '{}' — skipping shard serving. \
             Download: kwaainet shard download",
            cfg.model
        ))
    } else if !overrides.no_contribute && cfg.contribute.shards_unset() && available {
        // Has a model and would have sharded under the old opt-out default:
        // say so rather than let its block contribution vanish on upgrade.
        Decision::Skip(
            "Block-shard serving is opt-in (experimental) — not starting it. \
             This node still serves whole-model inference via Ollama. \
             Serve blocks anyway: kwaainet config set contribute.shards true"
                .into(),
        )
    } else if overrides.no_contribute {
        Decision::Skip("Contribution disabled for this instance (--no-contribute).".into())
    } else {
        Decision::Skip("Shard serving is off (contribute.shards).".into())
    };

    let storage = if !cfg!(feature = "storage") {
        Decision::Skip("Built without the storage feature.".into())
    } else if policy.storage && cfg.storage.is_some() {
        Decision::Start
    } else if policy.storage {
        Decision::Skip("Storage not initialised — skipping. Run: kwaainet storage init".into())
    } else {
        Decision::Skip("Storage serving is off (contribute.storage / --no-contribute).".into())
    };

    ChildPlan { shard, storage }
}

struct ChildSpec {
    name: &'static str,
    args: &'static [&'static str],
    log: &'static str,
}

const SHARD: ChildSpec = ChildSpec {
    name: "shard serve",
    args: &["shard", "serve", "--auto-rebalance"],
    log: "shard.log",
};
const STORAGE: ChildSpec = ChildSpec {
    name: "storage serve",
    args: &["storage", "serve"],
    log: "storage_serve.log",
};

/// Handle on the running children. Dropping it without `shutdown` leaves the
/// supervise tasks running; `kill_on_drop` still reaps the children when the
/// runtime goes down.
pub struct Supervisor {
    stop: watch::Sender<bool>,
    tasks: Vec<JoinHandle<()>>,
}

impl Supervisor {
    pub fn start(plan: &ChildPlan) -> Self {
        sweep_orphans();
        let (stop, rx) = watch::channel(false);
        let mut tasks = Vec::new();
        for (decision, spec) in [(&plan.shard, &SHARD), (&plan.storage, &STORAGE)] {
            match decision {
                Decision::Start => tasks.push(tokio::spawn(supervise(spec, rx.clone()))),
                Decision::Skip(why) => info!("{}: {why}", spec.name),
            }
        }
        Self { stop, tasks }
    }

    /// Terminate every child (SIGTERM, then kill after a grace period) and
    /// wait for the supervise tasks to finish.
    pub async fn shutdown(self) {
        let _ = self.stop.send(true);
        for t in self.tasks {
            if tokio::time::timeout(TERM_GRACE * 3, t).await.is_err() {
                warn!("a child supervisor did not finish in time");
            }
        }
    }
}

/// Children an older binary left running. Before the supervisor existed,
/// `start` spawned them detached and nothing stopped them when the daemon
/// went — so the first start after an upgrade finds the previous shard still
/// holding its memory and the previous storage server still on its port.
/// Their PID files are the only handle on them.
fn sweep_orphans() {
    let shard = crate::daemon::ShardManager::new();
    if shard.is_running() {
        warn!("stopping a shard server left by a previous daemon");
        shard.stop_process();
    }
    let storage = crate::daemon::StorageApiManager::new();
    if storage.is_running() {
        warn!("stopping a storage server left by a previous daemon");
        storage.stop_process();
    }
}

async fn stopped(rx: &mut watch::Receiver<bool>) {
    while !*rx.borrow() {
        if rx.changed().await.is_err() {
            return;
        }
    }
}

async fn supervise(spec: &'static ChildSpec, mut stop: watch::Receiver<bool>) {
    tokio::select! {
        _ = wait_for_control_socket() => {}
        _ = stopped(&mut stop) => return,
    }

    let mut backoff = BACKOFF_MIN;
    loop {
        let started = Instant::now();
        let mut child = match spawn(spec) {
            Ok(c) => c,
            Err(e) => {
                warn!(
                    "{}: could not spawn: {e:#}; retrying in {backoff:?}",
                    spec.name
                );
                tokio::select! {
                    _ = tokio::time::sleep(backoff) => {}
                    _ = stopped(&mut stop) => return,
                }
                backoff = (backoff * 2).min(BACKOFF_MAX);
                continue;
            }
        };
        info!("{} started (PID {})", spec.name, child.id().unwrap_or(0));

        tokio::select! {
            status = child.wait() => {
                let ran = started.elapsed();
                backoff = if ran >= STABLE_AFTER { BACKOFF_MIN } else { (backoff * 2).min(BACKOFF_MAX) };
                warn!(
                    "{} exited after {ran:?} ({}); restarting in {backoff:?}",
                    spec.name,
                    status.map(|s| s.to_string()).unwrap_or_else(|e| e.to_string())
                );
                tokio::select! {
                    _ = tokio::time::sleep(backoff) => {}
                    _ = stopped(&mut stop) => return,
                }
            }
            _ = stopped(&mut stop) => {
                terminate(spec.name, &mut child).await;
                return;
            }
        }
    }
}

fn spawn(spec: &ChildSpec) -> Result<tokio::process::Child> {
    let exe = std::env::current_exe().context("finding own executable")?;
    let log: PathBuf = log_dir().join(spec.log);
    std::fs::create_dir_all(log_dir()).ok();
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log)
        .with_context(|| format!("opening {}", log.display()))?;

    // No setsid: the child shares the daemon's session and process group,
    // so a signal to the group reaches it too.
    let mut cmd = tokio::process::Command::new(exe);
    cmd.args(spec.args)
        .env(SUPERVISOR_PID_ENV, std::process::id().to_string())
        .stdin(std::process::Stdio::null())
        .stdout(log_file.try_clone()?)
        .stderr(log_file)
        .kill_on_drop(true);
    cmd.spawn()
        .with_context(|| format!("spawning {}", spec.name))
}

async fn terminate(name: &str, child: &mut tokio::process::Child) {
    #[cfg(unix)]
    if let Some(pid) = child.id() {
        use nix::sys::signal::{kill, Signal};
        let _ = kill(nix::unistd::Pid::from_raw(pid as i32), Signal::SIGTERM);
        if tokio::time::timeout(TERM_GRACE, child.wait()).await.is_ok() {
            info!("{name} stopped");
            return;
        }
        warn!("{name} did not exit after SIGTERM — killing");
    }
    let _ = child.kill().await;
    info!("{name} killed");
}

/// The children register handlers over the control socket, so it must answer
/// first. Gives up after [`SOCKET_WAIT`] and lets the spawn/backoff loop cope.
async fn wait_for_control_socket() {
    let addr = crate::shard_cmd::daemon_socket();
    let deadline = Instant::now() + SOCKET_WAIT;
    loop {
        if kwaai_p2p_daemon::P2PClient::connect(&addr).await.is_ok() {
            return;
        }
        if Instant::now() >= deadline {
            warn!("control socket {addr} not answering after {SOCKET_WAIT:?}; spawning anyway");
            return;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

// ---------------------------------------------------------------------------
// Child side
// ---------------------------------------------------------------------------

/// Whether this process was spawned by a `run-node` supervisor.
pub fn is_supervised() -> bool {
    std::env::var_os(SUPERVISOR_PID_ENV).is_some()
}

/// Resolves when the supervising parent is gone. Pending forever when not
/// supervised (a manual `kwaainet shard serve`).
async fn parent_gone() {
    let Some(pid) = std::env::var(SUPERVISOR_PID_ENV)
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
    else {
        return std::future::pending().await;
    };
    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;
        // A fresh probe each time: a reused `System` keeps a stale entry for
        // a PID that has been reaped (seen on macOS), so it never reads gone.
        if !crate::daemon::pid_alive(pid) {
            info!("supervising daemon (PID {pid}) is gone — stopping");
            return;
        }
    }
}

/// Ctrl-C, SIGTERM (unix) or the parent disappearing: everything that should
/// make a child stop cleanly.
pub async fn stop_requested() {
    #[cfg(unix)]
    let term = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut s) => {
                s.recv().await;
            }
            Err(_) => std::future::pending().await,
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = term => {}
        _ = parent_gone() => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg_with(shards: Option<bool>, storage: bool, has_store: bool) -> KwaaiNetConfig {
        let mut cfg = KwaaiNetConfig::default();
        cfg.contribute.shards = shards;
        cfg.contribute.storage = storage;
        if has_store {
            cfg.storage = serde_yaml::from_str("data_dir: /tmp/x\ncapacity_gb: 1.0\n").ok();
        }
        cfg
    }

    const PLENTY: u64 = 64 << 30;

    #[test]
    fn shard_flag_starts_the_shard_regardless_of_model_and_ram() {
        let mut cfg = cfg_with(None, true, false);
        let o = StartOverrides {
            shard: true,
            ..Default::default()
        };
        o.apply_to(&mut cfg);
        assert_eq!(plan_with(&cfg, &o, false, 0).shard, Decision::Start);
    }

    #[test]
    fn config_opt_in_still_needs_a_model_and_ram() {
        let cfg = cfg_with(Some(true), true, false);
        let o = StartOverrides::default();
        assert_eq!(plan_with(&cfg, &o, true, PLENTY).shard, Decision::Start);
        assert!(matches!(
            plan_with(&cfg, &o, false, PLENTY).shard,
            Decision::Skip(_)
        ));
        assert!(matches!(
            plan_with(&cfg, &o, true, 0).shard,
            Decision::Skip(_)
        ));
    }

    #[test]
    fn opted_out_shard_is_skipped_with_a_reason() {
        let cfg = cfg_with(Some(false), true, false);
        match plan_with(&cfg, &StartOverrides::default(), true, PLENTY).shard {
            Decision::Skip(why) => assert!(why.contains("contribute.shards"), "{why}"),
            Decision::Start => panic!("must not start"),
        }
    }

    #[test]
    fn no_contribute_skips_both() {
        let mut cfg = cfg_with(Some(true), true, true);
        let o = StartOverrides {
            no_contribute: true,
            ..Default::default()
        };
        o.apply_to(&mut cfg);
        let p = plan_with(&cfg, &o, true, PLENTY);
        assert!(matches!(p.shard, Decision::Skip(_)));
        assert!(matches!(p.storage, Decision::Skip(_)));
    }

    #[cfg(feature = "storage")]
    #[test]
    fn storage_needs_an_initialised_store() {
        let o = StartOverrides::default();
        assert!(matches!(
            plan_with(&cfg_with(None, true, false), &o, true, PLENTY).storage,
            Decision::Skip(_)
        ));
        assert_eq!(
            plan_with(&cfg_with(None, true, true), &o, true, PLENTY).storage,
            Decision::Start
        );
    }

    #[test]
    fn a_skipped_plan_starts_no_tasks() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let plan = ChildPlan {
                shard: Decision::Skip("no".into()),
                storage: Decision::Skip("no".into()),
            };
            let s = Supervisor::start(&plan);
            assert!(s.tasks.is_empty());
            s.shutdown().await;
        });
    }
}
