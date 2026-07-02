[2m2026-07-02T08:15:41.060526Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T08:15:41.186773Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  19190ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T08:16:03.102179Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/3 keywords  21309ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T08:16:23.995975Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  23522ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T08:16:47.160616Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  21641ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T08:17:08.981519Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  38251ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T08:17:47.578222Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  30932ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T08:18:19.283645Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  20859ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T08:18:39.540754Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  20989ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T08:19:00.627566Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/9 keywords  23828ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T08:19:24.638077Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/7 keywords  27951ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T08:19:53.244713Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  22118ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T08:20:15.168435Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  25128ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T08:20:40.512443Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  20499ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T08:21:01.186377Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  19512ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T08:21:20.325605Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 91 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""Forced relocations of residents from District Six Cape Town apartheid"
  ○ Round 3   added 20 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  33552ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T08:21:53.690412Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  25765ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T08:22:19.659103Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  21021ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T08:22:40.882320Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23619ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T08:23:04.707113Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23521ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T08:23:28.481865Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  24184ms
  [21/40] Who was the author's mother?
[2m2026-07-02T08:23:54.104241Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/5 keywords  22397ms
  [22/40] Who was the author's father?
[2m2026-07-02T08:24:15.819913Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/4 keywords  20590ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T08:24:37.274295Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  24797ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T08:25:01.868014Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  28923ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T08:25:33.057209Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27582ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T08:26:04.151988Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  33546ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T08:26:33.683076Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  28410ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T08:27:01.708146Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  25564ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T08:27:28.033007Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  23851ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T08:27:51.268165Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/6 keywords  27719ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T08:28:36.912942Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  48174ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T08:29:49.485284Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  67937ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T08:30:39.818417Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  54474ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T08:31:23.171666Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  42135ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T08:32:11.601531Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  48307ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T08:33:01.402114Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  51747ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T08:33:49.777423Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  43150ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T08:34:45.828353Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/5 keywords  52318ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T08:35:22.661450Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  40056ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T08:36:15.603351Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  51371ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 77.5% (162.0/209) |
| Avg latency | 31260ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 19190ms |
| q02 | Who are the author's children? | 0/3 (0%) | LEST WE FORGET -rev25.pdf | 21309ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23522ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 21641ms |
| q05 | Who was J.M.H. Gool? | 7/8 (88%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 38251ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 30932ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 20859ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 20989ms |
| q09 | Who was the author's grandfather? | 7/9 (78%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 23828ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | LEST WE FORGET -rev25.pdf | 27951ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 22118ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 25128ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 20499ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 19512ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, sequence_diagram:District Six, [Graph: District Six] | 33552ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 25765ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 21021ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 23619ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 23521ms |
| q20 | Describe the author's involvement in cricket. | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 24184ms |
| q21 | Who was the author's mother? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 22397ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 20590ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 24797ms |
| q24 | Who were the children of J.M.H. Gool? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28923ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 27582ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 33546ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28410ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 25564ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/4 (50%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 23851ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool | 27719ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 48174ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 67937ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 54474ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 42135ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 48307ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 51747ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 43150ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 52318ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 40056ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 51371ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1], also known as Joe Rassool, Yusuf Rassool, etc.

### q02 — Who are the author's children?

The provided sources do not contain that information.

### q03 — Who are the author's grandchildren?

The provided sources do not explicitly list all of the author's grandchildren. However, based on [1] and [2], the following names are mentioned as the grandchildren:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool

These names appear in the dedication of the book, which states "For my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis."

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [5]

### q05 — Who was J.M.H. Gool?

According to [1], Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) was a person of striking good looks, tall, and proud. He was born in Swat (via Mauritius), India. His father was a Pushto-speaking Maulvi (religious leader) who moved from Swat to Rander in Gujarat with his wife and two sons, Abbas and Yusuf. The Gool family spoke Urdu, but Gujarati was generally the language spoken in that part of India.

J.M.H. Gool was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six. He lived at 7 Buitencingle Street in Cape Town, where he received notable visitors including Mahatma Gandhi (during Gandhi's 1914 South Africa farewell visit), Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman.

He had two wives: Bibi Gool and Wahida Gool. With Bibi, he had children, including Ayesha Rassool (the author of the memoir). With Wahida, he had children, including Abdul Hamid Gool. His son Abdul Hamid Gool married Zainunnissa (Cissie) Gool, daughter of Dr. Abdulla Abdurahman.

J.M.H. Gool performed the Hajj in 1899 and died on April 1, 1940. He was a prominent figure in early Cape Muslim and Indian politics, associated with organizations such as the British Indian League and the Coloured People's Vigilant Committee.

The following specific facts can be listed for J.M.H. Gool:

* Born in Swat (via Mauritius), India
* Father: Pushto-speaking Maulvi from Swat
* Spouse 1: Bibi Gool (married in 1879 or 1880)
* Spouse 2: Wahida Gool (mother of Abdul Hamid Gool)
* Children:
	+ With Bibi: Ayesha Rassool (author's mother), possibly others listed in [1]
	+ With Wahida: Abdul Hamid Gool, possibly others
* Community leader and founder trustee of Hanaffi Quwatul Islam Mosque
* Lived at 7 Buitencingle Street in Cape Town
* Received notable visitors including Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman
* Performed Hajj in 1899
* Died on April 1, 1940

### q06 — Tell me about Buitencingle.

Buitencingle is mentioned in various excerpts throughout the sources [1]–[20]. Here's a comprehensive list of specific facts about Buitencingle based on these excerpts:

**Name and Pronunciation:** Buitencingle was also written as No. 7 Buitenkingle Street or Buitenkingle, with some variations in spelling (e.g., Baytencingle). The "cingle" referred to a belt.

**Location:** Buitencingle was the outermost belt of Cape Town at that time, situated near Table Mountain and Devil's Peak. Beyond it lay the foothills of Table Mountain, accessible via Kloof Street.

**Layout and Description:**

* It had 13 rooms (Excerpt [11])
* The street was wide and cobbled with traffic islands in the middle (Excerpt [2])
* There were stunted palm trees on these islands (Excerpt [2])
* The mansion at No. 7 Buitencingle Street had a view of Table Mountain and Devil's Peak (Excerpt [1])

**History:**

* J.M.H. Gool, the owner of the grand mansion at No. 7 Buitencingle Street, was a prosperous merchant who lived there with his family.
* His son, Haji Joosub Maulvi Hamid Gooli, married a Malay girl in Cape Town (Excerpt [17])
* The Gools hosted meetings and gatherings, including one where they received Prime Minister Louis Botha at their residence (Excerpt [11])

**Cultural Significance:**

* Eid celebrations were held at Buitencingle (Excerpt [6])
* My grandfather instituted a custom of giving pennies to children who came to give Eid greetings (Excerpt [6])
* The house had a "sovereign effect" on the growth and development of its residents, particularly Baby Ismail (Excerpt [13])

**Demolition:**

* Unfortunately, the Gool mansion in District Six is no longer standing.

These are all the specific details I was able to gather about Buitencingle from the provided sources.

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

Based on the provided sources, here are the specific facts known about the author's wife:

1. Her name is Nazima Rassool [1].
2. She is an academic researcher [15], [19].
3. She played a central role in shaping the memoir by urging the author to ground his personal memories in historical research [1].
4. The author owes a great deal to her, who insisted that he should not rely merely on his own memories but do some research to lend authenticity to the story [4], [19].

The provided sources do not contain information about the author's wife beyond these specific facts.

### q09 — Who was the author's grandfather?

The author's grandfather was Haji Joosub Maulvi Hamid Gool (also referred to as J.M.H. Gool). 

Specific facts about the author's grandfather, cited from [1]–[20], include:
- Born in Swat via Gujarat
- Lived in Mauritius and later Cape Town, arriving there in 1884
- Founder of the Hanaffi Quwatul Islam Mosque in District Six
- Married to two women: Bibi Gool (who was from India) and Wahida Ta’al
- Grandfather of the author Yousuf (Joe) Rassool

### q10 — Tell me about Kloof Nek.

According to the sources:

* Kloof Nek was a place of haunting beauty [3].
* It was a woodland dell [4] where the author and his friend read poetry and felt entranced by its beauty.
* Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion's Head [2].
* It was a popular spot for lovers, mountaineers, nature lovers, and ramblers [17].
* The area had various cavelets, culverts, and bushes where down-and-outs (known as "Bergies") would drink cheap wine and methylated spirits.
* Kloof Nek sprawled protectively on the eastern flank of the city with its head gazing inscrutably down on Table Bay [18].
* It was a popular spot for hiking, and those who found the slogs up Platteklip Gorge, Slangolie or Woody too onerous could use the Table Mountain cable car to ascend the mountain.

Overall, Kloof Nek seems to be a beautiful and scenic area with a mix of natural beauty and social activity.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1].

### q12 — Who was Cissie Gool?

Cissie Gool (also known as Zainunnisa Cissie Gool) was a prominent Coloured woman in South African political life. She was:

* Born on February 14, 1897 [1]
* Daughter of Dr. Abdulla Abdurahman [2]
* Member of the Non-European Unity Movement [1]
* Wife of Abdul Hamid Gool and mother of Shaheen Gool [1]
* Died on December 2, 1963 [1]

She was a Cape Town city councillor, lawyer, and political activist who played a significant role in the anti-apartheid struggle.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was an organisation that united a broad coalition of African political organisations seeking unity against racial oppression [1]. It was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights [1].

This information is present in excerpt [1] and [19].

### q14 — Where was District Six and what kind of place was it?

According to [1], District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

Source: [1]

### q15 — What were the forced removals from District Six?

The provided sources do not directly address what the forced removals from District Six specifically entailed or what they involved. They do, however, provide a historical context of the area and its inhabitants, including the Gool family, who lived there.

Source [2] mentions that "Barrabhai and his Nurjehan fell in love with the place and settled there" in The Strand (about thirty miles from Cape Town), but it does not discuss forced removals specifically.

Forced removals were a reality for many communities in South Africa during the apartheid era. According to general knowledge, these removals involved the relocation of people from urban areas designated as "white" or "coloured" spaces to other areas, often far from their original homes and communities, under the pretext of urban renewal or reorganization. However, since this information is not present in the provided sources, a direct answer based on the given documents cannot be offered.

To learn more about the forced removals from District Six specifically, additional research would be required that focuses on the historical context of District Six and its inhabitants during the apartheid era.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi (1869-1948) was an Indian independence activist who played a key role in leading India's struggle for independence from British rule. He is widely regarded as one of the most influential leaders of the 20th century.

The connection between Gandhi and the Gool family is significant, particularly with Dr. Yusuf Hamid Gool, a doctor and member of the Gool family. According to the sources:

* The Gool family was associated with Gandhi's stay in South Africa (Source [3]). 
* Dr. Yusuf Hamid Gool arranged for meals at his place when Gandhi arrived in Cape Town (Source [18]).
* The Gool family provided hospitality to Gandhis in February 1914, which led to a friendship between Joe Rassool and the author of this memoir (Source [19]).
* Gandhi corresponded with Dr. Yusuf Hamid Gool, as evidenced by letters mentioned in Source [16] and Source [20].
* Gandhi's family stayed at the Gool mansion at 7 Buitencingle, where they were received by Emily Hobhouse and other European women (Source [19]).

Gandhi also interacted with other members of the Gool family, including Amod Gool and Adam Haji Gool Mahomed (Source [11]).

### q17 — What was Hewat Training College?

Hewat Training College is described in [2] as a school for training teachers. It had been previously known as a Whites only Primary school, but the authorities considered it too close to a prison and District Six, so they decided to use its grounds for teacher training instead ([3]). The college trained teachers for the upper years of the primary division ([6]).

According to [20], when Yousuf (Joe) Rassool entered Hewat Training College in January 1947, it had become a Post Matriculation Teacher Training facility for Coloureds, opening in 1944 with "considerable fanfare" ([3]). The staff at the college was recruited from the United Kingdom and chosen to ensure that the college did not become an incubator of left-wing radical ideas like Livingston and Trafalgar High schools [20].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community.

[1] mentions that it was "a cultural organisation that had been established in 1937 'to discuss everything under the sun'".

Additionally, [3] states that it issued a clarion call to all Non-European organisations to meet to counter the predicted assault on the remaining rights of the "Coloured" community.

[12] calls the NEF a "gathering place for progressive intellectuals, teachers, and activists in the Coloured community", indicating its role as a hub for intellectual and activist activities among the Coloured community.

### q19 — What was the Non-European Unity Movement?

The provided sources describe the Non-European Unity Movement (NEUM) as an organisation that was founded in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism. According to [1], its programme was based on the Ten Point plan for equal rights. The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

[8] mentions that Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, were among the moving spirits of the Non-European Unity Movement when it was founded in 1943. [9] states that this movement "revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon" and galvanised the non-white disenfranchised to a realisation of the need for a programme-based struggle.

[14] describes the NEUM as bringing the principle of non-Collaboration, the refutation of "race," and the ideal of a non-racial society to the struggle for liberation.

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool, was actively involved in cricket through Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in Cape Town.

He mentions being a member of various teams, including Kismets, where he met some friends like Hassen Mall. They were part of the Western Province Indian Cricket Union team that was chosen to play against the West Indians in 1947 [9].

The author also describes his own cricketing skills and experience. He began learning to play cricket seriously under the guidance of Hassen Mall, but mentions being a "kebab" (a term used by the Gujarati community) or an absolute beginner, indicating he was not yet skilled at playing the game [7]. 

He further shares that one of his innings at Rosmead scored several eighties against certain players' names in their scorebooks. He was known to be aggressive as a batsman and gentle as a person with a great sense of humour [6].

The author also tried to organize a union to facilitate the playing of cricket at High School and College level but was thwarted by the entrenched bureaucracy of the Schools’ Sports Union [11].

### q21 — Who was the author's mother?

According to [5] and [7], the author's mother was Wahida. 

The provided sources do not contain detailed biographical information about Wahida, but they mention that she:

* Was a beauty with fair skin and dark hair (Source: [1])
* Had several children (Source: [2])
* Spoke English at home, despite being from an Indian family (Source: [2])
* Was pregnant with the twins Nurjehan and Zarina when the author was still very young (Source: [4])

Further details about Wahida's life are not provided in the sources.

### q22 — Who was the author's father?

The author's father was Yousuf Rassool. 

Source: [2] "My father, Yousuf Rassool, sent me text to the second edition in 2002 and I have to admit that I did not read it until this summer (2007)."

### q23 — Who were the author's siblings?

According to source [1], the author Yousuf (Joe) Rassool had several siblings:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool (also mentioned as "the fretful baby")
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool (also known as Jessie)
8. Lallie (also known as the author's mother, implying she may have been a sibling or half-sibling)

Note that source [13] mentions the children of Nurjehan and her husband Sayed Fethi, who became foster parents to the Gool children, but this is not directly related to Yousuf (Joe) Rassool's siblings.

Also, note that in source [14], Nasim has his hair cut off by their grandmother as a remedy after he was teased by girls.

### q24 — Who were the children of J.M.H. Gool?

According to excerpt [1], Haji Joosub Maulvi Hamid Gool's children are:

1. Abdul Hamid Gool (by wife Wahida)
2. Ayesha Rassool (by first wife Bibi)

From excerpt [8], the following children of J.M.H. Gool are listed:
 
1. Ayesha “Lallie” Gool
2. Cissie Gool
3. Fatima “Timmie” Gool
4. Gadija “Dija” Noor
5. Toetie Noor
6. Zohra “Jolly” Gool
7. Mariam “Margie” Gool

From excerpt [8], it is also mentioned that Gadija Dija Noor, Toetie Ally, and Hanief Ally are the foster children of J.M.H. Gool.

Excerpt [1] also mentions that there were other children with names starting with "Z" and "M", but does not provide their exact number or full names (e.g., Zobeida Gool, Fatima Gool, Mohamed Saaid Gool)

### q25 — Who was I.B. Tabata?

I.B. Tabata (also known as B.M. Tabata) was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He was married to Jane Gool-Tabata, daughter of J.M.H. Gool. Tabata was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].

The sources describe Tabata as having a significant impact on the Unity Movement through his publications, including "The Awakening of a People", which recorded the formation of the All African Convention, one of the pillars of the Non-European Unity Movement. He was also involved in debates and discussions with other leaders, such as Ben Kies [3, 13].

Tabata's political ideology was criticized by some for being too focused on nationalism, and he faced opposition from groups like The Forum Club, who regarded him as a Black Nationalist [4]. However, his supporters saw him as a dedicated leader of the Unity Movement.

The sources do not provide extensive information about Tabata's personal life or background beyond his involvement in politics.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the sources:

* Dr. Abdulla Abdurahman [Person] (Source [1])
	+ Also known as: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman.
	+ Associated with: Haji Joosub Maulvi Hamid Gool, Non-European Unity Movement.
	+ Father of: Cissie Gool.
	+ Died: 1940-02-20.
	+ Born: 1872-09-08.

Dr. Abdullah Abdurahman was a Cape Town doctor and leader of the African Political (later People's) Organisation (APO). He was the first Coloured person elected to the Cape Town City Council, and his son, Dr. A. Abdurahman's grandfather was brought to South Africa as a slave but bought his freedom and became prominent in the community.

Dr. Abdullah Abdurahman is also mentioned in several other sources ([2], [3], [4], [5], [6], [7], [9]) where he is associated with various individuals, including Gandhi, Cissie Gool, and J.M.H. Gool.

Additionally, source [19] lists him as one of the notable people who visited Buitencingle, along with other dignitaries such as the Prince of Wales, George Bernard Shaw, Sarojini Naidu, and Gandhi.

Sources:
[1], [2], [3], [4], [5], [6], [7], [9]

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources indicate a close relationship between Mahatma Gandhi and J.M.H. Gool.

Gandhi stayed at 7 Buitencingle Street, which was the residence of J.M.H. Gool, on several occasions (excerpts [10] and [11]). In 1911, Gandhi helped to furbish Dr. Gool's surgery by puttying and staining the floor (excerpt [6]).

Gandhi was also a guest at 7 Buitencingle Street in 1906 when he received J.M.H. Gool's son, Abdul Hamid Gool, who was visiting from London as a medical student (excerpts [3] and [19]). 

The sources suggest that Gandhi was impressed with the Gool family and their generosity. In fact, the correspondence between Gandhi and J.M.H. Gool was mentioned in various articles (excerpt [3]).

Furthermore, the Gool family provided hospitality to Gandhis on several occasions, including February 1914, which led to the author's friendship with Joe Rassool (excerpts [10] and [16]).

In addition, Gandhi intervened in a romantic affair between his son and Fatima Timmie Gool (excerpt [13]). 

Overall, the sources indicate that Gandhi and J.M.H. Gool had a close relationship based on mutual respect, trust, and friendship.

### q28 — Which organisations was the author involved in?

The provided sources list several organisations the author, Yousuf (Joe) Rassool, was involved with. These include:

1. New Era Fellowship (NEF)
2. Non-European Unity Movement (NEUM)
3. Teachers League of South Africa (TLSA)
4. Anti-CAD 
5. African People’s Organisation (APO)
6. Communist Party of South Africa
7. Fourth International of South Africa
8. Liberation League
9. Workers’ Party
10. The Coloured branch of Smuts’s United Party

Note: Some sources ([13] and [17]) mention sports clubs and other organisations, but they are not explicitly stated to be affiliated with the author or his activities.

Additionally, some sources imply involvement in or affiliation with:

1. The Moravian Church
2. The Trafalgar Players (amateur theatrical society)

Source citations:
[4], [9] for New Era Fellowship; 
[8] and [17] for Non-European Unity Movement;
[10] for Teachers League of South Africa; 
[14] implies involvement with Anti-CAD, but the author's exact role is not specified.
The other organisations are mentioned in [17].

Please note that some sources mention the author's connections to various groups or events without explicitly stating his involvement.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not explicitly state the nature of the relationship between the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement (NEUM). However, it can be inferred from [6] that the author was drawn to the NEUM when he began his teaching career and threw his energies into the attempt to "take a nation to school," an aphorism that aptly captured the role of the Movement during those years. 

The TLSA is mentioned as being involved in the formulation of a unified strategy with the Congress movement to counter the anti-educational plans of the regime, but it is not specified whether this was in conjunction with the NEUM or independently ([11]).

It can be noted that the TLSA did join the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society ([18]).

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain explicit information on when J.M.H. Gool (Haji Joosub Maulvi Hamid Gool) arrived in Cape Town or his origin, apart from a reference to him being brought to South Africa as a slave [2]. However, it is mentioned that he left Bombay in April or early May 1901 for Cape Town [4].

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Quwatul Islam Mosque or Loop Street Mosque, was a mosque located in Loop Street, Cape Town. It was founded in 1898 and was one of the oldest mosques in the Cape Muslim community (Source: [1]). The author's grandfather, J.M.H. Gool, was a founding trustee of the mosque and its history is central to the family's history in District Six (Source: [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was the wife of Abdul Hamid Gool (also known as Dr. A.), who is stated to be one of the sons of J.M.H. Gool [4, 13]. This makes Cissie Gool the daughter-in-law of J.M.H. Gool.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources:

* Mahatma Gandhi visited J.M.H. Gool's home at 7 Buitencingle Street, Cape Town in 1914 [1]
* Cecil John Rhodes was a notable visitor to J.M.H. Gool's home [1]
* George Bernard Shaw and Dr. Abdulla Abdurahman were also visitors to J.M.H. Gool's home [1]

Additionally, the following sources mention Gandhi being at 7 Buitencingle Street in 1911:

* Gandhi helped to furbish J.M.H. Gool's surgery by puttying and staining the floor [9]
* J.M.H. Gool became president of the Cape British Indian Union in 1911, and Gandhi was present at his home that year [9]

J.M.H. Gool also hosted Gandhi in February 1914:

* Cissie Gool (Gandhi's daughter-in-law) mentioned that Gandhi stayed with the Gools in February 1914 [10]

These sources provide a glimpse into J.M.H. Gool's personal connections with notable historical figures of his time.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not contain a comprehensive description of the Group Areas Act. However, they mention that the act resulted in the forced removals of communities from District Six to the Cape Flats (excerpts [2] and [3]).

Excerpt [2] states: "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Excerpt [3] describes how low-income housing schemes were built to entice people away from District Six: "But once the exit began they started to build the sub-economic schemes, which, ironically, sported the flashy name like 'Silvertown'."

### q35 — Who was Hassen Mall?

Hassen Mall was a close friend and fellow cricket player of the author Yousuf Rassool, from District Six in Cape Town. He was known for his exceptional cricket skills, being a brilliant cricketer who spun a wristy leg break that zipped like a circular saw [6]. He had come to study medicine at the University of Cape Town and later qualified with a degree BA LLb [6]. Hassen Mall played an important role in the author's life and was part of several cricket teams, including one captained by him for a biennial tournament in Durban. 

Specific facts about Hassen Mall:

* Occupation: Medicine student, lawyer (later)
* Location: District Six, Cape Town
* Nationality: South African
* Association with the author:
	+ Close friend and fellow cricket player [1]
	+ Introduced to the author by Muddy [3]
	+ Shared a friendship that affected the author's outlook and development [6], [17]
	+ Was part of several cricket teams, including one captained by him for a biennial tournament in Durban
* Cricket skills:
	+ Spun a wristy leg break that zipped like a circular saw [6]
	+ Batted at number four but decided to bat at number one in the mentioned tournament [17]
* Education: Qualified with a degree BA LLb [6]

Note: There is limited information about Hassen Mall's personal life outside of his relationship with the author and his cricket career.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpts [13], [15], and [20], several political organizations were active in the Cape Coloured community during the author's lifetime:

1. New Era Fellowship: A cultural organization that issued a clarion call for Non-European organizations to meet and counter the predicted assault on the remaining rights of the "Coloured" community.
2. The African People’s Organisation (A.P.O.)
3. The Moravian church
4. The Communist Party of South Africa
5. The Fourth International of South Africa
6. The Liberation League
7. The Workers’ Party
8. The Coloured branch of Smuts's United Party

These organizations were responding to the call for unity and full democratic rights, and some had been dormant or defunct but responded to the New Era Fellowship's call to action.

Additionally, excerpts [3], [6], and [18] mention other individuals who were involved in politics, including:

1. Dr. Abdurahman
2. Uncle Hanief
3. Cissie Gool
4. Aunt Jane
5. My uncle, Dr.
6. Cassim Amra (a member of the Communist Party)
7. Ismail (councillor and opponent of Cassim Amra)
8. Salie Dollie (pharmacist and member of the Coloured Advisory Council)
9. George Golding (member of the pro-government Coloured Advisory Council)
10. Van der Ross
11. Rev. Gow

These individuals and organizations played significant roles in the politics of the Cape Coloured community during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was known as Mohandas Karamchand Gandhi. He came to South Africa in 1893 as a lawyer to work on a legal case [1]. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance, or passive resistance) as a tool of political struggle [1].

Gandhi was connected to the Gool family in Cape Town, staying at their home at 7 Buitencingle Street during his 1914 South Africa farewell visit [1]. He also had close relationships with prominent individuals such as Dr. Abdulla Abdurahman and Haji Joosub Maulvi Hamid Gool [1].

Gandhi's work in South Africa involved advocating for the rights of Indian immigrants, including the right to vote, equal employment opportunities, and an end to discriminatory legislation [16]. He was particularly opposed to the Transvaal Asiatic Act of 1907, which restricted the rights of Indians in the region [16].

Gandhi returned to India in 1914 after more than 20 years in South Africa.

### q38 — Who was Cissie Gool's father?

Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources [1] and [18], District Six was a vibrant, multicultural community near the centre of Cape Town. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

Source [4] mentions that in Victorian times, there was a community of freed slaves from America and the West Indies thriving in District Six, some of whom achieved great position and wealth. However, their expulsion at the turn of the century foreshadowed the Apartheid era evictions of the same neighbourhood decades later.

Source [18] describes District Six as having "more life, more zest, more fire, more warmth and care than many other places one could mention." It was a place where people lived in close proximity to each other, with Caledon Street running from Corporation Street straight through to Clifton Street, getting increasingly run-down along the way.

Source [19] notes that before the forced removals, District Six had already seen the displacement of its residents, with government "economic housing schemes" being built in the suburbs to entice people away from the area.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in multiple excerpts. 

According to [1], the Non-European Unity Movement (NEUM) adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

[2] explains that Jadwat and Seedat disagreed with this policy, suggesting it was better to have "our man" inside promoting "our point of view" than have the seat occupied by a government stooge. However, this does not represent the Unity Movement's official boycott policy.

[4] describes the NEUM as adopting the boycott weapon as part of its struggle against apartheid. 

[11] states that all those who accepted service on the C.A.C. were boycotted in every aspect of their lives, socially, economically and otherwise.

The boycott was meant to be a principled programme of demands, [12] notes, which is supported by [5], where it is mentioned that some members simply ignored the injunction to boycott.

It seems that the Unity Movement's boycott policy was not always successful or consistently implemented. For instance, [8] mentions that the boycott was effective in paralysing the Coloured Affairs Council but ineffective against Apartheid in general.


  ✅ Overall: 77.5% recall (token-overlap)  (162.0/209)  avg 31260ms
