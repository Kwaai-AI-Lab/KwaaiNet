# Per-KB ontology: making the knowledge schema a first-class artifact

Status: **phases 0-2 landed** in #150 (merged 2026-08-31). Written 2026-08-25.

| Phase | State |
|---|---|
| 0 — reconcile vocabularies, persist `--entity-types` per KB | done |
| 1 — `Ontology` struct + loader, `ontology load` CLI | done for `ontology.yaml` (`Ontology::from_yaml`); the **LinkML loader was not built**, so the §5.1 deferral is spent rather than exercised — the format question in §7.2 is still open and now has one implementation, not two |
| 2 — `genealogy` module; extraction, axioms and `scorer.rs` read the ontology | done (`schema_type_for_ontology`, `query_patterns_from_ontology`); **query-side abstention (§3.9) was not built** |
| 3 — run the bakeoff | D6 only, and inconclusive — see [`D6-FullAB-results.md`](./D6-FullAB-results.md). Arms B/D over Legal, Astrophysics, NIST not run |
| 4 — ontology-driven extraction becomes the default | not done; a KB uses its ontology only when one is loaded |

The sections below are the plan **as written on 2026-08-25** and are kept in that
tense as a record of what was predicted. Two of its predictions did not survive:
§5's bakeoff was defeated by run-to-run variance (`OntologySession-assessment.md`
§4d), and the recall benefit it was designed to demonstrate has not appeared in
four attempts.

Implements step 1 of *Concrete first steps* in
[`DreamRAG-Ontology-Eval-Compression.md`](./DreamRAG-Ontology-Eval-Compression.md)
— "make the ontology a per-KB artifact" — at full scope: entity types, relation
types with domain/range, and declarative axioms, in one versioned per-KB file.

**Method (Reza, 2026-08-25): measure, don't decide upfront.** We have 15 KBs, a
recorded baseline, and a working eval harness. Where a question about ontology
design is empirical, it goes into the bakeoff in §5 rather than being settled in
advance by argument. §7 now separates the questions eval *can* answer from the
ones it structurally cannot. This changes the shape of the build: phase 1 exists
to make a bakeoff runnable, not to freeze a format.

The motivating observation, in Reza's words: *asking a climate science paper
"who is your uncle?" makes no sense.* Today the extraction prompt for the
Climate KB literally offers `uncle_of`, `niece_of` and `foster_child_of` as
candidate predicates, and offers `birthDate / birthPlace / deathDate` as the
fields to populate. Climate has 1979 entities and **4** relations. That is the
ontology mismatch showing up as a number.

---

## 1. Audit of the current state

### 1.1 Four global vocabularies, all `const` in Rust

| Vocabulary | Location | Size | Consumed by |
|---|---|---|---|
| `ENTITY_TYPES` | `core/crates/kwaai-rag/src/graph.rs:19` | 17 | extraction prompt, when `--entity-types` is not passed |
| `RELATION_TYPES` | `graph.rs:40` | 35 (14 kinship) | extraction prompt; `dream.rs:334` and `dream_tasks.rs:365` validation |
| `PERSON_RELATION_TYPES` | `graph.rs:96` | 20 | extraction prompt when `entity_types == ["Person"]` |
| `IN_SCOPE_RELATION_TYPES` | `relation_extract.rs:100` | 15 (9 kinship) | Phase-4 axiomatic relation pipeline + its verify prompt (`rag_cmd.rs:7130`, `:7226`) |

Correction to the framing doc: it stated the complete relation vocabulary is
"thirteen kinship predicates and one escape hatch." That describes
`PERSON_RELATION_TYPES` / `FAMILIAL_RELS`, not the general set — `RELATION_TYPES`
does carry `measured_by`, `caused_by`, `cites`, `contradicts`, `supports`,
`part_of`, `located_in`. The argument is unaffected: the general set is still one
uniform vocabulary applied to memoir, physics, law and poetry alike, with 40% of
it kinship, and with nothing for Quantity, Dataset, Finding, Obligation, Clause,
Motif or Speaker.

### 1.2 Three disagreements between those lists (fix regardless of ontology work)

1. `IN_SCOPE_RELATION_TYPES` contains **`affiliated_with`**, which is absent from
   `RELATION_TYPES`. The axiomatic path can therefore commit an edge that
   `dream.rs:334` subsequently treats as invalid.
2. `PERSON_RELATION_TYPES` contains **`supported`**; `RELATION_TYPES` has
   **`supports`**. Same failure mode, person-only runs.
3. The field-key block in the extraction prompt (`graph.rs:5168–5174`) is a
   hardcoded D6-shaped literal — `Person: birthDate, birthPlace, deathDate…;
   Legislation: dateEnacted…` — sent verbatim to every KB including RFCs and
   PythonDocs. It duplicates `expected_fields()` (`graph.rs:600`) rather than
   deriving from it, so the two can drift.

### 1.3 What is already per-KB

More than the framing doc credits, and it is the right foundation to build on:

- **`KBEntityTypeSchema`** (`graph.rs:365`) — `name`, `description`, `examples`,
  `anti_examples`, `fields`.
- Persisted per-KB in the graph store's `metadata` table under key
  `kb_entity_schemas` (`graph.rs:4299` set / `:4309` get), alongside
  `doc_metadata` and `document_titles`.
- Loaded from YAML by `kwaainet rag graph schema set --kb X --file Y`
  (`cli.rs:2281`, `rag_cmd.rs:9990`).
- Injected into the extraction prompt as the `KB-SPECIFIC ENTITY TYPE GUIDANCE`
  block (`graph.rs:5133–5151`) and read at `ingestion.rs:318` and `:836`.
- Two authored instances exist, both D6: `tests/kwaai-knowledge/d6_entity_schema.yaml`
  and `d6_entity_type_schemas.yaml`.

### 1.4 Gaps in that foundation

- **Entity types only.** No relation types, no axioms.
- **`KBEntityTypeSchema.fields` is declared and never read anywhere.** Dead field.
- **The allowed entity-type list is not persisted.** `--entity-types` is a
  per-ingest-run CLI flag (`rag_cmd.rs:2947`); `GraphConfig.entity_types`
  (`ingestion.rs:35`) defaults to empty → falls back to the global 17. A rebuild
  that forgets the flag silently changes the ontology.
- **Axioms are hardcoded Rust**, with no per-KB dimension:
  - familial-requires-Person — `relation_extract.rs:368` (Axiom 1)
  - ambiguous-window — Axiom 2, same function
  - contradiction table — Axiom 3, via `family_role_contradicts`
  - `FAMILIAL_INVERSE` (`graph.rs:183`), applied at `:1017`, `:1524`, `:2042`, `:3605`
  - the gender heuristic in the spouse/sanitise path (`graph.rs:1357–1409`),
    which deleted valid D6 relations because Gadija was recorded as Male

### 1.5 The graph scorer is itself hardcoded to the memoir ontology

`scorer.rs` — the `kwaainet rag graph score` number recorded in every baseline —
maps our types to schema.org via `schema_type_for()` and then checks
`expected_relation_groups()`. For `schema:Person` the expected group is the
kinship set. An entity scores well on the *relationship* pillar when it has the
relations the memoir ontology predicts.

**Consequence for measurement: `graph score` cannot be used to compare ontology
arms.** A Climate KB re-extracted under a climate ontology would score *worse*
on it — not because the ontology is worse, but because the scorer expects
kinship and finds `measured_by`. The 2026-08-22 baseline's "graph score" column
is a memoir-ontology-conformance score, and reading it as a quality score across
15 heterogeneous KBs was already an error.

Either the scorer becomes ontology-parameterised (`expected_relation_groups`
derived from the KB's ontology rather than a Rust match) or it is excluded from
the bakeoff. §5 excludes it, and §6 phase 2 parameterises it.

Related: `schema_type_for()` hardcodes a 20-arm match including `Group`,
`Language`, `Family` — types not in `ENTITY_TYPES`, added because they appeared
in D6. A fifth de facto vocabulary, undeclared.

### 1.6 Corpus hygiene bounds what any ontology can achieve

Measured 2026-08-25 while sampling text to author arm B
([`results/corpus_hygiene_*.md`](../../../tests/kwaai-knowledge/results/)).
The share of each KB's chunks that are usable prose about its subject:

| KB | clean | dominant defect |
|---|---|---|
| Meetings | **2.0%** | 1875/1914 chunks are WebVTT cue numbers and timestamps |
| PythonDocs | **16.1%** | 2850 chunks of raw Sphinx HTML |
| Poems | **22.3%** | 22202 chunks of raw HTML (`<div class="verse">`) |
| CountryHistory | **25.0%** | 12296 chunks of raw HTML |
| RFCs | **30.6%** | 4454 chunks character-spaced (`L I S T  t y p i c a l l y`) |
| OSMDocs | **33.9%** | markup + character-spaced PDF |
| Climate | 68.7% | bibliographies, funding acknowledgements, download watermarks |
| DreamMem / DeepSea | 74–75% | reference lists |
| NIST / Astrophysics / Legal / MobyDick / D6 / WarPeace | 90.7–99.9% | — |

**This resolves the H1/H2 question in §5.4.2 before any arm is run.** Crossing
clean-rate with relation density partitions the corpora:

- **Genuinely vocabulary-bound** (clean, no relations): **Legal** (96.9%,
  0.0007), **Astrophysics** (94.2%, 0.0036), **NIST** (90.7%, 0.0014). Arm B
  can pay here and nowhere else.
- **Already productive**: D6, WarPeace, MobyDick, Manhattan.
- **Data-bound**: everything else. An ontology applied to a corpus that is 68%
  character-spaced PDF measures the dirt.

Two consequences for the plan. The bakeoff's KB set (§5.3) should be **Legal,
Astrophysics, NIST**, with D6 as the regression guard — not Climate, which was
chosen precisely because it looked worst and turns out to be 31% unusable.
And **an ingestion-hygiene pass is a higher-value piece of work than this entire
plan for nine of sixteen KBs.** `doc_schema.rs` already has the machinery
(section `skip`, `SectionType`); only D6 has ever used it.

---

## 2. Design

> **This is arm B's format, not a settled decision.** §5.1 defers the format
> question by parsing both this and a LinkML subset into one internal struct.
> What follows is the concrete proposal to measure, not the conclusion.

### 2.1 The file

One per-KB `ontology.yaml`, loaded through the **existing** command and stored
under the **existing** metadata key family, so D6 does not move and no new
surface is introduced:

```bash
kwaainet rag graph schema set --kb Climate --file climate_ontology.yaml
kwaainet rag graph schema show --kb Climate
```

```yaml
ontology:
  name: climate-science
  version: 1
  extends: none            # none | genealogy | scholarly | legal | standards | software | poetics

entity_types:
  - name: ClimateVariable
    description: >
      A measurable physical quantity of the climate system…
    examples: ["sea surface temperature", "albedo"]
    anti_examples: ["IPCC"]        # an Organization, not a variable
    irreducible: false             # see below — must be kept verbatim or not at all
    fields:                        # activates the currently-dead `fields` member
      - { key: units,    desc: "SI units of measurement" }
      - { key: timescale, desc: "characteristic timescale" }
  - name: Measurement
    description: "A specific reported value with units and provenance."
    irreducible: true              # exact numbers survive compression verbatim

relation_types:
  - name: measured_by
    description: "…"
    domain: [ClimateVariable, Phenomenon]     # allowed subject types
    range:  [Instrument, Dataset]             # allowed object types
  - name: forces
    domain: [Forcing]
    range:  [ClimateVariable]
    inverse: forced_by
  - name: part_of
    transitive: true
    acyclic: true
  - name: correlates_with
    symmetric: true

axioms:
  - { kind: contradiction, pairs: [["increases", "decreases"]] }
  - { kind: functional, relations: [published_in] }   # at most one object per subject

# Per-stream sections — a personal KB ingesting email + telemetry + financial
# records needs different types and retention rules per stream, in one KB.
streams:
  - name: email
    entity_types: [Person, Organization, Commitment]
    retention: thread_aware
  - name: telemetry
    ingest: aggregate_only         # never enters the text pipeline at all
```

#### Three requirements imposed by the parent document's refinements

Reading the full
[`DreamRAG-Ontology-Eval-Compression.md`](./DreamRAG-Ontology-Eval-Compression.md)
(not just §1) adds three things to the artifact that this plan originally
omitted. Its *dynamic ingest* refinement revises first-step 1 to read: "the
schema should carry per-stream sections and an `irreducible` flag on entity
types **from the start**."

1. **`irreducible` on entity types.** Exact quotations, numbers, identifiers and
   legal text must be kept verbatim or not at all. The ontology is where that is
   declared, because the compression tier needs to read it. Cheap to add now,
   expensive to retrofit once ontologies are authored.
2. **`streams:` sections.** The parent doc's open question "does a KB have one
   ontology or several?" stops being theoretical for a personal KB ingesting six
   streams with different velocities, value densities and retention obligations.
   Phase 1 should parse and store this even if nothing consumes it yet, so the
   format does not need a breaking change later.
3. **A temporal model — deliberately left blank.** `works_at` needs a validity
   interval; without one a new fact contradicting an old one is a *conflict*
   rather than a *succession*. The parent doc's reading order puts TOKI third
   precisely so we do not "design a schema without one". This plan does not
   invent a temporal syntax; it reserves the slot and defers to TOKI's bitemporal
   vocabulary (valid-time vs transaction-time). **Recorded as a known hole, not
   an oversight** — see §7.2.

Every axiom kind generalises something currently hardcoded:

| Axiom kind | Generalises |
|---|---|
| `domain` / `range` on a relation | Axiom 1, familial-requires-Person (`relation_extract.rs:368`) |
| `inverse` | `FAMILIAL_INVERSE` (`graph.rs:183`) |
| `symmetric` | the caller-side both-directions storage for `spouse_of`/`sibling_of` |
| `contradiction` | `family_role_contradicts` / Axiom 3 |
| `functional`, `transitive`, `acyclic` | new — no current equivalent |

The ambiguous-window axiom (Axiom 2) stays in Rust: it is a property of the
extraction window, not of the domain.

### 2.2 Built-in ontology modules

`extends:` names a compiled-in module expressed in the *same* declarative form.
`genealogy` reproduces today's kinship behaviour exactly — the 14 familial
predicates, their inverses, their Person domain/range, and the contradiction
table. D6's ontology becomes `extends: genealogy` plus its authored
Publication/Legislation types.

**This is the regression guard.** If `genealogy` is faithful, a D6 rebuild under
the new machinery must reproduce the current graph. That is the acceptance test
for Phase 2 below.

### 2.3 Default when no ontology is declared

A KB with no stored ontology behaves exactly as today: `ENTITY_TYPES` +
`RELATION_TYPES`, no domain/range enforcement. Fourteen KBs are in this state
and must not change until each is given an ontology deliberately.

---

## 3. Wiring points, in dependency order

1. **`graph.rs`** — new `Ontology` type; `KBEntityTypeSchema` becomes its
   `entity_types` member (same serde shape, so stored D6 JSON still
   deserialises). `set_kb_ontology` / `get_kb_ontology` beside the existing
   `set_kb_entity_schemas` / `get_kb_entity_schemas`, which stay as a
   back-compat read path.
2. **`extract_from_text`** (`graph.rs:5563`) — entity list, relation list, the
   field-key block and the guidance block all derive from the ontology. Delete
   the hardcoded field literal at `:5168–5174`; derive from `expected_fields()`
   merged with the ontology's `fields`.
3. **`validate_relation_axioms`** (`relation_extract.rs:453`) — replace the
   hardcoded Axiom 1 with a generic domain/range check driven by
   `RelationAxiomSnapshot.entity_types`; drive Axiom 3 from the ontology's
   `contradiction` pairs. Keep demotion-not-deletion, so demoted candidates keep
   showing up in metrics.
4. **`upsert_relation`** (`graph.rs:1215`) — inverse and symmetry from the
   ontology rather than `FAMILIAL_INVERSE`; enforce domain/range at commit.
5. **`dream.rs:334` / `dream_tasks.rs:365`** — validate against the KB's
   vocabulary, not the global const. Fixes the `affiliated_with` disagreement by
   construction.
6. **Phase-4 verify prompt** (`rag_cmd.rs:7130`, `:7226`) — `IN_SCOPE_RELATION_TYPES`
   becomes the ontology's relation list.
7. **Persist the allowed entity-type list.** `--entity-types` becomes an
   override of the stored ontology rather than the only source.
8. **`schema show`** — print relation types and axioms, not just entity types.
9. **Query-side abstention.** An ontology that declares what types exist also
   declares what is *out of scope*, which is the basis for answering "who is
   your uncle?" against Climate with *"this knowledge base has no notion of
   kinship"* rather than confabulating. This was scoped out of the first draft
   and is now back in: the parent doc's literature scan names **abstention** as
   the one requirement missing from the whole plan (from Karpathy's list of six
   unsolved problems), and Reza's original framing of this work is an abstention
   case. It is also the cheapest user-visible win here.

Deliberately **out of scope** for this plan: retrieval-side traversal policy and
ontology-shaped question generation. Both depend on this landing first and are
tracked in the parent doc (§2).

---

## 4. Migration

**`entity_id = sha256(name.lower() + "::" + entity_type)[..8]`** (`graph.rs:329`).
Entity type is part of the identity hash. Renaming a type in an ontology —
`Location` → `Place`, say — re-IDs every node of that type and orphans every
relation endpoint pointing at it.

Consequences:

- Ontology edits on a mature graph are **not** free. Either a `schema migrate`
  that rewrites entity IDs plus all relation endpoints and the
  `chunk_to_entities` index transactionally, or an accepted full rebuild.
- D6 is ~5h to rebuild. Fifteen KBs multiply that. Prefer `schema migrate` for
  pure renames; accept rebuild when the type *set* changes, since that changes
  what extraction would have found in the first place.
- Ontology carries `version:`; bumping it without a migrate should warn.
- This is an ontology-alignment problem with mature prior art (AgreementMakerLight,
  LogMap, DeepOnto) — see §8.6 when `schema migrate` is designed.

---

## 5. The bakeoff

Rather than deciding which sourcing path is right, build the thinnest machinery
that can host all of them and measure. `kwaainet rag` already does most of this.

### 5.1 What makes deferral possible

**Decouple the internal representation from the file syntax.** Phase 1 defines
an internal `Ontology` struct and *two* loaders — one for `ontology.yaml`, one
for a LinkML subset. Both parse to the same struct; everything downstream reads
the struct. The format question (§7) then stops being a fork in the road and
becomes a swappable loader, decidable later on ergonomics rather than guessed at
now. This is the single design move that lets the rest be measured instead of
argued.

### 5.2 Arms

> **Corrected 2026-08-25 by arm A data** — see
> [`results/ontology_bakeoff_armA_findings.md`](../../../tests/kwaai-knowledge/results/ontology_bakeoff_armA_findings.md).
> Arm A as originally written has never been run: every KB was built with a
> hand-tuned `--entity-types` restriction from `multi_corpus_graph_build.sh`, so
> the 2026-08-22 baseline is already a degenerate arm B. Arm A is redefined
> below as that bash declaration, which is the real control everything was
> measured against.

| arm | ontology source | cost |
|---|---|---|
| **A — control** | the per-KB `--entity-types` restriction in `multi_corpus_graph_build.sh`: entity types only, no relation vocabulary, no axioms | free (already measured) |
| **B — declared** | hand-authored per KB, as D6 already is | ~half a day per KB |
| **C — induced** | sampled corpus → LLM proposes → grounded → ratified (§8.4/8.5) | ~1% of corpus per KB |
| **D — selected** | pick a built-in module by coverage score (§8.2) | free once modules exist |

Arm A is the baseline we already hold. Arm D is cheapest to *run* but needs the
modules that arm B produces, so it comes last and reuses B's output as its
library.

### 5.3 KBs

Three, chosen to span the interesting range rather than to be representative:

| KB | entities | relations | recall | why |
|---|---|---|---|---|
| **D6** | — | — | ~90% | regression guard: the ontology is right and known-good |
| **Legal** | 2955 | 2 | 87.5% | 96.9% clean, 0.0007 rel/ent — the purest vocabulary-bound case |
| **Astrophysics** | 6437 | 23 | 57.3% | 94.2% clean; also tests the multi-stream mechanism |
| **NIST** | 5182 | 7 | 76.4% | 90.7% clean; standards vocabulary is *given* by the corpus |
| ~~Climate~~ | 1979 | 4 | 48.4% | **dropped — 31% of its chunks are unusable (§1.6)** |

Adding a fourth (WarPeace: 742 relations, 63.7% recall) tests the opposite
failure — a KB with many relations that are not paying.

### 5.4 Metrics

Deliberately *not* a single number, because §5.5 says the end metric is weak.

| metric | source | new work |
|---|---|---|
| **escape-hatch rate** — share of relations that are `associated_with`/`related_to` | graph | `graph stats --by-type` |
| **ontology coverage** — share of extracted candidates the type set accommodates | graph + ontology | small; same readout |
| **eviction eligibility** — share of chunks passing the consolidation gate | graph + chunks | moderate; see §5.4.1 |
| entity-type distribution | graph | same |
| relation-type distribution | graph | same |
| axiom demotion counts | `RelationAxiomMetrics.candidates_demoted_by_axiom` | **none — already instrumented** |
| entity / relation counts | `rag graph stats` | none |
| recall on the 20-question set | `eval_all_kbs.sh` | none |
| per-question detail | existing per-KB reports | none (standing rule) |
| ~~graph score~~ | — | **excluded — see §1.5** |

**The escape-hatch rate is the most diagnostic and the cheapest.** The framing
doc's own observation was that dream added 1000+ relations and moved recall not
at all because "every relation dream extracts is `associated_with`".

*Measured 2026-08-25: that claim is overstated.* Escape-hatch runs 8.5% (D6) to
44.1% (CountryHistory). The dominant predicate is `located_in` — 54% of WarPeace,
51% of MobyDick. The graphs are **spatial**, not associational. The conclusion
holds (these edges carry no domain meaning) but "the graph is a gazetteer" is the
accurate description. Escape-hatch alone would have missed this, so the metric
is the *full relation-type distribution*, with escape-hatch as its headline. If Climate
under a climate ontology still dumps everything into `associated_with`, the
ontology did not take — and we know that without waiting for a recall number.
It also fails fast: it is readable after extraction, before any eval run.

`graph stats --by-type` is the only new instrumentation the bakeoff strictly
needs. `graph stats` currently prints two integers (`rag_cmd.rs:2813`).

#### 5.4.1 Two metrics taken from the parent document

Reading the full parent doc supplied two ontology-discriminating metrics better
than the ones this plan started with, both of which need **no eval run** — which
is exactly what makes the bakeoff runnable before the curriculum exists (§5.5).

**Eviction eligibility.** The parent doc's *Metrics for a steady state* lists it
as "a direct, cheap proxy for extraction quality that needs no eval run." The
consolidation gate is: *a chunk becomes evictable only once its claims are
represented in the graph, corroborated by at least one independent chunk or
explicitly marked irreducible.* Applied to the bakeoff, an ontology that lets
more of Climate's chunks pass that gate has genuinely captured more, whatever
the 20-question recall says. This is the same argument as the parent doc's
compressibility thesis — *if extraction captured the facts, you can discard the
text and still answer* — reduced to something measurable in an afternoon.

**Ontology coverage.** The harness section names "ontology coverage falling as a
stream drifts" as a sense-signal for proposing new predicates. That is the same
number NCBO's recommender uses as its *coverage* criterion (§8.2), and the same
number a module selector would need for arm D. One metric, three uses — worth
implementing once and properly.

Both are stronger than escape-hatch rate, which stays because it is the
cheapest and fails fastest.

### 5.4.2 The prediction the bakeoff must falsify

Arm A separated two hypotheses that the plan had conflated. Climate's declared
types (Organization/Legislation/Publication) admit **nine** already-applicable
predicates from `RELATION_TYPES` — `cites`, `part_of`, `contains`,
`described_in`, `defined_by`, `supports`, `contradicts`, `implements`,
`associated_with`. It produced **four relations from 1979 entities**.

- **H1 — vocabulary.** The ontology offers nothing assertable. Arms B/C fix this.
- **H2 — extraction.** Applicable predicates exist and are not being found. No
  ontology fixes this.

**Recorded prediction, before running the arms: if arm B or C gives Climate a
domain ontology and relation density stays near zero, H2 dominates and this
entire plan will not move the number.** Poems is the existing evidence for H2 —
it declares Person+Place, the same pair that gives WarPeace 0.205 relations per
entity, and returns 0.0006.

This is the most valuable thing the bakeoff can produce, and it is worth more
than knowing which arm wins.

### 5.5 The honest caveat about the end metric

The 20-question sets are a weak instrument for this, for reasons already
recorded: they saturate (D6 sat at ~90% before and after 114 new relations),
they cannot see what they do not ask, and relation count does not predict recall
(Pearson +0.081 across 15 KBs).

So the bakeoff is powered to detect a **large** effect and not a subtle one.
That is acceptable if we read it correctly:

- A **big move** in escape-hatch rate or recall is real signal.
- A **null** on recall is *not* evidence the ontology did not help — it is the
  known limit of the instrument, and the same null the dream sweep produced.

This is the strongest argument for doing §2 of the parent doc (tiered,
corpus-generated questions) in parallel rather than after. The bakeoff is worth
running on the current instrument; it is worth more on a better one.

**How this squares with "evaluation first."** The parent doc states the
dependency order three separate times — instrument, then ontology, then
compression — and the agentic-harness refinement calls it "the third independent
argument for doing evaluation first." Taken literally, phase 3 of this plan
front-runs that order.

The reconciliation is the metric split in §5.4. Escape-hatch rate, ontology
coverage and eviction eligibility are **instrument-independent**: they read the
graph, not the exam, and they discriminate between ontologies without a
curriculum. Recall is the only bakeoff metric that depends on the weak
instrument, and it is explicitly demoted to a corroborating signal rather than
the verdict.

So the honest statement of the order is:

- Phases 0–2 (the artifact, the loaders, the parameterised scorer) are the
  "mechanical unblocker" the parent doc's first-step 1 describes. No dependency
  on evaluation.
- Phase 3 runs on instrument-independent metrics **now**, and re-runs against
  the curriculum when it exists. The arms are cheap to re-measure; the
  extractions are not, and they are the expensive part.
- Any conclusion of the form "ontology X improves recall by N points" waits for
  the curriculum. Conclusions of the form "ontology X moved 60% of Climate's
  edges out of `associated_with`" do not.

### 5.6 Order of operations

1. `graph stats --by-type` — hours, unblocks every arm's cheap metric.
2. Re-run arm A on D6 / Climate / Legal to refresh the control under the current
   binary (the 2026-08-22 baseline predates any of this).
3. Arm C on Climate and Legal — cheapest per §8.4, and its output is a *draft*
   that arm B can start from rather than a competitor to it.
4. Arm B on Climate and Legal, hand-authored (or ratified from C's draft).
5. D6 under arm B with the `genealogy` module — the regression gate.
6. Arm D once B has produced two modules worth selecting between.

Steps 3 and 4 compose rather than compete: if induction produces a usable draft,
"declared" and "induced" are the same pipeline with a human in the middle, and
the bakeoff has answered the more interesting question — *how much* ratification
the draft needs.

---

## 6. Phasing

| Phase | Content | Gate |
|---|---|---|
| 0 | Reconcile the vocabulary disagreements (§1.2); persist `--entity-types` per KB; `graph stats --by-type` | `cargo test`; D6 eval unchanged |
| 1 | Internal `Ontology` struct + **two** loaders (`ontology.yaml`, LinkML subset); store/show. Parses `irreducible` and `streams:` even though nothing consumes them yet (§2.1) | both loaders round-trip to the same struct; D6's stored schemas still load |
| 2 | `genealogy` module; extraction + axioms + upsert read the ontology; **`scorer.rs` parameterised by ontology (§1.5)**; **query-side abstention** (§3.9) | D6 rebuild reproduces current graph and score; "who is your uncle?" against Climate abstains |
| 3 | Run the bakeoff (§5.6) on instrument-independent metrics | arms recorded with per-question logs; re-runnable against the curriculum later |
| 4 | Ontology-driven extraction becomes the default | phase 3 shows no regression on D6 |

Phase 1's two loaders are the deferral mechanism, not redundant work — see §5.1.

Each phase gets a regression test (standing rule: every bug fixed ships with a
test that catches it next time). Phase 0's three disagreements each get one.

---

## 7. Open questions

Split by whether the bakeoff can settle them.

### 7.1 Empirical — goes in the bakeoff

- **Which sourcing path wins?** Declared vs induced vs selected. Arms B/C/D.
- **How much ratification does an induced draft need?** The interesting version
  of the above — measure edit distance from arm C's draft to arm B's authored
  version.
- **Is `associated_with` legitimate?** Probably yes as an honest fallback for
  genuinely unstructured association — the bug is that it is the *default*. Test
  it: run one arm with a declared fallback predicate and one without, and read
  the escape-hatch rate against recall.
- **Does a narrow ontology cost recall?** The ghost-prune risk (§7.3). Domain/range violations are demoted and counted, never silently
  dropped, so a too-narrow ontology shows up in the demotion metric rather than
  as an unexplained recall drop.
- ~~**One ontology per KB, or several?**~~ **Decided (Reza, 2026-08-25):
  several.** `streams:` is load-bearing, not a reserved slot. Arm B's authored
  ontologies use it in seven KBs; Astrophysics is the clearest case — sampling
  showed it is Apollo programme history *and* Planck cosmology in one KB, with
  almost no shared vocabulary. The `scholarly` module's body/bibliography split
  is the one that matters most: Climate's graph is 1163 Publications and 655
  Organizations because it modelled the reference lists.

### 7.2 Not empirical — decide by argument or constraint

- **`ontology.yaml` vs LinkML.** Both express the same content, so no eval can
  separate them. Decided on ergonomics and ecosystem: LinkML brings tooling, a
  registry, and SPIRES as a working reference on local models (§8.3). §5.1
  defers this by making it a loader; it still needs an answer eventually.
- **Entity IDs.** `entity_id = hash(name + entity_type)` (§4) is a storage
  constraint, not a hypothesis. Either `schema migrate` or rebuild.
- **Reproducibility of induced schemas.** An LLM asked twice gives two answers
  (§8.7). The artifact is versioned and committed, so induction must emit a
  draft for ratification, never a step re-run at ingest. Constraint, not question.
- **The temporal model.** Relations need validity intervals or succession is
  indistinguishable from contradiction (§2.1). Not an ontology question and not
  ours to invent — the parent doc's reading order puts TOKI's bitemporal algebra
  third for exactly this reason. Phase 1 reserves the slot; the syntax waits.
- **Should `extends` compose?** `extends: [genealogy, legal]` for a memoir
  containing statutes. Single inheritance is simpler and is what phase 1 assumes.

### 7.3 Retained risk

**Narrowing can cost recall.** The ghost-prune lesson was a 7pp regression from
removing things that looked useless. A tight ontology that rejects out-of-domain
triples is the same shape of move. Mitigation is in §7.1: demote and count, do
not drop.

---

## 8. Prior art

Scanned 2026-08-25, in response to "how do open source systems determine the
appropriate ontology for a document?" Organised by the three sourcing paths the
framing doc names — **declared**, **selected**, **induced**.

### 8.1 The headline finding

**There is no mature, domain-agnostic open-source system that takes a document
and tells you which ontology fits it.** The only production-grade ontology
*detector* is biomedical-only. Everywhere else the field solved the problem by
not solving it: systems either induce a schema from the corpus, or require one
to be declared. That is worth knowing before building a selector — the
"selected" path in the framing doc has the least prior art behind it, not the
most.

### 8.2 Selected — pick from a library

| System | Scope | Licence | Relevance |
|---|---|---|---|
| [NCBO Ontology Recommender 2.0](https://jbiomedsem.biomedcentral.com/articles/10.1186/s13326-017-0128-y) | 500+ biomedical ontologies | BioPortal, open source | **The reference design.** Scoring function is portable |
| [LOV](https://lov.linkeddata.es/dataset/lov/api) | 500+ RDF vocabularies | open | Vocabulary-level, not document-level |
| [OntoLearner](https://ontolearner.readthedocs.io/) | 200+ ontologies, 20+ domains | MIT | Best ready-made *library*; selection is manual |
| KONDA ([SCI-K 2025](https://sci-k.github.io/2025/papers/paper14.pdf)) | domain- and format-agnostic | research prototype | Closest to a general detector; not production |

NCBO Recommender is the only one that really works: POST a corpus or keyword
list, get back scored ontologies *and groups to use together*. Its four criteria
are the transferable part and are better thought through than anything we would
invent:

| criterion | measures |
|---|---|
| **coverage** | how much of the input the ontology's terms annotate |
| **acceptance** | community uptake of the ontology |
| **detail** | specificity of the covering classes |
| **specialization** | how domain-focused the ontology is vs. the input |

**Take:** if we build a selector over built-in modules (`genealogy`,
`scholarly`, `legal`, `standards`, `software`, `poetics`), copy *coverage* first
— what fraction of a sampled chunk's proper-noun candidates does this module's
type set accommodate. That alone is probably enough to pick a module for 15 KBs,
and it needs no annotator infrastructure. NCBO presumes a text-span→class
annotator we do not have.

### 8.3 Declared — author it, machine-check extraction against it

| System | Format | Licence | Note |
|---|---|---|---|
| [OntoGPT / SPIRES](https://github.com/monarch-initiative/ontogpt) | LinkML YAML | BSD-3 | **Runs on ollama/litellm** — local models |
| [TrustGraph Ontology RAG](https://trustgraph.ai/guides/key-concepts/ontology-rag/) | OWL → internal JSON | not stated on page | Classes, properties, axioms incl. domain/range |
| [OMD-GraphRAG](https://arxiv.org/abs/2603.25152) | schema templates in prompt | paper | The empirical case for this whole plan |

**OntoGPT/SPIRES is the closest fit to KwaaiNet.** Schema in LinkML YAML + free
text → JSON/YAML/RDF/OWL. Two properties matter to us: it **grounds** extracted
terms through OAKlib (Gilda, BioPortal annotator, OLS) rather than trusting the
LLM, and it runs on ollama/litellm, so it works over our p2p inference rather
than requiring a hosted API. It has no template auto-selection — you pick.

**This raises a format question (now in §7):** LinkML is a real YAML schema
language with a registry and tooling, and SPIRES proves the schema → extraction
path end-to-end on local models. We should have an explicit reason for inventing
`ontology.yaml` instead, decided before phase 1 freezes the format.

TrustGraph independently arrived at exactly the triple this plan specifies —
classes, properties, axioms with domain/range. Two contrasts worth noting: its
ontology is **system-wide config, not per-KB** (the limitation we are avoiding),
and its documented answer to "where does an ontology come from" is literally
"use Claude or GPT-4 to generate one from domain text."

OMD-GraphRAG is the empirical argument for the plan: schema-free extraction in
professional domains yields "low entity recognition rates, blurred semantics,
and excessive noise," and without ontology constraints the graph is too loose to
sustain reasoning chains. Beats LightRAG on MultiHop-RAG F1, most on inference
and temporal queries. Youtu-GraphRAG's "seed graph schema" is a related
bounded-extraction approach.

### 8.4 Induced — derive the schema from the corpus

This is where the momentum and the best-engineered code are.

**[Microsoft GraphRAG auto prompt tuning](https://microsoft.github.io/graphrag/prompt_tuning/auto_prompt_tuning/)**
(MIT) is the closest working analogue to our §7 induction path, and the one to
read first. Mechanism:

1. chunk corpus into text units
2. sample them — `random` (default), `top`, `all`, or `auto` (embed, take the
   *k* nearest to the centroid)
3. infer `--domain` from the sample when not given
4. `--discover-entity-types` lets the LLM propose the type set
5. emit `extract_graph.txt`, `summarize_descriptions.txt`, `community_report.txt`

Defaults are small: 15 text units sampled, 300 embedded for auto-selection,
k=15. Microsoft's writeup describes tuning on ~1% of 10,000 chemistry papers.
**The useful datum is the budget: induction is cheap.** A
`kwaainet rag graph schema infer --kb Climate` in this shape is days, not a
research project.

**[AutoSchemaKG](https://github.com/HKUST-KnowComp/AutoSchemaKG)** (MIT, ACL
2026) is the strongest result: extract entity *and event* triples, then
conceptualize into semantic categories. Reports **92% semantic alignment with
human-crafted schemas at zero manual intervention**, validated at scale (50M
documents → 900M nodes, 5.9B edges). Caveat for us: the induced schema is
**implicit in the graph** — output is GraphML plus concept CSVs, no separate
schema document. We want a versioned artifact, so adopting it means extracting
the schema back out of its output.

**[Neo4j LLM Graph Builder](https://neo4j.com/labs/genai-ecosystem/llm-graph-builder/)**
does the pragmatic middle version: infer a schema from the input text **once**,
then use that fixed schema to guide extraction across all chunks. Three modes —
predefined schema, schema from an existing DB, or LLM-suggested. That
infer-once/apply-everywhere shape is exactly our phase 1 → 2 wiring.

**[Text2Onto](https://link.springer.com/chapter/10.1007/11428817_21)** (2005) is
old but contributes two ideas we need. It keeps a *Probabilistic Ontology Model*
carrying a confidence value on every learned object — which is the framing
doc's "require N grounded chunks per predicate", twenty years early. And it has
**data-driven change discovery**: when the corpus changes, avoid reprocessing
the whole thing. Directly relevant to dream and to the dynamic-ingest refinement.

**[LLMs4OL](https://github.com/HamedBabaei/LLMs4OL)** is an evaluation harness
rather than a system: benchmarks for term typing, taxonomy discovery, and
non-taxonomic relation extraction over WordNet/GeoNames/UMLS. If we induce
vocabularies for 15 KBs we need a way to score them. The
[2025 challenge overview](https://www.tib-op.org/ojs/index.php/ocp/article/view/2913)
reports hybrid pipelines (commercial LLM + domain-tuned embeddings) beating
pure-LLM approaches.

### 8.5 What this contributes to the bakeoff — arm C

Arm C (§5.2) is this scan's main practical output. Point GraphRAG's auto-tuner
at **Climate** and **Legal** and diff the entity types it discovers against our
17 `ENTITY_TYPES`. Two KBs, ~1% sampling, one day.

- If it independently proposes something like Quantity / Dataset / Finding for
  Climate, induction is validated before we hand-author anything.
- If it proposes noise, we have learned that cheaply and arm B stays
  hand-authored.

Note this runs *before* arm B in §5.6, not against it: an induced draft is the
cheapest starting point for a hand-authored ontology, so the two arms compose.
The measurement that matters is not "which wins" but **how much ratification the
draft needs**.

### 8.6 Ontology matching — the answer to our migration problem (§4)

Renaming a type re-IDs every node because `entity_type` is in the identity hash.
That is an ontology-alignment problem, and alignment is a mature field with an
annual evaluation (OAEI).

- [AgreementMakerLight](https://link.springer.com/chapter/10.1007/978-3-642-41030-7_38)
  — built for very large ontologies; best on the OAEI Anatomy track by F-measure.
- **LogMap** — lexical matching plus structural repair and logical consistency
  checking; strong on 100k+ class ontologies.
- [DeepOnto](https://github.com/KRR-Oxford/DeepOnto) (Apache 2.0) — BERTMap
  (learned alignment) and BERTSubs (subsumption prediction). Note it bridges to
  the Java OWL API via JPype, so it is a heavier dependency than it looks.

**Take:** alignment is how ontology v1 → v2 happens without a 5-hour rebuild per
KB. Worth revisiting when `schema migrate` is designed, not before.

### 8.7 Constraint this scan adds — and a convergence

KONDA's paper flags the hazard we would otherwise walk into: **LLM-based mapping
is not reproducible run-to-run.** Prompt for the same mapping twice, get two
answers. Since `ontology.yaml` is a *versioned, committed* artifact, induction
must produce a **draft for human ratification that is then committed** — never a
step re-run at ingest time. Recorded as a design constraint in §7.2.

This is not a new finding so much as external confirmation: the parent doc's
agentic-harness refinement already lists *ontology ratification* first among the
places a human stays in the loop — "induced predicates are proposals… wrong
ontologies are expensive to unwind." The literature and the harness design
arrived at the same requirement from opposite directions, which is a reason to
trust it.

The harness section also names **`ontology propose/apply`** among the CLI
primitives that do not yet exist. Arm C should emit exactly that command surface
rather than inventing a parallel one.

### 8.8 Reading order

1. GraphRAG auto prompt tuning docs — shortest path from "we should induce
   schemas" to a working implementation shape.
2. OntoGPT/SPIRES + LinkML — decides the §7 format question, and is the only
   system in this scan that already runs on local models.
3. AutoSchemaKG paper — the strongest evidence that induction scales, and the
   source of the 92% number.
4. Text2Onto — for change discovery and per-object confidence, both of which the
   dream loop needs.
