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
use kwaai_inference::{DeviceType, TransformerShard};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
use kwaai_hivemind_dht::protocol::{FindRequest, FindResponse, NodeInfo, RequestAuthInfo};
use libp2p::PeerId;
use prost::Message as _;
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use crate::block_rpc::{
    call_block_forward, f16_bytes_to_tensor, make_block_rpc_handler, token_ids_to_bytes,
    InferenceRequest, PayloadType,
};
use crate::cli::{ShardAction, ShardArgs, ShardChainArgs, ShardDownloadArgs, ShardRunArgs, ShardServeArgs};
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::hf;

// ── Entrypoint ────────────────────────────────────────────────────────────────

pub async fn run(args: ShardArgs) -> Result<()> {
    match args.action {
        ShardAction::Serve(a)    => cmd_shard_serve(a).await,
        ShardAction::Run(a)      => cmd_shard_run(a).await,
        ShardAction::Status      => cmd_shard_status().await,
        ShardAction::Chain(a)    => cmd_shard_chain(a).await,
        ShardAction::Api(a)      => crate::shard_api::run(a).await,
        ShardAction::Download(a) => cmd_shard_download(a).await,
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

// ── serve ─────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_serve(args: ShardServeArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    let target_blocks = args.blocks.unwrap_or(cfg.blocks) as usize;

    let (start_block, end_block) = if args.auto || args.start_block.is_none() {
        let daemon_addr = daemon_socket();
        let mut qc = P2PClient::connect(&daemon_addr).await
            .context("Cannot connect to node — start it first with `kwaainet start --daemon`")?;
        let peer_id_hex = qc.identify().await.context("Failed to get local peer ID")?;
        let our_peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)
            .context("parse our peer ID")?;
        let total = cfg.model_total_blocks() as usize;
        let prefix = cfg.model_dht_prefix.as_deref().unwrap_or("unknown");
        let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
            NetworkConfig::with_petals_bootstrap().bootstrap_peers
        } else {
            cfg.initial_peers.clone()
        };

        print_info(&format!("Querying DHT for gap in {} ({} blocks)…", prefix, total));
        let (s, e) = pick_gap_blocks(&mut qc, &our_peer_id, prefix, total, target_blocks, &bootstrap_peers).await?;
        print_success(&format!("Auto-assigned blocks [{}, {})", s, e));

        // Persist so the DHT announcer picks up the new range on restart
        let mut updated = cfg.clone();
        updated.start_block = s as u32;
        updated.save().context("Failed to save config.yaml")?;
        print_info("Updated config.yaml — restarting node daemon to re-announce…");

        // Restart daemon so it announces the new start_block (same pattern as Command::Restart)
        let mgr = crate::daemon::DaemonManager::new();
        if mgr.is_running() { mgr.stop_process()?; }
        crate::daemon::DaemonManager::spawn_daemon_child(&[])?;

        (s, e)
    } else {
        // --start-block was explicitly provided; use it as-is
        let s = args.start_block.unwrap() as usize;
        (s, s + target_blocks)
    };

    // Resolve model directory (CLI override > HF snapshot for config.model)
    let model_dir: PathBuf = if let Some(p) = args.model_path {
        p
    } else {
        hf::resolve_snapshot(&cfg.model)?
    };

    let config_path = model_dir.join("config.json");

    // Collect all *.safetensors files in the directory
    let safetensors: Vec<PathBuf> = collect_safetensors(&model_dir)?;
    if safetensors.is_empty() {
        bail!(
            "No .safetensors files found in {}. \
             Pass --model-path to a HuggingFace snapshot directory.",
            model_dir.display()
        );
    }
    let paths: Vec<&Path> = safetensors.iter().map(|p| p.as_path()).collect();

    // Detect device
    let device_type = if cfg.use_gpu && !args.no_gpu {
        DeviceType::detect_best()
    } else {
        DeviceType::Cpu
    };
    let device = device_type.to_candle_device()
        .context("Failed to create compute device")?;

    print_box_header("🧩 KwaaiNet Shard Server");
    println!("  Model:       {}", model_dir.display());
    println!("  Blocks:      [{}, {})", start_block, end_block);
    println!("  Device:      {:?}", device_type);
    println!("  Shards:      {} file(s)", safetensors.len());
    println!();
    println!("  Loading model shard…");

    let shard = Arc::new(
        TransformerShard::load(&paths, &config_path, &device, start_block, end_block)
            .context("Failed to load transformer shard")?,
    );

    print_success(&format!(
        "Shard loaded  ({} blocks)",
        end_block - start_block
    ));
    println!(
        "  is_first={} is_last={}",
        shard.is_first(),
        shard.is_last()
    );
    println!();

    // Connect to the running p2pd
    let daemon_addr = daemon_socket();
    let client = match P2PClient::connect(&daemon_addr).await {
        Ok(c) => c,
        Err(_) => {
            print_error("Cannot connect to the running KwaaiNet node.");
            print_info("Start the node first: kwaainet start --daemon");
            print_separator();
            return Ok(());
        }
    };

    // Register the inference RPC handler
    let handler = make_block_rpc_handler(shard.clone(), device.clone());
    client
        .add_unary_handler(crate::block_rpc::INFERENCE_PROTO, handler, false)
        .await
        .context("Failed to register inference handler with p2pd")?;

    print_success(&format!(
        "Inference handler registered on protocol {}",
        crate::block_rpc::INFERENCE_PROTO
    ));
    print_info("Serving inference requests — press Ctrl-C to stop");
    print_separator();

    // Background GC task: evict idle sessions every 30 s
    let shard_gc = shard.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            shard_gc.gc_sessions();
        }
    });

    // Wait for Ctrl-C
    tokio::signal::ctrl_c().await.context("ctrl-c handler")?;
    println!();
    print_info("Shard server stopped.");
    Ok(())
}

// ── run ───────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_run(args: ShardRunArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    let model_ref = args.model.as_deref().unwrap_or(&cfg.model);
    let dht_prefix = match &cfg.model_dht_prefix {
        Some(p) => p.clone(),
        None    => derive_dht_prefix(model_ref),
    };
    let total_blocks = args.total_blocks.unwrap_or_else(|| cfg.model_total_blocks() as usize);

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
            print_error("Cannot connect to the running KwaaiNet node.");
            print_info("Start the node first: kwaainet start --daemon");
            print_separator();
            return Ok(());
        }
    };

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)
        .context("parse our peer ID")?;

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
        let filtered: Vec<_> = chain.into_iter().filter(|e| e.public_name.contains(f.as_str())).collect();
        if filtered.is_empty() {
            println!("no nodes matched filter {:?}", f);
            print_warning(&format!("No block servers with name containing {:?} found.", f));
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
        print_info(&format!(
            "Start serving with: kwaainet shard serve --model <path>"
        ));
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
            entry.peer_id.to_base58().chars().take(16).collect::<String>() + "…",
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

    // Tokenize prompt
    let mut token_ids: Vec<u32> = tokenizer
        .encode(&args.prompt)
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
            logits_tensor
                .i((0, seq_len - 1, ..))?
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
        cfg.start_block + cfg.blocks
    );
    println!("  GPU:          {}", cfg.use_gpu);
    if let Some(ref prefix) = cfg.model_dht_prefix {
        println!("  DHT prefix:   {}", prefix);
    }
    println!();
    print_info("To serve this shard: kwaainet shard serve");
    print_info("To change range:     kwaainet config --set start_block 4");
    print_separator();

    Ok(())
}

// ── chain ─────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_chain(args: ShardChainArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    let dht_prefix = args.dht_prefix.as_deref()
        .or(cfg.model_dht_prefix.as_deref())
        .map(str::to_string)
        .unwrap_or_else(|| derive_dht_prefix(&cfg.model));

    let total_blocks = args.total_blocks;

    print_box_header("🗺  KwaaiNet Block Chain");
    println!("  Model prefix: {}", dht_prefix);
    println!("  Querying {} blocks from DHT…", total_blocks);
    println!();

    let daemon_addr = daemon_socket();
    let mut client = match P2PClient::connect(&daemon_addr).await {
        Ok(c) => c,
        Err(_) => {
            print_error("Cannot connect to the running KwaaiNet node.");
            print_info("Start the node first: kwaainet start --daemon");
            print_separator();
            return Ok(());
        }
    };

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let our_peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)
        .context("parse our peer ID")?;

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
        for b in entry.start_block..entry.end_block.min(total_blocks) {
            covered[b] = true;
        }
    }
    let n_covered = covered.iter().filter(|&&c| c).count();

    println!("  {:>3} server(s) — {}/{} blocks covered\n", chain.len(), n_covered, total_blocks);
    println!("  {:<6} {:<6} {:<18} {}", "START", "END", "PEER ID (prefix)", "NAME");
    println!("  {}", "─".repeat(60));
    for entry in &chain {
        let peer_short = {
            let b58 = entry.peer_id.to_base58();
            if b58.len() > 16 { format!("{}…", &b58[..16]) } else { b58 }
        };
        println!(
            "  {:>5}  {:>5}  {:<18} {}",
            entry.start_block,
            entry.end_block,
            peer_short,
            entry.public_name,
        );
    }
    println!();

    // Coverage bar
    print!("  Blocks: [");
    for b in 0..total_blocks {
        print!("{}", if covered[b] { "█" } else { "░" });
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
        let Some(peer_str) = addr.split("/p2p/").nth(1) else { continue };
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

        let Ok(resp) = FindResponse::decode(&resp_bytes[..]) else { continue };

        for result in resp.results {
            if result.value.is_empty() {
                continue;
            }
            let rt = result.result_type;
            if rt == 1 {
                // FoundRegular — single value, peer_id embedded in map
                if let Some(entry) = decode_server_info_regular(&result.value) {
                    servers.entry(entry.peer_id.to_base58()).or_insert(entry);
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

/// Query the DHT, find the first uncovered block range, and return (start, end).
///
/// This is the core of `kwaainet shard serve --auto`: each new node fills the
/// next gap so the network grows toward full coverage organically.
async fn pick_gap_blocks(
    client: &mut P2PClient,
    our_peer_id: &PeerId,
    dht_prefix: &str,
    total_blocks: usize,
    target_blocks: usize,
    bootstrap_peers: &[String],
) -> Result<(usize, usize)> {
    let chain = discover_chain(client, our_peer_id, dht_prefix, total_blocks, bootstrap_peers).await;

    let mut covered = vec![false; total_blocks];
    for e in &chain {
        for b in e.start_block..e.end_block.min(total_blocks) {
            covered[b] = true;
        }
    }

    let start = covered
        .iter()
        .position(|&c| !c)
        .ok_or_else(|| anyhow::anyhow!(
            "All {} blocks already served by {} node(s). \
             Remove --auto or add more nodes.",
            total_blocks,
            chain.len()
        ))?;

    let end = (start + target_blocks).min(total_blocks);
    Ok((start, end))
}

// ── Server info decoding ──────────────────────────────────────────────────────

/// Parse `Ext(64, [state, throughput, {start_block, end_block, peer_id, …}])`
/// from a FoundRegular value.
fn decode_server_info_regular(bytes: &[u8]) -> Option<BlockServerEntry> {
    let (start_block, end_block, public_name, peer_id_b58) =
        decode_server_info_ext(bytes)?;
    let peer_id = peer_id_b58.parse::<PeerId>().ok()?;
    Some(BlockServerEntry { peer_id, start_block, end_block, public_name })
}

/// Parse `Ext(80, [expiry, created, [[subkey_bytes, value_bytes, expiry], …]])`
/// from a FoundDictionary value. Appends into `out` (deduplicates by peer_id).
fn decode_server_info_dictionary(
    bytes: &[u8],
    out: &mut HashMap<String, BlockServerEntry>,
) {
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

        if let Some((start_block, end_block, public_name, _)) = decode_server_info_ext(value_bytes) {
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
    let end_block   = get_i("end_block")? as usize;
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
) -> Result<crate::block_rpc::InferenceResponse> {
    use crate::block_rpc::InferenceResponse;

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
            match call_block_forward(client, &candidate.peer_id, &request).await {
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
                        candidate.peer_id.to_base58().chars().take(12).collect::<String>(),
                        candidate.public_name,
                    ));
                }
            }
        }

        if !succeeded {
            anyhow::bail!("All {} candidate(s) for block {} failed", candidates.len(), pos);
        }
    }

    response.ok_or_else(|| anyhow::anyhow!("Empty chain — no peers to forward through"))
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
    {
        let sock = std::env::var("KWAAINET_SOCKET")
            .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        return format!("/unix/{}", sock);
    }
    #[cfg(not(unix))]
    return "/ip4/127.0.0.1/tcp/5005".to_string();
}

/// Check whether chain entries cover every block in `0..total_blocks`.
fn coverage_check(chain: &[BlockServerEntry], total_blocks: usize) -> bool {
    let mut covered = vec![false; total_blocks];
    for entry in chain {
        for b in entry.start_block..entry.end_block.min(total_blocks) {
            covered[b] = true;
        }
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
    let mut indexed: Vec<(f32, usize)> = vals
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
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


/// Generate a random u64 session ID.
fn rand_session_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42)
}

/// Derive a DHT prefix from a model name/path using Petals conventions.
/// e.g. `"meta-llama/Llama-3.1-8B-Instruct"` → `"Llama-3-1-8B-Instruct"`.
fn derive_dht_prefix(model: &str) -> String {
    let base = model.split('/').last().unwrap_or(model);
    base.replace('.', "-")
}

