[2m2026-07-02T07:53:17.446419Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T07:53:17.563109Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  19681ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T07:53:40.895390Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  24644ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T07:54:03.988514Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23150ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T07:54:26.605919Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  22079ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T07:54:48.734284Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  31495ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T07:55:20.645315Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  26942ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T07:55:47.603594Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  24642ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T07:56:12.488193Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  21488ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T07:56:41.810553Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/9 keywords  31241ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T07:57:25.620449Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 6/7 keywords  50639ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T07:58:14.761566Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  50012ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T07:59:05.536068Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  52346ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T07:59:56.952755Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  40058ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T08:00:35.686788Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  36597ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T08:01:09.194591Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 90 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced relocation South Africa history""
  ○ Round 3   added 18 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  57815ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T08:02:20.986230Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  54733ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T08:02:50.605125Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  20813ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T08:03:09.782515Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  21825ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T08:03:37.036112Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  31680ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T08:04:07.087974Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/5 keywords  35440ms
  [21/40] Who was the author's mother?
[2m2026-07-02T08:04:41.963070Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  30336ms
  [22/40] Who was the author's father?
[2m2026-07-02T08:05:15.877832Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24828ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T08:05:37.319599Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23611ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T08:06:05.627924Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  33384ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T08:06:35.652911Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/5 keywords  29254ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T08:07:03.794873Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  27697ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T08:07:31.244357Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  27007ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T08:07:58.377705Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  23622ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T08:08:22.419598Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  27225ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T08:08:49.655367Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0.8/6 keywords  30571ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T08:09:20.388461Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  29188ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T08:09:50.197555Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  29816ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T08:10:20.027738Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  30940ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T08:10:51.085881Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  34553ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T08:11:26.153021Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/4 keywords  27653ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T08:11:53.644330Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  24772ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T08:12:18.610968Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  23305ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T08:12:42.060980Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  21710ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T08:13:04.019323Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  27579ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T08:13:32.785856Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  25617ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 78.4% (163.8/209) |
| Avg latency | 30749ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 19681ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 24644ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23150ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22079ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31495ms |
| q06 | Tell me about Buitencingle. | 8/8 (100%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 26942ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 24642ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 21488ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31241ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf | 50639ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 50012ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 52346ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 40058ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 36597ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 57815ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Gool family] | 54733ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 20813ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 21825ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 31680ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 35440ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 30336ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 24828ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 23611ms |
| q24 | Who were the children of J.M.H. Gool? | 6/7 (86%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 33384ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 29254ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 27697ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 27007ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23622ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 27225ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0.8/6 (13%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool | 30571ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 29188ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29816ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 30940ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 34553ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 27653ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/4 (50%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 24772ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: South African Directory], LEST WE FORGET -rev25.pdf | 23305ms |
| q38 | Who was Cissie Gool's father? | 2/5 (40%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 21710ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 27579ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 25617ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The author's children are:

1. Feyruz [4]
2. Reza [4]
3. Zarina [4]

He also has grandchildren, including:
 
1. Joseph [4]
2. Samuel [4]
3. Alexander [4]
4. Leonardo [4]
5. Dylan [4]
6. Lewis [4]

### q03 — Who are the author's grandchildren?

The author's grandchildren, according to [1] Yousuf Rassool (also known as Joe Rassool), are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool

([1])

### q04 — To whom is the book dedicated?

The book "District Six - Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [17]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool is a historical figure mentioned in the provided sources. Here are the specific facts about him based on the sources:

* He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.
* He was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He lived at 7 Buitencingle Street and had two wives: Bibi Gool (whom he married in 1879 or 1880) and Wahida Ta'al (with whom he had children including Abdul Hamid Gool).
* He was associated with early Cape Muslim and Indian political circles.
* He performed the Hajj in 1899.
* He died on April 1, 1940.
* He was a grandfather of the memoir's author, Yousuf (Joe) Rassool.

He also had notable connections to:

* Mahatma Gandhi: Gool and Gandhi were acquainted during Gandhi's 1914 visit to Cape Town, with Gool even helping Gandhi puttying and staining the floor at his surgery.
* Dr. Abdurahman: The two men were in close proximity due to their residences being near each other, facilitating co-operation between them.

Note that these facts are based solely on the provided sources and may not be a comprehensive list of J.M.H. Gool's life and activities.

### q06 — Tell me about Buitencingle.

Buitencingle was the grand 13-room mansion of J.M.H. Gool in District Six, Cape Town [1]. It was located at No.7 Buitenkingle Street, and its name is Dutch in origin, meaning "cingle" or belt [19], as it was once the outermost belt of Cape Town.

The street where the mansion stood was wide and cobbled with traffic islands sporting stunted palm trees [1]. The mansion itself had a view of Table Mountain and Devil's Peak. Buitencingle was described as being vibrant with élan, grandeur, and joie-de-vivre at the turn of the century [12].

However, by the time the author grew up in the house, it seemed to have lost its former spirit and became a dead shell [12]. The downstairs rooms were eventually apportioned, and the author never occupied the upstairs rooms, finding them too creepy [11].

The author's grandfather had lived in the house with his two wives, who had trained themselves to stop arguing whenever he entered the room and instead cooed melodiously [9]. The house was staffed by servants without whom Buitencingle's umpteen rooms would have been unmanageable [10] [9].

The author also mentioned that their grandfather had tried to hit his son Aboeta, but due to frailty, it became a farce [13].

### q07 — Who is the author's wife?

According to [1] Nazima Rassool is the author's wife. She is also mentioned in [2], [12], [18], and [20]. 

Here are the specific facts about her from the excerpts:

* [1]: Also known as: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool. Married to Yousuf Rassool.
* [2]: The author owes a great deal to his wife, Nazima, who insisted that he should not rely merely on his own memories, but should do some research in order to lend authenticity to the story.
* [18]: The result, I submit, produced some staggering information. ... I owe a great deal to my wife, who is an academic researcher, and who urged me to ground my writing in background investigation.
* [20] is not directly related to her.

From these excerpts, we know that Nazima Rassool is the author's wife, a professor, and an academic researcher.

### q08 — Tell me more about the author's wife.

The provided sources contain information about the author's wife, Nazima Rassool, and her role in his writing process. Here are the specific facts mentioned:

* [1] mentions that Nazima is married to Yousuf (Joe) Rassool, the author.
* [9] states that Nazima is an academic researcher who urged the author to ground his writing in background investigation.
* [17] mentions that Nazima insisted the author should not rely solely on his own memories but do research to lend authenticity to the story.

The sources do not provide detailed information about Nazima's personal life, family, or background.

### q09 — Who was the author's grandfather?

The provided sources mention that the author's (Yousuf or Joe Rassool) grandfather was Haji Joosub Maulvi Hamid Gool, also known as J.M.H. Gool. [1], [2], [3] He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India. He was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.

### q10 — Tell me about Kloof Nek.

According to the sources [2], [4], and [17]-[20], here are some specific facts about Kloof Nek:

* It is a connecting saddle of land that joins Table Mountain to Lion's Head ([4] and [20]).
* It was accessible by bus from Adderley Street outside Fletchers & Cartwrights, a department store in Cape Town ([20]).
* Kloof Nek had gardens and was a scenic area with a "haunting beauty" ([18] and [17]).
* The Glen, which is also referred to as Kloof Nek, was a popular spot for study and relaxation, especially for students preparing for exams ([5], [7], and [17]).
* It was a place where people would go to escape the hustle and bustle of city life, and some even took refuge in its cavelets and bushes ([5]).

Note that while Kloof Nek is mentioned in various contexts throughout the excerpts, there isn't much information about its specific history or significance beyond being a scenic area.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers [1]. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1].

The TLSA had branches across South Africa, including one at Chapel Street Primary School, where the author was a teacher [3]. The organisation's conferences were considered important civic occasions, attended by mayors and featuring banquets [2][20].

One of its key goals was to fight for full democratic rights for non-citizens in South Africa. It is mentioned that members of the TLSA, such as Solly Edross and Ben Kies, were involved in trying to teach patriotism in schools, but their methods may have been ineffective or even counterproductive [7].

The TLSA had a Journal where teachers received training in political/educational composition and published articles [8]. Members of the TLSA included Danny De Beer, who was a PE teacher with a degree from the University of South Africa [10], and several other notable individuals.

However, as noted in sources [11] and [19], the TLSA's leadership underwent significant changes. The young Turks of the New Era Fellowship and the Teachers' League of South Africa were involved in ousting the old guard, including the Goldings, Van der Rosses, Quints, Sonns, and others, from control of the organisation.

The TLSA also developed strategies to collaborate with other organisations, such as the Cape African Teachers' Association, to prepare for potential government actions against Coloured and Indian communities [17].

### q12 — Who was Cissie Gool?

Cissie Gool (also known as Zainunnisa Gool or Zainunnissa) was a prominent Coloured woman in South African political life, daughter of Dr. Abdulla Abdurahman [1]. She was a member of the Non-European Unity Movement and an anti-apartheid activist. She served as a Cape Town city councillor, was a lawyer, and was married to Abdul Hamid Gool.

Cissie Gool was born on February 14, 1897, and died on December 2, 1963 [1]. 

She met luminaries in art, music, and politics, including Fred Bodmer, Gregoire Boonzaaier, Terence Macaw, Ben Kies, Paul Kostin, Wolf Kiebel, J.G. Tabata, Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdulla Abdurahman, Leah Solomon, and Zaid Gamiet [4].

She had a complex personality, as evidenced by her involvement in the Anti-CAD Movement and her later support for measures to deal with 'influx control' and street gangs, which shocked some of her acquaintances [3, 16]. 

Cissie Gool was known to have been involved in the Liberation League, where she called a massive demonstration on the Grand Parade to rally against residential segregation [18].

She had four daughters: Ayesha "Lallie" Gool, Fatima "Timmie" Gool, Zohra "Jolly" Gool, and Mariam "Margie" Gool. Her husband was Abdul Hamid Gool.

### q13 — What was the All Africa Convention?

The All-African Convention (AAC) [1] was an organization founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organizations seeking unity against racial oppression. The AAC later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

### q15 — What were the forced removals from District Six?

Unfortunately, the provided sources do not contain specific information about the forced removals from District Six. However, based on general knowledge, it is known that the forced removals of the residents of District Six occurred in 1968 as part of the apartheid regime's efforts to relocate non-whites and create a "white" area. The community was forcibly removed and relocated to new townships such as Cape Flats.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi's full name is not explicitly mentioned in the provided sources. However, [2] mentions "Dr. Afr." as possibly referring to Dr. Abdul Gool, but it appears that the author actually refers to Mahatma Gandhi throughout the excerpts.

The connection between Gandhi and the Gool family can be summarized as follows:

- Gandhi stayed at the Gool mansion at 7 Buitencingle Street in Cape Town on multiple occasions, including in February 1914 ([1] and [20]).
- The Gool family, particularly Dr. Abdul Hamid Gool (also referred to as "Uncle Doctor" by some family members), provided hospitality to Gandhi during these visits.
- Dr. Abdul Hamid Gool was a medical professional who repaid Gandhi with attentive medical care and wrote articles for Indian Opinion ([10] and [13]).
- Gandhi's interactions with the Gool family extended beyond just hospitality; they also participated in various community events, such as attending meetings of the Coloured People’s Vigilant Committee on May 3, 1906 ([15]).
- The relationship between Gandhi and the Gools was significant enough that Gandhi mentioned Dr. Abdul Hamid Gool's contributions to the Cape British Indian Union, a group he helped establish in 1911 ([6] and [11]).

### q17 — What was Hewat Training College?

Hewat Training College is a college where the author attended for teacher training. It trained teachers for the upper years of the primary division (source [6]). The college had an undeveloped site, possibly due to being previously used as a Whites-only Primary school and later being closed (sources [2], [3], [4]).

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community.

[1]

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain detailed information about the formation and goals of the Non-European Unity Movement (NEUM). However, according to [1], the NEUM was founded in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism. Its programme was based on the Ten Point plan for equal rights. The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions.

It is also mentioned in [8] that the founders of the NEUM were known as the "young Turks," among whom Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, were moving spirits. The NEUM is described in [10] as an organisation that brought the principle of non-Collaboration, the refutation of “race”, and the ideal of a non-racial society to the struggle for liberation.

The movement's influence is also discussed in [11], where it is mentioned that its ideals would not be achieved as expected due to changing circumstances. The movement's impact on education is noted in [18] with the Teachers' League joining the NEUM and acknowledging the connection between equal education and a democratic society.

Overall, while there are no comprehensive details about the formation and goals of the Non-European Unity Movement, it appears that it was an anti-apartheid movement aiming to unite non-white people against colonialism and apartheid, promoting principles such as non-collaboration, anti-racism, and a non-racial society.

### q20 — Describe the author's involvement in cricket.

Based on the provided sources [1]–[20], here is a comprehensive description of the author's involvement in cricket:

**Early Involvement**: The author was involved in cricket from an early age and was part of the Kismets Cricket Club. He mentions that he was selected to play for the Western Province Indian Cricket Union team, but unfortunately, no selector had seen his innings at Rosmead (Source [13]).

**Development as a Cricketer**: The author's development as a cricketer is evident from his descriptions of his experiences. He began playing cricket seriously with the help of Hassen Mall and eventually developed skills in batting, including mastering the forward defensive stroke (Sources [7] & [9]).

**Competitions**: The author participated in various cricket competitions, including the biennial tournament (Source [3]). He was part of a team that played against the West Indian Cricket team to South Africa with players like Worrel, Weekes, and Walcott (Source [14]).

**Cricket Union**: The author was involved in organizing cricket matches at High School and College level but was thwarted by the entrenched bureaucracy of the Schools' Sports Union (Source [10]). He also participated in the Cricket Tournament of their Union that took place in Cape Town for the first time, where he was happy to be chosen as one of the fifteen representatives from the Western Province (Source [18]).

**Cricketing Skills**: The author's cricketing skills are evident from his descriptions. He made an "oh so glorious" 12 runs and was hailed as a hero in one match (Source [13]). He also developed a strong forward defensive stroke, which was praised by Edross, the permanent secretary of the Barnato Cricket Board (Source [17]).

**Cricket and Politics**: The author's involvement in cricket is intertwined with politics. He often mentions how cricket was used as a tool to distract from the harsh realities of apartheid rule (Sources [14] & [15]). He also reflects on how cricket provided an escape from the trauma and suffering caused by the forced removals and discrimination faced by the Indian community.

**Legacy**: The author's involvement in cricket has left a lasting impact. He was instrumental in organizing cricket matches at High School and College level, which helped to bring about non-racialism in the sport (Source [5]). His contributions to the Cricket Union also strengthened its organizational expertise and led to the participation of teams from various communities.

The provided sources paint a vivid picture of the author's involvement in cricket, showcasing his skills, experiences, and reflections on the sport.

### q21 — Who was the author's mother?

The provided sources do not directly mention the name of the author's mother. However, [1] mentions that Ayesha Rassool (also known as Lallie) is the daughter of Bibi Gool and Haji Joosub Maulvi Hamid Gool, which suggests that Bibi Gool might be the author's grandmother. 

In [17], Malick Rassool's marriage certificate is mentioned, but it does not provide information about the name or identity of his wife. However, [18] mentions Dija, who is described as a beauty and the daughter of Mr. Gool, who helped found a school. It seems that Dija might be the author's mother.

To clarify, we need to examine every numbered excerpt again: Ayesha Rassool (the author) is mentioned in [1] as a child of Bibi Gool and Haji Joosub Maulvi Hamid Gool; however, it is not explicitly stated that Bibi Gool is the mother. In [18], Dija is described as a daughter of Mr. Gool. Since Dija's appearance in [18] does not provide any connections to Bibi Gool or Haji Joosub Maulvi Hamid Gool, and considering that [17] might be referring to Malick Rassool (Joe Rassool) and his wife being mentioned separately from Ayesha Rassool (the author), the text in [18] suggests Dija as the mother.

List of source-cited facts for this answer:

1. The author is Joe Rassool [9].
2. Bibi Gool might be the grandmother [1].
3. Dija is described as a beauty and the daughter of Mr. Gool, who helped found a school in [18].

Please note that some assumptions are made to clarify the information provided in the sources; however, there may still be missing connections or details that cannot be inferred from the provided excerpts.

### q22 — Who was the author's father?

The author's father is Peter Alexander Rassool [1]. He was also known as Peerbhai or Peru [9].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool [1]
2. Abdul Rassool [1]
3. Berina Rassool [1]
4. Nasim Rassool [17]
5. Zain Rassool [1]
6. Fazil Rassool [19]
7. Yasmin Rassool (also known as Jessie) [18, 19]

Note that the list may not be exhaustive, but it includes all siblings mentioned in the provided sources.

### q24 — Who were the children of J.M.H. Gool?

According to [1] and [12], the children of J.M.H. Gool are:

1. Abdul Hamid Gool (by Wahida)
2. Ayesha Rassool (by Bibi) 
3. Mariam Margie Gool
4. Minnie Gool
5. Zobeida Gool
6. Goolam Gool
7. Fatima Timmie Gool (also known as Fatima Gool)
8. Mohamed Saaid Gool
9. Mohammed Hanief Gool

Additionally, [12] mentions that Abdul Hamid Gool was married to Cissie Gool and had an adopted sister Toetie Noor.

Note: The provided sources do not give a complete list of all children of J.M.H. Gool, but it mentions some of them.

### q25 — Who was I.B. Tabata?

I.B. Tabata is mentioned in several excerpts as a prominent figure in the Unity Movement and the New Era Fellowship. Here are the specific facts about him that can be gathered from the sources:

* He was a leader of the NEF (New Era Fellowship) [9].
* He completed his book "The Awakening of a People" after becoming chairman of the NEF [3, 19].
* The book recorded the coming into existence of the All African Convention, which was one of the pillars of the Non-European Unity Movement [19].
* He gave a lecture at the NEF on "One Day in the Life of Ivan Denisovich", which made a tremendous impression on the author [14].
* Tabata was behind the formation of the Cape Peninsula Students Union [15].
* He was involved in a split within the movement, with some members supporting him and others Kies [4, 5, 17].
* The Forum Club, who regarded themselves as Marxist Trotskyists, condemned "The Awakening" for failing to have a class analysis and accused Tabata of being a Black Nationalist [18].

Overall, I.B. Tabata was an important figure in the Unity Movement and the New Era Fellowship, known for his leadership and intellectual contributions to the movement.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman was a prominent Cape Town doctor, city councillor, and leader of the African Political (later People's) Organisation (APO). He was the first Coloured person elected to the Cape Town City Council [1]. His father's name is not mentioned in any of the excerpts.

He had a grandfather who was brought to South Africa as a slave and bought his freedom [4]. Dr. Abdurahman's mother, Gadija Dollie, was a member of the Dollie clan and considered one of Cape Town's great beauties [4].

Dr. Abdurahman was married to Helen "Nellie" Potter James in Glasgow University, where he also met her [18]. He qualified as a doctor from Glasgow University in 1893 [18].

He was associated with the Non-European Unity Movement and Haji Joosub Maulvi Hamid Gool [1]. His daughter was Cissie Gool [1].

As mentioned earlier:

His mother's name is Gadija Dollie [4].
He married Helen "Nellie" Potter James in Glasgow University, where he also met her [18].
He qualified as a doctor from Glasgow University in 1893 [18].

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources describe a close connection between Mahatma Gandhi and J.M.H. Gool (also known as Joosub Maulvi Hamid Gool). They were acquaintances who corresponded with each other, and Gandhi even stayed at the Gool residence in Cape Town on multiple occasions. Specifically:

* In 1914, Gandhi stayed at the Gool mansion at 7 Buitencingle Street in February (Source: [5] and [10]).
* Gandhi was a guest of Adam Gool (J.M.H.'s son) at his residence "Noor Bagh" in 1911 (Source: [11]).
* Gandhi helped to furbish J.M.H. Gool's surgery by puttying and staining the floor (Source: [6]).
* J.M.H. Gool was one of the joint secretaries of the Cape British Indian Union, which Gandhi had helped to bring together as two rival organisations (Source: [6]).
* The correspondence between Gandhi and J.M.H.'s son, Abdul Hamid Gool, is mentioned in Source [3] and [19].

Their connection went beyond just acquaintanceship; the sources suggest a deep respect and appreciation for each other's work.

### q28 — Which organisations was the author involved in?

The author, Yousuf (Joe) Rassool, was involved in several organisations, including:

1. Non-European Unity Movement
2. New Era Fellowship
3. Teachers League of South Africa (TLSA)

He also mentioned that he attended a meeting where organisations such as sports bodies, churches, charities, mothers' unions, rate-payers and civics were present to be politicised and brought into the Movement.

Additionally, his family members were involved in or associated with:

1. The African People's Organisation (APO)
2. The Communist Party of South Africa
3. The Fourth International of South Africa
4. The Liberation League
5. The Workers' Party
6. Smuts's United Party (Coloured branch)

His uncle, Dr. Gool, was also involved in the politics of the Indians in Cape Town and was a joint secretary of "The Cape British Indian Union".

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not provide a detailed description of the relationship between the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement. However, we can infer that there was some connection or affiliation between the two organisations.

In excerpt [6], it is mentioned that Irwin Combrinck, a classmate of the author, tried to sell the Guardian paper but later broke with the party and became a supporter of the Ten-Point programme of the Non-European Unity Movement. This suggests that there was some overlap or shared ideology between the TLSA (mentioned in excerpt [7] as being affiliated with the Congress movement) and the Non-European Unity Movement.

In excerpt [13], it is mentioned that the Unity Movement revolutionised the political scene, and one of its pillars was the All African Convention, which was an organisation affiliated with the Non-European Unity Movement. However, this excerpt does not explicitly state the relationship between the TLSA and the Non-European Unity Movement.

It's worth noting that in excerpt [18], it is mentioned that after the League took "the new road" (i.e., broke with its previous stance of non-political professionalism), they joined the Non-European Unity Movement. This suggests that there was some level of cooperation or affiliation between the TLSA and the Non-European Unity Movement, but the exact nature of this relationship is not specified.

In summary, while the sources do not provide a detailed description of the relationship between the TLSA and the Non-European Unity Movement, they suggest that there was some overlap or shared ideology between the two organisations.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain information about when J.M.H. Gool arrived in Cape Town or his original homeland/ancestry. However, it is mentioned that his son was born in 1886 (Excerpt [1]), but the sources do not directly state when J.M.H. Gool himself arrived.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile [1]. It was completed in 1898 and was founded by the grandfather of the author. The mosque had a domed minaret, topped by a spire with a moon and star, from which the muezzin called the faithful to prayer (not too stridently for fear of annoying the European residents!) [20].

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) had two wives: Bibi Gool and Wahida Gool. 

[4] states that Dr. Gool was the half-uncle of the author, Yousuf (Joe) Rassool. This relationship is due to the fact that J.M.H. Gool had two wives.

However, it seems there might be an inconsistency in [1] and other excerpts regarding Cissie Gool's relationship with J.M.H. Gool. [6] states that Dr. A.H. Gool (not mentioned as being related to J.M.H. Gool) says Indians are right in not tying up their cause with that of the coloured, which suggests he might be married to someone who is a strong advocate for the rights of coloured people.

However, [4], [6] and other excerpts imply that Cissie Gool was likely one of J.M.H. Gool's wives (Bibi or Wahida), as she is often mentioned in conjunction with his family.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool knew Mahatma Gandhi personally. This is mentioned in [1], where it is stated that "He was associated with early Cape Muslim and Indian political circles" and that he "was acquainted with Mahatma Gandhi during Gandhi's 1914 visit."

Additionally, [13] mentions that Dr. Abdurahman's grandfather was brought to South Africa as a slave, and that Gool and Abdurahman were related through marriage (Gool's second wife, Wahida Ta'al, had a mother who was a Dollie, which makes her the aunt of Abdurahman). However, this is not an explicit statement that they knew each other personally.

[3] mentions Mr. Wilson, but it does not provide any information about their relationship or whether they knew each other personally.

The notable historical figures that J.M.H. Gool worked with or associated with include:

* Dr. Abdurahman (related through marriage)
* Mahatma Gandhi
* Adam H. G. Mahomed (a senior, financier, and possibly mentor)

It is also mentioned in [16] that J.M.H. Gool worked with a man named Mahomed, who was the President of the British Indian League. However, it does not specify whether this is the same Adam H. G. Mahomed mentioned earlier.

In [17], J.M.H. Gool attended a meeting with Wilson, F Gow, James Currey, and Rev R.A. Jackson, but there is no information about their personal relationships.

Gool was also involved in activities related to the Indian Settlers' community and worked on petitions and advocacy efforts with other notable figures such as Mahomed.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act, also known as the Group Areas Proclamation, was a law passed in South Africa in 1950 that aimed to segregate people of different racial groups into specific areas. The law allowed the government to reclassify existing urban areas and establish new "group areas" for various ethnic groups.

In the case of District Six, which was a thriving cosmopolitan area in Cape Town, the Group Areas Act led to the forced removal of thousands of people from their homes. Under the law, the government designated District Six as an "Indian" group area, despite its diverse population that included people of various racial and ethnic backgrounds.

The residents of District Six were given six months to leave their homes, but many were unable to find alternative accommodation in other areas. The forced removals had a devastating impact on the community, causing significant social and economic disruption.

According to Excerpt [2], "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Excerpt [4] provides more details on how the government built new townships to entice people away from District Six: "But once the exit began they started to build the sub-economic schemes, which, ironically, sported the flashy name like 'Silvertown'. Soon other townships were erected to entice the people away from District Six: places like Bonteheuwel, Hanover Park, Mannenberg, Mitchell’s Plain, Vanguard Estate, and Bishop Lavis."

The Group Areas Act had a profound impact on the community of District Six, leading to widespread displacement and disruption. The law was used as a tool for apartheid-era segregation and oppression, causing significant harm to the residents of the area.

### q35 — Who was Hassen Mall?

Hassen Mall was a brilliant cricketer and lawyer who qualified with a degree in BA LLB [16]. He was also known for his magnetic personality, clipped brush moustache, and talent for spinning a wristy leg break [16].

According to the sources, he:

* Was a student at the University of Cape Town where he was doing medicine, but later switched to Law [17].
* Had a significant impact on Hassen Mall's life in the years that followed [18].
* Left an indelible imprint of his personality when he departed for Durban in 1951 [16].
* Was involved in cricket and served as the captain of a team [2, 5].
* Was related to Malick Hayat, as Mylie Begg's brother-in-law was Hamid Khan, who was Hassen Mall's relative [13].

Hassen Mall played an important role in Hassen Mall's life and cricket career. He also seemed to be someone with a strong personality and abilities that left a lasting impression on those around him.

Sources: [2], [5], [16], [17]

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The following organisations were mentioned as being active in the Cape Coloured community:

1. The New Era Fellowship
2. The Teachers' League of South Africa (TLSA)
3. The Liberation League
4. The Workers' Party
5. The Communist Party of South Africa
6. The Fourth International of South Africa
7. The African People's Organisation (APO)

Additionally, the Coloured branch of Smuts's United Party was mentioned as being active.

It appears that these organisations were involved in various aspects of politics and activism within the Cape Coloured community during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

The provided sources do not contain information about Gandhi's ancestry or his life before coming to South Africa.

However, it can be inferred that [2] provides some information about Gandhi's title "Mahatma". It is mentioned that the term was used a few months prior to August 1914 in an article published in Indian Opinion and was also reprinted by Ramsey MacDonald. However, the sources do not provide any further biographical details.

According to [16], Gandhi came to South Africa on a one-year assignment from Dada Abdullah in 1893 to assist his attorney in a civil suit against a merchant in Pretoria. The accounts were kept in Gujarati, which was Gandhi's mother tongue.

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information about Cissie Gool's father.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources, District Six was a vibrant and multicultural community near the centre of Cape Town. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1]. The author describes it as having "more life, more zest, more fire, more warmth and care than many other places one could mention" [3].

The neighbourhood was known for its political struggle against apartheid, but it was also a place where people from different cultures and backgrounds lived together. The author mentions that the community had a rich history of politics, with some residents being part of a pre-Garveyite, pan-Africanist group of forward thinkers who counted Gool among their cohort [4].

The area was bustling with activity, and the streets were filled with people going about their daily business. The author describes Caledon Street as "getting tattier along the way" but still having a vibrant atmosphere [3]. The community had a strong sense of identity and culture, which is reflected in the author's description of the area as "inimitable District Six" [20].

It is worth noting that the sources also mention the presence of forced removals and the impact of apartheid on the community. However, when describing District Six before the forced removals, the focus is on its vibrancy, diversity, and cultural richness.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in various excerpts. According to [11], the movement's weapon of choice was the boycott, which became a powerful tactic against apartheid.

In [4], it is stated that the NEUM adopted a boycott and non-collaboration policy - refusing to participate in any apartheid government institutions.

The policy specifically applied to dummy elections, as mentioned in [5]. The Communists Party intended to put up candidates despite the general desire among Blacks to boycott these "dummy" elections. However, the Unity Movement insisted that participation would be tantamount to collaborating with the apartheid regime.

In terms of everyday life, [8] describes how the movement's members were encouraged to boycott the Coloured Affairs Council (C.A.C.). This involved boycotting all those who accepted service on the C.A.C. in every aspect of their lives - socially, economically, and otherwise.

The issue pervaded the minds of the community, with meetings held almost every night at halls across Cape Town ([11]).

Not everyone within the movement followed this injunction to boycott, however. Some members ignored the boycott call when it came to property ownership issues related to the Group Areas Act ([8] & [16]).


  ✅ Overall: 78.4% recall (token-overlap)  (163.8/209)  avg 30749ms
