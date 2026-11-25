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
//! code so a calling pipeline sees the truth, not a translation of it.

use anyhow::{bail, Context, Result};
use serde_json::Value;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::hive::fetch_manifest;

/// The peer's terminal run states.
const DONE: &[&str] = &["completed", "failed", "timeout", "rejected"];

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
        _ => bail!(
            "cannot infer a language from .{ext} — pass --language \
             (blnq runs: python, node, ruby, bash, php, perl, c, cpp)"
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
    let code = std::fs::read_to_string(&args.file)
        .with_context(|| format!("reading {}", args.file.display()))?;
    let language = language_for(&args.file, args.language.as_deref())?;

    // No redirects: the door must answer as itself (see hive::http_client).
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .context("building the HTTP client")?;

    let m = fetch_manifest(&client, &args.target).await?;
    for w in &m.warnings {
        println!("warning    {w}");
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

    println!(
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
        let v: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
        let level = v["safety"]["level"].as_str().unwrap_or("?");
        let reason = v["safety"]["reason"]
            .as_str()
            .unwrap_or("(no reason given)");
        let review = v["safety"]["review"].as_str().unwrap_or("?");
        let id = v["id"].as_str().unwrap_or("?");
        bail!(
            "rejected by AI safety review (level {level}, via {review}):\n{reason}\n\
             run id {id} — quote it in a bug report if this is wrong"
        );
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
    println!("submitted {id} (safety: pattern {pattern}, ai {ai})");

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
            // Final response is the authoritative DB copy; print anything the
            // buffer had not shown yet.
            if let Some(new) = new_bytes(out, printed_out) {
                print!("{new}");
            }
            if let Some(new) = new_bytes(err_out, printed_err) {
                eprint!("{new}");
            }
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
                    // failed
                    let detail = v["stderr"].as_str().filter(|s| !s.is_empty());
                    bail!(
                        "run failed after {secs}s{}",
                        detail.map(|d| format!(": {}", clip(d))).unwrap_or_default()
                    );
                }
            }
        }
    }
}

fn user_agent() -> String {
    format!("kwaainet/{}/bridge", env!("CARGO_PKG_VERSION"))
}

fn clip(s: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn language_comes_from_the_extension_unless_explicit() {
        assert_eq!(language_for(Path::new("a.py"), None).unwrap(), "python");
        assert_eq!(language_for(Path::new("a.js"), None).unwrap(), "node");
        assert_eq!(language_for(Path::new("a.mjs"), None).unwrap(), "node");
        assert_eq!(language_for(Path::new("a.sh"), None).unwrap(), "bash");
        assert_eq!(language_for(Path::new("a.cpp"), None).unwrap(), "cpp");
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
    fn run_args_file_is_read_before_anything_else() {
        // The missing-file path must name the file, not some later step.
        let args = crate::cli::RunArgs {
            target: "example.org".into(),
            file: PathBuf::from("/nonexistent/definitely-missing.py"),
            language: None,
            stdin: None,
            args: vec![],
            public: false,
            timeout: 1,
        };
        // the same chain run() uses: the context is what names the file
        let err = std::fs::read_to_string(&args.file)
            .with_context(|| format!("reading {}", args.file.display()))
            .unwrap_err();
        assert!(err.to_string().contains("definitely-missing"));
    }
}
