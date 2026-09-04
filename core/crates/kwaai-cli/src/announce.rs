//! DHT announce records and their native delivery path.
//!
//! The record value types' msgpack encodings are a published wire format:
//! `map.kwaai.ai`'s crawler and Python Hivemind both decode them, so they must
//! not be "improved" without a coordinated change on the consumer side.
//!
//! # The four record kinds
//!
//! Every announce writes the same four kinds, all with a 360 s TTL,
//! `in_cache = false`, and a **real subkey** (`msgpack(peer_id_base58)` — never
//! [`kwaai_hivemind_dht::IS_REGULAR_VALUE`] or
//! [`kwaai_hivemind_dht::IS_DICTIONARY`]), which is what lets many servers
//! accumulate under one key rather than overwrite each other:
//!
//! | key | subkey | value |
//! | --- | --- | --- |
//! | `{prefix}.{block}`, one per served block | msgpack(peer b58) | `Ext(64)` ServerInfo |
//! | `_petals.models` | msgpack(prefix) | msgpack `{repository, num_blocks}` |
//! | `_kwaai.inference.nodes` | msgpack(peer b58) | `Ext(64)` ServerInfo |
//! | `_kwaai.vpk.nodes` (VPK nodes only) | msgpack(peer b58) | msgpack VPK map |

use anyhow::Result;
use kwaai_hivemind_dht::protocol::{NodeInfo, RequestAuthInfo, StoreRequest, StoreResponse};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_p2p::{NetworkHandle, PeerId};
use libp2p::Multiaddr;
use prost::Message;
use sha1::{Digest, Sha1};
use std::collections::{HashMap, HashSet};
use tracing::{info, warn};

use crate::config::KwaaiNetConfig;

/// TTL applied to every announced record, in seconds.
///
/// Hivemind bootstraps reject a store whose expiration is not **strictly
/// greater** than the record it replaces, so the re-announce interval must stay
/// comfortably below this and every refresh must move the timestamp forward.
pub const ANNOUNCE_TTL_SECS: f64 = 360.0;

/// The `_petals.models` registry key — one entry per model prefix.
pub const PETALS_MODELS_KEY: &str = "_petals.models";

/// The `_kwaai.vpk.nodes` registry key — one entry per VPK-capable node.
pub const VPK_NODES_KEY: &str = "_kwaai.vpk.nodes";

// ---------------------------------------------------------------------------
// VPK capability info
// ---------------------------------------------------------------------------

/// VPK (Virtual Private Knowledge) capability snapshot used in DHT records.
///
/// Populated by polling `GET http://localhost:{vpk_local_port}/api/health`
/// immediately before each DHT announcement. When VPK is unreachable the
/// field is absent from both the per-block record and the nodes registry.
///
/// Nodes are identified solely by PeerId — no IP addresses are advertised.
/// Remote Bobs connect via `/kwaai/storage/1.0.0` over the libp2p relay.
pub struct VpkInfo {
    pub mode: String,
    pub capacity_gb: f64,
    pub tenant_count: u32,
    pub vpk_version: String,
    pub public_name: String,
}

impl VpkInfo {
    /// Build the rmpv Map that appears as the `"vpk"` value in DHT field maps.
    pub fn to_msgpack_value(&self) -> rmpv::Value {
        rmpv::Value::Map(vec![
            (
                rmpv::Value::from("mode"),
                rmpv::Value::from(self.mode.as_str()),
            ),
            (
                rmpv::Value::from("capacity_gb"),
                rmpv::Value::from(self.capacity_gb),
            ),
            (
                rmpv::Value::from("tenant_count"),
                rmpv::Value::from(i64::from(self.tenant_count)),
            ),
            (
                rmpv::Value::from("vpk_version"),
                rmpv::Value::from(self.vpk_version.as_str()),
            ),
            (
                rmpv::Value::from("public_name"),
                rmpv::Value::from(self.public_name.as_str()),
            ),
        ])
    }

    /// Standalone msgpack bytes for the `_kwaai.vpk.nodes` DHT record value.
    pub fn to_msgpack_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &self.to_msgpack_value())?;
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// DHT value types (Hivemind wire format)
// ---------------------------------------------------------------------------

/// Server info serialised as ExtType(64, [state, throughput, {fields}])
/// — the exact format Python Hivemind / map.kwaai.ai expects.
///
/// The optional `trust_attestations` field carries the node's Verifiable
/// Credentials as compact JSON strings. Clients that understand the KwaaiNet
/// trust model (e.g., map.kwaai.ai v2) display trust badges; legacy clients
/// ignore the field.
///
/// `state` follows petals' `ServerState` — `0` offline, `1` joining, `2`
/// online — with one KwaaiNet extension: `-1`, an explicit departure tombstone
/// (a shutdown cannot shorten a record's expiration, so leaving has to be said
/// in the value). The map decodes these directly; preserve them verbatim.
pub struct DHTServerInfo {
    pub state: i32,
    pub throughput: f64,
    pub start_block: i32,
    pub end_block: i32,
    pub public_name: String,
    pub version: String,
    pub torch_dtype: String,
    pub using_relay: bool,
    pub cache_tokens_left: i64,
    #[allow(dead_code)]
    pub next_pings: HashMap<String, f64>,
    #[allow(dead_code)]
    pub adapters: Vec<String>,
    /// Compact JSON representations of the node's valid Verifiable Credentials.
    /// Empty when no credentials are stored; included in the DHT fields map
    /// only when non-empty to keep announcement payloads minimal.
    pub trust_attestations: Vec<String>,

    /// VPK capability snapshot. None when VPK is disabled or unreachable.
    /// Included in the DHT fields map only when Some.
    pub vpk_info: Option<VpkInfo>,

    /// Peer ID in base58 encoding. Included in the value map so that chain
    /// discovery can identify the serving peer even from FoundRegular responses
    /// (which do not carry the DHT subkey). Unknown fields are silently ignored
    /// by legacy Python Hivemind clients.
    pub peer_id_b58: String,

    /// Capacity Lease capability flag: this node supports negotiating a
    /// GPU-slot lease before dispatch (see `capacity_lease.rs`), over
    /// either `/kwaai/capacity-lease/1.0.0` (unary) or lease frames on an
    /// already-open `/kwaai/inference-mux/1.0.0` stream. Always `true` for
    /// any binary built with this field — it describes this binary's own
    /// capability, not configuration, exactly like `version`. Lets a
    /// requester skip straight to negotiation instead of an
    /// attempt-and-fallback probe; absence of this key entirely (a legacy
    /// pre-Capacity-Lease peer) is itself the "false" signal on decode.
    pub lease_v1: bool,

    /// A block shard is loading right now, as opposed to this node having
    /// nothing to load. Only meaningful while `state` is JOINING; encoded only
    /// when true, so a node that is merely idle says nothing extra.
    pub shard_loading: bool,

    /// A signed RFC 0003 peer record naming the multiaddrs a peer can dial us
    /// on, protobuf-encoded as a `SignedEnvelope`. Empty for a node with no
    /// reachable address yet, and for every path that does not fill it in —
    /// see [`crate::node_native::NativeNode::announce`], which populates it.
    ///
    /// # Why the record has to carry this
    ///
    /// Discovery hands dispatch a bare PeerId, and dialing by PeerId resolves
    /// through kad — which cannot answer. rust-libp2p serves `FIND_NODE` from
    /// its k-buckets alone (`find_closest_local_peers`), where the kad-DHT spec
    /// §6.1.1 says a server MUST answer for a requested peer it holds in its
    /// *peerstore* "even if the target node isn't a DHT Server or only
    /// advertises private addresses", and where go-libp2p's `handleFindPeer`
    /// does exactly that. So a peer is findable only while it occupies one of
    /// the twenty slots in the right bucket on a peer the walk happens to
    /// reach, and a disconnected entry is the first one replaced. That is the
    /// half the Go p2pd bootstrap used to supply; on the native stack it made
    /// dispatch fail fleet-wide with `peer not found in DHT (no addresses)` —
    /// ~2000 of them in one node's log.
    ///
    /// Note this is *not* about kad's mode: `dht_server` is true on every
    /// native node (`node_native.rs`), so kad is pinned to `Mode::Server`
    /// whether the node is reachable or not.
    ///
    /// The relay path works fine once the address is known; what is missing is
    /// only the address. Nothing else carries it — a circuit address names the
    /// relay a node happens to hold a reservation on, reservations rotate
    /// (`kwaai_p2p::relay_manager`), and a dialer cannot guess which relay that
    /// is: dialing a peer through a bootstrap it has not reserved on returns
    /// `Relay has no reservation for destination`.
    ///
    /// # Why it is signed rather than a plain list
    ///
    /// `kwaai-hivemind-dht` implements no record validators, so any peer can
    /// `STORE` under another node's subkey with a later expiration. A plain
    /// address list would therefore be an instruction, from anyone, about
    /// where to send a victim's traffic: black-hole a node by pointing its
    /// addresses at a dead host, or aim dials at a third party. Impersonation
    /// is already caught at the Noise handshake, but neither of those needs
    /// the attacker to complete a handshake.
    ///
    /// The envelope closes *that*, and only that.
    /// [`kwaai_p2p::peer_record::verified_addrs`] is the reader:
    /// `PeerRecord::from_signed_envelope_interop` verifies the signature and
    /// binds the record's peer id to the signing key, and the caller's
    /// comparison against the announced peer ties it to the peer being dialed
    /// — so an address list the announced peer did not sign is one the reader
    /// drops. The interop format (`libp2p-peer-record`) is used rather than
    /// rust-libp2p's legacy domain because this record is a published wire
    /// format that a Go or JS reader may one day verify.
    ///
    /// What it does **not** close, because the signature covers the address
    /// list rather than the record: an attacker can still overwrite a peer's
    /// announcement with one that omits this field — dropping it back to the
    /// bare-PeerId dial that fails — tombstone it with `state = -1`, or replay
    /// an older genuine list of addresses the peer has since moved off. All
    /// three are plain overwrites, which is what a record validator in the DHT
    /// store would answer; none of them need a forged signature.
    pub signed_addrs: Vec<u8>,
}

impl DHTServerInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start: i32,
        end: i32,
        name: &str,
        relay: bool,
        throughput: f64,
        trust_attestations: Vec<String>,
        vpk_info: Option<VpkInfo>,
        peer_id_b58: String,
    ) -> Self {
        Self {
            state: KwaaiNetConfig::announce_state(),
            throughput,
            start_block: start,
            end_block: end,
            public_name: name.to_string(),
            version: concat!("kwaai-", env!("CARGO_PKG_VERSION")).to_string(),
            torch_dtype: "float16".to_string(),
            using_relay: relay,
            cache_tokens_left: 100_000,
            next_pings: HashMap::new(),
            adapters: vec![],
            trust_attestations,
            vpk_info,
            peer_id_b58,
            lease_v1: true,
            shard_loading: KwaaiNetConfig::announce_shard_loading(),
            // Filled in per-announce from the live swarm rather than here: the
            // set changes as reservations rotate, and a value captured at
            // startup would be wrong by the first re-announce.
            signed_addrs: Vec::new(),
        }
    }

    pub fn to_msgpack(&self) -> Result<Vec<u8>> {
        let mut fields: Vec<(rmpv::Value, rmpv::Value)> = vec![
            (
                rmpv::Value::from("start_block"),
                rmpv::Value::from(self.start_block),
            ),
            (
                rmpv::Value::from("end_block"),
                rmpv::Value::from(self.end_block),
            ),
            (
                rmpv::Value::from("public_name"),
                rmpv::Value::from(self.public_name.as_str()),
            ),
            (
                rmpv::Value::from("version"),
                rmpv::Value::from(self.version.as_str()),
            ),
            (
                rmpv::Value::from("torch_dtype"),
                rmpv::Value::from(self.torch_dtype.as_str()),
            ),
            (
                rmpv::Value::from("using_relay"),
                rmpv::Value::from(self.using_relay),
            ),
            (
                rmpv::Value::from("cache_tokens_left"),
                rmpv::Value::from(self.cache_tokens_left),
            ),
            (rmpv::Value::from("adapters"), rmpv::Value::Array(vec![])),
            (rmpv::Value::from("next_pings"), rmpv::Value::Map(vec![])),
            (
                rmpv::Value::from("peer_id"),
                rmpv::Value::from(self.peer_id_b58.as_str()),
            ),
            // Capacity Lease capability flag. Unconditional (unlike
            // trust_attestations/vpk below) since this binary's own
            // capability has no "empty" state to skip — legacy clients
            // ignore the unknown key exactly as they do those two.
            (
                rmpv::Value::from("lease_v1"),
                rmpv::Value::from(self.lease_v1),
            ),
        ];

        // Include trust attestations when present — zero-cost for nodes without VCs.
        // Legacy clients (Python Hivemind, old map viewers) ignore unknown fields.
        if !self.trust_attestations.is_empty() {
            let ta_values: Vec<rmpv::Value> = self
                .trust_attestations
                .iter()
                .map(|s| rmpv::Value::String(rmpv::Utf8String::from(s.as_str())))
                .collect();
            fields.push((
                rmpv::Value::from("trust_attestations"),
                rmpv::Value::Array(ta_values),
            ));
        }

        if self.shard_loading {
            fields.push((rmpv::Value::from("shard_loading"), rmpv::Value::from(true)));
        }

        // Dial addresses, omitted entirely when empty so a node with nothing
        // reachable to say adds no bytes. Same unknown-key tolerance as the
        // fields above: a legacy client ignores `addrs_signed` and keeps
        // dialing by PeerId exactly as it does today. Binary, not a string
        // list — the value is the signed envelope, and only what the node's
        // own key signed is worth publishing.
        if !self.signed_addrs.is_empty() {
            fields.push((
                rmpv::Value::from("addrs_signed"),
                rmpv::Value::Binary(self.signed_addrs.clone()),
            ));
        }

        // Include VPK capability when enabled and reachable.
        // Unknown map keys are silently ignored by legacy Hivemind clients
        // and old map viewers — no backward-compatibility risk.
        if let Some(ref vpk) = self.vpk_info {
            fields.push((rmpv::Value::from("vpk"), vpk.to_msgpack_value()));
        }

        let inner = rmpv::Value::Array(vec![
            rmpv::Value::from(self.state),
            rmpv::Value::from(self.throughput),
            rmpv::Value::Map(fields),
        ]);

        let mut inner_bytes = Vec::new();
        rmpv::encode::write_value(&mut inner_bytes, &inner)?;

        // Wrap in ExtType(64 = 0x40) — Python Hivemind tuple marker
        let ext = rmpv::Value::Ext(64, inner_bytes);
        let mut out = Vec::new();
        rmpv::encode::write_value(&mut out, &ext)?;
        Ok(out)
    }
}

// ── Reading one back ─────────────────────────────────────────────────────────
//
// The decoder sits beside `DHTServerInfo::to_msgpack` deliberately. The two
// halves are one published wire format, and every bug this code has had was a
// disagreement between them — a key spelled differently, a field the encoder
// gained and the reader never learned. Side by side they can be tested by
// round trip through the real producer rather than against a hand-built map,
// and a field added to one is visibly missing from the other.
//
// Bytes to fields is all that lives here. What a reader then *does* with those
// fields — which states count as usable, which versions are too old to trust,
// what struct they become — is the consumer's policy, and putting it here
// would make the wire format depend on the sharding layer that reads it. So
// `decode_server_info_regular` / `_dictionary` sit in `shard_cmd.rs` instead,
// on top of `decode_server_info_ext`.

/// One decoded announcement, as a struct rather than the positional tuple this
/// used to return: nine fields where three call sites want different subsets is
/// exactly the shape where `_`-holes drift out of alignment with the producer.
pub struct ServerInfoFields {
    pub state: i32,
    pub start_block: usize,
    pub end_block: usize,
    pub public_name: String,
    pub peer_id_b58: String,
    pub version: String,
    pub throughput: f64,
    pub lease_v1: bool,
    /// Verified multiaddrs the peer can be dialed on, **bare** — the
    /// destination `/p2p/<peer>` stripped and a circuit's relay hop kept,
    /// exactly as [`kwaai_p2p::peer_record::verified_addrs`] returns them.
    /// Empty for a peer that published none, and for one whose record did not
    /// verify.
    pub dial_addrs: Vec<Multiaddr>,
}

pub fn decode_server_info_ext(bytes: &[u8]) -> Option<ServerInfoFields> {
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
    let throughput = arr[1].as_f64().unwrap_or(0.0);
    let start_block = get_i("start_block")? as usize;
    let end_block = get_i("end_block")? as usize;
    let public_name = get_s("public_name");
    let peer_id_b58 = get_s("peer_id");
    let version = get_s("version");
    // Absent key (a peer built before Capacity Lease existed) defaults to
    // false — the exact "legacy peer" signal a requester falls back on.
    let lease_v1 = map
        .iter()
        .find(|(ky, _)| ky.as_str() == Some("lease_v1"))
        .and_then(|(_, v)| v.as_bool())
        .unwrap_or(false);

    // The record is only worth reading against the peer it claims to be, and
    // that binding is the whole of the check — see `verified_addrs`. A record
    // whose `peer_id` does not parse has nothing to bind to, so it yields no
    // addresses rather than unverified ones.
    let dial_addrs = map
        .iter()
        .find(|(ky, _)| ky.as_str() == Some("addrs_signed"))
        .and_then(|(_, v)| v.as_slice())
        .zip(peer_id_b58.parse::<PeerId>().ok())
        .map(|(envelope, claimed)| kwaai_p2p::peer_record::verified_addrs(envelope, claimed))
        .unwrap_or_default();

    Some(ServerInfoFields {
        state,
        start_block,
        end_block,
        public_name,
        peer_id_b58,
        version,
        throughput,
        lease_v1,
        dial_addrs,
    })
}

/// How many dial addresses one announcement may carry.
///
/// The same value is stored under every block key the node serves — 32 keys
/// for an 8B model — so an address costs its own length times the block count
/// in DHT bytes. Four is enough for the shapes that occur: a direct address
/// plus the circuits from the two or three reservations `relay_manager` holds
/// at once, or one address per transport on a public node.
pub const MAX_DIAL_ADDRS: usize = 4;

/// A signed peer record naming the addresses other peers can dial this node
/// on — most useful first, at most [`MAX_DIAL_ADDRS`] of them — protobuf
/// encoded and ready to publish. Empty when the node has no address worth
/// announcing, or when signing fails.
///
/// Sourced from the swarm's live listeners, which is where a relay reservation
/// shows up: `libp2p-relay` turns an accepted reservation into a listen address
/// of the form `<relay-addr>/p2p/<relay>/p2p-circuit/p2p/<us>`, already carrying
/// both hops a dialer needs. Three filters apply:
///
/// - [`is_announceable`](kwaai_p2p::is_announceable) drops the loopback and
///   RFC1918 listeners that every node has and no remote peer can use, while
///   passing circuits unconditionally (the relay's address is the routable
///   half).
/// - [`uses_dialable_transport`](kwaai_p2p::uses_dialable_transport) drops
///   webtransport/websocket forms this build has no transport for.
/// - One address per relay. A relay that offers TCP *and* QUIC produces a
///   circuit on each, and both traverse the same hop: keeping the second
///   spends a scarce slot on a path that fails with the first.
///
/// Direct addresses sort ahead of circuits so a dialer that can reach one
/// never pays for a relay, and a `declared` announce address — an operator
/// saying "I forwarded this port" — sorts ahead of everything, since it is the
/// one address the swarm cannot observe for itself. Circuits nonetheless
/// claim their slots first: a NATed host with several global v6 addresses or a
/// VPN interface would otherwise fill the record with direct addresses that
/// only work from its own network and lose the one address that works from
/// anywhere.
///
/// Listeners are read alongside the swarm's confirmed external addresses. A
/// UPnP mapping is not a listener, and a node that got one has released its
/// relay reservations — without it that node would publish nothing at all.
///
/// The result is signed with the node's own identity key, the same key the
/// peer id is derived from — see [`DHTServerInfo::signed_addrs`] for why an
/// unsigned list would be a standing invitation to redirect a node's traffic.
pub async fn signed_dial_addrs(
    handle: &NetworkHandle,
    keypair: &libp2p::identity::Keypair,
    declared: Option<&str>,
) -> Vec<u8> {
    let external = handle.external_addrs().await.unwrap_or_default();
    let listeners = handle.listen_addrs().await.unwrap_or_default();
    let addrs = select_dial_addrs(external.into_iter().chain(listeners).collect(), declared);
    if addrs.is_empty() {
        return Vec::new();
    }
    match libp2p::core::PeerRecord::new_interop(keypair, addrs) {
        Ok(record) => record.into_signed_envelope().into_protobuf_encoding(),
        Err(e) => {
            // Signing failing means the identity key is unusable, which the
            // swarm would already have died on. Publish nothing rather than
            // an unsigned list a reader would be right to refuse.
            warn!("could not sign the dial-address record: {e}");
            Vec::new()
        }
    }
}

/// The selection itself, split from the swarm call and the signing so it can
/// be tested against a fixed address list rather than a live node.
///
/// Addresses come back **without** a trailing `/p2p/<us>`: the peer record
/// names the peer once, in a field the signature covers, and a reader
/// re-attaches it when it dials. `strip_dest_p2p` is what does that, and it
/// keeps a circuit's relay hop — the half a dialer cannot reconstruct.
fn select_dial_addrs(
    listeners: Vec<libp2p::Multiaddr>,
    declared: Option<&str>,
) -> Vec<libp2p::Multiaddr> {
    use kwaai_p2p::addresses::{peer_id_from_multiaddr, strip_dest_p2p};
    use kwaai_p2p::{is_announceable, is_circuit, uses_dialable_transport};

    let mut sorted: Vec<libp2p::Multiaddr> = declared
        .and_then(|d| d.parse().ok())
        .into_iter()
        .chain(listeners)
        .filter(|a| is_announceable(a) && uses_dialable_transport(a))
        .collect();
    // Direct before circuit (`false` sorts before `true`), stably, so the
    // declared address stays ahead of the listeners it was chained onto.
    sorted.sort_by_key(is_circuit);

    let mut direct: Vec<libp2p::Multiaddr> = Vec::new();
    let mut circuits: Vec<libp2p::Multiaddr> = Vec::new();
    let mut seen_hops: HashSet<String> = HashSet::new();
    for addr in sorted {
        // A circuit is keyed on the relay itself, so its TCP and QUIC forms
        // collapse to one entry — both cross the same hop, and if that hop is
        // down neither works. A direct address is keyed on the address, where
        // TCP and QUIC are genuinely separate chances.
        let key = match (is_circuit(&addr), peer_id_from_multiaddr(&addr)) {
            (true, Some(relay)) => relay.to_base58(),
            _ => strip_dest_p2p(&addr).to_string(),
        };
        if !seen_hops.insert(key) {
            continue;
        }
        if is_circuit(&addr) {
            circuits.push(strip_dest_p2p(&addr));
        } else {
            direct.push(strip_dest_p2p(&addr));
        }
    }
    // Circuits take their slots first; direct addresses get what is left and
    // still lead the output. See the function docs for why.
    circuits.truncate(MAX_DIAL_ADDRS);
    direct.truncate(MAX_DIAL_ADDRS - circuits.len());
    direct.extend(circuits);
    direct
}

/// Model info stored in the `_petals.models` DHT registry.
pub struct ModelInfo {
    pub num_blocks: i32,
    pub repository: String,
}

impl ModelInfo {
    pub fn to_msgpack(&self) -> Result<Vec<u8>> {
        let map = vec![
            (
                rmpv::Value::from("repository"),
                rmpv::Value::from(self.repository.as_str()),
            ),
            (
                rmpv::Value::from("num_blocks"),
                rmpv::Value::from(self.num_blocks),
            ),
        ];
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &rmpv::Value::Map(map))?;
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// DHT key helpers
// ---------------------------------------------------------------------------

/// SHA1(msgpack(raw_key)) — Hivemind's DHTID.generate() equivalent.
pub fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

// ---------------------------------------------------------------------------
// Record builders
// ---------------------------------------------------------------------------

/// What this node is publishing, minus the values that are derived from
/// [`DHTServerInfo`] itself.
pub struct AnnounceContext<'a> {
    /// This node's peer ID; the base58 form becomes every record's subkey.
    pub peer_id: PeerId,
    /// The model's DHT prefix, e.g. `Qwen/Qwen3-8B-hf`.
    pub prefix: &'a str,
    /// HuggingFace repository the prefix refers to.
    pub repository: &'a str,
    /// Total blocks in the model, for the `_petals.models` registry.
    pub total_blocks: i32,
}

/// Build every STORE request a single announcement round sends, in order:
/// blocks, `_petals.models`, `_kwaai.inference.nodes`, then `_kwaai.vpk.nodes`
/// when VPK is enabled.
///
/// The block range comes from `server_info.start_block..server_info.end_block`.
///
/// All expirations are `now + `[`ANNOUNCE_TTL_SECS`], computed once per call so
/// every record in one round shares a timestamp.
pub fn build_announce_records(
    ctx: &AnnounceContext<'_>,
    server_info: &DHTServerInfo,
) -> Result<Vec<StoreRequest>> {
    let info_bytes = server_info.to_msgpack()?;
    let subkey = rmp_serde::to_vec(&ctx.peer_id.to_base58())?;
    let node_info = NodeInfo::from_peer_id(ctx.peer_id);
    let expiration = get_dht_time() + ANNOUNCE_TTL_SECS;

    let mut records = Vec::with_capacity(4);

    // 1. Per-block records — one key per served block, all sharing one request.
    let mut keys = Vec::new();
    let mut subkeys = Vec::new();
    let mut values = Vec::new();
    let mut expirations = Vec::new();
    let mut in_cache = Vec::new();
    for block in server_info.start_block..server_info.end_block {
        keys.push(dht_id(&format!("{}.{}", ctx.prefix, block)));
        subkeys.push(subkey.clone());
        values.push(info_bytes.clone());
        expirations.push(expiration);
        in_cache.push(false);
    }
    records.push(StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys,
        subkeys,
        values,
        expiration_time: expirations,
        in_cache,
        peer: Some(node_info.clone()),
    });

    // 2. Model registry — subkeyed by *prefix*, not by peer, so one entry per
    //    model rather than one per server.
    let model_info = ModelInfo {
        num_blocks: ctx.total_blocks,
        repository: ctx.repository.to_string(),
    };
    records.push(StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id(PETALS_MODELS_KEY)],
        subkeys: vec![rmp_serde::to_vec(&ctx.prefix)?],
        values: vec![model_info.to_msgpack()?],
        expiration_time: vec![expiration],
        in_cache: vec![false],
        peer: Some(node_info.clone()),
    });

    // 3. Inference-node registry — the same ServerInfo value under a
    //    block-independent key, so `p2p://auto` can find us without knowing
    //    block coverage.
    records.push(StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id(crate::shard_cmd::INFERENCE_NODES_DHT_KEY)],
        subkeys: vec![subkey.clone()],
        values: vec![info_bytes.clone()],
        expiration_time: vec![expiration],
        in_cache: vec![false],
        peer: Some(node_info.clone()),
    });

    // 4. VPK registry — only for nodes that actually serve VPK.
    if let Some(ref vpk) = server_info.vpk_info {
        records.push(StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![dht_id(VPK_NODES_KEY)],
            subkeys: vec![subkey],
            values: vec![vpk.to_msgpack_bytes()?],
            expiration_time: vec![expiration],
            in_cache: vec![false],
            peer: Some(node_info),
        });
    }

    Ok(records)
}

/// Build the STORE requests that take this node off the map on clean shutdown.
///
/// Only the block records and the VPK record are rewritten: `_petals.models`
/// describes the model, not this server, and `_kwaai.inference.nodes` is left
/// to age out.
///
/// # Why the expiration is in the *future*
///
/// This is the detail implementers get backwards. A tombstone is **not** written
/// with an expired or shortened timestamp: hivemind's storage rejects a store
/// whose expiration is not strictly greater than the record it replaces
/// (`stale_and_equal_expirations_are_rejected` in `kwaai-hivemind-dht`), and it
/// rejects already-expired stores outright. Either would leave the *live*
/// record in place and the node would keep showing as ready until its TTL ran
/// out. So the tombstone carries a normal `now + `[`ANNOUNCE_TTL_SECS`]
/// expiration and does its work through `state = -1` (offline), which the map
/// treats as "remove immediately".
pub fn build_unannounce_records(
    ctx: &AnnounceContext<'_>,
    server_info: &DHTServerInfo,
) -> Result<Vec<StoreRequest>> {
    let offline_info = DHTServerInfo {
        state: -1, // OFFLINE — tells map.kwaai.ai to remove the node immediately
        throughput: 0.0,
        start_block: server_info.start_block,
        end_block: server_info.end_block,
        public_name: server_info.public_name.clone(),
        version: server_info.version.clone(),
        torch_dtype: server_info.torch_dtype.clone(),
        using_relay: server_info.using_relay,
        cache_tokens_left: 0,
        next_pings: HashMap::new(),
        adapters: vec![],
        trust_attestations: vec![],
        vpk_info: None,
        peer_id_b58: server_info.peer_id_b58.clone(),
        lease_v1: server_info.lease_v1,
        shard_loading: false,
        // A tombstone says "stop using me"; where to reach the node is
        // exactly the thing it is withdrawing.
        signed_addrs: Vec::new(),
    };

    let info_bytes = offline_info.to_msgpack()?;
    let subkey = rmp_serde::to_vec(&ctx.peer_id.to_base58())?;
    let node_info = NodeInfo::from_peer_id(ctx.peer_id);
    let expiration = get_dht_time() + ANNOUNCE_TTL_SECS;

    let mut records = Vec::with_capacity(2);

    let mut keys = Vec::new();
    let mut subkeys = Vec::new();
    let mut values = Vec::new();
    let mut expirations = Vec::new();
    let mut in_cache = Vec::new();
    for block in server_info.start_block..server_info.end_block {
        keys.push(dht_id(&format!("{}.{}", ctx.prefix, block)));
        subkeys.push(subkey.clone());
        values.push(info_bytes.clone());
        expirations.push(expiration);
        in_cache.push(false);
    }
    records.push(StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys,
        subkeys,
        values,
        expiration_time: expirations,
        in_cache,
        peer: Some(node_info.clone()),
    });

    // The VPK record is withdrawn with the *live* VPK value, not the offline
    // ServerInfo — it has no state field to flip, so the only signal available
    // is that it stops being refreshed.
    if let Some(ref vpk) = server_info.vpk_info {
        records.push(StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![dht_id(VPK_NODES_KEY)],
            subkeys: vec![subkey],
            values: vec![vpk.to_msgpack_bytes()?],
            expiration_time: vec![expiration],
            in_cache: vec![false],
            peer: Some(node_info),
        });
    }

    Ok(records)
}

// ---------------------------------------------------------------------------
// Native delivery
// ---------------------------------------------------------------------------

/// Per-bootstrap timing of one STORE round:
/// `(peer_id_base58, multiaddr, latency_ms, success)`.
///
/// Fed to `reputation::ReputationStore` — the announce path doubles as the
/// reputation probe, so no extra RPCs are needed.
pub type StoreTiming = (String, String, f64, bool);

/// Push `records` to every bootstrap peer over a native [`NetworkHandle`].
///
/// Returns `(any_success, timings)` where success means **at least one** record
/// in **at least one** request was accepted by **at least one** bootstrap
/// (`store_ok` containing a `true`): a partially reachable bootstrap set is
/// normal.
///
/// The per-call timeout is `NetworkConfig::request_timeout`, applied by
/// [`NetworkHandle::call_unary_handler`] and surfacing as
/// [`kwaai_p2p::P2PError::Timeout`]. The handle dials the bootstrap if not
/// already connected, matching Go's `host.NewStream`.
pub async fn send_records_via_handle(
    handle: &NetworkHandle,
    bootstrap_peers: &[String],
    records: &[StoreRequest],
) -> (bool, Vec<StoreTiming>) {
    if bootstrap_peers.is_empty() || records.is_empty() {
        return (false, vec![]);
    }

    let mut succeeded = 0usize;
    let mut timings: Vec<StoreTiming> = Vec::with_capacity(bootstrap_peers.len());

    for addr in bootstrap_peers {
        let Some(peer_id_str) = addr.split("/p2p/").nth(1) else {
            warn!("Bootstrap peer has no /p2p/ component: {}", addr);
            continue;
        };
        let bp = match peer_id_str.parse::<PeerId>() {
            Ok(p) => p,
            Err(e) => {
                warn!("Invalid peer ID in {}: {}", addr, e);
                continue;
            }
        };

        // One timing per bootstrap, covering the whole round, so the reputation
        // signal is "how long did announcing to this peer take" rather than a
        // per-record sample that would over-weight nodes serving many blocks.
        let t0 = std::time::Instant::now();
        let mut peer_ok = false;

        for req in records {
            let mut bytes = Vec::new();
            if let Err(e) = req.encode(&mut bytes) {
                warn!("Encode STORE request failed: {}", e);
                continue;
            }

            match handle
                .call_unary_handler(bp, kwaai_hivemind_dht::PROTOCOL_STORE, &bytes)
                .await
            {
                Ok(resp_bytes) => match StoreResponse::decode(&resp_bytes[..]) {
                    Ok(resp) => {
                        let ok = resp.store_ok.iter().filter(|&&s| s).count();
                        info!(
                            "STORE response from {}: {}/{} stored",
                            peer_id_str,
                            ok,
                            resp.store_ok.len()
                        );
                        peer_ok |= ok > 0;
                    }
                    Err(e) => warn!("STORE response from {} was undecodable: {}", addr, e),
                },
                Err(e) => warn!("STORE RPC failed ({}): {}", addr, e),
            }
        }

        let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;
        timings.push((peer_id_str.to_string(), addr.clone(), latency_ms, peer_ok));
        if peer_ok {
            succeeded += 1;
        }
    }

    if succeeded > 0 {
        info!(
            "✅ Announced to {} of {} bootstrap peers",
            succeeded,
            bootstrap_peers.len()
        );
    } else {
        warn!(
            "❌ Announcement failed on all {} bootstrap peers — see warnings above",
            bootstrap_peers.len()
        );
    }

    (succeeded > 0, timings)
}

// ---------------------------------------------------------------------------
// Decentralized delivery
// ---------------------------------------------------------------------------

/// Every peer this node could place a record on, nearest-ranking aside.
///
/// Three sources, merged in preference order:
///
/// 1. **Known peers** — [`NetworkHandle::known_peers`], i.e. the kad routing
///    table merged with live connections, address-filtered. This is the bulk of
///    the set on an established node and the reason the configured list stops
///    mattering.
/// 2. **Configured peers** — `initial_peers` / `bootstrap_peers`, now ordinary
///    candidates with no privileged role. They still matter on a cold start,
///    when the routing table is empty.
/// 3. **Cached peers** — the peer cache, so a node whose configured peers are
///    all gone still has somewhere to publish.
///
/// Earlier sources win per peer, so a live address beats a stale configured
/// one for the same peer ID. Our own peer ID is never a candidate.
pub async fn gather_candidates(
    handle: &NetworkHandle,
    configured: &[String],
    cached: &[String],
) -> Vec<crate::placement::Candidate> {
    use crate::placement::{candidates_from_addrs, candidates_from_known, merge_candidates};

    let us = handle.peer_id();
    let known = match handle.known_peers().await {
        Ok(k) => candidates_from_known(&k, &us),
        Err(e) => {
            warn!("Could not enumerate known peers ({e}) — falling back to configured peers");
            vec![]
        }
    };

    merge_candidates(&[
        known,
        candidates_from_addrs(configured, &us),
        candidates_from_addrs(cached, &us),
    ])
}

/// Split a [`StoreRequest`] into one request per key.
///
/// The block record packs every served block into a single request with
/// index-aligned parallel arrays. That is exactly right for the bootstrap
/// fan-out — one RPC carries the lot — but wrong for placement, where each key
/// has its own position in the keyspace and therefore its own set of holders.
/// This is the seam between the two: one request in, N single-key requests out,
/// each still carrying its own subkey, value, expiration and `in_cache` flag.
///
/// A malformed request whose arrays are not index-aligned yields only the
/// entries present in all of them, rather than panicking on the short one.
fn split_by_key(request: &StoreRequest) -> Vec<StoreRequest> {
    (0..request.keys.len())
        .filter_map(|i| {
            Some(StoreRequest {
                auth: Some(RequestAuthInfo::new()),
                keys: vec![request.keys.get(i)?.clone()],
                subkeys: vec![request.subkeys.get(i)?.clone()],
                values: vec![request.values.get(i)?.clone()],
                expiration_time: vec![*request.expiration_time.get(i)?],
                in_cache: vec![*request.in_cache.get(i)?],
                peer: request.peer.clone(),
            })
        })
        .collect()
}

/// Push `records` to the peers nearest each record's own key.
///
/// The decentralized counterpart to [`send_records_via_handle`]. Where that
/// stores every record on every configured bootstrap, this asks — per key —
/// *"who in the network should hold this"*, and stores on the `k` nearest
/// candidates that accept it.
///
/// # Per-key, not per-round
///
/// Records are split by key first ([`split_by_key`]), so a node serving blocks
/// 0–8 runs eight independent placements plus one each for `_petals.models`
/// and `_kwaai.inference.nodes`. Those land on **different peer sets**, which
/// is the entire point: no single peer is required to hold everything, and a
/// reader looking for one block walks to the peers holding that block rather
/// than to a bootstrap that happened to be told about all of them.
///
/// # Timings
///
/// [`StoreTiming`] is still reported per peer, aggregated across every key that
/// peer was asked to hold: one observation per peer per round, so the
/// reputation store sees the same shape it does on the bootstrap path rather
/// than one sample per block. A peer counts as successful if any of its stores
/// in the round succeeded.
///
/// Returns `(any_success, timings)` on the same rule as the bootstrap path:
/// success means at least one record reached at least one peer.
pub async fn send_records_decentralized(
    handle: &NetworkHandle,
    candidates: &[crate::placement::Candidate],
    records: &[StoreRequest],
    replication: usize,
) -> (bool, Vec<StoreTiming>) {
    use crate::placement::{place_with, rank_candidates};

    if records.is_empty() {
        return (false, vec![]);
    }
    if candidates.is_empty() {
        warn!("Decentralized announce: no candidate peers with a dialable address — nothing published");
        return (false, vec![]);
    }

    // Per-peer aggregation across every key this round places.
    //
    // A `RefCell` rather than a plain `&mut`: `place_with` takes an `FnMut`
    // whose futures may not hold a borrow across calls, but the walk is
    // strictly sequential (each store is awaited before the next begins), so
    // the borrow is only ever held inside one future at a time and cannot
    // conflict.
    let latency_by_peer: std::cell::RefCell<HashMap<PeerId, (String, f64, bool)>> =
        std::cell::RefCell::new(HashMap::new());
    let mut any_success = false;
    let mut total_shortfall = 0usize;
    let mut placed_keys = 0usize;

    let single_key_records: Vec<StoreRequest> = records.iter().flat_map(split_by_key).collect();

    for record in &single_key_records {
        let Some(record_id) = record.keys.first() else {
            continue;
        };
        let mut bytes = Vec::new();
        if let Err(e) = record.encode(&mut bytes) {
            warn!("Encode STORE request failed: {}", e);
            continue;
        }

        let ranked = rank_candidates(candidates, record_id);
        let outcome = place_with(&ranked, replication, |candidate| {
            let bytes = bytes.clone();
            let acc = &latency_by_peer;
            async move {
                let t0 = std::time::Instant::now();
                let ok = match handle
                    .call_unary_handler(
                        candidate.peer_id,
                        kwaai_hivemind_dht::PROTOCOL_STORE,
                        &bytes,
                    )
                    .await
                {
                    Ok(resp) => match StoreResponse::decode(&resp[..]) {
                        Ok(resp) => resp.store_ok.iter().any(|&s| s),
                        Err(e) => {
                            warn!(
                                "STORE response from {} was undecodable: {}",
                                candidate.peer_id, e
                            );
                            false
                        }
                    },
                    Err(e) => {
                        warn!("STORE RPC to {} failed: {}", candidate.peer_id, e);
                        false
                    }
                };

                // One observation per peer per round: latencies across the keys
                // this peer held are summed and successes OR-ed, so a peer
                // serving eight blocks is not weighted eight times.
                let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;
                {
                    let mut acc = acc.borrow_mut();
                    let entry = acc
                        .entry(candidate.peer_id)
                        .or_insert_with(|| (candidate.addr.clone(), 0.0, false));
                    entry.1 += latency_ms;
                    entry.2 |= ok;
                }

                ok
            }
        })
        .await;

        placed_keys += 1;
        total_shortfall += outcome.shortfall;
        any_success |= outcome.any_success();
    }

    let timings: Vec<StoreTiming> = latency_by_peer
        .into_inner()
        .into_iter()
        .map(|(peer, (addr, latency_ms, ok))| (peer.to_base58(), addr, latency_ms, ok))
        .collect();

    if total_shortfall > 0 {
        warn!(
            "Decentralized announce: {} replica slot(s) short of {} across {} key(s) — \
             the known-peer set may be too small or partly unreachable",
            total_shortfall, replication, placed_keys
        );
    }
    if any_success {
        info!(
            "✅ Announced {} key(s) to their nearest peers (k={})",
            placed_keys, replication
        );
    } else {
        warn!("❌ Decentralized announcement reached no peer — see warnings above");
    }

    (any_success, timings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kwaai_hivemind_dht::{IS_DICTIONARY, IS_REGULAR_VALUE};

    fn peer() -> PeerId {
        "12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN"
            .parse()
            .expect("a valid peer id")
    }

    fn server_info(vpk: Option<VpkInfo>) -> DHTServerInfo {
        DHTServerInfo {
            state: 2,
            throughput: 12.5,
            start_block: 0,
            end_block: 3,
            public_name: "test-node".to_string(),
            version: "kwaai-test".to_string(),
            torch_dtype: "float16".to_string(),
            using_relay: false,
            cache_tokens_left: 100_000,
            next_pings: HashMap::new(),
            adapters: vec![],
            trust_attestations: vec![],
            vpk_info: vpk,
            peer_id_b58: peer().to_base58(),
            lease_v1: true,
            shard_loading: false,
            signed_addrs: vec![],
        }
    }

    /// A relay's peer id, distinct from [`peer`] — the node being announced.
    fn relay() -> PeerId {
        "12D3KooWF7ckKo2HQojbtueQNuLYRT2XC2yzbvBbh4NK2rbi2Azg"
            .parse()
            .expect("a valid peer id")
    }

    fn addrs(list: &[String]) -> Vec<libp2p::Multiaddr> {
        list.iter()
            .map(|a| a.parse().expect("a valid addr"))
            .collect()
    }

    /// The listener set a NATed node actually has: loopback, two LAN
    /// interfaces, and the circuit its reservation produced.
    #[test]
    fn select_dial_addrs_keeps_only_the_circuit_for_a_natted_node() {
        let circuit = format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        );
        let listeners = addrs(&[
            "/ip4/127.0.0.1/tcp/54428".to_string(),
            "/ip4/192.168.68.135/tcp/54428".to_string(),
            "/ip6/::1/udp/54428/quic-v1".to_string(),
            circuit,
        ]);

        // The destination hop is stripped — the record names the peer itself —
        // but the relay hop, which a dialer cannot reconstruct, is kept.
        let expected = format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit",
            relay().to_base58()
        );
        assert_eq!(select_dial_addrs(listeners, None), addrs(&[expected]));
    }

    /// Both circuits cross the same relay, so publishing both spends two of
    /// four slots on one hop.
    #[test]
    fn select_dial_addrs_collapses_two_transports_on_one_relay() {
        let tcp = format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        );
        let quic = format!(
            "/ip4/76.13.5.74/udp/4001/quic-v1/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        );

        let out = select_dial_addrs(addrs(&[tcp, quic]), None);
        let expected = format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit",
            relay().to_base58()
        );
        assert_eq!(out, addrs(&[expected]), "one address per relay, first wins");
    }

    /// This build has no webtransport transport, and the certhash form is the
    /// longest address a relay offers — publishing it is pure cost.
    #[test]
    fn select_dial_addrs_drops_transports_this_build_cannot_dial() {
        let wt = format!(
            "/ip4/76.13.5.74/udp/4001/quic-v1/webtransport/certhash/uEiBIeyYi7BYMq_u71nPi3WJna-9kL5yAURJ5HYy0qXW3YQ/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        );
        assert!(select_dial_addrs(addrs(&[wt]), None).is_empty());
    }

    /// The peer id belongs in the record, under the signature, not repeated on
    /// every address — a reader re-attaches it when it dials.
    #[test]
    fn select_dial_addrs_leaves_the_peer_id_to_the_record() {
        let out = select_dial_addrs(addrs(&["/ip4/198.18.0.40/tcp/8080".to_string()]), None);
        assert_eq!(out, addrs(&["/ip4/198.18.0.40/tcp/8080".to_string()]));
    }

    /// A declared address is the operator saying "I forwarded this port" — the
    /// one address the swarm cannot observe for itself, and the cheapest hop
    /// for a dialer, so it leads.
    #[test]
    fn select_dial_addrs_puts_a_declared_address_first() {
        let circuit = format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        );
        let out = select_dial_addrs(addrs(&[circuit]), Some("/ip4/203.0.113.7/tcp/4001"));
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].to_string(), "/ip4/203.0.113.7/tcp/4001");
    }

    /// A host with many interfaces — several global v6 addresses, a VPN —
    /// must not crowd its circuit out of the record: for a NATed node the
    /// circuit is the only entry that works from outside its own network.
    #[test]
    fn select_dial_addrs_reserves_room_for_a_circuit() {
        let mut listeners: Vec<String> = (1..=MAX_DIAL_ADDRS + 2)
            .map(|i| format!("/ip6/2001:db8::{i}/tcp/4001"))
            .collect();
        listeners.push(format!(
            "/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit/p2p/{}",
            relay().to_base58(),
            peer().to_base58()
        ));

        let out = select_dial_addrs(addrs(&listeners), None);
        assert_eq!(out.len(), MAX_DIAL_ADDRS);
        assert!(
            kwaai_p2p::is_circuit(out.last().expect("non-empty")),
            "the circuit survives, last because direct is cheaper to try"
        );
        assert!(!kwaai_p2p::is_circuit(&out[0]));
    }

    /// The same value is stored under every block key, so the list is capped.
    #[test]
    fn select_dial_addrs_stops_at_the_cap() {
        let listeners: Vec<String> = (1..=MAX_DIAL_ADDRS + 3)
            .map(|i| format!("/ip4/203.0.113.{i}/tcp/4001"))
            .collect();
        assert_eq!(
            select_dial_addrs(addrs(&listeners), None).len(),
            MAX_DIAL_ADDRS
        );
    }

    /// A node with nothing reachable to say must add no bytes to the record,
    /// so that `addrs` is absent rather than an empty array.
    #[test]
    fn to_msgpack_omits_addrs_when_there_are_none() {
        let bytes = server_info(None).to_msgpack().expect("encodes");
        let text = String::from_utf8_lossy(&bytes);
        assert!(!text.contains("addrs"));
    }

    #[test]
    fn to_msgpack_carries_addrs_when_present() {
        let mut info = server_info(None);
        info.signed_addrs = vec![1, 2, 3];
        let bytes = info.to_msgpack().expect("encodes");
        assert!(String::from_utf8_lossy(&bytes).contains("addrs_signed"));
    }

    /// The whole point of the field: what is published is signed by the key
    /// the peer id is derived from, so a reader can bind the two.
    #[test]
    fn a_signed_record_verifies_and_names_the_signer() {
        let key = libp2p::identity::Keypair::generate_ed25519();
        let me = key.public().to_peer_id();
        let addr: libp2p::Multiaddr = "/ip4/203.0.113.7/tcp/4001".parse().expect("parses");

        let bytes = libp2p::core::PeerRecord::new_interop(&key, vec![addr.clone()])
            .expect("signs")
            .into_signed_envelope()
            .into_protobuf_encoding();

        let envelope =
            libp2p::core::SignedEnvelope::from_protobuf_encoding(&bytes).expect("decodes");
        let record =
            libp2p::core::PeerRecord::from_signed_envelope_interop(envelope).expect("verifies");
        assert_eq!(record.peer_id(), me);
        assert_eq!(record.addresses(), &[addr]);
    }

    fn vpk_info() -> VpkInfo {
        VpkInfo {
            mode: "both".to_string(),
            capacity_gb: 42.0,
            tenant_count: 3,
            vpk_version: "1.2.3".to_string(),
            public_name: "test-node".to_string(),
        }
    }

    fn ctx(p: PeerId) -> AnnounceContext<'static> {
        AnnounceContext {
            peer_id: p,
            prefix: "Qwen/Qwen3-8B-hf",
            repository: "Qwen/Qwen3-8B",
            total_blocks: 36,
        }
    }

    // ── Key derivation ──────────────────────────────────────────────────────

    /// `dht_id` is SHA1 over the *msgpack* encoding of the key, not the raw
    /// UTF-8 — the map crawler and every Python peer derive it the same way, so
    /// this constant is a wire fact.
    #[test]
    fn dht_id_is_sha1_of_the_msgpack_encoding() {
        let key = "_petals.models";
        let packed = rmp_serde::to_vec(key).unwrap();
        let expected: Vec<u8> = Sha1::new().chain_update(&packed).finalize().to_vec();

        assert_eq!(dht_id(key), expected);
        assert_eq!(dht_id(key).len(), 20, "hivemind DHTIDs are 20 bytes");
        assert_ne!(
            dht_id(key),
            Sha1::new().chain_update(key.as_bytes()).finalize().to_vec(),
            "hashing the raw string would silently address a different key"
        );
    }

    // ── Record shape ────────────────────────────────────────────────────────

    /// The four record kinds, in order, with the right keys.
    #[test]
    fn announce_builds_the_four_record_kinds() {
        let p = peer();
        let records = build_announce_records(&ctx(p), &server_info(Some(vpk_info()))).unwrap();
        assert_eq!(records.len(), 4);

        // Blocks: one key per block in [start, end).
        assert_eq!(records[0].keys.len(), 3);
        for (i, key) in records[0].keys.iter().enumerate() {
            assert_eq!(key, &dht_id(&format!("Qwen/Qwen3-8B-hf.{i}")));
        }

        assert_eq!(records[1].keys, vec![dht_id(PETALS_MODELS_KEY)]);
        assert_eq!(
            records[2].keys,
            vec![dht_id(crate::shard_cmd::INFERENCE_NODES_DHT_KEY)]
        );
        assert_eq!(records[3].keys, vec![dht_id(VPK_NODES_KEY)]);
    }

    /// A node without VPK writes three records, not four.
    #[test]
    fn vpk_record_is_omitted_when_vpk_is_disabled() {
        let p = peer();
        let records = build_announce_records(&ctx(p), &server_info(None)).unwrap();
        assert_eq!(records.len(), 3);
        assert!(records
            .iter()
            .all(|r| r.keys != vec![dht_id(VPK_NODES_KEY)]));
    }

    /// Every record must use a **real** subkey. Either sentinel would change the
    /// storage semantics: `IS_REGULAR_VALUE` overwrites the whole key (one
    /// server per block instead of many) and `IS_DICTIONARY` would be parsed as
    /// a serialized dictionary and rejected.
    #[test]
    fn every_record_uses_a_real_subkey_never_a_sentinel() {
        let p = peer();
        let expected_peer_subkey = rmp_serde::to_vec(&p.to_base58()).unwrap();
        let records = build_announce_records(&ctx(p), &server_info(Some(vpk_info()))).unwrap();

        for (i, record) in records.iter().enumerate() {
            assert_eq!(
                record.subkeys.len(),
                record.keys.len(),
                "record {i}: subkeys must be index-aligned with keys"
            );
            for subkey in &record.subkeys {
                assert_ne!(subkey.as_slice(), IS_REGULAR_VALUE, "record {i}");
                assert_ne!(subkey.as_slice(), IS_DICTIONARY, "record {i}");
                assert!(!subkey.is_empty(), "record {i}");
            }
        }

        // Peer-keyed records carry msgpack(peer_b58)…
        assert!(records[0]
            .subkeys
            .iter()
            .all(|s| s == &expected_peer_subkey));
        assert_eq!(records[2].subkeys, vec![expected_peer_subkey.clone()]);
        assert_eq!(records[3].subkeys, vec![expected_peer_subkey]);
        // …but the model registry is keyed by prefix, one entry per model.
        assert_eq!(
            records[1].subkeys,
            vec![rmp_serde::to_vec(&"Qwen/Qwen3-8B-hf").unwrap()]
        );
    }

    /// Every parallel array in a `StoreRequest` must be the same length, every
    /// TTL must be ~360 s out, and nothing may be marked `in_cache` — cache-tier
    /// records are evictable and would silently drop us off the map.
    #[test]
    fn records_are_index_aligned_with_a_360s_ttl_and_never_cached() {
        let p = peer();
        let before = get_dht_time();
        let records = build_announce_records(&ctx(p), &server_info(Some(vpk_info()))).unwrap();
        let after = get_dht_time();

        for (i, r) in records.iter().enumerate() {
            let n = r.keys.len();
            assert_eq!(r.subkeys.len(), n, "record {i}");
            assert_eq!(r.values.len(), n, "record {i}");
            assert_eq!(r.expiration_time.len(), n, "record {i}");
            assert_eq!(r.in_cache.len(), n, "record {i}");
            assert!(r.in_cache.iter().all(|&c| !c), "record {i} must not cache");
            assert!(r.auth.is_some(), "record {i}");
            assert_eq!(
                r.peer.as_ref().map(|p| p.node_id.clone()),
                Some(NodeInfo::from_peer_id(p).node_id),
                "record {i} must carry our own DHTID"
            );
            for &exp in &r.expiration_time {
                assert!(
                    exp >= before + ANNOUNCE_TTL_SECS && exp <= after + ANNOUNCE_TTL_SECS,
                    "record {i}: expiration {exp} is not ~now+360s"
                );
            }
        }
    }

    // ── Byte-for-byte equality with the p2pd path ───────────────────────────

    /// The block and inference records carry the identical `Ext(64)` ServerInfo
    /// bytes that `DHTServerInfo::to_msgpack` produces — the value the crawler
    /// decodes. Any divergence here takes the node off the map.
    #[test]
    fn record_values_are_the_encoder_output() {
        let p = peer();
        let info = server_info(Some(vpk_info()));
        let records = build_announce_records(&ctx(p), &info).unwrap();

        let expected_info = info.to_msgpack().unwrap();
        assert!(
            records[0].values.iter().all(|v| v == &expected_info),
            "block records carry the Ext(64) ServerInfo"
        );
        assert_eq!(records[2].values, vec![expected_info]);

        let expected_model = ModelInfo {
            num_blocks: 36,
            repository: "Qwen/Qwen3-8B".to_string(),
        }
        .to_msgpack()
        .unwrap();
        assert_eq!(records[1].values, vec![expected_model]);

        assert_eq!(
            records[3].values,
            vec![info.vpk_info.as_ref().unwrap().to_msgpack_bytes().unwrap()]
        );
    }

    /// The ServerInfo value is an `Ext(64)` wrapping a 3-element array whose
    /// first element is the state — the layout `map.kwaai.ai` and Python
    /// Hivemind decode. Asserted structurally so a change to `rmpv`'s integer
    /// packing cannot silently reshape it.
    #[test]
    fn server_info_is_ext64_state_throughput_fields() {
        let info = server_info(None);
        let bytes = info.to_msgpack().unwrap();

        let value = rmpv::decode::read_value(&mut &bytes[..]).unwrap();
        let rmpv::Value::Ext(tag, inner) = value else {
            panic!("ServerInfo must be an Ext value, got {value:?}");
        };
        assert_eq!(tag, 64, "Ext tag 64 is the Python Hivemind tuple marker");

        let inner = rmpv::decode::read_value(&mut &inner[..]).unwrap();
        let rmpv::Value::Array(parts) = inner else {
            panic!("the Ext payload must be an array, got {inner:?}");
        };
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].as_i64(), Some(2), "state");
        assert_eq!(parts[1].as_f64(), Some(12.5), "throughput");
        assert!(matches!(parts[2], rmpv::Value::Map(_)), "fields");
    }

    // ── Unannounce ──────────────────────────────────────────────────────────

    /// The tombstone flips `state` to -1 and zeroes throughput, and drops
    /// attestations and VPK from the ServerInfo — but keeps the block range so
    /// it addresses exactly the records the announce wrote.
    #[test]
    fn unannounce_writes_state_minus_one_over_the_same_keys() {
        let p = peer();
        let info = server_info(Some(vpk_info()));
        let announced = build_announce_records(&ctx(p), &info).unwrap();
        let withdrawn = build_unannounce_records(&ctx(p), &info).unwrap();

        // Blocks + VPK only — the model registry is left alone.
        assert_eq!(withdrawn.len(), 2);
        assert_eq!(withdrawn[0].keys, announced[0].keys);
        assert_eq!(withdrawn[0].subkeys, announced[0].subkeys);
        assert_eq!(withdrawn[1].keys, vec![dht_id(VPK_NODES_KEY)]);

        let value = rmpv::decode::read_value(&mut &withdrawn[0].values[0][..]).unwrap();
        let rmpv::Value::Ext(64, inner) = value else {
            panic!("tombstone must still be an Ext(64) ServerInfo");
        };
        let rmpv::Value::Array(parts) = rmpv::decode::read_value(&mut &inner[..]).unwrap() else {
            panic!("Ext payload must be an array");
        };
        assert_eq!(parts[0].as_i64(), Some(-1), "state = -1 (offline)");
        assert_eq!(parts[1].as_f64(), Some(0.0), "throughput zeroed");
    }

    /// The load-bearing correction: the tombstone's expiration is in the
    /// **future**, and strictly greater than nothing — it is a normal
    /// `now + 360 s`. An expired or shortened timestamp is rejected by the
    /// bootstrap and would leave the live record standing.
    #[test]
    fn unannounce_expiration_is_in_the_future_not_expired() {
        let p = peer();
        let now = get_dht_time();
        let withdrawn = build_unannounce_records(&ctx(p), &server_info(None)).unwrap();

        for exp in &withdrawn[0].expiration_time {
            assert!(
                *exp > now,
                "an already-expired tombstone is rejected by hivemind, not applied"
            );
            assert!(
                *exp >= now + ANNOUNCE_TTL_SECS,
                "the tombstone must carry the full TTL, not a shortened one"
            );
        }
    }

    /// A tombstone stored after an announce must actually replace it in a real
    /// hivemind store — the end-to-end property the two rules above exist to
    /// guarantee. Uses `DHTStorage` as the reference implementation of a
    /// bootstrap's acceptance rules.
    #[test]
    fn tombstone_replaces_the_live_record_in_a_real_store() {
        use kwaai_hivemind_dht::DHTStorage;

        let p = peer();
        let info = server_info(None);
        let storage = DHTStorage::new(PeerId::random());

        let announced = build_announce_records(&ctx(p), &info).unwrap();
        assert!(
            storage.handle_store(announced[0].clone()).store_ok[0],
            "the announce must be accepted"
        );

        // A tombstone built in the same second ties on expiration and would be
        // rejected; the real re-announce loop is 300 s apart. Nudge past it the
        // way wall-clock time does.
        let mut withdrawn = build_unannounce_records(&ctx(p), &info).unwrap();
        for exp in &mut withdrawn[0].expiration_time {
            *exp += 1.0;
        }
        assert!(
            storage.handle_store(withdrawn[0].clone()).store_ok[0],
            "the tombstone must be accepted over the live record"
        );

        // The key still exists — a store can never delete — but its value is now
        // the offline ServerInfo.
        let found = storage.handle_find(kwaai_hivemind_dht::protocol::FindRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![announced[0].keys[0].clone()],
            peer: None,
        });
        let dict = kwaai_hivemind_dht::parse_dictionary(&found.results[0].value).unwrap();
        let subkey = rmp_serde::to_vec(&p.to_base58()).unwrap();
        assert_eq!(
            dict.entries[&subkey].0, withdrawn[0].values[0],
            "the stored value must be the tombstone"
        );
    }

    // ── Delivery ────────────────────────────────────────────────────────────

    /// Degenerate inputs must not report success — a node with no bootstraps is
    /// not announced, and saying otherwise would suppress the operator warning.
    #[tokio::test]
    async fn empty_bootstraps_or_records_report_failure() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let (handle, task) =
            kwaai_p2p::NetworkService::spawn(kwaai_p2p::NetworkConfig::for_tests(), keypair)
                .expect("service should start");

        let records = build_announce_records(&ctx(peer()), &server_info(None)).unwrap();

        let (ok, timings) = send_records_via_handle(&handle, &[], &records).await;
        assert!(!ok);
        assert!(timings.is_empty());

        let (ok, timings) = send_records_via_handle(
            &handle,
            &[
                "/ip4/127.0.0.1/tcp/1/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN"
                    .to_string(),
            ],
            &[],
        )
        .await;
        assert!(!ok);
        assert!(timings.is_empty());

        let _ = handle.shutdown().await;
        let _ = task.await;
    }

    // ── Decentralized delivery ──────────────────────────────────────────────

    /// The block record's packed parallel arrays split into one request per
    /// key, each still index-aligned — this is what lets eight blocks land on
    /// eight independently-chosen peer sets rather than all on one.
    #[test]
    fn splitting_a_packed_record_preserves_every_field() {
        let p = peer();
        let records = build_announce_records(&ctx(p), &server_info(None)).unwrap();
        let blocks = &records[0];
        assert_eq!(blocks.keys.len(), 3, "the fixture serves blocks 0..3");

        let split = split_by_key(blocks);
        assert_eq!(split.len(), 3, "one request per key");

        for (i, single) in split.iter().enumerate() {
            assert_eq!(single.keys, vec![blocks.keys[i].clone()]);
            assert_eq!(single.subkeys, vec![blocks.subkeys[i].clone()]);
            assert_eq!(single.values, vec![blocks.values[i].clone()]);
            assert_eq!(single.expiration_time, vec![blocks.expiration_time[i]]);
            assert_eq!(single.in_cache, vec![blocks.in_cache[i]]);
            assert_eq!(
                single.peer, blocks.peer,
                "every split keeps our own DHTID as the requester"
            );
        }
    }

    /// A single-key record splits to itself, and an empty one to nothing.
    #[test]
    fn splitting_degenerate_records_is_safe() {
        let p = peer();
        let records = build_announce_records(&ctx(p), &server_info(None)).unwrap();

        // `_petals.models` is already one key.
        assert_eq!(split_by_key(&records[1]).len(), 1);

        let empty = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![],
            subkeys: vec![],
            values: vec![],
            expiration_time: vec![],
            in_cache: vec![],
            peer: None,
        };
        assert!(split_by_key(&empty).is_empty());
    }

    /// A request whose arrays disagree in length yields only the fully-formed
    /// prefix rather than panicking on the short array.
    #[test]
    fn splitting_a_misaligned_record_drops_the_incomplete_tail() {
        let misaligned = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![dht_id("a"), dht_id("b")],
            subkeys: vec![b"sub".to_vec()], // one short
            values: vec![b"v1".to_vec(), b"v2".to_vec()],
            expiration_time: vec![1.0, 2.0],
            in_cache: vec![false, false],
            peer: None,
        };
        assert_eq!(split_by_key(&misaligned).len(), 1);
    }

    /// With no candidates, decentralized delivery reports failure rather than
    /// silently claiming success — the same contract the bootstrap path has for
    /// an empty peer list.
    #[tokio::test]
    async fn decentralized_delivery_with_no_candidates_reports_failure() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let (handle, task) =
            kwaai_p2p::NetworkService::spawn(kwaai_p2p::NetworkConfig::for_tests(), keypair)
                .expect("service should start");

        let records = build_announce_records(&ctx(peer()), &server_info(None)).unwrap();

        let (ok, timings) = send_records_decentralized(&handle, &[], &records, 3).await;
        assert!(!ok, "a node that knows no peers has not announced");
        assert!(timings.is_empty());

        // And no records is equally a non-announcement, even with candidates.
        let candidate = crate::placement::Candidate::new(PeerId::random(), "/addr".to_string());
        let (ok, timings) = send_records_decentralized(&handle, &[candidate], &[], 3).await;
        assert!(!ok);
        assert!(timings.is_empty());

        let _ = handle.shutdown().await;
        let _ = task.await;
    }

    /// Unreachable candidates produce a per-peer timing marked failed — the
    /// reputation store must learn about peers that did not answer, and each
    /// peer must appear once however many keys it was asked to hold.
    #[tokio::test]
    async fn decentralized_delivery_reports_one_failed_timing_per_peer() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let (handle, task) =
            kwaai_p2p::NetworkService::spawn(kwaai_p2p::NetworkConfig::for_tests(), keypair)
                .expect("service should start");

        // Two peers we have no route to: every store fails.
        let candidates: Vec<crate::placement::Candidate> = (0..2)
            .map(|i| {
                crate::placement::Candidate::new(
                    PeerId::random(),
                    format!("/ip4/127.0.0.1/tcp/{}", 9000 + i),
                )
            })
            .collect();

        // Three block keys, so each peer is tried repeatedly across the round.
        let records = build_announce_records(&ctx(peer()), &server_info(None)).unwrap();
        let (ok, timings) = send_records_decentralized(&handle, &candidates, &records, 3).await;

        assert!(!ok, "no peer accepted anything");
        assert_eq!(
            timings.len(),
            2,
            "one aggregated observation per peer, not one per key"
        );
        assert!(
            timings.iter().all(|(_, _, _, success)| !success),
            "unreachable peers must be recorded as failures"
        );

        let _ = handle.shutdown().await;
        let _ = task.await;
    }

    /// A bootstrap address without a `/p2p/` suffix is skipped with a warning
    /// rather than dialed blind — and produces no timing, so it cannot pollute
    /// the reputation store.
    #[tokio::test]
    async fn malformed_bootstrap_addresses_are_skipped() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let (handle, task) =
            kwaai_p2p::NetworkService::spawn(kwaai_p2p::NetworkConfig::for_tests(), keypair)
                .expect("service should start");

        let records = build_announce_records(&ctx(peer()), &server_info(None)).unwrap();
        let (ok, timings) = send_records_via_handle(
            &handle,
            &[
                "/ip4/127.0.0.1/tcp/1".to_string(),                   // no /p2p/
                "/ip4/127.0.0.1/tcp/1/p2p/not-a-peer-id".to_string(), // unparseable
            ],
            &records,
        )
        .await;

        assert!(!ok);
        assert!(
            timings.is_empty(),
            "unusable bootstrap addresses must not produce reputation observations"
        );

        let _ = handle.shutdown().await;
        let _ = task.await;
    }

    /// Build a minimal `Ext(64, [state, throughput, {fields}])` blob matching
    /// `DHTServerInfo::to_msgpack()`'s shape, with `lease_v1` present or
    /// absent — mirrors what a real (or legacy, pre-Capacity-Lease) peer's
    /// DHT announcement decodes from.
    fn make_server_info_ext_bytes(lease_v1: Option<bool>) -> Vec<u8> {
        let mut fields = vec![
            (rmpv::Value::from("start_block"), rmpv::Value::from(0i64)),
            (rmpv::Value::from("end_block"), rmpv::Value::from(32i64)),
            (
                rmpv::Value::from("public_name"),
                rmpv::Value::from("test-node"),
            ),
            (
                rmpv::Value::from("peer_id"),
                rmpv::Value::from(PeerId::random().to_base58().as_str()),
            ),
            (
                rmpv::Value::from("version"),
                rmpv::Value::from("kwaai-0.5.4"),
            ),
        ];
        if let Some(v) = lease_v1 {
            fields.push((rmpv::Value::from("lease_v1"), rmpv::Value::from(v)));
        }
        let inner = rmpv::Value::Array(vec![
            rmpv::Value::from(2i32), // state = ONLINE
            rmpv::Value::from(10.0), // throughput
            rmpv::Value::Map(fields),
        ]);
        let mut inner_bytes = Vec::new();
        rmpv::encode::write_value(&mut inner_bytes, &inner).unwrap();
        let ext = rmpv::Value::Ext(64, inner_bytes);
        let mut out = Vec::new();
        rmpv::encode::write_value(&mut out, &ext).unwrap();
        out
    }

    #[test]
    fn decode_server_info_ext_reads_lease_v1_when_present() {
        let bytes = make_server_info_ext_bytes(Some(true));
        let info = decode_server_info_ext(&bytes).expect("decodes");
        assert!(info.lease_v1);
    }

    /// Helper: a signed envelope for `key` naming `addrs`, exactly as the
    /// publisher emits one.
    fn signed_envelope(key: &libp2p::identity::Keypair, addrs: &[&str]) -> Vec<u8> {
        let addrs = addrs
            .iter()
            .map(|a| a.parse().expect("a valid addr"))
            .collect();
        libp2p::core::PeerRecord::new_interop(key, addrs)
            .expect("signs")
            .into_signed_envelope()
            .into_protobuf_encoding()
    }

    /// The publisher's own encoder is the producer of record, so the decoder is
    /// tested against what it emits rather than against a hand-built map. The
    /// address comes back bare, the form the daemon takes and keeps in its
    /// learned-address map.
    #[test]
    fn decode_server_info_ext_round_trips_a_signed_address() {
        let key = libp2p::identity::Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let mut published = DHTServerInfo::new(
            0,
            32,
            "test-node",
            true,
            10.0,
            Vec::new(),
            None,
            peer.to_base58(),
        );
        published.signed_addrs = signed_envelope(&key, &["/ip4/203.0.113.7/tcp/4001"]);

        let info =
            decode_server_info_ext(&published.to_msgpack().expect("encodes")).expect("decodes");
        assert_eq!(
            info.dial_addrs,
            vec!["/ip4/203.0.113.7/tcp/4001".parse::<Multiaddr>().unwrap()]
        );
    }

    /// The attack the signature exists to stop: anyone may write under any
    /// subkey, so a record whose addresses were signed by a different key must
    /// yield nothing — the peer falls back to a bare-PeerId dial rather than
    /// being steered at an address its own key never vouched for.
    #[test]
    fn decode_server_info_ext_rejects_addresses_signed_by_another_key() {
        let victim = libp2p::identity::Keypair::generate_ed25519()
            .public()
            .to_peer_id();
        let attacker = libp2p::identity::Keypair::generate_ed25519();
        let mut forged = DHTServerInfo::new(
            0,
            32,
            "test-node",
            true,
            10.0,
            Vec::new(),
            None,
            victim.to_base58(),
        );
        forged.signed_addrs = signed_envelope(&attacker, &["/ip4/198.51.100.9/tcp/4001"]);

        let info = decode_server_info_ext(&forged.to_msgpack().expect("encodes")).expect("decodes");
        assert!(
            info.dial_addrs.is_empty(),
            "an address list the announced peer did not sign must not be dialed"
        );
        assert_eq!(
            info.public_name, "test-node",
            "the rest of the record still decodes"
        );
    }

    /// Garbage in the field costs the addresses, not the record.
    #[test]
    fn decode_server_info_ext_ignores_an_unparseable_envelope() {
        let peer = PeerId::random();
        let mut info = DHTServerInfo::new(
            0,
            32,
            "test-node",
            true,
            10.0,
            Vec::new(),
            None,
            peer.to_base58(),
        );
        info.signed_addrs = vec![0xde, 0xad, 0xbe, 0xef];

        let decoded =
            decode_server_info_ext(&info.to_msgpack().expect("encodes")).expect("decodes");
        assert!(decoded.dial_addrs.is_empty());
        assert_eq!(
            decoded.end_block, 32,
            "the rest of the record still decodes"
        );
    }

    /// Circuits are the whole motivation, and their shape is the one a dialer
    /// cannot reconstruct: the relay hop must survive the round trip, and only
    /// the destination `/p2p` is stripped.
    #[test]
    fn decode_server_info_ext_round_trips_a_circuit_address() {
        let key = libp2p::identity::Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let relay = PeerId::random();
        let circuit = format!("/ip4/76.13.5.74/tcp/4001/p2p/{}/p2p-circuit", relay);

        let mut published = DHTServerInfo::new(
            0,
            32,
            "test-node",
            true,
            10.0,
            Vec::new(),
            None,
            peer.to_base58(),
        );
        published.signed_addrs = signed_envelope(&key, &[&circuit]);

        let info =
            decode_server_info_ext(&published.to_msgpack().expect("encodes")).expect("decodes");
        assert_eq!(info.dial_addrs, vec![circuit.parse::<Multiaddr>().unwrap()]);
    }

    /// A peer on a binary that predates the field decodes to an empty list,
    /// not a dropped record — it is still dialable by PeerId if it happens to
    /// be in the routing table.
    #[test]
    fn decode_server_info_ext_defaults_addrs_empty_for_legacy_bytes() {
        let bytes = make_server_info_ext_bytes(None);
        let info = decode_server_info_ext(&bytes).expect("decodes");
        assert_eq!(info.state, 2, "the rest of the record must still decode");
        assert!(info.dial_addrs.is_empty());
    }

    #[test]
    fn decode_server_info_ext_defaults_lease_v1_false_for_legacy_bytes() {
        // No lease_v1 key at all — exactly what a pre-Capacity-Lease peer's
        // announcement looks like. Must decode successfully (not error) and
        // default to false, not panic or silently drop the record.
        let bytes = make_server_info_ext_bytes(None);
        let info = decode_server_info_ext(&bytes).expect("decodes");
        let (state, lease_v1) = (info.state, info.lease_v1);
        assert_eq!(
            state, 2,
            "pre-existing fields must still decode alongside the new one"
        );
        assert!(!lease_v1);
    }
}
