# Ontology bakeoff — arm A (control) — 2026-08-25T19:04:55Z

Instrument-independent metrics read straight from the built graphs
(`PerKBOntology-plan.md` §5.4). No eval run, no rebuild.

`graph score` is deliberately absent: it is hardcoded to the memoir
ontology (plan §1.5) and cannot compare arms.

| kb | entities | relations | rel/ent | escape-hatch | kinship | ent types | rel types |
|---|---|---|---|---|---|---|---|
| Astrophysics | 6437 | 23 | 0.0036 | 34.8% | 0.0% | 3 | 5 |
| Climate | 1979 | 4 | 0.0020 | 0.0% | 0.0% | 3 | 3 |
| CountryHistory | 7613 | 68 | 0.0089 | 44.1% | 0.0% | 3 | 6 |
| D6 | 1510 | 340 | 0.2252 | 8.5% | 56.5% | 5 | 19 |
| D6_10pct_A | 0 | 0 | — | — | — | 0 | 0 |
| D6_ord_A | 0 | 0 | — | — | — | 0 | 0 |
| D6_ord_B | 0 | 0 | — | — | — | 0 | 0 |
| D6_ord_C | 0 | 0 | — | — | — | 0 | 0 |
| DeepSea | 2358 | 0 | 0.0000 | — | — | 3 | 0 |
| DreamMem | 2968 | 8 | 0.0027 | 0.0% | 0.0% | 3 | 4 |
| Legal | 2955 | 2 | 0.0007 | 0.0% | 0.0% | 3 | 1 |
| Manhattan | 725 | 16 | 0.0221 | 18.8% | 0.0% | 3 | 5 |
| Meetings | 728 | 3 | 0.0041 | 0.0% | 0.0% | 2 | 2 |
| MetroWinProbe | 0 | 0 | — | — | — | 0 | 0 |
| MobyDick | 12183 | 275 | 0.0226 | 31.6% | 0.0% | 3 | 7 |
| NIST | 5182 | 7 | 0.0014 | 0.0% | 0.0% | 3 | 5 |
| OSMDocs | 240 | 0 | 0.0000 | — | — | 1 | 0 |
| Poems | 7163 | 4 | 0.0006 | 25.0% | 0.0% | 2 | 3 |
| PythonDocs | 998 | 3 | 0.0030 | 0.0% | 0.0% | 2 | 3 |
| RFCs | 1928 | 11 | 0.0057 | 0.0% | 0.0% | 2 | 2 |
| WarPeace | 3624 | 742 | 0.2047 | 32.4% | 0.0% | 3 | 6 |
| ragbench | 0 | 0 | — | — | — | 0 | 0 |

## Per-KB vocabulary in use

### Astrophysics

**Entity types (3):** Organization 2214, Person 2164, Publication 2059

**Relation types (5):** associated_with 8, part_of 7, works_at 5, contains 2, described_in 1

### Climate

**Entity types (3):** Publication 1163, Organization 655, Legislation 161

**Relation types (3):** works_at 2, part_of 1, cites 1

### CountryHistory

**Entity types (3):** Person 3123, Organization 2777, Place 1713

**Relation types (6):** associated_with 30, located_in 16, part_of 10, belongs_to 8, works_at 3, contains 1

### D6

**Entity types (5):** Person 712, Organization 320, Place 314, Publication 116, Legislation 48

**Relation types (19):** located_in 77, child_of 50, parent_of 50, associated_with 29, spouse_of 18, sibling_of 16, half_sibling_of 14, grandchild_of 14, grandparent_of 14, belongs_to 13, member_of 10, lived_in 8, foster_child_of 8, foster_parent_of 8, part_of 4, works_at 2, romantic_interest_of 2, affiliated_with 2, founded 1

### D6_10pct_A

**Entity types (0):** (none)

**Relation types (0):** (none)

### D6_ord_A

**Entity types (0):** (none)

**Relation types (0):** (none)

### D6_ord_B

**Entity types (0):** (none)

**Relation types (0):** (none)

### D6_ord_C

**Entity types (0):** (none)

**Relation types (0):** (none)

### DeepSea

**Entity types (3):** Person 1207, Publication 773, Organization 378

**Relation types (0):** (none)

### DreamMem

**Entity types (3):** Person 1868, Publication 904, Organization 196

**Relation types (4):** cites 5, works_at 1, located_in 1, part_of 1

### Legal

**Entity types (3):** Person 1241, Legislation 968, Organization 746

**Relation types (1):** contains 2

### Manhattan

**Entity types (3):** Organization 272, Person 267, Place 186

**Relation types (5):** works_at 6, located_in 5, associated_with 3, part_of 1, belongs_to 1

### Meetings

**Entity types (2):** Organization 470, Person 258

**Relation types (2):** works_at 2, contains 1

### MetroWinProbe

**Entity types (0):** (none)

**Relation types (0):** (none)

### MobyDick

**Entity types (3):** Person 6725, Place 2859, Organization 2599

**Relation types (7):** located_in 140, associated_with 87, part_of 22, works_at 16, belongs_to 6, contains 3, visited 1

### NIST

**Entity types (3):** Publication 2992, Organization 1550, Legislation 640

**Relation types (5):** cites 2, contains 2, works_at 1, part_of 1, located_in 1

### OSMDocs

**Entity types (1):** Organization 240

**Relation types (0):** (none)

### Poems

**Entity types (2):** Person 4915, Place 2248

**Relation types (3):** located_in 2, associated_with 1, part_of 1

### PythonDocs

**Entity types (2):** Publication 761, Organization 237

**Relation types (3):** part_of 1, contains 1, works_at 1

### RFCs

**Entity types (2):** Publication 1626, Organization 302

**Relation types (2):** cites 8, part_of 3

### WarPeace

**Entity types (3):** Person 2451, Place 722, Organization 451

**Relation types (6):** located_in 404, associated_with 240, belongs_to 54, part_of 29, works_at 11, contains 4

### ragbench

**Entity types (0):** (none)

**Relation types (0):** (none)
