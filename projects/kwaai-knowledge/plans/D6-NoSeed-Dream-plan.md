# D6 without seed data: does recall rise with dream cycles? (first-10% trial)

## Context

Eval v2 found that D6 recall (nugget coverage) *fell* as dreaming added relations:
arm B −0.064 at cycle 12. Three things confound that result:

1. **Seed data.** Eval v2 re-seeded D6 from `tests/kwaai-knowledge/d6_family_tree.yaml` after the
   build (`eval2/driver/build.py → _run_build`, the `graph seed` call). The seed adds 216 family
   relations, merges aliases, sets fields, and plants **32 hand-written descriptions**, some of
   which state eval answers outright. An example is Ayesha Rassool: "mother of the memoir's
   author". Cycle 0 therefore starts near the ceiling, and dreaming can only disturb it.
2. **Retrieval bug.** `retriever.rs → retrieve_graph_anchored` builds its graph-chunk list from
   `graph.entity_chunks()`, which returns a `HashSet` in random order. The 1.0/0.6 scores are never
   sorted, and `rrf_merge` uses only list position. More relations mean a bigger, randomly ordered
   list, and that scrambles retrieval. On Manhattan, arm B's prompt overlap with cycle 0 fell to
   Jaccard 0.51, against 0.84–0.87 for repeats.
3. **Description overwrite.** In `dream.rs` field completion, any structured field makes
   `description_from_fields` replace the prose description with "Name — key: value; …".

Reza's decisions (2026-09-28):
- D6 only.
- Drop the seed.
- Test on the **first 10%** first.
- Restrict the **graph only** to the slice.
- **Fix both bugs** first.
- **Both arms**.
- **24 cycles**.

Hypothesis: without seeding, and with the bugs fixed, arm B's coverage rises with dream cycles.

## Design

- **Slice:** D6 chunks with `chunk_index` 0–114, i.e. the first 10% of 1,152, in document order.
  This covers pages iii–xi plus 2–15. The graph is built from these chunks only; vector and BM25
  search still cover the whole book (same KB and chunk ids, so Eval v2's gold passages remain
  valid).
- **Gold:** nuggets whose NLI-verified gold passage has `chunk_index < 115`. That gives
  **108 nuggets across 31 questions**. The eval runs only those 31 questions.
- **Cycle 0:** `graph build --no-relations --graph-window 1 --reset-graph --sample-pct 10` with
  Eval v2's five entity types, llama3.1:8b, and **no `graph seed`**. D6 stays on this Mac.
- **Arms:** A = `dream run --no-relations`; B = relation completion. 200 completions per cycle,
  24 cycles each. Every cycle is snapshotted and its graph score logged.
- **Evals:**
  - Checkpoints: cycles 0, 1, 2, 4, 8, 12, 16, 20 and 24.
  - Repeats: cycle 0 three times, and cycle 24 twice per arm, for the noise floor.
  - Settings: iterative mode, `-k 20`, `--dump-jsonl`, as in Eval v2.
  - Plus one vector-only run at cycle 0.
- **Pre-registered verdict** (written before the first eval, in the tracked plan):
  - **Primary:** coverage on the 108 slice nuggets.
  - **Pass** if arm B's paired per-question Δcoverage from cycle 0 to 24 has a 95% bootstrap CI
    above 0 and exceeds the cycle-0 retest spread.
  - **Secondary:** arm B − arm A at cycle 24; Spearman correlation of graph score against
    coverage across checkpoints (descriptive); answer recall; keyword recall.

## Steps

1. **Plan file.** Copy this plan to `projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md`,
   tracked, with the verdict criteria above.
2. **Branch.** Create `fix/d6-noseed-dream` from `feat/eval-dump-jsonl`. That branch carries the
   local-only `2b582428`, which is needed for `--dump-jsonl`. Confirm with
   `git branch --show-current`. Leave the uncommitted Eval v2 work and the `GLOSSARY.md` edit as
   they are; don't sweep them into this branch.
3. **Three fixes, each with a regression test** (bug-driven tests rule):
   - **Sample order.** `rag_cmd.rs`, the graph build's `sample_pct` / `limit` truncation. Sort
     `all_chunks` by `(doc_name, chunk_index)` before truncating, so "first N%" means document
     order, as the help text says. `MetaStore::all_chunks` orders by hashed key. Put the sort in a
     small helper and test it.
   - **Graph-chunk ranking.** `retriever.rs → retrieve_graph_anchored`, step 4.
     - Sort the graph chunks by score, descending: seed-entity chunks (1.0) before neighbour
       chunks (0.6).
     - Break ties by how many BFS entities the chunk mentions, then by chunk id, so the order is
       deterministic.
     - Extract this as `rank_graph_chunks`. Test that seed chunks come first and that two calls
       give the same order.
     - **Found during implementation:** `iterative.rs`, the mode the eval runs, had the same
       unsorted list in round 1. Its round-2 gap-fill also added *every* chunk in the gap entities'
       2-hop neighbourhood at score 0.45, above every round-1 RRF score (about 0.02–0.03), so a
       denser graph pushed its neighbourhood ahead of the vector hits. Reza chose (2026-09-28) to
       rank the gap-fill the same way and **cap it at 10 chunks** (`GAP_FILL_MAX`), the same bound
       as round 3's level-2 expansion. Both paths go through `retriever::rank_traversed_chunks`.
   - **Keep prose descriptions.** `dream.rs`, the field-completion `new_desc` selection.
     - When the existing description holds prose, keep the prose and append or refresh the
       field-summary line (`"{name} — …"`).
     - Make this idempotent across cycles: no repeated summary lines.
     - Put it in a helper next to `graph.rs → description_from_fields` and test it.
     - Check whether `graph.rs`'s reembed path (the `description_from_fields` call near "Reembed
       uses description_from_fields") overwrites in the same way; if so, use the same helper there.
4. **CI checks.** `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test -p kwaainet -p kwaai-rag`.
5. **Review.** Fresh-context review of `git diff 2b582428...` with `/code-review`. Fix what it
   finds.
6. **Install the binary** as `~/.cargo/bin/kwaainet-d6s`, then run
   `codesign -s - --force ~/.cargo/bin/kwaainet-d6s`. Don't replace `kwaainet`, which the daemon
   runs.
7. **Driver.** `tests/kwaai-knowledge/eval2/pilot/d6_noseed.py`, modelled on
   `pilot/ontology_pilot.py` (resumable `step()`, `progress.json`, `state.json`).
   - Reuse `driver/build.py` (`clone`, `graph_score`, `restore_metadata`, `sqlite_copy`) and the
     restore/snapshot logic from `driver/run_experiment.py` (`restore_graph`, `do_dream`'s
     report checks). Don't fork them.
   - Clones: `D6_s10` (build), `D6_s10A` / `D6_s10B` (arms), `D6_s10E` (evals). Nothing named
     `_e2*`, so Eval v2's KBs are untouched.
   - Output: `results/d6_noseed_s10/`.
   - Slice gold: write `D6_s10_gold.json` (the filtered nuggets) and a 31-question questions file.
     Score with `metrics.py`, pointed at them via `EVAL2_METRICS_OUT` and a gold override, not by
     editing the Eval v2 gold. Extend `metrics.base_kb` so `D6_s10*` maps to the slice gold.
   - **Run order:** a strictly sequential queue (one heavy local job at a time, per the Mac OOM
     lesson):
     1. build
     2. cycle-0 evals
     3. arm A's 24 cycles, with evals at the checkpoints
     4. arm B's 24 cycles, the same way
     5. final repeats
     6. NLI scoring, last, so the NLI model and Ollama never share memory.
   - Before each job, check free memory and disk; pause if free memory is under 20% or free disk
     under 15 GB.
8. **Analysis.**
   - `analysis/analyze.py`-style numbers: paired bootstrap Δ against cycle 0, and the retest
     spread.
   - A republished recall-vs-cycles artifact for the slice, with the graph score overlaid (reuse
     the existing chart page's generator).

## Time (this Mac, sequential)

- Build: about 1 h (Eval v2: 8.9 h for 1,152 chunks).
- Dreaming: 48 cycles, roughly 5–10 min each, since about 150 entities fit in 200 completions.
- Evals: 23 runs of 31 questions, about 15 min each.
- NLI scoring: about 1 h.
- **Total: about 13–15 h.** Can run overnight.

## Verification

- Unit tests for the three fixes pass. The clippy/fmt/test trio is clean.
- **Slice check:** after the build, every chunk that mentions an entity has `chunk_index < 115`,
  checked read-only against the graph DB. There are 0 relations and no seeded aliases (e.g.
  "Grandpa" isn't an alias of Haji Joosub Maulvi Hamid Gool).
- **Determinism check:** two graph-mode retrievals of the same query on the same snapshot return
  identical chunk lists (this failed before fix 2).
- **Description check:** after arm B's cycle 1, sample cards keep their prose plus exactly one
  field-summary line.
- **Pilot smoke run:** build, one dream cycle per arm, and one eval, before starting the full
  queue.
- **Monitoring:** step results, failures, and memory or disk warnings
  (`tail -F … | grep --line-buffered …`, no `cut` in the pipe).

## Result (2026-09-29, first-10% trial)

Run: 2026-09-28 19:04 → 2026-09-29 07:00 on this Mac, binary `9e7349eb`.
- **Graph:** 116 chunks, no seed.
  - Arm A (`--no-relations`) stayed at **0 relations**; graph score 42.2 → 53.4.
  - Arm B reached **497 relations**; graph score 42.2 → 75.4, flattening by cycle ~12.
- **Retrieval is deterministic.** The three cycle-0 repeats produced identical prompts (Jaccard 1.00), so the retest spread is 0.

**Pre-registered verdict: FAIL.**
- Arm B's Δcoverage from cycle 0 to 24 was +0.010 (95% CI −0.020 to +0.053).
- Arm A: +0.021 (CI 0.000 to +0.048).
- B − A at cycle 24: −0.010 (CI −0.053 to +0.038).

| Measure (31 questions) | c0 | A c24 | B c24 | Note |
|---|---|---|---|---|
| Coverage (NLI, primary) | 0.564 | 0.585 | 0.575 | B is flat from cycle 1 on |
| Gold passage in prompt | 0.796 | 0.801 | 0.791 | Pooled over nuggets: 81 → 85 of 108 (+5 / −1) |
| Keyword recall | 0.711 | 0.683 | 0.714 | Retest spread 0.047 |
| Answer recall (NLI) | 0.390 | 0.338 | 0.392 | Retest spread 0.049 |

What the trial shows:
1. **Relations change what is retrieved, once.** Arm B's prompts differ from cycle 0 (Jaccard 0.71, against 0.88 for arm A). They bring about 2 more slice chunks per question (8.4 against 6.4). After cycle 1 the prompts stop changing, although relations grow from 288 to 497.
2. **The change is a net wash on gold passages.** Arm B gained a gold passage for 5 nuggets (q30 0/6 → 3/6; q09 and q39 +1 each) and lost one (q22, 1/1 → 0/1). The per-question mean is therefore flat.
3. **Coverage has a ceiling that retrieval cannot pass.** At B cycle 24, 32 nuggets have their gold passage in the prompt but score below 0.5 against it, 10 of them at 0.0. Many gold passages were accepted by Claude adjudication at NLI 0.3–0.9. The primary measure under-reads retrieval by about 20 points here: 0.564 coverage against 0.796 gold-in-prompt at cycle 0.

Not tested here: whether relations help *answers* when the graph is built from the whole book. The slice's vector search still covers all 1,152 chunks, and it finds most slice gold passages without the graph (vector-only gold-in-prompt 0.724).

Analysis: `tests/kwaai-knowledge/eval2/pilot/d6_noseed_analyze.py` → `results/d6_noseed_s10/report.{md,json}`.

## Follow-up: graph-only retrieval (pre-registered 2026-09-29, before any graph-only eval)

Reza, 2026-09-29: DreamRAG's thesis is that short-term memory (the vector store) is indexed and
transferred into long-term memory (the graph), and that short-term memory is then forgotten. The
trial above never tested that, because every retrieval mode still read the chunks. `--mode
graph-only` (`ce804064`) gives the model entity fact cards only, with every relation listed.

- **Snapshots:** the trial's saved snapshots; nothing is rebuilt or re-dreamed.
  - Cycle 0: r1 and r2.
  - Arms A and B: cycles 1, 4, 12 and 24, plus an r2 at cycle 24.
  - The same 31 questions and 108 slice nuggets.
- **Primary:** coverage under graph-only retrieval.
  - **Pass** if arm B's paired Δcoverage from c0 to c24 has a 95% bootstrap CI above 0 and a mean
    larger than the graph-only cycle-0 retest spread.
- **Secondary:** B − A at c24; answer recall; keyword recall.
- **Expectation stated in advance:** graph-only coverage is far below the chunk-retrieval figure
  (0.564). A cycle-0 card holds only what extraction wrote. Without the seed's alias merges, the
  graph also splits entities: the author appears as "Joe Rassool", "Rassool" and "Y.S.". If
  dreaming transfers knowledge into the graph, it shows up here or nowhere.

### Graph-only result (2026-09-29, 09:43)

**Pre-registered verdict: PASS.**
- Arm B's graph-only Δcoverage from c0 to c24 was **+0.183** (95% CI +0.087 to +0.296), against a
  cycle-0 retest spread of 0.0065.

| Graph-only coverage | c0 | c1 | c4 | c12 | c24 |
|---|---|---|---|---|---|
| Arm A (descriptions only) | 0.140 | 0.257 | 0.259 | 0.248 | 0.310 |
| Arm B (+ relations) | 0.140 | 0.332 | 0.329 | 0.366 | 0.323 |

- **Dreaming more than doubles what the graph alone holds**: 0.140 → 0.32. It is the transfer the
  thesis claims.
- **Most of the gain comes in cycle 1.** Arm B gets there faster (0.332 at c1 against A's 0.257),
  but by c24 the arms are level: B − A = +0.013 (CI −0.078 to +0.089). So relations speed the
  transfer up, but this trial cannot separate their effect from better descriptions.
- **The graph alone still falls short of the chunks.** Graph-only coverage at c24 is 0.32, against
  0.56 for chunks + graph. Graph-only answer recall rose 0.144 → 0.188 in arm B, and its CI
  includes zero. Graph-only keyword recall is flat (0.592 → 0.580).

Chart: https://claude.ai/artifact/CemutmjtBcZi26hiboKpP5 (retrieval switch).
