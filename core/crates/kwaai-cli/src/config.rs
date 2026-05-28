//! Configuration management for KwaaiNet
//!
//! Config file lives at `~/.kwaainet/config.yaml`.
//! On first run a default config is written and returned.

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
    dirs_home().join(".kwaainet")
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

    /// First transformer block this node serves (0-indexed).
    /// The node serves blocks [start_block .. start_block + blocks).
    /// Defaults to 0 (start of model); set this on non-first nodes.
    #[serde(default)]
    pub start_block: u32,

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

    /// Multiaddrs of peers to use as trusted circuit relays (for AutoRelay).
    /// When set, AutoRelay reserves circuits with these peers instead of (or
    /// in addition to) discovering relays via the DHT. Useful when DHT
    /// discovery is unreliable (small networks, NAT-isolated test topologies)
    /// or when you want to force traffic through specific known-good relays.
    /// Empty means "let AutoRelay discover relays via DHT" (the default).
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

    /// Automatically start shard serving on daemon start (when a model is available).
    #[serde(default = "default_true")]
    pub shards: bool,

    /// Automatically install updates when a new version is available (pre-v1.0 default: true).
    #[serde(default = "default_true")]
    pub auto_update: bool,
}

impl Default for ContributeConfig {
    fn default() -> Self {
        Self {
            storage: true,
            shards: true,
            auto_update: true,
        }
    }
}

fn contribute_config_is_default(c: &ContributeConfig) -> bool {
    c.storage && c.shards && c.auto_update
}

/// Resolved contribution policy after applying CLI overrides.
pub struct ContributePolicy {
    pub storage: bool,
    pub shards: bool,
    pub auto_update: bool,
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
fn default_trusted_relays() -> Vec<String> {
    vec![
        "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
            .to_string(),
        "/dns/bootstrap-2.kwaai.ai/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY"
            .to_string(),
    ]
}
fn default_force_private() -> bool {
    true
}
fn default_api_endpoint() -> String {
    "https://map.kwaai.ai/api/v1/state".to_string()
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
            start_block: 0,
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
            health_monitoring: HealthConfig::default(),
            model_dht_prefix: None,
            model_repository: None,
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

impl KwaaiNetConfig {
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
        (self.start_block + self.blocks).min(total)
    }

    /// Resolve the effective contribution policy, honouring the CLI override.
    pub fn contribute_policy(&self, cli_no_contribute: bool) -> ContributePolicy {
        ContributePolicy {
            storage: self.contribute.storage && !cli_no_contribute,
            shards: self.contribute.shards && !cli_no_contribute,
            auto_update: self.contribute.auto_update && is_pre_release(),
        }
    }

    /// Set a top-level key by name (string value coerced to the right type).
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
            "start_block" => {
                self.start_block = value
                    .parse()
                    .map_err(|_| anyhow::anyhow!("start_block must be a non-negative integer"))?
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
            "contribute.shards" => self.contribute.shards = parse_bool(value)?,
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
        self.save()
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
mod tests {
    use super::*;

    fn cfg(start: u32, blocks: u32, total_hint: &str) -> KwaaiNetConfig {
        KwaaiNetConfig {
            model: total_hint.to_string(), // name heuristic drives model_total_blocks()
            start_block: start,
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
}
