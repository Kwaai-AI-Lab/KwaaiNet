//! Configuration management for KwaaiNet
//!
//! Config file lives at `~/.kwaainet/config.yaml`.
//! On first run a default config is written and returned.

use crate::daemon::ShardManager;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{debug, info};

// ---------------------------------------------------------------------------
// Directory helpers
// ---------------------------------------------------------------------------

pub fn kwaainet_dir() -> PathBuf {
    if let Ok(home) = std::env::var("KWAAINET_HOME") {
        return PathBuf::from(home);
    }
    // Unit tests must never resolve to the developer's real ~/.kwaainet.
    //
    // `set_key` ends in `save()`, and several tests call it. With the real path
    // that serialises a `Default`-derived config straight over the live one,
    // silently deleting every key a default does not carry. On this machine
    // `cargo test` destroyed 23 registered knowledge bases, `inference_url`,
    // the storage block and the vpk settings — the KB data survived on disk,
    // but nothing referenced it any more.
    //
    // The sandbox is deliberately here rather than in the tests: a guard each
    // test has to remember is a guard that eventually gets forgotten, and the
    // failure is silent and off-target. `KWAAINET_HOME` still wins, so
    // integration tests that want a specific directory keep working.
    #[cfg(test)]
    {
        test_sandbox_dir()
    }
    #[cfg(not(test))]
    {
        dirs_home().join(".kwaainet")
    }
}

/// Per-process temp directory standing in for `~/.kwaainet` under `cargo test`.
#[cfg(test)]
fn test_sandbox_dir() -> PathBuf {
    use std::sync::OnceLock;
    static DIR: OnceLock<PathBuf> = OnceLock::new();
    DIR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("kwaainet-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        dir
    })
    .clone()
}

pub fn config_file() -> PathBuf {
    kwaainet_dir().join("config.yaml")
}

pub fn run_dir() -> PathBuf {
    kwaainet_dir().join("run")
}

pub fn log_dir() -> PathBuf {
    kwaainet_dir().join("logs")
}

pub fn log_file() -> PathBuf {
    log_dir().join("kwaainet.log")
}

fn dirs_home() -> PathBuf {
    dirs_sys::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

// ---------------------------------------------------------------------------
// Config structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KwaaiNetConfig {
    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default = "default_blocks")]
    pub blocks: u32,

    /// First transformer block this node serves (0-indexed). The node serves
    /// blocks `[start_block .. start_block + blocks)`.
    ///
    /// **Three states.** `Some(n)` pins the range — set by the operator or
    /// written back after an auto-assignment — and `None` means the node has
    /// never been given one, so `shard serve` may pick a gap to fill.
    ///
    /// It is an `Option` because `0` is a legitimate pinned value and was
    /// previously indistinguishable from "unset": the auto-assign condition
    /// consulted only the CLI flag, so a node with `start_block: 0` in its
    /// config was reassigned anyway and had its config overwritten. See #116.
    ///
    /// Read it through [`KwaaiNetConfig::start_block`], never directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_block: Option<u32>,

    /// Whether the `start_block` above was chosen by auto-assignment rather
    /// than by the operator.
    ///
    /// Both sources write the same field, so without this the two are
    /// indistinguishable — and they must not be treated alike. A recorded
    /// auto-assignment exists to keep the range stable across restarts (#124);
    /// an operator's pin is a statement that this node serves *that* range and
    /// the rebalancer must leave it alone. Collapsing the two would either
    /// disable rebalancing for every node that ever auto-assigned, or let the
    /// rebalancer move a node the operator placed deliberately.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub start_block_auto: bool,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_true")]
    pub use_gpu: bool,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Inference API base URL used by RAG chat/query when no per-KB value is set.
    /// Point this at a remote kwaainet node (port 8080) or Ollama instance (port 11434)
    /// to offload LLM calls while keeping retrieval local.
    /// Example: kwaainet config set inference_url http://192.168.1.10:11434
    #[serde(
        default = "default_inference_url",
        skip_serializing_if = "is_default_inference_url"
    )]
    pub inference_url: String,

    #[serde(default)]
    pub public_name: Option<String>,

    #[serde(default)]
    pub public_ip: Option<String>,

    /// Public-side TCP port to advertise in the announce_addr derived from
    /// `public_ip`. When `None`, defaults to the listening `port`. Use this
    /// for port-forwarded deployments where the router maps an external port
    /// (e.g. 443 or 8443) to the node's internal listen port (e.g. 8080).
    /// Has no effect when `announce_addr` is set explicitly — that's a full
    /// multiaddr and carries its own port.
    #[serde(default)]
    pub public_port: Option<u16>,

    #[serde(default)]
    pub announce_addr: Option<String>,

    /// Override path to the persistent identity key file. When set, this
    /// libp2p-protobuf-encoded key is used instead of the default one at
    /// `~/.kwaainet/identity.key` (which is auto-generated as Ed25519).
    /// Used by bootstrap deployments to keep their existing RSA peer IDs.
    /// CLI: `--identity-key <path>`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_key: Option<std::path::PathBuf>,

    #[serde(default)]
    pub no_relay: bool,

    #[serde(default = "default_peers")]
    pub initial_peers: Vec<String>,

    /// Multiaddrs of peers to pin as circuit relays.
    ///
    /// An **operator override**, empty by default. Relay candidates normally
    /// come from identify: any peer advertising
    /// `/libp2p/circuit/relay/0.2.0/hop` is one, which on the live network
    /// includes both bootstraps and every KwaaiNet node not run with
    /// `no_relay`. Set this to force traffic through specific known-good
    /// relays — a NAT-isolated test topology, or a deployment where one relay
    /// must be preferred. Pinned relays are tried before discovered ones.
    #[serde(default = "default_trusted_relays")]
    pub trusted_relays: Vec<String>,

    /// When true, pre-declare this node as private (`-forceReachabilityPrivate`)
    /// so AutoRelay activates immediately without waiting for AutoNAT probes.
    /// Side effect: AutoNAT can never *promote* the node to public, even if
    /// it actually is. Defaults to true so AutoRelay activates on start without
    /// waiting for AutoNAT probes (AutoNAT can falsely detect public reachability
    /// via NAT-PMP, preventing relay circuits from forming).
    #[serde(default = "default_force_private")]
    pub force_private: bool,

    /// Run the node on the in-process rust-libp2p stack instead of spawning the
    /// Go `p2pd` child process.
    ///
    /// The native path reuses every other setting on this struct — `port`,
    /// `initial_peers`, `identity_key` (so the PeerId is identical either way)
    /// and `KWAAINET_SOCKET` — and serves the same p2pd control socket, so
    /// external clients (the GUI, `kwaainet p2p …`, `shard serve`) cannot tell
    /// the two apart. NAT traversal is included: AutoNAT, circuit relay, DCUtR
    /// and UPnP all run in-process, and `no_relay`, `force_private`,
    /// `trusted_relays`, `announce_addr` and `public_ip` all take effect on
    /// this path (see `node_native`'s module docs for the flag-by-flag mapping
    /// against p2pd).
    ///
    /// **Three states, deliberately.** `Some(true)` runs native, `Some(false)`
    /// is an explicit opt-out, and `None` means "never chosen" and takes
    /// [`DEFAULT_NATIVE_P2P`].
    ///
    /// It is an `Option` so the cutover can flip the default without overriding
    /// anyone who has already said no. A plain `bool` could not distinguish the
    /// two: `load()` only writes the config file when it is absent, so an
    /// existing `config.yaml` is never rewritten on upgrade, and whether the key
    /// is present at all depends on which version first created that file.
    /// Flipping a `bool` default would therefore have flipped some nodes and not
    /// others, decided by config vintage rather than by anyone's intent.
    ///
    /// Read it through [`KwaaiNetConfig::native_p2p`], never directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_p2p: Option<bool>,
    /// Ask the local gateway to map our listen port via UPnP/IGD
    /// (`-natPortMap`).
    ///
    /// On by default, which is the behaviour this replaces — the flag was
    /// previously passed unconditionally. Turning it off is how you get a
    /// genuinely NATed node without touching router settings, which makes the
    /// relay and hole-punch paths testable; it also lets an operator decline
    /// to have the node ask the router to open ports at all.
    ///
    /// Takes effect at startup, since the mapping is requested when the daemon
    /// launches. Changing it needs a restart.
    #[serde(default = "default_enable_upnp")]
    pub enable_upnp: bool,

    /// Whether this node publishes DHT records *of its own* — its block range,
    /// `_petals.models`, `_kwaai.inference.nodes` and the VPK registry entry —
    /// and, symmetrically, the `state = -1` tombstone on shutdown.
    ///
    /// A bootstrap node should set this false: it stores and serves other
    /// peers' records without publishing any. Left true, it would appear on the
    /// map as an inference node offering zero blocks.
    ///
    /// This is about *publishing*, not *serving*. Serving — answering
    /// rpc_ping/rpc_store/rpc_find for other peers — is not configurable: every
    /// native node does it, and there is no record validation, so a served
    /// store accepts any key from any peer that can reach it. Validators are
    /// the control for that, not a config key.
    #[serde(default = "default_true")]
    pub announce_self: bool,

    #[serde(default)]
    pub health_monitoring: HealthConfig,

    /// Canonical Hivemind DHT prefix for the selected model
    /// (e.g. "Llama-3-1-8B-Instruct-hf"), set from the network map.
    /// Used as the DHT key prefix when announcing blocks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_dht_prefix: Option<String>,

    /// HuggingFace repository URL for the selected model, set from the network map.
    /// Used in the _petals.models DHT registry entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_repository: Option<String>,

    // ── VPK (Virtual Private Knowledge) integration ──────────────────────────
    /// Whether this node hosts a local VPK service.
    /// When true, KwaaiNet polls the VPK health endpoint and advertises
    /// capability on the DHT. Defaults to false (opt-in).
    #[serde(default)]
    pub vpk_enabled: bool,

    /// VPK operating mode: "bob" (query-only), "eve" (storage), or "both".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpk_mode: Option<String>,

    /// Local port for the VPK health-check and REST API (default: 7432).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpk_local_port: Option<u16>,

    // ── Ollama supervision ────────────────────────────────────────────────────
    /// When true, kwaainet supervises Ollama: health-checks every 15 s and
    /// optionally spawns `ollama serve` on failure (if ollama is in PATH).
    /// Set via `kwaainet config set ollama_manage true`.
    #[serde(default)]
    pub ollama_manage: bool,

    /// Local port Ollama listens on (default: 11434).
    /// Used by the Ollama health watcher and ollama-proxy handler.
    #[serde(
        default = "default_ollama_port",
        skip_serializing_if = "is_default_ollama_port"
    )]
    pub ollama_port: u16,

    // ── Storage fabric (Eve role) ──────────────────────────────────────────────
    /// Local storage configuration for Eve role (multi-tenant vector DB).
    /// Set by `kwaainet storage init --pg-url <DSN>`. When present,
    /// `kwaainet start` also manages the storage API process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<StorageConfig>,

    /// Minimum number of independent IDENTIFY responses that must report the
    /// same address before it is accepted as confirmed (default: 2).
    #[serde(default = "default_identify_min_confirmations")]
    pub identify_min_confirmations: usize,

    /// How long to poll IDENTIFY for address confirmations, in seconds (default: 10).
    #[serde(default = "default_identify_timeout_secs")]
    pub identify_timeout_secs: u64,

    // ── Block rebalancing ─────────────────────────────────────────────────────
    /// Enable periodic block rebalancing (only active with `shard serve --auto`).
    /// When true, the shard server periodically checks DHT coverage and moves
    /// its blocks to fill gaps if its current range is well-covered by others.
    #[serde(default)]
    pub auto_rebalance: bool,

    /// How often to check coverage and potentially rebalance (seconds).
    #[serde(default = "default_rebalance_interval")]
    pub rebalance_interval_secs: u64,

    /// Minimum number of OTHER nodes that must cover our range before we will
    /// consider moving. Prevents moving when we are the sole coverage.
    #[serde(default = "default_rebalance_min_redundancy")]
    pub rebalance_min_redundancy: usize,

    // ── Peer reputation ──────────────────────────────────────────────────────
    /// Local peer reputation and trust scoring configuration.
    #[serde(default, skip_serializing_if = "reputation_config_is_default")]
    pub reputation: ReputationConfig,

    // ── Contribution policy ───────────────────────────────────────────────────
    /// Whether to auto-start storage and shard serving when the daemon starts.
    /// Defaults to true for both (opt-out model for insider builds).
    #[serde(default, skip_serializing_if = "contribute_config_is_default")]
    pub contribute: ContributeConfig,

    // ── RAG (Bob role) ────────────────────────────────────────────────────────
    /// Named RAG knowledge bases. Key = KB name (e.g. "default", "work", "research").
    /// Use `kwaainet rag init --name <name>` to create additional KBs.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub rag_kbs: std::collections::HashMap<String, RagConfig>,

    /// Legacy single-KB config — kept for deserialization only; migrated to rag_kbs on first save.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rag: Option<RagConfig>,
}

fn reputation_config_is_default(r: &ReputationConfig) -> bool {
    r.enabled && r.max_observations_per_peer == 100
}

// ---------------------------------------------------------------------------
// Contribute config
// ---------------------------------------------------------------------------

/// Controls whether this node automatically contributes storage and shard
/// serving when the daemon starts. Opt-out via `--no-contribute` or config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributeConfig {
    /// Automatically start storage serving on daemon start (when storage is initialised).
    #[serde(default = "default_true")]
    pub storage: bool,

    /// Automatically start **experimental** block-shard serving on daemon start.
    ///
    /// **Opt-in, and deliberately three-state.** `Some(true)` serves blocks,
    /// `Some(false)` is an explicit refusal, and `None` means the operator has
    /// never chosen — which now resolves to [`DEFAULT_CONTRIBUTE_SHARDS`],
    /// i.e. off.
    ///
    /// This used to default to on. Block sharding is experimental: it does not
    /// work on Apple Silicon at all (#117), where a node serving blocks runs
    /// ~20x slower than the same machine serving whole models through Ollama —
    /// so the opt-out default enrolled every Mac into the one path that makes
    /// the network worse. Turning contribution *off* by default would be the
    /// wrong reading of this change: whole-model serving over
    /// `/kwaai/ollama-proxy/1.0.0` is registered by the node itself and is
    /// unaffected, so a node with `shards: None` still contributes inference.
    ///
    /// It is an `Option` for the same reason `native_p2p` is: flipping a plain
    /// `bool` default cannot distinguish "never chosen" from "explicitly set",
    /// so it would silently override operators who deliberately asked for
    /// sharding. Read it through [`ContributeConfig::shards`], never directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<bool>,

    /// Automatically install updates when a new version is available (pre-v1.0 default: true).
    #[serde(default = "default_true")]
    pub auto_update: bool,
}

/// Block-shard serving is opt-in: experimental, and actively harmful on Metal.
pub const DEFAULT_CONTRIBUTE_SHARDS: bool = false;

impl ContributeConfig {
    /// Whether to serve block shards — the explicit choice if there is one,
    /// otherwise [`DEFAULT_CONTRIBUTE_SHARDS`].
    pub fn shards(&self) -> bool {
        self.shards.unwrap_or(DEFAULT_CONTRIBUTE_SHARDS)
    }

    /// True when the operator has said nothing about shard serving, so the
    /// opt-in default applies. Used to explain the change once at startup
    /// rather than silently dropping a node's block contribution.
    pub fn shards_unset(&self) -> bool {
        self.shards.is_none()
    }
}

impl Default for ContributeConfig {
    fn default() -> Self {
        Self {
            storage: true,
            shards: None,
            auto_update: true,
        }
    }
}

fn contribute_config_is_default(c: &ContributeConfig) -> bool {
    c.storage && c.shards.is_none() && c.auto_update
}

/// Resolved contribution policy after applying CLI overrides.
pub struct ContributePolicy {
    pub storage: bool,
    pub shards: bool,
    pub auto_update: bool,
}

impl ContributePolicy {
    /// Whether to start block-shard serving, given an explicit `--shard` flag.
    ///
    /// `--shard` is itself an opt-in and must win over an unset config. While
    /// `contribute.shards` defaulted to true this distinction did not matter,
    /// because `self.shards` was already true whenever the flag was plausible.
    /// With shard serving opt-in it matters a great deal: without it,
    /// `kwaainet start --daemon --shard` refuses and advises the operator to
    /// set the config key for the thing they just asked for.
    ///
    /// `--no-contribute` still outranks both — it is the bigger hammer.
    pub fn serve_shards(&self, shard_flag: bool, no_contribute: bool) -> bool {
        self.shards || (shard_flag && !no_contribute)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_api_endpoint")]
    pub api_endpoint: String,

    #[serde(default = "default_check_interval")]
    pub check_interval: u64,

    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,

    #[serde(default = "default_failure_threshold")]
    pub failure_threshold: u32,

    #[serde(default)]
    pub reconnection: ReconnectionConfig,

    #[serde(default)]
    pub alerting: AlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconnectionConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_max_attempts")]
    pub max_attempts: u32,

    #[serde(default = "default_backoff_strategy")]
    pub backoff_strategy: String,

    #[serde(default = "default_initial_delay")]
    pub initial_delay: u64,

    #[serde(default = "default_max_delay")]
    pub max_delay: u64,

    #[serde(default = "default_backoff_multiplier")]
    pub backoff_multiplier: f64,

    #[serde(default = "default_true")]
    pub jitter: bool,

    #[serde(default = "default_jitter_factor")]
    pub jitter_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub on_disconnect: bool,

    #[serde(default = "default_true")]
    pub on_reconnect: bool,

    #[serde(default = "default_true")]
    pub on_critical: bool,

    #[serde(default)]
    pub webhook_url: Option<String>,

    #[serde(default)]
    pub email: Option<String>,
}

// ---------------------------------------------------------------------------
// Reputation config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationConfig {
    /// Enable local peer reputation tracking.
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Maximum number of observations stored per peer (ring buffer).
    #[serde(default = "default_max_observations")]
    pub max_observations_per_peer: usize,
}

fn default_max_observations() -> usize {
    100
}

impl Default for ReputationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_observations_per_peer: default_max_observations(),
        }
    }
}

// RAG config (Bob role)
// ---------------------------------------------------------------------------

/// Configuration for the local RAG knowledge base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagConfig {
    /// UUID of the Eve tenant that holds the vector index for this KB.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    /// Peer ID of the Eve node this KB is stored on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eve_peer_id: Option<String>,

    /// Ollama embedding model.
    #[serde(default = "default_embed_model")]
    pub embed_model: String,

    /// Dimensionality of the embedding vectors (determined at `rag init` time).
    #[serde(default = "default_embed_dim")]
    pub embed_dim: usize,

    /// Base URL of the shard inference API.
    #[serde(default = "default_inference_url")]
    pub inference_url: String,

    /// Base URL for embedding requests. Accepts `http://...` or `p2p://PEER_ID`.
    /// Defaults to `inference_url` when absent (same host for both).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,

    /// Number of context chunks to inject per request.
    #[serde(default = "default_top_k")]
    pub top_k: usize,

    /// When Eve is the local node, store its HTTP base URL here (e.g. "http://localhost:7432")
    /// so commands bypass the P2P dial-to-self restriction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_url: Option<String>,

    /// Directory for chunk metadata (text, doc index, sync state).
    /// Defaults to ~/.kwaainet/rag/. Set to an external drive path for large corpora.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rag_data_dir: Option<String>,
}

fn default_embed_model() -> String {
    "nomic-embed-text".to_string()
}

fn default_embed_dim() -> usize {
    768
}

fn default_inference_url() -> String {
    "http://localhost:11434".to_string()
}

fn is_default_inference_url(s: &str) -> bool {
    s == default_inference_url()
}

fn default_top_k() -> usize {
    5
}

impl RagConfig {
    /// Resolve the chunk-metadata directory: explicit config or ~/.kwaainet/rag/<name>.
    /// For the "default" KB the path is ~/.kwaainet/rag/ (backward-compatible).
    pub fn data_dir(&self) -> std::path::PathBuf {
        match &self.rag_data_dir {
            Some(p) => std::path::PathBuf::from(p),
            None => kwaainet_dir().join("rag"),
        }
    }

    /// Return the data dir scoped to a KB name (used by rag init for non-default KBs).
    pub fn default_data_dir_for(name: &str) -> std::path::PathBuf {
        if name == "default" {
            kwaainet_dir().join("rag")
        } else {
            kwaainet_dir().join("rag").join(name)
        }
    }
}

impl Default for RagConfig {
    fn default() -> Self {
        Self {
            tenant_id: None,
            eve_peer_id: None,
            embed_model: default_embed_model(),
            embed_dim: default_embed_dim(),
            inference_url: default_inference_url(),
            top_k: default_top_k(),
            embed_url: None,
            storage_url: None,
            rag_data_dir: None,
        }
    }
}

// Storage config (Eve role)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Directory where the embedded store (redb + HNSW indices) lives.
    /// Defaults to ~/.kwaainet/storage/.
    #[serde(default = "default_storage_dir")]
    pub data_dir: String,

    /// Maximum storage capacity to offer (GB).
    #[serde(default = "default_capacity_gb")]
    pub capacity_gb: f64,

    // Legacy fields — ignored on read, never written.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _legacy_pg_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _legacy_data_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _legacy_pg_port: Option<u16>,
}

fn default_storage_dir() -> String {
    kwaainet_dir()
        .join("storage")
        .to_string_lossy()
        .into_owned()
}

fn default_capacity_gb() -> f64 {
    5.0
}

// ---------------------------------------------------------------------------
// Default value functions (required by serde)
// ---------------------------------------------------------------------------

fn default_model() -> String {
    std::env::var("KWAAINET_MODEL").unwrap_or_else(|_| "unsloth/Llama-3.1-8B-Instruct".to_string())
}
fn default_blocks() -> u32 {
    std::env::var("KWAAINET_BLOCKS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8)
}
fn default_port() -> u16 {
    std::env::var("KWAAINET_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080)
}
fn default_true() -> bool {
    true
}

/// Returns true when the running binary is a pre-v1.0 build (major version == 0).
pub fn is_pre_release() -> bool {
    crate::updater::CURRENT_VERSION
        .split('.')
        .next()
        .and_then(|major| major.parse::<u32>().ok())
        .map(|major| major == 0)
        .unwrap_or(true)
}
fn default_log_level() -> String {
    std::env::var("KWAAINET_LOG_LEVEL").unwrap_or_else(|_| "info".to_string())
}
fn default_peers() -> Vec<String> {
    vec![
        "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
            .to_string(),
        "/dns/bootstrap-2.kwaai.ai/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY"
            .to_string(),
    ]
}
/// No trusted relays by default — candidates come from identify hop discovery;
/// see the `trusted_relays` field.
fn default_trusted_relays() -> Vec<String> {
    Vec::new()
}
fn default_force_private() -> bool {
    true
}
fn default_enable_upnp() -> bool {
    true
}
fn default_api_endpoint() -> String {
    "https://map.kwaai.ai/api/v1/state".to_string()
}
fn default_ollama_port() -> u16 {
    11434
}
fn is_default_ollama_port(p: &u16) -> bool {
    *p == 11434
}
fn default_check_interval() -> u64 {
    60
}
fn default_request_timeout() -> u64 {
    10
}
fn default_failure_threshold() -> u32 {
    3
}
fn default_max_attempts() -> u32 {
    10
}
fn default_backoff_strategy() -> String {
    "exponential".to_string()
}
fn default_initial_delay() -> u64 {
    30
}
fn default_max_delay() -> u64 {
    1800
}
fn default_backoff_multiplier() -> f64 {
    2.0
}
fn default_jitter_factor() -> f64 {
    0.5
}
fn default_identify_min_confirmations() -> usize {
    2
}
fn default_identify_timeout_secs() -> u64 {
    45
}
fn default_rebalance_interval() -> u64 {
    300
}
fn default_rebalance_min_redundancy() -> usize {
    1
}

impl Default for KwaaiNetConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            blocks: default_blocks(),
            start_block: None,
            start_block_auto: false,
            port: default_port(),
            use_gpu: true,
            log_level: default_log_level(),
            inference_url: default_inference_url(),
            public_name: Some(format!(
                "{}-{}-{}",
                std::env::var("USER").unwrap_or_else(|_| "anonymous".to_string()),
                std::env::consts::OS,
                std::env::consts::ARCH,
            )),
            public_ip: None,
            public_port: None,
            announce_addr: None,
            identity_key: None,
            no_relay: false,
            initial_peers: default_peers(),
            trusted_relays: default_trusted_relays(),
            force_private: default_force_private(),
            native_p2p: None,
            enable_upnp: default_enable_upnp(),
            announce_self: true,
            health_monitoring: HealthConfig::default(),
            model_dht_prefix: None,
            model_repository: None,
            ollama_manage: false,
            ollama_port: default_ollama_port(),
            vpk_enabled: false,
            vpk_mode: None,
            vpk_local_port: None,
            storage: None,
            identify_min_confirmations: default_identify_min_confirmations(),
            identify_timeout_secs: default_identify_timeout_secs(),
            auto_rebalance: false,
            rebalance_interval_secs: default_rebalance_interval(),
            rebalance_min_redundancy: default_rebalance_min_redundancy(),
            reputation: ReputationConfig::default(),
            contribute: ContributeConfig::default(),
            rag_kbs: std::collections::HashMap::new(),
            rag: None,
        }
    }
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_endpoint: default_api_endpoint(),
            check_interval: default_check_interval(),
            request_timeout: default_request_timeout(),
            failure_threshold: default_failure_threshold(),
            reconnection: ReconnectionConfig::default(),
            alerting: AlertingConfig::default(),
        }
    }
}

impl Default for ReconnectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: default_max_attempts(),
            backoff_strategy: default_backoff_strategy(),
            initial_delay: default_initial_delay(),
            max_delay: default_max_delay(),
            backoff_multiplier: default_backoff_multiplier(),
            jitter: true,
            jitter_factor: default_jitter_factor(),
        }
    }
}

// ---------------------------------------------------------------------------
// Load / save
// ---------------------------------------------------------------------------

/// What `native_p2p` means when the user has never chosen.
///
/// **This is the 0.6 cutover, flipped in v0.6.0.** Every node that has not
/// explicitly opted out runs the native stack; nodes with `native_p2p: false`
/// written in their config keep the p2pd path regardless. No config file is
/// rewritten, and rolling back is the same one-line change.
pub const DEFAULT_NATIVE_P2P: bool = true;

impl KwaaiNetConfig {
    /// The block this node starts serving at — the pinned value if there is
    /// one, otherwise 0.
    pub fn start_block(&self) -> u32 {
        self.start_block.unwrap_or(0)
    }

    /// True when a range has been pinned, by the operator or by a previous
    /// auto-assignment. `shard serve` must not reassign a pinned node: doing so
    /// was #116, which silently overwrote an operator's configured range.
    pub fn start_block_pinned(&self) -> bool {
        self.start_block.is_some()
    }

    /// True only when the *operator* pinned the range — not when it was
    /// recorded by auto-assignment.
    ///
    /// This is the rebalancer's gate. [`Self::start_block_pinned`] answers a
    /// different question ("is there a range to reuse at startup?") and
    /// deliberately treats both sources alike; using it here would stop
    /// rebalancing any node that had ever auto-assigned, since #124 records
    /// every assignment.
    pub fn start_block_user_pinned(&self) -> bool {
        self.start_block.is_some() && !self.start_block_auto
    }

    /// Whether to run the native stack: the explicit choice if there is one,
    /// otherwise [`DEFAULT_NATIVE_P2P`].
    ///
    /// Always read the flag through this — `self.native_p2p` is three-state and
    /// `Some(false)` must keep beating a flipped default.
    pub fn native_p2p(&self) -> bool {
        self.native_p2p.unwrap_or(DEFAULT_NATIVE_P2P)
    }

    /// True when the user has explicitly opted out of the native stack, as
    /// opposed to simply never having chosen. Callers that want to say
    /// something about the cutover need the distinction; callers that just want
    /// the behaviour want [`Self::native_p2p`].
    pub fn opted_out_of_native_p2p(&self) -> bool {
        self.native_p2p == Some(false)
    }

    /// The `state` field for a DHT announcement: `2` ONLINE, `0` JOINING.
    ///
    /// ONLINE means "this node will serve inference", not merely "this node is
    /// up" — `shard run` filters on `state == 2`, so a node that claims it
    /// without a loaded shard gets dialled and fails the session with
    /// "protocols not supported". Hence the gate on `shard_is_ready()`.
    pub fn announce_state() -> i32 {
        if ShardManager::shard_is_ready() {
            2
        } else {
            0
        }
    }

    /// Re-read config.yaml before a save so writes by other processes are kept;
    /// falls back to `self` if the file cannot be read.
    pub fn reloaded(&self) -> Self {
        Self::load_or_create().unwrap_or_else(|_| self.clone())
    }

    /// Load config from `~/.kwaainet/config.yaml`, creating it with defaults if absent.
    pub fn load_or_create() -> Result<Self> {
        let cfg_file = config_file();
        std::fs::create_dir_all(cfg_file.parent().unwrap())?;

        if cfg_file.exists() {
            let text = std::fs::read_to_string(&cfg_file)
                .with_context(|| format!("reading {}", cfg_file.display()))?;
            let mut cfg: KwaaiNetConfig = serde_yaml::from_str(&text)
                .with_context(|| format!("parsing {}", cfg_file.display()))?;
            // Map-derived fields are only valid for the model that was active when
            // the map was consulted. If the configured model is an explicit HF path,
            // clear them so node.rs derives the correct values from the model name.
            if cfg.model.contains('/') {
                cfg.model_dht_prefix = None;
                cfg.model_repository = None;
            }
            debug!("Loaded config from {}", cfg_file.display());
            Ok(cfg)
        } else {
            let cfg = KwaaiNetConfig::default();
            cfg.save()?;
            info!("Created default config at {}", cfg_file.display());
            Ok(cfg)
        }
    }

    /// Persist the current config to disk, migrating legacy `rag:` to `rag_kbs` first.
    pub fn save(&self) -> Result<()> {
        let cfg_file = config_file();
        std::fs::create_dir_all(cfg_file.parent().unwrap())?;
        // Migrate legacy single-KB entry to rag_kbs before serializing.
        let mut out = self.clone();
        if let Some(legacy) = out.rag.take() {
            out.rag_kbs.entry("default".to_string()).or_insert(legacy);
        }
        let text = serde_yaml::to_string(&out).context("serializing config")?;
        std::fs::write(&cfg_file, text)
            .with_context(|| format!("writing {}", cfg_file.display()))?;
        debug!("Saved config to {}", cfg_file.display());
        Ok(())
    }

    /// Get the RAG config for the given KB name, falling back to the legacy `rag` field.
    pub fn get_rag_kb(&self, name: &str) -> Option<&RagConfig> {
        if let Some(kb) = self.rag_kbs.get(name) {
            return Some(kb);
        }
        // Legacy compat: single-KB config before named KBs were introduced.
        if name == "default" {
            return self.rag.as_ref();
        }
        None
    }

    /// Set (insert or replace) the RAG config for a named KB.
    pub fn set_rag_kb(&mut self, name: &str, cfg: RagConfig) {
        // Ensure legacy field is cleared — everything lives in rag_kbs now.
        if name == "default" {
            self.rag = None;
        }
        self.rag_kbs.insert(name.to_string(), cfg);
    }

    /// Remove a KB by name. Returns the removed config if it existed.
    pub fn remove_rag_kb(&mut self, name: &str) -> Option<RagConfig> {
        let from_map = self.rag_kbs.remove(name);
        if name == "default" && from_map.is_none() {
            return self.rag.take();
        }
        from_map
    }

    /// List all KB names, including the legacy "default" entry if present.
    pub fn rag_kb_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.rag_kbs.keys().cloned().collect();
        if self.rag.is_some() && !names.contains(&"default".to_string()) {
            names.insert(0, "default".to_string());
        }
        names.sort();
        names
    }

    /// Return the effective DHT prefix for this node's model.
    ///
    /// Uses the canonical prefix set by the map API when available.
    /// Falls back to deriving it from the model name using Petals conventions:
    /// `"org/Model-Name.1B"` → `"Model-Name-1B"` (basename only, dots to dashes).
    ///
    /// This is the single source of truth — both `node.rs` and `shard_cmd.rs`
    /// call this so they always agree on the DHT key.
    pub fn effective_dht_prefix(&self) -> String {
        if let Some(ref p) = self.model_dht_prefix {
            return p.clone();
        }
        let base = self.model.split('/').next_back().unwrap_or(&self.model);
        base.replace('.', "-")
    }

    /// Total transformer blocks in the full model.
    ///
    /// Reads `num_hidden_layers` from the model's `config.json` when the
    /// snapshot is available locally. Falls back to a name-based heuristic
    /// (32 / 40 / 80) when the model has not been downloaded yet.
    pub fn model_total_blocks(&self) -> i32 {
        if let Ok(model_dir) = crate::hf::resolve_snapshot(&self.model) {
            let config_path = model_dir.join("config.json");
            if let Ok(s) = std::fs::read_to_string(&config_path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                    if let Some(n) = v["num_hidden_layers"].as_i64() {
                        return n as i32;
                    }
                }
            }
        }
        // Fallback: name heuristic when model is not yet downloaded.
        let m = self.model.to_lowercase();
        if m.contains("70b") {
            80
        } else if m.contains("13b") {
            40
        } else {
            32
        }
    }

    /// Effective last block (exclusive) this node serves, clamped to the
    /// total number of transformer blocks in the model.
    ///
    /// Prevents `end_block = start_block + blocks` from exceeding the model
    /// size when the operator sets a large `blocks` value.
    pub fn effective_end_block(&self) -> u32 {
        let total = self.model_total_blocks() as u32;
        (self.start_block() + self.blocks).min(total)
    }

    /// Resolve the effective contribution policy, honouring the CLI override.
    pub fn contribute_policy(&self, cli_no_contribute: bool) -> ContributePolicy {
        ContributePolicy {
            storage: self.contribute.storage && !cli_no_contribute,
            shards: self.contribute.shards() && !cli_no_contribute,
            auto_update: self.contribute.auto_update && is_pre_release(),
        }
    }

    /// Set a top-level key by name (string value coerced to the right type).
    ///
    /// **Mutates only — the caller persists.** This used to end in `save()`,
    /// which made a setter perform disk I/O: every caller wrote the whole
    /// config to `~/.kwaainet/config.yaml` as a side effect of assigning one
    /// field. Unit tests calling `set_key` on a `KwaaiNetConfig::default()`
    /// therefore serialised an empty config over the developer's real one,
    /// destroying every key a default does not carry — on one machine, 23
    /// registered knowledge bases, `inference_url`, the storage block and the
    /// vpk settings.
    ///
    /// Separating the two also lets a caller validate several keys before
    /// committing any of them, which the implicit save made impossible.
    pub fn set_key(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "model" => self.model = value.to_string(),
            "blocks" => self.blocks = value.parse().context("blocks must be a number")?,
            "port" => self.port = value.parse().context("port must be a number")?,
            "use_gpu" => self.use_gpu = parse_bool(value)?,
            "log_level" => self.log_level = value.to_string(),
            "public_name" => self.public_name = Some(value.to_string()),
            "public_ip" => self.public_ip = Some(value.to_string()),
            "public_port" => {
                self.public_port = Some(value.parse().map_err(|_| {
                    anyhow::anyhow!("public_port must be a positive integer between 1 and 65535")
                })?)
            }
            "announce_addr" => self.announce_addr = Some(value.to_string()),
            "no_relay" => self.no_relay = parse_bool(value)?,
            "native_p2p" => self.native_p2p = Some(parse_bool(value)?),
            "announce_self" => self.announce_self = parse_bool(value)?,
            "enable_upnp" => self.enable_upnp = parse_bool(value)?,
            "start_block" => {
                self.start_block =
                    Some(value.parse().map_err(|_| {
                        anyhow::anyhow!("start_block must be a non-negative integer")
                    })?);
                // Set by hand, so the rebalancer must not move this node.
                self.start_block_auto = false;
            }
            "auto_rebalance" => self.auto_rebalance = parse_bool(value)?,
            "rebalance_interval_secs" => {
                self.rebalance_interval_secs = value.parse().map_err(|_| {
                    anyhow::anyhow!("rebalance_interval_secs must be a positive integer")
                })?
            }
            "rebalance_min_redundancy" => {
                self.rebalance_min_redundancy = value.parse().map_err(|_| {
                    anyhow::anyhow!("rebalance_min_redundancy must be a positive integer")
                })?
            }
            "inference_url" => self.inference_url = value.to_string(),
            "contribute.storage" => self.contribute.storage = parse_bool(value)?,
            "contribute.shards" => self.contribute.shards = Some(parse_bool(value)?),
            "contribute.auto_update" => self.contribute.auto_update = parse_bool(value)?,
            "identify_min_confirmations" => {
                self.identify_min_confirmations = value.parse().map_err(|_| {
                    anyhow::anyhow!("identify_min_confirmations must be a positive integer")
                })?
            }
            "identify_timeout_secs" => {
                self.identify_timeout_secs = value.parse().map_err(|_| {
                    anyhow::anyhow!("identify_timeout_secs must be a positive integer")
                })?
            }
            _ => anyhow::bail!(
                "Unknown config key '{}'. Run `kwaainet config set --help` to see valid keys.",
                key
            ),
        }
        Ok(())
    }
}

fn parse_bool(s: &str) -> Result<bool> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => anyhow::bail!("Expected true/false, got {}", s),
    }
}

mod dirs_sys {
    use std::path::PathBuf;
    pub fn home_dir() -> Option<PathBuf> {
        dirs::home_dir()
    }
}

#[cfg(test)]
mod start_block_pinning {
    use super::*;

    // #116: a node with a configured range was reassigned anyway, and its
    // config overwritten, because the auto-assign condition consulted only the
    // CLI flag. `0` is a legitimate pinned value, so the field has to be
    // three-state for "configured 0" to differ from "never set".

    #[test]
    fn a_fresh_config_pins_nothing() {
        let cfg = KwaaiNetConfig::default();
        assert_eq!(cfg.start_block, None);
        assert!(!cfg.start_block_pinned(), "a fresh node may auto-assign");
        assert_eq!(cfg.start_block(), 0, "but still reads as 0");
    }

    #[test]
    fn zero_is_a_real_pin_not_an_absence() {
        // The heart of #116. Serving from block 0 is the most common explicit
        // choice — a full-model node — and it must not read as "unset".
        let cfg = KwaaiNetConfig {
            start_block: Some(0),
            ..Default::default()
        };
        assert!(
            cfg.start_block_pinned(),
            "start_block: 0 is a choice, not a default"
        );
        assert_eq!(cfg.start_block(), 0);
    }

    #[test]
    fn set_records_a_pin() {
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("start_block", "0").expect("set");
        assert_eq!(
            cfg.start_block,
            Some(0),
            "`config set start_block 0` must pin"
        );
        cfg.set_key("start_block", "8").expect("set");
        assert_eq!(cfg.start_block, Some(8));
    }

    #[test]
    fn an_unpinned_node_is_not_serialised() {
        // Otherwise merely starting once would pin a range nobody chose.
        let cfg = KwaaiNetConfig::default();
        let y = serde_yaml::to_string(&cfg).expect("serialise");
        assert!(
            !y.contains("start_block"),
            "an unmade choice must not be written:\n{y}"
        );
    }

    #[test]
    fn a_pin_round_trips_through_yaml() {
        let cfg = KwaaiNetConfig {
            start_block: Some(0),
            blocks: 32,
            ..Default::default()
        };
        let y = serde_yaml::to_string(&cfg).expect("serialise");
        assert!(y.contains("start_block: 0"), "a pin must persist:\n{y}");
        let back: KwaaiNetConfig = serde_yaml::from_str(&y).expect("round trip");
        assert_eq!(back.start_block, Some(0));
        assert!(back.start_block_pinned());
    }

    #[test]
    fn end_block_is_computed_from_the_effective_start() {
        let cfg = KwaaiNetConfig {
            start_block: Some(8),
            blocks: 8,
            ..Default::default()
        };
        assert_eq!(cfg.effective_end_block(), 16);
        let unpinned = KwaaiNetConfig {
            start_block: None,
            blocks: 8,
            ..Default::default()
        };
        assert_eq!(unpinned.effective_end_block(), 8, "unset starts at 0");
    }
}

#[cfg(test)]
mod native_p2p_tri_state {
    use super::*;

    // The whole reason the field is an `Option`: the cutover must be able to
    // flip the default without overriding anyone who already said no.

    #[test]
    fn unset_takes_the_default() {
        let cfg = KwaaiNetConfig::default();
        assert_eq!(cfg.native_p2p, None, "a fresh config records no choice");
        assert_eq!(cfg.native_p2p(), DEFAULT_NATIVE_P2P);
    }

    #[test]
    fn an_explicit_choice_beats_the_default_both_ways() {
        let on = KwaaiNetConfig {
            native_p2p: Some(true),
            ..Default::default()
        };
        assert!(on.native_p2p());
        let off = KwaaiNetConfig {
            native_p2p: Some(false),
            ..Default::default()
        };
        assert!(!off.native_p2p());
    }

    #[test]
    fn opting_out_survives_a_flipped_default() {
        // Simulates the cutover: whatever DEFAULT_NATIVE_P2P becomes, an
        // explicit `false` must still mean the p2pd path. Expressed without
        // reading the constant so the test keeps its meaning after the flip.
        let off = KwaaiNetConfig {
            native_p2p: Some(false),
            ..Default::default()
        };
        assert!(!off.native_p2p(), "an explicit opt-out is never overridden");
        assert!(off.opted_out_of_native_p2p());

        let unset = KwaaiNetConfig {
            native_p2p: None,
            ..Default::default()
        };
        assert!(
            !unset.opted_out_of_native_p2p(),
            "never having chosen is not the same as opting out"
        );
    }

    #[test]
    fn set_records_an_explicit_choice() {
        // `config set native_p2p false` must write `Some(false)`, not leave the
        // field unset — otherwise the opt-out evaporates at the cutover.
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("native_p2p", "false").expect("set");
        assert_eq!(cfg.native_p2p, Some(false));
        cfg.set_key("native_p2p", "true").expect("set");
        assert_eq!(cfg.native_p2p, Some(true));
    }

    #[test]
    fn an_unset_flag_is_not_serialised() {
        // Absent must stay absent on save, or every node that has merely
        // started once would acquire a pinned choice it never made.
        let cfg = KwaaiNetConfig::default();
        let y = serde_yaml::to_string(&cfg).expect("serialise");
        assert!(
            !y.contains("native_p2p"),
            "an unmade choice must not be written to disk:\n{y}"
        );
    }

    #[test]
    fn a_legacy_config_without_the_key_loads_as_unset() {
        // Configs written before the flag existed have no key at all.
        let y = "port: 8080\nnative_p2p_absent_marker: true\n";
        let cfg: KwaaiNetConfig = serde_yaml::from_str(y).expect("legacy config parses");
        assert_eq!(cfg.native_p2p, None);
        assert_eq!(cfg.native_p2p(), DEFAULT_NATIVE_P2P);
    }

    #[test]
    fn an_explicit_false_round_trips_through_yaml() {
        let cfg = KwaaiNetConfig {
            native_p2p: Some(false),
            ..Default::default()
        };
        let y = serde_yaml::to_string(&cfg).expect("serialise");
        assert!(
            y.contains("native_p2p: false"),
            "opt-out must persist:\n{y}"
        );
        let back: KwaaiNetConfig = serde_yaml::from_str(&y).expect("round trip");
        assert_eq!(back.native_p2p, Some(false));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(start: u32, blocks: u32, total_hint: &str) -> KwaaiNetConfig {
        KwaaiNetConfig {
            model: total_hint.to_string(), // name heuristic drives model_total_blocks()
            start_block: Some(start),
            blocks,
            ..KwaaiNetConfig::default()
        }
    }

    #[test]
    fn effective_end_block_no_clamp() {
        // 0 + 8 = 8 < 32 — no clamping needed
        let c = cfg(0, 8, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 8);
    }

    #[test]
    fn effective_end_block_clamps_to_model_total() {
        // 8 + 32 = 40, but model has 32 blocks → clamped to 32
        let c = cfg(8, 32, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 32);
    }

    #[test]
    fn effective_end_block_exact_fit() {
        // 0 + 32 = 32 == total — no clamping
        let c = cfg(0, 32, "unsloth/Llama-3-8B");
        assert_eq!(c.effective_end_block(), 32);
    }

    #[test]
    fn effective_end_block_70b_model() {
        // 70B has 80 blocks; 72 + 32 = 104 → clamped to 80
        let c = cfg(72, 32, "meta/Llama-2-70B");
        assert_eq!(c.effective_end_block(), 80);
    }

    // ── The bootstrap-node config keys ─────────────────────────────────────

    /// Both new keys default to ordinary-node behaviour, so an existing
    /// `config.yaml` written before they existed deserialises into a node that
    /// behaves exactly as it did.
    #[test]
    fn the_new_keys_default_to_ordinary_node_behaviour() {
        let c = KwaaiNetConfig::default();
        assert!(
            c.announce_self,
            "an ordinary node publishes its own records"
        );
        assert!(
            c.enable_upnp,
            "an ordinary node asks its gateway for a mapping"
        );
    }

    /// A config file predating these keys still loads, and the missing fields
    /// take the ordinary-node defaults rather than `bool::default()` — which
    /// for `announce_self` would be `false` and silently mute every existing
    /// node on the network, and for `enable_upnp` would drop the port mapping
    /// a NATed node depends on.
    #[test]
    fn a_config_without_the_new_keys_deserialises_to_node_defaults() {
        let c: KwaaiNetConfig =
            serde_yaml::from_str("model: unsloth/Llama-3-8B\nblocks: 8\n").expect("legacy config");
        assert!(
            c.announce_self,
            "a config written before announce_self existed must keep announcing"
        );
        assert!(c.enable_upnp, "likewise for enable_upnp");
    }

    /// `config set` round-trips each key through YAML, which is how an operator
    /// configures a bootstrap node; see `docs/BOOTSTRAP.md`.
    #[test]
    fn the_new_keys_round_trip_through_set_key_and_yaml() {
        // In memory, not via `save()`/`load_or_create()`: see `test_isolation`
        // for why a disk round-trip cannot be made reliable in this suite.
        let mut c = KwaaiNetConfig::default();
        c.set_key("announce_self", "false")
            .expect("announce_self is a valid key");
        c.set_key("enable_upnp", "false")
            .expect("enable_upnp is a valid key");

        let yaml = serde_yaml::to_string(&c).expect("serialise");
        let reloaded: KwaaiNetConfig = serde_yaml::from_str(&yaml).expect("reload");
        assert!(!reloaded.announce_self);
        assert!(!reloaded.enable_upnp);

        // And back again, so neither direction is a one-way door.
        let mut c = reloaded;
        c.set_key("announce_self", "true").expect("set back");
        let yaml = serde_yaml::to_string(&c).expect("serialise");
        let reloaded: KwaaiNetConfig = serde_yaml::from_str(&yaml).expect("reload");
        assert!(reloaded.announce_self);
    }

    /// A non-boolean value is rejected rather than coerced, so a typo'd
    /// `config set announce_self flase` fails instead of muting the node.
    #[test]
    fn a_non_boolean_value_is_rejected() {
        let mut c = KwaaiNetConfig::default();
        assert!(c.set_key("announce_self", "flase").is_err());
        assert!(
            c.announce_self,
            "a rejected set must not have changed the field"
        );
    }
}

/// Provenance of `start_block`: an auto-assigned range must not read as an
/// operator pin. Regression for the rebalance gate — see `should_rebalance`.
#[cfg(test)]
mod start_block_provenance {
    use super::*;

    #[test]
    fn auto_assigned_range_is_not_an_operator_pin() {
        let cfg = KwaaiNetConfig {
            start_block: Some(8),
            start_block_auto: true,
            ..Default::default()
        };
        assert!(
            cfg.start_block_pinned(),
            "a recorded range is still reused at startup (#124)"
        );
        assert!(
            !cfg.start_block_user_pinned(),
            "but the rebalancer may still move an auto-assigned node"
        );
    }

    #[test]
    fn operator_pin_blocks_the_rebalancer() {
        let cfg = KwaaiNetConfig {
            start_block: Some(8),
            start_block_auto: false,
            ..Default::default()
        };
        assert!(
            cfg.start_block_user_pinned(),
            "set by hand — do not move it"
        );
    }

    #[test]
    fn setting_start_block_by_hand_clears_the_auto_flag() {
        let mut cfg = KwaaiNetConfig {
            start_block: Some(8),
            start_block_auto: true,
            ..Default::default()
        };
        cfg.set_key("start_block", "16").expect("set");
        assert!(
            !cfg.start_block_auto,
            "`config set start_block` is an operator decision and outranks a \
             previously recorded auto-assignment"
        );
        assert!(cfg.start_block_user_pinned());
    }

    #[test]
    fn unset_start_block_is_neither() {
        let cfg = KwaaiNetConfig::default();
        assert!(!cfg.start_block_pinned());
        assert!(!cfg.start_block_user_pinned());
    }

    #[test]
    fn auto_flag_is_omitted_from_yaml_when_false() {
        let cfg = KwaaiNetConfig::default();
        let yaml = serde_yaml::to_string(&cfg).expect("serialize");
        assert!(
            !yaml.contains("start_block_auto"),
            "an internal provenance marker should not clutter a hand-edited \
             config until it is actually true"
        );
    }

    #[test]
    fn auto_flag_survives_a_round_trip() {
        let cfg = KwaaiNetConfig {
            start_block: Some(8),
            start_block_auto: true,
            ..Default::default()
        };
        let yaml = serde_yaml::to_string(&cfg).expect("serialize");
        let back: KwaaiNetConfig = serde_yaml::from_str(&yaml).expect("deserialize");
        assert!(
            back.start_block_auto,
            "provenance must persist, or every restart re-pins the node"
        );
    }

    #[test]
    fn a_config_written_before_this_field_existed_reads_as_an_operator_pin() {
        // Upgrade path: 0.6.1/0.6.2 wrote `start_block` with no provenance.
        // Defaulting to "operator pin" is the safe reading — it declines to
        // move a node rather than moving one that was placed deliberately.
        let cfg: KwaaiNetConfig =
            serde_yaml::from_str("start_block: 8\nblocks: 8\n").expect("deserialize");
        assert!(!cfg.start_block_auto);
        assert!(cfg.start_block_user_pinned());
    }
}

/// `cargo test` must not be able to touch the developer's real config.
#[cfg(test)]
mod test_isolation {
    use super::*;

    #[test]
    fn config_path_is_sandboxed_under_test() {
        // No KWAAINET_HOME is set in this process, so this is the path a test
        // that calls save() would write to.
        if std::env::var("KWAAINET_HOME").is_ok() {
            return; // explicitly directed elsewhere; nothing to prove
        }
        let real = dirs_home().join(".kwaainet");
        assert_ne!(
            kwaainet_dir(),
            real,
            "a test that calls save() would overwrite the real config"
        );
    }

    #[test]
    fn set_key_does_not_touch_the_disk() {
        // The hazard at its source: a setter must not perform I/O.
        if std::env::var("KWAAINET_HOME").is_ok() {
            return;
        }
        let path = config_file();
        let before = std::fs::read(&path).ok();
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("start_block", "16").expect("set");
        assert_eq!(
            std::fs::read(&path).ok(),
            before,
            "set_key wrote to disk; it must only mutate the struct"
        );
        assert_eq!(cfg.start_block, Some(16), "but it must still mutate");
    }

    // There is deliberately no test here that calls `save()` and then asserts
    // the file landed in the sandbox. `grpc_server`'s tests set `KWAAINET_HOME`
    // process-wide (guarded by a mutex private to that module, so it does not
    // serialise against this one), which means a concurrent `save()` can be
    // redirected into their temp dir between the write and the assertion. Such
    // a test passes alone and fails in the suite.
    //
    // `config_path_is_sandboxed_under_test` covers the guard without touching
    // the filesystem, and `set_key_does_not_touch_the_disk` above covers the
    // behaviour this module actually protects.
}

/// Block-shard serving is opt-in. Regression for the default flip.
#[cfg(test)]
mod contribute_shards_is_opt_in {
    use super::*;

    #[test]
    fn a_fresh_node_does_not_serve_blocks() {
        let cfg = KwaaiNetConfig::default();
        assert!(cfg.contribute.shards_unset(), "nothing chosen");
        assert!(
            !cfg.contribute.shards(),
            "sharding is experimental — a node must opt in"
        );
    }

    #[test]
    fn an_explicit_opt_in_is_honoured() {
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("contribute.shards", "true").expect("set");
        assert_eq!(cfg.contribute.shards, Some(true));
        assert!(cfg.contribute.shards());
        assert!(!cfg.contribute.shards_unset(), "the operator chose");
    }

    #[test]
    fn an_explicit_opt_out_is_distinguishable_from_unset() {
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("contribute.shards", "false").expect("set");
        assert_eq!(
            cfg.contribute.shards,
            Some(false),
            "an explicit no must not read as 'never chosen'"
        );
        assert!(!cfg.contribute.shards_unset());
    }

    #[test]
    fn a_config_written_before_the_flip_still_opts_in() {
        // Upgrade path: `contribute.shards: true` was written by an older
        // version. That is an explicit value and must keep serving blocks.
        let cfg: KwaaiNetConfig =
            serde_yaml::from_str("contribute:\n  shards: true\n").expect("deserialize");
        assert!(
            cfg.contribute.shards(),
            "an operator who asked for sharding keeps it across the flip"
        );
    }

    #[test]
    fn a_config_with_no_contribute_block_reads_as_opt_in_off() {
        let cfg: KwaaiNetConfig = serde_yaml::from_str("model: foo/bar\n").expect("deserialize");
        assert!(!cfg.contribute.shards());
        assert!(cfg.contribute.shards_unset());
    }

    #[test]
    fn storage_contribution_is_untouched_by_the_flip() {
        let cfg = KwaaiNetConfig::default();
        assert!(
            cfg.contribute.storage,
            "only shard serving became opt-in; storage stays opt-out"
        );
    }

    #[test]
    fn no_contribute_still_overrides_an_explicit_opt_in() {
        let mut cfg = KwaaiNetConfig::default();
        cfg.set_key("contribute.shards", "true").expect("set");
        let policy = cfg.contribute_policy(true);
        assert!(!policy.shards, "--no-contribute wins over config");
        assert!(!policy.storage);
    }

    #[test]
    fn unset_shards_is_omitted_from_yaml() {
        // `storage: false` forces the `contribute:` block to serialize at all —
        // a wholly default config is skipped by `contribute_config_is_default`,
        // so without this the assertion would hold even if the
        // `skip_serializing_if` on `shards` were missing, and prove nothing.
        let mut cfg = KwaaiNetConfig::default();
        cfg.contribute.storage = false;
        let yaml = serde_yaml::to_string(&cfg).expect("serialize");
        assert!(
            yaml.contains("contribute:"),
            "precondition: the contribute block must actually be serialized"
        );
        assert!(
            !yaml.contains("shards:"),
            "an unchosen value should not be written as though it were a decision"
        );
    }
}

/// `--shard` must win over an unset config. Regression for the opt-in flip.
#[cfg(test)]
mod shard_flag_is_an_opt_in {
    use super::*;

    fn policy(cfg_shards: Option<bool>, no_contribute: bool) -> ContributePolicy {
        let mut cfg = KwaaiNetConfig::default();
        cfg.contribute.shards = cfg_shards;
        cfg.contribute_policy(no_contribute)
    }

    #[test]
    fn the_flag_starts_shard_serving_on_an_unset_config() {
        let p = policy(None, false);
        assert!(!p.shards, "config alone would not serve");
        assert!(
            p.serve_shards(true, false),
            "`--daemon --shard` must serve; telling the user to set a config \
             key for what they just typed is not an acceptable answer"
        );
    }

    #[test]
    fn without_the_flag_an_unset_config_does_not_serve() {
        assert!(!policy(None, false).serve_shards(false, false));
    }

    #[test]
    fn config_opt_in_serves_without_the_flag() {
        assert!(policy(Some(true), false).serve_shards(false, false));
    }

    #[test]
    fn no_contribute_outranks_the_flag() {
        let p = policy(None, true);
        assert!(
            !p.serve_shards(true, true),
            "--no-contribute is the bigger hammer"
        );
    }

    #[test]
    fn no_contribute_outranks_a_config_opt_in_too() {
        assert!(!policy(Some(true), true).serve_shards(false, true));
    }

    #[test]
    fn the_flag_overrides_an_explicit_config_opt_out() {
        // Deliberate: a flag typed now is a later decision than a config file.
        let p = policy(Some(false), false);
        assert!(!p.shards);
        assert!(p.serve_shards(true, false));
    }
}
