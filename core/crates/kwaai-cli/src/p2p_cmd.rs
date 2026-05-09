//! `kwaainet p2p` — live diagnostics for the local p2pd
//!
//! All commands talk only to the local p2pd over its IPC socket. `info` and
//! `peers list` return p2pd's in-memory view; `peers find` issues an active
//! Kademlia lookup via p2pd.

use std::collections::HashSet;

use anyhow::{Context, Result};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PClient;
use libp2p::{Multiaddr, PeerId};

use crate::cli::{P2pAction, P2pArgs, PeersAction, PeersArgs};
use crate::config::KwaaiNetConfig;
use crate::display::*;
use crate::shard_cmd::daemon_socket;

pub async fn run(args: P2pArgs) -> Result<()> {
    match args.action {
        P2pAction::Info => info().await,
        P2pAction::Peers(p) => peers(p).await,
    }
}

async fn peers(args: PeersArgs) -> Result<()> {
    match args.action {
        PeersAction::List => peers_list().await,
        PeersAction::Find { peer_id, timeout } => peers_find(peer_id, timeout).await,
    }
}

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Connect to the local p2pd, or print the standard "node not running" message
/// and return `Ok(None)` so the caller exits cleanly with status 0.
async fn connect_p2pd() -> Result<Option<P2PClient>> {
    let addr = daemon_socket();
    match P2PClient::connect(&addr).await {
        Ok(c) => Ok(Some(c)),
        Err(_) => {
            print_error("Cannot connect to the KwaaiNet node — is it running?");
            print_info("Start it:     kwaainet start --daemon");
            print_info("Check status: kwaainet status");
            print_info("View logs:    kwaainet logs --follow");
            print_separator();
            Ok(None)
        }
    }
}

/// Decode raw protobuf-bytes multiaddr into the displayable text form.
/// Falls back to a hex preview so `peers list` never panics on a malformed
/// addr from p2pd.
fn fmt_addr(bytes: &[u8]) -> String {
    Multiaddr::try_from(bytes.to_vec())
        .map(|m| m.to_string())
        .unwrap_or_else(|_| format!("0x{} (unparseable)", hex::encode(bytes)))
}

/// Classify a multiaddr for the dcutr signal we care about most: is this
/// connection going through a relay circuit, or directly?
fn is_relayed(m: &Multiaddr) -> bool {
    m.iter()
        .any(|p| matches!(p, libp2p::multiaddr::Protocol::P2pCircuit))
}

/// Build the set of bootstrap peer IDs the local node was configured to use.
/// Prefers the user's `initial_peers` override; falls back to the built-in
/// KwaaiNet/Petals defaults. Same precedence as `vpk discover` and `node.rs`.
fn bootstrap_peer_ids() -> HashSet<PeerId> {
    let bootstraps: Vec<String> = match KwaaiNetConfig::load_or_create() {
        Ok(cfg) if !cfg.initial_peers.is_empty() => cfg.initial_peers,
        _ => NetworkConfig::with_petals_bootstrap().bootstrap_peers,
    };

    bootstraps
        .iter()
        .filter_map(|addr| addr.split("/p2p/").nth(1))
        .filter_map(|id| id.parse::<PeerId>().ok())
        .collect()
}

// ---------------------------------------------------------------------------
// info
// ---------------------------------------------------------------------------

async fn info() -> Result<()> {
    let Some(mut client) = connect_p2pd().await? else {
        return Ok(());
    };

    let (peer_id_hex, addrs_bytes) = client
        .identify_with_addrs()
        .await
        .context("IDENTIFY request to p2pd failed")?;

    let peer_id = hex::decode(&peer_id_hex)
        .ok()
        .and_then(|b| PeerId::from_bytes(&b).ok())
        .map(|p| p.to_base58())
        .unwrap_or_else(|| format!("0x{} (unparseable)", peer_id_hex));

    let addrs: Vec<Multiaddr> = addrs_bytes
        .iter()
        .filter_map(|a| Multiaddr::try_from(a.clone()).ok())
        .collect();

    print_box_header("🛰  KwaaiNet P2P — Local Node Identity");
    println!("  Peer ID:  {}", peer_id);
    println!();

    if addrs.is_empty() {
        println!("  Addresses: (none reported by p2pd)");
        print_warning(
            "p2pd hasn't reported any listen/observed addresses yet. \
             The node may have just started — try again in a few seconds.",
        );
    } else {
        println!("  Addresses ({}):", addrs.len());
        for a in &addrs {
            let kind = if is_relayed(a) { "relay" } else { "direct" };
            println!("    [{:>6}] {}", kind, a);
        }
    }

    println!();
    let direct_count = addrs.iter().filter(|a| !is_relayed(a)).count();
    let relay_count = addrs.len() - direct_count;
    if addrs.is_empty() {
        print_info("Reachability: unknown (no addresses yet)");
    } else if direct_count > 0 && relay_count == 0 {
        print_info("Reachability: direct addresses only — looks publicly reachable");
    } else if direct_count > 0 {
        print_info(&format!(
            "Reachability: mixed ({} direct, {} via relay) — partially reachable",
            direct_count, relay_count
        ));
    } else {
        print_info(&format!(
            "Reachability: relay-only ({} circuit addrs) — likely behind NAT",
            relay_count
        ));
    }
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// peers list / info / find
// ---------------------------------------------------------------------------

async fn peers_list() -> Result<()> {
    let Some(mut client) = connect_p2pd().await? else {
        return Ok(());
    };

    let peers = client
        .list_peers()
        .await
        .context("LIST_PEERS request to p2pd failed")?;

    print_box_header("🛰  KwaaiNet P2P — Active Connections");

    if peers.is_empty() {
        println!("  (no active connections)");
        print_separator();
        return Ok(());
    }

    let bootstraps = bootstrap_peer_ids();

    let mut direct = 0usize;
    let mut relayed = 0usize;
    let mut bootstrap_hits = 0usize;
    for p in &peers {
        let parsed_id = PeerId::from_bytes(&p.id).ok();
        let id_str = parsed_id
            .map(|p| p.to_base58())
            .unwrap_or_else(|| format!("0x{}", hex::encode(&p.id)));
        let is_bootstrap = parsed_id.map_or(false, |pid| bootstraps.contains(&pid));
        if is_bootstrap {
            bootstrap_hits += 1;
        }

        let any_relay = p
            .addrs
            .iter()
            .filter_map(|a| Multiaddr::try_from(a.clone()).ok())
            .any(|m| is_relayed(&m));
        if any_relay {
            relayed += 1;
        } else {
            direct += 1;
        }
        let kind = if any_relay { "relay" } else { "direct" };
        let preview = p
            .addrs
            .first()
            .map(|a| fmt_addr(a))
            .unwrap_or_else(|| "(no addrs)".to_string());
        let extra = if p.addrs.len() > 1 {
            format!(" (+{} more)", p.addrs.len() - 1)
        } else {
            String::new()
        };
        // Cyan for bootstrap so the label stands out at a glance without
        // being garish. Inline ANSI keeps us off the dep treadmill for a
        // single annotation; revisit if color use spreads.
        let label = if is_bootstrap {
            "  \x1b[36m(bootstrap)\x1b[0m"
        } else {
            ""
        };
        println!("  [{:>6}] {}{}", kind, id_str, label);
        println!("           {}{}", preview, extra);
    }

    println!();
    print_info(&format!(
        "Total {} connection(s): {} direct, {} via relay; {} to bootstrap peer(s)",
        peers.len(),
        direct,
        relayed,
        bootstrap_hits
    ));
    print_info(
        "Each row is one live connection — a peer with both a direct and a \
         relay path (e.g. during a dcutr upgrade) appears twice.",
    );
    print_separator();
    Ok(())
}

async fn peers_find(peer_id_str: String, timeout: i64) -> Result<()> {
    let target: PeerId = peer_id_str
        .parse()
        .context("invalid peer ID (expected base58, e.g. 12D3KooW…)")?;

    let Some(mut client) = connect_p2pd().await? else {
        return Ok(());
    };

    print_box_header("🛰  KwaaiNet P2P — DHT Peer Lookup");
    println!("  Looking up: {}", target.to_base58());
    println!("  Timeout:    {}s", timeout);
    println!();

    match client.dht_find_peer(target.to_bytes(), Some(timeout)).await {
        Ok(info) => {
            if info.addrs.is_empty() {
                println!("  Found in DHT, but no addresses advertised.");
            } else {
                println!("  Addresses advertised in DHT ({}):", info.addrs.len());
                for a in &info.addrs {
                    match Multiaddr::try_from(a.clone()) {
                        Ok(m) => {
                            let kind = if is_relayed(&m) { "relay" } else { "direct" };
                            println!("    [{:>6}] {}", kind, m);
                        }
                        Err(_) => {
                            println!("    [   ?   ] 0x{} (unparseable)", hex::encode(a))
                        }
                    }
                }
            }
        }
        Err(_) => {
            print_warning(&format!("not found in DHT (timeout: {}s)", timeout));
            print_info(
                "Either the peer hasn't published its addresses, or the \
                 lookup didn't finish in time. Try a longer --timeout.",
            );
        }
    }
    print_separator();
    Ok(())
}
