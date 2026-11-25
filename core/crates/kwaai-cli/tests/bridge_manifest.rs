//! Conformance tests for the open-hives/0.1 bridge manifest.
//!
//! The manifest at `docs/bridge/hivv.json` is the contract between KwaaiNet
//! and every peer hive (hivv.org first among them). It is fetched by the peer
//! over HTTP and trusted: if it advertises an endpoint that does not work, or
//! an auth scheme that does not exist, the peer has no way to tell the
//! difference until a request fails in production.
//!
//! These tests pin the properties a peer is entitled to assume. They are
//! cheap, offline, and read the manifest straight off disk.
//!
//! The regression this exists for: the manifest once advertised
//! `chat: http://192.168.1.209:8080/v1/chat/completions` — a LAN address on
//! a contributor's box. It leaked internal topology, it could not work from
//! any host outside that LAN (hivv.org's own SSRF guard refuses private
//! addresses), and it advertised `"auth": "per-node bearer"` for a server
//! that implemented no bearer check at all. A manifest is a promise; this
//! file is the promise-keeper.

use std::path::PathBuf;

use serde_json::Value;

/// The repo root, found by walking up from this crate's manifest dir.
fn repo_root() -> PathBuf {
    // core/crates/kwaai-cli -> core/crates -> core -> <repo root>
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("crate lives at core/crates/<name>")
        .to_path_buf()
}

fn manifest_path() -> PathBuf {
    repo_root().join("docs/bridge/hivv.json")
}

fn manifest() -> Value {
    let path = manifest_path();
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("{} is not valid JSON: {e}", path.display()))
}

/// Every string value anywhere in the manifest.
fn all_strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) => out.push(s.clone()),
        Value::Array(a) => a.iter().for_each(|x| all_strings(x, out)),
        Value::Object(o) => o.values().for_each(|x| all_strings(x, out)),
        _ => {}
    }
}

#[test]
fn manifest_is_valid_json_with_the_required_shape() {
    let m = manifest();
    assert_eq!(
        m["protocol"].as_str().unwrap_or_default(),
        "open-hives/0.1",
        "protocol must be exactly open-hives/0.1"
    );
    assert_eq!(m["hive"].as_str(), Some("kwaai"));
    assert!(m["status"].as_str().is_some(), "status must be present");
    assert!(
        m["human"]
            .as_str()
            .unwrap_or_default()
            .starts_with("https://"),
        "human must be an https URL"
    );
    assert!(m["endpoints"].is_object(), "endpoints must be an object");
    assert!(
        m["endpoints"].get("allowance").is_some()
            || m["endpoints"].get("load").is_some()
            || m["endpoints"].get("state").is_some(),
        "endpoints must publish at least one load/allowance/state signal"
    );
}

#[test]
fn endpoints_chat_is_a_public_https_url() {
    // The doorbell rule (open-hives/0.1 §12.1): a hive WITHOUT a usable
    // endpoints.chat is "visible but silent". A peer reads this field and
    // POSTs to it directly, so it has to be an absolute http(s) URL or the
    // peer refuses to ring it at all.
    let m = manifest();
    let chat = m["endpoints"]["chat"].as_str().unwrap_or_default();
    assert!(
        chat.starts_with("https://"),
        "endpoints.chat must be an https URL (open-hives/0.1 §12.1), got: {chat:?}"
    );
    assert!(
        !chat.contains(' '),
        "endpoints.chat must be a URL, not a description: {chat:?}"
    );
}

#[test]
fn no_private_address_is_published_anywhere_in_the_manifest() {
    // A private address in a public manifest leaks internal topology and
    // cannot be reached from outside the LAN that owns it — hivv.org's SSRF
    // guard (api/lib/ssrf.js) refuses exactly these ranges.
    let m = manifest();
    let mut strings = Vec::new();
    all_strings(&m, &mut strings);

    // MIRROR: src/hive.rs carries the same list (as ADDRESS_MARKERS /
    // HOSTNAME_MARKERS) when it validates a PEER's manifest — and there it
    // is backed by real IPv4/IPv6 parsing, because the client acts on those
    // values. Keep the two in sync.
    let forbidden = [
        "192.168.",
        "10.0.",
        "127.0.0.1",
        "localhost",
        "0.0.0.0",
        "169.254.",
        ".local",
        ".internal",
    ];
    for s in &strings {
        for f in forbidden {
            assert!(
                !s.contains(f),
                "manifest publishes a private address ({f:?}) in: {s:?} — \
                 a peer outside this LAN can never reach it"
            );
        }
    }
}

#[test]
fn auth_claim_matches_what_the_server_actually_implements() {
    // The manifest used to claim "per-node bearer (OpenAI-compatible)" for a
    // server with no bearer check anywhere in shard_api.rs. A peer that
    // trusted that claim would send a token and believe it was authenticated
    // when it was not. Only claim an auth scheme that exists.
    let m = manifest();
    let scheme = m["auth"]["scheme"].as_str().unwrap_or_default();

    let implemented = ["bearer-optional", "none", "anonymous"];
    assert!(
        implemented.contains(&scheme),
        "auth.scheme {scheme:?} is not one of the implemented schemes {implemented:?} — \
         either implement it or stop advertising it"
    );
}

#[test]
fn the_manifest_has_a_single_source_of_truth_note() {
    // Two copies of this file once drifted apart: the copy in the
    // KwaaiNetMap repo advertised a dynamic discovery string (not a valid
    // doorbell), while the copy here advertised a LAN URL. Only the second
    // was ever fetched, so only the second mattered — and it was wrong.
    // The discovery note must record that this file is the source.
    let m = manifest();
    let discovery = m["discovery"]
        .as_object()
        .expect("discovery must be an object");
    assert!(
        discovery.contains_key("note"),
        "discovery.note must record which copy of this manifest is authoritative"
    );
    assert!(
        discovery.contains_key("state"),
        "discovery.state must point at the endpoint that resolves the doorbell"
    );
}

#[test]
fn limits_are_present_and_bounded() {
    let m = manifest();
    let limits = m["limits"].as_object().expect("limits must be an object");
    assert!(
        limits.contains_key("allowance_max_percent"),
        "limits.allowance_max_percent must be published"
    );
    let pct = limits["allowance_max_percent"]
        .as_u64()
        .expect("must be a number");
    assert!(
        (0..=100).contains(&pct),
        "allowance_max_percent must be a percentage, got {pct}"
    );
    assert!(
        limits.contains_key("rate_per_min"),
        "limits.rate_per_min must be published so a peer can budget"
    );
}
