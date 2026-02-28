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
use crate::cli::{ShardAction, ShardArgs, ShardChainArgs, ShardRunArgs, ShardServeArgs};
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::hf;

// ── Entrypoint ────────────────────────────────────────────────────────────────

pub async fn run(args: ShardArgs) -> Result<()> {
    match args.action {
        ShardAction::Serve(a) => cmd_shard_serve(a).await,
        ShardAction::Run(a)   => cmd_shard_run(a).await,
        ShardAction::Status   => cmd_shard_status().await,
        ShardAction::Chain(a) => cmd_shard_chain(a).await,
    }
}

// ── serve ─────────────────────────────────────────────────────────────────────

pub async fn cmd_shard_serve(args: ShardServeArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    let start_block = args.start_block.unwrap_or(cfg.start_block) as usize;
    let blocks      = args.blocks.unwrap_or(cfg.blocks) as usize;
    let end_block   = start_block + blocks;

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
    let model_dir = hf::resolve_snapshot(model_ref)?;
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

        // Greedy sampling: argmax
        let next_id = argmax_f16(&last_logits)? as u32;

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
#[derive(Debug)]
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
async fn forward_through_chain(
    client: &mut P2PClient,
    chain: &[BlockServerEntry],
    session_id: u64,
    seq_pos: u32,
    first_request: InferenceRequest,
) -> Result<crate::block_rpc::InferenceResponse> {
    use crate::block_rpc::InferenceResponse;

    let mut request = first_request;
    let mut response: Option<InferenceResponse> = None;

    for (idx, entry) in chain.iter().enumerate() {
        let resp = call_block_forward(client, &entry.peer_id, &request)
            .await
            .with_context(|| {
                format!(
                    "RPC call to peer {} (blocks {}-{}) failed",
                    entry.peer_id.to_base58().chars().take(12).collect::<String>(),
                    entry.start_block,
                    entry.end_block,
                )
            })?;

        // If not the last node, build the next request with hidden states
        if idx + 1 < chain.len() {
            request = InferenceRequest {
                session_id,
                seq_pos,
                payload_type: PayloadType::HiddenStates,
                shape: resp.shape.clone(),
                data: resp.data.clone(),
            };
        }
        response = Some(resp);
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

/// UDS socket path for p2pd, matching the daemon's DEFAULT_SOCKET_NAME.
fn daemon_socket() -> String {
    #[cfg(unix)]
    return format!("/unix/{}", DEFAULT_SOCKET_NAME);
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

/// Greedy sampler: return the argmax over the last (vocab) dimension.
/// Input tensor must be a flat f16 vector (already on CPU).
fn argmax_f16(logits: &candle_core::Tensor) -> Result<usize> {
    use candle_core::DType;
    let logits_f32 = logits
        .to_dtype(DType::F32)
        .context("convert logits to f32")?;
    let flat: Vec<f32> = logits_f32
        .flatten_all()
        .context("flatten logits")?
        .to_vec1()
        .context("logits to_vec1")?;
    let best = flat
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    Ok(best)
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

