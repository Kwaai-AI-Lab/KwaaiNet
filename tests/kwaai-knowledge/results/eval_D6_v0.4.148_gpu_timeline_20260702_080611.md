[2m2026-07-02T14:58:35.696422Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:62763 (via ollama-proxy)
[2m2026-07-02T14:58:37.835183Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:62763
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T14:58:37.958714Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  6511ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T14:58:44.746365Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  2924ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T14:58:47.886129Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2633ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T14:58:50.714792Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  2281ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T14:58:53.205347Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  5329ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T14:58:58.735000Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  4525ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T14:59:03.475968Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  2446ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T14:59:06.124667Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4028ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T14:59:10.349619Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/9 keywords  3691ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T14:59:14.247286Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  5935ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T14:59:20.386777Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/4 keywords  4553ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T14:59:25.141408Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4438ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T14:59:29.783528Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  4151ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T14:59:34.140219Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  2816ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T14:59:37.156530Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 87 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced removals apartheid South Africa history""
  ○ Round 3   added 22 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  11161ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T14:59:48.521572Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  5303ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T14:59:54.079755Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3223ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T14:59:57.453883Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  2776ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T15:00:00.441424Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  4130ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T15:00:04.769145Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  5544ms
  [21/40] Who was the author's mother?
[2m2026-07-02T15:00:10.516742Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2690ms
  [22/40] Who was the author's father?
[2m2026-07-02T15:00:13.408069Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  2054ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T15:00:15.668893Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3600ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T15:00:19.933501Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  5140ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T15:00:25.241054Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  7834ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T15:00:33.766955Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  8536ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T15:00:42.483542Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  7567ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T15:00:50.966862Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  9262ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T15:01:00.222836Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  5571ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T15:01:05.383524Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  5504ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T15:01:10.838020Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  4481ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T15:01:15.683926Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  4739ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T15:01:20.933608Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  5090ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T15:01:25.627691Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  5524ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T15:01:31.361076Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  5402ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T15:01:36.953702Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  4470ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T15:01:41.708623Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  3217ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T15:01:45.050741Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/5 keywords  2213ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T15:01:47.461316Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4102ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T15:01:51.775178Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  3983ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 81.3% (170.0/209) |
| Avg latency | 4734ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 6511ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 2924ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 2633ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 2281ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 5329ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 4525ms |
| q07 | Who is the author's wife? | 2/3 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 2446ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 4028ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 3691ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 5935ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/4 (75%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 4553ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 4438ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 4151ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 2816ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | sequence_diagram:District Six, LEST WE FORGET -rev25.pdf, [Graph: District Six] | 11161ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 5303ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 3223ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 2776ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 4130ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 5544ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 2690ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 2054ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3600ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 5140ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 7834ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 4/6 (67%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 8536ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 2/5 (40%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 7567ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 9262ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 5571ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf, sequence_diagram:Haji Joosub Maulvi Hamid Gool | 5504ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 4481ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 4739ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 5090ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 5524ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 5402ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 4470ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 3217ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 2213ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 4102ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 3983ms |

## Answers

### q01 — Who is the author?

The provided sources do not explicitly state the name "Yousuf (Joe) Rassool" as the author. However, [1] mentions "Y.S. Rassool is the author of the book and has children and grandchildren mentioned in the dedication." 

Since there are multiple mentions of a person named Yousuf (Joe) Rassool/Y.S. Rassool throughout the sources, it can be inferred that he is likely the author referred to in [1].

### q02 — Who are the author's children?

The provided sources do not contain a comprehensive list of the author's children, but they mention some of them:

* Feyruz
* Reza
* Zarina
* Joseph (grandchild)
* Samuel (grandchild)
* Alexander (grandchild)
* Leonardo (grandchild)
* Dylan (grandchild)
* Lewis (grandchild)

However, the sources do not provide information on any additional children beyond these.

### q03 — Who are the author's grandchildren?

According to the sources [1] and [2], the author's (Yousuf Rassool/Joe Rassool) grandchildren are:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [4]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool, also known as Joosub Maulvi Hamid Gool, was a spice merchant and community leader in Cape Town, South Africa. 

According to [1], he arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.

[5] describes him as a person of striking good looks, tall and proud, who carried himself well. His father was a Pushto-speaking Maulvi (religious leader) from the Province of Swat in India.

As [10] states, he became a rich man after his business took off, and he moved to more spacious premises in Castle Street with rooms upstairs.

[12] mentions that he hosted Gokhale at his residence "Noor Bagh" in 1912. 

He was married twice: once to Bibi Gool, who had children including Abdul Hamid Gool (who later became a prominent figure), and then to Wahida Ta'al. He also had connections with Adam Haji G. Mahomed, who was his senior, financier, and possibly mentor.

As [16] describes, he wrote medical articles for the Indian Opinion in 1912, on topics such as Modern Footwear and Tuberculosis.

He died in April 1940, according to [1].

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion in District Six, Cape Town. It was built by J.M.H. Gool, grandfather of the author [1]. The name "Buitencingle" is Dutch in origin, with "cingle" meaning belt, and Buitencingle being at that time the outermost belt of Cape Town [3].

The mansion stood on a wide street with cobbled roads and traffic islands sporting stunted palm trees. A German Lutheran Church was located opposite it [4]. The house had a view of Table Mountain and Devil's Peak.

Buitencingle had umpteen rooms, which would have been unmanageable without the help of servants [6]. The author remembers spending time in the garden with fruit trees and playing in the streets around Buitencingle. He also recalls using the books in the house for drawing and making paper planes [11].

The mansion's personality was described as vibrant with élan, grandeur, and joie-de-vivre when all was going well, but when it was shut up and left to dust and rodents, its spirit died [12].

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1]. She is an academic researcher who encouraged the author to ground his writing in background investigation, which transformed the memoir from a subjective recollection into a historically documented account.

### q08 — Tell me more about the author's wife.

The provided sources contain the following information about the author's wife:

1. The author's wife is named Nazima Rassool [1].
2. She is an academic researcher [4, 15, 19].
3. She played a central role in shaping the memoir by urging the author to ground his personal memories in historical research [1].
4. She insisted that the author should not rely merely on his own memories, but rather conduct background investigation to lend authenticity to the story [4].
5. She is quoted as saying "The result, I submit, produced some staggering information" regarding the outcome of their joint research efforts [15].
6. The author owes a great deal to her for urging him to engage in this background investigation and for providing her expertise as an academic researcher [19].

It's worth noting that the sources provide limited biographical information about Nazima Rassool beyond her role as the author's wife and her professional background as an academic researcher.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Joosub Maulvi Hamid Gool or Grandpa. 

 Specific facts about J.M.H. Gool include:
- He arrived in Cape Town from Mauritius in 1884.
- His family origins were in Swat via Gujarat, India.
- He was a spice merchant and community leader.
- He lived at 7 Buitencingle Street, District Six, Cape Town.
- He had two wives: Bibi Gool and Wahida Gool.
- He performed the Hajj in 1899.
- He died on April 1, 1940, following a final stroke.

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It connects Table Mountain to Lion's Head, offering views of both mountains [4]. The area is described as a place of haunting beauty [4], [18] with its own unique character and attractions. Kloof Nek was used as a meeting point by Persotem Patel and Khadija, who would meet there after visiting the South African Public Library in the Gardens on Saturdays [3].

Kloof Nek was also an important location for transportation, with a bus terminus at the site, connecting it to other parts of Cape Town. The area had various attractions, including the Table Mountain cable car, which began its run near Kloof Nek [16]. It was a popular spot for nature lovers and locals alike, but also served as a refuge for down-and-outs who would drink away their sorrows in the nearby cavelets and bushes [16].

The name "Kloof Nek" is derived from the Afrikaans language, with "kloof" meaning a valley or gorge, and "nek" meaning a neck or pass [19]. The correct pronunciation of the name is Baytencingle, with the "Ay" sound made rounder by pursing the lips [7], [19].

In terms of geography, Kloof Nek lies beyond Buitencingle Street in District Six, which sits below Table Mountain [1]. It was also described as being connected to other locations, including Adderley Street and Fletchers & Cartwrights department store [19].

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement, and its goal was to challenge racial discrimination in education and fight for equal opportunities for Coloured teachers and students [1]. The TLSA had conferences that were civic occasions opened by the mayor, where teachers could share ideas on inoculating their pupils against impending government onslaughts on education [4].

The organisation's motto was "Let Us Live for Our Children," and its members aimed to prepare for an eventuality in which the government would target education, following what they had done to African people. Members of TLSA were involved in various activities, including developing strategies with other associations like the Cape African Teachers' Association [4].

Notable members included Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, and Solly Edross, among others. These individuals were seen as leaders in their community due to their profession, which is why they played a significant role in making TLSA political [5].

### q12 — Who was Cissie Gool?

Cissie Gool (1897-1963) was a South African politician, lawyer, and activist. She was a daughter of Dr. Abdulla Abdurahman, a pioneering Cape Town city councillor, physician, and political leader of the Cape Coloured community. Cissie was one of the most prominent Coloured women in South African political life and was closely associated with the Non-European Unity Movement and the anti-apartheid struggle.

She was a member of the Unity Movement and later became involved in municipal politics, serving as a city councillor in Cape Town. Cissie Gool also attended meetings of the Coloured People's Vigilant Committee and the British Indian League. She was known for her passionate speeches and leadership skills, which inspired many people to join the anti-apartheid movement.

Cissie married Abdul Hamid Gool and had a son named Shaheen. Her family connections and involvement in politics made her a significant figure in South African history.

### q13 — What was the All Africa Convention?

[1] states that the All African Convention (AAC) was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organisations seeking unity against racial oppression. It later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

[2] mentions that Vic Wessels, Les Jacobs, and the author attended the All African Convention conference in Bloemfontein in 1954, but notes that the authorities had invoked a by-law to prevent the meeting from taking place, so arrangements were made to take them to Johannesburg instead.

[3] states that The Awakening of a People, written by I.B. Tabata and published as an NEF publication, recorded the coming into existence of the All African Convention, one of the pillars of the Non-European Unity Movement.

The information provided does not give further details about its goals or activities beyond these mentions.

### q14 — Where was District Six and what kind of place was it?

District Six was located near the center of Cape Town, situated below Devil's Peak [1]. It was a vibrant, multicultural community with a rich urban culture of music, jazz, and community life [3]. The area was home to Coloured, African, Indian, and White residents who lived together in a diverse community [1].

This information can be found in source [1] and supported by other sources.

### q15 — What were the forced removals from District Six?

The provided sources do not explicitly mention the forced removals from District Six. However, they do provide some context about the area and its inhabitants.

In excerpt [5], it is mentioned that "There was always a rank, fetid smell in the air, which became worse where a narrow lane separated two blocks of balconied flats." This suggests that the area was densely populated and had poor living conditions.

In excerpt [20] from 1972, it is mentioned that Verwoerd, Minister of Native Affairs, proclaimed: "Too long have the Bantu gazed on the green pastures where they may never graze.” This quote implies a policy of forced removals or segregation.

The sources do not explicitly mention the date of the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the sources:

* Gandhi was a notable figure who visited the Gool's home in District Six, Cape Town (Source [1]).
* He had a significant relationship with the Gool family, particularly Dr. Abdul Hamid Gool, Joe Rassool's uncle (Source [3], [18], and [20]).
* Gandhi stayed at 7 Buitencingle Street, the Gool mansion, during his visit to Cape Town in 1914 (Source [17] and [19]).
* He was a guest of Adam Gool, Dr. Abdul Hamid's brother, at Noor Bagh, their residence (Source [10]).
* The Gool family members, including Dr. Abdul Hamid and Yusuf Hamid Gool, were close to Gandhi and provided him with hospitality during his stay in Cape Town (Sources [4], [5], and [20]).

Gandhi's connection to the Gool family was that of a respected guest, friend, and possibly a mentor or advisor. He would often visit the Gools at their home, where he received warm welcomes and attention from the family members.

The sources do not provide detailed information about Gandhi's personal life or background before his arrival in Cape Town, but they suggest that he had significant connections with the Indian community and was respected for his social activism and contributions to the struggle against anti-Indian legislation.

### q17 — What was Hewat Training College?

According to [1], Hewat Training College was a teacher training college in Cape Town for Coloured students. It trained Coloured teachers under the apartheid-era segregated education system. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat.

Additionally, from [3] we learn that it was initially a Whites only Primary school, but it was deemed too close to the prison and District Six, so it was converted into a Post Matriculation Teacher Training facility for Coloureds in 1944.

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organisation that was founded in 1943 [1]. Its programme was based on the Ten Point plan for equal rights, and its aim was to unite African, Coloured, and Indian people against apartheid and colonialism [1].

According to various sources ([7], [8], [9]), the NEUM's ideology included:

* The principle of non-Collaboration
* The refutation of "race"
* The ideal of a non-racial society

The movement also used the weapon of boycott, including refusing to participate in any apartheid government institutions, such as the Coloured Advisory Council [1].

The NEUM was affiliated with the All African Convention and had leaders like Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam [8]. It played a significant role in politicizing non-political structures and bringing them into the movement ([12]).

### q20 — Describe the author's involvement in cricket.

Based on the provided sources:

* The author, Yousuf Rassool, was actively involved in cricket through Kismets Cricket Club and non-European cricket (Source [1]).
* He played cricket and administered club cricket in the Cape (Source [1]).
* The author mentions that he began his cricketing days when a particular player, who is not named, was at his peak, and they had several eighties against his name (Source [6]).
* Yousuf Rassool also discovered a brilliant leg spinner named Stanley Abrahams, who could bat, bowl, and field like a wizard (Source [5]).
* He mentions trying to organise a union to facilitate the playing of cricket at High School and College level but was thwarted by bureaucracy (Source [11]).
* The author expresses his joy when he heard that he had been selected for the biennial cricket tournament in Durban, but unfortunately, no selector had seen his innings at Rosmead (Source [9] and [18]).

However, the most notable aspect of the author's involvement in cricket is:

* He was part of a team that had the potential to win a match, but was thwarted by Wally Hendricks, who took a ball between the author's bat and pad, costing them a 10-run lead (Source [9]).
* The author also played against the West Indians with players like Worrel, Weekes, and Walcott, although he does not specify his role in this match (Sources [8] and [20]).

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool, also known as Lallie. She was the daughter of Bibi Gool and J.M.H. Gool (by his wife Bibi Gool) and grew up in the Gool household at 7 Buitencingle Street in District Six, Cape Town [1].

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool [1].

### q23 — Who were the author's siblings?

The provided sources mention the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool (also known as Jessie)
8. [No mention of Yousuf J. Rassool's other sibling.]

Note that Fazil was mentioned in source [20], but he is also listed in the initial sources [1] as a grandson, so it seems there might be some confusion between the two generations.

The author's siblings' ages and personalities are not specified in these excerpts.

### q24 — Who were the children of J.M.H. Gool?

The sources mention multiple wives of J.M.H. Gool:

1. Bibi Gool (married in 1879 or 1880):
   Children mentioned: 
   - Ayesha Rassool
   - Mohamed Saaid Gool
   - Zohra Abdurahman
   - Ahmed
   - Goolam Gool
   - Jane Gool-Tabata
   - Minnie
   - Fatima Gool

2. Wahida Gool (second wife):
   Children mentioned:
   - Abdul Hamid Gool
   - Mariam Margie Gool
   - Zobeida Gool
   - Mohammed Hanief Gool
   - Fatima Timmie Gool

### q25 — Who was I.B. Tabata?

I.B. Tabata (also known as B.M. Tabata) was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He was married to Jane Gool-Tabata, daughter of J.M.H. Gool [1]. Tabata was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].

He completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship [3], [19]. The book was an occasion of considerable excitement as it was to be an NEF publication [3], [19].

Tabata was accused by the Forum Club (a Marxist Trotskyist group) of failing to have a class analysis in his historical tract "The Awakening" and also being a Black Nationalist, one of the worst forms of political denigration at the time [4]. However, this did not diminish his influence and importance within the Unity Movement.

Tabata was present at gatherings where notable individuals like Ben Kies, Victor Wessels, Alie Fataar, Jane Gool-Tabata, I.B. Tabata, Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdulla Abdurahman, Leah Solomon, and Zaid Gamiet were also in attendance [12]. 

He was involved in discussions about the Labour Party and the Parliamentary road to Socialism [13].

At one point, Tabata rose from his seat to attack Kies at a Unity Movement conference, saying "It is an insult to say I have personal differences. My differences are not personal they are political!" [20].

### q26 — Who was Dr. Abdullah Abdurahman?

Here is a comprehensive list of facts about Dr. Abdullah Abdurahaman based on the provided sources:

From [1]:
* Born: 1872-09-08
* Died: 1940-02-20
* Associated with: Haji Joosub Maulvi Hamid Gool, Non-European Unity Movement

From [4]:
* Grandfather was brought to South Africa as a slave and bought his freedom. He became prominent in the community.
* Dr. Abdurahman's mother, Gadija Dollie, was a member of the Dollie clan.

From [5] (same information as [4]):
* Dr. Abdurahman's grandfather was brought to South Africa as a slave and bought his freedom. He became prominent in the community.
* Dr. Abdurahman's mother, Gadija Dollie, was a member of the Dollie clan.

From [6]:
* Eldest son took over ownership of Buitencingle upon returning from medical school in 1911.
* Second non-European to qualify as a physician from South Africa, studied at Guys Hospital, London. Married Cissie (Zainunnisa) Abdurahman, daughter of Dr. Abdulla Abdurahman.

From [17]:
* Present at meeting with Progressive Party of Dr. Jamieson on January 9th, 1904, along with Dr. Abdurahman.
* Asked about provision for higher education of coloured people; no response mentioned in The Indian Opinion.

From [18] and [19]:
* First doctor from the community to qualify at Glasgow University in 1893.
* Married Helen "Nellie" Potter James.
* Linked to the Ta'Als through the Dollie family. Dr. Abdurahman's mother, Gadija Dollie, was a beautiful woman.

Note: There are multiple sources that mention Dr. Abdullah Abdurahman but do not provide new information about him.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources do not fully explain the nature of the connection between Gandhi and J.M.H. Gool [1-20]. However, they mention several instances where Gandhi interacted with or corresponded with members of the Gool family.

From the excerpts:

* Gandhi visited J.M.H. Gool at his residence in 1911 and helped with minor repairs to Dr. Gool's surgery ([6]).
* Gandhi was a guest at the Gools' residence, likely multiple times, including in 1914 when he came to receive the Indian Viceroy's Legislative Assembly member G. H. Gokhale ([11]).
* Gandhi's son fell in love with one of Goulam Gool's sisters (a Moslem), and Gandhi refused to approve their marriage due to caste concerns ([3], [14]).
* The Gools provided hospitality to Gandhis in February 1914, which led to a friendship between the author Yousuf Rassool and Joe Rassool ([10], [16]).

It appears that Gandhi had a close relationship with the Gool family, particularly J.M.H. Gool, but the specific nature of their connection is not clearly stated in the provided sources.

### q28 — Which organisations was the author involved in?

The provided sources list the following organisations with which the author, Joe Rassool, was involved:

1. New Era Fellowship (NEF) [3] - a cultural organisation established in 1937
2. Non-European Unity Movement (NEUM) [1] - mentioned as an umbrella body of various organisations, including the NEF
3. Teachers League of South Africa (TLSA) [1] - organised Coloured teachers
4. All African Convention (AAC) [1] - a broader African political body that joined forces with the Unity Movement

Additionally, the author mentions being involved in other organisations and activities, such as:

* A Students' Union, which was considered but not established [19]
* The Cape Peninisula Students Union [19], although the author expresses misgivings about its development
* Anti-CAD (Anti-Coloured Affairs Department) movement [12]

The author also mentions his grandfather's involvement in various organisations and activities, including:

* Indian Congress [8]
* Liberation League [17]
* Workers' Party [17]
* Fourth International of South Africa [17]
* Communist Party of South Africa [17]
* Coloured branch of Smuts's United Party [17]

Please note that the author's involvement in these organisations is not always explicitly stated, and some information may be inferred from the context.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to [1], the Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM). They shared a boycott and non-collaboration programme. 

Additionally, [13] mentions that when Cissie Gool spoke in support of "influx control" and the street gangs problem, Mr. Kies (who was involved in the TLSA) reacted with shock.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information on when or from where J.M.H. Gool arrived in Cape Town.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was a mosque located in Loop Street, Cape Town. It was founded in 1898 [1] and was an important part of the history of the Gool family and the Cape Muslim community in District Six. The mosque was central to Islamic practice in the area, where Muslims would gather for Eid festivals and prayers [20].

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was one of the wives of J.M.H. Gool's son, Abdul Hamid Gool [9]. This makes her the daughter-in-law of J.M.H. Gool.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources list the following notable historical figures as people whom J.M.H. Gool knew personally:

* Mahatma Gandhi (excerpts [1], [10], and [11])
* Cecil John Rhodes (excerpt [1])
* George Bernard Shaw (excerpt [1])
* Dr. Abdulla Abdurahman (extracts [5] and [18])

These individuals are mentioned as having visited J.M.H. Gool or having interacted with him in some way, either as acquaintances or through shared public activities.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not explicitly state what the Group Areas Act was. However, based on excerpts [2], [3], [4], [7], and [9], it can be inferred that the Group Areas Act was a piece of legislation implemented by the South African government to segregate communities along racial lines.

According to excerpt [3], the Group Areas Act led to the displacement of residents from District Six, a thriving multi-racial community in Cape Town. The act resulted in the inhabitants losing their municipal franchise because they were living in sub-economic houses. This effectively disfranchised the residents.

Excerpt [4] mentions that the Non-White group most aggrieved by the Group Areas Act was the South African Indians. The Indian population, who had achieved considerable wealth despite the odds against them, were dispersed from District Six and other areas to make way for White communities.

In excerpt [7], Hassen Mall is quoted as saying that the Group Areas Act could not be fought in isolation; that the only hope lay in the struggle for full democratic rights. This suggests that the act was a part of a larger system of apartheid, which aimed to restrict the rights and freedoms of Non-White communities.

Excerpt [9] describes the Group Areas Act as "the lynch pin" of the Apartheid weaponry, implying its significance in the implementation of apartheid policies.

### q35 — Who was Hassen Mall?

Hassen Mall was a close friend and fellow cricket player of the author Yousuf Rassool. He was from Durban, staying with his family in Cape Town, where he came to study medicine at the University of Cape Town. Hassen Mall was a brilliant cricketer who played a wristy leg break that zipped like a circular saw and later switched to Law, discovering his true talent.

Some key facts about Hassen Mall are:

* He was from Durban.
* He stayed with his family in Cape Town while studying medicine at the University of Cape Town [4].
* He was a brilliant cricketer who played for the team that Yousuf Rassool was part of [6, 12].
* He batted number one and wanted Yousuf to bat number two [17].
* He helped collect funds for the tournament by walking from one end of Hanover Street to the other with Ebrahim, Muddy's brother [15, 18].
* He captained the team in the biennial cricket tournament at Durban's Curry's Fountain in January [16].
* His family was involved in The Indian Congress [20].

Sources:
[1], [2]–[5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15], [16], [17], [18], [19], and [20].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpt [1], the main political organisations active in the Cape Coloured community during the mid-twentieth century were:

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA)
3. The New Era Fellowship (NEF)
4. The All African Convention (AAC)

These organisations were united by their boycott and non-collaboration policy against apartheid institutions.

Additionally, excerpts [16] and [17] mention the following organisations that played a role in the Cape Coloured community:

1. The Communist Party of South Africa
2. The Fourth International of South Africa
3. The Liberation League
4. The Workers’ Party

Excerpt [15] also mentions the New Era Fellowship (NEF) and the Teachers' League of South Africa as part of the young Turks movement that emerged in response to the Coloured Advisory Council.

The provided sources do not provide an exhaustive list of all political organisations active in the Cape Coloured community during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to excerpt [1], Mohandas Karamchand Gandhi (also known as Mahatma Gandhi) came to South Africa in 1893 as a lawyer to work on a legal case. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance, or passive resistance) as a tool of political struggle.

The provided sources do not contain detailed information about Gandhi's background before he came to South Africa.

### q38 — Who was Cissie Gool's father?

Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the provided sources:

* District Six was a vibrant and multicultural community near the centre of Cape Town, known for its political struggle against apartheid [1].
* It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].
* The neighbourhood was "bare bulldozed acres" had more life, more zest, more fire, more warmth and care than many other places one could mention [3].
* Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way, but it buzzed with the joy of life [3].
* The area was described as having a rich history of political activism and not being defined by a culture of silence and acquiescence [4].

Overall, District Six was a dynamic and culturally diverse neighbourhood that played an important role in the fight against apartheid.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy was to refuse to participate in any apartheid government institutions or "dummy" institutions. This included refusing to serve on the Coloured Advisory Council (C.A.C.) and other similar bodies, as well as boycotting elections where the outcome was predetermined by the ruling party [11]. The movement also encouraged members to spoil their ballot papers if they were forced to vote in an election [18].

The boycott policy was seen as a key strategy for challenging apartheid policies and institutions, and was part of a broader programme-based struggle against apartheid [4]. However, not all members of the Unity Movement adhered to the boycott policy, with some individuals choosing to ignore it or participate in elections despite its opposition [8] and [15].

It's worth noting that the Unity Movement also used other tactics, such as ostracizing individuals who worked for the apartheid government, to further their goals [11].


  ✅ Overall: 81.3% recall (token-overlap)  (170.0/209)  avg 4734ms
