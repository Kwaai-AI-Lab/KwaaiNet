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

/// Connection classification. `LIST_PEERS` only gives us `(id, addrs)` so this
/// is derived from the multiaddr alone.
#[derive(Copy, Clone, PartialEq, Eq)]
enum ConnKind {
    /// Plain `/ip4/.../tcp/...` — directly dialable.
    Direct,
    /// Path includes `/p2p-circuit/` — going through a relay.
    Relay,
}

fn classify_addr(m: &Multiaddr) -> ConnKind {
    if m.iter()
        .any(|p| matches!(p, libp2p::multiaddr::Protocol::P2pCircuit))
    {
        ConnKind::Relay
    } else {
        ConnKind::Direct
    }
}

/// Backwards-compat shim retained so `info` keeps working unchanged.
fn is_relayed(m: &Multiaddr) -> bool {
    classify_addr(m) == ConnKind::Relay
}

/// Inline ANSI label for a `ConnKind`. Green for direct, yellow for relay.
/// Keeping ANSI inline avoids pulling in a colour crate for two strings.
fn fmt_kind(k: ConnKind) -> &'static str {
    match k {
        ConnKind::Direct => "\x1b[32m[direct]\x1b[0m",
        ConnKind::Relay => "\x1b[33m[ relay]\x1b[0m",
    }
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

/// Build the set of trusted-relay peer IDs the local node was configured with.
/// Empty when the user hasn't configured any (the production default).
fn trusted_relay_peer_ids() -> HashSet<PeerId> {
    let relays = KwaaiNetConfig::load_or_create()
        .map(|cfg| cfg.trusted_relays)
        .unwrap_or_default();

    relays
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
        .identify_full()
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
    let trusted_relays = trusted_relay_peer_ids();

    // Group ordering for the sort: bootstrap first (regardless of conn kind),
    // then trusted relay, then plain direct, then via-relay. Within each
    // group, sort by peer ID so output is stable across polls — useful when
    // watching the list change as peers come and go.
    fn group_index(is_bootstrap: bool, is_trusted_relay: bool, kind: ConnKind) -> u8 {
        if is_bootstrap {
            return 0;
        }
        if is_trusted_relay {
            return 1;
        }
        match kind {
            ConnKind::Direct => 2,
            ConnKind::Relay => 3,
        }
    }

    struct Row {
        group: u8,
        id_str: String,
        kind: ConnKind,
        is_bootstrap: bool,
        is_trusted_relay: bool,
        preview: String,
        extra_count: usize,
    }

    let mut rows: Vec<Row> = peers
        .iter()
        .map(|p| {
            let parsed_id = PeerId::from_bytes(&p.id).ok();
            let id_str = parsed_id
                .map(|p| p.to_base58())
                .unwrap_or_else(|| format!("0x{}", hex::encode(&p.id)));
            let is_bootstrap = parsed_id.map_or(false, |pid| bootstraps.contains(&pid));
            let is_trusted_relay = parsed_id.map_or(false, |pid| trusted_relays.contains(&pid));

            // Classify by the primary (first) addr. p2pd returns one PeerInfo
            // per connection; multiple PeerInfos for the same peer ID indicate
            // multiple simultaneous connections (e.g. relay path + direct path
            // during a hole-punch upgrade).
            let kind = p
                .addrs
                .first()
                .and_then(|a| Multiaddr::try_from(a.clone()).ok())
                .map(|m| classify_addr(&m))
                .unwrap_or(ConnKind::Direct);

            let preview = p
                .addrs
                .first()
                .map(|a| fmt_addr(a))
                .unwrap_or_else(|| "(no addrs)".to_string());
            let extra_count = p.addrs.len().saturating_sub(1);

            Row {
                group: group_index(is_bootstrap, is_trusted_relay, kind),
                id_str,
                kind,
                is_bootstrap,
                is_trusted_relay,
                preview,
                extra_count,
            }
        })
        .collect();

    rows.sort_by(|a, b| a.group.cmp(&b.group).then_with(|| a.id_str.cmp(&b.id_str)));

    let mut direct = 0usize;
    let mut relayed = 0usize;
    let mut bootstrap_hits = 0usize;
    let mut trusted_relay_hits = 0usize;
    for r in &rows {
        match r.kind {
            ConnKind::Direct => direct += 1,
            ConnKind::Relay => relayed += 1,
        }
        if r.is_bootstrap {
            bootstrap_hits += 1;
        }
        if r.is_trusted_relay {
            trusted_relay_hits += 1;
        }

        let extra = if r.extra_count > 0 {
            format!(" (+{} more)", r.extra_count)
        } else {
            String::new()
        };
        // Cyan for bootstrap, gold (256-color 220) for trusted relay — both
        // surface configuration that the user explicitly chose. Inline ANSI
        // avoids a colour-crate dep for two strings; if colour use spreads,
        // revisit.
        let label = if r.is_bootstrap {
            "  \x1b[36m(bootstrap)\x1b[0m"
        } else if r.is_trusted_relay {
            "  \x1b[38;5;220m(trusted relay)\x1b[0m"
        } else {
            ""
        };
        println!("  {} {}{}", fmt_kind(r.kind), r.id_str, label);
        println!("           {}{}", r.preview, extra);
    }

    println!();
    print_info(&format!(
        "Total {} connection(s): {} direct, {} via relay; \
         {} to bootstrap peer(s), {} to trusted relay(s)",
        peers.len(),
        direct,
        relayed,
        bootstrap_hits,
        trusted_relay_hits
    ));
    print_info(
        "Each row is one live connection — a peer with both a direct and a \
         relay path (e.g. during a hole-punch upgrade) appears twice.",
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
