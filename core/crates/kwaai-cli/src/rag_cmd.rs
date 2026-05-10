use std::io::{self, BufRead, Write as _};
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use libp2p::PeerId;
use uuid::Uuid;

use kwaai_rag::{
    document,
    embedder::EmbedClient,
    ingestion::{ingest_text, IngestConfig},
    meta_store::{MetaStore, SyncMeta},
    prompt::{build_chat_messages, ChatMessage},
    retriever::{retrieve_hybrid, RetrieveConfig},
};

use crate::cli::{RagAction, RagArgs};
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
        } => cmd_init(name, embed_model, rag_dir).await,

        RagAction::List => cmd_list().await,

        RagAction::ConnectEve { peer_id, url, kb } => cmd_connect_eve(peer_id, url, kb).await,

        RagAction::Ingest {
            file,
            doc_name,
            chunk_size,
            chunk_overlap,
            kb,
        } => cmd_ingest(file, doc_name, chunk_size, chunk_overlap, kb).await,

        RagAction::Query {
            text,
            top_k,
            min_score,
            json,
            kb,
            understand,
            inference_url,
        } => cmd_query(text, top_k, min_score, json, kb, understand, inference_url).await,

        RagAction::Chat {
            top_k,
            inference_url,
            kb,
            understand,
        } => cmd_chat(top_k, inference_url, kb, understand).await,

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
            kb,
        } => cmd_sync(folder, extensions, delete, watch, interval, kb).await,
    }
}

// ── init ──────────────────────────────────────────────────────────────────────

async fn cmd_init(
    name: String,
    embed_model: String,
    rag_dir: Option<std::path::PathBuf>,
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

        // Probe embedding model before touching storage.
        let embed = EmbedClient::new(None, Some(embed_model.clone()));
        print_info(&format!("Probing Ollama ({embed_model})…"));
        embed.check_dim().await?;
        print_success("Embedding model OK (768 dimensions)");

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
                768,
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

        let retrieve_cfg = RetrieveConfig {
            top_k,
            min_score,
            use_sentence_window: false,
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

            let mut chunks = match rag_cfg.storage_url.as_deref() {
                Some("local") => {
                    let vs = Arc::new(open_local_vs(&rag_cfg.data_dir())?);
                    if understand {
                        kwaai_rag::query_understanding::retrieve_with_understanding(
                            &query,
                            &retrieve_cfg,
                            &embed,
                            &meta,
                            &infer_url,
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
        all_results.truncate(top_k);

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

async fn cmd_chat(top_k: usize, inference_url: String, kb: String, understand: bool) -> Result<()> {
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

            let messages = build_chat_messages(&query, &chunks, &history, 8192);
            let payload = serde_json::json!({
                "model": "default",
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
        print_separator();

        loop {
            let result = run_sync_pass(&folder, &exts, delete, &kb).await?;

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
        let ingest_cfg = IngestConfig::new(embed);

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
