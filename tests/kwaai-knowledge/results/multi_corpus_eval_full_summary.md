# Multi-Corpus RAG Eval Summary — Full (retrieval + generation + LLM judge)

Method: token-overlap + semantic (cosine; low=0.30, high=0.85) + --llm-judge
Compare against projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md

| KB | Retrieval | Generation | Judge (avg/2) | Date |
|----|-----------|------------|---------------|------|
| Manhattan | 91.3% | 75.4% | 1.60/2 | 2026-08-04 |
| MobyDick | 88.6% | 86.1% | 1.60/2 | 2026-08-04 |
| Legal | 94.0% | 85.1% | 1.85/2 | 2026-08-04 |
| Meetings | 77.6% | 68.5% | 1.20/2 | 2026-08-04 |
| PythonDocs | 88.1% | 73.6% | 1.60/2 | 2026-08-04 |
| NIST | 94.3% | 81.8% | 1.65/2 | 2026-08-04 |
| Climate | 86.3% | 73.5% | 1.45/2 | 2026-08-04 |
| RFCs | 90.0% | 79.1% | 1.55/2 | 2026-08-04 |
| DeepSea | 90.2% | 69.6% | 1.75/2 | 2026-08-04 |
| DreamMem | 88.5% | 71.7% | 1.80/2 | 2026-08-04 |
| Astrophysics | 89.5% | 77.1% | 1.65/2 | 2026-08-04 |
