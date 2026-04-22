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
  kwaainet storage init --capacity-gb 10  provision PostgreSQL+pgvector
  kwaainet storage status                 show PG health and tenants
  kwaainet vpk status                     VPK health and DHT status

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
    Status,

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

    /// Manage VPK (Virtual Private Knowledge) vector database integration
    Vpk(VpkArgs),

    /// Manage the local storage fabric (Eve role — host opaque vectors for Bob nodes on the network)
    Storage(StorageArgs),

    /// Uninstall KwaaiNet — stop the node, remove all data, and delete binaries
    Uninstall(UninstallArgs),

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
// config
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ConfigArgs {
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
    ///   vpk_enabled, vpk_mode, vpk_endpoint, vpk_local_port,
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

        /// Public HTTP endpoint to advertise to peers (omit for local-only)
        #[arg(long, value_name = "URL")]
        endpoint: Option<String>,

        /// Local VPK REST API port for health checks
        #[arg(long, default_value = "7432")]
        port: u16,
    },

    /// Disable VPK integration and stop DHT advertisement
    Disable,

    /// Show local VPK health and DHT advertisement status
    Status,

    /// Discover VPK-capable nodes via DHT
    Discover,

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
        /// Storage capacity to offer to the network, in GB
        #[arg(long, default_value = "5")]
        capacity_gb: f64,

        /// Port the storage API will listen on
        #[arg(long, default_value = "7432")]
        port: u16,

        /// Directory to write vector data (default: ~/.kwaainet/storage).
        /// Point this at an external or secondary drive for extra capacity.
        #[arg(long, value_name = "PATH")]
        data_dir: Option<std::path::PathBuf>,

        /// Public HTTP endpoint advertised on the DHT so Bob nodes can reach this Eve.
        /// Omit for local-only / LAN use.
        #[arg(long, value_name = "URL")]
        endpoint: Option<String>,
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
