//! Distributed block sharding commands.
//!
//! Implements `kwaainet shard <subcommand>`:
//!
//! - **serve**  — Load model shard and register inference RPC handler with p2pd.
//! - **run**    — Discover block chain from DHT and run distributed inference.
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
    ShardAction, ShardArgs, ShardChainArgs, ShardDownloadArgs, ShardRunArgs, ShardServeArgs,
};
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::hf;

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
    }
}

// ── download ──────────────────────────────────────────────────────────────────

pub async fn cmd_shard_download(args: ShardDownloadArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let model_id = args.model.as_deref().unwrap_or(&cfg.model).to_string();

    print_box_header("Downloading HuggingFace Model");
    println!("  Model: {}", model_id);
    println!();

    let snapshot_dir = hf::download(&model_id, args.hf_token.as_deref()).await?;

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

    let target_blocks = args.blocks.unwrap_or(cfg.blocks) as usize;

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

    // Detect device early (before any model I/O)
    let device_type = if cfg.use_gpu && !args.no_gpu {
        DeviceType::detect_best()
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

    print_box_header("🧩 KwaaiNet Shard Server");
    println!("  Blocks:      [{}, {})", start_block, end_block);
    println!("  Device:      {:?}", device_type);
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
    let device_bg = device.clone();
    let total_blocks_bg = cfg.model_total_blocks() as usize;

    tokio::spawn(async move {
        let result: anyhow::Result<()> = async {
            // Resolve model directory: CLI override > cached snapshot > download.
            let model_dir: PathBuf = if let Some(p) = model_path_bg {
                p
            } else {
                match hf::resolve_snapshot(&model_id_bg) {
                    Ok(d) => d,
                    Err(_) => {
                        let is_first = start_block == 0;
                        let is_last = end_block >= total_blocks_bg;
                        print_info(&format!(
                            "Model not cached — downloading files for blocks [{}, {})…",
                            start_block, end_block
                        ));
                        hf::download_for_blocks(
                            &model_id_bg,
                            start_block,
                            end_block,
                            is_first,
                            is_last,
                            None,
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

    // Pick device
    let device_type = if args.no_gpu {
        kwaai_inference::DeviceType::Cpu
    } else {
        kwaai_inference::DeviceType::detect_best()
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

    print_info(&format!(
        "Loading {} shard(s) on {:?}…",
        paths.len(),
        device_type
    ));

    let path_refs: Vec<&std::path::Path> = paths.iter().map(|p| p.as_path()).collect();
    let shard = Arc::new(
        TransformerShard::load(&path_refs, &config_path, &device, 0, total_blocks)
            .context("Failed to load model")?,
    );

    print_success(&format!("Model loaded ({} blocks)", total_blocks));
    println!("  Input tokens: {}", token_ids.len());
    println!();

    let session_id: u64 = args.session_id.unwrap_or_else(rand_session_id);
    let max_tokens = args.max_tokens;
    let temperature = args.temperature;
    let top_k = args.top_k;
    let top_p = args.top_p;

    let mut generated_ids: Vec<u32> = Vec::new();
    let mut seq_pos: usize = 0;
    let mut current_ids = token_ids.clone();

    print!("  Assistant: ");
    use std::io::Write as _;
    std::io::stdout().flush().ok();

    loop {
        // Run full forward pass in-process
        let logits = tokio::task::spawn_blocking({
            let shard = shard.clone();
            let ids = current_ids.clone();
            let sp = seq_pos;
            move || shard.forward_full(session_id, &ids, sp)
        })
        .await
        .context("join forward_full")?
        .context("forward_full")?;

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

        generated_ids.push(next_id);
        seq_pos += current_ids.len();

        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }

        current_ids = vec![next_id];
    }

    println!();
    println!();
    print_success(&format!("Generated {} token(s)", generated_ids.len()));
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

    // Discover the block chain from DHT
    print!("  Discovering block chain from DHT… ");
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
    println!("{} node(s)", chain.len());

    // Validate coverage
    let covered = coverage_check(&chain, total_blocks);
    if !covered {
        print_warning("Block chain has gaps — inference may be incomplete.");
    }

    println!();
    for (i, entry) in chain.iter().enumerate() {
        println!(
            "  [{:>2}] blocks {:>3}–{:>3}  {}  ({})",
            i + 1,
            entry.start_block,
            entry.end_block - 1,
            entry
                .peer_id
                .to_base58()
                .chars()
                .take(16)
                .collect::<String>()
                + "…",
            entry.public_name,
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
    let is_prefill_first = true;
    let _ = is_prefill_first; // used below

    print!("  Assistant: ");
    use std::io::Write as _;
    std::io::stdout().flush().ok();

    loop {
        // Build first request
        let (shape, data) = token_ids_to_bytes(&current_ids);
        let request = InferenceRequest {
            session_id,
            seq_pos: seq_pos as u32,
            payload_type: PayloadType::TokenIds,
            shape,
            data,
        };

        // Forward through chain
        let logits_bytes = forward_through_chain(
            &mut client,
            &chain,
            total_blocks,
            session_id,
            seq_pos as u32,
            request,
            Some(&our_peer_id),
        )
        .await?;

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
            std::io::stdout().flush().ok();
        }

        generated_ids.push(next_id);
        seq_pos += current_ids.len(); // advance by tokens sent this step

        // Stopping conditions
        if next_id == eos_id || generated_ids.len() >= max_tokens {
            break;
        }

        // Next decode step: send just the new token
        current_ids = vec![next_id];
    }

    println!();
    println!();
    print_success(&format!("Generated {} token(s)", generated_ids.len()));
    print_separator();

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

    print_box_header("🗺  KwaaiNet Block Chain");
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
        "  {:<6} {:<6} {:<18} NAME",
        "START", "END", "PEER ID (prefix)"
    );
    println!("  {}", "─".repeat(60));
    for entry in &chain {
        let peer_short = {
            let b58 = entry.peer_id.to_base58();
            if b58.len() > 16 {
                format!("{}…", &b58[..16])
            } else {
                b58
            }
        };
        println!(
            "  {:>5}  {:>5}  {:<18} {}",
            entry.start_block, entry.end_block, peer_short, entry.public_name,
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
    let (start_block, end_block, public_name, peer_id_b58) = decode_server_info_ext(bytes)?;
    let (dedup_key, peer_id) = match peer_id_b58.parse::<PeerId>() {
        Ok(pid) => (pid.to_base58(), pid),
        Err(_) => {
            // Legacy node: no peer_id field.  Use a synthetic stable key so the
            // coverage slot is filled.  PeerId::random() is unroutable but harmless.
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

        if let Some((start_block, end_block, public_name, _)) = decode_server_info_ext(value_bytes)
        {
            let key = peer_id_b58.clone();
            out.entry(key).or_insert(BlockServerEntry {
                peer_id,
                start_block,
                end_block,
                public_name,
            });
        }
    }
}

/// Core decoder: `Ext(64, msgpack([state, throughput, {start_block, end_block, …}]))`
/// Returns `(start_block, end_block, public_name, peer_id_b58)`.
fn decode_server_info_ext(bytes: &[u8]) -> Option<(usize, usize, String, String)> {
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

    let start_block = get_i("start_block")? as usize;
    let end_block = get_i("end_block")? as usize;
    let public_name = get_s("public_name");
    let peer_id_b58 = get_s("peer_id");

    Some((start_block, end_block, public_name, peer_id_b58))
}

// ── Forward through chain ─────────────────────────────────────────────────────

/// Send an `InferenceRequest` to the first peer in the chain, routing the
/// activation tensor through each subsequent peer until the last returns logits.
/// Forward a request through the block chain, advancing greedily by block position.
///
/// At each position, all candidates covering that position are tried in order of
/// widest coverage (largest end_block first). This allows nodes running older code
/// without an inference handler to be transparently skipped in favour of the next
/// available peer that covers the same range.
pub async fn forward_through_chain(
    client: &mut P2PClient,
    chain: &[BlockServerEntry],
    total_blocks: usize,
    session_id: u64,
    seq_pos: u32,
    first_request: InferenceRequest,
    our_peer_id: Option<&PeerId>,
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
        // All nodes whose range covers `pos`, widest first
        let mut candidates: Vec<&BlockServerEntry> = chain
            .iter()
            .filter(|e| e.start_block <= pos && e.end_block > pos)
            .collect();
        candidates.sort_by(|a, b| b.end_block.cmp(&a.end_block));

        if candidates.is_empty() {
            anyhow::bail!("No server covers block {} — chain has a gap", pos);
        }

        let mut succeeded = false;
        for candidate in &candidates {
            // Self-bypass: avoid libp2p "dial to self" by using the local TCP server.
            let is_self = our_peer_id == Some(&candidate.peer_id);
            let result = if is_self {
                match local_port {
                    Some(port) => local_inference_call(port, &request).await,
                    None => Err(anyhow::anyhow!(
                        "shard serve is not running on this machine (no local port file)"
                    )),
                }
            } else {
                call_block_forward(client, &candidate.peer_id, &request).await
            };

            match result {
                Ok(resp) => {
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

// ── Local inference bypass (avoids libp2p self-dial) ─────────────────────────

/// Path to the file that holds the local TCP bypass port written by `shard serve`.
fn local_server_port_file() -> std::path::PathBuf {
    crate::config::run_dir().join("shard_local.port")
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
                        match crate::block_rpc::handle_inference_request(&s, &device, &buf).await {
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
