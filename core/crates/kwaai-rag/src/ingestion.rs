use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use tracing::{debug, info, warn};

use crate::chunker::{split_text, Chunk, ChunkConfig};
use crate::doc_schema::DocSchema;
use crate::embedder::EmbedClient;
use crate::gliner::GliNERClient;
use crate::graph::{
    description_from_fields, entity_id, extract_from_text, EntityNode, ExtractedEntity,
    ExtractedRelation, FieldValue, GraphStore,
};
use crate::meta_store::{ChunkMeta, MetaStore};
use crate::ner;

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
    /// When non-empty, only these entity types are extracted (overrides ENTITY_TYPES).
    pub entity_types: Vec<String>,
    /// When true, no relations are extracted or stored.
    pub no_relations: bool,
    /// Number of adjacent chunks to include as surrounding context when extracting
    /// entities from a chunk. 0 = current chunk only (legacy). 1 = include one chunk
    /// before and after (recommended; +7pp recall in experiments). Default: 1.
    pub context_window: usize,
    /// Optional GLiNER NER client. When set, person spans are detected before each LLM
    /// call and injected as high-confidence hints into the extraction prompt.
    pub gliner_client: Option<GliNERClient>,
    /// When true, use entity-centric extraction: GLiNER scans all chunks first to
    /// collect unique person names, then one focused LLM call is made per name with
    /// aggregated multi-chunk context. Requires gliner_client to be set.
    pub entity_centric: bool,
    /// After CC build, escalate entities whose confidence score (from score_entity()) is
    /// below this threshold to a focused EC refinement pass. 0.0 = disabled (default).
    pub ec_refine_threshold: f32,
    /// Max entities to escalate per run (cost guard). Default 50.
    pub ec_refine_budget: usize,
    /// When true, skip the CC extraction loop entirely — just re-score existing entities
    /// and run the EC refinement pass. Requires ec_refine_threshold > 0.0.
    pub ec_refine_only: bool,
    /// Process N consecutive chunks per LLM call (default 1 = one chunk + context_window).
    /// chunk_batch=3: loop strides by 3, each call covers chunks [i..i+3] plus the
    /// context_window on each side. Reduces calls by 3× at the cost of denser context.
    pub chunk_batch: usize,
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
    /// Optional document schema — controls section tagging, skip flags, and narrator overrides.
    pub doc_schema: Option<DocSchema>,
}

impl IngestConfig {
    pub fn new(embed: EmbedClient) -> Self {
        Self {
            embed,
            chunk_cfg: ChunkConfig::default(),
            upload_batch_size: 64,
            graph: None,
            doc_meta: HashMap::new(),
            doc_schema: None,
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
    let raw_chunks = split_text(text, doc_name, &cfg.chunk_cfg, cfg.doc_schema.as_ref());
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
        // Prepend section name when available so the embedding carries chapter/section
        // context and improves Round 1 retrieval for section-specific questions (1.5).
        let embed_strings: Vec<String> = batch
            .iter()
            .map(|c| match c.section_name.as_deref() {
                Some(s) if !s.is_empty() => format!("[{}] {}", s, c.text),
                _ => c.text.clone(),
            })
            .collect();
        let texts: Vec<&str> = embed_strings.iter().map(|s| s.as_str()).collect();
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
                section_name: c.section_name.clone(),
                skip_extraction: c.skip_extraction,
                section_note: c.section_note.clone(),
                section_type: c.section_type.clone(),
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
#[allow(clippy::type_complexity)]
pub async fn extract_and_store_entities_pub(
    chunks: &[Chunk],
    chunk_ids: &[i64],
    embed: &EmbedClient,
    graph_cfg: &GraphIngestConfig,
    progress: Option<Arc<dyn Fn(usize, usize, usize, usize) + Send + Sync>>,
) {
    if graph_cfg.entity_centric {
        extract_entity_centric(chunks, chunk_ids, embed, graph_cfg, progress).await;
        return;
    }

    // EC-refine-only: skip CC extraction, just re-score and run refinement pass.
    if graph_cfg.ec_refine_only {
        println!("  EC refine-only: skipping CC extraction, re-scoring existing entities");
        {
            let mut g = graph_cfg.store.lock().unwrap_or_else(|e| {
                warn!("graph store mutex was poisoned; recovering inner value");
                e.into_inner()
            });
            g.sync_evidence();
            if let Err(e) = g.score_all_confidences() {
                warn!("confidence scoring failed: {e}");
            }
        }
        if graph_cfg.ec_refine_threshold > 0.0 {
            refine_low_confidence_entities(chunks, chunk_ids, embed, graph_cfg).await;
        }
        return;
    }
    let total = chunks.len();
    let urls = Arc::new(graph_cfg.effective_urls());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let workers = graph_cfg.workers.max(1);
    let model = Arc::new(graph_cfg.model.clone());
    let entity_types_cfg = Arc::new(graph_cfg.entity_types.clone());
    let no_relations = graph_cfg.no_relations;
    let context_window = graph_cfg.context_window;
    let chunk_batch = graph_cfg.chunk_batch.max(1);
    let store = graph_cfg.store.clone();
    let gliner = Arc::new(graph_cfg.gliner_client.clone());

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
                    let (nc, rc) = drain_store
                        .lock()
                        .map(|g| (g.node_count(), g.relation_count()))
                        .unwrap_or((0, 0));
                    prog(done, total, nc, rc);
                }
                continue;
            }

            let mut graph = match drain_store.lock() {
                Ok(g) => g,
                Err(_) => {
                    warn!("graph store mutex poisoned");
                    continue;
                }
            };

            let mut entity_ids_for_chunk = Vec::new();
            for (extracted, emb) in res.entities.iter().zip(res.embeddings) {
                let Some(extracted_name_owned) = clean_extracted_name(&extracted.name) else {
                    continue;
                };
                let extracted_name = extracted_name_owned.as_str();
                let eid = entity_id(extracted_name, &extracted.entity_type);
                // Build FieldValue map: wrap each extracted string value with chunk provenance.
                let fields: HashMap<String, FieldValue> = extracted
                    .fields
                    .iter()
                    .filter(|(_, v)| !v.is_empty())
                    .map(|(k, v)| (k.clone(), FieldValue::new(v.clone(), res.chunk_id)))
                    .collect();
                let description = {
                    let from_fields =
                        description_from_fields(extracted_name, &extracted.entity_type, &fields);
                    if from_fields.is_empty() {
                        extracted.description.clone()
                    } else {
                        from_fields
                    }
                };
                let node = EntityNode {
                    id: eid,
                    name: extracted_name.to_string(),
                    entity_type: extracted.entity_type.clone(),
                    description,
                    embedding: emb,
                    mention_count: 1,
                    first_chunk_id: res.chunk_id,
                    aliases: vec![],
                    schema_type: None,
                    evidence: Vec::new(),
                    gender: None,
                    fields,
                    confidence: 0.0,
                    extraction_confidence: extracted.extraction_confidence,
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

    // Snapshot Person entity genders for pronoun resolution.  Taken once before
    // the spawn loop so async tasks never need to hold the graph lock.
    let gender_context = Arc::new({
        let g = store.lock().unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        });
        g.all_entities()
            .filter(|e| e.entity_type == "Person")
            .map(|e| (e.name.clone(), e.gender.clone()))
            .collect::<Vec<_>>()
    });

    // Spawn one extraction task per chunk (or per batch when chunk_batch > 1).
    // With chunk_batch=N the loop strides by N: chunks 0,N,2N,… are centers,
    // each covering [center .. center+N-1] plus context_window on each side.
    let mut i = 0;
    while i < total {
        let batch_end = (i + chunk_batch).min(total);
        let chunk = &chunks[i];
        let chunk_id = chunk_ids[i];

        // Skip entire batch if the leading chunk is flagged (index/appendix sections).
        if chunk.skip_extraction {
            debug!(
                chunk_id,
                section = ?chunk.section_name,
                "skipping extraction for flagged section"
            );
            i += chunk_batch;
            continue;
        }
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        // Build context: [i-window .. batch_end+window), preserving narrative context.
        let text = if context_window > 0 || chunk_batch > 1 {
            let start = i.saturating_sub(context_window);
            let end = (batch_end + context_window).min(total);
            // Only include chunks from the same section zone as the center chunk.
            // This prevents context bleeding across section boundaries (e.g.
            // Acknowledgements text leaking into the adjacent Dedication chunk).
            let center_type = &chunk.section_type;
            chunks[start..end]
                .iter()
                .filter(|c| center_type.same_window_zone(&c.section_type))
                .map(|c| c.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n[...]\n\n")
        } else {
            chunk.text.clone()
        };
        let section_note = chunk.section_note.clone();
        let urls = urls.clone();
        let url_counter = url_counter.clone();
        let model = model.clone();
        let embed = embed.clone();
        let entity_types_cfg = entity_types_cfg.clone();
        let gender_context = gender_context.clone();
        let gliner = gliner.clone();

        tokio::spawn(async move {
            let _permit = permit;
            let idx = url_counter.fetch_add(1, Ordering::Relaxed) % urls.len();
            let url = &urls[idx];
            let et: Vec<&str> = entity_types_cfg.iter().map(|s| s.as_str()).collect();

            // GLiNER runs first when available — higher recall for Person names than
            // the regex pre-screener (catches mid-sentence names, OCR artifacts, etc.).
            let gliner_hints: Vec<String> = match gliner.as_ref() {
                Some(client) => client.person_spans(&text).await,
                None => vec![],
            };

            // Candidates = regex proper-nouns ∪ GLiNER spans.
            // Regex still provides coverage for non-Person types (Place, Organization).
            // The union is what the LLM-skip gate in extract_from_text() checks, so a
            // chunk is only skipped when both the regex AND GLiNER find nothing.
            let mut candidates = ner::extract_proper_noun_candidates(&text);
            for span in &gliner_hints {
                if !candidates.contains(span) {
                    candidates.push(span.clone());
                }
            }

            // Pass only GLiNER-confirmed Person spans as backward-scan candidates.
            // Gendered pronouns (he/she) must resolve to Person entities only — using
            // the full candidates list caused Place names (e.g. "District Six") to be
            // selected when they appeared closest before the pronoun in the text.
            // GLiNER hints are Person-specific; when unavailable, backward scan is
            // skipped and forward scan is used as before.
            let pronoun_map = ner::resolve_pronouns(&text, &gender_context, &gliner_hints);

            // Merge resolved pronoun targets into candidates so the LLM sees them in
            // the classification list, not only in the KNOWN COREFERENCES preamble.
            // This ensures a chunk like "He sat down." still produces a Person entity
            // when the pronoun resolved to a known name.
            for (_, name) in &pronoun_map {
                if !candidates.contains(name) {
                    candidates.push(name.clone());
                }
            }

            let hints_opt: Option<&[String]> = if gliner_hints.is_empty() {
                None
            } else {
                Some(&gliner_hints)
            };

            let (mut entities, relations) = match extract_from_text(
                &text,
                &candidates,
                &pronoun_map,
                section_note.as_deref(),
                url,
                &model,
                &et,
                no_relations,
                hints_opt,
            )
            .await
            {
                Ok(r) => r,
                Err(e) => {
                    warn!("entity extraction error for chunk {chunk_id}: {e}");
                    let _ = tx
                        .send(ChunkResult {
                            chunk_id,
                            entities: vec![],
                            relations: vec![],
                            embeddings: vec![],
                        })
                        .await;
                    return;
                }
            };

            // Drop entities whose type the LLM returned outside the allowed list.
            if !et.is_empty() {
                entities.retain(|e| et.iter().any(|t| t.eq_ignore_ascii_case(&e.entity_type)));
            }

            let embeddings = if entities.is_empty() {
                vec![]
            } else {
                let texts: Vec<String> = entities
                    .iter()
                    .map(|e| {
                        let desc = if e.fields.is_empty() {
                            e.description.clone()
                        } else {
                            let fv_map: HashMap<String, FieldValue> = e
                                .fields
                                .iter()
                                .filter(|(_, v)| !v.is_empty())
                                .map(|(k, v)| (k.clone(), FieldValue::new(v.clone(), chunk_id)))
                                .collect();
                            let s = description_from_fields(&e.name, &e.entity_type, &fv_map);
                            if s.is_empty() {
                                e.description.clone()
                            } else {
                                s
                            }
                        };
                        if desc.is_empty() {
                            e.name.clone()
                        } else {
                            format!("{}: {}", e.name, desc)
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

            let _ = tx
                .send(ChunkResult {
                    chunk_id,
                    entities,
                    relations,
                    embeddings,
                })
                .await;
        });
        i += chunk_batch;
    }
    drop(tx); // close sender — drain task's rx.recv() will return None once queue empties

    drain_handle.await.unwrap_or(());

    // Sync entity.evidence from the chunk index (populated during the drain by
    // link_chunk(), but not automatically reflected on in-memory EntityNode fields).
    // Needed so EC refinement and confidence scoring can read per-entity chunk lists.
    {
        let mut g = store.lock().unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        });
        g.sync_evidence();
        if let Err(e) = g.score_all_confidences() {
            warn!("confidence scoring failed: {e}");
        }
    }

    // EC refinement: escalate low-confidence entities for a targeted second pass.
    if graph_cfg.ec_refine_threshold > 0.0 {
        refine_low_confidence_entities(chunks, chunk_ids, embed, graph_cfg).await;
    }
}

/// Extract entities from all chunks and persist them to the GraphStore.
/// Errors are logged and swallowed so they never block document ingestion.
async fn extract_and_store_entities(
    chunks: &[Chunk],
    chunk_ids: &[i64],
    embed: &EmbedClient,
    graph_cfg: &GraphIngestConfig,
) {
    let total = chunks.len();
    let context_window = graph_cfg.context_window;
    for (i, (chunk, &chunk_id)) in chunks.iter().zip(chunk_ids.iter()).enumerate() {
        if chunk.skip_extraction {
            debug!(
                chunk_id,
                section = ?chunk.section_name,
                "skipping extraction for flagged section"
            );
            continue;
        }
        let text = if context_window > 0 {
            let start = i.saturating_sub(context_window);
            let end = (i + context_window + 1).min(total);
            let center_type = &chunk.section_type;
            chunks[start..end]
                .iter()
                .filter(|c| center_type.same_window_zone(&c.section_type))
                .map(|c| c.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n[...]\n\n")
        } else {
            chunk.text.clone()
        };
        let et: Vec<&str> = graph_cfg.entity_types.iter().map(|s| s.as_str()).collect();
        let candidates = ner::extract_proper_noun_candidates(&text);
        let gliner_hints: Vec<String> = match &graph_cfg.gliner_client {
            Some(client) => client.person_spans(&text).await,
            None => vec![],
        };
        let hints_opt: Option<&[String]> = if gliner_hints.is_empty() {
            None
        } else {
            Some(&gliner_hints)
        };
        let (mut entities, relations) = match extract_from_text(
            &text,
            &candidates,
            &[],
            chunk.section_note.as_deref(),
            &graph_cfg.inference_url,
            &graph_cfg.model,
            &et,
            graph_cfg.no_relations,
            hints_opt,
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                warn!("entity extraction error for chunk {chunk_id}: {e}");
                continue;
            }
        };

        if !et.is_empty() {
            entities.retain(|e| et.iter().any(|t| t.eq_ignore_ascii_case(&e.entity_type)));
        }

        if entities.is_empty() {
            continue;
        }

        let texts: Vec<String> = entities
            .iter()
            .map(|e| {
                let desc = if e.fields.is_empty() {
                    e.description.clone()
                } else {
                    let fv_map: HashMap<String, FieldValue> = e
                        .fields
                        .iter()
                        .filter(|(_, v)| !v.is_empty())
                        .map(|(k, v)| (k.clone(), FieldValue::new(v.clone(), chunk_id)))
                        .collect();
                    let s = description_from_fields(&e.name, &e.entity_type, &fv_map);
                    if s.is_empty() {
                        e.description.clone()
                    } else {
                        s
                    }
                };
                if desc.is_empty() {
                    e.name.clone()
                } else {
                    format!("{}: {}", e.name, desc)
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
        for (extracted, emb) in entities.iter().zip(embeddings) {
            let eid = entity_id(&extracted.name, &extracted.entity_type);
            let fields: HashMap<String, FieldValue> = extracted
                .fields
                .iter()
                .filter(|(_, v)| !v.is_empty())
                .map(|(k, v)| (k.clone(), FieldValue::new(v.clone(), chunk_id)))
                .collect();
            let description = {
                let from_fields =
                    description_from_fields(&extracted.name, &extracted.entity_type, &fields);
                if from_fields.is_empty() {
                    extracted.description.clone()
                } else {
                    from_fields
                }
            };
            let node = EntityNode {
                id: eid,
                name: extracted.name.clone(),
                entity_type: extracted.entity_type.clone(),
                description,
                embedding: emb,
                mention_count: 1,
                first_chunk_id: chunk_id,
                aliases: vec![],
                schema_type: None,
                evidence: Vec::new(),
                gender: None,
                fields,
                confidence: 0.0,
                extraction_confidence: extracted.extraction_confidence,
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
fn apply_doc_meta(
    mut chunks: Vec<Chunk>,
    doc_name: &str,
    doc_meta: &HashMap<String, String>,
) -> Vec<Chunk> {
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

// ── Shared entity name filter ─────────────────────────────────────────────────

/// Filter and normalise an extracted entity name.
///
/// Returns `None` if the name should be discarded (blocklist hit, role prefix,
/// sentence starter, empty after normalisation). Returns `Some(clean_name)`
/// otherwise. Used by both chunk-centric and entity-centric drains so they
/// apply identical filtering.
pub(crate) fn clean_extracted_name(raw: &str) -> Option<String> {
    const GENERIC_ROLE_BLOCKLIST: &[&str] = &[
        "granny",
        "gran",
        "grandma",
        "grandfather",
        "grandpa",
        "gramps",
        "dad",
        "daddy",
        "father",
        "mother",
        "mom",
        "mum",
        "mama",
        "uncle",
        "auntie",
        "aunt",
        "cousin",
        "son",
        "daughter",
        "me",
        "i",
        "he",
        "she",
        "they",
        "we",
        "the narrator",
        "the author",
        "narrator",
        "author",
        "herrenvolk",
        "herrenvolkism",
        "apartheid",
        "coloured",
        "coloureds",
        "blacks",
        "whites",
        "white",
        "black",
        "indians",
        "africans",
        "europeans",
        "non-white",
        "non-whites",
        "non-european",
        "cape malay",
        "cape malay_indian",
        "pathan",
        "pathans",
        "xhosa",
        "slavic",
        "hungarian",
        "jewish",
        "aryan",
        "moslem",
        "muslim",
        "nationalist",
        "nationalists",
        "german",
        "french",
        "russian",
        "british",
        "english",
        "african",
        "indian",
        "arab",
        "arabs",
        "chinese",
        "boer",
        "bantu",
        "coolie",
        "coolies",
        "malay",
        "malays",
        "griqua",
        "hindu",
        "hindus",
        "irish",
        "japanese",
        "norwegian",
        "sikh",
        "turks",
        "zulus",
        "afrikaner",
        "afrikaners",
        "west indians",
        "south african",
        "cape coloured",
        "non-white muslim south africans",
        "socialist",
        "marxist",
        "labour",
        "communist",
        "fascist",
        "nazi",
        "nats",
        "native",
        "christmas",
        "eid",
        "eid mubarak",
        "islam",
        "ramadan",
        "victorian",
        "history",
        "science",
        "schooling",
        "mother tongue",
        "everything",
        "something",
        "nothing",
        "anything",
        "there",
        "here",
        "this",
        "that",
        "these",
        "those",
        "each",
        "every",
        "all",
        "none",
        "some",
        "any",
        "both",
        "one",
        "many",
        "such",
        "how",
        "when",
        "moreover",
        "sometime",
        "alas",
        "half",
        "apart",
        "being",
        "blot",
        "do",
        "everyone",
        "figure",
        "found",
        "great",
        "had",
        "hatless",
        "just",
        "later",
        "little",
        "much",
        "needless",
        "next",
        "now",
        "ob",
        "perh",
        "perhaps",
        "peru",
        "piccadilly",
        "regrettably",
        "several",
        "shyly",
        "soon",
        "still",
        "tell",
        "theoretically",
        "v1",
        "va",
        "whether",
        "wo",
        "worse",
        "poor abdul",
        "flash",
        "dandy",
        "lobo",
        "baby",
        "youth",
        "legless",
        "muddy",
        "polly",
        "tiny",
        "vic",
        "bill",
        "solly",
        "nina",
        "kismets",
        "zoology",
        "cadbury",
        "freubel",
        "south african indian",
        "head of british muslims",
        "non-white councillors",
        "prof",
        "prof.",
        "prof_",
        "gools",
        "rassools",
        "goldings",
        "killers",
        "stranglers",
        "royal family",
        "mr.",
        "mr_",
        "rev.",
        "rev_",
        "dr.",
        "dr_",
        "god",
        "allah",
        "lord",
        "devil",
        "fate",
        "nature",
        "y_allah",
        "y allah",
        "hadji",
        "haji",
        "hajj",
        "maulvi",
        "molvi",
        "imam",
        "sheikh",
        "black maria",
        "homer",
        "longfellow",
        "wordsworth",
        "robert browning",
        "robert louis stevenson",
        "john milton",
        "mark twain",
        "charles dickens",
        "shakespeare",
        "william shakespeare",
        "bernard shaw",
        "shaw",
        "chekov",
        "chekhov",
        "dostoevsky",
        "gogol",
        "gorki",
        "emile zola",
        "sinclair lewis",
        "steinbeck",
        "jack london",
        "damon runyon",
        "tarzan",
        "buck rogers",
        "buck jones",
        "hopalong cassidy",
        "roy rogers",
        "gene autry",
        "bob steele",
        "cobra woman",
        "brick bradford",
        "globi",
        "ali baba",
        "tsotsi",
        "banquo",
        "mephistopheles",
        "dorian gray",
        "pharaoh cheops",
        "hunchback of notre dame",
        "goofy",
        "captain america",
        "captain marvel",
        "captain britain",
        "superman",
        "batman",
        "spiderman",
        "spider-man",
        "hamlet",
        "cassandra",
        "mommy",
        "mummy",
        "then",
        "tb",
        "cac",
        "gandhian",
        "berlin hitler",
        "mom ayesha",
        "european native coloured indian malay griqua",
        "lot",
    ];
    const ROLE_PREFIXES: &[&str] = &[
        "uncle ",
        "auntie ",
        "aunt ",
        "granny ",
        "gran ",
        "grandpa ",
        "grandma ",
        "grandfather ",
        "grandmother ",
        "sis ",
        "boeta ",
        "boetie ",
    ];
    const SENTENCE_STARTERS: &[&str] = &[
        "when", "where", "while", "that", "this", "those", "these", "what", "which", "who", "whom",
        "whose", "how", "why", "if", "although", "because", "since", "after", "before", "as",
        "and", "but", "or", "nor", "so", "yet", "for", "the", "a", "an",
    ];
    const TRAILING_JUNK: &[&str] = &[
        "please", "thank", "thanks", "yes", "no", "too", "also", "only", "said", "asked",
        "replied", "told", "wrote", "was", "is", "are", "the", "a", "an", "and", "but", "or",
        "for", "to", "of", "in", "on", "at", "with", "from", "by", "as", "his", "her", "their",
    ];

    let name_lc = raw.to_lowercase();
    let name_lc = name_lc.trim();
    if GENERIC_ROLE_BLOCKLIST.contains(&name_lc) {
        return None;
    }
    let word_count = name_lc.split_whitespace().count();
    // The extraction prompt instructs ≤5 words; anything over 7 is an NER phrase-merge
    // artifact where the LLM concatenated a list of names into one entity.
    if word_count > 7 {
        return None;
    }
    if word_count <= 3 && ROLE_PREFIXES.iter().any(|p| name_lc.starts_with(p)) {
        return None;
    }
    let first_word = name_lc.split_whitespace().next().unwrap_or("");
    if SENTENCE_STARTERS.contains(&first_word) {
        return None;
    }

    // OCR underscore normalisation: _Word_ → (Word), _s → 's, M_ → M., else strip
    let normalised = {
        let paren_fixed = {
            let mut result = raw.to_string();
            loop {
                let b = result.as_bytes().to_vec();
                let mut found: Option<(usize, usize)> = None;
                let mut ii = 0;
                while ii < b.len() {
                    if b[ii] == b'_' && (ii == 0 || b[ii - 1] == b' ') {
                        let mut jj = ii + 1;
                        while jj < b.len() {
                            if b[jj] == b'_'
                                && jj > ii + 1
                                && (jj + 1 >= b.len() || b[jj + 1] == b' ')
                            {
                                found = Some((ii, jj));
                                break;
                            }
                            jj += 1;
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                    ii += 1;
                }
                match found {
                    Some((open, close)) => {
                        let content = result[open + 1..close].to_string();
                        result =
                            format!("{}({}){}", &result[..open], content, &result[close + 1..]);
                    }
                    None => break,
                }
            }
            result
        };
        let chars: Vec<char> = paren_fixed.chars().collect();
        let n = chars.len();
        let mut s = String::with_capacity(paren_fixed.len());
        let mut i = 0;
        while i < n {
            let c = chars[i];
            if c == '_' {
                if i + 1 < n && chars[i + 1] == 's' {
                    let after = i + 2;
                    if after >= n || !chars[after].is_alphabetic() || chars[after].is_uppercase() {
                        s.push('\'');
                        s.push('s');
                        i += 2;
                        continue;
                    }
                }
                let prev_alpha = s.chars().last().map(|p| p.is_alphabetic()).unwrap_or(false);
                let next_break = i + 1 >= n || chars[i + 1] == ' ' || chars[i + 1].is_uppercase();
                if prev_alpha && next_break {
                    s.push('.');
                }
            } else {
                s.push(c);
            }
            i += 1;
        }
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    };

    // Strip possessives
    let after_poss = normalised
        .trim_end_matches("'s")
        .trim_end_matches('\u{2019}')
        .trim_end_matches("s'")
        .trim()
        .to_string();

    // Strip trailing junk words
    let mut clean = after_poss;
    loop {
        let words: Vec<&str> = clean.split_whitespace().collect();
        if words.len() <= 1 {
            break;
        }
        let last = words.last().unwrap().to_lowercase();
        if TRAILING_JUNK.contains(&last.as_str()) {
            let trim_to = clean.len() - words.last().unwrap().len();
            clean = clean[..trim_to].trim_end().to_string();
        } else {
            break;
        }
    }

    if clean.is_empty() {
        None
    } else {
        Some(clean)
    }
}

// ── Entity-centric extraction ─────────────────────────────────────────────────

/// Build the text window for a single chunk center with adjacent context.
fn window_text(chunks: &[Chunk], center: usize, window: usize) -> String {
    if window == 0 {
        return chunks[center].text.clone();
    }
    let start = center.saturating_sub(window);
    let end = (center + window + 1).min(chunks.len());
    let center_type = &chunks[center].section_type;
    chunks[start..end]
        .iter()
        .filter(|c| center_type.same_window_zone(&c.section_type))
        .map(|c| c.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n[...]\n\n")
}

/// Entity-centric extraction.
///
/// Phase 1 — GLiNER scans every chunk window to build `name → [chunk_indices]`.
/// Phase 2 — For each unique name, aggregate up to MAX_SOURCE_CHUNKS distinct
///            chunk windows, make one focused LLM call, and store the result.
///
/// Requires `--gliner-url`. Prints comparison metrics on completion.
#[allow(clippy::type_complexity)]
async fn extract_entity_centric(
    chunks: &[Chunk],
    _chunk_ids: &[i64],
    embed: &EmbedClient,
    graph_cfg: &GraphIngestConfig,
    progress: Option<Arc<dyn Fn(usize, usize, usize, usize) + Send + Sync>>,
) {
    const MAX_SOURCE_CHUNKS: usize = 3;

    let gliner = match graph_cfg.gliner_client.as_ref() {
        Some(g) => g,
        None => {
            warn!("--entity-centric requires --gliner-url; aborting entity-centric run");
            return;
        }
    };

    let total = chunks.len();
    let cw = graph_cfg.context_window;

    // ── Phase 1: GLiNER scan ──────────────────────────────────────────────────
    let mut entity_to_chunks: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();
    for (i, _) in chunks.iter().enumerate() {
        let text = window_text(chunks, i, cw);
        for span in gliner.person_spans(&text).await {
            entity_to_chunks.entry(span).or_default().push(i);
        }
    }
    let unique_names: Vec<(String, Vec<usize>)> = entity_to_chunks.into_iter().collect();
    let n_unique = unique_names.len();
    info!(
        "entity-centric phase 1: {} unique spans in {} chunks",
        n_unique, total
    );
    println!("  EC phase 1: {n_unique} unique GLiNER spans → one LLM call each");

    // ── Phase 2: per-entity LLM calls ────────────────────────────────────────
    let urls = Arc::new(graph_cfg.effective_urls());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let model = Arc::new(graph_cfg.model.clone());
    let store = graph_cfg.store.clone();
    let workers = graph_cfg.workers.max(1);
    let no_relations = graph_cfg.no_relations;
    let et_owned: Arc<Vec<String>> = Arc::new(graph_cfg.entity_types.clone());

    let llm_calls = Arc::new(AtomicUsize::new(0));
    let context_chars = Arc::new(AtomicUsize::new(0));

    let sem = Arc::new(tokio::sync::Semaphore::new(workers));
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ChunkResult>(workers.max(4) * 4);

    // Drain: build EntityNode from each extracted entity and upsert into graph.
    let drain_store = store.clone();
    let done_ctr = Arc::new(AtomicUsize::new(0));
    let drain = {
        let done_ctr = done_ctr.clone();
        tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                let mut g = match drain_store.lock() {
                    Ok(g) => g,
                    Err(_) => {
                        warn!("graph mutex poisoned");
                        continue;
                    }
                };
                for (extracted, emb) in res.entities.iter().zip(res.embeddings.iter()) {
                    let Some(clean_name) = clean_extracted_name(&extracted.name) else {
                        continue;
                    };
                    let fields: std::collections::HashMap<String, crate::graph::FieldValue> =
                        extracted
                            .fields
                            .iter()
                            .map(|(k, v)| {
                                (
                                    k.clone(),
                                    crate::graph::FieldValue {
                                        value: v.clone(),
                                        evidence_chunk_ids: vec![],
                                        confidence: 1.0,
                                    },
                                )
                            })
                            .collect();
                    let description = {
                        let from_fields = crate::graph::description_from_fields(
                            &clean_name,
                            &extracted.entity_type,
                            &fields,
                        );
                        if from_fields.is_empty() {
                            extracted.description.clone()
                        } else {
                            from_fields
                        }
                    };
                    let eid = crate::graph::entity_id(&clean_name, &extracted.entity_type);
                    let node = crate::graph::EntityNode {
                        id: eid,
                        name: clean_name.clone(),
                        entity_type: extracted.entity_type.clone(),
                        description,
                        embedding: emb.clone(),
                        mention_count: 1,
                        first_chunk_id: res.chunk_id,
                        aliases: vec![],
                        schema_type: None,
                        evidence: vec![],
                        gender: None,
                        fields,
                        confidence: 0.0,
                        extraction_confidence: extracted.extraction_confidence,
                    };
                    if let Err(e) = g.upsert_entity(node) {
                        warn!("ec upsert: {e}");
                    }
                }
                let done = done_ctr.fetch_add(1, Ordering::Relaxed) + 1;
                if let Some(ref cb) = progress {
                    cb(done, n_unique, g.node_count(), g.relation_count());
                }
            }
        })
    };

    for (entity_name, source_indices) in unique_names {
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        let urls = urls.clone();
        let url_counter = url_counter.clone();
        let model = model.clone();
        let embed = embed.clone();
        let llm_calls = llm_calls.clone();
        let context_chars = context_chars.clone();
        let et_owned = et_owned.clone();

        // Aggregate up to MAX_SOURCE_CHUNKS distinct windows
        let mut seen = std::collections::BTreeSet::new();
        for &ci in source_indices.iter().take(MAX_SOURCE_CHUNKS) {
            let start = ci.saturating_sub(cw);
            let end = (ci + cw + 1).min(total);
            for idx in start..end {
                seen.insert(idx);
            }
        }
        let context: String = seen
            .iter()
            .map(|&ci| chunks[ci].text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n[...]\n\n");
        let ctx_len = context.len();

        tokio::spawn(async move {
            let _permit = permit;
            let url = &urls[url_counter.fetch_add(1, Ordering::Relaxed) % urls.len()];

            llm_calls.fetch_add(1, Ordering::Relaxed);
            context_chars.fetch_add(ctx_len, Ordering::Relaxed);

            let candidates = vec![entity_name.clone()];
            let hints = vec![entity_name.clone()];
            let et_refs: Vec<&str> = et_owned.iter().map(|s| s.as_str()).collect();

            let (mut entities, _) = match extract_from_text(
                &context,
                &candidates,
                &[],
                None,
                url,
                &model,
                &et_refs,
                no_relations,
                Some(&hints),
            )
            .await
            {
                Ok(r) => r,
                Err(e) => {
                    warn!("ec extract error for '{entity_name}': {e}");
                    let _ = tx
                        .send(ChunkResult {
                            chunk_id: 0,
                            entities: vec![],
                            relations: vec![],
                            embeddings: vec![],
                        })
                        .await;
                    return;
                }
            };

            let texts: Vec<String> = entities
                .iter()
                .map(|e| format!("{}: {}", e.name, e.description))
                .collect();
            let embeddings = match embed
                .embed_batch(&texts.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    warn!("ec embed error: {e}");
                    vec![]
                }
            };
            entities.truncate(embeddings.len());

            let _ = tx
                .send(ChunkResult {
                    chunk_id: 0,
                    entities,
                    relations: vec![],
                    embeddings,
                })
                .await;
        });
    }

    drop(tx);
    let _ = drain.await;

    let calls = llm_calls.load(Ordering::Relaxed);
    let chars = context_chars.load(Ordering::Relaxed);
    let avg_ctx = chars.checked_div(calls).unwrap_or(0);
    let entity_count = store.lock().map(|g| g.node_count()).unwrap_or(0);
    info!(
        "entity-centric complete: {} calls, {} avg ctx chars, {} entities",
        calls, avg_ctx, entity_count
    );
    println!(
        "  EC metrics: {} LLM calls  |  {} avg context chars  |  {} entities",
        calls, avg_ctx, entity_count
    );
}

// ── EC Refinement pass ────────────────────────────────────────────────────────

/// After a CC build, escalate low-confidence entities for a focused EC second pass.
///
/// For each entity whose confidence < cfg.ec_refine_threshold:
///   1. Collect the chunk IDs from entity.evidence, map to chunk indices.
///   2. Aggregate up to 3 source-chunk windows into a person-centric dossier.
///   3. Call extract_entity_refinement() with a focused "fill in missing fields" prompt.
///   4. Merge the improved entity back via upsert_entity(); re-score confidence.
async fn refine_low_confidence_entities(
    chunks: &[Chunk],
    chunk_ids: &[i64],
    embed: &EmbedClient,
    cfg: &GraphIngestConfig,
) {
    const MAX_SOURCE_CHUNKS: usize = 3;

    // Build chunk-db-id → index map for context window lookup.
    let id_to_index: std::collections::HashMap<i64, usize> = chunk_ids
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();

    let total = chunks.len();
    let cw = cfg.context_window;
    let threshold = cfg.ec_refine_threshold;
    let budget = cfg.ec_refine_budget.max(1);

    // Collect targets: low-confidence entities matching our entity_types filter.
    let targets: Vec<(i64, String, String, Vec<i64>, f32)> = {
        let g = cfg.store.lock().unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        });
        let mut v: Vec<_> = g
            .all_entities()
            .filter(|n| {
                // Target entities that are structurally incomplete (low completeness score)
                // OR were extracted with low LLM confidence (possible hallucination).
                (n.confidence < threshold || n.extraction_confidence < 0.5)
                    && (cfg.entity_types.is_empty()
                        || cfg
                            .entity_types
                            .iter()
                            .any(|t| t.eq_ignore_ascii_case(&n.entity_type)))
            })
            .map(|n| {
                (
                    n.id,
                    n.name.clone(),
                    n.entity_type.clone(),
                    n.evidence.clone(),
                    n.confidence,
                )
            })
            .collect();
        // Lowest confidence first so the budget targets the weakest entities.
        v.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(std::cmp::Ordering::Equal));
        v.truncate(budget);
        v
    };

    if targets.is_empty() {
        println!("  EC refinement: 0 entities below threshold {threshold:.2}");
        return;
    }
    println!(
        "  EC refinement: {} entities below threshold {threshold:.2} → escalating (budget={budget})",
        targets.len()
    );

    let urls = Arc::new(cfg.effective_urls());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let model = Arc::new(cfg.model.clone());
    let workers = cfg.workers.max(1);
    let sem = Arc::new(tokio::sync::Semaphore::new(workers));
    let et_owned: Arc<Vec<String>> = Arc::new(cfg.entity_types.clone());

    let mut improved = 0usize;
    let mut confidence_delta_sum = 0f32;
    let initial_entity_count = cfg
        .store
        .lock()
        .unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        })
        .node_count();

    // Sequential refinement (EC calls are already expensive; no need to parallelize at budget=50).
    for (target_id, entity_name, _entity_type, evidence, old_conf) in &targets {
        // Build aggregated context from evidence chunks.
        let source_indices: Vec<usize> = evidence
            .iter()
            .filter_map(|cid| id_to_index.get(cid).copied())
            .collect();

        if source_indices.is_empty() {
            // Entity has no chunk evidence in this corpus slice — skip.
            continue;
        }

        let mut seen = std::collections::BTreeSet::new();
        for &ci in source_indices.iter().take(MAX_SOURCE_CHUNKS) {
            let start = ci.saturating_sub(cw);
            let end = (ci + cw + 1).min(total);
            for idx in start..end {
                seen.insert(idx);
            }
        }
        let context: String = seen
            .iter()
            .map(|&ci| chunks[ci].text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n[...]\n\n");

        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let url = &urls[url_counter.fetch_add(1, Ordering::Relaxed) % urls.len()];

        // Reuse extract_from_text with a single candidate + hint for focused extraction.
        let candidates = vec![entity_name.clone()];
        let hints = vec![entity_name.clone()];
        let et_refs: Vec<&str> = et_owned.iter().map(|s| s.as_str()).collect();
        let (mut entities, _) = match extract_from_text(
            &context,
            &candidates,
            &[],
            None,
            url,
            &model,
            &et_refs,
            true, // no_relations — refinement is field-enrichment only
            Some(&hints),
        )
        .await
        {
            Ok(r) => r,
            Err(e) => {
                warn!("EC refinement error for '{}': {e}", entity_name);
                drop(permit);
                continue;
            }
        };
        drop(permit);

        // Filter by entity type and blocklist.
        entities.retain(|e| {
            et_owned.is_empty()
                || et_owned
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case(&e.entity_type))
        });

        for extracted in &entities {
            let Some(clean_name) = clean_extracted_name(&extracted.name) else {
                continue;
            };
            let eid = crate::graph::entity_id(&clean_name, &extracted.entity_type);

            // Embed the refined entity.
            let embed_text = format!("{}: {}", clean_name, extracted.description);
            let embedding = match embed.embed_batch(&[embed_text.as_str()]).await {
                Ok(mut v) => v.pop().unwrap_or_default(),
                Err(e) => {
                    warn!("EC refinement embed error: {e}");
                    continue;
                }
            };

            let fields: std::collections::HashMap<String, crate::graph::FieldValue> = extracted
                .fields
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        crate::graph::FieldValue {
                            value: v.clone(),
                            evidence_chunk_ids: vec![],
                            confidence: 1.0,
                        },
                    )
                })
                .collect();

            let description = {
                let from_fields = crate::graph::description_from_fields(
                    &clean_name,
                    &extracted.entity_type,
                    &fields,
                );
                if from_fields.is_empty() {
                    extracted.description.clone()
                } else {
                    from_fields
                }
            };

            let node = crate::graph::EntityNode {
                id: eid,
                name: clean_name.clone(),
                entity_type: extracted.entity_type.clone(),
                description,
                embedding,
                mention_count: 1,
                first_chunk_id: 0,
                aliases: vec![],
                schema_type: None,
                evidence: vec![],
                gender: None,
                fields,
                confidence: 0.0, // will be re-scored below
                extraction_confidence: extracted.extraction_confidence,
            };

            let mut g = cfg.store.lock().unwrap_or_else(|e| {
                warn!("graph store mutex was poisoned; recovering inner value");
                e.into_inner()
            });
            if let Err(e) = g.upsert_entity(node) {
                warn!("EC refinement upsert error: {e}");
            }
        }

        // Re-score the target entity to measure improvement.
        let new_conf = {
            let mut g = cfg.store.lock().unwrap_or_else(|e| {
                warn!("graph store mutex was poisoned; recovering inner value");
                e.into_inner()
            });
            g.rescore_entity(*target_id)
        };

        if new_conf > *old_conf + 0.01 {
            improved += 1;
            confidence_delta_sum += new_conf - old_conf;
        }
    }

    // Persist updated confidence scores.
    {
        let mut g = cfg.store.lock().unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        });
        if let Err(e) = g.score_all_confidences() {
            warn!("EC refinement confidence persist failed: {e}");
        }
    }

    let final_entity_count = cfg
        .store
        .lock()
        .unwrap_or_else(|e| {
            warn!("graph store mutex was poisoned; recovering inner value");
            e.into_inner()
        })
        .node_count();
    let new_entities = final_entity_count.saturating_sub(initial_entity_count);
    let avg_delta = if improved > 0 {
        confidence_delta_sum / improved as f32
    } else {
        0.0
    };
    println!(
        "  EC refinement done: {}/{} existing entities improved (avg confidence ↑ +{avg_delta:.2}), {} new entities discovered",
        improved,
        targets.len(),
        new_entities,
    );
}
