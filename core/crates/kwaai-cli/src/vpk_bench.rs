//! `kwaainet vpk bench` — sharded vs local vector DB performance experiment.
//!
//! Hypothesis: a Bob node fanning out searches to multiple remote Eve shards
//! achieves competitive latency vs a single local index, because each shard
//! is smaller (faster HNSW traversal) and the fan-out is parallel.
//!
//! Methodology:
//!   1. Measure pure P2P round-trip overhead (health ping × 20 samples).
//!   2. For each vector scale in [vectors/4, vectors/2, vectors]:
//!      a. Local: load all N vectors, benchmark Q queries.
//!      b. Sharded: split N across K Eves, fan-out, benchmark Q queries.
//!   3. Crossover table: at what N does local_p50 - local_p50/K > p2p_overhead?
//!   4. Recall@K: |sharded_top_k ∩ local_top_k| / K.

use anyhow::{Context, Result};
use futures::future::join_all;
use kwaai_p2p_daemon::P2PClient;
use kwaai_storage::{StorageDb, TenantManager, VectorStore};
use libp2p::PeerId;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::cli::BenchArgs;
use crate::display::*;
use crate::storage_rpc::{
    rpc_create_tenant, rpc_delete_tenant, rpc_health, rpc_search_vectors, rpc_upload_vectors,
    CreateTenantPayload,
};

// ── Entry point ──────────────────────────────────────────────────────────────

pub async fn run(args: BenchArgs) -> Result<()> {
    print_box_header("VPK Shard Benchmark");

    let eves: Vec<PeerId> = args
        .eve_peer_ids
        .split(',')
        .map(|s| s.trim().parse::<PeerId>())
        .collect::<std::result::Result<_, _>>()
        .context("invalid --eve-peer-ids (expected comma-separated base58 PeerIds)")?;

    if eves.is_empty() {
        anyhow::bail!("--eve-peer-ids must specify at least one Eve node");
    }

    let n_max = args.vectors;
    let dim = args.dimensions;
    let n_queries = args.queries;
    let top_k = args.top_k;
    let batch = args.batch_size;
    let qdrant_url = args.qdrant_url.clone();
    let qdrant_api_key = args.qdrant_api_key.clone();
    let k = eves.len();

    println!("  Max vectors:   {:>8}", n_max);
    println!("  Dimensions:    {:>8}", dim);
    println!("  Queries/scale: {:>8}", n_queries);
    println!("  Top-K:         {:>8}", top_k);
    println!("  Eve shards:    {:>8}", k);
    println!("  Upload batch:  {:>8}", batch);
    println!();

    // ── Connect — one client per Eve ─────────────────────────────────────────

    let daemon_addr = crate::shard_cmd::daemon_socket();
    let my_peer_id = crate::identity::NodeIdentity::load_or_create()?.peer_id.to_base58();

    println!("  Connecting to {} Eve node(s)...", k);
    let mut shards: Vec<EveShard> = Vec::with_capacity(k);
    for (i, eve) in eves.iter().enumerate() {
        let client = P2PClient::connect(&daemon_addr)
            .await
            .context("connect to p2pd — is 'kwaainet start --daemon' running?")?;
        shards.push(EveShard {
            peer_id: *eve,
            tenant_id: Uuid::nil(),
            client: Arc::new(Mutex::new(client)),
        });
        println!("    shard-{}: {}", i, short_id(eve));
    }
    println!();

    // ── Phase 0: P2P RTT measurement + storage probe ─────────────────────────

    println!("  [0/5] Measuring P2P round-trip overhead (20 health pings per Eve)...");
    let mut all_rtt_samples: Vec<Duration> = Vec::new();
    let mut healthy_shards: Vec<EveShard> = Vec::new();
    for shard in shards {
        let client_guard = shard.client.lock().await;
        let probe = rpc_health(&*client_guard, &shard.peer_id).await;
        drop(client_guard);
        if let Err(e) = probe {
            println!(
                "    {} SKIP — storage health probe failed: {:#}",
                short_id(&shard.peer_id), e
            );
            continue;
        }
        let mut samples: Vec<Duration> = Vec::with_capacity(20);
        let client_guard = shard.client.lock().await;
        for _ in 0..19 {
            let t0 = Instant::now();
            let _ = rpc_health(&*client_guard, &shard.peer_id).await;
            samples.push(t0.elapsed());
        }
        drop(client_guard);
        let p = percentiles(&mut samples);
        println!(
            "    {} RTT  p50={} µs  p95={} µs  min={} µs",
            short_id(&shard.peer_id), p.p50, p.p95, p.min
        );
        all_rtt_samples.extend(samples);
        healthy_shards.push(shard);
    }
    if healthy_shards.is_empty() {
        anyhow::bail!("no Eve nodes responded to storage health probe — check 'kwaainet storage serve' is running on Eve nodes");
    }
    let mut shards = healthy_shards;
    let k = shards.len();
    if k < eves.len() {
        println!("  ⚠  {}/{} Eve node(s) are storage-capable — skipped {} unresponsive", k, eves.len(), eves.len() - k);
    }
    let rtt_stats = percentiles(&mut all_rtt_samples);
    let rtt_overhead_us = rtt_stats.p50;
    println!("    {}/{} nodes responding  P2P overhead p50: {} µs", k, eves.len(), rtt_overhead_us);
    println!();

    // ── Phase 1: Generate corpus ──────────────────────────────────────────────

    println!("  [1/5] Generating {} vectors (dim={})...", n_max, dim);
    let corpus: Vec<(i64, Vec<f32>)> = gen_corpus(0xdeadbeef_cafe_u64, n_max, dim);
    let query_vecs: Vec<Vec<f32>> = gen_query_vecs(0xcafe_f00d_dead_u64, n_queries, dim);
    println!("         Done.");
    println!();

    // ── Phase 2-4: Sweep over 3 vector scales ────────────────────────────────

    let scales = [n_max / 4, n_max / 2, n_max];
    let mut scale_results: Vec<ScaleResult> = Vec::new();

    for (si, &n) in scales.iter().enumerate() {
        let n = n.max(1000); // floor
        println!(
            "  [{}/5] Scale {}/{}: {} vectors ({} per shard)",
            si + 2,
            si + 1,
            scales.len(),
            n,
            (n + k - 1) / k
        );

        // ── Local setup ──────────────────────────────────────────────────────
        let local_dir =
            std::env::temp_dir().join(format!("kwaainet-bench-{}-{}", std::process::id(), si));
        let local_db = StorageDb::open(&local_dir).context("open local temp StorageDb")?;
        let local_tm = TenantManager::new(local_db.clone());
        let local_vs = VectorStore::new(local_db.clone());
        let local_info = local_tm
            .create(&my_peer_id, 8192, Some("bench-local"), dim)
            .await?;
        let local_tid = local_info.tenant_id;

        let t0 = Instant::now();
        for chunk in corpus[..n].chunks(batch) {
            local_vs.upload(local_tid, chunk).await?;
        }
        let local_upload_ms = t0.elapsed().as_millis() as u64;

        // ── Remote setup ─────────────────────────────────────────────────────
        let shard_size = (n + k - 1) / k;
        for (i, shard) in shards.iter_mut().enumerate() {
            let payload = CreateTenantPayload {
                peer_id: my_peer_id.clone(),
                capacity_limit_mb: 2048,
                display_name: Some(format!("bench-s{}-{}", si, i)),
                vector_dimension: dim,
            };
            let client = shard.client.lock().await;
            let info = rpc_create_tenant(&*client, &shard.peer_id, payload)
                .await
                .with_context(|| format!("create tenant on {}", short_id(&shard.peer_id)))?;
            shard.tenant_id = info.tenant_id;
        }

        let t0 = Instant::now();
        for (i, shard) in shards.iter().enumerate() {
            let start = i * shard_size;
            let end = ((i + 1) * shard_size).min(n);
            let client = shard.client.lock().await;
            for chunk in corpus[start..end].chunks(batch) {
                rpc_upload_vectors(&*client, &shard.peer_id, shard.tenant_id, chunk.to_vec())
                    .await?;
            }
        }
        let shard_upload_ms = t0.elapsed().as_millis() as u64;

        // ── Warmup ───────────────────────────────────────────────────────────
        let warmup_vecs = gen_query_vecs(0xbeef0000_u64.wrapping_add(si as u64), 5, dim);
        for q in &warmup_vecs {
            local_vs.search(local_tid, q, top_k).await?;
            search_sharded(&shards, q, top_k).await?;
        }

        // ── Benchmark ────────────────────────────────────────────────────────
        let mut local_lat: Vec<Duration> = Vec::with_capacity(n_queries);
        let mut shard_lat: Vec<Duration> = Vec::with_capacity(n_queries);
        let mut recall_sum: f64 = 0.0;

        for q in &query_vecs {
            let t0 = Instant::now();
            let local_res = local_vs.search(local_tid, q, top_k).await?;
            local_lat.push(t0.elapsed());

            let t0 = Instant::now();
            let shard_res = search_sharded(&shards, q, top_k).await?;
            shard_lat.push(t0.elapsed());

            let local_ids: std::collections::HashSet<i64> =
                local_res.iter().map(|r| r.id).collect();
            let hits = shard_res.iter().filter(|r| local_ids.contains(&r.id)).count();
            recall_sum += hits as f64 / top_k as f64;
        }

        // ── Qdrant benchmark ─────────────────────────────────────────────────
        let qdrant_result = bench_qdrant(
            &qdrant_url,
            qdrant_api_key.as_deref(),
            &corpus[..n],
            &query_vecs,
            top_k,
            batch,
            dim,
        )
        .await
        .unwrap_or_else(|e| {
            println!("    ⚠ Qdrant error: {e}");
            None
        });

        if let Some(ref qr) = qdrant_result {
            println!("         qdrant p50={} µs", qr.search.p50);
        }

        scale_results.push(ScaleResult {
            n,
            local_upload_ms,
            shard_upload_ms,
            local: percentiles(&mut local_lat),
            sharded: percentiles(&mut shard_lat),
            recall_pct: recall_sum / n_queries as f64 * 100.0,
            qdrant: qdrant_result,
        });

        println!(
            "         local p50={} µs  sharded p50={} µs  recall={:.0}%",
            scale_results.last().unwrap().local.p50,
            scale_results.last().unwrap().sharded.p50,
            scale_results.last().unwrap().recall_pct,
        );

        // ── Cleanup ──────────────────────────────────────────────────────────
        for shard in &shards {
            let client = shard.client.lock().await;
            let _ = rpc_delete_tenant(&*client, &shard.peer_id, shard.tenant_id).await;
        }
        let _ = std::fs::remove_dir_all(&local_dir);
        println!();
    }

    // ── Phase 5: Results ──────────────────────────────────────────────────────

    println!("  [5/5] Results");
    println!();

    // Search latency table
    let has_qdrant = scale_results.iter().any(|r| r.qdrant.is_some());
    println!("  Search latency (µs)");
    if has_qdrant {
        println!(
            "  {:>10}  {:>10}  {:>10}  {:>11}  {:>8}  {:>8}  {:>8}  {:>9}",
            "Vectors", "Local p50", "Shard p50", "Qdrant p50", "Speedup", "L p95", "S p95", "Recall"
        );
        println!("  {}", "─".repeat(92));
        for r in &scale_results {
            let speedup = r.local.p50 as f64 / r.sharded.p50.max(1) as f64;
            let winner = if r.sharded.p50 < r.local.p50 { "✅" } else { "❌" };
            let qdrant_col = r.qdrant.as_ref()
                .map(|q| format!("{:>11}", q.search.p50))
                .unwrap_or_else(|| format!("{:>11}", "n/a"));
            println!(
                "  {:>10}  {:>10}  {:>10}  {}  {:>7.2}×{}  {:>8}  {:>8}  {:>8.1}%",
                r.n, r.local.p50, r.sharded.p50, qdrant_col, speedup, winner,
                r.local.p95, r.sharded.p95, r.recall_pct
            );
        }
    } else {
        println!(
            "  {:>10}  {:>10}  {:>10}  {:>8}  {:>8}  {:>8}  {:>9}",
            "Vectors", "Local p50", "Shard p50", "Speedup", "L p95", "S p95", "Recall"
        );
        println!("  {}", "─".repeat(78));
        for r in &scale_results {
            let speedup = r.local.p50 as f64 / r.sharded.p50.max(1) as f64;
            let winner = if r.sharded.p50 < r.local.p50 { "✅" } else { "❌" };
            println!(
                "  {:>10}  {:>10}  {:>10}  {:>7.2}×{}  {:>8}  {:>8}  {:>8.1}%",
                r.n, r.local.p50, r.sharded.p50, speedup, winner,
                r.local.p95, r.sharded.p95, r.recall_pct
            );
        }
    }
    println!();

    // Crossover projection
    println!("  P2P overhead breakdown (p50)");
    println!("  ──────────────────────────────────────────────────────────────");
    println!("  Pure P2P ping (protocol overhead):  {:>8} µs", rtt_overhead_us);
    for r in &scale_results {
        let protocol_plus_hnsw = r.sharded.p50;
        let hnsw_only = r.local.p50 / k as u64;
        let wire_overhead = protocol_plus_hnsw.saturating_sub(hnsw_only);
        println!(
            "  At {:>6}v  remote_p50={:>6}µs  local/shard={:>6}µs  wire={:>6}µs",
            r.n, r.sharded.p50, hnsw_only, wire_overhead
        );
    }
    println!();

    // Crossover projection: find N where local_p50(N) = sharded_p50(N/K)
    // We model: local_p50(N) ≈ a * N^b  (fit from two data points)
    // Sharding wins when local_p50(N) > P2P_overhead + local_p50(N/K)
    // i.e. local_p50(N) - local_p50(N/K) > P2P_overhead
    if scale_results.len() >= 2 {
        let r1 = &scale_results[scale_results.len() - 2];
        let r2 = &scale_results[scale_results.len() - 1];
        // Power law fit: p50 = a * N^b
        let (n1, t1) = (r1.n as f64, r1.local.p50 as f64);
        let (n2, t2) = (r2.n as f64, r2.local.p50 as f64);
        if n1 > 0.0 && n2 > n1 && t1 > 0.0 && t2 > 0.0 {
            let b = (t2 / t1).ln() / (n2 / n1).ln();
            let a = t1 / n1.powf(b);

            println!("  Crossover projection (local p50 model: {:.2e} × N^{:.3})", a, b);
            println!("  P2P overhead to overcome: {} µs", rtt_overhead_us);
            println!();

            // Find N where a*N^b - a*(N/K)^b = rtt_overhead
            // a * N^b * (1 - 1/K^b) = rtt_overhead
            let factor = 1.0 - 1.0 / (k as f64).powf(b);
            if factor > 1e-6 {
                let crossover_n = (rtt_overhead_us as f64 / (a * factor)).powf(1.0 / b);
                println!(
                    "  Sharding ({} Eves) breaks even at approximately {:>8.0} vectors",
                    k, crossover_n
                );
                if crossover_n > 1e9 {
                    println!("  (breakeven beyond 1B vectors — WAN sharding is unlikely to win on latency)");
                } else if crossover_n > 1e6 {
                    println!("  (breakeven at ~{:.0}M vectors)", crossover_n / 1e6);
                } else {
                    println!("  (breakeven at ~{:.0}K vectors)", crossover_n / 1e3);
                }
            }
            println!();
        }
    }

    // Upload time comparison
    println!("  Upload time (ms)");
    if has_qdrant {
        println!(
            "  {:>10}  {:>12}  {:>12}  {:>12}",
            "Vectors", "Local", "Sharded", "Qdrant"
        );
        println!("  {}", "─".repeat(52));
        for r in &scale_results {
            let q_col = r.qdrant.as_ref()
                .map(|q| format!("{:>12}", q.upload_ms))
                .unwrap_or_else(|| format!("{:>12}", "n/a"));
            println!(
                "  {:>10}  {:>12}  {:>12}  {}",
                r.n, r.local_upload_ms, r.shard_upload_ms, q_col
            );
        }
    } else {
        println!(
            "  {:>10}  {:>12}  {:>12}",
            "Vectors", "Local (ms)", "Sharded (ms)"
        );
        println!("  {}", "─".repeat(40));
        for r in &scale_results {
            println!("  {:>10}  {:>12}  {:>12}", r.n, r.local_upload_ms, r.shard_upload_ms);
        }
    }
    println!();

    // Recall analysis
    println!("  Recall note:");
    println!("  HNSW (m=16, ef=64) on random 384-dim unit vectors has low intrinsic");
    println!("  recall because score gaps are tiny (cosine similarity ≈ 0 for random");
    println!("  vectors). Production embeddings have more structure → higher recall.");
    for r in &scale_results {
        println!(
            "  At {:>6}v, {} shards: recall@{} = {:.1}%",
            r.n, k, top_k, r.recall_pct
        );
    }
    println!();

    print_separator();
    Ok(())
}

// ── Fan-out search ────────────────────────────────────────────────────────────

async fn search_sharded(
    shards: &[EveShard],
    query: &[f32],
    top_k: usize,
) -> Result<Vec<kwaai_storage::SearchResult>> {
    let futs: Vec<_> = shards
        .iter()
        .map(|s| {
            let client = Arc::clone(&s.client);
            let peer_id = s.peer_id;
            let tid = s.tenant_id;
            let q = query.to_vec();
            async move {
                let c = client.lock().await;
                rpc_search_vectors(&*c, &peer_id, tid, q, top_k).await
            }
        })
        .collect();

    let results: Vec<Result<Vec<kwaai_storage::SearchResult>>> = join_all(futs).await;

    let mut merged: Vec<kwaai_storage::SearchResult> = Vec::new();
    for r in results {
        merged.extend(r?);
    }
    merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    merged.truncate(top_k);
    Ok(merged)
}

// ── Qdrant benchmark ─────────────────────────────────────────────────────────

/// Benchmark a Qdrant instance at `url`. Returns `None` if unreachable or
/// Qdrant is not running — bench proceeds without it.
async fn bench_qdrant(
    url: &str,
    api_key: Option<&str>,
    corpus: &[(i64, Vec<f32>)],
    queries: &[Vec<f32>],
    top_k: usize,
    batch: usize,
    dim: usize,
) -> Result<Option<QdrantStats>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let auth = |req: reqwest::RequestBuilder| -> reqwest::RequestBuilder {
        if let Some(key) = api_key {
            req.header("api-key", key)
        } else {
            req
        }
    };

    // Probe liveness — skip silently if Qdrant not running.
    let probe = auth(client.get(format!("{url}/healthz"))).send().await;
    if probe.is_err() || !probe.unwrap().status().is_success() {
        println!("    (Qdrant not reachable at {url} — skipping Qdrant comparison)");
        return Ok(None);
    }

    const COLL: &str = "kwaainet-bench";

    // Clean up any leftover collection from a prior run.
    let _ = auth(client.delete(format!("{url}/collections/{COLL}"))).send().await;

    // Create collection with cosine HNSW.
    let resp = auth(
        client
            .put(format!("{url}/collections/{COLL}"))
            .json(&serde_json::json!({"vectors": {"size": dim, "distance": "Cosine"}})),
    )
    .send()
    .await?;
    if !resp.status().is_success() {
        anyhow::bail!("Qdrant create collection: HTTP {}", resp.status());
    }

    // Upload vectors.
    let t0 = Instant::now();
    for chunk in corpus.chunks(batch) {
        let points: Vec<serde_json::Value> = chunk
            .iter()
            .map(|(id, v)| serde_json::json!({"id": *id as u64, "vector": v}))
            .collect();
        let resp = auth(
            client
                .put(format!("{url}/collections/{COLL}/points?wait=true"))
                .json(&serde_json::json!({"points": points})),
        )
        .send()
        .await?;
        if !resp.status().is_success() {
            anyhow::bail!("Qdrant upload: HTTP {}", resp.status());
        }
    }
    let upload_ms = t0.elapsed().as_millis() as u64;

    // Search benchmark.
    let mut latencies: Vec<Duration> = Vec::with_capacity(queries.len());
    for q in queries {
        let body = serde_json::json!({
            "vector": q,
            "limit": top_k,
            "with_vector": false,
            "with_payload": false,
        });
        let t0 = Instant::now();
        let resp = auth(
            client
                .post(format!("{url}/collections/{COLL}/points/search"))
                .json(&body),
        )
        .send()
        .await?;
        latencies.push(t0.elapsed());
        if !resp.status().is_success() {
            anyhow::bail!("Qdrant search: HTTP {}", resp.status());
        }
    }

    // Cleanup.
    let _ = auth(client.delete(format!("{url}/collections/{COLL}"))).send().await;

    Ok(Some(QdrantStats {
        upload_ms,
        search: percentiles(&mut latencies),
    }))
}

// ── Vector generation ────────────────────────────────────────────────────────

fn gen_corpus(seed: u64, n: usize, dim: usize) -> Vec<(i64, Vec<f32>)> {
    let mut state = seed;
    (0..n)
        .map(|i| (i as i64, gen_unit_vec(&mut state, dim)))
        .collect()
}

fn gen_query_vecs(seed: u64, n: usize, dim: usize) -> Vec<Vec<f32>> {
    let mut state = seed;
    (0..n).map(|_| gen_unit_vec(&mut state, dim)).collect()
}

fn gen_unit_vec(state: &mut u64, dim: usize) -> Vec<f32> {
    let mut v: Vec<f32> = (0..dim).map(|_| xorshift_f32(state)).collect();
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-8 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
    v
}

fn xorshift_f32(state: &mut u64) -> f32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state as f32) / (u64::MAX as f32) * 2.0 - 1.0
}

// ── Statistics ────────────────────────────────────────────────────────────────

#[allow(dead_code)]
struct Stats {
    min: u64,
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
}

fn percentiles(samples: &mut Vec<Duration>) -> Stats {
    samples.sort_unstable();
    let n = samples.len();
    let us = |d: Duration| d.as_micros() as u64;
    Stats {
        min: us(samples[0]),
        p50: us(samples[n * 50 / 100]),
        p95: us(samples[n * 95 / 100]),
        p99: us(samples[(n * 99 / 100).min(n - 1)]),
        max: us(samples[n - 1]),
    }
}

// ── Types ─────────────────────────────────────────────────────────────────────

struct EveShard {
    peer_id: PeerId,
    tenant_id: Uuid,
    client: Arc<Mutex<P2PClient>>,
}

struct ScaleResult {
    n: usize,
    local_upload_ms: u64,
    shard_upload_ms: u64,
    local: Stats,
    sharded: Stats,
    recall_pct: f64,
    qdrant: Option<QdrantStats>,
}

struct QdrantStats {
    upload_ms: u64,
    search: Stats,
}

fn short_id(peer: &PeerId) -> String {
    let s = peer.to_base58();
    format!("{}…{}", &s[..8], &s[s.len() - 6..])
}
