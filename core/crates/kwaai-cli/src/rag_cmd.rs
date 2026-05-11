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
    graph::GraphStore,
    ingestion::{ingest_text, GraphIngestConfig, IngestConfig},
    meta_store::{MetaStore, SyncMeta},
    prompt::{build_chat_messages, ChatMessage},
    retriever::{retrieve_graph_anchored, retrieve_hybrid, RetrieveConfig},
};

use crate::cli::{CacheAction, GraphAction, RagAction, RagArgs};
use crate::config::{kwaainet_dir, KwaaiNetConfig, RagConfig};
use crate::display::*;

#[cfg(feature = "storage")]
use crate::storage_rpc::{
    http_create_tenant, http_delete_vectors, http_search_vectors, http_upload_vectors,
    rpc_create_tenant, rpc_delete_vectors, rpc_search_vectors, rpc_upload_vectors,
    CreateTenantPayload,
};

pub async fn run(args: RagArgs) -> Result<()> {
    match args.action {
        RagAction::Init {
            name,
            embed_model,
            rag_dir,
            graph,
        } => cmd_init(name, embed_model, rag_dir, graph).await,

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
            )
            .await
        }

        RagAction::Docs { kb } => cmd_docs(kb).await,

        RagAction::DeleteDoc { name, yes, kb } => cmd_delete_doc(name, yes, kb).await,

        RagAction::Destroy { yes, kb } => cmd_destroy(yes, kb).await,

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
                kb,
            )
            .await
        }

        RagAction::Graph { action, kb } => cmd_graph(action, kb).await,

        RagAction::Cache { action, kb } => cmd_cache(action, kb).await,

        RagAction::Export { output_dir, kb } => cmd_export(output_dir, kb).await,

        RagAction::Import { input_dir, since, kb } => cmd_import(input_dir, since, kb).await,

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
        let embed_dim = embed.probe_dim().await?;
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

        if extract_entities {
            let infer_url = inference_url
                .clone()
                .unwrap_or_else(|| rag_cfg.inference_url.clone());
            let store =
                GraphStore::open(&rag_cfg.data_dir(), tenant_id).context("opening graph store")?;
            cfg.graph = Some(GraphIngestConfig {
                store: Arc::new(Mutex::new(store)),
                inference_url: infer_url,
                model: extraction_model.clone(),
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
                            rpc_upload_vectors(&*guard, &eve_peer_id, tenant_id, vectors).await
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
        let spinner = if json_out {
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
                    if effective_mode == "graph" {
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
                            let raw = rpc_search_vectors(&*guard, &ep, tenant_id, emb, k).await?;
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

async fn cmd_chat(
    top_k: usize,
    inference_url: String,
    kb: String,
    understand: bool,
    model: String,
    hyde: bool,
    hyde_alpha: Option<f32>,
    rerank: bool,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id) = load_rag_config_for(&kb)?;

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

            // Retrieve context (with optional query understanding).
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
                if understand {
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
                        let raw =
                            rpc_search_vectors(&*guard, &eve_peer_id, tenant_id, embedding, k)
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
    let _ = match rag_cfg.storage_url.as_deref() {
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

fn parse_chunk_strategy(s: &str) -> kwaai_rag::chunker::ChunkStrategy {
    match s.to_lowercase().as_str() {
        "paragraph" => kwaai_rag::chunker::ChunkStrategy::Paragraph,
        _ => kwaai_rag::chunker::ChunkStrategy::Character,
    }
}

fn truncate(s: &str, max: usize) -> &str {
    let mut end = s.len().min(max);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

// ── destroy ───────────────────────────────────────────────────────────────────

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

        let chunk_cfg = kwaai_rag::chunker::ChunkConfig {
            chunk_size,
            chunk_overlap,
            min_chunk_len,
            strategy: parse_chunk_strategy(&chunk_strategy),
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
async fn run_sync_pass(
    folder: &std::path::Path,
    exts: &[String],
    delete: bool,
    kb: &str,
    chunk_cfg: &kwaai_rag::chunker::ChunkConfig,
    extract_entities: bool,
    inference_url: Option<String>,
    extraction_model: String,
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

        if extract_entities {
            let infer_url = inference_url
                .clone()
                .unwrap_or_else(|| rag_cfg.inference_url.clone());
            if let Ok(store) = GraphStore::open(&rag_cfg.data_dir(), tenant_id) {
                ingest_cfg.graph = Some(GraphIngestConfig {
                    store: Arc::new(Mutex::new(store)),
                    inference_url: infer_url,
                    model: extraction_model.clone(),
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
                            crate::storage_rpc::rpc_upload_vectors(&*guard, &ep, tenant_id, vectors)
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
            } => {
                let infer_url = inference_url.unwrap_or_else(|| rag_cfg.inference_url.clone());

                let meta = MetaStore::open(&rag_cfg.data_dir(), tenant_id)?;
                let mut all_chunks = meta.all_chunks()?;
                if let Some(n) = limit {
                    all_chunks.truncate(n);
                }
                let total = all_chunks.len();

                if total == 0 {
                    print_warning("No chunks found — ingest documents first.");
                    return Ok(());
                }

                print_box_header(&format!("Graph Build ({})", kb));
                println!("  Chunks to process: {total}");
                println!("  Inference URL:     {infer_url}");
                println!("  This may take a while — one LLM call per chunk.\n");

                let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
                let store = Arc::new(Mutex::new(
                    GraphStore::open(&rag_cfg.data_dir(), tenant_id)
                        .context("opening graph store")?,
                ));
                let graph_cfg = kwaai_rag::ingestion::GraphIngestConfig {
                    store: store.clone(),
                    inference_url: infer_url,
                    model,
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
                    Some(|done: usize, total: usize, entities: usize, relations: usize| {
                        if done % 50 == 0 || done == total {
                            println!("  [{done:>4}/{total}]  entities={entities}  relations={relations}");
                        }
                    }),
                )
                .await;

                let final_store = store.lock().unwrap();
                print_success(&format!(
                    "Graph built — {} entities, {} relations",
                    final_store.node_count(),
                    final_store.relation_count()
                ));
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
            print!(
                "  [{:>2}/{}] {} … ",
                i + 1,
                questions.len(),
                truncate(&q.question, 60)
            );
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

            let mut chunks = if effective_mode == "graph" {
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
                        "Question: {}\n\nReference answer: {}\n\nCandidate answer: {}\n\n\
                         Score the candidate answer:\n\
                         0 = wrong, fabricated, or does not answer the question\n\
                         1 = partially correct (some right facts, but incomplete or mixed with errors)\n\
                         2 = fully correct\n\n\
                         Return ONLY the digit (0, 1, or 2). Nothing else.",
                        q.question, expected, answer
                    );
                    let judge_payload = serde_json::json!({
                        "model": judge_mdl,
                        "messages": [
                            {"role": "system", "content": "You are a strict grader. Return only a digit."},
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
            println!("{keyword_hits}/{total_keywords} keywords{judge_str}  {latency_ms}ms");

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
        let mut graph = GraphStore::open(&rag_cfg.data_dir(), tenant_id)
            .context("opening graph store")?;
        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));

        print_box_header(&format!("RAG Import ({})", kb));
        if since > 0 {
            print_info(&format!("Processing files modified after Unix timestamp {since}"));
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
