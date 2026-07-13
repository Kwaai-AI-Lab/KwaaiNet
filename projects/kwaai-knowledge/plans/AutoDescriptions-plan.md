# Plan: Auto-generate entity descriptions from text (remove YAML seeds)

## Context

D6 eval is at 90.7% (r53, 204/225). Entity descriptions in `d6_family_tree.yaml` are
manually curated — they don't scale to new documents and prevent the system from being
general-purpose. The goal is to remove all hand-written `description:` fields and have
`enrich-entities` generate them automatically from the source text.

`enrich-entities` already exists and is nearly sufficient. Three changes make it production-ready.

---

## Change 1 — Fix seed embedding regression in `family.rs`

**File**: `core/crates/kwaai-rag/src/family.rs` lines 94–99

**Problem**: When YAML has no `description` field, seed embeds the entity using only its
canonical name — overwriting any previously-enriched description's embedding. This would
happen whenever `seed` is re-run after `enrich-entities` (e.g. to add a new YAML relation).

**Fix**: Use the *effective* description (YAML if present, otherwise existing in graph) for
the embedding text:

```rust
// Before
let embed_text = if desc.is_empty() {
    person.canonical.clone()
} else {
    format!("{}: {}", person.canonical, desc)
};

// After
let effective_desc = if !desc.is_empty() {
    desc.clone()
} else {
    existing.as_ref().map(|e| e.description.clone()).unwrap_or_default()
};
let embed_text = if effective_desc.is_empty() {
    person.canonical.clone()
} else {
    format!("{}: {}", person.canonical, effective_desc)
};
```

Note: `merge_entity_into` already correctly transfers chunk links from alias entities to
canonical entities (graph.rs lines 1678–1775) — no change needed there.

---

## Change 2 — Improve enrichment prompts in `enrich.rs`

**File**: `core/crates/kwaai-rag/src/enrich.rs`

### 2a — Richer Person prompt (lines 350–360)

Change `"2-3 sentences"` → `"3-5 sentences"` in the JSON-mode Person prompt:

```
"description": "<3-5 sentences: who {name} is, their background and role in the story,
significance to the family or community, and key relationships — text-grounded only>"
```

### 2b — Type-specific Place and Organization prompts (lines 363–372)

Replace the single generic non-Person prose prompt with type-specific variants:

```rust
let (sentence_count, type_guidance) = match entity_type.to_lowercase().as_str() {
    "place" | "location" => (
        "3–5",
        "Include: (1) physical description and geographic setting, \
         (2) historical context or origin, \
         (3) its role in the story or family history, \
         (4) notable persons or events associated with it.",
    ),
    "organization" => (
        "3–5",
        "Include: (1) what type of organisation it is and its purpose, \
         (2) historical significance and activities, \
         (3) relationship to the apartheid struggle or political context, \
         (4) affiliated persons.",
    ),
    _ => (
        "2–3",
        "Capture who or what it is, its significance in the story, \
         and key relationships or roles.",
    ),
};
```

### 2c — Increase max_tokens (line 460)

Change `"max_tokens": 400` → `"max_tokens": 600` to prevent truncation on longer
Place/Org descriptions.

---

## Change 3 — Strip descriptions from `d6_family_tree.yaml`

**File**: `tests/kwaai-knowledge/d6_family_tree.yaml`

Remove all `description:` fields from every entity entry (~38 entities). Keep:
- `canonical`, `aliases`, `entity_type`, `gender` — structural ground truth
- All `relations:` entries — family tree is still needed

The seed code (family.rs lines 121–127) already falls through to `existing.description`
when the YAML field is absent, so re-seeding won't erase enriched descriptions after
Change 1 is applied.

---

## Verification

**Build**:
```bash
cd /Users/rezarassool/Source/KwaaiNet/core
cargo build -p kwaainet --release
cp target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet
```

**Pipeline** (seed BEFORE enrich — order matters):
```bash
# Re-seed with stripped YAML (aliases + relations only, no descriptions)
kwaainet rag graph seed --kb D6 --file tests/kwaai-knowledge/d6_family_tree.yaml

# Auto-generate descriptions from evidence chunks + re-embed
kwaainet rag graph enrich-entities \
  --kb D6 --force --model llama3.1:8b --workers 4 \
  --inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs" \
  --entity-types "Person,Place,Organization"

# Spot-check descriptions for 3 key entities
kwaainet rag graph show --kb D6 --entity "7 Buitencingle Street"
kwaainet rag graph show --kb D6 --entity "Haji Joosub Maulvi Hamid Gool"
kwaainet rag graph show --kb D6 --entity "Non-European Unity Movement"

# Full eval
kwaainet rag eval \
  --kb D6 \
  --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --mode smart --biographical-expansion
```

**Regression gate**: Score ≥ 199/225 (88.4%, the r50 baseline).
Current best: 204/225 (90.7%). Log results in `tests/kwaai-knowledge/results/eval_log.md`.

**Rollback**: `git checkout tests/kwaai-knowledge/d6_family_tree.yaml` then re-seed.

---

## Future (Phase 2 — out of scope here)

- Integrate `enrich-entities` into `kwaainet rag rebuild` as a standard pipeline step
- Remove D6-specific YAML entirely once enrichment quality is validated
- Support arbitrary documents without any YAML: entity types inferred from doc schema
