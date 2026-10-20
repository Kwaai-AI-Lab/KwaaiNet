# Ontology selection from document metadata

Status: **design agreed** (2026-09-27); §7 decisions 1–3 settled, 4 open. No code written.

The goal: at ingest, work out what kind of document each file is, choose the entity and relation
vocabulary that fits it, grounded in schema.org, and have every pipeline stage use that vocabulary.
A climate paper should never be asked "who is your uncle?".

This completes [`PerKBOntology-plan.md`](PerKBOntology-plan.md): its phase 2 wiring and its
never-built "arm D — selected". It does not replace that plan.

**Why now.** DreamRAG Eval v2 ran with no ontology loaded in any graph (deviations log,
2026-09-27). Every corpus was offered the compiled relation list: 44 predicates, 14 of them
kinship. Eval v3 restarts once this lands.

---

## 1. What exists today (verified against the code, 2026-09-27)

Symbols are named rather than line numbers, so this survives a rebase.

### 1.1 Document typing at ingest: exists, and too weak to choose anything

- **`doc_schema::auto_detect_schema`** scans the first 4,000 characters for an ISBN, publisher,
  copyright year and a "by" author.
  - The only `schema_type` it can set is `Book`, and only when it finds an ISBN.
  - What it finds is saved as `doc_metadata` (`GraphStore::set_doc_metadata`).
  - That metadata is only used by `DocSchema::context_line`, a context line in prompts. It never
    affects which types get extracted.
- **Results on the eval2 corpora:**
  - Manhattan got `{year: 1998}`.
  - Legal got nothing.
  - D6's rich metadata came from the hand-written `d6_doc_schema.yaml`.
- **The PDFs' built-in metadata is not read at all** (`document.rs` only extracts text). It would
  add little anyway. In a sample of the corpus files:
  - the author fields hold the producing software (`Mozilla/5.0 …`, `LaTeX with hyperref`,
    `CorelDRAW`);
  - titles are missing, or read like `rogrisk.cdr`;
  - keywords are empty.

  **Metadata has to be derived from the text, not read from the file.**
- **`scorer::schema_type_for`** maps extracted entity types to schema.org types. It labels
  entities after extraction; it does not choose what to extract.

### 1.2 Consuming a stored ontology: partly wired, with outright bugs

A stored ontology (`GraphStore::set_ontology`, metadata key `kb_ontology`) reaches the following
stages:

| Stage | Uses the ontology? | Evidence |
|---|---|---|
| Entity extraction vocabulary (`graph build`) | yes, but `--entity-types` **replaces** it | `extract_and_store_entities_pub` per-chunk worker: CLI types, then the ontology, then `ENTITY_TYPES` |
| Relation writes | yes: coercion, domain/range, inverse and symmetric edges | `GraphStore::upsert_relation_with_confidence` → `ont.coerce_relation` |
| Phase-4 domain/range, contradictions and verify menu | yes | `validate_relation_axioms_with_ontology`, `in_scope_relation_types` |
| Phase-4 lexical triggers | **no**: compiled tables only | `classify_relation_candidates` → `find_best_trigger`. `find_trigger_with_ontology` is only called from tests |
| Legacy relation gate | **no** | `lexical_relation_trigger(text)` |
| `dream run` prompts and filters | **no**: always `RELATION_TYPES` | `dream::complete_entity`, and the `dream_tasks.rs` validation |
| `sanitize_relations` (every dream cycle) | **no**: the compiled `FAMILIAL_RELS` rules and the gender heuristic always run | |
| `graph score` | **no**: compiled `expected_relation_groups`. The ontology version is only called from tests | `scorer::score_entity` |
| Retrieval / query understanding | **no**: compiled `OF_PATTERNS` (kinship) | `query_understand::understand_query_rule` |
| Coreference | **no**: compiled `DEFINITE_DESCRIPTIONS` ("my grandfather") | `ner::resolve_definite_descriptions` |

**Two bugs make a loaded ontology unreliable wherever it is used:**

1. **`extends:` only resolves `genealogy`.** In `Ontology::resolve_extends`,
   `Some("genealogy") => genealogy_module(), _ => return`. So `narrative-history`, `scholarly`
   and `software-docs` are silently ignored, and Manhattan, DeepSea and Climate lose their whole
   module.

   The test `shipped_ontologies_have_no_dangling_references` splices the module's types by hand,
   which hides the gap.
2. **`--reset-graph` deletes the graph file** (the `cmd_graph_build` reset branch,
   `std::fs::remove_file(&graph_path)`). That destroys `kb_ontology`, `kb_entity_schemas` and
   `doc_metadata`. `rag graph clear` without `--all` keeps them. The eval drivers pass
   `--reset-graph`.

**Memoir text is in the extraction prompt for every KB.** In the prompt builders in
`graph::extract_from_text`:

- a roster of comic-strip names from one memoir (a code comment claimed it had been removed);
- that memoir's family nicknames;
- its regional ethnic vocabulary;
- the `spouse_of`/`child_of` rules.

This breaks the standing rule that domain knowledge lives in per-KB YAML, never in compiled Rust.

### 1.3 The ontology library

| File | State |
|---|---|
| `D6.yaml` | derived from D6's own text; extends `genealogy` (the one path that works) |
| `Legal.yaml` | derived; standalone; the README says "apply first" |
| `Climate.yaml`, `DeepSea.yaml`, `DreamMem.yaml` and 5 others | header says `BLOCKED ON INGESTION — do not apply yet` |
| `Manhattan.yaml`, `WarPeace.yaml`, `CountryHistory.yaml` | thin extensions of a genre module written from priors; not derived from their own text |
| `_module_*.yaml` | three genre modules (narrative-history, scholarly, software-docs); unused by the real loader |

**Manhattan re-derived, 2026-09-27.** `Manhattan.yaml` v2 is derived from its own text; it
replaces the genre extension. The derivation measured:

- **Kinship: 2.2% of prose chunks**, and only 2 real facts. So `genealogy` was rejected.
- **The KB itself is 54% non-prose.** Of 773 chunks:
  - 225 are binary junk from one mis-ingested file (`ingest_XXXXX.html`);
  - 104 are CSS/JavaScript: the Einstein–Szilard page was ingested with its stylesheet, and
    103 of its 158 chunks are code;
  - 95 are empty fragments or navigation chrome.
- **Two eval questions ask about documents the KB lacks:** the Smyth Report (q03) and the
  Handy–Spaatz order (q08).

These are ingestion defects, and they must be fixed before Manhattan is used in Eval v3. The
hygiene audit's 86.2% figure missed all of them.

The 2026-08-25 session also recorded (`OntologySession-assessment.md`) that three of its own
ontology claims were wrong. Each time, a property of a corpus was asserted without being
measured. **This plan measures every selection decision; none is asserted.**

---

## 2. Design

### 2.1 Three layers

| Layer | Source | Scope |
|---|---|---|
| **Backbone** | schema.org core: a pinned release of `schemaorg-current-https.jsonld`, vendored | types every document shares: Person, Organization, Place, Event, CreativeWork and its subtypes, Legislation; properties with `domainIncludes`/`rangeIncludes` become relation types with domain/range |
| **Document-type profile** | one per schema.org document type (`ScholarlyArticle`, `Legislation`, `Book`, `Report`, …) | which backbone types and properties matter for this kind of document, and which **streams** it has (body, bibliography, front matter) |
| **Domain extension** | per subject, from the existing ratified YAML library, or induced (`ontology_induce.py`) and then ratified | the content schema.org does not have: Forcing, TippingElement, Holding, Statute, Control… |

**Why three layers, not "select a schema.org type".** schema.org describes the *container* well:
a `ScholarlyArticle` has an author, citations, a publisher and a funder. It describes the
*content* of a climate paper barely at all.

Choosing by container type alone is how Climate's earlier graph came to model its bibliography
(Publication 1,163 vs Organization 655). The profile sends container properties to the
front-matter and bibliography streams, and the body stream gets the domain extension.

### 2.2 Choosing per document

For each document at ingest, in order of trust:

1. **Explicit:** a `--doc-schema` YAML with `schema_type` and `about`. It always wins; D6 stays
   exactly as it is.
2. **Identifiers in the text:**
   - an ISBN means `Book`;
   - a DOI means `ScholarlyArticle`, with the DOI kept as evidence;
   - an enacting clause or Public Law number means `Legislation`;
   - a reporter citation (`71 U.S. 2`) means a court opinion. *Open: check whether core
     schema.org has a type for this. If not, the profile is named locally and mapped to the
     nearest type.*
3. **Classifier:** one local-LLM call on the first ~2 pages. It must choose from a closed list of
   profile types, return `about` as a few keywords, and quote the passage that justifies its
   answer.
   - Local only, so a private document never leaves the machine.
   - Output is JSON, validated against the closed list.
   - Low confidence gets recorded as such, not guessed.

The result is saved per document, with `evidence` and `method` recorded. `rag schema show`
prints it, and `rag schema set-doc` can override it.

**Choosing the domain extension.** Each candidate extension gets a *coverage* score on a sample
of that document's body chunks: the share of proper-noun and noun-phrase candidates whose
vocabulary the extension's types can hold. This is NCBO's coverage criterion, as recommended in
`PerKBOntology-plan.md` §8.2. The highest scorer is chosen if it clears a threshold; otherwise
there is no extension, and the reason is recorded. The measured scores are stored next to the
choice.

### 2.2a The test set is a source and a check for every derivation

A KB's questions say what readers ask of it. Its gold nuggets say what a correct answer must
assert. A derivation therefore uses both:

1. **As a source,** alongside chunk samples and lexical probes. Manhattan's questions made
   `Document` the central type: every question asks what a document argued, recommended or
   was addressed to.
2. **As a check:** each gold nugget must be expressible as the ontology's entities and
   predicates. For Manhattan, this check added `Quotation` (about half the nuggets quote the
   source), `Project`, `established`, `reports_to`, `said`, and `Document.sourceType`.

This applies only to ontologies derived for evaluation corpora. An ontology chosen
automatically at ingest (§2.2) has no test set, which is why the pilot (§4.2) measures what
it extracts.

**The risk is overfitting.** An ontology tuned to its test questions could inflate that
corpus's eval score.

- The mitigation: every type still needs chunk-level evidence, and the question or nugget it
  serves is recorded next to it, so a reviewer can tell which types exist only for the test
  set. In Manhattan, those are `reports_to` (1.4%) and `Committee` (8.1%).
- Eval v3 reports this per corpus.

### 2.3 Per-document vocabularies (decided: per-document first)

Each document gets its **own resolved ontology**: backbone, plus its profile, plus its chosen
extension. The KB stores the set, keyed by `doc_name`, together with a **KB union** computed from
it. The union is never used as a prompt vocabulary; it only answers "may this edge exist anywhere
in this KB?".

The links needed already exist: every stored chunk carries `doc_name` (`meta_store::ChunkMeta`),
and every entity carries `first_chunk_id` and `evidence` chunk ids (`graph::GraphNode`).

| Consumer | Vocabulary it uses |
|---|---|
| Entity and relation extraction for a chunk | the chunk's document's ontology (looked up through `ChunkMeta.doc_name`) |
| Chunks of different documents inside one context window (`--graph-window 1`) | must not happen. *To verify in phase 4:* windows respect `SectionType::same_window_zone`, but a document boundary has not been checked. If one can be crossed, phase 4 makes the document boundary a hard stop |
| `upsert_relation_with_confidence` (coercion, domain/range) | the ontology of the document the evidence chunk comes from; the KB union when a write has no chunk (seed files, merges) |
| Dream completion of an entity | the union of the ontologies of the documents in its evidence chunks. An entity in one climate paper and one statute gets both, and nothing else |
| `sanitize_relations` kinship rules | only for entities whose documents include `genealogy` |
| Scorer's expected relations | per entity, from the same per-entity union as dream |
| Query understanding and abstention | the KB union: a question is out of scope only when no document supports it |

**Storage:** a new metadata key `kb_doc_ontologies` (a JSON map from `doc_name` to an ontology
id, the resolved ontology, typing evidence and coverage scores), next to `kb_ontology`, which
holds the union.

- A KB with only `kb_ontology` behaves as today: every document gets that one ontology. So D6,
  a single document with a hand-written ontology, is unchanged.
- Adding a document to a KB recomputes the union. It never retypes existing documents unless
  asked to.

**Prompt size:** each vocabulary is capped for the 8B extractor and ranked by coverage. Because
it is per document, the cap applies to one document's vocabulary, not the whole KB's. The
entity-cap rule depends on the type count, so it is measured in the pilot.

### 2.4 Every stage uses the result

Fix the §1.2 gaps so that one stored ontology drives all of these:

- entity extraction;
- relation extraction, both the legacy path and Phase 4 (including triggers);
- dream prompts, filters and task selection (via `schema_type`);
- `sanitize_relations`: the kinship rules apply only when the ontology includes `genealogy`;
- the scorer's expected relations;
- query understanding's relation patterns;
- abstention (`PerKBOntology-plan.md` §3.9).

The memoir text in the extraction prompt moves into `D6.yaml`. Compiled tables remain only as
the fallback for a KB with no ontology, which keeps §2.3 of the parent plan.

---

## 3. Phases

Each phase ships with regression tests for the bugs it fixes (standing rule).

| Phase | Content | Gate |
|---|---|---|
| **0 — loader correctness** | Make `extends:` load the YAML modules: embedded with `include_str!`, recursive, cycles detected, an unknown module an **error**. Keep ontology and metadata through `--reset-graph`. Make `--entity-types` an intersection with the ontology or a documented override, and warn on use. Fix the hiding test. | `cargo test`; each `ontologies/*.yaml` loads to the type counts expected by hand; reset-graph test |
| **1 — every stage uses the ontology** | §2.4: dream, sanitize, scorer, Phase-4 triggers, query patterns, coreference descriptions. Move the memoir prompt text into `D6.yaml` (done). **Seed extraction candidates from the ontology:** `ner::extract_proper_noun_candidates` proposes only capitalised spans, so types whose members are lowercase (Material, Policy, Quotation; Climate's Phenomenon and Quantity) never get a candidate. Add the ontology's `markers.exact` terms found in the chunk, and quoted spans for irreducible types. | D6 rebuild on the new code: graph and eval within noise of the current 90.4%; a non-genealogy KB's prompt contains no kinship predicate (asserted in a test) |
| **2 — document typing** | §2.2 steps 1–3; stored per document; `schema show` / `schema set-doc` | typing accuracy ≥ 90% on the labelled set (§4.1) |
| **3 — backbone and profiles** | vendored schema.org release; generator from JSON-LD to backbone YAML (reproducible, not hand-written); profiles for the document types found in §4.1 | generated backbone round-trips; profile coverage reported per corpus |
| **4 — extension selection and per-document storage** | coverage scoring; `kb_doc_ontologies` + union (§2.3); the per-chunk lookup in extraction; per-entity unions in dream, the scorer and sanitize | selection agrees with a human choice on the eval corpora; a mixed-KB test: a statute chunk's prompt carries no scholarly types, and a paper chunk's carries no legislation types |
| **5 — pilot** (§4.2) | small extraction A/B before any full build | the pilot's criteria are met, or we stop and revise |

Phases 0 and 1 are needed whatever the selection method, and they are also the part that makes
Eval v2's defect impossible to repeat.

---

## 3a. Status, 2026-09-28: per-document selection v1 works (`52dc39db`)

**Built:**
- `ontology_select.rs`:
  - a library loaded from a directory of YAMLs;
  - coverage scoring over distinctive vocabulary: examples, markers, rare type names and
    multi-word triggers, IDF-weighted, with single words at half weight;
  - document types from identifiers only;
  - narrowing to the ontologies that declare the detected type (`document_types:`);
  - `scope: corpus` ontologies eligible only in their own KB.
- `rag graph schema select`: `--dry-run`, `--explain`, `--own-corpus`.
- Per-document storage (`kb_doc_ontologies`), with the union as `kb_ontology`.
- Extraction uses each chunk's document ontology, for both the vocabulary and the type
  guidance.

**Measured**, as dry runs on the existing KBs with the shipped library of 19:

| | Correct | None | Wrong |
|---|---|---|---|
| Full library (52 documents) | 49 | 3 (the near-empty SPD-1 and Golden Record transcript, and one NIST paper) | 0 |
| Leave-one-out (each KB without its own ontology) | — | 50 | 2 |

In a mixed KB (one court opinion, one NIST standard, one Apollo review), each document's
entities came out in its own ontology's types, with no cross-domain types.

**Found on the way:**
- Generic type names ("person") and one-word triggers ("said") made the largest ontology
  (D6's) the runner-up everywhere.
- Without corpus scope, D6's memoir ontology took 8 of 13 court opinions in leave-one-out.
- Extraction samples at temperature 0.1 by default (`KWAAI_EXTRACTION_TEMPERATURE`): the same
  6 chunks gave 52 and then 28 entities. Eval v3 and pilot v3 should set it to 0.

**Not yet built:**
1. **Automatic selection at ingest.** `select` is a separate command today; `rag ingest` should
   run it for new documents when a library is configured.
2. **The schema.org backbone** (phase 3). Domain ontologies currently declare Person,
   Organization and so on themselves.
3. **Per-document validation on write.** `upsert_relation` coerces against the KB union.
4. **Dream's per-entity union** (§2.3). Dream uses the KB union today.
5. **Candidate libraries with no distinctive vocabulary can never be chosen:** the genre modules
   (narrative-history, scholarly, software-docs), CountryHistory and PythonDocs. A module is
   useful as a fallback only if it can be selected. Needs module-specific markers, or an
   explicit "fallback" rule.

**Behaviour changes that reach KBs with no ontology** (fresh-context review, 2026-09-28).
None of these is gated on an ontology, so any graph rebuilt with this branch differs
from one built before it, even with no ontology loaded. Eval v2 numbers are not
comparable with rebuilt graphs for that reason alone:
- **Graph-window order.** `graph build --graph-window` now reads a chunk's real neighbours
  (`sort_document_order`), and only neighbours from the same document. Before, the window
  was a random chunk from anywhere in the KB (1 in 3,722 pairs were adjacent in Legal).
- **Relation endpoints.** `resolve_entity_id` resolves an endpoint through the same name
  cleaning as the entity, and an edge whose endpoint does not resolve is skipped. Before,
  such an edge was written against an ID that matched no entity.
- **Dream prune.** The ontology dream task and the scorer's relation fraction apply
  only declared predicates. Without an ontology the prune behaves as before, but
  `--reset-graph` now keeps the stored ontology (`clear_graph_data`).

**Review fixes, same day:** the stored per-document record is read with
`doc_ontologies_checked` (an unreadable record is an error, never an empty map that
overwrites every other document); a re-selection keeps the hand-loaded `base`
ontology, which is also the fallback for chunks whose document has no choice;
`schema load` clears the per-document choices it would otherwise be shadowed by;
`--chunk-ids` is checked against the whole KB and refused with `--chunk-batch` > 1;
the HTML extractor no longer drops `<form>`, or everything after an unclosed `<head>`.

## 4. Validation before Eval v3

### 4.1 A labelled set of document types

Each document in the candidate Eval v3 corpora gets a hand label: schema.org type, `about`, and
whether it has a bibliography. There are about 70 documents across Legal, Astrophysics, NIST, one
of WarPeace/MobyDick, and D6.

- Claude labels the public documents from their first pages.
- Reza labels D6, and checks a sample of Claude's labels.
- Phase 2's classifier is measured against this set. The set is frozen before the classifier is
  tuned.

### 4.2 Extraction pilot

For each corpus, sample 100 body chunks with a fixed seed and extract twice: once with the
compiled defaults, once with the chosen ontology. Measure:

- relation yield per chunk, and the predicate distribution (no kinship on a non-narrative corpus);
- relation precision: Claude judges a sample for the public KBs, and Reza a sample for D6;
- entity-type distribution: for example, a scholarly corpus not dominated by Publication;
- prompt length and the entity-cap interaction on the 8B model.

Pass means precision at least as good as the defaults, and relation yield higher, on at least
3 of the 4 public corpora. If the pilot fails, it is written up and the design revisited before
anything is built at scale.

### 4.2a Pilot v1 result (2026-09-27 20:44 – 2026-09-28 00:10): **fails, 2 of 4**

Full report: `tests/kwaai-knowledge/results/ontology_pilot/report.md`. Binary `75a028a9`.

| Corpus | Relation precision (lenient), ontology vs defaults | Resolved relations per chunk | Verdict |
|---|---|---|---|
| Legal | 0.14 vs 0.18 (intervals overlap) | 0.51 vs 1.19 | fail |
| Astrophysics | 0.47 vs 0.42 (n = 15 and 20) | 0.10 vs 0.13 | fail (tiny samples; a tie) |
| NIST | 0.23 vs 0.10 | 0.54 vs 0.20 | pass |
| Manhattan | 0.47 vs 0.45 | 0.76 vs 0.67 | pass |

- **Entity precision** (correct type and a real entity): ontology vs defaults.
  - Legal 0.49 vs 0.42, NIST 0.23 vs 0.05: better with the ontology.
  - Manhattan 0.58 vs 0.69, Astrophysics 0.35 vs 0.40: worse.
- **Kinship edges outside D6:** 0 with the ontology, against 4 in Legal and 2 in Manhattan without.
  `associated_with` is 0% with the ontology outside D6; in D6, which declares it as its fallback,
  it rose to 33%.
- **Two process faults:**
  - Astrophysics and NIST were first left unjudged: the pilot started before they were added to
    `PUBLIC_KBS`. They were judged at 00:05 by resuming the script.
  - The v1 report's interval was computed on strict counts, next to a lenient estimate. The report
    now shows lenient, strict and any-support separately.
- **Diagnosis: 23–43% of relations had an endpoint that was not an entity** (Legal: 53 of 232
  defaults, 58 of 135 ontology). `resolve_entity_id` built endpoints from the raw name, while the
  drain stored the cleaned one. This affects both arms, and the ontology arm more, since its
  larger type set yields more cleaned names. Fixed in `b5336eba`.

### 4.2b Pilot v2: pre-registered 2026-09-28 00:20, before it ran

A bug-fix re-run, not a redesign:
- **Same:** the 150 chunks per corpus (seed 20260927), the machines, the judge (`claude-opus-5`,
  the same prompts), the arms, and the ontology files (unchanged since v1, including Astrophysics's
  and NIST's known gaps).
- **Changed:** the binary, `b5336eba` (endpoint resolution, plus the dream, scorer and HTML
  changes, none of which the build step uses).
- **Same pass criterion as v1:** on ≥ 3 of 4 public corpora, the ontology arm's lenient relation
  precision ≥ the defaults arm's, and its resolved relations per chunk are higher.
- Results go to `results/ontology_pilot_v2/`, and v1's graphs and report stay as they are.
- If v2 passes, the Eval v3 c00 builds for Legal and D6 start (`results/eval3/after_pilot.sh`).
- **Either way, both results are reported together.** A v2 pass after a v1 fail is reported as
  that, not as a clean pass.

### 4.2c Pilot v2 result (00:16 – 03:40): **fails again, 2 of 4**

Full report: `results/ontology_pilot_v2/report.md`. The endpoint fix works: 0 unresolved
relations in every arm, against 3–172 in v1.

| Corpus | Lenient relation precision, ontology vs defaults | Relations per chunk | Verdict |
|---|---|---|---|
| Legal | **0.25 vs 0.19** | 0.41 vs 1.34 | fail (yield) |
| Astrophysics | **0.65 vs 0.40** | 0.16 vs 0.27 | fail (yield) |
| NIST | 0.19 vs 0.14 | 0.60 vs 0.07 | pass |
| Manhattan | 0.46 vs 0.44 | 0.85 vs 0.65 | pass |

**The ontology now raises relation precision on all four corpora. It fails on yield, on Legal and
Astrophysics.** The verdict stands as pre-registered: the pilot failed, and no Eval v3 build
started.

**Exploratory (not pre-registered): judged-correct edges per chunk** (yield × fraction judged
supported):

| Corpus | v1 strict (defaults → ontology) | v2 strict | v2 supported or partial |
|---|---|---|---|
| Legal | 0.080 → 0.033 | **0.123 → 0.053** | 0.380 → 0.153 |
| Astrophysics | 0.040 → 0.040 | 0.067 → **0.093** | 0.147 → 0.113 |
| NIST | 0.007 → 0.067 | 0.007 → **0.040** | 0.013 → **0.193** |
| Manhattan | 0.220 → 0.280 | 0.187 → **0.289** | 0.393 → **0.487** |

Raw yield counts wrong edges too, and the defaults arm's extra Legal edges are mostly
`related_to` and `works_at`. Counted by correct edges, the ontology wins 3 of 4. **Legal is the
one consistent loser.**

**Diagnosis for Legal.** The defaults arm's supported edges there are legislative and political:
- `Congress passed Missouri Compromise`
- `BCRA amends FECA`
- `General DeWitt issued Proclamation`
- `John Randolph member_of House`

`Legal.yaml` is built around case law. It has no Person, Organization or Legislature type, and no
`amends`, `passed`/`enacted_by`, `issued` or `upholds`, so these facts cannot be written. The
ontology arm's errors are forced fits of the narrow vocabulary:
- `Circuit Court has_jurisdiction_over Missouri`;
- "Page", a citation fragment, `enacted_in` United States.

The nugget check (Eval v3 plan) found the same gaps, so the ontology is incomplete for the
corpus, and the extractor is not the cause.

**Other observations:**
- **D6's fallback is abused.** 37% of its ontology-arm edges are `associated_with`, against 9%
  with defaults. A declared fallback invites vague edges.
- **Entity precision is mixed:**
  - better with the ontology: Astrophysics 0.48 vs 0.43, NIST 0.21 vs 0.16;
  - worse: Legal 0.45 vs 0.48, Manhattan 0.57 vs 0.70.

  Manhattan's drop is worth a look: its Document and Policy types may be over-applied.

**Proposed next steps (Reza decides):**
1. **Complete the ontologies from the test set and the defaults arm's correct edges,** measured
   as for Manhattan:
   - Legal: Person, Organization or Legislature, `amends`, `enacted_by`, `issued`, `upholds`,
     `member_of`;
   - Astrophysics: Document and Legislation types, astronomical objects, detection methods;
   - NIST: Person, Tactic, authorship, `profile_of`.
2. **Decide D6's fallback:** drop it, or cap its share.
3. **Pre-register pilot v3** with the metric the evidence supports (judged-correct edges per
   chunk), declared as the primary before the run. Reporting v1–v3 together is the honest
   record.
4. **Only then** start the Eval v3 builds.

### 4.2d Pilot v3: pre-registered 2026-09-28, before it ran (Reza: "go")

**What changes from v2:**
- The ontologies: completed Legal v2, Astrophysics v2, NIST v2, Manhattan v3, and D6 v10 with no
  fallback.
- **The ontology arm's vocabulary is chosen by `rag graph schema select`** against the shipped
  library, per document, not loaded by hand. So the pilot also tests selection.
  - The corpus-scoped ontologies are eligible only in their own KB (`--own-corpus` for
    Manhattan_v3 and D6).
- **Temperature 0** for extraction in both arms (`KWAAI_EXTRACTION_TEMPERATURE=0`). At 0.1 the
  same 6 chunks gave 52 and then 28 entities.
- **Manhattan becomes Manhattan_v3**, the re-ingested KB Eval v3 would use, with a fresh
  sample of 150 prose chunks (seed 20260927).
- **Binary:** `feat/ontology-selection` at the commit that launches the run, recorded in the
  report.

**Unchanged:**
- The other corpora's 150-chunk samples and seed.
- The judge (`claude-opus-5`, the same prompts), with ±1-neighbour passages.
- Both arms on the same machine: metro-win for the public corpora, the Mac for D6.

**Primary measure: supported relations per chunk.** That is resolved relations per chunk ×
the fraction the judge rates `supported`.
- **Pass:** the ontology arm's is higher on at least 3 of the 4 public corpora (Legal,
  Astrophysics, NIST, Manhattan_v3).
- **Why this measure:** v1 and v2's rule (lenient precision and raw yield both higher) counted
  vague edges as yield. This measure counts edges the judge accepts.
- **Secondary, reported but not deciding:**
  - lenient precision;
  - supported-or-partial edges per chunk;
  - entity precision;
  - the `associated_with` share;
  - kinship edges outside D6;
  - the per-document selection table.

D6 is counts only; it is never sent to the API. **v1, v2 and v3 are reported together.**

### 4.3 Checks that must fail loudly

- A KB whose ontology failed to load, or has an unknown `extends:`, is an error, not a fallback.
- Eval drivers assert `kb_ontology` is present in every snapshot, and record its hash in the dump
  records, so a snapshot without an ontology can't be scored.

---

## 5. Eval v3 changes (to go into a new pre-registration)

- **Corpora:** only those ≥ 90% clean in `corpus_hygiene_20260825_*.md` with a NotebookLM
  tracker. Candidates: Legal (96.9%), Astrophysics (94.2%), NIST (90.7%), WarPeace (99.9%) or
  MobyDick (98.8%), and D6.
  - Each tracker's answers are checked against its questions before the tracker is used; the
    check is the question/answer vocabulary overlap that caught DreamMem.
  - Climate and DeepSea are dropped, as blocked on ingestion.
- **Arms:** A with `--no-relations`, and B with relations under the chosen ontology.
- **Eval v2 as a descriptive comparison:** its Legal arm B serves as a "default vocabulary"
  comparison, clearly labelled as coming from a different pipeline version.
- **Driver:**
  - load the ontology after cloning and before building;
  - assert it survives the build;
  - hash it into the snapshot metadata.
- **Hardware:** metro-linux is CPU-only until its GPU build is fixed. Every trajectory stays on
  one machine; the pinning is recorded in the pre-registration.
- **Carried over:** the NLI and judge caches, the driver fixes from 2026-09-27, and the gold
  pipeline. D6's gold carries over unchanged. New corpora need nugget drafting, NLI checks and
  adjudication, as eval2 did.

---

## 6. Working constraints

- **The Eval v2 driver is still running** on the installed `~/.cargo/bin/kwaainet` and
  `kwaainet-eval2`, and the score loop launches Python files from the main working tree.
  - Development happens in a **separate git worktree** on a branch from a freshly fetched
    `origin/main`.
  - Nothing is installed over `kwaainet` or `kwaainet-eval2` until the run finishes.
  - Test binaries are installed under a new name (`kwaainet-ontsel`), then `codesign`ed.
- `feat/eval-dump-jsonl` (the committed eval2 harness plus today's uncommitted fixes) is
  finished and proposed separately from this work.
- **Before the PR** (CLAUDE.md):
  - test from a fresh `KWAAINET_HOME`, since ingest defaults change;
  - fresh-context review of `git diff origin/main...`;
  - fmt, clippy and tests.

---

## 7. Decisions

Settled by Reza on 2026-09-27:

1. **Network lookups at ingest.** Off by default. Never for a KB marked private. Opt-in
   (`--metadata-lookup`) for public corpora: DOI via Crossref, ISBN via Open Library. What a
   lookup returns is stored as evidence, next to what was derived from the text.
2. **Where domain extensions come from.** The existing ratified YAML first. Induction
   (`ontology_induce.py`) only with Reza's sign-off before an induced extension is used in any
   build.
3. **Per-document vocabularies first** (§2.3). The KB-level union is kept only for write-time
   validation and query scope.

Still open:

4. **The Oct 1 paper.** Phases 0–5 plus fresh cycle-0 builds do not fit in 4 days, with
   metro-linux on CPU. The options:
   - report Eval v2 honestly, with the vocabulary defect stated as a limitation and arm A
     (no relations) as the headline, since the defect affects it least;
   - request an extension;
   - narrow the paper to what Eval v2 can support.
