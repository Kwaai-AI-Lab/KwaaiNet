//! Resolve a HuggingFace model ID to a local SafeTensors snapshot directory.
//!
//! Searches the Petals cache and HuggingFace Hub cache for a fully-downloaded
//! snapshot of the requested model.
//!
//! Model IDs use the standard HuggingFace format: `owner/model-name`
//! e.g. `unsloth/Llama-3.1-8B-Instruct`

use anyhow::{anyhow, Result};
use std::path::PathBuf;

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
             To download: huggingface-cli download {}",
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
    shards
        .iter()
        .all(|p| std::fs::metadata(p).is_ok())
}

/// Return the list of directories to search for HuggingFace model caches.
fn cache_roots() -> Result<Vec<PathBuf>> {
    let home = std::env::var("HOME").map_err(|_| anyhow!("$HOME is not set"))?;
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

    // Standard HuggingFace Hub cache.
    let hf = PathBuf::from(&home).join(".cache/huggingface");
    if hf.exists() {
        roots.push(hf.clone());
        // Fallback: some setups nest the cache one level deeper.
        let nested = hf.join("huggingface");
        if nested.exists() {
            roots.push(nested);
        }
    }

    Ok(roots)
}
