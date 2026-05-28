# kwaai-rag crate

This crate implements the full RAG pipeline: ingestion, chunking, embedding, entity/graph extraction,
hybrid BM25+vector retrieval, dream (iterative self-RAG), and eval harness.

**Full project context:** `projects/kwaai-knowledge/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/ingestion.rs` | `GraphIngestConfig`, chunking, embedding, entity extraction |
| `src/graph.rs` | Entity/relation graph store, LLM extraction calls |
| `src/retriever.rs` | Hybrid retrieval: BM25 + vector + graph, entity injection |
| `src/doc_schema.rs` | `DocSchema`, `auto_detect_schema`, `parse_index_seeds` |
| `src/dream.rs` | Iterative self-RAG loop |
| `src/embedder.rs` | `EmbedClient` |
| `src/eval_retrieve.rs` | Eval harness |
| `src/family.rs` | Family tree YAML seed injection |

## Build

```bash
cargo build -p kwaai-rag
cargo test -p kwaai-rag
```
