# DreamRAG Eval v2: does graph completion improve retrieval and answers? (re-run plan)

## Context

The AIAS+ 2026 paper (4 pages, due **Thu 2026-10-01**) reports that dream consolidation raises graph
completeness but not accuracy. The evidence is weak on three counts:

- **The gold answers are wrong.** Each per-corpus `expected_answer` is a Mistral-7B RAG output (the
  OpenWebUI column of the QA-Tracker XLSX files), and it contains errors.
- **The measurement is circular and coarse.** The only judge is llama3.1:8b grading its own answers,
  and scoring is keyword overlap.
- **The design can't answer the question.** No per-cycle graph snapshots exist, the memoir series has
  n = 11, and cross-corpus correlations are confounded by domain.

We invest the next ~4 days to re-measure. Retrieval is judged per passage by Claude, which is
independent of the generator. Answers are judged by deterministic NLI nugget entailment against
verified NotebookLM gold. Two dream arms isolate the paper's own claim that relations are the
prerequisite:

- **Arm A:** `--no-relations`, as in the paper.
- **Arm B:** relation completion on.

Either outcome strengthens the paper: a detected effect, or a pre-registered equivalence result
("no effect larger than δ").

User decisions (fixed): Claude as the retrieval judge; NLI nugget entailment for answers; NotebookLM
gold, verified against source; Oct 1 target with both arms.

## Facts that shape the design (verified 2026-09-25)

- `graph build --reset-graph` wipes entities and relations but **keeps chunks** (`cli.rs`, the
  `reset_graph` flag). Chunk ids are therefore stable across every snapshot, and gold passages can
  anchor to them.
- Each KB's graph is one SQLite file, `~/.kwaainet/rag/<KB>/graph-<tenant>.db`. Dream writes only
  this file.
  - **Snapshot:** `sqlite3 src ".backup dst"` while no kwaainet process holds it.
  - **Clone a KB:** copy the directory and add a `rag_kbs` entry in `~/.kwaainet/config.yaml` with the
    same `tenant_id` and a new `rag_data_dir`. D6_ctl and D6_narr are the precedent.
- `cmd_eval` (`rag_cmd.rs`) does not use `QueryCache`, so a graph swap is safe. It keeps only doc
  names and scores (local `struct Row`) and writes no machine-readable output. That is the one Rust
  change needed.
- The graph reaches retrieval (`kwaai-rag/src/iterative.rs`) through entity-embedding seeds and a
  2-hop `bfs_neighbors`, then `entity_chunks`, plus `inject_entity_descriptions` fact cards. With
  no relations the BFS adds nothing, so **arm A can act only through descriptions and embeddings.**
  Arm B is the only arm that can widen what the graph retrieves.
- QA-Tracker XLSX files (`/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus/Corpus_Final_Review/QA-Trackers/`)
  have columns `Question | NotebookLM Answer | OpenWebUI Answer`. There are 20 rows each, and
  NotebookLM is filled for Manhattan, PythonDocs, DeepSea and DreamMem. **Meetings has no tracker**,
  so it is excluded.
- `tests/kwaai-knowledge/corpus_rebuild_dream_pipeline.sh` holds each KB's doc paths and entity
  types, but also the **dead** metro-linux id. Use `12D3KooWA33TMz7ss8K2oPQr99KsLwW6AbLYhrL7P1V36vJXXM38`
  (metro-linux) plus metro-win `12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE`. Both are A5000
  24 GB cards. Run one inference job per GPU at a time; concurrent jobs crash it.

## Experimental design

**Corpora.** The primary four are public: Manhattan (0.9 h build), PythonDocs (1.3 h), DeepSea
(3.0 h) and DreamMem (3.2 h), with 80 questions in total. The memoir (D6, 40 questions, ~35 min
build) is secondary and scored **locally only**; it is a development set, reported separately.
Climate is a stretch goal.

**Arms and snapshots.**
1. Clone each KB to `<KB>_e2` and run `graph build --reset-graph` with its entity types. This is
   cycle 0, shared by both arms.
2. Clone that to `<KB>_e2A` (`--no-relations`) and `<KB>_e2B` (relations on). Dream with
   `--max-completions 200 --workers 4`, alternating A1, B1, A2, B2, … so that drift over time hits
   both arms equally.
3. Snapshot at cycles **0, 1, 2, 4, 8, 12**, with 16 as a stretch for Manhattan and D6. Record
   `graph score` Overall **and each pillar**, because arm B's completeness rises through the relation
   pillar.
4. Evaluate each snapshot by restoring it into `<KB>_e2E`, pinned to one peer per corpus.

**Controls.** For each corpus, add two more cycle-0 evaluations (test–retest noise floor) and one
`--mode vector` run at cycle 0 (graph-free baseline). That also gives the paper the static-RAG
comparison reviewers asked for.

**Metrics.**
- **Primary (deterministic): context nugget coverage.** The fraction of reachable gold nuggets
  entailed by at least one passage in the prompt, using NLI with passages windowed at ~350 tokens and
  stride 128.
- **Key secondaries (Holm-corrected):**
  - answer nugget recall (answer ⊨ nugget);
  - Claude-graded nDCG@20.
- **Other secondaries:** faithfulness (each answer claim sentence ⊨ context), contradiction rate,
  P@5/P@20, gold-passage recall@20, retrieved-set Jaccard against cycle 0 (the mechanism check), and
  the old keyword recall for continuity with the paper.
- **The Claude judge** grades each (question, passage) pair 0–2 against the nuggets: 2 = states a
  nugget, 1 = partial, 0 = irrelevant. Output is JSON `{grade, nugget_ids, rationale}`, and grades
  are cached by (qid, sha256 of the normalized passage, model, prompt version).

**Statistics** (pre-registered in `eval2/prereg.md` and committed before the first trajectory eval
finishes):
- **H1 (arm B):** the mean per-question Δcoverage from cycle 0 to cycle 12 is greater than 0.
- **H2 (dose–response, each arm):** a mixed model `Y_qs = β·ΔC_s + u_q + γ_corpus + ε` (statsmodels
  MixedLM). CIs come from a bootstrap that resamples questions within each corpus. A sensitivity run
  uses the type and content pillars only.
- **H3 (arm A):** TOST equivalence within ±δ, where δ = max(minimum detectable effect, 2 × test–retest
  SD). With 120 paired questions per arm, the minimum detectable effect is about 4 coverage points.
- **Reporting:**
  - Per-corpus Spearman values are descriptive only.
  - The paper must say the effective unit is about 10 trajectories.
- **Blinding:** the analysis code is developed on the noise-floor data only, then run once on the
  trajectories.

## Implementation units

**U0 — Housekeeping.**
- Copy this plan to `projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md`, per the plan-storage
  rule.
- Back up the current `graph-*.db` files for the KBs involved.
- Never mutate the original KBs; all work happens on `_e2*` clones.

**U1 — Rust: `rag eval --dump-jsonl <FILE> [--run-tag]`.**
- **Flags:** in `cli.rs` (`RagAction::Eval`), threaded through the dispatch into `cmd_eval`.
- **Record type:** a new module `kwaai-cli/src/eval_dump.rs` defines `EvalDumpRecord` and
  `normalize_hash()`. After `rows.push(Row{..})`, append one line and flush.
- **Each record holds:**
  - schema_version, kb, model, mode, top_k, peer;
  - graph entity and relation counts (proof of which snapshot was loaded);
  - qid, question, answer, latency;
  - `retrieved[]` with `{rank, chunk_id|null, synthetic, doc_name, chunk_index, section_name, text, text_sha256, score, rerank_score, in_prompt}`;
  - the rendered messages;
  - the existing keyword scores.
- **Prompt manifest:** add `build_chat_messages_with_manifest` in `kwaai-rag/src/prompt.rs`, which
  returns which chunks survived the 24 000-character budget, to fill `in_prompt`.
- **Regression tests:**
  - the manifest matches the rendered prompt when the budget overflows;
  - a synthetic card serializes as `chunk_id: null, synthetic: true`;
  - hashes match `tests/kwaai-knowledge/eval2/fixtures/hash_vectors.json`, which pytest shares.
- **CI checks:** `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test -p kwaainet`.
- **Binary:** build it, codesign it, and install it as `~/.cargo/bin/kwaainet-eval2`, so the running
  daemon's binary is left alone.

**U2 — Python package `tests/kwaai-knowledge/eval2/`.**
- **`common.py`**: the KB registry (doc paths and entity types ported from the pipeline script),
  normalization and hashing, and a `PUBLIC_KBS` allowlist. The Claude judge hard-fails on D6.
- **`gold/`**:
  - `xlsx_import.py`: openpyxl; strips `[n]` markers; fuzzy-matches each row to
    `eval_questions.json` and flags misses.
  - `nuggetize.py`: Claude with structured output; each nugget is atomic, self-contained, and tagged
    core or supporting; at most 8 per question.
  - `verify.py`: reads the KB's `<tenant>.db` read-only, reusing the SQLite readers in
    `tests/kwaai-knowledge/dump_samples.py`; takes BM25 + embedding top-30 candidates; runs NLI.
    - ≥ 0.9 → verified; ≥ 0.7 → gold passage;
    - 0.3–0.9 → Claude adjudicates;
    - unsupported → *unreachable*, excluded from the primary endpoint and counted.
  - `review_cli.py`: the author reviews flagged nuggets and a 10% audit.
  - `export.py`: writes a new `tests/kwaai-knowledge/<KB>/eval_questions_v2.json` with `gold_answer`,
    `nuggets[{id, text, core, status, gold_passages[{chunk_id, doc_name, chunk_index, text_sha256}]}]`.
    The originals are left untouched.
  - **D6:** nuggets are drafted by a local model from the authored answers and reviewed by the
    author. Nothing is sent to the API.
- **`judge/relevance.py`**: the Anthropic Batches API with model `claude-opus-5-5`.
  - Keep the rubric stable at the front of the prompt so prompt caching works.
  - Keep a SQLite cache.
  - On `stop_reason == "refusal"`, resubmit synchronously.
  - Load the `claude-api` skill before writing it.
  - Estimated cost: well under $100 for about 5–8k unique pairs.
- **`nli/scorer.py`**:
  - transformers on MPS with `MoritzLaurer/DeBERTa-v3-large-mnli-fever-anli-ling-wanli`;
  - hypotheses split into sentences, premises windowed;
  - cached by (premise hash, hypothesis hash);
  - threshold 0.5 with sensitivity checks at 0.3 and 0.7, calibrated on about 150 author-labelled
    pairs.
- **`driver/run_experiment.py`**:
  - a resumable job DAG with two resource tokens, win and linux;
  - builds and dreams take both tokens, evals take one;
  - handles clone, build, snapshot and restore (checking that no process holds the DB and recording
    each snapshot's sha256), dream, score (Overall plus pillars), and eval via `kwaainet-eval2`
    `--dump-jsonl`;
  - writes `state.json` and `results/eval2/eval2_progress.json`, per the long-process telemetry rule;
  - saves per-question results for every eval, per the eval-logging rule.
- **`metrics.py`, `analysis/analyze.py`, `analysis/figures.py`**: reuse the pairing logic in
  `tests/kwaai-knowledge/dream_plot.py` and the figure style of
  `projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py`.

**U3 — Paper.** Update `papers/aias2026/DreamRAG-AIAS2026-short.md` §3–4. Replace or supplement
Figure 1 with the coverage-vs-cycle plot for both arms, and Table 1 with the H1–H3 results. Then
update the letter:
- R1.2, R1.6 and R2.3 get the new evidence;
- R2.2 gets the graph-free baseline;
- R1.6 gets the independent judge;
- R1.10 gets the faithfulness result.

Rebuild with `build_manuscript_docx.py --short`, and keep the paper at 4 pages (check with the Word
PDF export).

## Budget and schedule (Fri Sep 25 → Thu Oct 1)

| Block | Wall time on two GPUs |
|---|---|
| Graph builds, 5 corpora | ~9 h |
| Dream cycles: 5 corpora × 24 cycles × ~13 min, +30% for arm B | ~30 h |
| Evals: 70 in total | ~9 h |
| **Total** | **~48 h**, about 60 h of slack before compute stops at Wed 30 08:00 |

- **Fri 25:** U0; minimal driver (clone, build, snapshot); start the overnight builds for D6,
  Manhattan, PythonDocs, DeepSea and DreamMem; XLSX import and nuggetization.
- **Sat 26:** dream cycles start at dawn (snapshot only). U1 plus tests and CI by midday, then rebuild
  the binary. Commit `prereg.md`, then start the evals. Build gold verification and the NLI scorer.
- **Sun 27:** the author reviews gold (~2–2.5 h, plus ~1 h for D6); freeze gold v1. Submit judge
  batches as dumps arrive. Run the noise-floor repeats. Write the analysis on noise-floor data only.
- **Mon 28:** dreams reach cycle 12; remaining evals; NLI overnight.
- **Tue 29, minimum-viable-result gate at 22:00:** Manhattan, PythonDocs and D6, both arms, cycles
  {0, 2, 8}, with the noise floor, coverage and answer recall, and H1–H3. Stretch runs go overnight.
- **Wed 30:** stop compute at 08:00. Run the analysis once. Produce figures, judge κ, and the results
  draft (U3).
- **Thu Oct 1:** final paper and letter edits; confirm 4 pages; submit.

**If a peer is down:** run on the one remaining peer with `--workers 2` and log the deviation. Cut in
this order: cycle 12, then DreamMem, then DeepSea. Never mix Mac Ollama into a trajectory; a whole
corpus may run there instead.

## Risks

- **Privacy:** the allowlist blocks D6 from the API.
- **NotebookLM gold errors:** verification plus the *unreachable* status.
- **NLI on long text:** windowing, calibration and sensitivity bands.
- **Build nondeterminism:** one build per corpus, shared by both arms; stated as a limitation.
  Stretch: repeat Manhattan's arm-A trajectory.
- **Arm A may not re-embed on the description path:** confirm in `dream.rs` (the completion write
  path re-embeds) before starting. If it doesn't, arm A cannot move retrieval, and that becomes a
  finding.
- **Overrun:** the minimum-viable-result gate above.

## Verification

- **U1:** the Rust tests pass with the CI trio. On Manhattan cycle 0, one `kwaainet-eval2 rag eval
  --dump-jsonl` run produces 20 records whose `in_prompt` chunks reproduce the rendered messages, and
  whose graph counts match `graph stats`.
- **Snapshots:** the restore round-trips (matching sha256, and `graph score` identical before and
  after). Originals are unchanged (their mtimes are checked).
- **Gold:** every question has at least one verified core nugget, or is flagged. The audit precision
  of auto-verified nuggets is reported. D6 sends no API calls (the allowlist test).
- **Judge:** weighted κ against 100 author-graded pairs. The cache hit rate is logged.
- **NLI:** the calibration set's accuracy is reported, and thresholds are fixed before the trajectory
  analysis.
- **End to end:** the MVR run on Manhattan cycles {0, 2} for both arms produces every metric, all
  three hypotheses, and the figures, before the full run is trusted.

## Deviations log (append-only)

- **2026-09-26 — GPU peers offline.** metro-linux (`12D3KooWA33TMz7s…`) and metro-win are not in
  the DHT (`p2p peers connect` → `dht_find_peer` failed; `shard chain` lists only two of Darren's
  nodes). The author is bringing them back. Meanwhile the Manhattan cycle-0 build runs on the Mac
  (M4 Pro, local Ollama llama3.1:8b: 35–43 tok/s generation, ~340 tok/s prompt; ~28 s per chunk
  with 2 workers, so ~6 h for Manhattan). The cycle-0 build's hardware does not enter any
  within-trajectory comparison; evals stay pinned to one machine per corpus.
- **2026-09-26 — PythonDocs replaced by Climate.** 80% of PythonDocs chunks are HTML-heavy (56% of
  all chunk text is markup); Manhattan 25% (13%); DeepSea, DreamMem, Climate and D6 are clean. The
  HTML ingestion path keeps markup: an ingestion defect, not fixed here so that the system under
  test is unchanged. The NLI scorer reads `corpus.clean_text()` (tags, entities and URLs stripped).
  Earlier PythonDocs results in the paper were measured on this markup-heavy text.
- **2026-09-26 — Judge and nuggetizer model: `claude-opus-5`** (the SDK skill's default; Opus 5.5
  only when named), with server-side `fallbacks: "default"` on synchronous calls.
- **2026-09-26 — Text hash is SHA-1** of whitespace-collapsed text (`eval_dump::text_hash`,
  `common.text_hash`), not SHA-256: `sha1` is already a normal dependency of the CLI, `sha2` only a
  dev-dependency. Pinned on both sides by `eval2/fixtures/hash_vectors.json`.
- **2026-09-26 — Primary corpora are now Manhattan, DeepSea, DreamMem and Climate** (80 questions);
  D6 stays secondary and local-only.
- **2026-09-26 — `--reset-graph` clears graph metadata.** It wiped `doc_metadata` and
  `document_titles` from the clone's graph; `rag eval` builds its "Document being queried" preamble
  from them. `driver/build.py` now copies both keys back from the original graph before the c00
  snapshot. The first Manhattan build was restarted.
- **2026-09-26 — No per-KB ontology is loaded on any experiment KB** (only `D6_narr` has one).
  Ontologies drive extraction only (`ingestion.rs`, `relation_extract.rs`; CLI `--entity-types`
  overrides them); the dream loop and `scorer.rs` ignore them — the score path uses the compiled
  per-type tables. Left as-is so the system matches the paper; the scorer's fixed expectations are a
  stated limitation of completeness as the predictor.
- **2026-09-26 — GPU peers are back** (checked over p2p: both in the DHT, ~25 ms RTT, both serve
  inference). The earlier "offline" verdict came from a node that had only just started. Builds
  moved to both peers (`--workers 4`); Manhattan ETA ~20 min instead of ~6 h on the Mac.
