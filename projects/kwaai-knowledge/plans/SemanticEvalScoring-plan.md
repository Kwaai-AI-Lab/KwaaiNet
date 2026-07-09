# Semantic Keyword Scoring for RAG Eval (Retrieval + Generation)

## Context

The current `kwaainet rag eval` scorer is binary per keyword group: a group either hits (1.0)
or misses (0.0) based on token overlap against the **LLM answer**. This has two problems:

1. **Paraphrase blindness** — LLM answers semantically correct but use different words score 0.0%
   (confirmed across MobyDick, Legal, Meetings, PythonDocs, NIST, Climate: all 0.0%)
2. **No retrieval visibility** — if retrieval surfaces the right content but the LLM fails to
   use it, the cause is invisible. Conversely, if retrieval misses the content entirely, we
   don't know without inspecting the sources manually.

Goal: add separate **retrieval** and **generation** semantic scores so each question shows
whether the failure is in retrieval (right content not surfaced) or generation (right content
retrieved but LLM didn't use it).

Previously implemented: generation semantic score (`keyword_group_score` in `rag_cmd.rs`).
This plan adds the retrieval score.

---

## Two-Score Model

For each `KeywordGroup` per question:

```
retrieval_score(keyword, chunks) =
    max over all retrieved chunks of:
        max(token_overlap(keyword, chunk_text),
            scale_cosine(embed(keyword), embed(chunk_text)))

generation_score(keyword, answer) =
    max(token_overlap(keyword, answer),
        scale_cosine(embed(keyword), embed(answer)))   ← already implemented
```

Both use `scale_cosine(cos, low=0.55, high=0.85)` linear mapping (configurable).

**Retrieval score** measures: did the retriever surface chunks containing this concept?
**Generation score** measures: did the LLM answer actually express this concept?

A low retrieval score + low generation score → retrieval failure.
A high retrieval score + low generation score → LLM failed to use retrieved content.
Both high → full success.

---

## Implementation

All changes in one file: `core/crates/kwaai-cli/src/rag_cmd.rs`.

### 1. `Row` struct — add `retrieval_hits` field

```rust
struct Row {
    id: String,
    question: String,
    answer: String,
    retrieved_docs: Vec<String>,
    keyword_hits: f32,       // generation score (already exists)
    retrieval_hits: f32,     // NEW: retrieval score
    total_keywords: f32,
    latency_ms: u128,
    judge_score: Option<u8>,
}
```

### 2. New helper `keyword_group_retrieval_score`

Add alongside the existing `keyword_group_score` (already in rag_cmd.rs):

```rust
/// Retrieval-side scoring: max over all retrieved chunk embeddings.
/// Token overlap is checked against the concatenated chunk text tokens.
fn keyword_group_retrieval_score(
    group: &KeywordGroup,
    context_text: &str,
    context_toks: &HashSet<String>,
    chunk_embs: &[Vec<f32>],
    keyword_embs: &HashMap<String, Vec<f32>>,
    low: f32,
    high: f32,
) -> f32 {
    let keywords: Vec<&String> = match group {
        KeywordGroup::Single(s) => vec![s],
        KeywordGroup::Synonyms(v) => v.iter().collect(),
    };
    keywords.iter().map(|kw| {
        let tok = if keyword_hit(kw, context_text, context_toks) { 1.0_f32 } else { 0.0 };
        let sem = keyword_embs.get(*kw).map_or(0.0, |ke| {
            chunk_embs.iter()
                .map(|ce| scale_cosine(eval_cosine_sim(ke, ce), low, high))
                .fold(0.0_f32, f32::max)
        });
        tok.max(sem)
    }).fold(0.0_f32, f32::max)
}
```

### 3. Per-question retrieval scoring (after retrieval, before LLM call)

Insert between the retrieval call and the LLM payload construction:

```rust
// Embed all retrieved chunk texts for retrieval-side semantic scoring.
let chunk_embs: Vec<Vec<f32>> = if semantic_score && !chunks.is_empty() {
    let texts: Vec<&str> = chunks.iter()
        .map(|c| c.chunk_meta.text.as_str())
        .collect();
    embed.embed_batch(&texts).await.unwrap_or_default()
} else {
    vec![]
};

// Build context token set for retrieval token-overlap scoring.
let context_text: String = chunks.iter()
    .map(|c| c.chunk_meta.text.as_str())
    .collect::<Vec<_>>()
    .join(" ");
let context_toks = eval_tokens(&context_text);

// Retrieval score: keyword recall over retrieved chunks.
let retrieval_hits: f32 = q.expected_keywords.iter().map(|group| {
    keyword_group_retrieval_score(
        group,
        &context_text,
        &context_toks,
        &chunk_embs,
        &keyword_embs,
        semantic_low,
        semantic_high,
    )
}).sum::<f32>() + q.numeric_answer.as_ref().map_or(0.0, |na| {
    numeric_proximity_score(&context_text, na.correct, na.tolerance)
});
```

This runs immediately after retrieval (where `chunks: Vec<RetrievedChunk>` is available)
and before the LLM call. When `--semantic-score` is off, `chunk_embs` is empty and only
token-overlap is used for retrieval scoring (cheap, always computed).

### 4. Per-question display

Update the inline progress line to show both scores:

```rust
// e.g.: "  ret=3/4  gen=2.7/4  1234ms"
println!("  ret={ret_display}  gen={kw_display}  {latency_ms}ms");
```

### 5. Report: two hit-rate columns + summary rows

Per-question table gains a "Retrieval" column alongside the existing "Hit rate" (renamed
to "Generation"):

```
| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|---------|
| Q01 | ... | 3/4 (75%) | 2.7/4 (68%) | doc.pdf | 1234ms |
```

Summary section gains two recall rows:

```
| Overall retrieval recall  | 68.5% (54.8/80) |
| Overall generation recall | 52.3% (41.8/80) |
```

The gap between them quantifies LLM generation loss on top of retrieval.

---

## Embedding Cost

Per question with `--semantic-score`:
- 1 `embed_batch(chunk_texts)` call (top_k chunks, default 20)
- 1 `embed_one(answer)` call (generation scoring, already implemented)

Keywords are pre-embedded once before the loop. Total additional calls per 20-question eval:
20 × (1 batch of 20 chunks + 1 answer embed) = 420 embedding calls across 40 API requests.
At ~30ms/request on local Ollama this adds ~1.2s overhead per question — acceptable.

---

## Files Changed

| File | Change |
|------|--------|
| `core/crates/kwaai-cli/src/rag_cmd.rs` | `Row` struct, new helper, retrieval embed + score, display, report |
| `core/crates/kwaai-cli/src/cli.rs` | Already has the 3 flags from previous implementation |

No other files.

---

## Verification

```bash
cd core && cargo build -p kwaainet
cp target/debug/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet

# Token-overlap only (no flag) — unchanged behaviour, retrieval score uses token-overlap
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json

# Both retrieval + generation semantic scoring
kwaainet rag eval --kb D6 \
  --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --semantic-score

# MobyDick — both retrieval and generation should rise above 0.0%
kwaainet rag eval --kb MobyDick \
  --questions tests/kwaai-knowledge/MobyDick/eval_questions.json \
  --semantic-score
```

Expected: report shows two recall rows and two columns; D6 retrieval recall ≥ generation recall
(retrieval surfaces right content, LLM may not always fully express it).

