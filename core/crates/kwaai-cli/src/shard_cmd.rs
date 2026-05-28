//! Distributed block sharding commands.
//!
//! Implements `kwaainet shard <subcommand>`:
//!
//! - **serve**  — Load model shard and register inference RPC handler with p2pd.
//! - **run**    — Discover block circuit from DHT and run distributed inference.
//! - **status** — Show local shard configuration from config.yaml.
//! - **chain**  — Query DHT and display block coverage table.
//!
//! # Architecture
//!
//! ```text
//! kwaainet shard serve  →  TransformerShard  →  P2PClient::add_unary_handler
//! kwaainet shard run    →  discover_chain (DHT)  →  call_block_forward (RPC)
//! ```

use anyhow::{bail, Context, Result};
use kwaai_hivemind_dht::protocol::{FindRequest, FindResponse, NodeInfo, RequestAuthInfo};
use kwaai_inference::{DeviceType, TransformerShard};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
use libp2p::PeerId;
use prost::Message as _;
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;

use crate::block_rpc::{
    call_block_forward, f16_bytes_to_tensor, make_block_rpc_handler, token_ids_to_bytes,
    InferenceRequest, PayloadType, ShardCell,
};
use crate::cli::{
    CircuitAction, CircuitCloseArgs, CircuitCreateArgs, ShardAction, ShardArgs, ShardChainArgs,
    ShardDownloadArgs, ShardRunArgs, ShardServeArgs,
};
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::hf;
use crate::reputation::{now_secs, PeerObservation, ReputationStore};

// ── Entrypoint ────────────────────────────────────────────────────────────────

/// Outcome of a `cmd_shard_serve` invocation.
enum ShardServeExit {
    /// User pressed Ctrl-C — stop serving entirely.
    UserStop,
    /// Rebalancer signalled that blocks should move — re-run serve with `--auto`.
    Rebalance,
}

pub async fn run(args: ShardArgs) -> Result<()> {
    match args.action {
        ShardAction::Serve(a) => {
            // When --auto-rebalance is active we loop: after each rebalance
            // signal we re-run cmd_shard_serve so pick_gap_blocks() re-queries
            // the DHT and loads a fresh shard at the new block range.
            loop {
                match cmd_shard_serve(a.clone()).await? {
                    ShardServeExit::UserStop => break,
                    ShardServeExit::Rebalance => {
                        print_info("Rebalancing — re-discovering gap and reloading shard…");
                        // Next iteration calls pick_gap_blocks() fresh (default auto path).
                    }
                }
            }
            Ok(())
        }
        ShardAction::Run(a) => cmd_shard_run(a).await,
        ShardAction::Status => cmd_shard_status().await,
        ShardAction::Chain(a) => cmd_shard_chain(a).await,
        ShardAction::Api(a) => crate::shard_api::run(a).await,
        ShardAction::Download(a) => cmd_shard_download(a).await,
        ShardAction::Gap => cmd_shard_gap().await,
        ShardAction::Circuit(action) => match action {
            CircuitAction::Create(a) => cmd_circuit_create(a).await,
            CircuitAction::List => cmd_circuit_list().await,
            CircuitAction::Close(a) => cmd_circuit_close(a).await,
        },
    }
}

// ── download ──────────────────────────────────────────────────────────────────

pub async fn cmd_shard_download(args: ShardDownloadArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let model_id = args.model.as_deref().unwrap_or(&cfg.model).to_string();

    print_box_header("Downloading HuggingFace Model");
    println!("  Model: {}", model_id);
    println!();

    let snapshot_dir = match (args.start_block, args.blocks) {
        (Some(start), Some(blocks)) => {
            let total = cfg.model_total_blocks() as usize;
            let end = (start + blocks).min(total);
            let is_first = start == 0;
            let is_last = end >= total;
            println!(
                "  Blocks: [{}, {}) of {} (selective download)",
                start, end, total
            );
            println!();
            hf::download_for_blocks(
                &model_id,
                start,
                end,
                is_first,
                is_last,
                args.hf_token.as_deref(),
            )
            .await?
        }
        _ => hf::download(&model_id, args.hf_token.as_deref()).await?,
    };

    println!();
    print_success(&format!("Saved to: {}", snapshot_dir.display()));
    print_info(&format!(
        "Start serving: kwaainet shard serve --model-path \"{}\"",
        snapshot_dir.display()
    ));
    print_separator();
    Ok(())
}

// ── gap (dry-run gap detection) ───────────────────────────────────────────────

/// Dry-run gap detection: show what block range this node would auto-assign
/// without loading a model or registering an RPC handler.  Exit immediately.
/// Use this to validate gap detection locally in seconds.
async fn cmd_shard_gap() -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let daemon_addr = daemon_socket();
    let mut client = P2PClient::connect(&daemon_addr)
        .await
        .context("Cannot connect to node — start it first with `kwaainet start --daemon`")?;
    let peer_id_hex = client
        .identify()
        .await
        .context("Failed to get local peer ID")?;
    let our_peer_id =
        PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;

    let total = cfg.model_total_blocks() as usize;
    let target_blocks = cfg.blocks as usize;
    let prefix = cfg.effective_dht_prefix();
    let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg.initial_peers.clone()
    };

    print_box_header("🔍 Gap Detection Dry-Run");
    println!("  Peer ID:      {}", our_peer_id.to_base58());
    println!("  DHT prefix:   {}", prefix);
    println!("  Total blocks: {}", total);
    println!("  Target blocks:{}", target_blocks);
    println!();

    // Run discovery (includes retry on empty chain).
    print_info("Querying DHT…");
    let chain = discover_chain(&mut client, &our_peer_id, &prefix, total, &bootstrap_peers).await;
    let chain = if chain.is_empty() {
        print_info("DHT returned no peers — waiting 5 s and retrying…");
        tokio::time::sleep(Duration::from_secs(5)).await;
        discover_chain(&mut client, &our_peer_id, &prefix, total, &bootstrap_peers).await
    } else {
        chain
    };

    let others: Vec<_> = chain.iter().filter(|e| e.peer_id != our_peer_id).collect();
    println!("  Other nodes visible: {}", others.len());
    for e in &others {
        println!(
            "    {} → [{}, {})",
            e.peer_id.to_base58(),
            e.start_block,
            e.end_block
        );
    }
    println!();

    let (start, end) =
        crate::rebalancer::pick_gap_from_chain(&chain, &our_peer_id, total, target_blocks);

    let is_gap = chain
        .iter()
        .filter(|e| e.peer_id != our_peer_id)
        .any(|_| true)
        && {
            let mut cov = vec![0usize; total];
            for e in chain.iter().filter(|e| e.peer_id != our_peer_id) {
                for c in &mut cov[e.start_block.min(total)..e.end_block.min(total)] {
                    *c += 1;
                }
            }
            cov.iter().copied().min().unwrap_or(0) == 0
        };

    if others.is_empty() {
        print_success(&format!(
            "Would serve [{start}, {end}) — first node on network"
        ));
    } else if is_gap {
        print_success(&format!(
            "Would serve [{start}, {end}) — fills a genuine gap"
        ));
    } else {
        print_success(&format!(
            "Would serve [{start}, {end}) — joins as redundant (network fully covered)"
        ));
    }
    print_separator();
    Ok(())
}

// ── serve ─────────────────────────────────────────────────────────────────────

async fn cmd_shard_serve(args: ShardServeArgs) -> Result<ShardServeExit> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    if crate::daemon::ShardManager::new().is_running() {
        print_warning("A shard server is already running (started via `kwaainet start --shard`).");
        print_info("If intentional, proceed — DHT announcements will overlap.");
    }

    let target_blocks = snap_to_valid_blocks(args.blocks.unwrap_or(cfg.blocks) as usize);

    // ── Gap detection — also yields a P2PClient we reuse for handler registration
    // to avoid a drop/reconnect race that causes "early eof" from p2pd.
    let (start_block, end_block, initial_client) = if !args.no_auto
        && (args.auto || args.start_block.is_none())
    {
        let daemon_addr = daemon_socket();
        let mut qc = P2PClient::connect(&daemon_addr)
            .await
            .context("Cannot connect to node — start it first with `kwaainet start --daemon`")?;
        let peer_id_hex = qc.identify().await.context("Failed to get local peer ID")?;
        let our_peer_id =
            PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;
        let total = cfg.model_total_blocks() as usize;
        let prefix_owned = cfg.effective_dht_prefix();
        let prefix = prefix_owned.as_str();
        let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
            NetworkConfig::with_petals_bootstrap().bootstrap_peers
        } else {
            cfg.initial_peers.clone()
        };

        // Stagger: spread DHT queries across 0–8 s using the last byte of our
        // peer ID.  Prevents nodes started simultaneously from all querying
        // before any announcement has propagated and all landing on block 0.
        let stagger_ms = (our_peer_id.to_bytes().last().copied().unwrap_or(0) as u64 % 8) * 1000;
        if stagger_ms > 0 {
            print_info(&format!(
                "Staggering DHT query by {}s (peer-ID jitter)…",
                stagger_ms / 1000
            ));
            tokio::time::sleep(Duration::from_millis(stagger_ms)).await;
        }

        print_info(&format!(
            "Querying DHT for gap in {} ({} blocks)…",
            prefix, total
        ));
        let (s, e) = pick_gap_blocks(
            &mut qc,
            &our_peer_id,
            prefix,
            total,
            target_blocks,
            &bootstrap_peers,
        )
        .await?;
        print_success(&format!("Auto-assigned blocks [{}, {})", s, e));

        // Only update config + signal daemon when the range actually changed.
        // If pick_gap returns the same range already in config the daemon is
        // already announcing the correct blocks — nothing to do.
        if s as u32 != cfg.start_block {
            let mut updated = cfg.clone();
            updated.start_block = s as u32;
            updated.save().context("Failed to save config.yaml")?;
            print_info("Updated config.yaml — signalling daemon to re-announce…");
            crate::daemon::DaemonManager::new().signal_reannounce();
        }

        // Reuse qc for handler registration — avoids a drop/reconnect race
        // that causes "early eof" from p2pd on rapid reconnect.
        (s, e, Some(qc))
    } else {
        // Explicit range: --start-block N was given, or --no-auto was passed.
        // Falls back to config.start_block when neither --start-block nor --no-auto gave a value.
        let s = args.start_block.unwrap_or(cfg.start_block) as usize;
        let total = cfg.model_total_blocks() as usize;
        let e = (s + target_blocks).min(total);
        let mut updated = cfg.clone();
        updated.start_block = s as u32;
        updated.blocks = (e - s) as u32;
        if updated.start_block != cfg.start_block || updated.blocks != cfg.blocks {
            updated.save().context("Failed to save config.yaml")?;
            print_info("Updated config.yaml — signalling daemon to re-announce…");
            crate::daemon::DaemonManager::new().signal_reannounce();
        }
        (s, e, None::<P2PClient>)
    };

    // Detect device early (before any model I/O).
    let device_type = if args.no_gpu {
        DeviceType::Cpu
    } else if args.use_gpu {
        DeviceType::require_gpu().context("--use-gpu was specified but no GPU is available")?
    } else if cfg.use_gpu {
        DeviceType::detect_best_logged()
    } else {
        DeviceType::Cpu
    };
    let device = device_type
        .to_candle_device()
        .context("Failed to create compute device")?;

    // ── Phase 1 complete — connect to p2pd and register placeholder handler ──
    // The node will appear on the map immediately while the model loads in the
    // background.  Inference requests arriving before loading completes receive
    // a structured "warming up" error response.
    let daemon_addr = daemon_socket();
    let client = match initial_client {
        Some(c) => c,
        None => match P2PClient::connect(&daemon_addr).await {
            Ok(c) => c,
            Err(_) => {
                print_error("Cannot connect to the KwaaiNet node — is it running?");
                print_info("Start it:     kwaainet start --daemon");
                print_info("Check status: kwaainet status");
                print_info("View logs:    kwaainet logs --follow");
                print_separator();
                bail!("KwaaiNet node is not running");
            }
        },
    };

    // Shared cell: None until the background load task writes Some(shard).
    let shard_cell: ShardCell = Arc::new(RwLock::new(None));

    let handler = make_block_rpc_handler(shard_cell.clone(), device.clone());
    client
        .add_unary_handler(crate::block_rpc::INFERENCE_PROTO, handler, false)
        .await
        .context("Failed to register inference handler with p2pd")?;

    // Ollama proxy — lets remote nodes route LLM requests to our local Ollama.
    let proxy_handler = crate::ollama_proxy::make_ollama_proxy_handler();
    let _ = client
        .add_unary_handler(
            crate::ollama_proxy::OLLAMA_PROXY_PROTO,
            proxy_handler,
            false,
        )
        .await;

    // Shard proxy — lets remote nodes route requests to our local shard API (no Ollama needed).
    let shard_proxy_handler = crate::ollama_proxy::make_shard_proxy_handler();
    let _ = client
        .add_unary_handler(
            crate::ollama_proxy::SHARD_PROXY_PROTO,
            shard_proxy_handler,
            false,
        )
        .await;

    print_box_header("🧩 KwaaiNet Shard Server");
    println!("  Blocks:      [{}, {})", start_block, end_block);
    println!("  Device:      {}", device_type);
    println!("  Model:       {}", cfg.model);
    println!();
    print_success(&format!(
        "Node registered on protocol {} — appearing on map.",
        crate::block_rpc::INFERENCE_PROTO
    ));
    print_info("Loading model in background. Requests return 'warming up' until ready.");
    print_separator();

    // Start local TCP bypass server so `shard run` on the same machine can
    // call us without triggering libp2p's "dial to self" rejection.
    let _ = std::fs::create_dir_all(crate::config::run_dir());
    match start_local_inference_server(shard_cell.clone(), device.clone()).await {
        Ok(port) => {
            if let Err(e) = std::fs::write(local_server_port_file(), port.to_string()) {
                tracing::warn!("Could not write shard_local.port: {e}");
            } else {
                print_info(&format!("Local bypass server on 127.0.0.1:{}", port));
            }
        }
        Err(e) => tracing::warn!("Could not start local bypass server: {e}"),
    }

    // ── Phase 2+3: download (if needed) + load in background task ─────────────
    let cell_bg = shard_cell.clone();
    let model_id_bg = cfg.model.clone();
    let model_path_bg = args.model_path.clone();
    let hf_token_bg = args.hf_token.clone();
    let device_bg = device.clone();
    let total_blocks_bg = cfg.model_total_blocks() as usize;

    tokio::spawn(async move {
        let result: anyhow::Result<()> = async {
            // Resolve model directory: CLI override > cached snapshot > download.
            let model_dir: PathBuf = if let Some(p) = model_path_bg {
                p
            } else {
                let is_first = start_block == 0;
                let is_last = end_block >= total_blocks_bg;
                let cached = hf::resolve_snapshot(&model_id_bg).ok().filter(|d| {
                    hf::blocks_are_cached(d, start_block, end_block, is_first, is_last)
                });
                match cached {
                    Some(d) => d,
                    None => {
                        print_info(&format!(
                            "Downloading model files for blocks [{}, {})…",
                            start_block, end_block
                        ));
                        hf::download_for_blocks(
                            &model_id_bg,
                            start_block,
                            end_block,
                            is_first,
                            is_last,
                            hf_token_bg.as_deref(),
                        )
                        .await
                        .context("selective download for blocks")?
                    }
                }
            };

            let config_path = model_dir.join("config.json");
            let safetensors: Vec<PathBuf> = collect_safetensors(&model_dir)?;
            if safetensors.is_empty() {
                anyhow::bail!(
                    "No .safetensors files found in {}. \
                     Pass --model-path to a HuggingFace snapshot directory.",
                    model_dir.display()
                );
            }
            let paths: Vec<&Path> = safetensors.iter().map(|p| p.as_path()).collect();

            print_info(&format!(
                "Loading shard ({} file(s), blocks [{}, {}))…",
                safetensors.len(),
                start_block,
                end_block
            ));
            let shard = Arc::new(
                TransformerShard::load(&paths, &config_path, &device_bg, start_block, end_block)
                    .context("Failed to load transformer shard")?,
            );

            print_success(&format!(
                "Shard ready  ({} blocks, is_first={}, is_last={})",
                end_block - start_block,
                shard.is_first(),
                shard.is_last()
            ));

            // Make the shard available to the RPC handler.
            *cell_bg.write().await = Some(shard.clone());

            // Signal daemon that inference is live — daemon will re-announce
            // with real block coverage instead of [0, 0).
            let ready_file = crate::daemon::ShardManager::ready_file();
            let _ = std::fs::write(&ready_file, "");
            crate::daemon::DaemonManager::new().signal_reannounce();

            // Background GC task: evict idle sessions every 30 s.
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    shard.gc_sessions();
                }
            });

            Ok(())
        }
        .await;

        if let Err(e) = result {
            print_error(&format!("Background model load failed: {e:#}"));
            print_info("Node will continue serving — requests will return 'warming up'.");
            print_info("Fix the error above and restart `kwaainet shard serve`.");
        }
    });

    // ── Rebalancer task ───────────────────────────────────────────────────────
    // Spawn a background task that periodically checks DHT coverage and signals
    // the main wait loop to exit with Rebalance when a block move is warranted.
    // When --auto-rebalance is not requested we use a never-resolving future so
    // tokio::select! compiles with the same shape in both branches.

    let cfg_rb = KwaaiNetConfig::load_or_create()?;
    let do_rebalance = args.auto_rebalance;
    let interval_secs = cfg_rb.rebalance_interval_secs;
    let min_redundancy = cfg_rb.rebalance_min_redundancy;
    let total_blocks_rb = cfg_rb.model_total_blocks() as usize;
    let target_blocks_rb = args.blocks.unwrap_or(cfg_rb.blocks) as usize;
    let dht_prefix_rb = cfg_rb.effective_dht_prefix();
    let bootstrap_peers_rb: Vec<String> = if cfg_rb.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg_rb.initial_peers.clone()
    };
    let daemon_addr_rb = daemon_socket();

    // oneshot used by the rebalancer to signal the main loop.
    let rebalance_fut: std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> =
        if do_rebalance {
            let (rebalance_tx, rebalance_rx) = tokio::sync::oneshot::channel::<()>();

            tokio::spawn(async move {
                // Jitter: 0–60 s derived from our peer ID's last byte so nodes with
                // the same rebalance_interval_secs don't all fire at the same instant.
                let jitter_secs: u64 = if let Ok(mut c) = P2PClient::connect(&daemon_addr_rb).await
                {
                    if let Ok(h) = c.identify().await {
                        hex::decode(&h)
                            .ok()
                            .and_then(|b| b.last().copied())
                            .unwrap_or(0) as u64
                            % 60
                    } else {
                        0
                    }
                } else {
                    0
                };
                tokio::time::sleep(Duration::from_secs(jitter_secs)).await;

                let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs.max(10)));
                // Skip the first (immediate) tick — we just loaded the shard.
                ticker.tick().await;
                loop {
                    ticker.tick().await;

                    // Connect to p2pd to identify ourselves and query DHT.
                    let mut c = match P2PClient::connect(&daemon_addr_rb).await {
                        Ok(c) => c,
                        Err(_) => {
                            tracing::warn!("Rebalancer: cannot connect to p2pd, skipping check");
                            continue;
                        }
                    };
                    let hex = match c.identify().await {
                        Ok(h) => h,
                        Err(_) => {
                            tracing::warn!("Rebalancer: identify failed, skipping check");
                            continue;
                        }
                    };
                    let pid = match hex::decode(&hex)
                        .ok()
                        .and_then(|b| PeerId::from_bytes(&b).ok())
                    {
                        Some(p) => p,
                        None => {
                            tracing::warn!("Rebalancer: could not parse peer ID, skipping");
                            continue;
                        }
                    };

                    let chain = discover_chain(
                        &mut c,
                        &pid,
                        &dht_prefix_rb,
                        total_blocks_rb,
                        &bootstrap_peers_rb,
                    )
                    .await;

                    if crate::rebalancer::check_rebalance(
                        &chain,
                        &pid,
                        start_block,
                        end_block,
                        total_blocks_rb,
                        target_blocks_rb,
                        min_redundancy,
                    )
                    .is_some()
                    {
                        print_info(&format!(
                            "Rebalance: blocks [{start_block},{end_block}) have \
                             ≥{min_redundancy} other node(s); gap detected — moving."
                        ));
                        let _ = rebalance_tx.send(());
                        break;
                    }
                    print_info("Rebalance check: coverage OK, no move needed.");
                }
            });

            Box::pin(async move {
                let _ = rebalance_rx.await;
            })
        } else {
            Box::pin(futures::future::pending::<()>())
        };

    // ── Wait: Ctrl-C or rebalance signal ─────────────────────────────────────
    let exit = tokio::select! {
        res = tokio::signal::ctrl_c() => {
            res.context("ctrl-c handler")?;
            ShardServeExit::UserStop
        }
        _ = rebalance_fut => {
            ShardServeExit::Rebalance
        }
    };

    let _ = std::fs::remove_file(local_server_port_file());
    let _ = std::fs::remove_file(crate::daemon::ShardManager::ready_file());
    println!();
    match exit {
        ShardServeExit::UserStop => print_info("Shard server stopped."),
        ShardServeExit::Rebalance => print_info("Shard server stopping for rebalance."),
    }
    Ok(exit)
}

// ── run --local (in-process, no networking) ───────────────────────────────────

/// Load the model in-process and run inference without any P2P or TCP overhead.
/// Used by `shard run --local` for single-machine testing.
async fn cmd_shard_run_local(args: ShardRunArgs) -> Result<()> {
    use kwaai_inference::tokenizer::Tokenizer as _;

    let cfg = KwaaiNetConfig::load_or_create()?;
    let model_ref = args.model.as_deref().unwrap_or(&cfg.model);

    print_box_header("🔗 KwaaiNet Local Inference");
    println!("  Model:  {}", model_ref);
    println!("  Prompt: {:?}", args.prompt);
    println!("  Device: {}", if args.no_gpu { "CPU" } else { "auto" });
    println!();

    // Resolve model path
    let model_dir = if let Some(p) = &args.model_path {
        p.clone()
    } else {
        hf::resolve_snapshot(model_ref)?
    };

    // Load tokenizer
    let tokenizer_path = model_dir.join("tokenizer.json");
    let tokenizer = kwaai_inference::tokenizer::BpeTokenizer::from_file(&tokenizer_path)
        .context("Failed to load tokenizer")?;

    // Apply chat template
    let formatted_prompt = if tokenizer.token_to_id("<|start_header_id|>").is_some() {
        format!(
            "<|start_header_id|>user<|end_header_id|>\n\n{}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n",
            args.prompt
        )
    } else if tokenizer.token_to_id("<|im_start|>").is_some() {
        format!(
            "<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            args.prompt
        )
    } else {
        args.prompt.clone()
    };

    let mut token_ids: Vec<u32> = tokenizer
        .encode(&formatted_prompt)
        .context("Failed to encode prompt")?;
    if let Some(bos) = tokenizer.bos_token_id() {
        token_ids.insert(0, bos);
    }
    let eos_id = tokenizer.eos_token_id().unwrap_or(2);

    // If `shard serve` is already running on this machine, reuse its loaded model
    // via the local TCP bypass server instead of loading the model a second time.
    let local_port: Option<u16> = std::fs::read_to_string(local_server_port_file())
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .filter(|&port| {
            // Quick liveness check: verify the port is still accepting connections.
            // A stale port file is common after an unclean shutdown.
            std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
        });
    if let Some(port) = local_port {
        print_info("Shard serve detected — reusing loaded model (skipping model load)");
        println!();
        let n_input = token_ids.len();
        return run_via_local_bypass(
            port,
            &tokenizer,
            token_ids,
            eos_id,
            args.session_id.unwrap_or_else(rand_session_id),
            args.max_tokens,
            args.temperature,
            args.top_k,
            args.top_p,
            n_input,
            args.stats,
        )
        .await;
    }

    let device_type = if args.no_gpu {
        kwaai_inference::DeviceType::Cpu
    } else {
        kwaai_inference::DeviceType::detect_best_logged()
    };
    let device = device_type
        .to_candle_device()
        .context("Failed to create compute device")?;

    // Load shard covering all blocks
    let config_path = model_dir.join("config.json");
    let total_blocks = args
        .total_blocks
        .unwrap_or_else(|| cfg.model_total_blocks() as usize);

    // Discover safetensors shards
    let paths: Vec<std::path::PathBuf> = {
        let mut p: Vec<_> = std::fs::read_dir(&model_dir)
            .context("read model dir")?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("safetensors"))
            .collect();
        p.sort();
        p
    };
    if paths.is_empty() {
        bail!("No .safetensors files found in {}", model_dir.display());
    }

    let load_spinner = crate::progress::Spinner::start(format!(
        "Loading model — {} block(s)  {} shard file(s)",
        total_blocks,
        paths.len()
    ));

    let paths_owned = paths.clone();
    let config_path_owned = config_path.clone();
    let device_clone = device.clone();
    let shard = Arc::new(
        tokio::task::spawn_blocking(move || {
            let refs: Vec<&std::path::Path> = paths_owned.iter().map(|p| p.as_path()).collect();
            TransformerShard::load(&refs, &config_path_owned, &device_clone, 0, total_blocks)
        })
        .await
        .context("model load task panicked")?
        .context("Failed to load model")?,
    );

    load_spinner
        .finish(format!("✓ Model loaded  {} blocks", total_blocks))
        .await;
    println!("  Input tokens: {}", token_ids.len());
    println!();

    let session_id: u64 = args.session_id.unwrap_or_else(rand_session_id);
    let max_tokens = args.max_tokens;
    let temperature = args.temperature;
    let top_k = args.top_k;
    let top_p = args.top_p;
    let n_input = token_ids.len();

    let mut generated_ids: Vec<u32> = Vec::new();
    let mut seq_pos: usize = 0;
    let mut current_ids = token_ids.clone();

    let mut prefill_spinner: Option<crate::progress::Spinner> = Some(
        crate::progress::Spinner::start(format!("Prefilling {} input tokens", n_input)),
    );
    let mut gen_bar = crate::progress::GenBar::new(max_tokens);
    let mut token_times_ms: Vec<f64> = Vec::new();

    use std::io::Write as _;

    loop {
        let token_start = std::time::Instant::now();

        // Run full forward pass in-process (already spawn_blocking inside local path)
        let logits = tokio::task::spawn_blocking({
            let shard = shard.clone();
            let ids = current_ids.clone();
            let sp = seq_pos;
            move || shard.forward_full(session_id, &ids, sp)
        })
        .await
        .context("join forward_full")?
        .context("forward_full")?;

        // Stop prefill spinner on first token
        if generated_ids.is_empty() {
            let prefill_ms = token_start.elapsed().as_secs_f64() * 1000.0;
            if let Some(sp) = prefill_spinner.take() {
                sp.finish(format!(
                    "✓ Prefill  {:.0} ms  ({} input tokens)",
                    prefill_ms, n_input
                ))
                .await;
            }
            println!();
            println!("  Assistant:");
            print!("  ");
            std::io::stdout().flush().ok();
        }

        // logits shape: [1, seq_len, vocab] or [vocab]
        let last_logits = {
            let dims = logits.dims();
            if dims.len() == 3 && dims[1] > 1 {
                use candle_core::IndexOp as _;
                logits.i((0, dims[1] - 1, ..))?
            } else {
                logits.flatten_all()?
            }
        };

        // Move logits to CPU for sampling
        let last_logits = last_logits.to_device(&candle_core::Device::Cpu)?;
        let next_id = sample_token(&last_logits, temperature, top_k, top_p)? as u32;

        if let Ok(piece) = tokenizer.decode(&[next_id]) {
            print!("{}", piece);
            std::io::stdout().flush().ok();
        }

        let token_ms = token_start.elapsed().as_secs_f64() * 1000.0;
        token_times_ms.push(token_ms);
        generated_ids.push(next_id);
        seq_pos += current_ids.len();

        if generated_ids.len() > 1 {
            gen_bar.tick(generated_ids.len(), token_ms);
        }

        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }

        current_ids = vec![next_id];
    }

    let n = generated_ids.len();
    let total_secs = token_times_ms.iter().sum::<f64>() / 1000.0;
    let tps = gen_bar.tps();

    println!();
    println!();
    print_success(&format!(
        "Generated {} tok  •  {:.1} tok/s  •  {:.1}s",
        n, tps, total_secs
    ));
    print_separator();
    Ok(())
}

// ── run ───────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_run(args: ShardRunArgs) -> Result<()> {
    // --local: bypass all networking, load model in-process and infer directly.
    if args.local {
        return cmd_shard_run_local(args).await;
    }

    let cfg = KwaaiNetConfig::load_or_create()?;

    let model_ref = args.model.as_deref().unwrap_or(&cfg.model);
    // If a model was passed on the CLI that differs from config, build a temporary
    // config with that model name so effective_dht_prefix() derives the right key.
    let dht_prefix = if args.model.is_some() && args.model.as_deref() != Some(&cfg.model) {
        let base = model_ref.split('/').next_back().unwrap_or(model_ref);
        base.replace('.', "-")
    } else {
        cfg.effective_dht_prefix()
    };
    let total_blocks = args
        .total_blocks
        .unwrap_or_else(|| cfg.model_total_blocks() as usize);

    print_box_header("🔗 KwaaiNet Distributed Inference");
    println!("  Model:        {}", model_ref);
    println!("  DHT prefix:   {}", dht_prefix);
    println!("  Total blocks: {}", total_blocks);
    println!("  Prompt:       {:?}", args.prompt);
    println!();

    // Connect to p2pd
    let daemon_addr = daemon_socket();
    let mut client = match P2PClient::connect(&daemon_addr).await {
        Ok(c) => c,
        Err(_) => {
            print_error("Cannot connect to the KwaaiNet node — is it running?");
            print_info("Start it:     kwaainet start --daemon");
            print_info("Check status: kwaainet status");
            print_info("View logs:    kwaainet logs --follow");
            print_separator();
            bail!("KwaaiNet node is not running");
        }
    };

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id =
        PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;

    // ── Resolve chain: from circuit or fresh DHT discovery ─────────────────
    let (chain, using_circuit) = if let Some(ref circuit_id) = args.circuit {
        // Load pre-formed circuit — skip DHT discovery
        let mut circuit = load_circuit_by_id(circuit_id)?;
        circuit.last_used_epoch = now_epoch();

        // Persist updated last_used timestamp
        let mut all = load_circuits();
        if let Some(c) = all.iter_mut().find(|c| c.id == circuit.id) {
            c.last_used_epoch = circuit.last_used_epoch;
        }
        let _ = save_circuits(&all);

        let entries: Vec<BlockServerEntry> =
            circuit.chain.iter().filter_map(|e| e.to_entry()).collect();
        println!("  Circuit:      {}", circuit.id);
        println!("  Nodes:        {} (from circuit)", entries.len());
        (entries, true)
    } else {
        // Fresh DHT discovery
        print!("  Discovering block circuit from DHT…");
        let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
            NetworkConfig::with_petals_bootstrap().bootstrap_peers
        } else {
            cfg.initial_peers.clone()
        };

        let chain = discover_chain(
            &mut client,
            &our_peer_id,
            &dht_prefix,
            total_blocks,
            &bootstrap_peers,
        )
        .await;

        // Apply optional name filter (e.g. --name-filter v0.2.3)
        let chain = if let Some(ref f) = args.name_filter {
            let filtered: Vec<_> = chain
                .into_iter()
                .filter(|e| e.public_name.contains(f.as_str()))
                .collect();
            if filtered.is_empty() {
                println!("no nodes matched filter {:?}", f);
                print_warning(&format!(
                    "No block servers with name containing {:?} found.",
                    f
                ));
                print_separator();
                return Ok(());
            }
            filtered
        } else {
            chain
        };

        if chain.is_empty() {
            println!("no nodes found");
            println!();
            print_warning("No block servers found in DHT for this model.");
            print_info("Start serving with: kwaainet shard serve --model <path>");
            print_separator();
            return Ok(());
        }
        (chain, false)
    };
    let _ = using_circuit; // used for display above
    println!("{} node(s)", chain.len());

    // Load reputation store and enrich chain entries with local trust scores.
    let rep_store = if cfg.reputation.enabled {
        let store = ReputationStore::load();
        let mut enriched = chain;
        for entry in &mut enriched {
            let peer_b58 = entry.peer_id.to_base58();
            let score = store.score(&peer_b58);
            entry.trust_score = Some(score.score);
        }
        // Re-assign so borrow checker is happy.
        let rep = Arc::new(std::sync::Mutex::new(store));
        (enriched, Some(rep))
    } else {
        (chain, None)
    };
    let (chain, reputation) = rep_store;

    // Validate coverage
    let covered = coverage_check(&chain, total_blocks);
    if !covered {
        print_warning("Block circuit has gaps — inference may be incomplete.");
    }

    println!();
    for (i, entry) in chain.iter().enumerate() {
        let tier_label = if let Some(ref rep) = reputation {
            if let Ok(store) = rep.lock() {
                format!(
                    "  {}",
                    store.score(&entry.peer_id.to_base58()).tier.as_str()
                )
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        println!(
            "  [{:>2}] blocks {:>3}–{:>3}  {}  ({}){}",
            i + 1,
            entry.start_block,
            entry.end_block - 1,
            truncate_str(&entry.peer_id.to_base58(), 17),
            truncate_str(&entry.public_name, 30),
            tier_label,
        );
    }
    println!();

    // Load tokenizer from the model directory for prompt encoding
    let model_dir = if let Some(p) = &args.model_path {
        p.clone()
    } else {
        hf::resolve_snapshot(model_ref)?
    };
    let tokenizer_path = model_dir.join("tokenizer.json");
    let tokenizer = kwaai_inference::tokenizer::BpeTokenizer::from_file(&tokenizer_path)
        .context("Failed to load tokenizer")?;

    use kwaai_inference::tokenizer::Tokenizer as _;

    // Apply chat template based on tokenizer vocab, then tokenize.
    // Instruct models require special header tokens around the user turn.
    let formatted_prompt = if tokenizer.token_to_id("<|start_header_id|>").is_some() {
        // Llama-3 instruct format
        format!(
            "<|start_header_id|>user<|end_header_id|>\n\n{}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n",
            args.prompt
        )
    } else if tokenizer.token_to_id("<|im_start|>").is_some() {
        // ChatML format (Mistral-Instruct, Qwen, etc.)
        format!(
            "<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            args.prompt
        )
    } else {
        // Base models / Llama-2: raw prompt
        args.prompt.clone()
    };

    let mut token_ids: Vec<u32> = tokenizer
        .encode(&formatted_prompt)
        .context("Failed to encode prompt")?;

    // Prepend BOS token if available
    if let Some(bos) = tokenizer.bos_token_id() {
        token_ids.insert(0, bos);
    }

    let eos_id = tokenizer.eos_token_id().unwrap_or(2);
    let session_id: u64 = args.session_id.unwrap_or_else(rand_session_id);
    let max_tokens = args.max_tokens;
    let temperature = args.temperature;
    let top_k = args.top_k;
    let top_p = args.top_p;

    println!("  Input tokens: {}", token_ids.len());
    println!("  Session ID:   {}", session_id);
    println!("  Max tokens:   {}", max_tokens);
    print_separator();

    // Connect to all block-server peers
    for entry in &chain {
        let multiaddr_hint = format!("/p2p/{}", entry.peer_id.to_base58());
        let _ = client.connect_peer(&multiaddr_hint).await;
        // best effort — may already be connected
    }

    // ── Inference loop ────────────────────────────────────────────────────────
    let mut generated_ids: Vec<u32> = Vec::new();
    let mut seq_pos: usize = 0;
    let mut current_ids = token_ids.clone();
    let mut failed_peers: std::collections::HashSet<PeerId> = std::collections::HashSet::new();

    // Pin the peer path for this session — same peers handle the same blocks
    // on every token so their KV-caches stay coherent.
    let mut pinned_path = build_pinned_path(&chain, total_blocks, &failed_peers)?;

    println!("  Pinned path:");
    for (i, entry) in pinned_path.iter().enumerate() {
        println!(
            "    [{:>2}] blocks {:>3}–{:>3}  {}",
            i + 1,
            entry.start_block,
            entry.end_block - 1,
            entry.public_name,
        );
    }
    println!();

    let show_stats = args.stats;
    let mut token_times_ms: Vec<f64> = Vec::new();
    // Per-token hop breakdowns: outer index = token, inner = hops for that token.
    let mut all_hop_timings: Vec<Vec<HopTiming>> = Vec::new();
    let generation_start = std::time::Instant::now();

    let n_input = token_ids.len();
    let mut prefill_spinner: Option<crate::progress::Spinner> = Some(
        crate::progress::Spinner::start(format!("Prefilling {n_input} input token(s)")),
    );
    let mut gen_bar = crate::progress::GenBar::new(max_tokens);

    loop {
        let token_start = std::time::Instant::now();

        // Build first request
        let (shape, data) = token_ids_to_bytes(&current_ids);
        let request = InferenceRequest {
            session_id,
            seq_pos: seq_pos as u32,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };

        // Forward through the pinned path
        let mut token_hops: Vec<HopTiming> = Vec::new();
        let logits_bytes = match forward_through_chain(
            &mut client,
            &pinned_path,
            total_blocks,
            session_id,
            seq_pos as u32,
            request,
            Some(&our_peer_id),
            &mut failed_peers,
            show_stats.then_some(&mut token_hops),
            reputation.clone(),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                // Move to a clean line before printing the warning so it doesn't
                // appear inline with the last-printed token.
                if !generated_ids.is_empty() {
                    println!();
                }
                print_warning(&format!(
                    "{e:#} — rebuilding path (KV-cache lost, output may degrade)"
                ));
                pinned_path = build_pinned_path(&chain, total_blocks, &failed_peers)?;
                token_hops.clear();
                // Retry this token with the new path
                let (shape2, data2) = token_ids_to_bytes(&current_ids);
                let retry_req = InferenceRequest {
                    session_id,
                    seq_pos: seq_pos as u32,
                    payload_type: PayloadType::TokenIds,
                    shape: shape2,
                    data: data2,
                };
                let result = forward_through_chain(
                    &mut client,
                    &pinned_path,
                    total_blocks,
                    session_id,
                    seq_pos as u32,
                    retry_req,
                    Some(&our_peer_id),
                    &mut failed_peers,
                    show_stats.then_some(&mut token_hops),
                    reputation.clone(),
                )
                .await?;
                // Re-indent so the resumed token stream starts cleanly.
                if !generated_ids.is_empty() {
                    use std::io::Write as _;
                    print!("  ");
                    std::io::stdout().flush().ok();
                }
                result
            }
        };
        if show_stats && !token_hops.is_empty() {
            all_hop_timings.push(token_hops);
        }

        // Stop prefill spinner and print "Assistant:" header after the first forward pass.
        if generated_ids.is_empty() {
            let prefill_ms = token_start.elapsed().as_secs_f64() * 1000.0;
            if let Some(sp) = prefill_spinner.take() {
                sp.finish(format!(
                    "✓ Prefill  {prefill_ms:.0} ms  ({n_input} input token(s))"
                ))
                .await;
            }
            println!();
            println!("  Assistant:");
            print!("  ");
            use std::io::Write as _;
            std::io::stdout().flush().ok();
        }

        // logits_bytes.data is f16 bytes of shape [1, 1, vocab_size] or [1, seq_len, vocab_size]
        // We need only the last position
        let logits_shape = &logits_bytes.shape;
        let device = candle_core::Device::Cpu;
        let logits_tensor = f16_bytes_to_tensor(&logits_bytes.data, logits_shape, &device)
            .context("decode logits tensor")?;

        // Take last token position: [1, seq_len, vocab_size] → [vocab_size]
        let last_logits = if logits_shape.len() == 3 && logits_shape[1] > 1 {
            use candle_core::IndexOp as _;
            let seq_len = logits_shape[1] as usize;
            logits_tensor.i((0, seq_len - 1, ..))?
        } else {
            // Shape [1, 1, vocab_size] or [vocab_size]
            logits_tensor.flatten_all()?
        };

        let next_id = sample_token(&last_logits, temperature, top_k, top_p)? as u32;

        // Decode and print incrementally
        if let Ok(piece) = tokenizer.decode(&[next_id]) {
            print!("{}", piece);
            use std::io::Write as _;
            std::io::stdout().flush().ok();
        }

        let token_ms = token_start.elapsed().as_secs_f64() * 1000.0;
        token_times_ms.push(token_ms);

        generated_ids.push(next_id);
        seq_pos += current_ids.len(); // advance by tokens sent this step

        if generated_ids.len() > 1 {
            gen_bar.tick(generated_ids.len(), token_ms);
        }

        // Stopping conditions
        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }

        // Next decode step: send just the new token
        current_ids = vec![next_id];
    }

    let total_secs = generation_start.elapsed().as_secs_f64();
    let n = generated_ids.len();
    let tps = gen_bar.tps();

    println!();
    println!();
    print_success(&format!(
        "Generated {} tok  •  {:.1} tok/s  •  {:.1}s",
        n, tps, total_secs
    ));

    if show_stats && !token_times_ms.is_empty() {
        let prefill_ms = token_times_ms[0];
        let decode_times: &[f64] = if token_times_ms.len() > 1 {
            &token_times_ms[1..]
        } else {
            &[]
        };
        let decode_avg_ms = if decode_times.is_empty() {
            0.0
        } else {
            decode_times.iter().sum::<f64>() / decode_times.len() as f64
        };
        let decode_min = decode_times.iter().copied().fold(f64::INFINITY, f64::min);
        let decode_max = decode_times
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let tps = if total_secs > 0.0 {
            n as f64 / total_secs
        } else {
            0.0
        };

        println!();
        println!("  ── Timing ────────────────────────────────────────────────");
        println!(
            "  Prefill:       {:>8.0}ms  ({} input tokens)",
            prefill_ms,
            token_ids.len()
        );
        if !decode_times.is_empty() {
            println!(
                "  Decode avg:    {:>8.0}ms/tok  (min {:.0}, max {:.0})",
                decode_avg_ms, decode_min, decode_max
            );
        }
        println!("  Total:         {:>8.1}s", total_secs);
        println!("  Throughput:    {:>8.1} tok/s", tps);
        println!("  Hops:          {:>8}", pinned_path.len());

        // Per-hop breakdown: average elapsed across all decode tokens (skip prefill at index 0).
        if !all_hop_timings.is_empty() {
            let decode_hops: Vec<&Vec<HopTiming>> = all_hop_timings.iter().skip(1).collect();
            if !decode_hops.is_empty() {
                println!();
                println!("  ── Per-hop (decode avg) ──────────────────────────────────");
                // Use first token's hops as the slot template (path is pinned, so slots are stable).
                let n_hops = decode_hops[0].len();
                for slot in 0..n_hops {
                    let times: Vec<f64> = decode_hops
                        .iter()
                        .filter_map(|tok| tok.get(slot).map(|h| h.3))
                        .collect();
                    if times.is_empty() {
                        continue;
                    }
                    let avg = times.iter().sum::<f64>() / times.len() as f64;
                    let min = times.iter().copied().fold(f64::INFINITY, f64::min);
                    let max = times.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let (name, start, end, _) = &decode_hops[0][slot];
                    let blocks = end - start;
                    println!(
                        "  [{slot}] {name:<35} blocks {start:>3}–{end:>3} ({blocks:>2} blks)  \
                         avg {avg:>6.0}ms  min {min:.0}  max {max:.0}",
                    );
                }
            }
        }
    }

    print_separator();

    Ok(())
}

// ── streaming API ─────────────────────────────────────────────────────────────
//
// `run_streaming` is the non-printing twin of [`cmd_shard_run`]. It drives the
// same end-to-end distributed-inference path (connect p2pd → DHT discovery →
// chain pinning → tokenize → per-token forward loop) but yields events on a
// stream instead of writing to stdout. The in-process gRPC server uses this
// to power the GUI's `Session::ShardRun` operation without re-implementing the
// dispatch logic.
//
// The CLI `cmd_shard_run` deliberately remains a separate path so its existing
// stdout decoration (box headers, spinners, per-hop stats, etc.) is unchanged
// and verifiable byte-for-byte. The shared substrate is the existing pub
// helpers — `discover_chain`, `build_pinned_path`, `forward_through_chain`,
// `sample_token`, `f16_bytes_to_tensor`, `token_ids_to_bytes`,
// `BpeTokenizer::from_file` — so the only duplication is the ~50-line outer
// loop driver, which differs between the two consumers anyway.

/// Options for [`run_streaming`].
pub struct ShardRunOptions {
    /// User prompt to format and run inference against.
    pub prompt: String,
    /// HF model id override. `None` means use `config.model`.
    pub model: Option<String>,
    /// Generation cap. `None` means the same default as `cmd_shard_run`
    /// uses today (200).
    pub max_tokens: Option<usize>,
    /// Pre-formed circuit id to use instead of fresh DHT discovery.
    pub circuit_id: Option<String>,
}

/// Events emitted by [`run_streaming`]. Every successful generation ends with
/// `Done`; every failure ends with `Error`. No other terminal markers.
pub enum ShardRunEvent {
    /// A decoded text piece for the next generated token. Multiple `Token`s
    /// arrive before the terminator.
    Token(String),
    /// Generation finished cleanly (EOS hit or `max_tokens` reached).
    Done,
    /// Unrecoverable error mid-generation. No further events follow.
    Error(anyhow::Error),
}

/// Drive distributed inference end-to-end and yield [`ShardRunEvent`]s as
/// tokens are produced. The final event is always `Done` or `Error`.
///
/// Behaviour notes vs [`cmd_shard_run`]:
///
/// - No stdout writes from this function. Side effects from helpers
///   (notably `print_warning` inside `forward_through_chain` on transient
///   peer errors) are unchanged — they're acceptable in the daemon context
///   today and out of scope for this refactor.
/// - The local node is honoured as a hop when it appears in the discovered
///   chain — `forward_through_chain` already calls into the local TCP bypass
///   server for `our_peer_id`, just like the CLI path.
/// - On an empty chain (no peers serving the model) we poll every 2 s for up
///   to 30 s before yielding
///   `Error(anyhow!("no peers serving model …"))`.
/// - Opens its own `P2PClient` via `daemon_socket()` — matches the CLI and
///   avoids contending with `run_node`'s ownership of the daemon's primary
///   client. p2pd accepts concurrent IPC connections, so this is fine.
pub fn run_streaming(opts: ShardRunOptions) -> impl futures::Stream<Item = ShardRunEvent> {
    // We run the actual generation on a dedicated task and pipe events back
    // through an mpsc channel. This avoids the awkwardness of `?`-across-
    // `yield` inside `async_stream::stream!` and means the worker can use
    // ordinary Result propagation. The stream! body just forwards.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ShardRunEvent>(64);

    tokio::spawn(async move {
        let result = run_streaming_inner(opts, tx.clone()).await;
        match result {
            Ok(()) => {
                let _ = tx.send(ShardRunEvent::Done).await;
            }
            Err(e) => {
                let _ = tx.send(ShardRunEvent::Error(e)).await;
            }
        }
    });

    async_stream::stream! {
        while let Some(ev) = rx.recv().await {
            yield ev;
        }
    }
}

/// Worker that does all the I/O for `run_streaming` and sends each generated
/// piece into `tx` as a `ShardRunEvent::Token`. The terminator (`Done` or
/// `Error`) is emitted by the caller in `run_streaming` based on the
/// `Result` returned from here, so the worker never sends `Done`/`Error`
/// itself.
async fn run_streaming_inner(
    opts: ShardRunOptions,
    tx: tokio::sync::mpsc::Sender<ShardRunEvent>,
) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let model_ref = opts.model.as_deref().unwrap_or(&cfg.model).to_string();
    let dht_prefix = if opts.model.is_some() && opts.model.as_deref() != Some(&cfg.model) {
        let base = model_ref.split('/').next_back().unwrap_or(&model_ref);
        base.replace('.', "-")
    } else {
        cfg.effective_dht_prefix()
    };
    let total_blocks = cfg.model_total_blocks() as usize;

    // Default max_tokens matches the CLI's clap default (see ShardRunArgs).
    let max_tokens = opts.max_tokens.unwrap_or(200);
    // Sampling defaults also mirror ShardRunArgs.
    let temperature: f32 = 1.0;
    let top_k: usize = 0;
    let top_p: f32 = 1.0;

    // p2pd connection — separate client from `run_node`'s; concurrent
    // IPC connections are supported by p2pd.
    let mut client = P2PClient::connect(&daemon_socket())
        .await
        .context("connecting to p2pd (is `kwaainet start` running?)")?;

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id =
        PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;

    // ── Resolve chain (circuit or fresh DHT discovery + 30 s wait) ──
    let chain: Vec<BlockServerEntry> = if let Some(ref cid) = opts.circuit_id {
        let mut circuit = load_circuit_by_id(cid)?;
        circuit.last_used_epoch = now_epoch();
        let mut all = load_circuits();
        if let Some(c) = all.iter_mut().find(|c| c.id == circuit.id) {
            c.last_used_epoch = circuit.last_used_epoch;
        }
        let _ = save_circuits(&all);
        circuit.chain.iter().filter_map(|e| e.to_entry()).collect()
    } else {
        let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
            NetworkConfig::with_petals_bootstrap().bootstrap_peers
        } else {
            cfg.initial_peers.clone()
        };

        // Poll DHT up to 30 s for peers serving this model.
        let deadline = std::time::Instant::now() + Duration::from_secs(30);
        let mut chain = discover_chain(
            &mut client,
            &our_peer_id,
            &dht_prefix,
            total_blocks,
            &bootstrap_peers,
        )
        .await;
        while chain.is_empty() && std::time::Instant::now() < deadline {
            tokio::time::sleep(Duration::from_secs(2)).await;
            chain = discover_chain(
                &mut client,
                &our_peer_id,
                &dht_prefix,
                total_blocks,
                &bootstrap_peers,
            )
            .await;
        }
        if chain.is_empty() {
            bail!("no peers serving model {dht_prefix} (waited 30s)");
        }
        chain
    };

    // Enrich with reputation scores if enabled (mirrors cmd_shard_run).
    let (chain, reputation) = if cfg.reputation.enabled {
        let store = ReputationStore::load();
        let mut enriched = chain;
        for entry in &mut enriched {
            let peer_b58 = entry.peer_id.to_base58();
            entry.trust_score = Some(store.score(&peer_b58).score);
        }
        (enriched, Some(Arc::new(std::sync::Mutex::new(store))))
    } else {
        (chain, None)
    };

    // ── Tokenize / format prompt (matches cmd_shard_run exactly) ──
    let model_dir = hf::resolve_snapshot(&model_ref)?;
    let tokenizer_path = model_dir.join("tokenizer.json");
    let tokenizer = kwaai_inference::tokenizer::BpeTokenizer::from_file(&tokenizer_path)
        .context("Failed to load tokenizer")?;
    use kwaai_inference::tokenizer::Tokenizer as _;

    let formatted_prompt = if tokenizer.token_to_id("<|start_header_id|>").is_some() {
        format!(
            "<|start_header_id|>user<|end_header_id|>\n\n{}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n",
            opts.prompt
        )
    } else if tokenizer.token_to_id("<|im_start|>").is_some() {
        format!(
            "<|im_start|>user\n{}\n<|im_end|>\n<|im_start|>assistant\n",
            opts.prompt
        )
    } else {
        opts.prompt.clone()
    };

    let mut token_ids: Vec<u32> = tokenizer
        .encode(&formatted_prompt)
        .context("Failed to encode prompt")?;
    if let Some(bos) = tokenizer.bos_token_id() {
        token_ids.insert(0, bos);
    }
    let eos_id = tokenizer.eos_token_id().unwrap_or(2);
    let session_id: u64 = rand_session_id();

    // Best-effort dial of every server in the chain (matches CLI).
    for entry in &chain {
        let _ = client
            .connect_peer(&format!("/p2p/{}", entry.peer_id.to_base58()))
            .await;
    }

    // Pin the path for this session.
    let mut failed_peers: std::collections::HashSet<PeerId> = std::collections::HashSet::new();
    let mut pinned_path = build_pinned_path(&chain, total_blocks, &failed_peers)?;

    let mut generated_ids: Vec<u32> = Vec::new();
    let mut seq_pos: usize = 0;
    let mut current_ids = token_ids.clone();

    loop {
        let (shape, data) = token_ids_to_bytes(&current_ids);
        let request = InferenceRequest {
            session_id,
            seq_pos: seq_pos as u32,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };

        let logits_bytes = match forward_through_chain(
            &mut client,
            &pinned_path,
            total_blocks,
            session_id,
            seq_pos as u32,
            request,
            Some(&our_peer_id),
            &mut failed_peers,
            None,
            reputation.clone(),
        )
        .await
        {
            Ok(r) => r,
            Err(_) => {
                // Rebuild path on transient failure and retry once,
                // matching cmd_shard_run's recovery behaviour.
                pinned_path = build_pinned_path(&chain, total_blocks, &failed_peers)?;
                let (shape2, data2) = token_ids_to_bytes(&current_ids);
                let retry_req = InferenceRequest {
                    session_id,
                    seq_pos: seq_pos as u32,
                    payload_type: PayloadType::TokenIds,
                    shape: shape2,
                    data: data2,
                };
                forward_through_chain(
                    &mut client,
                    &pinned_path,
                    total_blocks,
                    session_id,
                    seq_pos as u32,
                    retry_req,
                    Some(&our_peer_id),
                    &mut failed_peers,
                    None,
                    reputation.clone(),
                )
                .await?
            }
        };

        let logits_shape = &logits_bytes.shape;
        let device = candle_core::Device::Cpu;
        let logits_tensor = f16_bytes_to_tensor(&logits_bytes.data, logits_shape, &device)
            .context("decode logits tensor")?;
        let last_logits = if logits_shape.len() == 3 && logits_shape[1] > 1 {
            use candle_core::IndexOp as _;
            let seq_len = logits_shape[1] as usize;
            logits_tensor.i((0, seq_len - 1, ..))?
        } else {
            logits_tensor.flatten_all()?
        };
        let next_id = sample_token(&last_logits, temperature, top_k, top_p)? as u32;

        if let Ok(piece) = tokenizer.decode(&[next_id]) {
            // If the consumer has dropped, stop generating — they're gone.
            if tx.send(ShardRunEvent::Token(piece)).await.is_err() {
                return Ok(());
            }
        }

        generated_ids.push(next_id);
        seq_pos += current_ids.len();

        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }
        current_ids = vec![next_id];
    }

    Ok(())
}

// ── status ────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_status() -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    print_box_header("🧩 KwaaiNet Shard Status");
    println!("  Model:        {}", cfg.model);
    println!("  Start block:  {}", cfg.start_block);
    println!("  Blocks:       {}", cfg.blocks);
    println!(
        "  Range:        [{}, {})",
        cfg.start_block,
        cfg.effective_end_block()
    );
    println!("  GPU:          {}", cfg.use_gpu);
    println!("  DHT prefix:   {}", cfg.effective_dht_prefix());
    println!();
    print_info("To serve this shard: kwaainet shard serve");
    print_info("To change range:     kwaainet config --set start_block 4");
    print_separator();

    Ok(())
}

// ── chain ─────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_chain(args: ShardChainArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    let dht_prefix = args
        .dht_prefix
        .clone()
        .unwrap_or_else(|| cfg.effective_dht_prefix());

    let total_blocks = args.total_blocks;

    print_box_header("🗺  KwaaiNet Block Circuit");
    println!("  Model prefix: {}", dht_prefix);
    println!("  Querying {} blocks from DHT…", total_blocks);
    println!();

    let daemon_addr = daemon_socket();
    let mut client = match P2PClient::connect(&daemon_addr).await {
        Ok(c) => c,
        Err(_) => {
            print_error("Cannot connect to the KwaaiNet node — is it running?");
            print_info("Start it:     kwaainet start --daemon");
            print_info("Check status: kwaainet status");
            print_info("View logs:    kwaainet logs --follow");
            print_separator();
            bail!("KwaaiNet node is not running");
        }
    };

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id =
        PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;

    let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg.initial_peers.clone()
    };

    let chain = discover_chain(
        &mut client,
        &our_peer_id,
        &dht_prefix,
        total_blocks,
        &bootstrap_peers,
    )
    .await;

    if chain.is_empty() {
        print_warning("No block servers found in DHT.");
        print_info("Start serving with: kwaainet shard serve");
        print_separator();
        return Ok(());
    }

    // Load reputation store for trust tier display.
    let rep_store = if cfg.reputation.enabled {
        Some(ReputationStore::load())
    } else {
        None
    };

    // Build coverage bitmap
    let mut covered = vec![false; total_blocks];
    for entry in &chain {
        covered[entry.start_block..entry.end_block.min(total_blocks)].fill(true);
    }
    let n_covered = covered.iter().filter(|&&c| c).count();

    println!(
        "  {:>3} server(s) — {}/{} blocks covered\n",
        chain.len(),
        n_covered,
        total_blocks
    );
    println!(
        "  {:<5}  {:<5}  {:<18} {:<10} NAME",
        "START", "END", "PEER ID (prefix)", "TRUST"
    );
    println!("  {}", "─".repeat(65));
    for entry in &chain {
        let peer_short = truncate_str(&entry.peer_id.to_base58(), 17);
        let tier_label = if let Some(ref store) = rep_store {
            store.score(&entry.peer_id.to_base58()).tier.as_str()
        } else {
            "—"
        };
        println!(
            "  {:>5}  {:>5}  {:<18} {:<10} {}",
            entry.start_block,
            entry.end_block,
            peer_short,
            tier_label,
            truncate_str(&entry.public_name, 24),
        );
    }
    println!();

    // Coverage bar
    print!("  Blocks: [");
    for &c in &covered {
        print!("{}", if c { "█" } else { "░" });
    }
    println!("]");
    println!();

    if n_covered < total_blocks {
        print_warning(&format!(
            "Gaps detected: {} block(s) not served",
            total_blocks - n_covered
        ));
    } else {
        print_success("Full model coverage — distributed inference ready");
    }
    print_separator();

    Ok(())
}

// ── Chain discovery ───────────────────────────────────────────────────────────

/// Metadata for one block-server node discovered from DHT.
#[derive(Debug, Clone)]
pub struct BlockServerEntry {
    pub peer_id: PeerId,
    pub start_block: usize,
    pub end_block: usize,
    pub public_name: String,
    /// Local trust score for this peer (None until enriched from ReputationStore).
    pub trust_score: Option<f64>,
}

/// Query bootstrap peers for all block keys of `dht_prefix` and return a
/// sorted, deduplicated list of [`BlockServerEntry`].
pub async fn discover_chain(
    client: &mut P2PClient,
    our_peer_id: &PeerId,
    dht_prefix: &str,
    total_blocks: usize,
    bootstrap_peers: &[String],
) -> Vec<BlockServerEntry> {
    let our_dhtid = Sha1::new()
        .chain_update(our_peer_id.to_bytes())
        .finalize()
        .to_vec();

    // All block keys in a single FindRequest
    let keys: Vec<Vec<u8>> = (0..total_blocks)
        .map(|b| block_dht_id(dht_prefix, b))
        .collect();

    let find_req = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys,
        peer: Some(NodeInfo { node_id: our_dhtid }),
    };
    let mut req_bytes = Vec::new();
    if find_req.encode(&mut req_bytes).is_err() {
        return vec![];
    }

    let mut servers: HashMap<String, BlockServerEntry> = HashMap::new();

    for addr in bootstrap_peers {
        let Some(peer_str) = addr.split("/p2p/").nth(1) else {
            continue;
        };
        let bp = match peer_str.parse::<PeerId>() {
            Ok(p) => p,
            Err(_) => continue,
        };
        if client.connect_peer(addr).await.is_err() {
            continue;
        }
        tokio::time::sleep(Duration::from_millis(400)).await;

        let resp_bytes = match client
            .call_unary_handler(&bp.to_bytes(), "DHTProtocol.rpc_find", &req_bytes)
            .await
        {
            Ok(b) => b,
            Err(_) => continue,
        };

        let Ok(resp) = FindResponse::decode(&resp_bytes[..]) else {
            continue;
        };

        for result in resp.results {
            if result.value.is_empty() {
                continue;
            }
            let rt = result.result_type;
            if rt == 1 {
                // FoundRegular — single value, peer_id embedded in map
                if let Some((key, entry)) = decode_server_info_regular(&result.value) {
                    servers.entry(key).or_insert(entry);
                }
            } else if rt == 2 {
                // FoundDictionary — multiple subkeys (Python Hivemind)
                decode_server_info_dictionary(&result.value, &mut servers);
            }
        }
    }

    let mut chain: Vec<BlockServerEntry> = servers.into_values().collect();
    chain.sort_by_key(|e| e.start_block);
    chain
}

// ── Gap detection ─────────────────────────────────────────────────────────────

/// Query the DHT and return the best `(start, end)` block range for this node.
///
/// Delegates all coverage logic to `rebalancer::pick_gap_from_chain()` so the
/// algorithm is unit-testable without a live daemon.  Never returns an error:
/// if the network is fully covered we join the least-covered window instead.
async fn pick_gap_blocks(
    client: &mut P2PClient,
    our_peer_id: &PeerId,
    dht_prefix: &str,
    total_blocks: usize,
    target_blocks: usize,
    bootstrap_peers: &[String],
) -> Result<(usize, usize)> {
    let chain = discover_chain(
        client,
        our_peer_id,
        dht_prefix,
        total_blocks,
        bootstrap_peers,
    )
    .await;

    // If the chain is empty the bootstrap peers may not have received other
    // nodes' announcements yet (propagation lag, simultaneous start).
    // Retry once after a short wait before defaulting to block 0.
    let chain = if chain.is_empty() {
        print_info("DHT returned no peers — waiting 5 s and retrying…");
        tokio::time::sleep(Duration::from_secs(5)).await;
        discover_chain(
            client,
            our_peer_id,
            dht_prefix,
            total_blocks,
            bootstrap_peers,
        )
        .await
    } else {
        chain
    };

    print_info(&format!(
        "DHT chain: {} other node(s) visible{}",
        chain.iter().filter(|e| e.peer_id != *our_peer_id).count(),
        if chain.iter().all(|e| e.peer_id == *our_peer_id) {
            " — joining as first node"
        } else {
            ""
        }
    ));

    let (start, end) =
        crate::rebalancer::pick_gap_from_chain(&chain, our_peer_id, total_blocks, target_blocks);

    // Log when joining as redundant (network is fully covered by others).
    let other_min_cov = {
        let mut cov = vec![0usize; total_blocks];
        for e in &chain {
            if e.peer_id == *our_peer_id {
                continue;
            }
            let s = e.start_block.min(total_blocks);
            let e2 = e.end_block.min(total_blocks);
            for c in &mut cov[s..e2] {
                *c += 1;
            }
        }
        cov.iter().copied().min().unwrap_or(0)
    };
    if other_min_cov > 0 {
        print_info(&format!(
            "Network fully covered (min {} node(s)/block) — \
             joining [{}, {}) as redundant.",
            other_min_cov, start, end
        ));
    }

    Ok((start, end))
}

// ── Server info decoding ──────────────────────────────────────────────────────

/// Parse `Ext(64, [state, throughput, {start_block, end_block, peer_id, …}])`
/// from a FoundRegular value.
///
/// Returns `(dedup_key, entry)`.  Legacy nodes (pre-v0.3.3) omit `peer_id`; we
/// synthesise a stable key from `public_name:start_block` so they still count
/// for gap detection even though they cannot be routed to directly.
fn decode_server_info_regular(bytes: &[u8]) -> Option<(String, BlockServerEntry)> {
    let (state, start_block, end_block, public_name, peer_id_b58, version) =
        decode_server_info_ext(bytes)?;
    // Only include ONLINE nodes (state=2); skip JOINING (0) and OFFLINE (-1).
    if state != 2 {
        return None;
    }
    if !version_meets_minimum(&version) {
        return None;
    }
    let (dedup_key, peer_id) = match peer_id_b58.parse::<PeerId>() {
        Ok(pid) => (pid.to_base58(), pid),
        Err(_) => {
            let key = format!("legacy:{}:{}", public_name, start_block);
            (key, PeerId::random())
        }
    };
    Some((
        dedup_key,
        BlockServerEntry {
            peer_id,
            start_block,
            end_block,
            public_name,
            trust_score: None,
        },
    ))
}

/// Parse `Ext(80, [expiry, created, [[subkey_bytes, value_bytes, expiry], …]])`
/// from a FoundDictionary value. Appends into `out` (deduplicates by peer_id).
fn decode_server_info_dictionary(bytes: &[u8], out: &mut HashMap<String, BlockServerEntry>) {
    let outer = match rmpv::decode::read_value(&mut &bytes[..]) {
        Ok(v) => v,
        Err(_) => return,
    };
    let inner_bytes = match &outer {
        rmpv::Value::Ext(80, b) => b.as_slice(),
        _ => return,
    };
    let inner = match rmpv::decode::read_value(&mut &inner_bytes[..]) {
        Ok(v) => v,
        Err(_) => return,
    };
    let outer_arr = match inner.as_array() {
        Some(a) if a.len() >= 3 => a,
        _ => return,
    };
    let entries = match outer_arr[2].as_array() {
        Some(e) => e,
        None => return,
    };

    for entry in entries {
        let arr = match entry.as_array() {
            Some(a) if a.len() >= 2 => a,
            _ => continue,
        };

        // Subkey is rmp_serde::to_vec(&peer_id_base58) = msgpack(string)
        let peer_id_b58 = match &arr[0] {
            rmpv::Value::String(s) => s.as_str().unwrap_or("").to_string(),
            rmpv::Value::Binary(b) => {
                // Decode as msgpack string
                match rmpv::decode::read_value(&mut b.as_slice()) {
                    Ok(rmpv::Value::String(s)) => s.as_str().unwrap_or("").to_string(),
                    _ => continue,
                }
            }
            _ => continue,
        };

        let value_bytes = match &arr[1] {
            rmpv::Value::Binary(b) => b.as_slice(),
            _ => continue,
        };

        if peer_id_b58.is_empty() {
            continue;
        }

        let peer_id = match peer_id_b58.parse::<PeerId>() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Some((state, start_block, end_block, public_name, _, version)) =
            decode_server_info_ext(value_bytes)
        {
            if state != 2 {
                continue;
            }
            if !version_meets_minimum(&version) {
                continue;
            }
            let key = peer_id_b58.clone();
            out.entry(key).or_insert(BlockServerEntry {
                peer_id,
                start_block,
                end_block,
                public_name,
                trust_score: None,
            });
        }
    }
}

/// Minimum version required for a node's DHT record to be trusted.
/// Nodes below this version announced stale block data unconditionally.
const MIN_VERSION: (u32, u32, u32) = (0, 3, 15);

/// Parse `"kwaai-X.Y.Z"` → `(major, minor, patch)`. Returns None if unparseable.
fn parse_kwaai_version(s: &str) -> Option<(u32, u32, u32)> {
    let s = s.strip_prefix("kwaai-").unwrap_or(s);
    let mut parts = s.splitn(3, '.');
    let maj = parts.next()?.parse().ok()?;
    let min = parts.next()?.parse().ok()?;
    let pat = parts
        .next()?
        .trim_end_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .ok()?;
    Some((maj, min, pat))
}

fn version_meets_minimum(version_str: &str) -> bool {
    match parse_kwaai_version(version_str) {
        Some(v) => v >= MIN_VERSION,
        None => false, // unparseable / missing version → pre-0.3.15, exclude
    }
}

/// The valid block counts a node may serve.
const VALID_BLOCK_COUNTS: [usize; 4] = [4, 8, 16, 32];

/// Round `n` to the nearest value in `VALID_BLOCK_COUNTS`.
pub fn snap_to_valid_blocks(n: usize) -> usize {
    *VALID_BLOCK_COUNTS
        .iter()
        .min_by_key(|&&v| (v as i64 - n as i64).unsigned_abs())
        .unwrap_or(&4)
}

/// Core decoder: `Ext(64, msgpack([state, throughput, {start_block, end_block, …}]))`
/// Returns `(state, start_block, end_block, public_name, peer_id_b58, version)`.
fn decode_server_info_ext(bytes: &[u8]) -> Option<(i32, usize, usize, String, String, String)> {
    let val = rmpv::decode::read_value(&mut &bytes[..]).ok()?;
    let inner_bytes = match &val {
        rmpv::Value::Ext(64, b) => b.as_slice(),
        _ => return None,
    };
    let inner = rmpv::decode::read_value(&mut &inner_bytes[..]).ok()?;
    let arr = inner.as_array()?;
    if arr.len() < 3 {
        return None;
    }
    let map = arr[2].as_map()?;

    let get_i = |k: &str| -> Option<i64> {
        map.iter()
            .find(|(ky, _)| ky.as_str() == Some(k))
            .and_then(|(_, v)| v.as_i64())
    };
    let get_s = |k: &str| -> String {
        map.iter()
            .find(|(ky, _)| ky.as_str() == Some(k))
            .and_then(|(_, v)| v.as_str())
            .unwrap_or("")
            .to_string()
    };

    let state = arr[0].as_i64().unwrap_or(0) as i32;
    let start_block = get_i("start_block")? as usize;
    let end_block = get_i("end_block")? as usize;
    let public_name = get_s("public_name");
    let peer_id_b58 = get_s("peer_id");
    let version = get_s("version");

    Some((
        state,
        start_block,
        end_block,
        public_name,
        peer_id_b58,
        version,
    ))
}

// ── Pinned path ──────────────────────────────────────────────────────────────

/// Build a deterministic, non-overlapping peer path for a session.
///
/// Greedy walk from block 0: at each position, pick the widest-coverage
/// candidate (largest `end_block`) not in `failed_peers`, then advance to
/// that candidate's `end_block`.  Returns an ordered list of entries that
/// together cover `[0, total_blocks)`.
pub fn build_pinned_path(
    chain: &[BlockServerEntry],
    total_blocks: usize,
    failed_peers: &std::collections::HashSet<PeerId>,
) -> Result<Vec<BlockServerEntry>> {
    let mut path = Vec::new();
    let mut pos = 0;
    while pos < total_blocks {
        let best = chain
            .iter()
            .filter(|e| e.start_block <= pos && e.end_block > pos)
            .filter(|e| !failed_peers.contains(&e.peer_id))
            .max_by_key(|e| e.end_block);
        match best {
            Some(entry) => {
                pos = entry.end_block;
                path.push(entry.clone());
            }
            None => {
                anyhow::bail!(
                    "No server covers block {} — chain has a gap \
                     (or all candidates blacklisted)",
                    pos
                );
            }
        }
    }
    Ok(path)
}

// ── Circuits ─────────────────────────────────────────────────────────────────

/// A long-lived peer path that can serve multiple chat completions.
/// Created once (chain discovery + path pinning), reused across invocations.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Circuit {
    pub id: String,
    pub pinned_path: Vec<SerializableEntry>,
    pub chain: Vec<SerializableEntry>,
    pub total_blocks: usize,
    pub ttl_secs: u64,
    pub created_epoch: u64,
    pub last_used_epoch: u64,
}

/// Serializable version of BlockServerEntry (PeerId is not Serialize).
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializableEntry {
    pub peer_id_b58: String,
    pub start_block: usize,
    pub end_block: usize,
    pub public_name: String,
}

impl SerializableEntry {
    fn from_entry(e: &BlockServerEntry) -> Self {
        Self {
            peer_id_b58: e.peer_id.to_base58(),
            start_block: e.start_block,
            end_block: e.end_block,
            public_name: e.public_name.clone(),
        }
    }

    fn to_entry(&self) -> Option<BlockServerEntry> {
        let peer_id = self.peer_id_b58.parse::<PeerId>().ok()?;
        Some(BlockServerEntry {
            peer_id,
            start_block: self.start_block,
            end_block: self.end_block,
            public_name: self.public_name.clone(),
            trust_score: None,
        })
    }
}

fn circuits_file() -> PathBuf {
    crate::config::run_dir().join("circuits.json")
}

fn load_circuits() -> Vec<Circuit> {
    let path = circuits_file();
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&text).unwrap_or_default()
}

fn save_circuits(circuits: &[Circuit]) -> Result<()> {
    let path = circuits_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(circuits)?;
    std::fs::write(&path, json)?;
    Ok(())
}

fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Truncate to `max` Unicode scalar values, appending "..." if cut.
/// Uses char boundaries — safe for any UTF-8 input.
fn truncate_str(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

fn circuit_id_from(seed: &str) -> String {
    use sha1::Digest;
    let hash = sha1::Sha1::digest(seed.as_bytes());
    hex::encode(&hash[..4])
}

/// Prune expired circuits and return the live ones.
fn prune_circuits(mut circuits: Vec<Circuit>) -> Vec<Circuit> {
    let now = now_epoch();
    circuits.retain(|c| now.saturating_sub(c.last_used_epoch) < c.ttl_secs);
    circuits
}

pub fn load_circuit_by_id(id: &str) -> Result<Circuit> {
    let circuits = prune_circuits(load_circuits());
    circuits
        .into_iter()
        .find(|c| c.id == id || c.id.starts_with(id))
        .ok_or_else(|| anyhow::anyhow!("Circuit '{}' not found (expired or never created)", id))
}

// ── Circuit commands ─────────────────────────────────────────────────────────

async fn cmd_circuit_create(args: CircuitCreateArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let dht_prefix = cfg.effective_dht_prefix();
    let total_blocks = cfg.model_total_blocks() as usize;

    print_box_header("🔗 Create Inference Circuit");
    println!("  Model prefix: {}", dht_prefix);
    println!("  Total blocks: {}", total_blocks);
    println!();

    // Connect to p2pd
    let daemon_addr = daemon_socket();
    let mut client = P2PClient::connect(&daemon_addr)
        .await
        .context("Cannot connect to node — is it running? (`kwaainet start --daemon`)")?;
    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id =
        PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse our peer ID")?;

    // Discover chain
    let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg.initial_peers.clone()
    };

    print_info("Discovering block circuit from DHT…");
    let chain = discover_chain(
        &mut client,
        &our_peer_id,
        &dht_prefix,
        total_blocks,
        &bootstrap_peers,
    )
    .await;

    // Apply optional name filter
    let chain = if let Some(ref f) = args.name_filter {
        let filtered: Vec<_> = chain
            .into_iter()
            .filter(|e| e.public_name.contains(f.as_str()))
            .collect();
        if filtered.is_empty() {
            bail!("No block servers matched name filter {:?}", f);
        }
        filtered
    } else {
        chain
    };

    println!("  Found {} node(s)", chain.len());

    // Build pinned path
    let failed_peers = std::collections::HashSet::new();
    let pinned_path = build_pinned_path(&chain, total_blocks, &failed_peers)?;

    // Generate circuit ID
    let seed = format!("{}{}", now_epoch(), our_peer_id.to_base58());
    let id = circuit_id_from(&seed);

    let circuit = Circuit {
        id: id.clone(),
        pinned_path: pinned_path
            .iter()
            .map(SerializableEntry::from_entry)
            .collect(),
        chain: chain.iter().map(SerializableEntry::from_entry).collect(),
        total_blocks,
        ttl_secs: args.ttl_minutes * 60,
        created_epoch: now_epoch(),
        last_used_epoch: now_epoch(),
    };

    // Persist
    let mut circuits = prune_circuits(load_circuits());
    circuits.push(circuit);
    save_circuits(&circuits)?;

    println!();
    print_success(&format!("Circuit {} established", id));
    println!();
    println!("  Pinned path:");
    for (i, entry) in pinned_path.iter().enumerate() {
        println!(
            "    [{:>2}] blocks {:>3}–{:>3}  {}",
            i + 1,
            entry.start_block,
            entry.end_block - 1,
            entry.public_name,
        );
    }
    println!();
    println!("  TTL: {} minutes", args.ttl_minutes);
    print_info(&format!(
        "Use: kwaainet shard run \"prompt\" --circuit {}",
        id
    ));
    print_separator();
    Ok(())
}

async fn cmd_circuit_list() -> Result<()> {
    let circuits = prune_circuits(load_circuits());
    save_circuits(&circuits)?; // write back pruned list

    print_box_header("🔗 Active Circuits");

    if circuits.is_empty() {
        println!("  No active circuits.");
        println!();
        print_info("Create one: kwaainet shard circuit create");
        print_separator();
        return Ok(());
    }

    let now = now_epoch();
    for c in &circuits {
        let age_secs = now.saturating_sub(c.created_epoch);
        let age = if age_secs >= 3600 {
            format!("{}h {}m", age_secs / 3600, (age_secs % 3600) / 60)
        } else {
            format!("{}m", age_secs / 60)
        };
        let hops = c.pinned_path.len();
        let path_str: Vec<String> = c
            .pinned_path
            .iter()
            .map(|e| {
                e.public_name
                    .split('/')
                    .next()
                    .unwrap_or(&e.public_name)
                    .to_string()
            })
            .collect();
        println!(
            "  {}  {} hop(s)  {}  created {} ago",
            c.id,
            hops,
            path_str.join(" → "),
            age
        );
    }

    println!();
    print_separator();
    Ok(())
}

async fn cmd_circuit_close(args: CircuitCloseArgs) -> Result<()> {
    let mut circuits = load_circuits();
    let before = circuits.len();
    circuits.retain(|c| c.id != args.id && !c.id.starts_with(&args.id));
    let removed = before - circuits.len();
    save_circuits(&circuits)?;

    if removed > 0 {
        print_success(&format!("Circuit {} closed", args.id));
    } else {
        print_warning(&format!("Circuit '{}' not found", args.id));
    }
    Ok(())
}

// ── Forward through chain ─────────────────────────────────────────────────────

/// Send an `InferenceRequest` to the first peer in the chain, routing the
/// activation tensor through each subsequent peer until the last returns logits.
/// Forward a request through the block circuit, advancing greedily by block position.
///
/// At each position, all candidates covering that position are tried in order of
/// widest coverage (largest end_block first). This allows nodes running older code
/// without an inference handler to be transparently skipped in favour of the next
/// available peer that covers the same range.
/// Per-hop timing record: (peer display name, start_block, end_block, elapsed_ms).
pub type HopTiming = (String, usize, usize, f64);

#[allow(clippy::too_many_arguments)]
pub async fn forward_through_chain(
    client: &mut P2PClient,
    chain: &[BlockServerEntry],
    total_blocks: usize,
    session_id: u64,
    seq_pos: u32,
    first_request: InferenceRequest,
    our_peer_id: Option<&PeerId>,
    failed_peers: &mut std::collections::HashSet<PeerId>,
    mut hop_timings: Option<&mut Vec<HopTiming>>,
    reputation: Option<Arc<std::sync::Mutex<ReputationStore>>>,
) -> Result<crate::block_rpc::InferenceResponse> {
    use crate::block_rpc::InferenceResponse;

    // Read local bypass port once (written by `shard serve` on this machine).
    let local_port: Option<u16> = std::fs::read_to_string(local_server_port_file())
        .ok()
        .and_then(|s| s.trim().parse().ok());

    let mut request = first_request;
    let mut response: Option<InferenceResponse> = None;
    let mut pos = 0;

    while pos < total_blocks {
        // All nodes whose range covers `pos`.
        // Primary sort: widest coverage first (largest end_block).
        // Secondary sort: highest local trust score first.
        // Tertiary sort: prefer self when we can dispatch locally — avoids
        // routing through libp2p when our own shard serve already has the
        // block range loaded, which is the cheapest possible hop. Gated on
        // `local_port.is_some()` so we don't pick self and then fail the
        // dispatch with "shard serve is not running on this machine".
        // Skip peers that already failed with protocol errors in this session.
        let mut candidates: Vec<&BlockServerEntry> = chain
            .iter()
            .filter(|e| e.start_block <= pos && e.end_block > pos)
            .filter(|e| !failed_peers.contains(&e.peer_id))
            .collect();
        let self_dispatchable = local_port.is_some();
        candidates.sort_by(|a, b| {
            b.end_block
                .cmp(&a.end_block)
                .then_with(|| {
                    b.trust_score
                        .partial_cmp(&a.trust_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .then_with(|| {
                    if !self_dispatchable {
                        return std::cmp::Ordering::Equal;
                    }
                    let a_self = our_peer_id == Some(&a.peer_id);
                    let b_self = our_peer_id == Some(&b.peer_id);
                    b_self.cmp(&a_self)
                })
        });

        if candidates.is_empty() {
            anyhow::bail!("No server covers block {} — chain has a gap (all candidates failed or blacklisted)", pos);
        }

        // Per-hop deadline: 5 minutes is generous enough for slow CPU prefill of
        // large models while still eventually unblocking a hung/crashed peer.
        const HOP_TIMEOUT: Duration = Duration::from_secs(300);

        let mut succeeded = false;
        for candidate in &candidates {
            // Self-bypass: avoid libp2p "dial to self" by using the local TCP server.
            let is_self = our_peer_id == Some(&candidate.peer_id);
            let hop_start = std::time::Instant::now();
            let result = tokio::time::timeout(HOP_TIMEOUT, async {
                if is_self {
                    match local_port {
                        Some(port) => local_inference_call(port, &request).await,
                        None => Err(anyhow::anyhow!(
                            "shard serve is not running on this machine (no local port file)"
                        )),
                    }
                } else {
                    call_block_forward(client, &candidate.peer_id, &request).await
                }
            })
            .await
            .unwrap_or_else(|_| {
                Err(anyhow::anyhow!(
                    "hop timeout after {}s — peer unresponsive or compute too slow",
                    HOP_TIMEOUT.as_secs()
                ))
            });
            let hop_ms = hop_start.elapsed().as_secs_f64() * 1000.0;

            match result {
                Ok(resp) => {
                    // Record successful hop in local reputation store.
                    if let Some(ref rep) = reputation {
                        if let Ok(mut store) = rep.lock() {
                            store.record(
                                &candidate.peer_id.to_base58(),
                                &candidate.public_name,
                                PeerObservation {
                                    timestamp_secs: now_secs(),
                                    latency_ms: hop_ms,
                                    success: true,
                                    observed_tps: None,
                                    claimed_tps: None,
                                },
                            );
                        }
                    }
                    if let Some(ref mut timings) = hop_timings {
                        timings.push((
                            candidate.public_name.clone(),
                            candidate.start_block,
                            candidate.end_block,
                            hop_ms,
                        ));
                    }
                    pos = candidate.end_block;
                    if pos < total_blocks {
                        request = InferenceRequest {
                            session_id,
                            seq_pos,
                            payload_type: PayloadType::HiddenStates,
                            shape: resp.shape.clone(),
                            data: resp.data.clone(),
                        };
                    }
                    response = Some(resp);
                    succeeded = true;
                    break;
                }
                Err(e) => {
                    // Record failed hop in local reputation store.
                    if let Some(ref rep) = reputation {
                        if let Ok(mut store) = rep.lock() {
                            store.record(
                                &candidate.peer_id.to_base58(),
                                &candidate.public_name,
                                PeerObservation {
                                    timestamp_secs: now_secs(),
                                    latency_ms: hop_ms,
                                    success: false,
                                    observed_tps: None,
                                    claimed_tps: None,
                                },
                            );
                        }
                    }
                    let err_str = format!("{e:#}");
                    let is_transient = err_str.contains("stream reset")
                        || err_str.contains("early eof")
                        || err_str.contains("connection closed");
                    // Protocol negotiation failures mean the peer has no inference
                    // handler registered — blacklist for the session.
                    // Transient stream errors are NOT blacklisted: the peer may recover.
                    if err_str.contains("protocols not supported") {
                        failed_peers.insert(candidate.peer_id);
                        print_warning(&format!(
                            "Peer {} ({}) has no inference handler — skipping for this session",
                            candidate
                                .peer_id
                                .to_base58()
                                .chars()
                                .take(12)
                                .collect::<String>(),
                            candidate.public_name,
                        ));
                    } else if is_transient {
                        // Transient: warn but keep peer eligible for future tokens.
                        print_warning(&format!(
                            "Peer {} ({}) transient error (not blacklisted): {e:#}",
                            candidate
                                .peer_id
                                .to_base58()
                                .chars()
                                .take(12)
                                .collect::<String>(),
                            candidate.public_name,
                        ));
                    } else {
                        failed_peers.insert(candidate.peer_id);
                        print_warning(&format!(
                            "Peer {} ({}) failed: {e:#}",
                            candidate
                                .peer_id
                                .to_base58()
                                .chars()
                                .take(12)
                                .collect::<String>(),
                            candidate.public_name,
                        ));
                    }
                }
            }
        }

        if !succeeded {
            anyhow::bail!(
                "All {} candidate(s) for block {} failed",
                candidates.len(),
                pos
            );
        }
    }

    response.ok_or_else(|| anyhow::anyhow!("Empty chain — no peers to forward through"))
}

// ── Bypass generation loop (reuse shard serve's loaded model) ────────────────

/// Run the `--local` generation loop through the running `shard serve` TCP bypass
/// server instead of loading the model a second time.
#[allow(clippy::too_many_arguments)]
async fn run_via_local_bypass(
    port: u16,
    tokenizer: &kwaai_inference::tokenizer::BpeTokenizer,
    token_ids: Vec<u32>,
    eos_id: u32,
    session_id: u64,
    max_tokens: usize,
    temperature: f32,
    top_k: usize,
    top_p: f32,
    n_input: usize,
    show_stats: bool,
) -> Result<()> {
    use kwaai_inference::tokenizer::Tokenizer as _;
    use std::io::Write as _;

    let mut current_ids = token_ids;
    let mut generated_ids: Vec<u32> = Vec::new();
    let mut seq_pos: usize = 0;

    let mut prefill_spinner: Option<crate::progress::Spinner> = Some(
        crate::progress::Spinner::start(format!("Prefilling {} input tokens", n_input)),
    );
    let mut gen_bar = crate::progress::GenBar::new(max_tokens);
    let mut token_times_ms: Vec<f64> = Vec::new();

    let cpu = candle_core::Device::Cpu;

    loop {
        let token_start = std::time::Instant::now();

        let (shape, data) = token_ids_to_bytes(&current_ids);
        let request = InferenceRequest {
            session_id,
            seq_pos: seq_pos as u32,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };

        let response = local_inference_call(port, &request).await?;

        if response.response_type != crate::block_rpc::ResponseType::Logits {
            bail!(
                "Local shard does not cover all blocks — got {:?} response, need Logits. \
                 Ensure shard serve is configured to serve the full model.",
                response.response_type
            );
        }

        let logits = f16_bytes_to_tensor(&response.data, &response.shape, &cpu)
            .context("decode logits from bypass response")?;

        // Stop prefill spinner on first token
        if generated_ids.is_empty() {
            let prefill_ms = token_start.elapsed().as_secs_f64() * 1000.0;
            if let Some(sp) = prefill_spinner.take() {
                sp.finish(format!(
                    "✓ Prefill  {:.0} ms  ({} input tokens)",
                    prefill_ms, n_input
                ))
                .await;
            }
            println!();
            println!("  Assistant:");
            print!("  ");
            std::io::stdout().flush().ok();
        }

        let last_logits = {
            let dims = logits.dims();
            if dims.len() == 3 && dims[1] > 1 {
                use candle_core::IndexOp as _;
                logits.i((0, dims[1] - 1, ..))?
            } else {
                logits.flatten_all()?
            }
        };
        let next_id = sample_token(&last_logits, temperature, top_k, top_p)? as u32;

        if let Ok(piece) = tokenizer.decode(&[next_id]) {
            print!("{}", piece);
            std::io::stdout().flush().ok();
        }

        let token_ms = token_start.elapsed().as_secs_f64() * 1000.0;
        token_times_ms.push(token_ms);
        generated_ids.push(next_id);
        seq_pos += current_ids.len();

        if generated_ids.len() > 1 {
            gen_bar.tick(generated_ids.len(), token_ms);
        }

        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }

        current_ids = vec![next_id];
    }

    let n = generated_ids.len();
    let total_secs = token_times_ms.iter().sum::<f64>() / 1000.0;
    let tps = gen_bar.tps();

    println!();
    println!();
    print_success(&format!(
        "Generated {} tok  •  {:.1} tok/s  •  {:.1}s",
        n, tps, total_secs
    ));

    if show_stats && !token_times_ms.is_empty() {
        println!();
        println!("  Per-token latency (ms):");
        for (i, ms) in token_times_ms.iter().enumerate() {
            println!("    [{:>4}]  {:.1}", i + 1, ms);
        }
    }

    print_separator();
    Ok(())
}

// ── Local inference bypass (avoids libp2p self-dial) ─────────────────────────

/// Path to the file that holds the local TCP bypass port written by `shard serve`.
fn local_server_port_file() -> std::path::PathBuf {
    crate::config::run_dir().join("shard_local.port")
}

/// Path to the file that holds the port written by `kwaainet shard api` at startup.
/// Read by `make_shard_proxy_handler` to forward incoming P2P requests to the local API.
pub fn shard_api_port_file() -> std::path::PathBuf {
    crate::config::run_dir().join("shard_api.port")
}

/// Spawn a local TCP server on `127.0.0.1:0` that serves the same
/// msgpack inference protocol as the p2pd handler, without going through p2pd.
/// Returns the bound port.  Called by `cmd_shard_serve`.
///
/// Accepts a [`ShardCell`] — returns a "warming up" error response when the
/// background load task hasn't written the shard yet.
async fn start_local_inference_server(
    shard: ShardCell,
    device: candle_core::Device,
) -> Result<u16> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("bind local inference server")?;
    let port = listener.local_addr()?.port();

    tokio::spawn(async move {
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let shard = shard.clone();
            let device = device.clone();
            tokio::spawn(async move {
                // Framing: 4-byte LE length prefix + msgpack bytes
                let mut len_buf = [0u8; 4];
                if stream.read_exact(&mut len_buf).await.is_err() {
                    return;
                }
                let len = u32::from_le_bytes(len_buf) as usize;
                let mut buf = vec![0u8; len];
                if stream.read_exact(&mut buf).await.is_err() {
                    return;
                }

                // Grab the shard (if loaded) without holding the lock during inference.
                let shard_arc: Option<Arc<TransformerShard>> = {
                    let guard = shard.read().await;
                    guard.as_ref().cloned()
                };

                let resp_bytes = match shard_arc {
                    None => {
                        let err_resp = crate::block_rpc::InferenceResponse {
                            session_id: 0,
                            response_type: crate::block_rpc::ResponseType::HiddenStates,
                            shape: vec![],
                            data: vec![],
                            error: Some(
                                "node warming up — model loading in background".to_string(),
                            ),
                        };
                        rmp_serde::to_vec_named(&err_resp).unwrap_or_default()
                    }
                    Some(s) => {
                        match crate::block_rpc::handle_inference_request(s, device.clone(), buf)
                            .await
                        {
                            Ok(r) => rmp_serde::to_vec_named(&r).unwrap_or_default(),
                            Err(e) => {
                                let err_resp = crate::block_rpc::InferenceResponse {
                                    session_id: 0,
                                    response_type: crate::block_rpc::ResponseType::HiddenStates,
                                    shape: vec![],
                                    data: vec![],
                                    error: Some(e.to_string()),
                                };
                                rmp_serde::to_vec_named(&err_resp).unwrap_or_default()
                            }
                        }
                    }
                };

                let len_bytes = (resp_bytes.len() as u32).to_le_bytes();
                let _ = stream.write_all(&len_bytes).await;
                let _ = stream.write_all(&resp_bytes).await;
            });
        }
    });

    Ok(port)
}

/// Call the local TCP inference bypass server (used instead of p2pd self-dial).
async fn local_inference_call(
    port: u16,
    request: &InferenceRequest,
) -> Result<crate::block_rpc::InferenceResponse> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let req_bytes = rmp_serde::to_vec_named(request).context("serialise InferenceRequest")?;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .context("connect to local inference server")?;

    let len_bytes = (req_bytes.len() as u32).to_le_bytes();
    stream.write_all(&len_bytes).await.context("write length")?;
    stream
        .write_all(&req_bytes)
        .await
        .context("write request")?;

    let mut len_buf = [0u8; 4];
    stream
        .read_exact(&mut len_buf)
        .await
        .context("read response length")?;
    let len = u32::from_le_bytes(len_buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await.context("read response")?;

    let response: crate::block_rpc::InferenceResponse =
        rmp_serde::from_slice(&buf).context("deserialise InferenceResponse")?;
    if let Some(ref err) = response.error {
        anyhow::bail!("Local inference error: {err}");
    }
    Ok(response)
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// SHA1(msgpack(raw_key)) — Hivemind's DHTID.generate() equivalent.
fn block_dht_id(prefix: &str, block: usize) -> Vec<u8> {
    let raw = format!("{}.{}", prefix, block);
    let packed = rmp_serde::to_vec(&raw).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// UDS socket path for p2pd.
/// Override with `KWAAINET_SOCKET=/tmp/my.sock` to point at a different p2pd instance
/// (e.g. when running multiple nodes on the same machine).
pub fn daemon_socket() -> String {
    #[cfg(unix)]
    let addr = {
        let sock =
            std::env::var("KWAAINET_SOCKET").unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        format!("/unix/{}", sock)
    };
    #[cfg(not(unix))]
    let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
    addr
}

/// Check whether chain entries cover every block in `0..total_blocks`.
fn coverage_check(chain: &[BlockServerEntry], total_blocks: usize) -> bool {
    let mut covered = vec![false; total_blocks];
    for entry in chain {
        covered[entry.start_block..entry.end_block.min(total_blocks)].fill(true);
    }
    covered.iter().all(|&c| c)
}

/// Collect all `*.safetensors` files in a directory (sorted for determinism).
fn collect_safetensors(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("Reading directory {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("safetensors"))
        .collect();
    paths.sort();
    Ok(paths)
}

/// Sample the next token id from logits using temperature + top-k + top-p (nucleus) filtering.
/// Falls back to greedy argmax when temperature == 1.0, top_k == 0, top_p >= 1.0.
pub fn sample_token(
    logits: &candle_core::Tensor,
    temperature: f32,
    top_k: usize,
    top_p: f32,
) -> Result<usize> {
    use candle_core::DType;
    let logits_f32 = logits.to_dtype(DType::F32)?.flatten_all()?;
    let mut vals: Vec<f32> = logits_f32.to_vec1()?;
    let n = vals.len();

    // Temperature scaling
    if temperature != 1.0 && temperature > 0.0 {
        vals.iter_mut().for_each(|v| *v /= temperature);
    }

    // Pure greedy when no sampling is requested
    if (temperature <= 0.0 || temperature == 1.0) && top_k == 0 && top_p >= 1.0 {
        return Ok(vals
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0));
    }

    // Softmax
    let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    vals.iter_mut().for_each(|v| *v = (*v - max).exp());
    let sum: f32 = vals.iter().sum();
    vals.iter_mut().for_each(|v| *v /= sum);

    // Build (prob, index) sorted descending by prob
    let mut indexed: Vec<(f32, usize)> =
        vals.into_iter().enumerate().map(|(i, p)| (p, i)).collect();
    indexed.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    // Top-k filter
    if top_k > 0 && top_k < n {
        indexed.truncate(top_k);
    }

    // Top-p nucleus filter
    if top_p < 1.0 {
        let mut cumsum = 0.0f32;
        let cutoff = indexed
            .iter()
            .position(|(p, _)| {
                cumsum += p;
                cumsum >= top_p
            })
            .map(|i| i + 1)
            .unwrap_or(indexed.len());
        indexed.truncate(cutoff.max(1));
    }

    // Renormalize and sample
    let total: f32 = indexed.iter().map(|(p, _)| p).sum();
    let mut rng = rand_f32() * total;
    for (p, i) in &indexed {
        rng -= p;
        if rng <= 0.0 {
            return Ok(*i);
        }
    }
    Ok(indexed[0].1)
}

/// Simple time-seeded float in [0, 1) — good enough for sampling.
fn rand_f32() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(42) as u64;
    let shuffled = ns
        .wrapping_mul(6_364_136_223_846_793_005_u64)
        .wrapping_add(1_442_695_040_888_963_407_u64);
    ((shuffled >> 33) as u32 as f32) / (u32::MAX as f32)
}

/// Generate a random u64 session ID using splitmix64 over (nanos ⊕ pid).
///
/// Using raw `as_nanos() as u64` truncates a u128 and collides if called
/// twice within the same nanosecond. Mixing in the process ID + splitmix64
/// gives adequate entropy without adding a dependency.
fn rand_session_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64) // low 64 bits of epoch-nanos
        .unwrap_or(42);
    let pid = std::process::id() as u64;
    // splitmix64: thoroughly mixes bits, eliminates collision from same-ns calls
    let mut x = ns ^ pid.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_meets_minimum() {
        assert!(!version_meets_minimum("kwaai-0.3.14"));
        assert!(!version_meets_minimum("kwaai-0.2.99"));
        assert!(!version_meets_minimum("kwaai-0.3.0"));
        assert!(!version_meets_minimum("")); // missing → exclude
        assert!(!version_meets_minimum("unknown"));
        assert!(version_meets_minimum("kwaai-0.3.15"));
        assert!(version_meets_minimum("kwaai-0.3.16"));
        assert!(version_meets_minimum("kwaai-0.4.0"));
        assert!(version_meets_minimum("kwaai-1.0.0"));
    }

    #[test]
    fn test_snap_to_valid_blocks() {
        assert_eq!(snap_to_valid_blocks(1), 4);
        assert_eq!(snap_to_valid_blocks(4), 4);
        assert_eq!(snap_to_valid_blocks(5), 4);
        assert_eq!(snap_to_valid_blocks(6), 4);
        assert_eq!(snap_to_valid_blocks(7), 8);
        assert_eq!(snap_to_valid_blocks(8), 8);
        assert_eq!(snap_to_valid_blocks(12), 8);
        assert_eq!(snap_to_valid_blocks(13), 16);
        assert_eq!(snap_to_valid_blocks(16), 16);
        assert_eq!(snap_to_valid_blocks(24), 16);
        assert_eq!(snap_to_valid_blocks(25), 32);
        assert_eq!(snap_to_valid_blocks(32), 32);
        assert_eq!(snap_to_valid_blocks(64), 32);
    }
}
