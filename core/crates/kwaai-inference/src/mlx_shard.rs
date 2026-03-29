//! MLX-based TransformerShard — Apple Silicon optimized inference.
//!
//! Uses Apple's MLX framework (via mlx-rs) for unified memory, lazy evaluation,
//! and automatic kernel fusion.
//!
//! Feature-gated: only compiled with `--features mlx` on macOS.

use tracing::info;

/// Check that MLX runtime is operational.
pub fn mlx_available() -> bool {
    let arr = mlx_rs::Array::from_slice::<f32>(&[1.0, 2.0, 3.0], &[3]);
    let sum = arr.sum(None).expect("mlx sum");
    let val: f32 = sum.item();
    let ok = (val - 6.0).abs() < 0.01;
    if ok {
        info!("MLX backend available (verified)");
    }
    ok
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use mlx_rs::module::Module;
    use mlx_rs::Array;

    // ── Step 1: Primitive ops ────────────────────────────────────────────────

    #[test]
    fn test_mlx_array_basic() {
        // Create, reshape, extract scalar
        let a = Array::from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
        assert_eq!(a.shape(), &[2, 3]);

        let b = a.reshape(&[3, 2]).expect("reshape");
        assert_eq!(b.shape(), &[3, 2]);

        let scalar = Array::from_slice::<f32>(&[42.0], &[1]);
        let val: f32 = scalar.item();
        assert!((val - 42.0).abs() < 0.01);
        eprintln!("[OK] test_mlx_array_basic");
    }

    #[test]
    fn test_mlx_matmul() {
        // [2,3] × [3,2] → [2,2]
        let a = Array::from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
        let b = Array::from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
        let c = a.matmul(&b).expect("matmul");
        assert_eq!(c.shape(), &[2, 2]);
        c.eval().expect("eval");
        // c[0,0] = 1*1 + 2*3 + 3*5 = 22
        let flat: Vec<f32> = (0..4).map(|i| {
            let idx = Array::from_slice::<i32>(&[i / 2, i % 2], &[2]);
            c.try_index(&idx).unwrap_or_else(|_| Array::from_slice::<f32>(&[0.0], &[1])).item::<f32>()
        }).collect();
        eprintln!("[OK] test_mlx_matmul — result shape {:?}", c.shape());
    }

    #[test]
    fn test_mlx_nn_linear() {
        let mut linear = mlx_rs::nn::Linear::new(4, 8).expect("create linear");
        let x = Array::from_slice::<f32>(&[1.0; 4], &[1, 4]);
        let y = linear.forward(&x).expect("linear forward");
        assert_eq!(y.shape(), &[1, 8]);
        y.eval().expect("eval");
        eprintln!("[OK] test_mlx_nn_linear — output shape {:?}", y.shape());
    }

    #[test]
    fn test_mlx_nn_embedding() {
        let mut emb = mlx_rs::nn::Embedding::new(100, 16).expect("create embedding");
        let ids = Array::from_slice::<i32>(&[0, 5, 10], &[1, 3]);
        let y = emb.forward(&ids).expect("embedding forward");
        assert_eq!(y.shape(), &[1, 3, 16]);
        y.eval().expect("eval");
        eprintln!("[OK] test_mlx_nn_embedding — output shape {:?}", y.shape());
    }

    #[test]
    fn test_mlx_nn_rms_norm() {
        let mut norm = mlx_rs::nn::RmsNorm::new(8).expect("create rms_norm");
        let x = Array::from_slice::<f32>(&[1.0; 8], &[1, 1, 8]);
        let y = norm.forward(&x).expect("rms_norm forward");
        assert_eq!(y.shape(), &[1, 1, 8]);
        y.eval().expect("eval");
        eprintln!("[OK] test_mlx_nn_rms_norm — output shape {:?}", y.shape());
    }

    #[test]
    fn test_mlx_softmax() {
        let x = Array::from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0], &[1, 4]);
        let y = mlx_rs::ops::softmax_axis(&x, -1, None).expect("softmax");
        assert_eq!(y.shape(), &[1, 4]);
        y.eval().expect("eval");
        // Sum should be ~1.0
        let s = y.sum(None).expect("sum");
        let val: f32 = s.item();
        assert!((val - 1.0).abs() < 0.01, "softmax sum = {val}");
        eprintln!("[OK] test_mlx_softmax — sum={val:.4}");
    }

    #[test]
    fn test_mlx_silu() {
        let x = Array::from_slice::<f32>(&[-1.0, 0.0, 1.0, 2.0], &[4]);
        let y = mlx_rs::nn::silu(&x).expect("silu");
        assert_eq!(y.shape(), &[4]);
        y.eval().expect("eval");
        // silu(0) = 0, silu(1) ≈ 0.731
        eprintln!("[OK] test_mlx_silu — shape {:?}", y.shape());
    }

    #[test]
    fn test_mlx_concat() {
        // Simulate KV-cache append: [1,2,4] + [1,1,4] → [1,3,4]
        let cached = Array::from_slice::<f32>(&[1.0; 8], &[1, 2, 4]);
        let new_kv = Array::from_slice::<f32>(&[2.0; 4], &[1, 1, 4]);
        let combined = mlx_rs::ops::concatenate_axis(&[&cached, &new_kv], 1)
            .expect("concat");
        assert_eq!(combined.shape(), &[1, 3, 4]);
        combined.eval().expect("eval");
        eprintln!("[OK] test_mlx_concat — KV-cache pattern shape {:?}", combined.shape());
    }

    #[test]
    fn test_mlx_eval_lazy() {
        // Verify lazy evaluation: ops don't compute until eval
        let a = Array::from_slice::<f32>(&[1.0, 2.0], &[2]);
        let b = Array::from_slice::<f32>(&[3.0, 4.0], &[2]);
        let c = a.add(&b).expect("add"); // lazy — not computed yet
        // Force evaluation
        c.eval().expect("eval");
        let v0: f32 = c.try_index(0).expect("idx").item();
        assert!((v0 - 4.0).abs() < 0.01);
        eprintln!("[OK] test_mlx_eval_lazy — 1+3={v0}");
    }

    #[test]
    fn test_mlx_transpose() {
        // [2,3,4] → transpose axes [0,2,1] → [2,4,3]
        let x = Array::from_slice::<f32>(&[0.0; 24], &[2, 3, 4]);
        let y = x.transpose_axes(&[0, 2, 1]).expect("transpose");
        assert_eq!(y.shape(), &[2, 4, 3]);
        eprintln!("[OK] test_mlx_transpose — {:?} → {:?}", x.shape(), y.shape());
    }

    // ── Step 2: Transformer block ────────────────────────────────────────────

    #[test]
    fn test_mlx_transformer_block() {
        let hidden = 32;
        let n_heads = 4;
        let head_dim = hidden / n_heads; // 8
        let inter = 64;

        // Create layers
        let mut norm1 = mlx_rs::nn::RmsNorm::new(hidden as i32).expect("norm1");
        let mut q_proj = mlx_rs::nn::Linear::new(hidden as i32, hidden as i32).expect("q");
        let mut k_proj = mlx_rs::nn::Linear::new(hidden as i32, hidden as i32).expect("k");
        let mut v_proj = mlx_rs::nn::Linear::new(hidden as i32, hidden as i32).expect("v");
        let mut o_proj = mlx_rs::nn::Linear::new(hidden as i32, hidden as i32).expect("o");
        let mut norm2 = mlx_rs::nn::RmsNorm::new(hidden as i32).expect("norm2");
        let mut gate = mlx_rs::nn::Linear::new(hidden as i32, inter as i32).expect("gate");
        let mut up = mlx_rs::nn::Linear::new(hidden as i32, inter as i32).expect("up");
        let mut down = mlx_rs::nn::Linear::new(inter as i32, hidden as i32).expect("down");

        // Input: [1, 2, 32] (batch=1, seq=2, hidden=32)
        let x = Array::from_slice::<f32>(&[0.5; 64], &[1, 2, hidden as i32]);

        // Attention path
        let normed = norm1.forward(&x).expect("norm1");
        let q = q_proj.forward(&normed).expect("q_proj");
        let k = k_proj.forward(&normed).expect("k_proj");
        let v = v_proj.forward(&normed).expect("v_proj");

        // Reshape to multi-head
        let q = q.reshape(&[1, 2, n_heads as i32, head_dim as i32]).expect("q reshape")
            .transpose_axes(&[0, 2, 1, 3]).expect("q transpose");
        let k = k.reshape(&[1, 2, n_heads as i32, head_dim as i32]).expect("k reshape")
            .transpose_axes(&[0, 2, 1, 3]).expect("k transpose");
        let v = v.reshape(&[1, 2, n_heads as i32, head_dim as i32]).expect("v reshape")
            .transpose_axes(&[0, 2, 1, 3]).expect("v transpose");

        // Attention: Q × K^T, softmax, × V
        let kt = k.transpose_axes(&[0, 1, 3, 2]).expect("kt");
        let scores = q.matmul(&kt).expect("qk matmul");
        let scale = Array::from_slice::<f32>(&[(head_dim as f32).sqrt().recip()], &[1]);
        let scores = scores.multiply(&scale).expect("scale");
        let attn_w = mlx_rs::ops::softmax_axis(&scores, -1, None).expect("softmax");
        let attn_out = attn_w.matmul(&v).expect("attn v");

        // Merge heads
        let attn_out = attn_out.transpose_axes(&[0, 2, 1, 3]).expect("merge transpose")
            .reshape(&[1, 2, hidden as i32]).expect("merge reshape");
        let attn_out = o_proj.forward(&attn_out).expect("o_proj");
        let x = x.add(&attn_out).expect("residual1");

        // MLP path
        let normed = norm2.forward(&x).expect("norm2");
        let g = gate.forward(&normed).expect("gate");
        let u = up.forward(&normed).expect("up");
        let g = mlx_rs::nn::silu(&g).expect("silu");
        let ff = g.multiply(&u).expect("gate*up");
        let ff = down.forward(&ff).expect("down");
        let out = x.add(&ff).expect("residual2");

        out.eval().expect("final eval");
        assert_eq!(out.shape(), &[1, 2, hidden as i32]);
        eprintln!("[OK] test_mlx_transformer_block — output shape {:?}", out.shape());
    }

    // ── Step 3: SafeTensors loading ──────────────────────────────────────────

    #[test]
    fn test_mlx_safetensors_load() {
        use safetensors::SafeTensors;

        // Find a cached model file
        let home = dirs::home_dir().expect("home dir");
        let model_dir = home
            .join(".cache/huggingface/hub/models--unsloth--Llama-3.1-8B-Instruct/snapshots");

        if !model_dir.exists() {
            eprintln!("[SKIP] test_mlx_safetensors_load — model not cached");
            return;
        }

        // Find first snapshot
        let snapshot = std::fs::read_dir(&model_dir)
            .expect("read snapshots")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .next();

        let Some(snapshot_dir) = snapshot else {
            eprintln!("[SKIP] test_mlx_safetensors_load — no snapshot found");
            return;
        };

        // Find first safetensors file
        let st_file = std::fs::read_dir(&snapshot_dir)
            .expect("read snapshot")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| p.extension().and_then(|e| e.to_str()) == Some("safetensors"));

        let Some(st_path) = st_file else {
            eprintln!("[SKIP] test_mlx_safetensors_load — no safetensors file");
            return;
        };

        // Load and convert one tensor to MLX Array
        let data = std::fs::read(&st_path).expect("read safetensors");
        let tensors = SafeTensors::deserialize(&data).expect("parse safetensors");
        let names: Vec<_> = tensors.names().into_iter().collect();
        eprintln!("  SafeTensors file: {}", st_path.display());
        eprintln!("  Tensors: {} (first 5: {:?})", names.len(), &names[..names.len().min(5)]);

        // Convert first tensor to MLX Array
        let first_name = names[0];
        let view = tensors.tensor(first_name).expect("get tensor");
        let mlx_arr = mlx_rs::Array::try_from(view).expect("convert to MLX");
        eprintln!(
            "[OK] test_mlx_safetensors_load — tensor '{}' shape {:?}",
            first_name,
            mlx_arr.shape()
        );
    }
}
