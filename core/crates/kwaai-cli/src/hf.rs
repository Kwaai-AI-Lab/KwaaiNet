//! Resolve and download HuggingFace model SafeTensors snapshots.
//!
//! Searches the Petals cache and HuggingFace Hub cache for a fully-downloaded
//! snapshot of the requested model, and can download models directly via the
//! HuggingFace Hub HTTP API (no Python or huggingface-cli required).
//!
//! Model IDs use the standard HuggingFace format: `owner/model-name`
//! e.g. `unsloth/Llama-3.1-8B-Instruct`

use anyhow::{anyhow, bail, Context, Result};
use futures::StreamExt;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

/// Resolve a HuggingFace model ID to a snapshot directory containing
/// `.safetensors` weight shards and `config.json`.
///
/// Searches ALL known cache locations and returns the most complete snapshot
/// (highest shard count). This handles cases where one cache copy has only
/// a partial download while another is fully downloaded.
///
/// Cache roots searched:
///   - `$HF_HOME/` (if set)
///   - `~/.cache/petals/`
///   - `~/.cache/huggingface/`
///   - `~/.cache/huggingface/huggingface/` (misconfigured HF_HOME fallback)
pub fn resolve_snapshot(model_id: &str) -> Result<PathBuf> {
    // HuggingFace converts `owner/model` → directory name `models--owner--model`
    let dir_name = format!("models--{}", model_id.replace('/', "--"));

    let roots = cache_roots()?;

    // Collect all valid snapshots across every cache root, tagged with shard count.
    let mut candidates: Vec<(PathBuf, usize)> = Vec::new();

    for root in &roots {
        let model_dir = root.join(&dir_name);
        if !model_dir.exists() {
            continue;
        }
        if let Some(snapshot) = find_best_snapshot(&model_dir)? {
            let shards = count_valid_shards(&snapshot);
            candidates.push((snapshot, shards));
        }
    }

    if candidates.is_empty() {
        let searched = roots
            .iter()
            .map(|p| format!("  • {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(anyhow!(
            "Model '{}' not found in local cache.\nSearched:\n{}\n\
             To download: kwaainet shard download {}",
            model_id,
            searched,
            model_id
        ));
    }

    // Pick the most complete snapshot (most valid shards).
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(candidates.into_iter().next().unwrap().0)
}

/// Find the snapshot directory that has `.safetensors` weight files.
/// Prefers the snapshot with the most shards (most complete download),
/// breaking ties by most recently modified.
fn find_best_snapshot(model_dir: &std::path::Path) -> Result<Option<PathBuf>> {
    let snapshots_dir = model_dir.join("snapshots");
    if !snapshots_dir.exists() {
        return Ok(None);
    }

    let mut candidates: Vec<PathBuf> = std::fs::read_dir(&snapshots_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter(|p| has_safetensors_shards(p))
        .collect();

    // Most shards first (incomplete downloads are ranked lower),
    // then newest modification time as a tie-breaker.
    candidates.sort_by(|a, b| {
        let ca = count_valid_shards(a);
        let cb = count_valid_shards(b);
        cb.cmp(&ca).then_with(|| {
            let ta = a.metadata().and_then(|m| m.modified()).ok();
            let tb = b.metadata().and_then(|m| m.modified()).ok();
            tb.cmp(&ta)
        })
    });

    Ok(candidates.into_iter().next())
}

/// Count the number of readable `.safetensors` files in a directory.
fn count_valid_shards(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .ok()
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("safetensors"))
                .filter(|p| std::fs::metadata(p).is_ok())
                .count()
        })
        .unwrap_or(0)
}

/// Returns true if the directory contains at least one readable `.safetensors`
/// file AND a readable `config.json`. All safetensors symlinks must resolve to
/// existing files. Incomplete snapshots (missing config.json or broken symlinks)
/// return false.
fn has_safetensors_shards(dir: &std::path::Path) -> bool {
    // config.json must be present and readable (follows symlinks).
    if std::fs::metadata(dir.join("config.json")).is_err() {
        return false;
    }

    let shards: Vec<std::path::PathBuf> = match std::fs::read_dir(dir) {
        Ok(rd) => rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("safetensors"))
            .collect(),
        Err(_) => return false,
    };

    if shards.is_empty() {
        return false;
    }

    // Every shard must be readable — follows symlinks via std::fs::metadata.
    shards.iter().all(|p| std::fs::metadata(p).is_ok())
}

/// Return the list of directories to search for HuggingFace model caches.
fn cache_roots() -> Result<Vec<PathBuf>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("cannot determine home directory"))?;
    let mut roots: Vec<PathBuf> = Vec::new();

    // Explicit HF_HOME override (highest priority).
    if let Ok(hf_home) = std::env::var("HF_HOME") {
        let p = PathBuf::from(hf_home);
        if p.exists() {
            roots.push(p);
        }
    }

    // Petals cache (downloaded for distributed inference sessions).
    let petals = PathBuf::from(&home).join(".cache/petals");
    if petals.exists() {
        roots.push(petals);
    }

    // Standard HuggingFace Hub cache — hub/ is the canonical subdirectory.
    let hf_hub = PathBuf::from(&home).join(".cache/huggingface/hub");
    if hf_hub.exists() {
        roots.push(hf_hub);
    }

    // Legacy / misconfigured fallback (no hub/ subdir).
    let hf = PathBuf::from(&home).join(".cache/huggingface");
    if hf.exists() {
        roots.push(hf.clone());
        let nested = hf.join("huggingface");
        if nested.exists() {
            roots.push(nested);
        }
    }

    Ok(roots)
}

// ── Download ──────────────────────────────────────────────────────────────────

/// Download a HuggingFace model SafeTensors snapshot to the local cache.
///
/// Uses the HuggingFace Hub HTTP API — no Python or `huggingface-cli` required.
/// Files are written to `~/.cache/huggingface/hub/models--{owner}--{name}/snapshots/{sha}/`.
///
/// Only SafeTensors weights and tokenizer/config files are downloaded.
/// PyTorch `.bin` files, README, and other non-inference files are skipped.
///
/// Pass `hf_token` (or set `HF_TOKEN` env var) for private/gated models.
pub async fn download(model_id: &str, hf_token: Option<&str>) -> Result<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow!("cannot determine home directory"))?;

    let cache_root = if let Ok(hf_home) = std::env::var("HF_HOME") {
        PathBuf::from(hf_home)
    } else {
        home.join(".cache/huggingface/hub")
    };

    let token = hf_token
        .map(String::from)
        .or_else(|| std::env::var("HF_TOKEN").ok())
        .or_else(|| std::env::var("HUGGING_FACE_HUB_TOKEN").ok());

    let client = build_hf_client(token.as_deref())?;

    // Fetch model metadata: commit sha + file list.
    let api_url = format!("https://huggingface.co/api/models/{}", model_id);
    let resp = client
        .get(&api_url)
        .send()
        .await
        .context("Failed to reach huggingface.co — check your internet connection")?;

    match resp.status().as_u16() {
        200 => {}
        401 | 403 => bail!(
            "Model '{}' requires authentication.\n\
             Set the HF_TOKEN environment variable or pass --hf-token.",
            model_id
        ),
        404 => bail!("Model '{}' not found on HuggingFace Hub.", model_id),
        s => bail!("HuggingFace API returned HTTP {}", s),
    }

    let meta: serde_json::Value = resp
        .json()
        .await
        .context("Failed to parse HuggingFace API response")?;

    let sha = meta["sha"]
        .as_str()
        .ok_or_else(|| anyhow!("HF API response missing 'sha' field"))?
        .to_string();

    let siblings = meta["siblings"]
        .as_array()
        .ok_or_else(|| anyhow!("HF API response missing 'siblings' field"))?;

    let files: Vec<String> = siblings
        .iter()
        .filter_map(|s| s["rfilename"].as_str())
        .filter(|f| should_download(f))
        .map(String::from)
        .collect();

    if files.is_empty() {
        bail!(
            "No SafeTensors files found in '{}'. \
             This model may not be in SafeTensors format.",
            model_id
        );
    }

    // Create the snapshot directory.
    let dir_name = format!("models--{}", model_id.replace('/', "--"));
    let snapshot_dir = cache_root.join(&dir_name).join("snapshots").join(&sha);
    std::fs::create_dir_all(&snapshot_dir)
        .with_context(|| format!("Cannot create cache dir: {}", snapshot_dir.display()))?;

    println!("  Commit: {}", &sha[..sha.len().min(12)]);
    println!("  Files:  {}", files.len());
    println!("  Dest:   {}", snapshot_dir.display());
    println!();

    let n = files.len();
    for (i, fname) in files.iter().enumerate() {
        let dest = snapshot_dir.join(fname);

        if dest.exists() {
            let size = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
            println!(
                "  [{:2}/{n}] {fname}  — already cached ({})",
                i + 1,
                fmt_bytes(size)
            );
            continue;
        }

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/{}/{}",
            model_id, sha, fname
        );
        download_file(&client, &url, &dest, i + 1, n, fname)
            .await
            .with_context(|| format!("Failed to download '{}'", fname))?;
    }

    Ok(snapshot_dir)
}

/// Returns true for files needed by SafeTensors distributed inference.
fn should_download(fname: &str) -> bool {
    let lower = fname.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "config.json"
            | "generation_config.json"
            | "tokenizer.json"
            | "tokenizer_config.json"
            | "special_tokens_map.json"
            | "tokenizer.model"
            | "model.safetensors.index.json"
    ) || lower.ends_with(".safetensors")
}

fn build_hf_client(token: Option<&str>) -> Result<reqwest::Client> {
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    let mut headers = HeaderMap::new();
    if let Some(t) = token {
        let val =
            HeaderValue::from_str(&format!("Bearer {}", t)).context("Invalid HF token value")?;
        headers.insert(AUTHORIZATION, val);
    }
    Ok(reqwest::Client::builder()
        .user_agent("kwaainet/0.2 (huggingface_hub compatible)")
        .default_headers(headers)
        .build()?)
}

async fn download_file(
    client: &reqwest::Client,
    url: &str,
    dest: &Path,
    idx: usize,
    total: usize,
    fname: &str,
) -> Result<()> {
    use std::io::Write as _;

    let tmp = dest.with_file_name(format!(
        "{}.download",
        dest.file_name().unwrap_or_default().to_string_lossy()
    ));
    if tmp.exists() {
        let _ = std::fs::remove_file(&tmp);
    }

    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        bail!("HTTP {} downloading {}", resp.status(), fname);
    }

    let content_length = resp.content_length();
    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();
    let mut file = tokio::fs::File::create(&tmp).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(chunk.as_ref()).await?;
        downloaded += chunk.len() as u64;

        if let Some(total_bytes) = content_length {
            if total_bytes > 0 {
                let pct = downloaded * 100 / total_bytes;
                print!(
                    "\r  [{idx:2}/{total}] {fname}  {pct}%  ({}/{})",
                    fmt_bytes(downloaded),
                    fmt_bytes(total_bytes)
                );
                let _ = std::io::stdout().flush();
            }
        }
    }
    file.flush().await?;
    drop(file);

    let size = std::fs::metadata(&tmp)
        .map(|m| m.len())
        .unwrap_or(downloaded);
    // \r moves to column 0; \x1b[K clears to end-of-line (works on all modern
    // terminals incl. Windows Terminal / PowerShell; harmless on others).
    print!(
        "\r\x1b[K  [{idx:2}/{total}] {fname}  ✓  {}\n",
        fmt_bytes(size)
    );
    let _ = std::io::stdout().flush();

    std::fs::rename(&tmp, dest)
        .with_context(|| format!("Failed to rename {} -> {}", tmp.display(), dest.display()))?;

    Ok(())
}

fn fmt_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.0} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.0} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}
