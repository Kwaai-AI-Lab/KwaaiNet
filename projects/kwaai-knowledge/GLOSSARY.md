# DreamRAG — taxonomy of terms

For newcomers and contributors to `kwaai-rag`. Grouped by where a term appears
in the pipeline rather than alphabetically, so it can be read straight through
as the shape of the system. Every term is one actually used in the code or the
plans; the source file is named where it helps.

Pipeline in one line:

> **ingest** → chunk → embed → **extract** entities and relations → **dream**
> (refine the graph) → **retrieve** → answer → **evaluate**

---

## 1. Storage and scope

**Knowledge base (KB)** — one corpus with its own chunk store, vector index,
knowledge graph and configuration. Named (`D6`, `Legal`, `Climate`), registered
under `rag_kbs` in `~/.kwaainet/config.yaml`. Sixteen exist. Everything else in
this glossary is scoped to a KB.

**Tenant / `tenant_id`** — the UUID that names a KB's storage. Files are
`<tenant>.db` (chunks) and `graph-<tenant>.db` (graph). Two KBs may share a
tenant when one is a copy of another.

**Chunk store / `MetaStore`** — SQLite table of `ChunkMeta`: the text, its
document, its position, and which section it came from. The corpus as ingested.

**Graph store / `GraphStore`** — SQLite file holding entities, relations, the
chunk↔entity index and a `metadata` table. Adjacency is rebuilt in memory on
open. *(`graph.rs`)*

**Eve node** — a peer on the KwaaiNet network that hosts vector storage for
someone else's KB. `rag connect-eve` outsources storage to one.

**VPK** — the encrypted multi-tenant storage fabric Eve nodes run. Relevant here
only as where a KB's vectors may live.

---

## 2. Ingestion

**Ingest** — read a document, split it, embed the pieces, store them. Does not
by itself build a graph.

**Chunk** — a unit of source text, embedded and retrievable. The atom of
retrieval.

**Chunk strategy** — how text is split. `Character` (sliding window over Unicode
scalars, the original) or `Paragraph` (paragraph → sentence → character cascade,
semantic). *(`chunker.rs`)*

**Context window / `--graph-window`** — how many neighbouring chunks are shown
to the extractor alongside the one being processed. 1 is the measured optimum
(+7pp recall); 2 costs more for nothing.

**Document schema / `DocSchema`** — per-document structure: the title, the
narrator, and a list of sections with patterns. Lets ingestion skip an index or
a bibliography, and stops a book's title being extracted as a place.
*(`doc_schema.rs`)*

**Section type** — the semantic kind of a document region: `Chapter`, `Preface`,
`Index`, `Appendix`, `EndNotes`, `EditorNote`, `Caption`. Context windows may not
span a boundary between zones.

**Corpus hygiene** — the measured share of a KB's chunks that are usable prose
rather than markup, PDF extraction failures, bibliography or boilerplate. Nine of
sixteen KBs are under 80%; one is at 2%. Bounds what any extraction can achieve.
*(`tests/kwaai-knowledge/corpus_hygiene.py`)*

---

## 3. The knowledge graph

**Entity / `EntityNode`** — a named thing: a person, a place, an organisation.
Carries a type, a description, aliases, an embedding, evidence chunk IDs and
structured fields.

**Entity ID** — `sha256(name.lower() + "::" + entity_type)`. **The type is part
of the identity**, so renaming a type re-IDs every node of that type and orphans
its relations. This is why ontology edits on a mature graph are a migration.

**Relation** — a directed, typed edge between two entities, with a strength, a
confidence, evidence chunks, and the method that produced it.

**Entity type** — what an entity *is*: `Person`, `Place`, `Organization`. The
global default list has 17; a KB's ontology may declare its own.

**Relation type / predicate** — what an edge *asserts*: `parent_of`, `works_at`,
`measured_by`. 35 in the global list, 14 of them kinship.

**Escape-hatch predicate** — `associated_with` or `related_to`: an edge that says
two things are connected without saying how. Honest for genuinely untyped
association, a bug when it is the default.

**Alias** — another name for the same entity ("J.M.H. Gool" / "JMH Gool"). Also,
separately, another name for a *predicate* (`lived_in` → `lived_at`), so a
model's conventional wording resolves to the corpus's declared one.

**Ghost entity** — an entity with no relations. Pruning them was measured to cost
7pp of accuracy; they carry connectivity. Do not prune.

**Zombie entity** — an entity the dream loop judges unsupported by evidence and
removes. Distinct from a ghost.

---

## 4. Extraction

Four phases. Phases 1–3 are deterministic and free; phase 4 costs an LLM call.

**Candidate** — a proper-noun span the pre-screener found, before anything has
decided whether it is an entity.

**Axiomatic classification** — typing a candidate by deterministic rule: a
lexical marker, a hit in the existing graph, or a GLiNER hint. No LLM.
*(`axiom_extract.rs`)*

**Marker** — a lexical rule that types a candidate. Four kinds, most to least
specific: `exact` (closed vocabulary — "Urdu"), `last` (final word — "… Street"),
`prefix` (honorific — "Haji …"), `any` (substring).

**Trigger** — a phrase that indicates a relation ("son of" → `child_of`).
Longest match wins, so "half-brother of" beats "brother of". A trigger may be
*reversed*, meaning the object precedes the subject ("X founded by Y").

**GLiNER** — an external NER model called before extraction to find person names
with higher recall than the regex pre-screener.

**Composite confidence** — `type_confidence × mention_confidence`. Decides which
of three tiers a candidate lands in.

**Confidence split** — commit (write directly), verify (send to a narrow LLM
confirm/reject/retype pass), or drop. Relations get a hard low-end cutoff because
candidate triples scale combinatorially with entities per window.

**Demotion** — an axiom setting a candidate's confidence to zero rather than
deleting it, so it still appears in metrics as rejected rather than vanishing.

**Coreference / mention resolution** — mapping pronouns and definite descriptions
("he", "my grandfather", "the district") onto entities. Resolved once per chunk
and stored, rather than re-derived by each consumer. *(`mentions.rs`)*

**Entity-centric (EC) vs chunk-centric (CC)** — CC walks chunks and extracts what
is in each. EC picks an entity and gathers all its chunks into one focused call.

---

## 5. Ontology

The per-KB knowledge schema. New in this branch; see
`plans/PerKBOntology-plan.md`.

**Ontology** — the entity types, relation types and axioms that govern one
corpus's graph. Loaded from YAML with `rag graph schema load`, stored in the
graph's metadata. A KB without one falls back to the global constants.

**Domain / range** — the entity types a predicate accepts as subject and object.
`measured_by(Phenomenon, Instrument)` lets extraction reject
`measured_by(Person, Poem)` without an LLM call.

**Axiom** — a declarative constraint: `contradiction` (two predicates that cannot
both hold), `functional` (at most one object per subject), `non_functional`,
`acyclic`, `transitive`, `symmetric`.

**Inverse** — the predicate implied in the other direction. Storing
`parent_of(A,B)` also stores `child_of(B,A)`.

**Fallback predicate** — what an undeclared relation becomes. `associated_with`
for narrative, which genuinely contains untyped association; `null` for law,
standards and science, where a vague edge is a bug and the edge is dropped.

**Irreducible** — an entity type whose content must survive compression verbatim:
exact quotations, numbers, statutes, requirement wording.

**Stream** — a document kind within one KB with its own type subset and ingest
policy. A research corpus is body text plus a bibliography plus front matter, and
they should not share a vocabulary.

**Module / `extends`** — a reusable built-in ontology fragment. `genealogy`
supplies the 14 kinship predicates with their inverses, symmetry and Person
domain/range.

**Evidence share** — the measured percentage of a corpus's chunks containing a
type's vocabulary. Recorded per type so a schema's claims are auditable rather
than asserted.

---

## 6. Dream

**Dream cycle** — an autonomous pass that improves an existing graph without new
source text: score → reclassify unknown types → enrich thin descriptions →
fill missing relations → merge near-duplicates → prune zombies → re-score.
*(`dream.rs`)*

**Completeness score** — three pillars per entity, averaged: **Type** (mapped to
a specific schema.org type?), **Summary** (is the description substantive?),
**Relationship** (are the expected relations present?). *(`scorer.rs`)*
*Caveat: the compiled expectations assume kinship, which makes the score a
memoir-conformance metric on other corpora.*

**Dream task kind** — the completion template chosen by an entity's type:
`Biography`, `Geography`, `OrgProfile`, `EventProfile`, `ConceptDef`,
`WorkProfile`, `General`, plus `FullSummary` (map-reduce over every chunk an
entity appears in).

**Enrichment** — building an entity's description and structured fields from all
its evidence chunks in one call. *(`enrich.rs`)*

**Consolidation gate (proposed)** — the rule that a chunk becomes evictable only
once its claims are in the graph and corroborated. Not implemented; see the
DreamRAG framing doc.

---

## 7. Retrieval

**Hybrid retrieval** — dense vector search plus BM25 keyword search. BM25 catches
acronyms and exact names that embeddings handle poorly on narrow corpora.
*(`bm25.rs`, Tantivy-backed)*

**Top-K** — how many chunks are returned for a query.

**HyDE** — Hypothetical Document Embeddings. Embed an LLM-written hypothetical
*answer* rather than the question, because document-to-document similarity is
tighter than question-to-document. *(`hyde.rs`)*

**Reranker** — one LLM call that reorders a wide candidate pool (`top_k × 4`) by
relevance, then truncates. *(`reranker.rs`)*

**Iterative mode** — multi-round retrieval that identifies gaps in what it has
and goes back for more. The default. `smart` uses it selectively.

**Query intent** — what a question is asking for: `FamilyRelation`,
`EntityDescription`, `OrgMembership`, `TemporalEvent`, `Unknown`. Determined by
rule, LLM, or hybrid. *(`query_understand.rs`)*

**Graph mode** — how graph facts enter the answer context. `Inject` (as a
synthetic chunk), `Prepend` (a structured facts block above the chunks), or
`Replace` (graph facts only, zero retrieval noise — for resolved family queries).

**Abstention** — answering "this knowledge base has no notion of that" instead of
confabulating. Identified as missing; an ontology makes it possible by declaring
what is out of scope.

---

## 8. Evaluation

**Eval set** — a KB's question/answer pairs. D6 has 40.

**Recall** — token overlap between the generated answer and the expected one.
The current metric. Saturates near 90% and cannot distinguish a good answer from
a verbose one.

**Graph score** — the averaged completeness score over a whole graph. **Not
comparable across KBs with different ontologies** — see §6.

**Escape-hatch rate** — share of edges that are `associated_with`/`related_to`.
Only meaningful when the vocabulary is closed: an arm scoring 5.2% while carrying
98 edges on invented one-off predicates was in fact worse than its control.

**Uninformative rate** — escape-hatch edges *plus* edges on predicates no
ontology declares. The corrected form of the metric above.

**Relation density** — relations per entity. Ranges 300× across the sixteen KBs
(D6 0.225, DeepSea 0.000).

**Arm** — one side of a comparison: control (no ontology) versus treatment.

**Instrument-independent metric** — one read from the graph rather than from an
eval run. Preferred while the eval instrument is weak.

**Curriculum eval (proposed)** — tiered questions (recall → paraphrase →
multi-hop → synthesis → inference) generated from the corpus, replacing the fixed
set. Not built.

---

## 9. Reading order for a newcomer

You are already in step 1 — this page is the vocabulary the rest assumes.

1. **This glossary**, skimmed. You do not need to retain it; you need to know
   which words are load-bearing.
2. **README → "RAG Knowledge Base"** — what the pipeline is and how to run it
   locally, no network required. The fastest way to see the thing work.
3. **`core/crates/kwaai-rag/src/graph.rs`**, module docs and the type
   definitions at the top — entities, relations, and the extraction prompt.
   The centre of gravity of the crate.
4. **`plans/PerKBOntology-plan.md` §1** — an audit of what governs extraction
   today. The clearest picture of the system's current seams.
5. **`plans/DreamRAG-Ontology-Eval-Compression.md`** — where the project is
   going: ontology, curriculum evaluation, compression.
6. **`plans/OntologySession-assessment.md`** — what has actually been measured,
   and what the measurements cannot yet support. Read it for the failures; they
   are more instructive than the results.

**Not** `CLAUDE.md`. Those files are configuration for Claude Code — build
commands, current operational settings, and per-crate do-not lists. They are
useful once you are working *in* the crate, and they assume the vocabulary this
page defines, so they read as circular if you start there.
