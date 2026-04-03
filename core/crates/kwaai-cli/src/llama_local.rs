//! llama.cpp local inference backend for Apple Silicon.
//!
//! Uses llama-cpp-2 Rust bindings with Metal GPU acceleration for
//! full-model inference at 50+ tok/s on M-series Macs.
//!
//! Feature-gated: only compiled with `--features llama-cpp`.

#[cfg(feature = "llama-cpp")]
use anyhow::{Context, Result};

#[cfg(feature = "llama-cpp")]
use llama_cpp_2::{
    context::params::LlamaContextParams,
    llama_backend::LlamaBackend,
    llama_batch::LlamaBatch,
    model::{params::LlamaModelParams, LlamaModel},
    token::LlamaToken,
};

#[cfg(feature = "llama-cpp")]
use std::io::Write as _;
#[cfg(feature = "llama-cpp")]
use std::path::Path;

#[cfg(feature = "llama-cpp")]
pub struct GenerationResult {
    pub text: String,
    pub tokens_generated: usize,
    pub prefill_ms: f64,
    pub decode_ms: f64,
}

/// Load a GGUF model via llama.cpp. Returns the backend and model for reuse
/// across multiple inference calls.
#[cfg(feature = "llama-cpp")]
pub fn load_model(model_path: &Path) -> Result<(LlamaBackend, LlamaModel)> {
    let backend = LlamaBackend::init().context("Failed to init llama.cpp backend")?;
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, model_path, &model_params)
        .map_err(|e| anyhow::anyhow!("Failed to load model: {e:?}"))?;
    Ok((backend, model))
}

/// Sample the next token from logits with temperature, top-k, and top-p.
#[cfg(feature = "llama-cpp")]
fn sample_next_token(
    logits: &[f32],
    eos: LlamaToken,
    temperature: f32,
    top_k: usize,
    top_p: f32,
) -> LlamaToken {
    if temperature <= 0.0 {
        // Greedy: argmax
        return logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| LlamaToken::new(i as i32))
            .unwrap_or(eos);
    }

    // Temperature scaling
    let scaled: Vec<f32> = logits.iter().map(|&l| l / temperature).collect();

    // Sort indices by descending logit for top-k / top-p filtering
    let mut indices: Vec<usize> = (0..scaled.len()).collect();
    indices.sort_unstable_by(|&a, &b| {
        scaled[b].partial_cmp(&scaled[a]).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Top-k: keep only the top k candidates
    if top_k > 0 && top_k < indices.len() {
        indices.truncate(top_k);
    }

    // Softmax over remaining candidates
    let max_val = scaled[indices[0]];
    let mut probs: Vec<(usize, f32)> = indices
        .iter()
        .map(|&i| (i, (scaled[i] - max_val).exp()))
        .collect();
    let sum: f32 = probs.iter().map(|(_, p)| p).sum();
    for item in &mut probs {
        item.1 /= sum;
    }

    // Top-p (nucleus): accumulate probs and cut off
    if top_p > 0.0 && top_p < 1.0 {
        let mut cumsum = 0.0f32;
        let mut cutoff = probs.len();
        for (i, &(_, p)) in probs.iter().enumerate() {
            cumsum += p;
            if cumsum >= top_p {
                cutoff = i + 1;
                break;
            }
        }
        probs.truncate(cutoff);
        // Renormalize
        let new_sum: f32 = probs.iter().map(|(_, p)| p).sum();
        for item in &mut probs {
            item.1 /= new_sum;
        }
    }

    // Random sample
    let r: f32 = {
        use std::time::SystemTime;
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        (nanos as f32) / (u32::MAX as f32)
    };
    let mut cumsum = 0.0f32;
    for &(idx, p) in &probs {
        cumsum += p;
        if cumsum >= r {
            return LlamaToken::new(idx as i32);
        }
    }
    // Fallback to last candidate
    LlamaToken::new(probs.last().map(|&(i, _)| i).unwrap_or(0) as i32)
}

/// Run local inference via llama.cpp with Metal acceleration.
///
/// Loads a GGUF model, tokenizes the prompt, and generates tokens
/// using llama.cpp's optimized Metal pipeline.
#[cfg(feature = "llama-cpp")]
pub fn run_inference(
    model_path: &Path,
    prompt: &str,
    max_tokens: usize,
    temperature: f32,
) -> Result<GenerationResult> {
    use crate::display::*;

    print_info("Loading GGUF model via llama.cpp...");
    let (backend, model) = load_model(model_path)?;

    run_inference_with_model(&backend, &model, prompt, max_tokens, temperature, 0, 1.0, |piece| {
        print!("{piece}");
        std::io::stdout().flush().ok();
        true
    })
}

/// Run streaming inference using a pre-loaded model.
///
/// Calls `on_token` with each generated text piece. Return `false` from the
/// callback to stop generation early (e.g. client disconnected).
#[cfg(feature = "llama-cpp")]
pub fn run_inference_streaming(
    backend: &LlamaBackend,
    model: &LlamaModel,
    prompt: &str,
    max_tokens: usize,
    temperature: f32,
    top_k: usize,
    top_p: f32,
    on_token: impl Fn(String) -> bool,
) -> Result<GenerationResult> {
    run_inference_with_model(backend, model, prompt, max_tokens, temperature, top_k, top_p, on_token)
}

/// Core inference loop shared by `run_inference` and `run_inference_streaming`.
#[cfg(feature = "llama-cpp")]
fn run_inference_with_model(
    backend: &LlamaBackend,
    model: &LlamaModel,
    prompt: &str,
    max_tokens: usize,
    temperature: f32,
    top_k: usize,
    top_p: f32,
    on_token: impl Fn(String) -> bool,
) -> Result<GenerationResult> {
    // Create context
    let ctx_params = LlamaContextParams::default().with_n_ctx(std::num::NonZeroU32::new(2048));
    let mut ctx = model
        .new_context(backend, ctx_params)
        .map_err(|e| anyhow::anyhow!("Failed to create context: {e:?}"))?;

    // Tokenize prompt
    let tokens = model
        .str_to_token(prompt, llama_cpp_2::model::AddBos::Always)
        .map_err(|e| anyhow::anyhow!("Tokenize failed: {e:?}"))?;

    let n_prompt = tokens.len();
    let eos = model.token_eos();

    // Prefill: process all prompt tokens
    let prefill_start = std::time::Instant::now();
    let mut batch = LlamaBatch::new(n_prompt.max(1), 1);
    for (i, &tok) in tokens.iter().enumerate() {
        let is_last = i == n_prompt - 1;
        batch
            .add(tok, i as i32, &[0], is_last)
            .map_err(|e| anyhow::anyhow!("Batch add failed: {e:?}"))?;
    }
    ctx.decode(&mut batch)
        .map_err(|e| anyhow::anyhow!("Prefill decode failed: {e:?}"))?;
    let prefill_ms = prefill_start.elapsed().as_secs_f64() * 1000.0;

    // Decode loop
    let decode_start = std::time::Instant::now();
    let mut generated = Vec::new();
    let mut pos = n_prompt as i32;

    for _ in 0..max_tokens {
        let logits = ctx.get_logits_ith((batch.n_tokens() - 1) as i32);
        let next_token = sample_next_token(logits, eos, temperature, top_k, top_p);

        // Check EOS
        if model.is_eog_token(next_token) {
            break;
        }

        // Decode token to text and send via callback
        if let Ok(piece) = model.token_to_str(
            next_token,
            #[allow(deprecated)]
            llama_cpp_2::model::Special::Tokenize,
        ) {
            if !on_token(piece) {
                break; // receiver signalled stop
            }
        }

        generated.push(next_token);

        // Prepare next batch (single token)
        batch.clear();
        batch
            .add(next_token, pos, &[0], true)
            .map_err(|e| anyhow::anyhow!("Batch add failed: {e:?}"))?;
        pos += 1;

        ctx.decode(&mut batch)
            .map_err(|e| anyhow::anyhow!("Decode failed: {e:?}"))?;
    }

    let decode_ms = decode_start.elapsed().as_secs_f64() * 1000.0;

    // Collect full text
    let mut text = String::new();
    for tok in &generated {
        if let Ok(piece) = model.token_to_str(
            *tok,
            #[allow(deprecated)]
            llama_cpp_2::model::Special::Tokenize,
        ) {
            text.push_str(&piece);
        }
    }

    Ok(GenerationResult {
        text,
        tokens_generated: generated.len(),
        prefill_ms,
        decode_ms,
    })
}

/// Check if llama.cpp backend is available (feature compiled in).
pub fn is_available() -> bool {
    cfg!(feature = "llama-cpp")
}

#[cfg(all(test, feature = "llama-cpp"))]
mod tests {
    use super::*;

    #[test]
    fn test_llama_local_inference() {
        let model_path = dirs::home_dir()
            .expect("home")
            .join(".kwaainet/models/llama-3.1-8b-instruct-q4_k_m.gguf");

        if !model_path.exists() {
            eprintln!("[SKIP] test_llama_local_inference — GGUF model not found");
            return;
        }

        eprintln!("  Loading model via llama.cpp...");
        let t0 = std::time::Instant::now();
        let (backend, model) = load_model(&model_path).expect("model load failed");
        let result = run_inference_streaming(
            &backend,
            &model,
            "The capital of France is",
            20,
            0.0,
            0,
            1.0,
            |piece| {
                eprint!("{piece}");
                true
            },
        )
        .expect("inference failed");
        let total = t0.elapsed().as_secs_f64();

        let tps = if result.decode_ms > 0.0 {
            result.tokens_generated as f64 / (result.decode_ms / 1000.0)
        } else {
            0.0
        };

        eprintln!();
        eprintln!("  ── llama.cpp Results ──────────────────────────────");
        eprintln!("  Text:      {:?}", result.text.trim());
        eprintln!("  Tokens:    {}", result.tokens_generated);
        eprintln!("  Prefill:   {:.0}ms", result.prefill_ms);
        eprintln!("  Decode:    {:.0}ms ({:.1} tok/s)", result.decode_ms, tps);
        eprintln!("  Total:     {:.1}s", total);
        eprintln!("  ──────────────────────────────────────────────────");
        eprintln!("[OK] test_llama_local_inference");

        assert!(
            result.tokens_generated > 0,
            "should generate at least 1 token"
        );
        assert!(
            result.text.to_lowercase().contains("paris"),
            "should mention Paris"
        );
    }
}
