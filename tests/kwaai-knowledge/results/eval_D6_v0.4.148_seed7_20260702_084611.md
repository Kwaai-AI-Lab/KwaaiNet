[2m2026-07-02T15:46:20.635998Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T15:46:20.856907Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  52872ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T15:47:17.178401Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/3 keywords  27914ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T15:47:43.409634Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23730ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T15:48:07.395874Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  22707ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T15:48:30.463158Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  34135ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T15:49:04.896696Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  28534ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T15:49:33.405117Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  20787ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T15:49:54.321758Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  22608ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T15:50:17.180212Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/9 keywords  22097ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T15:50:39.455343Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  28662ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T15:51:08.438628Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  29547ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T15:51:38.130429Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  32643ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T15:52:11.172561Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23956ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T15:52:35.089321Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  20590ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T15:52:56.073723Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 89 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced relocation history South Africa apartheid era""
  ○ Round 3   added 19 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  49470ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T15:53:44.272380Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  31859ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T15:54:16.336436Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  18466ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T15:54:35.007781Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24456ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T15:55:01.520233Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23647ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T15:55:25.020747Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  27388ms
  [21/40] Who was the author's mother?
[2m2026-07-02T15:55:52.629022Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  21150ms
  [22/40] Who was the author's father?
[2m2026-07-02T15:56:14.060448Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  21880ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T15:56:35.965065Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23118ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T15:56:59.241270Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  31377ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T15:57:31.208706Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  33509ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T15:58:07.759349Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  29663ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T15:58:36.544617Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  31151ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T15:59:06.036522Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  23106ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T15:59:29.603235Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  21404ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T15:59:50.366267Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  25462ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T16:00:26.766604Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  39152ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T16:00:58.762568Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  30354ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T16:01:26.950426Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  33011ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T16:01:59.884122Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  27087ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T16:02:27.052882Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  25023ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T16:02:51.821426Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  27883ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T16:03:19.788143Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  25123ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T16:03:45.130129Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  21878ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T16:04:06.487860Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  24786ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T16:04:32.280215Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23792ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 86.1% (180.0/209) |
| Avg latency | 27649ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 52872ms |
| q02 | Who are the author's children? | 0/3 (0%) | LEST WE FORGET -rev25.pdf | 27914ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 23730ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22707ms |
| q05 | Who was J.M.H. Gool? | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 34135ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 28534ms |
| q07 | Who is the author's wife? | 2/3 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 20787ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 22608ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 22097ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 28662ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 29547ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 32643ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 23956ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 20590ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 49470ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 31859ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 18466ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 24456ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23647ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 27388ms |
| q21 | Who was the author's mother? | 4/5 (80%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 21150ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 21880ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 23118ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31377ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 33509ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 29663ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31151ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 23106ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 21404ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 25462ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 39152ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 30354ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 33011ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 27087ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 25023ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 27883ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 25123ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 21878ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 24786ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 23792ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1, 5].

### q02 — Who are the author's children?

The provided sources mention the following information about the author's children:

[9] The author's mother, Magdalena Isabella, bore six children to her husband Malick Rassool. However, [18] and [20] both provide a list of grandchildren with the names: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

The text does not explicitly mention the author's own children by name, only listing them as "my children" in the dedications in [8] and [18].

### q03 — Who are the author's grandchildren?

The provided sources list the following individuals as the author's grandchildren:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool 

These names appear in [1] and [20].

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [4]

### q05 — Who was J.M.H. Gool?

According to the sources:

J.M.H. Gool (Joosub Maulvi Hamid Gool) was a spice merchant and community leader who lived in District Six, Cape Town. He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.

He had two wives: Bibi Gool and Wahida Ta'Al, whom he married in 1879 or 1880 and 1900, respectively. He was a father of several children, including Abdul Hamid Gool, Jane Gool-Tabata, Mariam Margie Gool, Minnie Gool, Zobeida Gool, Goolam Gool, Fatima Gool, Mohamed Saaid Gool, Ayesha Rassool, and Mohammed Hanief Gool.

He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six. He also wrote two medical articles for the Indian Opinion, a newspaper edited by Mahatma Gandhi.

Gool's family hosted Gandhi at their residence in 1914, and he helped furbish my uncle’s surgery by puttying and staining the floor. Gool was involved in the politics of the Indians in Cape Town and became one of the joint secretaries of the "The Cape British Indian Union" after it was formed by Gandhi.

He died on April 1, 1940.

Sources: [1], [3], [4], [5], [9], [10], [11], [15], [16], [17], and [18]

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion located in District Six, Cape Town. It was the home of J.M.H. Gool, the grandfather of the memoir's author [1]. The street where it was situated, Buitencingle Street, was wide and cobbled with traffic islands sporting stunted palm trees [1]. The mansion had a view of Table Mountain and Devil's Peak [1].

Buitencingle was at the time the outermost belt of Cape Town, and its name is derived from the Dutch word "cingle," meaning belt or girdle [18]. The area beyond Buitencingle Street lay the foothills of Table Mountain, reachable via Kloof Street [18].

The mansion had a garden with fruit trees, providing ample space for the family to grow and thrive [5]. The author recalls spending time in the garden, climbing the trees, and making up fanciful stories about the pears [5]. Buitencingle was described as having a "vibrant" and "grandeur" quality to it, but its spirit eventually died when the house was shut up and left to decay [11].

The author's family spent time at Buitencingle, and his grandfather, Haji Joosub Maulvi Hamid Gooli, lived there with his Malay wife and children. The author's mother joked about his grandmother being unable to understand Afrikaans, but eventually she learned the language to communicate with the servants [13, 15].

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

Based on the provided sources, here is what we know about the author's wife:

1. Her name is Nazima Rassool [1].
2. She is a Professor and academic researcher [1].
3. She is married to Yousuf (Joe) Rassool, the author of "LEST WE FORGET - rev25.pdf" [1].
4. She played a central role in shaping the memoir by urging her husband to ground his personal memories in historical research [1].
5. She is an academic researcher who urged her husband to do background investigation and grounding in history, which transformed the book from a purely subjective recollection into a historically documented account of District Six [4] and [19].

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool (also known as Haji Joosub Maulvi Hamid Gool).

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It is located on the eastern flank of the city, with its head gazing down on Table Bay [17]. 

The area is described as a "hauntingly beautiful" place [18] [17], with the Glen being a significant feature. It was frequented by lovers, mountaineers, nature lovers, and ramblers, but also served as a refuge for those who were homeless or struggling with poverty [16].

Kloof Nek is situated on Kloof Street, which leads up to it from the city bowl [1]. Beyond Kloof Nek lies the Glen, which offers stunning views of Table Mountain and Lion's Head. The area can be accessed by walking through the gardens and up Kloof Street until reaching Kloof Nek, as described in one account [18].

The Du Toit’s Kloof pass was being constructed near Kloof Nek, which would shorten the distance from Cape Town to Worcester by about eleven miles and avoid a treacherous pass that ran past Clanwilliam [19].

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement, which adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1]. The TLSA aimed to promote full democratic rights for non-citizens in South Africa, particularly through education [6].

TLSA was known for being a political union, as it became the first teachers' union to take on a political role. This made it a target of the authorities, who saw it as an unacceptable situation for a teachers' union to become involved in politics [5]. Despite this, the TLSA continued its efforts to promote equality and democracy through education.

The organisation was led by young Turks, including Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O. Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, Frank Landsman, and others [5]. These leaders were committed to the cause of promoting democracy and equality for all people in South Africa.

The TLSA was also involved in various activities, including publishing articles in its Education Journal, where members received training in political/educational composition and had their works published. Some notable members included Danny De Beer, who held a degree from the University of South Africa, and Solly Edross, who was a skilled artist and educator [12].

Throughout its existence, the TLSA played an important role in promoting equality and democracy through education.

### q12 — Who was Cissie Gool?

Cissie Gool was a Cape Town city councillor, lawyer, and political activist. She was the daughter of Dr. Abdulla Abdurahman (also written Dr. Abdullah Abdurahman), a pioneering Cape Town city councillor, physician, and political leader of the Cape Coloured community.

She was one of the most prominent Coloured women in South African political life and was closely associated with the Non-European Unity Movement and the anti-apartheid struggle [1].

Cissie Gool was also married to Abdul Hamid Gool. 

She met luminaries in art, music, and politics such as Fred Bodmer, Gregoire Boonzaaier, Terence Macaw, Ben Kies, Paul Kostin, Wolf Kiebel, J.G., Dora Taylor, Jane Gool, I.B. Tabata, Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdulla Abdurahman, Leah Solomon, and Zaid Gamiet [4].

She submitted a subscription to the Indian Opinion on behalf of Mr. Wilson [9] and wrote that it is essential that the New South Africa accord the Indian Settlers their proper place in the historical record.

Cissie Gool was described as having been part of the platform at the first meeting at the Grand Parade, where her presence shocked the narrator [3].

She was also known to have called a massive demonstration of the Liberation League on the Grand Parade in 1937 to rally the vote-less masses against the government’s plan to introduce residential segregation [18].

The provided sources describe Cissie Gool as being involved in various political movements and activities, but there is no comprehensive list of her specific affiliations or achievements.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was a political organization founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organizations seeking unity against racial oppression and later aligned closely with the Non-European Unity Movement and its non-collaboration policy [1].

Some key facts about the All African Convention:

* Founded: 1935
* Response to: Hertzog Bills, which stripped Africans of their limited voting rights
* Goal: Unity among African political organizations against racial oppression
* Later aligned with: Non-European Unity Movement and its non-collaboration policy

The AAC is mentioned in several other excerpts as well, indicating its importance in the context of South African politics during this period.

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

The provided sources do not contain that information.

### q15 — What were the forced removals from District Six?

The provided sources do not contain detailed information about the forced removals from District Six. However, it can be inferred from [1] that the forced removals involved the bulldozing and demolishing of almost every building in the area, resulting in the destruction of a thriving community. Additionally, [16] mentions that the residents of District Six were relocated to the Cape Flats, which is mentioned as an "eyesore and health hazard" ironically named Windermere.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi (1869-1948) was a key figure in the Indian independence movement. He spent over 20 years in South Africa, where he developed and practiced his philosophy of non-violent resistance, known as satyagraha.

Gandhi's connection to the Gool family is significant. The Gools hosted him at their home, 7 Buitencingle Street, Cape Town, on several occasions, including when he was leaving South Africa in 1914 (Excerpt [1]). Gandhi also maintained a close relationship with the Gools, particularly Dr. Abdul Hamid Gool, who provided medical care to Mrs. Gandhi during her illness and was involved in various political activities with Gandhi.

Gandhi's diary mentions that he helped putty and stain the floor of Dr. Gool's surgery at 7 Buitencingle in 1911 (Excerpt [6]). Additionally, Gandhi attended functions hosted by the Gools, such as a farewell function for him in March 1914 (Excerpt [4]).

The correspondence between Gandhi and Abdul Hamid Gool is documented, with letters exchanged while Gandhi was a medical student at Guy's Hospital in London (Excerpt [3]). Gandhi also visited Dr. Gool's residence, "Noor Bagh," in Cape Town on several occasions, including when he hosted G.H. Gokhale, a member of the Indian Viceroy's Legislative Assembly (Excerpts [10] and [11]).

Overall, Gandhi had a close and lasting connection with the Gool family, particularly Dr. Abdul Hamid Gool, which is evident through their shared political activities, social interactions, and personal correspondence.

Here are some specific facts about Gandhi's connection to the Gools:

* Gandhi was hosted at the Gool home in Cape Town on several occasions (Excerpts [1], [4], [10]).
* Dr. Abdul Hamid Gool provided medical care to Mrs. Gandhi during her illness (Excerpt [4]).
* The Gools were involved in various political activities with Gandhi, including the formation of the "Cape British Indian Union" (Excerpt [6]).
* Gandhi's diary mentions helping putty and stain the floor of Dr. Gool's surgery at 7 Buitencingle in 1911 (Excerpt [6]).
* The correspondence between Gandhi and Abdul Hamid Gool is documented, with letters exchanged while Gandhi was a medical student at Guy's Hospital in London (Excerpt [3]).

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained Coloured teachers under the apartheid-era segregated education system.

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

It was established in 1937 "to discuss everything under the sun" [3]. The NEF issued a clarion call to all Non-European organisations to meet to counter the predicted assault on the remaining rights of the "Coloured" community [3].

The book "The Awakening of a People" by I.B. Tabata was published as an NEF publication and recorded the coming into existence of the All African Convention, one of the pillars of the Non-European Unity Movement [6].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was founded in 1943 [1]. Its programme was based on the Ten Point plan for equal rights. The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council. It was affiliated with the All African Convention [1].

The NEUM had several key principles: 

*   Non-collaboration 
*   Boycott weapon 
*   Principle of non-Collaboration 
*   Refutation of “race” and the ideal of a non-racial society

### q20 — Describe the author's involvement in cricket.

The author, Yousuf (Joe) Rassool, was actively involved in cricket through Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in the Cape [1].

The author learned to play cricket seriously with the help of his friend Hassen Mall [7] and began to master the elements of the forward defensive stroke [7]. He even attempted to organize a union for playing cricket at High School and College level, but was thwarted by bureaucracy [11].

Despite not making the final team that went on tour to Johannesburg, the author had some notable cricketing achievements. He scored 12 runs in a game and was hailed as a hero [9]. The author's innings at Rosmead also impressed Edross [17], who thought he would be a great choice for the Indian cricket Union if he were a selector.

The author was part of several teams that competed against each other, including the Kismets team that participated in the biennial tournament. Some of his teammates included Hassen Mall, Councillor Hoosain Parker, and others [14].

Cricket played an important role in the author's life, and he often referenced it throughout the memoir. However, the author also emphasized that cricket was not just a game but also a platform for building relationships and community among people from different backgrounds.

The provided sources do not contain information about the author's participation in professional or national-level cricket competitions outside of South Africa.

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool [1] and later also referred to as Bibi Gool. She was a daughter of J.M.H. Gool (by his wife Bibi Gool) and grew up in the Gool household at 7 Buitencingle Street in District Six, Cape Town.

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool. He was a Muslim/Indian, but his children were reared and baptized in the Wesleyan church [7]. The book provides several details about Peter Alexander Rassool, including that he had himself circumcised and was given the name Peerbhai or Peru by his family [9].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin (also known as Jessie) Rassool

Source: [1]

### q24 — Who were the children of J.M.H. Gool?

According to [1], J.M.H. Gool's children with Bibi Gool include:

1. Abdul Hamid Gool
2. Jane Gool-Tabata
3. Mariam Margie Gool
4. Minnie Gool
5. Zobeida Gool
6. Goolam Gool
7. Fatima Gool
8. Mariam Gool
9. Mohamed Saaid Gool

According to [1], J.M.H. Gool's children with Wahida Gool include:

1. Ayesha Rassool (with Bibi Gool, but later adopted by Wahida)
2. Mohammed Hanief Gool
3. Fatima Timmie Gool
4. Zohra Abdurahman

Note: [1] also mentions that J.M.H. Gool had two wives and multiple children, but the provided list may not be exhaustive.

Additionally, according to [12], the following people are shown in a photo as being related to J.MH Gool:

Back Row: 
- Grandpa (JMH Gool)
- Sayed Noor (Chota) 

Front Row:
- Nurjehan
- Mom (Ayesha), 
- Granny (Bibi),
- Uncle Aity (Sa-aid),
- Hoosain Ally

The relationship of these individuals to J.MH Gool is not explicitly stated in the provided excerpt.

### q25 — Who was I.B. Tabata?

According to source [1], I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool, and was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy.

Source [2] lists I.B. Tabata as one of the individuals who were in the forefront of contradicting historical fabrications served up in school textbooks during a festival.

Source [12] mentions I.B. Tabata, along with other notable figures such as Fred Bodmer, Gregoire Boonzaaier, Terence Macaw, Ben Kies, Paul Kostin, Wolf Kiebel, J.G., Dora Taylor, Jane Gool, and others, as luminaries in art, music, and politics who were met by the narrator.

Source [13] mentions I.B. Tabata's discussion of the Labour Party and the Parliamentary road to Socialism.

Source [14] describes I.B. Tabata as saying "My differences are not personal, they are political!" during a meeting where he rose from his seat in a scathing tone.

Source [20] shows that I.B. Tabata wrote an indictment against Ben Kies's analysis of the conflict between the Capitalist West and the Soviet Bloc as a conflict between two camps, which was evident because he rose to attack Kies.

Here is a comprehensive list of specific facts about I.B. Tabata from the sources:

* [1]: prominent political activist and leader in the Non-European Unity Movement (Unity Movement)
* [2]: involved in contradicting historical fabrications served up in school textbooks
* [12]: luminaries in art, music, and politics who were met by the narrator
* [13]: discussed the Labour Party and the Parliamentary road to Socialism
* [14]: rose from his seat in a scathing tone, saying "My differences are not personal, they are political!"
* [20]: wrote an indictment against Ben Kies's analysis of the conflict between the Capitalist West and the Soviet Bloc as a conflict between two camps

I.B. Tabata was involved in various activities related to politics and activism, including contradicting historical fabrications, discussing the Labour Party and the Parliamentary road to Socialism, and writing indictments against others' analyses.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman (also written as Abdulla Abdurahman) was a Cape Town physician and city councillor. He was the leader of the African Political Organisation (APO), later known as the People's Organisation, and was the first Coloured person elected to the Cape Town City Council as a councillor [1]. 

He was born on 1872-09-08 and died on 1940-02-20 [1].

According to excerpt [4], Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community.

Excerpts [7] and [19] mention that Dr. Abdurahman was part of a wealthy community, having had contact with notable figures such as Barney Barnato, Cecil Rhodes, Gandhi, and the Prince of Wales.

He married Helen "Nellie" Potter James [18], and his daughter Cissie Gool was involved in activism against residential segregation in Cape Town [18].

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources describe a close relationship between Gandhi and J.M.H. Gool, particularly through correspondence and personal interactions:

- [1] Haji Joosub Maulvi Hamid Gool was a community leader who received notable visitors at his residence in District Six, including Mahatma Gandhi.
- [2] Ralph Bunche's travel notes of 1937 do not mention J.M.H. Gool, suggesting the old man may have retired from public life.
- [3] The correspondence between J.M.H. Gool and Abdul Hamid Gool (his son) is mentioned in Indian Opinion Oct 6, 1906. When Abdul Hamid was a medical student at Guy's Hospital in London, Gandhi had written to him.
- [4] Gandhi had visited the Gools' residence at Buitencingle Street in 1911 and helped puttying and staining the floor of Dr. Gool's surgery, as recorded in his diary.

Additionally:

- [13] J.M.H. Gool and M.K. Gandhi exchanged letters; a letter from J.M.H. Gool to M.K. Gandhi dated January 23, 1897, is mentioned.
- [18] Gool submitted subscription dues for the Indian Opinion on behalf of Wilson, as per Gandhi's reference in his letter.

The connection between Gandhi and J.M.H. Gool suggests a close personal relationship, with interactions including correspondence, hospitality, and mutual respect.

Gandhi visited the Gools' residence at least twice (in 1911), indicating a regular and close relationship.

### q28 — Which organisations was the author involved in?

The provided sources list the following organisations the author, Joe Rassool, was involved with:

1. New Era Fellowship (NEF) [4], [11]
2. Non-European Unity Movement (NEUM) [1] (as an umbrella body)
3. Teachers League of South Africa (TLSA) [1] (organised Coloured teachers)
4. African People's Organisation (APO) [4] (responded to the call for unity and democratic rights)
5. Communist Party of South Africa [4]
6. Fourth International of South Africa [4]
7. Liberation League [4]
8. Workers' Party [4]
9. Smuts's United Party (Coloured branch) [4]
10. Hewat Amateur Theatrical Society [14] 
11. Trafalgar Players [14]

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources state that the Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM) [1]. The TLSA shared the boycott and non-collaboration programme of the NEUM [1]. This suggests a close relationship between the two organizations, with the TLSA actively supporting and participating in the activities of the NEUM.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) arrived in Cape Town in 1884, coming from Mauritius. Specifically, his family origins were from Swat via Gujarat, India. 

The provided sources do not contain information on a specific date of arrival for the party mentioned in [18] and [20], however it is stated that they arrived in December 1900.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque or Quwatul Islam Mosque, was a mosque founded in 1898 (source [1]) and one of the oldest mosques in the Cape Muslim community. It was located on Loop Street in Cape Town (sources [1], [19]), [20]). The author's grandfather was a founder and life trustee of the mosque (source [20]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1] Haji Joosub Maulvi Hamid Gool, Cissie Gool is the daughter-in-law of J.M.H. Gool, being married to his son Abdul Hamid Gool.

Later in the sources, it is mentioned that Cissie Gool was the wife of Dr. A (Abdul) Gool [6] and also referred to as "my aunt Cissie Gool" by the author [7], indicating a close family relationship.

However, no source explicitly states how she is related to J.M.H. Gool beyond being married to his son.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool knew the following notable historical figures personally:

[1] - Haji Joosub Maulvi Hamid Gool mentions meeting Mahatma Gandhi at his home in 1914. Gandhi helped putty and stain the floor of Dr. Gool's surgery during a visit.

[5] - Dr. Abdulla Abdurahman is mentioned as possibly having shared a clan affiliation with J.M.H. Gool, but not being related to him.

[9] - Cissie Gool (Zainunnissa) was the daughter of Dr. Abdulla Abdurahman and the future mother-in-law of Abdul Hamid Gool.

[10] - Dr. Gool is mentioned as writing two medical articles for the Indian Opinion, where it is likely that he met other notable individuals.

[11] - Dr. Gool's involvement in politics and his connections to the Indian community in Cape Town are mentioned, but no specific individual is named.

[12] - J.M.H. Gool & Co. supplied goods to the British troops during World War I, which may have led to interactions with notable military leaders or officials.

[18] - Dr. Abdurahman's grandfather was brought to South Africa as a slave and became prominent in the community, and his mother, Gadija Dollie, was related to Wahida Ta' al (Dr. Gool's second wife). This suggests that J.M.H. Gool had connections with the Abdurahman family.

[19] - Wahida Ta' al (Dr. Gool's second wife) is mentioned as having performed Hajj in 1899, which may have involved interactions with notable Islamic leaders or scholars.

In addition to these specific mentions, [8] and [16] provide general context about the community and events that J.M.H. Gool was part of, but do not mention specific individuals.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act (1950) was apartheid legislation in South Africa that aimed to separate different racial groups into distinct areas. The law declared various areas as "White" or designated them for specific racial groups, such as Indians, Africans, and Coloureds.

District Six, a vibrant multicultural community near the center of Cape Town, was declared a White area under the Group Areas Act in 1966. As a result, all non-White residents were forcibly removed to the Cape Flats, bulldozing and demolishing almost every building in the process. This destroyed a thriving community and scattered its residents.

The act affected District Six by:

1. Forcing non-White residents out of the area: The Group Areas Act led to the displacement of thousands of people from District Six, who were relocated to less desirable areas.
2. Bulldozing and demolishing buildings: The apartheid government destroyed almost every building in District Six, leaving few structures intact.
3. Disrupting community life: The removals shattered the social fabric of the area, separating families and friends.
4. Creating housing shortages: The Group Areas Act exacerbated existing housing problems, as people were forced to live in cramped conditions on the Cape Flats.

The Group Areas Act is often seen as a prime example of apartheid's destructive policies, leading to the displacement and marginalization of communities across South Africa. [Sources: 1, 8, 13]

### q35 — Who was Hassen Mall?

Hassen Mall was a close friend and fellow cricket player from District Six of Yousuf Rassool. He is described as brilliant cricketer, having spun a wristy leg break that zipped like a circular saw. After discovering that the sciences were not his forte, he switched to Law and went on to qualify with the degree BA LLb 44 (from University of Cape Town). 

Specific facts about Hassen Mall from various excerpts are:

* He was staying in Durban when Yousuf Rassool first met him.
* He had come to do medicine at Cape Town but later switched to Law.
* He became a brilliant cricketer, and his wristy leg break zipped like a circular saw.
* In 1951, he left for Durban after completing his LLB degree.
* He was the captain of the cricket team in the biennial tournament that took place at Durban's Curry's Fountain in early January.

Note: The provided sources do not contain detailed information about Hassen Mall's background, family, or ancestry.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to [1], the main political organisations active in the Cape Coloured community during the mid-twentieth century were:

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA)
3. The New Era Fellowship (NEF)
4. The All African Convention (AAC)

These organisations shared a boycott and non-collaboration policy against apartheid institutions.

Additionally, [17] mentions that during the author's lifetime, other notable organisations active in the Cape Coloured community included:

1. The New Era Fellowship
2. The Teachers' League of South Africa
3. The Communist Party of South Africa
4. The Fourth International of South Africa
5. The Liberation League
6. The Workers' Party

[15] mentions that these organisations, including the New Era Fellowship and the Teachers' League of South Africa, responded to the call for unity and full democratic rights in the Coloured community.

It's worth noting that [14] states that this book is a remembrance of the author's family ties and cultural roots, but it doesn't provide specific information on political organisations. However, it does mention the importance of understanding the past to appreciate freedom and equity in South Africa.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer to work on a legal case [1]. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance) as a tool of political struggle. 

He was in South Africa on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria [16]. The accounts were kept in Gujarati, which was Gandhi's mother tongue [16].

Gandhi later came back to Cape Town in 1914 and was taken in procession to the Cape Town Docks and presented with gifts and addresses on his last visit before leaving South Africa for good [6].

Sources do not specify why exactly he decided to stay in South Africa for over 20 years, but it is implied that during this period, Gandhi became deeply involved in the Indian community's struggles against racial segregation and discrimination.

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to various sources, including [1] and [17], District Six was a vibrant, multicultural community near the centre of Cape Town. It was situated below Devil's Peak and was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

Source [4] describes it as a place where "a community of freed slaves from America and the West Indies thrived in Victorian times." Some of them achieved great position and wealth, but were later expelled at the turn of the century.

[18] mentions that before the demolition, District Six was a pulsing community with life, zest, fire, warmth, and care. It's described as having "more life, more zest, more fire, more warmth and care than many other places one could mention."

Source [20] describes it as an inimitable neighborhood which would soon be bulldozed out of existence, a "black spot" in the Apartheid scheme of things.

Overall, District Six was a cosmopolitan area with a rich cultural heritage, where people from different backgrounds lived together and built a thriving community.

### q40 — What was the Unity Movement's boycott policy?

According to [1], the Non-European Unity Movement (NEUM) adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

Additionally, [8] mentions that the boycott of the Group Areas Act would mean refusing to cooperate with the Group Areas Boards without having any guarantee that this would stop the implementation of the law. Some members of the "Movement" ignored the injunction to boycott and appeared before the board or sent their legal representatives to register their protest.

Furthermore, [10] states that all those who accepted service on the C.A.C. were boycotted in every aspect of their lives, socially, economically, and otherwise. The issue pervaded the minds of the community, with meetings held almost every night at halls all over Cape Town.

Lastly, [11] mentions that the boycott was a weapon of choice for the Anti-CAD (Anti-Coloured Affairs Department), which won an important battle using this tactic and pointed the direction to be taken by the Anti-CAD.


  ✅ Overall: 86.1% recall (token-overlap)  (180.0/209)  avg 27649ms
