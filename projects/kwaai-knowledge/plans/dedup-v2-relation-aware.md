# Dedup V2 — Relation-Aware + Description-Aware Entity Deduplication

**Status:** Phase 3 implemented  
**Owner:** kwaai-rag  
**Graph snapshot:** `D6_person_full_dream5_20260604` → now labelled **dedup_v1**  
**Entity count at freeze:** 1051 entities, 150 relations

## Implementation status

| Feature | Status | Location |
|---------|--------|----------|
| R1 — contradict family role block | ✅ Complete | `graph.rs:dedup_block_r1` |
| R2 — half-sibling block (shared parent, different co-parent) | ✅ Complete | `graph.rs:dedup_block_r2` |
| R3 — high-risk surname deferred (no matching family relation) | ✅ Complete | `graph.rs:dedup_r3_high_risk_surname` |
| Description divergence block (Jaccard < 12% → cap to 0.94) | ✅ Complete | `graph.rs:dedup_desc_diverges` |
| `[BLOCKED:DESC]` label in dry-run + auto-merge output | ✅ Complete | `rag_cmd.rs` |
| `pinned: true` YAML seed override for OCR typo canonicals | ⏳ Planned | — |
| Cross-chunk relation support counter | ⏳ Planned | — |

### Description-aware dedup (new, 2026-06-09)

`dedup_desc_diverges(a, b)` — activates when both entities have descriptions ≥100 chars (i.e., after `enrich-entities`). Computes Jaccard word-overlap on significant words (≥4 chars, excluding stop words). If Jaccard < 0.12 (< 12% of unique words shared), descriptions clearly describe different people → sim is capped at 0.94 in Tier 2, preventing auto-merge at the 0.97 threshold.

**Why Jaccard not embedding similarity:** The entity embedding after `reembed` already encodes description content. Jaccard adds a complementary signal: raw word co-occurrence catches explicit shared names, roles, and places that embedding similarity may blur together when context is similar (e.g., two family members in the same household both mention "District Six" and "Gool family" → embeddings cluster → but their descriptions mention different names and roles → Jaccard diverges).

---

## Phase 1 — Freeze and failure-mode audit

### Current dedup architecture (dedup_v1)

| Tier | Function | Signal | Auto? |
|------|----------|--------|-------|
| 1 | `find_dedup_candidates_exact` | Normalized-name equality | Always |
| 2 | `find_dedup_candidates` | Embedding cosine sim ≥ threshold (default 0.97 auto, 0.85 review) | Configurable |
| 3A | `find_dedup_candidates_name_structure` → honorific | Stripped honorific match | Always |
| 3A2 | → alias_match | Entity name = alias on another entity | Always |
| 3B | → subset | Word-set subset + ≥2 shared neighbors | Always |
| 3C | → fuzzy | Edit distance ≤ 2 within shared-token bucket | Always |
| 3D | → qualification | Trailing degree suffix stripped | Always |
| 3E | → first_name_only | Single word = unique first token of one multi-word entity | Always |
| 4 | `find_dedup_candidates_neighbor_containment` | Role-word entity with ≥60% neighbor overlap | Review only |

**Freeze rule:** Do not add further Tier 3/3E name-only sub-rules until Phase 3 relation guards are in place.

---

### Known bad merges in dedup_v1

#### Already fixed (unmerged in last session)

| Alias → Canonical | Tier that fired | Root cause | Relation signal that would block |
|-------------------|----------------|------------|----------------------------------|
| `I.B. Tabata` → `Jane Gool-Tabata` | Tier 2 (sim=0.984) | Both have "Tabata" token; sparse descriptions cluster in embedding space | Jane has `child_of` Wahida/JMH Gool — I.B. has no family relations. Parent divergence → block. |
| `Nasim Rassool` → `Nazima Rassool` | Tier 3 A2 (alias_match) | "Nasim Rassool" was accidentally stored as alias on Nazima after an earlier Tier 2 merge | Joe Rassool has `sibling_of Nasim` AND `spouse_of Nazima`. Merging Nasim→Nazima creates X is both sibling_of and spouse_of the same person → hard contradiction → block. |

#### Still in graph — need to be fixed

| Alias → Canonical | Tier that fired | Root cause | Fix |
|-------------------|----------------|------------|-----|
| `Mohamed Saaid Gool` → `Mohammed Hanief Gool` | Tier 3 A2 (alias_match) | Mohamed Saaid's aliases were incorrectly loaded onto Mohammed Hanief via an earlier embedding similarity merge. Once stored as aliases, alias_match confirmed the merge. | **Unmerge now.** Distinguishing signal: both are `child_of` JMH Gool but via **different mothers** — Mohamed Saaid via Bibi, Mohammed Hanief via Wahida. Same-parent-different-other-parent = half-siblings, not duplicates. |
| `Haji Joosub Maulvi Hamid Gool` → `Haji Joosub Maulvi Hamid Gooli` (OCR typo canonical) | Tier 1 (via alias_match at seed) | OCR produced "Gooli" many times (104 mentions). Tier 1/alias_match made the typo form the canonical because it had more mentions. | **Deferred to Phase 3** — needs `pinned: true` seed override. Entity is correctly resolved (both names are the same person); only the display name has the OCR typo. No CLI rename command exists today. |
| `Mohammed Hanief Gool` description contaminated with "Uncle Aity Mohamed Saaid Gool" text | Artifact of bad Tier 2 merge | When Mohamed Saaid was wrongly merged into Mohammed Hanief, his description became Mohammed Hanief's. Unmerge does not restore the original description. | **Deferred** — description is wrong but aliases and relations are clean. Fix in Phase 2 when relations add a clean description source. |

---

### Near-misses catalog (blocked but structurally fragile)

These pairs were NOT merged in dedup_v1 but represent the highest-risk area for future runs.

| Pair | Tier 2 sim | Why dangerous | What currently blocks | Relation signal that would add certainty |
|------|-----------|---------------|----------------------|------------------------------------------|
| `Hamid Khan` ↔ `Abdul Hamid Gool` | 0.960 | Share "Hamid" token; similar 1920s Cape Town context | JW distance gate | Abdul Hamid has `child_of` Wahida Gool, `spouse_of` Cissie Gool. Hamid Khan has no family links. Parent presence vs absence → strong evidence of distinction. |
| `Ruth Rassool` ↔ `Rebecca Rassool` | ~0.95 | Same surname, overlapping contexts | JW gate (different first names) | Both are in the Rassool family but Joe Rassool has `sibling_of` links to specific people. Cross-checking sibling_of chains would distinguish them. |
| `Peter Abrahams` ↔ `Hassen Abrahams` | ~0.92 | Same surname, literary/political context | JW gate | No shared relations — currently unresolvable without external signal. |
| Generals cluster (10 pairs: Zhukov/Paulus/MacArthur/…) | 0.960 | All share "General" title; similar military contexts | Title-prefix guard in Tier 2 | No family relations → not a blocker, just identity signal. Unresolvable via family relations; requires affiliation (military service) to distinguish. |
| Cross-Gool-family pairs (5 pairs) | 0.60–0.67 | Shared "Gool" surname + overlapping political context | Low sim (below 0.85 threshold) | Family relations are the definitive discriminator. E.g., Abdul Hamid ≠ Goolam because different `child_of` sets when both parents are tracked. |

---

### Root cause analysis by failure mode

#### Failure mode A: Embedding similarity collapse for same-family entities

Entities from the same family share context (same text, same locations, same events) making their embeddings cluster together even when they are different people. The Gool and Rassool families are extreme cases — nearly every member co-occurs with every other member.

**Current mitigation:** JW gate on token overlap.  
**Gap:** JW gate is calibrated for names, not for family-context embedding collapse.  
**Phase 3 fix:** For entities that share a surname known to be a "high-density family" (Gool, Rassool), require at least one matching relation before auto-merging.

#### Failure mode B: Alias propagation via earlier bad Tier 2 merge

Once a bad Tier 2 merge puts entity A's aliases onto entity B, Tier 3 A2 (alias_match) will *confirm* the bad merge on the next dedup run. The merge is no longer detectable as bad without external ground truth.

**Current mitigation:** None — dedup is not idempotent in this failure mode.  
**Phase 3 fix:** When alias_match fires, cross-check that the alias entity and the canonical don't have contradictory family relations before confirming.

#### Failure mode C: OCR/extraction typo becoming canonical

The entity with more extracted mentions wins "canonical" in Tier 1/alias_match even if its name is a typo. "Gooli" (104 mentions) > "Gool" (seed entity).

**Current mitigation:** Manual review; the seed's canonical name is just an alias.  
**Phase 3 fix:** Seed entities should always win canonical regardless of mention count. Add a `seed_canonical: true` flag to pinned YAML entities that overrides Tier 1/alias_match direction.

---

## Phase 2 — Conservative relation extraction for dedup features

### Goal

Build just enough relational structure to support dedup confidence adjustments and blocking rules. Precision matters more than recall: a wrong relation plants false blocking signals.

### Relation set (minimal)

| Relation | Directional? | Lexical triggers (strict) | Risk |
|----------|-------------|--------------------------|------|
| `spouse_of` | Symmetric | "wife of", "husband of", "married to", "wed", "his wife X", "her husband X" | Low — triggers are specific |
| `parent_of` / `child_of` | Directed | "son of", "daughter of", "father of", "mother of", "born to", "child of" | Low — high precision |
| `sibling_of` | Symmetric | "brother of", "sister of", "siblings", "X and Y were sisters/brothers" | Medium — "X and Y" patterns need both endpoints anchored |
| `member_of` (optional) | Directed | "member of [org]", "joined [org]", "elected to [org]" | Skip Phase 2 — too hard to anchor without org entity resolution |

### Extraction approach

Use the **existing LLM-based extraction pipeline** (`graph.rs` extract_relations_llm) but with:

1. **Stricter entity anchoring:** only extract a relation when both endpoints resolve to ≥1 entity already in the graph (no phantom-entity relations).
2. **Lexical pre-filter:** before calling the LLM, check the chunk text for at least one trigger phrase from the table above. Skip chunks with no triggers (saves inference cost).
3. **Confidence filter:** only promote relations where the same (A, relation, B) triple is supported by ≥2 separate chunks, OR the relation was seeded from the YAML.

### What to reuse

The `family.rs` seed and YAML-based relation planting already provide high-confidence ground-truth relations for the Gool/Rassool family. These do not need re-extraction — they are Phase 2's starting point.

### What to add (new Phase 2 work)

- **Lexical trigger pre-filter** on chunk text before LLM relation extraction (reduces noise).
- **Cross-chunk support counter:** relations are marked `high_confidence` only after appearing in ≥2 chunks.
- **Contradition detector:** after extracting, flag (do not auto-remove) triples that would create a logical contradiction in the graph (e.g., X is both `spouse_of` and `parent_of` the same node; X has two `spouse_of` links to different nodes without a `divorced_from` link).

---

## Phase 3 — Relation-aware dedup rules

### New function: `find_dedup_relation_blocks`

Called at the start of each dedup tier before merging. Returns `(alias_id, canonical_id, reason)` pairs that should be **blocked** from merging, even if another tier nominated them.

#### Block rule R1 — Contradictory role to shared third entity

If merging A→B would result in a single entity being simultaneously:
- `spouse_of X` AND `child_of X` for the same X
- `spouse_of X` AND `sibling_of X` for the same X (the Nasim/Nazima case)
- `child_of X` AND `parent_of X` for the same X

→ **BLOCK** unconditionally.

#### Block rule R2 — Same parent set, different second parent

If A and B are both `child_of` the same node P, but:
- A is also `child_of` M1 (different from M2)
- B is also `child_of` M2 (different from M1)
- M1 ≠ M2

→ **BLOCK**. A and B are half-siblings, not the same person.  
_(This would have blocked Mohamed Saaid Gool → Mohammed Hanief Gool.)_

#### Block rule R3 — High-risk surname without matching relation evidence

For entities whose normalized surname is in `HIGH_RISK_SURNAMES = ["gool", "rassool", "abdurahman", "tabata"]`:
- Tier 2 auto-merge requires **either**:
  - At least one matching relation with the same third-entity endpoint (e.g., both `child_of` the same parent), **or**
  - Explicit YAML seed entry declaring the merge.
- If neither condition is met, demote from "auto-merge" to "review only", even if sim ≥ auto_threshold.

_(This would have blocked I.B. Tabata → Jane Gool-Tabata: both named Tabata but no shared relations.)_

#### Confidence boost — Same spouse or same parent

When two entities A and B are merge candidates:
- Both have `spouse_of` to the same third entity → **strong positive signal** (boost sim weight)
- Both have identical `child_of` sets (same two parents) → **strong positive signal**
- Both have `member_of` the same organisation with no conflicting family links → **weak positive signal**

#### Implementation sketch

```rust
pub fn find_dedup_relation_blocks(
    &self,
    candidates: &[(i64, i64)],  // (alias_id, canonical_id) pairs
) -> HashSet<(i64, i64)> {
    let mut blocked = HashSet::new();
    for &(alias_id, canonical_id) in candidates {
        if self.would_contradict_role(alias_id, canonical_id) {  // R1
            blocked.insert(ord_pair(alias_id, canonical_id));
        } else if self.same_parent_different_co_parent(alias_id, canonical_id) {  // R2
            blocked.insert(ord_pair(alias_id, canonical_id));
        }
    }
    blocked
}

fn would_contradict_role(&self, a: i64, b: i64) -> bool {
    // Check if merging a→b would make one entity both spouse_of and
    // sibling_of (or child_of) the same third entity.
    let a_rels = self.family_relation_targets(a);
    let b_rels = self.family_relation_targets(b);
    for (target, rel_a) in &a_rels {
        if let Some(rel_b) = b_rels.get(target) {
            if contradicts(rel_a, rel_b) {
                return true;
            }
        }
    }
    false
}

fn same_parent_different_co_parent(&self, a: i64, b: i64) -> bool {
    // Both child_of same parent P, but each also child_of a different
    // other parent → half-siblings.
    let a_parents: HashSet<i64> = self.parents_of(a).collect();
    let b_parents: HashSet<i64> = self.parents_of(b).collect();
    let shared: HashSet<_> = a_parents.intersection(&b_parents).collect();
    if shared.is_empty() { return false; }
    // Have at least one shared parent (plausible duplicate)
    // but also have non-overlapping parents → half-siblings
    let a_only: HashSet<_> = a_parents.difference(&b_parents).collect();
    let b_only: HashSet<_> = b_parents.difference(&a_parents).collect();
    !a_only.is_empty() && !b_only.is_empty()
}
```

### Integration point

`find_dedup_relation_blocks` is called in `run_dedup` after each tier computes its candidates but **before** any merge is applied. Blocked pairs are logged with their reason and excluded from the merge set. They appear in `--dry-run` output so the user can review them.

### Seed canonical pinning (fixes OCR typo canonical)

Add `pinned: true` field to YAML person entries. When the seed upserts a `pinned` entity, it writes a `is_seed_canonical` flag to the entity. In Tier 1 canonical selection, `is_seed_canonical` entities always win over higher-mention entities. This prevents OCR artifacts from stealing the canonical position.

---

## Implementation order

### Immediate (Phase 1 cleanup — done 2026-06-05)

- [x] Unmerge `Mohamed Saaid Gool` from `Mohammed Hanief Gool` — also unmerged bleed-over aliases (Aity, Uncle Aity, etc.), re-seeded to restore them on the correct entity
- [ ] Fix `Haji Joosub Maulvi Hamid Gooli` OCR typo canonical — **deferred to Phase 3** (needs `pinned: true` YAML field; no CLI rename command today)
- [x] Run `graph reembed --kb D6` after fixes — graph now at 1054 entities, 200 relations
- **Freeze confirmed:** no further name-only dedup rules until Phase 3 relation guards are in place

### Phase 2 (next session)

- [ ] Lexical trigger pre-filter in `extract_relations_llm`
- [ ] Cross-chunk support counter on relations (`support_count` field on `RelationRecord`)
- [ ] Mark existing seeded relations as `high_confidence = true` (they already pass precision bar)
- [ ] Contradiction detector (log-only, no auto-removal)

### Phase 3 (after Phase 2 validation)

- [ ] `find_dedup_relation_blocks` → R1, R2 rules
- [ ] R3 high-risk surname guard in Tier 2 auto-merge
- [ ] Confidence boost for same-spouse / same-parent-set evidence
- [ ] `pinned: true` YAML field → seed canonical override in Tier 1

---

## Test cases for Phase 3

Each rule needs a regression test in `graph.rs` unit tests:

| Test | Expected outcome |
|------|-----------------|
| R1: Merge X where X would be both spouse_of and sibling_of Joe Rassool | BLOCKED |
| R2: Merge Mohamed Saaid Gool → Mohammed Hanief Gool | BLOCKED (same parent JMH, different co-parents Bibi/Wahida) |
| R3: Merge I.B. Tabata → Jane Gool-Tabata (sim=0.984) | DOWNGRADED to review-only |
| Positive: Merge "Cissie" → "Cissie Gool" (no relations, unique first-name) | ALLOWED |
| Positive: Merge "J.M.H. Gool" → "Haji Joosub Maulvi Hamid Gool" via alias_match | ALLOWED |
