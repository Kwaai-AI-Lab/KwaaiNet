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
        PeersAction::Connect {
            addr,
            peer,
            message,
        } => peers_connect(addr, peer, message).await,
        PeersAction::Send {
            peer,
            proto,
            message,
            payload_hex,
            payload_bin,
            stdin,
            timeout,
        } => {
            peers_send(
                peer,
                proto,
                message,
                payload_hex,
                payload_bin,
                stdin,
                timeout,
            )
            .await
        }
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
                // Append `/p2p/<peer-id>` to each address so the printed
                // form is directly usable as `peers connect --addr …`.
                // p2pd / Kademlia returns addresses without the trailing
                // `/p2p/<self>` because the DHT entry is keyed on the peer
                // ID anyway, but the CLI consumer needs the full form.
                for a in &info.addrs {
                    match Multiaddr::try_from(a.clone()) {
                        Ok(m) => {
                            let kind = if is_relayed(&m) { "relay" } else { "direct" };
                            let with_dest = m.with(libp2p::multiaddr::Protocol::P2p(target.into()));
                            println!("    [{:>6}] {}", kind, with_dest);
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

// ---------------------------------------------------------------------------
// peers connect — manual dial + optional hello DM
// ---------------------------------------------------------------------------

async fn peers_connect(
    addr: Option<String>,
    peer: Option<String>,
    message: Option<String>,
) -> Result<()> {
    let Some(mut client) = connect_p2pd().await? else {
        return Ok(());
    };

    // Resolve the input form (--addr | --peer) to a (multiaddr_to_dial,
    // dest_peer_id) pair. clap's `required_unless_present` enforces that
    // exactly one of the two is set.
    let (multiaddr, dest_peer) = match (addr, peer) {
        (Some(a), None) => match resolve_addr(&a) {
            Ok(pair) => pair,
            Err(e) => {
                print_error(&format!("--addr rejected: {}", e));
                print_separator();
                return Ok(());
            }
        },
        (None, Some(p)) => {
            let target: PeerId = match p.parse() {
                Ok(p) => p,
                Err(e) => {
                    print_error(&format!("--peer is not a valid base58 peer ID: {}", e));
                    print_separator();
                    return Ok(());
                }
            };
            match resolve_peer(&mut client, target).await {
                Ok(pair) => pair,
                Err(e) => {
                    print_error(&format!("--peer DHT lookup failed: {}", e));
                    print_separator();
                    return Ok(());
                }
            }
        }
        // clap should make these unreachable, but be explicit.
        (Some(_), Some(_)) | (None, None) => {
            print_error("specify exactly one of --addr or --peer");
            print_separator();
            return Ok(());
        }
    };

    print_box_header("🛰  KwaaiNet P2P — Manual Dial");
    println!("  Target peer: {}", dest_peer);
    println!("  Multiaddr:   {}", multiaddr);
    println!();

    if let Err(e) = client.connect_peer(&multiaddr).await {
        print_error(&format!("connect_peer failed: {}", e));
        print_separator();
        return Ok(());
    }
    print_success("Connected.");

    if let Some(msg) = message {
        match client
            .call_unary_handler(
                &dest_peer.to_bytes(),
                crate::p2p_hello::HELLO_PROTO,
                msg.as_bytes(),
            )
            .await
        {
            Ok(resp) => {
                print_success(&format!("Sent hello to {} — see their logs.", dest_peer));
                print_response(&resp);
            }
            Err(e) => print_error(&format!("hello send failed: {}", e)),
        }
    } else {
        print_info("Pass --message <text> to send a hello DM that the peer logs.");
    }
    print_separator();
    Ok(())
}

/// Parse an `--addr` multiaddr and identify the destination peer ID.
///
/// For a relay'd address (`…/p2p/<RELAY>/p2p-circuit/p2p/<DEST>`) the
/// destination is the LAST `/p2p/` component. We refuse multiaddrs where
/// `/p2p-circuit` is present but no `/p2p/<peer>` follows it — those addresses
/// (as returned by p2pd's DHT layer) name only the relay, not the destination,
/// and dialing them is almost certainly a copy-paste mistake. The user wanted
/// to reach the peer, not the relay.
fn resolve_addr(addr: &str) -> Result<(String, PeerId)> {
    let maddr: Multiaddr = addr.parse().context("parse --addr as multiaddr")?;

    let mut last_p2p_was_after_circuit = !is_relayed(&maddr);
    let mut saw_circuit = false;
    let mut last_p2p = None;
    for proto in maddr.iter() {
        match proto {
            libp2p::multiaddr::Protocol::P2pCircuit => {
                saw_circuit = true;
                // Reset: only `/p2p/` components AFTER the circuit hop name
                // the destination. A `/p2p/` before the circuit names the
                // relay.
                last_p2p = None;
            }
            libp2p::multiaddr::Protocol::P2p(hash) => {
                last_p2p = Some(hash);
                if saw_circuit {
                    last_p2p_was_after_circuit = true;
                }
            }
            _ => {}
        }
    }

    let dest_hash = last_p2p.context(
        "multiaddr has no /p2p/<peer-id> component — pass a complete address \
         like /ip4/<IP>/tcp/<PORT>/p2p/<PEER-ID> or use --peer to look it up \
         in the DHT",
    )?;

    if saw_circuit && !last_p2p_was_after_circuit {
        anyhow::bail!(
            "multiaddr ends with /p2p-circuit but has no /p2p/<destination> \
             after it — this names only the relay, not the peer you want to \
             reach. Append /p2p/<destination-peer-id> to the multiaddr, or \
             use --peer <peer-id> to let the CLI do the DHT lookup for you."
        );
    }

    let dest_peer = PeerId::from_multihash(dest_hash.into())
        .map_err(|e| anyhow::anyhow!("invalid peer multihash in /p2p/: {:?}", e))?;
    Ok((maddr.to_string(), dest_peer))
}

/// Look up `target` in the DHT and pick a multiaddr to dial. Prefers a direct
/// address; falls back to the first relay'd address. Always appends
/// `/p2p/<target>` so the returned multiaddr names the destination
/// (p2pd / Kademlia returns addresses without the trailing `/p2p/<self>` since
/// the DHT entry is keyed on the peer ID anyway).
async fn resolve_peer(
    client: &mut kwaai_p2p_daemon::P2PClient,
    target: PeerId,
) -> Result<(String, PeerId)> {
    let info = client
        .dht_find_peer(target.to_bytes(), Some(10))
        .await
        .context("dht_find_peer")?;
    if info.addrs.is_empty() {
        anyhow::bail!("peer found in DHT but no addresses advertised");
    }

    let parsed: Vec<Multiaddr> = info
        .addrs
        .iter()
        .filter_map(|a| Multiaddr::try_from(a.clone()).ok())
        .collect();
    if parsed.is_empty() {
        anyhow::bail!("DHT returned addresses but none parsed as a multiaddr");
    }

    // Prefer direct over relay'd. If both kinds are present a direct dial is
    // always faster and gives hole-punching nothing to work on.
    let pick = parsed
        .iter()
        .find(|m| !is_relayed(m))
        .or_else(|| parsed.first())
        .expect("parsed is non-empty");

    let with_dest = pick
        .clone()
        .with(libp2p::multiaddr::Protocol::P2p(target.into()));
    Ok((with_dest.to_string(), target))
}

// ---------------------------------------------------------------------------
// peers send — invoke a unary RPC on a connected peer
// ---------------------------------------------------------------------------

async fn peers_send(
    peer: String,
    proto: String,
    message: Option<String>,
    payload_hex: Option<String>,
    payload_bin: Option<std::path::PathBuf>,
    stdin: bool,
    timeout_secs: u64,
) -> Result<()> {
    let payload = match resolve_payload(message, payload_hex, payload_bin, stdin) {
        Ok(bytes) => bytes,
        Err(e) => {
            print_error(&format!("payload: {}", e));
            print_separator();
            return Ok(());
        }
    };

    let dest_peer: PeerId = match peer.parse() {
        Ok(p) => p,
        Err(e) => {
            print_error(&format!("invalid recipient peer ID: {}", e));
            print_separator();
            return Ok(());
        }
    };

    let Some(mut client) = connect_p2pd().await? else {
        return Ok(());
    };

    print_box_header("🛰  KwaaiNet P2P — Unary RPC");
    println!("  To:      {}", dest_peer);
    println!("  Proto:   {}", proto);
    println!("  Payload: {} bytes", payload.len());
    println!("  Timeout: {}s", timeout_secs);
    println!();

    let dest_bytes = dest_peer.to_bytes();
    let call = client.call_unary_handler(&dest_bytes, &proto, &payload);
    match tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), call).await {
        Ok(Ok(resp)) => {
            print_success("Sent.");
            print_response(&resp);
        }
        Ok(Err(e)) => print_error(&format!("call_unary_handler failed: {}", e)),
        Err(_) => {
            print_error(&format!(
                "no response within {}s — recipient may not handle this protocol, \
                 or the handler hung on the payload. Try a longer --timeout.",
                timeout_secs
            ));
        }
    }
    print_separator();
    Ok(())
}

/// Resolve the user's payload-source flags to the concrete bytes that go on
/// the wire. Exactly one source must be set; clap groups them but doesn't
/// enforce one-and-only-one across the whole set, so we validate here.
fn resolve_payload(
    message: Option<String>,
    payload_hex: Option<String>,
    payload_bin: Option<std::path::PathBuf>,
    stdin: bool,
) -> Result<Vec<u8>> {
    let sources: Vec<&str> = [
        message.as_ref().map(|_| "--message"),
        payload_hex.as_ref().map(|_| "--payload-hex"),
        payload_bin.as_ref().map(|_| "--payload-bin"),
        stdin.then_some("--stdin"),
    ]
    .into_iter()
    .flatten()
    .collect();
    match sources.len() {
        0 => anyhow::bail!(
            "no payload — pass exactly one of --message, --payload-hex, \
             --payload-bin, or --stdin"
        ),
        1 => {}
        _ => anyhow::bail!(
            "multiple payload sources given ({}); pass exactly one",
            sources.join(", ")
        ),
    }

    if let Some(text) = message {
        return Ok(text.into_bytes());
    }
    if let Some(hex_str) = payload_hex {
        let stripped: String = hex_str.chars().filter(|c| !c.is_whitespace()).collect();
        return hex::decode(&stripped).context("decode --payload-hex");
    }
    if let Some(path) = payload_bin {
        return std::fs::read(&path)
            .with_context(|| format!("read --payload-bin {}", path.display()));
    }
    if stdin {
        use std::io::Read;
        let mut buf = Vec::new();
        std::io::stdin()
            .read_to_end(&mut buf)
            .context("read --stdin")?;
        return Ok(buf);
    }
    unreachable!("payload source validation above ensures one is set")
}

/// Display response bytes uniformly: short summary, then either UTF-8 text
/// (if the bytes are printable) or hex. Mirrors what curl-style tools do
/// and avoids guessing the wire format the protocol owns.
fn print_response(resp: &[u8]) {
    if resp.is_empty() {
        print_info("Response: (empty)");
        return;
    }
    match std::str::from_utf8(resp) {
        Ok(s) if s.chars().all(|c| !c.is_control() || c == '\n' || c == '\t') => {
            print_info(&format!("Response ({} bytes): {}", resp.len(), s));
        }
        _ => {
            let preview: String = resp
                .iter()
                .take(64)
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let suffix = if resp.len() > 64 { " …" } else { "" };
            print_info(&format!(
                "Response ({} bytes): {}{}",
                resp.len(),
                preview,
                suffix
            ));
        }
    }
}
