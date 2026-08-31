# Arm C draft quality — 2026-08-25T19:22:48Z

Deterministic guards over induced drafts. Grounding-by-quotation is
necessary but not sufficient — a real quote attached to a predicate it
does not instantiate still grounds. These catch that.

`usable_score` 1.0 = no duplicate predicates, no vague predicates, no
reused evidence, no range collapse. Below ~0.6 the draft needs rewriting
rather than editing.

| kb | domain | ents | rels | grounded | dupes | vague | reused ev | range collapse | usable |
|---|---|---|---|---|---|---|---|---|---|
| Legal | US Supreme Court Jurisprudence | 0 | 0 | 0% | 0 | 0 | 0 | 0% | **1.0** |
| OSMDocs | OpenStreetMap Wiki | 4 | 4 | 38% | 0 | 0 | 0 | 25% | **1.0** |
| RFCs | Network Protocols and Standards | 7 | 6 | 68% | 0 | 0 | 0 | 33% | **1.0** |
| WarPeace | War and Peace | 7 | 10 | 85% | 0 | 0 | 1 | 40% | **0.952** |
| Meetings | Neural Knowledge Representation | 8 | 13 | 100% | 0 | 0 | 3 | 23% | **0.942** |
| DeepSea | Deep-Sea Ecology and Biology | 8 | 12 | 71% | 1 | 0 | 3 | 17% | **0.917** |
| MobyDick | Whaling and Maritime History | 8 | 8 | 100% | 0 | 0 | 1 | 50% | **0.908** |
| NIST | Artificial Intelligence and Machine Learning Security | 8 | 14 | 100% | 0 | 0 | 6 | 36% | **0.886** |
| Astrophysics | Space Exploration and Astronomy | 7 | 10 | 94% | 0 | 2 | 4 | 30% | **0.85** |
| DreamMem | Sleep and Memory Consolidation | 4 | 7 | 52% | 0 | 4 | 1 | 29% | **0.821** |
| Poems | Poetry and Dramatic Works | 7 | 11 | 86% | 4 | 1 | 4 | 36% | **0.787** |
| CountryHistory | Indian History and Culture | 1 | 1 | 14% | 0 | 0 | 0 | 100% | **0.75** |
| Climate | Climate Science | 9 | 15 | 100% | 6 | 4 | 3 | 47% | **0.735** |

## Detail

### Legal — 1.0

- clean

### OSMDocs — 1.0

- clean

### RFCs — 1.0

- clean

### WarPeace — 0.952

- 1 quote(s) justifying more than one predicate

### Meetings — 0.942

- 3 quote(s) justifying more than one predicate

### DeepSea — 0.917

- duplicate predicates: `is_part_of`
- 3 quote(s) justifying more than one predicate

### MobyDick — 0.908

- 1 quote(s) justifying more than one predicate

### NIST — 0.886

- 6 quote(s) justifying more than one predicate

### Astrophysics — 0.85

- vague predicates (the `associated_with` problem renamed): `has_effect`, `is_related_to`
- 4 quote(s) justifying more than one predicate

### DreamMem — 0.821

- vague predicates (the `associated_with` problem renamed): `involves`, `has_effect`, `is_associated_with`, `is_influenced_by`
- 1 quote(s) justifying more than one predicate

### Poems — 0.787

- duplicate predicates: `features`, `takes_place`, `expresses`, `is_about`
- vague predicates (the `associated_with` problem renamed): `involves`
- 4 quote(s) justifying more than one predicate

### CountryHistory — 0.75

- 100% of predicates share one range — degenerate

### Climate — 0.735

- duplicate predicates: `contributes_to`, `affects`, `influences`, `exceeds`, `triggers`, `reduces`
- vague predicates (the `associated_with` problem renamed): `affects`, `influences`, `affects`, `influences`
- 3 quote(s) justifying more than one predicate
