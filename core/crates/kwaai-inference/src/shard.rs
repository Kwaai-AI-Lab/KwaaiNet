//! Distributed block sharding — Petals-style inter-node transformer inference.
//!
//! A [`TransformerShard`] holds a contiguous range of transformer blocks from a
//! SafeTensors Llama model.  Three forward modes cover the roles a node can play:
//!
//! * **First node** (`start_block == 0`): embeds token IDs, runs its blocks, returns
//!   hidden states `[1, seq_len, hidden_dim]`.
//! * **Middle node**: receives hidden states, runs its block slice, returns hidden states.
//! * **Last node** (`end_block == num_total_blocks`): receives hidden states, runs its
//!   blocks, applies the final RMSNorm + LM head, returns logits `[1, 1, vocab_size]`.
//!
//! KV-cache is managed per session (`session_id: u64`).  Sessions expire after 60 s of
//! inactivity; call [`TransformerShard::gc_sessions`] periodically.

use crate::{
    error::{InferenceError, InferenceResult},
    tokenizer::BpeTokenizer,
};
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder};
use std::{
    collections::HashMap,
    path::Path,
    sync::Mutex,
    time::Instant,
};
use tracing::{debug, info};

// ── Model hyperparameters ─────────────────────────────────────────────────────

/// Hyperparameters shared across all components of the shard.
#[derive(Debug, Clone)]
pub struct ShardConfig {
    pub num_total_blocks: usize,
    pub hidden_dim: usize,
    pub num_heads: usize,
    pub num_kv_heads: usize,
    /// `hidden_dim / num_heads`
    pub head_dim: usize,
    pub intermediate_dim: usize,
    pub vocab_size: usize,
    pub rope_theta: f64,
    pub max_seq_len: usize,
    pub rms_norm_eps: f64,
    pub dtype: DType,
}

impl ShardConfig {
    /// Number of query heads per KV head (GQA repeat factor).
    pub fn n_rep(&self) -> usize {
        self.num_heads / self.num_kv_heads
    }
}

// ── Precomputed RoPE tables ───────────────────────────────────────────────────

pub(crate) struct RopeCache {
    cos: Tensor, // [max_seq_len, head_dim/2]
    sin: Tensor, // [max_seq_len, head_dim/2]
}

impl RopeCache {
    fn new(cfg: &ShardConfig, device: &Device) -> InferenceResult<Self> {
        let half = cfg.head_dim / 2;
        let max = cfg.max_seq_len;
        let theta = cfg.rope_theta;

        let freqs: Vec<f64> = (0..half)
            .map(|i| 1.0 / theta.powf(2.0 * i as f64 / cfg.head_dim as f64))
            .collect();

        let mut cos_data = vec![0f32; max * half];
        let mut sin_data = vec![0f32; max * half];
        for pos in 0..max {
            for (i, &freq) in freqs.iter().enumerate() {
                let angle = pos as f64 * freq;
                cos_data[pos * half + i] = angle.cos() as f32;
                sin_data[pos * half + i] = angle.sin() as f32;
            }
        }

        let cos = Tensor::from_vec(cos_data, (max, half), device)
            .and_then(|t| t.to_dtype(cfg.dtype))
            .map_err(InferenceError::from)?;
        let sin = Tensor::from_vec(sin_data, (max, half), device)
            .and_then(|t| t.to_dtype(cfg.dtype))
            .map_err(InferenceError::from)?;

        Ok(Self { cos, sin })
    }

    /// Apply RoPE to query and key tensors.
    ///
    /// `q` / `k` shape: `[batch, n_heads, seq_len, head_dim]`
    fn apply(&self, q: &Tensor, k: &Tensor, seq_pos: usize) -> InferenceResult<(Tensor, Tensor)> {
        fn rotate(t: &Tensor, cos: &Tensor, sin: &Tensor) -> InferenceResult<Tensor> {
            let (_b, _h, _s, d) = t.dims4().map_err(InferenceError::from)?;
            let half = d / 2;
            let x1 = t.narrow(3, 0, half).map_err(InferenceError::from)?;
            let x2 = t.narrow(3, half, half).map_err(InferenceError::from)?;
            // cos/sin: [s, half] → broadcast [1, 1, s, half]
            let cos4 = cos.unsqueeze(0).and_then(|t| t.unsqueeze(0)).map_err(InferenceError::from)?;
            let sin4 = sin.unsqueeze(0).and_then(|t| t.unsqueeze(0)).map_err(InferenceError::from)?;
            let out1 = ((&x1 * &cos4).map_err(InferenceError::from)?
                - (&x2 * &sin4).map_err(InferenceError::from)?)
                .map_err(InferenceError::from)?;
            let out2 = ((&x1 * &sin4).map_err(InferenceError::from)?
                + (&x2 * &cos4).map_err(InferenceError::from)?)
                .map_err(InferenceError::from)?;
            Tensor::cat(&[&out1, &out2], 3).map_err(InferenceError::from)
        }

        let s = q.dim(2).map_err(InferenceError::from)?;
        let cos_slice = self.cos.narrow(0, seq_pos, s).map_err(InferenceError::from)?;
        let sin_slice = self.sin.narrow(0, seq_pos, s).map_err(InferenceError::from)?;

        Ok((rotate(q, &cos_slice, &sin_slice)?, rotate(k, &cos_slice, &sin_slice)?))
    }
}

// ── Causal attention mask ─────────────────────────────────────────────────────

/// Build a causal mask of shape `[q_len, kv_len]`.
///
/// `mask[i][j] = 0.0` if key `j` is at or before query position `seq_pos + i`;
/// `mask[i][j] = -inf` otherwise (blocks future-token attention).
fn causal_mask(
    q_len: usize,
    kv_len: usize,
    seq_pos: usize,
    device: &Device,
    dtype: DType,
) -> InferenceResult<Tensor> {
    let mut data = vec![f32::NEG_INFINITY; q_len * kv_len];
    for i in 0..q_len {
        let limit = (seq_pos + i + 1).min(kv_len);
        for j in 0..limit {
            data[i * kv_len + j] = 0.0;
        }
    }
    Tensor::from_vec(data, (q_len, kv_len), device)
        .and_then(|t| t.to_dtype(dtype))
        .map_err(InferenceError::from)
}

// ── GQA key/value repeat ──────────────────────────────────────────────────────

fn repeat_kv(t: &Tensor, n_rep: usize) -> InferenceResult<Tensor> {
    if n_rep == 1 {
        return Ok(t.clone());
    }
    let (b, n_kv, s, hd) = t.dims4().map_err(InferenceError::from)?;
    t.unsqueeze(2)
        .and_then(|t| t.expand((b, n_kv, n_rep, s, hd)))
        .and_then(|t| t.reshape((b, n_kv * n_rep, s, hd)))
        .map_err(InferenceError::from)
}

// ── Single transformer block ──────────────────────────────────────────────────

pub(crate) struct ShardBlock {
    input_layernorm: candle_nn::RmsNorm,
    q_proj: candle_nn::Linear,
    k_proj: candle_nn::Linear,
    v_proj: candle_nn::Linear,
    o_proj: candle_nn::Linear,
    post_attn_layernorm: candle_nn::RmsNorm,
    gate_proj: candle_nn::Linear,
    up_proj: candle_nn::Linear,
    down_proj: candle_nn::Linear,
    cfg: ShardConfig,
}

impl ShardBlock {
    /// Load one transformer block from the VarBuilder already scoped to
    /// `model.layers.{global_idx}`.
    pub(crate) fn load(vb: VarBuilder, cfg: &ShardConfig) -> InferenceResult<Self> {
        let h = cfg.hidden_dim;
        let kv_dim = cfg.num_kv_heads * cfg.head_dim;
        let inter = cfg.intermediate_dim;
        let eps = cfg.rms_norm_eps;

        let input_layernorm =
            candle_nn::rms_norm(h, eps, vb.pp("input_layernorm"))
                .map_err(|e| InferenceError::ModelLoadError(format!("input_layernorm: {e}")))?;
        let post_attn_layernorm =
            candle_nn::rms_norm(h, eps, vb.pp("post_attention_layernorm"))
                .map_err(|e| InferenceError::ModelLoadError(format!("post_attn_layernorm: {e}")))?;

        let q_proj = candle_nn::linear_no_bias(h, h, vb.pp("self_attn.q_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("q_proj: {e}")))?;
        let k_proj = candle_nn::linear_no_bias(h, kv_dim, vb.pp("self_attn.k_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("k_proj: {e}")))?;
        let v_proj = candle_nn::linear_no_bias(h, kv_dim, vb.pp("self_attn.v_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("v_proj: {e}")))?;
        let o_proj = candle_nn::linear_no_bias(h, h, vb.pp("self_attn.o_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("o_proj: {e}")))?;

        let gate_proj = candle_nn::linear_no_bias(h, inter, vb.pp("mlp.gate_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("gate_proj: {e}")))?;
        let up_proj = candle_nn::linear_no_bias(h, inter, vb.pp("mlp.up_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("up_proj: {e}")))?;
        let down_proj = candle_nn::linear_no_bias(inter, h, vb.pp("mlp.down_proj"))
            .map_err(|e| InferenceError::ModelLoadError(format!("down_proj: {e}")))?;

        Ok(Self {
            input_layernorm,
            q_proj, k_proj, v_proj, o_proj,
            post_attn_layernorm,
            gate_proj, up_proj, down_proj,
            cfg: cfg.clone(),
        })
    }

    /// Forward pass through one transformer block.
    ///
    /// * `x`       — `[1, seq_len, hidden_dim]`
    /// * `seq_pos` — global sequence position of the first token in `x`
    /// * `kv`      — mutable per-block KV-cache (initialised to `None` for a new session)
    /// * `rope`    — shared precomputed RoPE tables
    pub(crate) fn forward(
        &self,
        x: &Tensor,
        seq_pos: usize,
        kv: &mut Option<(Tensor, Tensor)>,
        rope: &RopeCache,
    ) -> InferenceResult<Tensor> {
        let (b, s, _h) = x.dims3().map_err(InferenceError::from)?;
        let n_h = self.cfg.num_heads;
        let n_kv = self.cfg.num_kv_heads;
        let hd = self.cfg.head_dim;
        let device = x.device();

        // ── Self-attention ─────────────────────────────────────────────────────
        let residual = x.clone();
        let normed = self.input_layernorm.forward(x).map_err(InferenceError::from)?;

        // Project to Q, K, V
        let q = self.q_proj.forward(&normed).map_err(InferenceError::from)?; // [b, s, h]
        let k = self.k_proj.forward(&normed).map_err(InferenceError::from)?; // [b, s, kv_dim]
        let v = self.v_proj.forward(&normed).map_err(InferenceError::from)?;

        // Reshape to multi-head: [b, n_heads, s, head_dim]
        let q = q.reshape((b, s, n_h, hd)).map_err(InferenceError::from)?
            .transpose(1, 2).map_err(InferenceError::from)?;
        let k = k.reshape((b, s, n_kv, hd)).map_err(InferenceError::from)?
            .transpose(1, 2).map_err(InferenceError::from)?;
        let v = v.reshape((b, s, n_kv, hd)).map_err(InferenceError::from)?
            .transpose(1, 2).map_err(InferenceError::from)?;

        // Apply rotary position embeddings
        let (q, k) = rope.apply(&q, &k, seq_pos)?;

        // Append to KV-cache and update it
        let (k, v) = if let Some((ck, cv)) = kv.take() {
            let k = Tensor::cat(&[&ck, &k], 2).map_err(InferenceError::from)?;
            let v = Tensor::cat(&[&cv, &v], 2).map_err(InferenceError::from)?;
            (k, v)
        } else {
            (k, v)
        };
        *kv = Some((k.clone(), v.clone()));

        let kv_seq = k.dim(2).map_err(InferenceError::from)?; // total keys

        // GQA: repeat K and V to match query heads
        let k_full = repeat_kv(&k, self.cfg.n_rep())?;
        let v_full = repeat_kv(&v, self.cfg.n_rep())?;

        // Scaled dot-product attention
        let scale = (hd as f64).sqrt().recip();
        let scores = q.matmul(&k_full.transpose(2, 3).map_err(InferenceError::from)?)
            .map_err(InferenceError::from)?; // [b, n_h, s, kv_seq]
        let scores = (scores * scale).map_err(InferenceError::from)?;

        // Causal mask (only needed when new tokens > 1, i.e. prefill)
        let scores = if s > 1 {
            let mask = causal_mask(s, kv_seq, seq_pos, device, self.cfg.dtype)?;
            // Broadcast mask from [s, kv_seq] to [1, 1, s, kv_seq]
            let mask = mask.unsqueeze(0).and_then(|t| t.unsqueeze(0))
                .map_err(InferenceError::from)?;
            (scores + mask).map_err(InferenceError::from)?
        } else {
            scores
        };

        let attn_w = candle_nn::ops::softmax(&scores, candle_core::D::Minus1)
            .map_err(InferenceError::from)?;
        let attn_out = attn_w.matmul(&v_full).map_err(InferenceError::from)?; // [b, n_h, s, hd]

        // Merge heads: [b, s, h]
        let h = self.cfg.hidden_dim;
        let attn_out = attn_out.transpose(1, 2).map_err(InferenceError::from)?
            .reshape((b, s, h)).map_err(InferenceError::from)?;
        let attn_out = self.o_proj.forward(&attn_out).map_err(InferenceError::from)?;
        let x = (&residual + &attn_out).map_err(InferenceError::from)?;

        // ── SwiGLU MLP ────────────────────────────────────────────────────────
        let residual = x.clone();
        let normed = self.post_attn_layernorm.forward(&x).map_err(InferenceError::from)?;

        let gate = self.gate_proj.forward(&normed).map_err(InferenceError::from)?;
        let up   = self.up_proj.forward(&normed).map_err(InferenceError::from)?;
        // SwiGLU: silu(gate) * up
        let gate = candle_nn::ops::silu(&gate).map_err(InferenceError::from)?;
        let ff = (gate * up).map_err(InferenceError::from)?;
        let ff = self.down_proj.forward(&ff).map_err(InferenceError::from)?;

        (&residual + &ff).map_err(InferenceError::from)
    }
}

// ── Session KV-cache ──────────────────────────────────────────────────────────

/// KV-cache for one inference session across all blocks in this shard.
struct Session {
    /// One `Option<(k_cache, v_cache)>` per block in the shard.
    kv: Vec<Option<(Tensor, Tensor)>>,
    last_access: Instant,
}

impl Session {
    fn new(num_blocks: usize) -> Self {
        Self { kv: vec![None; num_blocks], last_access: Instant::now() }
    }
}

// ── TransformerShard ──────────────────────────────────────────────────────────

/// A partial transformer model that serves blocks `[start_block..end_block)`.
///
/// Load with [`TransformerShard::load`] then call the appropriate forward method
/// based on this node's position in the inference chain.
pub struct TransformerShard {
    embedding: Option<candle_nn::Embedding>,   // first node only (start_block == 0)
    blocks: Vec<ShardBlock>,                   // blocks [start_block..end_block)
    norm: Option<candle_nn::RmsNorm>,          // last node only
    lm_head: Option<candle_nn::Linear>,        // last node only
    rope: RopeCache,
    /// Tokenizer — all nodes load it; only the coordinator uses it actively.
    pub tokenizer: BpeTokenizer,
    pub start_block: usize,
    pub end_block: usize,
    pub cfg: ShardConfig,
    sessions: Mutex<HashMap<u64, Session>>,
}

impl TransformerShard {
    /// Load a shard from SafeTensors files.
    ///
    /// Components are loaded selectively:
    /// - Embedding for first node (`start_block == 0`)
    /// - Transformer blocks `[start_block..end_block)`
    /// - Final RMSNorm + LM head for last node (`end_block == num_total_blocks`)
    pub fn load(
        safetensors_paths: &[&Path],
        config_path: &Path,
        device: &Device,
        start_block: usize,
        end_block: usize,
    ) -> InferenceResult<Self> {
        use candle_transformers::models::llama::LlamaConfig;

        // Parse HuggingFace config.json
        let config_str = std::fs::read_to_string(config_path).map_err(|e| {
            InferenceError::ModelLoadError(format!("Cannot read {}: {e}", config_path.display()))
        })?;
        let hf_config: LlamaConfig = serde_json::from_str(&config_str).map_err(|e| {
            InferenceError::ModelLoadError(format!("Cannot parse config.json: {e}"))
        })?;

        let num_total_blocks = hf_config.num_hidden_layers;
        if start_block >= end_block || end_block > num_total_blocks {
            return Err(InferenceError::ModelLoadError(format!(
                "Invalid shard range [{start_block}..{end_block}) for \
                 model with {num_total_blocks} layers"
            )));
        }

        let hidden_dim       = hf_config.hidden_size;
        let num_heads        = hf_config.num_attention_heads;
        let num_kv_heads     = hf_config.num_key_value_heads();
        let head_dim         = hidden_dim / num_heads;
        let intermediate_dim = hf_config.intermediate_size;
        let vocab_size       = hf_config.vocab_size;
        let rope_theta       = hf_config.rope_theta as f64;
        let max_seq_len      = hf_config.max_position_embeddings;
        let rms_norm_eps     = hf_config.rms_norm_eps;

        let cfg = ShardConfig {
            num_total_blocks,
            hidden_dim,
            num_heads,
            num_kv_heads,
            head_dim,
            intermediate_dim,
            vocab_size,
            rope_theta,
            max_seq_len,
            rms_norm_eps,
            dtype: DType::F16,
        };

        info!(
            "Loading shard [{start_block}..{end_block}) of {num_total_blocks}: \
             hidden={hidden_dim} heads={num_heads} ({num_kv_heads} kv)"
        );

        // Memory-map all safetensors shards (only accessed pages are read)
        // SAFETY: files must not be modified while the model is loaded.
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(safetensors_paths, DType::F16, device)
                .map_err(|e| InferenceError::ModelLoadError(format!("mmap safetensors: {e}")))?
        };

        // Embedding: only for the first node in the chain
        let embedding = if start_block == 0 {
            info!("  Loading embedding (vocab={vocab_size}, dim={hidden_dim})");
            Some(
                candle_nn::embedding(vocab_size, hidden_dim, vb.pp("model.embed_tokens"))
                    .map_err(|e| InferenceError::ModelLoadError(format!("embedding: {e}")))?,
            )
        } else {
            None
        };

        // Transformer blocks
        let shard_size = end_block - start_block;
        let mut blocks = Vec::with_capacity(shard_size);
        for global_idx in start_block..end_block {
            info!("  Loading block {global_idx}");
            let block_vb = vb.pp(format!("model.layers.{global_idx}"));
            blocks.push(ShardBlock::load(block_vb, &cfg)?);
        }

        // Final norm + LM head: only for the last node in the chain
        let (norm, lm_head) = if end_block == num_total_blocks {
            info!("  Loading norm + lm_head");
            let n = candle_nn::rms_norm(hidden_dim, rms_norm_eps, vb.pp("model.norm"))
                .map_err(|e| InferenceError::ModelLoadError(format!("final norm: {e}")))?;
            let lh = candle_nn::linear_no_bias(hidden_dim, vocab_size, vb.pp("lm_head"))
                .map_err(|e| InferenceError::ModelLoadError(format!("lm_head: {e}")))?;
            (Some(n), Some(lh))
        } else {
            (None, None)
        };

        // Tokenizer
        let tokenizer_path = config_path
            .parent()
            .unwrap_or(Path::new("."))
            .join("tokenizer.json");
        let tokenizer = BpeTokenizer::from_file(&tokenizer_path)?;

        // Precompute RoPE tables
        let rope = RopeCache::new(&cfg, device)?;

        info!(
            "Shard [{start_block}..{end_block}) ready — \
             embedding={} final_head={}",
            embedding.is_some(),
            norm.is_some(),
        );

        Ok(Self {
            embedding,
            blocks,
            norm,
            lm_head,
            rope,
            tokenizer,
            start_block,
            end_block,
            cfg,
            sessions: Mutex::new(HashMap::new()),
        })
    }

    /// Returns `true` if this is the first node in the chain (holds the embedding).
    pub fn is_first(&self) -> bool {
        self.start_block == 0
    }

    /// Returns `true` if this is the last node in the chain (holds the LM head).
    pub fn is_last(&self) -> bool {
        self.end_block == self.cfg.num_total_blocks
    }

    // ── Session management ────────────────────────────────────────────────────

    /// Open a new inference session (empty KV-cache for each block in this shard).
    pub fn open_session(&self, session_id: u64) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id, Session::new(self.blocks.len()));
        debug!("Opened session {session_id}");
    }

    /// Drop a session and free its KV-cache tensors.
    pub fn close_session(&self, session_id: u64) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(&session_id);
        debug!("Closed session {session_id}");
    }

    /// Evict sessions that have been idle for more than 60 seconds.
    /// Call this periodically (e.g., every 30 s) from a background task.
    pub fn gc_sessions(&self) {
        let mut sessions = self.sessions.lock().unwrap();
        let before = sessions.len();
        sessions.retain(|_, s| s.last_access.elapsed().as_secs() < 60);
        let evicted = before - sessions.len();
        if evicted > 0 {
            info!("GC evicted {evicted} stale sessions ({} remaining)", sessions.len());
        }
    }

    // ── Core block execution ──────────────────────────────────────────────────

    /// Run the hidden state through all blocks in this shard, updating the KV-cache.
    fn run_blocks(
        &self,
        mut x: Tensor,
        seq_pos: usize,
        session_id: u64,
    ) -> InferenceResult<Tensor> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions
            .entry(session_id)
            .or_insert_with(|| Session::new(self.blocks.len()));
        session.last_access = Instant::now();

        for (local_idx, block) in self.blocks.iter().enumerate() {
            x = block.forward(&x, seq_pos, &mut session.kv[local_idx], &self.rope)?;
        }
        Ok(x)
    }

    // ── Public forward entry points ───────────────────────────────────────────

    /// **First node**: embed token IDs, run blocks, return hidden states.
    ///
    /// * `session_id` — unique inference session (KV-cache key)
    /// * `token_ids`  — token IDs for the current step (full prompt on prefill, one token on decode)
    /// * `seq_pos`    — global sequence position of the first token (`0` for prefill)
    ///
    /// Returns `[1, seq_len, hidden_dim]`.
    pub fn forward_first(
        &self,
        session_id: u64,
        token_ids: &[u32],
        seq_pos: usize,
    ) -> InferenceResult<Tensor> {
        let emb = self.embedding.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_first() called on a shard that does not hold the embedding \
                 (start_block != 0)"
                    .to_string(),
            )
        })?;

        // Build [1, seq_len] token tensor
        let tok = Tensor::new(token_ids, emb.embeddings().device())
            .and_then(|t| t.unsqueeze(0))
            .map_err(InferenceError::from)?;

        // Embed → [1, seq_len, hidden_dim]
        let hidden = emb.forward(&tok).map_err(InferenceError::from)?;

        self.run_blocks(hidden, seq_pos, session_id)
    }

    /// **Middle node**: receive hidden states, run blocks, return hidden states.
    ///
    /// * `hidden` — `[1, seq_len, hidden_dim]`
    ///
    /// Returns `[1, seq_len, hidden_dim]`.
    pub fn forward_middle(
        &self,
        session_id: u64,
        hidden: Tensor,
        seq_pos: usize,
    ) -> InferenceResult<Tensor> {
        self.run_blocks(hidden, seq_pos, session_id)
    }

    /// **Single-node** (first AND last): embed token IDs, run all blocks, return logits.
    ///
    /// Convenience method for when one node serves the entire model.
    /// Returns `[1, 1, vocab_size]`.
    pub fn forward_full(
        &self,
        session_id: u64,
        token_ids: &[u32],
        seq_pos: usize,
    ) -> InferenceResult<Tensor> {
        let emb = self.embedding.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_full() called on a shard without embedding".to_string(),
            )
        })?;
        let norm = self.norm.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_full() called on a shard without final norm".to_string(),
            )
        })?;
        let lm_head = self.lm_head.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_full() called on a shard without lm_head".to_string(),
            )
        })?;

        let tok = Tensor::new(token_ids, emb.embeddings().device())
            .and_then(|t| t.unsqueeze(0))
            .map_err(InferenceError::from)?;
        let hidden = emb.forward(&tok).map_err(InferenceError::from)?;
        let x = self.run_blocks(hidden, seq_pos, session_id)?;
        let seq_len = x.dim(1).map_err(InferenceError::from)?;
        let x_last = x.narrow(1, seq_len - 1, 1).map_err(InferenceError::from)?;
        let x_last = norm.forward(&x_last).map_err(InferenceError::from)?;
        lm_head.forward(&x_last).map_err(InferenceError::from)
    }

    /// **Last node**: receive hidden states, run blocks, apply norm + LM head, return logits.
    ///
    /// Returns `[1, 1, vocab_size]` (last-token logits, ready for sampling).
    pub fn forward_last(
        &self,
        session_id: u64,
        hidden: Tensor,
        seq_pos: usize,
    ) -> InferenceResult<Tensor> {
        let norm = self.norm.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_last() called on a shard that is not the last node".to_string(),
            )
        })?;
        let lm_head = self.lm_head.as_ref().ok_or_else(|| {
            InferenceError::InferenceFailed(
                "forward_last() called but lm_head is missing".to_string(),
            )
        })?;

        let x = self.run_blocks(hidden, seq_pos, session_id)?; // [1, seq_len, h]
        let seq_len = x.dim(1).map_err(InferenceError::from)?;

        // Keep only the last-token hidden state for efficient logit computation
        let x_last = x.narrow(1, seq_len - 1, 1).map_err(InferenceError::from)?; // [1, 1, h]
        let x_last = norm.forward(&x_last).map_err(InferenceError::from)?;
        let logits = lm_head.forward(&x_last).map_err(InferenceError::from)?; // [1, 1, vocab]

        Ok(logits)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rope_cache_shape() {
        let cfg = ShardConfig {
            num_total_blocks: 8,
            hidden_dim: 64,
            num_heads: 4,
            num_kv_heads: 4,
            head_dim: 16,
            intermediate_dim: 128,
            vocab_size: 256,
            rope_theta: 10000.0,
            max_seq_len: 64,
            rms_norm_eps: 1e-5,
            dtype: DType::F32,
        };
        let device = Device::Cpu;
        let rope = RopeCache::new(&cfg, &device).unwrap();
        assert_eq!(rope.cos.dims(), &[64, 8]); // [max_seq, head_dim/2]
        assert_eq!(rope.sin.dims(), &[64, 8]);
    }

    #[test]
    fn causal_mask_shape() {
        let mask = causal_mask(3, 5, 2, &Device::Cpu, DType::F32).unwrap();
        assert_eq!(mask.dims(), &[3, 5]);
        let data = mask.to_vec2::<f32>().unwrap();
        // Position 0 (global 2): can see keys 0,1,2  → j<=2
        assert_eq!(data[0][0], 0.0);
        assert_eq!(data[0][2], 0.0);
        assert!(data[0][3].is_infinite() && data[0][3] < 0.0);
    }

    #[test]
    fn repeat_kv_expands_correctly() {
        let device = Device::Cpu;
        // [1, 2, 3, 4] with n_rep=2 → [1, 4, 3, 4]
        let t = Tensor::zeros((1usize, 2usize, 3usize, 4usize), DType::F32, &device).unwrap();
        let out = repeat_kv(&t, 2).unwrap();
        assert_eq!(out.dims(), &[1, 4, 3, 4]);
    }
}
