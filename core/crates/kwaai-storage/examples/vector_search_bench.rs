//! HNSW vs brute-force vector search benchmark.
//!
//! Usage:
//!   cargo run --example vector_search_bench --release -p kwaai-storage
//!
//! Tests four dimensions:
//!   1. ef_search sweep     — how recall varies with ef at fixed corpus size (10K)
//!   2. Corpus size sweep   — random vectors (worst case: uniform on hypersphere)
//!   3. Corpus size sweep   — clustered vectors (realistic: text embeddings cluster by topic)
//!   4. Build params sweep  — how m and ef_construction affect recall (clustered, 10K)
//!
//! Reports latency (µs, median) and Recall@10 for each condition.

use kwaai_storage::TenantIndex;
use std::time::Instant;

const DIM: usize = 768;
const N_QUERIES: usize = 200;
const TOP_K: usize = 10;
const SIZES: &[usize] = &[200, 500, 865, 1_000, 2_000, 5_000, 10_000, 50_000];
const EF_VALUES: &[usize] = &[32, 64, 128, 256, 400, 512, 1024];
const EF_CORPUS: usize = 10_000;
const N_CLUSTERS: usize = 200; // for clustered synthetic data

struct Xorshift(u64);
impl Xorshift {
    fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }
    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn next_f32(&mut self) -> f32 {
        (self.next_u64() as f32) / (u64::MAX as f32)
    }
    fn unit_vec(&mut self, dim: usize) -> Vec<f32> {
        let v: Vec<f32> = (0..dim).map(|_| self.next_f32() * 2.0 - 1.0).collect();
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-9);
        v.into_iter().map(|x| x / norm).collect()
    }
    /// Clustered vector: pick random cluster centre, add Gaussian-like noise (σ≈0.08).
    fn clustered_vec(&mut self, centres: &[Vec<f32>]) -> Vec<f32> {
        let ci = (self.next_u64() as usize) % centres.len();
        let c = &centres[ci];
        let v: Vec<f32> = c
            .iter()
            .map(|&x| x + (self.next_f32() - 0.5) * 0.16)
            .collect();
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-9);
        v.into_iter().map(|x| x / norm).collect()
    }
}

fn build_index(vecs: &[Vec<f32>]) -> TenantIndex {
    build_index_params(vecs, 16, 200)
}

fn build_index_params(vecs: &[Vec<f32>], m: usize, ef_construction: usize) -> TenantIndex {
    let mut idx = TenantIndex::new_with_params(DIM, m, ef_construction);
    for (i, v) in vecs.iter().enumerate() {
        idx.insert(i as i64, v);
    }
    idx
}

fn recall_and_latency(index: &TenantIndex, queries: &[Vec<f32>], ef: usize) -> (f64, f64, f64) {
    let mut brute_times = Vec::with_capacity(N_QUERIES);
    let mut hnsw_times = Vec::with_capacity(N_QUERIES);
    let mut recall_sum = 0.0f64;

    for q in queries {
        let t0 = Instant::now();
        let exact = index.search_exact(q, TOP_K);
        brute_times.push(t0.elapsed().as_micros());

        let t1 = Instant::now();
        let approx = index.search_hnsw_ef(q, TOP_K, ef);
        hnsw_times.push(t1.elapsed().as_micros());

        let exact_ids: std::collections::HashSet<i64> = exact.iter().map(|(id, _)| *id).collect();
        let hits = approx
            .iter()
            .filter(|(id, _)| exact_ids.contains(id))
            .count();
        recall_sum += hits as f64 / TOP_K.min(exact.len()).max(1) as f64;
    }

    brute_times.sort_unstable();
    hnsw_times.sort_unstable();
    let n = N_QUERIES as f64;
    (
        brute_times[N_QUERIES / 2] as f64,
        hnsw_times[N_QUERIES / 2] as f64,
        recall_sum / n,
    )
}

fn main() {
    // ── 1. ef_search sweep at 10K corpus (random vectors) ────────────────────
    println!("════════════════════════════════════════════════════════════════════");
    println!(
        "  1. ef_search sweep  |  corpus={EF_CORPUS}  random vectors  dim={DIM}  top_k={TOP_K}"
    );
    println!("════════════════════════════════════════════════════════════════════");
    println!(
        "{:>8}  {:>14}  {:>14}  {:>10}  {:>8}",
        "ef_search", "Brute (µs)", "HNSW (µs)", "Recall@10", "Speedup"
    );
    println!("{}", "-".repeat(62));

    let mut rng = Xorshift::new(42);
    let corpus_vecs: Vec<Vec<f32>> = (0..EF_CORPUS).map(|_| rng.unit_vec(DIM)).collect();
    let queries: Vec<Vec<f32>> = (0..N_QUERIES).map(|_| rng.unit_vec(DIM)).collect();
    let index = build_index(&corpus_vecs);

    for &ef in EF_VALUES {
        let (brute, hnsw, recall) = recall_and_latency(&index, &queries, ef);
        let speedup = brute / hnsw.max(0.001);
        let marker = if ef == 64 { " ← current default" } else { "" };
        println!(
            "{:>8}  {:>14.1}  {:>14.1}  {:>9.1}%  {:>7.1}x{}",
            ef,
            brute,
            hnsw,
            recall * 100.0,
            speedup,
            marker
        );
    }

    // ── 2. Corpus sweep — random vectors (worst case) ────────────────────────
    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("  2. Corpus sweep  |  RANDOM vectors  ef_search=64 vs 200  dim={DIM}  top_k={TOP_K}");
    println!("════════════════════════════════════════════════════════════════════");
    println!(
        "{:>8}  {:>12}  {:>12}  {:>12}  {:>12}  Mode",
        "Corpus", "Brute(µs)", "HNSW64(%)", "HNSW200(%)", "Speedup@200"
    );
    println!("{}", "-".repeat(72));

    for &size in SIZES {
        let mut rng2 = Xorshift::new(42);
        let vecs: Vec<Vec<f32>> = (0..size).map(|_| rng2.unit_vec(DIM)).collect();
        let qs: Vec<Vec<f32>> = (0..N_QUERIES).map(|_| rng2.unit_vec(DIM)).collect();
        let idx = build_index(&vecs);
        let (brute, _hnsw64, rec64) = recall_and_latency(&idx, &qs, 64);
        let (_, hnsw200, rec200) = recall_and_latency(&idx, &qs, 200);
        let speedup = brute / hnsw200.max(0.001);
        let mode = if size < 2_000 {
            "brute ◀"
        } else {
            "HNSW ◀"
        };
        println!(
            "{:>8}  {:>12.1}  {:>11.1}%  {:>11.1}%  {:>11.1}x  {}",
            size,
            brute,
            rec64 * 100.0,
            rec200 * 100.0,
            speedup,
            mode
        );
    }

    // ── 3. Corpus sweep — clustered vectors (realistic text embeddings) ───────
    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("  3. Corpus sweep  |  CLUSTERED vectors ({N_CLUSTERS} centres, σ≈0.08)  ef_search=64 vs 200");
    println!("     Simulates real text embedding distributions (topically grouped)");
    println!("════════════════════════════════════════════════════════════════════");
    println!(
        "{:>8}  {:>12}  {:>12}  {:>12}  {:>12}  Mode",
        "Corpus", "Brute(µs)", "HNSW64(%)", "HNSW200(%)", "Speedup@200"
    );
    println!("{}", "-".repeat(72));

    // Generate cluster centres once.
    let mut rng3 = Xorshift::new(7);
    let centres: Vec<Vec<f32>> = (0..N_CLUSTERS).map(|_| rng3.unit_vec(DIM)).collect();

    for &size in SIZES {
        let mut rng4 = Xorshift::new(42);
        let vecs: Vec<Vec<f32>> = (0..size).map(|_| rng4.clustered_vec(&centres)).collect();
        let qs: Vec<Vec<f32>> = (0..N_QUERIES)
            .map(|_| rng4.clustered_vec(&centres))
            .collect();
        let idx = build_index(&vecs);
        let (brute, _hnsw64, rec64) = recall_and_latency(&idx, &qs, 64);
        let (_, hnsw200, rec200) = recall_and_latency(&idx, &qs, 200);
        let speedup = brute / hnsw200.max(0.001);
        let mode = if size < 2_000 {
            "brute ◀"
        } else {
            "HNSW ◀"
        };
        println!(
            "{:>8}  {:>12.1}  {:>11.1}%  {:>11.1}%  {:>11.1}x  {}",
            size,
            brute,
            rec64 * 100.0,
            rec200 * 100.0,
            speedup,
            mode
        );
    }

    // ── 4. Build params sweep — m and ef_construction at 10K clustered ─────────
    println!();
    println!("════════════════════════════════════════════════════════════════════");
    println!("  4. Build params sweep  |  corpus={EF_CORPUS}  clustered vectors  ef_search=128");
    println!("════════════════════════════════════════════════════════════════════");

    let mut rng5 = Xorshift::new(7);
    let bp_centres: Vec<Vec<f32>> = (0..N_CLUSTERS).map(|_| rng5.unit_vec(DIM)).collect();
    let mut rng6 = Xorshift::new(42);
    let bp_corpus: Vec<Vec<f32>> = (0..EF_CORPUS)
        .map(|_| rng6.clustered_vec(&bp_centres))
        .collect();
    let bp_queries: Vec<Vec<f32>> = (0..N_QUERIES)
        .map(|_| rng6.clustered_vec(&bp_centres))
        .collect();

    println!();
    println!("  4a. ef_construction sweep  (m=16 fixed)");
    println!(
        "{:>18}  {:>14}  {:>10}",
        "ef_construction", "Build (ms)", "Recall@10"
    );
    println!("{}", "-".repeat(48));

    for &efc in &[32usize, 64, 100, 200, 400] {
        let t_build = Instant::now();
        let idx = build_index_params(&bp_corpus, 16, efc);
        let build_ms = t_build.elapsed().as_millis();
        let (_, _, recall) = recall_and_latency(&idx, &bp_queries, 128);
        let marker = if efc == 64 {
            " ← old default"
        } else if efc == 200 {
            " ← new default"
        } else {
            ""
        };
        println!(
            "{:>18}  {:>13}ms  {:>9.1}%{}",
            efc,
            build_ms,
            recall * 100.0,
            marker
        );
    }

    println!();
    println!("  4b. m sweep  (ef_construction=200 fixed)");
    println!(
        "{:>6}  {:>14}  {:>10}  {:>12}",
        "m", "Build (ms)", "Recall@10", "Δ vs m=16"
    );
    println!("{}", "-".repeat(48));

    let (_, _, baseline_recall) = {
        let idx = build_index_params(&bp_corpus, 16, 200);
        recall_and_latency(&idx, &bp_queries, 128)
    };
    for &m in &[8usize, 12, 16, 24, 32, 48] {
        let t_build = Instant::now();
        let idx = build_index_params(&bp_corpus, m, 200);
        let build_ms = t_build.elapsed().as_millis();
        let (_, _, recall) = recall_and_latency(&idx, &bp_queries, 128);
        let delta = recall * 100.0 - baseline_recall * 100.0;
        let marker = if m == 16 { " ← default" } else { "" };
        println!(
            "{:>6}  {:>13}ms  {:>9.1}%  {:>+10.1}pp{}",
            m,
            build_ms,
            recall * 100.0,
            delta,
            marker
        );
    }

    println!();
    println!("Notes:");
    println!("  • Random vectors = worst case: uniform on hypersphere, no cluster structure");
    println!(
        "  • Clustered vectors = realistic: {N_CLUSTERS} topic clusters, Gaussian spread σ≈0.08"
    );
    println!("  • Adjacent benchmark used real text embeddings (inherently clustered)");
    println!("  • Adjacent benchmark measured avg similarity, not recall@k vs exact ground truth");
    println!("  • ef_construction minimum safe floor: 40 (hnswlib upstream warning)");
    println!("  • Default changed: ef_construction 64 → 200 (one-time rebuild cost on startup)");
}
