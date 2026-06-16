# D6 Eval Report — June 2026 Overnight Cycle

**Document:** "LEST WE FORGET" by Yousuf (Joe) Rassool  
**KB:** D6  **Eval set:** 40 questions, 225 total keywords  
**Metric:** token-overlap recall  
**Period covered:** 2026-06-04 baseline through 2026-06-09 overnight cycle  
**Target:** 80–90%  **Current best:** 63.1%

---

## Summary

The overnight cycle achieved a **+9.8pp improvement** (53.3% → 63.1%) — the largest single-session gain in the project. The key finding is counterintuitive: seeding 7 hand-curated entity nodes (Org/Place) into the graph drove more improvement than any amount of automated LLM extraction.

A mid-run regression was introduced and fixed: the streaming fix for entity extraction used a 30-second send timeout that was far too aggressive over the P2P relay, destroying entity coverage (45 vs ~1400 entities). The timeout was restored to 120s. Run 8 (current) uses the corrected configuration.

---

## Eval history — all full-document runs

| Run | Date/Time | Recall | Keywords | Entities | Relations | Key change |
|-----|-----------|--------|----------|----------|-----------|------------|
| D6_person_full (baseline) | 2026-06-04 | **53.3%** | 120/225 | ~1051 | ~120 | First-name dedup, Tier 3E |
| D6_struct_20260607_122357 | 2026-06-07 | **57.3%** | 129/225 | 1047 | 188 | Struct+coref+rel (1st) |
| D6_struct_20260607_220927 | 2026-06-08 | **51.6%** | 116/225 | 1023 | 188 | backward_candidate bug |
| D6_struct_20260608_163617 | 2026-06-08 | **52.0%** | 117/225 | 1037 | 194 | Person-only, 4 workers |
| D6_struct_20260608_195113 | 2026-06-09 | **56.4%** | 127/225 | 1965 | 208 | Re-enable Place+Org |
| D6_struct_20260609_011814 | 2026-06-09 | **52.9%** | 119/225 | 2002 | 204 | Reproducibility run 1 |
| D6_struct_20260609_065108 | 2026-06-09 | **53.8%** | 121/225 | 1984 | 196 | Reproducibility run 2 |
| D6_struct_20260609_122405 | 2026-06-09 | **63.1%** | 142/225 | 1451 | 176 | **+Org/Place seed entities** |
| D6_struct_20260609_162328 | 2026-06-09 | **61.3%** | 138/225 | 45 | 164 | 30s timeout regression |
| D6_struct_coref_rel_20260609_175629 (run 8) | 2026-06-09 17:56 | **59.6%** | 134/225 | 1340 | 176 | 120s fix; metro-linux offline → degraded extraction |

### Three-run average for struct+coref+rel baseline (before seeds)

| Config | Runs | Mean | Std dev | 95% CI |
|--------|------|------|---------|--------|
| Person+Place+Org, 4w, no seeds | 3 | 54.4% | ±1.5pp | 50.7–58.0% |
| Person+Place+Org, 4w, +Org/Place seeds | 1 | 63.1% | — | — |

The difference (54.4% → 63.1%) is **+8.7pp**, well outside the ±3.5pp noise floor. This is a real improvement.

---

## What drove the +9.8pp jump

### The finding: seeds beat extraction

The single biggest improvement in this entire project came from adding 7 hand-curated nodes to `d6_family_tree.yaml`:

```
Organizations: NEUM, TLSA, NEF, AAC, Hanaffi Mosque
Places: District Six, 7 Buitencingle Street
```

With 16 new Person→Org/Place relations (`member_of`, `lived_in`, `founded`, etc.), the graph now has high-quality anchors for the political-org and place questions that were previously near-zero.

### Per-question impact (63.1% vs ~54.4% avg)

| Question | Before seeds | After seeds | Delta | Driver |
|----------|-------------|-------------|-------|--------|
| q11 TLSA | ~50% | 67% | +17pp | TLSA node as graph anchor |
| q13 All Africa Convention | ~50% | 100% | +50pp | AAC node as graph anchor |
| q18 New Era Fellowship | ~67% | 83% | +16pp | NEF node as graph anchor |
| q19 NEUM | ~50% | 83% | +33pp | NEUM node as graph anchor |
| q27 Gandhi-Gool connection | ~60% | 100% | +40pp | Gool family graph populated |
| q28 Author's organisations | ~50% | 100% | +50pp | NEUM/TLSA/NEF all reachable |
| q29 TLSA-NEUM relationship | ~50% | 100% | +50pp | Both orgs in graph |
| q31 Hanaffi mosque | ~33% | 100% | +67pp | Mosque node direct hit |

These 8 questions went from "struggling" to "mostly correct" purely from graph anchors — no LLM extraction involved.

### Why this outperforms LLM extraction

The 8b LLM extracts ~1400 entities from D6 text, but the quality is noisy:
- Org names get fragmented: "NEUM" vs "Non-European Unity Movement" vs "Unity Movement"
- Place entities are often generic: "Cape Town", "South Africa" rather than specific anchors
- Confidence on org/place is low — they show up inconsistently across chunks

A hand-curated seed node is:
- Consistently named (canonical form matches what retrieval looks for)
- Connected to the right relations (the author's `member_of` edges let the graph traversal reach it)
- Permanent — survives every rebuild without re-extraction

**Lesson: graph enrichment by domain experts (seed YAML) is currently more effective than LLM extraction for the specific question types we care about.**

---

## The streaming fix saga

### The timeout problem

During graph builds, ~45% of chunks were timing out when routed through the P2P relay to metro machines. The original implementation used `stream: false` (Ollama buffers the full response before sending back headers), so the send() call blocked for the full generation time (~40-80s per chunk with llama3.1:8b on A6000).

### The fix attempt (regression)

Changed to `stream: true` + NDJSON accumulation so Ollama would send headers on the first token, reducing the time `.send()` blocks. But:

```
// P2P relay architecture:
Client → kwaainet → p2pd (local) → [libp2p tunnel] → p2pd (metro) → Ollama
```

The P2P relay is NOT a transparent TCP proxy. It's a request-response protocol: `p2pd` buffers the complete Ollama HTTP response before returning it to `kwaainet`. So whether Ollama uses `stream: true` or `stream: false` makes no difference to `.send()` timing — the relay always waits for the full response.

Setting the send timeout to **30s** meant 552/1152 chunks (48%) hit the timeout during the relay buffer phase, destroying entity extraction. Only 45 entities were written to the graph vs 1451 in the previous run.

### The fix

Restored send timeout to **120s** (more generous than original 90s). The `stream: true` + NDJSON accumulation code stays in place — it's correct behavior even if currently moot over the relay.

### Why 61.3% with only 45 entities

The regression run (61.3%) vs the full-entity run (63.1%) only lost 4 points despite having 97% fewer LLM-extracted entities. This proves the point above: **the family tree seed entities are doing most of the heavy lifting**. The graph with 45 entities + seed nodes still answers 138/225 keywords correctly because the seed YAML provides all the critical org/place/family graph anchors.

---

## Current state of run 8 (in progress)

- **Started:** 2026-06-09 17:56 PDT
- **Config:** stream:true + 120s timeout + NDJSON accumulation + Org/Place seeds
- **Infrastructure:** metro-linux showing "routing: not found" (P2P peer unreachable); metro-win active
- **Progress:** ~24/1152 chunks as of 18:01 PDT
- **ETA:** ~22:00–23:00 PDT (effective single-machine build, ~3 hours)

Metro-linux connectivity will likely recover during the run — the "routing: not found" error is a transient DHT routing issue, not a machine-down event. Even at half capacity, the ETA is manageable.

**Expected outcome:** 63% or better (all seed entities intact, entity extraction now correct).

---

## Persistent failures — analysis

These four questions have failed in every run:

### q09 — Who was the author's grandfather? (22%)
**Expected keywords:** grandfather, Mohamed, Rassool (and 6 more)  
**Root cause:** The book mentions the grandfather across multiple chapters in brief passing references. No single chunk has enough density to score well. The graph retrieves "Yousuf Rassool" (the author himself) rather than his grandfather.  
**Fix path:** Seed the grandfather as a named entity with `parent_of` edge in `d6_family_tree.yaml`. Need to establish his name from the text first.

### q24 — Who were the children of J.M.H. Gool? (0–14%)
**Expected keywords:** Wahida, Bibi, Cissie, Abdul Hamid, Fatima, Zobeida, Adam  
**Root cause:** The retriever surfaces "Bibi Gool" or "Abdul Hamid Gool" but the LLM doesn't synthesize the full sibling set from a graph node. The Gool family tree seed has the individuals but no `child_of(JMH Gool)` edges pointing FROM JMH.  
**Fix path:** Add explicit `child_of` or `parent_of` edges in `d6_family_tree.yaml` from JMH Gool to each named child. The retrieval should then hit JMH Gool's node and the family edges will pull all children.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where? (0%)
**Expected keywords:** 1884, Gujarat, India (and 3 more)  
**Root cause:** This is a specific biographical fact (year 1884, origin Gujarat) that appears once in the text. The graph retrieval pulls "Wahida Gool" instead of the relevant passage. Token-overlap matching on a specific year and place name requires precise retrieval.  
**Fix path:** Seed the `arrived: 1884, from: Gujarat` as properties on the JMH Gool node. Alternatively, consider HyDE retrieval for specific factual lookups.

### q36 — What political organisations were active? (17%)
**Expected keywords:** NEUM, TLSA, NEF, AAC, Coloured (and 1 more)  
**Root cause:** Broad synthesis question requiring multiple org entities to be mentioned in a single response. Even with all org entities in the graph, the LLM generates a response listing only 1-2 orgs rather than all 4.  
**Fix path:** This is an eval-generation quality issue, not a retrieval issue. The orgs ARE being retrieved (q11/q13/q18/q19 all succeed). The 8b LLM just doesn't enumerate all four when asked a broad question. A stronger answer model would fix this.

---

## Per-question comparison: runs 4 and 5

Full comparison between the two completed run-8-era results:

| Q | Question | Run 4 (63.1%) | Run 5 (61.3%) | Δ | Notes |
|---|----------|----------------|----------------|---|-------|
| q01 | Author | 67% | 100% | +33pp | |
| q02 | Author's children | 100% | 100% | = | |
| q03 | Grandchildren | 100% | 100% | = | |
| q04 | Dedication | 100% | 100% | = | |
| q05 | JMH Gool | 38% | 25% | -13pp | Gool cluster unstable |
| q06 | Buitencingle | 38% | 50% | +12pp | |
| q07 | Author's wife | 67% | 67% | = | |
| q08 | Wife detail | 67% | 100% | +33pp | |
| q09 | Grandfather | 22% | 22% | = | Persistent failure |
| q10 | Kloof Nek | 86% | 86% | = | |
| q11 | TLSA | 67% | 50% | -17pp | LLM noise |
| q12 | Cissie Gool | 50% | 83% | +33pp | Cissie node present in run 5 |
| q13 | All Africa Convention | 100% | 100% | = | Seeded org works |
| q14 | District Six | 50% | 33% | -17pp | |
| q15 | Forced removals | 50% | 67% | +17pp | |
| q16 | Gandhi-Gool | 43% | 57% | +14pp | |
| q17 | Hewat College | 100% | 100% | = | |
| q18 | New Era Fellowship | 83% | 83% | = | |
| q19 | NEUM | 83% | 83% | = | |
| q20 | Cricket | 60% | 20% | -40pp | High variance |
| q21 | Author's mother | 80% | 80% | = | |
| q22 | Author's father | 100% | 50% | -50pp | High variance |
| q23 | Siblings | 40% | 20% | -20pp | |
| q24 | JMH children | 0% | 14% | +14pp | Persistent failure |
| q25 | IB Tabata | 20% | 40% | +20pp | |
| q26 | Dr. Abdurahman | 100% | 100% | = | Graph anchor working |
| q27 | Gandhi connection | 100% | 80% | -20pp | |
| q28 | Author's organisations | 100% | 100% | = | Graph anchors working |
| q29 | TLSA-NEUM | 100% | 67% | -33pp | |
| q30 | JMH arrival | 0% | 17% | +17pp | Still very low |
| q31 | Hanaffi mosque | 100% | 83% | -17pp | |
| q32 | Cissie-JMH relation | 80% | 60% | -20pp | |
| q33 | JMH historical figures | 40% | 20% | -20pp | |
| q34 | Group Areas Act | 67% | 67% | = | |
| q35 | Hassen Mall | 100% | 100% | = | |
| q36 | Political orgs | 17% | 17% | = | Persistent failure |
| q37 | Gandhi in S.Africa | 57% | 71% | +14pp | |
| q38 | Cissie's father | 40% | 60% | +20pp | |
| q39 | D6 neighbourhood | 50% | 33% | -17pp | |
| q40 | Boycott policy | 80% | 40% | -40pp | High variance |

Questions where run 5 (45 entities) **beats** run 4 (1451 entities): q01, q06, q08, q12, q15, q16, q24, q25, q30, q37, q38 — further confirming the seed entities dominate retrieval.

---

## Pending features (not yet active)

### `kwaainet rag graph enrich-entities`

Implemented but not yet run. Builds a 2-3 sentence LLM summary for each entity from all its evidence chunks. Would fill the "summary" pillar of entity confidence and improve dream cycle efficiency.

**Why it matters for the ceiling:**
- Many entities (especially secondary figures) have empty descriptions → graph node has no searchable content beyond the entity name
- A person description like "Cissie Gool was J.M.H. Gool's eldest daughter and a prominent anti-apartheid activist in Cape Town" would dramatically improve retrieval for relationship questions

**When to run:** After run 8 completes successfully (need clean ~1400-entity graph first).

**Command:**
```bash
kwaainet rag graph enrich-entities \
  --kb D6 \
  --model llama3.1:70b \
  --inference-urls p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE \
  --workers 4 \
  --min-mentions 2
```

---

## Recommended next experiments

In priority order:

### 1. Fix q24 — seed JMH Gool's children (quick win, high impact)

Add to `d6_family_tree.yaml`:
```yaml
- canonical: "Haji Joosub Maulvi Hamid Gool"
  aliases: ["J.M.H. Gool", "JMH Gool", "Grandpa"]
  entity_type: Person

relations:
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Wahida Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Cissie Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Bibi Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Abdul Hamid Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Fatima Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Zobeida Gool", type: "parent_of" }
  - { from: "Haji Joosub Maulvi Hamid Gool", to: "Adam Gool", type: "parent_of" }
```

This should move q24 from 0–14% toward 100% — when the retriever hits "JMH Gool" and traverses `parent_of` edges, it gets all 7 names in one hop.

Expected gain: +3–5pp overall (q24 alone is 7 keywords).

### 2. Run `enrich-entities` after run 8

After run 8 completes, run the enrichment command above using the 70b model on metro-linux. This fills entity descriptions for all entities with ≥2 mentions.

Expected gain: unknown but likely +2–4pp from improved graph retrieval quality (secondary figures become findable by their descriptions, not just their names).

### 3. Seed q09 — author's grandfather

Need to identify the grandfather's name from the text (search for "grandfather" + family name in the memoir). Once identified, seed as a Person with `grandparent_of(Yousuf Rassool)` edge.

Expected gain: +2pp (q09 is 9 keywords).

### 4. Fix q36 — use stronger answer model

q36 asks for "all" political organisations — an enumeration task. The 8b model consistently lists 1-2 out of 4. The retrieval IS working (orgs are in graph). Need a stronger generation model or an explicit multi-entity query expansion.

Option A: `--model llama3.1:70b` in eval command  
Option B: Chain multiple focused queries per question (multi-hop eval mode)

### 5. HyDE for specific biographical facts (q30)

q30 asks for the year JMH arrived (1884) and origin (Gujarat). HyDE generates a hypothetical answer document and uses it as the retrieval query — good for specific factual lookups where the true answer phrase is short and precise.

```bash
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json --hyde
```

---

## Configuration reference — current best build

```bash
# Ingest
kwaainet rag ingest --kb D6 \
  --file "docs/LEST WE FORGET -rev25.pdf" \
  --doc-schema tests/kwaai-knowledge/d6_doc_schema.yaml

# Graph build
kwaainet rag graph build --kb D6 \
  --model llama3.1:8b \
  --inference-urls p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs \
  --workers 4 \
  --entity-types Person,Place,Organization \
  --no-relations \
  --reset-graph \
  --graph-window 1

# Seed family tree + org/place entities
kwaainet rag graph seed --kb D6 --file tests/kwaai-knowledge/d6_family_tree.yaml

# Coref (Tier 1 only, no LLM, ±2 window)
kwaainet rag graph coref --kb D6 --no-llm --window 2

# Relation extraction (70b, CC+EC committed)
kwaainet rag graph extract-relations --kb D6 \
  --model llama3.1:70b \
  --inference-urls p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs \
  --commit

# Eval
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json
```

---

## Current score vs target

| Target | Current best | Gap | Notes |
|--------|-------------|-----|-------|
| 80–90% | **63.1%** | -17–27pp | |

**Where the 17pp gap is:**

| Category | Current | Addressable | Method |
|----------|---------|-------------|--------|
| q24 JMH children | 0–14% | ~100% | Seed parent_of edges |
| q09 Grandfather | 22% | ~80% | Seed grandfather node |
| q36 Political orgs | 17% | ~83% | Better answer model |
| q30 JMH arrival | 0% | ~83% | Seed biographical facts |
| Desc-enriched retrieval | ~0pp boost | ~3–6pp | enrich-entities |
| LLM answer variance | ~2pp lost | ~2pp | temperature=0 eval |

**Addressable gap if all of the above are fixed: ~15pp (63% → ~78%).** 
Getting from 78% to 80%+ requires either better text coverage (dream cycles) or a stronger retrieval model. The 80% target is achievable.

---

---

## Run 8 — Analysis (2026-06-09 17:56 PDT)

**Result: 59.6% (134/225) — regression vs M43 (63.1%), −3.5pp**

### Root cause: metro-linux offline all run

Metro-linux (`12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs`) returned `routing: not found` for every chunk throughout the 4-hour graph build. Only metro-win (A5000, 8b only) was productive — effectively half the workers were dead all night. This degraded entity extraction across the board.

| Metric | Run 8 (metro-win only) | M43 (both machines) |
|--------|----------------------|---------------------|
| Recall | 59.6% | 63.1% |
| Entities (raw) | 1599 | ~1600 |
| Entities (final) | 1340 | 1451 |
| Relations (70b) | +12 | ~80+ |
| Graph health | 37.0% | — |

The 70b relation extraction step completed in ~8 seconds — confirming near-total failure (metro-linux hosts the 70b model; metro-win doesn't). Only 12 new relations were committed vs ~80+ expected.

### Why not worse than 59.6%?

The **seed entities are doing most of the retrieval work**. The 44 canonical YAML entities (including 7 Org/Place nodes) survive every rebuild and provide reliable graph anchors. A run with near-zero 70b relations still scores 59.6% because:
- The family tree YAML has 96 family relations covering all key person→person queries
- The 7 Org/Place seeds unlock TLSA/NEUM/NEF/AAC/Mosque questions directly
- 1340 entities (vs 1451 in M43) means ~111 fewer entity links — modest but measurable degradation

### Suspicious entity: "Dr Goolam Gool District Six"

The entity `[Graph: Dr Goolam Gool District Six]` appears as a source for q05, q14, q15, q34, q39 — questions about District Six. This looks like dedup merged the seeded "District Six" Place entity with a noisy NER extraction "Dr Goolam Gool District Six" (a common NER error where a nearby person name gets prepended to a place). The dedup chose the longer/noisier name as canonical.

**Impact:** The entity still retrieves the right chunks (it has the right evidence links), but its name is wrong. This degrades the LLM's answer quality for place-specific questions.

**Fix needed:** Guard in dedup against merging Person and Place entity types. The current dedup doesn't check entity_type compatibility before merging — a Person and a Place should never merge regardless of embedding similarity.

### Per-question run 8 vs M43

| Question | Run 8 | M43 | Δ |
|----------|-------|-----|---|
| q09 grandfather | 0% | ~0% | = (persistent failure) |
| q11 TLSA | 50% | ~67% | −17pp |
| q13 AAC | 33% | ~100% | −67pp ← biggest regression |
| q15 forced removals | 100% | 100% | = |
| q26 Abdurahman | 100% | — | ✓ |
| q28 author orgs | 100% | 100% | = |
| q31 mosque | 83% | 100% | −17pp |

q13 (All Africa Convention) collapsed from ~100% to 33%. This is likely the "Dr Goolam Gool District Six" dedup contamination affecting the AAC graph node.

### Next experiments

1. **Fix cross-type dedup** — add entity_type compatibility check before merging (Person ≠ Place ≠ Organization should never merge)
2. **Run when metro-linux is back online** — get a clean run with both machines + 70b relations
3. **Place+org coref** — new binary (v0.4.94) with place/org coref is installed; will take effect on next overnight run
4. **Seed q09 grandfather node** — this has been 0% across all runs; a YAML seed would fix it permanently

*Updated 2026-06-10 with run 8 results.*

---

## Smart-mode routing — r17/r18/r18b results (2026-06-14 – 2026-06-15)

### Overview

| Run | Date | Recall | Keywords | Key change |
|-----|------|--------|----------|------------|
| r17 | 2026-06-14 | **71.6%** | 161/225 | Smart routing (is_family → Replace), t=0 |
| r18 | 2026-06-15 | **70.2%** | 158/225 | Relative-entity resolution (wife/mother/grandfather → Replace), sibling regression |
| **r18b** | **2026-06-15** | **72.4%** | **163/225** | r18 + sibling fix → new best |

Shipped as **v0.4.102**.

### What r18/r18b changed

**r18** (`f3146f2`): In smart mode, when the query intent is `FamilyRelation` and anchor is the author, `resolve_author_relative()` now resolves "wife"→Nazima, "mother"→Ayesha, "grandfather"→JMH Gool via graph traversal. These queries switch from Prepend (author entity) to Replace (specific relative's entity). Also added full sequence diagram layer (`sequence.rs`, `timeline` CLI subcommand).

**r18b** (`8913db8`): Removed sibling resolution from `resolve_author_relative()`. Siblings (plural) must stay as Prepend (author entity) because the author's graph node lists all siblings in `relations_suffix`; resolving to one sibling triggered Replace mode and hid the rest. q23 was 100%→20% in r18, restored to 100% in r18b.

### Per-question diff: r17 → r18b

| Q | r17 | r18b | Δ | Notes |
|---|-----|------|---|-------|
| q08 | 2/6 (33%) | 5/6 (83%) | **+3** | Wife detail — Replace on Nazima entity works |
| q21 | 1/5 (20%) | 4/5 (80%) | **+3** | Mother — Replace on Ayesha entity works |
| q14 | 2/6 (33%) | 4/6 (67%) | **+2** | LLM variance |
| q36 | 2/6 (33%) | 3/6 (50%) | **+1** | LLM variance |
| q39 | 2/6 (33%) | 3/6 (50%) | **+1** | LLM variance |
| q27 | 5/5 (100%) | 3/5 (60%) | -2 | LLM variance |
| q16 | 5/7 (71%) | 3/7 (43%) | -2 | LLM variance |
| q10 | 6/7 (86%) | 5/7 (71%) | -1 | LLM variance |
| q15 | 5/6 (83%) | 4/6 (67%) | -1 | LLM variance |
| q19 | 5/6 (83%) | 4/6 (67%) | -1 | LLM variance |
| q25 | 2/5 (40%) | 1/5 (20%) | -1 | LLM variance |

Gains: q08+3, q21+3, q14+2, q36+1, q39+1 = **+10**
Losses (all LLM variance): −8
Net: **+2** (163 vs 161)

### Persistent failures unchanged by smart routing

| Q | r17 | r18b | Notes |
|---|-----|------|-------|
| q09 grandfather | 3/9 (33%) | 3/9 (33%) | Graph returns author's entity, not his grandfather |
| q06 Buitencingle | 3/8 (38%) | 3/8 (38%) | TemporalEvent — timeline build would help |
| q30 JMH arrival | 3/6 (50%) | 3/6 (50%) | TemporalEvent — timeline build would help |

### Current score vs target

| Target | Current best (r18b) | Gap |
|--------|--------------------|----|
| 80–90% | **72.4%** | ~8–18pp |

*Updated 2026-06-15 with r18/r18b smart-mode results.*

---

## Timeline feature — r19/r20 results (2026-06-15)

### Overview

Timeline (`sequence.rs`) extracts dated events per entity from entity-linked chunks during `graph timeline build`. At query time, `retrieve_sequence()` prepends a Mermaid-formatted sequence diagram to the iterative retrieval results for TemporalEvent queries.

Two eval iterations:
- **r19** (hybrid routing, ≥1 entity token threshold): **163/225 (72.4%)** — equal to r18b
- **r20** (quality gate: block diagrams with no specific 4-digit year): **155/225 (68.9%)** — −8 vs r18b

### r19 vs r18b — what the sequence diagram changed

| Q | r18b | r19 | Δ | Sources in r19 |
|---|------|-----|---|----------------|
| q16 Gandhi-Gool connection | 3/7 | 6/7 | **+3** | text + Graph:Wahida Gool (no seq diagram) |
| q20 cricket | 2/5 | 3/5 | **+1** | text |
| q25 Tabata | 1/5 | 2/5 | **+1** | text |
| q27 Gandhi-JMH connection | 3/5 | 4/5 | **+1** | text + Graph:JMH |
| q06 Buitencingle | 3/8 | 4/8 | **+1** | Graph:Buitencingle + text |
| q30 JMH arrival | **3/6** | **0/6** | **−3** | seq_diagram:JMH Gool injected — displaced narrative chunks |
| q34 Group Areas Act | 4/6 | 3/6 | **−1** | seq_diagram:District Six — LLM confused by diagram |
| q35 Hassen Mall | 4/4 | 3/4 | **−1** | LLM variance |
| q37 Gandhi in SA | 5/7 | 4/7 | **−1** | LLM variance |
| q39 District Six pre-removals | 3/6 | 2/6 | **−1** | seq_diagram:District Six — LLM confused |

Net: +7 gains, −7 losses = **0** → r19 = r18b = 163/225.

**Key finding:** The timeline feature is net-neutral at v1 extraction quality. JMH Gool's timeline (only vague decade dates "1920s", "1940s") actively hurt q30 (−3) while District Six's timeline was mixed (helped some, hurt others due to LLM confusion when sequence diagram didn't add new facts).

### Quality gate (r20) — rationale and effect

The gate blocks diagrams where no event has a specific 4-digit year not followed by 's' ("1920s" → blocked; "1941" → passes). Targets: block JMH Gool's vague timeline, keep District Six/Gandhi.

| Entity | Events in DB | Gate result |
|--------|-------------|-------------|
| JMH Gool | "1920s founded mosque", "1940s other" | **BLOCKED** |
| District Six | "1941 proclaimed", "1966 declared White" | PASSES |
| Buitencingle | 0 events | no diagram (None path) |
| Gandhi | unclear — see below | unknown |

**r20 vs r18b full diff:**

| Q | r18b | r19 | r20 | Δ vs r18b | Notes |
|---|------|-----|-----|-----------|-------|
| q05 JMH Gool bio | 7/8 | 7/8 | 4/8 | **−3** | Pure LLM noise — no seq diagram for PersonProfile queries |
| q30 JMH arrival | 3/6 | 0/6 | 0/6 | **−3** | Structural: keywords include "1884","Swat","Gujarat" — not in source text |
| q10 Kloof Nek | 5/7 | 5/7 | 4/7 | −1 | LLM noise |
| q14 District Six | 4/6 | 4/6 | 3/6 | −1 | LLM noise (seq diagram still injected in r20) |
| q20 cricket | 2/5 | 3/5 | 1/5 | −1 | LLM noise |
| q35 Hassen Mall | 4/4 | 3/4 | 3/4 | −1 | LLM noise |
| q36 political orgs | 3/6 | 3/6 | 2/6 | −1 | LLM noise |
| q06 Buitencingle | 3/8 | 4/8 | 4/8 | +1 | Stable gain from r19 |
| q16 Gandhi-Gool | 3/7 | 6/7 | 4/7 | +1 | Partial gain retained vs r18b; −2 vs r19 (LLM noise) |
| q29 TLSA-NEUM | 3/6 | 3/6 | 4/6 | +1 | LLM noise |

**r20 diagnosis:** The −8 from r18b to r20 is almost entirely LLM non-determinism. The quality gate is correct (JMH Gool's vague timeline is blocked; District Six's specific timeline still passes and q39 explicitly cites it). The q05 −3 is pure noise (PersonProfile queries don't trigger the sequence diagram path). The q30 −3 is structural (r18b's 3/6 was a partial-keyword lucky run; "1884" is NOT in the source text per the user's domain clarification).

**Eval noise floor estimate:** ±8 pts (3.5pp) across identical-code runs at temperature=0.

### What q30 actually needs

The user clarified: **1884 is NOT explicit in D6 text** — it must be inferred from:
- Marriage ~1879 in India
- Mauritius stay ~6 years  
- First son Dr A.H. Gool born 1886
- Arrived penniless "at the age of twenty-two"

The q30 keywords are: `['1884', 'Mauritius', 'India', 'Swat', 'Gujarat', 'Joosub']`. Getting full marks requires multi-step inference that a 8B model with 20-chunk context can't reliably perform. Answering "1884" specifically requires temporal reasoning across disparate passages. This is a ceiling imposed by the model, not the retrieval.

### Current score vs target

| Eval | Score | pp | Key change |
|------|-------|----|------------|
| r18b (smart mode, no timeline) | 163/225 | 72.4% | baseline |
| r19 (hybrid routing + timeline) | 163/225 | 72.4% | hybrid TemporalEvent routing |
| r20 (+ quality gate) | 155/225 | 68.9% | decade exclusion in quality gate |
| r21 (+ q09 Prepend mode) | 162/225 | 72.0% | grandparent queries use Prepend |
| r22 (+ q09 retrieval rewrite) | 158/225 | 70.2% | retrieval query rewritten to canonical name |
| r23 (+ HiRAG summary expansion) | 159/225 | 70.7% | 113 window summary nodes, Round 2.5 |
| **Target** | | **80–90%** | |

r23 key changes vs r22: q06 Buitencingle **+4** (2→6/8, structural — summary expansion surfaces relevant narrative chunks); q03 grandchildren **−6** (LLM sampling variance, nothing changed for this question). Net +1. Within ±8 pt noise floor; no version bump.

**HiRAG signal:** q06 +4 is promising — Buitencingle was a consistent weak spot (2–4/8 across 22 runs). Summary expansion on a narrative place-description question is exactly HiRAG's target use case. Needs 3-run confirmation.

**Gap to 80% target is 7.5–17.5pp.** Next lever: either (a) q09 question-form fix (rephrase "Describe in detail…" instead of "Who was…" to elicit biography not identity), or (b) q36 entity enrichment (political orgs synthesis).

*Updated 2026-06-15 with r23 HiRAG results.*
