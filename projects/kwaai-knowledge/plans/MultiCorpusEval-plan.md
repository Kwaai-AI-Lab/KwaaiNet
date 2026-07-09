# Multi-Corpus RAG Evaluation Plan

## Context

D6 (District Six memoir) achieved 88.9% eval accuracy (v0.4.148 seed6). To validate that
the RAG pipeline generalizes beyond one memoir-style document, we evaluate across 15+ diverse
corpora — covering technical standards, literary fiction, scientific papers, legal documents,
and conversational transcripts — each with its own KB and eval question set.

Goal: identify domain-specific failure modes and drive targeted improvements to chunking,
entity types, retrieval, and graph extraction.

---

## Available Corpora (Google Drive)

From `https://drive.google.com/drive/folders/1vdU-DQTI0eRr7M5444oswcgasiXY43Zu`:

| Folder | Domain type | KB name |
|--------|-------------|---------|
| Manhattan Project | Historical narrative | Manhattan |
| Moby-Dick and companion works | Literary fiction | MobyDick |
| Legal Documents | Legal / formal | Legal |
| Meeting Transcripts | Conversational | Meetings |
| Python Documentation | Technical reference | PythonDocs |
| NIST_AI Security & Governance | Technical standards | NIST |
| Climate Science | Policy / science | Climate |
| Internet Standards (RFCs) | Technical standards | RFCs |
| Deep Sea Biology | Scientific | DeepSea |
| Dream-Based Memory Consolidation | Academic paper | DreamMem |
| Astrophysics - Space Exploration | Scientific | Astrophysics |
| Country History/Culture | Historical | CountryHistory |
| War and Peace | Literary fiction (long) | WarPeace |
| Poems | Literary / sparse | Poems |
| OpenStreetMap Data Documentation | Technical reference | OSMDocs |

---

## Step 1: Download and Organize

Download each Drive folder. Place files at:
```
tests/kwaai-knowledge/{KB}/
  docs/                  ← source documents
  eval_questions.json    ← from Drive (may need format conversion)
  doc_schema.yaml        ← to be created per corpus (see Step 3)
```

Check Q&A format compatibility — `kwaainet rag eval` expects:
```json
{
  "id": "q01",
  "type": "entity_description",
  "question": "...",
  "expected_answer": "...",
  "expected_keywords": ["word1", "word2"]
}
```
If the Drive Q&A format differs, write a one-off conversion script per corpus.

---

## Step 2: Priority Order for Evaluation

Run in this order — most-to-least informative for generalization:

| Priority | KB | Rationale |
|----------|----|-----------|
| 1 | Manhattan | Closest to D6 (historical narrative) — baseline comparison |
| 2 | MobyDick | Character-heavy fiction — tests Person/Place entity recall |
| 3 | Legal | Structured formal language — tests Legislation entity type |
| 4 | Meetings | Conversational — tests chunking on non-prose, speaker turns |
| 5 | PythonDocs | Technical reference — tests retrieval on dense structured text |
| 6 | NIST | Standards doc — Legislation + Concept heavy |
| 7 | Climate | Policy/science hybrid — multi-domain entity mix |
| 8 | RFCs | Technical standards — highly structured, Concept-heavy |
| 9 | DeepSea | Scientific paper — few named persons, many concepts |
| 10 | DreamMem | Academic — citation-heavy, Publication entity test |
| 11 | Astrophysics | Scientific — Person/Place/Concept mix |
| 12 | CountryHistory | Historical — depends on country, likely Person/Place/Org |
| 13 | WarPeace | Very long doc, many characters — scalability test |
| 14 | Poems | Sparse entities, short chunks — edge case |
| 15 | OSMDocs | Technical reference — mostly concepts, few named entities |

---

## Step 3: Per-Corpus Entity Types

Current supported types: `Person, Place, Organization, Legislation, Publication`

| KB | Recommended `--entity-types` | Notes |
|----|------------------------------|-------|
| Manhattan | Person,Place,Organization | Same as D6 |
| MobyDick | Person,Place,Organization | Fictional names — interesting test |
| Legal | Person,Organization,Legislation | Drop Publication |
| Meetings | Person,Organization | No Legislation or Publication |
| PythonDocs | Organization,Publication | Mostly module/function concepts; no Legislation |
| NIST | Organization,Legislation,Publication | Standards citations |
| Climate | Organization,Legislation,Publication | Policy-heavy |
| RFCs | Organization,Publication | RFC numbers as Publication |
| DeepSea | Person,Organization,Publication | Citations matter |
| DreamMem | Person,Organization,Publication | Academic citations |
| Astrophysics | Person,Organization,Publication | Discoverers, journals |
| CountryHistory | Person,Place,Organization | Historical events |
| WarPeace | Person,Place,Organization | Many characters |
| Poems | Person,Place | Sparse |
| OSMDocs | Organization | Mostly technical terms |

**Known gap**: `Concept` and `Event` are not yet implemented entity types. Queries asking
"what is X?" or "what happened at Y?" will miss graph boost — flag as Concept/Event
failure mode during analysis.

---

## Step 4: Per-Corpus Build Commands

### Template (adapt per corpus):
```bash
KB=Manhattan
ET="Person,Place,Organization"

kwaainet rag init --name $KB --embed-model nomic-embed-text

kwaainet rag ingest --kb $KB \
  --file "tests/kwaai-knowledge/$KB/docs/<document.pdf>" \
  --doc-schema "tests/kwaai-knowledge/$KB/doc_schema.yaml"

kwaainet rag graph build --kb $KB \
  --model llama3.1:8b \
  --inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd" \
  --workers 4 \
  --entity-types "$ET" \
  --no-relations \
  --graph-window 1 \
  --timeline

kwaainet rag graph score --kb $KB

kwaainet rag eval \
  --kb $KB \
  --questions "tests/kwaai-knowledge/$KB/eval_questions.json" \
  | tee "tests/kwaai-knowledge/results/eval_${KB}_$(date +%Y%m%d_%H%M%S).md"
```

### Doc schema per corpus
Start with a minimal schema and add `skip: true` overrides only after inspecting the
first ingest for structural issues (TOC pulled into body, appendices mis-labelled, etc.):
```yaml
doc_type: general
sections: []
```

---

## Step 5: Logging and Results

Every eval run logs to `tests/kwaai-knowledge/results/eval_{KB}_{YYYYMMDD_HHMMSS}.md`.

Running summary: `tests/kwaai-knowledge/results/multi_corpus_summary.md`

```markdown
| KB | Score | Chunk size | Entity types | Notes |
|----|-------|------------|--------------|-------|
| D6 | 88.9% | 100w | Person,Place,Org,Legislation,Publication | Baseline |
| Manhattan | ?% | 100w | Person,Place,Organization | |
...
```

---

## Step 6: Generalization Analysis

For each corpus scoring significantly below D6, investigate:

### Retrieval failures
- **Missing entity recall**: entity in answer absent from graph → entity type gap or poor extraction
- **BM25 vs vector imbalance**: technical corpora may need higher BM25 weight for exact terms
- **Chunk size mismatch**: 100w may be too large for poetry, too small for dense technical text

### Entity extraction failures
- **Concept gap**: "what is X?" queries score 0 with no Concept entity type
- **Event gap**: "what happened at X?" queries score 0 with no Event entity type
- **Fictional hallucination**: Moby-Dick character names may trigger false-positive entity types

### Structural failures
- **Meeting transcripts**: sentence-aligned chunking may split mid-turn
- **RFC-style**: very structured sections may confuse section detection
- **Poetry**: stanzas must not split mid-line

### Metrics per corpus
- Overall eval score (%)
- Per-question type breakdown (entity_description, factual, relational, temporal)
- Graph entity count vs. expected from eval ground truth
- Timeline event count

---

## Step 7: Corpus-Driven Improvements

After all evals, group failures into actionable work items:

| Failure pattern | Proposed fix |
|-----------------|-------------|
| Concept queries always miss | Add `Concept` entity type (Phase 5) |
| Event queries always miss | Add `Event` entity type with temporal linking |
| Low BM25 precision on technical terms | Tunable BM25 weight per KB |
| Poetry chunks split mid-line | Stanza-aware chunker |
| Meeting transcript chunking poor | Turn-boundary chunker |
| Fiction entity hallucination | Fiction-mode flag: disable entity validation |
| Long doc scalability (War & Peace) | Verify index build time and retrieval latency |

---

## Verification

After each corpus eval:
1. Confirm eval log written to `results/`
2. Score recorded in `multi_corpus_summary.md`
3. If score < 50%, inspect per-question failures before proceeding

Final success criteria: all 15 corpora evaluated, summary table complete, at least 3
confirmed generalizable failure modes with proposed fixes.

---

## Files to Create

| File | Purpose |
|------|---------|
| `tests/kwaai-knowledge/{KB}/doc_schema.yaml` | Per-corpus section overrides |
| `tests/kwaai-knowledge/{KB}/eval_questions.json` | Q&A (from Drive or converted) |
| `tests/kwaai-knowledge/multi_corpus_eval.sh` | Shell script iterating all corpora |
| `tests/kwaai-knowledge/results/multi_corpus_summary.md` | Running summary table |
