# kwaai-knowledge — Claude Code Instructions

## Project scope

kwaai-knowledge owns the RAG (Retrieval-Augmented Generation) pipeline: document ingestion,
sentence-aligned chunking, embedding, entity/graph extraction, hybrid BM25+vector retrieval,
knowledge graph, dream (iterative self-RAG), eval harness, and document schema (schema.org).
It does **not** own storage encryption (kwaai-storage) or inference serving (kwaai-compute).

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-rag` | `core/crates/kwaai-rag/` | Full RAG pipeline |

CLI entry points in `core/crates/kwaai-cli/src/`:
- `rag_cmd.rs` — all `kwaainet rag` subcommands (ingest, query, graph build/score/seed, rebuild, dream, eval)
- `rag_api.rs` — REST API for external RAG access

## Active knowledge base: D6

**D6** = "District Six — Lest We Forget" by Yousuf (Joe) Rassool.
This is the primary eval document. Accuracy target: 80–90% on `d6_eval_questions.json`.

**D6 rebuild command** (optimal settings, GPU via p2p relay):
```bash
kwaainet rag rebuild "docs/LEST WE FORGET -rev25.pdf" --kb D6 \
  --model llama3.1:8b \
  --inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd" \
  --workers 4 \
  --entity-types "Person,Place,Organization,Legislation,Publication" \
  --no-relations \
  --graph-window 1 \
  --timeline \
  --seed-file tests/kwaai-knowledge/d6_family_tree.yaml \
  --doc-schema tests/kwaai-knowledge/d6_doc_schema.yaml \
  --yes
```

**D6 eval command**:
```bash
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json
```

**Graph seed** (inject family tree relations):
```bash
kwaainet rag graph seed --kb D6 --file tests/kwaai-knowledge/d6_family_tree.yaml
```

## Optimal entity extraction settings (Phase 3 confirmed)

| Setting | Value | Reason |
|---------|-------|--------|
| Chunk size | 100w sentence-aligned | Best balance recall/precision |
| Context window | 1 (adjacent chunk) | +7pp recall |
| Entity cap | 25 (3-type) | Prevents hallucination flooding |
| Entity types | Person, Place, Organization, Legislation, Publication | Phase 3 — KB schema guides 8B model |
| Relations | Disabled by default | 0–17% precision without lexical trigger pre-filter |
| Lexical trigger | Auto (Phase 4) | Relations only extracted from chunks with kinship/membership keywords |
| Timeline | Enabled | Dated events extracted (second LLM call per chunk) |

**Per-type recall** (Phase 2 experiments):
- Person: ~0.96–1.00 ✓
- Place: ~0.75–1.00 (sensitive to chunk size)
- Organization: 0.00–0.67 (weakest — D6 is a memoir, few orgs)
- Legislation: TBD (Phase 3 — KB schema injected, r108 pending)
- Publication: TBD (Phase 3 — KB schema injected, r108 pending)

**Relations**: 8B models achieve 0–17% precision on unconstrained extraction.
Phase 4 adds a lexical trigger pre-filter (kinship/membership keywords gate per chunk) and
a cross-chunk support filter (≥2 chunk appearances required). Family tree YAML seed remains
the only trustworthy source for seeded relations.

## Document schema (d6_doc_schema.yaml)

Located at `tests/kwaai-knowledge/d6_doc_schema.yaml`. Sections:
- `CONTENTS`, `Acknowledgements`, `APPENDIX`, `ENDNOTES` → `skip: true`
- `Editor's Note` → `narrator_note` override (editor ≠ Yousuf Rassool)
- `INDEX` → `skip: true`, `index_seeds: true` (parse index entries as entity name seeds)

Pass to ingest: `kwaainet rag ingest --doc-schema tests/kwaai-knowledge/d6_doc_schema.yaml`

## Infrastructure

**P2P relay inference** (GPU, preferred — no DNS/IP needed):

Inference URLs use the `p2p://PEER_ID` or `mux://PEER_ID` scheme. The running kwaainet daemon
resolves them via the p2p relay to the remote machine's Ollama. `mux://` is preferred for
concurrent requests (saturates OLLAMA_NUM_PARALLEL); `p2p://` works for sequential.

| Machine | Scheme | Peer ID | GPU |
|---------|--------|---------|-----|
| metro-linux | `p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` | A6000 48GB |
| metro-win   | `p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE` | A5000 |
| jerome      | `p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd` | |

Example multi-machine build (all three GPUs):
```bash
kwaainet rag graph build --kb D6 \
  --model llama3.1:8b \
  --inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd" \
  --workers 4 --entity-types Person,Place,Organization --no-relations --reset-graph --graph-window 1
```

**Local Ollama** (fallback, CPU only): `http://localhost:11434`
- Very slow on CPU — 1152 chunks takes ~12 hours (43s/call). Use p2p machines instead.

## Build & test

```bash
# Build
cargo build -p kwaai-rag

# Full pipeline test
kwaainet rag ingest --kb D6 --file "docs/LEST WE FORGET -rev25.pdf" \
  --doc-schema tests/kwaai-knowledge/d6_doc_schema.yaml
kwaainet rag graph build --kb D6 --model llama3.1:8b --no-relations --graph-window 1
kwaainet rag graph score --kb D6
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json

# Dream (iterative self-RAG)
kwaainet rag dream --kb D6 --query "Tell me about Walied Rassool"
```

## Key source files

| File | Description |
|------|-------------|
| `kwaai-rag/src/ingestion.rs` | Chunking, embedding, entity extraction, `GraphIngestConfig` |
| `kwaai-rag/src/graph.rs` | Entity/relation graph store, extraction LLM calls |
| `kwaai-rag/src/retriever.rs` | Hybrid retrieval: BM25 + vector + graph, entity injection |
| `kwaai-rag/src/doc_schema.rs` | `DocSchema`, `auto_detect_schema`, `parse_index_seeds` |
| `kwaai-rag/src/dream.rs` | Iterative self-RAG loop |
| `kwaai-rag/src/embedder.rs` | `EmbedClient`, embedding models |
| `kwaai-rag/src/eval_retrieve.rs` | Eval harness |
| `kwaai-cli/src/rag_cmd.rs` | All `kwaainet rag` command handlers |

## Do not

- Do not re-enable relation extraction for 8B models — precision is too low; use seed YAML
- Do not change chunk size away from 100w without re-running Phase 2 experiments
- Do not use `cargo build -p kwaai-core` to test CLI changes — the binary is `-p kwaainet`
- Do not skip `codesign -s - --force ~/.cargo/bin/kwaainet` after installing on macOS 26+

## Full project context

`projects/kwaai-knowledge/` — requirements, design docs, roadmap, TODO
`tests/kwaai-knowledge/` — eval questions, family tree, doc schema, experiment scripts, results
