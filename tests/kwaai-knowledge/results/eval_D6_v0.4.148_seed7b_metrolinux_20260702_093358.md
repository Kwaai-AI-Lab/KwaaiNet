[2m2026-07-02T16:34:00.111059Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:52235 (via ollama-proxy)
[2m2026-07-02T16:34:05.801798Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:52235
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T16:34:05.936324Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  8630ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T16:34:14.811686Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  2721ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T16:34:17.695811Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  2907ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T16:34:20.823614Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  2627ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T16:34:23.661615Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  6078ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T16:34:29.942771Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  5105ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T16:34:35.256016Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  2073ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T16:34:37.520371Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  4329ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T16:34:42.062495Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/9 keywords  5055ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T16:34:47.312313Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  4343ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T16:34:51.859022Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  4216ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T16:34:56.280471Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  5559ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T16:35:02.035736Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2791ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T16:35:05.037518Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  2825ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T16:35:08.062930Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 88 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six apartheid era forced evictions history""
  ○ Round 3   added 20 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  9595ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T16:35:17.861492Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  5220ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T16:35:23.288377Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  2216ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T16:35:25.705566Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  2758ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T16:35:28.668480Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  4812ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T16:35:33.680458Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  5221ms
  [21/40] Who was the author's mother?
[2m2026-07-02T16:35:39.107812Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  5767ms
  [22/40] Who was the author's father?
[2m2026-07-02T16:35:45.073531Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  2251ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T16:35:47.531515Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3587ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T16:35:51.324991Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  3968ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T16:35:55.487218Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  5719ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T16:36:01.476464Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4493ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T16:36:06.111022Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  5375ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T16:36:11.695626Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  3815ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T16:36:15.704906Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  2708ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T16:36:18.624619Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  2546ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T16:36:21.361560Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  3358ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T16:36:24.926403Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3445ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T16:36:28.577222Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  5638ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T16:36:34.412909Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  5683ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T16:36:40.293721Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  4014ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T16:36:44.542538Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  3511ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T16:36:48.226822Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  3152ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T16:36:51.594748Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  2326ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T16:36:54.117843Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4763ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T16:36:59.083790Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  3006ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 89.5% (187.0/209) |
| Avg latency | 4205ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 8630ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 2721ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 2907ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 2627ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 6078ms |
| q06 | Tell me about Buitencingle. | 8/8 (100%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 5105ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 2073ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 4329ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5055ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 4343ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 4216ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 5559ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 2791ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 2825ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 9595ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 5220ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 2216ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 2758ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4812ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 5221ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 5767ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 2251ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3587ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 3968ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 5719ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 4493ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5375ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 3815ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 2708ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 2546ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 3358ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 3445ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5638ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 5683ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 4014ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 3511ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 3152ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 2326ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 4763ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 3006ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [9].

### q02 — Who are the author's children?

The provided sources list the following as the author's (Yousuf "Joe" Rassool) children:

1. Feyruz
2. Reza
3. Zarina
4. Joseph
5. Samuel
6. Alexander
7. Leonardo
8. Dylan
9. Lewis

### q03 — Who are the author's grandchildren?

The provided sources list the following as the author's grandchildren:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool

These names appear in the dedication of the book, which states that they are the grandchildren of the author (Y.S. Rassool). [1]

### q04 — To whom is the book dedicated?

The book "District Six - Lest We Forget" by Yousuf (Joe) Rassool is dedicated to:

* Joe's children: Feyruz, Reza, and Zarina
* And his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [17].

### q05 — Who was J.M.H. Gool?

J.M.H. Gool was a prominent Indian community leader and spice merchant who lived in Cape Town during the late 19th and early 20th centuries. He was married to two women, Bibi (whom he married when she was 11 years old) and Wahida Ta'Al (a member of the Dollie clan), and had many children with them.

Gool was a well-respected citizen in Cape Town and was involved in various community activities, including the establishment of the Hanaffi Quwatul Islam Mosque. He also played an important role in the Indian community's struggle for civic rights and equality.

Gool's son, Abdul Hamid Gool, married Cissie Gool (the daughter of Dr. Abdulla Abdurahman), making her his daughter-in-law. J.M.H. Gool was a grandfather to Yousuf Rassool, the author of "LEST WE FORGET" memoir.

Some specific facts about J.M.H. Gool include:

* He arrived in Cape Town from Mauritius in 1884 (Source: [1])
* He was a founder trustee of the Hanaffi Quwatul Islam Mosque in District Six (Source: [1])
* He lived at 7 Buitencingle Street, where he received notable visitors such as Mahatma Gandhi and Cecil John Rhodes (Source: [1])
* He had two wives and many children with them (Sources: [1], [3])
* He performed the Hajj in 1899 (Source: [1])
* He died in April 1940 (Source: [1])

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion located at No. 7 Buitencingle Street in District Six, Cape Town. It belonged to J.M.H. Gool, the grandfather of the author [1]. The street was wide and cobbled with traffic islands sporting stunted palm trees [18].

Buitencingle's origin is Dutch, with "cingle" meaning belt [19]. At that time, Buitencingle was the outermost belt of Cape Town [19]. 

The mansion had a view of Table Mountain and Devil's Peak. The author mentions that when they lived there, they had more space than they could cope with, plus a garden with fruit trees [5].

There were multiple rooms in the mansion, which would have been unmanageable without the help of servants. The author's grandmother tried to learn Afrikaans not only to interpret insults from her rival but also to communicate with the servants [6]. 

The Gool family was known for their grandeur and joie-de-vivre, as evident from their lifestyle at Buitencingle during the turn of the century [12].

In 1905, the Buitencingle mansion was home to a thriving trading community. The author's grandfather, J.M.H. Gool, had an advert in Indian Opinion on February 25, 1905 [16].

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

Here is a list of specific facts mentioned in the sources about the author's wife, Professor Nazima Rassool:

1. Her name is Nazima Rassool (source [1]).
2. She is married to Yousuf Rassool, the author (source [1], and multiple other sources that mention "the author" or "Yousuf Rassool").
3. She is an academic researcher (sources [4] and [19]).
4. She urged the author to ground his writing in background investigation to lend authenticity to the story (sources [4] and [19]).
5. The author owes a great deal to her, as she was instrumental in transforming the book from a purely subjective recollection into a historically documented account of District Six (source [1]).
6. She is mentioned by multiple authors in their prefaces or editor's notes, which suggests that she has been involved in the publication and editing process of the book.

There is no information about her ancestry, family connections, occupations, achievements, or historical associations beyond what is mentioned above.

### q09 — Who was the author's grandfather?

The provided sources state the following facts about the author's grandfather:

* His name is Joosub Maulvi Hamid Gool (J.M.H. Gool) [1]
* He was a spice merchant and community leader
* He arrived in Cape Town from Mauritius in 1884, with family origins in Swat via Gujarat, India [1]
* He lived at 7 Buitencingle Street, District Six, Cape Town [1]
* He received notable visitors, including Mahatma Gandhi during his farewell visit to South Africa before returning to India in 1914 [1]
* He was married twice:
	+ First wife: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool)
	+ Second wife: Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool) [1]
* He was the grandfather of the author, Yousuf (Joe) Rassool [1]

The complete origin chain for J.M.H. Gool is:

* Mauritius (via Swat, Gujarat, India)

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It sprawls protectively on the eastern flank of the city with its head gazing inscrutably down on Table Bay [4]. The Glen, located near Kloof Nek, was described as a place of haunting beauty [18].

Kloof Nek was the connecting saddle of land that joined Table Mountain to Lion's Head [2, 3, 4, 17]. It was also referred to as the Glen [17, 18]. The area is a landmark of Cape Town's topography, offering views of both Table Mountain and Lion's Head [1].

The Glen near Kloof Nek was frequented by lovers, mountaineers, nature lovers, and ramblers. A growing number of down-and-outs known as "Bergies" found refuge in the area to drink away their sorrows [16].

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organization for Coloured teachers. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy to challenge racial discrimination in education and fight for equal opportunities for Coloured teachers and students. [1, 2]

The TLSA played an important role in the struggle for democracy and equal rights for non-Whites in South Africa. It was a key organization in the forefront of the movement, and its members, including Alie Fataar, Edgar Maurice, Jane Gool, and others, were considered leaders in the community. [5]

The TLSA's motto was "Let Us Live for Our Children," and it aimed to prepare teachers and pupils for the challenges they would face under apartheid. The organization also had a publication called the Education Journal, where members received training in political/educational composition and had their articles published. [8]

### q12 — Who was Cissie Gool?

Cissie Gool (full name Zainunnissa Gool) was a prominent South African anti-apartheid activist and politician. The sources list the following facts about her:

* She was born on February 14, 1897 [1].
* Her father was Dr. Abdulla Abdurahman, a physician and city councillor in Cape Town [2].
* Cissie Gool was a member of the Non-European Unity Movement [1] and later became a leader in the anti-apartheid struggle.
* She was a lawyer and a Cape Town city councillor [1].
* Her husband was Abdul Hamid Gool, whom she married after being divorced from another man, Toetie (also spelled "Toeti" or "Totee"), who was her adopted brother's widow [2-3, 10].
* Cissie Gool was described as a brave and strong woman fighting against social, economic, and political injustice [8].
* She was known to have been involved in the Unity Movement, but later changed her stance on issues like influx control and supported measures that were previously opposed by the Anti-CAD (Coloured Affairs Department) Movement [16-17, 19].
* Cissie Gool called a massive demonstration of the Liberation League on the Grand Parade in 1937 to rally against residential segregation [18].

The provided sources do not contain comprehensive information about her entire life or other aspects of her biography.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was a broad coalition of African political organisations that sought unity against racial oppression. It was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights [1]. 

No further information is provided about the AAC's goals, structure, or notable achievements beyond its founding and alignment with the Non-European Unity Movement [1].

### q14 — Where was District Six and what kind of place was it?

According to the sources [1], District Six was a district in Cape Town, located below Devil's Peak. It was known for its vibrant, multicultural community, where Coloured, African, Indian, and White residents lived together and built a rich urban culture of music, jazz, and community life.

Source 1: "District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak."

### q15 — What were the forced removals from District Six?

According to source [1], the apartheid government declared District Six a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building. This event destroyed a thriving community.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi, also known as Mahatma Gandhi, was a leader in the Indian independence movement. He spent over 20 years in South Africa pioneering satyagraha (non-violent resistance). According to [1] and [3], Gandhi had a close relationship with the Gool family of District Six, Cape Town, where he stayed at their home, Buitencingle Street, several times.

Specific connections mentioned between Gandhi and the Gool family include:

* Gandhi's uncle, Abdul Hamid Gool, was in correspondence with him while Gandhi was studying medicine in London ([3]).
* In 1911, Gandhi helped furbish Dr. Gool's surgery by puttying and staining the floor ([8] and [9]).

Gandhi also interacted with other family members, including:

* In March 1914, Gandhi attended a function at Buitencingle Street in his honor, where he thanked Mr. Gool for their services to his wife during her illness (Indian Opinion, March 1914, cited in [4]).
* Gandhi visited Buitencingle yet again in February-March 1914 (Gandhi's stay at the Gool mansion) ([4] and [19]).

The Gool family played a central role in early Cape Muslim and Indian political life, connecting District Six to national and international anti-colonial struggles.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students. It trained Coloured teachers under the apartheid-era segregated education system [1].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organisation that united African, Coloured, and Indian people against apartheid and colonialism in South Africa. It was founded in 1943 [1] and adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions.

According to the sources, the NEUM's programme was based on the Ten Point plan for equal rights, which included demands such as full citizenship, equal education, and an end to segregation [7]. The organisation aimed to unite the non-white population of South Africa under a common banner, promoting a non-racial society and challenging the apartheid regime.

The NEUM was affiliated with the All African Convention and had ties to other anti-apartheid organisations. It played a significant role in the broader anti-apartheid struggle, as evident from its influence on subsequent movements and events, such as the Kliptown conference of the African National Congress, which issued the Freedom Charter [10].

The NEUM's impact was felt across various aspects of South African society, including education. The Teachers' League of South Africa (TLSA) was affiliated with the NEUM and shared its boycott and non-collaboration programme [1].

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool, was actively involved in cricket through Kismets Cricket Club and the Western Province Indian Cricket Union. He played and administered club cricket in Cape Town.

He mentions that he was still an "absolute rabbit" (a beginner) at cricket when his sister's friend, Stanley Abrahams, introduced him to the sport (Source: [14]). However, with the guidance of Hassen Mall, he began to learn and master the elements of batting, particularly the forward defensive stroke.

Rassool also mentions that he was part of a team, "Kismet Cricket Club", which had transformed the Indian Cricket Union through its non-racial policy (Source: [5]). The club attracted several non-Indian teams and brought in new talent, including James Africa and "Woody" Woodman from the Heathrow Cricket Club.

Rassool's cricketing career included participating in the biennial cricket tournament, where he was not initially selected but later joined the team after a four-year wait (Source: [18]). He also mentions that he had made an oh-so-glorious 12 runs and was hailed as a hero in one of his games (Source: [9]).

In addition to playing, Rassool also tried to organize a union to facilitate the playing of cricket at High School and College level but was thwarted by bureaucracy (Source: [11]).

### q21 — Who was the author's mother?

The provided sources state that:

* The author's mother was named Ayesha Rassool [1].
* She lived in District Six and was known as Lallie or Ayesha Gool [1].
* She had a roman nose, fair skin, dark wavy shoulder-length hair that curled over her broad forehead [3].
* She wanted to marry for love, rejecting the proposal of Sir Sayed Raza Ali, the Agent-General for India [3].

Biographical information from all sources:

Dates: The author's mother, Ayesha Rassool (Lallie or Ayesha Gool), was born on December 10th, 1900. 

Places: She lived in District Six.

She had a daughter named Rasheda Rassool, four sons named Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf (Joe) Rassool, Zain Rassool, Fazil Rassool, and Yasmin (Jessie) Rassool [1].

Family connections: Her parents were Bibi Gool and Haji Joosub Maulvi Hamid Gool. She had a half-sibling named Abdul Hamid Gool, Jane Gool-Tabata, Goolam Gool, Mohammed Hanief Gool [1]. 

She was married to Peter Alexander Rassool (Peerbhai) [17].

Other facts: Her family's home language was English, and she did not disabuse the author of their delusion that speaking English made them Europeans and not Coloureds, despite their dark complexion [19].

### q22 — Who was the author's father?

The author's (Yousuf Rassool) father was Peter Alexander Rassool, also known as Peerbhai or Peru [1].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool (also spelled as "Bibi" in [9])
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool
8. Lallie (the author's mother, but mentioned here because she is a sibling in the context of other children) 
9. Reza Rassool (author's brother)
10. Zarina Rassool (author's sister)
11. Feyruz Rassool (author's sister)

### q24 — Who were the children of J.M.H. Gool?

According to the sources [1] and [9], J.M.H. Gool had multiple wives and numerous children. From his first wife, Bibi Gool, he had:

* Abdul Hamid Gool
* Jane Gool-Tabata
* Mariam Margie Gool
* Minnie Gool
* Zobeida Gool
* Goolam Gool
* Fatima Gool
* Mariam Gool
* Mohamed Saaid Gool

From his second wife, Wahida Gool (also known as Ta’al), he had:

* Ayesha Rassool
* Mohammed Hanief Gool
* Fatima Timmie Gool
* Zohra Abdurahman

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool [1]. 

Specifically, sources mention that he:

* Was a member of the All African Convention and the Non-European Unity Movement [1]
* Completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship [3], [18]
* Was involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1]
* Was accused of being a Black Nationalist, which was seen as a form of political denigration at the time [4]
* Had differences with Ben Kies, which ultimately led to a split in the Movement [20]

Overall, I.B. Tabata was an important figure in South African politics during the apartheid era, and his work and legacy are significant for understanding the history of the Non-European Unity Movement and the anti-apartheid struggle.

List of facts about I.B. Tabata:

* [1] He was a member of the All African Convention and the Non-European Unity Movement.
* [3] He completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship.
* [4] He was accused of being a Black Nationalist, which was seen as a form of political denigration at the time.
* [20] He had differences with Ben Kies, which ultimately led to a split in the Movement.

References:
[1]
[3]
[4]
[20]

### q26 — Who was Dr. Abdullah Abdurahman?

According to the provided sources:

* Dr. Abdullah Abdurahman (also known as Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdulla Abdurahman) was a Cape Town physician and city councillor [1].
* He was the first Coloured person elected to the Cape Town City Council as a councillor [1].
* He was a leader of the African Political (later People's) Organisation (APO) [1].
* His grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [4].
* He was associated with Haji Joosub Maulvi Hamid Gool, Non-European Unity Movement [1].
* He had two children: Cissie Gool [1] and Nellie Abdurahman [9].
* He lived on Loop Street, which is a mere walk away from Buitencingle [4, 5].

These are the facts explicitly mentioned in the provided sources.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to the sources [1], [3], [9], [10], [12], [13], [14], and [15], the connection between Gandhi and J.M.H. Gool was a close one, with numerous visits, correspondence, and collaborations:

* Gandhi stayed at 7 Buitencingle Street, the home of J.M.H. Gool, on multiple occasions (sources [3] and [10]).
* The two men exchanged letters, with Gool writing to Gandhi in 1897 (source [9]) and Gandhi responding.
* Gool's family provided hospitality to Gandhis, including when they visited South Africa in 1914 (source [10]).
* Gandhi was a guest at the Gool residence in 1911, where he helped puttying and staining the floor of Dr. Abdul Hamid Gool's surgery (source [15]).
* The Gools were close friends and allies of Gandhi, who often sought their advice and support.
* Gandhi referred to J.M.H. Gool as a friend and supporter in his letters and writings.

The connection between Gandhi and J.M.H. Gool was likely due to their shared interests in social activism, community building, and Indian rights in South Africa. The two men worked together on various projects and campaigns, including the struggle for Indian citizenship and civic rights (source [14]).

### q28 — Which organisations was the author involved in?

Based on the provided sources, the author (Yousuf Rassool) was involved in the following organisations:

1. New Era Fellowship (NEF) [3], [15]
2. Teachers League of South Africa (TLSA) [11]
3. Non-European Unity Movement (NEUM) is mentioned as an umbrella body for various organisations, but it's not clear if the author was directly involved in it [1]

Additionally, the author mentions being part of a sociological survey team from the Students' Representative Council (SRC), which suggests he might have been involved with the SRC, but this is not explicitly stated.

The author also mentions attending meetings and participating in discussions at Hewat Amateur Theatrical Society, Trafalgar Players, and the Coloured branch of Smuts's United Party [17].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM). They shared a boycott and non-collaboration programme [1]. The TLSA approached the Congress movement suggesting the formulation of a unified strategy to counter the anti-educational plans of the regime, but the Congress leadership declined the invitation to cooperate.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1] Haji Joosub Maulvi Hamid Gool arrived in Cape Town in 1884, from Mauritius.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is an old mosque located on Loop Street, Cape Town, South Africa. It was founded in [1] 1898 and is one of the oldest mosques in the Cape Muslim community. According to [20], the author's grandfather was a founder and life trustee of the mosque. The mosque was completed in 1898, and the family used to go there to celebrate Eid festivals.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1] and [17], J.M.H. Gool had two wives: Bibi Gool (whom he married in 1879 or 1880) and Wahida Ta'al (also known as Hajima). Cissie Gool was the daughter of Dr. Abdulla Abdurahman, whom she later married. This makes her the daughter-in-law of J.M.H. Gool through her marriage to Abdul Hamid Gool, one of his sons.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to [1], Haji Joosub Maulvi Hamid Gool, received notable visitors including:

* Mahatma Gandhi: publisher of Indian Opinion and champion of satyagraha
* Cecil John Rhodes
* George Bernard Shaw
* Dr. Abdulla Abdurahman

Additionally, [13] states that J.M.H. Gool's uncle, Dr. Gool, was involved in politics with Gandhi, who helped bring together two rival organizations as "The Cape British Indian Union", and later became president of this body.

[11] mentions that the Gool family, including Dr. Abdurahman, were central to the history of District Six and the Indians and Coloured people of South Africa.

It is also mentioned in [12] that Gandhi helped furbish J.M.H. Gool's uncle's surgery by puttying and staining the floor during his visit to 7 Buitencingle Street, Cape Town in 1911.

Therefore, notable historical figures known personally by J.M.H. Gool include:

* Mahatma Gandhi
* Cecil John Rhodes (although no specific details are provided)
* George Bernard Shaw (although no specific details are provided)
* Dr. Abdulla Abdurahman

Note: The list of notable historical figures who knew J.M.H. Gool personally may not be exhaustive, as there might be other individuals mentioned in the sources that are not explicitly stated to have had a personal relationship with him.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources state:

[1] District Six was known for its political struggle against apartheid, and the Group Areas Act devastated the lives of hundreds of thousands of unfranchised people in South Africa [3].
 
[8] This obscene social experiment, that would devastate the lives of hundreds of thousands of the unfranchised of South Africa, occurred.  stillborn death on page 3, column 5 of the Cape Argus.
 
[13] By 1950, the Juggernaut of Apartheid was confidently grinding onwards, ploughing under the few remaining rights of the non-White oppressed. The crowning piece of the Government legislation was a law that would transform the country into a patchwork of Black “Coloured”, Indian and White areas. The Group Areas Act, was the lynch pin, the Big Bertha of the Apartheid weaponry.
 
[9] ...a law that would transform the country into a patchwork of Black “Coloured”, Indian and White areas.  The Group Areas Act, was the lynch pin, the Big Bertha
61  of the Apartheid weaponry.

The Group Areas Act was a key piece of apartheid legislation passed in 1950 [13], which aimed to forcibly segregate people along racial lines by allocating specific areas for different racial groups. This act had a devastating impact on District Six, a vibrant and multicultural community in Cape Town [1]. The act led to the forced removals of non-White residents from District Six to the Cape Flats, bulldozing and demolishing almost every building [1].

### q35 — Who was Hassen Mall?

Hassen Mall was:

- A close friend and fellow cricket player from District Six (source [1])
- A student who came to Cape Town to study medicine at the University of Cape Town (sources [4], [6], and [16])
- The team captain for a biennial cricket tournament in Durban in early January, having completed his LLB degree (sources [10] and [17])
- Introduced table tennis to his friends and taught them how to play (source [14])
- A brilliant cricketer who bowled a wristy leg break that zipped like a circular saw (source [6])
- The chairman of the Athlone Branch of the TLSA (source [10])
- Secretary of the Western Province Indian Cricket Union (source [10])

He was known for his leadership, cricketing skills, and personality, leaving an indelible imprint on those around him.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were:

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA), which organised Coloured teachers
3. The New Era Fellowship (NEF), a discussion and lecture forum
4. The All African Convention (AAC), a broader African political body that joined forces with the Unity Movement [1].

These organisations were united by their boycott and non-collaboration policy against apartheid institutions, and the NEUM was the umbrella body for these efforts [1].

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer to work on a legal case [1]. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance) as a tool of political struggle. 

Gandhi had been sent by Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria because the accounts were kept in Gujarati, which was Gandhi's mother tongue [6].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources describe District Six as a vibrant and multicultural community. According to [1], it was "a thriving cosmopolitan area in the heart of Cape Town" with residents from various backgrounds, including Coloured, African, Indian, and White people. The neighborhood was known for its rich urban culture, music, jazz, and community life ([2]).

In terms of architecture, District Six had a mix of different housing types, but [3] notes that the area had "more life, more zest, more fire, more warmth and care" than many other places. The neighborhood was described as having "bare bulldozed acres" after the forced removals ([3]).

[4] provides historical context on the impact of apartheid policies, stating that the Group Areas Act (1950) dispersed people across the Cape Flats, decimating the once-vibrant community and way of life. The area was described as having a "rich history of politi[c]" ([4]), indicating a sense of resistance and activism within the community.

Overall, the sources suggest that District Six was a diverse and lively neighborhood with a strong sense of community before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy involved refusing to participate in or cooperate with apartheid government institutions, including the Coloured Advisory Council. They also called for people to refuse to register their protest against the Group Areas Act by appearing before the board or sending legal representatives, instead encouraging them to spoil their ballot by writing "For Full Democratic Rights" across it if they insisted on voting (source [18]). The policy was based on the principle of non-collaboration and the use of the boycott as a weapon.


  ✅ Overall: 89.5% recall (token-overlap)  (187.0/209)  avg 4205ms
