# Arm B — foundation-model-authored per-corpus ontologies

Authored 2026-08-25 for [`PerKBOntology-plan.md`](../../../projects/kwaai-knowledge/plans/PerKBOntology-plan.md) §5.2 arm B.
Grounded in sampled corpus text from each KB, not in priors about what a
"climate ontology" should look like — the sampling is what turned up the
ingestion problems recorded below.

Format is the plan's §2.1: `entity_types` with `irreducible` flags and fields,
`relation_types` with domain/range/inverse/symmetric/transitive/acyclic,
declarative `axioms`, and `streams:` sections.

## Correction, 2026-08-25 — modules were applied before the sharing was verified

Reza asked why the narrative-history module was giving D6 a `Vessel` type, and
whether vessels are prominent in the D6 text. They are not. Measured share of
chunks containing vessel language:

| MobyDick | D6 | WarPeace | Manhattan | CountryHistory |
|---|---|---|---|---|
| **31%** | 1% | 0% | 0% | 0% |

It was a MobyDick type placed in a shared module on genre reasoning
("maritime history treats vessels as agents"), and four corpora inherited it for
nothing. Moved to `MobyDick.yaml`.

The larger fault it exposed: **the modules were authored top-down from genre
priors, and D6's ontology was written as a thin extension of one rather than
derived from D6's text.** Reading the corpus properly turned up
`Community` (caste and religious identity governing who may marry whom — "Jaht
47… from a particular community and could never marry"), `School`, `Language`
(the grandfather's two wives insulting each other in Urdu and Afrikaans,
each incomprehensible to the other, deliberately), and `Address` (the memoir is
an account of a demolished neighbourhood, so the individual street address is
the unit of loss). A genre module contains none of those.

`D6.yaml` is rewritten from its own text and now extends only `genealogy`.

**Still outstanding:** `WarPeace`, `Manhattan` and `CountryHistory` remain thin
extensions of the narrative module and have not had this treatment. They should
be assumed to carry the same fault until each is derived from its own corpus.

## A KB has several ontologies, not one

Reza, 2026-08-25, settling the parent doc's open question. `streams:` is
therefore load-bearing here rather than a reserved slot. Corpora where sampling
showed genuinely different document kinds:

| KB | streams |
|---|---|
| Astrophysics | `mission-history` (Apollo programme) + `cosmology` (Planck) — two corpora in one KB |
| NIST | `normative-standard` + `threat-research` + `incident-report` |
| RFCs | `normative` + `rationale` + `diagram` (skip) |
| _scholarly_ | `body` + `bibliography` (citations only) + `front-matter` (skip) |
| _software-docs_ | `reference` + `prose` + `chrome` (skip) |
| Poems | `verse` + `apparatus` (skip) |
| Meetings | `utterance` + `cue-metadata` (skip) |

The `scholarly` split is the important one. Climate's existing graph is
Publication 1163 / Organization 655 — it modelled the bibliography and the
funding acknowledgements, not the science. Separating body from references is
what stops that.

## Layout

Three shared modules, then thin per-KB files that extend them:

| module | covers |
|---|---|
| `_module_narrative_history.yaml` (extends `genealogy`) | D6, WarPeace, MobyDick, Manhattan, CountryHistory |
| `_module_scholarly.yaml` | Climate, DeepSea, DreamMem |
| `_module_software_docs.yaml` | PythonDocs, OSMDocs |

Standalone: `Legal`, `NIST`, `Astrophysics`, `RFCs`, `Poems`, `Meetings`.

Five corpora sharing one narrative vocabulary with per-corpus deltas of 1–2
types is the point of modules, and it is also what arm D's selector would pick
between.

## Apply order — only three are worth applying now

Cross-referencing `results/corpus_hygiene_*.md` with relation density:

| KB | clean | rel/ent | status |
|---|---|---|---|
| **Legal** | 96.9% | 0.0007 | **apply first** — cleanest genuinely vocabulary-bound corpus |
| **Astrophysics** | 94.2% | 0.0036 | **apply** |
| **NIST** | 90.7% | 0.0014 | **apply** |
| D6 / WarPeace / MobyDick / Manhattan | 86–99.7% | 0.02–0.23 | already productive; use as regression guard |
| Climate, DeepSea, DreamMem, CountryHistory, RFCs, Poems, PythonDocs, OSMDocs, Meetings | 2–75% | ~0 | **blocked on ingestion** |

Files for blocked KBs carry a `BLOCKED ON INGESTION` header. They are authored
so the work is not lost, but applying an ontology to a corpus that is 68%
character-spaced PDF or 98% WebVTT scaffolding measures the dirt, not the
ontology.

## Design decisions worth arguing with

- **`fallback_predicate: null` everywhere except narrative-history.** The parent
  doc asked whether `associated_with` is ever legitimate. Answer taken here:
  yes for narrative, where genuine untyped association exists; no for law,
  standards, science, poetry and software docs, where a vague edge is a bug.
- **`irreducible: true`** on Holding, ConstitutionalProvision, Statute, Control,
  Requirement, Finding, Constraint, Image, Decision, ActionItem, Tag. These are
  the things that must survive compression verbatim.
- **Every module declares axioms** — contradiction pairs, acyclic and functional
  constraints — so the sanitiser has something structural to check instead of
  the gender heuristic that deleted valid D6 relations.
- **Temporal validity is deliberately absent.** Meetings needs it most (a
  superseded decision was still true last week). Deferred to TOKI's bitemporal
  model per plan §2.1 rather than invented here.
