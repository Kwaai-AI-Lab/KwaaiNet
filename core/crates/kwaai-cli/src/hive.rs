//! `kwaainet ask` / `kwaainet manifest` — the open-hives/0.1 doorbell client.
//!
//! A hive is a whole network of nodes behind one domain; the doorbell is the
//! single `endpoints.chat` URL its discovery manifest publishes. Ringing it
//! does not name a node — the peer hive resolves the request across its own
//! nodes, so the door keeps working when individual machines go down.
//!
//! This module is the kwaai-side counterpart of hivv.org's `hivv ask` flow
//! (`public_html/hivv-cli.py::_ask_hive`): resolve a brand or host, try the
//! three manifest doors, validate what comes back, then ring.
//!
//! ## How our validation differs from our own conformance tests
//!
//! [`tests/bridge_manifest.rs`] pins what *our* manifest may claim; this
//! module decides what *a peer's* manifest may claim. The two deliberately
//! differ, because the jobs differ:
//!
//! - **protocol** — ours must be exactly `open-hives/0.1`; a peer may be any
//!   `open-hives/*` (we interoperate with the future, and hivv.org's client
//!   is likewise prefix-tolerant).
//! - **auth schemes** — ours may only claim what `shard_api.rs` implements;
//!   a peer may claim any scheme *we know how to sit next to*, which includes
//!   hivv.org's `bearer+hmac` (accounting bearer plus HMAC over the bridge
//!   endpoints, HIVE-TO-HIVE §3).
//! - **required fields** — ours must carry the full contract (`discovery.note`,
//!   `discovery.state`, `status`, …); a peer need only be ringable — a hive
//!   without `endpoints.chat` is *visible but silent* (§12.1) and is fetched
//!   with a warning, not rejected.
//!
//! The two files share the private-address marker list by copy — each carries
//! a comment pointing at the other; keep them in sync. Here the list is
//! backed by real IP parsing (see [`string_publishes_private_address`])
//! because this client *acts* on the values: it GETs `endpoints.models` and
//! POSTs to `endpoints.chat`.
//!
//! The regression this exists for: our manifest once advertised
//! `http://192.168.1.209:8080` as the doorbell — a LAN address no peer
//! outside that network could ever reach.

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

/// Hostname markers that are never a public address. Kept as substrings
/// because they identify *names*, not address ranges — ranges are parsed by
/// [`string_publishes_private_address`].
///
/// MIRROR: `tests/bridge_manifest.rs` carries the same list for our own
/// manifest. Keep the two in sync.
const HOSTNAME_MARKERS: &[&str] = &["localhost", ".local", ".internal"];

/// Partial-octet address fragments — the cheap substring net that catches
/// shorthand forms a strict four-octet parse would miss (`10.0.5`,
/// `192.168.1`).
///
/// MIRROR: `tests/bridge_manifest.rs` carries the same list for our own
/// manifest. Keep the two in sync.
const ADDRESS_MARKERS: &[&str] = &["192.168.", "10.0.", "127.0.0.1", "0.0.0.0", "169.254."];

/// Auth schemes a peer hive may advertise. Claiming a scheme the server does
/// not implement makes a peer believe it authenticated when it did not — only
/// claim what exists.
///
/// - `bearer-optional` / `none` / `anonymous` — our own door's schemes.
/// - `bearer+hmac` — hivv.org's: an accounting bearer plus an HMAC
///   signature over the bridge endpoints (HIVE-TO-HIVE §3). The doorbell
///   itself is ringable with the bearer alone, which is what we send.
const PEER_AUTH_SCHEMES: &[&str] = &["bearer-optional", "none", "anonymous", "bearer+hmac"];

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
    /// `endpoints.chat` — the doorbell; empty when the hive is
    /// visible but silent (§12.1).
    pub chat: String,
    /// `endpoints.models`, if published; derived from the chat URL otherwise.
    pub models: Option<String>,
    /// `auth.scheme`, or `"none"` when unadvertised.
    pub scheme: String,
    /// Non-fatal advisories from validation (e.g. no doorbell published).
    pub warnings: Vec<String>,
    pub raw: Value,
}

impl PeerManifest {
    /// The models endpoint: published explicitly, else derived from the chat
    /// door (`…/v1/chat/completions` → `…/v1/models`), else the origin's
    /// standard `/v1/models`.
    pub fn models_url(&self) -> String {
        if let Some(m) = &self.models {
            return m.clone();
        }
        if let Some(base) = self.chat.strip_suffix("/chat/completions") {
            return format!("{base}/models");
        }
        match self.chat.split_once("://") {
            Some((scheme, rest)) => {
                let host = rest.split(['/', '?', '#']).next().unwrap_or(rest);
                format!("{scheme}://{host}/v1/models")
            }
            None => format!("{}/v1/models", self.chat),
        }
    }
}

/// Validate a manifest against the peer-side open-hives rules.
///
/// Returns `Ok(warnings)` when the manifest is fetchable (possibly with
/// advisories — a hive with no `endpoints.chat` is visible but silent, §12.1),
/// or `Err(errors)` listing *every* violation: the error IS the documentation,
/// so a peer author sees all the screws to remove at once.
pub fn validate(m: &Value) -> Result<Vec<String>, Vec<String>> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Prefix, not exact: we interoperate with future open-hives revisions the
    // way hivv.org's client does. "open-hivex" and bare "open-hive" do not
    // start with "open-hives/" and are rejected — pinned by test.
    let protocol = m["protocol"].as_str().unwrap_or_default();
    if !protocol.starts_with("open-hives/") {
        errors.push(format!(
            "protocol must start with \"open-hives/\" (current: open-hives/0.1), \
             got {protocol:?} — this does not look like an open-hives manifest"
        ));
    }

    if m["hive"].as_str().unwrap_or_default().is_empty() {
        errors.push("hive must name the peer hive".into());
    }

    // The doorbell rule (open-hives/0.1 §12.1): a hive WITHOUT a usable
    // endpoints.chat is "visible but silent" — fetchable, not ringable.
    let chat = m["endpoints"]["chat"].as_str().unwrap_or_default();
    if chat.is_empty() {
        warnings.push(
            "no endpoints.chat — this hive is visible but silent (open-hives/0.1 §12.1): \
             fetch works, ask does not"
                .into(),
        );
    } else if !chat.starts_with("https://") {
        errors.push(format!(
            "endpoints.chat must be an absolute https URL (open-hives/0.1 §12.1), got {chat:?} — \
             kwaainet rings doorbells over https only, a deliberate tightening over the spec's \
             intranet allowance"
        ));
    } else if chat.chars().any(char::is_whitespace) {
        errors.push(format!(
            "endpoints.chat must be a single URL, not a description: {chat:?}"
        ));
    }

    let mut strings = Vec::new();
    all_strings(m, &mut strings);
    for s in &strings {
        if let Some(found) = private_address_in(s) {
            errors.push(format!(
                "manifest publishes a private address ({found}) in: {s:?} — \
                 a peer outside this LAN can never reach it"
            ));
        }
    }

    if let Some(scheme) = m["auth"].get("scheme").and_then(|s| s.as_str()) {
        if !PEER_AUTH_SCHEMES.contains(&scheme) {
            errors.push(format!(
                "auth.scheme {scheme:?} is not one of the schemes this client knows how to \
                 sit next to {PEER_AUTH_SCHEMES:?} — implement it or stop advertising it"
            ));
        }
    }

    if let Some(pct) = m["limits"].get("allowance_max_percent") {
        match pct.as_f64() {
            Some(p) if (0.0..=100.0).contains(&p) => {}
            _ => errors.push(format!(
                "limits.allowance_max_percent must be a number 0–100, got {pct}"
            )),
        }
    }

    if errors.is_empty() {
        Ok(warnings)
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
// Private-address detection
// ---------------------------------------------------------------------------

/// Does this string publish a private address? Three nets, strongest first:
///
/// 1. bracketed IPv6 (`[::1]`, `[fc00::1]`) — parsed, not matched;
/// 2. address-shaped tokens — dotted quad parsed as `Ipv4Addr`, decimal
///    (`2130706433`) and hex (`0x7f000001`) decoded inet-aton style;
/// 3. the substring markers for shorthand forms and hostnames.
fn private_address_in(s: &str) -> Option<String> {
    // 1) bracketed IPv6
    let mut rest = s;
    while let Some(open) = rest.find('[') {
        if let Some(close) = rest[open..].find(']') {
            let inner = &rest[open + 1..open + close];
            if inner.contains(':') {
                if let Ok(ip) = inner.parse::<std::net::Ipv6Addr>() {
                    if ipv6_is_private(ip) {
                        return Some(format!("[{inner}]"));
                    }
                }
            }
            rest = &rest[open + close + 1..];
        } else {
            break;
        }
    }

    // 2) address-shaped tokens
    for tok in s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '.')) {
        if tok.is_empty() {
            continue;
        }
        if let Some(found) = private_ipv4_token(tok) {
            return Some(found);
        }
    }

    // 3) markers
    for m in ADDRESS_MARKERS.iter().chain(HOSTNAME_MARKERS) {
        if s.contains(m) {
            return Some((*m).to_string());
        }
    }
    None
}

/// One token that might be an IPv4 address in any of the forms a URL host
/// may take. Returns the token when it names a private address.
fn private_ipv4_token(tok: &str) -> Option<String> {
    if let Ok(ip) = tok.parse::<std::net::Ipv4Addr>() {
        return if ipv4_is_private(ip) {
            Some(tok.to_string())
        } else {
            None
        };
    }
    // decimal (2130706433) and hex (0x7f000001) — inet-aton big-endian
    let n = if let Some(h) = tok
        .strip_prefix("0x")
        .or_else(|| tok.strip_prefix("0X"))
        .filter(|h| !h.is_empty() && h.chars().all(|c| c.is_ascii_hexdigit()))
    {
        u32::from_str_radix(h, 16).ok()
    } else if tok.len() <= 10 && tok.chars().all(|c| c.is_ascii_digit()) && !tok.is_empty() {
        tok.parse::<u32>().ok()
    } else {
        None
    }?;
    let o = n.to_be_bytes();
    let ip = std::net::Ipv4Addr::new(o[0], o[1], o[2], o[3]);
    if ipv4_is_private(ip) {
        Some(format!("{tok} (= {ip})"))
    } else {
        None
    }
}

fn ipv4_is_private(ip: std::net::Ipv4Addr) -> bool {
    // is_private covers 10/8, 172.16/12, 192.168/16
    ip.is_private() || ip.is_loopback() || ip.is_link_local() || ip.is_unspecified()
}

fn ipv6_is_private(ip: std::net::Ipv6Addr) -> bool {
    let s = ip.segments();
    ip.is_loopback()
        || ip.is_unspecified()
        || (s[0] & 0xfe00) == 0xfc00 // fc00::/7 unique local
        || (s[0] & 0xffc0) == 0xfe80 // fe80::/10 link local
}

/// Is this a hostname a *registry* may hand us? Unlike a user-typed target,
/// a brand-registry value is untrusted input: it must be a bare public
/// hostname — no scheme, path, port, userinfo, or private address.
fn registry_host_is_usable(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("the registry returned an empty host".into());
    }
    if host
        .chars()
        .any(|c| matches!(c, '@' | '/' | '\\' | '?' | '#' | ':' | ' ' | '\t'))
    {
        return Err(format!(
            "{host:?} is not a bare hostname (no scheme, path, port, or userinfo)"
        ));
    }
    if let Some(found) = private_address_in(host) {
        return Err(format!("{host:?} names a private address ({found})"));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Fetch — resolve the target, then try the doors
// ---------------------------------------------------------------------------

/// Fetch and validate a peer manifest.
///
/// `target` may be a manifest URL (ends in `.json`), a host (`hivv.org`),
/// or a brand (`hivv` — resolved through the published brands registry).
pub async fn fetch_manifest(client: &reqwest::Client, target: &str) -> Result<PeerManifest> {
    let target = target.trim();
    if target.is_empty() {
        bail!("give a hive: a brand (hivv), a host (hivv.org), or a manifest URL");
    }

    // An explicit URL is a single door — try exactly what was asked for.
    // Case is preserved: URL paths can be case-sensitive.
    if target.starts_with("http://") || target.starts_with("https://") {
        if !target.ends_with(".json") {
            bail!("{target} is not a manifest URL — pass a host instead, or the full .json URL");
        }
        return try_door(client, target).await;
    }

    let target = target.to_lowercase();
    let origin = if target.contains('.') {
        format!("https://{target}")
    } else {
        let host = resolve_brand(client, &target).await?;
        if let Err(why) = registry_host_is_usable(&host) {
            bail!(
                "the brand registry {BRANDS_REGISTRY} returned an unusable host for {target:?}: \
                 {why} — pass a host explicitly instead"
            );
        }
        format!("https://{host}")
    };

    let mut tried = Vec::new();
    for door in DOORS {
        let url = format!("{origin}{door}");
        match try_door(client, &url).await {
            Ok(m) => return Ok(m),
            Err(e) => tried.push(format!("  {url}: {e:#}")),
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
        // Redirects are NOT followed (the client is built with
        // Policy::none): a door must answer as itself, and a 3xx body is
        // where the peer's explanation (or its soft-404 page) lives.
        let body = resp.text().await.unwrap_or_default();
        bail!("{url} answered HTTP {status}: {}", snippet(&body));
    }
    let text = resp
        .text()
        .await
        .with_context(|| format!("reading the body from {url}"))?;
    let m: Value = serde_json::from_str(&text).with_context(|| {
        format!(
            "{url} answered {status} but not JSON — is it a manifest door? {}",
            snippet(&text)
        )
    })?;

    let warnings = validate(&m).map_err(|errors| {
        anyhow::anyhow!(
            "manifest at {url} failed validation:\n    {}",
            errors.join("\n    ")
        )
    })?;

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
        warnings,
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
/// Retry-After is reported as-is, never retried harder. Redirects are not
/// followed: a 307 would forward the message body to a host we never
/// validated.
pub async fn ask(
    client: &reqwest::Client,
    m: &PeerManifest,
    message: &str,
    model: Option<&str>,
) -> Result<AskReply> {
    if m.chat.is_empty() {
        bail!(
            "{} publishes no endpoints.chat — visible but silent (open-hives/0.1 §12.1): \
             there is no doorbell to ring",
            m.hive
        );
    }
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

    let text_body = resp.text().await.context("reading the doorbell's answer")?;
    let v: Value = serde_json::from_str(&text_body).with_context(|| {
        format!(
            "the door answered but not with OpenAI-shaped JSON: {}",
            snippet(&text_body)
        )
    })?;
    let text = extract_text(&v).with_context(|| {
        format!(
            "response carried no message content: {}",
            snippet(&text_body)
        )
    })?;
    let elapsed = started.elapsed();

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

pub(crate) fn user_agent() -> String {
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
pub(crate) fn snippet(s: &str) -> String {
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

/// The one HTTP client the cross-hive commands share.
///
/// No redirects: a manifest door must answer as itself. Following a 302
/// could land us on a host (or an http:// URL) that validate() never saw,
/// and a 307 on the doorbell POST would forward the user's message there.
pub(crate) fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent(user_agent())
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("building the HTTP client")
}

/// `kwaainet manifest <hive>` — fetch, validate, and show a peer manifest.
pub async fn manifest_cmd(args: crate::cli::ManifestArgs) -> Result<()> {
    let client = http_client()?;
    let m = fetch_manifest(&client, &args.target).await?;
    if args.json {
        println!("{}", serde_json::to_string_pretty(&m.raw)?);
        return Ok(());
    }
    println!("hive       {} ({})", m.hive, m.protocol);
    println!(
        "doorbell   {}",
        if m.chat.is_empty() {
            "(none — visible but silent)"
        } else {
            &m.chat
        }
    );
    println!("auth       {}", m.scheme);
    if m.chat.is_empty() {
        println!("models     (none — no doorbell to derive from)");
    } else {
        println!("models     {}", m.models_url());
    }
    println!("manifest   {}", m.source);
    println!("validation ok — no private addresses, https doorbell, known auth scheme");
    for w in &m.warnings {
        println!("warning    {w}");
    }
    Ok(())
}

/// `kwaainet ask <hive> "<message>"` — ring a peer hive's doorbell.
///
/// Mirrors hivv.org's `hivv ask <hive> "<message>"` so the two CLIs read
/// the same way to anyone who knows one of them.
pub async fn ask_cmd(args: crate::cli::AskArgs) -> Result<()> {
    let client = http_client()?;
    let m = fetch_manifest(&client, &args.target).await?;
    for w in &m.warnings {
        println!("warning    {w}");
    }
    println!("ringing {} …", m.hive);
    let reply = ask(&client, &m, &args.message, args.model.as_deref()).await?;
    println!(
        "answered by {} (model {}, {} ms)\n",
        reply.door,
        reply.model,
        reply.elapsed.as_millis()
    );
    println!("{}", reply.text);
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests — offline, plus the live contracts they pin
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

    /// hivv.org's live manifest, saved as a fixture: the real interop
    /// contract (bearer+hmac, relative heartbeat, templated inbox).
    fn hivv_manifest() -> Value {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hivv_manifest.json");
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        serde_json::from_str(&raw).expect("fixture must be valid JSON")
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

    fn manifest_with_chat(chat: &str) -> Value {
        let mut m = peer_manifest();
        m["endpoints"]["chat"] = Value::String(chat.into());
        // Drop the published models too, so models_url() must derive it.
        m["endpoints"].as_object_mut().unwrap().remove("models");
        m
    }

    fn wrap(m: &Value) -> PeerManifest {
        PeerManifest {
            source: "test".into(),
            hive: m["hive"].as_str().unwrap_or_default().to_string(),
            protocol: m["protocol"].as_str().unwrap_or_default().to_string(),
            chat: m["endpoints"]["chat"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            models: m["endpoints"]["models"].as_str().map(|s| s.to_string()),
            scheme: m["auth"]["scheme"].as_str().unwrap_or("none").to_string(),
            warnings: vec![],
            raw: m.clone(),
        }
    }

    // -- validation ---------------------------------------------------------

    #[test]
    fn our_own_manifest_passes_peer_validation() {
        // Dogfood: what a peer does with our manifest must succeed.
        validate(&our_manifest()).expect("docs/bridge/hivv.json must satisfy the peer rules");
    }

    #[test]
    fn hivv_orgs_live_manifest_passes_with_its_hmac_scheme() {
        // The headline interop claim, pinned in CI: hivv.org publishes
        // bearer+hmac, a relative heartbeat and a templated inbox — all real.
        let m = hivv_manifest();
        let warnings = validate(&m).expect("hivv.org's manifest must validate");
        assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
        assert_eq!(m["auth"]["scheme"], "bearer+hmac");
        assert_eq!(m["hive"], "hivv.org");
    }

    #[test]
    fn a_reasonable_peer_manifest_passes() {
        let warnings = validate(&peer_manifest()).expect("a canonical peer manifest must validate");
        assert!(warnings.is_empty(), "unexpected warnings: {warnings:?}");
    }

    #[test]
    fn lan_doorbell_is_rejected() {
        // The regression: our manifest once published a LAN address.
        let m = manifest_with_chat("http://192.168.1.209:8080/v1/chat/completions");
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
    fn every_private_address_form_is_rejected() {
        // F5: the forms a substring list alone would miss — full 172.16/12,
        // non-.1 loopbacks, IPv6, decimal and hex host forms.
        let private_hosts = [
            "https://10.1.2.3/v1/chat/completions",
            "https://172.16.0.1/v1/chat/completions",
            "https://172.31.255.1/v1/chat/completions",
            "https://127.0.0.2/v1/chat/completions",
            "https://169.254.1.1/v1/chat/completions",
            "https://0.0.0.0/v1/chat/completions",
            "https://192.168.1.1/v1/chat/completions",
            "https://[::1]/v1/chat/completions",
            "https://[fc00::1]/v1/chat/completions",
            "https://[fe80::1]/v1/chat/completions",
            "https://2130706433/v1/chat/completions",
            "https://0x7f000001/v1/chat/completions",
        ];
        for chat in private_hosts {
            let m = manifest_with_chat(chat);
            let errors = validate(&m).expect_err(&format!("must reject {chat}"));
            assert!(
                errors.iter().any(|e| e.contains("private address")),
                "error must name the private address for {chat}, got: {errors:?}"
            );
        }
    }

    #[test]
    fn public_addresses_are_allowed() {
        for chat in [
            "https://8.8.8.8/v1/chat/completions",
            "https://93.184.216.34/v1/chat/completions",
            "https://[2606:4700:4700::1111]/v1/chat/completions",
        ] {
            let m = manifest_with_chat(chat);
            validate(&m)
                .unwrap_or_else(|e| panic!("public {chat} must pass validation, got {e:?}"));
        }
    }

    #[test]
    fn a_private_address_in_any_field_is_rejected_not_just_chat() {
        let mut m = peer_manifest();
        m["endpoints"]["state"] = Value::String("http://10.0.0.5:9090/state".into());
        let errors = validate(&m).expect_err("a private address in any field must be rejected");
        assert!(errors.iter().any(|e| e.contains("private address")));
    }

    #[test]
    fn missing_doorbell_is_fetchable_but_warns() {
        // §12.1: visible but silent — fetchable with a warning, not rejected.
        let mut m = peer_manifest();
        m["endpoints"] = serde_json::json!({ "models": "https://example.org/v1/models" });
        let warnings = validate(&m).expect("a chat-less manifest must still be fetchable");
        assert!(
            warnings.iter().any(|w| w.contains("endpoints.chat")),
            "the warning must name the missing door, got: {warnings:?}"
        );
    }

    #[test]
    fn unknown_auth_scheme_is_rejected() {
        let mut m = peer_manifest();
        m["auth"]["scheme"] = Value::String("per-node bearer".into());
        let errors = validate(&m).expect_err("an unimplemented auth scheme must be rejected");
        assert!(errors.iter().any(|e| e.contains("auth.scheme")));
    }

    #[test]
    fn protocol_prefix_is_pinned() {
        // P3: prefix-tolerant for future revisions, but "open-hives/" means
        // what it says — "open-hivex" and bare "open-hive" are not protocols.
        for good in ["open-hives/0.1", "open-hives/0.2"] {
            let mut m = peer_manifest();
            m["protocol"] = Value::String(good.into());
            let r = validate(&m);
            assert!(r.is_ok(), "{good} must be accepted, got {r:?}");
        }
        for bad in ["open-hivex", "open-hive", "something/1.0", ""] {
            let mut m = peer_manifest();
            m["protocol"] = Value::String(bad.into());
            let r = validate(&m);
            assert!(r.is_err(), "{bad:?} must be rejected");
            let errors = r.unwrap_err();
            assert!(
                errors[0].contains("open-hives/"),
                "the message must state the prefix rule, got: {:?}",
                errors[0]
            );
        }
    }

    #[test]
    fn out_of_range_allowance_is_rejected_and_the_raw_value_is_shown() {
        let mut m = peer_manifest();
        m["limits"]["allowance_max_percent"] = Value::from(150);
        let errors = validate(&m).expect_err("allowance > 100 must be rejected");
        assert!(errors.iter().any(|e| e.contains("150")));

        // A float percentage is a percentage (F9: must not print "None").
        let mut m = peer_manifest();
        m["limits"]["allowance_max_percent"] = serde_json::json!(20.0);
        validate(&m).expect("20.0 is a valid percentage");

        // A string is not — and the message shows what arrived.
        let mut m = peer_manifest();
        m["limits"]["allowance_max_percent"] = Value::String("lots".into());
        let errors = validate(&m).expect_err("a non-number must be rejected");
        assert!(errors.iter().any(|e| e.contains("lots")));
    }

    // -- models_url ---------------------------------------------------------

    #[test]
    fn models_url_is_derived_from_the_chat_door_when_unpublished() {
        let mut m = peer_manifest();
        m["endpoints"].as_object_mut().unwrap().remove("models");
        assert_eq!(
            wrap(&m).models_url(),
            "https://example.org/v1/models",
            "strip_suffix branch"
        );

        // Suffix-less chat falls back to the origin's standard path (F4.2).
        let m = manifest_with_chat("https://x.example/v1/chat");
        assert_eq!(wrap(&m).models_url(), "https://x.example/v1/models");

        // A trailing slash must not produce a double slash.
        let m = manifest_with_chat("https://x.example/v1/chat/completions/");
        assert_eq!(wrap(&m).models_url(), "https://x.example/v1/models");
    }

    // -- registry host vetting (F7) ------------------------------------------

    #[test]
    fn registry_hosts_must_be_bare_public_hostnames() {
        assert!(registry_host_is_usable("hivv.org").is_ok());
        assert!(registry_host_is_usable("hivv.org@evil.com").is_err());
        assert!(registry_host_is_usable("https://hivv.org").is_err());
        assert!(registry_host_is_usable("hivv.org/path").is_err());
        assert!(registry_host_is_usable("hivv.org:8443").is_err());
        assert!(registry_host_is_usable("192.168.1.10").is_err());
        assert!(registry_host_is_usable("").is_err());
    }

    // -- door HTTP behaviour (F2, F6) ----------------------------------------

    /// A canned HTTP server on loopback: answers exactly one request with
    /// `status` and `body`, then drops the connection.
    fn canned_server(status_line: &'static str, body: &'static str) -> String {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let addr = listener.local_addr().expect("local addr");
        let response = format!(
            "HTTP/1.1 {status_line}\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        std::thread::spawn(move || {
            if let Ok((mut sock, _)) = listener.accept() {
                let mut buf = [0u8; 2048];
                let _ = sock.read(&mut buf);
                let _ = sock.write_all(response.as_bytes());
                let _ = sock.flush();
            }
        });
        format!("http://{addr}/manifest.json")
    }

    #[tokio::test]
    async fn try_door_reports_status_url_and_body_snippet_on_error() {
        // F2: the live 404 used to surface as a bare "HTTP 404 Not Found".
        let url = canned_server("404 Not Found", "no such manifest here");
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let err = try_door(&client, &url)
            .await
            .expect_err("a 404 must fail")
            .to_string();
        assert!(err.contains("404"), "status in error: {err}");
        assert!(err.contains("no such manifest"), "body in error: {err}");
        assert!(err.contains(&url), "url in error: {err}");
    }

    #[tokio::test]
    async fn try_door_does_not_follow_redirects() {
        // F6: a door that redirects must fail as itself, not chase the hop.
        let url = canned_server(
            "301 Moved Permanently",
            "moved to a host validate never saw",
        );
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();
        let err = try_door(&client, &url)
            .await
            .expect_err("a 301 must fail, not follow")
            .to_string();
        assert!(err.contains("301"), "status in error: {err}");
        assert!(err.contains("moved to a host"), "body in error: {err}");
    }

    // -- misc ----------------------------------------------------------------

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

    #[test]
    fn whitespace_in_chat_is_rejected() {
        let m = manifest_with_chat("https://example.org/v1/chat\ncompletions");
        let errors = validate(&m).expect_err("a multiline chat must be rejected");
        assert!(errors
            .iter()
            .any(|e| e.contains("whitespace") || e.contains("single URL")));
    }
}
