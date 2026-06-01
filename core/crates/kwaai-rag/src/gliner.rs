//! Thin async HTTP client for a GLiNER NER server.
//!
//! GLiNER (Generalist and Lightweight Named Entity Recognition) is a compact encoder model
//! (~0.5B) that runs on CPU in ~5 ms/chunk. This client treats it as an advisory pre-screener:
//! detected Person spans are injected into the LLM prompt as high-confidence hints, improving
//! recall of real names and reducing hallucination of generic roles.
//!
//! Expected API (see `scripts/gliner_server.py`):
//!   POST /ner
//!   Request:  {"text": "...", "labels": ["person"], "threshold": 0.4}
//!   Response: [{"text": "Joe Rassool", "label": "person", "score": 0.92}, ...]
//!
//! All errors are treated as soft failures — `person_spans()` returns `[]` so the build
//! continues without GLiNER guidance rather than aborting.

/// HTTP client for a running GLiNER server.
#[derive(Clone, Debug)]
pub struct GliNERClient {
    url: String,
    threshold: f32,
    client: reqwest::Client,
}

impl GliNERClient {
    pub fn new(url: &str, threshold: f32) -> Self {
        Self {
            url: url.trim_end_matches('/').to_owned(),
            threshold,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("building reqwest client for GLiNER"),
        }
    }

    /// Returns unique Person span texts detected in `text` above the configured threshold.
    /// Never panics or returns an error — any failure degrades to an empty hint list.
    pub async fn person_spans(&self, text: &str) -> Vec<String> {
        let body = serde_json::json!({
            "text": text,
            "labels": ["person"],
            "threshold": self.threshold,
        });
        let resp = match self
            .client
            .post(format!("{}/ner", self.url))
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!("GLiNER request failed (continuing without hints): {e}");
                return vec![];
            }
        };
        let json: serde_json::Value = match resp.json().await {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("GLiNER response parse error: {e}");
                return vec![];
            }
        };
        let arr = match json.as_array() {
            Some(a) => a,
            None => {
                tracing::warn!("GLiNER response is not a JSON array");
                return vec![];
            }
        };
        let mut spans: Vec<String> = arr
            .iter()
            .filter_map(|item| {
                let score = item.get("score")?.as_f64()? as f32;
                if score < self.threshold {
                    return None;
                }
                item.get("text")?.as_str().map(|s| s.to_owned())
            })
            .collect();
        spans.sort();
        spans.dedup();
        spans
    }
}
