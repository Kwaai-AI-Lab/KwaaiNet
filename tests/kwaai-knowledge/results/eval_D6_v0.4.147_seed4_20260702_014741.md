[2m2026-07-02T08:47:49.540439Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T08:47:49.742960Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/3 keywords  18799ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T08:48:11.797205Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  24809ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T08:48:35.730909Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23937ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T08:48:59.218608Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  22090ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T08:49:21.402039Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  30763ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T08:49:52.243910Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/8 keywords  28360ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T08:50:20.969060Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  20532ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T08:50:41.919575Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23290ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T08:51:04.944426Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/9 keywords  26632ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T08:51:32.061290Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  27747ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T08:52:00.169972Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  25029ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T08:52:25.344870Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  28617ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T08:52:53.928221Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  22238ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T08:53:16.411964Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  21286ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T08:53:37.803525Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 89 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced relocation history""
  ○ Round 3   added 14 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  32658ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T08:54:09.586339Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  26583ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T08:54:36.369133Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  19891ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T08:54:56.464113Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24413ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T08:55:21.080567Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24359ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T08:55:45.636685Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  22654ms
  [21/40] Who was the author's mother?
[2m2026-07-02T08:56:10.004223Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/5 keywords  21278ms
  [22/40] Who was the author's father?
[2m2026-07-02T08:56:30.430922Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  20511ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T08:56:51.033565Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  20823ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T08:57:12.024480Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  29603ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T08:57:41.834943Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  24488ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T08:58:06.525380Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  26097ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T08:58:32.833079Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  27175ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T08:59:00.213232Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  20541ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T08:59:20.960685Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  20029ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T08:59:41.215047Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/6 keywords  28294ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T09:00:09.705988Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  28391ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T09:00:38.287302Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  30585ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T09:01:09.109697Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  23257ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T09:01:32.579321Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  31151ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T09:02:04.671125Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  25425ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T09:02:29.527839Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  22597ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T09:02:58.683068Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  30285ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T09:03:28.286002Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27919ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T09:03:52.177716Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  26207ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T09:04:18.324553Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  23848ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 82.8% (173.0/209) |
| Avg latency | 25079ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 0/3 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 18799ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 24809ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23937ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22090ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 30763ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 28360ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 20532ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 23290ms |
| q09 | Who was the author's grandfather? | 7/9 (78%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 26632ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 27747ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 25029ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 28617ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 22238ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 21286ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | [Graph: District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 32658ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 2/5 (40%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 26583ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 19891ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 24413ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 24359ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 22654ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 21278ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 20511ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 20823ms |
| q24 | Who were the children of J.M.H. Gool? | 6/7 (86%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 29603ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 24488ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 26097ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 27175ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 20541ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/4 (50%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 20029ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | LEST WE FORGET -rev25.pdf, sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: Haji Joosub Maulvi Hamid Gool] | 28294ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 28391ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 30585ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 23257ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | [Graph: District Six], sequence_diagram:District Six, LEST WE FORGET -rev25.pdf | 31151ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 25425ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 22597ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 30285ms |
| q38 | Who was Cissie Gool's father? | 5/5 (100%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 27919ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 26207ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23848ms |

## Answers

### q01 — Who is the author?

The provided sources do not contain that information.

### q02 — Who are the author's children?

The provided sources state the following about the author's children:

* Feyruz
* Reza
* Zarina (also mentioned as "my grandchildren" in [18] and [19])
* Joseph
* Samuel
* Alexander
* Leonardo
* Dylan
* Lewis 

All of these names appear in the dedication section of the book, listed as the author's children and grandchildren.

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool 

(Listed in [1] and repeated in the dedication on pages 2 and 11)

### q04 — To whom is the book dedicated?

The book is dedicated "For my children: Feyruz, Reza, and Zarina, and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis." [18]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a spice merchant and community leader who lived in Cape Town, South Africa. According to the sources:

* He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.
* He was associated with early Cape Muslim and Indian political circles.
* He became involved in the politics of the Indians in Cape Town and helped bring two rival organizations together as "The Cape British Indian Union" (Source [6]).
* In 1911, he became president of this organization but later resigned that year (Source [6]).
* He was married to two women: Bibi Gool (whom he married in 1879 or 1880) and Wahida Ta'al.
* He had several children with his wives, including Ayesha Rassool (by Bibi).
* He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six (Source [1]).
* In 1899, he performed the Hajj (pilgrimage to Mecca) (Source [1]).
* He died on April 1, 1940.
* He was the grandfather of the memoir's author, Yousuf (Joe) Rassool.

Gool also had a medical practice and wrote articles for the Indian Opinion journal.

### q06 — Tell me about Buitencingle.

Buitencingle is a place mentioned throughout the excerpts as being located in Cape Town, South Africa. Here are the specific details that can be gathered from the sources:

* It was the location of the grand 13-room mansion owned by J.M.H. Gool (Excerpt [1]). The street on which the mansion stood, Buitencingle Street, was wide and cobbled with traffic islands sporting stunted palm trees (Excerpts [4] & [10]).
* In the early 20th century, Buitencingle was a vibrant area with élan and grandeur (Excerpt [10]). However, when the house is "going to be shut up, barred and left to the dust and rodents then its spirit dies and it becomes a dead shell" (Excerpt [10]).
* In Excerpt [18], Buitencingle Street is mentioned as being located near Kloof Nek, which was a connecting saddle of land that joined Table Mountain to Lion's Head.
* It is also noted in Excerpt [19] that the pronunciation of Buitencingle may have been Baytencingle (although this is not universally accepted).
* The cingle referred to in the name Buitencingle refers to a belt, and at the time, Buitencingle was the outermost belt of Cape Town.
* Beyond it lay the foothills of Table Mountain.

Overall, while there are limited specific details about the layout or features of Buitencingle itself, it is clear that it was an affluent area with large homes like J.M.H. Gool's mansion, and its location provided a scenic view.

### q07 — Who is the author's wife?

According to [1], the author's wife is Professor Nazima Rassool.

### q08 — Tell me more about the author's wife.

According to the sources, specifically [1] and [19], the author's wife is Professor Nazima Rassool, who is described as an academic researcher. She plays a significant role in the writing of this memoir, urging the author to ground his personal memories in historical research.

From the available information:

* She is married to the author, Yousuf Rassool [1].
* She has academic credentials, which enabled her to encourage and assist the author with background investigation and research.
* Her input helped transform the book from a purely subjective recollection into a historically documented account of District Six [4].
* The author owes a great deal to her, as mentioned in excerpts [4] and [19].

It is not possible to provide more specific biographical information about Professor Nazima Rassool beyond what is provided in the sources.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Joosub Maulvi Hamid Gool. He was a spice merchant and community leader who lived in Cape Town. Specific facts about him include:

* Arrived in Cape Town: 1884 [1]
* Origin: Mauritius (via Swat, Gujarat) [1]
* Lived in: District Six, 7 Buitencingle Street [1]
* Had two wives: Bibi Gool and Wahida Gool [1]
* Had several children with his wives, including:
	+ Abdul Hamid Gool
	+ Ayesha Rassool (by Bibi)
	+ Mohammed Hanief Gool
	+ Zobeida Gool
	+ Jane Gool-Tabata
	+ Mariam Margie Gool
	+ Minnie Gool
	+ Zohra Abdurahman
	+ Fatima Timmie Gool
* Grandfather of the author, Yousuf (Joe) Rassool [1]
* Performed Hajj in 1899 [3,10]

### q10 — Tell me about Kloof Nek.

According to the sources, here's what we know about Kloof Nek:

* It is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1].
* It offers views of both Table Mountain and Lion's Head [1].
* The area is described as a place of "haunting beauty" [4, 18].
* Kloof Nek was the connecting saddle of land that joined Table Mountain to Lion’s Head [2, 3].
* The Glen in Kloof Nek was where people would go for relaxation and leisure activities, including reading poetry and enjoying nature [5, 17].
* It was a popular spot for lovers, mountaineers, nature lovers, and ramblers [5].
* By the time of the author's writing (around 1984), it was also a place where down-and-outs known as "Bergies" would drink cheap wine and methylated spirits in the area's cavelets, culverts, and bushes [5].

The sources do not provide extensive information about the history or geography of Kloof Nek.

### q11 — What was the Teachers League of South Africa (TLSA)?

According to [1], the Teachers League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement, and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students.

Later excerpts [3], [5], [6], and [13] mention that TLSA became a political organisation, and that its leadership changed to include new members who were more radical and activist-oriented. They note that the TLSA's transformation was part of a broader movement among the "young Turks" within the Coloured community.

In [18], various individuals associated with the TLSA are listed, including Solly Edross, who is mentioned multiple times (78, 89, 124, 126).

### q12 — Who was Cissie Gool?

Cissie Gool (1897-1963) was a South African Cape Town city councillor, lawyer, and political activist. She was:

* The daughter of Dr. Abdulla Abdurahman [1]
* A member of the Non-European Unity Movement [2]
* Married to Abdul Hamid Gool [1], with children including Shaheen Gool [13]
* Born on February 14, 1897 [1]
* Died on December 2, 1963 [1]

She was known for her significant role in South African politics and her association with the anti-apartheid struggle. Cissie Gool was also closely linked to various influential people of her time, including politicians, activists, and community leaders.

According to excerpts [4] and [12], some notable individuals she associated with include:

* Ben Kies
* Solly Edross
* Jane (her aunt)
* Her uncle, Dr.
* Various other political figures, artists, and musicians

Cissie Gool was also an important figure in her community, serving as a councillor and contributing to the history of District Six.

### q13 — What was the All Africa Convention?

According to [1], The All African Convention (AAC) was an organisation founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organisations seeking unity against racial oppression. The AAC later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

Additionally, according to [4], the All African Convention is mentioned in a list of various organisations and individuals, but no further information about the convention itself is provided.

### q14 — Where was District Six and what kind of place was it?

District Six is described as a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1]. Under the Group Areas Act, the apartheid government declared it a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building.

### q15 — What were the forced removals from District Six?

The provided sources do not contain information about the specific circumstances or details of the forced removals from District Six. However, based on general knowledge and external information, it is known that the forced removals from District Six took place in 1960-1970 as part of the Group Areas Act implemented by the apartheid government in South Africa. The residents were forcibly relocated to other areas, often with inadequate housing and facilities, leading to significant disruption and hardship for those affected.

The sources do mention the year 1956, which is before the main period of forced removals (1960-1970). However, it does not provide any information about the specific circumstances or events related to the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi's connection to the Gool family is described in various excerpts:

* Gandhi was a close friend and associate of the Gool family. [3]
* The correspondence between him and my uncle Abdul Hamid Gool whilst he was a medical student at Guy’s Hospital in London. [3] [4]
* Gandhi visited Buitencingle yet again in 1914 (February-March) and was a guest at 7 Buitencingle Street, where Dr. Gool lived. [4] [11]
* In October 1912, Gandhi was a guest at 7 Buitencingle Street, where he received the visiting dignitary G. H. Gokhale, who was read an address of welcome by Dr. Gool. [11]
* The British Government asked the Natal authorities to take action against the assailants when Gandhi and his family returned to Natal in December 1896, but it was Gandhi's wish not to lay charges against the misguided bigots. [8]

Gandhi's connection to the Gool family is described as close friendship, association, and regular interactions, including:

* The Gool family attended functions honoring Gandhi.
* Gandhi visited the Gool family at their home in Buitencingle.
* The Gool family provided hospitality to Gandhis in February 1914. [19] [20]

Overall, Gandhi was a significant figure in the lives of the Gool family, and they maintained close relationships with him throughout his visits to South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students. It trained Coloured teachers under the apartheid-era segregated education system [1]. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

It was established on undeveloped grounds opposite the Roeland Street Gaol, which had previously been a Whites-only Primary school. The authorities considered it too close to the prison and District Six, but it opened for its first intake of students in 1944 with considerable fanfare [3].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement [1]. It held public lectures and debates on political, social, and philosophical topics, serving as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1]. The NEF had a programme of events, which was overseen by its Organising Secretaries, one of whom was Hassan Bavasah [12].

The New Era Fellowship was founded in 1937 "to discuss everything under the sun" [3] and had become an important part of the Coloured community's political landscape. Its members were involved in various activities, including representing the NEF at conferences and participating in discussions about social and political issues.

However, it is also mentioned that the New Era Fellowship was seen as "too advanced for the African youth" [10], suggesting that the NEF had a specific focus or mission, possibly related to the development of young Africans.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization founded in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism. According to source [1], its programme was based on the Ten Point plan for equal rights, and it adopted a boycott and non-collaboration policy.

Source [7] notes that the Unity Movement brought principles such as non-Collaboration, refutation of "race", and the ideal of a non-racial society to the struggle for liberation. However, source [10] states that despite being convinced their ideas would triumph, the movement ultimately failed to meet the challenge of rapidly changing times.

The NEUM was active in various areas, including education, politics, and community organizing. Source [8] mentions its role in politicizing non-political structures such as sports bodies, churches, charities, mothers' unions, rate-payers, and civics, bringing them into the Movement. The organization also had a significant impact on the broader anti-apartheid struggle.

It's worth noting that some individuals within the NEUM, like Richard Rive and Alex La Guma, made notable contributions to literature despite the system of Apartheid restricting publication opportunities for non-whites (source [13]).

### q20 — Describe the author's involvement in cricket.

The author, Yousuf (Joe) Rassool, was actively involved in cricket through Kismets Cricket Club and the Western Province Indian Cricket Union. He played and administered club cricket in the Cape. According to [1], he was part of a team that poached top cricketers from other teams, including Salie Van Haacht and Ben Malamba.

The author mentions his own cricketing abilities, stating that he could score 10 runs as a personal achievement (source not specified), but it is likely referring to [5]. He also describes his experience playing with Hassen Mall, saying that they were "close friends" through cricket ([6]). 

Rassool was a member of the Kismets Cricket Club and took part in organizing cricket at High School and College level, though he was thwarted by bureaucracy ([11]).

He also mentions his experience captaining a team alongside Hassen Mall in [13] and again in [19].

### q21 — Who was the author's mother?

The provided sources do not contain that information. However, it can be inferred from [20] that the author's grandmother (Bibi) and grandfather had a child named Wahida, who in turn, gave birth to Ayesha. Ayesha is identified as the author's mother in [10].

### q22 — Who was the author's father?

The author's father is Peter Alexander Rassool, who later became known as Peerbhai or Peru after converting to Islam. He was previously named Malick Rassool and had a Christian upbringing before marrying Ayesha (Lallie) in 1928 [11].

### q23 — Who were the author's siblings?

According to [1], the author's siblings are:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool

### q24 — Who were the children of J.M.H. Gool?

According to the sources, the children of J.M.H. Gool are:

1. Abdul Hamid Gool (fathered by Bibi Gool)
2. Ayesha Rassool (half-sister, fathered by Wahida Gool)
3. Fatima "Timmie" Gool
4. Fatima Gool (twins, or one mentioned separately with a nickname)
5. Goolam Gool
6. Mariam Margie Gool
7. Minnie Gool
8. Mohamed Saaid Gool
9. Zobeida Gool

Note: Some of the sources mention multiple children with similar names, but they are not explicitly stated to be twins or separate individuals. However, some of them can be identified as being related to different wives (Bibi and Wahida) based on the context.

Sources:
[1] Lists J.M.H. Gool's children.
[4] Mentions Ayesha Rassool (half-sister).
[8] Does not list specific children but mentions the large number of children.
[10] Refers to Fatima "Timmie" Gool, who was a beauty and had interactions with Agent-General Sir Sayed Raza Ali.
[12] Includes Abdul Hamid Gool, Ayesha Rassool, Fatima Timmie Gool, and possibly others in the same group of children.

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool, and was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1]. 

Tabata qualified as BA LLB and made a tremendous impression on the author with his talk at the NEF on One Day in the Life of Ivan Denisovich [8].

Tabata completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship, which was to be an NEF publication [3] and [19]. The book recorded the coming into existence of the All African Convention, one of the pillars of the Non-European Unity Movement [19].

He was accused by some in the Forum Club (a Marxist Trotskyist group) of failing to have a class analysis in his historical tract "The Awakening" [4], but he was also described as having written an indictment against another leader, Kies, which he rose to attack during a conference [20].

### q26 — Who was Dr. Abdullah Abdurahman?

Here are the specific facts about Dr. Abdullah Abdurahman mentioned in the sources:

* He was born on September 8, 1872 [1].
* He was a Cape Town doctor and city councillor [1].
* He led the African Political (later People's) Organisation (APO) [1].
* He was the first Coloured person elected to the Cape Town City Council [1].
* His father was associated with Haji Joosub Maulvi Hamid Gool and the Non-European Unity Movement [1].
* Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [4].
* He had a residence on Loop Street, which was close to Buitencingle [4-5].
* He was involved in informal discussions and more formal collaboration with Grandpa (Yousuf Rassool's grandfather) and their organisations [5].
* Dr. Abdurahman was also associated with the Indian Opinion, as his daughter Cissie Gool submitted a subscription on his behalf [14].

These sources provide some information about Dr. Abdullah Abdurahman's personal life, his involvement in politics and community service, and his relationships with other notable individuals of the time.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to sources [1], [5], [6], [13], and [19], the connection between Gandhi and J.M.H. Gool was a close one. Here are some specific points mentioned in the sources:

* Gandhi visited J.M.H. Gool's home at 7 Buitencingle Street, Cape Town, on several occasions ([5] and [13]).
* In 1897, Gandhi wrote to J.M.H. Gool, and J.M.H. Gool responded with a letter on January 23rd of the same year ([13]).
* Gandhi helped to furbish (fix up) Dr. Gool's surgery by puttingtying and staining the floor in 1911 ([5] and [13]).
* Gandhi was a guest at the Gools' residence in February 1914, and this visit led to a friendship between the Gools and Joe Rassool's family ([7] and [12]).
* Gandhi corresponded with Dr. Gool while he (Gandhi) was on deputation in London, and there is also a record of correspondence between Gandhi and Abdul Hamid Gool, J.M.H.'s son ([3], [5], and [19]).

It's clear that the two men had a close relationship, with Gandhi visiting Dr. Gool's home on several occasions and corresponding with him regularly. The sources suggest that they shared a mutual respect and admiration for each other.

### q28 — Which organisations was the author involved in?

The author, Yousuf (Joe) Rassool, was involved in the following organisations:

* The New Era Fellowship (NEF)
* The Teachers League of South Africa (TLSA)
* The Non-European Unity Movement (NEUM)
* The All African Convention (AAC)
* The Anti-CAD
* The Hewat Amateur Theatrical Society
* The Trafalgar Players
* The Liberation League

He was also involved in various student and community organisations, including the Cape Peninsula Students Union.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The TLSA (Teachers' League of South Africa) approached the Congress movement suggesting a unified strategy to counter anti-educational plans of the regime [10]. The TLAS also joined the Non-European Unity Movement, acknowledging that the struggle for equal education was bound up with the struggle for a democratic society [18] and [19].

Note: There is no direct information in the provided excerpts about whether this relationship was collaborative or adversarial.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information about when J.M.H. Gool (also referred to as Haji Joosub Maulvi Hamid Gool or Grandpa) arrived in Cape Town, except for a mention that the party including Bibi Gool and her children arrived in December (source [3]). However, source [4] suggests that Grandpa had to be back in Cape Town at least nine months prior to his daughter Jane's birth on March 19th, 1902. This implies that he departed from Bombay in April or early May 1901.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque, is a mosque located on Loop Street in Cape Town. According to [1], it was founded in 1898 and is one of the oldest mosques in the Cape Muslim community.

[20] also mentions that the mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile, and that the author's grandfather was a founder and life trustee of the mosque.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], Haji Joosub Maulvi Hamid Gool's daughter with his first wife, Bibi Gool, is Ayesha "Lallie" Gool. His son with his second wife, Wahida Gool, is Abdul Hamid Gool. Cissie Gool was married to Dr. Abdulla Abdurahman and became the mother-in-law of Abdul Hamid Gool.

However, [9] states that Abdul Hamid Gool had a close relationship with Cissie Gool as they were part of the same family circle. 

[12] also indicates that Shaheen, son of Uncle Doctor (Abdulla Abdurahman) and Cissie Gool, met the narrator for the first time at Grandpa's funeral.

But most directly, [19] mentions "Cissie Gool, confidently returns the gaze of the camera lens and holds an angelic pose in the center of the frame. Abdul Hamid, a decade into his marriage with Cissie, chooses, instead, to cling protectively to his adopted sister, Toetie."

[1] also states that J.M.H. Gool had two wives: Bibi Gool and Wahida Gool.

Therefore, according to these excerpts, Cissie Gool was the second wife's daughter-in-law (married to Dr. Abdulla Abdurahman) but also appeared in a context where Abdul Hamid is described as being in a marriage with her.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the sources, the following are notable historical figures that J.M.H. Gool knew personally:

* Mahatma Gandhi (Source: [1], [11])
* Cecil John Rhodes (Source: [1])
* George Bernard Shaw (Source: [1])
* Dr. Abdulla Abdurahman (Source: [1], [18])

Note: The sources do not mention personal relationships or interactions with these individuals, but it is clear that they were acquaintances of J.M.H. Gool.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not contain comprehensive information about the Group Areas Act itself. However, [2] states that "Apartheid legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life." This implies that the Group Areas Act was part of the apartheid regime's efforts to control people's living spaces.

[3] mentions that "the Non-White group most aggrieved by the Group areas Act was the South African Indians." However, it does not specify how this act affected them or District Six specifically.

[4] explains how the apartheid government built new townships to entice people away from District Six. It states: "But once the exit began they started to build the sub-economic schemes, which, ironically, sported the flashy name like 'Silvertown'. Soon other townships were erected to entice the people away from District Six: places like Bonteheuwel, Hanover Park, Mannenberg, Mitchell's Plain, Vanguard Estate, and Bishop Lavis." This shows that the Group Areas Act led to the forced removal of people from District Six to these new townships.

[5] implies that this relocation resulted in some negative consequences, such as the loss of municipal franchise. However, it does not specify how the Group Areas Act directly affected District Six.

In summary, while the provided sources do not contain comprehensive information about the Group Areas Act, they suggest that it was a part of the apartheid regime's efforts to control people's living spaces and lead to forced removals from District Six.

### q35 — Who was Hassen Mall?

According to sources [1] and [6], Hassen Mall was a close friend of the author Yousuf Rassool, a fellow cricket player from District Six in Cape Town. 

From source [1]: Hassen Mall was "a cricketer" who lived in Cape Town; he was affiliated with the University of Cape Town.

From source [6]: Hassen Mall was described as being around average height but radiating magnetism and having a clipped brush moustache when older, which suited him well. He had a brilliant cricket career and could spin a wristy leg break that zipped like a circular saw.

From source [7]: It is noted that Hassen Mall was aware of the Group Areas Act and the struggle for full democratic rights in South Africa.

From source [12]: Hassen Mall captained the team, was secretary of the Western Province Indian Cricket Union, and decided to bat at number one instead of his usual position, which was number four.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to [1], the main political organisations active in the Cape Coloured community during the mid-twentieth century were:

* The Non-European Unity Movement (NEUM)
* The Teachers League of South Africa (TLSA)
* The New Era Fellowship (NEF)
* The All African Convention (AAC)

These organisations were united by their boycott and non-collaboration policy against apartheid institutions, with NEUM as the umbrella body.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to [1], Mohandas Karamchand Gandhi, known as Mahatma Gandhi (Mahatma meaning "Great Soul"), came to South Africa in 1893 as a lawyer to work on a legal case. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance, or passive resistance) as a tool of political struggle.

[1] states that Gandhi was associated with Dr. Abdulla Abdurahman and Haji Joosub Maulvi Hamid Gool, and his son, Manilal Gandhi, is mentioned as the father of Mohandas Karamchand Gandhi.

### q38 — Who was Cissie Gool's father?

According to excerpt [1], Cissie Gool's father was Dr. Abdulla Abdurahman (also written as Dr. Abdullah Abdurahman). He is described as a pioneering Cape Town city councillor, physician, and political leader of the Cape Coloured community.

Source: [1]

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to [1] and [3], District Six was "a vibrant, multicultural community near the centre of Cape Town", situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

In [2], it is described as having a mix of economic housing schemes, including some that were considered sub-economic, but these resulted in the inhabitants losing their municipal franchise.

[4] describes District Six as "a thriving cosmopolitan area in the heart of Cape Town" before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats. It had a rich history of politi c activism, and was not defined by a culture of silence and acquiescence.

In [3], the author describes walking through District Six with his ten-year-old eyes as he headed for school each morning, describing Caledon Street as "getting tattier along the way".

These sources suggest that District Six was a dynamic, multicultural community with a strong sense of culture and history.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy involved refusing to participate in any apartheid government institutions, including the Coloured Advisory Council (C.A.C.) [1]. They also used the boycott as a weapon of choice in various situations, such as the Van Riebeeck Festival [6], where they successfully used the occasion to do some intensive political education and to contradict historical fabrications [20].

In one instance, the boycott was applied against members who accepted service on the C.A.C., with all those accepting service being boycotted in every aspect of their lives, socially, economically, and otherwise [11]. 

The Movement's stand during elections was also to boycott them, but if Coloured voters insisted on voting, they were asked to spoil their ballot by writing "For Full Democratic Rights" across the ballot slip [18].

Note: The policy is not explicitly stated as a formal set of rules in any excerpt. However, based on the information provided, it appears that the Unity Movement used boycotts as a strategic tactic to resist apartheid institutions and policies, often accompanied by non-cooperation or other forms of direct action.


  ✅ Overall: 82.8% recall (token-overlap)  (173.0/209)  avg 25079ms
