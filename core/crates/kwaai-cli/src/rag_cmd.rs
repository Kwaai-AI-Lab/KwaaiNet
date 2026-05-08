use std::io::{self, BufRead, Write as _};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use libp2p::PeerId;
use uuid::Uuid;

use kwaai_rag::{
    embedder::EmbedClient,
    ingestion::{ingest_text, IngestConfig},
    meta_store::MetaStore,
    prompt::{build_chat_messages, ChatMessage},
    retriever::{retrieve, RetrieveConfig},
};

use crate::cli::{RagAction, RagArgs};
use crate::config::{kwaainet_dir, KwaaiNetConfig, RagConfig};
use crate::display::*;

#[cfg(feature = "storage")]
use crate::storage_rpc::{
    rpc_create_tenant, rpc_delete_vectors, rpc_list_tenants, rpc_search_vectors,
    rpc_upload_vectors, CreateTenantPayload,
};

pub async fn run(args: RagArgs) -> Result<()> {
    match args.action {
        RagAction::Init {
            eve_peer_id,
            capacity_mb,
            embed_model,
        } => cmd_init(eve_peer_id, capacity_mb, embed_model).await,

        RagAction::Ingest {
            file,
            doc_name,
            chunk_size,
            chunk_overlap,
        } => cmd_ingest(file, doc_name, chunk_size, chunk_overlap).await,

        RagAction::Query {
            text,
            top_k,
            min_score,
        } => cmd_query(text, top_k, min_score).await,

        RagAction::Chat { top_k, inference_url } => cmd_chat(top_k, inference_url).await,

        RagAction::Docs => cmd_docs().await,

        RagAction::DeleteDoc { name, yes } => cmd_delete_doc(name, yes).await,

        RagAction::Serve {
            port,
            inference_url,
            top_k,
        } => crate::rag_api::run(port, inference_url, top_k).await,
    }
}

// ── init ──────────────────────────────────────────────────────────────────────

async fn cmd_init(
    eve_peer_id_str: Option<String>,
    capacity_mb: i64,
    embed_model: String,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature. Rebuild with: cargo build --features storage");

    #[cfg(feature = "storage")]
    {
        print_box_header("RAG Init");

        // Resolve Eve peer ID.
        let (client, local_peer_id) = crate::vpk::p2p_connect().await?;
        let eve_peer_id: PeerId = match &eve_peer_id_str {
            Some(s) => s.parse().context("invalid Eve peer ID")?,
            None => {
                print_info("No --eve-peer-id given; using local peer ID (single-node mode)");
                local_peer_id
            }
        };

        // Probe embedding model before creating the tenant.
        let embed = EmbedClient::new(None, Some(embed_model.clone()));
        print_info(&format!("Probing Ollama ({embed_model})…"));
        embed.check_dim().await?;
        print_success("Embedding model OK (768 dimensions)");

        // Create tenant on Eve.
        let my_peer_id = local_peer_id.to_string();
        let info = rpc_create_tenant(
            &client,
            &eve_peer_id,
            CreateTenantPayload {
                peer_id: my_peer_id,
                capacity_limit_mb: capacity_mb,
                display_name: Some("kwaai-rag".to_string()),
                vector_dimension: 768,
            },
        )
        .await
        .context("creating Eve tenant")?;

        let tenant_id = info.tenant_id;
        print_success(&format!("Tenant created: {tenant_id}"));

        // Create MetaStore directory and open to verify.
        let data_dir = kwaainet_dir().join("rag");
        MetaStore::open(&data_dir, tenant_id)?;

        // Save config.
        let mut cfg = KwaaiNetConfig::load_or_create()?;
        cfg.rag = Some(RagConfig {
            tenant_id: Some(tenant_id.to_string()),
            eve_peer_id: Some(eve_peer_id.to_string()),
            embed_model,
            inference_url: "http://localhost:8080".to_string(),
            top_k: 5,
        });
        cfg.save()?;

        print_success(&format!(
            "RAG initialised. Eve: {}  Tenant: {}",
            eve_peer_id, tenant_id
        ));
        println!("  Next: kwaainet rag ingest <file>");
        Ok(())
    }
}

// ── ingest ────────────────────────────────────────────────────────────────────

async fn cmd_ingest(
    file: std::path::PathBuf,
    doc_name: Option<String>,
    chunk_size: usize,
    chunk_overlap: usize,
) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id, eve_peer_id) = load_rag_config()?;

        let doc_name = doc_name.unwrap_or_else(|| {
            file.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        });

        let text = read_file_text(&file)?;
        print_info(&format!(
            "Ingesting '{}' ({} chars, {} byte file)",
            doc_name,
            text.chars().count(),
            text.len()
        ));

        let (client, _) = crate::vpk::p2p_connect().await?;
        let client = Arc::new(tokio::sync::Mutex::new(client));

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
        let data_dir = kwaainet_dir().join("rag");
        let meta = MetaStore::open(&data_dir, tenant_id)?;

        let mut cfg = IngestConfig::new(embed);
        cfg.chunk_cfg.chunk_size = chunk_size;
        cfg.chunk_cfg.chunk_overlap = chunk_overlap;

        let spinner = crate::progress::Spinner::start("Ingesting…");

        let result = ingest_text(
            &cfg,
            &meta,
            &doc_name,
            &text,
            move |vectors| {
                let client = client.clone();
                Box::pin(async move {
                    let guard = client.lock().await;
                    rpc_upload_vectors(&*guard, &eve_peer_id, tenant_id, vectors).await
                }) as Pin<Box<dyn std::future::Future<Output = Result<usize>> + Send>>
            },
            Some(|done: usize, total: usize| {
                let _ = (done, total);
            }),
        )
        .await?;

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

async fn cmd_query(query: String, top_k: usize, min_score: f64) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id, eve_peer_id) = load_rag_config()?;
        let (client, _) = crate::vpk::p2p_connect().await?;
        let client = Arc::new(tokio::sync::Mutex::new(client));

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
        let data_dir = kwaainet_dir().join("rag");
        let meta = MetaStore::open(&data_dir, tenant_id)?;

        let cfg = RetrieveConfig {
            top_k,
            min_score,
            use_sentence_window: false,
        };

        let spinner = crate::progress::Spinner::start("Retrieving…");
        let results = retrieve(
            &query,
            &cfg,
            &embed,
            &meta,
            move |embedding, k| {
                let client = client.clone();
                Box::pin(async move {
                    let guard = client.lock().await;
                    let raw = rpc_search_vectors(&*guard, &eve_peer_id, tenant_id, embedding, k).await?;
                    Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                }) as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
            },
        )
        .await?;
        spinner.finish("").await;

        if results.is_empty() {
            print_warning("No results found.");
            return Ok(());
        }

        print_box_header(&format!("Top {} results for: {}", results.len(), query));
        for (i, r) in results.iter().enumerate() {
            println!(
                "  [{}] score={:.4}  doc={}  chunk={}",
                i + 1,
                r.score,
                r.chunk_meta.doc_name,
                r.chunk_meta.chunk_index
            );
            println!("      {}", truncate(&r.chunk_meta.text, 200));
            println!();
        }
        Ok(())
    }
}

// ── chat ──────────────────────────────────────────────────────────────────────

async fn cmd_chat(top_k: usize, inference_url: String) -> Result<()> {
    #[cfg(not(feature = "storage"))]
    bail!("RAG requires the 'storage' feature.");

    #[cfg(feature = "storage")]
    {
        let (rag_cfg, tenant_id, eve_peer_id) = load_rag_config()?;
        let (client, _) = crate::vpk::p2p_connect().await?;
        let client = Arc::new(tokio::sync::Mutex::new(client));

        let embed = EmbedClient::new(None, Some(rag_cfg.embed_model.clone()));
        let data_dir = kwaainet_dir().join("rag");
        let meta = MetaStore::open(&data_dir, tenant_id)?;

        let retrieve_cfg = RetrieveConfig {
            top_k,
            min_score: 0.0,
            use_sentence_window: false,
        };

        let http = reqwest::Client::new();
        let mut history: Vec<ChatMessage> = vec![];

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

            // Retrieve context.
            let client2 = client.clone();
            let chunks = retrieve(
                &query,
                &retrieve_cfg,
                &embed,
                &meta,
                move |embedding, k| {
                    let client = client2.clone();
                    Box::pin(async move {
                        let guard = client.lock().await;
                        let raw = rpc_search_vectors(&*guard, &eve_peer_id, tenant_id, embedding, k).await?;
                        Ok(raw.into_iter().map(|r| (r.id, r.score)).collect())
                    }) as Pin<Box<dyn std::future::Future<Output = Result<Vec<(i64, f64)>>> + Send>>
                },
            )
            .await?;

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

async fn cmd_docs() -> Result<()> {
    let (_, tenant_id, _) = load_rag_config()?;
    let data_dir = kwaainet_dir().join("rag");
    let meta = MetaStore::open(&data_dir, tenant_id)?;

    let docs = meta.list_docs()?;
    if docs.is_empty() {
        print_info("No documents ingested yet. Run: kwaainet rag ingest <file>");
    } else {
        print_box_header(&format!("{} document(s)", docs.len()));
        for d in &docs {
            println!("  • {d}");
        }
    }
    Ok(())
}

// ── delete-doc ────────────────────────────────────────────────────────────────

async fn cmd_delete_doc(name: String, yes: bool) -> Result<()> {
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

        let (_, tenant_id, eve_peer_id) = load_rag_config()?;
        let (client, _) = crate::vpk::p2p_connect().await?;
        let data_dir = kwaainet_dir().join("rag");
        let meta = MetaStore::open(&data_dir, tenant_id)?;

        let ids = meta.delete_doc(&name)?;
        if ids.is_empty() {
            print_warning(&format!("Document '{name}' not found."));
            return Ok(());
        }

        rpc_delete_vectors(&client, &eve_peer_id, tenant_id, ids.clone())
            .await
            .context("deleting vectors from Eve")?;

        print_success(&format!(
            "Deleted '{name}' ({} chunks removed)",
            ids.len()
        ));
        Ok(())
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn load_rag_config() -> Result<(RagConfig, Uuid, PeerId)> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let rag = cfg
        .rag
        .context("RAG not initialised. Run: kwaainet rag init")?;

    let tenant_id: Uuid = rag
        .tenant_id
        .as_deref()
        .context("no tenant_id in RAG config")?
        .parse()
        .context("invalid tenant_id")?;

    let eve_peer_id: PeerId = rag
        .eve_peer_id
        .as_deref()
        .context("no eve_peer_id in RAG config")?
        .parse()
        .context("invalid eve_peer_id")?;

    Ok((rag, tenant_id, eve_peer_id))
}

fn read_file_text(path: &Path) -> Result<String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match ext.to_lowercase().as_str() {
        "txt" | "md" | "" => {
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
        }
        other => {
            // Try reading as UTF-8 text; warn on unknown extension.
            eprintln!(
                "Warning: unknown extension '.{other}', attempting to read as UTF-8 text"
            );
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
        }
    }
}

fn truncate(s: &str, max: usize) -> &str {
    let mut end = s.len().min(max);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}
