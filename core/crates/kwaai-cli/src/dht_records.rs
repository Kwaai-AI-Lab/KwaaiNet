//! Projecting hivemind DHT records onto the gRPC surface.
//!
//! [`render_value`] is structural and schema-free: it decodes msgpack and shows
//! what is there, never guessing at field names it cannot verify.

use kwaai_hivemind_dht::{DHTExpiration, LocalRecord, Stored};
use kwaai_rpc::v1::{DhtKnownKey, DhtRecord, DhtRecordKind, DhtSubkey, DhtValueView};

use crate::announce::{dht_id, PETALS_MODELS_KEY, VPK_NODES_KEY};
use crate::config::KwaaiNetConfig;

/// Cap on the per-block keys [`known_keys`] derives; the block count is a
/// heuristic when the model is not on disk, so it needs a bound.
const MAX_KNOWN_BLOCK_KEYS: u32 = 1024;

/// Every key this node can name, with the DHT id each hashes to — the reverse
/// table for `key_name`.
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

/// Value bytes one reply may carry. Leaves room under tonic's 4 MiB decode
/// limit for the metadata of up to 5000 rows and the known-key table.
pub const VALUE_BUDGET_BYTES: usize = 3 << 20;

/// Subkeys returned per dictionary, whatever the budget.
pub const MAX_SUBKEYS_PER_RECORD: usize = 256;

/// What is left of a reply's [`VALUE_BUDGET_BYTES`].
#[derive(Debug)]
pub struct ValueBudget {
    remaining: usize,
}

impl Default for ValueBudget {
    fn default() -> Self {
        Self {
            remaining: VALUE_BUDGET_BYTES,
        }
    }
}

impl ValueBudget {
    /// Charge `cost` if it fits; otherwise spend the rest, so no later value
    /// is attached either.
    fn charge(&mut self, cost: usize) -> bool {
        let fits = cost <= self.remaining;
        self.remaining = if fits { self.remaining - cost } else { 0 };
        fits
    }

    fn exhaust(&mut self) {
        self.remaining = 0;
    }
}

/// The part of a stored value a reply can carry, copied out under the
/// storage lock so rendering happens without it.
#[derive(Debug)]
pub enum HeldValue {
    /// `None` when the raw value alone exceeds the remaining budget.
    Regular(Option<Vec<u8>>),
    /// A prefix of the subkeys, and how many the dictionary holds in all.
    Dictionary {
        entries: Vec<(Vec<u8>, Vec<u8>, DHTExpiration)>,
        total: usize,
    },
}

/// Copy as much of `stored` as `budget` could possibly admit: raw size is a
/// lower bound on what [`local_record`] charges, so nothing it would keep is cut.
pub fn hold_value(stored: &Stored, budget: &ValueBudget) -> HeldValue {
    match stored {
        Stored::Regular { value, .. } => {
            HeldValue::Regular((value.len() <= budget.remaining).then(|| value.clone()))
        }
        Stored::Dictionary { entries, .. } => {
            let mut room = budget.remaining;
            let held = entries
                .iter()
                .take(MAX_SUBKEYS_PER_RECORD)
                .map_while(|(subkey, (value, expiration))| {
                    room = room.checked_sub(subkey.len() + value.len())?;
                    Some((subkey.clone(), value.clone(), *expiration))
                })
                .collect();
            HeldValue::Dictionary {
                entries: held,
                total: entries.len(),
            }
        }
    }
}

/// Project one locally-held record. `held` is the value when the caller asked
/// for one; it is rendered only as far as `budget` allows.
pub fn local_record(
    record: &LocalRecord,
    held: Option<HeldValue>,
    key_name: &str,
    now: f64,
    budget: &mut ValueBudget,
) -> DhtRecord {
    let s = &record.summary;
    let kind = match s.subkeys {
        Some(_) => DhtRecordKind::Dictionary,
        None => DhtRecordKind::Regular,
    };

    let mut values_truncated = false;
    let mut value = None;
    let mut subkeys = Vec::new();
    match held {
        Some(HeldValue::Regular(Some(raw))) => {
            let view = render_value(&raw);
            if budget.charge(raw.len().max(view.rendered.len())) {
                value = Some(view);
            } else {
                values_truncated = true;
            }
        }
        Some(HeldValue::Regular(None)) => {
            budget.exhaust();
            values_truncated = true;
        }
        Some(HeldValue::Dictionary { entries, total }) => {
            for (subkey, raw, expiration) in &entries {
                let view = subkey_view(subkey, raw, *expiration, now);
                let rendered =
                    view.subkey.len() + view.value.as_ref().map_or(0, |v| v.rendered.len());
                if !budget.charge((subkey.len() + raw.len()).max(rendered)) {
                    break;
                }
                subkeys.push(view);
            }
            values_truncated = subkeys.len() < total;
            if entries.len() < total.min(MAX_SUBKEYS_PER_RECORD) {
                // The copy stopped on the budget, not the cap: it is spent.
                budget.exhaust();
            }
        }
        None => {}
    }

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
        values_truncated,
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

/// Decode msgpack bytes into a displayable form, falling back to hex: bytes
/// that are not msgpack are a finding, not something to paper over.
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
        rendered: to_json(&value).to_string(),
        raw_bytes: bytes.len() as u64,
        ext_code: match &value {
            rmpv::Value::Ext(code, _) => Some(*code as i32),
            _ => None,
        },
        decoded: true,
    }
}

/// Nesting limit for [`scan`]: a hostile peer must not be able to overflow the
/// stack with depth alone.
const MAX_SCAN_DEPTH: u32 = 64;

/// Decode exactly one msgpack value, requiring well-formed bytes and nothing
/// else in the buffer.
///
/// `rmpv::decode::read_value` is permissive twice over: it calls trailing
/// garbage success, and it decodes the reserved marker `0xc1` to `Nil`
/// (rmpv 1.3.1 `decode/value.rs:203`), so `0xc1` — alone or nested — would
/// render as a clean `null`. [`scan`] catches both.
fn decode_full(bytes: &[u8]) -> Option<rmpv::Value> {
    if scan(bytes, 0, 0)? != bytes.len() {
        return None;
    }
    rmpv::decode::read_value(&mut &bytes[..]).ok()
}

/// Walk one msgpack value from `at`, returning the offset just past it, or
/// `None` if the bytes are malformed or carry the reserved marker. Ext payloads
/// are opaque here; [`to_json`] scans a body it re-decodes.
fn scan(bytes: &[u8], at: usize, depth: u32) -> Option<usize> {
    use rmp::Marker::*;

    if depth > MAX_SCAN_DEPTH {
        return None;
    }
    let marker = rmp::Marker::from_u8(*bytes.get(at)?);
    let at = at + 1;

    // `skip` counts payload bytes after any length prefix; `children` counts
    // nested values, already doubled for maps.
    let (skip, children) = match marker {
        Reserved => return None,
        Null | True | False | FixPos(_) | FixNeg(_) => (0, 0),
        U8 | I8 => (1, 0),
        U16 | I16 => (2, 0),
        U32 | I32 | F32 => (4, 0),
        U64 | I64 | F64 => (8, 0),
        // One type byte ahead of the fixed-width body.
        FixExt1 => (2, 0),
        FixExt2 => (3, 0),
        FixExt4 => (5, 0),
        FixExt8 => (9, 0),
        FixExt16 => (17, 0),
        FixStr(n) => (n as usize, 0),
        FixArray(n) => (0, n as usize),
        FixMap(n) => (0, 2 * n as usize),
        Str8 | Bin8 => return scan_after(bytes, at, 1, 0),
        Str16 | Bin16 => return scan_after(bytes, at, 2, 0),
        Str32 | Bin32 => return scan_after(bytes, at, 4, 0),
        Ext8 => return scan_after(bytes, at, 1, 1),
        Ext16 => return scan_after(bytes, at, 2, 1),
        Ext32 => return scan_after(bytes, at, 4, 1),
        Array16 => return scan_children(bytes, at, 2, 1, depth),
        Array32 => return scan_children(bytes, at, 4, 1, depth),
        Map16 => return scan_children(bytes, at, 2, 2, depth),
        Map32 => return scan_children(bytes, at, 4, 2, depth),
    };

    let mut at = bounded_end(bytes, at, skip)?;
    for _ in 0..children {
        at = scan(bytes, at, depth + 1)?;
    }
    Some(at)
}

/// Skip a `width`-byte big-endian length, then that many payload bytes plus
/// `extra` (the Ext type byte).
fn scan_after(bytes: &[u8], at: usize, width: usize, extra: usize) -> Option<usize> {
    let (len, at) = read_len(bytes, at, width)?;
    bounded_end(bytes, at, len.checked_add(extra)?)
}

/// Skip a `width`-byte big-endian count, then scan that many values, times
/// `per` (2 for a map's key/value pairs).
fn scan_children(bytes: &[u8], at: usize, width: usize, per: usize, depth: u32) -> Option<usize> {
    let (count, mut at) = read_len(bytes, at, width)?;
    for _ in 0..count.checked_mul(per)? {
        at = scan(bytes, at, depth + 1)?;
    }
    Some(at)
}

fn read_len(bytes: &[u8], at: usize, width: usize) -> Option<(usize, usize)> {
    let end = bounded_end(bytes, at, width)?;
    let mut len = 0usize;
    for b in &bytes[at..end] {
        len = len.checked_mul(256)?.checked_add(*b as usize)?;
    }
    Some((len, end))
}

/// `at + n`, rejecting an overflow or a length running past the buffer — which
/// is how a truncated value is caught.
fn bounded_end(bytes: &[u8], at: usize, n: usize) -> Option<usize> {
    let end = at.checked_add(n)?;
    (end <= bytes.len()).then_some(end)
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

/// Encode a decoded msgpack value as JSON.
///
/// Everything JSON cannot say directly is wrapped in a one-key object with a
/// `$`-prefixed tag: `{"$hex": …}` for bytes that are not UTF-8, `{"$float": …}`
/// for NaN/±inf, `{"$ext": code, "value": …}` for an Ext whose body is itself
/// msgpack (Python's tuple encoding) and `{"$ext": code, "$hex": …}` otherwise.
fn to_json(v: &rmpv::Value) -> serde_json::Value {
    use rmpv::Value;
    use serde_json::{json, Value as J};
    match v {
        Value::Nil => J::Null,
        Value::Boolean(b) => J::Bool(*b),
        Value::Integer(i) => match (i.as_i64(), i.as_u64()) {
            (Some(n), _) => json!(n),
            (None, Some(n)) => json!(n),
            (None, None) => json!({ "$float": i.as_f64() }),
        },
        Value::F32(f) => float_json(f64::from(*f)),
        Value::F64(f) => float_json(*f),
        Value::String(s) => match s.as_str() {
            Some(text) => J::String(text.to_string()),
            None => hex_json(s.as_bytes()),
        },
        Value::Binary(b) => match std::str::from_utf8(b) {
            Ok(text) => J::String(text.to_string()),
            Err(_) => hex_json(b),
        },
        Value::Array(items) => J::Array(items.iter().map(to_json).collect()),
        // A non-string key becomes its compact JSON text; duplicates after that
        // collapse, last wins.
        Value::Map(pairs) => J::Object(
            pairs
                .iter()
                .map(|(k, v)| {
                    let key = match to_json(k) {
                        J::String(text) => text,
                        other => other.to_string(),
                    };
                    (key, to_json(v))
                })
                .collect(),
        ),
        Value::Ext(code, body) => match decode_full(body) {
            Some(inner) => json!({ "$ext": code, "value": to_json(&inner) }),
            None => json!({ "$ext": code, "$hex": hex::encode(body) }),
        },
    }
}

fn float_json(f: f64) -> serde_json::Value {
    use serde_json::json;
    if f.is_finite() {
        json!(f)
    } else if f.is_nan() {
        json!({ "$float": "nan" })
    } else if f > 0.0 {
        json!({ "$float": "inf" })
    } else {
        json!({ "$float": "-inf" })
    }
}

fn hex_json(bytes: &[u8]) -> serde_json::Value {
    serde_json::json!({ "$hex": hex::encode(bytes) })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unwrapping a `ServerInfo`'s `Ext(64)` body is structural — no field
    /// names are invented — and the code stays visible, so a reader is not
    /// shown a bare array.
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
        assert_eq!(
            view.rendered,
            r#"{"$ext":64,"value":[2,1.5,{"start_block":0}]}"#
        );
        assert_json(&view);
    }

    /// Every `decoded` rendering must parse as JSON — that is the contract a
    /// client's `jsonDecode` relies on.
    fn assert_json(view: &DhtValueView) -> serde_json::Value {
        assert!(view.decoded);
        serde_json::from_str(&view.rendered)
            .unwrap_or_else(|e| panic!("not JSON: {e}: {}", view.rendered))
    }

    /// Everything JSON cannot say directly, each in its tagged form.
    #[test]
    fn json_escape_hatches_are_tagged_objects() {
        use rmpv::Value;
        let enc = |v: &Value| {
            let mut b = Vec::new();
            rmpv::encode::write_value(&mut b, v).unwrap();
            b
        };

        // A non-string map key becomes its compact JSON text.
        let map = Value::Map(vec![
            (Value::Integer(1.into()), Value::Boolean(true)),
            (
                Value::Array(vec![Value::Integer(1.into()), Value::Integer(2.into())]),
                Value::Nil,
            ),
        ]);
        let view = render_value(&enc(&map));
        assert_eq!(view.rendered, r#"{"1":true,"[1,2]":null}"#);
        assert_json(&view);

        // A msgpack str that is not UTF-8, and a NaN.
        let mut bad_str = vec![0xa2, 0xff, 0xfe];
        bad_str.extend(enc(&Value::F64(f64::NAN)));
        let arr = [0x92].into_iter().chain(bad_str).collect::<Vec<u8>>();
        let view = render_value(&arr);
        assert_eq!(view.rendered, r#"[{"$hex":"fffe"},{"$float":"nan"}]"#);
        assert_json(&view);

        let inf = render_value(&enc(&Value::Array(vec![
            Value::F64(f64::INFINITY),
            Value::F64(f64::NEG_INFINITY),
            Value::F32(0.5),
        ])));
        assert_eq!(inf.rendered, r#"[{"$float":"inf"},{"$float":"-inf"},0.5]"#);
        assert_json(&inf);

        // A nested ext inside an array, and one with a non-msgpack body.
        let nested = Value::Array(vec![
            Value::Ext(64, enc(&Value::Integer(7.into()))),
            Value::Ext(99, vec![0x92, 0x01]),
        ]);
        let view = render_value(&enc(&nested));
        assert_eq!(
            view.rendered,
            r#"[{"$ext":64,"value":7},{"$ext":99,"$hex":"9201"}]"#
        );
        assert_eq!(view.msgpack_type, "array");
        assert_eq!(view.ext_code, None);
        assert_json(&view);
    }

    /// An Ext body that is not msgpack must stay opaque — guessing would turn
    /// bytes from an unknown peer into text that looks decoded.
    #[test]
    fn an_undecodable_ext_body_stays_hex_under_its_code() {
        use rmpv::Value;
        // A 2-element array header supplying one element: truncated.
        let mut bytes = Vec::new();
        rmpv::encode::write_value(&mut bytes, &Value::Ext(99, vec![0x92, 0x01])).unwrap();

        let view = render_value(&bytes);
        assert_eq!(view.ext_code, Some(99));
        assert_eq!(view.rendered, r#"{"$ext":99,"$hex":"9201"}"#);
        assert_json(&view);
    }

    /// The reserved marker 0xc1, at every position it can hide in.
    ///
    /// rmpv decodes it to `Nil` rather than failing (1.3.1
    /// `decode/value.rs:203`), so each of these rendered as a clean `null` with
    /// `decoded: true` until [`scan`] went in. Trailing-byte checking alone
    /// caught only the last one.
    #[test]
    fn the_reserved_marker_is_never_rendered_as_null() {
        for (bytes, what) in [
            (vec![0xc1u8], "bare"),
            (vec![0x91, 0xc1], "inside an array"),
            (vec![0x81, 0xc1, 0xc1], "as a map key and value"),
            (vec![0xc1, 0xc1], "with trailing bytes"),
        ] {
            let view = render_value(&bytes);
            assert!(!view.decoded, "0xc1 {what} must not decode");
            assert_eq!(view.msgpack_type, "undecodable", "0xc1 {what}");
            assert_eq!(view.rendered, hex::encode(&bytes), "0xc1 {what}");
        }

        // And inside an Ext body, which `to_json` re-decodes separately.
        let mut ext = Vec::new();
        rmpv::encode::write_value(&mut ext, &rmpv::Value::Ext(64, vec![0xc1])).unwrap();
        assert_eq!(render_value(&ext).rendered, r#"{"$ext":64,"$hex":"c1"}"#);
    }

    /// 0xc1 inside a str or bin *payload* is data, not a marker. A byte scan
    /// would reject these; the marker walk must not.
    #[test]
    fn a_reserved_byte_inside_a_payload_is_just_data() {
        use rmpv::Value;
        let mut bin = Vec::new();
        rmpv::encode::write_value(&mut bin, &Value::Binary(vec![0xc1, 0xc1])).unwrap();
        let view = render_value(&bin);
        assert!(view.decoded, "0xc1 in a bin payload is a legitimate byte");
        assert_eq!(view.rendered, r#"{"$hex":"c1c1"}"#);
        assert_json(&view);

        // The shape every announce actually uses, for good measure.
        let packed = rmp_serde::to_vec("12D3KooWExample").unwrap();
        assert!(render_value(&packed).decoded);
    }

    /// Hostile shapes must fail, not panic: an oversized declared length and
    /// nesting past [`MAX_SCAN_DEPTH`] both come from bytes a peer chooses.
    #[test]
    fn malformed_input_is_rejected_rather_than_trusted() {
        // bin32 claiming 4 GiB of payload it does not have.
        assert!(!render_value(&[0xc6, 0xff, 0xff, 0xff, 0xff]).decoded);

        // 200 nested single-element arrays, past the depth limit.
        let mut deep = vec![0x91u8; 200];
        deep.push(0xc0);
        assert!(!render_value(&deep).decoded);

        // Just inside it, the same shape is fine.
        let mut shallow = vec![0x91u8; 32];
        shallow.push(0xc0);
        assert!(render_value(&shallow).decoded);
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

    /// Binary renders as a string only when it really is UTF-8, so a textual
    /// subkey and a binary one are never confused.
    #[test]
    fn binary_renders_as_text_only_when_it_is_text() {
        use rmpv::Value;
        let text = {
            let mut b = Vec::new();
            rmpv::encode::write_value(&mut b, &Value::Binary(b"12D3KooW".to_vec())).unwrap();
            b
        };
        assert_eq!(render_value(&text).rendered, r#""12D3KooW""#);
        assert_json(&render_value(&text));

        let raw = {
            let mut b = Vec::new();
            rmpv::encode::write_value(&mut b, &Value::Binary(vec![0xff, 0xfe])).unwrap();
            b
        };
        assert_eq!(render_value(&raw).rendered, r#"{"$hex":"fffe"}"#);
        assert_json(&render_value(&raw));
    }

    /// The ids must be the same `SHA1(msgpack(name))` the announce path
    /// publishes under, or every row comes back unnamed.
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
