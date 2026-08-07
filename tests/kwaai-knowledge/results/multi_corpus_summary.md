# Multi-Corpus RAG Eval Summary

Baseline: D6 = 88.9% (v0.4.148 seed6)

| KB | Score | Questions | Entity types | Date | Notes |
|----|-------|-----------|--------------|------|-------|
| D6 | 88.9% | 36 | Person,Place,Org,Legislation,Publication | 2026-07-02 | Baseline |
| ragbench | 88.9% | 7 scored | — | 2026-07-08 | 7/30 have keywords |
| Manhattan | 65.8% | 20 | Person,Place,Organization | 2026-07-08 | |
| MobyDick | 0.0% | 0/78 | Person,Place,Organization | 2026-07-09 | |
| Legal | 0.0% | 0/80 | Person,Organization,Legislation | 2026-07-09 | |
| Meetings | 0.0% | 0/59 | Person,Organization | 2026-07-09 | |
| PythonDocs | 0.0% | 0/63 | Organization,Publication | 2026-07-09 | |
| NIST | 0.0% | 0/72 | Organization,Legislation,Publication | 2026-07-09 | |
| Climate | 0.0% | 0/62 | Organization,Legislation,Publication | 2026-07-09 | |
| RFCs | 0.0% | 0/78 | Organization,Publication | 2026-07-09 | |
| MobyDick (semantic) | ret=?% gen=?% | — | Person,Place,Organization | 2026-07-09 | --semantic-score --inference-url metro-linux |
| Legal (semantic) | ret=90.6% gen=0.0% | — | Person,Organization,Legislation | 2026-07-09 | --semantic-score --inference-url metro-linux |
| Meetings (semantic) | ret=61.5% gen=0.0% | — | Person,Organization | 2026-07-09 | --semantic-score --inference-url metro-linux |
| PythonDocs (semantic) | ret=73.9% gen=0.6% | — | Organization,Publication | 2026-07-09 | --semantic-score --inference-url metro-linux |
| NIST (semantic) | ret=87.2% gen=0.0% | — | Organization,Legislation,Publication | 2026-07-09 | --semantic-score --inference-url metro-linux |
| Climate (semantic) | ret=80.2% gen=0.6% | — | Organization,Legislation,Publication | 2026-07-09 | --semantic-score --inference-url metro-linux |
| DeepSea | ret=?% gen=?% | — | Person,Organization,Publication | 2026-07-09 | --semantic-score |
| DeepSea | 0.0% | 0/58 | Person,Organization,Publication | 2026-07-09 | |
| DreamMem | ?% | ?/? | Person,Organization,Publication | 2026-07-09 | |
| DreamMem | ret=80.3% gen=0.0% | — | Person,Organization,Publication | 2026-07-09 | --semantic-score |
| Astrophysics | ret=?% gen=?% | — | Person,Organization,Publication | 2026-07-09 | --semantic-score |
| MobyDick (semantic) | ret=?% gen=?% | — | Person,Place,Organization | 2026-07-09 | --semantic-score |
| Legal (semantic) | ret=?% gen=?% | — | Person,Organization,Legislation | 2026-07-09 | --semantic-score |
| MobyDick (semantic) | ret=87.0% gen=32.5% | — | — | 2026-07-09 | sem-low=0.3 |
| Legal (semantic) | ret=94.4% gen=30.9% | — | — | 2026-07-09 | sem-low=0.3 |
| Astrophysics | ret=0.0% gen=31.3% | — | Person,Organization,Publication | 2026-07-09 | sem-low=0.3 |
| Manhattan | 73.4% | 0/79 | paragraph | — | 2026-08-04 | |
| MobyDick | 74.4% | 0/78 | paragraph | — | 2026-08-04 | |
| Legal | 77.5% | 0/80 | paragraph | — | 2026-08-04 | |
| Meetings | 54.2% | 0/59 | paragraph | — | 2026-08-04 | |
| PythonDocs | 58.7% | 0/63 | paragraph | — | 2026-08-04 | |
| NIST | 69.4% | 0/72 | paragraph | — | 2026-08-04 | |
| Climate | 61.3% | 0/62 | paragraph | — | 2026-08-04 | |
| RFCs | 71.8% | 0/78 | paragraph | — | 2026-08-04 | |
| DeepSea | 60.3% | 0/58 | paragraph | — | 2026-08-04 | |
| DreamMem | 68.7% | 0/67 | paragraph | — | 2026-08-04 | |
| Astrophysics | 62.7% | 0/75 | paragraph | — | 2026-08-04 | |
