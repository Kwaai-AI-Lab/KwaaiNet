# kwaai-knowledge — Roadmap

## Shipped

- PDF, Markdown, plain-text ingestion
- Sentence-aligned 100-word chunking
- BM25 + vector hybrid retrieval
- Entity extraction (Person/Place/Organization) with configurable context window
- Knowledge graph: entity/relation store, deduplication, scoring
- Family tree YAML seed for ground-truth relation injection
- `GraphIngestConfig.context_window` (+7pp recall with window=1)
- `--no-relations` flag (disables LLM relation extraction for 8B models)
- Document schema (schema.org Book): ISBN, author, publisher, copyright, genre
- Auto-detect schema from PDF headers
- Section-aware ingestion: skip ToC/Appendix/Endnotes; narrator override for Editor's Note
- Index seed parsing: parse book index entries as entity name seeds (`index_seeds: true`)
- Top entity description injected as synthetic chunk in retrieval
- Relation-aware entity injection in retrieval context
- Dream (iterative self-RAG) mode
- Eval harness scoring against JSON question set
- REST API for external RAG access
- D6 knowledge base (District Six memoir) with document schema
- `kwaainet rag rebuild --entity-types --no-relations --graph-window`

## In progress

- **D6 graph rebuild** — full rebuild with optimal settings (100w chunks, window=1, no-relations, 3-type). Local build running (CPU Ollama, ~2.5h). Metro rebuild blocked on DNS/routing fix.
- **D6 accuracy** — current target: ≥ 80% on d6_eval_questions.json. Last validated: M24 in progress.

## Planned

- **Index seed embedding** — embed index-seeded entity names into the vector index (not just graph)
- **Graph scoring v2** — entity confidence weighting by extraction frequency
- **Multi-document knowledge bases** — cross-document entity deduplication
- **Obsidian vault ingestion** — `obsidian.rs` integration into CLI

## Research / future

- Graph neural network reranking over entity graph
- Cross-knowledge-base federated retrieval (query multiple VPK nodes)
- Streaming dream cycles with real-time token output
