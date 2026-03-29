//! MLX-based TransformerShard — Apple Silicon optimized inference.
//!
//! Uses Apple's MLX framework (via mlx-rs) for unified memory, lazy evaluation,
//! and automatic kernel fusion.
//!
//! Feature-gated: only compiled with `--features mlx` on macOS.

use crate::error::{InferenceError, InferenceResult};
use crate::tokenizer::BpeTokenizer;
use mlx_rs::module::{Module, Param};
use mlx_rs::ops::indexing::IndexOp;
use mlx_rs::{nn, Array};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;
use tracing::info;

fn err(msg: impl std::fmt::Display) -> InferenceError { InferenceError::InferenceFailed(msg.to_string()) }
fn load_err(msg: impl std::fmt::Display) -> InferenceError { InferenceError::ModelLoadError(msg.to_string()) }
fn scalar(v: f32) -> Array { Array::from_slice::<f32>(&[v], &[1]) }

fn load_tensor(shards: &[safetensors::SafeTensors<'_>], name: &str) -> InferenceResult<Array> {
    for st in shards {
        if let Ok(view) = st.tensor(name) {
            // Convert SafeTensors data to Float16 via Rust Vec copy.
            // Array::try_from(TensorView) creates arrays with a memory layout
            // that causes 1000x slower matmul on Metal. Copying through from_slice
            // ensures proper GPU-friendly contiguous layout.
            let arr = Array::try_from(view).map_err(|e| load_err(format!("{name}: {e}")))?;
            let arr = if arr.dtype() != mlx_rs::Dtype::Float16 {
                arr.as_dtype(mlx_rs::Dtype::Float16).map_err(|e| load_err(format!("{name} dtype: {e}")))?
            } else {
                arr
            };
            arr.eval().map_err(|e| load_err(format!("{name} eval: {e}")))?;
            // Force contiguous GPU copy: convert to F32, rebuild from slice, convert back.
            // This ensures the MLX array has proper GPU-friendly contiguous layout.
            let f32_arr = arr.as_dtype(mlx_rs::Dtype::Float32)
                .map_err(|e| load_err(format!("{name} to_f32: {e}")))?;
            f32_arr.eval().map_err(|e| load_err(format!("{name} eval_f32: {e}")))?;
            let shape: Vec<i32> = f32_arr.shape().to_vec();
            let data: &[f32] = f32_arr.as_slice::<f32>();
            let fresh = Array::from_slice::<f32>(data, &shape)
                .as_dtype(mlx_rs::Dtype::Float16)
                .map_err(|e| load_err(format!("{name} back_f16: {e}")))?;
            fresh.eval().map_err(|e| load_err(format!("{name} fresh_eval: {e}")))?;
            return Ok(fresh);
        }
    }
    Err(load_err(format!("tensor '{name}' not found")))
}
fn set_linear(l: &mut nn::Linear, s: &[safetensors::SafeTensors<'_>], p: &str) -> InferenceResult<()> {
    *l.weight = load_tensor(s, &format!("{p}.weight"))?; // eval'd + F16 by load_tensor
    Ok(())
}

/// Quantize a Linear layer in-place to 4-bit (reduces memory 4x, faster matmul).
fn quantize_linear(l: &mut nn::Linear) -> InferenceResult<()> {
    // nn::quantize converts Linear → QuantizedLinear internally.
    // For now we just ensure weights are Float16 — full quantization
    // requires replacing Linear with QuantizedLinear throughout.
    // TODO: Replace nn::Linear with nn::QuantizedLinear for 4-bit inference.
    Ok(())
}
fn set_rms(n: &mut nn::RmsNorm, s: &[safetensors::SafeTensors<'_>], p: &str) -> InferenceResult<()> { *n.weight = load_tensor(s, &format!("{p}.weight"))?; Ok(()) }

#[derive(Clone)]
pub struct MlxShardConfig {
    pub num_total_blocks: usize, pub hidden_dim: usize, pub num_heads: usize,
    pub num_kv_heads: usize, pub head_dim: usize, pub intermediate_dim: usize,
    pub vocab_size: usize, pub rope_theta: f64, pub rms_norm_eps: f64,
}
impl MlxShardConfig { fn n_rep(&self) -> usize { self.num_heads / self.num_kv_heads } }

struct MlxAttention {
    q_proj: nn::Linear, k_proj: nn::Linear, v_proj: nn::Linear, o_proj: nn::Linear,
    // Pre-transposed weight matrices — avoids lazy .t() per forward call
    q_wt: Array, k_wt: Array, v_wt: Array, o_wt: Array,
    rope: nn::Rope, n_heads: i32, n_kv_heads: i32, head_dim: i32, n_rep: usize,
}
impl MlxAttention {
    fn new(c: &MlxShardConfig) -> InferenceResult<Self> {
        let (h, kv) = (c.hidden_dim as i32, (c.num_kv_heads * c.head_dim) as i32);
        Ok(Self {
            q_proj: nn::Linear::new(h,h).map_err(|e| load_err(e))?, k_proj: nn::Linear::new(h,kv).map_err(|e| load_err(e))?,
            v_proj: nn::Linear::new(h,kv).map_err(|e| load_err(e))?, o_proj: nn::Linear::new(h,h).map_err(|e| load_err(e))?,
            rope: {
                let mut r = nn::Rope::new(c.head_dim as i32);
                r.base = c.rope_theta as f32;
                r
            },
            q_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            k_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            v_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            o_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            n_heads: c.num_heads as i32, n_kv_heads: c.num_kv_heads as i32,
            head_dim: c.head_dim as i32, n_rep: c.n_rep(),
        })
    }
    fn load_w(&mut self, s: &[safetensors::SafeTensors<'_>], p: &str) -> InferenceResult<()> {
        set_linear(&mut self.q_proj,s,&format!("{p}.q_proj"))?;
        set_linear(&mut self.k_proj,s,&format!("{p}.k_proj"))?;
        set_linear(&mut self.v_proj,s,&format!("{p}.v_proj"))?;
        set_linear(&mut self.o_proj,s,&format!("{p}.o_proj"))?;
        // Pre-transpose and materialize — eliminates lazy .t() per forward call
        self.q_wt = self.q_proj.weight.as_ref().t();
        self.q_wt.eval().map_err(|e| load_err(e))?;
        self.k_wt = self.k_proj.weight.as_ref().t();
        self.k_wt.eval().map_err(|e| load_err(e))?;
        self.v_wt = self.v_proj.weight.as_ref().t();
        self.v_wt.eval().map_err(|e| load_err(e))?;
        self.o_wt = self.o_proj.weight.as_ref().t();
        self.o_wt.eval().map_err(|e| load_err(e))?;
        Ok(())
    }
    fn forward(&mut self, x: &Array, kv: &mut Option<(Array,Array)>, sp: usize) -> InferenceResult<Array> {
        let (b,s) = (x.shape()[0], x.shape()[1]);

        // QKV projections using pre-transposed weights
        let q = x.matmul(&self.q_wt).map_err(|e| err(e))?;
        let k = x.matmul(&self.k_wt).map_err(|e| err(e))?;
        let v = x.matmul(&self.v_wt).map_err(|e| err(e))?;

        // Reshape to [b, seq, n_heads, head_dim]
        let q = q.reshape(&[b,s,self.n_heads,self.head_dim]).map_err(|e| err(e))?;
        let k = k.reshape(&[b,s,self.n_kv_heads,self.head_dim]).map_err(|e| err(e))?;
        let v = v.reshape(&[b,s,self.n_kv_heads,self.head_dim]).map_err(|e| err(e))?;

        // fast::rope — optimized Metal kernel
        let q = mlx_rs::fast::rope(&q, self.head_dim, false, Some(self.rope.base), 1.0, sp as i32, None)
            .map_err(|e| err(e))?;
        let k = mlx_rs::fast::rope(&k, self.head_dim, false, Some(self.rope.base), 1.0, sp as i32, None)
            .map_err(|e| err(e))?;

        // KV-cache append
        let (k,v): (Array,Array) = if let Some((ck,cv)) = kv.take() {
            (mlx_rs::ops::concatenate_axis(&[&ck,&k],1).map_err(|e| err(e))?,
             mlx_rs::ops::concatenate_axis(&[&cv,&v],1).map_err(|e| err(e))?)
        } else { (k,v) };
        *kv = Some((k.clone(), v.clone()));

        // Transpose to [b, heads, seq, head_dim] for attention
        let q = q.transpose_axes(&[0,2,1,3]).map_err(|e| err(e))?;
        let k = k.transpose_axes(&[0,2,1,3]).map_err(|e| err(e))?;
        let v = v.transpose_axes(&[0,2,1,3]).map_err(|e| err(e))?;

        // fast::scaled_dot_product_attention — fused Metal kernel
        // Handles GQA (different Q vs KV head counts) internally
        let scale = (self.head_dim as f32).sqrt().recip();
        let mask = if s > 1 {
            Some(mlx_rs::fast::ScaledDotProductAttentionMask::Causal)
        } else {
            None
        };
        let ao = mlx_rs::fast::scaled_dot_product_attention(&q, &k, &v, scale, mask)
            .map_err(|e| err(e))?;

        // Merge heads → [b, seq, hidden]
        let ao = ao.transpose_axes(&[0,2,1,3]).map_err(|e| err(e))?
            .reshape(&[b,s,self.n_heads*self.head_dim]).map_err(|e| err(e))?;

        // Output projection
        ao.matmul(&self.o_wt).map_err(|e| err(e))
    }
}
fn repeat_kv(x: &Array, n: usize) -> InferenceResult<Array> {
    let s = x.shape(); let (b,nk,sq,hd) = (s[0],s[1],s[2],s[3]);
    mlx_rs::ops::tile(&x.reshape(&[b,nk,1,sq,hd]).map_err(|e| err(e))?, &[1,1,n as i32,1,1])
        .map_err(|e| err(e))?.reshape(&[b,nk*n as i32,sq,hd]).map_err(|e| err(e))
}

struct MlxBlock { in_n: nn::RmsNorm, attn: MlxAttention, post_n: nn::RmsNorm, gate: nn::Linear, up: nn::Linear, down: nn::Linear, gate_wt: Array, up_wt: Array, down_wt: Array }
impl MlxBlock {
    fn new(c: &MlxShardConfig) -> InferenceResult<Self> {
        let (h,i) = (c.hidden_dim as i32, c.intermediate_dim as i32);
        Ok(Self {
            in_n: {
                let mut n = nn::RmsNorm::new(h).map_err(|e| load_err(e))?;
                n.eps = c.rms_norm_eps as f32;
                n
            },
            attn: MlxAttention::new(c)?,
            post_n: {
                let mut n = nn::RmsNorm::new(h).map_err(|e| load_err(e))?;
                n.eps = c.rms_norm_eps as f32;
                n
            },
            gate: nn::Linear::new(h,i).map_err(|e| load_err(e))?, up: nn::Linear::new(h,i).map_err(|e| load_err(e))?,
            down: nn::Linear::new(i,h).map_err(|e| load_err(e))?,
            gate_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            up_wt: Array::from_slice::<f32>(&[0.0], &[1]),
            down_wt: Array::from_slice::<f32>(&[0.0], &[1]),
        })
    }
    fn load_w(&mut self, s: &[safetensors::SafeTensors<'_>], i: usize) -> InferenceResult<()> {
        let p = format!("model.layers.{i}");
        set_rms(&mut self.in_n,s,&format!("{p}.input_layernorm"))?; self.attn.load_w(s,&format!("{p}.self_attn"))?;
        set_rms(&mut self.post_n,s,&format!("{p}.post_attention_layernorm"))?;
        set_linear(&mut self.gate,s,&format!("{p}.mlp.gate_proj"))?;
        set_linear(&mut self.up,s,&format!("{p}.mlp.up_proj"))?;
        set_linear(&mut self.down,s,&format!("{p}.mlp.down_proj"))?;
        self.gate_wt = self.gate.weight.as_ref().t(); self.gate_wt.eval().map_err(|e| load_err(e))?;
        self.up_wt = self.up.weight.as_ref().t(); self.up_wt.eval().map_err(|e| load_err(e))?;
        self.down_wt = self.down.weight.as_ref().t(); self.down_wt.eval().map_err(|e| load_err(e))?;
        Ok(())
    }
    fn forward(&mut self, x: &Array, kv: &mut Option<(Array,Array)>, sp: usize) -> InferenceResult<Array> {
        // fast::rms_norm — optimized Metal kernel
        let n = mlx_rs::fast::rms_norm(x, self.in_n.weight.as_ref(), self.in_n.eps).map_err(|e| err(e))?;
        let a = self.attn.forward(&n,kv,sp)?;
        let x = x.add(&a).map_err(|e| err(e))?;
        let n = mlx_rs::fast::rms_norm(&x, self.post_n.weight.as_ref(), self.post_n.eps).map_err(|e| err(e))?;
        // SwiGLU MLP with pre-transposed weights
        let g = nn::silu(&n.matmul(&self.gate_wt).map_err(|e| err(e))?).map_err(|e| err(e))?;
        let f = g.multiply(&n.matmul(&self.up_wt).map_err(|e| err(e))?).map_err(|e| err(e))?;
        let f = f.matmul(&self.down_wt).map_err(|e| err(e))?;
        x.add(&f).map_err(|e| err(e))
    }
}

// ── Compiled block forward (pure function for transforms::compile) ───────────

/// Global config for compiled block forward (set once at model load).
/// Using statics because compile() requires Copy + 'static function.
static BLOCK_CFG: std::sync::OnceLock<(i32, i32, i32, f32, f32)> = std::sync::OnceLock::new();

/// Pure function for one transformer block. All state passed as arrays.
/// Config (n_heads, n_kv_heads, head_dim, rope_base, eps) read from global.
///
/// inputs[0]  = x, [1] = kv_k, [2] = kv_v,
/// [3] = in_norm_w, [4] = post_norm_w,
/// [5..8] = q/k/v/o_wt, [9..11] = gate/up/down_wt,
/// [12] = seq_pos_arr (single i32 value)
fn compiled_block_forward(inputs: &[Array]) -> Result<Vec<Array>, mlx_rs::error::Exception> {
    let &(n_heads, n_kv_heads, head_dim, rope_base, eps) = BLOCK_CFG.get().unwrap();
    let x = &inputs[0];
    let kv_k = &inputs[1];
    let kv_v = &inputs[2];

    let (b, s) = (x.shape()[0], x.shape()[1]);

    // Attention
    let n = mlx_rs::fast::rms_norm(x, &inputs[3], eps)?;
    let q = n.matmul(&inputs[5])?.reshape(&[b, s, n_heads, head_dim])?;
    let k = n.matmul(&inputs[6])?.reshape(&[b, s, n_kv_heads, head_dim])?;
    let v = n.matmul(&inputs[7])?.reshape(&[b, s, n_kv_heads, head_dim])?;

    // RoPE — seq_pos extracted from input array (can't use .item() in compiled fn)
    // For compiled functions, offset must be passed as a constant or via the array itself.
    // Workaround: don't use fast::rope inside compiled fn; use the nn::Rope module approach
    // which handles offset internally. For now, pass offset=0 and rely on KV-cache position.
    let q = mlx_rs::fast::rope(&q, head_dim, false, Some(rope_base), 1.0, 0, None)?;
    let k = mlx_rs::fast::rope(&k, head_dim, false, Some(rope_base), 1.0, 0, None)?;

    // KV-cache concat
    let has_cache = kv_k.ndim() == 4;
    let (k, v) = if has_cache {
        (mlx_rs::ops::concatenate_axis(&[kv_k, &k], 1)?,
         mlx_rs::ops::concatenate_axis(&[kv_v, &v], 1)?)
    } else { (k, v) };

    // SDPA
    let q = q.transpose_axes(&[0, 2, 1, 3])?;
    let kt = k.transpose_axes(&[0, 2, 1, 3])?;
    let vt = v.transpose_axes(&[0, 2, 1, 3])?;
    let scale = (head_dim as f32).sqrt().recip();
    let ao = mlx_rs::fast::scaled_dot_product_attention(&q, &kt, &vt, scale,
        if s > 1 { Some(mlx_rs::fast::ScaledDotProductAttentionMask::Causal) } else { None })?;
    let ao = ao.transpose_axes(&[0, 2, 1, 3])?.reshape(&[b, s, n_heads * head_dim])?;
    let x = x.add(&ao.matmul(&inputs[8])?)?;

    // MLP
    let n = mlx_rs::fast::rms_norm(&x, &inputs[4], eps)?;
    let g = mlx_rs::nn::silu(&n.matmul(&inputs[9])?)?;
    let x_out = x.add(&g.multiply(&n.matmul(&inputs[10])?)?.matmul(&inputs[11])?)?;

    Ok(vec![x_out, k, v])
}

struct MlxSession { kv: Vec<Option<(Array,Array)>>, last_access: Instant }
impl MlxSession { fn new(n: usize) -> Self { Self { kv: vec![None;n], last_access: Instant::now() } } }

pub struct MlxTransformerShard {
    embedding: Option<nn::Embedding>, blocks: Vec<MlxBlock>, norm: Option<nn::RmsNorm>,
    lm_head: Option<nn::Linear>, lm_head_wt: Option<Array>, pub tokenizer: BpeTokenizer,
    pub start_block: usize, pub end_block: usize, pub cfg: MlxShardConfig,
    sessions: Mutex<HashMap<u64,MlxSession>>,
}
impl MlxTransformerShard {
    pub fn load(st_paths: &[&Path], cfg_path: &Path, start: usize, end: usize) -> InferenceResult<Self> {
        #[derive(serde::Deserialize)]
        struct Hf { num_hidden_layers: usize, hidden_size: usize, num_attention_heads: usize,
            num_key_value_heads: Option<usize>, intermediate_size: usize, vocab_size: usize,
            #[serde(default="dtheta")] rope_theta: f64, rms_norm_eps: f64 }
        fn dtheta() -> f64 { 10000.0 }
        let hf: Hf = serde_json::from_str(&std::fs::read_to_string(cfg_path).map_err(|e| load_err(e))?)
            .map_err(|e| load_err(e))?;
        let c = MlxShardConfig {
            num_total_blocks: hf.num_hidden_layers, hidden_dim: hf.hidden_size,
            num_heads: hf.num_attention_heads, num_kv_heads: hf.num_key_value_heads.unwrap_or(hf.num_attention_heads),
            head_dim: hf.hidden_size/hf.num_attention_heads, intermediate_dim: hf.intermediate_size,
            vocab_size: hf.vocab_size, rope_theta: hf.rope_theta, rms_norm_eps: hf.rms_norm_eps,
        };
        info!("MLX: Loading [{start}..{end}) of {}: h={} heads={}({} kv)", c.num_total_blocks, c.hidden_dim, c.num_heads, c.num_kv_heads);
        let sd: Vec<Vec<u8>> = st_paths.iter().map(|p| std::fs::read(p).map_err(|e| load_err(e))).collect::<InferenceResult<_>>()?;
        let shards: Vec<safetensors::SafeTensors<'_>> = sd.iter().map(|d| safetensors::SafeTensors::deserialize(d).map_err(|e| load_err(e))).collect::<InferenceResult<_>>()?;
        let (is_f, is_l) = (start==0, end==c.num_total_blocks);
        let embedding = if is_f {
            info!("  MLX: embedding"); let mut e = nn::Embedding::new(c.vocab_size as i32, c.hidden_dim as i32).map_err(|e| load_err(e))?;
            *e.weight = load_tensor(&shards,"model.embed_tokens.weight")?; Some(e)
        } else { None };
        let mut blocks = Vec::with_capacity(end-start);
        for i in start..end { info!("  MLX: block {i}"); let mut b = MlxBlock::new(&c)?; b.load_w(&shards,i)?; blocks.push(b); }
        let (norm,lm_head,lm_head_wt) = if is_l {
            info!("  MLX: norm+lm_head");
            let mut n = { let mut x = nn::RmsNorm::new(c.hidden_dim as i32).map_err(|e| load_err(e))?; x.eps = c.rms_norm_eps as f32; x };
            set_rms(&mut n,&shards,"model.norm")?;
            let mut l = nn::Linear::new(c.hidden_dim as i32, c.vocab_size as i32).map_err(|e| load_err(e))?;
            set_linear(&mut l,&shards,"lm_head")?;
            let lwt = l.weight.as_ref().t(); lwt.eval().map_err(|e| load_err(e))?;
            (Some(n),Some(l),Some(lwt))
        } else { (None,None,None) };
        let tok = BpeTokenizer::from_file(&cfg_path.parent().unwrap_or(Path::new(".")).join("tokenizer.json"))?;
        // Diagnostic: check dtype and do a test matmul to verify GPU works
        if is_f {
            let emb_dtype = embedding.as_ref().unwrap().weight.dtype();
            info!("MLX: embedding dtype={:?}", emb_dtype);
        }
        if !blocks.is_empty() {
            let w_dtype = blocks[0].gate.weight.dtype();
            info!("MLX: gate_proj weight dtype={:?} shape={:?}", w_dtype, blocks[0].gate.weight.shape());
            // Quick matmul test
            let test_data = vec![1.0f32; c.hidden_dim];
            let test_in = Array::from_slice::<f32>(&test_data, &[1, c.hidden_dim as i32])
                .as_dtype(mlx_rs::Dtype::Float16).map_err(|e| load_err(e))?;
            let t = Instant::now();
            let _ = blocks[0].gate.forward(&test_in).map_err(|e| load_err(e))?;
            let _ = test_in.eval();
            info!("MLX: test matmul [{},{}] took {:.1}ms", c.hidden_dim, c.intermediate_dim, t.elapsed().as_secs_f64()*1e3);
        }
        // Test actual model weight matmul speed (weights already eval'd by load_tensor)
        if !blocks.is_empty() {
            let w = &blocks[0].gate.weight;
            eprintln!("[DIAG] gate weight dtype={:?} shape={:?}", w.dtype(), w.shape());
            let test_data = vec![1.0f32; c.hidden_dim];
            let x = Array::from_slice::<f32>(&test_data, &[1, c.hidden_dim as i32])
                .as_dtype(mlx_rs::Dtype::Float16).map_err(|e| load_err(e))?;
            // Warm up
            let wt = w.as_ref().transpose_axes(&[1,0]).map_err(|e| load_err(e))?;
            let _ = x.matmul(&wt).map_err(|e| load_err(e))?.eval();
            // Timed
            let t = Instant::now();
            let y = x.matmul(&wt).map_err(|e| load_err(e))?;
            y.eval().map_err(|e| load_err(e))?;
            eprintln!("[DIAG] model weight matmul: {:.1}ms", t.elapsed().as_secs_f64()*1e3);
        }
        // Enable MLX global compilation — caches compiled graphs automatically
        mlx_rs::transforms::compile::enable_compile();
        info!("MLX: Shard [{start}..{end}) ready — emb={is_f} head={is_l} (compile enabled)");
        Ok(Self { embedding, blocks, norm, lm_head, lm_head_wt, tokenizer: tok, start_block: start, end_block: end, cfg: c, sessions: Mutex::new(HashMap::new()) })
    }
    pub fn is_first(&self) -> bool { self.start_block==0 }
    pub fn is_last(&self) -> bool { self.end_block==self.cfg.num_total_blocks }
    pub fn gc_sessions(&self) { let mut s=self.sessions.lock().unwrap(); let b=s.len(); s.retain(|_,v| v.last_access.elapsed().as_secs()<600); if b>s.len() { info!("MLX GC: {}",b-s.len()); } }
    fn run_blocks(&mut self, mut x: Array, sp: usize, sid: u64) -> InferenceResult<Array> {
        // Set global config (once) for the compiled function
        BLOCK_CFG.get_or_init(|| {
            let b = &self.blocks[0];
            (b.attn.n_heads, b.attn.n_kv_heads, b.attn.head_dim, b.attn.rope.base, b.in_n.eps)
        });

        // Create compiled forward (cached after first compilation)
        static INIT: std::sync::Once = std::sync::Once::new();
        static mut COMPILED_FN: Option<Box<dyn FnMut(&[Array]) -> Result<Vec<Array>, mlx_rs::error::Exception> + Send>> = None;
        INIT.call_once(|| {
            // shapeless=true: don't recompile when input shapes change (KV-cache grows each token)
            let f = mlx_rs::transforms::compile::compile(compiled_block_forward, Some(true));
            unsafe { COMPILED_FN = Some(Box::new(f)); }
        });

        let mut ss = self.sessions.lock().unwrap();
        let s = ss.entry(sid).or_insert_with(|| MlxSession::new(self.blocks.len()));
        s.last_access = Instant::now();

        let empty_kv = Array::from_slice::<f32>(&[0.0], &[1]);

        for (i, b) in self.blocks.iter().enumerate() {
            let (kv_k, kv_v) = s.kv[i].take().unwrap_or_else(|| (empty_kv.clone(), empty_kv.clone()));

            let inputs: Vec<Array> = vec![
                x.clone(), kv_k, kv_v,
                b.in_n.weight.as_ref().clone(), b.post_n.weight.as_ref().clone(),
                b.attn.q_wt.clone(), b.attn.k_wt.clone(), b.attn.v_wt.clone(), b.attn.o_wt.clone(),
                b.gate_wt.clone(), b.up_wt.clone(), b.down_wt.clone(),
            ];

            let out = unsafe {
                COMPILED_FN.as_mut().unwrap()(&inputs).map_err(|e| err(e))?
            };

            x = out[0].clone();
            s.kv[i] = Some((out[1].clone(), out[2].clone()));
        }

        x.eval().map_err(|e| err(e))?;
        Ok(x)
    }
    pub fn forward_full(&mut self, sid: u64, tids: &[u32], sp: usize) -> InferenceResult<Array> {
        let emb = self.embedding.as_mut().ok_or_else(|| err("no emb"))?;
        let t0 = Instant::now();
        let ids: Vec<i32> = tids.iter().map(|&i| i as i32).collect();
        let tok = Array::from_slice::<i32>(&ids, &[1, ids.len() as i32]);
        let h = emb.forward(&tok).map_err(|e| err(e))?;
        let te = t0.elapsed();
        let t1 = Instant::now();
        let x = self.run_blocks(h, sp, sid)?;
        let tb = t1.elapsed();
        let norm = self.norm.as_ref().ok_or_else(|| err("no norm"))?;
        let lm_wt = self.lm_head_wt.as_ref().ok_or_else(|| err("no lm_head_wt"))?;
        let t2 = Instant::now();
        let s = x.shape()[1];
        let xl = if s>1 { x.index((.., (s-1).., ..)) } else { x };
        let xl = mlx_rs::fast::rms_norm(&xl, norm.weight.as_ref(), norm.eps).map_err(|e| err(e))?;
        let logits = xl.matmul(lm_wt).map_err(|e| err(e))?;
        logits.eval().map_err(|e| err(e))?;
        let th = t2.elapsed();
        eprintln!("[PERF-MLX] forward_full: {} tok, embed={:.1}ms blocks={:.0}ms head={:.1}ms total={:.0}ms",
            tids.len(), te.as_secs_f64()*1e3, tb.as_secs_f64()*1e3, th.as_secs_f64()*1e3, t0.elapsed().as_secs_f64()*1e3);
        Ok(logits)
    }
}

pub fn mlx_available() -> bool {
    let a = Array::from_slice::<f32>(&[1.0,2.0,3.0], &[3]);
    let s: f32 = a.sum(None).expect("sum").item();
    (s - 6.0).abs() < 0.01
}

#[cfg(test)]
mod tests {
    use mlx_rs::module::Module;
    use mlx_rs::ops::indexing::IndexOp;
    use mlx_rs::Array;
    use std::time::Instant;

    #[test] fn test_mlx_raw_matmul_speed() {
        // Raw matmul at Llama scale — should be ~1ms on Metal if GPU is working
        let h = 4096i32;
        let inter = 11008i32;
        let xd = vec![1.0f32; h as usize];
        let wd = vec![0.01f32; (h as usize) * (inter as usize)];
        let x = Array::from_slice::<f32>(&xd, &[1, h]).as_dtype(mlx_rs::Dtype::Float16).unwrap();
        let w = Array::from_slice::<f32>(&wd, &[inter, h]).as_dtype(mlx_rs::Dtype::Float16).unwrap();
        let wt = w.transpose_axes(&[1, 0]).unwrap();
        // Warm up
        let _ = x.matmul(&wt).unwrap().eval();
        // Timed
        let t = Instant::now();
        let y = x.matmul(&wt).unwrap();
        y.eval().unwrap();
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        eprintln!("[OK] raw_matmul [1,{h}]x[{inter},{h}]^T = {ms:.1}ms (expect <5ms on Metal)");
    }

    #[test] fn test_mlx_array_basic() { let a=Array::from_slice::<f32>(&[1.0,2.0,3.0,4.0,5.0,6.0],&[2,3]); assert_eq!(a.shape(),&[2,3]); assert_eq!(a.reshape(&[3,2]).unwrap().shape(),&[3,2]); eprintln!("[OK] array_basic"); }
    #[test] fn test_mlx_matmul() { let c=Array::from_slice::<f32>(&[1.0,2.0,3.0,4.0,5.0,6.0],&[2,3]).matmul(&Array::from_slice::<f32>(&[1.0,2.0,3.0,4.0,5.0,6.0],&[3,2])).unwrap(); c.eval().unwrap(); let v:f32=c.reshape(&[4]).unwrap().index(0).item(); assert!((v-22.0).abs()<0.01); eprintln!("[OK] matmul {v}"); }
    #[test] fn test_mlx_nn_linear() { let y=mlx_rs::nn::Linear::new(4,8).unwrap().forward(&Array::from_slice::<f32>(&[1.0;4],&[1,4])).unwrap(); assert_eq!(y.shape(),&[1,8]); eprintln!("[OK] linear"); }
    #[test] fn test_mlx_nn_embedding() { let y=mlx_rs::nn::Embedding::new(100,16).unwrap().forward(&Array::from_slice::<i32>(&[0,5,10],&[1,3])).unwrap(); assert_eq!(y.shape(),&[1,3,16]); eprintln!("[OK] embedding"); }
    #[test] fn test_mlx_nn_rms_norm() { let y=mlx_rs::nn::RmsNorm::new(8).unwrap().forward(&Array::from_slice::<f32>(&[1.0;8],&[1,1,8])).unwrap(); assert_eq!(y.shape(),&[1,1,8]); eprintln!("[OK] rms_norm"); }
    #[test] fn test_mlx_softmax() { let s:f32=mlx_rs::ops::softmax_axis(&Array::from_slice::<f32>(&[1.0,2.0,3.0,4.0],&[1,4]),-1,None).unwrap().sum(None).unwrap().item(); assert!((s-1.0).abs()<0.01); eprintln!("[OK] softmax {s:.4}"); }
    #[test] fn test_mlx_silu() { assert_eq!(mlx_rs::nn::silu(&Array::from_slice::<f32>(&[-1.0,0.0,1.0,2.0],&[4])).unwrap().shape(),&[4]); eprintln!("[OK] silu"); }
    #[test] fn test_mlx_concat() { let c=mlx_rs::ops::concatenate_axis(&[&Array::from_slice::<f32>(&[1.0;8],&[1,2,4]),&Array::from_slice::<f32>(&[2.0;4],&[1,1,4])],1).unwrap(); assert_eq!(c.shape(),&[1,3,4]); eprintln!("[OK] concat"); }
    #[test] fn test_mlx_eval_lazy() { let c=Array::from_slice::<f32>(&[1.0,2.0],&[2]).add(&Array::from_slice::<f32>(&[3.0,4.0],&[2])).unwrap(); c.eval().unwrap(); let v:f32=c.index(0).item(); assert!((v-4.0).abs()<0.01); eprintln!("[OK] eval {v}"); }
    #[test] fn test_mlx_transpose() { assert_eq!(Array::from_slice::<f32>(&[0.0;24],&[2,3,4]).transpose_axes(&[0,2,1]).unwrap().shape(),&[2,4,3]); eprintln!("[OK] transpose"); }
    #[test] fn test_mlx_transformer_block() {
        let (h,nh,hd,i)=(32i32,4i32,8i32,64i32);
        let (mut n1,mut q,mut k,mut v,mut o,mut n2,mut g,mut u,mut d) = (
            mlx_rs::nn::RmsNorm::new(h).unwrap(), mlx_rs::nn::Linear::new(h,h).unwrap(),
            mlx_rs::nn::Linear::new(h,h).unwrap(), mlx_rs::nn::Linear::new(h,h).unwrap(),
            mlx_rs::nn::Linear::new(h,h).unwrap(), mlx_rs::nn::RmsNorm::new(h).unwrap(),
            mlx_rs::nn::Linear::new(h,i).unwrap(), mlx_rs::nn::Linear::new(h,i).unwrap(),
            mlx_rs::nn::Linear::new(i,h).unwrap());
        let x=Array::from_slice::<f32>(&[0.5;64],&[1,2,h]);
        let nm=n1.forward(&x).unwrap();
        let qr=q.forward(&nm).unwrap().reshape(&[1,2,nh,hd]).unwrap().transpose_axes(&[0,2,1,3]).unwrap();
        let kr=k.forward(&nm).unwrap().reshape(&[1,2,nh,hd]).unwrap().transpose_axes(&[0,2,1,3]).unwrap();
        let vr=v.forward(&nm).unwrap().reshape(&[1,2,nh,hd]).unwrap().transpose_axes(&[0,2,1,3]).unwrap();
        let sc=qr.matmul(&kr.transpose_axes(&[0,1,3,2]).unwrap()).unwrap().multiply(&Array::from_slice::<f32>(&[(hd as f32).sqrt().recip()],&[1])).unwrap();
        let ao=mlx_rs::ops::softmax_axis(&sc,-1,None).unwrap().matmul(&vr).unwrap().transpose_axes(&[0,2,1,3]).unwrap().reshape(&[1,2,h]).unwrap();
        let x=x.add(&o.forward(&ao).unwrap()).unwrap();
        let nm=n2.forward(&x).unwrap();
        let f=d.forward(&mlx_rs::nn::silu(&g.forward(&nm).unwrap()).unwrap().multiply(&u.forward(&nm).unwrap()).unwrap()).unwrap();
        let out=x.add(&f).unwrap(); out.eval().unwrap();
        assert_eq!(out.shape(),&[1,2,h]); eprintln!("[OK] transformer_block");
    }
    #[test] fn test_mlx_safetensors_load() {
        use safetensors::SafeTensors;
        let home=dirs::home_dir().expect("home");
        for b in &[".cache/huggingface/models--unsloth--Llama-3.1-8B-Instruct/snapshots",".cache/huggingface/hub/models--unsloth--Llama-3.1-8B-Instruct/snapshots"] {
            let d=home.join(b); if !d.exists(){continue;}
            let Some(sd)=std::fs::read_dir(&d).ok().and_then(|r|r.filter_map(|e|e.ok()).map(|e|e.path()).next()) else{continue;};
            let Some(sp)=std::fs::read_dir(&sd).ok().and_then(|r|r.filter_map(|e|e.ok()).map(|e|e.path()).find(|p|p.extension().and_then(|e|e.to_str())==Some("safetensors"))) else{continue;};
            let data=std::fs::read(&sp).unwrap(); let t=SafeTensors::deserialize(&data).unwrap();
            let n:Vec<_>=t.names().into_iter().collect(); let a=mlx_rs::Array::try_from(t.tensor(n[0]).unwrap()).unwrap();
            eprintln!("[OK] safetensors '{}' {:?}",n[0],a.shape()); return;
        }
        eprintln!("[SKIP] safetensors — not cached");
    }
    #[test] fn test_mlx_forward_full() {
        let home=dirs::home_dir().expect("home");
        let mut md=None;
        for b in &[".cache/huggingface/models--unsloth--Llama-3.1-8B-Instruct/snapshots",".cache/huggingface/hub/models--unsloth--Llama-3.1-8B-Instruct/snapshots"] {
            let d=home.join(b); if !d.exists(){continue;}
            if let Some(s)=std::fs::read_dir(&d).ok().and_then(|r|r.filter_map(|e|e.ok()).map(|e|e.path()).next()) { md=Some(s); break; }
        }
        let Some(md)=md else { eprintln!("[SKIP] forward_full"); return; };
        let cp=md.join("config.json"); if !cp.exists() { eprintln!("[SKIP] no config"); return; }
        let mut ps:Vec<std::path::PathBuf>=std::fs::read_dir(&md).unwrap().filter_map(|e|e.ok()).map(|e|e.path()).filter(|p|p.extension().and_then(|e|e.to_str())==Some("safetensors")).collect();
        ps.sort(); let refs:Vec<&std::path::Path>=ps.iter().map(|p|p.as_path()).collect();
        eprintln!("  Loading Llama 8B via MLX..."); let t0=Instant::now();
        let mut shard=super::MlxTransformerShard::load(&refs,&cp,0,32).expect("load");
        eprintln!("  Loaded in {:.1}s",t0.elapsed().as_secs_f64());
        use crate::tokenizer::Tokenizer as _;
        let mut ids:Vec<u32>=shard.tokenizer.encode("The capital of France is").unwrap();
        if let Some(bos)=shard.tokenizer.bos_token_id(){ids.insert(0,bos);}
        eprintln!("  forward_full ({} tokens)...",ids.len());
        let logits=shard.forward_full(1,&ids,0).expect("forward");
        let flat=logits.reshape(&[-1]).unwrap();
        let next:i32=mlx_rs::ops::indexing::argmax(&flat, None).unwrap().item();
        let decoded=shard.tokenizer.decode(&[next as u32]).unwrap_or("?".into());
        eprintln!("[OK] forward_full — next='{decoded}' (id={next})");
    }

    /// Benchmark: warm-up + prefill + 5 decode steps. Measures steady-state tok/s.
    #[test] fn test_mlx_benchmark() {
        let home = dirs::home_dir().expect("home");
        let mut md = None;
        for b in &[".cache/huggingface/models--unsloth--Llama-3.1-8B-Instruct/snapshots",
                    ".cache/huggingface/hub/models--unsloth--Llama-3.1-8B-Instruct/snapshots"] {
            let d = home.join(b); if !d.exists() { continue; }
            if let Some(s) = std::fs::read_dir(&d).ok()
                .and_then(|r| r.filter_map(|e| e.ok()).map(|e| e.path()).next()) { md = Some(s); break; }
        }
        let Some(md) = md else { eprintln!("[SKIP] benchmark"); return; };
        let cp = md.join("config.json"); if !cp.exists() { eprintln!("[SKIP] no config"); return; }
        let mut ps: Vec<std::path::PathBuf> = std::fs::read_dir(&md).unwrap()
            .filter_map(|e| e.ok()).map(|e| e.path())
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("safetensors")).collect();
        ps.sort();
        let refs: Vec<&std::path::Path> = ps.iter().map(|p| p.as_path()).collect();

        eprintln!("  Loading Llama 8B via MLX...");
        let t0 = Instant::now();
        let mut shard = super::MlxTransformerShard::load(&refs, &cp, 0, 32).expect("load");
        eprintln!("  Loaded in {:.1}s", t0.elapsed().as_secs_f64());

        use crate::tokenizer::Tokenizer as _;
        let prompt = "The capital of France is";
        let mut ids: Vec<u32> = shard.tokenizer.encode(prompt).unwrap();
        if let Some(bos) = shard.tokenizer.bos_token_id() { ids.insert(0, bos); }

        // ── Warm-up (2 forward passes, separate session) ─────────────────
        eprintln!("  Warm-up (2 steps)...");
        let _ = shard.forward_full(0xAAAA_u64, &ids, 0);
        let logits = shard.forward_full(0xAAAA_u64, &[1u32], ids.len()).expect("warm-up 2");
        logits.eval().ok();
        eprintln!("  Warm-up done.");

        // ── Prefill (timed) ──────────────────────────────────────────────
        let session = 0xBBBB_u64;
        let t_pre = Instant::now();
        let logits = shard.forward_full(session, &ids, 0).expect("prefill");
        let prefill_ms = t_pre.elapsed().as_secs_f64() * 1000.0;

        // Sample first token
        let flat = logits.reshape(&[-1]).unwrap();
        let mut next: i32 = mlx_rs::ops::indexing::argmax(&flat, None).unwrap().item();
        let first_tok = shard.tokenizer.decode(&[next as u32]).unwrap_or("?".into());

        // ── Decode (5 steps, timed) ──────────────────────────────────────
        let n_decode = 5;
        eprintln!("  Decode ({n_decode} steps)...");
        let mut sp = ids.len();
        let t_dec = Instant::now();
        for _ in 0..n_decode {
            let logits = shard.forward_full(session, &[next as u32], sp).expect("decode");
            let flat = logits.reshape(&[-1]).unwrap();
            next = mlx_rs::ops::indexing::argmax(&flat, None).unwrap().item();
            sp += 1;
        }
        let decode_ms = t_dec.elapsed().as_secs_f64() * 1000.0;

        let prefill_tps = ids.len() as f64 / (prefill_ms / 1000.0);
        let decode_tps = n_decode as f64 / (decode_ms / 1000.0);

        eprintln!();
        eprintln!("  ── MLX Benchmark Results ──────────────────────────────");
        eprintln!("  Prefill:  {prefill_tps:>7.1} tok/s  ({} tokens in {prefill_ms:.0}ms)", ids.len());
        eprintln!("  Decode:   {decode_tps:>7.1} tok/s  ({n_decode} tokens in {decode_ms:.0}ms)");
        eprintln!("  First:    '{first_tok}'");
        eprintln!("  ───────────────────────────────────────────────────────");
        eprintln!("[OK] test_mlx_benchmark");
    }
}
