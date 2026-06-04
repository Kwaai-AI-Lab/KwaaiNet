# Sub-project A: Document Structure-Aware Ingestion

## Context

Current RAG treats documents as flat chunk sequences. Structural metadata exists at the
section level (`section_name`, `skip_extraction`, `section_note` on `Chunk`/`ChunkMeta`)
but there is no semantic typing of sections, no depth hierarchy, and no retrieval policy
that exploits structure. For long-form documents (memoirs, books, academic papers) this
causes:

- TOC and index chunks polluting retrieval results
- No way to prefer narrative chapters over appendices for "who was X?" queries
- Entity extraction from back matter (bibliography, index) adding noise
- No signal to the LLM about where in the document structure retrieved chunks live

The goal is to treat documents as trees (front matter → chapters → sections → paragraphs)
and use that structure to improve both extraction quality and retrieval precision.

---

## What already exists (reuse)

| Asset | File | Role |
|---|---|---|
| `SectionDef` (pattern, skip, narrator_note, index_seeds) | `doc_schema.rs` | Extend with `section_type` |
| `DocSchema` (sections, metadata, schema_type) | `doc_schema.rs` | Add `retrieval_policy` |
| `Chunk.section_name`, `skip_extraction`, `section_note` | `chunker.rs` | Add `section_type`, `depth`, `section_path` |
| `ChunkMeta` with serde defaults | `meta_store.rs` | Add same three fields |
| Section state machine in `split_paragraph()` | `chunker.rs` | Extend to track depth + type |
| `auto_detect_schema()` heuristic | `doc_schema.rs` | Extend to detect structure |
| `d6_doc_schema.yaml` with skip/narrator patterns | `tests/kwaai-knowledge/` | Extend with section_type annotations |
| `retrieve_hybrid()`, `retrieve_graph_anchored()` | `retriever.rs` | Add section-type filter |

---

## Implementation — 3 phases, smallest first

### Phase 1 — Semantic section typing (highest ROI, ~1 day)

**What**: Add a `section_type` enum to `SectionDef` and propagate it to `Chunk`/`ChunkMeta`.
This costs nothing at retrieval time but unlocks Phases 2 and 3.

**`doc_schema.rs`**: Add enum and field:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SectionType {
    #[default]
    Main,          // default narrative body
    FrontMatter,   // title page, copyright, dedication
    TableOfContents,
    Preface,
    Introduction,
    Chapter,
    Appendix,
    Bibliography,
    Index,
    EndNotes,
    Footnote,      // inline footnote text (if extracted separately)
    Caption,       // figure/table/image caption
    Acknowledgements,
    EditorNote,
}
```

Add to `SectionDef`:
```rust
#[serde(default)]
pub section_type: SectionType,
```

**`chunker.rs`**: Add to `Chunk`:
```rust
pub section_type: SectionType,
```

**`meta_store.rs`**: Add to `ChunkMeta`:
```rust
#[serde(default)]
pub section_type: SectionType,
```

**`d6_doc_schema.yaml`**: Annotate existing sections:
```yaml
- pattern: "CONTENTS"
  skip: true
  section_type: table_of_contents
- pattern: "Editor's Note"
  section_type: editor_note
- pattern: "INDEX"
  skip: true
  index_seeds: true
  section_type: index
- pattern: "APPENDIX"
  skip: true
  section_type: appendix
- pattern: "ENDNOTES"
  skip: true
  section_type: end_notes
- pattern: "Acknowledgements"
  skip: true
  section_type: acknowledgements
```

**Backwards compat**: `#[serde(default)]` everywhere; existing serialized chunks
deserialize with `SectionType::Main`.

---

### Phase 2 — Section hierarchy depth (medium complexity, ~1 day)

**What**: Add `depth: u8` (0=document, 1=chapter, 2=section, 3=subsection) and
`section_path: Vec<String>` (breadcrumb) to `Chunk`/`ChunkMeta`.

**Why**: Enables "retrieve from chapter-level context" — knowing a chunk is inside
"Chapter 3 > Section 2" vs "Introduction" changes how it should be weighted.

**Approach**: Extend the heading-detection state machine in `split_paragraph()` with
depth inference:
- Single ALL-CAPS heading with no preceding text → depth 1 (chapter)
- Mixed-case heading ≤ 60 chars after a chapter heading → depth 2 (section)
- Fallback: inherit parent depth

`SectionDef` gains an optional `depth_hint: Option<u8>` for explicit override in YAML.

**ChunkMeta changes**:
```rust
#[serde(default)]
pub depth: u8,          // 0 = unstructured, 1 = chapter, 2 = section, 3 = subsection
#[serde(default)]
pub section_path: Vec<String>,  // ["Chapter 3: The Gools", "The Buitencingle House"]
```

---

### Phase 3 — Retrieval policies (biggest accuracy win, ~1 day)

**What**: Add a `retrieval_policy` block to `DocSchema` that defines per-query-type
section preferences. Extend `retrieve_hybrid()` to respect these.

**Schema addition** (`doc_schema.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetrievalPolicy {
    /// Section types to always exclude from retrieval results.
    pub exclude_types: Vec<SectionType>,
    /// Section types to prefer (boost score) for named-entity queries.
    pub prefer_for_entity_queries: Vec<SectionType>,
    /// Section types to prefer for source/citation queries.
    pub prefer_for_source_queries: Vec<SectionType>,
}
```

**`d6_doc_schema.yaml`** addition:
```yaml
retrieval_policy:
  exclude_types: [table_of_contents, index]
  prefer_for_entity_queries: [main, chapter, introduction]
  prefer_for_source_queries: [end_notes, footnote, appendix, bibliography]
  downweight_types: [caption]   # captions carry context but rarely answer questions directly
```

**Footnote and caption handling notes:**
- `footnote`: high-value for source queries ("where does the author cite X?") — include
  in `prefer_for_source_queries`. In D6 style these are endnotes; pattern-match on
  superscript markers or "ENDNOTES" section heading.
- `caption`: figure/table captions are often brief and descriptive rather than narrative.
  They should be retrievable but downweighted vs. main body text. Score multiplier 0.8×.
- Both types should participate in entity extraction (skip=false by default) — a caption
  naming a person or place is just as valid as body text.

**`retriever.rs`**: In `retrieve_hybrid()`, after the initial chunk pool is assembled:
1. Load policy from `GraphStore::doc_metadata()` (already persisted at ingest)
2. Filter out `exclude_types` chunks entirely
3. Boost score by `+0.05` for chunks in `prefer_for_entity_queries` when query contains
   a named entity (detected via GLiNER or name-token lookup)

**Retrieval score adjustment** (additive, not multiplicative, to preserve relative ordering):
```rust
if policy.exclude_types.contains(&chunk.section_type) {
    continue;  // drop from pool
}
if is_entity_query && policy.prefer_for_entity_queries.contains(&chunk.section_type) {
    chunk.score += 0.05;
}
```

---

## YAML schema additions for other document types

One `doc_schema.yaml` per document type — add to `tests/kwaai-knowledge/`:
```yaml
# d6_doc_schema.yaml (extend existing)
# For a future academic paper schema:
# paper_doc_schema.yaml
schema_type: "ScholarlyArticle"
sections:
  - pattern: "Abstract"
    section_type: front_matter
  - pattern: "Introduction"
    section_type: introduction
  - pattern: "Related Work"
    section_type: main
  - pattern: "References"
    section_type: bibliography
    skip: true
    index_seeds: true
  - pattern: "Appendix"
    section_type: appendix
  # Footnotes / captions if extracted as separate blocks:
  - pattern: "^\\d+\\."        # numbered footnote lines
    section_type: footnote
  - pattern: "Figure"
    section_type: caption
  - pattern: "Table"
    section_type: caption
```

---

## Verification

1. **Phase 1 smoke test**: Ingest D6 with updated schema, run
   `kwaainet rag chunks list --kb D6 | grep section_type` — verify CONTENTS chunks
   show `table_of_contents`, main body shows `main`.

2. **Phase 3 eval**: Run D6 40-question eval before and after adding `exclude_types:
   [table_of_contents, index]`. Expect marginal precision improvement on broad queries
   (q13, q14, q34) where TOC chunks were polluting the pool.

3. **Regression**: All existing `d6_doc_schema.yaml` behaviour (skip, narrator_note,
   index_seeds) must be unchanged — `section_type` is purely additive.

---

## Sequencing note

Phase 1 is a prerequisite for Phase 3. Phase 2 (depth) is independently useful but
not required for retrieval policies. Ship Phase 1 + 3 as one unit; Phase 2 can follow.
The eval running right now provides the baseline to measure Phase 3's impact against.
