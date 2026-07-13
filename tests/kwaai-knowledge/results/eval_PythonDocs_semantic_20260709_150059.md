[2m2026-07-09T22:02:28.974784Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call dbeb7657-960f-4d1b-ba64-490eb2fcb49c: routing: not found
[2m2026-07-09T22:02:28.974886Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:50472 (via ollama-proxy)
[2m2026-07-09T22:02:29.651717Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│               RAG Eval  (20 questions, kb=PythonDocs)               │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:50472
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] According to the Python Language Reference, what is "lexical analysis,
[2m2026-07-09T22:02:30.171060Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 10 documents — passing to LLM

[2m2026-07-09T22:02:33.151389Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 135ca31e-0b03-40fc-8a7c-f5b99dbcd739: routing: not found
[2m2026-07-09T22:02:33.151439Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=3/3  gen=0/3  3061ms
  [ 2/20] What is Python's "data model," and why is it foundational to understan
[2m2026-07-09T22:02:33.543975Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  6/8 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-09T22:02:35.372617Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 4c831a77-06ce-47ef-9864-d86fbac8afb3: routing: not found
[2m2026-07-09T22:02:35.372688Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=4/4  gen=0/4  1937ms
  [ 3/20] What does Python's "execution model" (Chapter 4) describe regarding ho
[2m2026-07-09T22:02:35.860058Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

[2m2026-07-09T22:02:37.685064Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 9347d3bb-a515-4f38-8b66-bdd1dceb6842: routing: not found
[2m2026-07-09T22:02:37.685116Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T22:02:37.685217Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
         → ret=2/2  gen=0/2  2025ms
  [ 4/20] What is Python's import system (Chapter 5), and what mechanism does it
[2m2026-07-09T22:02:38.051899Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 6 documents
  ○ Coverage  7/9 query terms found  (78%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=2.2/4  gen=0/4  1922ms
  [ 5/20] According to the Expressions chapter, what is Python's defined order o
[2m2026-07-09T22:02:40.204299Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  10/10 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=1.8/4  gen=0.1/4  1801ms
  [ 6/20] What distinguishes a "simple statement" from a "compound statement" in
[2m2026-07-09T22:02:42.227958Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  6/7 query terms found  (86%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.1/4  gen=0.1/4  1919ms
  [ 7/20] What is a Python class according to the tutorial's Classes chapter, an
[2m2026-07-09T22:02:44.422267Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  7/9 query terms found  (78%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=0/4  1869ms
  [ 8/20] What does the official Python tutorial's Introduction chapter say abou
[2m2026-07-09T22:02:46.486817Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  6/9 query terms found  (67%)
  ○ Round 2   gap-filling for [official, philosophy, audience]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [official, philosophy, audience]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=3.2/4  gen=0/4  1914ms
  [ 9/20] What is the purpose of the os module according to the Python 3.14 Libr
[2m2026-07-09T22:02:48.612728Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  7/7 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=3/3  gen=0/3  1914ms
  [10/20] What is the purpose of the sys module, and how does it differ function
[2m2026-07-09T22:02:50.778997Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  3/4 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=1/2  gen=0/2  1808ms
  [11/20] Name three built-in functions documented in the Python 3.14 Built-in F
[2m2026-07-09T22:02:52.802626Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=2.1/4  gen=0/4  1899ms
  [12/20] What does the Full Grammar Specification document formally define, and
[2m2026-07-09T22:02:54.909155Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  9/11 query terms found  (82%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.1/4  gen=0/4  1945ms
  [13/20] Cross-document: How does the formal grammar in the Full Grammar Specif
[2m2026-07-09T22:02:57.232071Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  9/12 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.2/4  gen=0/4  2146ms
  [14/20] Cross-document: The Data Model chapter and the Classes tutorial chapte
[2m2026-07-09T22:02:59.517477Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  13/18 query terms found  (72%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.1/4  gen=0/4  1898ms
  [15/20] What role does the import system (Chapter 5) play in relation to the s
[2m2026-07-09T22:03:01.633036Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  4/8 query terms found  (50%)
  ○ Round 2   gap-filling for [role, chapter, play, relation]
  ○ Round 2   added 7 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [role, chapter, relation]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=0.2/2  gen=0/2  1873ms
  [16/20] Near-miss: A bare expression is a valid simple statement in Python —
[2m2026-07-09T22:03:03.717911Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  6/10 query terms found  (60%)
  ○ Round 2   gap-filling for [near-miss, bare, formally, distinguishes]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, bare, formally]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 9 documents — passing to LLM

         → ret=3.2/4  gen=0.2/4  1882ms
  [17/20] What Python version's documentation is represented in this cluster, an
[2m2026-07-09T22:03:05.821699Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  7/10 query terms found  (70%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 9 documents — passing to LLM

         → ret=1.2/2  gen=0/2  1863ms
  [18/20] Which built-in function would be used to determine the type of a Pytho
[2m2026-07-09T22:03:07.920773Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-09T22:03:09.738852Z[0m [32m INFO[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → HalfOpen (cooldown elapsed)
[2m2026-07-09T22:03:09.828353Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 6d0c128d-6df3-447c-9a49-dacab3350331: routing: not found
[2m2026-07-09T22:03:09.828410Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T22:03:09.828557Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (4 consecutive connection failure(s)); will retry in 30s
         → ret=1/1  gen=0/1  2008ms
  [19/20] Edge case: The os and sys modules are both standard library modules in
[2m2026-07-09T22:03:10.174477Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  14/17 query terms found  (82%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=1.2/2  gen=0/2  1882ms
  [20/20] Cross-document: How does the execution model chapter's discussion of s
[2m2026-07-09T22:03:12.314286Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m318 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  8/11 query terms found  (73%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=1.0/2  gen=0/2  1848ms

# RAG Eval Report

**KB:** `PythonDocs`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 73.9% (46.5/63) |
| Generation recall (token-overlap + semantic) | 0.6% (0.4/63) |
| Scoring mode | token-overlap + semantic embedding (low=0.55, high=0.85) |
| Avg latency | 1970ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | According to the Python Language Reference, what is "lexical analysis," and what does Chapter 2 of the docs cover under this heading? | 3/3 (100%) | 0/3 (0%) | Python 3.14 Tutorial - Introduction.html, Python 3.14 Library Reference - sys.html, [Graph: Python Standard Library], 6. Expressions — Python 3.14.6 documentation.pdf, 2. Lexical analysis — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, Python 3.14 Full Grammar Specification.html, Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Tutorial - Classes.html, 3. Data model — Python 3.14.6 documentation.pdf | 3061ms |
| q02 | What is Python's "data model," and why is it foundational to understanding how Python objects behave, per Chapter 3? | 4/4 (100%) | 0/4 (0%) | [Graph: Python Software Foundation License Version 2], 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, Python 3.14 Library Reference - os.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html | 1937ms |
| q03 | What does Python's "execution model" (Chapter 4) describe regarding how and when code actually runs? | 2/2 (100%) | 0/2 (0%) | Python 3.14 Library Reference - Built-in Functions.html, 4. Execution model — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Introduction.html, 3. Data model — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, 6. Expressions — Python 3.14.6 documentation.pdf | 2025ms |
| q04 | What is Python's import system (Chapter 5), and what mechanism does it define for locating and loading modules? | 2.2/4 (54%) | 0/4 (0%) | 3. Data model — Python 3.14.6 documentation.pdf, 7. Simple statements — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf, [Graph: Generic Operating System Services] | 1922ms |
| q05 | According to the Expressions chapter, what is Python's defined order of operator precedence? Give two examples. | 1.8/4 (44%) | 0.1/4 (2%) | [Graph: Generator Expressions], Python 3.14 Tutorial - Classes.html, 2. Lexical analysis — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Introduction.html, 3. Data model — Python 3.14.6 documentation.pdf, 6. Expressions — Python 3.14.6 documentation.pdf | 1801ms |
| q06 | What distinguishes a "simple statement" from a "compound statement" in Python's grammar, per Chapters 7 and 8? | 3.1/4 (78%) | 0.1/4 (2%) | Python 3.14 Tutorial - Introduction.html, 6. Expressions — Python 3.14.6 documentation.pdf, 2. Lexical analysis — Python 3.14.6 documentation.pdf, 8. Compound statements — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html, 7. Simple statements — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html | 1919ms |
| q07 | What is a Python class according to the tutorial's Classes chapter, and what object-oriented concepts does it introduce? | 4/4 (100%) | 0/4 (0%) | Python 3.14 Tutorial - Classes.html, Python 3.14 Tutorial - Introduction.html, 3. Data model — Python 3.14.6 documentation.pdf, [Graph: Python Software Foundation License Version 2], 8. Compound statements — Python 3.14.6 documentation.pdf | 1869ms |
| q08 | What does the official Python tutorial's Introduction chapter say about Python's design philosophy or intended audience? | 3.2/4 (79%) | 0/4 (0%) | Python 3.14 Tutorial - Introduction.html, Python 3.14 Library Reference - os.html, [Graph: Python Software Foundation License Version 2], Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Full Grammar Specification.html, Python 3.14 Tutorial - Classes.html | 1914ms |
| q09 | What is the purpose of the os module according to the Python 3.14 Library Reference? | 3/3 (100%) | 0/3 (0%) | 3. Data model — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html, Python 3.14 Library Reference - sys.html, Python 3.14 Tutorial - Classes.html, 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, [Graph: Python Standard Library] | 1914ms |
| q10 | What is the purpose of the sys module, and how does it differ functionally from the os module? | 1/2 (50%) | 0/2 (0%) | Python 3.14 Library Reference - sys.html, [Graph: sys.settrace], 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html | 1808ms |
| q11 | Name three built-in functions documented in the Python 3.14 Built-in Functions reference and briefly describe what each does. | 2.1/4 (54%) | 0/4 (0%) | Python 3.14 Library Reference - sys.html, 5. The import system — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library], Python 3.14 Library Reference - Built-in Functions.html | 1899ms |
| q12 | What does the Full Grammar Specification document formally define, and how does it relate to the more prose-based Language Reference chapters? | 3.1/4 (78%) | 0/4 (0%) | 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html | 1945ms |
| q13 | Cross-document: How does the formal grammar in the Full Grammar Specification relate to the informal explanation of simple and compound statements in Chapters 7-8? | 3.2/4 (79%) | 0/4 (0%) | Python 3.14 Tutorial - Introduction.html, 8. Compound statements — Python 3.14.6 documentation.pdf, 7. Simple statements — Python 3.14.6 documentation.pdf, Python 3.14 Full Grammar Specification.html | 2146ms |
| q14 | Cross-document: The Data Model chapter and the Classes tutorial chapter both discuss object-oriented behavior — how does the tutorial's introductory framing differ from the Language Reference's more technical data-model description? | 3.1/4 (78%) | 0/4 (0%) | Python 3.14 Tutorial - Classes.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, Python 3.14 Library Reference - sys.html | 1898ms |
| q15 | What role does the import system (Chapter 5) play in relation to the sys module (e.g., sys.path)? | 0.2/2 (11%) | 0/2 (0%) | 8. Compound statements — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, [Graph: Generic Operating System Services], Python 3.14 Tutorial - Introduction.html, 5. The import system — Python 3.14.6 documentation.pdf, 6. Expressions — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html | 1873ms |
| q16 | Near-miss: A bare expression is a valid simple statement in Python — what formally distinguishes an expression from a statement in Python's grammar? | 3.2/4 (81%) | 0.2/4 (6%) | Python 3.14 Library Reference - os.html, Python 3.14 Tutorial - Introduction.html, 7. Simple statements — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - Built-in Functions.html, [Graph: Python Standard Library], Python 3.14 Tutorial - Classes.html, 8. Compound statements — Python 3.14.6 documentation.pdf, 6. Expressions — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf | 1882ms |
| q17 | What Python version's documentation is represented in this cluster, and why might that version-specificity matter for a technical Q&A benchmark? | 1.2/2 (59%) | 0/2 (0%) | [Graph: Python Software Foundation License Version 2], Python 3.14 Tutorial - Introduction.html, 5. The import system — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, Python 3.14 Tutorial - Classes.html, Python 3.14 Full Grammar Specification.html, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - os.html, Python 3.14 Library Reference - Built-in Functions.html | 1863ms |
| q18 | Which built-in function would be used to determine the type of a Python object at runtime? | 1/1 (100%) | 0/1 (0%) | Python 3.14 Library Reference - Built-in Functions.html, 5. The import system — Python 3.14.6 documentation.pdf, [Graph: Python Software Foundation License Version 2], 8. Compound statements — Python 3.14.6 documentation.pdf, 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html | 2008ms |
| q19 | Edge case: The os and sys modules are both standard library modules involved in interacting with the operating system or interpreter — name a specific capability that belongs to sys but not os, or vice versa. | 1.2/2 (58%) | 0/2 (0%) | 5. The import system — Python 3.14.6 documentation.pdf, [Graph: Python Standard Library], 3. Data model — Python 3.14.6 documentation.pdf, Python 3.14 Library Reference - sys.html, Python 3.14 Library Reference - os.html | 1882ms |
| q20 | Cross-document: How does the execution model chapter's discussion of scopes and namespaces relate to the import system chapter's discussion of module namespaces? | 1.0/2 (51%) | 0/2 (0%) | 7. Simple statements — Python 3.14.6 documentation.pdf, Python 3.14 Tutorial - Classes.html, [Graph: Generic Operating System Services], 3. Data model — Python 3.14.6 documentation.pdf, 5. The import system — Python 3.14.6 documentation.pdf | 1848ms |

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


  ✅ Retrieval: 73.9%  Generation: 0.6%  (0.4/63)  avg 1970ms
