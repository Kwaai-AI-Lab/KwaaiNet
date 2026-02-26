//! CLI argument definitions using clap

use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(
    name = "kwaainet",
    about = "KwaaiNet – Distributed AI node CLI",
    long_about = None,
    version,
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

    /// Initial setup
    Setup,

    /// Manage node identity and verifiable credentials
    Identity(IdentityArgs),

    /// Manage VPK (Virtual Private Knowledge) vector database integration
    Vpk(VpkArgs),

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
}

// ---------------------------------------------------------------------------
// config
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct ConfigArgs {
    /// View current configuration
    #[arg(long)]
    pub view: bool,

    /// Set a configuration key to a value
    #[arg(long, num_args = 2, value_names = ["KEY", "VALUE"])]
    pub set: Option<Vec<String>>,
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
    /// Model to benchmark (Ollama: `llama3.1:8b`, HF: `owner/model`).
    /// Defaults to the model in ~/.kwaainet/config.yaml.
    pub model: Option<String>,

    /// Number of decode steps to time (after a warm-up pass).
    #[arg(long, default_value = "20")]
    pub steps: usize,
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
