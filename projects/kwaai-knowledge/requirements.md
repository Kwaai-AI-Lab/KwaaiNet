# kwaai-knowledge — Requirements

## Purpose

Ingest, index, and retrieve from personal and organizational knowledge bases using hybrid
RAG so KwaaiNet nodes can answer questions grounded in private documents — with accuracy
sufficient for real-world memoir and archival use cases.

## Functional requirements

1. **FR-K1** Ingest PDF, Markdown, and plain-text documents; chunk into sentence-aligned ~100-word segments.
2. **FR-K2** Embed chunks using a configurable embedding model; store in vector index with BM25 parallel index.
3. **FR-K3** Extract named entities (Person, Place, Organization) from each chunk using an LLM, with configurable context window (adjacent chunks).
4. **FR-K4** Store entities and relations in a knowledge graph; support graph scoring and deduplication.
5. **FR-K5** Accept a family tree YAML seed file to inject ground-truth familial relations that LLM extraction cannot reliably produce.
6. **FR-K6** Support document schema (schema.org) metadata for Books: ISBN, author, publisher, copyright, genre; use metadata to improve extraction context.
7. **FR-K7** Auto-detect schema from PDF headers when `--doc-schema` is omitted (ISBN pattern, publisher, copyright year/holder).
8. **FR-K8** Parse book index sections (`index_seeds: true`) as entity name seeds without LLM extraction.
9. **FR-K9** Support section-aware ingestion: skip noisy sections (ToC, appendix, endnotes); apply narrator overrides (Editor's Note).
10. **FR-K10** Inject top entity description as synthetic chunk in retrieval for entity-centric queries.
11. **FR-K11** Hybrid retrieval: BM25 + vector similarity + graph reranking; return ranked, deduplicated chunks.
12. **FR-K12** Dream mode: iterative self-RAG loop that refines answers over multiple retrieval cycles.
13. **FR-K13** Eval harness: score accuracy against a JSON question set; report per-question pass/fail.
14. **FR-K14** REST API for external RAG access (used by VPK / external clients).

## Non-functional requirements

- **NFR-K1** Graph build throughput: ≥ 10 chunks/minute on NVIDIA A6000 with llama3.1:8b.
- **NFR-K2** Retrieval latency: < 500ms for a hybrid BM25+vector query on a 1,136-chunk corpus.
- **NFR-K3** Entity extraction precision (Person): ≥ 0.90 with optimal settings.
- **NFR-K4** D6 eval accuracy target: ≥ 80% on `d6_eval_questions.json`.
- **NFR-K5** Graph build must be resumable (progress saved per chunk; `--reset-graph` to start fresh).
- **NFR-K6** Multi-tenant: knowledge bases must be isolated by tenant UUID.
- **NFR-K7** Targets: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows.

## Out of scope

- Homomorphic encryption of vectors — that is kwaai-storage.
- LLM inference serving — that is kwaai-compute; RAG calls inference via HTTP.
- Relation extraction for 8B models — disabled by default (precision too low).

## Dependencies

- **kwaai-compute** — LLM inference for entity extraction and answer generation.
- **kwaai-storage** — optional: VPK-backed vector storage for encrypted knowledge bases.
