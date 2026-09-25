# Dream RAG: AIAS+ 2026 revision plan (submission 54)

> Internal working document. **It is not the anonymized submission.** It names the repository,
> the corpora and the result files. The anonymized manuscript and response are produced from it.
> The response draft is in [`DreamRAG-AIAS2026-response.md`](DreamRAG-AIAS2026-response.md).

**Deadline 2026-10-01**: anonymized manuscript (corrected EasyChair/ACM template),
anonymized point-by-point response, and Accepted Author registration.

**Scope decision (2026-09-25):** the paper writes up only the Rust `kwaai-rag` implementation and
experiments already run in this repo. No new DreamRAG code.

These claims from the original abstract are **removed**. Per Reza (2026-09-25), they were
aspirational when the abstract was written: planned design, not results measured on the implemented
system. The response letter says they "reflected the architecture as planned" and are withdrawn. It
does not claim they came from any prototype.

- Ebbinghaus strength tiers
- Bayesian abstention
- query-likelihood reranking (MRR 0.57 → 0.70)
- rank-AUC 0.66
- "prune 25%, keep 94%"
- *a formal* free-energy model. Free energy stays as a design heuristic: the system optimizes **fact density** (fewer, more complete entities) through confidence-based filtering at ingestion, duplicate merging, pruning and completion. Reza, 2026-09-25.

---

## Proposed title

**Dream RAG: Offline Consolidation of a Retrieval Knowledge Graph, and When It Helps**

## Revised abstract (draft, ~220 words)

Most retrieval-augmented generation (RAG) systems build their index once, at ingestion. We describe
Dream RAG, an implemented RAG system with an offline "dream" loop that optimizes the knowledge graph for
*fact density*: the same evidence represented by fewer, more complete entities. This is a practical
counterpart to the free-energy principle's trade-off between model complexity and fit to evidence.
During idle time, the loop:

- scores every entity in the knowledge graph for structural completeness;
- selects the weakest entities and completes them from their own evidence passages with a small (8B)
  language model;
- merges duplicates and prunes unsupported nodes;
- is re-run by an external scheduler; once no candidate remains, a cycle makes no model calls.

Retrieval fuses BM25 and dense search by reciprocal rank and adds entity context from the graph. We
evaluate on eleven corpora spanning history, law, science, technical standards, fiction and meeting
transcripts (0.8k–15k passages), and study the loop over time on a 1,152-passage memoir.

The loop reliably increases fact density. Over five cycles on twelve corpora (three on one), it cut
the entity count by 7–26% (median 15%), almost all of it by duplicate merging in the first cycle, and
mean completeness rose on eleven of them. On the memoir, completeness rose from 51.5% to 78.1% over 31 cycles
(about two hours) and then plateaued. Retrieval accuracy did not follow. It was uncorrelated with
completeness across cycles (Spearman ρ = 0.17, p = 0.62, n = 11), and two consolidation operations
measurably reduced it: pruning weakly evidenced but well-connected entities, and overwriting curated
descriptions. Across fifteen corpora, the number of extracted relations did not predict recall
(r = 0.08). We quantify the evaluation noise floor and conclude that structural graph metrics are not a
valid proxy for retrieval quality in consolidating RAG systems. We identify typed relation extraction
as the prerequisite for consolidation to pay off.

**Keywords:** retrieval-augmented generation, GraphRAG, knowledge-graph consolidation, offline
enrichment, fact density, free-energy principle, evaluation methodology, negative results

---

## Section outline (the response letter cites these numbers)

| § | Section | Content | Evidence |
|---|---|---|---|
| 1 | Introduction | Static indexes. The dream loop as an *operational* process. Memory consolidation and the free-energy complexity/fit trade-off serve as design heuristics, not formal models, and lead to the fact-density objective. Contributions: (a) an implemented consolidation loop that optimizes fact density, with completeness-scored candidate selection; (b) a multi-corpus study showing structure improves while accuracy does not; (c) documented failure modes and safeguards | — |
| 2 | Related work | GraphRAG (Edge et al. 2024), HippoRAG (Gutiérrez et al. 2024), LightRAG (Guo et al. 2024), RAPTOR (Sarthi et al. 2024), Self-RAG (Asai et al. 2023), MemGPT (Packer et al. 2023), reflection in Generative Agents (Park et al. 2023), complementary learning systems (McClelland et al. 1995), RRF (Cormack et al. 2009). ⟦verify every bib entry⟧ | — |
| 3 | System | ⚠ **Superseded by "Code-verified system facts" below: the chunking, confidence-filtering, scheduling, merge-protection and relation claims in this cell are wrong.** 3.1 ingestion (100-word sentence-aligned chunks, embeddings, NER + LLM entity extraction; **confidence filtering**: a candidate with type confidence < 0.60 that appears once is discarded (`axiom_extract.rs`, Axiom 6); an entity failing schema validation is demoted to 0.1, not deleted (`ingestion.rs` `validate_entities_against_schemas`); low-confidence entities get an entity-centric refinement pass (`refine_low_confidence_entities`); per-chunk entity cap 25/20); 3.2 hybrid retrieval (BM25 + dense via RRF; modes `vector` / `iterative`); 3.3 knowledge graph; **3.4 the dream loop, with Algorithm 1**: score → select → complete from evidence → merge duplicates → prune zombies → sanitize relations, cycles run on an interval or a set count, and a converged cycle makes 0 LLM calls (there is no automatic stop; `DreamConfig.interval_secs`, `dream.rs` 'Nothing to do' path); prune = score below threshold AND no chunks AND degree 0; merge = three candidate tiers: exact normalized name; Jaro-Winkler ≥ 0.60 gate, then embedding cosine ≥ 0.92 (`dedup_threshold`); name structure (`dream.rs` L943–949, `graph.rs` `find_dedup_candidates*`); 3.5 safeguards (curated-description protection on merge; improvement gate on replacement) | `dream.rs`, `dream_tasks.rs`, `graph.rs` upsert; `design/architecture/dfd-4-dream.mmd` |
| 4 | Experimental setup | 4.1 corpora (T1); 4.2 protocol: 20 questions per corpus, keyword recall on retrieved context and on the answer, an LLM judge, llama3.1:8b, top-k 20; 4.3 **noise floor** | see ledger |
| 5.1 | Cross-corpus results | F4 | Aug-4 full round |
| 5.2 | Component comparisons | T2 | memoir runs |
| 5.3 | Dreaming: fact density vs accuracy (within the memoir, ρ = 0.17; across 15 corpora, ρ = 0.28) | F2, F3, F6 | `dream_scores.json`, rebuild/overnight logs |
| 5.4 | When consolidation hurts | ghost prune; description overwrite | progress table M30–M37; r25–r38 |
| 5.5 | Cost and scale | F5, T1 | rebuild and overnight driver logs |
| 6 | Discussion and limitations | Untyped relations (every extracted relation is `associated_with`); single-run evals; keyword-overlap metric; no re-implementation of GraphRAG/HippoRAG; no corpus larger than 37k passages; synthesized descriptions not fact-checked | — |
| 7 | Conclusion | — | — |
| App. | Prompts, parameters, question-set construction | — | — |

## Figures and tables

| ID | Content | Source | Status |
|---|---|---|---|
| F1 | System overview | `design/architecture/dfd-0.mmd`, redrawn generically; stores = SQLite | ✓ `figures/` |
| F2 | Graph completeness vs eval accuracy over 31 cycles | `tests/kwaai-knowledge/results/dream_scores.json`. Drop cycle 12 (eval 0.0, failed run). Mark cycles 1–9 as the 3B model | ✓ `figures/` |
| F3 | **Fact density**: entity count vs mean completeness over 5 cycles on 12 corpora (paired before/after per corpus) | `results/rebuild_dream_*.log`, `results/warpeace_rebuild_dream_timeline.log`; cycles 6–10 in `overnight_dream_timeline_*.log` | ✓ `figures/` |
| F4 | Retrieval / generation / judge on 11 corpora | `results/multi_corpus_eval_full_driver.log` (2026-08-04) | ✓ `figures/` |
| F5 | Build time vs passages; minutes per dream cycle | (a) last progress line of each `rebuild_dream_*.log` (same builds as F3/F4); (b) `overnight_dream_timeline_driver*.log` | ✓ `figures/` |
| F6 | Completeness per dream cycle, 12 corpora (small multiples, shared y) | `Overall:` lines of `rebuild_dream_*.log` + `overnight_dream_timeline_<KB>.log` | ✓ `figures/` |
| F7 | Development history on the memoir, 88 milestones, instrument changes and dream-only / seed-only milestones marked | `projects/kwaai-knowledge/d6_progress_chart.py` `MILESTONES`; classification from `d6_accuracy_progress.md` rows and commit diffs (M83, M85–M88 touch only `d6_family_tree.yaml`) | ✓ `figures/` |
| T1 | Corpus statistics | `rebuild_dream_*.log`, `warpeace_rebuild_dream_timeline.log`, `<KB>/eval_questions.json` | ✓ drafted below |
| T2 | Controlled comparisons with n and noise band | ledger below | ✓ drafted below |
| T3 | Consolidation harms | ledger below | ✓ drafted below |

### Draft captions

All figures are produced by `figures/gen_paper_figures.py` (Figure 1: `figures/fig1_architecture.mmd`
via `mmdc`). Each is written as PDF with embedded TrueType fonts, as ACM requires, plus a PNG.

- **Figure 1.** System overview. Ingestion builds passages, a hybrid lexical-dense index and an entity
  graph; passages with no named-entity candidates are not sent to the model. Retrieval fuses BM25 and dense rankings,
  then adds graph entity context over iterative rounds. The offline dream loop rewrites the graph
  toward higher fact density.
- **Figure 5.** Structural completeness (a) and answer keyword recall (b) over 31 dream cycles on the
  memoir corpus (20 questions, 116 keywords). The shaded region marks cycles that used a 3B completion
  model; later cycles used an 8B model. Completeness rose from 51.5% to 78.1% and plateaued, while
  recall stayed within ±1 SD of its mean (54.1 ± 3.4%). Spearman ρ = 0.17, p = 0.62, n = 11. Cycle 12
  is omitted because its evaluation run failed and returned 0.
- **Figure 3.** Fact density after five dream cycles on twelve corpora: the change in entity count (a)
  and mean entity completeness before and after (b). Entity count fell by 7–26% (median 15%), and
  completeness rose on 11 of 12 corpora. 92–100% of each corpus's reduction occurred in the first
  cycle. The Manhattan Project corpus completed three cycles; the fourth failed on a text-encoding
  defect since fixed. Merging can raise completeness mechanically; see §5.3.
- **Figure 2.** Retrieval and answer keyword recall (a) and LLM-judge score (b) on eleven corpora, all
  with the same configuration (llama3.1:8b, iterative hybrid retrieval, top-k 20, 20 questions per
  corpus).
- **Figure 4.** Mean entity completeness after each dream cycle on twelve corpora, on a shared scale.
  Cycle 0 is the graph as built. Shaded cycles ran in a second, overnight session on the same graph.
  Gains are largest on the smallest graphs (meeting transcripts +4.6 points, Manhattan Project +4.4) and
  under 1.5 points on most others; two corpora dip in cycle 1, when duplicate merging removes entities.
- **Figure 7.** Answer keyword recall on the memoir across 88 development milestones, in order. Dashed
  lines mark changes to the instrument: the question set grew from 20 to 40 questions (116 to 225
  keywords); generation moved to temperature 0; the scorer added partial credit for nearby years.
  Orange: milestones whose only change was running dream cycles. Green: milestones whose only change
  was to curated seed data, several of them edited in response to specific evaluation questions.
  Every milestone was developed against these same questions, so the curve is a development history,
  not held-out accuracy.
- **Figure 6.** Cost. (a) Graph construction time against corpus size (log-log) for the twelve graphs
  of Figures 2–3, with an 8B extraction model on two commodity GPUs and four workers. Throughput ranged
  from 0.17 to 0.83 passages per second and follows entity density rather than size alone: passages
  with no named-entity candidates skip the language-model call. (b) Wall-clock time per dream cycle against graph size, with a fixed
  budget of 200 completions per cycle; small points are individual cycles, large points the medians.
  Cycle time does not grow with graph size. Its variation tracks position in the overnight run (median
  7–9 minutes for the first four corpora, 14–17 minutes for the later ones), which points to load on the
  shared inference servers rather than the corpus itself.

### Draft tables

**Table 1.** Corpora. Passages are ~100-word sentence-aligned chunks. Entities: after graph construction
→ after the dream cycles. Build: wall-clock hours for entity extraction (8B model, four workers, two
commodity GPUs). Every corpus has 20 questions; *kw* is the total number of gold keywords.

| Corpus | Domain type | Passages | Entities | Dream cycles | Build (h) | kw |
|---|---|---:|---:|---:|---:|---:|
| Manhattan Project | historical narrative | 773 | 866 → 736 | 3ᵃ | 0.9 | 79 |
| Deep-sea biology | scientific literature | 1,886 | 2,657 → 2,374 | 5 | 3.0 | 58 |
| Meeting transcripts | conversational | 1,914 | 797 → 729 | 5 | 1.1 | 59 |
| Sleep & memory papers | academic papers | 2,254 | 3,221 → 2,983 | 5 | 3.2 | 67 |
| Climate science | scientific literature | 2,747 | 2,532 → 2,006 | 5 | 3.3 | 62 |
| Legal opinions | legal | 3,723 | 3,523 → 2,962 | 5 | 3.9 | 80 |
| Python documentation | technical documentation | 3,783 | 1,233 → 999 | 5 | 1.3 | 63 |
| AI security standards | technical standards | 3,934 | 7,019 → 5,193 | 5 | 4.7 | 72 |
| Astrophysics | scientific literature | 5,638 | 7,550 → 6,453 | 5 | 7.9 | 75 |
| Internet RFCs | technical standards | 6,544 | 2,369 → 1,964 | 5 | 3.4 | 78 |
| *War and Peace*ᵇ | literary fiction | 6,451 | 4,194 → 3,689 | 5 | 8.9 | 80 |
| *Moby-Dick* & companions | literary fiction | 15,018 | 13,995 → 12,186 | 5 | 18.0 | 78 |
| Memoirᶜ | single-author memoir | 1,152 | ⟦from the 2026-05-20 build⟧ | 31 | ⟦⟧ | 116 |

ᵃ The fourth cycle failed on a text-encoding defect, since fixed. ᵇ Used for Figures 3 and 6 only; not
in the eleven-corpus evaluation of Figure 2. ᶜ Used for the dream-over-time study (§5.3) and the
component comparisons (Table 2).
⟦Domain labels are my reading of the corpus names; check them against the source documents.⟧

**Table 2.** Controlled comparisons on the memoir corpus. Answer keyword recall; llama3.1:8b generator;
*n* = evaluation runs per arm. The noise floor (§4.3): two runs on one graph differed by 7.6 points;
ten runs of pipeline A had SD 1.6. Differences smaller than these are inconclusive.

| Factor | Arm A | Arm B | n (A / B) | Question set | Reading |
|---|---|---|---|---|---|
| Retrieval routing | iterative multi-round 57.8% | single-pass automatic 39.7% | 3 / 1 | 20 q, 116 kw | large; outside noise |
| Curated seed nodes (+7 org/place) | without 54.4% (95% CI 50.7–58.0) | with 63.1% | 3 / 1 | 20 q, 116 kw | B above A's CI |
| Post-processing order | A 61.8% · B 54.2% · C 56.9% | — | 1 each (A: 10 runs, 62.9 ± 1.6) | 20 q, 116 kw | A best |
| Graph-entity context injection | on 53.4% | off 56.9% | 1 / 1 | 20 q, 116 kw | inconclusive; "on" had a merge defect |
| Domain ontology (same 17 types) | global predicates 59.3% | memoir-specific 55.5% | 1 / 1 | 40 q, 209 kw | inconclusive |
| Dream cycles (31) | completeness 51.5 → 78.1% | recall 54.1 ± 3.4% over 11 evaluated cycles | — | 20 q, 116 kw | uncorrelated (ρ = 0.17, p = 0.62) |

**Table 3.** When consolidation reduced accuracy (memoir corpus).

| Operation | Graph change | Recall before → after | n | Safeguard now in the system |
|---|---|---|---|---|
| Pruning unevidenced entities *with* relations | 1,905 → 1,544 entities; 6,164 → 3,192 relations | 56.7% → 52.6% (Welch t = 2.50, p = 0.055) | 4 / 3 | prune removes only unevidenced entities with no relations; connected ones need an explicit opt-in |
| Overwriting curated descriptions (cycle 6) | generated summaries replaced hand-curated descriptions | 166–173 → 155 of 225 kw; 178 after re-seeding | 1 per cycle | re-seeding restores them (a seeded description takes precedence on insertion); inside the loop, only the improvement gate |

**Scope: existing results only (Reza, 2026-09-25).** No new runs, including the `--mode vector` static
baseline and a before/after evaluation of the Aug-21 dream-sweep graph backups. R1.5 and R2.2 say
plainly that no static-RAG baseline was run, and that the paper does not claim to beat one.

---

## Code-verified system facts (2026-09-25, `main` at `2a19d81a`)

Manuscript §3 is written from these, not from the outline above. Claims the earlier drafts made that the
code contradicts:

- **Chunking** is a paragraph → sentence → character cascade, with at most 800 characters and 200
  overlap (`ChunkConfig::default`; the corpus pipeline passes `--chunk-strategy paragraph`). It is not
  "100-word sentence-aligned", which `projects/kwaai-knowledge/CLAUDE.md` also claims.
- **Confidence filtering (Axiom 6)** demotes a candidate so that it falls through to the LLM; it does
  not discard it. The axiomatic path is off by default (`--axiomatic-threshold 0.0`), and
  `corpus_rebuild_dream_pipeline.sh` never enables it. Schema validation and low-confidence refinement
  are off by default too. None of the three was active in these builds.
- **Relations.** Every corpus build used `--no-relations`. There were no `associated_with` relations,
  just almost no relations at all.
- **Curated descriptions** are not protected in dream merges: `merge_entity_into` keeps the longer
  description. Seed precedence applies only in `upsert_entity`. The cycle-6 fix was re-seeding.
- **Scheduling.** `DreamConfig.interval_secs` is dead config; `rag dream run` performs one cycle.
- **Prune rule** (dream step 7): mention_count ≤ 1, no relations, no chunks, and score < 0.3.
- **Merge tier 1** has no type guard. Tier 2 has no qualifier stripping (disambiguated names are
  skipped instead). The name-guard caps (0.94, 0.96) sit above the 0.92 threshold, so they never block
  a merge at default settings.
- **The score** uses the compiled per-type tables (`schema_type_for`, `expected_relation_groups`), not
  the per-KB ontology helpers in the same file.
- **Eval `--semantic-low`** defaults to 0.55; the multi-corpus script passes 0.30 explicitly.
- **Retrieval-quirk observation (not in the paper):** RRF scores are at most about 0.033, so the 0.45
  gap-fill chunks and the +0.05 term bonus outrank all Round-1 ordering.

## Evidence ledger (every number the paper or response cites)

Paths are relative to the repository root. R = `tests/kwaai-knowledge/results/`.

| Claim | Value | Source | Verified |
|---|---|---|---|
| Completeness over 31 dream cycles | 51.5 → 78.1%, flat from cycle 24 | R/`dream_scores.json` | ✓ |
| Wall-clock for 31 cycles | 17:10 → 19:13, 2026-05-20 (~2 h) | R/`dream_scores.json` timestamps | ✓ |
| Health vs eval correlation | Spearman 0.17 (p = 0.62); Pearson 0.25 (p = 0.46); n = 11 cycles with both; eval mean 54.1%, SD 3.4 | computed from R/`dream_scores.json` | ✓ |
| Same graph, two eval runs | 158/225 (70.2%) vs 175/225 (77.8%) | R/`eval_D6_r26_dream_t0_20260615_232346.md`, R/`eval_D6_r27_postdream_20260615_235134.md` | ✓ |
| Ten runs of pipeline A | mean 62.9%, SD 1.6pp. Code changed between some runs, so these are not pure repeats | R/`eval_D6_ordA_10pct_*` | ✓ (computed) |
| Three-run CI | 54.4% ± 1.5pp, 95% CI 50.7–58.0% | `plans/D6-eval-report-20260609.md` L36–41 | ✓ |
| Unpruned vs ghost-pruned graph | unpruned 53.4 / 59.5 / 57.8 / 56.0 (mean 56.7); pruned 54.3 / 52.6 / 50.9 (mean 52.6); Welch t = 2.50, p = 0.055. All Ollama backend; two of the pruned runs follow one extra dream cycle | `d6_accuracy_progress.md` rows 30–37 (no per-run eval files) | ✓ (table) |
| Ghost prune size | 1905 → 1544 entities; 6164 → 3192 relations | same, row 31 | ✓ |
| Description overwrite at cycle 6 | 155/225 (68.9%) vs 166–173 over cycles 1–5; fixed, then 178 (79.1%) | R/`eval_D6_r37_dream6_20260617.md`; r29–r35, r38 | ✓ (r37) |
| Retrieval modes, 20 q | vector 44.8 / graph 43.1 / auto 46.6% | progress rows 9–11 | table only |
| auto vs iterative | 39.7% vs 57.8% (mean of 3) | progress rows 35–38 | ✓ |
| Entity injection | on 53.4% vs off 56.9% (n = 1 each) | progress rows 40–41 | ✓ |
| Pipeline ordering | A 61.8 / B 54.2 / C 56.9% | R/`eval_D6_ord{A,B,C}_2026061*.md` | ✓ |
| Curated seed nodes (+7) | 54.4% (n = 3) → 63.1% (n = 1) | `D6-eval-report-20260609.md` | ✓ |
| Ontology A/B | 59.3% (124/209) vs 55.5% (116/209) | R/`eval2_ctl_20260826_220033.md`, R/`eval2_narr_…` | ✓ |
| Cross-corpus, Aug 4 full | e.g. Manhattan ret 91.3 / gen 75.4 / judge 1.60; Meetings 77.6 / 68.5 / 1.20; Legal 94.0 / 85.1 / 1.85 | R/`multi_corpus_eval_full_driver.log` (per-KB .md missing) | ✓ |
| Relations vs recall | Pearson +0.081, n = 15 | R/`eval_all_kbs_20260822_071707.md` | ✓ |
| Graph score vs recall, across corpora | Pearson +0.252, Spearman +0.279 (p ≈ 0.3), n = 15; iterative mode, local Ollama, token-overlap recall | R/`eval_all_kbs_20260822_071707.md` Analysis table | ✓ (p computed) |
| Dream cost, memoir | 100 completions, 4 workers, ~4.7–6 min per cycle | R/`dream_D6_cycle{1-5}_20260615_*.log` timestamps | ✓ |
| Dream cost, other corpora | 200 completions, 4 workers, 11 corpora × 5 cycles (overnight cycles 6–10): 3.0–21.1 min per cycle, median 13.0; not size-dependent (see F5 caption) | `overnight_dream_timeline_driver*.log`, via `gen_paper_figures.py` `dream_cycle_minutes` | ✓ (computed) |
| Fact density, 5 cycles × 11 corpora + 3 cycles × Manhattan | entities −7.4 to −26.0% (median −14.8%); health up on 11 of 12 (NIST 38.8 → 38.6). Astrophysics 7550→6453, 37.2→37.8; Climate 2532→2006, 40.0→40.5; DeepSea 2657→2374, 36.2→37.1; DreamMem 3221→2983, 36.8→37.4; Legal 3523→2962, 45.4→47.2; Manhattan 866→736, 39.7→43.2; Meetings 797→729, 39.3→43.2; MobyDick 13995→12186, 38.2→38.7; NIST 7019→5193; PythonDocs 1233→999, 37.5→38.1; RFCs 2369→1964, 39.1→39.7; WarPeace 4194→3689, 37.8→39.8. Caveat: merging duplicates can raise the health score mechanically (merged fields); no retrieval eval before/after | parsed from `rebuild_dream_*.log`, `warpeace_rebuild_dream_timeline.log` | ✓ (computed) |
| Health gain, other corpora | Manhattan 39.7 → 43.2; Meetings 39.3 → 43.2; Legal 45.4 → 47.2; NIST 38.8 → 38.6; about +0.1pp/cycle on cycles 6–10 | `rebuild_dream_*.log`, `overnight_dream_timeline_*.log` | Manhattan ✓ |
| Largest build (consistent round) | Moby-Dick & companions: 15,018 passages, 18.0 h, 0.23 passages/s; all 12 builds 0.17–0.83 passages/s | last progress line of `rebuild_dream_MobyDick.log` etc. | ✓ |
| Cycle-1 share of entity reduction | 92–100% on all 12 corpora (e.g. Astrophysics 7550 → 6454 in cycle 1, then 6453; MobyDick 13995 → 12186, then flat). Later cycles: completeness +0.1 to +0.5 pp each, up to +1.9 (Meetings, cycle 1) | `Overall:` lines of `rebuild_dream_*.log` | ✓ (computed) |
| Manhattan completed 3 dream cycles, not 5 | cycle 4 panicked: `byte index 8000 is not a char boundary` in `dream_tasks.rs` `trim_evidence`; fixed on `main` (the `is_char_boundary` loop) | `rebuild_dream_Manhattan.log` | ✓ |
| Build panics in Astrophysics, Climate | one PDF each failed in `cff-parser` at ingestion; the dream loop ran all 5 cycles | same logs | ✓ |
| Replacement gate, no fact check | completions conditioned on the entity's evidence chunks; replaced only if the tier/length improvement gate passes; curated descriptions protected on merge | `dream.rs` L45–58, L483–491; `graph.rs` L1059–1063 | ✓ |

**Do not cite** (no backing file, or wrong):

- the 10% "mini-loop" at 52.0%
- the 2026-07-23 relation-summary evals
- "7pp" for ghost prune (it's the best single run against the pruned runs)
- "M30 pure dense 56.9%" (that run had injection off, not pure dense)
- the July report's "Manhattan 20 chunks"
- the RAGPerformanceReport chart2 range 73.9–96.3%, which mixes three scorers
- "WarPeace 37,486 passages, 28.9 h". The July KB really held 37,486 chunks (`reembed_WarPeace_20260715_*.log`),
  but the August rebuild of the same single `warandpeace.pdf` gives 6,451, so the July KB almost certainly
  held repeated ingests. The 28.9 h has no log either. Use the August builds (Table 1)
- any July-report build time or chunks/s (0.2–3.7 c/s); they are not the graphs of F3/F4
- "build time scales roughly linearly with corpus size" (F5a: 3,783 passages → 1.3 h, 3,934 → 4.7 h)
- "five dream cycles on twelve corpora" without the Manhattan exception
- the v0.4.49 mode runs (44.8 / 43.1 / 46.6%) as "the static baseline": they predate iterative retrieval,
  NER pre-screening and the current graph, so they baseline an older system

## Anonymization checklist

- [ ] No author names, affiliations, acknowledgments, ORCID, emails or funding.
- [ ] No product or network names (the project, `p2p://`, peer IDs, machine names).
- [ ] No repository or Overleaf links. The public prototype README links the Overleaf draft, so don't cite it.
- [ ] Refer to the memoir only generically ("a 1,152-passage single-author memoir"); no title or author.
- [ ] Neutral file names; strip PDF/DOCX metadata (author, company).
- [ ] Cite prior work in the third person.
