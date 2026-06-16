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

    /// Route inference to a remote Ollama instance instead of block sharding.
    /// Accepts http://host:port or mux://PEER_ID (p2p relay to remote Ollama).
    /// When set, --model is the Ollama model name (e.g. "llama3.1:8b").
    /// Example: --inference-url mux://12D3KooW...
    #[arg(long, value_name = "URL")]
    pub inference_url: Option<String>,
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

    /// Only use block servers whose public_name contains this string.
    /// Matches the same filter available on `shard run`.
    /// Example: --name-filter metro-linux
    #[arg(long, value_name = "SUBSTR")]
    pub name_filter: Option<String>,

    /// Use a pre-formed circuit instead of discovering the chain fresh.
    /// Create one with: kwaainet shard circuit create [--name-filter SUBSTR]
    /// The server uses the circuit's chain snapshot; DHT discovery is skipped.
    #[arg(long, value_name = "ID")]
    pub circuit: Option<String>,

    /// Pin all requests to a single peer (base58 PeerId).
    /// The peer must cover the full block range or inference will fail.
    #[arg(long, value_name = "PEER_ID")]
    pub peer: Option<String>,
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
    /// Initialise a local RAG knowledge base (requires Ollama running with an embed model)
    Init {
        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Ollama embedding model. Dimension is auto-detected at init time.
        /// Popular choices: nomic-embed-text (768-dim), all-minilm (384-dim),
        /// mxbai-embed-large (1024-dim, best accuracy). Pull with: ollama pull <model>
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

        /// Surrounding context mode: "truncated" (±chunk_size/4 chars, default) or
        /// "full" (complete adjacent paragraphs passed to LLM; paragraph strategy only)
        #[arg(long, default_value = "truncated", value_name = "MODE")]
        surr_mode: String,

        /// YAML file mapping doc-name substrings to metadata prefixes prepended to each chunk.
        /// Example: intro.docx: "Author: Y.S. Rassool."
        #[arg(long, value_name = "YAML_FILE")]
        doc_meta: Option<std::path::PathBuf>,

        /// YAML file describing the document's section structure (skip zones, narrator overrides).
        #[arg(long, value_name = "YAML_FILE")]
        doc_schema: Option<std::path::PathBuf>,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Retrieve top-K chunks for a query (no LLM — debug tool)
    Query {
        /// Query text
        text: String,

        /// Number of results to return
        #[arg(long, short = 'k', default_value = "20")]
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

        /// Retrieval mode: "vector" (hybrid), "graph" (entity-anchored), "auto" (router),
        /// "iterative" (multi-round gap-fill with narration), "smart" (iterative for most queries,
        /// graph+replace for family-relation queries)
        #[arg(long, default_value = "iterative", value_name = "MODE")]
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

        /// How to integrate graph query results into the LLM context.
        /// "inject" (default) — prepend one entity description chunk (existing behaviour).
        /// "prepend" — structured graph-facts block at score 3.0, Top-K chunks kept.
        /// "replace" — for family-relation queries, replace all chunks with graph facts only.
        #[arg(long, default_value = "inject", value_name = "MODE")]
        graph_mode: String,

        /// Query intent classification method for the graph pipeline.
        /// "rule" (default) — fast keyword patterns, zero latency.
        /// "llm" — single LLM JSON call, flexible, ~2 s added latency.
        /// "hybrid" — rule first, LLM fallback when intent is unknown.
        #[arg(long, default_value = "rule", value_name = "METHOD")]
        query_classify: String,
    },

    /// Interactive RAG chat REPL (streams from shard API)
    Chat {
        /// Number of context chunks to inject
        #[arg(long, short = 'k', default_value = "20")]
        top_k: usize,

        /// Inference API base URL (defaults to inference_url in ~/.kwaainet/config.yaml,
        /// or http://localhost:11434 if not configured)
        #[arg(long)]
        inference_url: Option<String>,

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

        /// Retrieval mode: "vector" (hybrid), "graph" (entity-anchored), "auto" (router),
        /// "iterative" (multi-round gap-fill with narration), "smart" (iterative for most queries,
        /// graph+replace for family-relation queries)
        #[arg(long, default_value = "iterative", value_name = "MODE")]
        mode: String,
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

    /// Destroy and fully rebuild a knowledge base in one command.
    /// Runs: destroy → init → ingest → graph build → seed → alias-scan → reembed → dedup → score
    Rebuild {
        /// Path to the document to ingest (PDF, txt, md)
        file: std::path::PathBuf,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Ollama embedding model
        #[arg(long, default_value = "nomic-embed-text")]
        embed_model: String,

        /// Comma-separated Ollama base URLs for graph build round-robin dispatch.
        /// Example: --inference-urls "http://metro-linux:11434,http://metro-win:11434"
        #[arg(long, default_value = "http://localhost:11434", value_name = "URLS")]
        inference_urls: String,

        /// LLM model name for entity extraction (e.g. "llama3.2:3b")
        #[arg(long, default_value = "llama3.2:3b", value_name = "MODEL")]
        model: String,

        /// Number of parallel extraction workers
        #[arg(long, default_value = "4", value_name = "N")]
        workers: usize,

        /// YAML seed file for canonical entities (see tests/kwaai-knowledge/d6_family_tree.yaml)
        #[arg(long, value_name = "FILE")]
        seed_file: Option<std::path::PathBuf>,

        /// Chunking strategy: "paragraph" (semantic, recommended) or "character" (sliding-window)
        #[arg(long, default_value = "paragraph", value_name = "STRATEGY")]
        chunk_strategy: String,

        /// YAML file mapping doc-name substrings to metadata prefixes prepended to each chunk
        #[arg(long, value_name = "YAML_FILE")]
        doc_meta: Option<std::path::PathBuf>,

        /// YAML file describing the document's section structure (skip zones, narrator overrides).
        #[arg(long, value_name = "YAML_FILE")]
        doc_schema: Option<std::path::PathBuf>,

        /// Comma-separated entity types to extract (default: all 15).
        /// Example: --entity-types Person,Place,Organization
        #[arg(long, value_name = "TYPES")]
        entity_types: Option<String>,

        /// Skip relation extraction entirely (recommended for 8B models — precision too low).
        #[arg(long)]
        no_relations: bool,

        /// Number of adjacent chunks passed as context per extraction call (default: 1).
        #[arg(long, default_value = "1", value_name = "N")]
        graph_window: usize,

        /// Process only this percentage of chunks for graph build (1–100).
        /// Useful for quick test cycles — ingest is always full, only graph extraction is sampled.
        /// Example: --sample 10 processes the first 10% of chunks.
        #[arg(long, value_name = "PERCENT")]
        sample_pct: Option<u8>,

        /// Skip the destroy confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
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
        #[arg(long, short = 'k', default_value = "20")]
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

        /// Surrounding context mode: "truncated" (±chunk_size/4 chars, default) or
        /// "full" (complete adjacent paragraphs passed to LLM; paragraph strategy only)
        #[arg(long, default_value = "truncated", value_name = "MODE")]
        surr_mode: String,

        /// YAML file mapping doc-name substrings to metadata prefixes prepended to each chunk.
        /// Example: intro.docx: "Author: Y.S. Rassool."
        #[arg(long, value_name = "YAML_FILE")]
        doc_meta: Option<std::path::PathBuf>,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Inspect, build, seed, and maintain the knowledge graph
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
        #[arg(long, short = 'k', default_value = "20")]
        top_k: usize,

        /// Retrieval mode: "vector" (hybrid), "graph" (entity-anchored), "auto" (router),
        /// "iterative" (multi-round gap-fill with narration), "smart" (iterative for most queries,
        /// graph+replace for family-relation queries)
        #[arg(long, default_value = "iterative", value_name = "MODE")]
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

        /// Path to write per-question progress JSON after each answer.
        /// Defaults to {data_dir}/eval-progress.json so it can be polled
        /// without knowing the path. JSON fields: done, total, running_recall,
        /// last_q, last_score, elapsed_s, eta_s.
        #[arg(long, value_name = "FILE")]
        progress_file: Option<std::path::PathBuf>,

        /// How to integrate graph query results into the LLM context.
        /// "inject" (default) — prepend one entity description chunk (existing behaviour).
        /// "prepend" — structured graph-facts block at score 3.0, Top-K chunks kept.
        /// "replace" — for family-relation queries, replace all chunks with graph facts only.
        #[arg(long, default_value = "inject", value_name = "MODE")]
        graph_mode: String,

        /// Query intent classification method for the graph pipeline.
        /// "rule" (default) — fast keyword patterns, zero latency.
        /// "llm" — single LLM JSON call, flexible, ~2 s added latency.
        /// "hybrid" — rule first, LLM fallback when intent is unknown.
        #[arg(long, default_value = "rule", value_name = "METHOD")]
        query_classify: String,

        /// Enable HiRAG Round 2.5: cosine-match summary nodes and expand to child chunks.
        /// Requires running `kwaainet rag summarize --kb <KB>` first.
        #[arg(long)]
        summary_expansion: bool,

        /// For "who was / who is" biographical questions: append a detailed-answer instruction
        /// to the user question before sending to the LLM, eliciting full biography rather
        /// than a one-liner identity response.
        #[arg(long)]
        biographical_expansion: bool,
    },

    /// Autonomous knowledge graph completion (Dream RAG)
    Dream {
        #[command(subcommand)]
        action: DreamAction,

        /// Knowledge base name (default: "default")
        #[arg(long, global = true, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Build HiRAG summary hierarchy (window + section summaries for Round 2.5 expansion)
    Summarize {
        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,

        /// Inference URL for LLM summarization
        #[arg(long, default_value = "http://localhost:11434", value_name = "URL")]
        inference_url: String,

        /// Model name for summarization
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Number of chunks per window summary (default: 10)
        #[arg(long, default_value_t = 10, value_name = "N")]
        window_size: usize,

        /// Drop existing summaries and rebuild from scratch
        #[arg(long)]
        reset: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum DreamAction {
    /// Run one dream cycle immediately (foreground, shows progress)
    Run {
        /// Inference URL (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// Comma-separated additional inference URLs for parallel completion
        #[arg(long, value_name = "URLS")]
        inference_urls: Option<String>,

        /// Model name for completion calls
        #[arg(long, default_value = "default", value_name = "MODEL")]
        model: String,

        /// Only complete entities with overall score below this threshold [default: 0.6]
        #[arg(long, default_value = "0.6", value_name = "FLOAT")]
        threshold: f32,

        /// Embedding similarity for auto-merge [default: 0.92]
        #[arg(long, default_value = "0.92", value_name = "FLOAT")]
        dedup_threshold: f32,

        /// Maximum LLM completion calls this cycle [default: 50]
        #[arg(long, default_value = "50", value_name = "N")]
        max_completions: usize,

        /// Concurrent completion workers [default: 4]
        #[arg(long, default_value = "4", value_name = "N")]
        workers: usize,

        /// Prune entities with no chunk evidence whose overall score is below this value.
        /// Default (0.3) keeps type-only entities (33%); raise to 0.4 to prune them.
        #[arg(long, default_value = "0.3", value_name = "FLOAT")]
        prune_threshold: f32,

        /// Skip relation extraction — only improve schema_type and description.
        /// Use this on graphs with seeded family relations to prevent the LLM from
        /// adding spurious parent_of / child_of / spouse_of edges from co-mentions.
        #[arg(long)]
        no_relations: bool,
    },

    /// Show the last dream cycle report
    Status,

    /// Evaluate retrieval quality: measures recall@k and MRR against entity evidence chunks.
    /// Used to correlate graph completeness score with downstream RAG quality for the paper.
    EmbedEval {
        /// Max number of entity queries to run [default: all entities with evidence]
        #[arg(long, value_name = "N")]
        max_queries: Option<usize>,

        /// Save the JSON report to a file
        #[arg(long, value_name = "PATH")]
        output: Option<std::path::PathBuf>,

        /// Include per-entity breakdown in JSON output
        #[arg(long)]
        verbose: bool,

        /// Print results as JSON
        #[arg(long)]
        json: bool,
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

        /// Number of parallel extraction workers (default: 1).
        /// Each worker holds one Ollama request slot.
        /// Values > 1 overlap LLM calls; combine with --inference-urls for true multi-machine speedup.
        #[arg(long, default_value = "1", value_name = "N")]
        workers: usize,

        /// Comma-separated Ollama base URLs for round-robin multi-endpoint dispatch.
        /// Overrides --inference-url when set.
        /// Example: --inference-urls "http://node1:11434,http://node2:11434"
        #[arg(long, value_name = "URLS")]
        inference_urls: Option<String>,

        /// Restrict extraction to these entity types (comma-separated).
        /// Example: --entity-types "Person,Place,Organization"
        /// Default: all 15 types.
        #[arg(long, value_name = "TYPES")]
        entity_types: Option<String>,

        /// Skip relation extraction entirely (entities only).
        #[arg(long)]
        no_relations: bool,

        /// Number of adjacent chunks to include as surrounding context per extraction call.
        /// 0 = current chunk only. 1 = one before + one after (default, +7pp recall).
        /// Experiments show window=1 is optimal; window=2 adds cost with no recall gain.
        #[arg(long, default_value = "1", value_name = "N")]
        graph_window: usize,

        /// Wipe the graph before building (entities + relations cleared, chunks preserved).
        #[arg(long)]
        reset_graph: bool,

        /// Process only this percentage of chunks (1–100). Useful for quick test cycles.
        /// Example: --sample 10 processes the first 10% of chunks.
        #[arg(long, value_name = "PERCENT")]
        sample_pct: Option<u8>,

        /// Base URL of a running GLiNER NER server (e.g. http://localhost:8080).
        /// When set, Person spans are detected before each LLM call and injected as
        /// high-confidence hints into the extraction prompt. Requires `scripts/gliner_server.py`.
        #[arg(long, value_name = "URL")]
        gliner_url: Option<String>,

        /// Use entity-centric extraction instead of chunk-centric.
        /// Requires --gliner-url. Phase 1 scans all chunks with GLiNER to collect unique
        /// person names; Phase 2 makes one focused LLM call per name, aggregating all
        /// chunks where that person is mentioned. Trades more LLM calls for better
        /// per-entity descriptions. Use with --sample-pct for comparison runs.
        #[arg(long)]
        entity_centric: bool,

        /// Process N consecutive chunks per LLM call (default 1).
        /// With --chunk-batch 3, the loop strides by 3 so 12 chunks → 4 calls, each
        /// covering 3 consecutive chunks plus the context window on each side.
        /// Reduces LLM calls by N× at the cost of denser context per call.
        #[arg(long, value_name = "N", default_value = "1")]
        chunk_batch: usize,

        /// After the CC build, escalate entities whose confidence score is below this
        /// threshold to a focused EC refinement pass. 0.0 = disabled (default).
        /// Score is computed by score_entity() (type + summary + relation completeness).
        /// Suggested starting value: 0.45.
        #[arg(long, value_name = "THRESHOLD", default_value = "0.0")]
        ec_refine_threshold: f32,

        /// Maximum number of entities to escalate per run (cost guard). Default 50.
        #[arg(long, value_name = "N", default_value = "50")]
        ec_refine_budget: usize,

        /// Skip the CC extraction phase: re-score existing entities and run only the
        /// EC refinement pass using chunks from the current corpus slice.
        /// Requires --ec-refine-threshold > 0. Use after dream cycles for a surgical
        /// second pass without re-extracting.
        #[arg(long)]
        ec_refine_only: bool,
    },

    /// Reverse a bad dedup merge: remove an alias from a canonical entity and restore it as its
    /// own stub entity. Run `graph reembed --kb <KB>` afterwards to give the restored entity a
    /// proper embedding. Accepts a --pairs-file with one "alias <- canonical" line per merge to fix.
    Unmerge {
        /// Entity type for both the canonical and alias (default: Person).
        #[arg(long, default_value = "Person", value_name = "TYPE")]
        entity_type: String,

        /// Canonical entity name to remove the alias from.
        #[arg(long, value_name = "NAME", conflicts_with = "pairs_file")]
        canonical: Option<String>,

        /// Alias name to split off from the canonical entity.
        #[arg(long, value_name = "NAME", conflicts_with = "pairs_file")]
        alias: Option<String>,

        /// Path to a file with one "alias <- canonical" line per bad merge to fix.
        #[arg(long, value_name = "FILE", conflicts_with_all = ["canonical", "alias"])]
        pairs_file: Option<std::path::PathBuf>,
    },

    /// Seed the graph from a ground-truth YAML family tree — upserts canonical entities with
    /// name+description embeddings, merges alias entities into their canonical, and plants
    /// family relations. Aliases declared in the YAML are stored on the canonical entity so
    /// name-token search finds them even after the alias entity is removed.
    Seed {
        /// Path to the YAML family tree file (see tests/kwaai-knowledge/d6_family_tree.yaml for format)
        #[arg(long, value_name = "FILE")]
        file: std::path::PathBuf,

        /// Knowledge base name (default: "default")
        #[arg(long, default_value = "default", value_name = "NAME")]
        kb: String,
    },

    /// Import ground-truth entities and relations from a NotebookLM JSON extraction.
    /// See tests/notebooklm_extraction_prompt.md for the prompt to use in NotebookLM.
    /// Low-confidence relations are skipped; all entities are included.
    SeedFromJson {
        /// Path to the JSON file output by NotebookLM
        #[arg(long, value_name = "FILE")]
        file: std::path::PathBuf,

        /// Also write the converted seed YAML to this path (optional)
        #[arg(long, value_name = "FILE")]
        emit_yaml: Option<std::path::PathBuf>,
    },

    /// Detect and merge duplicate entities.
    /// Tier 1 (exact normalized name match) is always auto-merged silently.
    /// Tier 2 (embedding similarity ≥ threshold) is shown for interactive review
    /// unless --auto or --dry-run is set.
    /// Merged aliases are preserved on the canonical entity so queries still find it.
    Dedup {
        /// Cosine similarity threshold for Tier 2 candidates [default: 0.85]
        #[arg(long, default_value = "0.85", value_name = "FLOAT")]
        threshold: f32,

        /// Auto-merge all candidates above 0.97 without interactive review
        #[arg(long)]
        auto: bool,

        /// Override the auto-merge similarity cutoff (default: 0.97).
        /// Set to 1.01 in pipeline/rebuild mode to run only Tier 1 + structural
        /// dedup without any Tier 2 embedding-similarity auto-merges.
        #[arg(long, value_name = "FLOAT")]
        auto_threshold: Option<f32>,

        /// Print candidates without merging anything
        #[arg(long)]
        dry_run: bool,
    },

    /// Re-embed all entities using "{name}: {description}" as the embedded text.
    /// Fixes abbreviation/acronym lookup (e.g. "J.M.H. Gool" finding its canonical entity).
    /// Run once after upgrading from a build that stored description-only embeddings,
    /// or after `graph seed` adds new aliases to canonical entities.
    Reembed {
        /// Embedding server URL (defaults to config embed_url)
        #[arg(long, value_name = "URL")]
        embed_url: Option<String>,
    },

    /// Re-embed every chunk that is linked to a graph entity, prepending [EntityName] to
    /// the embed text before calling the embedding model.  The original chunk text stored
    /// in the metadata store is NOT modified — only the vector in the vector store changes.
    ///
    /// Use --restore to undo: re-embeds the same entity-linked chunks using raw text only,
    /// restoring them to the same embedding as a fresh ingest.
    ChunkTag {
        /// Embedding server URL (defaults to config embed_url)
        #[arg(long, value_name = "URL")]
        embed_url: Option<String>,

        /// Maximum number of entity tags to prepend per chunk, ranked by mention count.
        /// Higher values improve multi-subject recall at the cost of embedding drift.
        #[arg(long, default_value = "3")]
        max_tags: usize,

        /// Undo a previous chunk-tag run: re-embed entity-linked chunks without the
        /// [EntityName] prefix, restoring natural ingest embeddings.
        #[arg(long)]
        restore: bool,
    },

    /// Remove entities that have no text evidence: no chunk links in the graph AND name
    /// not found in any MetaStore chunk.  These are typically dream-hallucinated entities
    /// Delete a specific entity by name and type. Use this to remove NER artifacts or
    /// incorrectly extracted multi-person blobs that ghost-prune won't catch.
    /// Run `graph reembed` afterwards to refresh embeddings.
    Delete {
        /// Entity name (case-insensitive exact match)
        name: String,
        /// Entity type (default: Person)
        #[arg(long, default_value = "Person", value_name = "TYPE")]
        entity_type: String,
    },

    /// that were never mentioned in the source documents.  Safe to run after dream cycles.
    ///
    /// By default only removes entities with no relations (isolated ghosts). Use --with-relations
    /// to also remove connected hallucinations.
    GhostPrune {
        /// Also remove ghost entities that have relations (connected hallucinations).
        /// Without this flag only truly isolated (no relations, no text) entities are removed.
        #[arg(long)]
        with_relations: bool,

        /// Dry-run: report how many entities would be pruned without deleting them.
        #[arg(long)]
        dry_run: bool,
    },

    /// Score every entity across three pillars: type (schema.org), summary, and relationships.
    /// Reports overall graph health and surfaces the worst-scoring entities.
    Score {
        /// Show only the N lowest-scoring entities (default: 20)
        #[arg(long, default_value = "20", value_name = "N")]
        top: usize,

        /// Output full results as JSON instead of the human-readable table
        #[arg(long)]
        json: bool,
    },

    /// Scan ingested chunk text for inline abbreviation definitions ("Full Name (ABBR)")
    /// and merge the abbreviated entity into the full-name entity.
    /// This is more accurate than algorithmic acronym expansion because the source text
    /// itself defines the abbreviation — no false positives.
    AliasScan {
        /// Auto-merge all unambiguous pairs (single full-name match) without prompting
        #[arg(long)]
        auto: bool,

        /// Print candidates without merging anything
        #[arg(long)]
        dry_run: bool,

        /// Minimum number of chunks a pair must appear in to be considered (default: 1)
        #[arg(long, default_value = "1", value_name = "N")]
        min_hits: usize,
    },

    /// Resolve pronouns and definite descriptions to canonical Person entities,
    /// then add chunk links to those entities in the graph.
    ///
    /// Three-tier approach:
    ///   Tier 1 — alias matching ("Grandpa" → entity with alias "grandpa")
    ///   Tier 1 — gender + nearest-Person pronoun resolution (rule-based, 0.9 confidence)
    ///   Tier 2 — LLM-assisted for uncertain cases (EC refinement pattern, 0.7 confidence)
    ///
    /// Run BEFORE `extract-relations` — coref adds the narrator to pronoun chunks,
    /// fixing the narrator-endpoint problem in the CC+EC relation extraction.
    /// Also surfaces dedup candidates: if two entity stubs coref to the same referent
    /// in the same chunk, they are likely duplicates.
    Coref {
        /// Inference URL for LLM tier (default: localhost:11434)
        #[arg(long, default_value = "http://localhost:11434", value_name = "URL")]
        inference_url: String,

        /// Model for LLM-assisted tier
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Fraction of chunks to process (default: all)
        #[arg(long, default_value = "1.0", value_name = "FRAC")]
        sample: f64,

        /// ±N adjacent chunks to include when building candidate antecedents
        #[arg(long, default_value = "2", value_name = "N")]
        window: usize,

        /// Write a review markdown file instead of printing to stdout
        #[arg(long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,

        /// Persist resolved links to the graph (default: dry-run only)
        #[arg(long)]
        commit: bool,

        /// Skip the LLM tier — run only rule-based resolution (Tier 1).
        /// Much faster; suitable for full-corpus runs where alias_match and
        /// gender_nearest provide most of the value.
        #[arg(long)]
        no_llm: bool,
    },

    /// Remove a specific relation from the knowledge graph.
    /// Also removes the logical inverse (e.g. parent_of ↔ child_of) and
    /// symmetric reverse (sibling_of, spouse_of stored in both directions).
    RemoveRelation {
        /// Name of the source entity
        #[arg(long, value_name = "NAME")]
        from: String,
        /// Relation type (e.g. spouse_of, sibling_of, parent_of)
        #[arg(long, value_name = "TYPE")]
        relation: String,
        /// Name of the target entity
        #[arg(long, value_name = "NAME")]
        to: String,
    },

    /// Extract family relations between known co-occurring entities.
    /// Finds chunks linked to ≥ 2 graph entities, applies a lexical-trigger filter
    /// for family-relation keywords, then calls the LLM with a focused prompt that asks
    /// only for spouse_of / parent_of / child_of / sibling_of between the listed entities.
    /// Writes a human-readable review file. Use --commit to persist results to the graph.
    ExtractRelations {
        /// Inference URL (default: http://localhost:11434)
        #[arg(long, default_value = "http://localhost:11434", value_name = "URL")]
        inference_url: String,

        /// Model for relation extraction
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Fraction of qualifying chunks to process (0.01 = 1%, 1.0 = all)
        #[arg(long, default_value = "0.01", value_name = "FRAC")]
        sample: f64,

        /// Write a review markdown file to this path (stdout if omitted)
        #[arg(long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,

        /// Actually persist extracted relations to the graph (default: dry-run only)
        #[arg(long)]
        commit: bool,

        /// List models available on the inference endpoint and exit
        #[arg(long)]
        list_models: bool,

        /// Pull this model on the (possibly remote) inference endpoint and exit.
        /// Useful for pulling a model on a remote node via p2p:// URL before extraction.
        #[arg(long, value_name = "MODEL")]
        pull: Option<String>,

        /// Use RC (Relation-Centric) extraction: anchor on each trigger-word occurrence
        /// and expand a window of ±N chars around it instead of a CC scan of the whole
        /// chunk. Analogous to the EC pass in ingestion — anchors on the relation word,
        /// not the entity. Window size can be set with --rc-window (default 200).
        #[arg(long)]
        rc: bool,

        /// Character window around each trigger word for RC extraction (default: 200)
        #[arg(long, default_value = "200", value_name = "N")]
        rc_window: usize,
    },

    /// Enforce relation integrity rules across the whole knowledge graph:
    ///   1. Remove familial relations (parent_of, spouse_of, sibling_of, …) where
    ///      either endpoint is not a Person entity.
    ///   2. Add missing logical inverses (parent_of ↔ child_of, grandparent_of ↔
    ///      grandchild_of, etc.) and missing symmetric directions (spouse_of, sibling_of).
    ///   3. Recompute relation strength from actual shared-evidence-chunk co-occurrence
    ///      so strength reflects how well-evidenced each relation is in the source text.
    ///   4. Infer and persist gender ("Male"/"Female") for all Person entities from
    ///      pronoun cues in their descriptions.
    ///   5. Log (warn) spouse_of pairs where both entities have the same inferred gender
    ///      so they can be reviewed.
    Sanitize,

    /// Load a doc-schema YAML and persist its metadata section into the knowledge base.
    /// Run this after a rebuild if you didn't use --doc-schema during ingest.
    SetMetadata {
        /// Path to the doc-schema YAML file
        #[arg(long, value_name = "FILE")]
        doc_schema: std::path::PathBuf,
    },

    /// Build LLM-generated descriptions and extract structured metadata (gender, etc.) for all
    /// entities from their evidence chunks. Person entities also get gender extracted from
    /// textual evidence (pronouns, titles, role words). Safe to re-run: existing non-empty
    /// values are preserved unless --force is set.
    EnrichEntities {
        /// Inference URL for the summarization LLM (default: http://localhost:11434)
        #[arg(long, default_value = "http://localhost:11434", value_name = "URL")]
        inference_url: String,

        /// Comma-separated inference URLs for round-robin multi-endpoint dispatch.
        /// Overrides --inference-url when set.
        #[arg(long, value_name = "URLS")]
        inference_urls: Option<String>,

        /// Model name (default: llama3.1:8b)
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Parallel LLM workers (default: 4)
        #[arg(long, default_value = "4", value_name = "N")]
        workers: usize,

        /// Only enrich entities with at least this many mentions (default: 2)
        #[arg(long, default_value = "2", value_name = "N")]
        min_mentions: u32,

        /// Comma-separated entity types to enrich (default: Person,Place,Organization)
        #[arg(
            long,
            default_value = "Person,Place,Organization",
            value_name = "TYPES"
        )]
        entity_types: String,

        /// Maximum number of entities to enrich in this run (default: all)
        #[arg(long, value_name = "N")]
        limit: Option<usize>,

        /// Overwrite fields that already have a value (default: skip populated fields)
        #[arg(long)]
        force: bool,

        /// Skip gender extraction for Person entities (default: extract gender)
        #[arg(long)]
        no_gender: bool,
    },

    /// Propose entity merges derived from marriage (spouse_of) relations.
    /// For each spouse pair, identifies the female entity by gender or shared surname,
    /// then searches for other Person entities with the same first name but a different
    /// surname — likely pre-marriage (maiden name) forms of the same person.
    /// Prints proposals by default. Use --commit to merge them into the graph.
    CorefMarriage {
        /// Write review markdown to this file instead of stdout
        #[arg(long, value_name = "FILE")]
        output: Option<std::path::PathBuf>,

        /// Actually merge the proposed entities (default: dry-run only)
        #[arg(long)]
        commit: bool,
    },

    /// Export the knowledge graph to an Obsidian vault
    Export {
        /// Output directory for the vault (created if absent)
        #[arg(long, value_name = "DIR")]
        output_dir: std::path::PathBuf,
    },

    /// Import curated edits from an Obsidian vault back into the knowledge graph
    Import {
        /// Path to the Obsidian vault directory (must contain `entities/`)
        #[arg(long, value_name = "DIR")]
        input_dir: std::path::PathBuf,

        /// Only process files modified after this Unix timestamp (default: 0 = all files)
        #[arg(long, default_value = "0", value_name = "SECS")]
        since: u64,
    },

    /// Build and query UML sequence diagrams: per-entity lifelines and interactions.
    /// Temporal questions ("when did…", "what happened to…") are routed here in smart mode.
    Timeline {
        #[command(subcommand)]
        action: TimelineAction,
    },
}

#[derive(Subcommand)]
pub enum TimelineAction {
    /// Extract temporal events from all entity-linked chunks and store them in the graph DB.
    /// Run after `graph build` to populate lifeline and interaction data.
    Build {
        /// Inference URL for event extraction (defaults to config inference_url)
        #[arg(long, value_name = "URL")]
        inference_url: Option<String>,

        /// LLM model name (e.g. "llama3.1:8b")
        #[arg(long, default_value = "llama3.1:8b", value_name = "MODEL")]
        model: String,

        /// Parallel extraction workers
        #[arg(long, default_value = "4", value_name = "N")]
        workers: usize,

        /// Drop all existing timeline data before building
        #[arg(long)]
        reset: bool,
    },

    /// Show the event lifeline for a named entity
    Show {
        /// Entity name (case-insensitive)
        #[arg(long, value_name = "NAME")]
        entity: String,
    },

    /// Render a Mermaid sequence diagram for a named entity (includes 1-hop neighbours)
    ExportMermaid {
        /// Entity name (case-insensitive)
        #[arg(long, value_name = "NAME")]
        entity: String,
    },

    /// Show event and interaction counts stored in the timeline tables
    Stats,
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
