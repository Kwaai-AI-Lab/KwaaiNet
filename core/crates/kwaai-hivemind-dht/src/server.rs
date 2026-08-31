//! Hivemind DHT serving: `rpc_store` / `rpc_find` / `rpc_ping`.
//!
//! Port of `hivemind/dht/protocol.py`'s server side. The storage semantics live
//! in [`crate::storage`]; this module is the request/response layer on top —
//! sentinel handling, the two storage tiers, and XOR-sorted nearest peers.
//!
//! Deliberately network-free: an owner (the swarm service) feeds it requests and
//! routing-table snapshots. Record validators/signatures are **not** implemented
//! (`_validate_record` / `_validate_dictionary` are no-ops here); that is a
//! tracked follow-up, and until it lands this node must not be treated as an
//! authenticated store.

use crate::codec::{DHTRequest, DHTResponse};
use crate::protocol::*;
use crate::storage::{parse_dictionary, LocalStorage, Stored};
use crate::value::get_dht_time;
use crate::Result;
use libp2p::PeerId;
use std::sync::{Arc, RwLock};
use tracing::{debug, trace, warn};

/// Reserved subkey tag meaning "this is a plain value, no subkey".
///
/// `IS_REGULAR_VALUE = serializer.dumps(None)` (`hivemind/dht/protocol.py:33`)
/// = msgpack nil = the single byte `0xC0`.
pub const IS_REGULAR_VALUE: &[u8] = &[0xc0];

/// Reserved subkey tag meaning "the value is a whole serialized
/// `DictionaryDHTValue`".
///
/// `IS_DICTIONARY = b""` (`hivemind/dht/protocol.py:33`) — **empty** bytes.
/// This is the opposite of the natural reading, and of the comment in our
/// `proto/dht.proto:30` ("None means no subkey"): an *empty* tag means the whole
/// dictionary is being shipped, while the msgpack-nil tag means no subkey.
/// Anything else is a real subkey, kept as opaque msgpack bytes.
pub const IS_DICTIONARY: &[u8] = b"";

/// Number of nearest neighbours returned per find result.
///
/// Hivemind uses `k=self.bucket_size` (`protocol.py:362`), which is
/// `DHTNode.bucket_size = 20` by default.
pub const DEFAULT_BUCKET_SIZE: usize = 20;

/// Default cache-tier capacity.
///
/// Matches hivemind's bounded cache (`DHTProtocol.create(cache_size=...)`).
pub const DEFAULT_CACHE_SIZE: usize = 32_768;

/// Default primary-tier capacity.
///
/// Hivemind leaves the primary tier unbounded — but it also gates writes
/// behind record validators, which this port does not implement yet. Since
/// `in_cache` is read straight off the wire, an unbounded primary tier would
/// let any dialable peer grow our memory without limit by sending
/// `in_cache = false` with a far-future expiration. A deliberately generous
/// bound (eviction drops the earliest-expiring entries first) keeps a network
/// root alive under abuse at the cost of strict hivemind parity; revisit when
/// validators land (tracked in the module docs).
pub const DEFAULT_STORAGE_SIZE: usize = 1_048_576;

/// Default primary-tier byte budget.
///
/// The entry-count bound above does not bound memory. A stored value is
/// capped only by the 10 MiB wire frame ([`crate::wire::MAX_FRAME_LEN`]), so
/// a million entries reach far past any figure the count suggests — and with
/// validators missing, the peer choosing those sizes is whoever dialled us.
/// This is the ceiling that actually holds, and it is the one to size against
/// the memory a bootstrap host can spare.
pub const DEFAULT_STORAGE_BYTES: usize = 256 * 1024 * 1024;

/// Default cache-tier byte budget. Smaller than the primary tier's: the cache
/// holds records this node fetched on someone else's behalf, and losing one
/// costs a re-fetch rather than a gap in what the network can find.
pub const DEFAULT_CACHE_BYTES: usize = 64 * 1024 * 1024;

/// Largest single value either tier accepts.
///
/// Well below [`crate::wire::MAX_FRAME_LEN`], which bounds what can arrive,
/// not what is worth keeping: every record this network actually publishes —
/// block ranges, `_petals.models`, the inference and VPK registries — is a
/// few hundred bytes. A megabyte is three orders of magnitude of headroom,
/// and refusing above it keeps one sender from evicting a whole tier with a
/// handful of stores.
pub const MAX_VALUE_BYTES: usize = 1024 * 1024;

/// Cap on `keys.len()` accepted from a single `StoreRequest`. Entries beyond
/// it are reported `store_ok = false`. A legitimate announce carries a
/// handful of keys; thousands in one frame is either a bug or an amplification
/// attempt (each key can cost an eviction scan under the write lock).
pub const MAX_STORE_KEYS_PER_REQUEST: usize = 1024;

/// A peer known to the routing table: its 20-byte DHTID and its libp2p peer ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPeer {
    /// `DHTID.to_bytes()` — 20 bytes, big-endian (`routing.py:288-290`).
    pub node_id: Vec<u8>,
    pub peer_id: PeerId,
}

impl RoutingPeer {
    /// Build a routing entry, deriving the DHTID from the peer ID.
    pub fn new(peer_id: PeerId) -> Self {
        Self {
            node_id: dht_id_from_peer_id(&peer_id),
            peer_id,
        }
    }

    /// Build a routing entry from an explicitly known DHTID.
    pub fn with_node_id(node_id: Vec<u8>, peer_id: PeerId) -> Self {
        Self { node_id, peer_id }
    }
}

/// DHT storage + serving backend.
///
/// Cloning shares the underlying state (`Arc`), so the swarm task and any
/// maintenance task can hold their own handles.
#[derive(Debug, Clone)]
pub struct DHTStorage {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    /// Primary tier — `in_cache = false` stores land here (`protocol.py:242`).
    storage: RwLock<LocalStorage>,
    /// Cache tier — `in_cache = true` stores land here. Capacity-bounded.
    cache: RwLock<LocalStorage>,
    /// Peers eligible to be returned as nearest neighbours.
    peers: RwLock<Vec<RoutingPeer>>,
    local_peer_id: PeerId,
    bucket_size: usize,
}

impl DHTStorage {
    /// Create a storage backend with default tier sizes.
    pub fn new(local_peer_id: PeerId) -> Self {
        Self::with_config(local_peer_id, DEFAULT_CACHE_SIZE, DEFAULT_BUCKET_SIZE)
    }

    /// Create a storage backend with explicit cache capacity and neighbour
    /// count. Both tiers are bounded by entry count *and* by bytes — see
    /// [`DEFAULT_STORAGE_BYTES`] for why the count alone is not a memory
    /// bound while validators are missing.
    pub fn with_config(local_peer_id: PeerId, cache_size: usize, bucket_size: usize) -> Self {
        Self {
            inner: Arc::new(Inner {
                storage: RwLock::new(LocalStorage::with_limits(
                    DEFAULT_STORAGE_SIZE,
                    DEFAULT_STORAGE_BYTES,
                    MAX_VALUE_BYTES,
                )),
                cache: RwLock::new(LocalStorage::with_limits(
                    cache_size,
                    DEFAULT_CACHE_BYTES,
                    MAX_VALUE_BYTES,
                )),
                peers: RwLock::new(Vec::new()),
                local_peer_id,
                bucket_size,
            }),
        }
    }

    /// This node's own DHTID.
    pub fn local_node_id(&self) -> Vec<u8> {
        dht_id_from_peer_id(&self.inner.local_peer_id)
    }

    /// Replace the known-peer set, e.g. from a Kademlia routing-table snapshot.
    ///
    /// These are the candidates for the `nearest_node_ids` / `nearest_peer_ids`
    /// arrays; hivemind draws them from its own routing table
    /// (`protocol.py:362-364`).
    pub fn update_peers(&self, peers: Vec<RoutingPeer>) {
        if let Ok(mut guard) = self.inner.peers.write() {
            *guard = peers;
        }
    }

    /// Convenience: set known peers from bare peer IDs, deriving each DHTID.
    pub fn update_peer_ids(&self, peers: impl IntoIterator<Item = PeerId>) {
        self.update_peers(peers.into_iter().map(RoutingPeer::new).collect());
    }

    /// Sweep expired entries from both tiers.
    ///
    /// Python removes outdated entries lazily on every access
    /// (`timed_storage.py:60-68`). We do that too, but a long-lived node also
    /// needs a sweep that does not require a request to arrive — otherwise a key
    /// nobody asks about is pinned forever.
    pub fn cleanup_expired(&self) {
        if let Ok(mut s) = self.inner.storage.write() {
            s.remove_outdated();
        }
        if let Ok(mut c) = self.inner.cache.write() {
            c.remove_outdated();
        }
    }

    /// `(total, valid)` entry counts across both tiers.
    ///
    /// Retained for the existing callers; after a [`Self::cleanup_expired`] the
    /// two numbers agree.
    pub fn stats(&self) -> (usize, usize) {
        let storage_len = self.inner.storage.read().map(|s| s.len()).unwrap_or(0);
        let cache_len = self.inner.cache.read().map(|c| c.len()).unwrap_or(0);
        let total = storage_len + cache_len;
        (total, total)
    }

    // ── STORE ───────────────────────────────────────────────────────────────

    /// Handle a STORE request.
    ///
    /// Port of `rpc_store` (`hivemind/dht/protocol.py:232-267`). The five
    /// parallel arrays are zipped in order and one `store_ok` is appended per
    /// element, so the response stays index-aligned with the request.
    ///
    /// Per element, the `subkeys[i]` tag selects the behaviour:
    /// * [`IS_DICTIONARY`] (empty) — `values[i]` is a serialized
    ///   `DictionaryDHTValue`; every entry is merged per-subkey and `store_ok` is
    ///   `all(...)` over the individual results (`protocol.py:244-256`).
    /// * [`IS_REGULAR_VALUE`] (msgpack nil) — a plain store
    ///   (`protocol.py:262-263`).
    /// * anything else — a real subkey; hivemind msgpack-decodes it
    ///   (`protocol.py:265`) but we key on the raw bytes, which is equivalent for
    ///   lookup and avoids a lossy decode/re-encode of arbitrary subkey types.
    pub fn handle_store(&self, request: StoreRequest) -> StoreResponse {
        debug!("STORE: {} keys", request.keys.len());

        let mut store_ok = Vec::with_capacity(request.keys.len());

        for (i, key) in request.keys.iter().enumerate() {
            // Bound the per-request work: each element can cost an eviction
            // scan under the write lock, and one 10 MiB frame can carry a lot
            // of tiny keys. Overflow elements are reported unstored, keeping
            // the response index-aligned.
            if i >= MAX_STORE_KEYS_PER_REQUEST {
                warn!(
                    total = request.keys.len(),
                    "STORE: request exceeds {MAX_STORE_KEYS_PER_REQUEST} keys; rejecting the rest"
                );
                store_ok.resize(request.keys.len(), false);
                break;
            }
            let Some(value) = request.values.get(i) else {
                warn!("STORE: missing value at index {i}; rejecting");
                store_ok.push(false);
                continue;
            };
            let Some(&expiration) = request.expiration_time.get(i) else {
                warn!("STORE: missing expiration at index {i}; rejecting");
                store_ok.push(false);
                continue;
            };
            // `in_cache` selects the tier (protocol.py:242). Absent → primary.
            let in_cache = request.in_cache.get(i).copied().unwrap_or(false);
            // An absent subkey entry is treated as "no subkey": proto3 repeated
            // fields drop trailing empties, and an empty tag would otherwise be
            // read as IS_DICTIONARY over a non-dictionary value.
            let tag: &[u8] = request
                .subkeys
                .get(i)
                .map(Vec::as_slice)
                .unwrap_or(IS_REGULAR_VALUE);

            let tier = if in_cache {
                &self.inner.cache
            } else {
                &self.inner.storage
            };
            let Ok(mut store) = tier.write() else {
                warn!("STORE: storage lock poisoned; rejecting");
                store_ok.push(false);
                continue;
            };

            let ok = if tag == IS_DICTIONARY {
                // Whole-dictionary store: merge every entry independently.
                match parse_dictionary(value) {
                    Some(dict) => {
                        // `all()` over an empty iterable is True in Python
                        // (protocol.py:251-255).
                        let mut all_ok = true;
                        for (subkey, (val, exp)) in dict.entries {
                            // Deliberately not `.all()`: that short-circuits, and
                            // these calls have side effects. Python's generator
                            // does short-circuit, but it iterates in insertion
                            // order while we iterate sorted, so short-circuiting
                            // would make *which* entries get applied depend on
                            // subkey ordering. Applying every entry is both
                            // deterministic and a superset of Python's effect.
                            let ok = store.store_subkey(key.clone(), subkey, val, exp);
                            all_ok &= ok;
                        }
                        all_ok
                    }
                    None => {
                        warn!("STORE: IS_DICTIONARY tag but value is not a DictionaryDHTValue");
                        false
                    }
                }
            } else if tag == IS_REGULAR_VALUE {
                store.store(key.clone(), value.clone(), expiration)
            } else {
                store.store_subkey(key.clone(), tag.to_vec(), value.clone(), expiration)
            };

            trace!(
                index = i,
                in_cache,
                ok,
                "STORE element {}",
                if tag == IS_DICTIONARY {
                    "dictionary"
                } else if tag == IS_REGULAR_VALUE {
                    "regular"
                } else {
                    "subkey"
                }
            );
            store_ok.push(ok);
        }

        StoreResponse {
            auth: Some(ResponseAuthInfo::new()),
            store_ok,
            peer: Some(NodeInfo::from_peer_id(self.inner.local_peer_id)),
        }
    }

    // ── FIND ────────────────────────────────────────────────────────────────

    /// Handle a FIND request.
    ///
    /// Port of `rpc_find` (`hivemind/dht/protocol.py:332-367`). For each key:
    ///
    /// * consult both tiers and prefer whichever has the **later** expiration —
    ///   the cache wins only if the primary is absent or strictly older
    ///   (`protocol.py:342-347`);
    /// * a dictionary hit returns `FOUND_DICTIONARY` with the `Ext(0x50)`
    ///   serialization and the **outer** entry's expiration
    ///   (`protocol.py:352-356`) — note our `proto/dht.proto:54` claims this is
    ///   `latest_expiration_time`, which the code contradicts;
    /// * nearest peers are attached to **every** result including `NOT_FOUND`
    ///   (`protocol.py:362-364`), which is what makes an iterative lookup
    ///   converge.
    pub fn handle_find(&self, request: FindRequest) -> FindResponse {
        debug!("FIND: {} keys", request.keys.len());

        // Exclude the requester from its own neighbour list (protocol.py:363).
        let exclude = request
            .peer
            .as_ref()
            .map(|p| p.node_id.clone())
            .unwrap_or_default();

        let mut results = Vec::with_capacity(request.keys.len());

        for key in &request.keys {
            let (nearest_node_ids, nearest_peer_ids) = self.nearest_peers(key, &exclude);

            let hit = self.lookup(key);

            let result = match hit {
                Some((stored, expiration)) => FindResult {
                    result_type: stored.result_type() as i32,
                    value: stored.to_find_value(),
                    expiration_time: expiration,
                    nearest_node_ids,
                    nearest_peer_ids,
                },
                None => FindResult {
                    result_type: ResultType::NotFound as i32,
                    value: vec![],
                    expiration_time: 0.0,
                    nearest_node_ids,
                    nearest_peer_ids,
                },
            };
            results.push(result);
        }

        FindResponse {
            auth: Some(ResponseAuthInfo::new()),
            results,
            peer: Some(NodeInfo::from_peer_id(self.inner.local_peer_id)),
        }
    }

    /// Read a key from both tiers, preferring the later expiration.
    ///
    /// `protocol.py:342-347`: the cached item replaces the primary one only when
    /// the primary is missing or `cached.expiration_time > maybe.expiration_time`
    /// (strictly greater — a tie keeps the primary).
    fn lookup(&self, key: &[u8]) -> Option<(Stored, f64)> {
        let primary = self
            .inner
            .storage
            .read()
            .ok()
            .and_then(|s| s.get(key).map(|(v, e)| (v.clone(), e)));
        let cached = self
            .inner
            .cache
            .read()
            .ok()
            .and_then(|c| c.get(key).map(|(v, e)| (v.clone(), e)));

        match (primary, cached) {
            (Some(p), Some(c)) => Some(if c.1 > p.1 { c } else { p }),
            (Some(p), None) => Some(p),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        }
    }

    /// The `bucket_size` peers nearest `key` by XOR distance.
    ///
    /// `protocol.py:362-364` + `routing.py:108-128`. Distance is the bitwise XOR
    /// of the two 20-byte big-endian DHTIDs; because the IDs are big-endian, a
    /// plain lexicographic comparison of the XOR bytes is exactly numeric
    /// comparison, so no bignum arithmetic is needed.
    ///
    /// `exclude` is the requester's DHTID — hivemind passes
    /// `DHTID.from_bytes(request.peer.node_id)` and never returns it. An empty
    /// `exclude` (no `peer` in the request) excludes nothing.
    fn nearest_peers(&self, key: &[u8], exclude: &[u8]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        let Ok(peers) = self.inner.peers.read() else {
            return (vec![], vec![]);
        };

        let mut ranked: Vec<(Vec<u8>, &RoutingPeer)> = peers
            .iter()
            .filter(|p| exclude.is_empty() || p.node_id != exclude)
            .map(|p| (xor_distance(key, &p.node_id), p))
            .collect();

        ranked.sort_by(|a, b| a.0.cmp(&b.0));
        ranked.truncate(self.inner.bucket_size);

        ranked
            .into_iter()
            .map(|(_, p)| (p.node_id.clone(), p.peer_id.to_bytes()))
            .unzip()
    }

    // ── Dispatch ────────────────────────────────────────────────────────────

    /// Handle any DHT request.
    pub fn handle_request(&self, request: DHTRequest) -> Result<DHTResponse> {
        match request {
            DHTRequest::Store(req) => Ok(DHTResponse::Store(self.handle_store(req))),
            DHTRequest::Find(req) => Ok(DHTResponse::Find(self.handle_find(req))),
            DHTRequest::Ping(_) => Ok(DHTResponse::Ping(PingResponse {
                auth: Some(ResponseAuthInfo::new()),
                peer: Some(NodeInfo::from_peer_id(self.inner.local_peer_id)),
                dht_time: get_dht_time(),
                available: true,
            })),
        }
    }
}

/// XOR distance between two DHTIDs, as raw bytes.
///
/// `DHTID.xor_distance` is `int(self) ^ int(other)` (`routing.py:273-281`) over
/// 20-byte big-endian integers; the byte-wise XOR compares identically.
/// Differing lengths are zero-extended on the left so a malformed ID still
/// orders deterministically rather than panicking.
fn xor_distance(a: &[u8], b: &[u8]) -> Vec<u8> {
    let n = a.len().max(b.len());
    // Index from the right so both operands are effectively left-padded with
    // zeros, preserving big-endian numeric alignment.
    (0..n)
        .map(|i| {
            let from_end = n - i;
            let ai = a.len().checked_sub(from_end).map_or(0, |idx| a[idx]);
            let bi = b.len().checked_sub(from_end).map_or(0, |idx| b[idx]);
            ai ^ bi
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::serialize_dictionary;
    use std::collections::BTreeMap;

    fn future() -> f64 {
        get_dht_time() + 3600.0
    }

    fn store_req(
        keys: Vec<Vec<u8>>,
        subkeys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
        expirations: Vec<f64>,
        in_cache: Vec<bool>,
    ) -> StoreRequest {
        StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys,
            subkeys,
            values,
            expiration_time: expirations,
            in_cache,
            peer: None,
        }
    }

    fn find_req(keys: Vec<Vec<u8>>) -> FindRequest {
        FindRequest {
            auth: Some(RequestAuthInfo::new()),
            keys,
            peer: None,
        }
    }

    // ── Store / find basics ─────────────────────────────────────────────────

    #[test]
    fn regular_store_and_find_round_trips() {
        let s = DHTStorage::new(PeerId::random());

        let res = s.handle_store(store_req(
            vec![b"k".to_vec()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"v".to_vec()],
            vec![future()],
            vec![false],
        ));
        assert_eq!(res.store_ok, vec![true]);

        let found = s.handle_find(find_req(vec![b"k".to_vec()]));
        assert_eq!(
            found.results[0].result_type,
            ResultType::FoundRegular as i32
        );
        assert_eq!(found.results[0].value, b"v");
    }

    #[test]
    fn expired_store_is_rejected() {
        let s = DHTStorage::new(PeerId::random());
        let res = s.handle_store(store_req(
            vec![b"k".to_vec()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"v".to_vec()],
            vec![0.0],
            vec![false],
        ));
        assert_eq!(res.store_ok, vec![false]);
    }

    #[test]
    fn stale_and_equal_expirations_are_rejected() {
        let s = DHTStorage::new(PeerId::random());
        let exp = future();

        let ok = |e: f64, v: &[u8]| {
            s.handle_store(store_req(
                vec![b"k".to_vec()],
                vec![IS_REGULAR_VALUE.to_vec()],
                vec![v.to_vec()],
                vec![e],
                vec![false],
            ))
            .store_ok[0]
        };

        assert!(ok(exp, b"first"));
        assert!(!ok(exp, b"equal"), "equal expiration must be rejected");
        assert!(
            !ok(exp - 60.0, b"stale"),
            "stale expiration must be rejected"
        );
        assert!(ok(exp + 60.0, b"fresh"));

        let found = s.handle_find(find_req(vec![b"k".to_vec()]));
        assert_eq!(found.results[0].value, b"fresh");
    }

    // ── THE petals-critical behaviour ───────────────────────────────────────

    /// Two peers storing *different* subkeys under one key must accumulate.
    #[test]
    fn two_peers_storing_different_subkeys_accumulate() {
        let s = DHTStorage::new(PeerId::random());
        let key = b"model.block.0".to_vec();
        let sk_a = rmp_serde::to_vec("QmPeerA").unwrap();
        let sk_b = rmp_serde::to_vec("QmPeerB").unwrap();

        assert_eq!(
            s.handle_store(store_req(
                vec![key.clone()],
                vec![sk_a.clone()],
                vec![b"infoA".to_vec()],
                vec![future()],
                vec![false],
            ))
            .store_ok,
            vec![true]
        );
        assert_eq!(
            s.handle_store(store_req(
                vec![key.clone()],
                vec![sk_b.clone()],
                vec![b"infoB".to_vec()],
                vec![future()],
                vec![false],
            ))
            .store_ok,
            vec![true]
        );

        let found = s.handle_find(find_req(vec![key]));
        assert_eq!(
            found.results[0].result_type,
            ResultType::FoundDictionary as i32,
            "a subkeyed key must serve as FOUND_DICTIONARY"
        );

        let dict = crate::storage::parse_dictionary(&found.results[0].value).unwrap();
        assert_eq!(dict.entries.len(), 2, "both peers must survive");
        assert_eq!(dict.entries[&sk_a].0, b"infoA");
        assert_eq!(dict.entries[&sk_b].0, b"infoB");
    }

    /// `FOUND_DICTIONARY.expiration_time` is the **outer** entry expiration
    /// (`protocol.py:355`), not the dictionary's `latest_expiration_time` as
    /// `proto/dht.proto:54` claims. Here the two coincide by construction, so we
    /// assert the outer value explicitly.
    #[test]
    fn found_dictionary_reports_outer_expiration() {
        let s = DHTStorage::new(PeerId::random());
        let key = b"k".to_vec();
        let early = future();
        let late = early + 500.0;

        s.handle_store(store_req(
            vec![key.clone()],
            vec![b"a".to_vec()],
            vec![b"v".to_vec()],
            vec![early],
            vec![false],
        ));
        s.handle_store(store_req(
            vec![key.clone()],
            vec![b"b".to_vec()],
            vec![b"v".to_vec()],
            vec![late],
            vec![false],
        ));

        let found = s.handle_find(find_req(vec![key]));
        assert_eq!(found.results[0].expiration_time, late);

        let dict = crate::storage::parse_dictionary(&found.results[0].value).unwrap();
        assert_eq!(dict.latest_expiration, late);
    }

    /// A subkeyed store over an older regular value promotes it to a dictionary
    /// (`storage.py:59-63`).
    #[test]
    fn regular_value_is_promoted_to_dictionary() {
        let s = DHTStorage::new(PeerId::random());
        let key = b"k".to_vec();
        let exp = future();

        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"plain".to_vec()],
            vec![exp],
            vec![false],
        ));
        assert_eq!(
            s.handle_find(find_req(vec![key.clone()])).results[0].result_type,
            ResultType::FoundRegular as i32
        );

        s.handle_store(store_req(
            vec![key.clone()],
            vec![b"sk".to_vec()],
            vec![b"v".to_vec()],
            vec![exp + 10.0],
            vec![false],
        ));

        assert_eq!(
            s.handle_find(find_req(vec![key])).results[0].result_type,
            ResultType::FoundDictionary as i32
        );
    }

    /// The `IS_DICTIONARY` (empty) tag ships a whole serialized dictionary; each
    /// entry merges per-subkey and `store_ok` is `all(...)`
    /// (`protocol.py:244-256`).
    #[test]
    fn is_dictionary_tag_merges_every_entry() {
        let s = DHTStorage::new(PeerId::random());
        let key = b"k".to_vec();
        let exp = future();

        let mut entries = BTreeMap::new();
        entries.insert(b"a".to_vec(), (b"va".to_vec(), exp));
        entries.insert(b"b".to_vec(), (b"vb".to_vec(), exp));
        let payload = serialize_dictionary(&entries, None, exp);

        let res = s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_DICTIONARY.to_vec()],
            vec![payload],
            vec![exp],
            vec![false],
        ));
        assert_eq!(res.store_ok, vec![true]);

        let found = s.handle_find(find_req(vec![key.clone()]));
        let dict = crate::storage::parse_dictionary(&found.results[0].value).unwrap();
        assert_eq!(dict.entries.len(), 2);

        // Re-storing the same dictionary: every entry now ties on expiration, so
        // each per-subkey store fails and `all(...)` is false.
        let mut same = BTreeMap::new();
        same.insert(b"a".to_vec(), (b"va2".to_vec(), exp));
        same.insert(b"b".to_vec(), (b"vb2".to_vec(), exp));
        let res = s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_DICTIONARY.to_vec()],
            vec![serialize_dictionary(&same, None, exp)],
            vec![exp],
            vec![false],
        ));
        assert_eq!(
            res.store_ok,
            vec![false],
            "all() over stale entries is false"
        );

        // A partially-fresh dictionary reports false but still applies the fresh
        // entry — hivemind's `all()` does not undo successful stores.
        let mut mixed = BTreeMap::new();
        mixed.insert(b"a".to_vec(), (b"va3".to_vec(), exp)); // stale → false
        mixed.insert(b"b".to_vec(), (b"vb3".to_vec(), exp + 60.0)); // fresh → true
        let res = s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_DICTIONARY.to_vec()],
            vec![serialize_dictionary(&mixed, None, exp + 60.0)],
            vec![exp + 60.0],
            vec![false],
        ));
        assert_eq!(res.store_ok, vec![false]);

        let found = s.handle_find(find_req(vec![key]));
        let dict = crate::storage::parse_dictionary(&found.results[0].value).unwrap();
        assert_eq!(
            dict.entries[&b"a".to_vec()].0,
            b"va",
            "stale entry unchanged"
        );
        assert_eq!(
            dict.entries[&b"b".to_vec()].0,
            b"vb3",
            "fresh entry applied"
        );
    }

    /// Batch requests stay index-aligned even when individual elements fail.
    #[test]
    fn batch_store_preserves_zip_order() {
        let s = DHTStorage::new(PeerId::random());
        let exp = future();

        let res = s.handle_store(store_req(
            vec![b"ok1".to_vec(), b"bad".to_vec(), b"ok2".to_vec()],
            vec![
                IS_REGULAR_VALUE.to_vec(),
                IS_REGULAR_VALUE.to_vec(),
                IS_REGULAR_VALUE.to_vec(),
            ],
            vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()],
            vec![exp, 0.0, exp],
            vec![false, false, false],
        ));
        assert_eq!(res.store_ok, vec![true, false, true]);
    }

    // ── Tier preference ─────────────────────────────────────────────────────

    /// `protocol.py:342-347` — the later expiration wins across tiers.
    #[test]
    fn find_prefers_the_tier_with_the_later_expiration() {
        let key = b"k".to_vec();
        let exp = future();

        // Cache newer than storage → cache wins.
        let s = DHTStorage::new(PeerId::random());
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_storage".to_vec()],
            vec![exp],
            vec![false],
        ));
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_cache".to_vec()],
            vec![exp + 100.0],
            vec![true],
        ));
        assert_eq!(
            s.handle_find(find_req(vec![key.clone()])).results[0].value,
            b"from_cache"
        );

        // Storage newer than cache → storage wins.
        let s = DHTStorage::new(PeerId::random());
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_storage".to_vec()],
            vec![exp + 100.0],
            vec![false],
        ));
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_cache".to_vec()],
            vec![exp],
            vec![true],
        ));
        assert_eq!(
            s.handle_find(find_req(vec![key.clone()])).results[0].value,
            b"from_storage"
        );

        // Tie → storage wins (the comparison is strictly greater).
        let s = DHTStorage::new(PeerId::random());
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_storage".to_vec()],
            vec![exp],
            vec![false],
        ));
        s.handle_store(store_req(
            vec![key.clone()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"from_cache".to_vec()],
            vec![exp],
            vec![true],
        ));
        assert_eq!(
            s.handle_find(find_req(vec![key])).results[0].value,
            b"from_storage"
        );
    }

    /// The tiers are separate: an `in_cache` store must not satisfy a lookup
    /// that the primary tier should have missed, and vice versa.
    #[test]
    fn tiers_are_independent() {
        let s = DHTStorage::new(PeerId::random());
        s.handle_store(store_req(
            vec![b"cached".to_vec()],
            vec![IS_REGULAR_VALUE.to_vec()],
            vec![b"v".to_vec()],
            vec![future()],
            vec![true],
        ));

        // Visible on find (find consults both tiers)…
        assert_eq!(
            s.handle_find(find_req(vec![b"cached".to_vec()])).results[0].result_type,
            ResultType::FoundRegular as i32
        );
        // …but it landed in the cache tier, not the primary one.
        assert_eq!(s.inner.storage.read().unwrap().len(), 0);
        assert_eq!(s.inner.cache.read().unwrap().len(), 1);
    }

    // ── XOR nearest peers ───────────────────────────────────────────────────

    /// Hand-computed ordering. With a key of all-zero bytes the XOR distance is
    /// the node ID itself, so ordering by distance is ordering by ID.
    #[test]
    fn nearest_peers_are_xor_sorted() {
        let s = DHTStorage::new(PeerId::random());

        let id = |first: u8| {
            let mut v = vec![0u8; 20];
            v[0] = first;
            v
        };
        // Deliberately inserted out of order — insertion order must not leak.
        let peers = vec![
            RoutingPeer::with_node_id(id(0x80), PeerId::random()),
            RoutingPeer::with_node_id(id(0x01), PeerId::random()),
            RoutingPeer::with_node_id(id(0x40), PeerId::random()),
            RoutingPeer::with_node_id(id(0x02), PeerId::random()),
        ];
        s.update_peers(peers);

        let key = vec![0u8; 20];
        let (node_ids, peer_ids) = s.nearest_peers(&key, &[]);

        assert_eq!(node_ids.len(), 4);
        assert_eq!(peer_ids.len(), 4);
        assert_eq!(
            node_ids.iter().map(|n| n[0]).collect::<Vec<_>>(),
            vec![0x01, 0x02, 0x40, 0x80],
            "ascending XOR distance"
        );
    }

    /// Distance is relative to the *key*, not to zero.
    #[test]
    fn nearest_peers_are_relative_to_the_query_key() {
        let s = DHTStorage::new(PeerId::random());

        let id = |first: u8| {
            let mut v = vec![0u8; 20];
            v[0] = first;
            v
        };
        s.update_peers(vec![
            RoutingPeer::with_node_id(id(0x00), PeerId::random()),
            RoutingPeer::with_node_id(id(0xFF), PeerId::random()),
            RoutingPeer::with_node_id(id(0xF0), PeerId::random()),
        ]);

        // Key = 0xFF… → distances 0xFF, 0x00, 0x0F.
        let mut key = vec![0u8; 20];
        key[0] = 0xFF;
        let (node_ids, _) = s.nearest_peers(&key, &[]);
        assert_eq!(
            node_ids.iter().map(|n| n[0]).collect::<Vec<_>>(),
            vec![0xFF, 0xF0, 0x00]
        );
    }

    /// The requester is never returned to itself (`protocol.py:363`).
    #[test]
    fn nearest_peers_exclude_the_requester() {
        let s = DHTStorage::new(PeerId::random());

        let requester_id = {
            let mut v = vec![0u8; 20];
            v[0] = 0x01;
            v
        };
        let other_id = {
            let mut v = vec![0u8; 20];
            v[0] = 0x02;
            v
        };
        s.update_peers(vec![
            RoutingPeer::with_node_id(requester_id.clone(), PeerId::random()),
            RoutingPeer::with_node_id(other_id.clone(), PeerId::random()),
        ]);

        // Without exclusion, the requester is nearest and would come first.
        let (all, _) = s.nearest_peers(&[0u8; 20], &[]);
        assert_eq!(all.len(), 2);

        let found = s.handle_find(FindRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![vec![0u8; 20]],
            peer: Some(NodeInfo {
                node_id: requester_id.clone(),
            }),
        });
        let returned = &found.results[0].nearest_node_ids;
        assert_eq!(returned, &vec![other_id]);
        assert!(!returned.contains(&requester_id));
    }

    /// `k = bucket_size` caps the neighbour list (`protocol.py:362`).
    #[test]
    fn nearest_peers_are_capped_at_bucket_size() {
        let s = DHTStorage::new(PeerId::random());
        s.update_peer_ids((0..50).map(|_| PeerId::random()));

        let found = s.handle_find(find_req(vec![vec![0u8; 20]]));
        assert_eq!(found.results[0].nearest_node_ids.len(), DEFAULT_BUCKET_SIZE);
        assert_eq!(found.results[0].nearest_peer_ids.len(), DEFAULT_BUCKET_SIZE);
    }

    /// Nearest peers accompany NOT_FOUND too — this is what lets an iterative
    /// lookup make progress (`protocol.py:362-364` runs for every result).
    #[test]
    fn not_found_still_returns_nearest_peers() {
        let s = DHTStorage::new(PeerId::random());
        s.update_peer_ids((0..3).map(|_| PeerId::random()));

        let found = s.handle_find(find_req(vec![b"absent".to_vec()]));
        assert_eq!(found.results[0].result_type, ResultType::NotFound as i32);
        assert!(found.results[0].value.is_empty());
        assert_eq!(found.results[0].nearest_node_ids.len(), 3);
        assert_eq!(found.results[0].nearest_peer_ids.len(), 3);
    }

    /// Returned node IDs must be the 20-byte DHTIDs and the peer IDs must be
    /// parseable back into `PeerId`s — the two arrays are aligned by index.
    #[test]
    fn nearest_peer_arrays_are_aligned_and_well_formed() {
        let s = DHTStorage::new(PeerId::random());
        let peers: Vec<PeerId> = (0..5).map(|_| PeerId::random()).collect();
        s.update_peer_ids(peers.clone());

        let found = s.handle_find(find_req(vec![vec![0u8; 20]]));
        let r = &found.results[0];

        for (node_id, peer_bytes) in r.nearest_node_ids.iter().zip(&r.nearest_peer_ids) {
            assert_eq!(node_id.len(), 20);
            let pid = PeerId::from_bytes(peer_bytes).expect("valid peer id");
            assert_eq!(&dht_id_from_peer_id(&pid), node_id, "arrays must align");
        }
    }

    // ── Housekeeping ────────────────────────────────────────────────────────

    #[test]
    fn ping_reports_availability_and_our_node_id() {
        let peer_id = PeerId::random();
        let s = DHTStorage::new(peer_id);

        let res = s
            .handle_request(DHTRequest::Ping(PingRequest::new(
                NodeInfo::from_peer_id(PeerId::random()),
                true,
            )))
            .unwrap();

        match res {
            DHTResponse::Ping(p) => {
                assert!(p.available);
                assert_eq!(p.peer.unwrap().node_id, dht_id_from_peer_id(&peer_id));
                assert!(p.dht_time > 0.0);
            }
            other => panic!("expected a ping response, got {other:?}"),
        }
    }

    #[test]
    fn cleanup_sweeps_both_tiers() {
        let s = DHTStorage::new(PeerId::random());
        s.handle_store(store_req(
            vec![b"a".to_vec(), b"b".to_vec()],
            vec![IS_REGULAR_VALUE.to_vec(); 2],
            vec![b"v".to_vec(), b"v".to_vec()],
            vec![future(), future()],
            vec![false, true],
        ));
        assert_eq!(s.stats().0, 2);

        s.cleanup_expired();
        assert_eq!(s.stats().0, 2, "live entries survive a sweep");
    }

    /// The cache tier is capacity-bounded so a node acting as a network root
    /// cannot be driven out of memory by cache stores.
    #[test]
    fn cache_tier_is_capacity_bounded() {
        let s = DHTStorage::with_config(PeerId::random(), 4, DEFAULT_BUCKET_SIZE);
        let base = future();

        for i in 0..20u32 {
            s.handle_store(store_req(
                vec![format!("k{i}").into_bytes()],
                vec![IS_REGULAR_VALUE.to_vec()],
                vec![b"v".to_vec()],
                vec![base + f64::from(i)],
                vec![true],
            ));
        }

        assert_eq!(s.inner.cache.read().unwrap().len(), 4);
        // The longest-lived keys are the survivors.
        assert!(s.inner.cache.read().unwrap().get(b"k19").is_some());
        assert!(s.inner.cache.read().unwrap().get(b"k0").is_none());
    }

    #[test]
    fn missing_subkey_entry_is_treated_as_a_regular_store() {
        let s = DHTStorage::new(PeerId::random());
        // proto3 drops trailing empty entries; an absent tag must not be read as
        // IS_DICTIONARY over a non-dictionary value.
        let res = s.handle_store(store_req(
            vec![b"k".to_vec()],
            vec![],
            vec![b"v".to_vec()],
            vec![future()],
            vec![false],
        ));
        assert_eq!(res.store_ok, vec![true]);
        assert_eq!(
            s.handle_find(find_req(vec![b"k".to_vec()])).results[0].result_type,
            ResultType::FoundRegular as i32
        );
    }

    /// A request larger than [`MAX_STORE_KEYS_PER_REQUEST`] processes only the
    /// first cap-many elements; the rest come back `store_ok = false`, and the
    /// response stays index-aligned with the request.
    #[test]
    fn oversized_store_request_is_truncated_not_processed() {
        let s = DHTStorage::new(PeerId::random());
        let total = MAX_STORE_KEYS_PER_REQUEST + 8;
        let now = get_dht_time();

        let keys: Vec<Vec<u8>> = (0..total)
            .map(|i| format!("key-{i}").into_bytes())
            .collect();
        let values = vec![b"v".to_vec(); total];
        let subkeys = vec![IS_REGULAR_VALUE.to_vec(); total];
        let expirations = vec![now + 60.0; total];
        let in_cache = vec![false; total];

        let resp = s.handle_store(store_req(keys, subkeys, values, expirations, in_cache));
        assert_eq!(
            resp.store_ok.len(),
            total,
            "response must stay index-aligned"
        );
        assert!(resp.store_ok[..MAX_STORE_KEYS_PER_REQUEST]
            .iter()
            .all(|&ok| ok));
        assert!(resp.store_ok[MAX_STORE_KEYS_PER_REQUEST..]
            .iter()
            .all(|&ok| !ok));
        let (stored, _) = s.stats();
        assert_eq!(stored, MAX_STORE_KEYS_PER_REQUEST);
    }
}
