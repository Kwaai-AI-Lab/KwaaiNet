# Semantic Keyword Scoring for RAG Eval

## Context

The current `kwaainet rag eval` scorer is binary per keyword group: a group either hits (1.0)
or misses (0.0) based on token overlap. This causes MobyDick to score 0.0% even when the LLM
retrieves correct passages and generates semantically accurate answers — because the answer
paraphrases rather than reusing the exact expected keywords (e.g. "attorney" instead of
"lawyer", "seafarer" instead of "sailor").

Goal: replace binary keyword matching with a continuous [0.0, 1.0] score per keyword group
using embedding-based cosine similarity, so partial credit is awarded when the answer captures
the right meaning in different words. Token-overlap remains the primary signal; semantic
similarity acts as a soft fallback that can never exceed 1.0 per group.

---

## Scoring Formula

For each `KeywordGroup` in a question:

```
semantic_score(keyword, answer) =
    clamp((cosine(embed(keyword), embed(answer)) - low) / (high - low), 0.0, 1.0)

group_score = max(token_overlap_binary as f32, semantic_score)
```

Defaults: `low = 0.55`, `high = 0.85` (configurable via `--semantic-low / --semantic-high`).

- Cosine ≥ 0.85 → full credit (1.0)
- Cosine 0.55–0.85 → partial credit (linear interpolation)
- Cosine < 0.55 → no credit (0.0)
- Token overlap hit always wins (score = 1.0, no embedding call needed)

For `Synonyms(vec)`, take `max(group_score)` across all synonyms — consistent with the
existing binary behaviour.

The overall recall percentage becomes a true float (e.g. `17.3/20` instead of `14/20`).

---

## Implementation

### 1. CLI flag — `EvalArgs` in `core/crates/kwaai-cli/src/rag_cmd.rs`

Add three new optional flags to `EvalArgs` (alongside existing `--top-k`, `--llm-judge`):

```rust
#[arg(long, default_value_t = false)]
semantic_score: bool,

#[arg(long, default_value_t = 0.55)]
semantic_low: f32,

#[arg(long, default_value_t = 0.85)]
semantic_high: f32,
```

### 2. Helper functions in `rag_cmd.rs` (near `keyword_hit`, ~line 7092)

```rust
fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    // identical to cosine_sim_f32 in graph.rs — 10 lines, no dep needed
}

fn scale_cosine(cos: f32, low: f32, high: f32) -> f32 {
    ((cos - low) / (high - low)).clamp(0.0, 1.0)
}
```

> Do NOT make `cosine_sim_f32` in `graph.rs` pub — it would expose an internal detail.
> Duplicate the 10-line function in `rag_cmd.rs` instead.

### 3. Pre-embed keywords before the question loop (in `cmd_eval`)

```rust
// Collect all unique keyword strings from all questions
let keyword_strings: Vec<String> = questions.iter()
    .flat_map(|q| q.expected_keywords.iter().flat_map(|g| match g {
        KeywordGroup::Single(s)   => vec![s.clone()],
        KeywordGroup::Synonyms(v) => v.clone(),
    }))
    .collect::<std::collections::HashSet<_>>()
    .into_iter().collect();

// Batch-embed once — reuse the KB's EmbedClient (already available in cmd_eval)
let keyword_embs: HashMap<String, Vec<f32>> = if args.semantic_score {
    let refs: Vec<&str> = keyword_strings.iter().map(|s| s.as_str()).collect();
    let vecs = embedder.embed_batch(&refs).await?;
    keyword_strings.into_iter().zip(vecs).collect()
} else {
    HashMap::new()
};
```

The `EmbedClient` / `embedder` variable already exists in `cmd_eval` — it's used for query
embedding in the retrieval path. Reuse it directly.

### 4. Per-question: embed the answer, score each keyword group

After the LLM answer is generated (around line 7631):

```rust
let answer_emb: Option<Vec<f32>> = if args.semantic_score && !answer.is_empty() {
    Some(embedder.embed_one(&answer).await.unwrap_or_default())
} else {
    None
};
```

Replace the binary `keyword_hit()` call with a float-returning function:

```rust
fn keyword_group_score(
    group: &KeywordGroup,
    answer_tokens: &HashSet<String>,
    answer_emb: Option<&[f32]>,
    keyword_embs: &HashMap<String, Vec<f32>>,
    low: f32,
    high: f32,
) -> f32 {
    let keywords: Vec<&String> = match group {
        KeywordGroup::Single(s)   => vec![s],
        KeywordGroup::Synonyms(v) => v.iter().collect(),
    };
    keywords.iter().map(|kw| {
        // Token-overlap (existing logic, now returns f32)
        let tok = if keyword_hit(kw, answer_tokens) { 1.0_f32 } else { 0.0 };
        // Semantic fallback
        let sem = match (answer_emb, keyword_embs.get(*kw)) {
            (Some(ae), Some(ke)) => scale_cosine(cosine_sim(ae, ke), low, high),
            _ => 0.0,
        };
        tok.max(sem)
    }).fold(0.0_f32, f32::max)
}
```

`keyword_hits` in `Row` is already `f32` — no type changes needed downstream.

### 5. Report output

Add one line to the Summary section when `--semantic-score` is active:

```
| Scoring mode | token-overlap + semantic embedding (low=0.55, high=0.85) |
```

Per-question hit rates naturally become float (e.g. `2.7/4 (67.5%)`) — no table format change needed.

---

## Files Changed

| File | Change |
|------|--------|
| `core/crates/kwaai-cli/src/rag_cmd.rs` | All changes — flags, helpers, embed loop, scoring |

No other files. The `EmbedClient` is already in scope; `cosine_sim` is duplicated inline.

---

## Verification

```bash
# Build
cd core && cargo build -p kwaainet

# Baseline (unchanged behaviour — no flag)
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json

# Semantic scoring enabled
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --semantic-score

# MobyDick — was 0.0%, should now show partial credit
kwaainet rag eval --kb MobyDick \
  --questions tests/kwaai-knowledge/MobyDick/eval_questions.json \
  --semantic-score

# Threshold tuning
kwaainet rag eval --kb MobyDick \
  --questions tests/kwaai-knowledge/MobyDick/eval_questions.json \
  --semantic-score --semantic-low 0.60 --semantic-high 0.90
```

Expected: D6 score stays ≥ 88.9% (token overlap still wins); MobyDick rises above 0.0%.

---

