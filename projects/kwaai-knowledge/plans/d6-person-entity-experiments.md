# D6 RAG Experiments — Person Entities Only

## Context

**Current baseline:** M30/M35 graph (~1905 entities, 6164 relations, ~57.8% avg keyword, 1.80/2
judge). Includes Person + Place + Organization entities, with family tree seed relations.

**Goal:** Strip away Place/Org noise and test whether a tightly-deduplicated Person-only graph
can close the remaining accuracy gap. Also validates a new Jaro-Winkler gate for Tier 2 dedup
before applying it to multi-type graphs.

**Scope:** Person entities only. No family tree YAML seed — intentional, to measure pure
extraction quality. Relations come only from `sanitize` inference (gender, inverse edges).

---

## 1. Prerequisites confirmed

All CLI flags exist — no code changes needed for:
- `--entity-types Person` (single type, triggers `person_only` path in graph.rs:2943)
- `--sample-pct N` (1–100; filters chunk list before extraction at CLI level)
- `--no-relations` (skips relation extraction in build + dream)
- `kwaainet rag graph stats/show/dedup/sanitize/reembed/clear`
- `kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json`
- `kwaainet rag dream run --no-relations --max-completions N --workers N`

**Dedup tier clarification** — `--auto --auto-threshold 1.01` runs:
- Tier 1 (exact normalized names): auto-merged ✓
- Tier 2 (cosine similarity): NOT auto-merged (1.01 > any possible cosine sim)
- Tier 3 (structural: honorific-strip, subset-name+shared-neighbours, edit-dist ≤ 2): auto-merged ✓
- Tier 4 (role/pronoun neighbor containment): auto-merged ✓

---

## 2. Code change: Jaro-Winkler gate for Tier 2 dedup

**Required before Experiment 2.** Experiment 1 can run without it.

**File:** `core/crates/kwaai-rag/src/graph.rs`  
**Function:** `find_dedup_candidates()` (line ~1861)

Add three guards at the top of the candidate-pair loop, before cosine similarity is computed:

### 2a. Type-match guard
```rust
if a.entity_type != b.entity_type {
    continue;
}
```

### 2b. Disambiguation token filter
```rust
fn has_disambiguation_token(name: &str) -> bool {
    // Parenthetical suffix: "John Smith (politician)"
    let t = name.trim_end();
    if t.ends_with(')') && t.rfind('(').map_or(false, |p| p > 0) {
        return true;
    }
    // Roman numeral suffix: "John Smith II", "John Smith III"
    matches!(
        name.split_whitespace().last().unwrap_or(""),
        "II" | "III" | "IV" | "VI" | "VII" | "VIII" | "IX"
    )
}

// In loop:
if has_disambiguation_token(&a.name) || has_disambiguation_token(&b.name) {
    continue;
}
```

### 2c. Jaro-Winkler gate
Implement `fn jaro_winkler(s1: &str, s2: &str) -> f32` natively (~35 lines, no new crate).
Apply on `normalize_name()` output (same normalization as Tier 1):

```rust
const JW_DEDUP_GATE: f32 = 0.60;  // calibrate in Experiment 2

let jw = jaro_winkler(&normalize_name(&a.name), &normalize_name(&b.name));
if jw < JW_DEDUP_GATE {
    continue;
}
```

**Threshold calibration:** 0.60 blocks clearly different names (e.g., "Yousuf Rassool" /
"Cissie Gool" ≈ 0.53) while allowing partial-name variants ("Abdul Hamid" / "Abdul Hamid
Gool" ≈ 0.82). Tune based on Experiment 2 false-positive analysis.

**Jaro-Winkler formula:**
- Jaro: matching chars within window `max(|s1|,|s2|)/2 - 1`, count transpositions
- Winkler boost: `prefix_len * 0.1 * (1 - jaro)` for up to 4 common prefix chars
- Add near `cosine_sim_f32` at line ~3208

**Build after change:**
```bash
cd core && cargo build -p kwaai-rag && cargo build -p kwaainet --release
cp target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet
```

---

## 3. Create experiment log file

Create `tests/kwaai-knowledge/d6_experiments_log.md` before starting. Append one section per
experiment run (format in Section 7).

---

## 4. Experiment sequence

All commands run from `KwaaiNet/` repo root. Fill in actual `--inference-urls` value.

### Pre-flight: archive M35 graph, then clear

```bash
# Confirm current state
kwaainet rag graph stats --kb D6

# Backup the redb file before clearing (locate in your data dir, e.g. ~/.kwaainet/data/D6/)
# cp ~/.kwaainet/data/D6/graph.redb ~/.kwaainet/data/D6/graph-M35-backup-$(date +%Y%m%d).redb

kwaainet rag graph clear --kb D6 --yes
```

---

### Experiment 0 — Baseline raw Person graph (full corpus)

```bash
kwaainet rag graph build --kb D6 \
  --entity-types Person \
  --no-relations \
  --workers 8 \
  --inference-urls "http://..." \
  --model llama3.1:8b

kwaainet rag graph stats --kb D6
```

Log: `D6_raw_person_YYYYMMDD`. Record raw entity count. No dedup. This is the noise floor.

---

### Experiment 1 — 1% sample sanity check (no Tier 2)

```bash
kwaainet rag graph clear --kb D6 --yes

kwaainet rag graph build --kb D6 \
  --entity-types Person \
  --no-relations \
  --sample-pct 1 \
  --workers 8 \
  --inference-urls "http://..." \
  --model llama3.1:8b

kwaainet rag graph dedup --kb D6 --auto --auto-threshold 1.01
kwaainet rag graph sanitize --kb D6
kwaainet rag graph reembed --kb D6
kwaainet rag graph stats --kb D6
```

Manual review: `kwaainet rag graph show --kb D6 --name "<name>"` on 10–20 entities.
Check: plausible names, correct merges, no obvious false merges.
Log: `D6_person_1pct_tier1_YYYYMMDD`. No eval — sanity check only.

---

### Experiment 2 — 10% sample with new Tier 2 JW gate

**Prerequisite:** Code change from Section 2 built and installed.

```bash
kwaainet rag graph clear --kb D6 --yes

kwaainet rag graph build --kb D6 \
  --entity-types Person \
  --no-relations \
  --sample-pct 10 \
  --workers 8 \
  --inference-urls "http://..." \
  --model llama3.1:8b

# Tier 2 now gated by JW + type-match + disambiguation filter
kwaainet rag graph dedup --kb D6 --auto --auto-threshold 1.01
# Sample Tier 2 candidates interactively (dry-run to list without merging)
kwaainet rag graph dedup --kb D6 --threshold 0.85 --dry-run

kwaainet rag graph sanitize --kb D6
kwaainet rag graph reembed --kb D6
kwaainet rag graph stats --kb D6
```

**Dedup quality review:** From the dry-run output, manually classify 20–30 Tier 2 proposals
as ✅ correct vs ❌ false positive. If false-positive rate > 5–10%, adjust `JW_DEDUP_GATE`
constant in graph.rs, rebuild, and rerun. Iterate until satisfied.

Log: `D6_person_10pct_dedup_v1_YYYYMMDD`. Record entities before/after, false-positive rate.
No RAG eval yet.

---

### Experiment 3 — Full Person build & dedup (production candidate)

```bash
kwaainet rag graph clear --kb D6 --yes

kwaainet rag graph build --kb D6 \
  --entity-types Person \
  --no-relations \
  --workers 8 \
  --inference-urls "http://..." \
  --model llama3.1:8b

# Same validated configuration from Experiment 2
kwaainet rag graph dedup --kb D6 --auto --auto-threshold 1.01

kwaainet rag graph sanitize --kb D6
kwaainet rag graph reembed --kb D6
kwaainet rag graph stats --kb D6
```

Log: `D6_person_full_dedup_v1_YYYYMMDD`. Record raw and post-dedup entity counts.
Compare raw/final ratio against Round 3 reference (2066 raw → 1377 final ≈ 33% reduction).

---

### Experiment 4 — Dream enrichment on Person entities

```bash
# Use --dedup-threshold 0.99 to avoid aggressively merging distinct persons after
# descriptions change — 0.92 (default) collapsed family members in Round 4
kwaainet rag dream run --kb D6 \
  --no-relations \
  --max-completions 300 \
  --workers 8 \
  --dedup-threshold 0.99 \
  --inference-urls "http://..." \
  --model llama3.1:8b

kwaainet rag graph stats --kb D6
```

Manual check: `graph show` on 10–20 newly-enriched entities — facts grounded in source
text, no hallucinations, no overwritten good descriptions.

Log: `D6_person_full_dedup_dream_v1_YYYYMMDD`.

---

### Experiment 5 — RAG eval (Person-only, no seed)

```bash
kwaainet rag eval --kb D6 \
  --questions tests/kwaai-knowledge/d6_eval_questions.json
```

Run 3 times and average (±4pp run-to-run variance is normal).

**Expected failure pattern:** Questions requiring spouse_of/grandchild_of traversal (q07, q09)
will underperform vs the seeded baseline — this is expected and NOT a bug in this experiment.
Note them as "relation-phase deferred" failures in the log.

**Compare against:**
- Unseeded baseline (Round 3, no dream): 54.3% kw / 1.65/2 judge
- Seeded + dreamed baseline (M30): ~57.8% kw / 1.80/2 judge

Log: `D6_person_eval_v1_YYYYMMDD`. Record recall, judge score, per-question notes.

---

## 5. Logging format

Append to `tests/kwaai-knowledge/d6_experiments_log.md` after each experiment:

```markdown
## YYYY-MM-DD – D6_person_full_dedup_dream_v1

- Experiment: 4 + 5 (Dream + Eval, Person-only, no seed)
- Build: full Person graph, no relations, llama3.1:8b, 8 workers
- Dedup: Tier 1 + JW gate (threshold=0.60) + Tier 3 + Tier 4; Tier 2 auto disabled
- Dream: max-completions=300, dedup-threshold=0.99, --no-relations
- Stats: raw=N, after-dedup=N, after-dream=N
- Eval (3-run avg): recall=XX.X%, judge=Y.YY/2.0
- Observations:
  - Strengths: ...
  - Failures (relation-phase deferred): q07, q09 — missing spouse_of/grandchild_of
  - Failures (entity-level, actionable): ...
```

---

## 6. Verification checkpoints

1. **After Experiment 1 (1%):** `graph show` on ~20 entities — plausible names, Tier 1/3
   merges are correct, no obvious false merges.

2. **After Experiment 2 JW calibration:** False-positive rate on 20–30 sampled Tier 2
   proposals < 5% before proceeding to full build.

3. **After Experiment 3 full build:** Entity count reduction from dedup is in the 25–40% range
   (reference: Round 3 was 33%). If > 50%, the JW threshold may be too permissive.

4. **After Experiment 5 eval:** Person-only + JW dedup + dream should beat the unseeded Round 3
   baseline (1.65/2). If it underperforms, the delta quantifies how much the family tree seed
   contributes vs extraction quality — useful signal for the next phase.
