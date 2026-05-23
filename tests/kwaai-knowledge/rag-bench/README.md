# RAG Benchmark: KwaaiNet vs Gemini

Scores kwaainet's RAG pipeline against Google AI Studio (Gemini 1.5 Pro) on the same 31-document corpus using identical questions. Re-run after each Phase 2/3 feature lands to track progress.

## Corpus

31 DOCX chapters from the PreRAG output at `/Users/rezarassool/Source/PreRAG/output/`.  
The same files are ingested into kwaainet (`rag sync`) and uploaded to Gemini via the File API.

## Question set

`questions.json` — 30 questions in three categories:

| Category | Count | Description |
|----------|-------|-------------|
| `factoid` | 15 | Single-hop, specific facts from the corpus |
| `inferential` | 10 | Multi-chapter reasoning questions |
| `out_of_scope` | 5 | Facts not in the corpus — tests hallucination resistance |

15 of the 30 questions have a `reference_answer` for golden-set F1 scoring.

## Metrics

| Metric | What it measures | Systems |
|--------|-----------------|---------|
| **LLM Judge (0–3)** | Faithfulness + Relevance + Completeness, rated by the kwaainet shard API | Both |
| **Golden F1** | Fraction of key reference phrases found in the answer | Both |
| **Hit@5** | Correct source document in top-5 retrieved chunks | kwaainet only |
| **MRR** | Mean Reciprocal Rank of correct source | kwaainet only |
| **Latency** | Retrieval ms, generation ms, total ms | kwaainet (retrieval split); Gemini (total only) |

## Prerequisites

```bash
# 1. kwaainet must be running with RAG initialised and corpus synced
kwaainet rag serve --port 9090 &
kwaainet shard api --port 8080 &   # used as LLM judge

# 2. Google AI Studio API key
export GOOGLE_API_KEY=<your key from aistudio.google.com>

# 3. Python dependency
pip install google-generativeai requests
```

## Run

```bash
cd tests/rag-bench

# Step 1 — collect kwaainet answers (~2 min)
python run_kwaainet.py

# Step 2 — collect Gemini answers (~5 min; uploads corpus on first run)
python run_gemini.py

# Step 3 — score both (~5 min for LLM judge on 60 answer pairs)
python score.py

# Step 4 — generate report
python report.py
cat results/report.md
```

## Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `KWAAINET_URL` | `http://localhost:9090` | kwaainet rag serve endpoint |
| `JUDGE_URL` | `http://localhost:8080` | kwaainet shard API (LLM judge) |
| `GOOGLE_API_KEY` | *(required)* | Google AI Studio API key |
| `GEMINI_MODEL` | `gemini-1.5-pro` | Gemini model to use |
| `CORPUS_DIR` | `/Users/rezarassool/Source/PreRAG/output` | Path to DOCX corpus |

## Output files

All outputs land in `results/` (gitignored except `.gitkeep`):

```
results/
  kwaainet_results.json   — answers + sources + latency per question
  gemini_results.json     — answers per question
  gemini_file_refs.json   — cached Gemini File API URIs (skip re-upload)
  scores.json             — aggregated metrics for both systems
  report.md               — human-readable comparison table
```

## Interpreting results

- **Judge score > 2.5** — good grounded answers
- **Judge score < 1.5** — hallucination or relevance problems
- **Hit@5 > 0.8** — retrieval is finding the right documents
- **MRR > 0.7** — correct document is near the top of retrieved results
- **Out-of-scope faithfulness > 2** — system correctly refuses to invent answers

After Phase 2 (hybrid BM25 + dense, reranking), expect Hit@5 and MRR to improve.  
After HyDE query expansion, expect inferential question judge scores to improve.
