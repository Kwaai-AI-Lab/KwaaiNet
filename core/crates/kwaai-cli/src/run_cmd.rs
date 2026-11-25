//! `kwaainet run <hive> <file>` — execute a script on a peer hive.
//!
//! The kwaai-side mirror of blnq.org's own `blnq run <file>`: submit JSON to
//! the run door the peer publishes (`endpoints.run` in its open-hives
//! manifest), then poll `/{id}/logs` with the returned read token, printing
//! output as it grows. The contract being spoken is blnq.org's published one
//! (`https://blnq.org/ai.txt` → `quickstart_http`):
//!
//! ```text
//! POST {run}  {"language","code","stdin","args","isPublic"}
//!   → 200 {id, read_token, safety:{patternLevel, aiLevel, reason, …}}
//!   → 403 {id, error:"Code rejected by AI safety review", safety:{level, reason, …}}
//! GET {run}/{id}/logs  (header X-Run-Token: <read_token>)
//!   → {status, stdout, stderr, exitCode}   # cumulative while running
//! ```
//!
//! Terminal states are `completed | failed | timeout | rejected`
//! (`routes/run.js → doneStatuses`). The process exits with the remote exit
//! code so a calling pipeline sees the truth, not a translation of it:
//! `completed` → the remote code (0 = success), `timeout` → 124 (the
//! convention), `rejected` on the poll → 2, `failed` → 1 with the hive's
//! `error_message`. A rejection surfaced at submit is a bail → 1; both say
//! why. stdout carries only the remote script's bytes — every status line
//! this command prints goes to stderr.

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::hive::{fetch_manifest, http_client, snippet as clip, user_agent};

/// The peer's terminal run states.
const DONE: &[&str] = &["completed", "failed", "timeout", "rejected"];

/// Read the script file; the context is what names the file on failure.
fn read_script(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

/// Map a file extension to blnq's language ids (`/api/run/languages`).
fn language_for(file: &Path, explicit: Option<&str>) -> Result<String> {
    if let Some(l) = explicit {
        return Ok(l.to_string());
    }
    let ext = file
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_lowercase();
    let lang = match ext.as_str() {
        "py" => "python",
        "js" | "mjs" | "cjs" => "node",
        "rb" => "ruby",
        "sh" | "bash" => "bash",
        "php" => "php",
        "pl" => "perl",
        "c" => "c",
        "cpp" | "cc" | "cxx" => "cpp",
        "go" => "go",
        _ => bail!(
            "cannot infer a language from .{ext} — pass --language \
             (blnq runs: python, node, ruby, bash, php, perl, c, cpp, go)"
        ),
    };
    Ok(lang.to_string())
}

/// The suffix of `full` not yet printed — `None` when `printed` is not a
/// char boundary (a re-rendered, shorter, or rewritten buffer: never slice
/// blindly into UTF-8).
fn new_bytes(full: &str, printed: usize) -> Option<&str> {
    if printed >= full.len() {
        return None;
    }
    full.get(printed..)
}

pub async fn run(args: crate::cli::RunArgs) -> Result<()> {
    let code = read_script(&args.file)?;
    let language = language_for(&args.file, args.language.as_deref())?;

    // Shared no-redirect client: the door must answer as itself.
    let client = http_client()?;

    let m = fetch_manifest(&client, &args.target).await?;
    // stderr, not stdout: this command's stdout belongs to the remote script,
    // so a calling pipeline (kwaainet run … > out.txt) sees only its bytes.
    for w in &m.warnings {
        eprintln!("warning    {w}");
    }
    let door = m.raw["endpoints"]["run"]
        .as_str()
        .filter(|s| !s.is_empty())
        .with_context(|| {
            format!(
                "{} publishes no run door (endpoints.run) — only a hive that \
             advertises one will execute code for us",
                m.hive
            )
        })?;
    // We POST source code and receive a read token over this door — the same
    // rule endpoints.chat lives by: https only, one URL, never cleartext.
    if !door.starts_with("https://") || door.chars().any(char::is_whitespace) {
        bail!(
            "endpoints.run must be a single https URL, got {door:?} — \
             source code and read tokens never travel in cleartext"
        );
    }
    let door = door.trim_end_matches('/'); // no // in {door}/{id}/logs

    let mut body = serde_json::json!({
        "language": language,
        "code": code,
        "isPublic": args.public,
    });
    if let Some(s) = &args.stdin {
        body["stdin"] = Value::String(s.clone());
    }
    if !args.args.is_empty() {
        body["args"] = Value::Array(args.args.iter().cloned().map(Value::String).collect());
    }

    eprintln!(
        "submitting to {} ({language}) — the safety review can take up to 120s …",
        m.hive
    );
    let resp = client
        .post(door)
        // The submit POST runs the AI safety review synchronously (ai.txt:
        // "allow 120s"), so this timeout is deliberately generous.
        .timeout(Duration::from_secs(150))
        .header(reqwest::header::USER_AGENT, user_agent())
        .json(&body)
        .send()
        .await
        .with_context(|| format!("posting to {door}"))?;
    let status = resp.status();
    let text = resp.text().await.context("reading the submit response")?;

    if status.as_u16() == 403 {
        // Their contract: the rejection reason is FULL — "no mystery cuts in
        // transit". Show it, plus the id for bug reports.
        bail!("{}", rejection(&text));
    }
    if !status.is_success() {
        bail!("submit to {door} returned HTTP {status}: {}", clip(&text));
    }

    let v: Value =
        serde_json::from_str(&text).context("the run door answered but not with JSON")?;
    let id = v["id"].as_str().context("response carried no run id")?;
    let token = v["read_token"]
        .as_str()
        .context("response carried no read_token — cannot fetch private output")?;
    let pattern = v["safety"]["patternLevel"].as_str().unwrap_or("-");
    let ai = v["safety"]["aiLevel"].as_str().unwrap_or("-");
    eprintln!("submitted {id} (safety: pattern {pattern}, ai {ai})");

    // ── poll: cumulative output, printed once, as it grows ──────────────
    let logs_url = format!("{door}/{id}/logs");
    let started = Instant::now();
    let mut printed_out = 0usize;
    let mut printed_err = 0usize;
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;
        if started.elapsed().as_secs() > args.timeout {
            bail!(
                "gave up after {}s — the run may still finish; check it with \
                 id {id} (blnq.org/run.html)",
                args.timeout
            );
        }

        let resp = client
            .get(&logs_url)
            .timeout(Duration::from_secs(20))
            .header(reqwest::header::USER_AGENT, user_agent())
            .header("X-Run-Token", token)
            .send()
            .await
            .with_context(|| format!("polling {logs_url}"))?;
        let st = resp.status();
        let body = resp.text().await.context("reading the logs response")?;
        if !st.is_success() {
            bail!("polling {logs_url} returned HTTP {st}: {}", clip(&body));
        }
        let v: Value = serde_json::from_str(&body).context("logs response was not JSON")?;

        let out = v["stdout"].as_str().unwrap_or("");
        if let Some(new) = new_bytes(out, printed_out) {
            print!("{new}");
            let _ = std::io::Write::flush(&mut std::io::stdout());
            printed_out = out.len();
        }
        let err_out = v["stderr"].as_str().unwrap_or("");
        if let Some(new) = new_bytes(err_out, printed_err) {
            eprint!("{new}");
            let _ = std::io::Write::flush(&mut std::io::stderr());
            printed_err = err_out.len();
        }

        let state = v["status"].as_str().unwrap_or("");
        if DONE.contains(&state) {
            // No final re-print is needed: this response IS the authoritative
            // DB copy, and the streaming block above already emitted its
            // unprinted suffix (blnq's buffer holds the same capped prefix the
            // final columns contain — runStream.js, "never printed twice").
            let remote_exit = &v["exitCode"];
            let secs = started.elapsed().as_secs();
            match state {
                "completed" => {
                    let code = remote_exit.as_i64().unwrap_or(0);
                    eprintln!("── completed (exit {code}) in {secs}s");
                    if code == 0 {
                        return Ok(());
                    }
                    std::process::exit(code.clamp(1, 255) as i32);
                }
                "timeout" => {
                    eprintln!("── timed out on the hive after {secs}s");
                    std::process::exit(124);
                }
                "rejected" => {
                    eprintln!("── rejected by the hive after {secs}s");
                    std::process::exit(2);
                }
                _ => {
                    // failed: /logs rarely carries the cause (the executor's
                    // outer .catch writes error_message only, stderr NULL),
                    // so read it off the run row itself.
                    let mut detail = v["stderr"]
                        .as_str()
                        .filter(|s| !s.is_empty())
                        .map(clip)
                        .unwrap_or_default();
                    if detail.is_empty() {
                        if let Ok(row) = client
                            .get(format!("{door}/{id}"))
                            .timeout(Duration::from_secs(20))
                            .header("X-Run-Token", token)
                            .send()
                            .await
                        {
                            if let Ok(body) = row.text().await {
                                if let Some(em) = serde_json::from_str::<Value>(&body)
                                    .ok()
                                    .and_then(|r| r["error_message"].as_str().map(str::to_string))
                                    .filter(|s| !s.is_empty())
                                {
                                    detail = clip(&em);
                                }
                            }
                        }
                    }
                    bail!(
                        "run failed after {secs}s{}",
                        if detail.is_empty() {
                            String::new()
                        } else {
                            format!(": {detail}")
                        }
                    );
                }
            }
        }
    }
}

/// Render a 403 rejection card as one message.
///
/// The server has two shapes (routes/run.js): the pattern gate emits
/// `safety:{level, reason, review:'pattern'}`, the AI review emits
/// `safety:{level, reason, review, reviewer}` — both carry a pre-minted
/// `id`. The reason is contractually FULL ("no mystery cuts in transit"),
/// so it is shown verbatim; only a missing/blank reason gets a placeholder.
fn rejection(text: &str) -> String {
    let v: Value = serde_json::from_str(text).unwrap_or(Value::Null);
    let level = v["safety"]["level"].as_str().unwrap_or("?");
    let reason = v["safety"]["reason"]
        .as_str()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("(no reason given)");
    let review = v["safety"]["review"].as_str().unwrap_or("?");
    let id = v["id"].as_str().unwrap_or("?");
    format!(
        "rejected by the hive's safety review (level {level}, via {review}):\n{reason}\n\
         run id {id} — quote it in a bug report if this is wrong"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_comes_from_the_extension_unless_explicit() {
        assert_eq!(language_for(Path::new("a.py"), None).unwrap(), "python");
        assert_eq!(language_for(Path::new("a.js"), None).unwrap(), "node");
        assert_eq!(language_for(Path::new("a.mjs"), None).unwrap(), "node");
        assert_eq!(language_for(Path::new("a.sh"), None).unwrap(), "bash");
        assert_eq!(language_for(Path::new("a.cpp"), None).unwrap(), "cpp");
        assert_eq!(language_for(Path::new("a.go"), None).unwrap(), "go");
        // explicit wins over a misleading extension
        assert_eq!(
            language_for(Path::new("a.txt"), Some("python")).unwrap(),
            "python"
        );
        // unknown extension is a documented failure, not a guess
        let err = language_for(Path::new("a.weird"), None).unwrap_err();
        assert!(err.to_string().contains("--language"), "{err}");
        // no extension at all
        assert!(language_for(Path::new("Makefile"), None).is_err());
    }

    #[test]
    fn only_unprinted_suffixes_are_ever_emitted() {
        assert_eq!(new_bytes("hello", 0), Some("hello"));
        assert_eq!(new_bytes("hello", 5), None); // nothing new
        assert_eq!(new_bytes("hello!", 5), Some("!")); // append-only growth
        assert_eq!(new_bytes("hi", 10), None); // buffer shrank: print nothing
    }

    #[test]
    fn suffix_slicing_never_splits_utf8() {
        // "héllo" — é is bytes 1..3; printed=2 lands mid-character and must
        // yield None rather than panic or emit mojibake.
        assert_eq!(new_bytes("héllo", 2), None);
        assert_eq!(new_bytes("héllo", 3), Some("llo"));
        assert_eq!(new_bytes("héllo", 1), Some("éllo"));
    }

    #[test]
    fn read_script_names_the_missing_file() {
        // Tests the real helper run() calls, not a re-implementation.
        let err = read_script(Path::new("/nonexistent/definitely-missing.py")).unwrap_err();
        assert!(err.to_string().contains("definitely-missing"));
    }

    #[test]
    fn rejection_card_parses_both_server_shapes() {
        // The AI review's shape (routes/run.js ~557).
        let ai = r#"{"id":"run-a","error":"Code rejected by AI safety review",
            "safety":{"level":"high","reason":"opens an outbound socket","review":"ai"}}"#;
        let msg = rejection(ai);
        assert!(msg.contains("run-a"), "{msg}");
        assert!(msg.contains("opens an outbound socket"), "{msg}");
        assert!(msg.contains("high"), "{msg}");
        assert!(msg.contains("via ai"), "{msg}");

        // The pattern gate's shape (routes/run.js ~318) — no error field,
        // review is the literal 'pattern'.
        let pattern = r#"{"id":"run-b","safety":{"level":"high",
            "reason":"matches a dangerous pattern","review":"pattern"}}"#;
        let msg = rejection(pattern);
        assert!(msg.contains("run-b"), "{msg}");
        assert!(msg.contains("matches a dangerous pattern"), "{msg}");
        assert!(msg.contains("via pattern"), "{msg}");

        // Blank reason must not print an empty line.
        let msg =
            rejection(r#"{"id":"run-c","safety":{"level":"low","reason":"  ","review":"ai"}}"#);
        assert!(msg.contains("(no reason given)"), "{msg}");

        // Not JSON at all: still a usable message, never a panic.
        let msg = rejection("<html>gateway error</html>");
        assert!(msg.contains("run id ?"), "{msg}");
    }
}
