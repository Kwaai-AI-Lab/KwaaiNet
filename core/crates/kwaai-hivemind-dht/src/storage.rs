//! Hivemind-compatible DHT local storage.
//!
//! This is a port of hivemind's `DHTLocalStorage` (`hivemind/dht/storage.py`) on
//! top of `TimedStorage` (`hivemind/utils/timed_storage.py`), including the
//! `DictionaryDHTValue` msgpack representation that Petals module records use.
//!
//! Everything here is deliberately network-free: it is the storage tier that
//! [`crate::server::DHTStorage`] serves `rpc_store` / `rpc_find` from.
//!
//! # Why dictionaries matter
//!
//! Petals servers announce themselves under a *shared* key (`{prefix}.{block}`)
//! using their own base58 peer ID as a **subkey**. Storing without subkey
//! support makes N servers announcing one block overwrite each other, so a
//! block ends up with exactly one visible server. Hivemind models this as a
//! `DictionaryDHTValue`: one DHT key → many (subkey, value, expiration) entries,
//! each with an *independent* expiration.
//!
//! # Serialization
//!
//! `DictionaryDHTValue` is registered as msgpack ext type `0x50`
//! (`storage.py:10`) and packs as (`storage.py:20-23`):
//!
//! ```text
//! Ext(0x50, msgpack([maxsize, latest_expiration_time, [[subkey, value, expiration], ...]]))
//! ```
//!
//! Note ext code `0x40` is hivemind's *tuple* code (`utils/serializer.py:27`)
//! and is unrelated — Petals `ServerInfo` values are `Ext(0x40, ...)` and travel
//! through this module as opaque bytes.
//!
//! Entry values are msgpack **Binary** (`dumps` uses `use_bin_type=True`,
//! `serializer.py:68`). Subkeys are stored exactly as received on the wire —
//! raw msgpack bytes, never decoded — so we can round-trip anything a Python
//! peer sends.

use crate::value::get_dht_time;
use std::collections::BTreeMap;

/// Absolute expiration, seconds since the UNIX epoch (hivemind `DHTExpiration`).
pub type DHTExpiration = f64;

/// msgpack ext type code for `DictionaryDHTValue` (`hivemind/dht/storage.py:10`).
pub const DICTIONARY_EXT_CODE: i8 = 0x50;

/// msgpack ext type code for Python tuples (`hivemind/utils/serializer.py:27`).
///
/// Not produced or consumed here; recorded so the two codes are never confused.
pub const TUPLE_EXT_CODE: i8 = 0x40;

/// A stored DHT entry: either an opaque binary value or a subkeyed dictionary.
///
/// Mirrors `DHTLocalStorage`'s value union
/// (`Union[BinaryDHTValue, DictionaryDHTValue]`, `storage.py:35`).
#[derive(Debug, Clone, PartialEq)]
pub enum Stored {
    /// A plain value stored without a subkey.
    Regular {
        value: Vec<u8>,
        expiration: DHTExpiration,
    },
    /// A dictionary of subkey → (value, expiration).
    ///
    /// `BTreeMap` keeps entries in a deterministic order so serialization is
    /// reproducible; Python preserves insertion order instead, but the wire
    /// format imposes no ordering requirement on readers.
    Dictionary {
        entries: BTreeMap<Vec<u8>, (Vec<u8>, DHTExpiration)>,
        /// `maxsize`; `None` = unbounded (Python `float("inf")`,
        /// `timed_storage.py:55`).
        maxsize: Option<u64>,
        /// `latest_expiration_time` — the running max over every entry ever
        /// stored (`storage.py:14,17`). Note this is *not* recomputed when an
        /// entry expires, matching Python.
        latest_expiration: DHTExpiration,
    },
}

/// An entry in the outer storage, carrying the outer expiration separately.
///
/// Hivemind tracks the outer expiration in `TimedStorage.data` independently of
/// `DictionaryDHTValue.latest_expiration_time`; the two can legitimately differ
/// (see `store_subkey`, `storage.py:64-67`), and `rpc_find` reports the *outer*
/// one (`protocol.py:355`).
#[derive(Debug, Clone)]
struct Entry {
    value: Stored,
    expiration: DHTExpiration,
}

/// A hivemind `TimedStorage`-alike: expiring entries with an optional size bound.
///
/// Port of `hivemind/utils/timed_storage.py` + `DHTLocalStorage`
/// (`hivemind/dht/storage.py:35-69`).
#[derive(Debug, Default)]
pub struct LocalStorage {
    data: BTreeMap<Vec<u8>, Entry>,
    /// Capacity bound; `None` = unbounded. Enforced by evicting the
    /// earliest-expiring entry (`timed_storage.py:60-68`).
    maxsize: Option<usize>,
}

impl LocalStorage {
    /// Create an unbounded storage (the hivemind default for the primary tier).
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            maxsize: None,
        }
    }

    /// Create a storage bounded to `maxsize` entries.
    ///
    /// Used for the cache tier, matching hivemind's `cache_size`
    /// (`protocol.py` `create(cache_size=...)` → `DHTLocalStorage(maxsize)`).
    pub fn with_maxsize(maxsize: usize) -> Self {
        Self {
            data: BTreeMap::new(),
            maxsize: Some(maxsize),
        }
    }

    /// Number of entries currently held (including not-yet-swept expired ones).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Drop every entry whose outer expiration has passed.
    ///
    /// Python does this lazily inside `_remove_outdated` on each access
    /// (`timed_storage.py:60-68`); we expose it so the owner can also sweep
    /// periodically without needing a request to arrive.
    pub fn remove_outdated(&mut self) {
        let now = get_dht_time();
        self.data.retain(|_, e| e.expiration >= now);
        self.enforce_capacity();
    }

    /// Evict earliest-expiring entries until the size bound is satisfied.
    ///
    /// `timed_storage.py:60-68` pops the expiration heap while
    /// `len(self.data) > self.maxsize`, i.e. it evicts the entry that expires
    /// soonest.
    fn enforce_capacity(&mut self) {
        let Some(maxsize) = self.maxsize else { return };
        while self.data.len() > maxsize {
            let victim = self
                .data
                .iter()
                .min_by(|a, b| {
                    a.1.expiration
                        .partial_cmp(&b.1.expiration)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(k, _)| k.clone());
            match victim {
                Some(k) => {
                    self.data.remove(&k);
                }
                None => break,
            }
        }
    }

    /// Look up a live entry, returning its value and *outer* expiration.
    ///
    /// Expired entries are treated as absent (`timed_storage.py:88-93` sweeps
    /// before reading).
    pub fn get(&self, key: &[u8]) -> Option<(&Stored, DHTExpiration)> {
        let entry = self.data.get(key)?;
        if entry.expiration < get_dht_time() {
            return None;
        }
        Some((&entry.value, entry.expiration))
    }

    /// Store a regular (subkey-less) value.
    ///
    /// Port of `TimedStorage.store` (`timed_storage.py:70-86`):
    /// * a store whose expiration is already in the past is rejected (`:75-76`);
    /// * an existing entry is replaced only if the new expiration is **strictly
    ///   greater** (`:80` uses `<`, so an *equal* expiration is rejected).
    ///
    /// Returns `true` if the value was stored.
    pub fn store(&mut self, key: Vec<u8>, value: Vec<u8>, expiration: DHTExpiration) -> bool {
        if expiration < get_dht_time() {
            return false;
        }
        if let Some(existing) = self.data.get(&key) {
            // Strictly-greater freshness. Equal expirations lose.
            if existing.expiration >= expiration {
                return false;
            }
        }
        self.data.insert(
            key,
            Entry {
                value: Stored::Regular { value, expiration },
                expiration,
            },
        );
        self.enforce_capacity();
        true
    }

    /// Store a single `(subkey, value)` pair into the dictionary under `key`.
    ///
    /// Port of `DHTLocalStorage.store_subkey` (`hivemind/dht/storage.py:51-69`),
    /// which has three cases:
    ///
    /// 1. **No entry, or a regular value with a smaller expiration** → replace it
    ///    with a fresh single-entry dictionary (`:59-63`). Python treats a
    ///    missing entry as `(b"", -inf)`, so absent and stale-regular take the
    ///    same branch.
    /// 2. **Existing dictionary** → refresh the outer expiration if the incoming
    ///    one is later (`:65-66`, note this happens even if the per-subkey store
    ///    below then fails), then delegate to the dictionary's own per-subkey
    ///    freshness check (`:67`).
    /// 3. **Regular value that is at least as fresh** → reject (`:68-69`). A
    ///    subkeyed store cannot clobber a newer regular value.
    ///
    /// Returns `true` if the subkey entry was stored.
    pub fn store_subkey(
        &mut self,
        key: Vec<u8>,
        subkey: Vec<u8>,
        value: Vec<u8>,
        expiration: DHTExpiration,
    ) -> bool {
        // `DictionaryDHTValue.store` delegates to `TimedStorage.store`, which
        // rejects already-expired entries (timed_storage.py:75-76).
        let now = get_dht_time();

        match self.data.get_mut(&key) {
            Some(entry) => {
                match &mut entry.value {
                    // Case 2: existing dictionary.
                    Stored::Dictionary {
                        entries,
                        latest_expiration,
                        maxsize: maxsize_field,
                    } => {
                        // storage.py:65-66 — refresh the *outer* expiration
                        // against the dictionary's latest_expiration_time.
                        if expiration > *latest_expiration {
                            entry.expiration = expiration;
                        }

                        if expiration < now {
                            return false;
                        }

                        // DictionaryDHTValue.store: bump latest_expiration_time
                        // unconditionally (storage.py:17) then apply per-subkey
                        // strictly-greater freshness (timed_storage.py:80).
                        if *latest_expiration < expiration {
                            *latest_expiration = expiration;
                        }
                        match entries.get(&subkey) {
                            Some((_, existing_exp)) if *existing_exp >= expiration => false,
                            _ => {
                                entries.insert(subkey, (value, expiration));
                                bound_dictionary(entries, *maxsize_field);
                                true
                            }
                        }
                    }
                    // Case 1b / 3: entry is a regular binary value.
                    Stored::Regular {
                        expiration: prev_exp,
                        ..
                    } => {
                        if expiration <= *prev_exp || expiration < now {
                            // Case 3 — existing regular value is at least as fresh.
                            return false;
                        }
                        // Case 1 — promote to a dictionary.
                        let mut entries = BTreeMap::new();
                        entries.insert(subkey, (value, expiration));
                        entry.value = Stored::Dictionary {
                            entries,
                            maxsize: self.maxsize.map(|m| m as u64),
                            latest_expiration: expiration,
                        };
                        entry.expiration = expiration;
                        true
                    }
                }
            }
            // Case 1a: no entry at all — Python's `(b"", -inf)` default.
            None => {
                if expiration < now {
                    return false;
                }
                let mut entries = BTreeMap::new();
                entries.insert(subkey, (value, expiration));
                self.data.insert(
                    key,
                    Entry {
                        value: Stored::Dictionary {
                            entries,
                            maxsize: self.maxsize.map(|m| m as u64),
                            latest_expiration: expiration,
                        },
                        expiration,
                    },
                );
                self.enforce_capacity();
                true
            }
        }
    }
}

/// Bound a dictionary's entry count, evicting the earliest-expiring subkeys.
///
/// Hivemind creates inner dictionaries with the tier's own bound
/// (`DictionaryDHTValue(self.maxsize)`, `storage.py:59`) and inherits
/// `TimedStorage`'s eviction. Without this, the tier's entry-count bound is
/// trivially bypassed: one key, unbounded subkeys.
fn bound_dictionary(
    entries: &mut BTreeMap<Vec<u8>, (Vec<u8>, DHTExpiration)>,
    maxsize: Option<u64>,
) {
    let Some(maxsize) = maxsize else { return };
    while entries.len() as u64 > maxsize {
        let Some(victim) = entries
            .iter()
            .min_by(|a, b| a.1 .1.total_cmp(&b.1 .1))
            .map(|(k, _)| k.clone())
        else {
            break;
        };
        entries.remove(&victim);
    }
}

// ============================================================================
// DictionaryDHTValue serialization
// ============================================================================

/// Serialize a dictionary to hivemind's `Ext(0x50, ...)` representation.
///
/// Layout per `DictionaryDHTValue.packb` (`hivemind/dht/storage.py:20-23`):
/// `Ext(0x50, msgpack([maxsize, latest_expiration_time, [[subkey, value, expiration], ...]]))`.
///
/// `maxsize` is emitted as msgpack **nil** when unbounded: Python's `maxsize` is
/// `float("inf")` in that case (`timed_storage.py:55`), and `unpackb` feeds it
/// straight back into `DictionaryDHTValue(maxsize)` where `None`/falsey again
/// becomes `inf`. Values are emitted as msgpack Binary (`use_bin_type=True`,
/// Python `bytes`). Subkeys are NOT: hivemind deserializes the stored subkey
/// (`protocol.py` `rpc_store`: `serializer.loads(tag)`) and `packb` re-encodes
/// the resulting object, so a petals subkey — a peer-id `str` — travels as
/// msgpack **str**. We keep subkeys as their raw msgpack wire bytes internally,
/// so here they are spliced back in as the object those bytes encode. Wrapping
/// them in Binary instead hands a Python reader `bytes` where every other peer
/// yields `str`; hivemind's `add_candidate` merge then compares the two for the
/// same record, raises `TypeError: '<' not supported between 'bytes' and
/// 'str'`, kills the traversal worker, and leaves the caller's `dht.get`
/// future unresolved forever — the health-map crawler freeze of 2026-07-31.
pub fn serialize_dictionary(
    entries: &BTreeMap<Vec<u8>, (Vec<u8>, DHTExpiration)>,
    maxsize: Option<u64>,
    latest_expiration: DHTExpiration,
) -> Vec<u8> {
    use rmpv::Value;

    let packed_items: Vec<Value> = entries
        .iter()
        .map(|(subkey, (value, expiration))| {
            Value::Array(vec![
                subkey_bytes_to_value(subkey),
                Value::Binary(value.clone()),
                Value::F64(*expiration),
            ])
        })
        .collect();

    let inner = Value::Array(vec![
        match maxsize {
            Some(n) => Value::from(n),
            None => Value::Nil,
        },
        Value::F64(latest_expiration),
        Value::Array(packed_items),
    ]);

    let mut inner_bytes = Vec::new();
    rmpv::encode::write_value(&mut inner_bytes, &inner).expect("writing to a Vec cannot fail");

    let mut out = Vec::new();
    rmpv::encode::write_value(&mut out, &Value::Ext(DICTIONARY_EXT_CODE, inner_bytes))
        .expect("writing to a Vec cannot fail");
    out
}

/// Parsed form of a serialized `DictionaryDHTValue`.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedDictionary {
    pub entries: BTreeMap<Vec<u8>, (Vec<u8>, DHTExpiration)>,
    pub maxsize: Option<u64>,
    pub latest_expiration: DHTExpiration,
}

/// Parse hivemind's `Ext(0x50, ...)` dictionary representation.
///
/// Inverse of [`serialize_dictionary`]; mirrors `DictionaryDHTValue.unpackb`
/// (`hivemind/dht/storage.py:25-32`). Returns `None` if `bytes` is not a
/// well-formed dictionary — in particular if it is a plain value or an
/// `Ext(0x40)` tuple.
///
/// Subkeys arrive as whatever msgpack object the storing peer's subkey was —
/// for petals, a peer-id **str** (hivemind stores the deserialized subkey and
/// `packb` re-encodes it). Whatever the type, it is normalized back to its
/// msgpack byte form so subkey identity matches the raw `StoreRequest` tag
/// bytes we key on internally.
pub fn parse_dictionary(bytes: &[u8]) -> Option<ParsedDictionary> {
    use rmpv::Value;

    let outer = rmpv::decode::read_value(&mut &bytes[..]).ok()?;
    let inner_bytes = match &outer {
        Value::Ext(DICTIONARY_EXT_CODE, b) => b.as_slice(),
        _ => return None,
    };

    let inner = rmpv::decode::read_value(&mut &inner_bytes[..]).ok()?;
    let arr = inner.as_array()?;
    if arr.len() < 3 {
        return None;
    }

    // maxsize: nil (unbounded), or a number. Python may also hand back `inf`.
    let maxsize = match &arr[0] {
        Value::Nil => None,
        v => match v.as_u64() {
            Some(n) => Some(n),
            None => match v.as_f64() {
                Some(f) if f.is_finite() && f >= 0.0 => Some(f as u64),
                _ => None,
            },
        },
    };

    let latest_expiration = arr[1].as_f64().unwrap_or(f64::NEG_INFINITY);

    let mut entries = BTreeMap::new();
    for item in arr[2].as_array()? {
        let triple = item.as_array()?;
        if triple.len() < 3 {
            continue;
        }
        let subkey = value_to_subkey_bytes(&triple[0])?;
        let value = match &triple[1] {
            Value::Binary(b) => b.clone(),
            Value::String(s) => s.as_bytes().to_vec(),
            _ => continue,
        };
        let expiration = triple[2].as_f64()?;
        entries.insert(subkey, (value, expiration));
    }

    Some(ParsedDictionary {
        entries,
        maxsize,
        latest_expiration,
    })
}

/// Recover the raw msgpack byte form of a subkey.
///
/// Subkeys are opaque on our side: a Python peer sends `serializer.dumps(subkey)`
/// and we key on exactly those bytes. Binary values pass through; a bare String
/// (which some encoders emit) is re-encoded so it compares equal to the Binary
/// form a Python peer would produce for the same subkey.
fn value_to_subkey_bytes(v: &rmpv::Value) -> Option<Vec<u8>> {
    use rmpv::Value;
    match v {
        Value::Binary(b) => Some(b.clone()),
        other => {
            let mut buf = Vec::new();
            rmpv::encode::write_value(&mut buf, other).ok()?;
            Some(buf)
        }
    }
}

/// Inverse of [`value_to_subkey_bytes`]: splice raw msgpack subkey bytes back
/// in as the msgpack object they encode, so a `str` subkey leaves as a `str`.
///
/// Falls back to Binary for bytes that are not one self-contained msgpack
/// object — that cannot come from a hivemind peer (its subkeys are
/// `serializer.dumps` output by construction), but an opaque passthrough beats
/// corrupting a key we do not understand.
fn subkey_bytes_to_value(raw: &[u8]) -> rmpv::Value {
    let mut cursor = raw;
    match rmpv::decode::read_value(&mut cursor) {
        Ok(v) if cursor.is_empty() => v,
        _ => rmpv::Value::Binary(raw.to_vec()),
    }
}

impl Stored {
    /// Serialize this entry for the `value` field of a `FindResult`.
    ///
    /// Regular values go out as-is; dictionaries are packed into `Ext(0x50, ...)`
    /// exactly as `rpc_find` does via `serializer.dumps(maybe_item.value)`
    /// (`hivemind/dht/protocol.py:354`).
    pub fn to_find_value(&self) -> Vec<u8> {
        match self {
            Self::Regular { value, .. } => value.clone(),
            Self::Dictionary {
                entries,
                maxsize,
                latest_expiration,
            } => serialize_dictionary(entries, *maxsize, *latest_expiration),
        }
    }

    /// The `ResultType` hivemind reports for this entry.
    pub fn result_type(&self) -> crate::protocol::ResultType {
        match self {
            Self::Regular { .. } => crate::protocol::ResultType::FoundRegular,
            Self::Dictionary { .. } => crate::protocol::ResultType::FoundDictionary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn future() -> f64 {
        get_dht_time() + 3600.0
    }

    // ── Golden bytes ────────────────────────────────────────────────────────

    /// `IS_REGULAR_VALUE = serializer.dumps(None)` (`protocol.py:33`) — msgpack
    /// nil, the single byte 0xC0. Guards against a serde/rmp change silently
    /// altering the sentinel and breaking every regular store on the wire.
    #[test]
    fn golden_msgpack_nil_is_single_0xc0_byte() {
        assert_eq!(rmp_serde::to_vec(&Option::<()>::None).unwrap(), vec![0xc0]);
        assert_eq!(crate::server::IS_REGULAR_VALUE, &[0xc0]);
        // IS_DICTIONARY is EMPTY bytes, NOT nil (protocol.py:33).
        assert!(crate::server::IS_DICTIONARY.is_empty());
    }

    /// Round-trip a multi-subkey dictionary and check the concrete header bytes.
    ///
    /// Cross-checked against Python:
    /// `msgpack.dumps(msgpack.ExtType(0x50, msgpack.dumps([None, 1234.5,
    ///  [[b"sk1", b"v1", 1.0], [b"sk2", b"v2", 2.0]]], use_bin_type=True)))`
    /// → `c7325093c0cb40934a0000000000 9293c403736b31c4027631cb3ff0…`
    #[test]
    fn golden_dictionary_ext80_layout() {
        let mut entries = BTreeMap::new();
        entries.insert(b"sk1".to_vec(), (b"v1".to_vec(), 1.0));
        entries.insert(b"sk2".to_vec(), (b"v2".to_vec(), 2.0));

        let bytes = serialize_dictionary(&entries, None, 1234.5);

        // ext8 (0xc7), payload length 0x32, ext code 0x50.
        assert_eq!(&bytes[0..3], &[0xc7, 0x32, 0x50]);
        assert_eq!(
            hex_of(&bytes),
            "c7325093c0cb40934a00000000009293c403736b31c4027631cb3ff0000000\
             00000093c403736b32c4027632cb4000000000000000"
        );

        let parsed = parse_dictionary(&bytes).expect("must parse");
        assert_eq!(parsed.maxsize, None);
        assert_eq!(parsed.latest_expiration, 1234.5);
        assert_eq!(parsed.entries, entries);
    }

    /// The map's DHT crawler (https://github.com/Kwaai-AI-Lab/KwaaiNetMap) decodes live bootstrap
    /// traffic with this exact shape: Ext(80) → [_, _, entries] → per entry
    /// [subkey, Binary(value), _]. Our output must satisfy that reader.
    #[test]
    fn dictionary_matches_crawler_decode_shape() {
        use rmpv::Value;

        let subkey = rmp_serde::to_vec("QmPeerAbc").unwrap();
        let mut entries = BTreeMap::new();
        entries.insert(subkey.clone(), (b"serverinfo".to_vec(), 99.0));

        let bytes = serialize_dictionary(&entries, None, 99.0);

        let outer = rmpv::decode::read_value(&mut &bytes[..]).unwrap();
        let inner_bytes = match &outer {
            Value::Ext(80, b) => b.clone(),
            other => panic!("expected Ext(80), got {other:?}"),
        };
        let inner = rmpv::decode::read_value(&mut &inner_bytes[..]).unwrap();
        let arr = inner.as_array().unwrap();
        assert_eq!(arr.len(), 3);

        let items = arr[2].as_array().unwrap();
        assert_eq!(items.len(), 1);
        let entry = items[0].as_array().unwrap();

        // The subkey travels as the msgpack object its raw bytes encode — a
        // str, exactly as a Python hivemind peer emits it. Emitting Binary
        // here instead hands Python a `bytes` subkey where every other peer
        // yields `str`; hivemind's candidate merge then compares the two and
        // dies (`TypeError: '<' not supported between 'bytes' and 'str'`),
        // hanging every `dht.get` — the 2026-07-31 health-map freeze.
        let sk = match &entry[0] {
            Value::String(s) => s.as_str().unwrap().to_string(),
            other => panic!("subkey must be a msgpack str, got {other:?}"),
        };
        assert_eq!(sk, "QmPeerAbc");

        assert!(matches!(&entry[1], Value::Binary(b) if b == b"serverinfo"));

        // And the round-trip preserves raw-byte subkey identity.
        let parsed = parse_dictionary(&bytes).expect("must parse");
        assert!(parsed.entries.contains_key(&subkey));
    }

    /// Ext(0x40) is the *tuple* code (`serializer.py:27`) — Petals ServerInfo
    /// values arrive that way and must pass through untouched, never being
    /// mistaken for a dictionary.
    #[test]
    fn tuple_ext64_is_not_a_dictionary_and_round_trips_opaquely() {
        use rmpv::Value;

        let mut tuple_bytes = Vec::new();
        rmpv::encode::write_value(
            &mut tuple_bytes,
            &Value::Ext(TUPLE_EXT_CODE, vec![0x93, 0x01, 0x02, 0x03]),
        )
        .unwrap();

        assert!(
            parse_dictionary(&tuple_bytes).is_none(),
            "Ext(64) must not parse as a dictionary"
        );

        // Stored opaquely as a regular value and returned byte-identical.
        let mut s = LocalStorage::new();
        assert!(s.store(b"k".to_vec(), tuple_bytes.clone(), future()));
        let (stored, _) = s.get(b"k").unwrap();
        assert_eq!(stored.to_find_value(), tuple_bytes);
        assert_eq!(
            stored.result_type(),
            crate::protocol::ResultType::FoundRegular
        );
    }

    #[test]
    fn maxsize_encodes_as_nil_when_unbounded_and_number_when_bounded() {
        let entries = BTreeMap::new();

        let unbounded = serialize_dictionary(&entries, None, 1.0);
        assert_eq!(parse_dictionary(&unbounded).unwrap().maxsize, None);

        let bounded = serialize_dictionary(&entries, Some(16), 1.0);
        assert_eq!(parse_dictionary(&bounded).unwrap().maxsize, Some(16));
    }

    // ── Freshness ───────────────────────────────────────────────────────────

    /// `timed_storage.py:80` uses `<`, so an equal expiration is rejected.
    #[test]
    fn regular_store_freshness_is_strictly_greater() {
        let mut s = LocalStorage::new();
        let exp = future();

        assert!(s.store(b"k".to_vec(), b"v1".to_vec(), exp));
        assert!(!s.store(b"k".to_vec(), b"v2".to_vec(), exp), "equal loses");
        assert!(
            !s.store(b"k".to_vec(), b"v3".to_vec(), exp - 10.0),
            "older loses"
        );
        assert!(
            s.store(b"k".to_vec(), b"v4".to_vec(), exp + 10.0),
            "newer wins"
        );

        let (stored, _) = s.get(b"k").unwrap();
        assert_eq!(stored.to_find_value(), b"v4");
    }

    /// `timed_storage.py:75-76` — a store already in the past is refused.
    #[test]
    fn expired_store_is_rejected() {
        let mut s = LocalStorage::new();
        assert!(!s.store(b"k".to_vec(), b"v".to_vec(), 0.0));
        assert!(s.get(b"k").is_none());

        assert!(!s.store_subkey(b"k2".to_vec(), b"sk".to_vec(), b"v".to_vec(), 0.0));
        assert!(s.get(b"k2").is_none());
    }

    // ── Subkey accumulation: THE petals-critical behaviour ───────────────────

    /// Two peers announcing different subkeys under one block key must
    /// ACCUMULATE, not overwrite (`storage.py:51-69`).
    #[test]
    fn two_peers_under_one_key_accumulate() {
        let mut s = LocalStorage::new();
        let key = b"model.block.0".to_vec();

        assert!(s.store_subkey(key.clone(), b"peerA".to_vec(), b"infoA".to_vec(), future()));
        assert!(s.store_subkey(key.clone(), b"peerB".to_vec(), b"infoB".to_vec(), future()));

        let (stored, _) = s.get(&key).unwrap();
        match stored {
            Stored::Dictionary { entries, .. } => {
                assert_eq!(entries.len(), 2, "both peers must be visible");
                assert_eq!(entries[&b"peerA".to_vec()].0, b"infoA");
                assert_eq!(entries[&b"peerB".to_vec()].0, b"infoB");
            }
            other => panic!("expected a dictionary, got {other:?}"),
        }
        assert_eq!(
            stored.result_type(),
            crate::protocol::ResultType::FoundDictionary
        );
    }

    /// Per-subkey freshness is independent and strictly-greater.
    #[test]
    fn subkey_freshness_is_per_subkey_and_strictly_greater() {
        let mut s = LocalStorage::new();
        let key = b"k".to_vec();
        let exp = future();

        assert!(s.store_subkey(key.clone(), b"a".to_vec(), b"a1".to_vec(), exp));
        assert!(
            !s.store_subkey(key.clone(), b"a".to_vec(), b"a2".to_vec(), exp),
            "equal loses"
        );
        assert!(
            !s.store_subkey(key.clone(), b"a".to_vec(), b"a3".to_vec(), exp - 5.0),
            "older loses"
        );
        assert!(s.store_subkey(key.clone(), b"a".to_vec(), b"a4".to_vec(), exp + 5.0));

        // A different subkey is unaffected by subkey "a"'s history.
        assert!(s.store_subkey(key.clone(), b"b".to_vec(), b"b1".to_vec(), exp));

        let (stored, _) = s.get(&key).unwrap();
        match stored {
            Stored::Dictionary { entries, .. } => {
                assert_eq!(entries[&b"a".to_vec()].0, b"a4");
                assert_eq!(entries[&b"b".to_vec()].0, b"b1");
            }
            other => panic!("expected a dictionary, got {other:?}"),
        }
    }

    /// `storage.py:59-63` — a subkeyed store over a *staler* regular value
    /// promotes the entry to a dictionary.
    #[test]
    fn regular_is_promoted_to_dictionary_when_newer_subkey_arrives() {
        let mut s = LocalStorage::new();
        let key = b"k".to_vec();
        let exp = future();

        assert!(s.store(key.clone(), b"plain".to_vec(), exp));
        assert!(matches!(s.get(&key).unwrap().0, Stored::Regular { .. }));

        assert!(s.store_subkey(key.clone(), b"sk".to_vec(), b"v".to_vec(), exp + 10.0));

        let (stored, outer) = s.get(&key).unwrap();
        match stored {
            Stored::Dictionary {
                entries,
                latest_expiration,
                ..
            } => {
                assert_eq!(entries.len(), 1, "promotion drops the old regular value");
                assert_eq!(entries[&b"sk".to_vec()].0, b"v");
                assert_eq!(*latest_expiration, exp + 10.0);
            }
            other => panic!("expected a dictionary, got {other:?}"),
        }
        assert_eq!(outer, exp + 10.0);
    }

    /// `storage.py:68-69` — a subkeyed store cannot clobber a regular value that
    /// is at least as fresh.
    #[test]
    fn subkey_store_rejected_against_fresher_regular_value() {
        let mut s = LocalStorage::new();
        let key = b"k".to_vec();
        let exp = future();

        assert!(s.store(key.clone(), b"plain".to_vec(), exp + 100.0));
        assert!(!s.store_subkey(key.clone(), b"sk".to_vec(), b"v".to_vec(), exp));
        assert!(
            !s.store_subkey(key.clone(), b"sk".to_vec(), b"v".to_vec(), exp + 100.0),
            "equal expiration also loses"
        );

        assert!(matches!(s.get(&key).unwrap().0, Stored::Regular { .. }));
    }

    /// `storage.py:64-66` — the outer expiration is refreshed to the max of
    /// entries, so a dictionary stays alive as long as its longest-lived subkey.
    #[test]
    fn dictionary_outer_expiration_tracks_latest_entry() {
        let mut s = LocalStorage::new();
        let key = b"k".to_vec();
        let early = future();
        let late = early + 500.0;

        assert!(s.store_subkey(key.clone(), b"a".to_vec(), b"v".to_vec(), early));
        assert_eq!(s.get(&key).unwrap().1, early);

        assert!(s.store_subkey(key.clone(), b"b".to_vec(), b"v".to_vec(), late));
        assert_eq!(s.get(&key).unwrap().1, late, "outer refreshed to the max");

        // A shorter-lived subkey must not pull the outer expiration back down.
        assert!(s.store_subkey(key.clone(), b"c".to_vec(), b"v".to_vec(), early + 1.0));
        assert_eq!(s.get(&key).unwrap().1, late);
    }

    // ── Capacity / cleanup ──────────────────────────────────────────────────

    /// `timed_storage.py:60-68` evicts the entry expiring soonest.
    #[test]
    fn capacity_bound_evicts_earliest_expiring() {
        let mut s = LocalStorage::with_maxsize(2);
        let base = future();

        s.store(b"soon".to_vec(), b"v".to_vec(), base + 1.0);
        s.store(b"mid".to_vec(), b"v".to_vec(), base + 100.0);
        s.store(b"late".to_vec(), b"v".to_vec(), base + 1000.0);

        assert_eq!(s.len(), 2);
        assert!(s.get(b"soon").is_none(), "earliest-expiring evicted");
        assert!(s.get(b"mid").is_some());
        assert!(s.get(b"late").is_some());
    }

    #[test]
    fn remove_outdated_drops_expired_entries() {
        let mut s = LocalStorage::new();
        s.store(b"live".to_vec(), b"v".to_vec(), future());
        // Inject an expired entry directly — `store` would refuse it.
        s.data.insert(
            b"dead".to_vec(),
            Entry {
                value: Stored::Regular {
                    value: b"v".to_vec(),
                    expiration: 1.0,
                },
                expiration: 1.0,
            },
        );
        assert_eq!(s.len(), 2);

        s.remove_outdated();

        assert_eq!(s.len(), 1);
        assert!(s.get(b"live").is_some());
        assert!(s.get(b"dead").is_none());
    }

    fn hex_of(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// One key with many subkeys must NOT bypass the tier bound: inner
    /// dictionaries inherit it and evict earliest-expiring subkeys (mirroring
    /// `DictionaryDHTValue(self.maxsize)`, `storage.py:59`).
    #[test]
    fn dictionary_subkeys_respect_the_tier_bound() {
        let mut s = LocalStorage::with_maxsize(4);
        let now = get_dht_time();

        for i in 0u32..10 {
            // Later subkeys expire later, so eviction keeps the newest four.
            assert!(s.store_subkey(
                b"one-key".to_vec(),
                format!("sub-{i:02}").into_bytes(),
                b"v".to_vec(),
                now + 60.0 + f64::from(i),
            ));
        }

        let Some((Stored::Dictionary { entries, .. }, _)) = s.get(b"one-key") else {
            panic!("expected a dictionary");
        };
        assert_eq!(entries.len(), 4, "inner dictionary must be bounded");
        let survivors: Vec<_> = entries.keys().cloned().collect();
        assert_eq!(
            survivors,
            (6u32..10)
                .map(|i| format!("sub-{i:02}").into_bytes())
                .collect::<Vec<_>>(),
            "eviction must drop the earliest-expiring subkeys"
        );
    }
}
