# Predicates as vectors — exploration

2026-08-26. Prompted by Reza: entities can be deduplicated and coreferenced
because they have *identity*; predicates cannot, because what they have is
*meaning*. `nursed_by` and `cared_for` are not one edge under two names — they
are neighbouring points in a space. Coercing the first to `associated_with`
discards the position.

Measured with `nomic-embed-text` (768-d) over D6's 43 declared predicates and
the 59 the ontology rejected. Data in `results/predicate_nn.json`, script
`tests/kwaai-knowledge/predicate_vectors.py`.

---

## 1. Where it works

**Alias discovery is nearly free.** The top of the nearest-neighbour ranking is
the alias table I hand-authored the day before, rediscovered:

```
brother_of   → sibling_of       0.922
founded_by   → founded          0.918
visited_with → visited          0.910
lived_in     → lived_at         0.870
worked_for   → worked_at        0.815
fostered_by  → foster_child_of  0.810
```

A ≥0.80 threshold proposes 11 of the 38 aliases unprompted. Authoring those by
hand took about an hour of reading the control arm's output.

**Clustering the residue induces the missing vocabulary.** The 59 rejected
predicates form coherent families at cos ≥ 0.62:

| cluster | reading |
|---|---|
| `authored, published_by, published_in, read` | publishing / authorship |
| `lived_in, migrated_to, immigrated_to` | movement and settlement |
| `friend_of, acquaintance_of` | social tie strength |
| `transported_by, transported_to` · `distributed, handed_over_to` | conveyance |
| `disagreed_with, spoke_to, spoke_with` | speech acts |

This is AutoSchemaKG-style induction running on extraction *residue* rather than
a fresh corpus pass — free, because the data is already generated on every build
and was previously discarded.

## 2. Where it fails, and it is disqualifying for one use

**The space does not encode polarity.**

```
supported    vs opposed    0.694     ← above the 0.624 median
advocated_by vs opposed    0.650
parent_of    vs child_of   0.758
```

Nearest-neighbour routing would map "supported the Unity Movement" onto
`opposed`. **Inverting an assertion is worse than losing it**: a fallback edge is
honestly vague, a wrong-polarity edge is a confident falsehood the graph will
serve. `parent_of`/`child_of` at 0.758 is the same failure in the relation D6 is
densest in, where direction is the entire content.

## 3. Conclusion — three roles, not a replacement

Vectors *alongside* symbols, never instead of them:

1. **Alias proposal at authoring time**, human-ratified. High similarity is
   reliable and a reviewer sees the polarity trap on a short list.
2. **Induction from residue** — cluster what the ontology rejected and surface
   the families as candidate predicates. Build this first; it is pure gain and
   needs no new trust.
3. **Retrieval-side matching** — "who looked after him?" reaching `nursed_by`.
   Safe because retrieval ranks rather than asserts.

**Not** automatic write-time coercion, on the `supported`/`opposed` evidence.

## 4. Implemented now, as the cheap fix

`RelationRecord.original_predicate: Option<String>` — coercion records what the
extractor actually emitted. The closed vocabulary still governs traversal and
axioms; the nuance survives beside it. `GraphStore::coercion_residue()` returns
those predicates with counts, most frequent first: the schema's own to-do list,
generated on every build.

This solves the irreversibility without taking on any of the vector risk.

## 5. Open — the embedder

`nomic-embed-text` is a general-purpose *text* embedder. It was never trained to
represent relations, which is the likeliest explanation for the antonym failure:
`supported` and `opposed` share every topical feature and differ only in a
direction the training objective had no reason to preserve.

Reza, 2026-08-26: we probably need our own embedder, or one fit for purpose.
Before training anything, the cheap checks are worth running:

- **Off-the-shelf relation-aware models.** Sentence-transformers trained on NLI
  separate contradiction from entailment by construction, which is close to the
  polarity property we need. Worth measuring against the same 59-predicate set
  before assuming a custom model is required.
- **Embed the evidence, not the label.** A predicate's meaning may be better
  carried by the sentences it was extracted from than by its name. We already
  store `evidence_chunk_ids` on every edge, so this is testable now and costs
  nothing to try.
- **Direction as a separate axis.** Polarity and inverse may not belong in the
  same vector at all — the ontology already declares `inverse:` and
  `contradiction` axioms symbolically, and those are exactly the facts the
  embedding gets wrong. A hybrid where symbols carry direction and vectors carry
  topic may beat either alone.

The 59-predicate set with hand labels is the ready-made benchmark for whichever
route is tried.
