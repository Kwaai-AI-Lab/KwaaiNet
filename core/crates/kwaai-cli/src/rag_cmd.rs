use std::io::{self, BufRead, Write as _};
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use anyhow::{bail, Context, Result};
use libp2p::PeerId;
use uuid::Uuid;

use kwaai_rag::{
    cache::QueryCache,
    document,
    embedder::EmbedClient,
    family,
    graph::GraphStore,
    ingestion::{ingest_text, GraphIngestConfig, IngestConfig},
    iterative::retrieve_iterative,
    meta_store::{MetaStore, SyncMeta},
    prompt::{build_chat_messages, ChatMessage},
    retriever::{retrieve_graph_anchored, retrieve_hybrid, RetrieveConfig},
    seed_json,
};

use crate::cli::{CacheAction, DreamAction, GraphAction, RagAction, RagArgs, TimelineAction};
use crate::config::{KwaaiNetConfig, RagConfig};
use crate::display::*;

fn parse_graph_mode(s: &str) -> kwaai_rag::query_understand::GraphMode {
    match s.to_lowercase().as_str() {
        "prepend" => kwaai_rag::query_understand::GraphMode::Prepend,
        "replace" => kwaai_rag::query_understand::GraphMode::Replace,
        _ => kwaai_rag::query_understand::GraphMode::Inject,
    }
}

fn parse_classify_method(s: &str) -> kwaai_rag::query_understand::ClassifyMethod {
    match s.to_lowercase().as_str() {
        "llm" => kwaai_rag::query_understand::ClassifyMethod::Llm,
        "hybrid" => kwaai_rag::query_understand::ClassifyMethod::Hybrid,
        _ => kwaai_rag::query_understand::ClassifyMethod::Rule,
    }
}

#[cfg(feature = "storage")]
use crate::storage_rpc::{
    http_delete_vectors, http_search_vectors, http_upload_vectors, rpc_delete_vectors,
    rpc_search_vectors, rpc_upload_vectors,
};

pub async fn run(args: RagArgs) -> Result<()> {
    match args.action {
        RagAction::Init {
            kb,
            embed_model,
            rag_dir,
            graph,
        } => cmd_init(kb, embed_model, rag_dir, graph).await,

        RagAction::List => cmd_list().await,

        RagAction::ConnectEve { peer_id, url, kb } => cmd_connect_eve(peer_id, url, kb).await,

        RagAction::Ingest {
            file,
            doc_name,
            chunk_size,
            chunk_overlap,
            min_chunk_len,
            extract_entities,
            inference_url,
            extraction_model,
            chunk_strategy,
            surr_mode,
            doc_meta,
            doc_schema,
            kb,
        } => {
            cmd_ingest(
                file,
                doc_name,
                chunk_size,
                chunk_overlap,
                min_chunk_len,
                extract_entities,
                inference_url,
                extraction_model,
                chunk_strategy,
                surr_mode,
                doc_meta,
                doc_schema,
                kb,
            )
            .await
        }

        RagAction::Query {
            text,
            top_k,
            min_score,
            json,
            kb,
            understand,
            inference_url,
            mode,
            model,
            hyde,
            hyde_alpha,
            rerank,
            graph_mode,
            query_classify,
        } => {
            cmd_query(
                text,
                top_k,
                min_score,
                json,
                kb,
                understand,
                inference_url,
                mode,
                model,
                hyde,
                hyde_alpha,
                rerank,
                graph_mode,
                query_classify,
            )
            .await
        }

        RagAction::Chat {
            top_k,
            inference_url,
            kb,
            understand,
            model,
            hyde,
            hyde_alpha,
            rerank,
            mode,
        } => {
            cmd_chat(
                top_k,
                inference_url,
                kb,
                understand,
                model,
                hyde,
                hyde_alpha,
                rerank,
                mode,
            )
            .await
        }

        RagAction::Docs { kb } => cmd_docs(kb).await,

        RagAction::DeleteDoc { name, yes, kb } => cmd_delete_doc(name, yes, kb).await,

        RagAction::Destroy { yes, kb } => cmd_destroy(yes, kb).await,

        RagAction::Rebuild {
            file,
            kb,
            embed_model,
            inference_urls,
            model,
            workers,
            seed_file,
            chunk_strategy,
            doc_meta,
            doc_schema,
            entity_types,
            no_relations,
            graph_window,
            sample_pct,
            yes,
        } => {
            cmd_rebuild(
                file,
                kb,
                embed_model,
                inference_urls,
                model,
                workers,
                seed_file,
                chunk_strategy,
                doc_meta,
                doc_schema,
                entity_types,
                no_relations,
                graph_window,
                sample_pct,
                yes,
            )
            .await
        }

        RagAction::Serve {
            port,
            inference_url,
            top_k,
            kb,
        } => crate::rag_api::run(port, inference_url, top_k, kb).await,

        RagAction::Sync {
            folder,
            extensions,
            delete,
            watch,
            interval,
            chunk_size,
            chunk_overlap,
            min_chunk_len,
            extract_entities,
            inference_url,
            extraction_model,
            chunk_strategy,
            surr_mode,
            doc_meta,
            kb,
        } => {
            cmd_sync(
                folder,
                extensions,
                delete,
                watch,
                interval,
                chunk_size,
                chunk_overlap,
                min_chunk_len,
                extract_entities,
                inference_url,
                extraction_model,
                chunk_strategy,
                surr_mode,
                doc_meta,
                kb,
            )
            .await
        }

        RagAction::Graph { action, kb } => cmd_graph(action, kb).await,

        RagAction::Cache { action, kb } => cmd_cache(action, kb).await,

        RagAction::Dream { action, kb } => cmd_dream(action, kb).await,

        RagAction::Summarize {
            kb,
            inference_url,
            model,
            window_size,
            reset,
        } => cmd_summarize(kb, inference_url, model, window_size, reset).await,

        RagAction::Eval {
            questions,
            kb,
            inference_url,
            model,
            top_k,
            mode,
            hyde,
            hyde_alpha,
            rerank,
            understand,
            llm_judge,
            judge_model,
            output,
            progress_file,
            graph_mode,
            query_classify,
            summary_expansion,
            biographical_expansion,
        } => {
            cmd_eval(
                questions,
                kb,
                inference_url,
                model,
                top_k,
                mode,
                hyde,
                hyde_alpha,
                rerank,
                understand,
                llm_judge,
                judge_model,
                output,
                progress_file,
                graph_mode,
                query_classify,
                summary_expansion,
                biographical_expansion,
            )
            .await
        }
    }
}

// ── init ──────────────────────────────────────────────────────────────────────

async fn cmd_init(
    name: String,
    embed_model: String,
    rag_dir: Option<std::path::PathBuf>,
    graph: bool,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature. Rebuild with: cargo build --features storage");

    #[cfg(feature = "storage")]
    {
        print_box_header(&format!("RAG Init ({})", name));

        let data_dir = rag_dir
            .clone()
            .unwrap_or_else(|| RagConfig::default_data_dir_for(&name));
        // For non-default KBs, always persist the resolved path so data_dir() works
        // correctly when the KB name is not available (e.g., in load_rag_config_for).
        let rag_data_dir_str = if name == "default" {
            rag_dir.as_ref().map(|p| p.to_string_lossy().into_owned())
        } else {
            Some(data_dir.to_string_lossy().into_owned())
        };

        // Probe embedding model before touching storage — auto-detect dimension.
        let embed = EmbedClient::new(None, Some(embed_model.clone()));
        print_info(&format!("Probing Ollama ({embed_model})…"));
        let embed_dim = embed.probe_dim().await.map_err(|e| {
            let msg = e.to_string();
            if msg.contains("404") || msg.contains("not found") {
                anyhow::anyhow!(
                    "Embedding model '{}' is not available in Ollama.\n\
                     Pull it first:  ollama pull {}\n\
                     Other supported models: ollama pull mxbai-embed-large\n\
                     Then re-run:    kwaainet rag init --name {} --embed-model {}",
                    embed_model,
                    embed_model,
                    name,
                    embed_model
                )
            } else if msg.contains("Connection refused") || msg.contains("connect") {
                anyhow::anyhow!(
                    "Cannot reach Ollama — is it running?\n  Start it with: ollama serve"
                )
            } else {
                e
            }
        })?;
        print_success(&format!("Embedding model OK ({embed_dim} dimensions)"));

        // If already initialised, verify the tenant exists in the DB (idempotent).
        // If the DB was wiped or rebuilt from an old format, fall through to recreate.
        let existing_cfg = KwaaiNetConfig::load_or_create()?;
        if let Some(existing_rag) = existing_cfg.get_rag_kb(&name) {
            if existing_rag.storage_url.as_deref() == Some("local") {
                if let Some(ref tid) = existing_rag.tenant_id {
                    let tenant_id: Uuid = tid.parse().context("invalid tenant_id in config")?;
                    let tenant_in_db = if let Ok(db) = kwaai_storage::StorageDb::open(&data_dir) {
                        let tm = kwaai_storage::TenantManager::new(db);
                        tm.get(tenant_id).await.ok().flatten().is_some()
                    } else {
                        false
                    };
                    if tenant_in_db {
                        print_info(&format!(
                            "Knowledge base '{}':  {}",
                            name,
                            data_dir.display()
                        ));
                        print_success(&format!(
                            "Already initialised (tenant {tenant_id}) — embedding model updated."
                        ));
                        let mut cfg = existing_cfg;
                        if let Some(r) = cfg.rag_kbs.get_mut(&name).or(cfg.rag.as_mut()) {
                            r.embed_model = embed_model;
                            if rag_data_dir_str.is_some() {
                                r.rag_data_dir = rag_data_dir_str;
                            }
                        }
                        cfg.save()?;
                        println!("  Next:  kwaainet rag ingest <file> --kb {name}");
                        return Ok(());
                    }
                    print_warning(
                        "Tenant record missing from local DB — recreating knowledge base.",
                    );
                }
            }
        }

        // Fresh init: create local embedded vector store + tenant.
        let db = kwaai_storage::StorageDb::open(&data_dir).context("opening local vector store")?;
        let tm = kwaai_storage::TenantManager::new(db);
        let local_peer_id = crate::identity::NodeIdentity::load_or_create()?.peer_id;
        let info = tm
            .create(
                &local_peer_id.to_base58(),
                0,
                Some(&format!("kwaai-rag/{name}")),
                embed_dim,
            )
            .await
            .context("creating local tenant")?;
        let tenant_id = info.tenant_id;

        MetaStore::open(&data_dir, tenant_id)?;
        print_info(&format!(
            "Knowledge base '{}':  {}",
            name,
            data_dir.display()
        ));

        let mut cfg = KwaaiNetConfig::load_or_create()?;
        cfg.set_rag_kb(
            &name,
            RagConfig {
                tenant_id: Some(tenant_id.to_string()),
                eve_peer_id: None,
                embed_model,
                embed_dim,
                embed_url: None,
                inference_url: "http://localhost:8080".to_string(),
                top_k: 5,
                storage_url: Some("local".to_string()),
                rag_data_dir: rag_data_dir_str,
            },
        );
        cfg.save()?;

        print_success(&format!(
            "Knowledge base '{}' initialised  (tenant {tenant_id})",
            name
        ));
        if graph {
            print_info("Graph extraction ready — use --extract-entities when ingesting");
        }
        if name == "default" {
            println!("  Next:  kwaainet rag ingest <file>");
        } else {
            println!("  Next:  kwaainet rag ingest <file> --kb {name}");
        }
        println!("  Later: kwaainet rag connect-eve <peer-id> --kb {name}  # outsource to Eve");
        Ok(())
    }
}

// ── list ─────────────────────────────────────────────────────────────────────

async fn cmd_list() -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let names = cfg.rag_kb_names();
    if names.is_empty() {
        print_info("No knowledge bases initialised. Run: kwaainet rag init");
        return Ok(());
    }
    print_box_header(&format!("{} knowledge base(s)", names.len()));
    for name in &names {
        if let Some(kb) = cfg.get_rag_kb(name) {
            let storage = match kb.storage_url.as_deref() {
                Some("local") => "local".to_string(),
                Some(url) => format!("HTTP ({url})"),
                None => kb
                    .eve_peer_id
                    .as_deref()
                    .map(|p| format!("P2P ({}…)", &p[..8.min(p.len())]))
                    .unwrap_or_else(|| "unconfigured".to_string()),
            };
            let data_dir = kb.data_dir();
            println!("  • {name}  [{storage}]  {}", data_dir.display());
        }
    }
    Ok(())
}

// ── connect-eve ───────────────────────────────────────────────────────────────

async fn cmd_connect_eve(peer_id: String, url: Option<String>, kb: String) -> Result<()> {
    let eve: PeerId = peer_id.parse().context("invalid peer ID")?;

    let mut cfg = KwaaiNetConfig::load_or_create()?;
    let rag = cfg
        .rag_kbs
        .get_mut(&kb)
        .or(if kb == "default" {
            cfg.rag.as_mut()
        } else {
            None
        })
        .with_context(|| {
            format!("KB '{kb}' not initialised. Run: kwaainet rag init --name {kb}")
        })?;

    rag.eve_peer_id = Some(peer_id);
    // url = Some("http://...") for HTTP transport, None for P2P RPC.
    rag.storage_url = url.clone();
    cfg.save()?;

    print_box_header(&format!("RAG Connect Eve ({})", kb));
    print_success(&format!("Eve: {eve}"));
    if let Some(ref u) = url {
        print_info(&format!("Transport: HTTP ({u})"));
    } else {
        print_info("Transport: P2P (daemon required for operations)");
    }
    print_warning(
        "Existing local vectors are NOT migrated automatically. \
         Re-ingest your documents so the Eve node indexes them.",
    );
    Ok(())
}

// ── ingest ────────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_ingest(
    file: std::path::PathBuf,
    doc_name: Option<String>,
    chunk_size: usize,
    chunk_overlap: usize,
    min_chunk_len: usize,
    extract_entities: bool,
    inference_url: Option<String>,
    extraction_model: String,
    chunk_strategy: String,
    surr_mode: String,
    doc_meta_path: Option<std::path::PathBuf>,
    doc_schema_path: Option<std::path::PathBuf>,
    kb: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;

        let doc_name = doc_name.unwrap_or_else(|| {
            file.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        });

        let text = document::extract_text(&file)?;
        print_info(&format!(
            "Ingesting '{}' ({} chars, {} byte file)",
            doc_name,
            text.chars().count(),
            text.len()
        ));

        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

        // Store document title in graph store for LLM prompt injection
        {
            if let Ok(mut g) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                let _ = g.set_document_titles(std::slice::from_ref(&doc_name));
            }
        }

        let mut cfg = IngestConfig::new(embed);
        cfg.chunk_cfg.chunk_size = chunk_size;
        cfg.chunk_cfg.chunk_overlap = chunk_overlap;
        cfg.chunk_cfg.min_chunk_len = min_chunk_len;
        cfg.chunk_cfg.strategy = parse_chunk_strategy(&chunk_strategy);
        cfg.chunk_cfg.surr_mode = parse_surr_mode(&surr_mode);

        if let Some(path) = doc_meta_path {
            cfg.doc_meta = load_doc_meta(&path)?;
            print_info(&format!("Doc-meta loaded: {} entries", cfg.doc_meta.len()));
        }

        // Doc schema: load from YAML, or auto-detect from the document header.
        let loaded_schema: Option<kwaai_rag::doc_schema::DocSchema> = if let Some(path) =
            doc_schema_path
        {
            let schema = kwaai_rag::doc_schema::load_doc_schema(&path)?;
            let skip_count = schema.sections.iter().filter(|s| s.skip).count();
            let seed_count = schema.sections.iter().filter(|s| s.index_seeds).count();
            let note_count = schema
                .sections
                .iter()
                .filter(|s| s.narrator_note.is_some())
                .count();
            print_info(&format!(
                "Doc-schema loaded: type={} sections={} (skip={}, index_seeds={}, narrator_notes={})",
                schema.schema_type.as_deref().unwrap_or("untyped"),
                schema.sections.len(), skip_count, seed_count, note_count
            ));
            Some(schema)
        } else {
            let preview = &text[..text.len().min(4000)];
            let detected = kwaai_rag::doc_schema::auto_detect_schema(preview);
            if detected.schema_type.is_some() || !detected.metadata.is_empty() {
                print_info(&format!(
                    "Doc-schema auto-detected: type={}, metadata_keys=[{}]",
                    detected.schema_type.as_deref().unwrap_or("unknown"),
                    detected
                        .metadata
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
                Some(detected)
            } else {
                None
            }
        };

        if let Some(ref schema) = loaded_schema {
            // Persist document metadata into the graph store for use at query/dream time.
            if !schema.metadata.is_empty() {
                if let Ok(mut g) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                    let _ = g.set_doc_metadata(&schema.metadata);
                    print_info(&format!(
                        "Doc metadata persisted: {} key(s)",
                        schema.metadata.len()
                    ));
                }
            }
            cfg.doc_schema = Some(schema.clone());
        }

        if extract_entities {
            let infer_url = inference_url
                .clone()
                .unwrap_or_else(|| rag_cfg.inference_url.clone());
            let store =
                GraphStore::open(&rag_cfg.data_dir(), tenant_id).context("opening graph store")?;
            cfg.graph = Some(GraphIngestConfig {
                store: Arc::new(Mutex::new(store)),
                inference_url: infer_url,
                inference_urls: vec![],
                model: extraction_model.clone(),
                workers: 1,
                entity_types: vec![],
                no_relations: false,
                context_window: 1,
                gliner_client: None,
                entity_centric: false,
                chunk_batch: 1,
                ec_refine_threshold: 0.0,
                ec_refine_budget: 50,
                ec_refine_only: false,
            });
            print_info("Entity extraction enabled — knowledge graph will be updated");
        }

        let spinner = crate::progress::Spinner::start("Ingesting…");

        let result = match rag_cfg.storage_url.as_deref() {
            Some("local") => {
                let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
                ingest_text(
                    &cfg,
                    &meta,
                    &doc_name,
                    &text,
                    move |vectors| {
                        let vs = vs.clone();
                        Box::pin(async move { vs.upload(tenant_id, &vectors).await })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    Some(|done: usize, total: usize| {
                        let _ = (done, total);
                    }),
                )
                .await?
            }
            Some(url) => {
                let http = reqwest::Client::new();
                let url = url.to_string();
                ingest_text(
                    &cfg,
                    &meta,
                    &doc_name,
                    &text,
                    move |vectors| {
                        let http = http.clone();
                        let url = url.clone();
                        Box::pin(async move {
                            http_upload_vectors(&http, &url, tenant_id, vectors).await
                        })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    Some(|done: usize, total: usize| {
                        let _ = (done, total);
                    }),
                )
                .await?
            }
            None => {
                let eve_peer_id = eve_peer_id(&rag_cfg)?;
                let (client, _) = crate::vpk::p2p_connect().await?;
                let client = Arc::new(tokio::sync::Mutex::new(client));
                ingest_text(
                    &cfg,
                    &meta,
                    &doc_name,
                    &text,
                    move |vectors| {
                        let client = client.clone();
                        Box::pin(async move {
                            let guard = client.lock().await;
                            rpc_upload_vectors(&guard, &eve_peer_id, tenant_id, vectors).await
                        })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    Some(|done: usize, total: usize| {
                        let _ = (done, total);
                    }),
                )
                .await?
            }
        };

        spinner
            .finish(&format!(
                "✓ Ingested {} chunks  •  {} vectors uploaded",
                result.chunks_ingested, result.vectors_uploaded
            ))
            .await;

        // After ingest: inject entity seeds from index sections.
        if let Some(ref schema) = loaded_schema {
            if schema.has_index_seeds() {
                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                inject_index_seeds(&text, schema, &rag_cfg, tenant_id, &embed).await;
            }
        }

        Ok(())
    }
}

// ── index seed injection ──────────────────────────────────────────────────────

/// Find and return the text of a section whose heading contains `pattern` (case-insensitive).
/// Returns the text from the heading line to the start of the next all-caps heading or EOF.
fn extract_section_text<'a>(full_text: &'a str, pattern: &str) -> Option<&'a str> {
    let lower_pattern = pattern.to_lowercase();
    let mut start_byte: Option<usize> = None;
    let mut end_byte = full_text.len();

    let mut offset = 0usize;
    for line in full_text.lines() {
        let line_lower = line.trim().to_lowercase();
        if start_byte.is_none() {
            if line_lower.contains(&lower_pattern) {
                start_byte = Some(offset);
            }
        } else {
            // Stop at the next standalone all-caps short heading (another section)
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && trimmed.len() < 30
                && trimmed == trimmed.to_uppercase()
                && trimmed.chars().any(|c| c.is_alphabetic())
                && !line_lower.contains(&lower_pattern)
            {
                end_byte = offset;
                break;
            }
        }
        offset += line.len() + 1; // +1 for the newline
    }

    start_byte.map(|s| &full_text[s..end_byte.min(full_text.len())])
}

async fn inject_index_seeds(
    full_text: &str,
    schema: &kwaai_rag::doc_schema::DocSchema,
    rag_cfg: &crate::config::RagConfig,
    tenant_id: Uuid,
    embed: &EmbedClient,
) {
    use kwaai_rag::graph::{entity_id, EntityNode, GraphStore};

    let mut total_added = 0usize;
    let mut total_skipped = 0usize;

    for sec in schema.sections.iter().filter(|s| s.index_seeds) {
        let section_text = match extract_section_text(full_text, &sec.pattern) {
            Some(t) => t,
            None => {
                print_warning(&format!(
                    "Index seeds: section '{}' not found in document text",
                    sec.pattern
                ));
                continue;
            }
        };

        let seeds = kwaai_rag::doc_schema::parse_index_seeds(section_text);
        if seeds.is_empty() {
            continue;
        }

        print_info(&format!(
            "Index seeds: found {} entries in '{}' section",
            seeds.len(),
            sec.pattern
        ));

        // Embed all seed names in one batch
        let names: Vec<&str> = seeds.iter().map(|(n, _)| n.as_str()).collect();
        let embeddings = match embed.embed_batch(&names).await {
            Ok(e) => e,
            Err(e) => {
                print_warning(&format!("Index seed embedding failed: {e}"));
                continue;
            }
        };

        let store = match GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
            Ok(s) => s,
            Err(e) => {
                print_warning(&format!("Could not open graph store for index seeds: {e}"));
                continue;
            }
        };
        let store = std::sync::Arc::new(std::sync::Mutex::new(store));

        for ((name, type_hint), embedding) in seeds.iter().zip(embeddings) {
            let entity_type = type_hint.as_deref().unwrap_or("Person");
            let eid = entity_id(name, entity_type);
            let node = EntityNode {
                id: eid,
                name: name.clone(),
                entity_type: entity_type.to_string(),
                description: String::new(),
                embedding,
                mention_count: 1,
                first_chunk_id: 0,
                aliases: vec![],
                schema_type: None,
                gender: None,
                evidence: vec![],
                fields: Default::default(),
                confidence: 0.0,
                extraction_confidence: 1.0,
            };
            match store.lock() {
                Ok(mut g) => match g.upsert_entity(node) {
                    Ok(_) => total_added += 1,
                    Err(_) => total_skipped += 1,
                },
                Err(_) => total_skipped += 1,
            }
        }
    }

    if total_added > 0 {
        print_success(&format!(
            "Index seeds: injected {total_added} entity seeds into graph ({total_skipped} skipped)"
        ));
    }
}

// ── query ─────────────────────────────────────────────────────────────────────

/// When `rag serve` is running it holds the redb files open.  Try proxying
/// the search through the running server before opening the DB directly.
///
/// Returns:
///   Ok(Some(results)) — server answered /api/search
///   Ok(None)          — server not running (connection refused)
///   Err(_)            — server running but /api/search missing (needs restart)
async fn try_serve_query(
    query: &str,
    top_k: usize,
    min_score: f64,
    port: u16,
) -> Result<Option<Vec<serde_json::Value>>> {
    let url = format!("http://localhost:{port}/api/search");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let resp = match client
        .post(&url)
        .json(&serde_json::json!({"q": query, "top_k": top_k, "min_score": min_score}))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) if e.is_connect() => return Ok(None), // server not running
        Err(e) => return Err(anyhow::anyhow!("{e}")),
    };
    if resp.status().is_success() {
        Ok(Some(resp.json().await?))
    } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
        // Server is running but doesn't have /api/search — needs restart.
        anyhow::bail!(
            "rag serve is running on port {port} but is out of date.\n  \
             Restart it with: kwaainet rag serve"
        )
    } else {
        anyhow::bail!("rag serve returned {}", resp.status())
    }
}

#[allow(clippy::too_many_arguments)]
async fn cmd_query(
    query: String,
    top_k: usize,
    min_score: f64,
    json_out: bool,
    kb: String,
    understand: bool,
    inference_url: Option<String>,
    mode: String,
    model: String,
    hyde: bool,
    hyde_alpha: Option<f32>,
    rerank: bool,
    graph_mode: String,
    query_classify: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let global_cfg = KwaaiNetConfig::load_or_create()?;
        let kb_names: Vec<String> = if kb == "all" {
            global_cfg.rag_kb_names()
        } else {
            vec![kb.clone()]
        };
        if kb_names.is_empty() {
            bail!("No knowledge bases initialised. Run: kwaainet rag init");
        }

        // Single local KB: try proxying through a running `rag serve` first.
        if kb_names.len() == 1 {
            if let Some(rag_cfg) = global_cfg.get_rag_kb(&kb_names[0]) {
                if rag_cfg.storage_url.as_deref() == Some("local") {
                    match try_serve_query(&query, top_k, min_score, 9090).await {
                        Ok(Some(results)) => {
                            return render_query_results(&query, &results, json_out)
                        }
                        Ok(None) => {}
                        Err(e) => return Err(e),
                    }
                }
            }
        }

        let hyde_url = if hyde { inference_url.clone() } else { None };
        let hyde_mdl = if hyde { Some(model.clone()) } else { None };
        let effective_alpha = if hyde {
            Some(hyde_alpha.unwrap_or(0.5))
        } else {
            None
        };
        let retrieve_cfg = RetrieveConfig {
            top_k,
            min_score,
            use_sentence_window: false,
            hyde_inference_url: hyde_url,
            hyde_model: hyde_mdl,
            hyde_alpha: effective_alpha,
            graph_mode: parse_graph_mode(&graph_mode),
            query_classify: parse_classify_method(&query_classify),
            query_multi_hop: false,
            use_summary_expansion: false,
        };
        let mut spinner = if json_out {
            None
        } else {
            Some(crate::progress::Spinner::start("Retrieving…"))
        };

        let mut all_results: Vec<kwaai_rag::retriever::RetrievedChunk> = vec![];

        for kb_name in &kb_names {
            let (rag_cfg, tenant_id) = match load_rag_config_for(kb_name) {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!("skipping KB '{kb_name}': {e}");
                    continue;
                }
            };
            let embed =
                EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
            let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;
            let infer_url = inference_url
                .clone()
                .unwrap_or_else(|| rag_cfg.inference_url.clone());

            // Resolve effective mode: "auto" routes to graph if entities exist.
            let effective_mode = if mode == "auto" {
                if rag_cfg.storage_url.as_deref() == Some("local") {
                    if let Ok(g) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                        if g.node_count() > 0 {
                            "graph"
                        } else {
                            "vector"
                        }
                    } else {
                        "vector"
                    }
                } else {
                    "vector"
                }
            } else {
                mode.as_str()
            };

            let mut chunks = match rag_cfg.storage_url.as_deref() {
                Some("local") => {
                    let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
                    if effective_mode == "smart" {
                        use kwaai_rag::query_understand::{
                            understand_query_rule, GraphMode, QueryIntent,
                        };
                        let qs = understand_query_rule(&query);
                        let is_family_nonauthor =
                            matches!(qs.intent, QueryIntent::FamilyRelation { .. })
                                && !qs.anchor_is_author;
                        let is_family_author =
                            matches!(qs.intent, QueryIntent::FamilyRelation { .. })
                                && qs.anchor_is_author;
                        let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                            .context("opening graph store for smart routing")?;
                        drop(spinner.take());
                        if is_family_nonauthor {
                            let mut smart_cfg = retrieve_cfg.clone();
                            smart_cfg.graph_mode = GraphMode::Replace;
                            retrieve_graph_anchored(
                                &query,
                                &smart_cfg,
                                &embed,
                                &meta,
                                &graph,
                                move |emb, k| {
                                    let vs = vs.clone();
                                    Box::pin(async move {
                                        let raw = vs.search(tenant_id, &emb, k).await?;
                                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                    })
                                        as Pin<
                                            Box<
                                                dyn std::future::Future<
                                                        Output = Result<Vec<(i64, f64)>>,
                                                    > + Send,
                                            >,
                                        >
                                },
                            )
                            .await?
                        } else if is_family_author {
                            // Author-anchored family query: Replace mode injects the resolved
                            // relative's entity description directly (wife/mother use this).
                            // Exception: grandparents use Prepend with a rewritten retrieval
                            // query: "Who was the author's grandfather?" → "Who was Haji Joosub
                            // Maulvi Hamid Gool?" so vector search finds the biographical
                            // chapters (Swat, spice, mosque, Buitencingle) rather than
                            // family-context chunks.
                            let q_lower = query.to_lowercase();
                            let is_grandparent = q_lower.contains("grandfather")
                                || q_lower.contains("grandpa")
                                || q_lower.contains("grandmother")
                                || q_lower.contains("grandma");
                            let mut smart_cfg = retrieve_cfg.clone();
                            smart_cfg.graph_mode = if is_grandparent {
                                GraphMode::Prepend
                            } else {
                                GraphMode::Replace
                            };
                            let retrieval_query: std::borrow::Cow<str> = if is_grandparent {
                                kwaai_rag::retriever::resolve_relative_entity_name(&query, &graph)
                                    .map(|name| {
                                        std::borrow::Cow::Owned(format!("Who was {name}?"))
                                    })
                                    .unwrap_or(std::borrow::Cow::Borrowed(&query))
                            } else {
                                std::borrow::Cow::Borrowed(&query)
                            };
                            retrieve_graph_anchored(
                                &retrieval_query,
                                &smart_cfg,
                                &embed,
                                &meta,
                                &graph,
                                move |emb, k| {
                                    let vs = vs.clone();
                                    Box::pin(async move {
                                        let raw = vs.search(tenant_id, &emb, k).await?;
                                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                    })
                                        as Pin<
                                            Box<
                                                dyn std::future::Future<
                                                        Output = Result<Vec<(i64, f64)>>,
                                                    > + Send,
                                            >,
                                        >
                                },
                            )
                            .await?
                        } else {
                            let is_temporal = matches!(qs.intent, QueryIntent::TemporalEvent);
                            // Temporal routing: hybrid — run iterative retrieval, then prepend the
                            // sequence diagram when timeline data exists. The LLM gets both the
                            // structured timeline (precise dates/events) and the narrative chunks.
                            let seq_chunk = if is_temporal {
                                let eids = kwaai_rag::sequence::extract_temporal_entity_ids(
                                    &query, &graph,
                                );
                                kwaai_rag::sequence::retrieve_sequence(&query, &eids, &graph)
                            } else {
                                None
                            };
                            let mut chunks = retrieve_iterative(
                                &query,
                                &retrieve_cfg,
                                &embed,
                                &meta,
                                &graph,
                                move |emb, k| {
                                    let vs = vs.clone();
                                    Box::pin(async move {
                                        let raw = vs.search(tenant_id, &emb, k).await?;
                                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                    })
                                        as Pin<
                                            Box<
                                                dyn std::future::Future<
                                                        Output = Result<Vec<(i64, f64)>>,
                                                    > + Send,
                                            >,
                                        >
                                },
                                &infer_url,
                                &model,
                                |msg| println!("{msg}"),
                            )
                            .await?;
                            if let Some(seq) = seq_chunk {
                                chunks.insert(0, seq);
                            }
                            chunks
                        }
                    } else if effective_mode == "iterative" {
                        let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                            .context("opening graph store for iterative retrieval")?;
                        drop(spinner.take());
                        retrieve_iterative(
                            &query,
                            &retrieve_cfg,
                            &embed,
                            &meta,
                            &graph,
                            move |emb, k| {
                                let vs = vs.clone();
                                Box::pin(async move {
                                    let raw = vs.search(tenant_id, &emb, k).await?;
                                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                })
                                    as Pin<
                                        Box<
                                            dyn std::future::Future<
                                                    Output = Result<Vec<(i64, f64)>>,
                                                > + Send,
                                        >,
                                    >
                            },
                            &infer_url,
                            &model,
                            |msg| println!("{msg}"),
                        )
                        .await?
                    } else if effective_mode == "graph" {
                        let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                            .context("opening graph store for graph-anchored retrieval")?;
                        retrieve_graph_anchored(
                            &query,
                            &retrieve_cfg,
                            &embed,
                            &meta,
                            &graph,
                            move |emb, k| {
                                let vs = vs.clone();
                                Box::pin(async move {
                                    let raw = vs.search(tenant_id, &emb, k).await?;
                                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                })
                                    as Pin<
                                        Box<
                                            dyn std::future::Future<
                                                    Output = Result<Vec<(i64, f64)>>,
                                                > + Send,
                                        >,
                                    >
                            },
                        )
                        .await?
                    } else if understand {
                        kwaai_rag::query_understanding::retrieve_with_understanding(
                            &query,
                            &retrieve_cfg,
                            &embed,
                            &meta,
                            &infer_url,
                            &model,
                            move |emb, k| {
                                let vs = vs.clone();
                                Box::pin(async move {
                                    let raw = vs.search(tenant_id, &emb, k).await?;
                                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                                })
                                    as Pin<
                                        Box<
                                            dyn std::future::Future<
                                                    Output = Result<Vec<(i64, f64)>>,
                                                > + Send,
                                        >,
                                    >
                            },
                        )
                        .await?
                    } else {
                        retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, move |emb, k| {
                            let vs = vs.clone();
                            Box::pin(async move {
                                let raw = vs.search(tenant_id, &emb, k).await?;
                                Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                            })
                                as Pin<
                                    Box<
                                        dyn std::future::Future<Output = Result<Vec<(i64, f64)>>>
                                            + Send,
                                    >,
                                >
                        })
                        .await?
                    }
                }
                Some(url) => {
                    let http = reqwest::Client::new();
                    let url = url.to_string();
                    retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, move |emb, k| {
                        let http = http.clone();
                        let url = url.clone();
                        Box::pin(async move {
                            let raw = http_search_vectors(&http, &url, tenant_id, emb, k).await?;
                            Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                        })
                            as Pin<
                                Box<
                                    dyn std::future::Future<Output = Result<Vec<(i64, f64)>>>
                                        + Send,
                                >,
                            >
                    })
                    .await?
                }
                None => {
                    let ep = eve_peer_id(&rag_cfg)?;
                    let (client, _) = crate::vpk::p2p_connect().await?;
                    let client = Arc::new(tokio::sync::Mutex::new(client));
                    retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, move |emb, k| {
                        let client = client.clone();
                        Box::pin(async move {
                            let guard = client.lock().await;
                            let raw = rpc_search_vectors(&guard, &ep, tenant_id, emb, k).await?;
                            Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                        })
                            as Pin<
                                Box<
                                    dyn std::future::Future<Output = Result<Vec<(i64, f64)>>>
                                        + Send,
                                >,
                            >
                    })
                    .await?
                }
            };
            if kb_names.len() > 1 {
                for c in &mut chunks {
                    c.source_kb = Some(kb_name.clone());
                }
            }
            all_results.append(&mut chunks);
        }

        all_results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // LLM listwise reranker — retrieve extra candidates then rerank to top_k.
        let all_results = if rerank {
            if let Some(ref url) = inference_url {
                kwaai_rag::reranker::rerank_chunks(&query, all_results, url, &model, top_k).await
            } else {
                all_results.truncate(top_k);
                all_results
            }
        } else {
            all_results.truncate(top_k);
            all_results
        };

        if let Some(s) = spinner {
            s.finish("").await;
        }

        let arr: Vec<serde_json::Value> = all_results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let mut v = serde_json::json!({
                    "rank": i + 1,
                    "score": r.score,
                    "doc": r.chunk_meta.doc_name,
                    "chunk": r.chunk_meta.chunk_index,
                    "text": r.chunk_meta.text,
                });
                if let Some(ref kb_name) = r.source_kb {
                    v["kb"] = serde_json::json!(kb_name);
                }
                v
            })
            .collect();
        render_query_results(&query, &arr, json_out)
    }
}

fn render_query_results(query: &str, results: &[serde_json::Value], json_out: bool) -> Result<()> {
    if json_out {
        println!("{}", serde_json::to_string_pretty(results)?);
        return Ok(());
    }
    if results.is_empty() {
        print_warning("No results found.");
        return Ok(());
    }
    print_box_header(&format!("Top {} results for: {}", results.len(), query));
    for r in results {
        println!(
            "  [{}] score={:.4}  doc={}  chunk={}",
            r["rank"].as_u64().unwrap_or(0),
            r["score"].as_f64().unwrap_or(0.0),
            r["doc"].as_str().unwrap_or(""),
            r["chunk"].as_u64().unwrap_or(0),
        );
        println!("      {}", truncate(r["text"].as_str().unwrap_or(""), 200));
        println!();
    }
    Ok(())
}

// ── chat ──────────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_chat(
    top_k: usize,
    inference_url: Option<String>,
    kb: String,
    understand: bool,
    model: String,
    hyde: bool,
    hyde_alpha: Option<f32>,
    rerank: bool,
    mode: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let inference_url = inference_url
            .or_else(|| {
                // Prefer the global kwaainet config when the user has pointed it
                // at a non-localhost host (i.e. a remote inference node).
                let global = crate::config::KwaaiNetConfig::load_or_create().ok()?;
                let url = &global.inference_url;
                let is_remote = !url.contains("localhost") && !url.contains("127.0.0.1");
                is_remote.then(|| url.clone())
            })
            .unwrap_or_else(|| rag_cfg.inference_url.clone());

        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

        let retrieve_cfg = RetrieveConfig {
            top_k,
            min_score: 0.0,
            use_sentence_window: false,
            hyde_inference_url: if hyde {
                Some(inference_url.clone())
            } else {
                None
            },
            hyde_model: if hyde { Some(model.clone()) } else { None },
            hyde_alpha: if hyde {
                Some(hyde_alpha.unwrap_or(0.5))
            } else {
                None
            },
            graph_mode: kwaai_rag::query_understand::GraphMode::Inject,
            query_classify: kwaai_rag::query_understand::ClassifyMethod::Rule,
            query_multi_hop: false,
            use_summary_expansion: false,
        };

        let http = reqwest::Client::new();
        let mut history: Vec<ChatMessage> = vec![];

        // Prepare storage backend once.
        let storage_mode = rag_cfg.storage_url.clone();
        let local_vs = if storage_mode.as_deref() == Some("local") {
            Some(Arc::new(open_local_vs(&rag_cfg.data_dir())?))
        } else {
            None
        };
        let p2p_client = if storage_mode.is_none() {
            let eve = eve_peer_id(&rag_cfg)?;
            let (c, _) = crate::vpk::p2p_connect().await?;
            Some((Arc::new(tokio::sync::Mutex::new(c)), eve))
        } else {
            None
        };

        // Open semantic query cache (local KB only).
        let mut query_cache = if storage_mode.as_deref() == Some("local") {
            QueryCache::open(&rag_cfg.data_dir(), tenant_id).ok()
        } else {
            None
        };

        print_box_header("RAG Chat  (type 'exit' to quit)");

        let stdin = io::stdin();
        loop {
            print!("\n  You: ");
            io::stdout().flush().ok();

            let mut line = String::new();
            if stdin.lock().read_line(&mut line).is_err() {
                break;
            }
            let query = line.trim().to_string();
            if query.is_empty() {
                continue;
            }
            if query == "exit" || query == "quit" {
                break;
            }

            // Semantic cache check (local KB only).
            if let Some(ref mut cache) = query_cache {
                if let Ok(query_emb) = embed.embed_one(&query).await {
                    if let Some(hit) = cache.get(&query_emb) {
                        println!("\n  Assistant: {}  \x1b[2m(cached)\x1b[0m", hit.answer);
                        history.push(ChatMessage {
                            role: "user".to_string(),
                            content: query.clone(),
                        });
                        history.push(ChatMessage {
                            role: "assistant".to_string(),
                            content: hit.answer,
                        });
                        if history.len() > 20 {
                            history.drain(0..2);
                        }
                        continue;
                    }
                }
            }

            // Load document context preamble from persisted schema metadata (if any).
            let doc_context_line: Option<String> = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                .ok()
                .and_then(|g| {
                    let meta = g.get_doc_metadata();
                    if meta.is_empty() {
                        return None;
                    }
                    let schema = kwaai_rag::doc_schema::DocSchema {
                        metadata: meta,
                        document_title: g.get_document_titles().into_iter().next(),
                        ..Default::default()
                    };
                    schema.context_line()
                });

            // Resolve effective mode for this turn (auto → graph if KB has entities).
            let effective_mode_chat: &str = if mode == "auto" {
                if let Ok(g) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                    if g.node_count() > 0 {
                        "graph"
                    } else {
                        "vector"
                    }
                } else {
                    "vector"
                }
            } else {
                mode.as_str()
            };

            // Retrieve context.
            let chunks = if let Some(ref vs) = local_vs {
                let vs2 = vs.clone();
                let search_fn = move |emb: Vec<f32>, k: usize| {
                    let vs = vs2.clone();
                    Box::pin(async move {
                        let raw = vs.search(tenant_id, &emb, k).await?;
                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                    })
                        as Pin<
                            Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>,
                        >
                };
                if effective_mode_chat == "iterative" {
                    let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                        .context("opening graph store for iterative retrieval")?;
                    retrieve_iterative(
                        &query,
                        &retrieve_cfg,
                        &embed,
                        &meta,
                        &graph,
                        search_fn,
                        &inference_url,
                        &model,
                        |msg| println!("{msg}"),
                    )
                    .await?
                } else if effective_mode_chat == "graph" {
                    let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                        .context("opening graph store for graph-anchored retrieval")?;
                    retrieve_graph_anchored(&query, &retrieve_cfg, &embed, &meta, &graph, search_fn)
                        .await?
                } else if understand {
                    kwaai_rag::query_understanding::retrieve_with_understanding(
                        &query,
                        &retrieve_cfg,
                        &embed,
                        &meta,
                        &inference_url,
                        &model,
                        search_fn,
                    )
                    .await?
                } else {
                    retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, search_fn).await?
                }
            } else if let Some(ref url) = storage_mode {
                let http2 = http.clone();
                let url2 = url.clone();
                retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, move |embedding, k| {
                    let h = http2.clone();
                    let u = url2.clone();
                    Box::pin(async move {
                        let raw = http_search_vectors(&h, &u, tenant_id, embedding, k).await?;
                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                    })
                        as Pin<
                            Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>,
                        >
                })
                .await?
            } else {
                let (client2, eve) = p2p_client.as_ref().unwrap();
                let client2 = client2.clone();
                let eve_peer_id = *eve;
                retrieve_hybrid(&query, &retrieve_cfg, &embed, &meta, move |embedding, k| {
                    let c = client2.clone();
                    Box::pin(async move {
                        let guard = c.lock().await;
                        let raw = rpc_search_vectors(&guard, &eve_peer_id, tenant_id, embedding, k)
                            .await?;
                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                    })
                        as Pin<
                            Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>,
                        >
                })
                .await?
            };

            // LLM listwise reranker (optional).
            let chunks = if rerank {
                kwaai_rag::reranker::rerank_chunks(&query, chunks, &inference_url, &model, top_k)
                    .await
            } else {
                chunks
            };

            let messages = build_chat_messages(
                &query,
                &chunks,
                &history,
                24000,
                doc_context_line.as_deref(),
            );
            let payload = serde_json::json!({
                "model": model,
                "messages": messages,
                "stream": false,
            });

            let resp = http
                .post(format!("{inference_url}/v1/chat/completions"))
                .json(&payload)
                .send()
                .await
                .context("calling shard API")?;

            let body: serde_json::Value = resp.json().await?;
            let answer = body["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("(no response)")
                .to_string();

            println!("\n  Assistant: {answer}");

            // Store in cache (local KB only, fire-and-forget).
            if let Some(ref mut cache) = query_cache {
                if let Ok(query_emb) = embed.embed_one(&query).await {
                    let chunk_ids: Vec<i64> = chunks.iter().map(|_| 0i64).collect();
                    let _ = cache.put(query.clone(), query_emb, answer.clone(), chunk_ids);
                }
            }

            history.push(ChatMessage {
                role: "user".to_string(),
                content: query,
            });
            history.push(ChatMessage {
                role: "assistant".to_string(),
                content: answer,
            });
            // Keep last 10 turns.
            if history.len() > 20 {
                history.drain(0..2);
            }
        }
        Ok(())
    }
}

// ── docs ──────────────────────────────────────────────────────────────────────

async fn cmd_docs(kb: String) -> Result<()> {
    let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
    let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

    let docs = meta.list_docs()?;
    if docs.is_empty() {
        print_info(&format!(
            "No documents ingested yet. Run: kwaainet rag ingest <file> --kb {kb}"
        ));
    } else {
        print_box_header(&format!("{} document(s)", docs.len()));
        for d in &docs {
            println!("  • {d}");
        }
    }
    Ok(())
}

// ── delete-doc ────────────────────────────────────────────────────────────────

async fn cmd_delete_doc(name: String, yes: bool, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        if !yes {
            print!("  Delete '{name}' from the knowledge base? [y/N] ");
            io::stdout().flush().ok();
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            if !line.trim().eq_ignore_ascii_case("y") {
                print_info("Aborted.");
                return Ok(());
            }
        }

        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

        let ids = meta.delete_doc(&name)?;
        if ids.is_empty() {
            print_warning(&format!("Document '{name}' not found."));
            return Ok(());
        }

        match rag_cfg.storage_url.as_deref() {
            Some("local") => {
                let vs = open_local_vs(&rag_cfg.data_dir())?;
                vs.delete(tenant_id, &ids)
                    .await
                    .context("deleting vectors from local store")?;
            }
            Some(url) => {
                let http = reqwest::Client::new();
                http_delete_vectors(&http, url, tenant_id, ids.clone())
                    .await
                    .context("deleting vectors from Eve (HTTP)")?;
            }
            None => {
                let ep = eve_peer_id(&rag_cfg)?;
                let (client, _) = crate::vpk::p2p_connect().await?;
                rpc_delete_vectors(&client, &ep, tenant_id, ids.clone())
                    .await
                    .context("deleting vectors from Eve (P2P)")?;
            }
        }

        print_success(&format!("Deleted '{name}' ({} chunks removed)", ids.len()));
        Ok(())
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn load_rag_config_for(kb: &str) -> Result<(RagConfig, Uuid)> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let rag = cfg.get_rag_kb(kb).cloned().with_context(|| {
        format!("KB '{kb}' not initialised. Run: kwaainet rag init --name {kb}")
    })?;

    let tenant_id: Uuid = rag
        .tenant_id
        .as_deref()
        .context("no tenant_id in RAG config")?
        .parse()
        .context("invalid tenant_id")?;

    Ok((rag, tenant_id))
}

fn eve_peer_id(rag: &RagConfig) -> Result<PeerId> {
    rag.eve_peer_id
        .as_deref()
        .context("no Eve configured — run: kwaainet rag connect-eve <peer-id>")?
        .parse()
        .context("invalid eve_peer_id")
}

#[cfg(feature = "storage")]
fn open_local_vs(data_dir: &std::path::Path) -> Result<kwaai_storage::VectorStore> {
    let db = kwaai_storage::StorageDb::open(data_dir).context("opening local vector store")?;
    Ok(kwaai_storage::VectorStore::new(db))
}

/// Fire-and-forget vector delete used by sync — errors are logged, not fatal.
#[cfg(feature = "storage")]
async fn sync_delete_vectors(rag_cfg: &RagConfig, tenant_id: uuid::Uuid, ids: Vec<i64>) {
    match rag_cfg.storage_url.as_deref() {
        Some("local") => {
            if let Ok(vs) = open_local_vs(&rag_cfg.data_dir()) {
                let _ = vs.delete(tenant_id, &ids).await;
            }
        }
        Some(url) => {
            let http = reqwest::Client::new();
            let _ = crate::storage_rpc::http_delete_vectors(&http, url, tenant_id, ids).await;
        }
        None => {
            if let Ok(ep) = eve_peer_id(rag_cfg) {
                if let Ok((client, _)) = crate::vpk::p2p_connect().await {
                    let _ =
                        crate::storage_rpc::rpc_delete_vectors(&client, &ep, tenant_id, ids).await;
                }
            }
        }
    };
}

fn parse_surr_mode(s: &str) -> kwaai_rag::chunker::SurrMode {
    match s.to_lowercase().as_str() {
        "full" => kwaai_rag::chunker::SurrMode::Full,
        _ => kwaai_rag::chunker::SurrMode::Truncated,
    }
}

fn parse_chunk_strategy(s: &str) -> kwaai_rag::chunker::ChunkStrategy {
    match s.to_lowercase().as_str() {
        "paragraph" => kwaai_rag::chunker::ChunkStrategy::Paragraph,
        _ => kwaai_rag::chunker::ChunkStrategy::Character,
    }
}

fn load_doc_meta(path: &std::path::Path) -> Result<std::collections::HashMap<String, String>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("reading doc-meta file '{}'", path.display()))?;
    let map: std::collections::HashMap<String, String> = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing doc-meta YAML '{}'", path.display()))?;
    Ok(map)
}

fn truncate(s: &str, max: usize) -> &str {
    let mut end = s.len().min(max);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// ── destroy ───────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_rebuild(
    file: std::path::PathBuf,
    kb: String,
    embed_model: String,
    inference_urls: String,
    model: String,
    workers: usize,
    seed_file: Option<std::path::PathBuf>,
    chunk_strategy: String,
    doc_meta: Option<std::path::PathBuf>,
    doc_schema: Option<std::path::PathBuf>,
    entity_types: Option<String>,
    no_relations: bool,
    graph_window: usize,
    sample_pct: Option<u8>,
    yes: bool,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        use crate::cli::GraphAction;

        print_box_header(&format!("RAG Rebuild ({})", kb));
        println!("  File:            {}", file.display());
        println!("  Embed model:     {embed_model}");
        println!("  Inference URLs:  {inference_urls}");
        println!("  Extract model:   {model}");
        println!("  Workers:         {workers}");
        if let Some(ref sf) = seed_file {
            println!("  Seed file:       {}", sf.display());
        }
        println!("  Chunk strategy:  {chunk_strategy}");
        println!();

        // ── Step 1: Destroy ───────────────────────────────────────────────
        println!("  ▶ Step 1/8  destroy");
        cmd_destroy(yes, kb.clone()).await?;

        // ── Step 2: Init ──────────────────────────────────────────────────
        println!();
        println!("  ▶ Step 2/8  init");
        cmd_init(kb.clone(), embed_model, None, false).await?;

        // ── Step 3: Ingest ────────────────────────────────────────────────
        println!();
        println!("  ▶ Step 3/8  ingest");
        cmd_ingest(
            file,
            None,
            800,
            200,
            20,
            false,
            None,
            "default".to_string(),
            chunk_strategy,
            "truncated".to_string(),
            doc_meta,
            doc_schema,
            kb.clone(),
        )
        .await?;

        // ── Step 4: Graph build ───────────────────────────────────────────
        println!();
        println!("  ▶ Step 4/8  graph build");
        cmd_graph(
            GraphAction::Build {
                inference_url: None,
                model,
                limit: None,
                docs: None,
                workers,
                inference_urls: Some(inference_urls),
                entity_types,
                no_relations,
                reset_graph: false,
                graph_window,
                sample_pct,
                gliner_url: None,
                entity_centric: false,
                chunk_batch: 1,
                ec_refine_threshold: 0.0,
                ec_refine_budget: 50,
                ec_refine_only: false,
            },
            kb.clone(),
        )
        .await?;

        // ── Step 5: Seed ──────────────────────────────────────────────────
        if let Some(seed_path) = seed_file {
            println!();
            println!("  ▶ Step 5/8  graph seed");
            cmd_graph(
                GraphAction::Seed {
                    file: seed_path,
                    kb: kb.clone(),
                },
                kb.clone(),
            )
            .await?;
        } else {
            println!("  ─ Step 5/8  graph seed  (skipped — no --seed-file)");
        }

        // ── Step 6: Alias scan ────────────────────────────────────────────
        println!();
        println!("  ▶ Step 6/8  graph alias-scan");
        cmd_graph(
            GraphAction::AliasScan {
                auto: true,
                dry_run: false,
                min_hits: 1,
            },
            kb.clone(),
        )
        .await?;

        // ── Step 7: Reembed ───────────────────────────────────────────────
        println!();
        println!("  ▶ Step 7/8  graph reembed");
        cmd_graph(GraphAction::Reembed { embed_url: None }, kb.clone()).await?;

        // ── Step 8: Dedup ─────────────────────────────────────────────────
        // auto_threshold=1.01 disables Tier 2 embedding-similarity auto-merges:
        // nomic-embed-text returns sim=1.000 for categorically similar but distinct
        // entities (e.g. "Nelson Eddy" → "Nelson Mandela"). Tiers 1 (exact name)
        // and 3 (structural/honorific/fuzzy) are still applied automatically.
        // Run `kwaainet rag graph dedup --kb <KB>` interactively after rebuild
        // to review the 0.85–1.0 Tier 2 candidates.
        println!();
        println!("  ▶ Step 8/8  graph dedup (Tier 1 + structural; Tier 2 deferred for review)");
        cmd_graph(
            GraphAction::Dedup {
                threshold: 0.85,
                auto: true,
                auto_threshold: Some(1.01),
                dry_run: false,
            },
            kb.clone(),
        )
        .await?;

        // ── Final: Score ──────────────────────────────────────────────────
        println!();
        cmd_graph(
            GraphAction::Score {
                top: 20,
                json: false,
            },
            kb.clone(),
        )
        .await?;

        Ok(())
    }
}

async fn cmd_destroy(yes: bool, kb: String) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let rag = cfg
        .get_rag_kb(&kb)
        .with_context(|| format!("KB '{kb}' not initialised (nothing to destroy)."))?;

    let data_dir = rag.data_dir();
    let tenant_id = rag.tenant_id.as_deref().unwrap_or("<unknown>");

    print_box_header(&format!("RAG Destroy ({})", kb));
    println!("  Knowledge base: {}", data_dir.display());
    println!("  Tenant:         {tenant_id}");
    println!();
    print_warning("This will permanently delete all vectors, chunks, and metadata.");
    println!();

    if !yes {
        print!("  Type 'yes' to confirm: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());
        let mut input = String::new();
        std::io::BufRead::read_line(&mut std::io::stdin().lock(), &mut input)?;
        if input.trim() != "yes" {
            print_info("Aborted — knowledge base not deleted.");
            return Ok(());
        }
    }

    if data_dir.exists() {
        std::fs::remove_dir_all(&data_dir)
            .with_context(|| format!("deleting {}", data_dir.display()))?;
    }

    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.remove_rag_kb(&kb);
    cfg.save()?;

    print_success(&format!("Knowledge base '{}' destroyed.", kb));
    println!("  Run  kwaainet rag init --name {kb}  to start fresh.");
    Ok(())
}

// ── sync ──────────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_sync(
    folder: std::path::PathBuf,
    extensions: String,
    delete: bool,
    watch: bool,
    interval: u64,
    chunk_size: usize,
    chunk_overlap: usize,
    min_chunk_len: usize,
    extract_entities: bool,
    inference_url: Option<String>,
    extraction_model: String,
    chunk_strategy: String,
    surr_mode: String,
    doc_meta_path: Option<std::path::PathBuf>,
    kb: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let exts: Vec<String> = extensions
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        if !folder.is_dir() {
            bail!("'{}' is not a directory", folder.display());
        }

        print_box_header(&format!("RAG Sync: {}", folder.display()));
        if watch {
            println!("  Watch mode: polling every {interval}s (Ctrl+C to stop)");
        }
        if extract_entities {
            print_info("Entity extraction enabled — knowledge graph will be updated");
        }
        print_separator();

        let doc_meta = if let Some(path) = doc_meta_path {
            let m = load_doc_meta(&path)?;
            print_info(&format!("Doc-meta loaded: {} entries", m.len()));
            m
        } else {
            std::collections::HashMap::new()
        };

        let chunk_cfg = kwaai_rag::chunker::ChunkConfig {
            chunk_size,
            chunk_overlap,
            min_chunk_len,
            strategy: parse_chunk_strategy(&chunk_strategy),
            surr_mode: parse_surr_mode(&surr_mode),
        };

        loop {
            let result = run_sync_pass(
                &folder,
                &exts,
                delete,
                &kb,
                &chunk_cfg,
                extract_entities,
                inference_url.clone(),
                extraction_model.clone(),
                doc_meta.clone(),
            )
            .await?;

            let SyncResult {
                ingested,
                updated,
                deleted,
                skipped,
            } = result;
            if ingested + updated + deleted > 0 {
                print_success(&format!(
                    "ingested={ingested}  updated={updated}  deleted={deleted}  skipped={skipped}"
                ));
            } else {
                print_info(&format!("No changes (skipped={skipped})"));
            }

            if !watch {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        }
        Ok(())
    }
}

#[cfg(feature = "storage")]
struct SyncResult {
    ingested: usize,
    updated: usize,
    deleted: usize,
    skipped: usize,
}

#[cfg(feature = "storage")]
#[allow(clippy::too_many_arguments)]
async fn run_sync_pass(
    folder: &std::path::Path,
    exts: &[String],
    delete: bool,
    kb: &str,
    chunk_cfg: &kwaai_rag::chunker::ChunkConfig,
    extract_entities: bool,
    inference_url: Option<String>,
    extraction_model: String,
    doc_meta: std::collections::HashMap<String, String>,
) -> Result<SyncResult> {
    use std::time::UNIX_EPOCH;

    let (rag_cfg, tenant_id) = load_rag_config_for(kb)?;
    let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

    // Discover all matching files under the folder.
    let mut disk_files: Vec<(String, std::path::PathBuf)> = Vec::new();
    collect_files(folder, folder, exts, &mut disk_files)?;
    let disk_set: std::collections::HashSet<String> =
        disk_files.iter().map(|(n, _)| n.clone()).collect();

    let mut result = SyncResult {
        ingested: 0,
        updated: 0,
        deleted: 0,
        skipped: 0,
    };

    // Determine which docs to ingest/update.
    for (doc_name, path) in &disk_files {
        let file_meta = std::fs::metadata(path)?;
        let mtime_secs = file_meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let file_size = file_meta.len();

        let needs_ingest = match meta.get_sync_meta(doc_name)? {
            None => true,
            Some(prev) => prev.mtime_secs != mtime_secs || prev.file_size != file_size,
        };

        if !needs_ingest {
            result.skipped += 1;
            continue;
        }

        let is_update = meta.get_sync_meta(doc_name)?.is_some();

        // Re-ingest: delete old vectors first if updating.
        if is_update {
            let old_ids = meta.delete_doc(doc_name)?;
            if !old_ids.is_empty() {
                sync_delete_vectors(&rag_cfg, tenant_id, old_ids).await;
            }
            meta.delete_sync_meta(doc_name)?;
        }

        // Ingest the file.
        let text = match document::extract_text(path) {
            Ok(t) => t,
            Err(e) => {
                print_warning(&format!("Skipping '{}': {e}", path.display()));
                result.skipped += 1;
                continue;
            }
        };

        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
        let mut ingest_cfg = IngestConfig::new(embed);
        ingest_cfg.chunk_cfg = chunk_cfg.clone();
        ingest_cfg.doc_meta = doc_meta.clone();

        if extract_entities {
            let infer_url = inference_url
                .clone()
                .unwrap_or_else(|| rag_cfg.inference_url.clone());
            if let Ok(store) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                ingest_cfg.graph = Some(GraphIngestConfig {
                    store: Arc::new(Mutex::new(store)),
                    inference_url: infer_url,
                    inference_urls: vec![],
                    model: extraction_model.clone(),
                    workers: 1,
                    entity_types: vec![],
                    no_relations: false,
                    context_window: 1,
                    gliner_client: None,
                    entity_centric: false,
                    chunk_batch: 1,
                    ec_refine_threshold: 0.0,
                    ec_refine_budget: 50,
                    ec_refine_only: false,
                });
            }
        }

        let ingest_result = match rag_cfg.storage_url.as_deref() {
            Some("local") => {
                let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
                ingest_text(
                    &ingest_cfg,
                    &meta,
                    doc_name,
                    &text,
                    move |vectors| {
                        let vs = vs.clone();
                        Box::pin(async move { vs.upload(tenant_id, &vectors).await })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    None::<fn(usize, usize)>,
                )
                .await?
            }
            Some(url) => {
                let http = reqwest::Client::new();
                let url = url.to_string();
                ingest_text(
                    &ingest_cfg,
                    &meta,
                    doc_name,
                    &text,
                    move |vectors| {
                        let h = http.clone();
                        let u = url.clone();
                        Box::pin(async move {
                            crate::storage_rpc::http_upload_vectors(&h, &u, tenant_id, vectors)
                                .await
                        })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    None::<fn(usize, usize)>,
                )
                .await?
            }
            None => {
                let ep = eve_peer_id(&rag_cfg)?;
                let (client, _) = crate::vpk::p2p_connect().await?;
                let client = Arc::new(tokio::sync::Mutex::new(client));
                ingest_text(
                    &ingest_cfg,
                    &meta,
                    doc_name,
                    &text,
                    move |vectors| {
                        let c = client.clone();
                        Box::pin(async move {
                            let guard = c.lock().await;
                            crate::storage_rpc::rpc_upload_vectors(&guard, &ep, tenant_id, vectors)
                                .await
                        })
                            as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
                    },
                    None::<fn(usize, usize)>,
                )
                .await?
            }
        };

        // Record sync state.
        meta.put_sync_meta(
            doc_name,
            &SyncMeta {
                file_path: path.to_string_lossy().into_owned(),
                mtime_secs,
                file_size,
            },
        )?;

        if is_update {
            println!(
                "  ↺ updated  '{}' ({} chunks)",
                doc_name, ingest_result.chunks_ingested
            );
            result.updated += 1;
        } else {
            println!(
                "  + ingested '{}' ({} chunks)",
                doc_name, ingest_result.chunks_ingested
            );
            result.ingested += 1;
        }
    }

    // Delete KB entries whose source files are gone.
    if delete {
        for (doc_name, sync) in meta.all_sync_metas()? {
            if disk_set.contains(&doc_name) {
                continue;
            }
            // File no longer on disk — remove from KB.
            let old_ids = meta.delete_doc(&doc_name)?;
            meta.delete_sync_meta(&doc_name)?;
            if !old_ids.is_empty() {
                sync_delete_vectors(&rag_cfg, tenant_id, old_ids).await;
            }
            println!("  - deleted  '{}' (source: {})", doc_name, sync.file_path);
            result.deleted += 1;
        }
    }

    Ok(result)
}

// ── graph ─────────────────────────────────────────────────────────────────────

async fn cmd_graph(action: GraphAction, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;

        match action {
            GraphAction::Stats => {
                let store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                print_box_header(&format!("Knowledge Graph ({})", kb));
                println!("  Entities:  {}", store.node_count());
                println!("  Relations: {}", store.relation_count());
            }

            GraphAction::Show { name } => {
                let store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                match store.find_by_name(&name) {
                    None => print_warning(&format!("Entity '{}' not found.", name)),
                    Some(node) => {
                        print_box_header(&format!("Entity: {} [{}]", node.name, node.entity_type));
                        println!("  Description: {}", node.description);
                        println!("  Mentions:    {}", node.mention_count);
                        if node.aliases.is_empty() {
                            println!("  Aliases:     (none)");
                        } else {
                            println!("  Aliases ({}):", node.aliases.len());
                            for a in &node.aliases {
                                println!("    • {a}");
                            }
                        }
                        let neighbors = store.neighbors_of(node.id);
                        if neighbors.is_empty() {
                            println!("  Neighbors:   (none)");
                        } else {
                            println!("  Neighbors ({}):", neighbors.len());
                            for (nid, rel, strength) in &neighbors {
                                if let Some(n) = store.get_entity(*nid) {
                                    println!(
                                        "    → {} [{}]  strength={:.2}",
                                        n.name, rel, strength
                                    );
                                }
                            }
                        }
                    }
                }
            }

            GraphAction::Clear { yes } => {
                if !yes {
                    print!("  Wipe the knowledge graph for '{kb}'? [y/N] ");
                    io::stdout().flush().ok();
                    let mut line = String::new();
                    io::stdin().lock().read_line(&mut line)?;
                    if !line.trim().eq_ignore_ascii_case("y") {
                        print_info("Aborted.");
                        return Ok(());
                    }
                }
                // Delete the graph redb file so it is recreated fresh on next open.
                let graph_path = rag_cfg.data_dir().join(format!("graph-{}.redb", tenant_id));
                if graph_path.exists() {
                    std::fs::remove_file(&graph_path)
                        .with_context(|| format!("deleting {}", graph_path.display()))?;
                }
                print_success(&format!(
                    "Knowledge graph for '{}' cleared. Run `kwaainet rag graph build --kb {}` to rebuild.",
                    kb, kb
                ));
            }

            GraphAction::Build {
                inference_url,
                model,
                limit,
                docs,
                workers,
                inference_urls,
                entity_types,
                no_relations,
                graph_window,
                reset_graph,
                sample_pct,
                gliner_url,
                entity_centric,
                chunk_batch,
                ec_refine_threshold,
                ec_refine_budget,
                ec_refine_only,
            } => {
                let raw_infer_url = inference_url.unwrap_or_else(|| rag_cfg.inference_url.clone());
                let raw_extra_urls: Vec<String> = inference_urls
                    .as_deref()
                    .map(|s| {
                        s.split(',')
                            .map(|u| u.trim().to_string())
                            .filter(|u| !u.is_empty())
                            .collect()
                    })
                    .unwrap_or_default();
                let effective_workers = workers.max(1);

                // Resolve p2p:// URLs to local HTTP proxies if needed.
                let all_raw: Vec<String> = std::iter::once(raw_infer_url)
                    .chain(raw_extra_urls)
                    .collect();
                let has_p2p = all_raw
                    .iter()
                    .any(|u| u.starts_with("p2p://") || u.starts_with("mux://"));
                let (_proxy_handles, resolved_all) = if has_p2p {
                    use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
                    let sock = std::env::var("KWAAINET_SOCKET")
                        .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
                    #[cfg(unix)]
                    let addr = format!("/unix/{sock}");
                    #[cfg(not(unix))]
                    let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
                    let p2p = Arc::new(
                        P2PClient::connect(&addr)
                            .await
                            .context("connecting to p2pd for p2p:// URL resolution")?,
                    );
                    let (res, handles) =
                        crate::ollama_proxy::resolve_inference_urls(&all_raw, &p2p).await?;
                    (handles, res)
                } else {
                    (vec![], all_raw)
                };
                let infer_url = resolved_all[0].clone();
                let extra_urls: Vec<String> = resolved_all[1..].to_vec();

                // Parse --entity-types into a Vec<String>
                let parsed_entity_types: Vec<String> = entity_types
                    .as_deref()
                    .map(|s| {
                        s.split(',')
                            .map(|t| t.trim().to_string())
                            .filter(|t| !t.is_empty())
                            .collect()
                    })
                    .unwrap_or_default();

                // Optionally wipe the graph before rebuilding.
                if reset_graph {
                    let graph_path = rag_cfg.data_dir().join(format!("graph-{}.redb", tenant_id));
                    if graph_path.exists() {
                        std::fs::remove_file(&graph_path).with_context(|| {
                            format!("clearing graph at {}", graph_path.display())
                        })?;
                        print_info("Graph cleared — rebuilding from scratch.");
                    }
                }

                let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;
                let mut all_chunks = meta.all_chunks()?;

                // Filter by document name patterns if --docs is set
                if let Some(ref patterns) = docs {
                    let pats: Vec<String> = patterns
                        .split(',')
                        .map(|s| s.trim().to_lowercase())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !pats.is_empty() {
                        all_chunks.retain(|(_, cm)| {
                            let name = cm.doc_name.to_lowercase();
                            pats.iter().any(|p| name.contains(p.as_str()))
                        });
                    }
                }

                if let Some(n) = limit {
                    all_chunks.truncate(n);
                }
                if let Some(pct) = sample_pct {
                    let n = (all_chunks.len() * pct.min(100) as usize).div_ceil(100);
                    all_chunks.truncate(n);
                }
                let total = all_chunks.len();

                if total == 0 {
                    print_warning(
                        "No chunks found — ingest documents first (or check --docs filter).",
                    );
                    return Ok(());
                }

                print_box_header(&format!("Graph Build ({})", kb));
                if let Some(ref p) = docs {
                    println!("  Doc filter:        {p}");
                }
                if let Some(pct) = sample_pct {
                    println!("  Sample:            {pct}% of corpus");
                }
                println!("  Chunks to process: {total}");
                if !extra_urls.is_empty() {
                    println!("  Inference URLs:    {}", extra_urls.join(", "));
                    println!("  Workers:           {effective_workers}");
                } else {
                    println!("  Inference URL:     {infer_url}");
                    if effective_workers > 1 {
                        println!("  Workers:           {effective_workers}");
                    }
                }
                if !parsed_entity_types.is_empty() {
                    println!("  Entity types:      {}", parsed_entity_types.join(", "));
                }
                if no_relations {
                    println!("  Relations:         disabled");
                }
                println!("  This may take a while — one LLM call per chunk.\n");

                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                let store = Arc::new(Mutex::new(
                    GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                        .context("opening graph store")?,
                ));
                let gliner_client = gliner_url.as_deref().map(|url| {
                    println!("  GLiNER NER:        {url}");
                    kwaai_rag::gliner::GliNERClient::new(url, 0.4)
                });

                let graph_cfg = kwaai_rag::ingestion::GraphIngestConfig {
                    store: store.clone(),
                    inference_url: infer_url,
                    inference_urls: extra_urls,
                    model,
                    workers: effective_workers,
                    entity_types: parsed_entity_types,
                    no_relations,
                    context_window: graph_window,
                    gliner_client,
                    entity_centric,
                    chunk_batch,
                    ec_refine_threshold,
                    ec_refine_budget,
                    ec_refine_only,
                };

                let chunks: Vec<kwaai_rag::chunker::Chunk> = all_chunks
                    .iter()
                    .map(|(id, cm)| kwaai_rag::chunker::Chunk {
                        id: *id,
                        doc_name: cm.doc_name.clone(),
                        chunk_index: cm.chunk_index,
                        text: cm.text.clone(),
                        surrounding: cm.surrounding.clone(),
                        page_num: cm.page_num,
                        section_name: cm.section_name.clone(),
                        skip_extraction: cm.skip_extraction,
                        section_note: cm.section_note.clone(),
                        section_type: cm.section_type.clone(),
                    })
                    .collect();
                let ids: Vec<i64> = all_chunks.iter().map(|(id, _)| *id).collect();

                let build_start = std::time::Instant::now();
                let progress_path = rag_cfg.data_dir().join("graph-build-progress.json");
                let started_at = chrono::Utc::now().to_rfc3339();
                kwaai_rag::ingestion::extract_and_store_entities_pub(
                    &chunks,
                    &ids,
                    &embed,
                    &graph_cfg,
                    Some(std::sync::Arc::new(move |done: usize, total: usize, entities: usize, relations: usize| {
                        let elapsed = build_start.elapsed().as_secs_f64();
                        let rate = if done > 0 { done as f64 / elapsed } else { 0.0 };
                        let eta_secs = if rate > 0.0 && done < total {
                            (total - done) as f64 / rate
                        } else {
                            0.0
                        };
                        let eta_str = if eta_secs > 0.0 {
                            format!("  ETA {:.0}s", eta_secs)
                        } else {
                            String::new()
                        };
                        eprint!(
                            "\r  [{done:>4}/{total}]  entities={entities:>4}  rels={relations:>4}  elapsed={elapsed:.0}s{eta_str}    "
                        );
                        if done == total {
                            eprintln!();
                        }
                        // Write progress JSON every 10 chunks (and on completion).
                        if done.is_multiple_of(10) || done == total {
                            let json = format!(
                                "{{\"chunks_done\":{done},\"chunks_total\":{total},\"entities\":{entities},\"relations\":{relations},\"elapsed_secs\":{elapsed:.1},\"chunks_per_sec\":{rate:.2},\"eta_secs\":{eta_secs:.0},\"started_at\":\"{started_at}\",\"updated_at\":\"{}\"}}\n",
                                chrono::Utc::now().to_rfc3339()
                            );
                            let _ = std::fs::write(&progress_path, json.as_bytes());
                        }
                    })),
                )
                .await;

                let final_store = store.lock().unwrap();
                print_success(&format!(
                    "Graph built — {} entities, {} relations",
                    final_store.node_count(),
                    final_store.relation_count()
                ));
            }

            GraphAction::Dedup {
                threshold,
                auto,
                auto_threshold,
                dry_run,
            } => {
                print_box_header(&format!("Graph Dedup ({})", kb));
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                println!(
                    "  Graph: {} entities, {} relations\n",
                    store.node_count(),
                    store.relation_count()
                );

                let mut total_merged = 0usize;
                let mut need_rebuild = false;

                // ── Pre-compute relation-based merge blocks ───────────────
                // R1: role contradiction (same entity would be spouse_of+sibling_of same person)
                // R2: half-sibling disambiguation (shared parent but different other parents)
                // These are computed once against all candidates and checked before any merge.
                let all_candidates: Vec<(i64, i64)> = {
                    let t1 = store.find_dedup_candidates_exact();
                    let t2 = store.find_dedup_candidates(threshold);
                    let t3 = store.find_dedup_candidates_name_structure();
                    let t4a = store.find_dedup_candidates_unique_surname();
                    let t4b = store.find_dedup_candidates_middle_drop();
                    t1.into_iter()
                        .chain(t2.into_iter().map(|(a, b, _)| (a, b)))
                        .chain(t3.into_iter().map(|(a, b, _)| (a, b)))
                        .chain(t4a.into_iter().map(|(a, b, _)| (a, b)))
                        .chain(t4b.into_iter().map(|(a, b, _)| (a, b)))
                        .collect()
                };
                let relation_blocks = store.find_dedup_relation_blocks(&all_candidates);
                if !relation_blocks.is_empty() {
                    println!(
                        "  Relation blocks: {} pair(s) blocked by R1/R2 relation guards\n",
                        relation_blocks.len()
                    );
                }

                // ── Tier 1: exact normalized name matches ─────────────────
                let exact = store.find_dedup_candidates_exact();
                if exact.is_empty() {
                    println!("  Tier 1  no exact-name duplicates");
                } else {
                    println!("  Tier 1  {} exact-name duplicate(s):", exact.len());
                    if !dry_run {
                        for (alias_id, canonical_id) in &exact {
                            if relation_blocks
                                .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                            {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    blocked (relation guard) '{}' skipped", aname);
                                continue;
                            }
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            store.merge_entity_into(*alias_id, *canonical_id)?;
                            println!("    merged '{}' → '{}'", aname, cname);
                            total_merged += 1;
                            need_rebuild = true;
                        }
                    } else {
                        for (alias_id, canonical_id) in &exact {
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            println!("    '{}' → '{}'", aname, cname);
                        }
                    }
                }
                println!();

                // ── Tier 2: embedding similarity ──────────────────────────
                let candidates = store.find_dedup_candidates(threshold);
                if candidates.is_empty() {
                    println!("  Tier 2  no candidates above threshold {threshold:.2}");
                } else {
                    let total_cands = candidates.len();
                    println!(
                        "  Tier 2  {} candidate pair(s) above threshold {threshold:.2}",
                        total_cands
                    );

                    if dry_run {
                        println!();
                        for (i, (alias_id, canonical_id, sim)) in candidates.iter().enumerate() {
                            let a = store.get_entity(*alias_id);
                            let b = store.get_entity(*canonical_id);
                            if let (Some(a), Some(b)) = (a, b) {
                                let a_rels = store.neighbors_of(*alias_id).len();
                                let b_rels = store.neighbors_of(*canonical_id).len();
                                let guard = if relation_blocks
                                    .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                                {
                                    "  [BLOCKED:R1/R2]"
                                } else if store.dedup_desc_diverges(*alias_id, *canonical_id) {
                                    "  [BLOCKED:DESC]"
                                } else if store.dedup_r3_high_risk_surname(*alias_id, *canonical_id)
                                {
                                    "  [DEFERRED:R3]"
                                } else {
                                    ""
                                };
                                println!(
                                    "  {:>3}.  \"{}\"  (mentions={}, relations={}){}",
                                    i + 1,
                                    a.name,
                                    a.mention_count,
                                    a_rels,
                                    guard
                                );
                                println!(
                                    "         ↔  \"{}\"  (mentions={}, relations={})  sim={:.3}",
                                    b.name, b.mention_count, b_rels, sim
                                );
                            }
                        }
                    } else if auto {
                        let auto_threshold = auto_threshold.unwrap_or(0.97f32);
                        println!("  Auto-merging pairs with sim ≥ {auto_threshold:.2}…\n");
                        let mut tier2 = 0;
                        for (alias_id, canonical_id, sim) in &candidates {
                            if *sim < auto_threshold {
                                break; // sorted descending, can stop early
                            }
                            // Entity may have been absorbed in a prior iteration
                            if store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            // R1/R2 hard block
                            if relation_blocks
                                .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                            {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                let cname = store
                                    .get_entity(*canonical_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!(
                                    "    blocked (R1/R2) '{}' ↔ '{}'  sim={:.3}",
                                    aname, cname, sim
                                );
                                continue;
                            }
                            // Description-divergence block: descriptions present and clearly differ
                            if store.dedup_desc_diverges(*alias_id, *canonical_id) {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                let cname = store
                                    .get_entity(*canonical_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!(
                                    "    blocked (DESC) '{}' ↔ '{}'  sim={:.3}  — descriptions diverge",
                                    aname, cname, sim
                                );
                                continue;
                            }
                            // R3 soft downgrade: high-risk surname without matching relation
                            if store.dedup_r3_high_risk_surname(*alias_id, *canonical_id) {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                let cname = store
                                    .get_entity(*canonical_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    deferred (R3:surname) '{}' ↔ '{}'  sim={:.3}  — review manually", aname, cname, sim);
                                continue;
                            }
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            store.merge_entity_into(*alias_id, *canonical_id)?;
                            println!("    merged '{}' → '{}'  sim={:.3}", aname, cname, sim);
                            tier2 += 1;
                            need_rebuild = true;
                        }
                        let skipped = total_cands - tier2;
                        if skipped > 0 {
                            println!("    skipped {} pair(s) below {auto_threshold:.2}", skipped);
                        }
                        total_merged += tier2;
                    } else {
                        // Interactive review
                        println!("  [y=merge, n=skip, q=quit, ?=show relations]\n");
                        let mut quit = false;
                        for (i, (alias_id, canonical_id, sim)) in candidates.iter().enumerate() {
                            if quit {
                                break;
                            }
                            let a = match store.get_entity(*alias_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let b = match store.get_entity(*canonical_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let a_rels = store.neighbors_of(*alias_id).len();
                            let b_rels = store.neighbors_of(*canonical_id).len();

                            println!("  Candidate {}/{}:", i + 1, total_cands);
                            println!(
                                "  \"{}\"  (mentions={}, relations={})",
                                a.name, a.mention_count, a_rels
                            );
                            println!(
                                "    ↔  \"{}\"  (mentions={}, relations={})  sim={:.3}",
                                b.name, b.mention_count, b_rels, sim
                            );

                            loop {
                                use std::io::Write;
                                print!("  Merge? [y/n/q/?] ");
                                std::io::stdout().flush()?;
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                match line.trim() {
                                    "y" | "Y" => {
                                        store.merge_entity_into(*alias_id, *canonical_id)?;
                                        println!("    ✓ merged\n");
                                        total_merged += 1;
                                        need_rebuild = true;
                                        break;
                                    }
                                    "q" | "Q" => {
                                        println!("  Stopping.\n");
                                        quit = true;
                                        break;
                                    }
                                    "?" => {
                                        println!("  Relations — \"{}\":", a.name);
                                        for (nbr_id, rel, _) in
                                            store.neighbors_of(*alias_id).iter().take(6)
                                        {
                                            if let Some(nbr) = store.get_entity(*nbr_id) {
                                                println!("    → {} [{}]", nbr.name, rel);
                                            }
                                        }
                                        println!("  Relations — \"{}\":", b.name);
                                        for (nbr_id, rel, _) in
                                            store.neighbors_of(*canonical_id).iter().take(6)
                                        {
                                            if let Some(nbr) = store.get_entity(*nbr_id) {
                                                println!("    → {} [{}]", nbr.name, rel);
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("    skipped\n");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Tier 3: structural name patterns ──────────────────────
                let name_struct = store.find_dedup_candidates_name_structure();
                if name_struct.is_empty() {
                    println!("  Tier 3  no structural name duplicates");
                } else {
                    println!(
                        "  Tier 3  {} structural name candidate(s):",
                        name_struct.len()
                    );
                    if dry_run {
                        println!();
                        for (alias_id, canonical_id, reason) in &name_struct {
                            let a = store.get_entity(*alias_id);
                            let b = store.get_entity(*canonical_id);
                            if let (Some(a), Some(b)) = (a, b) {
                                let guard = if relation_blocks
                                    .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                                {
                                    "  [BLOCKED:R1/R2]"
                                } else if store.dedup_desc_diverges(*alias_id, *canonical_id) {
                                    "  [BLOCKED:DESC]"
                                } else if store.dedup_r3_high_risk_surname(*alias_id, *canonical_id)
                                {
                                    "  [DEFERRED:R3]"
                                } else {
                                    ""
                                };
                                println!(
                                    "        \"{}\"  →  \"{}\"  [{}]{}",
                                    a.name, b.name, reason, guard
                                );
                            }
                        }
                    } else if auto {
                        for (alias_id, canonical_id, reason) in &name_struct {
                            if store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            // R1/R2 hard block
                            if relation_blocks
                                .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                            {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    blocked (R1/R2) '{}' skipped  [{}]", aname, reason);
                                continue;
                            }
                            // Description-divergence block
                            if store.dedup_desc_diverges(*alias_id, *canonical_id) {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                let cname = store
                                    .get_entity(*canonical_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!(
                                    "    blocked (DESC) '{}' → '{}'  [{}]  — descriptions diverge",
                                    aname, cname, reason
                                );
                                continue;
                            }
                            // R3 soft downgrade for high-risk surnames
                            if store.dedup_r3_high_risk_surname(*alias_id, *canonical_id) {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                let cname = store
                                    .get_entity(*canonical_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    deferred (R3:surname) '{}' → '{}'  [{}]  — review manually", aname, cname, reason);
                                continue;
                            }
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            store.merge_entity_into(*alias_id, *canonical_id)?;
                            println!("    merged '{}' → '{}'  [{}]", aname, cname, reason);
                            total_merged += 1;
                            need_rebuild = true;
                        }
                    } else {
                        println!("  [y=merge, n=skip, q=quit]\n");
                        let mut quit = false;
                        for (alias_id, canonical_id, reason) in &name_struct {
                            if quit || store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            let a = match store.get_entity(*alias_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let b = match store.get_entity(*canonical_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            println!("  \"{}\"  →  \"{}\"  [{}]", a.name, b.name, reason);
                            loop {
                                use std::io::Write;
                                print!("  Merge? [y/n/q] ");
                                std::io::stdout().flush()?;
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                match line.trim() {
                                    "y" | "Y" => {
                                        store.merge_entity_into(*alias_id, *canonical_id)?;
                                        println!("    ✓ merged\n");
                                        total_merged += 1;
                                        need_rebuild = true;
                                        break;
                                    }
                                    "n" | "N" => {
                                        println!("    skipped\n");
                                        break;
                                    }
                                    "q" | "Q" => {
                                        quit = true;
                                        break;
                                    }
                                    _ => {
                                        println!("    please enter y, n, or q");
                                    }
                                }
                            }
                        }
                    }
                }
                println!();

                // ── Tier 4a: unique-surname dedup ─────────────────────────
                // "Mr Kies" → "Benjamin Maximilian Kies" when Kies is the only
                // entity in the graph with that surname.  Safe to auto-merge.
                let unique_surname = store.find_dedup_candidates_unique_surname();
                if unique_surname.is_empty() {
                    println!("  Tier 4a no unique-surname candidates");
                } else {
                    println!(
                        "  Tier 4a {} unique-surname candidate(s):",
                        unique_surname.len()
                    );
                    if dry_run {
                        println!();
                        for (alias_id, canonical_id, reason) in &unique_surname {
                            let a = store.get_entity(*alias_id);
                            let b = store.get_entity(*canonical_id);
                            if let (Some(a), Some(b)) = (a, b) {
                                let guard = if relation_blocks
                                    .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                                {
                                    "  [BLOCKED:R1/R2]"
                                } else {
                                    ""
                                };
                                println!(
                                    "        \"{}\"  →  \"{}\"  [{}]{}",
                                    a.name, b.name, reason, guard
                                );
                            }
                        }
                    } else if auto {
                        for (alias_id, canonical_id, reason) in &unique_surname {
                            if store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            if relation_blocks
                                .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                            {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    blocked (R1/R2) '{}'  [{}]", aname, reason);
                                continue;
                            }
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            store.merge_entity_into(*alias_id, *canonical_id)?;
                            println!("    merged '{}' → '{}'  [{}]", aname, cname, reason);
                            total_merged += 1;
                            need_rebuild = true;
                        }
                    } else {
                        println!("  [y=merge, n=skip, q=quit]\n");
                        let mut quit = false;
                        for (alias_id, canonical_id, reason) in &unique_surname {
                            if quit || store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            let a = match store.get_entity(*alias_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let b = match store.get_entity(*canonical_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            println!("  \"{}\"  →  \"{}\"  [{}]", a.name, b.name, reason);
                            loop {
                                use std::io::Write;
                                print!("  Merge? [y/n/q] ");
                                std::io::stdout().flush()?;
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                match line.trim() {
                                    "y" | "Y" => {
                                        store.merge_entity_into(*alias_id, *canonical_id)?;
                                        println!("    ✓ merged\n");
                                        total_merged += 1;
                                        need_rebuild = true;
                                        break;
                                    }
                                    "n" | "N" => {
                                        println!("    skipped\n");
                                        break;
                                    }
                                    "q" | "Q" => {
                                        quit = true;
                                        break;
                                    }
                                    _ => println!("    please enter y, n, or q"),
                                }
                            }
                        }
                    }
                }
                println!();

                // ── Tier 4b: middle-name-drop dedup ───────────────────────
                // "Victor Wessels" → "Victor Walter Wesley Wessels" when the
                // (first, last) pair is unique.  Also catches bare first names
                // with a shared graph neighbour as confirmation.
                let middle_drop = store.find_dedup_candidates_middle_drop();
                if middle_drop.is_empty() {
                    println!("  Tier 4b no middle-name-drop candidates");
                } else {
                    println!(
                        "  Tier 4b {} middle-name-drop candidate(s):",
                        middle_drop.len()
                    );
                    if dry_run {
                        println!();
                        for (alias_id, canonical_id, reason) in &middle_drop {
                            let a = store.get_entity(*alias_id);
                            let b = store.get_entity(*canonical_id);
                            if let (Some(a), Some(b)) = (a, b) {
                                let guard = if relation_blocks
                                    .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                                {
                                    "  [BLOCKED:R1/R2]"
                                } else {
                                    ""
                                };
                                println!(
                                    "        \"{}\"  →  \"{}\"  [{}]{}",
                                    a.name, b.name, reason, guard
                                );
                            }
                        }
                    } else if auto {
                        for (alias_id, canonical_id, reason) in &middle_drop {
                            if store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            if relation_blocks
                                .contains(&kwaai_rag::graph::ord_pair(*alias_id, *canonical_id))
                            {
                                let aname = store
                                    .get_entity(*alias_id)
                                    .map(|n| n.name.clone())
                                    .unwrap_or_default();
                                println!("    blocked (R1/R2) '{}'  [{}]", aname, reason);
                                continue;
                            }
                            let aname = store
                                .get_entity(*alias_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            let cname = store
                                .get_entity(*canonical_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_default();
                            store.merge_entity_into(*alias_id, *canonical_id)?;
                            println!("    merged '{}' → '{}'  [{}]", aname, cname, reason);
                            total_merged += 1;
                            need_rebuild = true;
                        }
                    } else {
                        println!("  [y=merge, n=skip, q=quit]\n");
                        let mut quit = false;
                        for (alias_id, canonical_id, reason) in &middle_drop {
                            if quit || store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            let a = match store.get_entity(*alias_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let b = match store.get_entity(*canonical_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            println!("  \"{}\"  →  \"{}\"  [{}]", a.name, b.name, reason);
                            loop {
                                use std::io::Write;
                                print!("  Merge? [y/n/q] ");
                                std::io::stdout().flush()?;
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                match line.trim() {
                                    "y" | "Y" => {
                                        store.merge_entity_into(*alias_id, *canonical_id)?;
                                        println!("    ✓ merged\n");
                                        total_merged += 1;
                                        need_rebuild = true;
                                        break;
                                    }
                                    "n" | "N" => {
                                        println!("    skipped\n");
                                        break;
                                    }
                                    "q" | "Q" => {
                                        quit = true;
                                        break;
                                    }
                                    _ => println!("    please enter y, n, or q"),
                                }
                            }
                        }
                    }
                }
                println!();

                // ── Tier 5: neighbour containment (review only) ───────────
                // Alias must have ≤ 15 neighbours and ≥ 60% of them covered by
                // the canonical's neighbour set.  Never auto-merged — too noisy
                // in memoir-style texts; always shown for human review.
                let containment_cands = store.find_dedup_candidates_neighbor_containment(0.60, 3);
                if containment_cands.is_empty() {
                    println!("  Tier 5  no neighbour-containment candidates");
                } else {
                    println!(
                        "  Tier 5  {} containment candidate(s) (review only):",
                        containment_cands.len()
                    );
                    println!();
                    if dry_run || auto {
                        for (alias_id, canonical_id, score) in &containment_cands {
                            let a = store.get_entity(*alias_id);
                            let b = store.get_entity(*canonical_id);
                            if let (Some(a), Some(b)) = (a, b) {
                                println!(
                                    "        \"{}\"  →  \"{}\"  (containment: {:.0}%)",
                                    a.name,
                                    b.name,
                                    score * 100.0
                                );
                            }
                        }
                    } else {
                        println!("  [y=merge, n=skip, q=quit]\n");
                        let mut quit = false;
                        for (alias_id, canonical_id, score) in &containment_cands {
                            if quit || store.get_entity(*alias_id).is_none() {
                                continue;
                            }
                            let a = match store.get_entity(*alias_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            let b = match store.get_entity(*canonical_id).cloned() {
                                Some(e) => e,
                                None => continue,
                            };
                            println!(
                                "  \"{}\"  →  \"{}\"  (containment: {:.0}%)",
                                a.name,
                                b.name,
                                score * 100.0
                            );
                            loop {
                                use std::io::Write;
                                print!("  Merge? [y/n/q] ");
                                std::io::stdout().flush()?;
                                let mut line = String::new();
                                std::io::stdin().read_line(&mut line)?;
                                match line.trim() {
                                    "y" | "Y" => {
                                        store.merge_entity_into(*alias_id, *canonical_id)?;
                                        println!("    ✓ merged\n");
                                        total_merged += 1;
                                        need_rebuild = true;
                                        break;
                                    }
                                    "n" | "N" => {
                                        println!("    skipped\n");
                                        break;
                                    }
                                    "q" | "Q" => {
                                        quit = true;
                                        break;
                                    }
                                    _ => {
                                        println!("    please enter y, n, or q");
                                    }
                                }
                            }
                        }
                    }
                }
                println!();

                if need_rebuild {
                    store.rebuild_in_memory()?;
                }

                println!();
                if dry_run {
                    print_info("Dry-run — no changes made.");
                } else {
                    print_success(&format!(
                        "Dedup complete — {} entities merged",
                        total_merged
                    ));
                    println!(
                        "  Graph now: {} entities, {} relations",
                        store.node_count(),
                        store.relation_count()
                    );
                    if total_merged > 0 {
                        println!(
                            "  Tip: run `kwaainet rag graph reembed --kb {kb}` to re-embed updated alias sets."
                        );
                    }
                }
            }

            GraphAction::Reembed { embed_url } => {
                print_box_header(&format!("Graph Reembed ({})", kb));
                let embed_url_str = embed_url.as_deref().unwrap_or("");
                let embed = EmbedClient::new(
                    if embed_url_str.is_empty() {
                        None
                    } else {
                        Some(embed_url_str.to_string())
                    },
                    Some(rag_cfg.embed_model.clone()),
                );
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                println!("  Entities to re-embed: {}", store.node_count());
                let n = store.reembed_all(&embed).await?;
                print_success(&format!(
                    "Re-embedded {n} entities with name+description text."
                ));
                println!("  Graph entity search now includes name tokens in the embedding.\n");
            }

            GraphAction::ChunkTag {
                embed_url,
                max_tags,
                restore,
            } => {
                let mode_label = if restore { "Restore" } else { "Chunk-Tag" };
                print_box_header(&format!("Graph {} ({})", mode_label, kb));
                let embed_url_str = embed_url.as_deref().unwrap_or("");
                let embed = EmbedClient::new(
                    if embed_url_str.is_empty() {
                        None
                    } else {
                        Some(embed_url_str.to_string())
                    },
                    Some(rag_cfg.embed_model.clone()),
                );
                let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening meta store")?;

                // Build (chunk_id, tag_prefix) pairs: up to max_tags entities per chunk.
                let mut pairs = graph.chunk_entity_tag_prefixes(max_tags);
                pairs.sort_by_key(|(cid, _)| *cid);
                let total = pairs.len();
                println!("  Chunks linked to entities: {total}");

                if total == 0 {
                    println!("  Nothing to tag — run `rag graph build` first.\n");
                    return Ok(());
                }

                // Collect all chunk IDs and resolve texts from MetaStore.
                let ids: Vec<i64> = pairs.iter().map(|(cid, _)| *cid).collect();
                let metas = meta.get_chunks(&ids).context("reading chunk metadata")?;

                // Build embed texts: tagged or raw depending on --restore.
                let tagged: Vec<(i64, String)> = pairs
                    .iter()
                    .zip(metas.iter())
                    .filter_map(|((cid, entity_name), opt_meta)| {
                        let m = opt_meta.as_ref()?;
                        let text = if restore {
                            m.text.clone()
                        } else {
                            format!("[{entity_name}] {}", m.text)
                        };
                        Some((*cid, text))
                    })
                    .collect();

                let found = tagged.len();
                let missing = total - found;
                if missing > 0 {
                    println!("  Chunks missing from meta store (skipped): {missing}");
                }
                let action_verb = if restore { "Restoring" } else { "Tagging" };
                if restore {
                    println!("  {action_verb} and re-embedding {found} chunks…\n");
                } else {
                    println!("  {action_verb} and re-embedding {found} chunks (up to {max_tags} entity tags each)…\n");
                }

                // Embed and upload in batches of 32.
                const BATCH: usize = 32;
                let mut uploaded_total = 0usize;
                let batches: Vec<&[(i64, String)]> = tagged.chunks(BATCH).collect();
                let n_batches = batches.len();

                for (bi, batch) in batches.into_iter().enumerate() {
                    let texts: Vec<&str> = batch.iter().map(|(_, t)| t.as_str()).collect();
                    let embeddings = embed
                        .embed_batch(&texts)
                        .await
                        .with_context(|| format!("embedding batch {}/{n_batches}", bi + 1))?;
                    let vectors: Vec<(i64, Vec<f32>)> = batch
                        .iter()
                        .zip(embeddings)
                        .map(|((cid, _), emb)| (*cid, emb))
                        .collect();

                    let n = match rag_cfg.storage_url.as_deref() {
                        Some("local") => {
                            let vs = open_local_vs(&rag_cfg.data_dir())?;
                            vs.upload(tenant_id, &vectors).await?
                        }
                        Some(url) => {
                            let http = reqwest::Client::new();
                            http_upload_vectors(&http, url, tenant_id, vectors).await?
                        }
                        None => {
                            let ep = eve_peer_id(&rag_cfg)?;
                            let (client, _) = crate::vpk::p2p_connect().await?;
                            rpc_upload_vectors(&client, &ep, tenant_id, vectors).await?
                        }
                    };
                    uploaded_total += n;

                    let done = (bi + 1) * BATCH;
                    eprint!(
                        "\r  [{:>4}/{found}]  uploaded={uploaded_total}    ",
                        done.min(found)
                    );
                }
                eprintln!();

                if restore {
                    print_success(&format!(
                        "Restored {uploaded_total} chunk vectors to natural (untagged) embeddings."
                    ));
                    println!("  Entity-linked chunks are back to fresh-ingest embedding state.\n");
                } else {
                    print_success(&format!(
                        "Tagged and re-embedded {uploaded_total} chunk vectors."
                    ));
                    println!("  Entity-linked chunks now carry [EntityName] prefix in their vector space.");
                    println!("  Run `rag eval` to measure recall improvement.\n");
                }
            }

            GraphAction::Seed { file, kb: _ } => {
                print_box_header(&format!("Graph Seed ({})", kb));

                let tree = family::load_family_tree(&file)
                    .with_context(|| format!("loading {}", file.display()))?;

                println!(
                    "  Loaded {} persons, {} relations from {}",
                    tree.persons.len(),
                    tree.relations.len(),
                    file.display()
                );

                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;

                println!(
                    "  Graph before: {} entities, {} relations\n",
                    store.node_count(),
                    store.relation_count()
                );

                let stats = family::seed_family_tree(&mut store, &tree, &embed, |msg| {
                    println!("  {msg}");
                })
                .await?;

                println!();
                let purge_note = if stats.relations_purged > 0 {
                    format!(
                        ", {} hallucinated parent/child edges purged",
                        stats.relations_purged
                    )
                } else {
                    String::new()
                };
                print_success(&format!(
                    "Seed complete — {} canonical entities upserted, {} aliases merged \
                     ({} relations re-pointed), {} family relations planted, {} aliases \
                     not found in graph{}",
                    stats.entities_upserted,
                    stats.aliases_merged,
                    stats.relations_merged,
                    stats.relations_upserted,
                    stats.aliases_not_found,
                    purge_note,
                ));
                println!(
                    "  Graph after:  {} entities, {} relations",
                    store.node_count(),
                    store.relation_count()
                );
                println!("\n  Tip: run `rag graph export` to view the updated graph in Obsidian.");
            }

            GraphAction::SeedFromJson { file, emit_yaml } => {
                print_box_header(&format!("Graph Seed from JSON ({})", kb));

                let payload = seed_json::load_nb_json(&file)
                    .with_context(|| format!("loading {}", file.display()))?;

                let (low_ents, low_rels) = seed_json::count_low_confidence(&payload);
                let total_ents = payload.entities.len();
                let total_rels = payload.relations.len();

                println!(
                    "  Loaded {} entities ({} low-confidence), {} relations ({} low-confidence skipped) from {}",
                    total_ents,
                    low_ents,
                    total_rels,
                    low_rels,
                    file.display()
                );

                // Optionally emit the converted YAML
                if let Some(yaml_path) = &emit_yaml {
                    let yaml = seed_json::to_seed_yaml(&payload);
                    std::fs::write(yaml_path, &yaml)
                        .with_context(|| format!("writing YAML to {}", yaml_path.display()))?;
                    print_success(&format!("Seed YAML written to {}", yaml_path.display()));
                }

                // Convert to FamilyTree and seed the graph directly
                let tree = seed_json::to_family_tree(&payload);

                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;

                println!(
                    "  Graph before: {} entities, {} relations\n",
                    store.node_count(),
                    store.relation_count()
                );

                let stats = family::seed_family_tree(&mut store, &tree, &embed, |msg| {
                    println!("  {msg}");
                })
                .await?;

                println!();
                print_success(&format!(
                    "Seed complete — {} canonical entities upserted, {} aliases merged \
                     ({} relations re-pointed), {} relations planted, {} aliases \
                     not found in graph",
                    stats.entities_upserted,
                    stats.aliases_merged,
                    stats.relations_merged,
                    stats.relations_upserted,
                    stats.aliases_not_found,
                ));
                println!(
                    "  Graph after:  {} entities, {} relations",
                    store.node_count(),
                    store.relation_count()
                );
                println!("\n  Tip: run `rag graph export` to view the updated graph in Obsidian.");
            }

            GraphAction::AliasScan {
                auto,
                dry_run,
                min_hits,
            } => {
                cmd_alias_scan(
                    &rag_cfg.data_dir(),
                    tenant_id,
                    &rag_cfg.embed_model,
                    auto,
                    dry_run,
                    min_hits,
                )
                .await?;
            }

            GraphAction::RemoveRelation { from, relation, to } => {
                print_box_header(&format!("Graph: Remove Relation ({})", kb));
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                let from_entity = store
                    .find_by_name_normalized(&from)
                    .map(|n| (n.id, n.name.clone(), n.entity_type.clone()));
                let to_entity = store
                    .find_by_name_normalized(&to)
                    .map(|n| (n.id, n.name.clone(), n.entity_type.clone()));
                match (from_entity, to_entity) {
                    (Some((fid, fname, _)), Some((tid, tname, _))) => {
                        let removed = store.delete_relation(fid, tid, &relation)?;
                        if removed {
                            println!("  ✅ Removed: '{}' --{}--> '{}'", fname, relation, tname);
                        } else {
                            println!(
                                "  ⚠️  Relation not found: '{}' --{}--> '{}'",
                                fname, relation, tname
                            );
                        }
                    }
                    (None, _) => println!("  ❌ Entity not found: '{}'", from),
                    (_, None) => println!("  ❌ Entity not found: '{}'", to),
                }
            }

            GraphAction::Coref {
                inference_url,
                model,
                sample,
                window,
                output,
                commit,
                no_llm,
            } => {
                cmd_coref(
                    &rag_cfg.data_dir(),
                    tenant_id,
                    &inference_url,
                    &model,
                    sample,
                    window,
                    output.as_deref(),
                    commit,
                    no_llm,
                )
                .await?;
            }

            GraphAction::ExtractRelations {
                inference_url,
                model,
                sample,
                output,
                commit,
                list_models,
                pull,
                rc,
                rc_window,
            } => {
                cmd_extract_relations(
                    &rag_cfg.data_dir(),
                    tenant_id,
                    &inference_url,
                    &model,
                    sample,
                    output.as_deref(),
                    commit,
                    list_models,
                    pull.as_deref(),
                    rc,
                    rc_window,
                )
                .await?;
            }

            GraphAction::Sanitize => {
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store for sanitize")?;
                if store.node_count() == 0 {
                    print_warning("Graph is empty — nothing to sanitize.");
                    return Ok(());
                }
                print_box_header(&format!("Graph Sanitize ({})", kb));
                println!("  Entities: {}", store.node_count());
                println!("  Relations before: {}", store.relation_count());
                println!();

                let spinner = crate::progress::Spinner::start("sanitizing…");
                let (removed, added, recomputed, gendered) = store.sanitize_relations()?;
                drop(spinner);

                println!("  Familial relations removed (non-Person endpoint): {removed}");
                println!("  Missing inverse / symmetric relations added:       {added}");
                println!("  Relation strengths recomputed from evidence:       {recomputed}");
                println!("  Person entities with gender inferred:              {gendered}");
                println!();
                println!("  Relations after: {}", store.relation_count());
                print_success("Sanitize complete. Run `graph score` to re-evaluate health.");
            }

            GraphAction::Score { top, json } => {
                let store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                if store.node_count() == 0 {
                    print_warning("Graph is empty. Run `kwaainet rag graph build` first.");
                    return Ok(());
                }
                let report = kwaai_rag::scorer::score_graph(&store);
                if json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else {
                    print_box_header(&format!("Graph Health Score ({})", kb));
                    println!(
                        "  Overall:      {:.1}%   ({} entities, {} relations)",
                        report.overall * 100.0,
                        report.entity_count,
                        report.relation_count,
                    );
                    println!(
                        "  Unknown type: {} entities need reclassification",
                        report.unknown_count
                    );
                    println!();

                    // Schema type distribution
                    let mut types: Vec<_> = report.by_schema_type.iter().collect();
                    types.sort_by_key(|b| std::cmp::Reverse(b.1));
                    println!("  Type distribution:");
                    for (t, n) in &types {
                        println!("    {:<32} {}", t, n);
                    }
                    println!();

                    // Worst-scoring entities table
                    println!("  Lowest-scoring entities (top {}):", top);
                    println!(
                        "  {:<32} {:<22} {:>5}  {:>7}  {:>8}  {:>8}",
                        "Name", "Schema type", "Type%", "Summary%", "Relatio%", "Overall%"
                    );
                    println!("  {}", "-".repeat(90));
                    for s in report.entity_scores.iter().take(top) {
                        let schema = s.schema_type.as_deref().unwrap_or("Unknown");
                        println!(
                            "  {:<32} {:<22} {:>5.0}  {:>7.0}  {:>8.0}  {:>8.0}",
                            truncate(&s.name, 32),
                            truncate(schema, 22),
                            s.type_score * 100.0,
                            s.summary_score * 100.0,
                            s.relation_score * 100.0,
                            s.overall * 100.0,
                        );
                    }
                    println!();

                    if !report.top_issues.is_empty() {
                        println!("  Issues:");
                        for issue in &report.top_issues {
                            print_warning(&format!("  {issue}"));
                        }
                    }
                }
            }

            GraphAction::SetMetadata { doc_schema } => {
                let schema = kwaai_rag::doc_schema::load_doc_schema(&doc_schema)
                    .context("loading doc schema")?;
                if schema.metadata.is_empty() {
                    print_warning("Doc schema has no metadata section — nothing to persist.");
                    return Ok(());
                }
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                store.set_doc_metadata(&schema.metadata)?;
                print_success(&format!(
                    "Persisted {} metadata key(s) into KB '{kb}':",
                    schema.metadata.len()
                ));
                for (k, v) in &schema.metadata {
                    println!("    {k}: {v}");
                }
            }

            GraphAction::Delete { name, entity_type } => {
                let eid = kwaai_rag::graph::entity_id(&name, &entity_type);
                let mut store = kwaai_rag::graph::GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                if store.get_entity(eid).is_none() {
                    print_error(&format!("Entity '{}' [{}] not found", name, entity_type));
                } else {
                    store.delete_entity(eid).context("deleting entity")?;
                    print_success(&format!("Deleted '{}' [{}]", name, entity_type));
                }
            }

            GraphAction::GhostPrune {
                with_relations,
                dry_run,
            } => {
                print_box_header(&format!("Ghost Prune ({})", kb));
                let mut store = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening meta store")?;

                // Build a lowercase text corpus from all MetaStore chunks for name-search.
                let all_chunk_texts: Vec<String> = meta
                    .all_chunks()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(_, m)| m.text.to_lowercase())
                    .collect();

                // Identify ghost entities: name not found in any source chunk text.
                // All NER-extracted entities have chunk links (the LLM linked them to a chunk
                // even when hallucinating), so chunk-link presence is not a useful ghost signal.
                // The definitive test is whether the name actually appears in any chunk text.
                let candidates: Vec<(i64, String, usize)> = store
                    .all_entities()
                    .map(|e| (e.id, e.name.clone(), store.neighbors_of(e.id).len()))
                    .collect();

                let mut ghosts: Vec<i64> = Vec::new();
                let mut skipped_has_relations = 0usize;

                for (eid, name, neighbor_count) in &candidates {
                    // Names shorter than 4 chars are ambiguous — never prune them.
                    let name_lower: String = name.to_lowercase();
                    if name_lower.len() < 4 {
                        continue;
                    }
                    // Normalize: remove periods so "Mr. Smith" matches "Mr Smith" in source text.
                    let name_normalized: String = name_lower
                        .replace('.', " ")
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");
                    // Keep if name (or period-stripped variant) appears in any chunk text.
                    if all_chunk_texts.iter().any(|t| {
                        t.contains(name_lower.as_str()) || t.contains(name_normalized.as_str())
                    }) {
                        continue;
                    }
                    // Ghost candidate: name not found in any source text.
                    if *neighbor_count > 0 && !with_relations {
                        skipped_has_relations += 1;
                        continue;
                    }
                    ghosts.push(*eid);
                }

                let total_entities = candidates.len();
                println!("  Total entities : {total_entities}");
                println!("  Ghost candidates: {}", ghosts.len());
                if skipped_has_relations > 0 {
                    println!(
                        "  Skipped (have relations, use --with-relations to include): {skipped_has_relations}"
                    );
                }

                if ghosts.is_empty() {
                    println!("\n  No ghost entities found.\n");
                    return Ok(());
                }

                if dry_run {
                    println!("\n  [dry-run] Would remove {} entities.\n", ghosts.len());
                    println!("  Re-run without --dry-run to apply.\n");
                    return Ok(());
                }

                print!("  Removing {} ghost entities…", ghosts.len());
                let _ = io::stdout().flush();
                let mut removed = 0usize;
                for eid in &ghosts {
                    if let Err(e) = store.delete_entity(*eid) {
                        eprintln!("\n  Warning: failed to delete entity {eid}: {e}");
                    } else {
                        removed += 1;
                    }
                }
                println!(" done.");

                // Clean up any dangling edges that pointed to deleted entities.
                let dangling = store.prune_dangling_relations().unwrap_or_else(|e| {
                    eprintln!("  Warning: relation cleanup failed: {e}");
                    0
                });
                if dangling > 0 {
                    println!("  Dangling relations removed: {dangling}");
                }

                let remaining = store.all_entities().count();
                print_success(&format!(
                    "Removed {removed} ghost entities ({remaining} remaining)."
                ));
                if skipped_has_relations > 0 {
                    println!(
                        "  Tip: run with --with-relations to also remove {skipped_has_relations} connected ghosts.\n"
                    );
                }
                println!("  Run `rag eval` to measure accuracy impact.\n");
            }

            GraphAction::EnrichEntities {
                inference_url,
                inference_urls,
                model,
                workers,
                min_mentions,
                entity_types,
                limit,
                force,
                no_gender,
            } => {
                return cmd_enrich_entities(
                    inference_url,
                    inference_urls,
                    model,
                    workers,
                    min_mentions,
                    entity_types,
                    limit,
                    force,
                    !no_gender,
                    kb,
                )
                .await;
            }

            GraphAction::CorefMarriage { output, commit } => {
                #[cfg(not(feature = "storage"))]
                bail!("RAG requires the 'storage' feature.");

                #[cfg(feature = "storage")]
                {
                    let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
                    let mut store =
                        kwaai_rag::graph::GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                            .context("opening graph store")?;

                    let candidates = store
                        .infer_maiden_name_candidates()
                        .context("maiden name inference")?;

                    if candidates.is_empty() {
                        println!("No maiden-name merge candidates found.");
                        return Ok(());
                    }

                    // Collect names upfront to avoid borrow conflicts during merge
                    let resolved: Vec<(i64, i64, String, String, String)> = candidates
                        .iter()
                        .map(|(married_id, maiden_id, reason)| {
                            let mn = store
                                .get_entity(*married_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_else(|| "?".to_string());
                            let dn = store
                                .get_entity(*maiden_id)
                                .map(|n| n.name.clone())
                                .unwrap_or_else(|| "?".to_string());
                            (*married_id, *maiden_id, mn, dn, reason.clone())
                        })
                        .collect();

                    let mut lines: Vec<String> = Vec::new();
                    lines.push(format!(
                        "# Marriage-based maiden name merge proposals ({})\n",
                        resolved.len()
                    ));
                    lines.push(
                        "Each row: married entity · maiden-name entity · reason\n".to_string(),
                    );

                    let mut merge_count = 0usize;
                    for (married_id, maiden_id, married_name, maiden_name, reason) in &resolved {
                        lines.push(format!(
                            "- **{}** ← merge ← **{}**\n  _{}_",
                            married_name, maiden_name, reason
                        ));

                        if commit {
                            match store.merge_entity_into(*maiden_id, *married_id) {
                                Ok(n) => {
                                    println!(
                                        "  merged '{}' into '{}' ({} relations moved)",
                                        maiden_name, married_name, n
                                    );
                                    merge_count += 1;
                                }
                                Err(e) => {
                                    eprintln!(
                                        "  WARN: could not merge '{}' into '{}': {}",
                                        maiden_name, married_name, e
                                    );
                                }
                            }
                        }
                    }

                    let report = lines.join("\n");

                    if let Some(path) = output {
                        std::fs::write(&path, &report)?;
                        println!("Written to {}", path.display());
                    } else {
                        println!("{}", report);
                    }

                    if commit {
                        store
                            .rebuild_in_memory()
                            .context("rebuild after marriage merges")?;
                        println!("\n{} entities merged via marriage deduction.", merge_count);
                    } else {
                        println!(
                            "\n{} candidate(s) found. Re-run with --commit to merge.",
                            candidates.len()
                        );
                    }
                }
            }

            GraphAction::Export { output_dir } => {
                return cmd_export(output_dir, kb).await;
            }

            GraphAction::Import { input_dir, since } => {
                return cmd_import(input_dir, since, kb).await;
            }

            GraphAction::Timeline { action } => {
                return cmd_graph_timeline(action, &kb).await;
            }

            GraphAction::Unmerge {
                entity_type,
                canonical,
                alias,
                pairs_file,
            } => {
                #[cfg(not(feature = "storage"))]
                bail!("RAG requires the 'storage' feature.");

                #[cfg(feature = "storage")]
                {
                    let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
                    let mut store =
                        kwaai_rag::graph::GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                            .context("opening graph store")?;

                    // Build list of (canonical, alias) pairs to fix.
                    let pairs: Vec<(String, String)> = if let Some(path) = pairs_file {
                        std::fs::read_to_string(&path)
                            .with_context(|| format!("reading {}", path.display()))?
                            .lines()
                            .filter(|l| l.contains("<-"))
                            .map(|l| {
                                let mut parts = l.splitn(2, "<-");
                                let a = parts.next().unwrap_or("").trim().to_string();
                                let c = parts.next().unwrap_or("").trim().to_string();
                                (c, a)
                            })
                            .filter(|(c, a)| !c.is_empty() && !a.is_empty())
                            .collect()
                    } else {
                        match (canonical, alias) {
                            (Some(c), Some(a)) => vec![(c, a)],
                            _ => bail!("provide either --canonical + --alias or --pairs-file"),
                        }
                    };

                    print_box_header(&format!("Graph Unmerge ({})", kb));
                    let mut total = 0usize;
                    for (c, a) in &pairs {
                        match store.unmerge_alias(c, &entity_type, a) {
                            Ok(0) => println!("  ⚠️  alias '{}' not found on '{}'", a, c),
                            Ok(_) => {
                                println!("  ✅ split '{}' off '{}'", a, c);
                                total += 1;
                            }
                            Err(e) => println!("  ❌ '{}' <- '{}': {}", a, c, e),
                        }
                    }
                    println!();
                    println!("  {} alias(es) restored as stub entities.", total);
                    println!(
                        "  Run `kwaainet rag graph reembed --kb {}` to restore embeddings.",
                        kb
                    );
                }
            }
        }
        Ok(())
    }
}

// ── alias-scan ────────────────────────────────────────────────────────────────

/// Stop-words that can appear inside a proper noun phrase (e.g. "League *of* South Africa").
const PROPER_NOUN_STOP_WORDS: &[&str] = &[
    "of", "the", "and", "in", "for", "a", "an", "at", "to", "de", "van", "du", "by", "on", "la",
    "le", "los", "las",
];

/// Walk backwards through `words` from the end collecting a proper-noun phrase.
///
/// Rules:
///   - Capitalised words (ignoring leading/trailing punctuation) are always included.
///   - Stop-words are included only when sandwiched between capitalised words.
///   - Stop at the first lowercase non-stop-word.
///
/// Returns `Some(phrase)` (≥2 words, leading/trailing punctuation stripped) or `None`.
fn extract_proper_noun_phrase(words: &[&str]) -> Option<String> {
    if words.is_empty() {
        return None;
    }
    let n = words.len();
    let mut start = n;

    loop {
        if start == 0 {
            break;
        }
        let w = words[start - 1];
        let alpha: String = w.chars().filter(|c| c.is_alphabetic()).collect();
        if alpha.is_empty() {
            break;
        }
        let first = alpha.chars().next().unwrap();
        let lower = alpha.to_lowercase();

        if first.is_uppercase() || (PROPER_NOUN_STOP_WORDS.contains(&lower.as_str()) && start < n) {
            start -= 1;
        } else {
            break;
        }
    }

    if n - start < 2 {
        return None;
    }

    // Strip leading stop-words ("the National Union" → "National Union").
    let mut phrase_start = start;
    while phrase_start < n {
        let alpha: String = words[phrase_start]
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        if PROPER_NOUN_STOP_WORDS.contains(&alpha.to_lowercase().as_str()) {
            phrase_start += 1;
        } else {
            break;
        }
    }
    if n - phrase_start < 2 {
        return None;
    }

    // Join words and trim trailing punctuation from the phrase.
    let joined = words[phrase_start..n].join(" ");
    let trimmed = joined
        .trim_end_matches(|c: char| !c.is_alphanumeric())
        .to_string();
    Some(trimmed)
}

/// Check whether the leading initials of `phrase` (skipping stop-words) spell `abbr`.
fn initials_match_abbr(phrase: &str, abbr: &str) -> bool {
    let initials: String = phrase
        .split_whitespace()
        .filter(|w| {
            let alpha: String = w.chars().filter(|c| c.is_alphabetic()).collect();
            !PROPER_NOUN_STOP_WORDS.contains(&alpha.to_lowercase().as_str())
        })
        .filter_map(|w| w.chars().find(|c| c.is_alphabetic()))
        .map(|c| c.to_uppercase().next().unwrap_or(c))
        .collect();
    initials == abbr.to_uppercase()
}

/// Scan a chunk of text for abbreviation definitions using two patterns:
///
/// Pattern 1 — parenthetical: "Proper Name (ABBR)"
///   e.g. "New Era Fellowship (NEF)"
///
/// Pattern 2 — prose: "Proper Name, the ABBR" / "Proper Name, the ABBR for short"
///   e.g. "the Teachers' League of South Africa, the TLSA for short"
///
/// In both cases the abbreviation's initials must match the extracted proper noun —
/// this is the validation that eliminates false positives like "Jane Gool (NEUM)".
fn extract_alias_pairs(text: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();

    // ── Pattern 1: Full Name (ABBR) ──────────────────────────────────────────
    let mut search_from = 0;
    while let Some(open_rel) = text[search_from..].find('(') {
        let open = search_from + open_rel;
        let after_open = open + 1;

        let Some(close_rel) = text[after_open..].find(')') else {
            break;
        };
        let close = after_open + close_rel;
        let inside = text[after_open..close].trim();

        // ABBR must be 2–8 uppercase ASCII letters only.
        let is_abbr =
            (2..=8).contains(&inside.len()) && inside.chars().all(|c| c.is_ascii_uppercase());

        if is_abbr {
            if let Some(phrase) = extract_preceding_phrase(text, open) {
                if phrase.len() >= 5 && initials_match_abbr(&phrase, inside) {
                    pairs.push((inside.to_string(), phrase));
                }
            }
        }

        search_from = close + 1;
    }

    // ── Pattern 2: "Full Name, the ABBR [for short]" ─────────────────────────
    // Tokenise on whitespace and look for a standalone ABBR token preceded by
    // "the" or "or" (and optionally a comma before that).
    let words: Vec<&str> = text.split_whitespace().collect();
    for (idx, &word) in words.iter().enumerate() {
        // Strip trailing punctuation (comma, period, etc.) to get the bare token.
        let bare: String = word.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        let is_abbr = (2..=8).contains(&bare.len()) && bare.chars().all(|c| c.is_ascii_uppercase());
        if !is_abbr || idx == 0 {
            continue;
        }
        // Preceding word must be "the" or "or" (case-insensitive).
        let prev = words[idx - 1]
            .trim_matches(|c: char| !c.is_alphabetic())
            .to_lowercase();
        if prev != "the" && prev != "or" {
            continue;
        }
        // Extract the proper-noun phrase from words before the "the"/"or" trigger.
        let before_words = if idx >= 2 {
            &words[..idx - 1]
        } else {
            continue;
        };
        if let Some(phrase) = extract_proper_noun_phrase(before_words) {
            let phrase = strip_article(&phrase).to_string();
            if phrase.len() >= 5 && initials_match_abbr(&phrase, &bare) {
                pairs.push((bare, phrase));
            }
        }
    }

    pairs
}

/// Extract the proper-noun phrase immediately preceding byte offset `before_pos` in `text`.
/// Resets at sentence-ending punctuation so we don't grab cross-sentence fragments.
fn extract_preceding_phrase(text: &str, before_pos: usize) -> Option<String> {
    let before = text[..before_pos].trim_end();
    let frag_start = before
        .rfind(['.', '!', '?', ':', ';', '\n', '\r'])
        .map(|i| i + 1)
        .unwrap_or(0);
    let fragment = before[frag_start..].trim_start();
    let words: Vec<&str> = fragment.split_whitespace().collect();
    extract_proper_noun_phrase(&words).map(|p| strip_article(&p).to_string())
}

fn strip_article(s: &str) -> &str {
    for prefix in &["The ", "the ", "A ", "An ", "an "] {
        if let Some(rest) = s.strip_prefix(prefix) {
            return rest.trim();
        }
    }
    s.trim()
}

async fn cmd_alias_scan(
    data_dir: &std::path::Path,
    tenant_id: uuid::Uuid,
    embed_model: &str,
    auto: bool,
    dry_run: bool,
    min_hits: usize,
) -> Result<()> {
    use kwaai_rag::graph::normalize_name;
    use std::collections::HashMap;
    use std::io::Write;

    print_box_header("Graph Alias Scan");

    let meta = kwaai_rag::meta_store::MetaStore::open(data_dir, tenant_id)
        .context("opening meta store")?;
    let store =
        kwaai_rag::graph::GraphStore::open(data_dir, tenant_id).context("opening graph store")?;

    println!(
        "  Scanning {} chunks across {} entities…",
        meta.all_chunks()?.len(),
        store.node_count()
    );

    // ── Step 1: extract (abbr, full_name) pairs from all chunk texts ──────────
    // Map: normalized_abbr -> (canonical_full_name, hit_count)
    let mut pair_hits: HashMap<String, (String, usize)> = HashMap::new();

    for (_id, chunk) in meta.all_chunks()? {
        for (abbr, full) in extract_alias_pairs(&chunk.text) {
            let key = format!("{}|||{}", normalize_name(&abbr), normalize_name(&full));
            pair_hits
                .entry(key)
                .and_modify(|(_, count)| *count += 1)
                .or_insert((format!("{abbr}|||{full}"), 1));
        }
    }

    // Collect and sort by hit count descending.
    let mut pairs: Vec<(String, String, usize)> = pair_hits
        .into_values()
        .filter_map(|(raw, hits)| {
            if hits < min_hits {
                return None;
            }
            let mut parts = raw.splitn(2, "|||");
            let abbr = parts.next()?.to_string();
            let full = parts.next()?.to_string();
            Some((abbr, full, hits))
        })
        .collect();
    pairs.sort_by_key(|b| std::cmp::Reverse(b.2));

    println!(
        "  Found {} unique abbreviation definitions in source text\n",
        pairs.len()
    );

    // ── Step 2: match pairs against graph entities ────────────────────────────
    struct Candidate {
        abbr: String,
        full: String,
        abbr_id: i64,
        full_id: i64,
        hits: usize,
    }

    let mut candidates: Vec<Candidate> = Vec::new();

    for (abbr, full, hits) in &pairs {
        // Find the abbreviated entity.
        let abbr_entity = store
            .find_by_name_normalized(abbr)
            .or_else(|| store.find_by_name_normalized(&abbr.to_uppercase()));
        // Find the full-name entity (try normalized, then strip trailing punctuation variants).
        let full_entity = store.find_by_name_normalized(full).or_else(|| {
            // Try without trailing possessive / punctuation noise.
            let trimmed = full.trim_end_matches(|c: char| !c.is_alphanumeric());
            store.find_by_name_normalized(trimmed)
        });

        if let (Some(ae), Some(fe)) = (abbr_entity, full_entity) {
            if ae.id != fe.id {
                candidates.push(Candidate {
                    abbr: abbr.clone(),
                    full: full.clone(),
                    abbr_id: ae.id,
                    full_id: fe.id,
                    hits: *hits,
                });
            }
        }
    }

    if candidates.is_empty() {
        print_info("No alias pairs found that match graph entities.");
        return Ok(());
    }

    println!(
        "  {} candidate pairs match graph entities:\n",
        candidates.len()
    );
    println!("  {:<12} {:<40} {:>5}", "Abbr", "Full name", "Hits");
    println!("  {}", "-".repeat(60));
    for c in &candidates {
        println!(
            "  {:<12} {:<40} {:>5}",
            c.abbr,
            truncate(&c.full, 40),
            c.hits
        );
    }
    println!();

    if dry_run {
        print_info("Dry run — no merges performed.");
        return Ok(());
    }

    // ── Step 3: merge ─────────────────────────────────────────────────────────
    // Drop the read handle before acquiring the write handle — redb allows only one open handle.
    drop(store);
    let mut store = kwaai_rag::graph::GraphStore::open(data_dir, tenant_id)
        .context("opening graph store for writes")?;
    let mut merged = 0usize;
    let mut merged_canonical_ids: Vec<i64> = Vec::new();
    let stdin = std::io::stdin();

    for c in &candidates {
        // Refresh entity names in case earlier merges changed things.
        let abbr_still_exists = store.get_entity(c.abbr_id).is_some();
        let full_still_exists = store.get_entity(c.full_id).is_some();
        if !abbr_still_exists || !full_still_exists {
            continue;
        }

        let abbr_name = store
            .get_entity(c.abbr_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();
        let full_name = store
            .get_entity(c.full_id)
            .map(|n| n.name.clone())
            .unwrap_or_default();

        if auto {
            store.merge_entity_into(c.abbr_id, c.full_id)?;
            println!(
                "  merged '{}' → '{}'  ({} hits)",
                abbr_name, full_name, c.hits
            );
            merged_canonical_ids.push(c.full_id);
            merged += 1;
        } else {
            print!(
                "  Merge '{}' → '{}' ({} hits)?  [y/n/q] ",
                abbr_name, full_name, c.hits
            );
            let _ = std::io::stdout().flush();
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            match line.trim() {
                "y" | "Y" => {
                    store.merge_entity_into(c.abbr_id, c.full_id)?;
                    println!("    merged.");
                    merged_canonical_ids.push(c.full_id);
                    merged += 1;
                }
                "q" | "Q" => break,
                _ => println!("    skipped."),
            }
        }
    }

    if merged > 0 {
        store.rebuild_in_memory()?;
        // Re-embed only the merged canonical entities so their aliases (e.g. "TLSA")
        // are baked into the embedding — without this, abbreviation queries miss the
        // merged entity because the embedded text only contains the canonical name.
        let embed = EmbedClient::new(None, Some(embed_model.to_string()));
        let reembedded = store
            .reembed_entities(&merged_canonical_ids, &embed)
            .await?;
        print_success(&format!(
            "Alias scan complete — {} entities merged, {} re-embedded",
            merged, reembedded
        ));
    } else {
        print_info("No merges performed.");
    }

    Ok(())
}

// ── coref ─────────────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_coref(
    data_dir: &std::path::Path,
    tenant_id: uuid::Uuid,
    inference_url: &str,
    model: &str,
    sample: f64,
    window: usize,
    output: Option<&std::path::Path>,
    commit: bool,
    no_llm: bool,
) -> Result<()> {
    use std::collections::HashMap;
    use std::io::Write;

    print_box_header("Graph: Coreference Resolution");

    // Resolve p2p:// URLs the same way extract-relations does
    let mut _proxy_handles: Vec<tokio::task::JoinHandle<()>> = vec![];
    let inference_url: String = if inference_url.starts_with("p2p://")
        || inference_url.starts_with("mux://")
    {
        use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
        let sock =
            std::env::var("KWAAINET_SOCKET").unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        #[cfg(unix)]
        let addr = format!("/unix/{sock}");
        #[cfg(not(unix))]
        let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
        let p2p = std::sync::Arc::new(
            P2PClient::connect(&addr)
                .await
                .context("connecting to p2pd for p2p:// URL resolution")?,
        );
        let (resolved, handles) =
            crate::ollama_proxy::resolve_inference_urls(&[inference_url.to_string()], &p2p).await?;
        _proxy_handles = handles;
        resolved.into_iter().next().unwrap_or_default()
    } else {
        inference_url.to_string()
    };
    let inference_url = inference_url.as_str();

    let meta = kwaai_rag::meta_store::MetaStore::open(data_dir, tenant_id)
        .context("opening meta store")?;
    let mut store =
        kwaai_rag::graph::GraphStore::open(data_dir, tenant_id).context("opening graph store")?;

    let all_chunks = meta.all_chunks().context("loading chunks")?;
    let total_chunks = all_chunks.len();

    // Build chunk_id → ChunkMeta. Sort by chunk_index (document order), not by chunk_id
    // hash — adjacency must follow narrative order so ±window gives context sentences.
    let chunk_map: HashMap<i64, _> = all_chunks.into_iter().collect();
    let mut sorted_chunk_ids: Vec<i64> = chunk_map.keys().copied().collect();
    sorted_chunk_ids.sort_by_key(|id| chunk_map[id].chunk_index);

    // Sample chunks to process
    let n_sample = ((total_chunks as f64 * sample).ceil() as usize)
        .max(1)
        .min(total_chunks);
    let sampled_ids = &sorted_chunk_ids[..n_sample];

    println!(
        "  Total chunks: {total_chunks}  |  Sample ({}%): {n_sample}\n",
        (sample * 100.0).round()
    );

    let mut out = String::new();
    out.push_str(&format!(
        "# Coreference Resolution — {}% sample\n\n\
         **Model:** {model}  \n\
         **Commit:** {}  \n\
         **Window:** ±{window} chunks\n\n---\n\n",
        (sample * 100.0).round(),
        if commit { "yes" } else { "dry-run" }
    ));

    let mut total_resolved = 0usize;
    let mut total_links_added = 0usize;
    // Dedup surface: surface_text → Vec<(chunk_id, entity_id)> — for dedup reporting
    let mut coref_surface_map: HashMap<String, Vec<(i64, i64)>> = HashMap::new();

    for (i, &chunk_id) in sampled_ids.iter().enumerate() {
        let chunk = match chunk_map.get(&chunk_id) {
            Some(c) => c,
            None => continue,
        };

        // Build adjacent chunk IDs (±window), enforcing section-zone boundaries.
        // Adjacent chunks from a different section zone (e.g. Acknowledgements
        // next to Dedication) are excluded so their entities don't bleed into
        // the wrong chunk's candidate antecedent set.
        let pos = sorted_chunk_ids.partition_point(|&id| id < chunk_id);
        let adj_start = pos.saturating_sub(window);
        let adj_end = (pos + window + 1).min(sorted_chunk_ids.len());
        let center_section_type = &chunk.section_type;
        let adjacent: Vec<i64> = sorted_chunk_ids[adj_start..adj_end]
            .iter()
            .filter(|&&id| {
                id != chunk_id
                    && chunk_map
                        .get(&id)
                        .map(|c| center_section_type.same_window_zone(&c.section_type))
                        .unwrap_or(false)
            })
            .copied()
            .collect();

        // Get candidate antecedents from graph (Person, Place, Organization)
        let candidates = store.coref_candidates_for_chunk(chunk_id, &adjacent);
        let place_candidates = store.coref_typed_candidates_for_chunk(chunk_id, &adjacent, "place");
        let org_candidates =
            store.coref_typed_candidates_for_chunk(chunk_id, &adjacent, "organization");
        if candidates.is_empty() && place_candidates.is_empty() && org_candidates.is_empty() {
            continue;
        }

        // ── Tier 1a: definite description / alias matching ────────────────────
        let desc_resolutions =
            kwaai_rag::ner::resolve_definite_descriptions(&chunk.text, &candidates);

        // ── Tier 1b: gender + nearest-Person pronoun resolution ───────────────
        // Only resolve pronouns to entities whose name/alias appears in the current
        // chunk text. Entities from the ±window context are available for definite
        // descriptions (alias match), but pronoun resolution is much noisier — we
        // need the entity to be explicitly named nearby to anchor the pronoun.
        let chunk_lower = chunk.text.to_lowercase();
        let in_chunk_candidates: Vec<(String, Vec<String>, Option<String>)> = candidates
            .iter()
            .filter(|(name, aliases, _)| {
                let nl = name.to_lowercase();
                // Check if name or any ≥4-char alias token appears in chunk text
                nl.split_whitespace().any(|w| w.len() >= 4 && chunk_lower.contains(w))
                    || aliases.iter().any(|a| {
                        let al = a.to_lowercase();
                        al.split_whitespace().any(|w| w.len() >= 4 && chunk_lower.contains(w))
                    })
                    // Always include narrator (they narrate even when not explicitly named)
                    || aliases.iter().any(|a| {
                        matches!(a.to_lowercase().as_str(),
                            "narrator" | "author" | "i" | "the author" | "the narrator")
                    })
            })
            .cloned()
            .collect();
        let pronoun_resolutions =
            kwaai_rag::ner::resolve_pronouns_from_candidates(&chunk.text, &in_chunk_candidates);

        // Merge Tier 1 results; deduplicate by entity_name (one link per entity per chunk)
        let mut tier1: Vec<kwaai_rag::ner::CorefResolution> = Vec::new();
        let mut seen_entities: std::collections::HashSet<String> = std::collections::HashSet::new();
        for r in desc_resolutions.into_iter().chain(pronoun_resolutions) {
            if seen_entities.insert(r.entity_name.clone()) {
                tier1.push(r);
            }
        }

        // ── Tier 2: LLM-assisted (skipped when --no-llm) ─────────────────────
        let mut tier2: Vec<kwaai_rag::ner::CorefResolution> = Vec::new();
        if !no_llm {
            let tier1_surfaces: std::collections::HashSet<String> =
                tier1.iter().map(|r| r.surface.to_lowercase()).collect();
            let unresolved_pronouns = extract_unresolved_pronouns(&chunk.text, &tier1_surfaces);
            for pronoun in &unresolved_pronouns {
                if seen_entities.len() >= candidates.len() {
                    break;
                }
                let window_text = extract_pronoun_window(&chunk.text, pronoun, 300);
                let resolved =
                    call_llm_for_coref(inference_url, model, pronoun, &window_text, &candidates)
                        .await
                        .ok()
                        .flatten();
                if let Some(entity_name) = resolved {
                    if seen_entities.insert(entity_name.clone()) {
                        tier2.push(kwaai_rag::ner::CorefResolution {
                            surface: pronoun.clone(),
                            entity_name,
                            offset: 0,
                            confidence: 0.7,
                            method: "llm",
                        });
                    }
                }
            }
        }

        // ── Tier 1 (Place): spatial pronouns + definite descriptions ─────────
        if !place_candidates.is_empty() {
            let in_chunk_places: Vec<(String, Vec<String>)> = place_candidates
                .iter()
                .filter(|(name, aliases)| {
                    let nl = name.to_lowercase();
                    nl.split_whitespace()
                        .any(|w| w.len() >= 4 && chunk_lower.contains(w))
                        || aliases.iter().any(|a| {
                            let al = a.to_lowercase();
                            al.split_whitespace()
                                .any(|w| w.len() >= 4 && chunk_lower.contains(w))
                        })
                })
                .cloned()
                .collect();
            for r in kwaai_rag::ner::resolve_place_pronouns_from_candidates(
                &chunk.text,
                &in_chunk_places,
            ) {
                if seen_entities.insert(r.entity_name.clone()) {
                    tier1.push(r);
                }
            }
            // Tier 2: LLM for unresolved spatial pronouns
            if !no_llm && !in_chunk_places.is_empty() {
                let resolved_surfaces: std::collections::HashSet<String> =
                    tier1.iter().map(|r| r.surface.to_lowercase()).collect();
                for pronoun in &extract_unresolved_spatial_pronouns(&chunk.text, &resolved_surfaces)
                {
                    let window_text = extract_pronoun_window(&chunk.text, pronoun, 300);
                    let resolved = call_llm_for_place_coref(
                        inference_url,
                        model,
                        pronoun,
                        &window_text,
                        &in_chunk_places,
                    )
                    .await
                    .ok()
                    .flatten();
                    if let Some(entity_name) = resolved {
                        if seen_entities.insert(entity_name.clone()) {
                            tier2.push(kwaai_rag::ner::CorefResolution {
                                surface: pronoun.clone(),
                                entity_name,
                                offset: 0,
                                confidence: 0.7,
                                method: "place_llm",
                            });
                        }
                    }
                }
            }
        }

        // ── Tier 1 (Organization): definite descriptions ─────────────────────
        if !org_candidates.is_empty() {
            for r in kwaai_rag::ner::resolve_org_descriptions_from_candidates(
                &chunk.text,
                &org_candidates,
            ) {
                if seen_entities.insert(r.entity_name.clone()) {
                    tier1.push(r);
                }
            }
        }

        let all_resolutions: Vec<&kwaai_rag::ner::CorefResolution> =
            tier1.iter().chain(tier2.iter()).collect();

        if all_resolutions.is_empty() {
            continue;
        }

        total_resolved += all_resolutions.len();

        // ── Write review section ─────────────────────────────────────────────
        out.push_str(&format!("## Chunk {} (id={})\n\n", i + 1, chunk_id));
        if let Some(sec) = chunk.section_name.as_deref() {
            out.push_str(&format!("**Section:** {sec}  \n"));
        }
        out.push_str(&format!(
            "**Doc:** {}  chunk #{}\n\n",
            chunk.doc_name, chunk.chunk_index
        ));
        if !candidates.is_empty() {
            out.push_str("**Person candidates:**\n");
            for (name, aliases, gender) in &candidates {
                let g = gender.as_deref().unwrap_or("?");
                let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(3).collect();
                if shown.is_empty() {
                    out.push_str(&format!("  - {name} [{g}]\n"));
                } else {
                    out.push_str(&format!("  - {name} [{g}]  ({})\n", shown.join(", ")));
                }
            }
        }
        if !place_candidates.is_empty() {
            out.push_str("**Place candidates:**\n");
            for (name, aliases) in &place_candidates {
                let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(3).collect();
                if shown.is_empty() {
                    out.push_str(&format!("  - {name}\n"));
                } else {
                    out.push_str(&format!("  - {name}  ({})\n", shown.join(", ")));
                }
            }
        }
        if !org_candidates.is_empty() {
            out.push_str("**Org candidates:**\n");
            for (name, aliases) in &org_candidates {
                let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(3).collect();
                if shown.is_empty() {
                    out.push_str(&format!("  - {name}\n"));
                } else {
                    out.push_str(&format!("  - {name}  ({})\n", shown.join(", ")));
                }
            }
        }
        out.push_str("\n**Resolutions:**\n");
        for r in &all_resolutions {
            out.push_str(&format!(
                "  - `{}` → **{}**  conf={:.1}  [{}]\n",
                r.surface, r.entity_name, r.confidence, r.method
            ));
        }
        out.push('\n');

        // ── Commit links + track for dedup ───────────────────────────────────
        for r in &all_resolutions {
            // Look up entity ID by name
            let eid = store
                .find_by_name_normalized(&r.entity_name)
                .map(|n| kwaai_rag::graph::entity_id(&n.name, &n.entity_type));
            if let Some(eid) = eid {
                // Track surface → (chunk, entity) for dedup signal
                coref_surface_map
                    .entry(r.entity_name.clone())
                    .or_default()
                    .push((chunk_id, eid));

                if commit {
                    store.link_chunk(chunk_id, &[eid])?;
                    total_links_added += 1;
                }
            }
        }

        out.push_str("---\n\n");
        print!(
            "  [{}/{}] chunk {}  → {} resolved\r",
            i + 1,
            n_sample,
            chunk_id,
            all_resolutions.len()
        );
        let _ = std::io::stdout().flush();
    }
    println!();

    // ── Dedup surface analysis ────────────────────────────────────────────────
    // Report chunk-entity pairs where multiple entity stubs resolved to the same
    // referent — these are strong dedup candidates.
    let dedup_candidates: Vec<(&String, &Vec<(i64, i64)>)> = coref_surface_map
        .iter()
        .filter(|(_, pairs)| pairs.len() >= 2)
        .collect();

    if !dedup_candidates.is_empty() {
        out.push_str("## Coref-Derived Dedup Candidates\n\n");
        out.push_str("Entity stubs that resolved to the same referent in multiple chunks:\n\n");
        for (entity_name, pairs) in &dedup_candidates {
            out.push_str(&format!(
                "- **{}**: {} chunk(s) added via coref\n",
                entity_name,
                pairs.len()
            ));
        }
        out.push('\n');
    }

    // ── Summary ───────────────────────────────────────────────────────────────
    let summary = format!(
        "## Summary\n\n\
         | Metric | Value |\n\
         |--------|-------|\n\
         | Chunks processed | {n_sample} |\n\
         | Total resolutions | {total_resolved} |\n\
         | Links written to graph | {} |\n\
         | Dedup candidates surfaced | {} |\n",
        if commit { total_links_added } else { 0 },
        dedup_candidates.len()
    );
    out.push_str(&summary);

    println!("  Chunks processed:   {n_sample}");
    println!("  Resolutions found:  {total_resolved}");
    if commit {
        println!("  Links written:      {total_links_added}");
    }
    println!("  Dedup candidates:   {}", dedup_candidates.len());

    if let Some(path) = output {
        std::fs::write(path, &out).with_context(|| format!("writing {}", path.display()))?;
        println!("\n  ✅ Review written to: {}", path.display());
    } else {
        println!("\n{out}");
    }

    Ok(())
}

fn extract_unresolved_pronouns(
    text: &str,
    already_resolved: &std::collections::HashSet<String>,
) -> Vec<String> {
    const ALL_PRONOUNS: &[&str] = &["he", "him", "his", "she", "her", "they", "them", "their"];
    let lower = text.to_lowercase();
    ALL_PRONOUNS
        .iter()
        .filter(|&&p| !already_resolved.contains(p))
        .filter(|&&p| {
            // whole-word match
            lower.split_whitespace().any(|w| {
                let w = w.trim_matches(|c: char| !c.is_alphanumeric());
                w == p
            })
        })
        .map(|&p| p.to_string())
        .collect()
}

fn extract_pronoun_window(text: &str, pronoun: &str, window_chars: usize) -> String {
    let lower = text.to_lowercase();
    // Find byte position of first whole-word occurrence via char_indices
    let pos = lower
        .char_indices()
        .scan(0usize, |word_start, (i, c)| {
            if c.is_whitespace() {
                *word_start = i + c.len_utf8();
            }
            Some((i, *word_start))
        })
        .find_map(|(i, _)| {
            let remaining = &lower[i..];
            if remaining.starts_with(pronoun) {
                let after = i + pronoun.len();
                let boundary = after >= lower.len()
                    || !lower[after..].starts_with(|c: char| c.is_alphanumeric());
                if boundary {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(0);

    // Snap start/end to valid char boundaries
    let raw_start = pos.saturating_sub(window_chars);
    let raw_end = (pos + pronoun.len() + window_chars).min(text.len());
    let start = text
        .char_indices()
        .map(|(i, _)| i)
        .rfind(|&i| i <= raw_start)
        .unwrap_or(0);
    let end = text
        .char_indices()
        .map(|(i, _)| i)
        .chain(std::iter::once(text.len()))
        .find(|&i| i >= raw_end)
        .unwrap_or(text.len());
    text[start..end].to_string()
}

async fn call_llm_for_coref(
    inference_url: &str,
    model: &str,
    pronoun: &str,
    window_text: &str,
    candidates: &[(String, Vec<String>, Option<String>)],
) -> Result<Option<String>> {
    let candidate_block: String = candidates
        .iter()
        .enumerate()
        .map(|(i, (name, aliases, _))| {
            if aliases.is_empty() {
                format!("  {}. {name}", i + 1)
            } else {
                let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(2).collect();
                format!("  {}. {name}  ({})", i + 1, shown.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let names_list: Vec<&str> = candidates.iter().map(|(n, _, _)| n.as_str()).collect();
    let names_csv = names_list.join(", ");

    let prompt = format!(
        "In the following passage, who does \"{pronoun}\" refer to?\n\
         \n\
         Candidates:\n{candidate_block}\n\
         \n\
         Rules:\n\
         - Answer with the EXACT canonical name from the list: {names_csv}\n\
         - If uncertain or not determinable, answer \"none\"\n\
         \n\
         Return ONLY valid JSON: {{\"referent\": \"Exact Name\"}} or {{\"referent\": \"none\"}}\n\
         \n\
         Passage:\n{window_text}"
    );

    let raw = call_llm_for_relations(inference_url, model, &prompt).await?;

    // Parse referent
    let start = raw.find('{').unwrap_or(0);
    let end = raw.rfind('}').map(|e| e + 1).unwrap_or(raw.len());
    let json_str = &raw[start..end];
    let v: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    let referent = v["referent"].as_str().unwrap_or("none");
    if referent == "none" || referent.is_empty() {
        return Ok(None);
    }
    // Validate against candidate list
    let valid = names_list.contains(&referent);
    Ok(if valid {
        Some(referent.to_string())
    } else {
        None
    })
}

fn extract_unresolved_spatial_pronouns(
    text: &str,
    already_resolved: &std::collections::HashSet<String>,
) -> Vec<String> {
    const SPATIAL: &[&str] = &["there", "where"];
    let lower = text.to_lowercase();
    SPATIAL
        .iter()
        .filter(|&&p| !already_resolved.contains(p))
        .filter(|&&p| {
            lower.split_whitespace().any(|w| {
                let w = w.trim_matches(|c: char| !c.is_alphanumeric());
                w == p
            })
        })
        .map(|&p| p.to_string())
        .collect()
}

async fn call_llm_for_place_coref(
    inference_url: &str,
    model: &str,
    pronoun: &str,
    window_text: &str,
    candidates: &[(String, Vec<String>)],
) -> Result<Option<String>> {
    let candidate_block: String = candidates
        .iter()
        .enumerate()
        .map(|(i, (name, aliases))| {
            if aliases.is_empty() {
                format!("  {}. {name}", i + 1)
            } else {
                let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(2).collect();
                format!("  {}. {name}  ({})", i + 1, shown.join(", "))
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let names_list: Vec<&str> = candidates.iter().map(|(n, _)| n.as_str()).collect();
    let names_csv = names_list.join(", ");
    let prompt = format!(
        "In the following passage, what place or location does \"{pronoun}\" refer to?\n\
         \n\
         Candidates:\n{candidate_block}\n\
         \n\
         Rules:\n\
         - Answer with the EXACT canonical name from the list: {names_csv}\n\
         - If uncertain or not determinable, answer \"none\"\n\
         \n\
         Return ONLY valid JSON: {{\"referent\": \"Exact Name\"}} or {{\"referent\": \"none\"}}\n\
         \n\
         Passage:\n{window_text}"
    );
    let raw = call_llm_for_relations(inference_url, model, &prompt).await?;
    let start = raw.find('{').unwrap_or(0);
    let end = raw.rfind('}').map(|e| e + 1).unwrap_or(raw.len());
    let json_str = &raw[start..end];
    let v: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    let referent = v["referent"].as_str().unwrap_or("none");
    if referent == "none" || referent.is_empty() {
        return Ok(None);
    }
    let valid = names_list.contains(&referent);
    Ok(if valid {
        Some(referent.to_string())
    } else {
        None
    })
}

// ── extract-relations ─────────────────────────────────────────────────────────

/// Lexical triggers indicating a family relation may be explicitly stated.
/// These must appear adjacent to an actual person name in context — some (like
/// "aunt", "uncle") also appear in index entries and are handled by the index
/// filter separately.
const FAMILY_TRIGGERS: &[&str] = &[
    "wife",
    "husband",
    "married",
    "wed ",
    "spouse",
    "son ",
    "daughter",
    "father",
    "mother",
    "parent",
    "sister",
    "brother",
    "sibling",
    "niece",
    "nephew",
    " cousin",
    " aunt ",
    " uncle ",  // spaces prevent matching "Auntie" in index entries
    "child of", // "children of" removed — too often a metaphor ("children of District Six")
    "born to",
    "gave birth",
    "half-brother",
    "half-sister",
    "half-sibling",
    "stepson",
    "stepdaughter",
    "in-law",
];

/// Returns true if the chunk text contains at least one family trigger.
fn has_family_trigger(text: &str) -> bool {
    let lower = text.to_lowercase();
    FAMILY_TRIGGERS.iter().any(|&t| lower.contains(t))
}

/// Returns true if the chunk looks like an index or table-of-contents page.
/// These are characterised by many short lines that end with bare page numbers.
fn is_index_chunk(text: &str) -> bool {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() < 5 {
        return false;
    }
    // Count lines that end with digits (page numbers) or are mostly numeric
    let num_heavy = lines
        .iter()
        .filter(|l| {
            let t = l.trim();
            if t.is_empty() {
                return false;
            }
            // Ends with ", NNN" or " NNN" or just a number
            t.rsplit_once(|c: char| !c.is_ascii_digit())
                .map(|(_, suffix)| !suffix.is_empty() && suffix.len() <= 4)
                .unwrap_or(false)
        })
        .count();
    let ratio = num_heavy as f32 / lines.len() as f32;
    ratio >= 0.35 // >35% of lines end with page numbers → index
}

/// Returns the count of entity names (or their aliases) that appear literally in the text.
/// We use a simple case-insensitive substring match. Only entities with ≥1 word of ≥4 chars
/// in their name are checked (to avoid noise from very short names like "Naz").
fn count_entity_mentions_in_text(text: &str, entities: &[(String, Vec<String>)]) -> usize {
    let lower = text.to_lowercase();
    entities
        .iter()
        .filter(|(name, aliases)| {
            let name_lc = name.to_lowercase();
            if name_lc.split_whitespace().any(|w| w.len() >= 4) && lower.contains(&name_lc) {
                return true;
            }
            aliases.iter().any(|a| {
                let a_lc = a.to_lowercase();
                a_lc.split_whitespace().any(|w| w.len() >= 4) && lower.contains(&a_lc)
            })
        })
        .count()
}

/// Extract sentences from `text` that contain a family trigger AND at least one
/// entity name (or alias). Returns only those sentences — this is the "focused
/// context" that goes to the LLM instead of the full chunk.
///
/// A "sentence" is loosely defined as text delimited by `.`, `!`, `?`, or `\n`.
/// We also include the previous sentence for context (a name may appear just
/// before the sentence that contains the trigger).
#[allow(dead_code)]
fn extract_focused_sentences(text: &str, entities: &[(String, Vec<String>)]) -> String {
    // Split into rough sentences / clauses
    let raw_sents: Vec<&str> = text
        .split(['.', '!', '?', '\n'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let lower_text = text.to_lowercase();

    // Build entity name set (canonical + aliases ≥4 chars)
    let entity_tokens: Vec<String> = entities
        .iter()
        .flat_map(|(name, aliases)| {
            std::iter::once(name.as_str())
                .chain(aliases.iter().map(|a| a.as_str()))
                .filter(|s| s.split_whitespace().any(|w| w.len() >= 4))
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>()
        })
        .collect();

    let sent_has_trigger = |s: &str| {
        let sl = s.to_lowercase();
        FAMILY_TRIGGERS.iter().any(|&t| sl.contains(t))
    };
    let sent_has_entity = |s: &str| {
        let sl = s.to_lowercase();
        entity_tokens.iter().any(|t| sl.contains(t.as_str()))
    };

    // Collect indices of trigger sentences that also mention at least one entity
    let mut keep_idxs: Vec<usize> = Vec::new();
    for (i, s) in raw_sents.iter().enumerate() {
        if sent_has_trigger(s) && sent_has_entity(s) {
            // Include previous sentence for subject context
            if i > 0 {
                keep_idxs.push(i - 1);
            }
            keep_idxs.push(i);
            if i + 1 < raw_sents.len() {
                keep_idxs.push(i + 1);
            }
        }
    }
    keep_idxs.dedup();

    if keep_idxs.is_empty() {
        // Fallback: return the first 300 chars of the chunk
        return lower_text.chars().take(300).collect::<String>();
    }

    keep_idxs
        .into_iter()
        .map(|i| raw_sents[i])
        .collect::<Vec<_>>()
        .join(". ")
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
async fn cmd_extract_relations(
    data_dir: &std::path::Path,
    tenant_id: uuid::Uuid,
    inference_url: &str,
    model: &str,
    sample: f64,
    output: Option<&std::path::Path>,
    commit: bool,
    list_models: bool,
    pull: Option<&str>,
    rc_mode: bool,
    rc_window: usize,
) -> Result<()> {
    use std::collections::HashMap;
    use std::io::Write;

    print_box_header("Graph: Extract Family Relations");

    // Resolve p2p:// or mux:// URLs to local TCP proxies (same as dream/eval).
    let mut _proxy_handles: Vec<tokio::task::JoinHandle<()>> = vec![];
    let inference_url: String = if inference_url.starts_with("p2p://")
        || inference_url.starts_with("mux://")
    {
        use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
        let sock =
            std::env::var("KWAAINET_SOCKET").unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
        #[cfg(unix)]
        let addr = format!("/unix/{sock}");
        #[cfg(not(unix))]
        let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
        let p2p = std::sync::Arc::new(
            P2PClient::connect(&addr)
                .await
                .context("connecting to p2pd for p2p:// URL resolution")?,
        );
        let (resolved, handles) =
            crate::ollama_proxy::resolve_inference_urls(&[inference_url.to_string()], &p2p).await?;
        _proxy_handles = handles;
        println!(
            "  P2P proxy: {} → {}",
            inference_url,
            resolved.first().map(|s| s.as_str()).unwrap_or("?")
        );
        resolved.into_iter().next().unwrap_or_default()
    } else {
        inference_url.to_string()
    };
    let inference_url = inference_url.as_str();

    // --pull <MODEL>: pull a model on the (remote) endpoint and stream progress
    if let Some(model_to_pull) = pull {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3600)) // 1h — large model downloads
            .build()?;
        let pull_url = format!("{}/api/pull", inference_url.trim_end_matches('/'));
        println!("  Pulling '{}' on {} …", model_to_pull, inference_url);
        println!("  (streaming progress — Ctrl-C safe: pull continues on the remote node)\n");

        let body = serde_json::json!({"name": model_to_pull, "stream": true});
        let mut resp = client
            .post(&pull_url)
            .json(&body)
            .send()
            .await
            .context("sending pull request")?;

        let mut last_pct = 0u64;
        while let Some(chunk) = resp.chunk().await? {
            // Ollama streams NDJSON lines during a pull
            for line in chunk.split(|&b| b == b'\n') {
                if line.is_empty() {
                    continue;
                }
                if let Ok(v) = serde_json::from_slice::<serde_json::Value>(line) {
                    let status = v["status"].as_str().unwrap_or("");
                    if let (Some(completed), Some(total)) =
                        (v["completed"].as_u64(), v["total"].as_u64())
                    {
                        let pct = (completed * 100).checked_div(total).unwrap_or(0);
                        if pct != last_pct || pct == 100 {
                            let gb_done = completed as f64 / 1e9;
                            let gb_total = total as f64 / 1e9;
                            print!("\r  [{pct:3}%]  {gb_done:.1}/{gb_total:.1} GB  {status}      ");
                            let _ = std::io::Write::flush(&mut std::io::stdout());
                            last_pct = pct;
                        }
                    } else if !status.is_empty() {
                        println!("\r  {status}");
                    }
                }
            }
        }
        println!("\n  ✅ Pull complete: '{model_to_pull}' is now available on {inference_url}");
        return Ok(());
    }

    // --list-models: show available models on the target endpoint and exit
    if list_models {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        let tags_url = format!("{}/api/tags", inference_url.trim_end_matches('/'));
        let resp = client
            .get(&tags_url)
            .send()
            .await
            .context("calling /api/tags")?;
        let raw: serde_json::Value = resp.json().await.context("parsing /api/tags")?;
        let models = raw["models"]
            .as_array()
            .map(|a| a.as_slice())
            .unwrap_or(&[]);
        println!(
            "  {} model(s) available on {}:",
            models.len(),
            inference_url
        );
        for m in models {
            let name = m["name"].as_str().unwrap_or("?");
            let size = m["size"]
                .as_u64()
                .map(|s| format!("  {:.1}GB", s as f64 / 1e9))
                .unwrap_or_default();
            println!("    • {name}{size}");
        }
        return Ok(());
    }

    let meta = kwaai_rag::meta_store::MetaStore::open(data_dir, tenant_id)
        .context("opening meta store")?;
    let mut store =
        kwaai_rag::graph::GraphStore::open(data_dir, tenant_id).context("opening graph store")?;

    let all_chunks = meta.all_chunks().context("loading chunks")?;
    let total_chunks = all_chunks.len();

    // Build chunk_id → ChunkMeta map for fast lookup
    let chunk_map: HashMap<i64, _> = all_chunks.into_iter().collect();

    // Find chunks linked to ≥2 graph entities, passing all pre-filters
    let mut qualifying: Vec<(i64, Vec<(String, Vec<String>)>)> = Vec::new();
    let mut stats_index = 0usize;
    let mut stats_no_trigger = 0usize;
    let mut stats_too_few_mentions = 0usize;

    for (chunk_id, entity_ids) in store.all_chunk_entity_pairs() {
        if entity_ids.len() < 2 {
            continue;
        }
        let Some(chunk) = chunk_map.get(&chunk_id) else {
            continue;
        };

        // Guard 1: skip index / table-of-contents pages
        if is_index_chunk(&chunk.text) {
            stats_index += 1;
            continue;
        }

        // Guard 2: lexical trigger required
        if !has_family_trigger(&chunk.text) {
            stats_no_trigger += 1;
            continue;
        }

        // Collect (canonical_name, aliases) for each entity in this chunk
        let mut entities: Vec<(String, Vec<String>)> = entity_ids
            .iter()
            .filter_map(|&eid| store.get_entity(eid))
            .map(|n| (n.name.clone(), n.aliases.clone()))
            .collect();
        entities.sort_by(|a, b| a.0.cmp(&b.0));

        // Guard 3: ≥2 entity names/aliases must appear literally in the text
        // This prevents the LLM from hallucinating relations between co-listed
        // entities that happen to be in the same chunk via graph links but aren't
        // both mentioned in the visible text.
        let n_mentions = count_entity_mentions_in_text(&chunk.text, &entities);
        if n_mentions < 2 {
            stats_too_few_mentions += 1;
            continue;
        }

        qualifying.push((chunk_id, entities));
    }

    // Sort by chunk_id for reproducibility, then sample
    qualifying.sort_by_key(|(cid, _)| *cid);
    let n_sample = ((qualifying.len() as f64 * sample).ceil() as usize).max(1);
    let sampled = &qualifying[..n_sample.min(qualifying.len())];

    println!(
        "  Total chunks:         {total_chunks}\n\
           Filtered (index):     {stats_index}\n\
           Filtered (no trigger):{stats_no_trigger}\n\
           Filtered (<2 mentions):{stats_too_few_mentions}\n\
           Qualifying:           {}\n\
           Sample ({}%):         {}\n",
        qualifying.len(),
        (sample * 100.0).round(),
        sampled.len()
    );

    if sampled.is_empty() {
        println!("  No qualifying chunks found.");
        return Ok(());
    }

    // ── Build the review output ──────────────────────────────────────────────
    let mut out = String::new();
    out.push_str(&format!(
        "# Family Relation Extraction — {}% sample\n\n",
        (sample * 100.0).round()
    ));
    out.push_str(&format!(
        "**Qualifying chunks (≥2 entities + trigger):** {}  \n\
         **Sampled:** {}  \n\
         **Model:** {}  \n\
         **Commit:** {}\n\n---\n\n",
        qualifying.len(),
        sampled.len(),
        model,
        if commit { "yes" } else { "dry-run" }
    ));

    let mut total_extracted = 0usize;
    let mut total_new = 0usize;

    for (i, (chunk_id, entities)) in sampled.iter().enumerate() {
        let chunk = match chunk_map.get(chunk_id) {
            Some(c) => c,
            None => continue,
        };

        // Build entity block: canonical name + top aliases.
        // Filter out pure role-description entities (no proper name tokens ≥4 chars
        // that aren't honorifics/roles). These are ghost extractions like "My mother",
        // "Uncle Hanief", "The author" — they confuse the EC by appearing as endpoints.
        const ROLE_WORDS: &[&str] = &[
            "my",
            "the",
            "his",
            "her",
            "our",
            "their",
            "your",
            "uncle",
            "aunt",
            "mother",
            "father",
            "brother",
            "sister",
            "grandpa",
            "grandma",
            "grandfather",
            "grandmother",
            "narrator",
            "author",
            "writer",
        ];
        let is_role_only = |name: &str| -> bool {
            let words: Vec<&str> = name.split_whitespace().collect();
            words.iter().all(|w| {
                let wl = w.to_lowercase();
                wl.len() < 4 || ROLE_WORDS.contains(&wl.as_str())
            })
        };

        // Keep entities that have at least one proper name token (not a role word)
        let proper_entities: Vec<&(String, Vec<String>)> = entities
            .iter()
            .filter(|(name, _)| !is_role_only(name))
            .collect();

        let entity_block: String = proper_entities
            .iter()
            .map(|(name, aliases)| {
                if aliases.is_empty() {
                    format!("  - {name}")
                } else {
                    let shown: Vec<&str> = aliases.iter().map(|a| a.as_str()).take(4).collect();
                    format!("  - {name}  (also: {})", shown.join(", "))
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let canonical_names: Vec<&str> = proper_entities.iter().map(|(n, _)| n.as_str()).collect();

        // Detect narrator entity: any entity whose aliases include "narrator", "author", or "I"
        let narrator_name: Option<&str> = entities.iter().find_map(|(name, aliases)| {
            let is_narrator = aliases.iter().any(|a| {
                matches!(
                    a.to_lowercase().as_str(),
                    "narrator" | "author" | "i" | "the author" | "the narrator"
                )
            });
            if is_narrator {
                Some(name.as_str())
            } else {
                None
            }
        });

        // ── Two-pass CC + EC extraction ──────────────────────────────────────
        //
        // CC pass: "Does this text explicitly state a family relation between
        // any of these people? If yes, quote the exact clause."
        // EC pass: fires only when CC returns a non-empty quote; extracts the
        // structured {from, relation, to} triple from that anchor.
        //
        // In RC mode (--rc): bypass the CC scan entirely. Instead, anchor on each
        // trigger-word occurrence in the text and expand a window of ±rc_window chars.
        // Each window that contains ≥1 entity token goes directly to EC. Analogous to
        // ingestion's EC pass (anchor on entity dossier); here we anchor on the
        // relation word rather than the entity.
        let rc_windows: Vec<(String, String)> = if rc_mode {
            extract_rc_windows(&chunk.text, entities.as_slice(), narrator_name, rc_window)
        } else {
            vec![]
        };

        // In RC mode: skip the CC LLM call entirely — RC windows replace the CC scan.
        // CC is only needed for non-RC mode and for display in the review output.
        let (cc_prompt_str, cc_raw, cc_quote) = if rc_mode {
            (String::new(), String::new(), None)
        } else {
            let p = build_cc_prompt(&chunk.text, &entity_block, &canonical_names, narrator_name);
            let r = call_llm_for_relations(inference_url, model, &p).await?;
            let q = parse_cc_quote(&r);
            (p, r, q)
        };
        let cc_prompt = cc_prompt_str.as_str();
        let _ = cc_prompt; // suppress unused warning

        // Name-anchor guard: the CC quote must contain at least one token (≥4 chars)
        // from a canonical entity name, alias, or narrator name. Quotes like "my mother"
        // with no named person don't anchor the EC — skip them to avoid hallucination.
        //
        // We check: canonical names, all aliases, and individual tokens of each (≥4 chars),
        // plus the narrator name. This catches "Fazil" (alias token of "Fazil Rassool"),
        // "Goolam" (token of "Goolam Gool"), etc.
        let all_name_tokens: Vec<String> = {
            let mut tokens = Vec::new();
            for &n in &canonical_names {
                tokens.push(n.to_lowercase());
                for w in n.split_whitespace() {
                    if w.len() >= 4 {
                        tokens.push(w.to_lowercase());
                    }
                }
            }
            // Also include entity aliases
            for (_name, aliases) in entities.iter() {
                for a in aliases.iter() {
                    tokens.push(a.to_lowercase());
                    for w in a.split_whitespace() {
                        if w.len() >= 4 {
                            tokens.push(w.to_lowercase());
                        }
                    }
                }
            }
            if let Some(n) = narrator_name {
                tokens.push(n.to_lowercase());
                for w in n.split_whitespace() {
                    if w.len() >= 4 {
                        tokens.push(w.to_lowercase());
                    }
                }
            }
            tokens
        };
        let cc_quote_anchored = cc_quote.as_ref().and_then(|q| {
            let ql = q.to_lowercase();
            let has_name = all_name_tokens.iter().any(|t| ql.contains(t.as_str()));
            if has_name {
                Some(q.as_str())
            } else {
                None
            }
        });

        // Code-level guard: if the CC quote only contains non-schema relation words
        // (aunt, uncle, nephew, niece, cousin) with no schema-covered relation word,
        // skip EC entirely. The 8b model ignores this instruction when embedded in the EC prompt.
        const NON_SCHEMA_RELS: &[&str] = &["aunt", "uncle", "nephew", "niece", "cousin"];
        const SCHEMA_REL_WORDS: &[&str] = &[
            "wife",
            "husband",
            "married",
            "wed",
            "spouse",
            "son",
            "daughter",
            "father",
            "mother",
            "parent",
            "sibling",
            "brother",
            "sister",
            "born to",
            "gave birth",
        ];
        let quote_is_non_schema = cc_quote_anchored.is_some_and(|q| {
            let ql = q.to_lowercase();
            NON_SCHEMA_RELS.iter().any(|&r| ql.contains(r))
                && !SCHEMA_REL_WORDS.iter().any(|&r| ql.contains(r))
        });

        // Skip EC if the CC quote contains no schema relation word.
        // Blocks family-group quotes like "the entire Abed family" which the CC
        // extracted despite the prompt example showing "the Gool family" → doesn't qualify.
        let quote_lacks_schema = cc_quote_anchored.is_some_and(|q| {
            let ql = q.to_lowercase();
            !SCHEMA_REL_WORDS.iter().any(|&r| ql.contains(r))
        });

        // Skip EC when the CC quote has fewer than 2 identifiable endpoints.
        //
        // An endpoint is identifiable when:
        //   (a) the entity's full canonical name OR any alias (≥4 chars) appears as a
        //       substring in the quote (avoids false-positive from shared 4-char tokens
        //       like "gool" which match many different Gool family members), OR
        //   (b) a first-person pronoun ("my"/"I"/"me") appears AND the narrator is known.
        //
        // This blocks two patterns:
        //   • pronoun-only: "his wife Wahida" → Wahida = 1 endpoint, "his" is unresolvable
        //   • genitive-missing-target: "Khadija's young brother" → Khadija = 1 endpoint,
        //     brother is unnamed; EC guesses the narrator
        //   • false-plural via shared token: "his wife Cissie Gool" → Goolam Gool matches
        //     via "gool" but is not actually named → correct count = 1 → blocked
        // Legitimate cases still pass:
        //   • "my brother Fazil"    → narrator(my) + Fazil = 2 endpoints
        //   • "Cissie married Abdul Hamid" → Cissie + Abdul = 2 endpoints
        //   • "JMH Gool and his wife Wahida" → JMH (alias match) + Wahida = 2 endpoints
        let quote_insufficient_endpoints = cc_quote_anchored.is_some_and(|q| {
            let ql = q.to_lowercase();
            // Match entity by: full canonical name OR any alias ≥4 chars as substring.
            // Tokens-only matching causes false plurals via shared short tokens ("gool").
            let named_count = proper_entities
                .iter()
                .filter(|(name, aliases)| {
                    let nl = name.to_lowercase();
                    // Full canonical name as substring
                    if ql.contains(&nl) {
                        return true;
                    }
                    // Any alias ≥4 chars as substring
                    if aliases.iter().any(|a| {
                        let al = a.to_lowercase();
                        al.len() >= 4 && ql.contains(&al)
                    }) {
                        return true;
                    }
                    // Fallback: any canonical token ≥5 chars as substring (avoids 4-char noise)
                    nl.split_whitespace()
                        .filter(|w| w.len() >= 5)
                        .any(|w| ql.contains(w))
                })
                .count();
            // First-person reference with a known narrator counts as one endpoint
            let has_first_person = narrator_name.is_some()
                && (ql.starts_with("my ")
                    || ql.contains(" my ")
                    || ql.starts_with("i ")
                    || ql.contains(" i ")
                    || ql.starts_with("me ")
                    || ql.contains(" me "));
            let total_endpoints = named_count + usize::from(has_first_person);
            total_endpoints < 2
        });

        // ── RC mode: run EC on each trigger window; CC already computed for review ─
        let (relations_json, extracted) = if rc_mode && !rc_windows.is_empty() {
            let mut all_rels = Vec::new();
            let mut all_ec = String::new();
            for (trigger, window) in &rc_windows {
                let wl = window.to_lowercase();
                // Skip non-schema triggers (aunt/uncle/etc.)
                let is_non_schema = NON_SCHEMA_RELS.iter().any(|&r| wl.contains(r))
                    && !SCHEMA_REL_WORDS.iter().any(|&r| wl.contains(r));
                if is_non_schema {
                    continue;
                }
                let ec_prompt =
                    build_ec_prompt(window, &entity_block, &canonical_names, narrator_name);
                let ec_raw = call_llm_for_relations(inference_url, model, &ec_prompt).await?;
                let rels = parse_relation_response(&ec_raw, &canonical_names);
                if !all_ec.is_empty() {
                    all_ec.push('\n');
                }
                all_ec.push_str(&format!("[trigger: {trigger}] "));
                all_ec.push_str(&ec_raw);
                all_rels.extend(rels);
            }
            // Deduplicate relations
            all_rels.sort();
            all_rels.dedup();
            (all_ec, all_rels)
        } else if let Some(quote) = cc_quote_anchored {
            if quote_is_non_schema {
                (String::from("[non-schema relation — EC skipped]"), vec![])
            } else if quote_lacks_schema {
                (
                    String::from("[no schema relation word in CC quote — EC skipped]"),
                    vec![],
                )
            } else if quote_insufficient_endpoints {
                (
                    String::from("[CC quote has fewer than 2 identifiable endpoints — EC skipped]"),
                    vec![],
                )
            } else {
                let ec_prompt =
                    build_ec_prompt(quote, &entity_block, &canonical_names, narrator_name);
                let ec_raw = call_llm_for_relations(inference_url, model, &ec_prompt).await?;
                let rels = parse_relation_response(&ec_raw, &canonical_names);
                (ec_raw, rels)
            }
        } else {
            (String::new(), vec![])
        };

        // ── Write review section ─────────────────────────────────────────────
        out.push_str(&format!(
            "## Chunk {} / {}  (id={})\n\n",
            i + 1,
            sampled.len(),
            chunk_id
        ));
        if let Some(doc) = chunk.section_name.as_deref() {
            out.push_str(&format!("**Section:** {doc}  \n"));
        }
        out.push_str(&format!(
            "**Doc:** {}  chunk #{}\n\n",
            chunk.doc_name, chunk.chunk_index
        ));
        out.push_str("**Entities in chunk:**\n");
        out.push_str(&entity_block);
        out.push_str("\n\n**Triggers found:** ");
        let triggers_hit: Vec<&str> = FAMILY_TRIGGERS
            .iter()
            .filter(|&&t| chunk.text.to_lowercase().contains(t))
            .copied()
            .collect();
        out.push_str(&triggers_hit.join(", "));

        if rc_mode {
            out.push_str(&format!("\n\n**RC windows ({}):**\n", rc_windows.len()));
            for (trigger, window) in &rc_windows {
                out.push_str(&format!(
                    "- trigger `{trigger}`: `{}`\n",
                    window
                        .chars()
                        .take(120)
                        .collect::<String>()
                        .replace('\n', " ")
                ));
            }
            out.push_str("\n\n**RC EC pass (raw):**\n```\n");
            out.push_str(relations_json.trim());
            out.push_str("\n```\n\n");
        } else {
            out.push_str("\n\n**CC pass (raw):**\n```json\n");
            out.push_str(cc_raw.trim());
            out.push_str("\n```\n\n**CC quote:** ");
            match &cc_quote {
                None => {
                    out.push_str("none — EC pass skipped\n\n");
                }
                Some(q) => {
                    out.push_str(&format!("`{q}`  "));
                    if cc_quote_anchored.is_some() {
                        out.push_str("✅ anchored → EC\n\n");
                    } else {
                        out.push_str("⚠️ no entity name in quote → EC skipped\n\n");
                    }
                }
            }
        }
        if !relations_json.is_empty() {
            out.push_str("**EC pass (raw):**\n```json\n");
            out.push_str(relations_json.trim());
            out.push_str("\n```\n\n");
        }

        if extracted.is_empty() {
            out.push_str("**Extracted relations:** none\n");
        } else {
            out.push_str("**Extracted relations:**\n");
            for (from, rel, to) in &extracted {
                out.push_str(&format!("- `{from}` **{rel}** `{to}`\n"));
            }
        }
        out.push_str("\n---\n\n");

        total_extracted += extracted.len();

        // ── Commit to graph if requested ─────────────────────────────────────
        // Family/personal relations only make sense between two Person entities.
        // Reject parent_of/sibling_of/spouse_of/child_of/married_to/born_to
        // when the target is not a Person — catches 8b model confusions like
        // "Yousuf Rassool parent_of Barnato Board" (a building).
        const PERSON_ONLY_RELS: &[&str] = &[
            "parent_of",
            "sibling_of",
            "spouse_of",
            "child_of",
            "married_to",
            "born_to",
        ];
        if commit && !extracted.is_empty() {
            for (from_name, rel_type, to_name) in &extracted {
                let from_node = store.find_by_name_normalized(from_name);
                let to_node = store.find_by_name_normalized(to_name);

                // Filter: person-only relation types require both ends to be Person.
                if PERSON_ONLY_RELS.contains(&rel_type.as_str()) {
                    let from_is_person = from_node
                        .as_ref()
                        .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                        .unwrap_or(false);
                    let to_is_person = to_node
                        .as_ref()
                        .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                        .unwrap_or(false);
                    if !from_is_person || !to_is_person {
                        tracing::debug!(
                            "filter: skipping {rel_type} ({from_name} → {to_name}): non-Person endpoint"
                        );
                        continue;
                    }
                }

                let from_id =
                    from_node.map(|n| kwaai_rag::graph::entity_id(&n.name, &n.entity_type));
                let to_id = to_node.map(|n| kwaai_rag::graph::entity_id(&n.name, &n.entity_type));
                if let (Some(fid), Some(tid)) = (from_id, to_id) {
                    let before = store.get_relation_strength(fid, tid, rel_type);
                    store.upsert_relation(fid, tid, rel_type, *chunk_id)?;
                    let after = store.get_relation_strength(fid, tid, rel_type);
                    if before.is_none() || (before.unwrap() - after.unwrap_or(0.0)).abs() > 0.01 {
                        total_new += 1;
                    }
                }
            }
        }

        let cc_status = if cc_quote.is_some() { "EC→" } else { "skip" };
        print!(
            "  [{}/{}] chunk {} [CC:{}] → {} relation(s)\r",
            i + 1,
            sampled.len(),
            chunk_id,
            cc_status,
            extracted.len()
        );
        let _ = std::io::stdout().flush();
    }
    println!();

    // ── Summary ──────────────────────────────────────────────────────────────
    let summary = format!(
        "\n## Summary\n\n\
         | Metric | Value |\n\
         |--------|-------|\n\
         | Chunks processed | {} |\n\
         | Relations extracted | {} |\n\
         | Relations written to graph | {} |\n",
        sampled.len(),
        total_extracted,
        if commit { total_new } else { 0 },
    );
    out.push_str(&summary);

    println!("  Chunks processed:  {}", sampled.len());
    println!("  Relations found:   {total_extracted}");
    if commit {
        println!("  Written to graph:  {total_new}");
    }

    // ── Write output ─────────────────────────────────────────────────────────
    if let Some(path) = output {
        std::fs::write(path, &out).with_context(|| format!("writing {}", path.display()))?;
        println!("\n  ✅ Review written to: {}", path.display());
    } else {
        println!("\n{out}");
    }

    Ok(())
}

/// CC pass: cheap broad scan.
/// Ask the LLM to scan the passage and quote any clause that EXPLICITLY DECLARES
/// a direct family relation between two named people from the list.
///
/// Key distinction: a DECLARATION names both people ("X is the wife of Y",
/// "Y's son Goolam", "married to Cissie"). A MENTION uses a role without
/// naming the person ("my mother was pleased", "his uncle arrived").
/// Only declarations proceed to EC.
/// RC (Relation-Centric) extraction: for each occurrence of a trigger word in
/// the chunk text, extract a window of ±`window_chars` characters around it.
///
/// Analogous to EC in ingestion: EC anchors on an entity and builds a dossier;
/// RC anchors on a relation trigger and expands to capture surrounding context.
///
/// Returns a Vec of (trigger_word, window_text) pairs for windows that contain
/// at least one entity name token — these go directly to the EC pass without a
/// CC scan (the trigger position is the anchor).
///
/// This catches cases like "my brother Fazil contributing towards the domestic
/// kitty" where the OCR fragments the sentence across a page break and the CC
/// scan misses the trigger because it doesn't look like a complete sentence.
fn extract_rc_windows(
    text: &str,
    entities: &[(String, Vec<String>)],
    narrator_name: Option<&str>,
    window_chars: usize,
) -> Vec<(String, String)> {
    let lower = text.to_lowercase();

    // Build set of name tokens to check window entity presence
    let name_tokens: Vec<String> = entities
        .iter()
        .flat_map(|(name, aliases)| {
            std::iter::once(name.as_str())
                .chain(aliases.iter().map(|a| a.as_str()))
                .flat_map(|s| s.split_whitespace())
                .filter(|w| w.len() >= 4)
                .map(|w| w.to_lowercase())
                .collect::<Vec<_>>()
        })
        .chain(
            narrator_name
                .into_iter()
                .flat_map(|n| n.split_whitespace())
                .filter(|w| w.len() >= 4)
                .map(|w| w.to_lowercase()),
        )
        .collect();

    let mut results: Vec<(String, String)> = Vec::new();
    let mut seen_windows: std::collections::HashSet<String> = std::collections::HashSet::new();

    for &trigger in FAMILY_TRIGGERS {
        let mut search_start = 0;
        while let Some(pos) = lower[search_start..].find(trigger) {
            let abs_pos = search_start + pos;
            let window_start = abs_pos.saturating_sub(window_chars);
            let window_end = (abs_pos + trigger.len() + window_chars).min(text.len());

            // Snap to UTF-8 character boundaries
            let window_start = text
                .char_indices()
                .map(|(i, _)| i)
                .rfind(|&i| i <= window_start)
                .unwrap_or(0);
            let window_end = text
                .char_indices()
                .map(|(i, _)| i)
                .chain(std::iter::once(text.len()))
                .find(|&i| i >= window_end)
                .unwrap_or(text.len());

            let window = &text[window_start..window_end];
            let window_lower = window.to_lowercase();

            // Only proceed if at least one entity name token appears in this window
            if name_tokens
                .iter()
                .any(|t| window_lower.contains(t.as_str()))
            {
                // Deduplicate by trigger+window content to avoid re-processing
                let key = format!("{trigger}||{}", &window_lower[..window_lower.len().min(80)]);
                if seen_windows.insert(key) {
                    results.push((trigger.trim().to_string(), window.to_string()));
                }
            }

            search_start = abs_pos + trigger.len();
            if search_start >= lower.len() {
                break;
            }
        }
    }

    results
}

fn build_cc_prompt(
    text: &str,
    entity_block: &str,
    canonical_names: &[&str],
    narrator_name: Option<&str>,
) -> String {
    let names_csv = canonical_names.join(", ");
    let narrator_line = match narrator_name {
        Some(name) => {
            format!("\nNARRATOR: \"{name}\" — 'I', 'me', 'my' in this text refer to this person.\n")
        }
        None => String::new(),
    };
    format!(
        "Scan this passage for a clause that identifies a direct family relationship \
         between two people — where at least one person from the list is named.\n\
         {narrator_line}\n\
         Persons: {names_csv}\n\
         Entity details:\n{entity_block}\n\
         \n\
         RULES — a clause QUALIFIES if it contains:\n\
         - A relationship word (wife, husband, married, son, daughter, father, mother, brother, \
           sister, sibling, cousin, uncle, aunt) AND\n\
         - At least one proper name from the persons list (or the narrator's name via 'I'/'my'/'me')\n\
         \n\
         QUALIFIES (quote these):\n\
         - \"my brother Fazil\" — role (brother) + name (Fazil)\n\
         - \"son of Goolam Gool\" — role (son) + name\n\
         - \"Cissie married Abdul Hamid\" — both names + relation word\n\
         - \"Gandhi's son Manilal\" — role + name\n\
         \n\
         DOES NOT QUALIFY (return none):\n\
         - \"my mother was pleased\" — role only, no name\n\
         - \"his uncle arrived\" — role only, no names\n\
         - \"they were brothers in arms\" — metaphorical\n\
         - \"the Gool family\" — family group, not a specific relation\n\
         - \"the entire Abed family\" — family group, not a specific relation\n\
         \n\
         CRITICAL QUOTING RULE: include BOTH named persons in the quote.\n\
         - Quote \"my brother Fazil\" — narrator 'my' + name (Fazil): both endpoints known\n\
         - Quote \"Cissie married Abdul Hamid\" — both names explicit\n\
         - Quote \"JMH Gool and his wife Wahida\" NOT just \"his wife Wahida\" — keep the \
           possessor's name so both endpoints are identified\n\
         - If a qualifying clause uses 'his'/'her' but the possessor's name does not appear \
           in the same clause, do NOT return that clause — return none instead.\n\
         \n\
         If a qualifying clause exists: quote it exactly, word-for-word from the passage.\n\
         If none exists: return none.\n\
         Return ONLY valid JSON:\n\
         {{\"quote\": \"exact words from passage\"}} or {{\"quote\": \"none\"}}\n\
         \n\
         Passage:\n{text}"
    )
}

/// Parse the CC pass response and return the quoted clause, or None if "none".
fn parse_cc_quote(raw: &str) -> Option<String> {
    let start = raw.find('{')?;
    let end = raw.rfind('}').map(|e| e + 1)?;
    let json_str = &raw[start..end];
    let v: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let quote = v["quote"].as_str()?;
    if quote.trim().to_lowercase() == "none" || quote.trim().is_empty() {
        None
    } else {
        Some(quote.to_string())
    }
}

/// EC pass: focused structured extraction anchored to the CC quote.
/// Only called when CC returned a non-empty quote anchored to a named entity.
fn build_ec_prompt(
    quote: &str,
    entity_block: &str,
    canonical_names: &[&str],
    narrator_name: Option<&str>,
) -> String {
    let names_csv = canonical_names.join(", ");
    let narrator_line = match narrator_name {
        Some(name) => format!("\nNARRATOR: \"{name}\" — 'I', 'me', 'my' refer to this person.\n"),
        None => String::new(),
    };
    format!(
        "Extract the family relationship stated in this exact quote.\n\
         {narrator_line}\n\
         Known persons:\n{entity_block}\n\
         \n\
         Quote: \"{quote}\"\n\
         \n\
         CRITICAL RULES:\n\
         - Use ONLY: spouse_of, parent_of, child_of, sibling_of, half_sibling_of\n\
         - \"from\" and \"to\" must be exact canonical names from: {names_csv}\n\
         - If the quote uses 'aunt', 'uncle', 'nephew', 'niece', or 'cousin', \
           return {{\"relations\":[]}} — these relation types are not in the schema\n\
         - 'I', 'me', 'my' (first person) → use the NARRATOR name as one endpoint\n\
         - 'his', 'her', 'their' (third person) are NOT automatically the narrator —\n\
           only use them if the possessor is explicitly named in the quote\n\
         - Extract ONLY what the quote directly states — no inference or deduction\n\
         - If either endpoint is not a canonical name in the quote, return empty\n\
         \n\
         Return ONLY valid JSON:\n\
         {{\"relations\":[{{\"from\":\"Name A\",\"relation\":\"sibling_of\",\"to\":\"Name B\"}}]}}\n\
         or {{\"relations\":[]}}"
    )
}

#[allow(dead_code)]
fn build_relation_prompt(
    text: &str,
    entity_block: &str,
    canonical_names: &[&str],
    narrator_name: Option<&str>,
) -> String {
    let names_csv = canonical_names.join(", ");
    let narrator_line = match narrator_name {
        Some(name) => format!(
            "\nNARRATOR: \"{name}\" — in this text, the pronouns 'I', 'me', 'my', 'we' \
             refer to this person. If a relation is stated using these pronouns \
             (e.g. \"my cousin\", \"my mother\"), use \"{name}\" as one endpoint.\n"
        ),
        None => String::new(),
    };
    format!(
        "You are extracting family relationships from a historical memoir passage.\n\
         {narrator_line}\n\
         The following persons MAY appear in this passage (some may not be named here):\n\
         {entity_block}\n\
         \n\
         Task: Identify ONLY direct family relationships BETWEEN the persons listed above.\n\
         \n\
         CRITICAL RULES:\n\
         - Use ONLY these relation types: spouse_of, parent_of, child_of, sibling_of, half_sibling_of\n\
         - \"from\" and \"to\" MUST be exact canonical names from this list: {names_csv}\n\
         - BOTH persons MUST be explicitly named OR clearly referred to via pronoun in the passage\n\
         - Do NOT connect persons who are not both mentioned/referenced in the passage text\n\
         - Do NOT guess or infer — only extract what is clearly and directly stated\n\
         - Do NOT extract: colleague_of, friend_of, associate_of, or any non-family relation\n\
         - Do NOT infer sibling_of just because two people share the same parent\n\
         - If the same relation appears multiple times, output it once\n\
         \n\
         Return ONLY valid JSON — no explanation, no markdown, no code block:\n\
         {{\"relations\":[{{\"from\":\"Canonical Name A\",\"relation\":\"spouse_of\",\"to\":\"Canonical Name B\"}}]}}\n\
         \n\
         If no relations are clearly stated between the listed persons, return: {{\"relations\":[]}}\n\
         \n\
         Passage:\n\
         {text}"
    )
}

async fn call_llm_for_relations(inference_url: &str, model: &str, prompt: &str) -> Result<String> {
    // Wrap with an explicit tokio timeout (120s) that fires even when the TCP connection
    // is still alive. The reqwest .timeout() alone is insufficient when the relay (p2p proxy)
    // holds the socket open while the remote 70b is generating — the TCP connection never
    // drops, so reqwest's deadline never fires.
    match tokio::time::timeout(
        std::time::Duration::from_secs(120),
        call_llm_for_relations_inner(inference_url, model, prompt),
    )
    .await
    {
        Ok(result) => result,
        Err(_) => {
            // Timeout — return empty JSON so the caller gracefully returns no relation
            Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"))
        }
    }
}

async fn call_llm_for_relations_inner(
    inference_url: &str,
    model: &str,
    prompt: &str,
) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(110))
        .build()?;

    #[derive(serde::Serialize)]
    struct Req<'a> {
        model: &'a str,
        messages: Vec<Msg<'a>>,
        stream: bool,
        options: Opts,
    }
    #[derive(serde::Serialize)]
    struct Msg<'a> {
        role: &'a str,
        content: &'a str,
    }
    #[derive(serde::Serialize)]
    struct Opts {
        temperature: f32,
        num_ctx: u32,
    }
    #[derive(serde::Deserialize)]
    struct Resp {
        message: Option<MsgResp>,
        error: Option<String>,
    }
    #[derive(serde::Deserialize)]
    struct MsgResp {
        content: String,
    }

    let url = format!("{}/api/chat", inference_url.trim_end_matches('/'));
    let body = Req {
        model,
        messages: vec![Msg {
            role: "user",
            content: prompt,
        }],
        stream: false,
        options: Opts {
            temperature: 0.0,
            num_ctx: 8192,
        },
    };

    // Network errors (relay stream resets, connection refused) → return empty result
    let http_resp = match client.post(&url).json(&body).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("LLM call failed (network): {e} — skipping chunk");
            return Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"));
        }
    };

    let status = http_resp.status();

    // Non-2xx responses (502 Bad Gateway from relay, etc.) → return empty result
    if !status.is_success() {
        tracing::warn!("LLM returned HTTP {status} — skipping chunk");
        return Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"));
    }

    let raw_body = match http_resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!("LLM response body read failed: {e} — skipping chunk");
            return Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"));
        }
    };

    let resp: Resp = match serde_json::from_slice(&raw_body) {
        Ok(r) => r,
        Err(_) => {
            tracing::warn!("LLM response parse failed — skipping chunk");
            return Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"));
        }
    };

    if let Some(err) = resp.error {
        tracing::warn!("LLM error: {err} — skipping chunk");
        return Ok(String::from("{\"quote\":\"none\",\"relations\":[]}"));
    }

    Ok(resp.message.map(|m| m.content).unwrap_or_default())
}

fn parse_relation_response(raw: &str, valid_names: &[&str]) -> Vec<(String, String, String)> {
    // Extract the JSON block (LLM sometimes wraps it in markdown)
    let json_str = if let Some(start) = raw.find('{') {
        let end = raw.rfind('}').map(|e| e + 1).unwrap_or(raw.len());
        &raw[start..end]
    } else {
        return vec![];
    };

    #[derive(serde::Deserialize)]
    struct Response {
        relations: Vec<RelationItem>,
    }
    #[derive(serde::Deserialize)]
    struct RelationItem {
        from: String,
        relation: String,
        to: String,
    }

    let Ok(resp) = serde_json::from_str::<Response>(json_str) else {
        return vec![];
    };

    const ALLOWED_RELS: &[&str] = &[
        "spouse_of",
        "parent_of",
        "child_of",
        "sibling_of",
        "half_sibling_of",
    ];

    let valid_set: std::collections::HashSet<&str> = valid_names.iter().copied().collect();

    resp.relations
        .into_iter()
        .filter(|r| {
            // Both endpoints must be in the known entity list
            valid_set.contains(r.from.as_str())
                && valid_set.contains(r.to.as_str())
                && r.from != r.to
                && ALLOWED_RELS.contains(&r.relation.as_str())
        })
        .map(|r| (r.from, r.relation, r.to))
        .collect()
}

// ── dream ─────────────────────────────────────────────────────────────────────

async fn cmd_dream(action: DreamAction, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let report_path = rag_cfg
            .data_dir()
            .join(format!("dream-report-{tenant_id}.json"));

        match action {
            DreamAction::Run {
                inference_url,
                inference_urls,
                model,
                threshold,
                dedup_threshold,
                max_completions,
                workers,
                prune_threshold,
                no_relations,
            } => {
                let raw_urls: Vec<String> = {
                    let mut v: Vec<String> = inference_urls
                        .as_deref()
                        .unwrap_or("")
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if v.is_empty() {
                        v.push(inference_url.unwrap_or_else(|| rag_cfg.inference_url.clone()));
                    }
                    v
                };

                // Resolve p2p:// URLs to local HTTP proxies (same pattern as graph build).
                let has_p2p = raw_urls
                    .iter()
                    .any(|u| u.starts_with("p2p://") || u.starts_with("mux://"));
                let (_proxy_handles, urls) = if has_p2p {
                    use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
                    let sock = std::env::var("KWAAINET_SOCKET")
                        .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
                    #[cfg(unix)]
                    let addr = format!("/unix/{sock}");
                    #[cfg(not(unix))]
                    let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
                    let p2p = std::sync::Arc::new(
                        P2PClient::connect(&addr)
                            .await
                            .context("connecting to p2pd for p2p:// URL resolution")?,
                    );
                    let (res, handles) =
                        crate::ollama_proxy::resolve_inference_urls(&raw_urls, &p2p).await?;
                    (handles, res)
                } else {
                    (vec![], raw_urls)
                };

                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                let cfg = kwaai_rag::dream::DreamConfig {
                    completeness_threshold: threshold,
                    dedup_threshold,
                    max_completions_per_cycle: max_completions,
                    workers,
                    prune_threshold,
                    no_relations,
                    ..Default::default()
                };

                print_box_header(&format!("Dream RAG ({})", kb));
                println!("  Inference: {}", urls.join(", "));
                println!("  Workers:   {workers}  |  Max completions: {max_completions}");
                println!(
                    "  Threshold: {:.0}%  |  Dedup: {dedup_threshold:.2}",
                    threshold * 100.0
                );
                println!();

                let report = kwaai_rag::dream::run_dream_cycle(
                    &rag_cfg.data_dir(),
                    tenant_id,
                    &embed,
                    &cfg,
                    &urls,
                    &model,
                    Some(|done: usize, total: usize, phase: &str| {
                        if total > 0 {
                            print!("\r  {phase}: {done}/{total}   ");
                            let _ = std::io::Write::flush(&mut std::io::stdout());
                        } else {
                            print!("\r  {phase}…   ");
                            let _ = std::io::Write::flush(&mut std::io::stdout());
                        }
                    }),
                )
                .await?;

                println!();
                print_success(&format!(
                    "Dream cycle complete in {:.1}s",
                    report.duration_secs
                ));
                println!(
                    "  Score:     {:.1}%  →  {:.1}%  ({:+.1}%)",
                    report.score_before * 100.0,
                    report.score_after * 100.0,
                    (report.score_after - report.score_before) * 100.0,
                );
                println!("  Type completions:     {}", report.entities_type_completed);
                println!(
                    "  Summary completions:  {}",
                    report.entities_summary_completed
                );
                println!(
                    "  Relations added:      {}",
                    report.entities_relations_added
                );
                println!("  Entities merged:      {}", report.entities_merged);
                println!("  Entities pruned:      {}", report.entities_pruned);
                if !report.cycle_errors.is_empty() {
                    println!();
                    print_warning(&format!("{} non-fatal errors:", report.cycle_errors.len()));
                    for e in report.cycle_errors.iter().take(5) {
                        print_warning(&format!("  {e}"));
                    }
                }

                std::fs::write(&report_path, serde_json::to_string_pretty(&report)?)?;
                println!("\n  Report saved: {}", report_path.display());
            }

            DreamAction::Status => {
                if !report_path.exists() {
                    print_warning("No dream report found. Run `kwaainet rag dream run` first.");
                } else {
                    let raw = std::fs::read_to_string(&report_path)?;
                    let report: kwaai_rag::dream::DreamReport = serde_json::from_str(&raw)?;
                    print_box_header(&format!("Last Dream Report ({})", kb));
                    println!("  Timestamp: {}", report.timestamp);
                    println!("  Duration:  {:.1}s", report.duration_secs);
                    println!(
                        "  Score:     {:.1}%  →  {:.1}%  ({:+.1}%)",
                        report.score_before * 100.0,
                        report.score_after * 100.0,
                        (report.score_after - report.score_before) * 100.0,
                    );
                    println!("  Type completions:     {}", report.entities_type_completed);
                    println!(
                        "  Summary completions:  {}",
                        report.entities_summary_completed
                    );
                    println!(
                        "  Relations added:      {}",
                        report.entities_relations_added
                    );
                    println!("  Entities merged:      {}", report.entities_merged);
                    println!("  Entities pruned:      {}", report.entities_pruned);
                    if !report.cycle_errors.is_empty() {
                        print_warning(&format!(
                            "{} errors during last cycle",
                            report.cycle_errors.len()
                        ));
                    }
                }
            }

            DreamAction::EmbedEval {
                max_queries,
                output,
                verbose,
                json,
            } => {
                let graph = kwaai_rag::graph::GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store for eval")?;
                let embed =
                    EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
                let vs = std::sync::Arc::new(open_local_vs(&rag_cfg.data_dir())?);

                if !json {
                    print_box_header(&format!("Dream RAG — Retrieval Eval ({})", kb));
                    let n = kwaai_rag::eval_retrieve::generate_eval_queries(
                        &graph,
                        max_queries.unwrap_or(usize::MAX),
                    )
                    .len();
                    println!("  Embed model: {}", rag_cfg.embed_model);
                    println!("  Queries:     {n}");
                    println!();
                }

                let spinner = if json {
                    None
                } else {
                    Some(crate::progress::Spinner::start("evaluating…"))
                };

                let report = kwaai_rag::eval_retrieve::evaluate_retrieval(
                    &graph,
                    &embed,
                    |emb, k| {
                        let vs = vs.clone();
                        async move {
                            let raw = vs.search(tenant_id, &emb, k).await?;
                            Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                        }
                    },
                    max_queries.unwrap_or(usize::MAX),
                    verbose,
                    if json {
                        None
                    } else {
                        Some(&|i, total| {
                            let _ = (i, total); // progress is shown via spinner
                        })
                    },
                )
                .await?;

                drop(spinner);

                if json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else {
                    println!("  Graph score:   {:.1}%", report.graph_score * 100.0);
                    println!("  Queries run:   {}", report.query_count);
                    println!(
                        "  Content queries (description-based): {:.0}%  Name queries: {:.0}%",
                        report.content_query_fraction * 100.0,
                        (1.0 - report.content_query_fraction) * 100.0
                    );
                    println!();
                    println!("  Entity-space retrieval (primary — query → entity embeddings):");
                    println!("    Recall@1:  {:.1}%", report.entity_recall_at_1 * 100.0);
                    println!("    Recall@3:  {:.1}%", report.entity_recall_at_3 * 100.0);
                    println!("    Recall@5:  {:.1}%", report.entity_recall_at_5 * 100.0);
                    println!("    Recall@10: {:.1}%", report.entity_recall_at_10 * 100.0);
                    println!("    MRR:       {:.3}", report.entity_mrr);
                    println!();
                    println!("  Chunk-space retrieval (lower bound — query → raw text chunks):");
                    println!("    Recall@1:  {:.1}%", report.chunk_recall_at_1 * 100.0);
                    println!("    Recall@5:  {:.1}%", report.chunk_recall_at_5 * 100.0);
                    println!("    Recall@10: {:.1}%", report.chunk_recall_at_10 * 100.0);
                    println!("    MRR:       {:.3}", report.chunk_mrr);
                }

                if let Some(path) = output {
                    std::fs::write(&path, serde_json::to_string_pretty(&report)?)?;
                    if !json {
                        print_success(&format!("Report saved: {}", path.display()));
                    }
                }
            }
        }
        Ok(())
    }
}

// ── cache ─────────────────────────────────────────────────────────────────────

async fn cmd_cache(action: CacheAction, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let mut cache =
            QueryCache::open(&rag_cfg.data_dir(), tenant_id).context("opening query cache")?;

        match action {
            CacheAction::Stats => {
                print_box_header(&format!("Query Cache ({})", kb));
                println!("  Entries:  {}", cache.entry_count());
                println!("  Hits:     {}", cache.total_hits());
                println!("  Expired:  {}", cache.expired_count());
                println!("  TTL:      {}h", cache.ttl_secs / 3600);
                println!("  Max:      {}", cache.max_entries);
                println!("  Threshold: {:.2}", cache.similarity_threshold);
            }
            CacheAction::Clear => {
                let removed = cache.clear()?;
                print_success(&format!("Cleared {removed} cached queries from '{kb}'"));
            }
        }
        Ok(())
    }
}

// ── eval ──────────────────────────────────────────────────────────────────────

/// Tokenise text into lowercase alphanumeric words (≥ 2 chars), ignoring punctuation.
fn eval_tokens(text: &str) -> std::collections::HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() >= 2)
        .map(|t| t.to_lowercase())
        .collect()
}

/// Token-overlap recall for one keyword phrase against a pre-built answer token set.
///
/// Rules (chosen to match RAG recall semantics):
/// - 1 token  → exact token match OR substring fallback (catches "India"→"Indian")
/// - 2 tokens → both tokens required OR substring fallback
/// - 3+ tokens → majority match: ⌈n/2⌉ tokens must appear (partial credit for long
///   phrases where the LLM uses a close variant, e.g. "All African" vs "All Africa")
///
/// The substring fallback ensures this metric is always ≥ the old exact-substring metric.
/// Numeric proximity score: finds the closest 4-digit year in the answer to `correct`
/// and returns a score in [0.0, 1.0] based on how close it is within `tolerance` years.
/// Exact match = 1.0; within tolerance = linear falloff; beyond = 0.0.
/// Used for questions where the answer is a quantity (e.g. a year) rather than a keyword.
fn numeric_proximity_score(answer: &str, correct: i64, tolerance: i64) -> f32 {
    let bytes = answer.as_bytes();
    let mut best_dist = i64::MAX;
    let mut i = 0;
    while i + 3 < bytes.len() {
        if bytes[i..i + 4].iter().all(|b| b.is_ascii_digit()) {
            let before_ok = i == 0 || !bytes[i - 1].is_ascii_digit();
            let after_ok = i + 4 >= bytes.len() || !bytes[i + 4].is_ascii_digit();
            if before_ok && after_ok {
                if let Ok(s) = std::str::from_utf8(&bytes[i..i + 4]) {
                    if let Ok(y) = s.parse::<i64>() {
                        if (1000..=2100).contains(&y) {
                            best_dist = best_dist.min((y - correct).abs());
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if best_dist == i64::MAX {
        return 0.0;
    }
    if best_dist == 0 {
        return 1.0;
    }
    if best_dist <= tolerance {
        1.0 - (best_dist as f32 / tolerance as f32)
    } else {
        0.0
    }
}

fn keyword_hit(kw: &str, answer: &str, answer_toks: &std::collections::HashSet<String>) -> bool {
    let kw_toks: Vec<String> = kw
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() >= 2)
        .map(|t| t.to_lowercase())
        .collect();
    if kw_toks.is_empty() {
        return false;
    }
    let need = match kw_toks.len() {
        1 | 2 => kw_toks.len(), // exact for short phrases
        n => n.div_ceil(2),     // majority for 3+ word phrases
    };
    let found = kw_toks.iter().filter(|t| answer_toks.contains(*t)).count();
    if found >= need {
        return true;
    }
    // Fallback: substring match catches morphological variants ("India"/"Indian", plurals, etc.)
    answer.to_lowercase().contains(&kw.to_lowercase())
}

#[derive(serde::Deserialize)]
struct NumericAnswer {
    /// The correct numeric value (e.g. 1884 for a year).
    correct: i64,
    /// Half-range: score > 0 only within this many units of `correct`.
    tolerance: i64,
}

#[derive(serde::Deserialize)]
struct EvalQuestion {
    id: String,
    question: String,
    expected_keywords: Vec<String>,
    #[serde(default)]
    expected_answer: Option<String>,
    /// Optional numeric answer for questions where the answer is a quantity
    /// (e.g. a year). Scored by proximity instead of keyword overlap.
    #[serde(default)]
    numeric_answer: Option<NumericAnswer>,
}

#[allow(clippy::too_many_arguments)]
async fn cmd_eval(
    questions_path: std::path::PathBuf,
    kb: String,
    inference_url: String,
    model: String,
    top_k: usize,
    mode: String,
    hyde: bool,
    hyde_alpha: Option<f32>,
    rerank: bool,
    understand: bool,
    llm_judge: bool,
    judge_model: Option<String>,
    output: Option<std::path::PathBuf>,
    progress_file: Option<std::path::PathBuf>,
    graph_mode: String,
    query_classify: String,
    summary_expansion: bool,
    biographical_expansion: bool,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        // Suppress tantivy/tracing INFO chatter so eval progress lines are readable.
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "warn");
        }

        let raw = std::fs::read_to_string(&questions_path)
            .with_context(|| format!("reading {}", questions_path.display()))?;
        let questions: Vec<EvalQuestion> = serde_json::from_str(&raw).context(
            "parsing questions JSON — expected array of {id, question, expected_keywords}",
        )?;

        if questions.is_empty() {
            print_warning("No questions found in file.");
            return Ok(());
        }

        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

        // Progress file: default to {data_dir}/eval-progress.json so it can be
        // read without knowing the path (e.g. via `cat ~/.kwaainet/rag/D6/eval-progress.json`).
        let progress_path =
            progress_file.unwrap_or_else(|| rag_cfg.data_dir().join("eval-progress.json"));
        let eval_start = std::time::Instant::now();
        let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

        // Resolve p2p:// URLs to local HTTP proxies (same pattern as dream/graph build).
        let mut _proxy_handles: Vec<tokio::task::JoinHandle<()>> = vec![];
        let inference_url =
            if inference_url.starts_with("p2p://") || inference_url.starts_with("mux://") {
                use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
                let sock = std::env::var("KWAAINET_SOCKET")
                    .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
                #[cfg(unix)]
                let addr = format!("/unix/{sock}");
                #[cfg(not(unix))]
                let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
                let p2p = std::sync::Arc::new(
                    P2PClient::connect(&addr)
                        .await
                        .context("connecting to p2pd for p2p:// URL resolution")?,
                );
                let (resolved, handles) =
                    crate::ollama_proxy::resolve_inference_urls(&[inference_url], &p2p).await?;
                _proxy_handles = handles;
                resolved.into_iter().next().unwrap_or_default()
            } else {
                inference_url
            };

        let retrieve_cfg = RetrieveConfig {
            top_k,
            min_score: 0.0,
            use_sentence_window: false,
            hyde_inference_url: if hyde {
                Some(inference_url.clone())
            } else {
                None
            },
            hyde_model: if hyde { Some(model.clone()) } else { None },
            hyde_alpha: if hyde {
                Some(hyde_alpha.unwrap_or(0.5))
            } else {
                None
            },
            graph_mode: parse_graph_mode(&graph_mode),
            query_classify: parse_classify_method(&query_classify),
            query_multi_hop: false,
            use_summary_expansion: summary_expansion,
        };

        // Load document context preamble from persisted schema metadata (if any).
        let eval_doc_context: Option<String> = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
            .ok()
            .and_then(|g| {
                let meta = g.get_doc_metadata();
                if meta.is_empty() {
                    return None;
                }
                let schema = kwaai_rag::doc_schema::DocSchema {
                    metadata: meta,
                    document_title: g.get_document_titles().into_iter().next(),
                    ..Default::default()
                };
                schema.context_line()
            });

        // Resolve "auto" mode: use graph if the KB has entities, else vector.
        let effective_mode = if mode == "auto" {
            if let Ok(g) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                if g.node_count() > 0 {
                    "graph"
                } else {
                    "vector"
                }
            } else {
                "vector"
            }
        } else {
            mode.as_str()
        };

        print_box_header(&format!(
            "RAG Eval  ({} questions, kb={})",
            questions.len(),
            kb
        ));
        println!("  Model:     {model}");
        println!("  Inference: {inference_url}");
        let judge_mdl = judge_model.as_deref().unwrap_or(&model);
        println!("  top_k={top_k}  mode={effective_mode}  graph_mode={graph_mode}  query_classify={query_classify}  hyde={hyde}  rerank={rerank}  understand={understand}  llm_judge={llm_judge}  summary_expansion={summary_expansion}  biographical_expansion={biographical_expansion}");
        if llm_judge {
            println!("  Judge model: {judge_mdl}");
        }
        print_separator();

        struct Row {
            id: String,
            question: String,
            answer: String,
            retrieved_docs: Vec<String>,
            keyword_hits: f32, // f32 to accommodate numeric proximity partial credit
            total_keywords: f32,
            latency_ms: u128,
            judge_score: Option<u8>,
        }

        let mut rows: Vec<Row> = Vec::new();

        for (i, q) in questions.iter().enumerate() {
            if effective_mode == "iterative" {
                println!(
                    "  [{:>2}/{}] {}",
                    i + 1,
                    questions.len(),
                    truncate(&q.question, 70)
                );
            } else {
                print!(
                    "  [{:>2}/{}] {} … ",
                    i + 1,
                    questions.len(),
                    truncate(&q.question, 60)
                );
            }
            io::stdout().flush().ok();

            let t0 = std::time::Instant::now();

            // Retrieve chunks.
            let vs2 = vs.clone();
            let search_fn = move |emb: Vec<f32>, k: usize| {
                let vs = vs2.clone();
                Box::pin(async move {
                    let raw = vs.search(tenant_id, &emb, k).await?;
                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                })
                    as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
            };

            let mut chunks = if effective_mode == "smart" {
                use kwaai_rag::query_understand::{understand_query_rule, GraphMode, QueryIntent};
                let qs = understand_query_rule(&q.question);
                let is_family_nonauthor =
                    matches!(qs.intent, QueryIntent::FamilyRelation { .. }) && !qs.anchor_is_author;
                let is_family_author =
                    matches!(qs.intent, QueryIntent::FamilyRelation { .. }) && qs.anchor_is_author;
                let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                if is_family_nonauthor {
                    // Non-author family query: graph+replace (graph facts only, zero noise).
                    let mut smart_cfg = retrieve_cfg.clone();
                    smart_cfg.graph_mode = GraphMode::Replace;
                    retrieve_graph_anchored(
                        &q.question,
                        &smart_cfg,
                        &embed,
                        &meta,
                        &graph,
                        search_fn,
                    )
                    .await
                    .unwrap_or_default()
                } else if is_family_author {
                    // Author-anchored family query: Replace injects the resolved relative's
                    // entity description directly (wife/mother use this).
                    // Exception: grandparents use Prepend with a rewritten retrieval query so
                    // vector search finds biographical chapters, not family-context chunks.
                    let q_lower = q.question.to_lowercase();
                    let is_grandparent = q_lower.contains("grandfather")
                        || q_lower.contains("grandpa")
                        || q_lower.contains("grandmother")
                        || q_lower.contains("grandma");
                    let mut smart_cfg = retrieve_cfg.clone();
                    smart_cfg.graph_mode = if is_grandparent {
                        GraphMode::Prepend
                    } else {
                        GraphMode::Replace
                    };
                    let retrieval_query: std::borrow::Cow<str> = if is_grandparent {
                        kwaai_rag::retriever::resolve_relative_entity_name(&q.question, &graph)
                            .map(|name| {
                                std::borrow::Cow::Owned(format!("Who was {name}?"))
                            })
                            .unwrap_or(std::borrow::Cow::Borrowed(q.question.as_str()))
                    } else {
                        std::borrow::Cow::Borrowed(q.question.as_str())
                    };
                    retrieve_graph_anchored(
                        &retrieval_query,
                        &smart_cfg,
                        &embed,
                        &meta,
                        &graph,
                        search_fn,
                    )
                    .await
                    .unwrap_or_default()
                } else {
                    let is_temporal = matches!(qs.intent, QueryIntent::TemporalEvent);
                    let seq_chunk = if is_temporal {
                        let eids =
                            kwaai_rag::sequence::extract_temporal_entity_ids(&q.question, &graph);
                        kwaai_rag::sequence::retrieve_sequence(&q.question, &eids, &graph)
                    } else {
                        None
                    };
                    let mut chunks = retrieve_iterative(
                        &q.question,
                        &retrieve_cfg,
                        &embed,
                        &meta,
                        &graph,
                        search_fn,
                        &inference_url,
                        &model,
                        |msg| println!("{msg}"),
                    )
                    .await
                    .unwrap_or_default();
                    if let Some(seq) = seq_chunk {
                        chunks.insert(0, seq);
                    }
                    chunks
                }
            } else if effective_mode == "iterative" {
                let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                retrieve_iterative(
                    &q.question,
                    &retrieve_cfg,
                    &embed,
                    &meta,
                    &graph,
                    search_fn,
                    &inference_url,
                    &model,
                    |msg| println!("{msg}"),
                )
                .await
                .unwrap_or_default()
            } else if effective_mode == "graph" {
                let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                    .context("opening graph store")?;
                retrieve_graph_anchored(
                    &q.question,
                    &retrieve_cfg,
                    &embed,
                    &meta,
                    &graph,
                    search_fn,
                )
                .await
                .unwrap_or_default()
            } else if understand {
                kwaai_rag::query_understanding::retrieve_with_understanding(
                    &q.question,
                    &retrieve_cfg,
                    &embed,
                    &meta,
                    &inference_url,
                    &model,
                    search_fn,
                )
                .await
                .unwrap_or_default()
            } else {
                retrieve_hybrid(&q.question, &retrieve_cfg, &embed, &meta, search_fn)
                    .await
                    .unwrap_or_default()
            };

            // Rerank (optional).
            if rerank {
                chunks = kwaai_rag::reranker::rerank_chunks(
                    &q.question,
                    chunks,
                    &inference_url,
                    &model,
                    top_k,
                )
                .await;
            }

            let retrieved_docs: Vec<String> = chunks
                .iter()
                .map(|c| c.chunk_meta.doc_name.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            // For biographical "who was / who is" questions with --biographical-expansion,
            // append a detailed-answer instruction so the LLM gives a full biography
            // instead of a one-liner identity response. Also handles enumeration questions
            // ("what organisations…") so the LLM lists ALL entities rather than 1-2.
            let answer_question: std::borrow::Cow<str> = if biographical_expansion {
                let q_lower = q.question.to_lowercase();
                let is_bio = q_lower.starts_with("who was ")
                    || q_lower.starts_with("who is ")
                    || q_lower.starts_with("who were ");
                let is_enum = q_lower.contains("organisation")
                    || q_lower.contains("organization")
                    || q_lower.contains("what political")
                    || q_lower.contains("which organisation")
                    || q_lower.contains("which organization");
                if is_bio {
                    std::borrow::Cow::Owned(format!(
                        "{}\n\nPlease give a detailed answer covering background, \
                         origins, family connections, role, and historical significance.",
                        q.question
                    ))
                } else if is_enum {
                    std::borrow::Cow::Owned(format!(
                        "{}\n\nPlease list EVERY organisation mentioned in the sources, \
                         not just the most prominent one. Include each organisation's \
                         full name and abbreviation.",
                        q.question
                    ))
                } else {
                    std::borrow::Cow::Borrowed(q.question.as_str())
                }
            } else {
                std::borrow::Cow::Borrowed(q.question.as_str())
            };

            // Generate answer.
            let messages = build_chat_messages(
                &answer_question,
                &chunks,
                &[],
                24000,
                eval_doc_context.as_deref(),
            );
            let payload = serde_json::json!({
                "model": model,
                "messages": messages,
                "stream": false,
                "temperature": 0,
            });
            let answer = match http
                .post(format!("{inference_url}/v1/chat/completions"))
                .json(&payload)
                .send()
                .await
            {
                Ok(resp) => {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    body["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("(no response)")
                        .to_string()
                }
                Err(e) => format!("(error: {e})"),
            };

            let latency_ms = t0.elapsed().as_millis();

            // Score keywords (token-overlap recall, substring fallback).
            let answer_toks = eval_tokens(&answer);
            let kw_exact: usize = q
                .expected_keywords
                .iter()
                .filter(|kw| keyword_hit(kw, &answer, &answer_toks))
                .count();
            // Numeric proximity score (e.g. year estimation): adds fractional credit.
            let num_score: f32 = q.numeric_answer.as_ref().map_or(0.0, |na| {
                numeric_proximity_score(&answer, na.correct, na.tolerance)
            });
            let keyword_hits: f32 = kw_exact as f32 + num_score;
            let total_keywords: f32 =
                q.expected_keywords.len() as f32 + q.numeric_answer.is_some() as u8 as f32;

            // LLM-as-judge (optional, only if expected_answer is present).
            let judge_score: Option<u8> = if llm_judge {
                if let Some(ref expected) = q.expected_answer {
                    let judge_prompt = format!(
                        "You are grading a RAG (retrieval-augmented generation) system.\n\
                         The system had access to documents that DO contain the answer.\n\n\
                         Question: {q}\n\n\
                         Reference answer: {ref_ans}\n\n\
                         Candidate answer: {cand}\n\n\
                         Scoring rules — judge the CONTENT, not the phrasing:\n\
                         0 = no correct facts about the question are present, OR the answer \
                             claims the information doesn't exist and provides nothing useful\n\
                         1 = some correct key facts present but important ones are missing or wrong\n\
                         2 = all key facts from the reference are present (wording may differ)\n\n\
                         An answer that hedges ('I couldn't find…') but still states correct facts \
                         should be scored on those facts, not the hedge.\n\n\
                         Return ONLY the digit 0, 1, or 2. Nothing else.",
                        q = q.question,
                        ref_ans = expected,
                        cand = answer
                    );
                    let judge_payload = serde_json::json!({
                        "model": judge_mdl,
                        "messages": [
                            {"role": "system", "content": "You are a strict RAG grader. \
                              Score the factual content of the candidate answer against the \
                              reference. Return only a single digit: 0, 1, or 2."},
                            {"role": "user", "content": judge_prompt}
                        ],
                        "stream": false,
                        "temperature": 0.0,
                        "max_tokens": 4,
                    });
                    match http
                        .post(format!("{inference_url}/v1/chat/completions"))
                        .json(&judge_payload)
                        .send()
                        .await
                    {
                        Ok(resp) => {
                            let body: serde_json::Value = resp.json().await.unwrap_or_default();
                            let text = body["choices"][0]["message"]["content"]
                                .as_str()
                                .unwrap_or("0");
                            text.trim()
                                .chars()
                                .next()
                                .and_then(|c| c.to_digit(10))
                                .map(|d| d.min(2) as u8)
                        }
                        Err(_) => None,
                    }
                } else {
                    None
                }
            } else {
                None
            };

            let judge_str = match judge_score {
                Some(s) => format!("  judge={s}/2"),
                None => String::new(),
            };
            let kw_display = {
                let hits_s = if keyword_hits == keyword_hits.floor() {
                    format!("{:.0}", keyword_hits)
                } else {
                    format!("{:.1}", keyword_hits)
                };
                let tot_s = if total_keywords == total_keywords.floor() {
                    format!("{:.0}", total_keywords)
                } else {
                    format!("{:.1}", total_keywords)
                };
                format!("{hits_s}/{tot_s}")
            };
            if effective_mode == "iterative" {
                println!(
                    "         → {kw_display} keywords{judge_str}  {latency_ms}ms"
                );
            } else {
                println!("{kw_display} keywords{judge_str}  {latency_ms}ms");
            }

            rows.push(Row {
                id: q.id.clone(),
                question: q.question.clone(),
                answer,
                retrieved_docs,
                keyword_hits,
                total_keywords,
                latency_ms,
                judge_score,
            });

            // Write per-question progress JSON so the eval can be monitored
            // without piping stdout (which buffers until process exit).
            {
                let done = rows.len();
                let total = questions.len();
                let hits_so_far: f64 = rows.iter().map(|r| r.keyword_hits as f64).sum();
                let kw_so_far: f64 = rows.iter().map(|r| r.total_keywords as f64).sum();
                let running_recall = if kw_so_far > 0.0 {
                    (hits_so_far / kw_so_far * 100.0).round() / 100.0
                } else {
                    0.0
                };
                let elapsed_s = eval_start.elapsed().as_secs();
                let eta_s = if done > 0 && done < total {
                    elapsed_s * (total - done) as u64 / done as u64
                } else {
                    0
                };
                let progress = serde_json::json!({
                    "done": done,
                    "total": total,
                    "running_recall": running_recall,
                    "last_q": q.id,
                    "last_score": kw_display,
                    "elapsed_s": elapsed_s,
                    "eta_s": eta_s,
                    "kb": kb,
                });
                let _ = std::fs::write(&progress_path, progress.to_string());
            }

            // Brief pause to avoid hammering Ollama.
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        // Build report.
        let total_hits: f64 = rows.iter().map(|r| r.keyword_hits as f64).sum();
        let total_kw: f64 = rows.iter().map(|r| r.total_keywords as f64).sum();
        let overall_score = if total_kw > 0.0 {
            total_hits / total_kw
        } else {
            0.0
        };
        let avg_latency_ms: u128 = if rows.is_empty() {
            0
        } else {
            rows.iter().map(|r| r.latency_ms).sum::<u128>() / rows.len() as u128
        };
        let judge_rows: Vec<u8> = rows.iter().filter_map(|r| r.judge_score).collect();
        let avg_judge = if judge_rows.is_empty() {
            None
        } else {
            Some(judge_rows.iter().map(|&s| s as f64).sum::<f64>() / judge_rows.len() as f64)
        };

        let mut report = String::new();
        report.push_str("# RAG Eval Report\n\n");
        report.push_str(&format!("**KB:** `{kb}`  **Model:** `{model}`\n\n"));
        report.push_str(&format!(
            "**Flags:** top_k={top_k}  hyde={hyde}  rerank={rerank}  understand={understand}  llm_judge={llm_judge}\n\n"
        ));
        let judge_summary = if let Some(avg) = avg_judge {
            format!(
                " | Avg judge score | {:.2}/2.00 ({} questions scored) |\n",
                avg,
                judge_rows.len()
            )
        } else {
            String::new()
        };
        report.push_str(&format!(
            "## Summary\n\n\
             | Metric | Value |\n\
             |--------|-------|\n\
             | Questions | {} |\n\
             | Overall recall (token-overlap) | {:.1}% ({:.1}/{:.0}) |\n\
             {judge_summary}\
             | Avg latency | {avg_latency_ms}ms |\n\n",
            rows.len(),
            overall_score * 100.0,
            total_hits,
            total_kw,
        ));
        report.push_str("## Per-question results\n\n");
        let has_judge = rows.iter().any(|r| r.judge_score.is_some());
        if has_judge {
            report.push_str("| ID | Question | Hit rate | Judge | Sources | Latency |\n");
            report.push_str("|----|----------|----------|-------|---------|--------|\n");
        } else {
            report.push_str("| ID | Question | Hit rate | Sources | Latency |\n");
            report.push_str("|----|----------|----------|---------|--------|\n");
        }
        for r in &rows {
            let pct = if r.total_keywords > 0.0 {
                let hits_s = if r.keyword_hits == r.keyword_hits.floor() {
                    format!("{:.0}", r.keyword_hits)
                } else {
                    format!("{:.1}", r.keyword_hits)
                };
                let tot_s = if r.total_keywords == r.total_keywords.floor() {
                    format!("{:.0}", r.total_keywords)
                } else {
                    format!("{:.1}", r.total_keywords)
                };
                format!(
                    "{hits_s}/{tot_s} ({:.0}%)",
                    r.keyword_hits as f64 / r.total_keywords as f64 * 100.0
                )
            } else {
                "n/a".to_string()
            };
            if has_judge {
                let j = r.judge_score.map_or("—".to_string(), |s| format!("{s}/2"));
                report.push_str(&format!(
                    "| {} | {} | {} | {} | {} | {}ms |\n",
                    r.id,
                    r.question.replace('|', "\\|"),
                    pct,
                    j,
                    r.retrieved_docs.join(", "),
                    r.latency_ms,
                ));
            } else {
                report.push_str(&format!(
                    "| {} | {} | {} | {} | {}ms |\n",
                    r.id,
                    r.question.replace('|', "\\|"),
                    pct,
                    r.retrieved_docs.join(", "),
                    r.latency_ms,
                ));
            }
        }
        report.push_str("\n## Answers\n\n");
        for r in &rows {
            report.push_str(&format!("### {} — {}\n\n", r.id, r.question));
            report.push_str(&format!("{}\n\n", r.answer));
        }

        // Output.
        let judge_note = avg_judge
            .map(|a| format!("  judge={:.2}/2", a))
            .unwrap_or_default();
        if let Some(ref path) = output {
            std::fs::write(path, &report)
                .with_context(|| format!("writing report to {}", path.display()))?;
            print_success(&format!(
                "Report written to {}  ({:.1}% hit rate{judge_note}, {avg_latency_ms}ms avg)",
                path.display(),
                overall_score * 100.0,
            ));
        } else {
            println!("\n{report}");
            print_success(&format!(
                "Overall: {:.1}% recall (token-overlap){judge_note}  ({:.1}/{:.0})  avg {avg_latency_ms}ms",
                overall_score * 100.0,
                total_hits,
                total_kw,
            ));
        }
        Ok(())
    }
}

/// Recursively collect files under `root` that match `exts`.
/// `doc_name` is the path relative to `base`.
#[cfg(feature = "storage")]
fn collect_files(
    dir: &std::path::Path,
    base: &std::path::Path,
    exts: &[String],
    out: &mut Vec<(String, std::path::PathBuf)>,
) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, base, exts, out)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if exts.contains(&ext) {
                let rel = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();
                out.push((rel, path));
            }
        }
    }
    Ok(())
}

// ── enrich-entities ────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
async fn cmd_enrich_entities(
    inference_url: String,
    inference_urls: Option<String>,
    model: String,
    workers: usize,
    min_mentions: u32,
    entity_types: String,
    limit: Option<usize>,
    force: bool,
    extract_gender: bool,
    kb: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        use kwaai_rag::enrich::{enrich_entity_descriptions, EnrichConfig};

        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));

        // Collect all comma-separated URLs; fall back to the positional arg.
        let raw_urls: Vec<String> = inference_urls
            .as_deref()
            .map(|s| {
                s.split(',')
                    .map(|u| u.trim().to_string())
                    .filter(|u| !u.is_empty())
                    .collect()
            })
            .unwrap_or_else(|| vec![inference_url]);

        let types: Vec<String> = entity_types
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let cfg = EnrichConfig {
            entity_types: types,
            min_mentions,
            limit: limit.unwrap_or(usize::MAX),
            workers,
            force,
            extract_gender,
            ..Default::default()
        };

        print_box_header(&format!("Enrich Entity Metadata ({})", kb));
        print_info(&format!("  Inference URLs:  {}", raw_urls.join(", ")));
        print_info(&format!("  Model:           {}", model));
        print_info(&format!("  Workers:         {}", workers));
        print_info(&format!("  Min mentions:    {}", min_mentions));
        print_info(&format!("  Entity types:    {}", entity_types));
        print_info(&format!("  Force overwrite: {}", force));
        print_info(&format!("  Extract gender:  {}", extract_gender));
        if let Some(l) = limit {
            print_info(&format!("  Limit:           {}", l));
        }

        // Resolve p2p:// or mux:// URLs to local HTTP proxy endpoints before handing
        // off to enrich (which uses raw reqwest and cannot handle non-HTTP schemes).
        let mut _proxy_handles: Vec<tokio::task::JoinHandle<()>> = vec![];
        let resolved_urls: Vec<String> = if raw_urls
            .iter()
            .any(|u| u.starts_with("p2p://") || u.starts_with("mux://"))
        {
            use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
            let sock = std::env::var("KWAAINET_SOCKET")
                .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
            #[cfg(unix)]
            let addr = format!("/unix/{sock}");
            #[cfg(not(unix))]
            let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
            let p2p = std::sync::Arc::new(
                P2PClient::connect(&addr)
                    .await
                    .context("connecting to p2pd for p2p:// URL resolution")?,
            );
            let (resolved, handles) =
                crate::ollama_proxy::resolve_inference_urls(&raw_urls, &p2p).await?;
            _proxy_handles = handles;
            resolved
        } else {
            raw_urls
        };

        let data_dir = rag_cfg.data_dir();
        let report = enrich_entity_descriptions(
            &cfg,
            &model,
            &resolved_urls,
            &embed,
            &data_dir,
            tenant_id,
            |done, total, _label| {
                if done.is_multiple_of(10) || done == total {
                    print_info(&format!("  [{done}/{total}]"));
                }
            },
        )
        .await?;

        print_success(&format!(
            "Enrich complete — {} processed, {} updated ({} genders set), {} skipped (no evidence), {} errors",
            report.entities_processed,
            report.entities_updated,
            report.genders_set,
            report.entities_skipped_no_evidence,
            report.errors.len(),
        ));
        for e in &report.errors {
            print_warning(e);
        }
        Ok(())
    }
}

// ── export ─────────────────────────────────────────────────────────────────────

async fn cmd_export(output_dir: std::path::PathBuf, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
            .context("opening graph store — run `rag graph build` first")?;

        if graph.node_count() == 0 {
            print_warning("Graph is empty. Run `kwaainet rag graph build --kb <name>` first.");
            return Ok(());
        }

        print_box_header(&format!("RAG Export ({})", kb));
        print_info(&format!(
            "Exporting {} entities to {}",
            graph.node_count(),
            output_dir.display()
        ));

        let stats = kwaai_rag::obsidian::export_vault(&graph, &output_dir, &kb)?;

        let stale_msg = if stats.stale_removed > 0 {
            format!(", {} stale files removed", stats.stale_removed)
        } else {
            String::new()
        };
        print_success(&format!(
            "Vault written — {} entity files, {} relation links{}",
            stats.entities, stats.relations, stale_msg
        ));
        println!(
            "  Open {} in Obsidian and enable Graph View (Ctrl/Cmd+G).",
            output_dir.display()
        );
        println!(
            "  After curation run:  kwaainet rag graph import --input-dir {} --kb {}",
            output_dir.display(),
            kb
        );
        Ok(())
    }
}

// ── import ─────────────────────────────────────────────────────────────────────

async fn cmd_import(input_dir: std::path::PathBuf, since: u64, kb: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let mut graph =
            GraphStore::open(&rag_cfg.data_dir(), tenant_id).context("opening graph store")?;
        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));

        print_box_header(&format!("RAG Import ({})", kb));
        if since > 0 {
            print_info(&format!(
                "Processing files modified after Unix timestamp {since}"
            ));
        } else {
            print_info("Processing all entity files (--since 0)");
        }

        let stats =
            kwaai_rag::obsidian::import_vault(&mut graph, &input_dir, since, &embed).await?;

        print_success(&format!(
            "Import complete — {} entities updated ({} descriptions re-embedded, {} relations upserted), {} skipped",
            stats.entities_processed,
            stats.descriptions_updated,
            stats.relations_updated,
            stats.skipped,
        ));
        Ok(())
    }
}

// ── graph timeline ────────────────────────────────────────────────────────────

async fn cmd_graph_timeline(action: TimelineAction, kb: &str) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(kb)?;
        let graph =
            GraphStore::open(&rag_cfg.data_dir(), tenant_id).context("opening graph store")?;
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id).context("opening meta store")?;

        match action {
            TimelineAction::Stats => {
                let (event_count, interaction_count) = graph.timeline_stats();
                print_box_header(&format!("Timeline Stats ({})", kb));
                println!("  Entity timelines stored : {event_count}");
                println!("  Interaction pairs stored: {interaction_count}");
                if event_count == 0 && interaction_count == 0 {
                    print_info(
                        "No timeline data yet. Run `kwaainet rag graph timeline build` to extract.",
                    );
                }
            }

            TimelineAction::Show { entity } => {
                let eid = graph.find_by_name(&entity).map(|e| e.id).or_else(|| {
                    // token-intersection fallback
                    let tokens: Vec<String> = entity
                        .split_whitespace()
                        .map(|t| t.to_lowercase())
                        .filter(|t| t.len() >= 2)
                        .collect();
                    let mut scores: std::collections::HashMap<i64, usize> =
                        std::collections::HashMap::new();
                    for t in &tokens {
                        for &id in graph.find_ids_by_alias_token(t) {
                            *scores.entry(id).or_default() += 1;
                        }
                    }
                    scores.into_iter().max_by_key(|(_, s)| *s).map(|(id, _)| id)
                });
                let Some(eid) = eid else {
                    bail!("entity '{}' not found in graph", entity);
                };
                let events = graph.get_timeline_events(&[eid]);
                let entity_name = graph
                    .get_entity(eid)
                    .map(|e| e.name.clone())
                    .unwrap_or_else(|| entity.clone());
                print_box_header(&format!("Timeline: {} ({})", entity_name, kb));
                if events.is_empty() {
                    print_info("No timeline events found. Run `graph timeline build` first.");
                } else {
                    let mut sorted = events;
                    sorted.sort_by(|a, b| a.date_sort.cmp(&b.date_sort));
                    for ev in &sorted {
                        let date = ev.date_raw.as_deref().unwrap_or("(date unknown)");
                        println!("  [{:12}] {} — {}", ev.event_class, date, ev.description);
                    }
                }
                let interactions = graph.get_interactions_for(&[eid]);
                if !interactions.is_empty() {
                    println!();
                    println!("  Interactions:");
                    let mut sorted_ia = interactions;
                    sorted_ia.sort_by(|a, b| a.date_sort.cmp(&b.date_sort));
                    for ia in &sorted_ia {
                        let date = ia.date_raw.as_deref().unwrap_or("(date unknown)");
                        println!(
                            "    {} — {} {} {}",
                            date, ia.from_entity_name, ia.label, ia.to_entity_name
                        );
                    }
                }
            }

            TimelineAction::ExportMermaid { entity } => {
                let eid = graph.find_by_name(&entity).map(|e| e.id).or_else(|| {
                    let tokens: Vec<String> = entity
                        .split_whitespace()
                        .map(|t| t.to_lowercase())
                        .filter(|t| t.len() >= 2)
                        .collect();
                    let mut scores: std::collections::HashMap<i64, usize> =
                        std::collections::HashMap::new();
                    for t in &tokens {
                        for &id in graph.find_ids_by_alias_token(t) {
                            *scores.entry(id).or_default() += 1;
                        }
                    }
                    scores.into_iter().max_by_key(|(_, s)| *s).map(|(id, _)| id)
                });
                let Some(eid) = eid else {
                    bail!("entity '{}' not found in graph", entity);
                };
                // Include 1-hop neighbours
                let mut all_ids: std::collections::HashSet<i64> = std::iter::once(eid).collect();
                for (nid, _, _) in graph.neighbors_of(eid) {
                    all_ids.insert(nid);
                }
                let all_ids_vec: Vec<i64> = all_ids.into_iter().collect();
                let events = graph.get_timeline_events(&all_ids_vec);
                let mut interactions = graph.get_interactions_for(&all_ids_vec);
                interactions.retain(|ia| {
                    all_ids_vec.contains(&ia.from_entity_id)
                        && all_ids_vec.contains(&ia.to_entity_id)
                });
                let entity_name = graph
                    .get_entity(eid)
                    .map(|e| e.name.clone())
                    .unwrap_or_else(|| entity.clone());
                let mermaid =
                    kwaai_rag::sequence::render_mermaid(&entity_name, &events, &interactions);
                println!("{mermaid}");
            }

            TimelineAction::Build {
                inference_url,
                model,
                workers,
                reset,
            } => {
                let raw_infer_url =
                    inference_url.unwrap_or_else(|| rag_cfg.inference_url.clone());

                // Resolve p2p:// / mux:// URLs to local HTTP proxies.
                let infer_url = {
                    use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};
                    let sock = std::env::var("KWAAINET_SOCKET")
                        .unwrap_or_else(|_| DEFAULT_SOCKET_NAME.to_string());
                    #[cfg(unix)]
                    let addr = format!("/unix/{sock}");
                    #[cfg(not(unix))]
                    let addr = "/ip4/127.0.0.1/tcp/5005".to_string();
                    let p2p = std::sync::Arc::new(
                        P2PClient::connect(&addr)
                            .await
                            .context("p2p daemon not running — start with `kwaainet start`")?,
                    );
                    let (mut resolved, _handles) =
                        crate::ollama_proxy::resolve_inference_urls(&[raw_infer_url], &p2p)
                            .await?;
                    resolved.remove(0)
                };

                if reset {
                    graph.reset_timeline()?;
                    print_info("Timeline tables cleared.");
                }

                print_box_header(&format!("Timeline Build ({})", kb));
                print_info(&format!(
                    "Model: {model}  Workers: {workers}  Inference: {infer_url}"
                ));

                // Collect all chunks that are linked to at least one entity.
                let chunk_ids: Vec<i64> =
                    graph.all_chunk_entity_pairs().map(|(cid, _)| cid).collect();

                let total = chunk_ids.len();
                println!("  Processing {total} entity-linked chunks…");

                let graph = std::sync::Arc::new(graph);
                let meta = std::sync::Arc::new(meta);
                let infer_url = std::sync::Arc::new(infer_url);
                let model = std::sync::Arc::new(model);

                let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(workers));
                let done = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
                let event_total = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
                let ia_total = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

                let mut handles = Vec::new();
                for cid in chunk_ids {
                    let graph = graph.clone();
                    let meta = meta.clone();
                    let infer_url = infer_url.clone();
                    let model = model.clone();
                    let sem = sem.clone();
                    let done = done.clone();
                    let ev_total = event_total.clone();
                    let ia_total = ia_total.clone();

                    let handle = tokio::spawn(async move {
                        let _permit = sem.acquire().await.ok()?;
                        let chunk = meta.get_chunks(&[cid]).ok()?.into_iter().next()??;
                        let entity_ids = graph.get_chunk_entities(cid);
                        let entity_names: Vec<String> = entity_ids
                            .iter()
                            .filter_map(|&id| graph.get_entity(id))
                            .map(|e| e.name.clone())
                            .collect();

                        let (raw_events, raw_interactions) =
                            kwaai_rag::sequence::extract_temporal_events(
                                &chunk.text,
                                &entity_names,
                                &infer_url,
                                &model,
                            )
                            .await
                            .ok()?;

                        let (events, interactions) = kwaai_rag::sequence::resolve_extracted(
                            raw_events,
                            raw_interactions,
                            cid,
                            &graph,
                        );

                        let n_ev = events.len();
                        let n_ia = interactions.len();

                        // Group events by entity_id and store
                        let mut by_entity: std::collections::HashMap<
                            i64,
                            Vec<kwaai_rag::sequence::TimelineEvent>,
                        > = std::collections::HashMap::new();
                        for ev in events {
                            by_entity.entry(ev.entity_id).or_default().push(ev);
                        }
                        for (eid, evs) in &by_entity {
                            graph.store_timeline_events(*eid, evs).ok()?;
                        }
                        for ia in &interactions {
                            graph.store_interaction(ia).ok()?;
                        }

                        let d = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                        ev_total.fetch_add(n_ev, std::sync::atomic::Ordering::Relaxed);
                        ia_total.fetch_add(n_ia, std::sync::atomic::Ordering::Relaxed);
                        if d.is_multiple_of(50) || d == total {
                            let evs = ev_total.load(std::sync::atomic::Ordering::Relaxed);
                            let ias = ia_total.load(std::sync::atomic::Ordering::Relaxed);
                            println!("  [{d}/{total}] events={evs} interactions={ias}");
                        }
                        Some(())
                    });
                    handles.push(handle);
                }
                for h in handles {
                    h.await.ok();
                }
                let ev_count = event_total.load(std::sync::atomic::Ordering::Relaxed);
                let ia_count = ia_total.load(std::sync::atomic::Ordering::Relaxed);
                print_success(&format!(
                    "Timeline build complete — {ev_count} events, {ia_count} interactions extracted from {total} chunks."
                ));
            }
        }
        Ok(())
    }
}

// ── summarize ─────────────────────────────────────────────────────────────────

async fn cmd_summarize(
    kb: String,
    inference_url: String,
    model: String,
    window_size: usize,
    reset: bool,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;
        let embed = EmbedClient::new(rag_cfg.embed_url.clone(), Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

        if reset {
            meta.clear_summary_nodes()?;
            print_info("Cleared existing summary nodes.");
        }

        print_box_header(&format!("HiRAG Summarize ({})", kb));
        println!("  inference_url={inference_url}  model={model}  window_size={window_size}");

        let start = std::time::Instant::now();
        let nodes = kwaai_rag::summary::generate_summaries(
            &meta,
            &embed,
            &inference_url,
            &model,
            window_size,
            |s| println!("{s}"),
        )
        .await?;

        meta.put_summary_nodes(&nodes)?;
        let elapsed = start.elapsed().as_secs_f64();
        print_success(&format!(
            "Stored {} summary nodes in {:.1}s",
            nodes.len(),
            elapsed
        ));
        Ok(())
    }
}
