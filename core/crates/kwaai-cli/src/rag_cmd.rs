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
};

use crate::cli::{CacheAction, DreamAction, GraphAction, RagAction, RagArgs};
use crate::config::{KwaaiNetConfig, RagConfig};
use crate::display::*;

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

        RagAction::Export { output_dir, kb } => cmd_export(output_dir, kb).await,

        RagAction::Import {
            input_dir,
            since,
            kb,
        } => cmd_import(input_dir, since, kb).await,

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

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;

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
        Ok(())
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
            let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
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
                    if effective_mode == "iterative" {
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

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
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

            let messages = build_chat_messages(&query, &chunks, &history, 24000);
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

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
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
                let has_p2p = all_raw.iter().any(|u| u.starts_with("p2p://"));
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
                println!("  This may take a while — one LLM call per chunk.\n");

                let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
                let store = Arc::new(Mutex::new(
                    GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                        .context("opening graph store")?,
                ));
                let graph_cfg = kwaai_rag::ingestion::GraphIngestConfig {
                    store: store.clone(),
                    inference_url: infer_url,
                    inference_urls: extra_urls,
                    model,
                    workers: effective_workers,
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
                    })
                    .collect();
                let ids: Vec<i64> = all_chunks.iter().map(|(id, _)| *id).collect();

                kwaai_rag::ingestion::extract_and_store_entities_pub(
                    &chunks,
                    &ids,
                    &embed,
                    &graph_cfg,
                    Some(std::sync::Arc::new(|done: usize, total: usize, entities: usize, relations: usize| {
                        if done.is_multiple_of(50) || done == total {
                            println!("  [{done:>4}/{total}]  entities={entities}  relations={relations}");
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

                // ── Tier 1: exact normalized name matches ─────────────────
                let exact = store.find_dedup_candidates_exact();
                if exact.is_empty() {
                    println!("  Tier 1  no exact-name duplicates");
                } else {
                    println!("  Tier 1  {} exact-name duplicate(s):", exact.len());
                    if !dry_run {
                        for (alias_id, canonical_id) in &exact {
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
                                println!(
                                    "  {:>3}.  \"{}\"  (mentions={}, relations={})",
                                    i + 1,
                                    a.name,
                                    a.mention_count,
                                    a_rels
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
                                println!("        \"{}\"  →  \"{}\"  [{}]", a.name, b.name, reason);
                            }
                        }
                    } else if auto {
                        for (alias_id, canonical_id, reason) in &name_struct {
                            if store.get_entity(*alias_id).is_none() {
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

                // ── Tier 4: neighbour containment (review only) ───────────
                // Alias must have ≤ 15 neighbours and ≥ 60% of them covered by
                // the canonical's neighbour set.  Never auto-merged — too noisy
                // in memoir-style texts; always shown for human review.
                let containment_cands = store.find_dedup_candidates_neighbor_containment(0.60, 3);
                if containment_cands.is_empty() {
                    println!("  Tier 4  no neighbour-containment candidates");
                } else {
                    println!(
                        "  Tier 4  {} containment candidate(s) (review only):",
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

                let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
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
                     ({} relations re-pointed), {} family relations planted, {} aliases \
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
                println!("\n  Tip: run `rag export` to view the updated graph in Obsidian.");
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
                    types.sort_by(|a, b| b.1.cmp(a.1));
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
/// Rules:
///   - Capitalised words (ignoring leading/trailing punctuation) are always included.
///   - Stop-words are included only when sandwiched between capitalised words.
///   - Stop at the first lowercase non-stop-word.
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

        if first.is_uppercase() {
            start -= 1;
        } else if PROPER_NOUN_STOP_WORDS.contains(&lower.as_str()) && start < n {
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
        .rfind(|c| matches!(c, '.' | '!' | '?' | ':' | ';' | '\n' | '\r'))
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
    pairs.sort_by(|a, b| b.2.cmp(&a.2));

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
                let has_p2p = raw_urls.iter().any(|u| u.starts_with("p2p://"));
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

                let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
                let cfg = kwaai_rag::dream::DreamConfig {
                    completeness_threshold: threshold,
                    dedup_threshold,
                    max_completions_per_cycle: max_completions,
                    workers,
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

#[derive(serde::Deserialize)]
struct EvalQuestion {
    id: String,
    question: String,
    expected_keywords: Vec<String>,
    #[serde(default)]
    expected_answer: Option<String>,
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
        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
        let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;
        let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;

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
        };

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
        println!("  top_k={top_k}  mode={effective_mode}  hyde={hyde}  rerank={rerank}  understand={understand}  llm_judge={llm_judge}");
        if llm_judge {
            println!("  Judge model: {judge_mdl}");
        }
        print_separator();

        struct Row {
            id: String,
            question: String,
            answer: String,
            retrieved_docs: Vec<String>,
            keyword_hits: usize,
            total_keywords: usize,
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

            let mut chunks = if effective_mode == "iterative" {
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

            // Generate answer.
            let messages = build_chat_messages(&q.question, &chunks, &[], 24000);
            let payload = serde_json::json!({
                "model": model,
                "messages": messages,
                "stream": false,
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

            // Score keywords (case-insensitive substring match).
            let answer_lower = answer.to_lowercase();
            let keyword_hits = q
                .expected_keywords
                .iter()
                .filter(|kw| answer_lower.contains(&kw.to_lowercase()))
                .count();
            let total_keywords = q.expected_keywords.len();

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
            if effective_mode == "iterative" {
                println!(
                    "         → {keyword_hits}/{total_keywords} keywords{judge_str}  {latency_ms}ms"
                );
            } else {
                println!("{keyword_hits}/{total_keywords} keywords{judge_str}  {latency_ms}ms");
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

            // Brief pause to avoid hammering Ollama.
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        // Build report.
        let total_hits: usize = rows.iter().map(|r| r.keyword_hits).sum();
        let total_kw: usize = rows.iter().map(|r| r.total_keywords).sum();
        let overall_score = if total_kw > 0 {
            total_hits as f64 / total_kw as f64
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
             | Overall keyword hit rate | {:.1}% ({total_hits}/{total_kw}) |\n\
             {judge_summary}\
             | Avg latency | {avg_latency_ms}ms |\n\n",
            rows.len(),
            overall_score * 100.0,
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
            let pct = if r.total_keywords > 0 {
                format!(
                    "{}/{} ({:.0}%)",
                    r.keyword_hits,
                    r.total_keywords,
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
                "Overall: {:.1}% keyword hit rate{judge_note}  ({total_hits}/{total_kw})  avg {avg_latency_ms}ms",
                overall_score * 100.0,
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

        print_success(&format!(
            "Vault written — {} entity files, {} relation links",
            stats.entities,
            stats.relations / 2
        ));
        println!(
            "  Open {} in Obsidian and enable Graph View (Ctrl/Cmd+G).",
            output_dir.display()
        );
        println!(
            "  After curation run:  kwaainet rag import --input-dir {} --kb {}",
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
        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));

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
