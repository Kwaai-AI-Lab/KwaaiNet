//! KwaaiNet map API — fetch the network state and pick the best local model.
//!
//! The map at <https://map.kwaai.ai/api/v1/state> returns a list of model
//! reports, each describing how many servers are currently serving that model.
//! We use this to choose which locally-installed Ollama model to serve so we
//! contribute to the most popular (most-needed) model on the network.

use anyhow::Result;
use serde::Deserialize;

// ---------------------------------------------------------------------------
// API types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct MapState {
    pub model_reports: Vec<MapModelReport>,
}

#[derive(Debug, Deserialize)]
pub struct MapModelReport {
    /// HuggingFace-style identifier, e.g. `"unsloth/Llama-3.1-8B-Instruct"`.
    pub short_name: String,
    /// DHT key prefix, e.g. `"Llama-3-1-8B-Instruct-hf"`.
    #[serde(default)]
    pub dht_prefix: String,
    /// Total transformer blocks in the full model.
    #[serde(default)]
    pub num_blocks: u32,
    /// Each entry is one server row — we only need the count.
    #[serde(default)]
    pub server_rows: Vec<serde_json::Value>,
}

impl MapModelReport {
    pub fn server_count(&self) -> usize {
        self.server_rows.len()
    }
}

// ---------------------------------------------------------------------------
// Network fetch
// ---------------------------------------------------------------------------

/// Fetch the current network state from the map API.
/// Uses a short timeout so startup is not blocked if the API is unreachable.
pub async fn fetch_map(endpoint: &str) -> Result<MapState> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()?;
    let state = client
        .get(endpoint)
        .send()
        .await?
        .json::<MapState>()
        .await?;
    Ok(state)
}

// ---------------------------------------------------------------------------
// Fuzzy name matching
// ---------------------------------------------------------------------------

/// Reduce a name to lowercase alphanumeric chars for comparison.
fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

/// Try to match a local Ollama model reference against a map model entry.
///
/// Returns a score:
/// - `2` — strong match (full ref, including tag, is a substring of the map name)
/// - `1` — partial match (base name without tag matches)
/// - `0` — no match
///
/// Example: `"llama3.1:8b"` → `"unsloth/Llama-3.1-8B-Instruct"`:
/// - `normalize("llama3.1:8b")` = `"llama318b"`
/// - `normalize("Llama-3.1-8B-Instruct")` = `"llama318binstruct"`
/// - `"llama318binstruct".contains("llama318b")` → score 2 ✓
pub fn match_score(ollama_ref: &str, map_model: &MapModelReport) -> u32 {
    let full_norm = normalize(ollama_ref);
    let base_norm = normalize(ollama_ref.split(':').next().unwrap_or(ollama_ref));

    // Three candidates from the map entry to check against.
    let model_part = map_model
        .short_name
        .split('/')
        .last()
        .unwrap_or(&map_model.short_name);
    let candidates = [
        normalize(&map_model.short_name),
        normalize(&map_model.dht_prefix),
        normalize(model_part),
    ];

    for c in &candidates {
        if c.is_empty() {
            continue;
        }
        // Strong: full ref (name + size tag) found in map name or vice versa.
        if c.contains(&full_norm) || full_norm.contains(c.as_str()) {
            return 2;
        }
        // Partial: base model name (without tag) matches.
        if !base_norm.is_empty() && (c.contains(&base_norm) || base_norm.contains(c.as_str())) {
            return 1;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Model selection
// ---------------------------------------------------------------------------

/// Result of picking a model: the Ollama reference to use plus the matching
/// map model name (for display purposes).
pub struct ModelChoice {
    /// Local Ollama model reference, e.g. `"llama3.1:8b"`.
    pub ollama_ref: String,
    /// Matched map model name, e.g. `"unsloth/Llama-3.1-8B-Instruct"`.
    pub map_name: Option<String>,
    /// Number of servers currently serving the matched map model.
    pub server_count: usize,
}

/// Given locally available models and the live network map, choose the best
/// model to serve.
///
/// Selection strategy (in priority order):
/// 1. Local model that matches a map entry **and** has the most servers
///    (i.e. the model is most in demand on the network).
/// 2. If the currently-configured model already has a good match, keep it
///    (avoid unnecessary model switches for stable nodes).
/// 3. If no local model matches the map at all, return `None` (caller keeps
///    whatever is configured).
pub fn pick_best_model(
    local_models: &[String],
    map: &MapState,
    current_model: &str,
) -> Option<ModelChoice> {
    if local_models.is_empty() || map.model_reports.is_empty() {
        return None;
    }

    // Score every local model against the map.
    let mut scored: Vec<(String, u32, usize, String)> = local_models
        .iter()
        .filter_map(|local| {
            // Find the best-matching map entry for this local model.
            map.model_reports
                .iter()
                .filter_map(|r| {
                    let s = match_score(local, r);
                    if s > 0 {
                        Some((s, r.server_count(), r.short_name.clone()))
                    } else {
                        None
                    }
                })
                .max_by_key(|&(score, count, _)| score as usize * 100_000 + count)
                .map(|(score, count, map_name)| (local.clone(), score, count, map_name))
        })
        .collect();

    if scored.is_empty() {
        return None;
    }

    // Sort: strong match first, then by server count (most popular first).
    scored.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)));

    // If the current model is in the list with a competitive score, keep it.
    if let Some(pos) = scored.iter().position(|(m, _, _, _)| m == current_model) {
        let (_, cur_score, cur_count, ref cur_map) = scored[pos];
        let (_, best_score, best_count, _) = scored[0];
        if cur_score == best_score && best_count <= cur_count + 2 {
            return Some(ModelChoice {
                ollama_ref: current_model.to_string(),
                map_name: Some(cur_map.clone()),
                server_count: cur_count,
            });
        }
    }

    let (ref best_local, _, best_count, ref best_map) = scored[0];
    Some(ModelChoice {
        ollama_ref: best_local.clone(),
        map_name: Some(best_map.clone()),
        server_count: best_count,
    })
}
