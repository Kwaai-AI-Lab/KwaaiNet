[2m2026-07-02T15:11:56.837270Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T15:11:56.961195Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  21111ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T15:12:18.337423Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  22458ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T15:12:43.605167Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  25765ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T15:13:08.584533Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  22758ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T15:13:31.401372Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  30611ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T15:14:02.476733Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  28313ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T15:14:30.827112Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  22109ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T15:14:52.091922Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23364ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T15:15:16.655491Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/9 keywords  24338ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T15:15:41.153085Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/7 keywords  28504ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T15:16:09.895298Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  26439ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T15:16:36.463074Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  30141ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T15:17:07.012026Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  22939ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T15:17:30.012371Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  22141ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T15:17:52.462781Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 89 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six apartheid era forced evictions and relocation history""
  ○ Round 3   added 18 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  25191ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T15:18:16.241456Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  24167ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T15:18:40.612091Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  20193ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T15:19:02.972809Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23528ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T15:19:25.362759Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24421ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T15:19:50.949428Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  27183ms
  [21/40] Who was the author's mother?
[2m2026-07-02T15:20:18.409507Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  20611ms
  [22/40] Who was the author's father?
[2m2026-07-02T15:20:39.084696Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  20908ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T15:21:00.212846Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23492ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T15:21:24.357392Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  29437ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T15:21:53.732723Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27501ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T15:22:20.949698Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  35042ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T15:22:56.458396Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  29157ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T15:23:25.308563Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  22316ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T15:23:47.808691Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24934ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T15:24:13.125387Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  24759ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T15:24:37.909095Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  28236ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T15:25:06.369674Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  29332ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T15:25:35.791256Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  26548ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T15:26:02.549159Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  28052ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T15:26:31.207628Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  26465ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T15:26:57.616234Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  25684ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T15:27:23.499711Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  23081ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T15:27:46.724838Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  26159ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T15:28:12.464855Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  24218ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T15:28:37.583510Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  22926ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 88.0% (184.0/209) |
| Avg latency | 25363ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 21111ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 22458ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 25765ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22758ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 30611ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 28313ms |
| q07 | Who is the author's wife? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 22109ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 23364ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 24338ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 28504ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 26439ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 30141ms |
| q13 | What was the All Africa Convention? | 4/5 (80%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 22939ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 22141ms |
| q15 | What were the forced removals from District Six? | 4/5 (80%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 25191ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 2/5 (40%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 24167ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 20193ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 23528ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 24421ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 27183ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 20611ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 20908ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23492ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29437ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 27501ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 35042ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 29157ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 22316ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 24934ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 6/6 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 24759ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 28236ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 29332ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 26548ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 28052ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 26465ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 25684ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 23081ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 26159ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 24218ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 22926ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources list the following as the author's children:

1. Feyruz
2. Reza
3. Zarina 

These names can be found in excerpt [4], which lists the dedication of the book to the author's children and grandchildren.

Additionally, the source lists the following as the author's grandchildren:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

### q03 — Who are the author's grandchildren?

According to [1], the author has grandsons named Leonardo Rassool and Joseph Rassool, and grandsons named Alexander Rassool and Samuel Rassool. Additionally, according to the Dedication in [2] and [20], the author also has grandchildren named Dylan and Lewis (the latter is likely a grandson as well).

Therefore, the author's grandchildren are: 

* Leonardo Rassool
* Joseph Rassool
* Alexander Rassool
* Samuel Rassool
* Dylan Rassool
* Lewis Rassool

### q04 — To whom is the book dedicated?

The book is dedicated to the author's children: Feyruz, Reza, and Zarina; and his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [4]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool is described as a spice merchant and community leader who arrived in Cape Town from Mauritius in 1884 with family origins in Swat via Gujarat, India. He became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six and lived at 7 Buitencingle Street where notable visitors such as Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman visited him.

He had two wives: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool) and Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool).

He performed the Hajj in 1899 and died in April 1940. He was the grandfather of Yousuf (Joe) Rassool, the author of "LEST WE FORGET" [1]

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion in District Six, Cape Town, South Africa. It was the residence of J.M.H. Gool, the grandfather of the memoir's author. The street on which the mansion stood, also called Buitencingle Street or No. 7 Buitenkingle Street, was wide and cobbled with traffic islands sporting stunted palm trees.

The mansion had a view of Table Mountain and Devil's Peak, and its umpteen rooms would have been unmanageable without the help of servants. The author's grandmother tried to learn Afrikaans to communicate with the servants and interpret the insults of her rival (Source [6]).

Buitencingle was at one time the outermost belt of Cape Town, beyond which lay the foothills of Table Mountain reached via Kloof Street (Source [4]). A bus terminus for a single-decker trackless tram was located outside Fletchers & Cartwrights department store in Adderley Street.

The mansion was eventually shut up and left to dust and rodents, but during its heyday, it must have been vibrant with élan, grandeur, and joie-de-vivre (Source [12]).

Several family members lived at Buitencingle, including the author's grandmother and her husband, who had two wives. The grandfather was a prosperous merchant living in Buitencingle Street and came from India to Cape Town in 1884 via Mauritius (Source [17]).

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1]. She is an academic researcher who played a central role in shaping the memoir by urging the author to ground his personal memories in historical research, which transformed the book from a purely subjective recollection into a historically documented account of District Six.

### q08 — Tell me more about the author's wife.

Based on the provided sources, here are the facts known about the author's wife:

1. [1] The author's wife is named Nazima Rassool. She is an academic researcher and Professor (referenced as "Professor Nazima Rassool").
2. [4] According to the book, Nazima urged her husband to ground his personal memories in historical research to make the story more authentic.
3. [5] In this excerpt, it seems that there might be a different wife mentioned, but without further information or context from other excerpts, it is not clear if she is directly related to the author's life or another character's narrative within the book. However, since we know Nazima Rassool is the author's wife from Excerpt [1], this second mention likely refers to a different person.
4. [16] The text mentions that "My wife and four sons would often peer over my shoulder" during the writing process, implying that the author has a supportive family.

These are the specific facts mentioned in the sources about the author's wife.

### q09 — Who was the author's grandfather?

The provided sources do not explicitly state the full name of the author's grandfather. However, based on excerpt [1], we can infer that Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) was the author's grandfather.

Further confirmation is found in excerpt [9] where it says "He died... and leaving behind his grandchild Yousuf (Joe) Rassool, who wrote this memoir."

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural mountain pass in Cape Town, South Africa. According to [1], it's located between Table Mountain and Lion's Head, offering views of both mountains. The area is a landmark of Cape Town's topography.

Excerpt [3] mentions that Kloof Nek was the connecting saddle of land that joined Table Mountain to Lion’s Head. It was also described as a place of "haunting beauty" in excerpt [4].

Kloof Nek Road, which runs through this area, is mentioned in excerpts [6] and [8]. Additionally, excerpts [1], [2], [7], and [17] provide more information about the location.

Excerpt [19] mentions that a schoolmate took the author to Kloof Nek when he was 14 years old. The author recalled being footsore on a hot October day but being entranced by its beauty once they descended into the woodland dell.

Overall, Kloof Nek is described as a picturesque and historically significant location in Cape Town, with connections to both Table Mountain and Lion's Head.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers League of South Africa (TLSA) was a professional and political organization for Coloured teachers [1]. It was affiliated with the Non-European Unity Movement [1] and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1].

The TLSA held conferences that were important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense [2]. These events also served as social occasions where teachers could network and socialize [2].

The organization was initially dominated by an old guard of leaders who were eventually ousted by a new generation of young Turks, including Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, and Frank Landsman [5].

### q12 — Who was Cissie Gool?

Cissie Gool (1897-1963) was a Cape Town city councillor, lawyer, and political activist from South Africa. According to [1], she was the daughter of Dr. Abdulla Abdurahman, a pioneering physician and politician. She was one of the most prominent Coloured women in South African political life and was closely associated with the Non-European Unity Movement and the anti-apartheid struggle.

From the excerpts, we can gather the following specific facts about Cissie Gool:

* Born: 1897-02-14 [1]
* Died: 1963-12-02 [1]
* Nationality: South African [1]
* Occupation: lawyer and city councillor
* Affiliation: Non-European Unity Movement
* Member of the Unity Movement, whose stand on Whites-only elections was "A plague on both your houses" [3]
* Involved in various campaigns against social, economic, and political injustice [8]
* Served tea to guests at her Mount Street residence in the 1930s [6]
* Addressed a meeting at the Banqueting Hall in support of measures to deal with 'influx control' and street gangs [14]

### q13 — What was the All Africa Convention?

According to source [1], the All African Convention (AAC) was a political organization founded in 1935. Its purpose was to unite a broad coalition of African political organizations against racial oppression. The AAC later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

Source [2] also mentions that Vic Wessels, Les Jacobs, and Joe Rassool (the author) represented the New Era Fellowship at the conference of the All African Convention in Bloemfontein in 1954.

### q14 — Where was District Six and what kind of place was it?

District Six is described as being located near the centre of Cape Town, situated below Devil's Peak [1]. It was a vibrant, multicultural community with Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life. Under the Group Areas Act, the apartheid government declared it a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building.

It is also mentioned as being near Buitencingle, which was at that time the outermost belt of Cape Town [4].

### q15 — What were the forced removals from District Six?

The provided sources do not contain detailed information about the specific circumstances or dates of the forced removals from District Six. However, it is mentioned that under the Group Areas Act, the apartheid government declared District Six a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building [1].

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi, a prominent figure in Indian independence movement, visited the Gool family's home in Cape Town, South Africa. He stayed there in February-March 1914 (according to [17]). The Gool family provided him with hospitality during his stay.

In [9], it is mentioned that Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion and other scholarly journals. This implies a close relationship between the two families, with Dr. Gool offering medical services to Gandhi.

Gandhi's connections to the Gool family also include his attendance at their home in 1911, where he helped furbish Dr. Gool's surgery by puttying and staining the floor (according to [14]). This shows that Gandhi was a frequent visitor to the Gool household and had a close relationship with its members.

Gandhi also corresponded with my uncle Abdul Hamid Gool while he was a medical student at Guy’s Hospital in London (according to [18]). This correspondence further emphasizes the close connection between the two families.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained teachers under the apartheid-era segregated education system. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

It opened for its first intake of students in 1944 with considerable fanfare [3] and was initially located on grounds that were once a Whites-only Primary school, but were deemed too close to the prison and District Six [2-3].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization that emerged in 1943 [1] to unite African, Coloured, and Indian people against apartheid and colonialism. Its program was based on the Ten Point plan for equal rights [7]. The NEUM adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

The organization's founders were referred to as "the young Turks" [8], among whom Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, were moving spirits. The NEUM was known for its doctrine of non-collaboration and the boycott weapon [8].

During its activities, the NEUM played a significant role in the struggle against apartheid, with events such as conferences taking place after New Year festivities [11]. However, despite its efforts, the ideals of the 10 Point Programme would not be achieved as expected [7].

### q20 — Describe the author's involvement in cricket.

The author, Joe Rassool, was actively involved in cricket through Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in the Cape. As a cricketer, he had notable innings at Rosmead, but unfortunately, no selector saw it, which might have influenced their selection of players for the biennial tournament [9], [18].

In addition to playing, Rassool was involved in organizing cricket matches and unions. He tried to organize a union to facilitate the playing of cricket at High School and College level but was thwarted by the entrenched bureaucracy of the Schools' Sports Union [11]. 

Rassool's club, Kismets, had a non-racial policy that transformed the Indian Cricket Union, allowing teams like the Seventh Day Adventists' Heathrow Cricket Club to join the Sunday League [16].

He also participated in cricket-related activities outside his club. For instance, he was captaining the side that was selected for the biennial tournament and was excited about it [12]. Rassool's friend Hassen Mall captained the team, which included other notable players like Hamid Khan, Jameel and Omar Abrahams, Abou Desai, and Tiny Abed [19].

In general, cricket played a significant role in Rassool's life, allowing him to build friendships (e.g., with Muddy Allie) and giving him opportunities for personal growth.

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool [1]. She was also known as Lallie or Lallie Gool/Ayesha Gool.

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool [1]. He is also referred to as Peerbhai or Peru by his mother and children.

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool (mentioned in [20])
7. Yasmin Rassool

Note that the author's name is Yusuf Rassool, and his sibling Fazil is mentioned in [20].

### q24 — Who were the children of J.M.H. Gool?

According to [1], the children of J.M.H. Gool and his first wife Bibi Gool are:

1. Ayesha Rassool
2. Mohamed Saaid Gool
3. Zohra Abdurahman
4. Ahmed
5. Goolam Gool
6. Jane Gool-Tabata
7. Minnie Gool
8. Fatima Gool

According to [10], J.M.H. Gool also had children with his second wife Wahida Gool:

1. Abdul Hamid Gool
2. Mariam (Mariam Margie Gool)
3. Zobeida Gool
4. Mohammed Hanief Gool

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool [1]. According to the sources:

* Tabata wrote The Awakening of a People, which recorded the coming into existence of the All African Convention, one of the pillars of the Non-European Unity Movement [3].
* Tabata was accused by a political study group called The Forum Club (regarded as Marxist Trotskyists) of failing to have a class analysis in his historical tract and being a Black Nationalist, which was considered a serious accusation at the time [4].
* Tabata led the faction that supported buying and selling land, which was seen by some as an aspirant middle-class desire for a place in the Bourgeois sun [17] [18].
* Tabata completed his book The Awakening of a People shortly after becoming chairman of the New Era Fellowship [18].

The sources also mention him attending meetings, giving talks, and being involved in various events, but they do not provide a detailed biography.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the sources:

Dr. Abdulla Abdurahman (also known as Abdullah Abdurahman) [1] was a Cape Town doctor, city councillor, and leader of the African Political (later People's) Organisation (APO). He was the first Coloured person elected to the Cape Town City Council.

Source [2] mentions him among notable figures who stayed at Buitencingle when visiting South Africa. Source [3] states that he and his family were central to the history of District Six and of the Indians and Coloured people of South Africa.

In source [4], it is mentioned that Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community. Source [5] elaborates on this, stating that Dr. Abdurahman's residence was in Loop Street, close to Buitencingle, which facilitated co-operation between Grandpa Abdurahman and Dr. Abdurahman.

Source [6] mentions J.M.H. Gool's association with Dr. Abdurahman as his father-in-law (Dr. Abdurahman being Cissie Gool's father).

In source [7], it is mentioned that Messrs Oosterwyk, Keraan, and Abdurahman were involved in making meaningful contact with people.

Source [16] mentions Dr. Abdurahman speaking at a farewell address to Gandhi on behalf of Port Elizabeth Indians.

The list below combines all specific facts stated about Dr. Abdullah Abdurahman from the sources:

1. Born: 1872-09-08 (source [1])
2. Died: 1940-02-20 (source [1])
3. A Cape Town doctor, city councillor, and leader of the African Political (later People's) Organisation (APO) (source [1])
4. First Coloured person elected to the Cape Town City Council (source [1])
5. His grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community (sources [4] and [5])
6. Dr. Abdurahman's residence was in Loop Street, close to Buitencingle (sources [4] and [5])
7. Involved in making meaningful contact with people (source [7])

The provided sources do not contain information on his occupation or achievements outside of the mentioned roles and events.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources state that Gandhi had a close relationship with the Gool family, particularly with Dr. Abdul Hamid Gool (the son of J.M.H. Gool). They were close friends, as evidenced by their correspondence while Dr. Abdul Hamid Gool was a medical student at Guy's Hospital in London [3] and again when Gandhi stayed with them at 7 Buitencingle Street in Cape Town [6].

Specifically:

* Dr. J.M.H. Gool was one of the joint secretaries of the "Cape British Indian Union", which Gandhi helped form by bringing together rival Indian organisations [5].
* Gandhi helped to furnish Dr. Abdul Hamid's surgery at 7 Buitencingle Street in Cape Town in 1911 [5].
* The Gool family hosted Gandhi on several occasions, including when he stayed with them in February 1914 [6].
* Gandhi and J.M.H. Gool exchanged letters, as seen in the letter written by Gandhi to Chaganlal in March 1906, where he mentions that J.M.H. Gool's fortunes had declined due to troop reductions and anti-Indian legislation [7].

The connection between Gandhi and J.M.H. Gool can be described as one of friendship, mutual respect, and collaboration on Indian community issues in South Africa during the early 20th century.

### q28 — Which organisations was the author involved in?

The provided sources do not contain a complete list of all organisations the author was involved in. However, based on the excerpts, the following organisations are mentioned as being associated with the author:

1. New Era Fellowship (NEF) [3, 16]
2. Non-European Unity Movement (NEUM) [1, 17]
3. Teachers League of South Africa (TLSA) [1, 11]
4. Anti-CAD [8]
5. Cape Peninisula Students Union [19]

Additionally, the author mentions attending a meeting of school representatives at Hewat to discuss establishing a Students' Union [19].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement [1]. They shared a boycott and non-collaboration programme [1].

According to Source [12], "It was to this movement that I was drawn when I began my teaching career and where I threw my energies in the attempt to ‘take a nation to school’, an aphorism that aptly captured the role of the Movement during those years."

Source [13] states: "The conference of the Unity Movement took place after the New Year festivities. A number of delegates had come down from Natal and Transvaal, and Auntie Minnie’s home was the focal point of the meeting." 

From these sources we can conclude that TLSA was associated with Non-European Unity Movement but no direct quote indicates a strong formal relationship, but [12] indicates that the author "threw his energies" into the movement.

In Source [19], the TLSA broke away from its stance on non-political professionalism and joined the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1] Haji Joosub Maulvi Hamid Gool, he arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque or Quwatul Islam Mosque (Excerpt [1]), is a mosque located on Loop Street in Cape Town. It was founded in 1898 (Excerpt [1]) and is central to the history of the Gool family and the Cape Muslim community in District Six. 

The mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile (Excerpt [20]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was the wife of Abdul Hamid Gool, who is listed as one of the children of J.M.H. Gool [1]. Therefore, Cissie Gool was J.M.H. Gool's daughter-in-law.

Additionally, Cissie Gool's mother was Nellie Abdurahman (also known as Helen "Nellie" Potter James), who is mentioned in excerpt [17] and listed as being married to Dr. Abdulla Abdurahman [18]. This makes Dr. Abdurahman a brother-in-law of J.M.H. Gool.

So, Cissie Gool was both the wife of Abdul Hamid Gool (son of J.M.H. Gool) and also the daughter of Nellie Abdurahman (sister of Dr. Abdulla Abdurahman), making her a member of the Abdurahman family which had close ties to J.M.H. Gool's family.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources state that J.M.H. Gool knew Mahatma Gandhi personally, as indicated by Gandhi's visits to Gool at his home in District Six (excerpts [1] and [7]). 

Gandhi was a publisher of Indian Opinion and a champion of satyagraha, and he visited Gool during his farewell visit to South Africa before returning to India. Other notable visitors to Gool's home included Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman (excerpt [1]).

Furthermore, excerpt [7] lists many other individuals who are mentioned in connection with J.M.H. Gool or his family, but it does not explicitly indicate that he personally knew all of them.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was legislation introduced by the apartheid government in South Africa, which declared that certain areas would be designated for specific racial groups. The act allowed for the forced removals of people from their homes to make way for these new group areas.

According to the sources [1] and [13], District Six was a vibrant, multicultural community near the centre of Cape Town that was home to Coloured, African, Indian, and White residents. However, under the Group Areas Act, the apartheid government declared it a White area in 1966, leading to the forced removals of all non-White residents to the Cape Flats.

The sources [2] and [14] state that the Group Areas Board's valuation of properties was generally below market value, resulting in Coloured owners being obliged to accept paltry offers from prospective White purchasers. Furthermore, half of any profit made by selling a property had to go to the Group Area Boards (Source [14]).

The sources [5], [8], and [18] mention that the Non-White group most aggrieved by the Group areas Act was the South African Indians, who were largely concentrated in Natal but also resided in Cape Town. The Indian population had achieved considerable wealth despite being a minority, which led to envy among some Whites (Source [17]).

The sources [16] and [18] discuss the limitations of the boycott as a means of struggle against Apartheid, citing the Train Apartheid Resistance as an example. Some members of the "Movement" ignored the injunction to boycott, instead appearing before or sending representatives to the Group Areas Boards to register their protest (Source [16]).

### q35 — Who was Hassen Mall?

Based on the sources, here are the facts about Hassen Mall:

* He was a close friend of the author Yousuf Rassool [1].
* He was from Durban and stayed with Muddy's family in Cape Town while pursuing his studies at the University of Cape Town [3, 4].
* He was a cricketer who captained a team that included the author [2, 6-8, 10, 17-18].
* He had completed an LLB degree and left for Durban after completing it [2, 16].
* He was known for his exceptional cricket skills, including spinning a wristy leg break that zipped like a circular saw [6].
* He later studied law and qualified with a BA LLb degree [6].
* He was the chairman of the Athlone Branch of the TLSA (Trafalgar Players) and faced a major problem regarding the selection of their team for the biennial tournament in Port Elizabeth, which he eventually resolved by organizing a collection list to raise funds [12, 15, 18].
* He was also involved in the Indian Congress movement [20].

Note that these facts are based on the specific sources provided and may not be an exhaustive list of Hassen Mall's achievements or activities.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were:

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA)
3. The New Era Fellowship (NEF)
4. The All African Convention (AAC)

These organisations were united by their boycott and non-collaboration policy against apartheid institutions, with the NEUM serving as the umbrella body [1].

Additionally, other organisations that were mentioned as being involved in the Cape Coloured community's politics include:

* The Communist Party of South Africa
* The Fourth International of South Africa
* The Liberation League
* The Workers' Party
* The Coloured branch of Smuts's United Party

These organisations often worked together to challenge apartheid policies and advocate for democratic rights [15].

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was a lawyer from India [1]. He came to South Africa in 1893 as a lawyer to work on a legal case [1].

The provided sources do not contain a comprehensive list of Gandhi's activities or the reasons behind his extended stay in South Africa. However, they mention that he developed his philosophy of satyagraha (non-violent resistance) while living and working in South Africa for over 20 years [1].

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information about who Cissie Gool's mother is. However, it is mentioned in [1] that Cissie Gool's daughter is Shaheen Gool and her husband is Abdul Hamid Gool.

It is also mentioned in [6] that Abdurahman was the first of the community to qualify as a doctor from Glasgow University in 1893. There he met and married Helen “Nellie” Potter James. However, there is no mention of whether Abdurahman is Cissie Gool's father or not.

But it is mentioned in [1] that Dr. Abdulla Abdurahman was Cissie Gool's father.

[1] Cissie Gool... Daughter of: Dr. Abdulla Abdurahman.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to excerpt [1], District Six was "a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life."

Excerpt [3] describes it as a place with "more life, more zest, more fire, more warmth and care than many other places one could mention" where children would play in the streets. The author recalls walking to school every morning along Caledon Street, which was getting "tattier" along the way.

Excerpt [4] provides a brief historical note: District Six was a thriving cosmopolitan area before the Group Areas Act (1950) dispersed its people across the Cape Flats, decimating the once vibrant community and way of life. 

Excerpt [1] also mentions that District Six was known for its political struggle against apartheid.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in several excerpts:

[1] The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

[3] They declared that no one could guarantee a hundred percent boycott. Inevitably someone would be elected; therefore, it was better to have “our man” inside promoting “our point of view” than have the seat occupied by a government stooge.

[11] All those who accepted service on the C.A.C. were boycotted in every aspect of their lives, socially, economically and otherwise.

This suggests that the Unity Movement's boycott policy was to refuse participation in apartheid institutions, such as the Coloured Advisory Council (C.A.C.), and to boycott individuals who took part in these institutions or supported them.


  ✅ Overall: 88.0% recall (token-overlap)  (184.0/209)  avg 25363ms
