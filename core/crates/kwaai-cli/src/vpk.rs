//! VPK (Virtual Private Knowledge) integration management
//!
//! Handles `kwaainet vpk <subcommand>` — enabling/disabling the local VPK
//! service integration and querying its DHT advertisement status.
//!
//! All inter-node storage access goes through the `/kwaai/storage/1.0.0`
//! libp2p relay protocol. Nodes are identified by PeerId only — no IP
//! addresses or HTTP endpoints are advertised in DHT records.

use anyhow::{Context, Result};
use std::time::Duration;

use kwaai_hivemind_dht::protocol::{FindRequest, FindResponse, NodeInfo, RequestAuthInfo};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use prost::Message as _;
use sha1::{Digest, Sha1};

use crate::cli::{TenantAction, VpkAction, VpkArgs};
use crate::config::KwaaiNetConfig;
use crate::display::*;

pub async fn run(args: VpkArgs) -> Result<()> {
    match args.action {
        VpkAction::Enable { mode, port } => enable(mode, port),
        VpkAction::Disable => disable(),
        VpkAction::Status => status().await,
        VpkAction::Discover { json } => discover(json).await,
        VpkAction::Shard { kb_id, eve_count } => shard(kb_id, eve_count).await,
        VpkAction::Resolve { kb_id } => resolve(kb_id).await,
        VpkAction::Tenant(args) => tenant(args.action).await,
        VpkAction::Bench(args) => crate::vpk_bench::run(args).await,
    }
}

// ---------------------------------------------------------------------------
// enable
// ---------------------------------------------------------------------------

fn enable(mode: String, port: u16) -> Result<()> {
    match mode.as_str() {
        "bob" | "eve" | "both" => {}
        _ => anyhow::bail!("Invalid mode '{}'. Must be: bob, eve, or both", mode),
    }

    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.vpk_enabled = true;
    cfg.vpk_mode = Some(mode.clone());
    cfg.vpk_local_port = Some(port);
    cfg.save()?;

    print_box_header("🔐 VPK Integration Enabled");
    println!("  Mode:     {}", mode);
    println!("  Port:     {} (local health check)", port);
    println!();
    print_success("VPK integration enabled. Restart the node to advertise on DHT.");
    print_info(
        "Remote peers connect via PeerId over /kwaai/storage/1.0.0 (no port forwarding needed).",
    );
    print_info("Check status: kwaainet vpk status");
    print_info("Restart node: kwaainet restart");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// disable
// ---------------------------------------------------------------------------

fn disable() -> Result<()> {
    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.vpk_enabled = false;
    cfg.vpk_mode = None;
    cfg.vpk_local_port = None;
    cfg.save()?;

    print_box_header("🔐 VPK Integration Disabled");
    print_success("VPK integration disabled.");
    print_info("Restart the node to remove the DHT advertisement: kwaainet restart");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

async fn status() -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    print_box_header("🔐 VPK Status");

    if !cfg.vpk_enabled {
        println!("  VPK:      Disabled");
        println!();
        print_info("Enable with: kwaainet vpk enable --mode both");
        print_separator();
        return Ok(());
    }

    let port = cfg.vpk_local_port.unwrap_or(7432);
    let mode = cfg.vpk_mode.as_deref().unwrap_or("unknown");

    println!("  VPK:      Enabled");
    println!("  Mode:     {}", mode);
    println!("  Port:     {} (local health check)", port);
    println!("  Access:   P2P relay via /kwaai/storage/1.0.0 (PeerId-addressed)");
    println!();

    // Poll local VPK health endpoint
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let url = format!("http://localhost:{}/api/health", port);

    print!("  Local VPK: ");
    match client.get(&url).send().await {
        Ok(resp) if resp.status().is_success() => match resp.json::<serde_json::Value>().await {
            Ok(json) => {
                let health_status = json["status"].as_str().unwrap_or("ok");
                let tenant_count = json["tenant_count"].as_u64().unwrap_or(0);
                let capacity_gb = json["capacity_gb_available"].as_f64().unwrap_or(0.0);
                let version = json["version"].as_str().unwrap_or("unknown");
                let peer_id_cfg = json["peer_id"].as_str().unwrap_or("(not set)");

                println!("🟢 {}", health_status);
                println!("  Version:   {}", version);
                println!("  Tenants:   {}", tenant_count);
                println!("  Capacity:  {:.1} GB available", capacity_gb);
                println!("  Peer ID:   {}", peer_id_cfg);
            }
            Err(_) => println!("🟢 reachable (non-JSON response)"),
        },
        Ok(resp) => {
            println!("🟡 HTTP {}", resp.status());
        }
        Err(e) => {
            println!("🔴 unreachable");
            println!();
            print_warning(&format!(
                "VPK not responding on port {} — is it running?",
                port
            ));
            print_info(&format!("Error: {}", e));
        }
    }

    println!();
    print_info("DHT advertisement is refreshed every 120 s while the node is running.");
    print_info("View node logs for announcement status: kwaainet logs");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// discover — DHT FIND on _kwaai.vpk.nodes
//
// Reuses the already-running node's p2pd via its IPC socket rather than
// spawning a new daemon. Requires `kwaainet start --daemon` to be running.
// ---------------------------------------------------------------------------

async fn discover(json_output: bool) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;

    if !json_output {
        print_box_header("🔐 VPK Node Discovery");
        println!("  Querying DHT for VPK-capable nodes…");
        println!();
    }

    // Connect to the running node's p2pd over its IPC socket.
    // Respects KWAAINET_SOCKET so multi-instance setups (KWAAINET_HOME) work.
    let daemon_addr = crate::shard_cmd::daemon_socket();

    let mut client = match P2PClient::connect(&daemon_addr).await {
        Ok(c) => c,
        Err(_) => {
            print_error("Cannot connect to the KwaaiNet node — is it running?");
            print_info("Start it:     kwaainet start --daemon");
            print_info("Check status: kwaainet status");
            print_info("View logs:    kwaainet logs --follow");
            print_separator();
            return Ok(());
        }
    };

    // Identify ourselves so we can populate the FindRequest's peer field.
    let peer_id_hex = client.identify().await.context("identify peer")?;
    let peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse peer ID")?;

    let bootstrap_peers: Vec<String> = if cfg.initial_peers.is_empty() {
        NetworkConfig::with_petals_bootstrap().bootstrap_peers
    } else {
        cfg.initial_peers.clone()
    };

    // Build the FIND request for the VPK nodes registry key.
    // Python Hivemind validates that every node_id is a 20-byte DHTID (SHA1-range).
    // Raw PeerId bytes are 38+ bytes and fail that check, so we SHA1 them first —
    // same as DHTID.generate(peer_id.to_bytes()) in Python.
    let key = vpk_dht_id("_kwaai.vpk.nodes");
    let our_dhtid = Sha1::new()
        .chain_update(peer_id.to_bytes())
        .finalize()
        .to_vec();
    let find_req = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: Some(NodeInfo { node_id: our_dhtid }),
    };
    let mut req_bytes = Vec::new();
    find_req.encode(&mut req_bytes)?;

    // Query each bootstrap peer; deduplicate results by peer_id.
    let mut found: Vec<VpkNodeEntry> = Vec::new();

    for addr in &bootstrap_peers {
        let Some(peer_id_str) = addr.split("/p2p/").nth(1) else {
            continue;
        };
        let bp = match peer_id_str.parse::<PeerId>() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if client.connect_peer(addr).await.is_err() {
            continue;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;

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
            let rt = result.result_type;
            if result.value.is_empty() {
                continue;
            }
            // FoundRegular = 1 (our Rust DHTStorage or single-entry bootstrap)
            // FoundDictionary = 2 (Python Hivemind bootstrap with multiple nodes)
            if rt == 1 {
                if let Some(entry) = parse_vpk_regular(&result.value) {
                    if !found.iter().any(|e| e.peer_id == entry.peer_id) {
                        found.push(entry);
                    }
                }
            } else if rt == 2 {
                parse_vpk_dictionary(&result.value, &mut found);
            }
        }
    }

    // Remove the local node — dialling self is always rejected by p2pd.
    let local_b58 = peer_id.to_string();
    found.retain(|e| e.peer_id != local_b58);

    if found.is_empty() {
        if json_output {
            println!("[]");
        } else {
            println!();
            print_warning("No VPK-capable nodes found in DHT.");
            print_info("If VPK was just enabled, wait up to 120 s for the first announce cycle.");
            print_info("Enable VPK on a node: kwaainet storage init then kwaainet start --daemon");
            print_separator();
        }
        return Ok(());
    }

    if json_output {
        // Machine-readable output — one JSON array, no ANSI codes.
        let mut items: Vec<String> = Vec::new();
        for entry in &found {
            items.push(format!(
                r#"{{"public_name":{},"peer_id":{},"mode":{},"capacity_gb":{},"tenant_count":{}}}"#,
                serde_json::to_string(&entry.public_name).unwrap_or_default(),
                serde_json::to_string(&entry.peer_id).unwrap_or_default(),
                serde_json::to_string(&entry.mode).unwrap_or_default(),
                entry.capacity_gb,
                entry.tenant_count,
            ));
        }
        println!("[{}]", items.join(","));
        return Ok(());
    }

    println!();
    println!(
        "  Found {} VPK-capable node(s) — verifying reachability…\n",
        found.len()
    );

    for (i, entry) in found.iter().enumerate() {
        let short_id = if entry.peer_id.len() > 20 {
            format!("{}…", &entry.peer_id[..20])
        } else {
            entry.peer_id.clone()
        };

        // Verify reachability via P2P relay.
        let relay_status = if entry.peer_id != "unknown" {
            if let Ok(pid) = entry.peer_id.parse::<PeerId>() {
                match crate::storage_rpc::rpc_health(&client, &pid).await {
                    Ok(h) => format!(
                        "🟢 relay  {} tenant(s)  {:.1} GB free",
                        h.tenant_count, h.capacity_gb_available
                    ),
                    Err(_) => "🟡 relay unreachable".to_string(),
                }
            } else {
                "🟡 invalid peer_id".to_string()
            }
        } else {
            "⚫ no peer_id".to_string()
        };

        println!("  [{:>2}] {}", i + 1, relay_status);
        if !entry.public_name.is_empty() && entry.public_name != "unknown" {
            println!("       Name:     {}", entry.public_name);
        }
        println!("       PeerID:   {}", short_id);
        println!("       Mode:     {}", entry.mode);
        if entry.vpk_version != "unknown" {
            println!("       Version:  {}", entry.vpk_version);
        }
        println!(
            "       Capacity: {:.1} GB  |  Tenants: {}",
            entry.capacity_gb, entry.tenant_count
        );
        println!();
    }

    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// shard  (Phase 2 — cross-node Eve sharding)
// ---------------------------------------------------------------------------

async fn shard(kb_id: String, eve_count: usize) -> Result<()> {
    print_box_header("VPK Knowledge Base Sharding");
    println!("  KB ID:     {}", kb_id);
    println!("  Eve nodes: {}", eve_count);
    println!();
    print_warning("Sharding is not yet implemented as a single command.");
    println!();
    print_info("Important: sharding distributes a corpus across Eve nodes for RAM capacity.");
    print_info("It does NOT reduce query latency on WAN-connected nodes.");
    println!();
    println!("  Benchmarks (May 2026) show WAN fan-out search latency is dominated by");
    println!("  P2P round-trip time (25–93 ms), regardless of corpus size or shard count.");
    println!("  HNSW compute per shard is 1–2 ms — irrelevant next to WAN overhead.");
    println!();
    println!("  Use sharding when your knowledge base exceeds a single Eve's available RAM.");
    println!("  For latency-sensitive workloads, store locally or on a single nearby Eve.");
    println!();
    print_info("To measure your network's RTT before committing to a sharded topology:");
    println!("  kwaainet vpk bench --eve-peer-ids <PEER_IDS> --vectors 10000");
    println!();
    print_info("To see available Eve nodes:  kwaainet vpk discover");
    print_info("To create tenants manually:  kwaainet vpk tenant create --eve-peer-id <ID>");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// resolve  (Phase 3 — DHT FIND on _kwaai.vpk.kb.{kb_id})
// ---------------------------------------------------------------------------

async fn resolve(kb_id: String) -> Result<()> {
    print_box_header("VPK KB Resolution");
    println!("  KB ID: {}", kb_id);
    println!();
    print_warning("KB resolution from DHT is not yet implemented.");
    print_info("Shard topology recovery (DHT FIND on _kwaai.vpk.kb.<id>) is planned for Phase 3.");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// tenant  (Phase 2 — manage tenants on remote Eve nodes via P2P)
// ---------------------------------------------------------------------------

async fn tenant(action: TenantAction) -> Result<()> {
    match action {
        TenantAction::Create {
            eve_peer_id,
            capacity_mb,
            name,
            dimension,
        } => tenant_create(eve_peer_id, capacity_mb, name, dimension).await,

        TenantAction::List { eve_peer_id } => tenant_list(eve_peer_id).await,

        TenantAction::Info {
            tenant_id,
            eve_peer_id,
        } => tenant_info(tenant_id, eve_peer_id).await,

        TenantAction::Delete {
            tenant_id,
            eve_peer_id,
            yes,
        } => tenant_delete(tenant_id, eve_peer_id, yes).await,
    }
}

pub(crate) async fn p2p_connect() -> Result<(P2PClient, PeerId)> {
    let daemon_addr = crate::shard_cmd::daemon_socket();
    let client = P2PClient::connect(&daemon_addr)
        .await
        .context("connect to p2pd — is 'kwaainet start --daemon' running?")?;
    let identity =
        crate::identity::NodeIdentity::load_or_create().context("load local identity")?;
    Ok((client, identity.peer_id))
}

fn parse_peer_id(s: &str) -> Result<PeerId> {
    s.parse::<PeerId>()
        .with_context(|| format!("invalid PeerId: {s}"))
}

async fn tenant_create(
    eve_peer_id: String,
    capacity_mb: i64,
    name: Option<String>,
    dimension: usize,
) -> Result<()> {
    let eve = parse_peer_id(&eve_peer_id)?;
    let (client, my_peer_id) = p2p_connect().await?;

    print_box_header("VPK Tenant — Create");
    println!("  Eve:      {}", eve_peer_id);
    println!("  Capacity: {} MB", capacity_mb);
    if let Some(ref n) = name {
        println!("  Name:     {}", n);
    }
    println!("  Dim:      {}", dimension);
    println!();

    let info = crate::storage_rpc::rpc_create_tenant(
        &client,
        &eve,
        crate::storage_rpc::CreateTenantPayload {
            peer_id: my_peer_id.to_base58(),
            capacity_limit_mb: capacity_mb,
            display_name: name,
            vector_dimension: dimension,
        },
    )
    .await
    .context("create tenant on Eve node")?;

    print_success("Tenant created");
    println!();
    println!("  Tenant ID: {}", info.tenant_id);
    println!("  Status:    {}", info.status);
    println!("  Capacity:  {} MB", info.capacity_limit_mb);
    println!("  Created:   {}", info.created_at);
    println!();
    print_info("Save the Tenant ID — you'll need it to upload vectors and query.");
    println!("  kwaainet vpk tenant list --eve-peer-id {}", eve_peer_id);
    print_separator();
    Ok(())
}

async fn tenant_list(eve_peer_id: String) -> Result<()> {
    let eve = parse_peer_id(&eve_peer_id)?;
    let (client, _) = p2p_connect().await?;

    let tenants = crate::storage_rpc::rpc_list_tenants(&client, &eve)
        .await
        .context("list tenants on Eve node")?;

    print_box_header("VPK Tenants");
    println!("  Eve: {}", eve_peer_id);
    println!();

    if tenants.is_empty() {
        println!("  No tenants found.");
    } else {
        println!(
            "  {:<36}  {:<20}  {:>8}  Status",
            "Tenant ID", "Name", "Cap MB"
        );
        println!(
            "  {}  {}  {}  {}",
            "─".repeat(36),
            "─".repeat(20),
            "─".repeat(8),
            "─".repeat(8)
        );
        for t in &tenants {
            println!(
                "  {}  {:<20}  {:>8}  {}",
                t.tenant_id,
                t.display_name.as_deref().unwrap_or("—"),
                t.capacity_limit_mb,
                t.status,
            );
        }
    }
    print_separator();
    Ok(())
}

async fn tenant_info(tenant_id: String, eve_peer_id: String) -> Result<()> {
    let eve = parse_peer_id(&eve_peer_id)?;
    let tid: uuid::Uuid = tenant_id
        .parse()
        .with_context(|| format!("invalid tenant UUID: {tenant_id}"))?;
    let (client, _) = p2p_connect().await?;

    // Get the tenant from the list (no dedicated GetTenant P2P call needed;
    // we already have rpc_list_tenants which is cheap at O(tenants) scale).
    let tenants = crate::storage_rpc::rpc_list_tenants(&client, &eve)
        .await
        .context("query Eve node")?;

    let t = tenants
        .into_iter()
        .find(|t| t.tenant_id == tid)
        .with_context(|| format!("tenant {tenant_id} not found on Eve {eve_peer_id}"))?;

    print_box_header("VPK Tenant Info");
    println!("  Tenant ID: {}", t.tenant_id);
    println!("  Bob:       {}", t.peer_id);
    println!("  Name:      {}", t.display_name.as_deref().unwrap_or("—"));
    println!("  Capacity:  {} MB", t.capacity_limit_mb);
    println!("  Status:    {}", t.status);
    println!("  Created:   {}", t.created_at);
    println!("  Eve:       {}", eve_peer_id);
    print_separator();
    Ok(())
}

async fn tenant_delete(tenant_id: String, eve_peer_id: String, yes: bool) -> Result<()> {
    let eve = parse_peer_id(&eve_peer_id)?;
    let tid: uuid::Uuid = tenant_id
        .parse()
        .with_context(|| format!("invalid tenant UUID: {tenant_id}"))?;

    if !yes {
        print_warning(&format!(
            "This will permanently delete tenant {} and all its vectors on {}.",
            tenant_id, eve_peer_id
        ));
        print_info("Re-run with --yes to confirm.");
        return Ok(());
    }

    let (client, _) = p2p_connect().await?;

    crate::storage_rpc::rpc_delete_tenant(&client, &eve, tid)
        .await
        .context("delete tenant on Eve node")?;

    print_success(&format!("Tenant {} deleted", tenant_id));
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// SHA1(msgpack(raw_key)) — same as Hivemind's DHTID.generate().
fn vpk_dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// A decoded VPK node advertisement from the DHT.
struct VpkNodeEntry {
    peer_id: String,
    mode: String,
    capacity_gb: f64,
    tenant_count: u32,
    vpk_version: String,
    public_name: String,
}

/// Decode a FoundRegular value (direct msgpack VPK map) into a VpkNodeEntry.
/// peer_id is unavailable in FoundRegular responses so it is set to "unknown".
fn parse_vpk_regular(bytes: &[u8]) -> Option<VpkNodeEntry> {
    decode_vpk_map(bytes, "unknown".to_string())
}

/// Decode a FoundDictionary value into one VpkNodeEntry per subkey.
///
/// Python Hivemind serialises DictionaryDHTValue as:
///   Ext(80, msgpack([global_expiry_f64, created_f64, [[subkey_str, value_bytes, entry_expiry_f64], …]]))
fn parse_vpk_dictionary(bytes: &[u8], out: &mut Vec<VpkNodeEntry>) {
    // Outer layer: Ext(80, inner_bytes)
    let outer = match rmpv::decode::read_value(&mut &bytes[..]) {
        Ok(v) => v,
        Err(_) => return,
    };
    let inner_bytes = match &outer {
        rmpv::Value::Ext(80, b) => b.as_slice(),
        _ => return,
    };

    // Inner layer: [global_expiry, created_time, [[subkey, value, expiry], …]]
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

        // subkey is a plain string — the peer_id base58
        let peer_id = match &arr[0] {
            rmpv::Value::String(s) => s.as_str().unwrap_or("?").to_string(),
            _ => continue,
        };

        // value is binary msgpack bytes of the VPK capability map
        let value_bytes = match &arr[1] {
            rmpv::Value::Binary(b) => b.as_slice(),
            _ => continue,
        };

        if let Some(entry) = decode_vpk_map(value_bytes, peer_id.clone()) {
            if !out.iter().any(|e| e.peer_id == peer_id) {
                out.push(VpkNodeEntry { peer_id, ..entry });
            }
        }
    }
}

/// Decode msgpack({ mode, capacity_gb, tenant_count, vpk_version, public_name })
/// into a VpkNodeEntry, using `peer_id` as the node identifier.
fn decode_vpk_map(bytes: &[u8], peer_id: String) -> Option<VpkNodeEntry> {
    let val = rmpv::decode::read_value(&mut &bytes[..]).ok()?;
    let map = val.as_map()?;

    let get_str = |key: &str| -> String {
        map.iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .and_then(|(_, v)| v.as_str())
            .unwrap_or("unknown")
            .to_string()
    };
    let get_f64 = |key: &str| -> f64 {
        map.iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .and_then(|(_, v)| v.as_f64())
            .unwrap_or(0.0)
    };
    let get_u32 = |key: &str| -> u32 {
        map.iter()
            .find(|(k, _)| k.as_str() == Some(key))
            .and_then(|(_, v)| v.as_u64())
            .unwrap_or(0) as u32
    };

    // If the map doesn't have "mode" it's probably not a VPK record — skip it.
    if get_str("mode") == "unknown" {
        return None;
    }

    Some(VpkNodeEntry {
        peer_id,
        mode: get_str("mode"),
        capacity_gb: get_f64("capacity_gb"),
        tenant_count: get_u32("tenant_count"),
        vpk_version: get_str("vpk_version"),
        public_name: get_str("public_name"),
    })
}
