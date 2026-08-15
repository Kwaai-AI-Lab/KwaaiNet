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
use prost::Message;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
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
/// `state` is a **deliberate KwaaiNet divergence** from upstream petals'
/// `ServerState` enum: `0` joining, `2` ready, `-1` offline. The map decodes
/// these values directly — preserve them verbatim.
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
        }
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
}
