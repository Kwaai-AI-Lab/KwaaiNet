[2m2026-07-09T21:58:50.324096Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 263541ee-b3f8-4a8e-9a65-38ae7cd8e8b4: routing: not found
[2m2026-07-09T21:58:50.324181Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:63539 (via ollama-proxy)
[2m2026-07-09T21:59:00.144021Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│                 RAG Eval  (20 questions, kb=Legal)                  │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:63539
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] What was the central legal question in Marbury v. Madison, and what do
[2m2026-07-09T21:59:03.194404Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  7/7 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

[2m2026-07-09T21:59:23.636411Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call c16e0553-f884-4618-981f-7993e39d219d: routing: not found
[2m2026-07-09T21:59:23.636486Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=4/4  gen=0/4  22801ms
  [ 2/20] In Miranda v. Arizona, what specific procedural warnings did the Court
[2m2026-07-09T21:59:25.623283Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  10/10 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

[2m2026-07-09T21:59:27.764243Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 7cef9503-c159-46ec-aec0-579b17618eaf: routing: not found
[2m2026-07-09T21:59:27.764268Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=4/4  gen=0/4  3768ms
  [ 3/20] What was the primary holding of Roe v. Wade regarding a woman's right 
[2m2026-07-09T21:59:28.215375Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 6 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

[2m2026-07-09T21:59:30.316579Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call f5eefc38-be7d-4d36-820b-1f64274ad44b: routing: not found
[2m2026-07-09T21:59:30.316639Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T21:59:30.316794Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
         → ret=4/4  gen=0/4  2299ms
  [ 4/20] What racial doctrine did Plessy v. Ferguson establish, and how did Bro
[2m2026-07-09T21:59:31.095381Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  9/12 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=4/4  gen=0/4  2571ms
  [ 5/20] What did Dred Scott v. Sandford hold regarding the citizenship rights 
[2m2026-07-09T21:59:33.580557Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  12/15 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=4/4  gen=0/4  2112ms
  [ 6/20] Trace the doctrinal chain from Dred Scott through Plessy to Brown v. B
[2m2026-07-09T21:59:36.156721Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  11/16 query terms found  (69%)
  ○ Round 2   gap-filling for [doctrinal, chain, connects, outcome, flip]
  ○ Round 2   added 5 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [doctrinal, chain, connects]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=4/4  gen=0/4  2461ms
  [ 7/20] What did Lochner v. New York hold about a state's ability to regulate 
[2m2026-07-09T21:59:38.621740Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  15/18 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.2/4  gen=0/4  2240ms
  [ 8/20] What New Deal-era pressures are commonly cited as context for the doct
[2m2026-07-09T21:59:41.520513Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  7/11 query terms found  (64%)
  ○ Round 2   gap-filling for [deal-era, pressures, commonly, doctrinal]
  ○ Round 2   added 8 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [deal-era, pressures, commonly]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=3/4  gen=0/4  2751ms
  [ 9/20] In Korematsu v. United States, what wartime measure did the Court upho
[2m2026-07-09T21:59:44.231986Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  12/16 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  2271ms
  [10/20] Which case, Korematsu or Ex parte Milligan, was decided first, and how
[2m2026-07-09T21:59:46.590366Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  10/13 query terms found  (77%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=3/4  gen=0/4  2146ms
  [11/20] What campaign-finance restriction did McConnell v. FEC uphold, and how
[2m2026-07-09T21:59:49.180843Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=3.1/4  gen=0/4  2688ms
  [12/20] What is the core First Amendment question that connects McConnell v. F
[2m2026-07-09T21:59:52.557074Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  6/8 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → ret=4/4  gen=0/4  3205ms
  [13/20] What did Bowers v. Hardwick hold regarding the criminalization of same
[2m2026-07-09T21:59:55.532484Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 2 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=4/4  gen=0/4  2655ms
  [14/20] Near-miss: Bowers v. Hardwick and Roe v. Wade both concern privacy-rel
[2m2026-07-09T21:59:58.998749Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  16/19 query terms found  (84%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

[2m2026-07-09T22:00:01.472245Z[0m [32m INFO[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → HalfOpen (cooldown elapsed)
[2m2026-07-09T22:00:01.557347Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 9ad48126-13f4-40c4-92c8-e2954c1dcd3e: routing: not found
[2m2026-07-09T22:00:01.557364Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T22:00:01.557396Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (4 consecutive connection failure(s)); will retry in 30s
         → ret=4/4  gen=0/4  3459ms
  [15/20] Cross-document: Miranda v. Arizona and Ex parte Milligan both concern 
[2m2026-07-09T22:00:02.340386Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  17/25 query terms found  (68%)
  ○ Round 2   gap-filling for [cross-document, protections, facing, context, wartime]
  ○ Round 2   added 5 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, protections, facing]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=4/4  gen=0/4  2873ms
  [16/20] What is the significance of Marbury v. Madison to every other case in 
[2m2026-07-09T22:00:05.148051Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  10/12 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  2467ms
  [17/20] Edge case: Both Lochner v. New York and Dred Scott v. Sandford are dec
[2m2026-07-09T22:00:07.933250Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  23/25 query terms found  (92%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=4/4  gen=0/4  2551ms
  [18/20] Cross-document: How does the doctrinal reasoning in West Coast Hotel C
[2m2026-07-09T22:00:10.719801Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  13/19 query terms found  (68%)
  ○ Round 2   gap-filling for [cross-document, doctrinal, reasoning, compare, separate-but-equal]
  ○ Round 2   added 10 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, doctrinal, compare]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=3.1/4  gen=0/4  2539ms
  [19/20] What was at stake in Citizens United v. FEC regarding corporate politi
[2m2026-07-09T22:00:13.479152Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  14/14 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=4/4  gen=0/4  2619ms
  [20/20] Edge case: Of the cases in this cluster, Bowers v. Hardwick has no exp
[2m2026-07-09T22:00:16.940555Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m539 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  18/27 query terms found  (67%)
  ○ Round 2   gap-filling for [cluster, explicit, reversal, corpus, doctrinal]
  ○ Round 2   added 1 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cluster, explicit, reversal]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=1.1/4  gen=0/4  3166ms

# RAG Eval Report

**KB:** `Legal`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 90.6% (72.5/80) |
| Generation recall (token-overlap + semantic) | 0.0% (0.0/80) |
| Scoring mode | token-overlap + semantic embedding (low=0.55, high=0.85) |
| Avg latency | 3682ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | What was the central legal question in Marbury v. Madison, and what doctrine did it establish? | 4/4 (100%) | 0/4 (0%) | [Graph: Mr Madison], supreme.justia.com-Plessy v Ferguson 163 US 537 1896.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Marbury v. Madison _ 5 U.S. 137 (1803) _ Justia U.S. Supreme Court Center.pdf | 22801ms |
| q02 | In Miranda v. Arizona, what specific procedural warnings did the Court require police to give suspects? | 4/4 (100%) | 0/4 (0%) | Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf, [Graph: Circuit Court of Appeals] | 3768ms |
| q03 | What was the primary holding of Roe v. Wade regarding a woman's right to abortion? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Roe v. Wade _ 410 U.S. 113 (1973) _ Justia U.S. Supreme Court Center.pdf | 2299ms |
| q04 | What racial doctrine did Plessy v. Ferguson establish, and how did Brown v. Board of Education explicitly overturn it decades later? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, Brown v. Board of Education of Topeka _ 347 U.S. 483 (1954) _ Justia U.S. Supreme Court Center.pdf, [Graph: Brown v. Fay], supreme.justia.com-Plessy v Ferguson 163 US 537 1896.pdf | 2571ms |
| q05 | What did Dred Scott v. Sandford hold regarding the citizenship rights of enslaved and formerly enslaved people, and how does it function as a doctrinal ancestor of Plessy v. Ferguson? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, supreme.justia.com-Plessy v Ferguson 163 US 537 1896.pdf, [Graph: Bill of Rights], Brown v. Board of Education of Topeka _ 347 U.S. 483 (1954) _ Justia U.S. Supreme Court Center.pdf | 2112ms |
| q06 | Trace the doctrinal chain from Dred Scott through Plessy to Brown v. Board — what common legal question connects all three, and how did the outcome flip? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Plessy v Ferguson 163 US 537 1896.pdf, supreme.justia.com-McConnell v FEC 540 US 93 2003.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf, [Graph: Dred Scott], supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, Brown v. Board of Education of Topeka _ 347 U.S. 483 (1954) _ Justia U.S. Supreme Court Center.pdf | 2461ms |
| q07 | What did Lochner v. New York hold about a state's ability to regulate labor contracts under the "freedom of contract" doctrine, and how did West Coast Hotel Co. v. Parrish explicitly reject that doctrine? | 3.2/4 (80%) | 0/4 (0%) | supreme.justia.com-Lochner v New York 198 US 45 1905.pdf, [Graph: Court of Appeals of New York], supreme.justia.com-West Coast Hotel Co v Parrish 300 US 379 1937.pdf, Roe v. Wade _ 410 U.S. 113 (1973) _ Justia U.S. Supreme Court Center.pdf | 2240ms |
| q08 | What New Deal-era pressures are commonly cited as context for the doctrinal shift from Lochner to West Coast Hotel? | 3/4 (75%) | 0/4 (0%) | Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Korematsu v United States 323 US 214 1944.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, [Graph: Court of Appeals of New York], supreme.justia.com-West Coast Hotel Co v Parrish 300 US 379 1937.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, Marbury v. Madison _ 5 U.S. 137 (1803) _ Justia U.S. Supreme Court Center.pdf, Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf | 2751ms |
| q09 | In Korematsu v. United States, what wartime measure did the Court uphold, and how does Ex parte Milligan's earlier holding on military trials of civilians stand in tension with it? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, [Graph: Court of Appeals of New York], Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, supreme.justia.com-Korematsu v United States 323 US 214 1944.pdf | 2271ms |
| q10 | Which case, Korematsu or Ex parte Milligan, was decided first, and how might that timing affect deference to military authority in each? | 3/4 (75%) | 0/4 (0%) | supreme.justia.com-Korematsu v United States 323 US 214 1944.pdf, supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, [Graph: First] | 2146ms |
| q11 | What campaign-finance restriction did McConnell v. FEC uphold, and how did Citizens United v. FEC later strike down a related restriction? | 3.1/4 (77%) | 0/4 (0%) | supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf, Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf | 2688ms |
| q12 | What is the core First Amendment question that connects McConnell v. FEC and Citizens United v. FEC? | 4/4 (100%) | 0/4 (0%) | Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf, [Graph: First Amendment] | 3205ms |
| q13 | What did Bowers v. Hardwick hold regarding the criminalization of same-sex intimate conduct? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf | 2655ms |
| q14 | Near-miss: Bowers v. Hardwick and Roe v. Wade both concern privacy-related liberty interests — how does the Court's reasoning in each case differ in its willingness to extend a constitutional right to privacy? | 4/4 (100%) | 0/4 (0%) | Roe v. Wade _ 410 U.S. 113 (1973) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf | 3459ms |
| q15 | Cross-document: Miranda v. Arizona and Ex parte Milligan both concern procedural protections owed to individuals facing government power — how does the context (ordinary criminal suspect vs. wartime military detainee) affect the scope of protection in each? | 4/4 (100%) | 0/4 (0%) | supreme.justia.com-Korematsu v United States 323 US 214 1944.pdf, Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf, Marbury v. Madison _ 5 U.S. 137 (1803) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-McConnell v FEC 540 US 93 2003.pdf, [Graph: Ernesto Miranda], supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, supreme.justia.com-West Coast Hotel Co v Parrish 300 US 379 1937.pdf | 2873ms |
| q16 | What is the significance of Marbury v. Madison to every other case in this cluster, given the power it establishes for the Supreme Court? | 4/4 (100%) | 0/4 (0%) | Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf, Marbury v. Madison _ 5 U.S. 137 (1803) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Lochner v New York 198 US 45 1905.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, [Graph: Court of Appeals of New York] | 2467ms |
| q17 | Edge case: Both Lochner v. New York and Dred Scott v. Sandford are decisions the Supreme Court has since widely repudiated — what does each decision's specific reasoning (not just its outcome) reveal about how legal doctrines become discredited? | 4/4 (100%) | 0/4 (0%) | [Graph: Court of Appeals of New York], supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Roe v. Wade _ 410 U.S. 113 (1973) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Lochner v New York 198 US 45 1905.pdf | 2551ms |
| q18 | Cross-document: How does the doctrinal reasoning in West Coast Hotel Co. v. Parrish (rejecting freedom of contract) compare to the reasoning in Brown v. Board (rejecting separate-but-equal) in terms of how the Court justified overturning precedent? | 3.1/4 (78%) | 0/4 (0%) | Marbury v. Madison _ 5 U.S. 137 (1803) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-West Coast Hotel Co v Parrish 300 US 379 1937.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, supreme.justia.com-McConnell v FEC 540 US 93 2003.pdf, Brown v. Board of Education of Topeka _ 347 U.S. 483 (1954) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf, Miranda v. Arizona _ 384 U.S. 436 (1966) _ Justia U.S. Supreme Court Center.pdf, [Graph: Court of Appeals of New York] | 2539ms |
| q19 | What was at stake in Citizens United v. FEC regarding corporate political spending, and how does the case's outcome relate to McConnell v. FEC's earlier, narrower restriction? | 4/4 (100%) | 0/4 (0%) | Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf | 2619ms |
| q20 | Edge case: Of the cases in this cluster, Bowers v. Hardwick has no explicit reversal partner present in the corpus — what broader doctrinal category (privacy/liberty) does it still belong to, and how might a system correctly recognize it as an outlier rather than force a false pairing? | 1.1/4 (27%) | 0/4 (0%) | Brown v. Board of Education of Topeka _ 347 U.S. 483 (1954) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Ex parte Milligan 71 US 2 1866.pdf, Roe v. Wade _ 410 U.S. 113 (1973) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Bowers v Hardwick 478 US 186 1986.pdf, Citizens United v. FEC _ 558 U.S. 310 (2010) _ Justia U.S. Supreme Court Center.pdf, supreme.justia.com-Dred Scott v Sandford 60 US 393 1856.pdf | 3166ms |

## Answers

### q01 — What was the central legal question in Marbury v. Madison, and what doctrine did it establish?

(no response)

### q02 — In Miranda v. Arizona, what specific procedural warnings did the Court require police to give suspects?

(no response)

### q03 — What was the primary holding of Roe v. Wade regarding a woman's right to abortion?

(no response)

### q04 — What racial doctrine did Plessy v. Ferguson establish, and how did Brown v. Board of Education explicitly overturn it decades later?

(no response)

### q05 — What did Dred Scott v. Sandford hold regarding the citizenship rights of enslaved and formerly enslaved people, and how does it function as a doctrinal ancestor of Plessy v. Ferguson?

(no response)

### q06 — Trace the doctrinal chain from Dred Scott through Plessy to Brown v. Board — what common legal question connects all three, and how did the outcome flip?

(no response)

### q07 — What did Lochner v. New York hold about a state's ability to regulate labor contracts under the "freedom of contract" doctrine, and how did West Coast Hotel Co. v. Parrish explicitly reject that doctrine?

(no response)

### q08 — What New Deal-era pressures are commonly cited as context for the doctrinal shift from Lochner to West Coast Hotel?

(no response)

### q09 — In Korematsu v. United States, what wartime measure did the Court uphold, and how does Ex parte Milligan's earlier holding on military trials of civilians stand in tension with it?

(no response)

### q10 — Which case, Korematsu or Ex parte Milligan, was decided first, and how might that timing affect deference to military authority in each?

(no response)

### q11 — What campaign-finance restriction did McConnell v. FEC uphold, and how did Citizens United v. FEC later strike down a related restriction?

(no response)

### q12 — What is the core First Amendment question that connects McConnell v. FEC and Citizens United v. FEC?

(no response)

### q13 — What did Bowers v. Hardwick hold regarding the criminalization of same-sex intimate conduct?

(no response)

### q14 — Near-miss: Bowers v. Hardwick and Roe v. Wade both concern privacy-related liberty interests — how does the Court's reasoning in each case differ in its willingness to extend a constitutional right to privacy?

(no response)

### q15 — Cross-document: Miranda v. Arizona and Ex parte Milligan both concern procedural protections owed to individuals facing government power — how does the context (ordinary criminal suspect vs. wartime military detainee) affect the scope of protection in each?

(no response)

### q16 — What is the significance of Marbury v. Madison to every other case in this cluster, given the power it establishes for the Supreme Court?

(no response)

### q17 — Edge case: Both Lochner v. New York and Dred Scott v. Sandford are decisions the Supreme Court has since widely repudiated — what does each decision's specific reasoning (not just its outcome) reveal about how legal doctrines become discredited?

(no response)

### q18 — Cross-document: How does the doctrinal reasoning in West Coast Hotel Co. v. Parrish (rejecting freedom of contract) compare to the reasoning in Brown v. Board (rejecting separate-but-equal) in terms of how the Court justified overturning precedent?

(no response)

### q19 — What was at stake in Citizens United v. FEC regarding corporate political spending, and how does the case's outcome relate to McConnell v. FEC's earlier, narrower restriction?

(no response)

### q20 — Edge case: Of the cases in this cluster, Bowers v. Hardwick has no explicit reversal partner present in the corpus — what broader doctrinal category (privacy/liberty) does it still belong to, and how might a system correctly recognize it as an outlier rather than force a false pairing?

(no response)


  ✅ Retrieval: 90.6%  Generation: 0.0%  (0.0/80)  avg 3682ms
