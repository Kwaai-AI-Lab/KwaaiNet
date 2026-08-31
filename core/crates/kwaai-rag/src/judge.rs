//! Reference-free scoring of a live chat answer.
//!
//! Distinct from the eval-time judge in `kwaai-cli`'s `cmd_eval`, which grades an
//! answer 0-2 *against a known `expected_answer`* and cannot run interactively. This
//! one grades 1-5 on whether the answer is supported by the passages that were
//! actually retrieved, which is the only question answerable without a reference.
//!
//! The eval judge is deliberately left alone: it defines a scoring scale that historic
//! benchmark numbers are stated on, and folding the two together would silently move
//! that baseline.

use anyhow::Result;
use serde_json::json;
use std::time::Duration;

use crate::retriever::RetrievedChunk;

/// How well an answer is supported by its retrieved context.
#[derive(Debug, Clone)]
pub struct AnswerScore {
    /// 1-5, where 5 is fully supported and directly responsive.
    pub score: f32,
    /// Whether every claim traces to the supplied passages.
    pub grounded: bool,
    /// One short sentence of justification.
    pub rationale: String,
}

const MAX_PASSAGE_CHARS: usize = 400;
const MAX_PASSAGES: usize = 10;

/// Grade `answer` against the passages it was given.
///
/// Returns `Err` if the model is unreachable or unparseable — the caller should treat
/// scoring as best-effort and carry on.
/// `plan` is [`crate::prompt::context_plan`]'s output — the indices into `chunks`, in
/// the order and count the model actually saw. It is required, not optional: numbering
/// the passages any other way grades the answer's `[n]` against a different passage
/// than the one it cites, and the resulting verdict is persisted.
pub async fn judge_answer(
    query: &str,
    answer: &str,
    chunks: &[RetrievedChunk],
    plan: &[usize],
    inference_url: &str,
    model: &str,
) -> Result<AnswerScore> {
    let mut ctx = String::new();
    for (i, &idx) in plan.iter().take(MAX_PASSAGES).enumerate() {
        let Some(c) = chunks.get(idx) else { continue };
        // The same text the prompt builder sent, not the bare chunk: the model was
        // shown the wider `surrounding` window whenever there was more of it.
        let t = crate::prompt::context_text(c);
        let t = if t.len() > MAX_PASSAGE_CHARS {
            &t[..t
                .char_indices()
                .take(MAX_PASSAGE_CHARS)
                .last()
                .map(|(i, ch)| i + ch.len_utf8())
                .unwrap_or(0)]
        } else {
            t
        };
        ctx.push_str(&format!("[{}] {}\n", i + 1, t));
    }

    let prompt = format!(
        "You are grading a retrieval-augmented answer. Judge ONLY whether the answer is \
supported by the passages and responsive to the question. Do not use outside knowledge, \
and do not reward or penalise style.\n\n\
Question: {query}\n\nPassages:\n{ctx}\nAnswer: {answer}\n\n\
Reply with JSON only, no prose, in exactly this form:\n\
{{\"score\": <integer 1-5>, \"grounded\": <true|false>, \"rationale\": \"<one short sentence>\"}}\n\
score 5 = every claim supported and the question fully answered; \
3 = partially supported or partially answered; \
1 = unsupported, contradicted, or a refusal when the passages do contain the answer. \
grounded = true only if every factual claim traces to a passage."
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(45))
        .build()?;
    let url = format!(
        "{}/v1/chat/completions",
        inference_url.trim_end_matches('/')
    );
    let body = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": 200,
    });

    let resp = client.post(&url).json(&body).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("judge: inference API returned {}", resp.status());
    }
    let v: serde_json::Value = resp.json().await?;
    let raw = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string();
    parse_verdict(&raw)
}

/// Extract the verdict from a model reply, tolerating fences and preamble.
pub fn parse_verdict(raw: &str) -> Result<AnswerScore> {
    let slice = match (raw.find('{'), raw.rfind('}')) {
        (Some(a), Some(b)) if b > a => &raw[a..=b],
        _ => anyhow::bail!("judge: no JSON object in reply: {raw:?}"),
    };
    let v: serde_json::Value = serde_json::from_str(slice)
        .map_err(|e| anyhow::anyhow!("judge: unparseable verdict: {e}\nraw: {raw:?}"))?;
    let score = v["score"]
        .as_f64()
        .or_else(|| v["score"].as_str().and_then(|s| s.parse().ok()))
        .ok_or_else(|| anyhow::anyhow!("judge: missing score in {slice:?}"))?;
    Ok(AnswerScore {
        score: (score as f32).clamp(1.0, 5.0),
        grounded: v["grounded"].as_bool().unwrap_or(false),
        rationale: v["rationale"].as_str().unwrap_or("").trim().to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_clean_verdict() {
        let s =
            parse_verdict(r#"{"score": 4, "grounded": true, "rationale": "Supported."}"#).unwrap();
        assert_eq!(s.score, 4.0);
        assert!(s.grounded);
        assert_eq!(s.rationale, "Supported.");
    }

    #[test]
    fn tolerates_fences_and_preamble() {
        let s = parse_verdict(
            "Here is my grade:\n```json\n{\"score\": 2, \"grounded\": false, \
             \"rationale\": \"Not in the passages.\"}\n```",
        )
        .unwrap();
        assert_eq!(s.score, 2.0);
        assert!(!s.grounded);
    }

    #[test]
    fn clamps_out_of_range_scores() {
        assert_eq!(parse_verdict(r#"{"score": 9}"#).unwrap().score, 5.0);
        assert_eq!(parse_verdict(r#"{"score": 0}"#).unwrap().score, 1.0);
    }

    #[test]
    fn rejects_a_reply_with_no_verdict() {
        assert!(parse_verdict("I cannot grade this.").is_err());
        assert!(parse_verdict(r#"{"rationale": "no score"}"#).is_err());
    }
}
