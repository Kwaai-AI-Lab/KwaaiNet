[2m2026-06-24T19:14:55.339223Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:57870 (via ollama-proxy)
[2m2026-06-24T19:14:56.089435Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:57870
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-06-24T19:14:56.236020Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  3236ms
  [ 2/40] Who are the author's children?
[2m2026-06-24T19:14:59.748616Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  2624ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-06-24T19:15:02.591963Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2524ms
  [ 4/40] To whom is the book dedicated?
[2m2026-06-24T19:15:05.322808Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  2709ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-06-24T19:15:08.232626Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  6966ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-06-24T19:15:15.424094Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  6799ms
  [ 7/40] Who is the author's wife?
[2m2026-06-24T19:15:22.424069Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  2166ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-06-24T19:15:24.793803Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2757ms
  [ 9/40] Who was the author's grandfather?
[2m2026-06-24T19:15:27.758077Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/9 keywords  2415ms
  [10/40] Tell me about Kloof Nek.
[2m2026-06-24T19:15:30.373790Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  5057ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-06-24T19:15:35.632678Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  3186ms
  [12/40] Who was Cissie Gool?
[2m2026-06-24T19:15:39.025560Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  3314ms
  [13/40] What was the All Africa Convention?
[2m2026-06-24T19:15:42.541807Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  5036ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-06-24T19:15:47.778863Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  4385ms
  [15/40] What were the forced removals from District Six?
[2m2026-06-24T19:15:52.371432Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 1 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced relocation history and effects on residents""
  ○ Round 3   added 11 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  11609ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-06-24T19:16:04.179612Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  5608ms
  [17/40] What was Hewat Training College?
[2m2026-06-24T19:16:09.990578Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2715ms
  [18/40] What was the New Era Fellowship?
[2m2026-06-24T19:16:12.909368Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2784ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-06-24T19:16:15.897839Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  4470ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-06-24T19:16:20.567331Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  6326ms
  [21/40] Who was the author's mother?
[2m2026-06-24T19:16:27.096448Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  5692ms
  [22/40] Who was the author's father?
[2m2026-06-24T19:16:32.990761Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  3898ms
  [23/40] Who were the author's siblings?
[2m2026-06-24T19:16:37.092560Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2714ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-06-24T19:16:40.009058Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  4082ms
  [25/40] Who was I.B. Tabata?
[2m2026-06-24T19:16:44.295384Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/5 keywords  3217ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-06-24T19:16:47.717873Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  5067ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-06-24T19:16:52.984093Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  6307ms
  [28/40] Which organisations was the author involved in?
[2m2026-06-24T19:16:59.492953Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  3881ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-06-24T19:17:03.577855Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  3737ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-06-24T19:17:07.519661Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  2370ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-06-24T19:17:10.093117Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4390ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-06-24T19:17:14.688504Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  2312ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-06-24T19:17:17.203538Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  3894ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-06-24T19:17:21.299553Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  5637ms
  [35/40] Who was Hassen Mall?
[2m2026-06-24T19:17:27.140934Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/4 keywords  3659ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-06-24T19:17:31.001966Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  5116ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-06-24T19:17:36.320162Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  4066ms
  [38/40] Who was Cissie Gool's father?
[2m2026-06-24T19:17:40.592580Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/5 keywords  4046ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-06-24T19:17:44.839017Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  5252ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-06-24T19:17:50.294131Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  4829ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 79.1% (178.0/225) |
| Avg latency | 4271ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3236ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 2624ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 2524ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 2709ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | [Graph: J.M.H. Gool & Co.], LEST WE FORGET -rev25.pdf | 6966ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 6799ms |
| q07 | Who is the author's wife? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 2166ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 2757ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 2415ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 5057ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 3186ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 3314ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 5036ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 4385ms |
| q15 | What were the forced removals from District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 11609ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 7/7 (100%) | [Graph: Manilal Gandhi], LEST WE FORGET -rev25.pdf | 5608ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 2715ms |
| q18 | What was the New Era Fellowship? | 6/6 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 2784ms |
| q19 | What was the Non-European Unity Movement? | 6/6 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4470ms |
| q20 | Describe the author's involvement in cricket. | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 6326ms |
| q21 | Who was the author's mother? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 5692ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 3898ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 2714ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 4082ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 3217ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 5067ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 6307ms |
| q28 | Which organisations was the author involved in? | 4/5 (80%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3881ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 3737ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 2370ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 4390ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 2312ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 3894ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 5637ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 3659ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 5116ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 4066ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 4046ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 4/6 (67%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 5252ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4829ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources do not contain a complete list of the author's children, but they mention that his children are Feyruz, Reza, and Zarina [20], and also mentions grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [15].

### q03 — Who are the author's grandchildren?

According to [1], the author's grandchildren are:

* Joseph Rassool
* Samuel Rassool
* Alexander Rassool
* Leonardo Rassool
* Dylan Rassool
* Lewis Rassool

### q04 — To whom is the book dedicated?

The book "District Six: Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children, Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [14], [20]

### q05 — Who was J.M.H. Gool?

The provided sources do not contain a comprehensive biography of J.M.H. Gool, but they offer various details and insights into his life.

J.M.H. Gool is mentioned as the author's grandfather [1]. He was a prosperous spice merchant from India who founded the Hanaffi mosque in Cape Town [1]. He hosted notable figures such as Mahatma Gandhi, Cecil Rhodes, Dr. Abdullah Abdurahman, and George Bernard Shaw at his Buitencingle mansion in Cape Town [1].

Gool's family moved from Swat to Rander in Gujarat, and then he journeyed to the Cape via Mauritius, arriving in 1884 [1]. He had two wives: Bibi Gool (mother of Ayesha, Mohamed Saaid, and Zohra) and Wahida Gool (mother of Abdul Hamid, Goolam, Jane, Fatima, Mariam, Zobeida, and Mohammed Hanief) [1].

Gool was involved in the establishment of the Muir Street Muslim Mission School and encouraged education for his children and others, especially for women at a time when it was not common for them to pursue education beyond puberty [9]. He was described as an "Oriental trader" in Supreme Court transcripts from 1892 [8].

Gool's business partner is mentioned as Mr. Wilson (also known as J.M. Wilson) in some sources [11, 12], but this information is not corroborated by other family members [11]. He was a member of the Non-European Unity Movement and shared his vision for a non-racial democracy under the British Empire with others, including Dr. Abdurahman and Advocate Henry Sylvester Williams [11].

Gool's marriage to Wahida Ta'al may have been complicated due to cultural differences between them, as Wahida came from a prominent family in the Cape and her mother was a Dollie, whereas Gool had two wives and a history of involvement with various organizations [1, 16].

### q06 — Tell me about Buitencingle.

According to the sources, Buitencingle is a street in Cape Town, South Africa. It was built by Haji Joosub Maulvi Hamid Gool, the grandfather of Yousuf (Joe) Rassool, in the Cape Dutch tradition [1]. The name "Buitencingle" means "outer belt" or "ring" in Dutch, referring to the street's location as the outermost part of the city at that time.

The street was cobbled and featured traffic islands with stunted palm trees [3]. Opposite Buitencingle Street stood the German Lutheran Church. The street was bounded by Long Street, Loop Street, and Waterkant Street, which led to the waterfront of Rogge Bay.

Buitencingle was a vibrant neighborhood at the turn of the century, with many notable residents and visitors, including Mahatma Gandhi, Cecil Rhodes, and Prince of Wales [1]. The Gool family mansion, located at No. 7 Buitencingle Street, was a celebrated gathering place for these individuals.

In later years, the street became a hub of activity during World War II, with the Gool family playing an important role in the city's history [9]. The mansion itself was eventually demolished, but its memory lives on through the stories and accounts shared by Yousuf (Joe) Rassool.

Buitencingle is also described as a place of "haunting beauty" [6], with a wooded vale known as Kloof Nek nearby. This area featured a babbling brook, varied green canopies, and a range of wildlife, creating a magical atmosphere.

When the Gool family moved to Buitencingle, they brought their furniture and belongings, including Grandpa's tins of rusty nails [20]. However, by the time Yousuf (Joe) Rassool arrived at the mansion, it was a "dead shell" with a lost spirit, having been shut up and left to dust and rodents.

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

According to [1], Nazima Rassool is the wife of Yousuf (Joe) Rassool, the author of the District Six memoir. She is an academic researcher and Professor who urged the author to ground his writing in historical research rather than relying on memory alone. 

We also learn that she died in May 1996 [6].

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool (also known as Haji Joosub Maulvi Hamid Gool). [1]

### q10 — Tell me about Kloof Nek.

Kloof Nek is a mountain saddle in Cape Town, South Africa, connecting Table Mountain to Lion's Head [1]. It lies above Buitencingle Street and was a beloved retreat for the author and his friends, who read poetry among its trees [1].

The area is described as having a "haunting beauty" [20] and is often referred to as "the Glen" in the text. When the author was taken there by a schoolmate at age 14, he was initially reluctant but became entranced by its beauty after descending into the woodland dell [4].

Kloof Nek was frequented by various groups of people, including lovers, mountaineers, nature lovers, and ramblers, as well as down-and-outs known as "Bergies" who found refuge in the area's cavelets and bushes [18]. The area also had a bus terminus for a single-decker trackless tram that ran from Adderley Street outside Fletchers & Cartwrights department store [19].

The author and his friends, including Persotem Patel, would often visit Kloof Nek to read poetry and enjoy its natural beauty [1, 7].

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources describe the Teachers' League of South Africa (TLSA) as a professional organisation for Coloured teachers that affiliated with the Non-European Unity Movement (NEUM) to resist apartheid education policies. They employed the boycott weapon against apartheid institutions and championed non-collaboration, advocating for the rights of the Coloured community and non-European people in South Africa [1].

The TLSA had a motto: "Let Us Live for our Children" [17].

### q12 — Who was Cissie Gool?

Cissie Gool was a renowned Cape Town city councillor and community activist known for her fiery speeches and dedication to non-European rights. She was the daughter of Dr. Abdullah Abdurahman, Cape Town's first Non-European city councillor [1]. 

She was a member of the Non-European Unity Movement [1] and had children, including Shaheen Gool [1]. Cissie Gool was married to Abdul Hamid Gool [1], making her a daughter-in-law of the Gool family.

### q13 — What was the All Africa Convention?

The provided sources do not contain detailed information about the All Africa Convention, but they do mention it in several excerpts.

Excerpts [1], [4], and [19] suggest that the All African Convention (not "All Africa") was an organization associated with the Non-European Unity Movement (NEUM). The TLSA, as mentioned in excerpt [1], affiliated with the NEUM to resist apartheid education policies. 

The Convention is also mentioned in excerpts [3] and [20]. Excerpt [3] states that Vic Wessels, Les Jacobs, and the author went to represent the New Era Fellowship at the conference of the All African Convention in Bloemfontein in 1954.

Excerpt [19] mentions the convention but does not provide further details about its purpose or activities. 

Excerpts [15], [17], and [20] contain lists that mention "All African Convention" (not "All Africa") as a separate entity, suggesting it was an organization distinct from the New Era Fellowship.

The provided sources do not provide a clear definition of what the All African Convention represented or its goals, but they do indicate its association with the Non-European Unity Movement and its connection to other organizations in the anti-apartheid movement.

### q14 — Where was District Six and what kind of place was it?

According to [1], District Six was a multicultural community in Cape Town, nestled below Devil's Peak. It was celebrated for its jazz and community spirit, and described as having "more life, more zest, more fire, more warmth and care than many other places one could mention." [1]

[2] describes it similarly: "Come for a stroll along the mean streets of life... To me those bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention."

It is mentioned that District Six was home to diverse residents, including Coloureds, Malays, Africans, and others. [1]

[19] notes that the legislation related to Group Areas Act played havoc with the lives of ordinary people in Cape Town, but specifically states that District Six began to die in the early fifties as shops in Hanover Street started closing.

Overall, it appears that District Six was a vibrant and diverse community with a strong sense of life and spirit.

### q15 — What were the forced removals from District Six?

According to [1], under the Group Areas Act, non-White residents of District Six were expelled to the Cape Flats. Their homes were demolished and the land bulldozed flat, leaving only the mosques and churches standing.

Additionally, [9] states that when a Coloured Area was declared White: "the house prices plummeted as the new buyers were aware that the Coloured owners, who realized that the Group Areas Board’s valuation of their properties was generally lower than market value." This led to a gradual but definite hemorrhaging of the community, with people moving out and eventually, only a handful needed to be persuaded by the authorities to quit... or face the rigors of the law.

[19] also mentions the forced removals in the context of Apartheid policy: "Apartheid legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life."

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the sources:

* [1] Manilal Gandhi is mentioned as the son of Mahatma Gandhi.
* [3] E.S. Reddy notes that the Gool family and the family of Dr. Abdurahman were central to the history of District Six and of the Indians and Coloured people of South Africa, suggesting a connection between the two families through their shared experiences and activism.
* [5] A letter from Gandhi references Wilson in connection with Gool, and Gool submits subscription dues for the Indian Opinion on behalf of Wilson, indicating that Gool was involved with Gandhi's newspaper and possibly had business connections with him.

However, it is specifically mentioned in:

* [1] Manilal Gandhi: "Manilal Gandhi was the son of Mahatma Gandhi — Mohandas Karamchand Gandhi — who led the passive resistance and non-violent satyagraha movement in South Africa."
* [9] Phoenix, Natal: According to the Indian Opinion (March 1914), Gandhi thanked the Gool family for their services to Mrs. Gandhi during her illness.
* [10] Works of Mahatma Gandhi Vol 11: In October 1912, Gandhi was a guest at 7 Buitencingle Street and received G. H. Gokhale, who was welcomed by Dr. Gool.

This suggests that the connection between Gandhi and the Gool family is primarily through their shared involvement in activism and community service in South Africa during the early 20th century.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college for Coloured students in Cape Town, preparing teachers for Coloured schools [1]. It allowed students to gain practical classroom teaching experience during their college years [1]. The college had close connections with the political and cultural life of the Cape Coloured community through its teachers and staff [1].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a Cape Town political and cultural organisation affiliated with the Non-European Unity Movement (NEUM). It hosted regular public lectures and debates on democracy, non-racialism, and resistance to apartheid. The NEF was an intellectual forum for developing ideas of non-collaboration among non-European communities in South Africa [1].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a South African anti-apartheid political movement founded on a Ten Point programme of full democratic rights [1]. Its core principle was non-collaboration - the refusal of non-European people to collaborate with apartheid institutions. The NEUM deployed the boycott as its primary political weapon, most notably the boycott of the Coloured Advisory Council (C.A.C.) in 1943 [1].

The movement was closely affiliated with the New Era Fellowship and had a significant presence among teachers, with some members being leaders of the Non-European Unity Movement, such as Dr. Goolam and his sister Aunt Jane [15]. The NEUM's ideals included non-collaboration, the refutation of "race", and the ideal of a non-racial society [9].

The movement's activities are described in several excerpts, including the selling of the newspaper "The Torch" every Tuesday evening, which was successful in raising sales from four dozen to thirteen dozen [12].

### q20 — Describe the author's involvement in cricket.

The author appears to have been heavily involved in cricket from a young age. He played for the Kismets Cricket Club, which was a District Six club competing in the Western Province Indian Cricket Union's non-European cricket competitions [1]. The author mentions that he was not chosen for the final team to tour Johannesburg, but was eventually selected four years later [7].

He also mentions that he had several friends who were excellent cricketers, including Hassen Mall and Mylie Begg [5], [17]. He recalls being impressed by his sister's brother-in-law, Mylie Begg, who had the ability to frustrate and break the heart of any fast bowler in the union at that time.

The author was also involved in organizing cricket matches and tournaments. He tried to organize a union to facilitate the playing of cricket at High School and College level but was thwarted by the entrenched bureaucracy of the Schools' Sports Union [11].

He mentions that he attended several cricket meetings, including one where the proposed visit of a West Indian Cricket team to South Africa with players of the calibre of the Three W's; Worrel, Weekes and Walcott was discussed [4], [10], [18]. He also mentions that he had to convince members that by supporting Apartheid cricket they would be relinquishing principle in favour of expediency [19].

The author's club, the Kismets, followed a non-racial policy and transformed the Indian Cricket Union to such an extent that several teams joined the Sunday League, including the Heathrow Cricket Club, which brought new talents [15]. Through cricket, he became close friends with Muddy Allie [16].

Overall, it appears that the author was deeply involved in cricket throughout his life, playing for a club team, organizing matches and tournaments, and advocating for non-racial policies in the sport.

### q21 — Who was the author's mother?

The provided sources do not explicitly state the full name of the author's mother. However, in [1], it is mentioned that Ayesha Rassool had children named Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf Rassool, Zain Rassool, Fazil Rassool, Yasmin Rassool. 

In [9], it is mentioned that Bibi looked at her firstborn and said "Your first born ... if it is a girl call her Ayesha." and later on, it is stated that "My mother, dark-haired and sad eyed, was born nine months later on the 10th December 1900. I am sure Bibi was in seventh heaven with the baby, duly named Ayesha."

It can be inferred from [9] that the author's mother was named Ayesha, but her full name is not explicitly mentioned in the provided sources.

In [5], it is stated "My father was handsome, bronzed like his mother, with hair slicked back, tall and sturdy. He had left school early, as soon as he was able to copy his letters and read the bible, and became apprenticed to a cobbler. My grandfather found him a position in the firm, probably because of his neat copperplate handwriting. It was there that he became engaged to my mother." 

It can be inferred from [5] that the author's father became engaged to Ayesha (the author's mother).

### q22 — Who was the author's father?

The provided sources do not contain that information directly. However, they mention "my dad" or "Peter Alexander Rassool thenceforth became known as Peerbhai or Peru (as my mother called him)" [15], indicating that Peter Alexander Rassool was the author's father.

Further information about his father is available in source [16]: He was a young man who had left school early, got apprenticed to a cobbler, and later became engaged to Ayesha from among all the spinsters of Castle Street and Buitencingle. His name is Peter Alexander Rassool.

Additionally, in source [15], his father's involvement with Messaris Junior, "Tykie," who assured him that he would have no difficulty joining the Dukes because Dad was fair enough to pass as White!

### q23 — Who were the author's siblings?

The author's siblings are listed in [1] as follows:

* Abdul Rassool
* Fazil Rassool
* Zain Rassool
* Rasheda Rassool
* Berina Rassool
* Yasmin Rassool
* Nasim Rassool

### q24 — Who were the children of J.M.H. Gool?

The sources indicate that J.M.H. Gool had two wives: Bibi Gool and Wahida Gool. The children of J.M.H. Gool are listed as:

* From Bibi Gool:
	+ Ayesha Rassool
	+ Mohamed Saaid Gool
	+ Zohra Abdurahman
* From Wahida Gool:
	+ Abdul Hamid Gool
	+ Jane Gool-Tabata
	+ Minnie Gool
	+ Zobeida Gool
	+ Goolam Gool
	+ Fatima Gool
	+ Mariam Gool
	+ Mohamed Hanief Gool

Note that some of these individuals are mentioned in multiple sources, and the consistency of their relationships is clear.

### q25 — Who was I.B. Tabata?

The provided sources do not contain a detailed biography of I.B. Tabata. However, [2] mentions that "Shortly after becoming chairman of the New Era Fellowship, Mr. I.B. Tabata completed his book The Awakening of a People."

It is also mentioned in various other excerpts that he was involved with the Unity Movement and had written an indictment against Ben Kies's analysis (e.g., [1], [3]), which suggests that Tabata played a significant role within the movement.

### q26 — Who was Dr. Abdullah Abdurahman?

[1] states that Dr. Abdulla Abdurahman (also written as Dr. Abdullah Abdurahman) was a pioneering Cape Town doctor and Coloured city councillor who became an influential politician, serving on the Cape Town city council as a leading Non-European representative and advocate for the rights of the Coloured community.

He was also mentioned in [16] as being closely associated with J.M.H. Gool and having a residence in Loop Street, near Buitencingle, which facilitated co-operation between them. 

Dr. Abdurahman's family is also mentioned: his grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [14]. His mother, Gadija Dollie, was said to be a most beautiful woman. 

[19] mentions that Dr. Abdullah Abdurahman was one of many notable individuals who visited or stayed at Buitencingle, including Gandhi, Cecil Rhodes, George Bernard Shaw, and Sarojini Naidu.

The sources confirm that Dr. Abdullah Abdurahman was an important figure in Cape Town's Coloured community and a prominent politician.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources suggest that Gandhi and J.M.H. Gool had a close relationship, with several instances of their interactions documented. Some key points include:

* [4] mentions that Gandhi's son fell in love with one of Goulam Gool's sisters, a Muslim girl.
* [8] notes that the Gool family invited Gandhis to stay at their mansion at 7 Buitencingle Street in February 1914, and this led to a friendship between Joe Rassool (who is not directly related to J.M.H. Gool) and Yousuf (Joe) Rassool.
* [8] also mentions that Gandhi received Elizabeth Molteno and other European women at the Gool residence during his satyagraha campaign in 1914.
* [10] notes that Gandhi helped putty the floor of Dr. A.H. Gool's surgery in 1911.
* [12] and [18] mention that Gandhi was a guest at the Gool mansion on several occasions, including in 1911 when he came to receive G.H. Gokhale.
* [19] notes that Gandhi had arranged for meals with J.M.H. Gool's family when they visited Cape Town.

These interactions suggest a close friendship or acquaintance between Gandhi and J.M.H. Gool, as well as the Gool family. The exact nature of their relationship is not specified in the sources.

Additionally, [3] suggests that J.M.H. Gool was part of a pan-Islamic network between Liverpool, Capetown, and Perth, which may have been connected to Gandhi's interests in Indian nationalism and Islamic reform.

[4] mentions that Gandhi's son fell in love with one of Goulam Gool's sisters.

### q28 — Which organisations was the author involved in?

The provided sources mention that Yousuf (Joe) Rassool was involved with the following organisations:

1. New Era Fellowship [1, 4]
2. Non-European Unity Movement (NEUM) is not mentioned directly but a reference to "the Movement" implies involvement [9, 13, 17]
3. Teachers League of South Africa (TLSA) or Teachers' League of South Africa [10, 11, 15]
4. The Anti-Coloured Affairs Council (Anti-CAC) or the Anti-Coloured Affairs Department (Anti-CAD) [5]
5. Fourth International of South Africa [5]

Additionally, Yousuf Rassool mentions that his uncle Dr. A.H. Gool was involved with:

1. Natal Indian Congress
2. Cape British Indian Union

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA), representing Coloured teachers, was closely affiliated with the Non-European Unity Movement (NEUM) [1]. The TLSA adopted the same boycott and non-collaboration principles as the NEUM [1]. Ben Kies, a leading figure in the TLSA, was also among the leading voices in the NEUM's non-European unity campaign [1].

The TLSA had broken with its previous stance of non-political professionalism and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was bound up with the struggle for a democratic society [5].

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The sources do not contain information about when J.M.H. Gool arrived in Cape Town and from where.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is a mosque located in Loop Street, Cape Town, South Africa. According to the text, it was an outpost of Islam in an area that was becoming increasingly European, Jewish, and gentile.

The mosque was completed in 1898 and was founded by Yousuf's (the author's) grandfather. He was a life trustee of the mosque, and his family would often go there to celebrate Eid festivals. The mosque had a distinctive design, with a whitewashed structure, a domed minaret, and a spire topped with a moon and star.

It is worth noting that the text also mentions the Quwatul Islam Mosque, which appears to be another name for the same mosque.

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not contain that information.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources list several notable historical figures that J.M.H. Gool knew personally, including:

* Mahatma Gandhi [1]: His friendship with Gandhi is documented in the Indian Opinion.
* Cecil Rhodes [1]: He was one of Haji Joosub Maulvi Hamid Gool's acquaintances at Buitencingle Street.
* Solly Joel [1]: He was also an acquaintance of J.M.H. Gool, as mentioned in [1].
* Dr. Abdulla Abdurahman [3], [13], and [18]: The sources indicate that the two men collaborated on various projects and lived close to each other.

These individuals were likely acquaintances or friends, although the nature of their relationships is not specified in the provided excerpts.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [1, 15] was legislation introduced by the South African government to enforce racial segregation and create separate areas for different racial groups. It was declared a White Group Area under the Group Areas Act in 1966, which meant that non-White residents were expelled to the Cape Flats, their homes were demolished, and the land bulldozed flat [1]. The forced removals destroyed one of Cape Town's most vibrant communities.

The Group Areas Act also had significant effects on the people living in District Six. It led to the classification of families like Rassool's, where some members were classified as Indian and others as Malay, creating a problem due to their complex family structure [10]. Many residents lost their properties when the areas were reclassified and demolished, leading to displacement and poverty.

The Group Areas Act was also ineffective in paralysing the Coloured Affairs Council, but it had no impact on those who owned property or those without any problems with the law. Some members of the “Movement” simply ignored the injunction to boycott [4]. 

The Group Areas Act also resulted in the residents losing the municipal franchise and being effectively disfranchised due to sub-economic schemes they were moved into, such as Silvertown [18]. These houses were designed for people living below the poverty line, and even if their income increased above this threshold, they had no choice but to vacate these homes.

These effects are evident in the sources, which describe the impact of the Group Areas Act on District Six and its residents.

### q35 — Who was Hassen Mall?

Hassen Mall is mentioned in multiple excerpts as a person who had a significant impact on the narrator's life. He was a student from Durban staying with Muddy's family and later did medicine at Cape Town. He played cricket for the team, captaining it during the biennial tournament in 1951, and seemed to be a charismatic leader and skilled cricketer. He also supported the narrator in his early attempts at playing cricket and showed confidence in him, which helped boost the narrator's morale and enthusiasm for the game.

Source: [Excerpts 2, 4, 7, 11, 14, 15]

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources do not contain a comprehensive list of all political organisations that were active in the Cape Coloured community during the author's lifetime. However, the following organisations are mentioned:

1. New Era Fellowship: A cultural organisation established in 1937 to discuss various topics and counter the predicted assault on the rights of the "Coloured" community.
2. Anti-Coloured Affairs Council (Anti-CAC) / Anti-Coloured Affairs Department (Anti-CAD): Formed to oppose the Coloured Advisory Council, with the goal of achieving unity and democratic rights for the Coloured community.
3. African People's Organisation (APO)
4. Communist Party of South Africa
5. Fourth International of South Africa
6. Liberation League
7. Workers' Party
8. Coloured branch of Smuts's United Party
9. Coloured Affairs Department (CAD)
10. Coloured Advisory Council (CAC)

Additionally, the sources mention other organisations that were involved in the struggle for the rights of the Coloured community, such as:

1. The Cape Standard: A weekly newspaper catering to the "Coloured" community.
2. The Guardian: The mouthpiece of the South African Communist Party.

It is also mentioned that various sports clubs and churches were involved in the struggle, but specific names are not provided.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to excerpt [1], Mahatma Gandhi was an Indian leader who came to South Africa in 1893 and stayed there until his departure on July 18, 1914. He championed the rights of Indians through passive resistance and non-violent satyagraha.

Excerpt [18] provides more information about why he was in South Africa: Gandhi initially arrived in South Africa in 1893 for a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria. However, when the Natal Government introduced a bill to disenfranchise Indians in 1894, Gandhi chose to stay on and help the Indian community.

Gandhi's time in South Africa was marked by his efforts to advocate for the rights of Indians through non-violent means, which ultimately had a lasting impact on the country.

### q38 — Who was Cissie Gool's father?

According to [4], Dr. Abdurahman was Cissie Gool's father. He was a doctor who qualified from Glasgow University in 1893, and his mother was Gadija Dollie. 

Additionally, [2] mentions that Dr. Abdurahman was present at a meeting with my grandfather (presumably Joe Rassool) on January 9th, 1904, where he asked about the higher education of coloured people.

[6] also refers to Cissie Gool's father, Dr. Abdurahman, in the context of the Abdurahman clan and their relationship with the Ta'Als through the Dollie family.

So, based on these excerpts, we can confirm that Cissie Gool's father was Dr. Abdurahman.

### q39 — What was District Six like as a neighbourhood before the forced removals?

Based on the provided sources, here's what can be inferred about District Six as a neighborhood before the forced removals:

* It was a multicultural community with diverse residents including Coloured, Malay, African, and others (Source [1]).
* The area was known for its jazz scene and community spirit (Source [1]).
* It had a vibrant cosmopolitan atmosphere, with people from different cultures living together (Source [2] and [4]).
* The neighborhood had a unique character, with narrow streets, terraced houses, and a mix of old and new buildings (Source [5], [6], and [7]).
* Caledon Street, in particular, was described as having "a unique buzz all of its own" (Source [11]), with a lively atmosphere and a diverse range of residents.
* The neighborhood had a rich cultural heritage, with a mix of African, Malay, and European influences (Source [1] and [16]).
* It was also a hub for community events, gatherings, and celebrations, such as Eid festivities (Source [5]).
* Despite its vibrant atmosphere, District Six was also known for its poverty, overcrowding, and poor living conditions (Source [3], [9], and [18]).

Overall, District Six was a dynamic and diverse neighborhood with a strong sense of community, but it was also facing significant social and economic challenges before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is not explicitly stated in all the sources provided [1-20]. However, some key points can be synthesized from various excerpts:

* The boycott was a primary weapon for the Unity Movement (NEUM) to fight against apartheid institutions. [1]
* In 1943, the NEUM deployed the boycott of the Coloured Advisory Council (C.A.C.) as its primary political action. [1]
* Members who accepted service on the C.A.C. were boycotted in every aspect of their lives, socially and economically. [13]
* When dealing with the Group Areas Act, some members simply ignored the injunction to boycott, which was seen as ineffective by the NEUM. [6]
* In elections, the Movement's stand was to boycott the election, but if "Coloured" voters insisted on voting, they were asked to spoil their ballot by writing: "For Full Democratic Rights" across the ballot slip. [2]

It appears that the Unity Movement used boycotts as a means of non-collaboration with apartheid institutions and to bring attention to its demands for full democratic rights. However, it is unclear if there was a single, unified policy governing all instances of boycotting.


  ✅ Overall: 79.1% recall (token-overlap)  (178.0/225)  avg 4271ms
