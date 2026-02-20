//! Resolve an Ollama model reference to the local GGUF blob path.
//!
//! Ollama can store models in two different directory layouts depending on
//! how it was configured:
//!
//! **Default layout** (`~/.ollama`):
//!   `~/.ollama/models/manifests/<registry>/<namespace>/<model>/<tag>`
//!   `~/.ollama/models/blobs/sha256-<hex>`
//!
//! **Custom layout** (`OLLAMA_MODELS=<path>` or a detected custom dir):
//!   `<path>/manifests/<registry>/<namespace>/<model>/<tag>`
//!   `<path>/blobs/sha256-<hex>`
//!
//! We detect which layout is in use by checking whether `<dir>/blobs/`
//! exists directly (custom) or only under `<dir>/models/blobs/` (default).

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

/// Resolve an Ollama model reference to the GGUF blob path on disk.
///
/// Accepted formats:
/// - `qwen3`                              → library/qwen3:latest
/// - `qwen3:0.6b`                         → library/qwen3:0.6b
/// - `hf.co/microsoft/bitnet-b1.58-2B-4T-gguf:latest`  → hf.co path
pub fn resolve_model_blob(model_ref: &str) -> Result<PathBuf> {
    let (models_root, blobs_root) = find_ollama_roots()?;

    let manifest_path = find_manifest(model_ref, &models_root)
        .with_context(|| format!("Cannot locate Ollama manifest for '{model_ref}'"))?;

    let content = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("Cannot read {}", manifest_path.display()))?;

    let manifest: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| "Manifest is not valid JSON")?;

    // Find the layer that carries the model weights.
    let layers = manifest["layers"]
        .as_array()
        .ok_or_else(|| anyhow!("Manifest has no 'layers' array"))?;

    let model_layer = layers
        .iter()
        .find(|l| {
            l["mediaType"].as_str()
                == Some("application/vnd.ollama.image.model")
        })
        .ok_or_else(|| anyhow!("No model layer found in manifest"))?;

    let digest = model_layer["digest"]
        .as_str()
        .ok_or_else(|| anyhow!("Model layer has no 'digest' field"))?;

    // "sha256:abc123…" → "sha256-abc123…"
    let blob_name = digest.replace(':', "-");
    let blob_path = blobs_root.join(&blob_name);

    if !blob_path.exists() {
        return Err(anyhow!(
            "Blob '{}' not found at {}.\nTry: ollama pull {}",
            blob_name,
            blob_path.display(),
            model_ref
        ));
    }

    Ok(blob_path)
}

/// Find the manifest file for a model reference.
fn find_manifest(model_ref: &str, manifests_root: &PathBuf) -> Result<PathBuf> {
    // Split off the tag, defaulting to "latest".
    let (name, tag) = model_ref
        .rsplit_once(':')
        .unwrap_or((model_ref, "latest"));

    // Candidates tried in order:
    //   1. registry.ollama.ai/library/<name>/<tag>  — standard Ollama library
    //   2. <name>/<tag>                              — fully-qualified (hf.co/…)
    let candidates = [
        manifests_root.join("registry.ollama.ai/library").join(name).join(tag),
        manifests_root.join(name).join(tag),
    ];

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(anyhow!(
        "Model '{}' is not pulled locally.\nRun: ollama pull {}",
        model_ref, model_ref
    ))
}

/// Return `(manifests_root, blobs_root)` by probing the possible Ollama
/// storage layouts in priority order:
///
/// 1. `OLLAMA_MODELS` env var (custom layout: `$dir/manifests/`, `$dir/blobs/`)
/// 2. Common macOS custom paths under `~/Documents` (same layout)
/// 3. Default `~/.ollama/models/` (default layout)
fn find_ollama_roots() -> Result<(PathBuf, PathBuf)> {
    let home = std::env::var("HOME").map_err(|_| anyhow!("$HOME is not set"))?;
    let home = PathBuf::from(home);

    // Candidate roots to probe, in priority order.
    let mut candidates: Vec<PathBuf> = Vec::new();

    // 1. Explicit OLLAMA_MODELS override.
    if let Ok(custom) = std::env::var("OLLAMA_MODELS") {
        candidates.push(PathBuf::from(custom));
    }

    // 2. Well-known custom locations (used by the Kwaai desktop app).
    for sub in &["Documents/Kwaai/ollama", "Documents/ollama"] {
        candidates.push(home.join(sub));
    }

    // For each candidate check whether it uses the "custom" layout
    // (blobs/ directly under the root) and return if found.
    for dir in &candidates {
        let blobs = dir.join("blobs");
        let manifests = dir.join("manifests");
        if blobs.is_dir() && manifests.is_dir() {
            return Ok((manifests, blobs));
        }
    }

    // 3. Default ~/.ollama with the `models/` subdirectory.
    let default = home.join(".ollama").join("models");
    Ok((default.join("manifests"), default.join("blobs")))
}
