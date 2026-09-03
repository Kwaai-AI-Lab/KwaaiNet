//! Projecting hivemind DHT records onto the gRPC surface.
//!
//! Two sources feed the same [`DhtRecord`] shape, which is the point: a record
//! this node happens to hold and one a remote peer returned should read
//! identically, so a client can put them side by side.
//!
//! # What can and cannot be listed
//!
//! Kademlia has no key-enumeration operation. A node can be asked for a key; it
//! cannot be asked what keys it has, and there is no operation at all that means
//! "every key in the DHT". So the listing here is the *local store* — complete
//! and cheap, but a statement about this node, not about the network — while
//! anything network-wide has to name a key up front. [`known_keys`] is what
//! makes that usable: the registries every node publishes under, plus the
//! per-block keys derived from the configured model prefix.
//!
//! # Rendering
//!
//! [`render_value`] decodes msgpack and shows what is there. It deliberately
//! knows no schemas: a petals `ServerInfo` is an `Ext(64)` wrapping a msgpack
//! tuple, so it renders as its contents rather than as named fields we would
//! have to guess at, and an Ext whose body is *not* msgpack is reported as
//! opaque bytes under its type code rather than mangled into text.

use kwaai_hivemind_dht::{DHTExpiration, LocalRecord, ParsedDictionary, Stored};
use kwaai_rpc::v1::{DhtKnownKey, DhtRecord, DhtRecordKind, DhtSubkey, DhtValueView};

use crate::announce::{dht_id, PETALS_MODELS_KEY, VPK_NODES_KEY};
use crate::config::KwaaiNetConfig;

/// Cap on the per-block keys [`known_keys`] derives.
///
/// The block count comes from the model's `config.json` when it is on disk and
/// from a name heuristic when it is not, so a misconfigured model could ask for
/// an unbounded list. No published model is near this.
const MAX_KNOWN_BLOCK_KEYS: u32 = 1024;

/// Every key this node can name, with the DHT id each hashes to.
///
/// Serves two jobs at once: the reverse table for [`DhtRecord::key_name`] — a
/// DHT id is `SHA1(msgpack(name))` and cannot be inverted — and the candidate
/// list a client offers for a network lookup.
pub fn known_keys(config: &KwaaiNetConfig) -> Vec<DhtKnownKey> {
    let prefix = config.effective_dht_prefix();
    let blocks = config.model_total_blocks().max(0) as u32;

    let mut names = vec![
        PETALS_MODELS_KEY.to_string(),
        crate::shard_cmd::INFERENCE_NODES_DHT_KEY.to_string(),
        VPK_NODES_KEY.to_string(),
    ];
    names.extend((0..blocks.min(MAX_KNOWN_BLOCK_KEYS)).map(|b| format!("{prefix}.{b}")));

    names
        .into_iter()
        .map(|name| DhtKnownKey {
            key_id: hex::encode(dht_id(&name)),
            name,
        })
        .collect()
}

/// Project one locally-held record. `value` is filled from `stored` when the
/// caller asked for it; the metadata alone needs no value read.
pub fn local_record(
    record: &LocalRecord,
    stored: Option<&Stored>,
    key_name: &str,
    now: f64,
) -> DhtRecord {
    let s = &record.summary;
    let kind = match s.subkeys {
        Some(_) => DhtRecordKind::Dictionary,
        None => DhtRecordKind::Regular,
    };

    let (value, subkeys) = match stored {
        Some(Stored::Regular { value, .. }) => (Some(render_value(value)), Vec::new()),
        Some(Stored::Dictionary { entries, .. }) => (
            None,
            entries
                .iter()
                .map(|(subkey, (value, expiration))| subkey_view(subkey, value, *expiration, now))
                .collect(),
        ),
        None => (None, Vec::new()),
    };

    DhtRecord {
        key_id: hex::encode(&s.key),
        key_name: key_name.to_string(),
        kind: kind as i32,
        in_cache: record.in_cache,
        subkey_count: s.subkeys.unwrap_or(0) as u32,
        value_bytes: s.value_bytes as u64,
        expiration: s.expiration,
        expires_in_secs: s.expiration - now,
        latest_expiration: s.latest_expiration,
        dict_maxsize: s.maxsize,
        value,
        subkeys,
    }
}

/// Project a `FindResult` value returned by a remote peer.
///
/// The remote gives us the serialized value and the outer expiration and
/// nothing else, so the shape has to be recovered by parsing: a value that
/// parses as hivemind's `Ext(0x50)` is a dictionary, anything else is regular.
/// That is the same test `handle_store` applies on the receiving side.
pub fn remote_record(
    key: &str,
    key_id: &[u8],
    value: &[u8],
    expiration: DHTExpiration,
    now: f64,
) -> DhtRecord {
    let dict = kwaai_hivemind_dht::parse_dictionary(value);

    let base = DhtRecord {
        key_id: hex::encode(key_id),
        key_name: key.to_string(),
        expiration,
        expires_in_secs: expiration - now,
        ..Default::default()
    };

    match dict {
        Some(ParsedDictionary {
            entries,
            maxsize,
            latest_expiration,
        }) => DhtRecord {
            kind: DhtRecordKind::Dictionary as i32,
            subkey_count: entries.len() as u32,
            value_bytes: entries.values().map(|(v, _)| v.len() as u64).sum(),
            latest_expiration: Some(latest_expiration),
            dict_maxsize: maxsize,
            subkeys: entries
                .iter()
                .map(|(subkey, (v, exp))| subkey_view(subkey, v, *exp, now))
                .collect(),
            ..base
        },
        None => DhtRecord {
            kind: DhtRecordKind::Regular as i32,
            value_bytes: value.len() as u64,
            value: Some(render_value(value)),
            ..base
        },
    }
}

fn subkey_view(subkey: &[u8], value: &[u8], expiration: DHTExpiration, now: f64) -> DhtSubkey {
    DhtSubkey {
        subkey: render_value(subkey).rendered,
        expiration,
        expires_in_secs: expiration - now,
        value_bytes: value.len() as u64,
        value: Some(render_value(value)),
    }
}

/// Decode msgpack bytes into a displayable form.
///
/// Falls back to hex rather than to a lossy string: a value that is not msgpack
/// is a genuine finding about the record, and printing its bytes as replacement
/// characters would hide it.
pub fn render_value(bytes: &[u8]) -> DhtValueView {
    let Some(value) = decode_full(bytes) else {
        return DhtValueView {
            msgpack_type: "undecodable".to_string(),
            rendered: hex::encode(bytes),
            raw_bytes: bytes.len() as u64,
            ext_code: None,
            decoded: false,
        };
    };

    DhtValueView {
        msgpack_type: type_name(&value).to_string(),
        rendered: render(&value),
        raw_bytes: bytes.len() as u64,
        ext_code: match &value {
            rmpv::Value::Ext(code, _) => Some(*code as i32),
            _ => None,
        },
        decoded: true,
    }
}

/// Decode exactly one msgpack value, requiring it to consume every byte.
///
/// `rmpv::decode::read_value` stops at the end of the first value and reports
/// success on trailing garbage, so a record with junk appended would render as
/// though it were clean. Full consumption is the check that catches it.
fn decode_full(bytes: &[u8]) -> Option<rmpv::Value> {
    let mut cursor = bytes;
    let value = rmpv::decode::read_value(&mut cursor).ok()?;
    cursor.is_empty().then_some(value)
}

fn type_name(v: &rmpv::Value) -> &'static str {
    use rmpv::Value;
    match v {
        Value::Nil => "nil",
        Value::Boolean(_) => "bool",
        Value::Integer(_) => "int",
        Value::F32(_) | Value::F64(_) => "float",
        Value::String(_) => "str",
        Value::Binary(_) => "bin",
        Value::Array(_) => "array",
        Value::Map(_) => "map",
        Value::Ext(_, _) => "ext",
    }
}

/// One-line JSON-ish rendering of a decoded msgpack value.
///
/// An Ext body is re-decoded when it is itself msgpack, which is what Python's
/// tuple encoding (`Ext(0x40, msgpack([...]))`) and hivemind's dictionary
/// encoding both produce — so a `ServerInfo` shows its contents without this
/// having to know its field names. A body that is not msgpack stays hex.
fn render(v: &rmpv::Value) -> String {
    use rmpv::Value;
    match v {
        Value::Nil => "null".to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::F32(f) => f.to_string(),
        Value::F64(f) => f.to_string(),
        Value::String(s) => match s.as_str() {
            Some(s) => format!("{s:?}"),
            None => format!("hex:{}", hex::encode(s.as_bytes())),
        },
        Value::Binary(b) => match std::str::from_utf8(b) {
            Ok(s) => format!("{s:?}"),
            Err(_) => format!("hex:{}", hex::encode(b)),
        },
        Value::Array(items) => {
            let inner: Vec<String> = items.iter().map(render).collect();
            format!("[{}]", inner.join(", "))
        }
        Value::Map(pairs) => {
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("{}: {}", render(k), render(v)))
                .collect();
            format!("{{{}}}", inner.join(", "))
        }
        Value::Ext(code, body) => match decode_full(body) {
            Some(inner) => format!("ext({code}){}", render(&inner)),
            None => format!("ext({code})hex:{}", hex::encode(body)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A petals `ServerInfo` is `Ext(64, msgpack([state, throughput, {…}]))`.
    /// Unwrapping the Ext body is structural — no field names are invented —
    /// and the type code stays visible so a reader knows what they are looking
    /// at rather than seeing a bare array.
    #[test]
    fn ext64_server_info_renders_as_its_contents_under_its_code() {
        use rmpv::Value;
        let inner = Value::Array(vec![
            Value::Integer(2.into()),
            Value::F64(1.5),
            Value::Map(vec![(
                Value::String("start_block".into()),
                Value::Integer(0.into()),
            )]),
        ]);
        let mut inner_bytes = Vec::new();
        rmpv::encode::write_value(&mut inner_bytes, &inner).unwrap();
        let mut bytes = Vec::new();
        rmpv::encode::write_value(&mut bytes, &Value::Ext(64, inner_bytes)).unwrap();

        let view = render_value(&bytes);
        assert_eq!(view.msgpack_type, "ext");
        assert_eq!(view.ext_code, Some(64));
        assert!(view.decoded);
        assert_eq!(view.rendered, r#"ext(64)[2, 1.5, {"start_block": 0}]"#);
    }

    /// An Ext body that is not msgpack must stay opaque. Guessing at it would
    /// turn arbitrary bytes from an unknown peer into text that looks decoded.
    #[test]
    fn an_undecodable_ext_body_stays_hex_under_its_code() {
        use rmpv::Value;
        // 0x92 declares a 2-element array and then supplies one — truncated,
        // not merely unrecognised.
        let mut bytes = Vec::new();
        rmpv::encode::write_value(&mut bytes, &Value::Ext(99, vec![0x92, 0x01])).unwrap();

        let view = render_value(&bytes);
        assert_eq!(view.ext_code, Some(99));
        assert_eq!(view.rendered, "ext(99)hex:9201");
    }

    /// Trailing bytes after a complete value make the whole thing suspect.
    ///
    /// `read_value` stops at the end of the first value and calls that success,
    /// so without the full-consumption check a record with junk appended would
    /// render as a clean `null` and the junk would never be seen.
    #[test]
    fn trailing_bytes_are_not_silently_dropped() {
        let view = render_value(&[0xc0, 0xc0]);
        assert!(!view.decoded);
        assert_eq!(view.rendered, "c0c0");
    }

    /// Bytes that are not msgpack at all are a finding about the record, not a
    /// reason to print replacement characters.
    #[test]
    fn non_msgpack_bytes_render_as_hex_and_say_so() {
        // A 2-element array header with nothing following it.
        let view = render_value(&[0x92]);
        assert!(!view.decoded);
        assert_eq!(view.msgpack_type, "undecodable");
        assert_eq!(view.rendered, "92");
    }

    /// Binary is rendered as a string only when it really is UTF-8; otherwise
    /// hex, so a subkey that happens to be text and one that is not can never
    /// be confused for each other.
    #[test]
    fn binary_renders_as_text_only_when_it_is_text() {
        use rmpv::Value;
        let text = {
            let mut b = Vec::new();
            rmpv::encode::write_value(&mut b, &Value::Binary(b"12D3KooW".to_vec())).unwrap();
            b
        };
        assert_eq!(render_value(&text).rendered, r#""12D3KooW""#);

        let raw = {
            let mut b = Vec::new();
            rmpv::encode::write_value(&mut b, &Value::Binary(vec![0xff, 0xfe])).unwrap();
            b
        };
        assert_eq!(render_value(&raw).rendered, "hex:fffe");
    }

    /// A dictionary returned by a remote peer arrives as opaque bytes: the
    /// `FindResult` carries no shape flag we trust, so the shape is recovered
    /// by parsing, and each subkey keeps its own expiration.
    #[test]
    fn a_remote_dictionary_is_recovered_from_its_bytes() {
        use std::collections::BTreeMap;
        let mut entries = BTreeMap::new();
        entries.insert(
            rmp_serde::to_vec("peer-a").unwrap(),
            (b"va".to_vec(), 100.0),
        );
        entries.insert(
            rmp_serde::to_vec("peer-b").unwrap(),
            (b"vbvb".to_vec(), 200.0),
        );
        let bytes = kwaai_hivemind_dht::serialize_dictionary(&entries, Some(20), 200.0);

        let rec = remote_record(
            "_petals.models",
            &dht_id("_petals.models"),
            &bytes,
            150.0,
            50.0,
        );
        assert_eq!(rec.kind, DhtRecordKind::Dictionary as i32);
        assert_eq!(rec.subkey_count, 2);
        assert_eq!(rec.value_bytes, 6);
        assert_eq!(rec.dict_maxsize, Some(20));
        assert_eq!(rec.latest_expiration, Some(200.0));
        assert_eq!(rec.expires_in_secs, 100.0);
        // The record's own value slot stays empty — a dictionary's values are
        // per-subkey, and filling both would show them twice.
        assert!(rec.value.is_none());
        assert_eq!(rec.subkeys[0].subkey, r#""peer-a""#);
        assert_eq!(rec.subkeys[1].expiration, 200.0);
        assert_eq!(rec.subkeys[1].value_bytes, 4);
    }

    /// A regular value under the same key is not a dictionary with one entry —
    /// it is a peer that overwrote every other announcer, which is exactly the
    /// failure a reader is looking for.
    #[test]
    fn a_remote_regular_value_is_not_mistaken_for_a_dictionary() {
        let value = rmp_serde::to_vec("solo").unwrap();
        let rec = remote_record("k", &dht_id("k"), &value, 10.0, 0.0);
        assert_eq!(rec.kind, DhtRecordKind::Regular as i32);
        assert_eq!(rec.subkey_count, 0);
        assert!(rec.subkeys.is_empty());
        assert_eq!(
            rec.value.expect("regular values are rendered").rendered,
            r#""solo""#
        );
    }

    /// The registries are always nameable; block keys follow the node's own
    /// prefix, and the ids must be the same `SHA1(msgpack(name))` the announce
    /// path publishes under — otherwise every row would come back unnamed.
    #[test]
    fn known_keys_cover_the_registries_and_agree_with_announce() {
        let config = KwaaiNetConfig::default();
        let keys = known_keys(&config);

        let by_name = |n: &str| keys.iter().find(|k| k.name == n).cloned();
        for name in [
            PETALS_MODELS_KEY,
            VPK_NODES_KEY,
            crate::shard_cmd::INFERENCE_NODES_DHT_KEY,
        ] {
            let found = by_name(name).unwrap_or_else(|| panic!("{name} must be nameable"));
            assert_eq!(found.key_id, hex::encode(dht_id(name)));
        }

        let block0 = format!("{}.0", config.effective_dht_prefix());
        assert!(by_name(&block0).is_some(), "block keys must be nameable");
    }
}
