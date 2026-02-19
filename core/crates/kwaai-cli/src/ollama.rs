//! Resolve an Ollama model reference to the local GGUF blob path.
//!
//! Ollama stores models in two places under `~/.ollama/models/`:
//!   manifests/<registry>/<namespace>/<model>/<tag>  — JSON manifest
//!   blobs/sha256-<hex>                               — raw content blobs
//!
//! The manifest's `layers` array contains one entry with
//! `mediaType = "application/vnd.ollama.image.model"` whose `digest`
//! (`sha256:<hex>`) maps to the GGUF blob file.

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

/// Resolve an Ollama model reference to the GGUF blob path on disk.
///
/// Accepted formats:
/// - `qwen3`                              → library/qwen3:latest
/// - `qwen3:0.6b`                         → library/qwen3:0.6b
/// - `hf.co/microsoft/bitnet-b1.58-2B-4T-gguf:latest`  → hf.co path
pub fn resolve_model_blob(model_ref: &str) -> Result<PathBuf> {
    let manifest_path = find_manifest(model_ref)
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

    let blob_path = ollama_home()?.join("models/blobs").join(&blob_name);

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
fn find_manifest(model_ref: &str) -> Result<PathBuf> {
    let base = ollama_home()?.join("models/manifests");

    // Split off the tag, defaulting to "latest".
    let (name, tag) = model_ref
        .rsplit_once(':')
        .unwrap_or((model_ref, "latest"));

    // Candidates tried in order:
    //   1. registry.ollama.ai/library/<name>/<tag>  — standard Ollama library
    //   2. <name>/<tag>                              — fully-qualified (hf.co/…)
    let candidates = [
        base.join("registry.ollama.ai/library").join(name).join(tag),
        base.join(name).join(tag),
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

fn ollama_home() -> Result<PathBuf> {
    // Respect OLLAMA_MODELS env var if set, otherwise use ~/.ollama
    if let Ok(custom) = std::env::var("OLLAMA_MODELS") {
        return Ok(PathBuf::from(custom));
    }
    let home = std::env::var("HOME").map_err(|_| anyhow!("$HOME is not set"))?;
    Ok(PathBuf::from(home).join(".ollama"))
}
