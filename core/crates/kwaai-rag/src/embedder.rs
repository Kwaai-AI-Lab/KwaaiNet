use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "nomic-embed-text";
pub const DEFAULT_DIM: usize = 768;

/// nomic-embed-text requires asymmetric instruction prefixes for accurate retrieval.
/// Without them all texts land in general-purpose space and score in a tight 0.62-0.66
/// cluster with no useful separation between relevant and irrelevant chunks.
const NOMIC_QUERY_PREFIX: &str = "search_query: ";
const NOMIC_DOC_PREFIX: &str = "search_document: ";

#[derive(Clone)]
pub struct EmbedClient {
    http: reqwest::Client,
    pub base_url: String,
    pub model: String,
}

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: serde_json::Value,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

impl EmbedClient {
    pub fn new(base_url: Option<String>, model: Option<String>) -> Self {
        let base_url = base_url
            .or_else(|| std::env::var("OLLAMA_BASE_URL").ok())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        Self {
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .connect_timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("embedder http client"),
            base_url,
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        }
    }

    fn is_nomic(&self) -> bool {
        self.model.contains("nomic")
    }

    /// Embed a single search query. Adds the `search_query:` prefix for nomic-embed-text.
    pub async fn embed_one(&self, text: &str) -> Result<Vec<f32>> {
        let owned;
        let text = if self.is_nomic() {
            owned = format!("{NOMIC_QUERY_PREFIX}{text}");
            owned.as_str()
        } else {
            text
        };
        let mut batch = self.embed_raw(&[text]).await?;
        batch.pop().context("empty embedding response")
    }

    /// Embed a batch of document chunks. Adds the `search_document:` prefix for nomic-embed-text.
    pub async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        if self.is_nomic() {
            let prefixed: Vec<String> = texts
                .iter()
                .map(|t| format!("{NOMIC_DOC_PREFIX}{t}"))
                .collect();
            let refs: Vec<&str> = prefixed.iter().map(|s| s.as_str()).collect();
            self.embed_raw(&refs).await
        } else {
            self.embed_raw(texts).await
        }
    }

    /// Raw embed call — no prefix manipulation.
    async fn embed_raw(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/api/embed", self.base_url);
        let body = EmbedRequest {
            model: &self.model,
            input: serde_json::json!(texts),
        };
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .with_context(|| format!("POST {url} — is Ollama running?"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("Ollama embed error {status}: {text}");
        }

        let parsed: EmbedResponse = resp.json().await.context("parsing embed response")?;
        Ok(parsed.embeddings)
    }

    /// Probe Ollama and return the embedding dimension.
    pub async fn probe_dim(&self) -> Result<usize> {
        let emb = self.embed_raw(&["probe"]).await?;
        let emb = emb.into_iter().next().context("empty probe response")?;
        Ok(emb.len())
    }

    /// Probe Ollama and verify the dimension matches `expected`. Call at startup.
    pub async fn check_dim_matches(&self, expected: usize) -> Result<()> {
        let actual = self.probe_dim().await?;
        if actual != expected {
            bail!(
                "Embedding model '{}' returns {} dimensions but KB was initialised with {}. \
                 Destroy and re-init the KB with the correct model.",
                self.model,
                actual,
                expected,
            );
        }
        Ok(())
    }
}
