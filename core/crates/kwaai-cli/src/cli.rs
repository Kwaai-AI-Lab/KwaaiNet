//! CLI argument definitions using clap

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kwaainet",
    about = "KwaaiNet – Distributed AI node CLI",
    long_about = "KwaaiNet — Sovereign AI Infrastructure

─── Install & first run ──────────────────────────────────────────────
  kwaainet setup                         create config dirs and identity
  kwaainet setup --get-deps              download p2pd (if not bundled)
  kwaainet benchmark                     measure GPU/CPU throughput

─── Join the network ─────────────────────────────────────────────────
  kwaainet config set public_name \"alice-m4\"   shown on map.kwaai.ai
  kwaainet start --daemon                       start node in background
  kwaainet status                               verify node is online
  kwaainet logs --follow                        tail the daemon log

─── Configuration ────────────────────────────────────────────────────
  kwaainet config                        show current config
  kwaainet config set KEY VALUE          update a value
  kwaainet config set blocks 8           transformer blocks to host
  kwaainet config set use_gpu true       enable GPU acceleration

─── Direct vs Relay connections ──────────────────────────────────────
  By default nodes connect via relay (no port forwarding required).
  For direct connections (lower latency, better throughput):
    kwaainet config set public_ip <YOUR_PUBLIC_IP>
    kwaainet config set announce_addr /ip4/<IP>/tcp/<PORT>
    • Forward the chosen TCP port in your router
    • Verify: kwaainet status  →  look for \"using_relay: false\"

─── Local inference (30+ tok/s on Apple Silicon) ────────────────────
  kwaainet shard run \"What is the capital of France?\" --local
  kwaainet benchmark                       measure local throughput

─── Distributed inference ────────────────────────────────────────────
  kwaainet shard circuit create            pre-form a peer path
  kwaainet shard run \"Hello\" --circuit ID  use the circuit
  kwaainet shard run \"Hello\" --stats       show per-token timing

  # Multi-machine — split the model
  #  Machine A                             Machine B
  shard serve --blocks 28                  shard serve --start-block 28 --blocks 4
  shard chain --total-blocks 32            # verify full coverage

─── Storage fabric (host encrypted vectors for the network) ─────────
  kwaainet storage init --capacity-gb 10  provision embedded vector store
  kwaainet storage status                 show store health and tenants
  kwaainet vpk status                     VPK health and DHT status
  kwaainet vpk discover                   find Eve nodes by PeerId via DHT

─── OpenAI-compatible API ────────────────────────────────────────────
  kwaainet shard api --port 8080
  curl http://localhost:8080/v1/chat/completions \\
    -d '{\"model\":\"default\",\"messages\":[{\"role\":\"user\",\"content\":\"Hello\"}]}'

Learn more: https://github.com/Kwaai-AI-Lab/KwaaiNet",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the KwaaiNet node
    Start(StartArgs),

    /// Stop the KwaaiNet daemon
    Stop,

    /// Restart the KwaaiNet daemon
    Restart,

    /// Show daemon status
    Status(StatusArgs),

    /// Show daemon logs
    Logs(LogsArgs),

    /// View or modify configuration
    Config(ConfigArgs),

    /// View health monitoring status
    HealthStatus,

    /// Enable health monitoring
    HealthEnable,

    /// Disable health monitoring
    HealthDisable,

    /// Manage the auto-start service
    Service(ServiceArgs),

    /// Force P2P network reconnection
    Reconnect,

    /// P2P connection monitoring
    Monitor(MonitorArgs),

    /// Check or install updates
    Update(UpdateArgs),

    /// Calibrate optimal block count for this hardware
    Calibrate(CalibrateArgs),

    /// Load and inspect a model from Ollama's local store
    LoadModel(LoadModelArgs),

    /// Generate text from a prompt (tokenizer smoke-test)
    Generate(GenerateArgs),

    /// Benchmark inference throughput and save to cache
    Benchmark(BenchmarkArgs),

    /// Serve an OpenAI-compatible API backed by the local model
    Serve(ServeArgs),

    /// Initial setup and dependency installation
    Setup(SetupArgs),

    /// Manage node identity and verifiable credentials
    Identity(IdentityArgs),

    /// Inspect live p2p state (identity, connected peers) via the local p2pd
    P2p(P2pArgs),

    /// Manage VPK (Virtual Private Knowledge) vector database integration
    Vpk(VpkArgs),

    /// View and manage the local peer reputation store (trust scores, observed performance)
    Reputation(ReputationArgs),

    /// Manage the local storage fabric (Eve role — host opaque vectors for Bob nodes on the network)
    Storage(StorageArgs),

    /// Uninstall KwaaiNet — stop the node, remove all data, and delete binaries
    Uninstall(UninstallArgs),

    /// Open the Node Dashboard (web UI). Run from repo root; requires Node.js.
    Ui,

    /// Distributed transformer block sharding
    #[command(long_about = "Distributed transformer block sharding (Petals-style)

Each machine loads a slice of the model and registers an RPC handler.
A coordinator discovers the chain via DHT and orchestrates inference hop-by-hop.

  shard serve     Load and serve a range of transformer blocks (run on each node)
  shard run       Coordinate inference across all serving nodes
  shard chain     Show block coverage across all online peers
  shard api       OpenAI-compatible HTTP server for distributed inference
  shard download  Download a HuggingFace SafeTensors model (no huggingface-cli needed)")]
    Shard(ShardArgs),

    /// Build and query a local RAG knowledge base
    Rag(RagArgs),

    /// Internal: run the node in the foreground (used by daemon mode)
    #[command(hide = true)]
    RunNode,
}

// ---------------------------------------------------------------------------
// start
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct StartArgs {
    /// Model to serve (e.g. unsloth/Llama-3.1-8B-Instruct)
    #[arg(long)]
    pub model: Option<String>,

    /// Number of transformer blocks to share
    #[arg(long)]
    pub blocks: Option<u32>,

    /// TCP port for P2P connections
    #[arg(long)]
    pub port: Option<u16>,

    /// Disable GPU acceleration (use CPU only)
    #[arg(long)]
    pub no_gpu: bool,

    /// Public display name for this node
    #[arg(long)]
    pub public_name: Option<String>,

    /// Override public IP address (auto-detected by default)
    #[arg(long)]
    pub public_ip: Option<String>,

    /// Custom announce multiaddr for P2P networking
    #[arg(long)]
    pub announce_addr: Option<String>,

    /// Path to a libp2p-protobuf-encoded identity key file.
    /// Overrides the default `~/.kwaainet/identity.key`. Used by bootstrap
    /// deployments to keep their existing RSA peer IDs across restarts.
    #[arg(long)]
    pub identity_key: Option<std::path::PathBuf>,

    /// Disable automatic relay
    #[arg(long)]
    pub no_relay: bool,

    /// Run in background (daemon mode)
    #[arg(long)]
    pub daemon: bool,

    /// Allow concurrent instances (don't stop existing processes)
    #[arg(long)]
    pub concurrent: bool,

    /// Also start the shard inference server in the background (auto-rebalancing)
    #[arg(long)]
    pub shard: bool,

    /// Disable automatic storage and shard serving (opt out of contributing)
    #[arg(long)]
    pub no_contribute: bool,
}

// ---------------------------------------------------------------------------
// logs
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct LogsArgs {
    /// Number of lines to show
    #[arg(long, short = 'n', default_value = "50")]
    pub lines: usize,

    /// Follow log output in real time
    #[arg(long, short = 'f')]
    pub follow: bool,

    /// Show shard server log instead of the node log
    #[arg(long)]
    pub shard: bool,

    /// Show storage API log instead of the node log
    #[arg(long)]
    pub storage: bool,
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct StatusArgs {
    /// Output machine-readable JSON
    #[arg(long)]
    pub json: bool,
}

// ---------------------------------------------------------------------------
// config
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ConfigArgs {
    /// Output machine-readable JSON (with config show)
    #[arg(long)]
    pub json: bool,
    #[command(subcommand)]
    pub action: Option<ConfigAction>,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration (default when no subcommand given)
    Show,
    /// Set a config value.
    ///
    /// Valid keys:
    ///   model, blocks, start_block, port, use_gpu, log_level,
    ///   public_name, public_ip, announce_addr, no_relay,
    ///   vpk_enabled, vpk_mode, vpk_local_port,
    ///   auto_rebalance, rebalance_interval_secs, rebalance_min_redundancy
    ///
    /// Example: kwaainet config set public_name "alice-m4"
    Set {
        /// Config key to set
        key: String,
        /// New value
        value: String,
    },
}

// ---------------------------------------------------------------------------
// service
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ServiceArgs {
    #[command(subcommand)]
    pub action: ServiceAction,
}

#[derive(Subcommand)]
pub enum ServiceAction {
    /// Install the auto-start service
    Install,
    /// Uninstall the auto-start service
    Uninstall,
    /// Show service status
    Status,
    /// Restart the auto-start service
    Restart,
}

// ---------------------------------------------------------------------------
// monitor
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct MonitorArgs {
    #[command(subcommand)]
    pub action: MonitorAction,
}

#[derive(Subcommand)]
pub enum MonitorAction {
    /// Show connection statistics
    Stats,
    /// Configure disconnect alerts
    Alert(AlertArgs),
}

#[derive(Args)]
pub struct AlertArgs {
    /// Enable alerts
    #[arg(long)]
    pub enable: bool,

    /// Disable alerts
    #[arg(long)]
    pub disable: bool,

    /// Alert after N minutes of disconnection
    #[arg(long, value_name = "MINUTES")]
    pub threshold: Option<u32>,

    /// Webhook URL for alerts
    #[arg(long, value_name = "URL")]
    pub webhook: Option<String>,

    /// Minimum connections before alerting
    #[arg(long)]
    pub min_connections: Option<u32>,
}

// ---------------------------------------------------------------------------
// update
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct UpdateArgs {
    /// Only check for updates, don't install
    #[arg(long)]
    pub check: bool,

    /// Force update check (bypass cache)
    #[arg(long)]
    pub force: bool,
}

// ---------------------------------------------------------------------------
// load-model
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct LoadModelArgs {
    /// Ollama model reference, e.g. `qwen3:0.6b` or `hf.co/org/model:tag`
    pub model: String,
}

// ---------------------------------------------------------------------------
// generate
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct GenerateArgs {
    /// Model reference (Ollama: `qwen:latest`, HuggingFace: `owner/model`)
    pub model: String,

    /// Prompt to tokenize (and eventually generate from)
    pub prompt: String,
}

// ---------------------------------------------------------------------------
// benchmark
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct BenchmarkArgs {
    /// Model to benchmark (HF: `owner/model`).
    /// Defaults to the model in ~/.kwaainet/config.yaml.
    pub model: Option<String>,

    /// Number of decode steps to time (after a warm-up pass).
    #[arg(long, default_value = "20")]
    pub steps: usize,

    /// Use GPU (Metal / CUDA) instead of CPU. Off by default because candle's
    /// Metal backend is not yet optimized for sequential decode.
    #[arg(long)]
    pub gpu: bool,

    /// Path to model directory (skip HF cache resolution).
    #[arg(long, value_name = "PATH")]
    pub model_path: Option<std::path::PathBuf>,
}

// ---------------------------------------------------------------------------
// serve
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ServeArgs {
    /// Model to load (Ollama: `llama3.1:8b`, HF: `owner/model`).
    /// Defaults to the model in ~/.kwaainet/config.yaml.
    pub model: Option<String>,

    /// HTTP port for the OpenAI-compatible API
    #[arg(long, default_value = "11435")]
    pub port: u16,
}

// ---------------------------------------------------------------------------
// identity
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct IdentityArgs {
    /// Output machine-readable JSON (with identity show)
    #[arg(long)]
    pub json: bool,
    #[command(subcommand)]
    pub action: IdentityAction,
}

#[derive(Subcommand)]
pub enum IdentityAction {
    /// Show this node's DID, Peer ID, trust tier, and credential summary
    Show,
    /// Import a Verifiable Credential from a JSON file into the local store
    ImportVc {
        /// Path to the VC JSON file (e.g. summit-attendee-vc.json)
        #[arg(value_name = "FILE")]
        path: std::path::PathBuf,
    },
    /// List all stored Verifiable Credentials
    ListVcs,
    /// Verify a Verifiable Credential (structure check + Ed25519 signature)
    VerifyVc {
        /// Path to the VC JSON file to verify
        #[arg(value_name = "FILE")]
        path: std::path::PathBuf,
    },
}

// ---------------------------------------------------------------------------
// vpk
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct VpkArgs {
    #[command(subcommand)]
    pub action: VpkAction,
}

#[derive(Subcommand)]
pub enum VpkAction {
    /// Enable VPK integration and start advertising on DHT
    Enable {
        /// Operating mode: bob (query-only), eve (storage), or both
        #[arg(long, value_name = "MODE")]
        mode: String,

        /// Local VPK REST API port for the health check endpoint (default: 7432).
        /// Remote peers connect via PeerId over /kwaai/storage/1.0.0 — no port forwarding needed.
        #[arg(long, default_value = "7432")]
        port: u16,
    },

    /// Disable VPK integration and stop DHT advertisement
    Disable,

    /// Show local VPK health and DHT advertisement status
    Status,

    /// Discover VPK-capable nodes via DHT
    Discover {
        /// Output machine-readable JSON (array of {public_name, peer_id, mode, capacity_gb, tenant_count})
        #[arg(long)]
        json: bool,
    },

    /// Shard a knowledge base across Eve nodes discovered via DHT
    Shard {
        /// Knowledge base identifier
        #[arg(long, value_name = "NAME")]
        kb_id: String,

        /// Number of Eve nodes to distribute shards across
        #[arg(long, value_name = "N", default_value = "1")]
        eve_count: usize,
    },

    /// Resolve shard endpoints for a knowledge base from DHT
    Resolve {
        /// Knowledge base identifier
        #[arg(long, value_name = "NAME")]
        kb_id: String,
    },

    /// Manage storage tenants on Eve nodes via /kwaai/storage/1.0.0
    Tenant(TenantArgs),

    /// Benchmark: sharded Eve vector DB vs single local index
    Bench(BenchArgs),
}

#[derive(Args)]
pub struct BenchArgs {
    /// Comma-separated Eve node PeerIds (base58) to shard across
    #[arg(long, value_name = "PEER_IDS")]
    pub eve_peer_ids: String,

    /// Total number of vectors in the corpus
    #[arg(long, default_value = "50000")]
    pub vectors: usize,

    /// Vector dimensions (must match Eve's dimension setting)
    #[arg(long, default_value = "384")]
    pub dimensions: usize,

    /// Number of search queries to run
    #[arg(long, default_value = "200")]
    pub queries: usize,

    /// Top-K results per query
    #[arg(long, default_value = "10")]
    pub top_k: usize,

    /// Vectors per RPC upload batch
    #[arg(long, default_value = "500")]
    pub batch_size: usize,

    /// Qdrant local base URL (default: http://localhost:6333, skip if unreachable)
    #[arg(long, default_value = "http://localhost:6333")]
    pub qdrant_url: String,

    /// Qdrant API key for local instance (usually not needed)
    #[arg(long, value_name = "KEY")]
    pub qdrant_api_key: Option<String>,

    /// Qdrant Cloud cluster URL (e.g. https://xyz.aws.cloud.qdrant.io:6333), skipped if omitted
    #[arg(long, value_name = "URL")]
    pub qdrant_cloud_url: Option<String>,

    /// Qdrant Cloud API key (required when --qdrant-cloud-url is set)
    #[arg(long, value_name = "KEY")]
    pub qdrant_cloud_api_key: Option<String>,
}

#[derive(Args)]
pub struct TenantArgs {
    #[command(subcommand)]
    pub action: TenantAction,
}

#[derive(Subcommand)]
pub enum TenantAction {
    /// Create a new tenant on a remote Eve node
    Create {
        /// Target Eve node's PeerId (base58)
        #[arg(long, value_name = "PEER_ID")]
        eve_peer_id: String,

        /// Maximum storage capacity for this tenant, in MB
        #[arg(long, value_name = "MB", default_value = "1024")]
        capacity_mb: i64,

        /// Human-readable label for this tenant
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// Vector dimension — must match your embedding model (default: 384)
        #[arg(long, value_name = "DIM", default_value = "384")]
        dimension: usize,
    },

    /// List tenants stored on a remote Eve node
    List {
        /// Target Eve node's PeerId (base58)
        #[arg(long, value_name = "PEER_ID")]
        eve_peer_id: String,
    },

    /// Show metadata for a specific tenant
    Info {
        /// Tenant UUID
        tenant_id: String,

        /// Target Eve node's PeerId (base58)
        #[arg(long, value_name = "PEER_ID")]
        eve_peer_id: String,
    },

    /// Delete a tenant and all its stored vectors
    Delete {
        /// Tenant UUID
        tenant_id: String,

        /// Target Eve node's PeerId (base58)
        #[arg(long, value_name = "PEER_ID")]
        eve_peer_id: String,

        /// Skip the confirmation prompt
        #[arg(long)]
        yes: bool,
    },
}

// ---------------------------------------------------------------------------
// storage
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct StorageArgs {
    #[command(subcommand)]
    pub action: StorageAction,
}

#[derive(Subcommand)]
pub enum StorageAction {
    /// One-time setup: create the embedded vector store and save config.
    /// No Docker or PostgreSQL required — pure embedded storage.
    Init {
        /// Storage capacity to offer to the network, in GB.
        /// Defaults to 10% of available disk space (automatically detected).
        #[arg(long)]
        capacity_gb: Option<f64>,

        /// Port for the local storage health-check API (localhost only).
        /// Remote Bobs connect via PeerId over /kwaai/storage/1.0.0 — no port forwarding needed.
        #[arg(long, default_value = "7432")]
        port: u16,

        /// Directory to write vector data (default: ~/.kwaainet/storage).
        /// Point this at an external or secondary drive for extra capacity.
        #[arg(long, value_name = "PATH")]
        data_dir: Option<std::path::PathBuf>,
    },

    /// Show embedded store health, tenant count, vector count, and disk usage
    Status,

    /// Run the storage API server in the foreground (Ctrl+C to stop).
    /// Use 'kwaainet start --daemon' to run everything as a background service.
    Serve,

    /// Alias for 'kwaainet start --daemon' (starts storage API alongside the node)
    Start,

    /// Alias for 'kwaainet stop' (stops all services including storage API)
    Stop,

    /// Permanently delete all vector data and remove storage configuration
    Destroy {
        /// Skip the confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
}

// ---------------------------------------------------------------------------
// uninstall
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct UninstallArgs {
    /// Skip the confirmation prompt
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Keep ~/.kwaainet/ data (config, logs, identity) — only remove binaries and service
    #[arg(long)]
    pub keep_data: bool,
}

// ---------------------------------------------------------------------------
// shard
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ShardArgs {
    #[command(subcommand)]
    pub action: ShardAction,
}

#[derive(Subcommand)]
pub enum ShardAction {
    /// Load a model shard and register it as an inference RPC handler
    Serve(ShardServeArgs),
    /// Run distributed inference across discovered block servers
    Run(ShardRunArgs),
    /// Show local shard configuration
    Status,
    /// Query DHT and display block-circuit coverage
    Chain(ShardChainArgs),
    /// Serve an OpenAI-compatible HTTP API backed by distributed shard inference
    Api(ShardApiArgs),
    /// Download a HuggingFace model SafeTensors snapshot (no huggingface-cli required)
    Download(ShardDownloadArgs),
    /// Show which block range this node would auto-assign (dry-run, no model load)
    Gap,
    /// Manage inference circuits (long-lived peer paths for multi-turn chat)
    #[command(subcommand)]
    Circuit(CircuitAction),
}

#[derive(Subcommand)]
pub enum CircuitAction {
    /// Create a new circuit (discover chain, pin peer path)
    Create(CircuitCreateArgs),
    /// List active circuits
    List,
    /// Close and remove a circuit
    Close(CircuitCloseArgs),
}

#[derive(Args)]
pub struct CircuitCreateArgs {
    /// Only use block servers whose public_name contains this string
    #[arg(long, value_name = "SUBSTR")]
    pub name_filter: Option<String>,

    /// Circuit time-to-live in minutes (default 30)
    #[arg(long, default_value = "30")]
    pub ttl_minutes: u64,
}

#[derive(Args)]
pub struct CircuitCloseArgs {
    /// Circuit ID to close
    pub id: String,
}

#[derive(Args, Clone)]
pub struct ShardServeArgs {
    /// Path to the model directory (config.json + *.safetensors + tokenizer.json).
    /// Defaults to the HuggingFace cache for the model in config.yaml.
    #[arg(long, value_name = "PATH")]
    pub model_path: Option<std::path::PathBuf>,

    /// Override start_block from config.yaml
    #[arg(long)]
    pub start_block: Option<u32>,

    /// Override number of blocks from config.yaml
    #[arg(long)]
    pub blocks: Option<u32>,

    /// Disable GPU acceleration and use CPU only
    #[arg(long)]
    pub no_gpu: bool,

    /// Force GPU even on Metal (experimental — Metal decode is currently 10x slower than CPU)
    #[arg(long)]
    pub use_gpu: bool,

    /// Auto-discover which blocks are unserved and load those instead of config start_block.
    /// Uses --blocks (or config.blocks) as the target count.
    /// This is now the default when --start-block is not given; kept as a no-op alias.
    #[arg(long)]
    pub auto: bool,

    /// Disable automatic DHT gap discovery and use start_block from config.yaml instead.
    /// Useful when you want a node to serve a fixed, pre-configured range.
    #[arg(long)]
    pub no_auto: bool,

    /// Periodically check DHT coverage and move blocks to fill gaps when our current
    /// range is already well-covered by other nodes.
    /// Interval and redundancy threshold are set via `kwaainet config set`.
    #[arg(long)]
    pub auto_rebalance: bool,

    /// HuggingFace access token for downloading private or gated models.
    /// Can also be set via the HF_TOKEN environment variable.
    #[arg(long, value_name = "TOKEN")]
    pub hf_token: Option<String>,
}

#[derive(Args)]
pub struct ShardRunArgs {
    /// Prompt to run distributed inference on
    pub prompt: String,

    /// HuggingFace model ID (defaults to config.model)
    #[arg(long)]
    pub model: Option<String>,

    /// Total transformer blocks in the full model (default: inferred from model name)
    #[arg(long)]
    pub total_blocks: Option<usize>,

    /// Maximum tokens to generate
    #[arg(long, default_value = "200")]
    pub max_tokens: usize,

    /// Explicit session ID (randomly generated if not set)
    #[arg(long)]
    pub session_id: Option<u64>,

    /// Only use block servers whose public_name contains this string.
    /// Useful for restricting to known-good nodes: --name-filter v0.2.3
    #[arg(long, value_name = "SUBSTR")]
    pub name_filter: Option<String>,

    /// Sampling temperature (1.0 = greedy, lower = more focused)
    #[arg(long, default_value = "1.0")]
    pub temperature: f32,

    /// Top-p nucleus sampling cutoff (1.0 = disabled)
    #[arg(long, default_value = "1.0")]
    pub top_p: f32,

    /// Top-k sampling cutoff (0 = disabled)
    #[arg(long, default_value = "0")]
    pub top_k: usize,

    /// Path to model dir for tokenizer (overrides HF cache lookup)
    #[arg(long, value_name = "PATH")]
    pub model_path: Option<std::path::PathBuf>,

    /// Run inference entirely in-process — load model locally without `shard serve`.
    /// Requires --model-path (or a cached HF snapshot). No P2P or TCP overhead.
    #[arg(long)]
    pub local: bool,

    /// Disable GPU and use CPU only (applies when --local is set)
    #[arg(long)]
    pub no_gpu: bool,

    /// Print per-token timing stats after generation
    #[arg(long)]
    pub stats: bool,

    /// Use an existing circuit instead of discovering the chain fresh.
    /// Create one with: kwaainet shard circuit create
    #[arg(long, value_name = "ID")]
    pub circuit: Option<String>,
}

#[derive(Args)]
pub struct ShardChainArgs {
    /// DHT prefix to query (e.g. "Llama-3-1-8B-Instruct-hf").
    /// Defaults to config.model_dht_prefix or derived from config.model.
    #[arg(long)]
    pub dht_prefix: Option<String>,

    /// Number of blocks to scan (default: 32)
    #[arg(long, default_value = "32")]
    pub total_blocks: usize,
}

#[derive(Args)]
pub struct ShardApiArgs {
    /// HTTP port to listen on
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Total transformer blocks in the full model (default: inferred)
    #[arg(long)]
    pub total_blocks: Option<usize>,

    /// HuggingFace model ID (defaults to config.model)
    #[arg(long)]
    pub model: Option<String>,

    /// Path to model dir for tokenizer
    #[arg(long, value_name = "PATH")]
    pub model_path: Option<std::path::PathBuf>,

    /// Default sampling temperature
    #[arg(long, default_value = "0.7")]
    pub temperature: f32,

    /// Ollama model name for llama.cpp fast path (e.g. "llama3.1:8b").
    /// Uses GPU-accelerated local inference when all blocks are hosted locally.
    #[arg(long, value_name = "MODEL")]
    pub ollama_model: Option<String>,

    /// Explicit path to a GGUF file for llama.cpp fast path.
    /// Takes precedence over --ollama-model.
    #[arg(long, value_name = "PATH")]
    pub gguf_path: Option<std::path::PathBuf>,
}

#[derive(Args)]
pub struct ShardDownloadArgs {
    /// HuggingFace model ID (e.g. unsloth/Llama-3.1-8B-Instruct).
    /// Defaults to the model in ~/.kwaainet/config.yaml.
    pub model: Option<String>,

    /// HuggingFace access token for private or gated models.
    /// Can also be set via the HF_TOKEN environment variable.
    #[arg(long, value_name = "TOKEN")]
    pub hf_token: Option<String>,

    /// First transformer block to download (0-indexed).
    /// When set together with --blocks, only weight files for that range are
    /// fetched — a 10× reduction for large models split across many nodes.
    /// Omit to download all weight files.
    #[arg(long)]
    pub start_block: Option<usize>,

    /// Number of transformer blocks to download (used with --start-block).
    #[arg(long)]
    pub blocks: Option<usize>,
}

// ---------------------------------------------------------------------------
// calibrate
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct CalibrateArgs {
    /// Model to calibrate for
    #[arg(long)]
    pub model: Option<String>,

    /// Force re-calibration (ignore cache)
    #[arg(long)]
    pub force: bool,

    /// Quick estimation mode (default)
    #[arg(long, default_value = "true")]
    pub quick: bool,

    /// Apply a calibration profile: min, recommended, or max
    #[arg(long, value_name = "PROFILE")]
    pub apply: Option<String>,
}

// ---------------------------------------------------------------------------
// setup
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct SetupArgs {
    /// Download and install missing dependencies (e.g. p2pd)
    #[arg(long)]
    pub get_deps: bool,
}

// ---------------------------------------------------------------------------
// reputation
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ReputationArgs {
    #[command(subcommand)]
    pub action: ReputationAction,
}

#[derive(Subcommand)]
pub enum ReputationAction {
    /// List all known peers with their local trust tier and score
    List,

    /// Show detailed reputation breakdown for a peer (accepts a peer ID prefix)
    Show {
        /// Full peer ID or unique prefix (≥ 12 chars)
        peer_id: String,
    },

    /// Clear all recorded observations for a peer (accepts a peer ID prefix)
    Reset {
        /// Full peer ID or unique prefix (≥ 12 chars)
        peer_id: String,
    },
}

// ---------------------------------------------------------------------------
// rag
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct RagArgs {
    #[command(subcommand)]
    pub action: RagAction,
}

#[derive(Subcommand)]
pub enum RagAction {
    /// Initialise a local RAG knowledge base (no network required)
    Init {
        /// Knowledge base name (default: "default"). Use --name to create additional KBs.
        #[arg(long, default_value = "default", value_name = "NAME")]
        name: String,

        /// Ollama embedding model (must produce 768-dim vectors)
        #[arg(long, default_value = "nomic-embed-text")]
        embed_model: String,

        /// Directory for the knowledge base (defaults to ~/.kwaainet/rag/ for "default",
        /// ~/.kwaainet/rag/<name>/ for named KBs). Point to an external drive for large corpora.
        #[arg(long, value_name = "PATH")]
        rag_dir: Option<std::path::PathBuf>,

        /// Enable knowledge graph construction (entity + relation extraction during ingest)
        #[arg(long)]
        graph: bool,
    },

    /// List all knowledge bases
    List,

    /// Outsource this knowledge base's vector storage to an Eve node on the network
    ConnectEve {
        /// Eve node peer ID (base58)
        peer_id: String,

        /// Eve HTTP URL (e.g. http://192.168.1.10:7432). Omit to use P2P transport.
        #[arg(long)]
        url: Option<String>,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Ingest a document into the knowledge base
    Ingest {
        /// Path to the file to ingest (.txt, .md)
        file: std::path::PathBuf,

        /// Override the document name stored in metadata (default: file name)
        #[arg(long)]
        doc_name: Option<String>,

        /// Chunk size in characters
        #[arg(long, default_value = "800")]
        chunk_size: usize,

        /// Chunk overlap in characters
        #[arg(long, default_value = "200")]
        chunk_overlap: usize,

        /// Minimum chunk length in characters (shorter chunks are dropped)
        #[arg(long, default_value = "20")]
        min_chunk_len: usize,

        /// Extract entities and relations into the knowledge graph during ingest
        #[arg(long)]
        extract_entities: bool,

        /// Inference URL for entity extraction (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// Model name for entity extraction (e.g. "llama3.2:3b"; default: "default")
        #[arg(long, default_value = "default", value_name = "MODEL")]
        extraction_model: String,

        /// Chunking strategy: "character" (default sliding-window) or "paragraph" (semantic)
        #[arg(long, default_value = "character", value_name = "STRATEGY")]
        chunk_strategy: String,

        /// YAML file mapping doc-name substrings to metadata prefixes prepended to each chunk.
        /// Example: intro.docx: "Author: Y.S. Rassool."
        #[arg(long, value_name = "YAML_FILE")]
        doc_meta: Option<std::path::PathBuf>,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Retrieve top-K chunks for a query (no LLM — debug tool)
    Query {
        /// Query text
        text: String,

        /// Number of results to return
        #[arg(long, short = 'k', default_value = "5")]
        top_k: usize,

        /// Minimum cosine similarity score (0.0–1.0)
        #[arg(long, default_value = "0.0")]
        min_score: f64,

        /// Output results as JSON (for benchmark scripts)
        #[arg(long)]
        json: bool,

        /// Knowledge base name, or "all" to query all KBs and merge results (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Decompose query into sub-queries via LLM for better recall on complex questions
        #[arg(long)]
        understand: bool,

        /// Inference URL for query understanding (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// Retrieval mode: "vector" (default hybrid), "graph" (entity-anchored), "auto" (router)
        #[arg(long, default_value = "vector", value_name = "MODE")]
        mode: String,

        /// Model name for query understanding / HyDE LLM call (e.g. "llama3.2:3b"; default: "default")
        #[arg(long, default_value = "default")]
        model: String,

        /// Use HyDE: generate a hypothetical answer and embed that instead of the raw query
        #[arg(long)]
        hyde: bool,

        /// HyDE blend factor (0.0=pure query, 1.0=pure HyDE, 0.5=equal blend; default when --hyde is set: 0.5)
        #[arg(long, value_name = "ALPHA")]
        hyde_alpha: Option<f32>,

        /// Rerank candidates with a single LLM call before selecting top-k
        #[arg(long)]
        rerank: bool,
    },

    /// Interactive RAG chat REPL (streams from shard API)
    Chat {
        /// Number of context chunks to inject
        #[arg(long, short = 'k', default_value = "5")]
        top_k: usize,

        /// Shard API base URL
        #[arg(long, default_value = "http://localhost:8080")]
        inference_url: String,

        /// Knowledge base name, or "all" to query all KBs (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Decompose each user query into sub-queries via LLM for better recall
        #[arg(long)]
        understand: bool,

        /// Model name for the chat completions call (e.g. "llama3.2:3b"; default: "default")
        #[arg(long, default_value = "default")]
        model: String,

        /// Use HyDE: generate a hypothetical answer and embed that instead of the raw query
        #[arg(long)]
        hyde: bool,

        /// HyDE blend factor (0.0=pure query, 1.0=pure HyDE, 0.5=equal blend; default when --hyde is set: 0.5)
        #[arg(long, value_name = "ALPHA")]
        hyde_alpha: Option<f32>,

        /// Rerank candidates with a single LLM call before selecting top-k
        #[arg(long)]
        rerank: bool,
    },

    /// List ingested documents
    Docs {
        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Delete a document from the knowledge base
    DeleteDoc {
        /// Document name (as shown by `rag docs`)
        name: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Permanently delete the local knowledge base and all its data
    Destroy {
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Serve an OpenAI-compatible RAG API (port 9090 by default)
    Serve {
        /// HTTP port to listen on
        #[arg(long, default_value = "9090")]
        port: u16,

        /// Shard API base URL for inference
        #[arg(long, default_value = "http://localhost:8080")]
        inference_url: String,

        /// Number of context chunks to inject per request
        #[arg(long, short = 'k', default_value = "5")]
        top_k: usize,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Sync a folder of documents into the knowledge base
    Sync {
        /// Folder to scan for documents
        folder: std::path::PathBuf,

        /// File extensions to include (comma-separated)
        #[arg(long, default_value = "txt,md,pdf,docx,doc")]
        extensions: String,

        /// Remove KB entries whose source files no longer exist
        #[arg(long)]
        delete: bool,

        /// Watch for changes and sync continuously
        #[arg(long)]
        watch: bool,

        /// Polling interval in seconds (watch mode only)
        #[arg(long, default_value = "60")]
        interval: u64,

        /// Chunk size in characters
        #[arg(long, default_value = "800")]
        chunk_size: usize,

        /// Chunk overlap in characters
        #[arg(long, default_value = "200")]
        chunk_overlap: usize,

        /// Minimum chunk length in characters (shorter chunks are dropped)
        #[arg(long, default_value = "20")]
        min_chunk_len: usize,

        /// Extract entities and relations into the knowledge graph during sync
        #[arg(long)]
        extract_entities: bool,

        /// Inference URL for entity extraction (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// Model name for entity extraction (e.g. "llama3.2:3b"; default: "default")
        #[arg(long, default_value = "default", value_name = "MODEL")]
        extraction_model: String,

        /// Chunking strategy: "character" (default sliding-window) or "paragraph" (semantic)
        #[arg(long, default_value = "character", value_name = "STRATEGY")]
        chunk_strategy: String,

        /// YAML file mapping doc-name substrings to metadata prefixes prepended to each chunk.
        /// Example: intro.docx: "Author: Y.S. Rassool."
        #[arg(long, value_name = "YAML_FILE")]
        doc_meta: Option<std::path::PathBuf>,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Inspect and query the knowledge graph
    Graph {
        #[command(subcommand)]
        action: GraphAction,

        /// Knowledge base name (default: "default")
        #[arg(long, global = true, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Manage the semantic query cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,

        /// Knowledge base name (default: "default")
        #[arg(long, global = true, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Evaluate RAG accuracy against a question set with expected keywords
    Eval {
        /// Path to questions JSON file (array of {id, question, expected_keywords: [...]})
        #[arg(long, value_name = "FILE")]
        questions: std::path::PathBuf,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Inference URL for LLM answer generation
        #[arg(long, default_value = "http://localhost:11434", value_name = "URL")]
        inference_url: String,

        /// Model name for answer generation (e.g. "llama3.2:3b")
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Number of context chunks to retrieve per question
        #[arg(long, short = 'k', default_value = "5")]
        top_k: usize,

        /// Retrieval mode: "vector" (default hybrid), "graph" (entity-anchored), "auto" (router)
        #[arg(long, default_value = "vector", value_name = "MODE")]
        mode: String,

        /// Use HyDE: embed a hypothetical answer instead of the raw query
        #[arg(long)]
        hyde: bool,

        /// HyDE blend factor (0.0=pure query, 1.0=pure HyDE, 0.5=equal blend; default when --hyde is set: 0.5)
        #[arg(long, value_name = "ALPHA")]
        hyde_alpha: Option<f32>,

        /// Rerank candidates with a single LLM call
        #[arg(long)]
        rerank: bool,

        /// Decompose queries via LLM for better recall
        #[arg(long)]
        understand: bool,

        /// Score answers with an LLM judge (0=wrong, 1=partial, 2=correct)
        /// Requires expected_answer field in the questions JSON
        #[arg(long)]
        llm_judge: bool,

        /// Model to use for LLM judging (defaults to --model if not set)
        #[arg(long, value_name = "MODEL")]
        judge_model: Option<String>,

        /// Write the markdown report to this file instead of stdout
        #[arg(long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,
    },

    /// Export the knowledge graph to an Obsidian vault
    Export {
        /// Output directory for the vault (created if absent)
        #[arg(long, value_name = "DIR")]
        output_dir: std::path::PathBuf,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Import curated edits from an Obsidian vault back into the knowledge graph
    Import {
        /// Path to the Obsidian vault directory (must contain `entities/`)
        #[arg(long, value_name = "DIR")]
        input_dir: std::path::PathBuf,

        /// Only process files modified after this Unix timestamp (default: 0 = all files)
        #[arg(long, default_value = "0", value_name = "SECS")]
        since: u64,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },
}

#[derive(Subcommand)]
pub enum GraphAction {
    /// Show entity and relation counts for the knowledge base
    Stats,

    /// Show an entity and its immediate neighbours
    Show {
        /// Entity name (case-insensitive)
        name: String,
    },

    /// Wipe the knowledge graph (entities + relations) — rebuild with `graph build` afterwards
    Clear {
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Extract entities from all already-ingested chunks (populates graph without re-ingesting)
    Build {
        /// Inference URL for entity extraction (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// Model name for the chat completions call (e.g. "llama3.2:3b"; default: "default")
        #[arg(long, default_value = "default", value_name = "MODEL")]
        model: String,

        /// Process at most this many chunks (default: all)
        #[arg(long, value_name = "N")]
        limit: Option<usize>,

        /// Only process chunks from documents whose name contains one of these substrings
        /// (comma-separated, case-insensitive). Example: --docs "Part_1,intro"
        #[arg(long, value_name = "PATTERNS")]
        docs: Option<String>,
    },

    /// Seed the graph from a ground-truth YAML family tree — merges aliases and plants family relations
    Seed {
        /// Path to the YAML family tree file (see tests/d6_family_tree.yaml for format)
        #[arg(long, value_name = "FILE")]
        file: std::path::PathBuf,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },
}

#[derive(Subcommand)]
pub enum CacheAction {
    /// Show cache statistics (entry count, hit count, expired entries)
    Stats,
    /// Remove all cached queries
    Clear,
}

// ---------------------------------------------------------------------------
// p2p
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct P2pArgs {
    #[command(subcommand)]
    pub action: P2pAction,
}

#[derive(Subcommand)]
pub enum P2pAction {
    /// Show this node's peer ID, listen addresses, observed addresses, and a NAT verdict
    Info,

    /// Inspect active connections to other peers
    Peers(PeersArgs),
}

#[derive(Args)]
pub struct PeersArgs {
    #[command(subcommand)]
    pub action: PeersAction,
}

#[derive(Subcommand)]
pub enum PeersAction {
    /// List all live connections (one row per connection — direct or via relay)
    List,

    /// Active DHT lookup: what addresses does the network advertise for this peer?
    Find {
        /// Peer ID (base58, e.g. 12D3KooW…)
        peer_id: String,

        /// Lookup timeout in seconds
        #[arg(long, default_value = "10")]
        timeout: i64,
    },

    /// Manually dial a peer. Forces a connection attempt that hole-punching
    /// can react to; optionally sends a hello message once the connection
    /// succeeds. Specify exactly one of --addr or --peer.
    Connect {
        /// Full multiaddr including /p2p/<peer-id> (and, for relay'd dials,
        /// /p2p-circuit/p2p/<destination>). Use this when you have a
        /// complete address in hand — typically copied from `peers list`
        /// or `peers find`. No DHT lookup is performed.
        #[arg(long, conflicts_with = "peer", required_unless_present = "peer")]
        addr: Option<String>,

        /// Peer ID (base58) to look up in the DHT and dial. The CLI runs
        /// `dht_find_peer`, picks an address (preferring direct over
        /// relay'd), appends /p2p/<peer-id>, and dials. Use this when you
        /// only have a peer ID — e.g. one you saw in `peers list`.
        #[arg(long, conflicts_with = "addr", required_unless_present = "addr")]
        peer: Option<String>,

        /// Optional message to send over /kwaai/p2p/hello/1.0.0 once the
        /// connection succeeds. The recipient logs the message to stdout.
        #[arg(long)]
        message: Option<String>,
    },

    /// Invoke a unary RPC on an already-connected peer. Defaults to the
    /// `/kwaai/p2p/hello/1.0.0` protocol (the recipient logs the payload
    /// and replies with `b"ok"`), but with `--proto` works as a generic
    /// diagnostic for any registered handler. Doubles as the in-tree
    /// example of how to invoke a custom unary protocol over the libp2p
    /// fabric — see `kwaai_p2p_daemon::hello` for the handler.
    ///
    /// Specify exactly one payload source: --message, --payload-hex,
    /// --payload-bin, or --stdin. The bytes are sent as-is; no encoding,
    /// no wrapper. The response is displayed below.
    Send {
        /// Recipient peer ID (base58)
        #[arg(long)]
        peer: String,

        /// Protocol ID. Default is the hello protocol (a peer running
        /// kwaainet logs the payload and replies with `b"ok"`); for any
        /// other in-tree or third-party unary protocol, name it here.
        #[arg(long, default_value = kwaai_p2p_daemon::hello::HELLO_PROTO)]
        proto: String,

        /// Send the bytes of this UTF-8 string as the payload. Identical
        /// to `--payload-bin` of a file containing the same bytes.
        #[arg(long, group = "payload")]
        message: Option<String>,

        /// Send the bytes decoded from this hex string as the payload.
        /// Whitespace in the hex is ignored. Use for short binary payloads
        /// you want to type inline.
        #[arg(long, group = "payload", value_name = "HEX")]
        payload_hex: Option<String>,

        /// Send the contents of this file as the payload. Use for any
        /// payload too large to type inline, or for binary blobs you
        /// already have on disk.
        #[arg(long, group = "payload", value_name = "PATH")]
        payload_bin: Option<std::path::PathBuf>,

        /// Read the payload from stdin. Combine with shell redirection
        /// (`< file.bin`), here-docs (`<<<'hi'`), or pipes
        /// (`echo -n hi | …`). The bytes are sent verbatim.
        #[arg(long, group = "payload")]
        stdin: bool,

        /// Maximum seconds to wait for the recipient's response. Useful when
        /// poking at unfamiliar protocols whose handlers might hang on
        /// malformed input. Default 10s.
        #[arg(long, default_value = "10")]
        timeout: u64,
    },
}
