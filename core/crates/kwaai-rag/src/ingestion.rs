use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use tracing::{debug, info, warn};

use crate::chunker::{split_text, Chunk, ChunkConfig};
use crate::embedder::EmbedClient;
use crate::graph::{entity_id, extract_from_text, EntityNode, ExtractedEntity, ExtractedRelation, GraphStore};
use crate::meta_store::{ChunkMeta, MetaStore};

/// Optional graph extraction config attached to an ingestion run.
pub struct GraphIngestConfig {
    pub store: Arc<Mutex<GraphStore>>,
    /// Primary inference URL. Used when `inference_urls` is empty.
    pub inference_url: String,
    /// Override URL list for round-robin multi-endpoint dispatch.
    /// Non-empty list takes precedence over `inference_url`.
    pub inference_urls: Vec<String>,
    /// Model name for the chat completions call (e.g. "llama3.2:3b").
    /// Defaults to "default" which works with the kwaainet shard API.
    pub model: String,
    /// Max concurrent extraction tasks. 1 = sequential (default). N = fan-out.
    pub workers: usize,
}

impl GraphIngestConfig {
    fn effective_urls(&self) -> Vec<String> {
        if self.inference_urls.is_empty() {
            vec![self.inference_url.clone()]
        } else {
            self.inference_urls.clone()
        }
    }
}

pub struct IngestConfig {
    pub embed: EmbedClient,
    pub chunk_cfg: ChunkConfig,
    pub upload_batch_size: usize,
    /// When set, entities are extracted and stored in the knowledge graph
    /// after chunks are embedded. Has no effect on chunk storage or vector upload.
    pub graph: Option<GraphIngestConfig>,
    /// Optional per-document metadata prefixes.
    /// Keys are substrings matched (case-insensitive) against doc_name.
    /// The matched prefix is prepended to each chunk's text before embedding and storage.
    /// Loaded from a YAML file via --doc-meta on rag ingest/sync.
    pub doc_meta: HashMap<String, String>,
}

impl IngestConfig {
    pub fn new(embed: EmbedClient) -> Self {
        Self {
            embed,
            chunk_cfg: ChunkConfig::default(),
            upload_batch_size: 64,
            graph: None,
            doc_meta: HashMap::new(),
        }
    }
}

pub struct IngestionResult {
    pub chunks_ingested: usize,
    pub vectors_uploaded: usize,
}

/// Ingest a document: chunk → embed → upload + store metadata.
///
/// `upload_fn` closes over P2PClient + Eve PeerId and handles the storage RPC.
/// Keeping this boundary here lets kwaai-rag stay free of kwaai-p2p-daemon.
pub async fn ingest_text(
    cfg: &IngestConfig,
    meta: &MetaStore,
    doc_name: &str,
    text: &str,
    upload_fn: impl Fn(Vec<(i64, Vec<f32>)>) -> Pin<Box<dyn Future<Output = Result<usize>> + Send>>,
    progress: Option<impl Fn(usize, usize)>,
) -> Result<IngestionResult> {
    let raw_chunks = split_text(text, doc_name, &cfg.chunk_cfg);
    let chunks = apply_doc_meta(raw_chunks, doc_name, &cfg.doc_meta);
    let total = chunks.len();
    info!(doc = doc_name, chunks = total, "ingesting document");

    if chunks.is_empty() {
        return Ok(IngestionResult {
            chunks_ingested: 0,
            vectors_uploaded: 0,
        });
    }

    let mut metas: Vec<ChunkMeta> = Vec::with_capacity(total);
    let mut ids: Vec<i64> = Vec::with_capacity(total);
    let mut total_uploaded = 0usize;
    let ingested_at = MetaStore::now_rfc3339();

    for batch in chunks.chunks(cfg.upload_batch_size) {
        let texts: Vec<&str> = batch.iter().map(|c| c.text.as_str()).collect();
        let embeddings = cfg.embed.embed_batch(&texts).await?;

        let vectors: Vec<(i64, Vec<f32>)> = batch
            .iter()
            .zip(embeddings.iter())
            .map(|(c, e)| (c.id, e.clone()))
            .collect();

        let uploaded = upload_fn(vectors).await?;
        total_uploaded += uploaded;

        for c in batch {
            metas.push(ChunkMeta {
                doc_name: c.doc_name.clone(),
                chunk_index: c.chunk_index,
                text: c.text.clone(),
                surrounding: c.surrounding.clone(),
                page_num: c.page_num,
                ingested_at: ingested_at.clone(),
            });
            ids.push(c.id);
        }

        if let Some(ref prog) = progress {
            prog(ids.len(), total);
        }
        debug!(done = ids.len(), total, "batch uploaded");
    }

    meta.put_chunks(&metas, &ids)?;

    // Optional: extract entities from each chunk and populate the knowledge graph.
    if let Some(graph_cfg) = &cfg.graph {
        extract_and_store_entities(&chunks, &ids, &cfg.embed, graph_cfg).await;
    }

    Ok(IngestionResult {
        chunks_ingested: total,
        vectors_uploaded: total_uploaded,
    })
}

/// Internal result type carrying one chunk's extraction output back to the drain task.
struct ChunkResult {
    chunk_id: i64,
    entities: Vec<ExtractedEntity>,
    relations: Vec<ExtractedRelation>,
    embeddings: Vec<Vec<f32>>,
}

/// Public entry point for the `graph build` command.
///
/// When `graph_cfg.workers > 1`, chunks are dispatched concurrently across all
/// effective inference URLs (round-robin). The GraphStore is only written by a single
/// drain task — LLM calls and embedding run in parallel without holding the mutex.
///
/// `progress` is called after each chunk is written to the graph:
/// `(chunks_done, total_chunks, entity_count, relation_count)`.
pub async fn extract_and_store_entities_pub(
    chunks: &[Chunk],
    chunk_ids: &[i64],
    embed: &EmbedClient,
    graph_cfg: &GraphIngestConfig,
    progress: Option<Arc<dyn Fn(usize, usize, usize, usize) + Send + Sync>>,
) {
    let total = chunks.len();
    let urls = Arc::new(graph_cfg.effective_urls());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let workers = graph_cfg.workers.max(1);
    let model = Arc::new(graph_cfg.model.clone());
    let store = graph_cfg.store.clone();

    // Channel capacity must be large enough that spawned tasks never block waiting
    // to send while the spawn loop holds the only tokio task slot.  Using a
    // concurrent drain task (below) makes the exact size irrelevant, but a modest
    // buffer still reduces contention.
    let sem = Arc::new(tokio::sync::Semaphore::new(workers));
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ChunkResult>(workers.max(4) * 4);

    // Drain runs as a *concurrent* task so it can consume results while the spawn
    // loop above is still blocking on semaphore acquisition.  Running it sequentially
    // after the loop causes a deadlock: spawn loop blocks on the semaphore waiting
    // for a task to finish; the task blocks trying to send into a full channel;
    // the channel is never drained because the drain hasn't started yet.
    let drain_store = store.clone();
    let drain_handle = tokio::spawn(async move {
        let mut done = 0usize;
        while let Some(res) = rx.recv().await {
            done += 1;
            if res.entities.is_empty() {
                if let Some(ref prog) = progress {
                    let (nc, rc) = drain_store.lock().map(|g| (g.node_count(), g.relation_count())).unwrap_or((0, 0));
                    prog(done, total, nc, rc);
                }
                continue;
            }

            let mut graph = match drain_store.lock() {
                Ok(g) => g,
                Err(_) => { warn!("graph store mutex poisoned"); continue; }
            };

            let mut entity_ids_for_chunk = Vec::new();
            for (extracted, emb) in res.entities.iter().zip(res.embeddings.into_iter()) {
                let eid = entity_id(&extracted.name, &extracted.entity_type);
                let node = EntityNode {
                    id: eid,
                    name: extracted.name.clone(),
                    entity_type: extracted.entity_type.clone(),
                    description: extracted.description.clone(),
                    embedding: emb,
                    mention_count: 1,
                    first_chunk_id: res.chunk_id,
                    aliases: vec![],
                };
                if let Err(e) = graph.upsert_entity(node) {
                    warn!("upsert_entity: {e}");
                    continue;
                }
                entity_ids_for_chunk.push(eid);
            }

            for rel in &res.relations {
                let src = resolve_entity_id(&rel.from, &res.entities, &graph);
                let dst = resolve_entity_id(&rel.to, &res.entities, &graph);
                if let Err(e) = graph.upsert_relation(src, dst, &rel.relation, res.chunk_id) {
                    warn!("upsert_relation: {e}");
                }
            }
            if let Err(e) = graph.link_chunk(res.chunk_id, &entity_ids_for_chunk) {
                warn!("link_chunk: {e}");
            }

            if let Some(ref prog) = progress {
                prog(done, total, graph.node_count(), graph.relation_count());
            }
        }
    });

    // Spawn one extraction task per chunk; semaphore caps concurrency.
    for (_i, (chunk, &chunk_id)) in chunks.iter().zip(chunk_ids.iter()).enumerate() {
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        let text = chunk.text.clone();
        let urls = urls.clone();
        let url_counter = url_counter.clone();
        let model = model.clone();
        let embed = embed.clone();

        tokio::spawn(async move {
            let _permit = permit;
            let idx = url_counter.fetch_add(1, Ordering::Relaxed) % urls.len();
            let url = &urls[idx];

            let (entities, relations) = match extract_from_text(&text, url, &model).await {
                Ok(r) => r,
                Err(e) => {
                    warn!("entity extraction error for chunk {chunk_id}: {e}");
                    let _ = tx.send(ChunkResult { chunk_id, entities: vec![], relations: vec![], embeddings: vec![] }).await;
                    return;
                }
            };

            let embeddings = if entities.is_empty() {
                vec![]
            } else {
                let texts: Vec<String> = entities
                    .iter()
                    .map(|e| {
                        if e.description.is_empty() {
                            e.name.clone()
                        } else {
                            format!("{}: {}", e.name, e.description)
                        }
                    })
                    .collect();
                let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
                match embed.embed_batch(&text_refs).await {
                    Ok(e) => e,
                    Err(e) => {
                        warn!("entity embedding error for chunk {chunk_id}: {e}");
                        vec![]
                    }
                }
            };

            let _ = tx.send(ChunkResult { chunk_id, entities, relations, embeddings }).await;
        });
    }
    drop(tx); // close sender — drain task's rx.recv() will return None once queue empties

    drain_handle.await.unwrap_or(());
}

/// Extract entities from all chunks and persist them to the GraphStore.
/// Errors are logged and swallowed so they never block document ingestion.
async fn extract_and_store_entities(
    chunks: &[Chunk],
    chunk_ids: &[i64],
    embed: &EmbedClient,
    graph_cfg: &GraphIngestConfig,
) {
    for (chunk, &chunk_id) in chunks.iter().zip(chunk_ids.iter()) {
        let (entities, relations) = match extract_from_text(
            &chunk.text,
            &graph_cfg.inference_url,
            &graph_cfg.model,
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                warn!("entity extraction error for chunk {chunk_id}: {e}");
                continue;
            }
        };

        if entities.is_empty() {
            continue;
        }

        // Embed "{name}: {description}" so abbreviations/acronyms match by name.
        let texts: Vec<String> = entities
            .iter()
            .map(|e| {
                if e.description.is_empty() {
                    e.name.clone()
                } else {
                    format!("{}: {}", e.name, e.description)
                }
            })
            .collect();
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let embeddings = match embed.embed_batch(&text_refs).await {
            Ok(e) => e,
            Err(e) => {
                warn!("entity embedding error for chunk {chunk_id}: {e}");
                continue;
            }
        };

        let mut graph = match graph_cfg.store.lock() {
            Ok(g) => g,
            Err(_) => {
                warn!("graph store mutex poisoned");
                continue;
            }
        };

        let mut entity_ids_for_chunk = Vec::new();
        for (extracted, emb) in entities.iter().zip(embeddings.into_iter()) {
            let eid = entity_id(&extracted.name, &extracted.entity_type);
            let node = EntityNode {
                id: eid,
                name: extracted.name.clone(),
                entity_type: extracted.entity_type.clone(),
                description: extracted.description.clone(),
                embedding: emb,
                mention_count: 1,
                first_chunk_id: chunk_id,
                aliases: vec![],
            };
            if let Err(e) = graph.upsert_entity(node) {
                warn!("upsert_entity failed: {e}");
                continue;
            }
            entity_ids_for_chunk.push(eid);
        }

        for rel in &relations {
            let src = resolve_entity_id(&rel.from, &entities, &graph);
            let dst = resolve_entity_id(&rel.to, &entities, &graph);
            if let Err(e) = graph.upsert_relation(src, dst, &rel.relation, chunk_id) {
                warn!("upsert_relation failed: {e}");
            }
        }

        if let Err(e) = graph.link_chunk(chunk_id, &entity_ids_for_chunk) {
            warn!("link_chunk failed: {e}");
        }

        debug!(
            chunk_id,
            entities = entity_ids_for_chunk.len(),
            relations = relations.len(),
            "graph updated"
        );
    }
}

/// Resolve a relation endpoint to an entity ID.
/// Priority: (1) current-chunk extraction, (2) existing graph node by name, (3) Unknown fallback.
/// This prevents dangling adjacency entries when a cross-chunk relation references an entity
/// that was extracted in an earlier chunk under a non-Unknown type.
fn resolve_entity_id(name: &str, current_entities: &[ExtractedEntity], graph: &GraphStore) -> i64 {
    if let Some(e) = current_entities
        .iter()
        .find(|e| e.name.to_lowercase() == name.to_lowercase())
    {
        return entity_id(&e.name, &e.entity_type);
    }
    if let Some(node) = graph.find_by_name(name) {
        return node.id;
    }
    entity_id(name, "Unknown")
}

/// Prepend doc-level metadata to each chunk's text if the doc_name matches any key.
/// Keys are matched as case-insensitive substrings of doc_name.
fn apply_doc_meta(mut chunks: Vec<Chunk>, doc_name: &str, doc_meta: &HashMap<String, String>) -> Vec<Chunk> {
    if doc_meta.is_empty() {
        return chunks;
    }
    let doc_lower = doc_name.to_lowercase();
    let prefix = doc_meta
        .iter()
        .find(|(k, _)| doc_lower.contains(k.to_lowercase().as_str()))
        .map(|(_, v)| v.as_str());

    if let Some(pfx) = prefix {
        for chunk in &mut chunks {
            chunk.text = format!("{pfx}\n\n{}", chunk.text);
        }
    }
    chunks
}
