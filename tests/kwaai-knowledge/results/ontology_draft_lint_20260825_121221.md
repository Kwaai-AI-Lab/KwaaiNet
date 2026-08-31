# Arm C draft quality — 2026-08-25T19:12:21Z

Deterministic guards over induced drafts. Grounding-by-quotation is
necessary but not sufficient — a real quote attached to a predicate it
does not instantiate still grounds. These catch that.

`usable_score` 1.0 = no duplicate predicates, no vague predicates, no
reused evidence, no range collapse. Below ~0.6 the draft needs rewriting
rather than editing.

| kb | domain | ents | rels | grounded | dupes | vague | reused ev | range collapse | usable |
|---|---|---|---|---|---|---|---|---|---|
| Legal | US Supreme Court Law | 9 | 12 | 100% | 0 | 1 | 3 | 25% | **0.917** |
| Astrophysics | Space Exploration and Astronomy | 7 | 10 | 94% | 0 | 2 | 4 | 30% | **0.85** |
| Climate | Climate Science | 9 | 15 | 100% | 6 | 4 | 3 | 47% | **0.735** |

## Detail

### Legal — 0.917

- vague predicates (the `associated_with` problem renamed): `has_effect`
- 3 quote(s) justifying more than one predicate

### Astrophysics — 0.85

- vague predicates (the `associated_with` problem renamed): `has_effect`, `is_related_to`
- 4 quote(s) justifying more than one predicate

### Climate — 0.735

- duplicate predicates: `contributes_to`, `affects`, `influences`, `exceeds`, `triggers`, `reduces`
- vague predicates (the `associated_with` problem renamed): `affects`, `influences`, `affects`, `influences`
- 3 quote(s) justifying more than one predicate
