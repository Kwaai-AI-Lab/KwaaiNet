# RAG Application Stack — Implementation Plan

## Context

KwaaiNet has a working storage layer (hnsw_rs+redb via `kwaainet storage`, port 7432) and an OpenAI-compatible inference layer (`kwaainet shard api`, port 8080). The next step is to build a full advanced RAG harness on top: document ingestion, hybrid retrieval, query transformation, reranking, agentic workflows, and adaptive strategy selection — across four phases.

**Critical constraint**: Eve stores only opaque `(i64, Vec<f32>)` tuples. All chunk text and metadata live locally on Bob at `~/.kwaainet/rag/`. Embedding dimension must be 768 (nomic-embed-text); tenants must be created with `vector_dimension: 768`.

**UI decision**: Build `kwaainet rag serve` as an OpenAI-compatible endpoint WITH RAG baked in (port 9090). OpenWebUI points at it as a custom OpenAI base URL. Also serve a minimal embedded HTML/JS UI at `/` for users without Docker.

---

## Confirmed Existing APIs (verified from codebase)

**P2P connect** — `crates/kwaai-cli/src/vpk.rs:419` (currently private, make `pub(crate)`):
```rust
async fn p2p_connect() -> Result<(P2PClient, PeerId)>
```

**Storage RPCs** — `crates/kwaai-cli/src/storage_rpc.rs`:
```rust
rpc_create_tenant(client: &P2PClient, peer_id: &PeerId, input: CreateTenantPayload) -> Result<TenantInfo>
rpc_upload_vectors(client: &P2PClient, peer_id: &PeerId, tenant_id: Uuid, vectors: Vec<(i64, Vec<f32>)>) -> Result<usize>
rpc_search_vectors(client: &P2PClient, peer_id: &PeerId, tenant_id: Uuid, query: Vec<f32>, top_k: usize) -> Result<Vec<SearchResult>>
rpc_delete_vectors(client: &P2PClient, peer_id: &PeerId, tenant_id: Uuid, ids: Vec<i64>) -> Result<usize>
rpc_list_tenants(client: &P2PClient, peer_id: &PeerId) -> Result<Vec<TenantInfo>>
```
`CreateTenantPayload` already has `vector_dimension: usize` field.

**redb table pattern** — `crates/kwaai-storage/src/db.rs`:
```rust
const MY_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("name");
// Keys are composite byte slices; values are serde_json bytes
// Composite key: tenant_id(16 bytes) ++ chunk_id(8 bytes) = 24 bytes
```

**SSE streaming** — `crates/kwaai-cli/src/shard_api.rs:500-540`:
```rust
stream::unfold(ctx, |mut ctx| async move {
    ctx.rx.recv().await.map(|piece| { /* wrap in Event */ })
}).chain(stream::once(async { Ok(Event::default().data("[DONE]")) }))
// Sse::new(...).into_response()
```

**Needed codebase changes before starting kwaai-rag:**
1. `crates/kwaai-storage/src/tenant.rs` — add `vector_dimension: u32` to `TenantInfo` (it's in `TenantRecord` already but not exposed). Required for dimension guard.
2. `crates/kwaai-cli/src/vpk.rs:419` — change `async fn p2p_connect` → `pub(crate) async fn p2p_connect`
3. `core/Cargo.toml` `[workspace.dependencies]` — add `sha2 = "0.10"` (redb is already present in kwaai-storage but not workspace.dependencies — promote it)

---

## New Crate: `crates/kwaai-rag`

Pure library crate across all phases. CLI crate owns all binary surface.

### Phase 1 Core Files

**`src/chunker.rs`**
```rust
pub struct ChunkConfig { pub chunk_size: usize, pub chunk_overlap: usize }  // 800 / 200 chars
pub struct Chunk { pub id: i64, pub text: String, pub doc_name: String,
                   pub chunk_index: u32, pub page_num: Option<u32> }

pub fn split_text(text: &str, doc_name: &str, cfg: &ChunkConfig) -> Vec<Chunk>
// Sliding window over Unicode scalar values (not bytes)
// Sentence-window variant: each chunk stores a wider "surrounding" field for context injection

pub fn chunk_id(doc_name: &str, chunk_index: u32) -> i64
// truncate(sha256(doc_name + "::" + chunk_index), 8 bytes) → i64
// Stable across re-ingest of same file at same position
```

**`src/embedder.rs`**
```rust
#[derive(Clone)]
pub struct EmbedClient { http: reqwest::Client, base_url: String, model: String }
// base_url default "http://localhost:11434"; OLLAMA_BASE_URL env overrides
// model default "nomic-embed-text" → 768-dim

impl EmbedClient {
    pub async fn embed_one(&self, text: &str) -> anyhow::Result<Vec<f32>>
    pub async fn embed_batch(&self, texts: &[&str]) -> anyhow::Result<Vec<Vec<f32>>>
    // POST /api/embed  {"model":..,"input":..}  → {"embeddings":[[f32,..]]}
}
// At rag serve startup: test-embed "" and assert len == 768. Clear error if Ollama not running.
```

**`src/meta_store.rs`** — redb sidecar at `~/.kwaainet/rag/<tenant_id>.redb`
```rust
// Table "chunks": tenant_uuid[16] + chunk_id_i64_le[8] = 24 bytes → ChunkMeta JSON
// Table "docs":   tenant_uuid[16] + doc_name_bytes → [chunk_id_i64] JSON
// Table "bm25":   tenant_uuid[16] + term_bytes → {df: u32, postings: [(chunk_id, tf: f32)]} JSON
// Table "graph":  tenant_uuid[16] + node_id_i64_le[8] → KgNode JSON  (Phase 4)

pub struct ChunkMeta { pub doc_name: String, pub chunk_index: u32, pub text: String,
                       pub surrounding: String,  // sentence-window wider context
                       pub page_num: Option<u32>, pub ingested_at: String }
pub struct MetaStore { db: redb::Database }
impl MetaStore {
    pub fn open(data_dir: &Path, tenant_id: Uuid) -> anyhow::Result<Self>
    pub fn put_chunks(&self, tenant_id: Uuid, chunks: &[ChunkMeta], ids: &[i64]) -> anyhow::Result<()>
    pub fn get_chunks(&self, tenant_id: Uuid, ids: &[i64]) -> anyhow::Result<Vec<Option<ChunkMeta>>>
    pub fn all_chunks(&self, tenant_id: Uuid) -> anyhow::Result<Vec<(i64, ChunkMeta)>>  // for BM25 build
    pub fn list_docs(&self, tenant_id: Uuid) -> anyhow::Result<Vec<String>>
    pub fn delete_doc(&self, tenant_id: Uuid, doc_name: &str) -> anyhow::Result<Vec<i64>>
    // Phase 2+:
    pub fn put_bm25_index(&self, tenant_id: Uuid, index: &BM25Index) -> anyhow::Result<()>
    pub fn get_bm25_index(&self, tenant_id: Uuid) -> anyhow::Result<Option<BM25Index>>
}
```

**`src/ingestion.rs`**
```rust
pub struct IngestConfig {
    pub tenant_id: Uuid, pub embed: EmbedClient, pub meta: MetaStore,
    pub chunk_cfg: ChunkConfig, pub upload_batch_size: usize,  // default 256
    pub build_bm25: bool,  // Phase 2: rebuild BM25 index after ingest
    pub extract_entities: bool,  // Phase 4: LLM entity extraction for graph
}

pub async fn ingest_text(
    cfg: &IngestConfig, doc_name: &str, text: &str,
    upload_fn: impl Fn(Vec<(i64, Vec<f32>)>) -> Pin<Box<dyn Future<Output=anyhow::Result<usize>> + Send>>,
    progress: Option<&ProgressFn>,
) -> anyhow::Result<IngestionResult>
// upload_fn closes over P2PClient + Eve PeerId — keeps kwaai-rag free of kwaai-p2p-daemon
```

**`src/retriever.rs`**
```rust
pub struct RetrievedChunk {
    pub chunk_meta: ChunkMeta, pub score: f64,
    pub source_kb: Option<String>,  // Phase 3: which federated KB
    pub rerank_score: Option<f64>,  // Phase 2: cross-encoder score
}

pub struct RetrieveConfig {
    pub top_k: usize, pub min_score: f64,
    pub use_sentence_window: bool,   // inject surrounding context in prompt
    pub retrieval_mode: RetrievalMode,  // Dense | BM25 | Hybrid(alpha: f32)
}

pub enum RetrievalMode { Dense, BM25, Hybrid { alpha: f32 } }  // alpha: dense weight (0.0–1.0)

pub async fn retrieve(
    query: &str, cfg: &RetrieveConfig, embed: &EmbedClient, meta: &MetaStore,
    search_fn: impl Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output=anyhow::Result<Vec<(i64, f64)>>> + Send>>,
) -> anyhow::Result<Vec<RetrievedChunk>>
// Hybrid path: dense via search_fn + BM25 from meta_store, merged via RRF
```

**`src/prompt.rs`**
```rust
pub struct ChatMessage { pub role: String, pub content: String }

pub fn build_rag_prompt(user_query: &str, chunks: &[RetrievedChunk], max_context_chars: usize) -> String
// Template: numbered context blocks; uses chunk_meta.surrounding if use_sentence_window

pub fn build_chat_messages(user_query: &str, chunks: &[RetrievedChunk],
                           history: &[ChatMessage], max_context_chars: usize) -> Vec<ChatMessage>
```

---

## Phase 2: Advanced Retrieval Quality

New files added to `crates/kwaai-rag/src/`:

**`bm25.rs`** — in-process BM25 over MetaStore text
```rust
pub struct BM25Index { pub doc_count: u32, pub avg_dl: f32,
                       pub term_data: HashMap<String, TermEntry> }
pub struct TermEntry { pub df: u32, pub postings: Vec<(i64, f32)> }  // chunk_id, tf

impl BM25Index {
    pub fn build(chunks: &[(i64, &str)]) -> Self           // tokenize, compute IDF
    pub fn search(&self, query: &str, top_k: usize) -> Vec<(i64, f64)>  // BM25 scores
}
// Persisted as JSON in meta_store "bm25" table; rebuilt on ingest
// Tokenization: lowercase, split on non-alphanumeric, stop-word removal (hardcoded list)
```

**`query_transform.rs`** — query rewriting before retrieval
```rust
pub struct QueryTransformer { embed: EmbedClient, llm_url: String }

impl QueryTransformer {
    // HyDE: generate a hypothetical answer, embed it, use that embedding for retrieval
    pub async fn hyde(&self, query: &str) -> anyhow::Result<Vec<f32>>
    // query → "answer" via LLM → embed(answer)

    // Sub-query decomposition: break complex query into 2-4 sub-queries
    pub async fn decompose(&self, query: &str) -> anyhow::Result<Vec<String>>
    // query → LLM → JSON array of sub-questions; retrieve each, merge+dedup results

    // Step-back prompting: reformulate to higher-level concept for better coverage
    pub async fn step_back(&self, query: &str) -> anyhow::Result<String>
}
```

**`reranker.rs`** — second-pass scoring of retrieved chunks
```rust
pub struct Reranker { llm_url: String, model: String }

impl Reranker {
    // Cross-encoder: prompt LLM to rate (query, chunk) relevance 0.0–1.0
    pub async fn rerank(
        &self, query: &str, chunks: Vec<RetrievedChunk>, top_n: usize,
    ) -> anyhow::Result<Vec<RetrievedChunk>>
    // Parallel batch of up to 8 (query, chunk) pairs → score each → sort → take top_n
    // Sets RetrievedChunk.rerank_score; falls back to original score on LLM error
}
```

**`eval.rs`** — RAG quality metrics
```rust
pub struct RagEval { pub faithfulness: f64, pub context_relevance: f64,
                     pub answer_relevance: f64 }

pub async fn evaluate(
    query: &str, context: &[RetrievedChunk], answer: &str, llm_url: &str,
) -> anyhow::Result<RagEval>
// LLM-as-judge: 3 structured prompts → scores 0.0–1.0
// Used by `kwaainet rag eval` command and logged to ~/.kwaainet/rag/eval.jsonl
```

**`retriever.rs` additions for Phase 2:**
- Hybrid merge via Reciprocal Rank Fusion (RRF) with configurable `alpha`
- After initial retrieval: optionally call `Reranker::rerank()` if `rerank: true` in config
- Sentence-window: replace `chunk_meta.text` with `surrounding` in prompt context, keep original for scoring

**CLI additions for Phase 2:**
- `rag ingest --build-bm25` flag (default true) — build/update BM25 index post-ingest
- `rag query --mode hybrid|dense|bm25 --rerank` flags
- `rag serve --query-transform hyde|decompose|none` flag
- `rag eval <question> <expected-answer>` — run eval metrics, print scores

---

## Phase 3: Federated Multi-KB RAG + Adaptive Strategy

**New**: `crates/kwaai-rag/src/federation.rs`

- Advertise KB on DHT under `_kwaai.vpk.kb.<kb_id>` (reuse `vpk_dht_id()` SHA1-msgpack hashing)
- `resolve_kb(client, kb_id) -> Vec<KbEndpoint>` — find remote Bob+Eve pairs
- New P2P protocol `/kwaai/rag/1.0.0` — Bob-to-Bob query relay:
  - Request: `{ query_embedding: Vec<f32>, top_k: usize, tenant_id: Uuid, vc_token: Option<String> }`
  - Response: `{ results: Vec<(i64, f64)> }` — only IDs+scores, no text (preserving Bob's privacy)
- Entitlement: optional kwaai-trust VC in `vc_token`; serving Bob validates before answering
- Fan-out: `retrieve()` across multiple `KbEndpoint`s via `tokio::join_all`, merge-sort by score

**New**: `crates/kwaai-rag/src/adaptive.rs`

```rust
pub enum RagStrategy { Naive, BM25Only, HybridDense, HyDE, Decomposed, GraphExpanded }

pub struct AdaptiveRouter { llm_url: String }

impl AdaptiveRouter {
    // Classify query complexity, select strategy
    // Simple factoid → Naive; multi-hop → Decomposed; ambiguous → HyDE; entity-heavy → GraphExpanded
    pub async fn route(&self, query: &str, kb_stats: &KbStats) -> anyhow::Result<RagStrategy>
}
// RagStrategy drives which combination of query_transform + retrieval_mode + rerank to use
```

**New**: `crates/kwaai-rag/src/crag.rs` — Corrective RAG

```rust
pub struct CragConfig { pub min_confidence: f64, pub web_fallback: bool }

pub async fn corrective_retrieve(
    query: &str, chunks: Vec<RetrievedChunk>, cfg: &CragConfig,
    embed: &EmbedClient, llm_url: &str,
) -> anyhow::Result<Vec<RetrievedChunk>>
// 1. Score confidence of top-K chunks (reranker or LLM-judge)
// 2. If max_score < min_confidence (default 0.5): refine query + re-retrieve (1 iteration)
// 3. If still low AND web_fallback=true: inject a "knowledge gap" note into context
// 4. Filter out chunks below min_confidence threshold before prompting
```

**CLI additions for Phase 3:**
- `rag kb-share --name <NAME>` — publish KB to DHT
- `rag kb-list` — show local + discoverable KBs
- `rag serve --kb-ids <id1,id2>` — fan-out to federated KBs
- `rag serve --strategy auto|naive|hybrid|hyde|decomposed` — override adaptive router

---

## Phase 4: GraphRAG + Agentic + RLM

**`src/graph.rs`** — knowledge graph over ingested corpus

```rust
pub struct KgNode { pub id: i64, pub entity: String, pub entity_type: String,
                    pub chunk_ids: Vec<i64> }
pub struct KgEdge { pub src: i64, pub dst: i64, pub relation: String, pub weight: f32 }
pub struct KnowledgeGraph { nodes: HashMap<i64, KgNode>, edges: Vec<KgEdge> }

impl KnowledgeGraph {
    // During ingestion: LLM structured-output extraction (calls shard_api — no new services)
    pub async fn extract_entities(chunk: &str, llm_url: &str) -> anyhow::Result<Vec<KgNode>>

    // Graph-augmented retrieval: dense top-K → expand 1-2 hops → add neighbor chunks to context
    pub fn expand_neighborhood(&self, seed_chunk_ids: &[i64], hops: u8) -> Vec<i64>
}
// Persisted in meta_store "graph" redb table
```

**`src/agent.rs`** — agentic RAG tool loop

```rust
pub struct AgentConfig {
    pub max_steps: u8,       // default 5
    pub tools: Vec<AgentTool>,  // Retrieve, Calculate, SelfAsk
    pub llm_url: String,
}

pub enum AgentTool { Retrieve, Calculate, SelfAsk }

pub async fn run_agent(
    query: &str, cfg: &AgentConfig, retriever: &dyn RetrieveFn,
) -> anyhow::Result<AgentResult>
// ReAct loop: LLM → Thought → Action(tool, input) → Observation → repeat
// Terminates on "Final Answer:" or max_steps
// Tool dispatch is local (no external APIs in Phase 4; WebSearch deferred)
```

**`src/rlm.rs`** — Recursive Memory Harness

```rust
pub struct MemoryConfig {
    pub working_memory_limit: usize,   // max chars held across turns
    pub summary_threshold: usize,      // compress when working_memory > this
    pub persist_entities: bool,        // merge extracted entities into graph between turns
}

pub struct RecursiveMemory {
    pub working: Vec<ChatMessage>,   // recent turns
    pub summary: String,             // compressed long-term context
    pub entity_delta: Vec<KgNode>,   // entities learned this session
}

impl RecursiveMemory {
    // When working_memory > threshold: summarize oldest N messages via LLM → compress
    pub async fn maybe_compress(&mut self, llm_url: &str) -> anyhow::Result<()>

    // At session end: merge entity_delta into persistent KnowledgeGraph
    pub fn flush_to_graph(&self, graph: &mut KnowledgeGraph)
}
// rag serve state includes RecursiveMemory per session_id (60s TTL, same pattern as shard KV cache)
```

**`src/adaptive.rs` additions for Phase 4:**
- `GraphExpanded` strategy: runs graph expansion after dense retrieval
- `Agentic` strategy: routes to `run_agent()` for complex multi-hop queries

**CLI additions for Phase 4:**
- `rag ingest --extract-entities` — LLM entity extraction during ingest (slow; opt-in)
- `rag serve --enable-agent --max-steps 5` — agentic loop in serve mode
- `rag serve --memory-mode none|session|persistent` — RLM configuration

---

## Decentralized Parallelism Architecture

KwaaiNet's P2P fabric enables six distinct parallelism axes that naive centralized RAG cannot exploit. These are cross-cutting concerns that shape the retriever, federation, and ingestion designs.

### 1. Sub-Query Fan-Out
`query_transform::decompose()` produces N sub-queries. All N retrieve concurrently via `tokio::join_all`, results merged+deduped before reranking. Latency = slowest sub-query, not sum.

```rust
// retriever.rs: decomposed retrieve path
let sub_results = futures::future::join_all(
    sub_queries.iter().map(|q| retrieve_one(q, cfg, embed, search_fn))
).await;
// merge, dedup by chunk_id, re-sort by max score
```

### 2. Multi-KB Parallel Fan-Out (Phase 3)
Federated search fires one `/kwaai/rag/1.0.0` RPC per known KB endpoint simultaneously. Each remote Bob returns `(id, score)` tuples only — no text transmitted (privacy-preserving). Bob merges all score lists with RRF.

```rust
let kb_futures: Vec<_> = endpoints.iter().map(|ep| query_remote_kb(ep, &embedding, top_k)).collect();
let all_results = futures::future::join_all(kb_futures).await;
// collect non-error results, flatten, RRF merge, resolve top-K chunk IDs locally
```

**Privacy boundary**: Remote Bobs never send chunk text across the wire — only integer IDs and float scores.

### 3. BM25 + Dense Parallel (Phase 2)
Both retrieval paths run concurrently on every hybrid query:

```rust
let (dense_fut, bm25_fut) = (
    search_fn(embedding, top_k * 2),                        // Eve RPC (network)
    async { meta_store.bm25_search(q, top_k * 2) },        // local redb (CPU)
);
let (dense_res, bm25_res) = tokio::join!(dense_fut, bm25_fut);
// RRF merge
```
Dense and BM25 run on different resources (network I/O vs local CPU), so overlap is nearly free.

### 4. Corpus Sharding Across Eve Nodes
Large corpora can be partitioned across multiple (Bob partition, Eve node) pairs at ingest time. Retrieval fans out identically to multi-KB federation — sharding is transparent to the query path.

```
kwaainet rag init --eve-peer-id <EVE1> --shard-index 0 --shard-total 3
kwaainet rag init --eve-peer-id <EVE2> --shard-index 1 --shard-total 3
kwaainet rag init --eve-peer-id <EVE3> --shard-index 2 --shard-total 3
kwaainet rag ingest corpus/ --shard-assign hash  # routes each doc to shard by hash(doc_name)
```
`rag serve` fans out to all shards, merges, returns global top-K. Capacity scales linearly with Eve nodes.

### 5. Distributed Reranking (Phase 3+)
New P2P protocol `/kwaai/rag/rerank/1.0.0`: Bob sends `(query, chunk_id_list)` to the remote Bob that holds those chunks; remote Bob scores `(query, chunk_text)` pairs locally and returns `(chunk_id, score)`. Text never leaves the owning Bob.

### 6. Speculative Strategy Parallelism (Phase 3)
When `adaptive_router` is uncertain between two strategies, run both speculatively:

```rust
let (naive_fut, hyde_fut) = (retrieve_naive(q, ..), retrieve_hyde(q, ..));
let (naive_res, hyde_res) = tokio::join!(naive_fut, hyde_fut);
let best = if hyde_res.max_score() > naive_res.max_score() { hyde_res } else { naive_res };
```

### Implementation Notes
- All fan-out uses `futures::future::join_all` or `tokio::join!` — no additional thread pools
- Timeout per fan-out branch: 5s (same pattern as `HOP_TIMEOUT` in `block_rpc.rs`)
- Failed branches are logged and skipped; minimum 1 branch must succeed
- `redb` BM25 reads are synchronous — wrap in `tokio::task::spawn_blocking` when concurrent with async Eve RPC

---

## New CLI Modules in `kwaai-cli`

### `src/rag_cmd.rs` — all `kwaainet rag` subcommands

| Subcommand | Phase | What it does |
|---|---|---|
| `rag init --eve-peer-id <PEER>` | 1 | `rpc_create_tenant(..768..)`, MetaStore, write config |
| `rag ingest <file>` | 1 | txt/md/pdf → chunk → embed → upload; BM25 rebuild |
| `rag query <text> [--mode] [--rerank]` | 1/2 | retrieve + display chunks, no LLM |
| `rag chat` | 1 | REPL: retrieve → prompt → stream from shard API |
| `rag docs` | 1 | list ingested documents |
| `rag delete-doc <name>` | 1 | remove vectors + metadata |
| `rag serve [--port 9090] [--strategy auto]` | 1–4 | OpenAI-compatible RAG HTTP server |
| `rag eval <q> <expected>` | 2 | faithfulness/relevance metrics |
| `rag kb-share --name <N>` | 3 | publish KB to DHT |
| `rag kb-list` | 3 | show local + federated KBs |

### `src/rag_api.rs` — `kwaainet rag serve`

```rust
struct RagState {
    tenant_id: Uuid, eve_peer: PeerId,
    p2p_client: Arc<Mutex<P2PClient>>,
    embed_client: EmbedClient, meta_store: Arc<MetaStore>,
    adaptive_router: Option<AdaptiveRouter>,                    // Phase 3+
    graph: Option<Arc<RwLock<KnowledgeGraph>>>,                // Phase 4+
    sessions: Arc<Mutex<HashMap<String, RecursiveMemory>>>,    // Phase 4+
    inference_base_url: String, top_k: usize, strategy: RagStrategy,
    http_client: reqwest::Client,
}

// Routes:
//   GET  /v1/models             → lists "kwaai-rag"
//   POST /v1/chat/completions   → full RAG pipeline (stream + non-stream)
//   POST /v1/completions        → RAG-augmented
//   POST /api/ingest            → multipart file upload
//   GET  /api/docs              → list ingested docs
//   DELETE /api/docs/:name      → delete doc
//   GET  /api/metrics           → last eval scores (Phase 2)
//   GET  /                      → include_str!("../assets/rag_ui.html")
```

RAG pipeline per `/v1/chat/completions` request:
1. Extract last `"user"` message; session_id from `X-Session-Id` header or random
2. Phase 3+: `adaptive_router.route(query)` → `RagStrategy`
3. Phase 2: optional `query_transform` (HyDE / decompose) based on strategy
4. `retrieve(query, ..)` — dense + optional BM25 hybrid
5. Phase 3+: CRAG confidence check; re-retrieve if needed
6. Phase 2: optional `reranker.rerank()`; sentence-window context expansion
7. Phase 4: optional graph neighborhood expansion
8. `build_chat_messages(query, chunks, history, max_context_chars)` — augmented messages
9. Phase 4: `RecursiveMemory` compression + entity tracking
10. POST augmented messages to shard API → re-stream SSE → return

---

## Modified Files

| File | Change |
|---|---|
| `core/Cargo.toml` | Add `"crates/kwaai-rag"` to `[members]`; promote `redb = "2"` and add `sha2 = "0.10"` to `[workspace.dependencies]` |
| `crates/kwaai-cli/Cargo.toml` | Add `kwaai-rag.workspace = true`, `lopdf = { version = "0.33", optional = true }`; feature `rag-pdf = ["lopdf"]` |
| `crates/kwaai-cli/src/cli.rs` | Add `Rag(RagArgs)` to `Command` enum with all subcommands |
| `crates/kwaai-cli/src/main.rs` | Add `mod rag_cmd; mod rag_api;` + dispatch arm |
| `crates/kwaai-cli/src/config.rs` | Add `RagConfig { tenant_id, eve_peer_id, embed_model, inference_url, strategy, top_k, rerank }` to `KwaaiNetConfig` |
| `crates/kwaai-storage/src/tenant.rs` | Add `vector_dimension: u32` to `TenantInfo` and `TenantRecord` — needed for dimension guard |
| `crates/kwaai-cli/src/vpk.rs:419` | `async fn p2p_connect` → `pub(crate) async fn p2p_connect` |

---

## Verification

```bash
# 1. Build
cd core && cargo build -p kwaainet

# 2. Ensure Ollama has embedding model
ollama pull nomic-embed-text

# 3. Start fabric
kwaainet start --daemon

# 4. Init RAG
kwaainet rag init --eve-peer-id $(kwaainet identity | grep PeerID | awk '{print $2}')

# 5. Ingest
echo "The capital of France is Paris." > /tmp/test.txt
kwaainet rag ingest /tmp/test.txt

# 6. Dense retrieval
kwaainet rag query "What is the capital of France?"
# Expected: chunk with score > 0.7 containing "Paris"

# 7. Hybrid retrieval (Phase 2)
kwaainet rag query "capital France" --mode hybrid
# Expected: same chunk, ranked first via RRF merge

# 8. Interactive chat
kwaainet shard api &
kwaainet rag chat
# "What is the capital of France?" → "Paris"

# 9. RAG HTTP server + embedded UI
kwaainet rag serve --port 9090
open http://localhost:9090

# 10. CRAG confidence check (Phase 3)
kwaainet rag query "Who invented quantum teleportation?" --rerank
# Expected: low-confidence result + knowledge-gap note in context

# 11. OpenWebUI integration
docker run -p 3000:8080 -e OPENAI_API_BASE_URL=http://host.docker.internal:9090 ghcr.io/open-webui/open-webui
open http://localhost:3000
```

---

## Out of Scope

- Cross-modal RAG (image/audio documents)
- Fine-tuning embedding model on domain corpus
- External web search tool in agentic loop (Phase 4 SelfAsk only)
- BM25 persistence across Eve node restarts (index lives in local redb, rebuilt on startup if missing)
