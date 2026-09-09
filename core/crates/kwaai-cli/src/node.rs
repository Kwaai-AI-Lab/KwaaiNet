//! Native Rust node runner
//!
//! Drives the in-process rust-libp2p stack (`node_native`) through the
//! kwaainet CLI lifecycle: PID file, gRPC surface, auto-update, SIGHUP.

use anyhow::{Context, Result};
use kwaai_p2p::NetworkConfig;
use std::time::Duration;
use tokio::signal;
use tracing::{info, warn};

use crate::announce::{DHTServerInfo, VpkInfo};
use crate::config::KwaaiNetConfig;
use crate::daemon::DaemonManager;
use crate::identity::NodeIdentity;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub async fn run_node(config: &KwaaiNetConfig, grpc_port: Option<u16>) -> Result<()> {
    // Register the SIGHUP handler BEFORE writing the PID file — see
    // [`SigHup::register`] for why the ordering is load-bearing.
    let mut sighup = SigHup::register();

    // PID tracking
    let daemon_mgr = DaemonManager::new();
    daemon_mgr
        .write_pid(std::process::id())
        .context("writing PID")?;
    info!("KwaaiNet node starting (PID {})", std::process::id());

    // -----------------------------------------------------------------------
    // gRPC IPC surface — bind FIRST, before any of the p2p / DHT /
    // inference init. This is a UI/management surface for kwaainet, so
    // the GUI (and `kwaainet` CLI subcommands) need to be able to dial
    // in immediately at startup to observe progress, not after the
    // 30-90 s p2p bootstrap completes.
    //
    // Ops that depend on p2p state (shard_run, distributed status)
    // will simply return UNAVAILABLE / NO_PEERS_FOR_MODEL until the
    // node is fully up; ops that don't (ping, status, generate) work
    // straight away. Failure on the default port is non-fatal: the p2p
    // node must keep running even if the IPC surface didn't come up. An
    // explicitly requested port that cannot bind does abort — see
    // `spawn_on_tcp_port`. The handle's Drop signals graceful shutdown
    // when run_node returns.
    //
    // The native path also hands it the swarm once that exists, which is what
    // lets the Network op serve; see `attach_network`.
    let grpc_handle = crate::grpc_server::spawn(config.clone(), grpc_port)?;

    // -----------------------------------------------------------------------
    // Persistent identity — load or generate the keypair so the PeerId is
    // stable across restarts. Credentials are bound to this DID.
    // `config.identity_key` (CLI: `--identity-key`) overrides the default
    // path, which lets bootstrap deployments mount a pre-existing key
    // (e.g. an RSA `bootstrap_keyN.bin`) without it living under
    // `~/.kwaainet/`.
    // -----------------------------------------------------------------------
    let node_identity = if let Some(ref key_path) = config.identity_key {
        NodeIdentity::load_from(key_path)
            .with_context(|| format!("loading node identity from {}", key_path.display()))?
    } else {
        NodeIdentity::load_or_create().context("loading node identity")?
    };
    let node_did = node_identity.did();
    info!("Node DID: {}", node_did);

    // Load valid VCs for this node's DID to include in DHT announcements
    let trust_attestations = match kwaai_trust::CredentialStore::open_default() {
        Ok(store) => {
            let vcs = store.load_valid_for_subject(&node_did);
            if vcs.is_empty() {
                info!("Trust attestations: none (run `kwaainet identity import-vc` to add)");
            } else {
                info!("Trust attestations: {} valid VC(s)", vcs.len());
                for vc in &vcs {
                    info!(
                        "  [{}] issued by {}",
                        vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown"),
                        &vc.issuer_did()[..vc.issuer_did().len().min(32)]
                    );
                }
            }
            vcs.iter()
                .filter_map(|vc| vc.to_compact_json().ok())
                .collect::<Vec<_>>()
        }
        Err(e) => {
            warn!(
                "Could not open credential store: {} — proceeding without VCs",
                e
            );
            vec![]
        }
    };

    let public_name = format!(
        "{}/v{}",
        config
            .public_name
            .clone()
            .unwrap_or_else(|| "kwaainet-node".to_string()),
        env!("CARGO_PKG_VERSION"),
    );

    info!(
        model = %config.model,
        blocks = config.blocks,
        port = config.port,
        name = %public_name,
        "Configuring KwaaiNet node"
    );

    // Bootstrap peers — prefer config, fall back to Petals defaults.
    //
    // The fallback is gated on `announce_self`, because "empty" means two
    // different things. For a node it means "not configured", and dialling the
    // public Petals bootstraps is a better guess than joining nothing. For a
    // bootstrap (`announce_self = false`) it is a *deliberate* empty list: a
    // bootstrap dials nobody, peers come to it, and inheriting the defaults
    // would have every bootstrap open connections to the public network on
    // startup.
    let bootstrap_peers: Vec<String> = if !config.initial_peers.is_empty() {
        config.initial_peers.clone()
    } else if config.announce_self {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        Vec::new()
    };

    if config.native_p2p == Some(false) {
        warn!("`native_p2p: false` in config.yaml is ignored — the Go p2pd path was removed; delete the key");
    }

    // The node's whole lifecycle lives in `node_native`.
    let pending_update_version = crate::node_native::run_native_node(
        config,
        &bootstrap_peers,
        &public_name,
        trust_attestations,
        &mut sighup,
        &grpc_handle,
    )
    .await?;

    daemon_mgr.remove_pid();
    respawn_after_update(pending_update_version);
    info!("KwaaiNet node stopped");
    Ok(())
}

/// Search PATH for `name`, returning the full path if found.
fn find_in_path(name: &str) -> Option<std::path::PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// Relaunch `kwaainet start --daemon` after an auto-update installed a new
/// binary, or do nothing when none was.
///
/// **Call only after this process's own cleanup has fully completed** — the PID
/// file gone, the transport down. The new process reads the PID file before
/// doing anything else, so spawning any earlier makes it see a live daemon and
/// exit(1), leaving no daemon running at all.
fn respawn_after_update(pending_update_version: Option<String>) {
    let Some(version) = pending_update_version else {
        return;
    };

    // Resolve via PATH, not current_exe(): install_update() replaces the
    // binary in place (unlink+rename on Unix, ETXTBSY-safe while this
    // process still has it open; a rename over the running EXE on
    // Windows, safe because the OS loader opens EXEs with
    // FILE_SHARE_DELETE). Either way, current_exe() can point at a stale
    // path after the swap (Linux's /proc/self/exe keeps resolving to the
    // old, now-deleted inode) — so spawning via current_exe() here could
    // silently relaunch the old binary, making the "respawned with new
    // binary" log line a lie. PATH lookup re-resolves the path fresh,
    // picking up the new file.
    #[cfg(windows)]
    let bin_name = "kwaainet.exe";
    #[cfg(not(windows))]
    let bin_name = "kwaainet";

    let new_bin = find_in_path(bin_name)
        .or_else(|| std::env::current_exe().ok())
        .unwrap_or_else(|| std::path::PathBuf::from(bin_name));
    match std::process::Command::new(&new_bin)
        .args(["start", "--daemon"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => info!(
            "Auto-update: v{} installed — respawned daemon with new binary.",
            version
        ),
        Err(e) => warn!(
            "Auto-update: v{} installed but respawn failed ({e}). \
             Run `kwaainet start --daemon` manually.",
            version
        ),
    }
}

// ---------------------------------------------------------------------------
// Timing helpers
// ---------------------------------------------------------------------------

/// Return `base ± spread` seconds using a fast LCG over the current nanosecond
/// timestamp. No `rand` crate needed. Range: `[base - spread, base + spread]`.
pub(crate) fn jitter_secs(base: u64, spread: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0) as u64;
    let r = ns
        .wrapping_mul(6_364_136_223_846_793_005_u64)
        .wrapping_add(1_442_695_040_888_963_407_u64);
    let range = 2 * spread + 1;
    base - spread + (r >> 32) % range
}

// ---------------------------------------------------------------------------
// Auto-update
// ---------------------------------------------------------------------------

/// Check for a newer release and, if found, install it automatically.
/// After a successful install the daemon exits cleanly so the OS service
/// manager (systemd, launchd) or the user can restart it with the new binary.
///
/// Returns `Some(version)` when an update was installed and the caller
/// should break the event loop; the caller is responsible for actually
/// respawning, and must only do so *after* its own cleanup (unannounce,
/// transport shutdown, PID-file removal) has fully completed. This function
/// used to spawn the replacement process itself, immediately after
/// install_update() succeeded — that raced this (still-running, not yet
/// cleaned up) process's PID file against the new process's own "is another
/// instance already running?" check, and could leave no daemon running at
/// all if the new process's startup won that race. See the respawn site at
/// the bottom of `run_node` for the fix and full explanation.
pub(crate) async fn maybe_auto_update() -> Option<String> {
    // Developer escape hatch: a long-running local debug daemon shouldn't
    // get silently replaced by the upstream release binary (which won't
    // contain whatever in-flight feature work is being tested). Setting
    // KWAAINET_NO_AUTO_UPDATE=1 disables every code path that would
    // download or install an update from inside the running node.
    if std::env::var("KWAAINET_NO_AUTO_UPDATE")
        .map(|v| !v.is_empty() && v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
    {
        return None;
    }

    let checker = crate::updater::UpdateChecker::new();
    let update = match checker.check(false).await {
        Ok(Some(u)) => u,
        _ => return None,
    };

    info!(
        "Auto-update: new version {} available — installing…",
        update.version
    );

    if let Err(e) = checker.install_update(&update.version).await {
        warn!("Auto-update install failed: {e:?}");
        return None;
    }

    // Windows can rename a running executable in place — the OS loader opens
    // EXEs with FILE_SHARE_DELETE, so the memory mapping stays valid after the
    // rename (see install_update()'s Windows branch in updater.rs). There is
    // no separate "installer batch" process that kills and replaces this one;
    // install_update() already did the file swap directly, in-process, on
    // every platform. So the respawn-after-cleanup handling below applies
    // identically to Windows and Unix — same code path, no special-casing.
    Some(update.version)
}

// ---------------------------------------------------------------------------
// Signal handling
// ---------------------------------------------------------------------------

pub(crate) async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut sigterm =
            signal::unix::signal(signal::unix::SignalKind::terminate()).expect("SIGTERM handler");
        tokio::select! {
            _ = signal::ctrl_c() => { info!("Received Ctrl-C"); }
            _ = sigterm.recv()   => { info!("Received SIGTERM"); }
        }
    }
    #[cfg(not(unix))]
    {
        signal::ctrl_c().await.expect("Ctrl-C handler");
        info!("Received Ctrl-C");
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Poll the local VPK health endpoint (non-blocking, 3 s timeout).
/// Returns the parsed JSON body on a 2xx response, None otherwise.
async fn check_vpk_health(port: u16) -> Option<serde_json::Value> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok()?;
    let url = format!("http://localhost:{}/api/health", port);
    let resp = client.get(&url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.json::<serde_json::Value>().await.ok()
}

// ---------------------------------------------------------------------------
// Lifecycle plumbing
// ---------------------------------------------------------------------------

/// A SIGHUP source that is a no-op on platforms without signals.
///
/// `tokio::select!` cannot take a `#[cfg]`-conditional arm, so the conditional
/// lives here instead.
///
/// The handler must be **registered before the PID file is written**. The shard
/// auto-rebalance path sends SIGHUP to the daemon PID to trigger a re-announce,
/// and an old shard still running when a new daemon starts reads the new PID
/// immediately — it can fire during startup, before the event loop exists.
/// Without an early registration the OS default fires and terminates the
/// process. Registering early queues the signal instead; the loop consumes it
/// once startup finishes.
pub(crate) struct SigHup {
    #[cfg(unix)]
    inner: tokio::signal::unix::Signal,
}

impl SigHup {
    /// Register the handler. Call before writing the PID file.
    pub(crate) fn register() -> Self {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            Self {
                inner: signal(SignalKind::hangup()).expect("SIGHUP handler"),
            }
        }
        #[cfg(not(unix))]
        {
            Self {}
        }
    }

    /// Resolve on the next SIGHUP; never resolves on platforms without one.
    pub(crate) async fn recv(&mut self) {
        #[cfg(unix)]
        {
            self.inner.recv().await;
        }
        #[cfg(not(unix))]
        {
            std::future::pending::<()>().await;
        }
    }
}

/// Watch the local Ollama and signal on every down→up transition.
///
/// Starts out assuming Ollama is up so a node without Ollama does not fire a
/// spurious recovery on its first tick.
///
/// The channel has capacity 1 and the send is a `try_send`: a recovery that
/// arrives while one is already queued is dropped rather than backing up, since
/// the queued one will trigger the same re-announce.
pub(crate) fn spawn_ollama_watcher(config: &KwaaiNetConfig) -> tokio::sync::mpsc::Receiver<()> {
    let (tx, rx) = tokio::sync::mpsc::channel::<()>(1);
    let ollama_port = config.ollama_port;
    let ollama_manage = config.ollama_manage;

    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_default();
        let url = format!("http://localhost:{}/api/tags", ollama_port);
        let mut was_up = true; // assume up at start to avoid a spurious recovery signal
        let mut fail_count: u32 = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(15)).await;
            let ok = client
                .get(&url)
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false);
            if ok {
                if !was_up {
                    info!(
                        "✅ Ollama recovered on port {} — signalling re-announce",
                        ollama_port
                    );
                    let _ = tx.try_send(());
                }
                was_up = true;
                fail_count = 0;
            } else {
                fail_count += 1;
                if fail_count == 3 {
                    warn!(
                        "⚠️  Ollama unreachable on port {} (3 consecutive failures)",
                        ollama_port
                    );
                    if ollama_manage {
                        info!("ollama_manage=true — attempting to start Ollama…");
                        let _ = tokio::process::Command::new("ollama").arg("serve").spawn();
                    }
                } else if fail_count > 3 && fail_count.is_multiple_of(12) {
                    // Log every ~3 min while still down
                    warn!(
                        "⚠️  Ollama still unreachable on port {} ({}× checks)",
                        ollama_port, fail_count
                    );
                }
                was_up = false;
            }
        }
    });

    rx
}

// ---------------------------------------------------------------------------
// Announce inputs
// ---------------------------------------------------------------------------

/// Measure download bandwidth once at startup, for the Petals throughput
/// formula. Returns 0.0 when no compute benchmark exists to combine it with —
/// there is nothing to cap, so the probe would only cost startup latency.
pub(crate) async fn measure_download_bps_for(model: &str) -> f64 {
    if crate::throughput::load(model).is_none() {
        return 0.0;
    }
    info!("  Measuring network bandwidth (1 MiB probe)...");
    let bps = crate::throughput::measure_download_bps().await;
    if bps > 0.0 {
        info!("  Network:  {:.1} Mbps download", bps / 1_000_000.0);
    } else {
        info!("  Network:  measurement failed — using compute limit only");
    }
    bps
}

/// [`compute_effective_tps`] plus the startup log lines that explain the number.
pub(crate) fn report_effective_tps(model: &str, dl_bps: f64, using_relay: bool) -> f64 {
    let throughput = compute_effective_tps(model, dl_bps, using_relay);
    if let Some(ref entry) = crate::throughput::load(model) {
        info!(
            "  Compute:  {:.1} tok/s (measured, hidden_dim={})",
            entry.compute_tps, entry.hidden_size
        );
        info!(
            "  Effective: {:.1} tok/s  connection={} (min({:.1}, {:.1}×{}))",
            throughput,
            if using_relay { "relay" } else { "direct" },
            entry.compute_tps,
            if dl_bps > 0.0 {
                dl_bps / (entry.hidden_size as f64 * 16.0)
            } else {
                f64::INFINITY
            },
            if using_relay { "0.2" } else { "1.0" },
        );
    } else {
        info!(
            "  Throughput: {:.1} tok/s (default — run `kwaainet benchmark` to measure)",
            throughput
        );
    }
    throughput
}

/// The HuggingFace repository URL for the `_petals.models` registry entry.
pub(crate) fn effective_repository(config: &KwaaiNetConfig) -> String {
    config.model_repository.clone().unwrap_or_else(|| {
        if config.model.contains('/') {
            format!("https://huggingface.co/{}", config.model)
        } else {
            format!("https://huggingface.co/meta-llama/{}", config.model)
        }
    })
}

/// Poll the local VPK health endpoint at startup.
///
/// Retries up to 5 times with 1 s gaps to avoid a race with the storage child
/// process, which `kwaainet start --daemon` spawns just before this one.
pub(crate) async fn initial_vpk_info(
    config: &KwaaiNetConfig,
    public_name: &str,
) -> Option<VpkInfo> {
    if !config.vpk_enabled {
        return None;
    }
    let port = config.vpk_local_port.unwrap_or(7432);
    info!("VPK enabled — checking local service on port {}", port);

    let mut health_result = None;
    for attempt in 0..5u32 {
        if let Some(h) = check_vpk_health(port).await {
            health_result = Some(h);
            break;
        }
        if attempt < 4 {
            info!("VPK not ready yet, retrying in 1 s… ({}/5)", attempt + 1);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    match health_result {
        Some(health) => {
            let info = vpk_info_from_health(&health, config, public_name);
            info!(
                "VPK healthy: mode={} tenants={} capacity={:.1}GB v={}",
                info.mode, info.tenant_count, info.capacity_gb, info.vpk_version
            );
            Some(info)
        }
        None => {
            warn!(
                "VPK health check failed on port {} after 5 attempts — skipping DHT advertisement",
                port
            );
            None
        }
    }
}

/// Decode one `/api/health` body into the announced [`VpkInfo`].
fn vpk_info_from_health(
    health: &serde_json::Value,
    config: &KwaaiNetConfig,
    public_name: &str,
) -> VpkInfo {
    VpkInfo {
        mode: config
            .vpk_mode
            .clone()
            .unwrap_or_else(|| "both".to_string()),
        capacity_gb: health["capacity_gb_available"].as_f64().unwrap_or(0.0),
        tenant_count: health["tenant_count"].as_u64().unwrap_or(0) as u32,
        vpk_version: health["version"].as_str().unwrap_or("unknown").to_string(),
        public_name: public_name.to_string(),
    }
}

/// Recompute throughput from the benchmark cache before a re-announce, logging
/// only a material change.
pub(crate) fn refresh_throughput(
    server_info: &mut DHTServerInfo,
    model: &str,
    dl_bps: f64,
    using_relay: bool,
) {
    let fresh_tps = compute_effective_tps(model, dl_bps, using_relay);
    if (fresh_tps - server_info.throughput).abs() > 0.05 {
        info!(
            "Throughput updated: {:.1} → {:.1} tok/s",
            server_info.throughput, fresh_tps
        );
        server_info.throughput = fresh_tps;
    }
}

/// Re-poll VPK health before a re-announce, so a `storage serve` started after
/// the node came up is advertised without a restart.
pub(crate) async fn refresh_vpk_info(
    server_info: &mut DHTServerInfo,
    config: &KwaaiNetConfig,
    public_name: &str,
) {
    if !config.vpk_enabled {
        return;
    }
    let port = config.vpk_local_port.unwrap_or(7432);
    let fresh_vpk = check_vpk_health(port)
        .await
        .map(|health| vpk_info_from_health(&health, config, public_name));

    if fresh_vpk.is_some() != server_info.vpk_info.is_some() {
        let label = |present: bool| if present { "enabled" } else { "disabled" };
        info!(
            "VPK state changed: {} → {}",
            label(server_info.vpk_info.is_some()),
            label(fresh_vpk.is_some()),
        );
    }
    server_info.vpk_info = fresh_vpk;
}

/// Compute effective throughput from the cached benchmark result.
///
/// Re-reads `~/.kwaainet/throughput_cache.json` on every call so that a
/// `kwaainet benchmark` run after the daemon started is reflected within
/// the next re-announcement cycle (120 s).
///
/// `dl_bps` is the download bandwidth measured at startup and reused here
/// to avoid a slow network probe on every re-announce.
pub(crate) fn compute_effective_tps(model: &str, dl_bps: f64, using_relay: bool) -> f64 {
    match crate::throughput::load(model) {
        Some(entry) => crate::throughput::effective_tps(&entry, dl_bps, using_relay),
        None => 10.0, // fallback until benchmark is run
    }
}

#[cfg(test)]
mod capacity_lease_dht_tests {
    use super::*;

    /// Decode `to_msgpack()`'s `Ext(64, ...)` wrapper down to the inner
    /// fields map, mirroring what `shard_cmd.rs`'s `decode_server_info_ext`
    /// does on the read side.
    fn decode_fields_map(bytes: &[u8]) -> rmpv::Value {
        let ext = rmpv::decode::read_value(&mut &bytes[..]).expect("valid outer msgpack");
        let inner_bytes = match ext {
            rmpv::Value::Ext(64, b) => b,
            other => panic!("expected Ext(64, ..), got {other:?}"),
        };
        let inner = rmpv::decode::read_value(&mut &inner_bytes[..]).expect("valid inner msgpack");
        match inner {
            rmpv::Value::Array(arr) if arr.len() == 3 => arr[2].clone(),
            other => {
                panic!("expected a 3-element [state, throughput, fields] array, got {other:?}")
            }
        }
    }

    fn find_field<'a>(fields: &'a rmpv::Value, key: &str) -> Option<&'a rmpv::Value> {
        match fields {
            rmpv::Value::Map(entries) => entries
                .iter()
                .find(|(k, _)| k.as_str() == Some(key))
                .map(|(_, v)| v),
            _ => None,
        }
    }

    #[test]
    fn to_msgpack_announces_lease_v1_true() {
        let info = DHTServerInfo::new(
            0,
            32,
            "test-node",
            false,
            10.0,
            vec![],
            None,
            "peer123".to_string(),
        );
        let bytes = info.to_msgpack().expect("encode");
        let fields = decode_fields_map(&bytes);

        let lease_v1 = find_field(&fields, "lease_v1").expect("lease_v1 key present");
        assert_eq!(lease_v1.as_bool(), Some(true));
    }
}
