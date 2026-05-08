use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "nomic-embed-text";
pub const EXPECTED_DIM: usize = 768;

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
            http: reqwest::Client::new(),
            base_url,
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        }
    }

    pub async fn embed_one(&self, text: &str) -> Result<Vec<f32>> {
        let mut batch = self.embed_batch(&[text]).await?;
        batch.pop().context("empty embedding response")
    }

    pub async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
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

    /// Probe Ollama and verify the embedding dimension.  Call at startup.
    pub async fn check_dim(&self) -> Result<()> {
        let emb = self.embed_one("probe").await?;
        if emb.len() != EXPECTED_DIM {
            bail!(
                "Embedding model '{}' returns {} dimensions; expected {}. \
                 Run: ollama pull {}",
                self.model,
                emb.len(),
                EXPECTED_DIM,
                DEFAULT_MODEL,
            );
        }
        Ok(())
    }
}
