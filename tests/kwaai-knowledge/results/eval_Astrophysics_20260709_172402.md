⚠  Legacy redb store detected for b3a3125e-f785-4d5c-b935-c358869319b6. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:12.270848Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call d22adc68-fe0e-4042-81e8-8b5a8c23df7a: routing: not found
[2m2026-07-10T00:24:12.270947Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:52120 (via ollama-proxy)
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:12.272219Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│              RAG Eval  (20 questions, kb=Astrophysics)              │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:52120
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] What is the Voyager Golden Record, and what does the "Golden Record De
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:12.683119Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/6 query terms found  (0%)
  ○ Round 2   gap-filling for [voyager, golden, record, decoded, transcript]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [voyager, golden, record]
[2m2026-07-10T00:24:12.809609Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 6caea145-2fdf-48b4-a289-1f7f68e7aed7: routing: not found
[2m2026-07-10T00:24:12.809630Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

[2m2026-07-10T00:24:12.898106Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call c2f087f5-0fca-41ce-adc2-6f545c7b9d94: routing: not found
[2m2026-07-10T00:24:12.898129Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=0/4  gen=1.3/4  216ms
  [ 2/20] According to the Wikipedia summary of "Contents of the Voyager Golden 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:13.149809Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/10 query terms found  (0%)
  ○ Round 2   gap-filling for [according, wikipedia, summary, contents, voyager]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, wikipedia, summary]
[2m2026-07-10T00:24:13.324398Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 1b14d60d-8dfa-4eba-a579-013d37167197: routing: not found
[2m2026-07-10T00:24:13.324416Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-10T00:24:13.324434Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/3  gen=1.0/3  177ms
  [ 3/20] What does Ertel's "The Apollo Spacecraft Volume IV" document, as part 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:13.577285Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/10 query terms found  (0%)
  ○ Round 2   gap-filling for [ertel, apollo, spacecraft, volume, document]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [ertel, apollo, spacecraft]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.2/4  88ms
  [ 4/20] What did the Event Horizon Telescope Collaboration's first M87 results
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:13.896037Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/9 query terms found  (0%)
  ○ Round 2   gap-filling for [event, horizon, telescope, collaboration, first]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [event, horizon, telescope]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.2/4  87ms
  [ 5/20] What did LIGO and Virgo Collaborations observe in their gravitational 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:14.218610Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/12 query terms found  (0%)
  ○ Round 2   gap-filling for [ligo, virgo, collaborations, observe, gravitational]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [ligo, virgo, collaborations]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.1/4  92ms
  [ 6/20] According to Launius's "Apollo: A Retrospective Analysis," what overal
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:14.540479Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/11 query terms found  (0%)
  ○ Round 2   gap-filling for [according, launius, apollo, retrospective, analysis]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, launius, apollo]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  80ms
  [ 7/20] What incident does the "Apollo 204 Review Board Final Report Summary" 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:14.844887Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/10 query terms found  (0%)
  ○ Round 2   gap-filling for [incident, apollo, review, board, final]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [incident, apollo, review]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/3  gen=1.0/3  79ms
  [ 8/20] What did the Planck Collaboration's 2018 results determine about cosmo
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:15.145453Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/11 query terms found  (0%)
  ○ Round 2   gap-filling for [planck, collaboration, 2018, results, determine]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [planck, collaboration, 2018]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  93ms
  [ 9/20] According to Roos, what lines of evidence support the existence of dar
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:15.468802Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/12 query terms found  (0%)
  ○ Round 2   gap-filling for [according, roos, lines, evidence, support]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, roos, lines]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  78ms
  [10/20] What were the JWST Early Release Observations, and what was their purp
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:15.781208Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/8 query terms found  (0%)
  ○ Round 2   gap-filling for [jwst, early, release, observations, purpose]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [jwst, early, release]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.2/4  94ms
  [11/20] What did the US Senate's Apollo 13 Mission Review investigate, and wha
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:16.107684Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/8 query terms found  (0%)
  ○ Round 2   gap-filling for [senate, apollo, mission, review, investigate]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [senate, apollo, mission]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  89ms
  [12/20] What is the National Aeronautics and Space Act of 1958, and what agenc
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:16.425132Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/6 query terms found  (0%)
  ○ Round 2   gap-filling for [national, aeronautics, space, 1958, agency]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [national, aeronautics, space]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.4/4  82ms
  [13/20] What does White House Space Policy Directive 1 direct NASA to prioriti
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:16.737809Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/8 query terms found  (0%)
  ○ Round 2   gap-filling for [white, house, space, policy, directive]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [white, house, space]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.1/4  91ms
  [14/20] According to Wright and Gaudi, what methods are used to detect exoplan
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:17.058440Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/9 query terms found  (0%)
  ○ Round 2   gap-filling for [according, wright, gaudi, methods, used]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, wright, gaudi]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/3  gen=1.0/3  89ms
  [15/20] Cross-document: How does the Apollo 204 (Apollo 1) fire investigation 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:17.382307Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/12 query terms found  (0%)
  ○ Round 2   gap-filling for [cross-document, apollo, fire, investigation, relate]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, apollo, fire]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  98ms
  [16/20] Cross-document: How does the National Aeronautics and Space Act of 195
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:17.711720Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/16 query terms found  (0%)
  ○ Round 2   gap-filling for [cross-document, national, aeronautics, space, 1958]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, national, aeronautics]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.2/4  81ms
  [17/20] Cross-document: How do the Event Horizon Telescope's black hole imagin
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:18.014605Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/22 query terms found  (0%)
  ○ Round 2   gap-filling for [cross-document, event, horizon, telescope, black]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, event, horizon]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.1/4  94ms
  [18/20] Cross-document: How does Planck's cosmological parameter data relate t
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:18.341152Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/14 query terms found  (0%)
  ○ Round 2   gap-filling for [cross-document, planck, cosmological, parameter, data]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, planck, cosmological]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.3/4  92ms
  [19/20] Near-miss: Both exoplanet detection and dark matter evidence involve i
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:18.661544Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/21 query terms found  (0%)
  ○ Round 2   gap-filling for [near-miss, both, exoplanet, detection, dark]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, both, exoplanet]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/2  gen=0.7/2  86ms
  [20/20] Edge case: This cluster spans human spaceflight history, policy/legisl
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-10T00:24:18.969157Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m0 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 0 chunks from 0 documents
  ○ Coverage  0/20 query terms found  (0%)
  ○ Round 2   gap-filling for [edge, case, cluster, spans, human]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [edge, case, cluster]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     0 chunks from 0 documents — passing to LLM

         → ret=0/4  gen=1.2/4  99ms

# RAG Eval Report

**KB:** `Astrophysics`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 0.0% (0.0/75) |
| Generation recall (token-overlap + semantic) | 31.3% (23.4/75) |
| Scoring mode | token-overlap + semantic embedding (low=0.30, high=0.85) |
| Avg latency | 99ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | What is the Voyager Golden Record, and what does the "Golden Record Decoded" transcript describe about its purpose? | 0/4 (0%) | 1.3/4 (31%) |  | 216ms |
| q02 | According to the Wikipedia summary of "Contents of the Voyager Golden Record," what categories of content were included? | 0/3 (0%) | 1.0/3 (33%) |  | 177ms |
| q03 | What does Ertel's "The Apollo Spacecraft Volume IV" document, as part of the official NASA history series? | 0/4 (0%) | 1.2/4 (29%) |  | 88ms |
| q04 | What did the Event Horizon Telescope Collaboration's first M87 results demonstrate, and what specifically did they image? | 0/4 (0%) | 1.2/4 (30%) |  | 87ms |
| q05 | What did LIGO and Virgo Collaborations observe in their gravitational wave detection paper, and what astronomical event produced the signal? | 0/4 (0%) | 1.1/4 (28%) |  | 92ms |
| q06 | According to Launius's "Apollo: A Retrospective Analysis," what overall assessment does the paper offer of the Apollo program's significance? | 0/4 (0%) | 1.3/4 (32%) |  | 80ms |
| q07 | What incident does the "Apollo 204 Review Board Final Report Summary" investigate, and what happened during that incident? | 0/3 (0%) | 1.0/3 (33%) |  | 79ms |
| q08 | What did the Planck Collaboration's 2018 results determine about cosmological parameters, and what satellite mission produced this data? | 0/4 (0%) | 1.3/4 (33%) |  | 93ms |
| q09 | According to Roos, what lines of evidence support the existence of dark matter across astronomy, astrophysics, and cosmology? | 0/4 (0%) | 1.3/4 (32%) |  | 78ms |
| q10 | What were the JWST Early Release Observations, and what was their purpose following the telescope's launch? | 0/4 (0%) | 1.2/4 (31%) |  | 94ms |
| q11 | What did the US Senate's Apollo 13 Mission Review investigate, and what went wrong during that mission? | 0/4 (0%) | 1.3/4 (34%) |  | 89ms |
| q12 | What is the National Aeronautics and Space Act of 1958, and what agency did it establish? | 0/4 (0%) | 1.4/4 (34%) |  | 82ms |
| q13 | What does White House Space Policy Directive 1 direct NASA to prioritize? | 0/4 (0%) | 1.1/4 (27%) |  | 91ms |
| q14 | According to Wright and Gaudi, what methods are used to detect exoplanets? Name at least two. | 0/3 (0%) | 1.0/3 (33%) |  | 89ms |
| q15 | Cross-document: How does the Apollo 204 (Apollo 1) fire investigation relate to the Apollo 13 Mission Review — are both examples of NASA safety/accident investigations? | 0/4 (0%) | 1.3/4 (33%) |  | 98ms |
| q16 | Cross-document: How does the National Aeronautics and Space Act of 1958 relate to Space Policy Directive 1, given both concern the same agency's mandate but from different eras? | 0/4 (0%) | 1.2/4 (29%) |  | 81ms |
| q17 | Cross-document: How do the Event Horizon Telescope's black hole imaging and LIGO's gravitational wave detection both provide evidence related to black holes, despite using completely different observational methods? | 0/4 (0%) | 1.1/4 (26%) |  | 94ms |
| q18 | Cross-document: How does Planck's cosmological parameter data relate to Roos's dark matter evidence review — does Planck's CMB data serve as one of the lines of evidence Roos discusses? | 0/4 (0%) | 1.3/4 (33%) |  | 92ms |
| q19 | Near-miss: Both exoplanet detection and dark matter evidence involve inferring the existence of something indirectly rather than observing it directly — what's the key methodological difference between how each is detected? | 0/2 (0%) | 0.7/2 (35%) |  | 86ms |
| q20 | Edge case: This cluster spans human spaceflight history, policy/legislative documents, and pure astrophysics research — what does this range suggest about the boundary between "space exploration" and "astrophysics" as a single topic cluster? | 0/4 (0%) | 1.2/4 (30%) |  | 99ms |

## Answers

### q01 — What is the Voyager Golden Record, and what does the "Golden Record Decoded" transcript describe about its purpose?

(no response)

### q02 — According to the Wikipedia summary of "Contents of the Voyager Golden Record," what categories of content were included?

(no response)

### q03 — What does Ertel's "The Apollo Spacecraft Volume IV" document, as part of the official NASA history series?

(no response)

### q04 — What did the Event Horizon Telescope Collaboration's first M87 results demonstrate, and what specifically did they image?

(no response)

### q05 — What did LIGO and Virgo Collaborations observe in their gravitational wave detection paper, and what astronomical event produced the signal?

(no response)

### q06 — According to Launius's "Apollo: A Retrospective Analysis," what overall assessment does the paper offer of the Apollo program's significance?

(no response)

### q07 — What incident does the "Apollo 204 Review Board Final Report Summary" investigate, and what happened during that incident?

(no response)

### q08 — What did the Planck Collaboration's 2018 results determine about cosmological parameters, and what satellite mission produced this data?

(no response)

### q09 — According to Roos, what lines of evidence support the existence of dark matter across astronomy, astrophysics, and cosmology?

(no response)

### q10 — What were the JWST Early Release Observations, and what was their purpose following the telescope's launch?

(no response)

### q11 — What did the US Senate's Apollo 13 Mission Review investigate, and what went wrong during that mission?

(no response)

### q12 — What is the National Aeronautics and Space Act of 1958, and what agency did it establish?

(no response)

### q13 — What does White House Space Policy Directive 1 direct NASA to prioritize?

(no response)

### q14 — According to Wright and Gaudi, what methods are used to detect exoplanets? Name at least two.

(no response)

### q15 — Cross-document: How does the Apollo 204 (Apollo 1) fire investigation relate to the Apollo 13 Mission Review — are both examples of NASA safety/accident investigations?

(no response)

### q16 — Cross-document: How does the National Aeronautics and Space Act of 1958 relate to Space Policy Directive 1, given both concern the same agency's mandate but from different eras?

(no response)

### q17 — Cross-document: How do the Event Horizon Telescope's black hole imaging and LIGO's gravitational wave detection both provide evidence related to black holes, despite using completely different observational methods?

(no response)

### q18 — Cross-document: How does Planck's cosmological parameter data relate to Roos's dark matter evidence review — does Planck's CMB data serve as one of the lines of evidence Roos discusses?

(no response)

### q19 — Near-miss: Both exoplanet detection and dark matter evidence involve inferring the existence of something indirectly rather than observing it directly — what's the key methodological difference between how each is detected?

(no response)

### q20 — Edge case: This cluster spans human spaceflight history, policy/legislative documents, and pure astrophysics research — what does this range suggest about the boundary between "space exploration" and "astrophysics" as a single topic cluster?

(no response)


  ✅ Retrieval: 0.0%  Generation: 31.3%  (23.4/75)  avg 99ms
