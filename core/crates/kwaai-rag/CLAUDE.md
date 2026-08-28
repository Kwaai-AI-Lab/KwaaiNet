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
| `src/ontology.rs` | Per-KB knowledge schema: entity/relation types, axioms, markers, triggers |
| `src/axiom_extract.rs` | Phases 1–3 of extraction: deterministic candidate typing, no LLM |
| `src/relation_extract.rs` | Relation candidate classification, trigger tables, axiom validation |
| `src/mentions.rs` | Per-chunk mention resolution (pronouns, aliases, definite descriptions) |
| `src/scorer.rs` | Graph completeness scoring — the three pillars |
| `src/bm25.rs` | Tantivy BM25 index for the keyword half of hybrid retrieval |
| `src/sequence.rs` | Timeline events and cross-entity interactions |

Thirty-odd modules in total; these are the ones worth reading first. Vocabulary
is defined once in [`projects/kwaai-knowledge/GLOSSARY.md`](../../../projects/kwaai-knowledge/GLOSSARY.md).

## Build

```bash
cargo build -p kwaai-rag
cargo test -p kwaai-rag
```
