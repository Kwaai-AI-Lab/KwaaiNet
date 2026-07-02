[2m2026-06-24T17:49:22.085393Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:52567 (via ollama-proxy)
[2m2026-06-24T17:49:22.096448Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:52567
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-06-24T17:49:22.156084Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  3312ms
  [ 2/40] Who are the author's children?
[2m2026-06-24T17:49:25.684200Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  2268ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-06-24T17:49:28.154386Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2605ms
  [ 4/40] To whom is the book dedicated?
[2m2026-06-24T17:49:30.978315Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  2341ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-06-24T17:49:33.509836Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  4657ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-06-24T17:49:38.366665Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  6079ms
  [ 7/40] Who is the author's wife?
[2m2026-06-24T17:49:44.653772Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  3338ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-06-24T17:49:48.190145Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  2981ms
  [ 9/40] Who was the author's grandfather?
[2m2026-06-24T17:49:51.372934Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/9 keywords  2290ms
  [10/40] Tell me about Kloof Nek.
[2m2026-06-24T17:49:53.871269Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  5252ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-06-24T17:49:59.322642Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2923ms
  [12/40] Who was Cissie Gool?
[2m2026-06-24T17:50:02.447580Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4551ms
  [13/40] What was the All Africa Convention?
[2m2026-06-24T17:50:07.201384Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  3374ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-06-24T17:50:10.795154Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  3657ms
  [15/40] What were the forced removals from District Six?
[2m2026-06-24T17:50:14.643907Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 1 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced removals South Africa history""
  ○ Round 3   added 14 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  12838ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-06-24T17:50:27.682649Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  5505ms
  [17/40] What was Hewat Training College?
[2m2026-06-24T17:50:33.391373Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2605ms
  [18/40] What was the New Era Fellowship?
[2m2026-06-24T17:50:36.198513Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2718ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-06-24T17:50:39.119985Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  3905ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-06-24T17:50:43.227907Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  7055ms
  [21/40] Who was the author's mother?
[2m2026-06-24T17:50:50.485465Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  2099ms
  [22/40] Who was the author's father?
[2m2026-06-24T17:50:52.827797Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  3586ms
  [23/40] Who were the author's siblings?
[2m2026-06-24T17:50:56.582000Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3303ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-06-24T17:51:00.088825Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  3857ms
  [25/40] Who was I.B. Tabata?
[2m2026-06-24T17:51:04.148922Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/5 keywords  3504ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-06-24T17:51:07.854911Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  3662ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-06-24T17:51:11.718954Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  5437ms
  [28/40] Which organisations was the author involved in?
[2m2026-06-24T17:51:17.356657Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  3994ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-06-24T17:51:21.557640Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  3657ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-06-24T17:51:25.415138Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  2484ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-06-24T17:51:28.104204Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4571ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-06-24T17:51:32.880128Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  4276ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-06-24T17:51:37.359912Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/7 query terms found  (86%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  4190ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-06-24T17:51:41.749086Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  6352ms
  [35/40] Who was Hassen Mall?
[2m2026-06-24T17:51:48.303061Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  4353ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-06-24T17:51:52.859946Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/6 keywords  6497ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-06-24T17:51:59.570486Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  3295ms
  [38/40] Who was Cissie Gool's father?
[2m2026-06-24T17:52:03.065853Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/5 keywords  2931ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-06-24T17:52:06.200910Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  6370ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-06-24T17:52:12.772777Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m97 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  2947ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 82.7% (186.0/225) |
| Avg latency | 4140ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 3312ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 2268ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 2605ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 2341ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | LEST WE FORGET -rev25.pdf, [Graph: J.M.H. Gool & Co.] | 4657ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 6079ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 3338ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 2981ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 2290ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 5252ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 6/6 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 2923ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 4551ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 3374ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 3657ms |
| q15 | What were the forced removals from District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 12838ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 7/7 (100%) | [Graph: Manilal Gandhi], LEST WE FORGET -rev25.pdf | 5505ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 2605ms |
| q18 | What was the New Era Fellowship? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 2718ms |
| q19 | What was the Non-European Unity Movement? | 6/6 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 3905ms |
| q20 | Describe the author's involvement in cricket. | 4/5 (80%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 7055ms |
| q21 | Who was the author's mother? | 3/5 (60%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 2099ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 3586ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3303ms |
| q24 | Who were the children of J.M.H. Gool? | 5/7 (71%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 3857ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 3504ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 3662ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 5437ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 3994ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 3657ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3/6 (50%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 2484ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 4571ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 4276ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 4190ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 6352ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 4353ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 6497ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 3295ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 2931ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 6370ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 2947ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

According to [1], the author's children are Feyruz, Reza, Zarina, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

### q03 — Who are the author's grandchildren?

The provided sources do not contain a complete list of all of Yousuf Rassool's grandchildren. However, they mention that his six grandchildren are: 

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis 

[1] and [20]

### q04 — To whom is the book dedicated?

The book is dedicated "For my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis" [5].

### q05 — Who was J.M.H. Gool?

According to the provided sources, [1] describes J.M.H. Gool as Joosub Maulvi Hamid Gool, the grandfather of the author. He is described as a prosperous spice merchant from India who founded the Hanaffi mosque in Cape Town. 

Source [9] states that J.M.H. Gool's world outlook was constructed less around the tenets of his religion and more around the vision of a non-racial democracy under the umbrella of the British Empire.

Other sources mention various aspects of J.M.H. Gool's life, such as his business dealings (Sources [10] and [14]), his relationships with other notable figures in Cape Town (Source [12]), and his involvement in the early days of South African politics (Source [18]). However, these additional details do not provide a comprehensive biography of J.M.H. Gool.

The sources do not contain any information about the relationship between J.M.H. Gool and Cissie, who was mentioned as his daughter-in-law in Source [1].

### q06 — Tell me about Buitencingle.

Buitencingle was a cobbled street in Cape Town at the foot of Table Mountain. It was built by the author's grandfather, Haji Joosub Maulvi Hamid Gool [1], and was considered one of the outermost belts of Cape Town. The name "Buitencingle" comes from the Dutch word for belt or cingle [17].

The street was wide and had traffic islands in the middle with stunted palm trees. Opposite Buitencingle Street stood a German Lutheran Church, which became active on Sundays with a clamour of bells [19]. The area around Buitencingle was becoming increasingly European, Jewish, and gentile.

Buitencingle was home to the author's family, including their grandfather, who built a 13-room mansion at number 7 Buitencingle Street. This mansion became a gathering place for notable visitors such as Solly Joel, Cecil Rhodes, Mahatma Gandhi, the Prince of Wales, George Bernard Shaw, and Sarojini Naidu [1].

The author describes Buitencingle as having a certain magic in its atmosphere, with the scent of shrubs, herbs, and flowers mixing with the smell of rich moist earth [8]. The area had several notable features, including the Glen, which was a wooded vale with a babbling brook and was considered a place of haunting beauty [18].

Buitencingle was also close to the South African Public Library in the Gardens, where the author's friend Persotem used to meet his classmate Khadija on Saturdays [16]. The area had a mix of cultures, including European, Jewish, and Muslim communities.

Overall, Buitencingle seems to have been a vibrant and eclectic neighborhood with a rich history.

### q07 — Who is the author's wife?

The sources do not contain that specific information directly. However, they mention that Nazima Rassool (also known as Mrs. Yousuf Rassool) is an academic researcher and Professor who is the author's wife [1] and that she urged the author to ground his writing in historical research rather than relying on memory alone.

Later, another source mentions that "my wife, who is an academic researcher, and who urged me to ground my writing in background investigation" [20]. 

These sources do not provide a direct name or further details about the author's wife beyond these mentions.

### q08 — Tell me more about the author's wife.

According to [1], Nazima Rassool is the author's wife, and she is described as a Professor who encouraged the author to conduct research on their family history rather than relying solely on memory.

Additionally, [19] mentions that the author's wife (Nazima) is an academic researcher, which suggests that she has professional expertise in research methods and critical thinking. This expertise likely influenced the author's approach to researching his family history and helped him uncover new information about their past.

### q09 — Who was the author's grandfather?

The author's grandfather was Haji Joosub Maulvi Hamid Gool [1].

### q10 — Tell me about Kloof Nek.

According to the sources, Kloof Nek is a mountain saddle in Cape Town that connects Table Mountain to Lion's Head [1]. It lies above Buitencingle Street and was a beloved retreat for the author and his friends, who would often read poetry among its trees [1].

Kloof Nek has a wooded dell called the Glen, which is described as being of "haunting beauty" [17]. The area was frequented by lovers, mountaineers, nature lovers, and ramblers, but also had a growing number of down-and-outs known as "Bergies", who would drink cheap wine and methylated spirits in its cavelets and bushes [18].

A bus terminus at Kloof Nek provided access to the area, and the Glen was considered an ideal place for studying poetry due to its natural beauty and tranquility. The author has fond memories of visiting Kloof Nek with his friends, including Persotem Patel and Ronnie, who would read and enjoy literature together in the peaceful surroundings [4, 18].

Kloof Nek Road, which connects Table Mountain to Lion's Head, provides a scenic route through the area, and the saddle offers breathtaking views of Table Bay from its eastern flank. The noonday gun on Signal Hill can be heard regularly from this location [20].

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional organization for Coloured teachers that affiliated with the Non-European Unity Movement (NEUM) to resist apartheid education policies. [1] The TLSA employed the boycott weapon against apartheid institutions and championed non-collaboration, advocating for the rights of the Coloured community and non-European people in South Africa.

### q12 — Who was Cissie Gool?

Cissie Gool was a Cape Town city councillor and community activist known for her fiery speeches and dedication to non-European rights [1]. She was the daughter of Dr. Abdullah Abdurahman, Cape Town's first Non-European city councillor [1] and married Dr. Abdul Hamid Gool (son of Haji Joosub Maulvi Hamid Gool), making her a daughter-in-law of the Gool family [1].

Cissie Gool was also a member of the Non-European Unity Movement [1]. According to sources, she had children named Shaheen Gool and possibly other daughters with their names listed as "Zohra 'Jolly' Gool, Mariam 'Margie' Gool, Fatima 'Timmie' Gool" in [4].

Cissie Gool was known for her involvement in politics and activism. She had a significant impact on the community, particularly during a massive demonstration she led against residential segregation in 1937, where she roused the crowd with her passionate oratory and singing [20].

### q13 — What was the All Africa Convention?

The provided sources do not contain detailed information about the All Africa Convention. However, [9] mentions that it is one of the pillars of the Non-European Unity Movement (NEUM), and [18] suggests that it was an organization involved in anti-apartheid efforts.

[4] does mention Mr. Tabata's book "The Awakening of a People," which recorded the coming into existence of the All African Convention, implying that the convention was related to or part of the NEUM movement.

Further research would be needed to provide a more detailed description of the All Africa Convention.

### q14 — Where was District Six and what kind of place was it?

According to source [1], District Six was a multicultural community located below Devil's Peak in Cape Town, celebrated for its jazz and community spirit. It was home to diverse residents including Coloured, Malay, African, and others.

Source [4] describes it as "a vibrant community" that scattered and disintegrated under the effects of Apartheid. Source [18] mentions that District Six was a place where beauty did not always lie in being white, indicating a sense of diversity and inclusivity.

Sources [1], [2], and [18] all mention the forced removals and demolition of homes in District Six due to the Group Areas Act, which was a legislative measure aimed at segregating different racial groups into separate areas.

### q15 — What were the forced removals from District Six?

The provided sources describe the forced removals as follows:

[1] The community suffered forced removals: non-White residents were expelled to the Cape Flats; their homes were demolished and the land bulldozed flat, leaving only the mosques and churches standing.

[7] In 1950, the Group Areas Act dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life.

[15] The government worked on an age-old theory that people tended to anticipate the wishes of the authorities. They knew that to resist to the bitter end often meant to accept homes in less favourable places where roads and facilities were non-existent, far from schools and shopping, and far from friends and relations. Therefore, most decided it was unwise to wait till the axe fell, and moved out with the result that when the final day arrived, only a handful needed to be “persuaded” by the authorities to quit... or face the rigours of the law.

[16] A potent source of misery occurred when a Coloured Area was declared White: the house prices plummeted as the new buyers were aware that the Coloured owners, who realized that the Group Areas Board’s valuation of their properties was generally low.

Note that the sources provide varying levels of detail about the forced removals, but they all agree that non-White residents were forcibly removed from District Six and relocated to the Cape Flats.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi [1] led the passive resistance and non-violent satyagraha movement in South Africa. He came to South Africa from India in 1893 and maintained a close friendship with the Gool family, staying at their mansion at Buitencingle in Cape Town in 1914 during a critical phase of the satyagraha campaign.

The Gool family was central to the history of District Six and the Indians and Coloured people of South Africa [3]. Manilal Gandhi, Mahatma Gandhi's son, lived and worked in South Africa for decades, editing the Indian Opinion newspaper and carrying forward his father's non-violent legacy. 

Mahatma Gandhi had a close friendship with the Gool family, particularly Dr. A.H. Gool, who was a medical student at Guy’s Hospital in London [7]. Dr. Gool repaid Gandhi's hope in him by providing attentive medical care and writing articles for Indian Opinion.

Gandhi visited the Gool mansion several times, including in 1912 and 1914. In February-March 1914, he stayed with the Gools while his wife Kasturba was being cared for at their house [14]. The Gools provided hospitality to Gandhi during his stay in Cape Town, and he reciprocated by praising them publicly for their services to him and his wife during her illness.

Gandhi's correspondence with Dr. JMH Gool shows the closeness of their relationship [18].

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college for Coloured students in Cape Town, preparing teachers for Coloured schools [1]. Teachers trained at Hewat gained practical classroom teaching experience during their college years and it was closely connected to the political and cultural life of the Cape Coloured community through its teachers and staff.

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a Cape Town political and cultural organisation affiliated with the Non-European Unity Movement (NEUM). It hosted regular public lectures and debates on democracy, non-racialism, and resistance to apartheid [1]. The NEF provided a platform for developing the ideas of non-collaboration among non-European communities in South Africa.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a South African anti-apartheid political movement founded on a Ten Point programme of full democratic rights. Its core principle was non-collaboration — the refusal of non-European people to collaborate with apartheid institutions. The NEUM deployed the boycott as its primary political weapon, most notably the boycott of the Coloured Advisory Council (C.A.C.) in 1943 [1].

The movement was affiliated with the New Era Fellowship and The Teachers League of South Africa (TLSA), which represented Coloured teachers and adopted the same boycott and non-collaboration principles. The TLSA's teachers were among the leading voices in the NEUM's non-European unity campaign [1].

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool (also referred to as Joe), had a significant involvement in cricket throughout his life. He played club cricket for the Kismets Cricket Club and was a member of the Western Province Indian Cricket Union.

At the age of 14 (source [1]), he started playing cricket alongside friends like Hassen Mall, and the club championed non-racial cricket in District Six.

Throughout his teenage years, Rassool continued to play cricket for Kismets, and despite being a good player, he didn't make it into the final team that went on tour to Johannesburg (source [12]).

In 1947, when he was still at school, Rassool's innings at Rosmead made an impression, but unfortunately, no selector had seen his performance, which might have influenced their decision not to choose him for the biennial tournament (source [15]).

The author also mentions that a great deal depended on his ability to convince members of the union that supporting Apartheid cricket would be relinquishing principle in favor of expediency. He opposed playing against the West Indians, as it would be seen as playing into the hands of the regime (source [18]).

Rassool's passion for cricket led him to become close friends with Muddy, who also shared his interest in poetry and politics. Their club, Kismets, transformed the Indian Cricket Union by adopting a non-racial policy, which attracted new teams, including the Heathrow Cricket Club and the Orient Cricket Club (source [14]).

The author's involvement in cricket was not limited to playing; he also worked as a secretary for the Western Province Indian Cricket Union, which brought about significant changes, such as moving from The Green Point Common to better grounds in Athlone and arranging film shows to raise funds for the team's tour (source [14]).

Rassool's favorite batsman was Mylie Begg, who had an impressive ability to frustrate and break the heart of any fast bowler in the union at that time. He considered Begg a great opening bat and a gentle person with a great sense of humor (source [17]).

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool [1].

### q22 — Who was the author's father?

The sources do not clearly state who the author's father was. However, it can be inferred from [15] that the author is discussing their family members, including their uncle Peerbhai (Peter Alexander Rassool), which suggests that their mother may have been Ayesha (Lallie) and their father may have been Peter Alexander Rassool, who converted to Islam and became known as Peerbhai or Peru. 

However, this is not explicitly stated in the sources, and it is also possible that the author's father was someone else mentioned in the text, such as Malick Rassool [14].

### q23 — Who were the author's siblings?

The provided sources do not contain information about all of Yousuf (Joe) Rassool's siblings, but they mention the following:

* Abdul Rassool [1]
* Fazil Rassool [19]
* Zain Rassool [1]
* Rasheda Rassool [1]
* Berina Rassool [1]
* Nasim Rassool [1]
* Yasmin Rassool [1]

The sources also mention that Yousuf (Joe) Rassool had other siblings, but their names are not provided.

### q24 — Who were the children of J.M.H. Gool?

According to [1], the children of Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) are:

* Abdul Hamid Gool
* Jane Gool-Tabata
* Minnie Gool
* Zobeida Gool
* Goolam Gool
* Fatima Gool
* Mariam Gool
* Mohamed Saaid Gool
* Ayesha Rassool
* Mohammed Hanief Gool
* Zohra Abdurahman

Note that the provided sources also mention other relationships, such as grandchildren and foster children, but this list only includes the children of J.M.H. Gool mentioned in [1].

### q25 — Who was I.B. Tabata?

The provided sources do not contain a detailed biography of I.B. Tabata. However, they mention that Mr. I.B. Tabata:

* Completed his book "The Awakening of a People" [14].
* Was the author of "The Awakening", which was criticized by the Forum Club as lacking a class analysis and being Black Nationalist [20].
* Was mentioned to be involved in the Cape Peninsula Students Union, with some people believing he was behind its establishment [8].
* Wrote an indictment against Ben Kies's analysis [1].

The sources do not provide any further information on I.B. Tabata's background or personal life.

### q26 — Who was Dr. Abdullah Abdurahman?

According to source [1], Dr. Abdulla Abdurahman (also written as Dr. Abdullah Abdurahman) was a pioneering Cape Town doctor and Coloured city councillor who became an influential politician, serving on the Cape Town city council as a leading Non-European representative and advocate for the rights of the Coloured community.

[No further information is provided in the sources about his personal life or activities beyond what is mentioned in source [1].]

Note: The name "Abdulla" and "Abdullah" are variants, but I have followed the exact wording of each source.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources indicate that J.M.H. Gool had a close relationship with Mahatma Gandhi, who would often stay at the Gools' residence in Cape Town. They were closely associated through their shared interests in social justice and community service (e.g., [9], [10]). Gandhi's son reportedly fell in love with one of the Gool sisters, a Muslim girl ([6], [20]), which further demonstrates their connection.

Additionally, J.M.H. Gool and his associates (including Dr. Abdulla Abdurahman and Mr. Wilson) shared a common goal of promoting education as a means to resist human rights assaults ([12]). Gandhi's close friendship with the Gools is evident from letters exchanged between Gandhi and Abdul Hamid Gool while he was studying in London ([19], [20]).

Their association can be seen through various sources, such as:

* Gandhi staying at 7 Buitencingle Street, the Gools' residence ([10])
* J.M.H. Gool submitting subscription dues to the Indian Opinion on behalf of Mr. Wilson, a friend and associate ([5], [17])
* The Gools being part of Gandhi's social circle, as evidenced by their shared interests in community service and social justice ([9])

Overall, it appears that the connection between Gandhi and J.M.H. Gool was built on mutual respect, trust, and shared values of social responsibility and community engagement.

### q28 — Which organisations was the author involved in?

The provided sources do not contain a complete list of all the organisations the author was involved in. However, some organisations mentioned that the author was associated with are:

1. The New Era Fellowship [Source: 1]
2. The Teachers League of South Africa [Source: 1]
3. The Non-European Unity Movement [Source: 1]
4. The Anti-Coloured Affairs Department (Anti-CAD) [Source: 18, 19]
5. The African People's Organisation (APO) [Source: 17]

Additionally, the author mentions affiliations with sports clubs, such as the Kismet Cricket Club and District Six Cricket Club [Source: 13], but does not explicitly state that they were directly involved in these organisations.

The author also attended conferences of the All African Convention in Bloemfontein, where they represented the New Era Fellowship [Source: 19].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA), representing Coloured teachers, was closely affiliated with the Non-European Unity Movement (NEUM) [1]. The teachers of the TLSA were among the leading voices in the NEUM's non-European unity campaign. They adopted the same boycott and non-collaboration principles as the NEUM, particularly deploying the boycott as their primary political weapon, most notably the boycott of the Coloured Advisory Council (C.A.C.) in 1943 [1].

The relationship between TLSA and NEUM was one of affiliation, with the former being part of the latter's campaign.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], Haji Joosub Maulvi Hamid Gool arrived at the Cape via Mauritius in 1884, establishing J.M.H. Gool & Co.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

According to the text, the Hanaffi Quwatul Islam Mosque was a mosque located on Loop Street in Cape Town, South Africa. It was an outpost of Islam in an area that was becoming increasingly European, Jewish, and gentile. The mosque was completed in 1898, and the author's grandfather was one of its founders and a life trustee.

The mosque was described as a whitewashed structure with a domed minaret topped by a spire featuring a moon and star. The muezzin would call the faithful to prayer from this minaret, but not too stridently so as not to annoy the European residents.

The author also mentioned that his family used to go to the mosque to celebrate Eid festivals, which were celebrated with great fanfare, including giving pennies to all the children who came to give their Eid greetings.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], Haji Joosub Maulvi Hamid Gool's grandson, Yousuf (Joe) Rassool states that: "Cissie Abdurahman was a daughter-in-law of the Gool family."

Additionally, in excerpt [4] it is mentioned that Dr. A. H. Gool married Cissie or Zainunnisa, who was "daughter of Dr. Abdulla Abdurahman, scion of Cape Town's other leading dynasty." This indicates that Cissie Gool (née Abdurahman) was the wife of Dr. A.H. Gool, who is also mentioned as a son of J.M.H. Gool in [9].

Therefore, Cissie Gool was the daughter-in-law of J.M.H. Gool through her marriage to his son Dr. A.H. Gool.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to source [1], Haji Joosub Maulvi Hamid Gool's acquaintances at Buitencingle included:

* Solly Joel
* Cecil Rhodes
* Mahatma Gandhi (documented in the Indian Opinion)
* The Prince of Wales
* George Bernard Shaw
* Sarojini Naidu
* Dr. Abdulla Abdurahman

Source [4] also mentions that J.M.H. Gool's letter to MK Gandhi is preserved, which suggests a personal relationship or correspondence between them.

Additionally, source [5] notes that Cissie Gool was among those who "loved" Jack London, but it does not establish a direct connection between J.M.H. Gool and Jack London.

There is no information in the provided sources about whether J.M.H. Gool knew Joseph Chamberlain personally or if he had any notable acquaintances beyond those mentioned above.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was a law passed by the South African government in 1950 that aimed to forcibly relocate people from certain areas, called "white group areas," to other designated areas based on their racial classification. The act was a key component of the Apartheid policy and was used to separate different racial groups into distinct geographic areas.

District Six, a multicultural community in Cape Town, was one of the areas affected by the Group Areas Act. According to [1], District Six was a vibrant jazz community that was home to people from various backgrounds, including Coloured, Malay, African, and others. However, under the Group Areas Act, non-White residents were forcibly removed from District Six and relocated to the Cape Flats, an area on the outskirts of Cape Town.

The effects of the Group Areas Act on District Six were devastating. [1] states that "the forced removals destroyed one of Cape Town's most vibrant communities." The act resulted in the demolition of homes and the bulldozing of the land, leaving only mosques and churches standing. Many residents were forcibly removed from their homes, and their livelihoods were disrupted.

[6] provides a poignant description of the decline of District Six: "I saw District Six begin to die in the early fifties as shops in Hanover Street, the main artery of the area, started closing... All that vibrant community scattered and disintegrated irrevocably pulverised by Apartheid."

The Group Areas Act had a profound impact on the lives of residents in District Six. [7] notes that the act resulted in poverty and inequality, as people were forced to move to sub-economic areas with poor living conditions.

Overall, the Group Areas Act was a key component of the Apartheid policy, and its effects on District Six were catastrophic, leading to the destruction of a vibrant community and the displacement of its residents.

### q35 — Who was Hassen Mall?

Hassen Mall is mentioned in excerpts [2], [4], [6], [7], [11], and [16] of the provided sources.

According to these excerpts:

* He was introduced as a slim, handsome young man with green eyes and neatly slicked-back hair (Excerpt [2]).
* He was described as radiating a kind of magnetism and being a brilliant cricketer who spun a wristy leg break that zipped like a circular saw (Excerpt [2]).
* He had completed his LLB degree and left for Durban in 1951, leaving behind an indelible imprint on the lives of those he knew (Excerpt [3]).
* He was captain of a cricket team in Durban and also mentioned as being involved in other activities such as playing whist with friends (Excerpts [16], [17] and [18]).
 

No further information is given about his background, family or other personal details.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources do not contain a comprehensive list of all political organizations active in the Cape Coloured community. However, they mention the following:

1. The New Era Fellowship: A cultural organization that issued a clarion call to Non-European organizations to counter the predicted assault on the remaining rights of the "Coloured" community (Source [2]).
2. The African People's Organisation (APO): Mentioned as an organisation that responded to the call for unity and full democratic rights (Source [15]).
3. The Coloured branch of Smuts's United Party: Also mentioned as responding to the call for unity (Source [15]).
4. The Anti-Coloured Affairs Council (Anti-CAC) / Anti-Coloured Affairs Department (Anti-CAD): Established to oppose the Coloured council, a body that was meant to advise the government on the needs of the Coloured community (Source [16]).
5. The Workers' Party: A front organisation of the Communist Party that had a candidate, Bertie Louw (Source [16]).
6. The Communist Party of South Africa: Mentioned as one of the organisations that responded to the call for unity (Source [15]).
7. The Fourth International of South Africa: Also mentioned as responding to the call for unity (Source [15]).

Other organisations mentioned include:

1. The Coloured People's Vigilance Association (Source [5] and [17])
2. The Unity Movement (Mentioned in Source [3], although not specified which organisation is referred to)
3. The Liberation League (Mentioned as responding to the call for unity, Source [15])
4. The Teachers' League of South Africa (Mentioned as siding with the opposition to the CAC and CAC-men, Source [16])

It's worth noting that this is not an exhaustive list, and other organisations may have been active in the community during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was an Indian leader who came to South Africa in 1893 [1]. He was a key figure in the Indian independence movement through non-violent civil resistance.

Gandhi's first assignment in South Africa was as a lawyer from Dada Abdullah, where he assisted with a civil suit against a merchant in Pretoria due to his fluency in Gujarati, his mother tongue [18].

He later became involved in the rights of Indians in South Africa through passive resistance and non-violent satyagraha.

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdurahman [19]. He met and married Helen "Nellie" Potter James while studying at Glasgow University, where he qualified as a doctor in 1893 [19]. 

The provided sources do not contain further information about Dr. Abdurahman's family or Cissie Gool's ancestry beyond this point.

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources describe District Six as a vibrant and multicultural neighborhood in Cape Town, South Africa, before it was bulldozed out of existence. Here's a synthesis of what they say:

[1] Describes District Six as "a multicultural community in Cape Town, nestled below Devil's Peak, celebrated for jazz and community spirit." It notes that the area was home to diverse residents: Coloured, Malay, African, and others.

[2] Recalls the author's memories of growing up in District Six at the age of eleven. He describes it as "inimitable" and mentions the birth of Apartheid, World War II, and other historical events that took place during his childhood.

[3] Describes the neighborhood as having a rich history of political struggle against the injustices of Apartheid.

[4] Notes that District Six was a thriving cosmopolitan area in the heart of Cape Town before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats, decimating the once-vibrant community and way of life. It mentions that the neighborhood had a "rich tapestry of lives" and a history of political struggle.

[5] Describes District Six as a community defined by its resistance against Apartheid, with residents engaging in various forms of activism and protest.

While these sources do not provide an exhaustive description of pre-removals District Six, they suggest that it was:

* Multicultural: Home to people of diverse backgrounds, including Coloured, Malay, African, and others.
* Vibrant: Celebrated for its jazz scene and community spirit.
* Politically active: Residents were engaged in various forms of resistance against Apartheid.

However, the sources do not provide information on other aspects of life in pre-removals District Six, such as economic conditions, social dynamics, or daily life.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy involved refusing to cooperate with apartheid institutions, including the Coloured Affairs Council (CAC). When people accepted service on the CAC, they were boycotted in every aspect of their lives, socially, economically, and otherwise [13]. 

In addition, when it came to voting, the Movement instructed its supporters that if "Coloured" voters insisted on voting, they should spoil their ballot by writing: "For Full Democratic Rights" across the ballot slip [20].


  ✅ Overall: 82.7% recall (token-overlap)  (186.0/225)  avg 4140ms
