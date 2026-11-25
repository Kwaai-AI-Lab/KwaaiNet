//! `kwaainet hive` — the open-hives/0.1 doorbell client.
//!
//! A hive is a whole network of nodes behind one domain; the doorbell is the
//! single `endpoints.chat` URL its discovery manifest publishes. Ringing it
//! does not name a node — the peer hive resolves the request across its own
//! nodes, so the door keeps working when individual machines go down.
//!
//! This module is the kwaai-side mirror of hivv.org's `hivv ask` flow
//! (`public_html/hivv-cli.py::_ask_hive`): resolve a brand or host, try the
//! three manifest doors, validate what comes back against the same rules our
//! own manifest is held to (`tests/bridge_manifest.rs`), then ring.
//!
//! The validation rules are deliberately shared with our own conformance
//! tests: a manifest is a promise, and both sides of the bridge are held to
//! the same promise. The regression this exists for: our manifest once
//! advertised `http://192.168.1.209:8080` as the doorbell — a LAN address no
//! peer outside that network could ever reach.

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The hive brand we identify as when ringing a peer (bearer + loop shield).
pub const HIVE_ORIGIN: &str = "kwaai";

/// Manifest doors, tried in order — the same three hivv.org's client tries.
/// `.json` under `/.bridge` is the canonical location, `/.well-known` is the
/// IETF-standard one, `hive.json` is the legacy name early peers published.
const DOORS: &[&str] = &[
    "/.bridge/hivv.json",
    "/.well-known/hivv.json",
    "/.well-known/hive.json",
];

/// The brand → host registry published by hivv.org. Consulted only when the
/// target is a bare word ("hivv"); a dotted target is treated as a host, and
/// an explicit URL is fetched directly.
const BRANDS_REGISTRY: &str = "https://hivv.org/brands.json";

/// Every string value in a manifest is checked against these: a private
/// address in a public manifest leaks internal topology and cannot be reached
/// from outside the LAN that owns it — hivv.org's SSRF guard (api/lib/ssrf.js)
/// refuses exactly these ranges, so a peer would too.
const FORBIDDEN_ADDRESS_MARKERS: &[&str] = &[
    "192.168.",
    "10.0.",
    "127.0.0.1",
    "localhost",
    "0.0.0.0",
    "169.254.",
    ".local",
    ".internal",
];

/// Auth schemes a peer hive may advertise. Claiming a scheme the server does
/// not implement makes a peer believe it authenticated when it did not — only
/// claim what exists.
///
/// - `bearer-optional` / `none` / `anonymous` — our own door's schemes.
/// - `bearer+hmac` — hivv.org's: an accounting bearer plus an HMAC
///   signature over the bridge endpoints (HIVE-TO-HIVE §3). The doorbell
///   itself is ringable with the bearer alone, which is what we send.
const IMPLEMENTED_AUTH_SCHEMES: &[&str] = &["bearer-optional", "none", "anonymous", "bearer+hmac"];

// ---------------------------------------------------------------------------
// Manifest
// ---------------------------------------------------------------------------

/// A peer hive's discovery manifest, validated and ready to ring.
#[derive(Debug, Clone)]
pub struct PeerManifest {
    /// Where the manifest was fetched from (the door that answered).
    pub source: String,
    pub hive: String,
    pub protocol: String,
    /// `endpoints.chat` — the doorbell.
    pub chat: String,
    /// `endpoints.models`, if published; derived from the chat URL otherwise.
    pub models: Option<String>,
    /// `auth.scheme`, or `"none"` when unadvertised.
    pub scheme: String,
    pub raw: Value,
}

impl PeerManifest {
    /// The models endpoint: published explicitly, else derived from the chat
    /// door (`…/v1/chat/completions` → `…/v1/models`).
    pub fn models_url(&self) -> String {
        if let Some(m) = &self.models {
            return m.clone();
        }
        if let Some(base) = self.chat.strip_suffix("/chat/completions") {
            return format!("{base}/models");
        }
        format!("{}/models", self.chat)
    }
}

/// Validate a manifest against the open-hives/0.1 rules our own manifest is
/// held to. Returns every violation, not just the first — the error IS the
/// documentation, so a peer author sees all the screws to remove at once.
pub fn validate(m: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    let protocol = m["protocol"].as_str().unwrap_or_default();
    if !protocol.starts_with("open-hive") {
        errors.push(format!(
            "protocol must be open-hives/0.1 (got {protocol:?}) — \
             this does not look like an open-hives manifest"
        ));
    }

    if m["hive"].as_str().unwrap_or_default().is_empty() {
        errors.push("hive must name the peer hive".into());
    }

    // The doorbell rule (open-hives/0.1 §12.1): a hive WITHOUT a usable
    // endpoints.chat is "visible but silent".
    let chat = m["endpoints"]["chat"].as_str().unwrap_or_default();
    if chat.is_empty() {
        errors.push(
            "endpoints.chat must publish the doorbell URL — \
             without it this hive is visible but silent"
                .into(),
        );
    } else if !chat.starts_with("https://") {
        errors.push(format!(
            "endpoints.chat must be an absolute https URL (open-hives/0.1 §12.1), got {chat:?}"
        ));
    } else if chat.contains(' ') {
        errors.push(format!(
            "endpoints.chat must be a URL, not a description: {chat:?}"
        ));
    }

    let mut strings = Vec::new();
    all_strings(m, &mut strings);
    for s in &strings {
        for f in FORBIDDEN_ADDRESS_MARKERS {
            if s.contains(f) {
                errors.push(format!(
                    "manifest publishes a private address ({f:?}) in: {s:?} — \
                     a peer outside this LAN can never reach it"
                ));
            }
        }
    }

    if let Some(scheme) = m["auth"].get("scheme").and_then(|s| s.as_str()) {
        if !IMPLEMENTED_AUTH_SCHEMES.contains(&scheme) {
            errors.push(format!(
                "auth.scheme {scheme:?} is not one of the implemented schemes \
                 {IMPLEMENTED_AUTH_SCHEMES:?} — implement it or stop advertising it"
            ));
        }
    }

    if let Some(pct) = m["limits"].get("allowance_max_percent") {
        match pct.as_u64() {
            Some(p) if p <= 100 => {}
            other => errors.push(format!(
                "limits.allowance_max_percent must be a percentage 0–100, got {other:?}"
            )),
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn all_strings(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) => out.push(s.clone()),
        Value::Array(a) => a.iter().for_each(|x| all_strings(x, out)),
        Value::Object(o) => o.values().for_each(|x| all_strings(x, out)),
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Fetch — resolve the target, then try the doors
// ---------------------------------------------------------------------------

/// Fetch and validate a peer manifest.
///
/// `target` may be a manifest URL (ends in `.json`), a host (`hivv.org`),
/// or a brand (`hivv` — resolved through the published brands registry).
pub async fn fetch_manifest(client: &reqwest::Client, target: &str) -> Result<PeerManifest> {
    let target = target.trim().to_lowercase();
    if target.is_empty() {
        bail!("give a hive: a brand (hivv), a host (hivv.org), or a manifest URL");
    }

    // An explicit URL is a single door — try exactly what was asked for.
    if target.starts_with("http://") || target.starts_with("https://") {
        if !target.ends_with(".json") {
            bail!("{target} is not a manifest URL — pass a host instead, or the full .json URL");
        }
        return try_door(client, &target).await;
    }

    let origin = if target.contains('.') {
        format!("https://{target}")
    } else {
        format!("https://{}", resolve_brand(client, &target).await?)
    };

    let mut tried = Vec::new();
    for door in DOORS {
        let url = format!("{origin}{door}");
        match try_door(client, &url).await {
            Ok(m) => return Ok(m),
            Err(e) => tried.push(format!("  {url}: {e}")),
        }
    }

    bail!(
        "no open-hives manifest found for {target}. Doors tried:\n{}\n\
         A peer hive publishes one of these three doors; if the hive exists but \
         publishes none, it is visible but silent (open-hives/0.1 §12.1).",
        tried.join("\n")
    )
}

/// Resolve a brand name to a host via the published brands registry.
async fn resolve_brand(client: &reqwest::Client, brand: &str) -> Result<String> {
    let resp = client
        .get(BRANDS_REGISTRY)
        .timeout(Duration::from_secs(8))
        .send()
        .await
        .with_context(|| format!("fetching the brand registry {BRANDS_REGISTRY}"))?;
    if !resp.status().is_success() {
        bail!(
            "brand registry {BRANDS_REGISTRY} returned {} — \
             pass a host (e.g. hivv.org) instead of the brand {brand:?}",
            resp.status()
        );
    }
    let body: Value = resp.json().await.context("parsing the brand registry")?;
    // Accept both {"brands": {name: host}} and a bare {name: host} map.
    let map = body.get("brands").unwrap_or(&body);
    match map.get(brand).and_then(|h| h.as_str()) {
        Some(host) if !host.is_empty() => Ok(host.to_string()),
        _ => bail!(
            "brand {brand:?} is not in {BRANDS_REGISTRY} — known brands: {}",
            known_brands(&body).join(", ")
        ),
    }
}

fn known_brands(body: &Value) -> Vec<String> {
    let map = body.get("brands").unwrap_or(body);
    map.as_object()
        .map(|o| o.keys().cloned().collect())
        .unwrap_or_default()
}

/// GET one manifest door and validate what comes back.
async fn try_door(client: &reqwest::Client, url: &str) -> Result<PeerManifest> {
    let resp = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .header(reqwest::header::USER_AGENT, user_agent())
        .send()
        .await
        .with_context(|| format!("connecting to {url}"))?;
    let status = resp.status();
    if !status.is_success() {
        bail!("HTTP {status}");
    }
    let m: Value = resp
        .json()
        .await
        .with_context(|| format!("{url} did not return JSON — is it a manifest door?"))?;

    if let Err(errors) = validate(&m) {
        bail!(
            "manifest at {url} failed validation:\n    {}",
            errors.join("\n    ")
        );
    }

    Ok(PeerManifest {
        source: url.to_string(),
        hive: m["hive"].as_str().unwrap_or_default().to_string(),
        protocol: m["protocol"].as_str().unwrap_or_default().to_string(),
        chat: m["endpoints"]["chat"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        models: m["endpoints"]["models"].as_str().map(|s| s.to_string()),
        scheme: m["auth"]["scheme"].as_str().unwrap_or("none").to_string(),
        raw: m,
    })
}

// ---------------------------------------------------------------------------
// Ask — ring the doorbell
// ---------------------------------------------------------------------------

/// The answer from a peer hive's doorbell.
#[derive(Debug)]
pub struct AskReply {
    pub text: String,
    pub model: String,
    pub door: String,
    pub elapsed: Duration,
}

/// Ring a peer hive's doorbell: one OpenAI-compatible chat POST to
/// `endpoints.chat`, carrying the loop-shield headers from HIVE-TO-HIVE §4.
///
/// Back-off is the peer's to give and ours to honour (§5): 429/503 with a
/// Retry-After is reported as-is, never retried harder.
pub async fn ask(
    client: &reqwest::Client,
    m: &PeerManifest,
    message: &str,
    model: Option<&str>,
) -> Result<AskReply> {
    let model = match model {
        Some(id) => id.to_string(),
        None => first_model(client, m).await?,
    };

    let body = serde_json::json!({
        "model": model,
        "messages": [{ "role": "user", "content": message }],
        "stream": false
    });

    let started = Instant::now();
    let resp = client
        .post(&m.chat)
        .timeout(Duration::from_secs(60))
        .header(reqwest::header::USER_AGENT, user_agent())
        .header(
            reqwest::header::AUTHORIZATION,
            format!("hive:{HIVE_ORIGIN}"),
        )
        .header("X-Hive-Origin", HIVE_ORIGIN)
        .header("X-Hive-Path", HIVE_ORIGIN)
        .header("X-Hive-Hops", "1")
        .header("X-Hive-Request-Id", request_id())
        .json(&body)
        .send()
        .await
        .with_context(|| format!("ringing {} — the door did not answer", m.chat))?;
    let status = resp.status();
    let elapsed = started.elapsed();

    if status.as_u16() == 409 {
        let detail = snippet(&resp.text().await.unwrap_or_default());
        bail!(
            "the peer refused with 409 loop_detected: {detail}\n\
             (open-hives/0.1 §4 — our own name reached our own request)"
        );
    }
    if status.as_u16() == 429 || status.as_u16() == 503 {
        let retry = resp
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("not specified")
            .to_string();
        let kind = if status.as_u16() == 429 {
            "rate-limited"
        } else {
            "shedding load"
        };
        bail!(
            "the peer hive is {kind} (HTTP {status}, Retry-After: {retry}) — \
             honour the back-off and try again later (open-hives/0.1 §5)"
        );
    }
    if !status.is_success() {
        let detail = snippet(&resp.text().await.unwrap_or_default());
        bail!("doorbell returned HTTP {status}: {detail}");
    }

    let v: Value = resp
        .json()
        .await
        .context("the door answered but not with OpenAI-shaped JSON")?;
    let text = extract_text(&v).context("response carried no message content")?;

    Ok(AskReply {
        text,
        model,
        door: m.chat.clone(),
        elapsed,
    })
}

/// Pick the model to ask for: the first one the peer publishes.
async fn first_model(client: &reqwest::Client, m: &PeerManifest) -> Result<String> {
    let url = m.models_url();
    let resp = client
        .get(&url)
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .with_context(|| format!("fetching the peer's models from {url}"))?;
    if !resp.status().is_success() {
        bail!(
            "the peer publishes no models at {url} (HTTP {}) — \
             pass one explicitly with --model",
            resp.status()
        );
    }
    let v: Value = resp.json().await.context("parsing the models list")?;
    v["data"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|m| m["id"].as_str())
        .map(|s| s.to_string())
        .with_context(|| format!("{url} returned no model ids — pass one with --model"))
}

/// Pull the assistant text out of an OpenAI-shaped chat completion.
fn extract_text(v: &Value) -> Option<String> {
    let content = v["choices"][0]["message"]["content"].as_str()?;
    if content.is_empty() {
        return None;
    }
    Some(content.to_string())
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

fn user_agent() -> String {
    format!("kwaainet/{}/bridge", env!("CARGO_PKG_VERSION"))
}

/// A dedupe key for the loop shield (HIVE-TO-HIVE §4): monotonic-ish, unique
/// per process, no new dependencies.
fn request_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("req_{nanos:x}{:04x}", std::process::id() & 0xffff)
}

/// Trim a peer's error body to something a terminal can carry.
fn snippet(s: &str) -> String {
    let flat = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() > 300 {
        let cut: String = flat.chars().take(300).collect();
        format!("{cut}…")
    } else if flat.is_empty() {
        "(empty body)".into()
    } else {
        flat
    }
}

// ---------------------------------------------------------------------------
// CLI entry point
// ---------------------------------------------------------------------------

/// Dispatch for `kwaainet hive fetch|ask`.
pub async fn run(args: crate::cli::HiveArgs) -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent(user_agent())
        .build()
        .context("building the HTTP client")?;

    match args.action {
        crate::cli::HiveAction::Fetch { target, json } => {
            let m = fetch_manifest(&client, &target).await?;
            if json {
                println!("{}", serde_json::to_string_pretty(&m.raw)?);
                return Ok(());
            }
            println!("hive       {} ({})", m.hive, m.protocol);
            println!("doorbell   {}", m.chat);
            println!("auth       {}", m.scheme);
            println!("models     {}", m.models_url());
            println!("manifest   {}", m.source);
            println!("validation ok — no private addresses, https doorbell, known auth scheme");
        }
        crate::cli::HiveAction::Ask {
            target,
            message,
            model,
        } => {
            let m = fetch_manifest(&client, &target).await?;
            println!("ringing {} …", m.hive);
            let reply = ask(&client, &m, &message, model.as_deref()).await?;
            println!(
                "answered by {} (model {}, {} ms)\n",
                reply.door,
                reply.model,
                reply.elapsed.as_millis()
            );
            println!("{}", reply.text);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests — offline, the same rules our own manifest is held to
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .expect("crate lives at core/crates/<name>")
            .to_path_buf()
    }

    fn our_manifest() -> Value {
        let path = repo_root().join("docs/bridge/hivv.json");
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        serde_json::from_str(&raw).expect("our manifest must be valid JSON")
    }

    fn peer_manifest() -> Value {
        serde_json::json!({
            "hive": "example.org",
            "protocol": "open-hives/0.1",
            "endpoints": {
                "chat": "https://example.org/v1/chat/completions",
                "models": "https://example.org/v1/models"
            },
            "auth": { "scheme": "bearer-optional" },
            "limits": { "allowance_max_percent": 20, "rate_per_min": "per node" }
        })
    }

    #[test]
    fn our_own_manifest_passes_peer_validation() {
        // Dogfood: what a peer does with our manifest must succeed.
        validate(&our_manifest()).expect("docs/bridge/hivv.json must satisfy the peer rules");
    }

    #[test]
    fn a_reasonable_peer_manifest_passes() {
        validate(&peer_manifest()).expect("a canonical peer manifest must validate");
    }

    #[test]
    fn lan_doorbell_is_rejected() {
        // The regression: our manifest once published a LAN address.
        let mut m = peer_manifest();
        m["endpoints"]["chat"] =
            Value::String("http://192.168.1.209:8080/v1/chat/completions".into());
        let errors = validate(&m).expect_err("LAN doorbell must be rejected");
        assert!(
            errors.iter().any(|e| e.contains("192.168.")),
            "the error must name the private address, got: {errors:?}"
        );
        assert!(
            errors.iter().any(|e| e.contains("https")),
            "the error must demand https, got: {errors:?}"
        );
    }

    #[test]
    fn missing_doorbell_is_rejected() {
        // Visible but silent (§12.1).
        let mut m = peer_manifest();
        m["endpoints"] = serde_json::json!({ "models": "https://example.org/v1/models" });
        let errors = validate(&m).expect_err("a manifest without chat must be rejected");
        assert!(errors.iter().any(|e| e.contains("endpoints.chat")));
    }

    #[test]
    fn unknown_auth_scheme_is_rejected() {
        let mut m = peer_manifest();
        m["auth"]["scheme"] = Value::String("per-node bearer".into());
        let errors = validate(&m).expect_err("an unimplemented auth scheme must be rejected");
        assert!(errors.iter().any(|e| e.contains("auth.scheme")));
    }

    #[test]
    fn wrong_protocol_is_rejected() {
        let mut m = peer_manifest();
        m["protocol"] = Value::String("something/1.0".into());
        let errors = validate(&m).expect_err("a foreign protocol must be rejected");
        assert!(errors.iter().any(|e| e.contains("protocol")));
    }

    #[test]
    fn out_of_range_allowance_is_rejected() {
        let mut m = peer_manifest();
        m["limits"]["allowance_max_percent"] = Value::from(150);
        let errors = validate(&m).expect_err("allowance > 100 must be rejected");
        assert!(errors.iter().any(|e| e.contains("allowance_max_percent")));
    }

    #[test]
    fn private_address_anywhere_is_rejected_even_not_in_chat() {
        let mut m = peer_manifest();
        m["endpoints"]["state"] = Value::String("http://10.0.0.5:9090/state".into());
        let errors = validate(&m).expect_err("a private address in any field must be rejected");
        assert!(errors.iter().any(|e| e.contains("10.0.")));
    }

    #[test]
    fn models_url_is_derived_from_the_chat_door_when_unpublished() {
        let mut m = peer_manifest();
        m["endpoints"].as_object_mut().unwrap().remove("models");
        let manifest = PeerManifest {
            source: "https://example.org/.well-known/hive.json".into(),
            hive: "example.org".into(),
            protocol: "open-hives/0.1".into(),
            chat: "https://example.org/v1/chat/completions".into(),
            models: None,
            scheme: "none".into(),
            raw: m,
        };
        assert_eq!(manifest.models_url(), "https://example.org/v1/models");
    }

    #[test]
    fn request_ids_are_unique_enough_for_dedupe() {
        let a = request_id();
        let b = request_id();
        assert!(a.starts_with("req_"));
        assert_ne!(a, b, "two requests must not share a loop-shield id");
    }

    #[test]
    fn snippet_trims_a_hostile_error_body() {
        let long = "x ".repeat(500);
        assert!(snippet(&long).chars().count() <= 301);
        assert_eq!(snippet(""), "(empty body)");
    }
}
