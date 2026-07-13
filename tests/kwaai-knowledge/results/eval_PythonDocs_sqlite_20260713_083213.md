⚠  Legacy redb store detected for 9027d69a-0352-4eaf-9581-447fb31d3f26. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:50.462454Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 735d29ef-b348-4094-9f05-cc116fe664ef: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
[2m2026-07-13T16:04:50.462582Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:59282 (via ollama-proxy)
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:50.477009Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│               RAG Eval  (20 questions, kb=PythonDocs)               │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:59282
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] According to the Python Language Reference, what is "lexical analysis,
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:51.046743Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 9 documents — passing to LLM

[2m2026-07-13T16:04:51.949817Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 84112c2a-7854-4671-b501-9aeb8c00e7b6: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
[2m2026-07-13T16:04:51.949853Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
         → ret=3/3  gen=0.9/3  916ms
  [ 2/20] What is Python's "data model," and why is it foundational to understan
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:52.238481Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  6/8 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-13T16:04:52.942849Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call a5a93f38-651a-4654-bfe1-09b6aff07c04: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
[2m2026-07-13T16:04:52.942865Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
         → ret=4/4  gen=1.3/4  719ms
  [ 3/20] What does Python's "execution model" (Chapter 4) describe regarding ho
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:53.194181Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-13T16:04:53.875498Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 73053e2e-9571-440d-a5d2-8b0a824b1787: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
[2m2026-07-13T16:04:53.875528Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: failed to dial: failed to dial 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs: all dials failed
  * [/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit] dial backoff
  * [/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY/p2p-circuit] dial backoff
[2m2026-07-13T16:04:53.875576Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
         → ret=2/2  gen=0.7/2  696ms
  [ 4/20] What is Python's import system (Chapter 5), and what mechanism does it
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:54.136185Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  7/9 query terms found  (78%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=3.0/4  gen=1.1/4  641ms
  [ 5/20] According to the Expressions chapter, what is Python's defined order o
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:55.018551Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  10/10 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=2.7/4  gen=1.6/4  532ms
  [ 6/20] What distinguishes a "simple statement" from a "compound statement" in
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:55.766744Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  6/7 query terms found  (86%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.1/4  gen=1.2/4  567ms
  [ 7/20] What is a Python class according to the tutorial's Classes chapter, an
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:56.598478Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  6/9 query terms found  (67%)
  ○ Round 2   gap-filling for [according, concepts, introduce]
  ○ Round 2   added 6 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, concepts, introduce]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=4/4  gen=1.2/4  640ms
  [ 8/20] What does the official Python tutorial's Introduction chapter say abou
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:57.448605Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  6/9 query terms found  (67%)
  ○ Round 2   gap-filling for [official, philosophy, audience]
  ○ Round 2   added 6 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [official, philosophy, audience]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=4/4  gen=1.3/4  891ms
  [ 9/20] What is the purpose of the os module according to the Python 3.14 Libr
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:58.577203Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  7/7 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3/3  gen=0.9/3  538ms
  [10/20] What is the purpose of the sys module, and how does it differ function
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:04:59.332743Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  3/4 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=1.5/2  gen=0.7/2  538ms
  [11/20] Name three built-in functions documented in the Python 3.14 Built-in F
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:00.094906Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3.0/4  gen=1.3/4  739ms
  [12/20] What does the Full Grammar Specification document formally define, and
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:01.102328Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  9/11 query terms found  (82%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3.6/4  gen=1.4/4  671ms
  [13/20] Cross-document: How does the formal grammar in the Full Grammar Specif
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:02.017310Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  9/12 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.5/4  gen=1.2/4  574ms
  [14/20] Cross-document: The Data Model chapter and the Classes tutorial chapte
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:02.823043Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  13/18 query terms found  (72%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.5/4  gen=1.2/4  732ms
  [15/20] What role does the import system (Chapter 5) play in relation to the s
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:03.762990Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  6/8 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=1.2/2  gen=0.5/2  561ms
  [16/20] Near-miss: A bare expression is a valid simple statement in Python —
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:04.567711Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  6/10 query terms found  (60%)
  ○ Round 2   gap-filling for [near-miss, bare, formally, distinguishes]
  ○ Round 2   added 8 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, bare, formally]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 11 documents — passing to LLM

         → ret=4/4  gen=1.6/4  618ms
  [17/20] What Python version's documentation is represented in this cluster, an
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:05.407919Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 9 documents — passing to LLM

         → ret=1.5/2  gen=0.6/2  612ms
  [18/20] Which built-in function would be used to determine the type of a Pytho
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:06.243036Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=1/1  gen=0.3/1  684ms
  [19/20] Edge case: The os and sys modules are both standard library modules in
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:07.174487Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  13/17 query terms found  (76%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=1.5/2  gen=0.6/2  603ms
  [20/20] Cross-document: How does the execution model chapter's discussion of s
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T16:05:08.019813Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m347 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  9/11 query terms found  (82%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=1.5/2  gen=0.6/2  631ms

# RAG Eval Report

**KB:** `PythonDocs`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 86.5% (54.5/63) |
| Generation recall (token-overlap + semantic) | 32.3% (20.3/63) |
| Scoring mode | token-overlap + semantic embedding (low=0.30, high=0.85) |
| Avg latency | 655ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | According to the Python Language Reference, what is "lexical analysis," and what does Chapter 2 of the docs cover under this heading? | 3/3 (100%) | 0.9/3 (30%) | Python 3.14 Full Grammar Specification.html, Python 3.14 Library Reference - os.html, [Graph: Python Standard Library], Python 3.14 Library Reference - sys.html, 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html, 2. Lexical analysis — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Tutorial - Introduction.html | 916ms |
| q02 | What is Python's "data model," and why is it foundational to understanding how Python objects behave, per Chapter 3? | 4/4 (100%) | 1.3/4 (33%) | Python 3.14 Library Reference - Built-in Functions.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, Python 3.14 Tutorial - Classes.html, 6. Expressions — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library] | 719ms |
| q03 | What does Python's "execution model" (Chapter 4) describe regarding how and when code actually runs? | 2/2 (100%) | 0.7/2 (33%) | 6. Expressions — Python 3.14.6 documentation.pdf, [Graph: Code], 3. Data model — Python 3.14.6 documentation.pdf, 4. Execution model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, 5. The import system — Python 3.14.6 documentation.pdf | 696ms |
| q04 | What is Python's import system (Chapter 5), and what mechanism does it define for locating and loading modules? | 3.0/4 (75%) | 1.1/4 (28%) | 5. The import system — Python 3.14.6 documentation.pdf, 7. Simple statements — Python 3.14.6 documentation.pdf, [Graph: Using] | 641ms |
| q05 | According to the Expressions chapter, what is Python's defined order of operator precedence? Give two examples. | 2.7/4 (67%) | 1.6/4 (40%) | [Graph: Pythonʼs], 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html, 6. Expressions — Python 3.14.6 documentation.pdf, 2. Lexical analysis — Python 3.14.6 documentation.pdf | 532ms |
| q06 | What distinguishes a "simple statement" from a "compound statement" in Python's grammar, per Chapters 7 and 8? | 3.1/4 (77%) | 1.2/4 (31%) | Python 3.14 Tutorial - Introduction.html, [Graph: Full Grammar], Python 3.14 Full Grammar Specification.html, 7. Simple statements — Python 3.14.6 documentation.pdf, 2. Lexical analysis — Python 3.14.6 documentation.pdf, 6. Expressions — Python 3.14.6 documentation.pdf, 8. Compound statements — Python 3.14.6 documentation.pdf | 567ms |
| q07 | What is a Python class according to the tutorial's Classes chapter, and what object-oriented concepts does it introduce? | 4/4 (100%) | 1.2/4 (29%) | 5. The import system — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library], Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Library Reference - os.html, Python 3.14 Tutorial - Classes.html, Python 3.14 Library Reference - sys.html, 7. Simple statements — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf | 640ms |
| q08 | What does the official Python tutorial's Introduction chapter say about Python's design philosophy or intended audience? | 4/4 (100%) | 1.3/4 (32%) | Python 3.14 Library Reference - os.html, Python 3.14 Tutorial - Introduction.html, [Graph: Python Standard Library], Python 3.14 Full Grammar Specification.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, 7. Simple statements — Python 3.14.6 documentation.pdf | 891ms |
| q09 | What is the purpose of the os module according to the Python 3.14 Library Reference? | 3/3 (100%) | 0.9/3 (29%) | 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, [Graph: Python Standard Library], Python 3.14 Library Reference - sys.html, 5. The import system — Python 3.14.6 documentation.pdf | 538ms |
| q10 | What is the purpose of the sys module, and how does it differ functionally from the os module? | 1.5/2 (73%) | 0.7/2 (34%) | Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Library Reference - sys.html, 5. The import system — Python 3.14.6 documentation.pdf | 538ms |
| q11 | Name three built-in functions documented in the Python 3.14 Built-in Functions reference and briefly describe what each does. | 3.0/4 (75%) | 1.3/4 (33%) | [Graph: Python Standard Library], 5. The import system — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, Python 3.14 Library Reference - Built-in Functions.html | 739ms |
| q12 | What does the Full Grammar Specification document formally define, and how does it relate to the more prose-based Language Reference chapters? | 3.6/4 (89%) | 1.4/4 (34%) | Python 3.14 Tutorial - Introduction.html, Python 3.14 Full Grammar Specification.html, 5. The import system — Python 3.14.6 documentation.pdf, 6. Expressions — Python 3.14.6 documentation.pdf, [Graph: Full Grammar] | 671ms |
| q13 | Cross-document: How does the formal grammar in the Full Grammar Specification relate to the informal explanation of simple and compound statements in Chapters 7-8? | 3.5/4 (89%) | 1.2/4 (31%) | 8. Compound statements — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html, [Graph: Full Grammar], 7. Simple statements — Python 3.14.6 documentation.pdf | 574ms |
| q14 | Cross-document: The Data Model chapter and the Classes tutorial chapter both discuss object-oriented behavior — how does the tutorial's introductory framing differ from the Language Reference's more technical data-model description? | 3.5/4 (88%) | 1.2/4 (31%) | 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Library Reference - sys.html, Python 3.14 Tutorial - Classes.html | 732ms |
| q15 | What role does the import system (Chapter 5) play in relation to the sys module (e.g., sys.path)? | 1.2/2 (59%) | 0.5/2 (26%) | 3. Data model — Python 3.14.6 documentation.pdf, [Graph: Entries], Python 3.14 Library Reference - os.html, 5. The import system — Python 3.14.6 documentation.pdf | 561ms |
| q16 | Near-miss: A bare expression is a valid simple statement in Python — what formally distinguishes an expression from a statement in Python's grammar? | 4/4 (100%) | 1.6/4 (41%) | 3. Data model — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library], 8. Compound statements — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Introduction.html, Python 3.14 Library Reference - sys.html, Python 3.14 Library Reference - os.html, 5. The import system — Python 3.14.6 documentation.pdf, 7. Simple statements — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html | 618ms |
| q17 | What Python version's documentation is represented in this cluster, and why might that version-specificity matter for a technical Q&A benchmark? | 1.5/2 (75%) | 0.6/2 (32%) | [Graph: Python Standard Library], Python 3.14 Tutorial - Classes.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html, Python 3.14 Library Reference - sys.html, 4. Execution model — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Library Reference - os.html | 612ms |
| q18 | Which built-in function would be used to determine the type of a Python object at runtime? | 1/1 (100%) | 0.3/1 (32%) | Python 3.14 Library Reference - Built-in Functions.html, 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, [Graph: Python Standard Library], 8. Compound statements — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf | 684ms |
| q19 | Edge case: The os and sys modules are both standard library modules involved in interacting with the operating system or interpreter — name a specific capability that belongs to sys but not os, or vice versa. | 1.5/2 (77%) | 0.6/2 (31%) | 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, 5. The import system — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library], Python 3.14 Library Reference - sys.html | 603ms |
| q20 | Cross-document: How does the execution model chapter's discussion of scopes and namespaces relate to the import system chapter's discussion of module namespaces? | 1.5/2 (73%) | 0.6/2 (31%) | Python 3.14 Library Reference - os.html, 5. The import system — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf, 7. Simple statements — Python 3.14.6 documentation.pdf | 631ms |

## Answers

### q01 — According to the Python Language Reference, what is "lexical analysis," and what does Chapter 2 of the docs cover under this heading?

(no response)

### q02 — What is Python's "data model," and why is it foundational to understanding how Python objects behave, per Chapter 3?

(no response)

### q03 — What does Python's "execution model" (Chapter 4) describe regarding how and when code actually runs?

(no response)

### q04 — What is Python's import system (Chapter 5), and what mechanism does it define for locating and loading modules?

(no response)

### q05 — According to the Expressions chapter, what is Python's defined order of operator precedence? Give two examples.

(no response)

### q06 — What distinguishes a "simple statement" from a "compound statement" in Python's grammar, per Chapters 7 and 8?

(no response)

### q07 — What is a Python class according to the tutorial's Classes chapter, and what object-oriented concepts does it introduce?

(no response)

### q08 — What does the official Python tutorial's Introduction chapter say about Python's design philosophy or intended audience?

(no response)

### q09 — What is the purpose of the os module according to the Python 3.14 Library Reference?

(no response)

### q10 — What is the purpose of the sys module, and how does it differ functionally from the os module?

(no response)

### q11 — Name three built-in functions documented in the Python 3.14 Built-in Functions reference and briefly describe what each does.

(no response)

### q12 — What does the Full Grammar Specification document formally define, and how does it relate to the more prose-based Language Reference chapters?

(no response)

### q13 — Cross-document: How does the formal grammar in the Full Grammar Specification relate to the informal explanation of simple and compound statements in Chapters 7-8?

(no response)

### q14 — Cross-document: The Data Model chapter and the Classes tutorial chapter both discuss object-oriented behavior — how does the tutorial's introductory framing differ from the Language Reference's more technical data-model description?

(no response)

### q15 — What role does the import system (Chapter 5) play in relation to the sys module (e.g., sys.path)?

(no response)

### q16 — Near-miss: A bare expression is a valid simple statement in Python — what formally distinguishes an expression from a statement in Python's grammar?

(no response)

### q17 — What Python version's documentation is represented in this cluster, and why might that version-specificity matter for a technical Q&A benchmark?

(no response)

### q18 — Which built-in function would be used to determine the type of a Python object at runtime?

(no response)

### q19 — Edge case: The os and sys modules are both standard library modules involved in interacting with the operating system or interpreter — name a specific capability that belongs to sys but not os, or vice versa.

(no response)

### q20 — Cross-document: How does the execution model chapter's discussion of scopes and namespaces relate to the import system chapter's discussion of module namespaces?

(no response)


  ✅ Retrieval: 86.5%  Generation: 32.3%  (20.3/63)  avg 655ms
