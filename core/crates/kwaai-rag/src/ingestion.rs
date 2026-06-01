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
    let total = chunks.len();
    let urls = Arc::new(graph_cfg.effective_urls());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let workers = graph_cfg.workers.max(1);
    let model = Arc::new(graph_cfg.model.clone());
    let entity_types_cfg = Arc::new(graph_cfg.entity_types.clone());
    let no_relations = graph_cfg.no_relations;
    let context_window = graph_cfg.context_window;
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
                // Drop generic family roles, pronouns, collective labels, and common words
                // that slip through the LLM prompt despite the extraction rules.
                const GENERIC_ROLE_BLOCKLIST: &[&str] = &[
                    // Generic family roles
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
                    // Pronouns / self-reference
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
                    // Political/ideological concepts extracted as persons
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
                    // Ethnic/national/religious group nouns (not individuals)
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
                    // Ideological / political labels (not persons)
                    "socialist",
                    "marxist",
                    "labour",
                    "communist",
                    "fascist",
                    "nazi",
                    "nats",
                    "native",
                    "bantu",
                    // Abstract concepts mistakenly extracted as persons
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
                    // Common English words mistakenly extracted as persons
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
                    // Single-word extraction artifacts and garbage tokens
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
                    // Single-word names that are too ambiguous (surname-only, nickname-only)
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
                    // Academic subjects / objects mistaken for persons
                    "zoology",
                    "cadbury",
                    "freubel",
                    // Ethnic group phrases not caught by exact-word check
                    "south african indian",
                    "head of british muslims",
                    "non-white councillors",
                    // Standalone title/role (no name attached)
                    "prof",
                    "prof.",
                    "prof_",
                    // Plural family/group names (not individuals)
                    "gools",
                    "rassools",
                    "goldings",
                    "killers",
                    "stranglers",
                    "royal family",
                    // Possessives / corrupted text artifacts
                    "mr.",
                    "mr_",
                    "rev.",
                    "rev_",
                    "dr.",
                    "dr_",
                    // Abstract / non-human concepts
                    "god",
                    "allah",
                    "lord",
                    "devil",
                    "fate",
                    "nature",
                    "y_allah",
                    "y allah",
                    // Islamic honorifics extracted as standalone entities
                    "hadji",
                    "haji",
                    "hajj",
                    "maulvi",
                    "molvi",
                    "imam",
                    "sheikh",
                    // Vehicles / objects mistaken for persons
                    "black maria",
                    // Literary authors (only referenced as writers of books/plays)
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
                    // Fictional characters from comics, films, novels
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
                    // More family role variants
                    "mommy",
                    "mummy",
                    // Common words / abbreviations extracted as persons
                    "then",
                    "tb",
                    "cac",
                    "gandhian",
                    // Fused bad extractions
                    "berlin hitler",
                    "mom ayesha",
                    // Long list-string artifacts
                    "european native coloured indian malay griqua",
                ];
                let name_lc = extracted.name.to_lowercase();
                let name_lc = name_lc.trim();
                if GENERIC_ROLE_BLOCKLIST.contains(&name_lc) {
                    continue;
                }
                // Drop names that start with a family role prefix but have no true surname —
                // "Uncle Aity", "Auntie Cissie", "Granny Bibi" are role-addressed individuals,
                // not canonical entity names.  We allow 4+ word names (e.g. "Aunty Minnie
                // Amina Gool") through so that compound proper names are not lost.
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
                let word_count = name_lc.split_whitespace().count();
                if word_count <= 3 && ROLE_PREFIXES.iter().any(|p| name_lc.starts_with(p)) {
                    continue;
                }
                // Drop entities whose name starts with a sentence-opening word —
                // these are extraction artifacts where the LLM grabbed a phrase
                // fragment ("When Auntie Jolly", "That Mr Smith", etc.).
                const SENTENCE_STARTERS: &[&str] = &[
                    "when", "where", "while", "that", "this", "those", "these", "what", "which",
                    "who", "whom", "whose", "how", "why", "if", "although", "because", "since",
                    "after", "before", "as", "and", "but", "or", "nor", "so", "yet", "for", "the",
                    "a", "an",
                ];
                let first_word = name_lc.split_whitespace().next().unwrap_or("");
                if SENTENCE_STARTERS.contains(&first_word) {
                    continue;
                }
                // Normalise OCR underscore artifacts in entity names returned by the LLM.
                // In this corpus `_` replaces `.` in initials (J_ M_ H_ → J. M. H.).
                let name_normalised = {
                    let mut s = extracted.name.replace("_ ", ". ");
                    if s.ends_with('_') {
                        s.pop();
                        s.push('.');
                    }
                    s
                };
                // Strip trailing possessive 's / 's from entity names so
                // "Ebrahim's" and "Khalifa's" don't persist as separate entities.
                let clean_name = name_normalised
                    .trim_end_matches("'s")
                    .trim_end_matches("\u{2019}s") // right single quotation mark
                    .trim_end_matches("s'") // plural possessive
                    .trim()
                    .to_string();
                let extracted_name = if clean_name.is_empty() {
                    name_normalised.as_str()
                } else {
                    &clean_name
                };
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
        let g = store.lock().unwrap();
        g.all_entities()
            .filter(|e| e.entity_type == "Person")
            .map(|e| (e.name.clone(), e.gender.clone()))
            .collect::<Vec<_>>()
    });

    // Spawn one extraction task per chunk; semaphore caps concurrency.
    for (i, (chunk, &chunk_id)) in chunks.iter().zip(chunk_ids.iter()).enumerate() {
        if chunk.skip_extraction {
            debug!(
                chunk_id,
                section = ?chunk.section_name,
                "skipping extraction for flagged section"
            );
            continue;
        }
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        // Build context text: current chunk plus up to context_window adjacent chunks.
        // Adjacent chunks provide surrounding narrative context that improves entity
        // identification by ~7pp recall (experiments on D6 memoir, 2026-05).
        let text = if context_window > 0 {
            let start = i.saturating_sub(context_window);
            let end = (i + context_window + 1).min(total);
            chunks[start..end]
                .iter()
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

        tokio::spawn(async move {
            let _permit = permit;
            let idx = url_counter.fetch_add(1, Ordering::Relaxed) % urls.len();
            let url = &urls[idx];
            let et: Vec<&str> = entity_types_cfg.iter().map(|s| s.as_str()).collect();

            let candidates = ner::extract_proper_noun_candidates(&text);
            let pronoun_map = ner::resolve_pronouns(&text, &gender_context);

            let (mut entities, relations) = match extract_from_text(
                &text,
                &candidates,
                &pronoun_map,
                section_note.as_deref(),
                url,
                &model,
                &et,
                no_relations,
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
            chunks[start..end]
                .iter()
                .map(|c| c.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n[...]\n\n")
        } else {
            chunk.text.clone()
        };
        let et: Vec<&str> = graph_cfg.entity_types.iter().map(|s| s.as_str()).collect();
        let candidates = ner::extract_proper_noun_candidates(&text);
        let (mut entities, relations) = match extract_from_text(
            &text,
            &candidates,
            &[],
            chunk.section_note.as_deref(),
            &graph_cfg.inference_url,
            &graph_cfg.model,
            &et,
            graph_cfg.no_relations,
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
