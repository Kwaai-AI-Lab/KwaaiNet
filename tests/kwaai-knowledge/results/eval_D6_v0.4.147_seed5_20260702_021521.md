[2m2026-07-02T09:15:24.243425Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T09:15:24.367535Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  24200ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T09:15:48.797023Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/3 keywords  20916ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T09:16:09.914636Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  19655ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T09:16:33.977456Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  24487ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T09:16:57.425431Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  33896ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T09:17:30.496055Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  29024ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T09:17:59.321481Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  20572ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T09:18:19.988250Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  24407ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T09:18:44.677671Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/9 keywords  28621ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T09:19:13.489293Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  28329ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T09:19:42.015202Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  27888ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T09:20:09.929314Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  28821ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T09:20:38.979549Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  23797ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T09:21:03.005261Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  20152ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T09:21:23.272005Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 89 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced removals history South Africa apartheid era""
  ○ Round 3   added 22 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  32728ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T09:21:55.035912Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  32574ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T09:22:27.781212Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  18428ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T09:22:46.415407Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  26842ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T09:23:13.592424Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  20840ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T09:23:40.059484Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  32268ms
  [21/40] Who was the author's mother?
[2m2026-07-02T09:24:10.726639Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/5 keywords  23702ms
  [22/40] Who was the author's father?
[2m2026-07-02T09:24:32.962714Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  21492ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T09:24:54.096048Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  23200ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T09:25:17.554867Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  30539ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T09:25:48.230612Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  26220ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T09:26:14.897802Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  27081ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T09:26:42.098196Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  27951ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T09:27:10.440170Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  22730ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T09:27:33.115536Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  24802ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T09:28:04.235922Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0.2/6 keywords  35040ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T09:28:36.647622Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  32632ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T09:29:06.998071Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  29044ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T09:29:34.940083Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  30670ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T09:30:05.765743Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  31959ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T09:30:38.091251Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  29648ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T09:31:07.733278Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23591ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T09:31:31.492006Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  25149ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T09:31:56.917460Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/5 keywords  21793ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T09:32:18.293198Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  31553ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T09:32:50.801752Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27828ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 87.2% (182.2/209) |
| Avg latency | 26626ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 24200ms |
| q02 | Who are the author's children? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 20916ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 19655ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 24487ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 33896ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 29024ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 20572ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 24407ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 28621ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 28329ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 27888ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 28821ms |
| q13 | What was the All Africa Convention? | 4/5 (80%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 23797ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 20152ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | sequence_diagram:District Six, [Graph: District Six], LEST WE FORGET -rev25.pdf | 32728ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Gool family] | 32574ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 18428ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 26842ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 20840ms |
| q20 | Describe the author's involvement in cricket. | 4/5 (80%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 32268ms |
| q21 | Who was the author's mother? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 23702ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 21492ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23200ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 30539ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 26220ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 27081ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 27951ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 22730ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 24802ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0.2/6 (3%) | [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool, LEST WE FORGET -rev25.pdf | 35040ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 32632ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29044ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 30670ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 31959ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 29648ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 23591ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 25149ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 21793ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 31553ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 27828ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1, 3, 20].

### q02 — Who are the author's children?

The provided sources do not contain information about the number or names of the author's biological children, but mention two of his children as "Feyruz" and "Reza". [18] and [14]. The same list is repeated in source [18], but it includes additional children: "Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis" who are listed as grandchildren.

### q03 — Who are the author's grandchildren?

According to the excerpts [1] and [2], the author's grandchildren are:

* Joseph Rassool
* Samuel Rassool
* Alexander Rassool
* Leonardo Rassool
* Dylan Rassool
* Lewis Rassool

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina; and his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [17]

### q05 — Who was J.M.H. Gool?

According to the sources [1]–[20], J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a community leader, a founder trustee of the Hanaffi Quwatul Islam Mosque in District Six, and a spice merchant. He was born with family origins in Swat via Gujarat, India, and arrived in Cape Town in 1884 from Mauritius. He had two wives: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra Abdurahman, etc.) and Wahida Gool (children: Abdul Hamid Gool, Mariam Margie Gool, Minnie Gool, etc.). He was the grandfather of Yousuf (Joe) Rassool, the author of this memoir.

Some key facts about J.M.H. Gool include:

* He lived at 7 Buitencingle Street in District Six.
* He was a community leader and was involved in various organizations, including the Coloured People's Vigilant Committee.
* He was a supporter of Gandhi's movement for Indian rights in South Africa and hosted Gandhi at his residence in 1914.
* He wrote medical articles for the Indian Opinion journal.
* He was mentioned in Ralph Bunche's travel notes as staying with him in 1937, although J.M.H. Gool himself is not mentioned in these notes.

Overall, J.M.H. Gool seems to have been an influential figure in Cape Town's Indian community during the early 20th century, known for his leadership and philanthropic efforts.

### q06 — Tell me about Buitencingle.

Buitencingle is described in several excerpts as a grand, 13-room mansion located at No. 7 Buitencingle Street in District Six, Cape Town. It was the residence of J.M.H. Gool, the grandfather of the author (Excerpt [1]). The house had a view of Table Mountain and Devil's Peak.

The street where the mansion is situated, Buitencingle Street, was wide and cobbled with traffic islands sporting stunted palm trees (Excerpt [18]). According to Excerpt [20], the name "Buitencingle" refers to an outer belt or a circle, which at that time denoted the outermost belt of Cape Town.

The house had numerous rooms, which would have been unmanageable without the help of servants (Excerpts [6] and [9]). The author's grandmother attempted to learn Afrikaans to communicate with the servants, but she found it difficult (Excerpt [6]).

Buitencingle was also mentioned as being a place where the family had many experiences and stories. The author reminisces about his time at Buitencingle, which included spending his childhood there with his siblings, exploring the garden, and climbing trees (Excerpts [5] and [15]). His grandfather, Haji Joosub Maulvi Hamid Gooli, was a prosperous merchant who lived in this mansion during his heyday (Excerpt [17]).

The house has a rich history, with many events taking place within its walls. However, it is now gone, but the author believes that its story and legacy should not be forgotten (Excerpts [2] and [17]).

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

The provided sources mention two women referred to as "wife" or associated with that role:

1. Nazima Rassool [1] - She is referred to as Professor Nazima Rassool, an academic researcher. The text states she urged her husband to ground his writing in background investigation and was instrumental in transforming the memoir from a subjective recollection into a historically documented account.
2. Bibi (second wife) [4, 14, 17] - She is referred to as Dr. Gool's second wife, whom he married after leaving India with his eleven-year-old bride. The text describes her reactions and concerns about being left behind in Rander.

The question "Tell me more about the author's wife" is likely referring to Nazima Rassool, based on the context of transforming the memoir from a subjective account into a historically documented one.

Specific facts about Nazima Rassool include:

* She is an academic researcher [1]
* She urged her husband (the author) to ground his writing in background investigation [7, 9, 19]
* Her role was instrumental in transforming the memoir into a historically documented account [1]

The sources do not provide detailed biographical information about Nazima Rassool beyond these points.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Joosub Maulvi Hamid Gool. He was a spice merchant and community leader who was born in Mauritius (via Swat, Gujarat) and arrived in Cape Town in 1884 [1]. He lived at 7 Buitencingle Street in District Six, where he received notable visitors including Mahatma Gandhi [1]. He had two wives: Bibi Gool and Wahida Gool, and was the grandfather of Yousuf (Joe) Rassool, the author of this memoir [1].

Specifically, the facts about J.M.H. Gool mentioned in the sources are:

* Born in Mauritius (via Swat, Gujarat)
* Arrived in Cape Town in 1884
* Lived at 7 Buitencingle Street in District Six
* Had two wives: Bibi Gool and Wahida Gool
* Grandfather of Yousuf (Joe) Rassool, the author

References:
[1] Haji Joosub Maulvi Hamid Gool [Person]
[2] 
[3] 10, 1906 p832  
     Courtesy National Gandhi Museum
20 Indian Opinion May 19, 1906 courtesy National Gandhi Museum 
Gool attends a meeting of the Coloured People’s Vigilant Committee on May 3, 1906.
...

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It connects Table Mountain to Lion’s Head, offering views of both [2]. According to the sources, Kloof Nek was a place of haunting beauty [4], [17], [18] with a Glen nearby. The area is also known for its natural scenery and historical significance.

The following specific facts about Kloof Nek are extracted from the sources:

- It lies beyond Buitencingle Street in District Six, which sits below Table Mountain [1].
- Beyond it, there are foothills of Table Mountain that can be reached via Kloof Street [2].
- A bus terminus for a single-decker trackless tram was located near Fletchers & Cartwrights department store at the bottom of Buitencingle [2].
- The area is frequented by lovers, mountaineers, nature lovers, and ramblers [5].
- It was also home to a growing number of down-and-outs known as "Bergies" who found refuge in cavelets, culverts, and bushes.

Kloof Nek Road passes through the area [7] and offers views of Table Mountain.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. [1] It adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students. [1] 

Affiliated with the Non-European Unity Movement, it had its conferences opened by the mayor with a civic banquet at the ratepayers' expense, making them important civic occasions where teachers from upcountry came to the city to spend a week of their winter vacation listening to eloquence about the nobility of the teaching profession. [2] 

The TLSA also provided training in political/educational composition and had its articles published in the Education Journal of the Teachers' League of South Africa, where writers received training and their pieces were considered for publication. [4] The organisation was led by members such as Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, and Frank Landsman. [5] 

It was a political organisation that challenged racial discrimination in education, which made the authorities view it as an untenable situation since teachers were regarded as leaders in the community, and there was no precedent for a teachers' union to become political. [6][19]

### q12 — Who was Cissie Gool?

Cissie Gool, also known as Zainunnisa Cissie Gool or Zainunnissa "Cissie" Gool, was a prominent South African political activist and lawyer. She was born on February 14, 1897 (Source [1]). Her father was Dr. Abdulla Abdurahman, a pioneering Cape Town city councillor, physician, and leader of the Cape Coloured community (Source [1]).

Cissie Gool was closely associated with the Non-European Unity Movement and the anti-apartheid struggle. She was a member of the Unity Movement, whose stand on the Whites-only elections was "A plague on both your houses" (Sources [3], [13]). 

She was known for her activism, including calling a massive demonstration against residential segregation in 1937 (Source [18]) and addressing meetings to deal with 'influx control' and street gangs (Source [16]). She was also involved in the Anti-CAD Movement.

Cissie Gool married Abdul Hamid Gool and had at least one daughter, Shaheen Gool (Source [1]). She died on December 2, 1963 (Source [1]).

The provided sources do not contain information about her personal life beyond marriage and children.

### q13 — What was the All Africa Convention?

The sources provide the following information about the All African Convention:

* Founded in 1935 (Source [1])
* Unified a broad coalition of African political organizations seeking unity against racial oppression (Source [1])
* Later aligned closely with the Non-European Unity Movement and its non-collaboration policy (Source [1])

The author also mentions attending a conference of the All African Convention in Bloemfontein, where he stayed overnight at the home of Dr. Murison (Source [2]). Additionally, Source [19] lists some of the leaders present at a meeting of the All African Convention.

Note that Source [4] mentions "All African Convention" as one of several entries listed, but does not provide any additional information about it.

Overall, based on these sources, we can say:

The All African Convention was an organization founded in 1935 that aimed to unite African political groups against racial oppression.

### q14 — Where was District Six and what kind of place was it?

District Six was located near the centre of Cape Town, situated below Devil's Peak [1]. It was a vibrant, multicultural community that housed Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

### q15 — What were the forced removals from District Six?

The provided sources do not contain information about the forced removals from District Six. However, given that the document "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool is a memoir about his experiences and those of others in District Six, it can be inferred that the forced removals are likely discussed in other parts of the book or in oral history accounts.

In general knowledge, the forced removals from District Six refer to the large-scale eviction of residents, mostly of Cape Malay and coloured communities, by the apartheid government in 1968. This event marked a significant milestone in the struggle against apartheid and has had lasting impacts on the community and its descendants.

To answer this question accurately, one would need access to information not present in the provided sources or to consult external historical records about the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi was Mahatma Gandhi, a prominent Indian leader who played a significant role in the Indian independence movement. According to the sources [1], [2], and [3], Gandhi had connections with the Gool family in Cape Town, South Africa. 

The Gool family, particularly J.M.H. Gool, hosted Gandhi at their home at 7 Buitencingle Street in Cape Town on multiple occasions, including during his farewell visit to South Africa before returning to India in 1914 [1]. Gandhi was a guest of the Gools and received hospitality from them, including medical care provided by Dr. J.M.H. Gool [2], [3].

Gandhi's connection with the Gool family also extended beyond their personal relationships. The sources mention that the Gool family played a central role in early Cape Muslim and Indian political life and were involved in anti-colonial struggles, connecting District Six to national and international anti-colonial movements [1], [2].

Gandhi wrote letters to J.M.H. Gool, and Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion [5]. Furthermore, the sources indicate that Gandhi was a frequent guest at 7 Buitencingle Street in Cape Town, staying there multiple times before his final departure from South Africa in 1914 [3].

The connection between Gandhi and the Gool family highlights their mutual respect and support during a critical period in Gandhi's life when he was fighting for Indian rights and independence.

List of specific facts mentioned in the sources about Gandhi:

* He had visited South Africa multiple times, including his farewell visit before returning to India in 1914 [1].
* He received hospitality from the Gool family at their home in Cape Town, particularly J.M.H. Gool's grand home at 7 Buitencingle Street [1], [2].
* Gandhi was a guest of the Prime Minister Botha at Groot Schuur and later by the Prime Minister and his wife, as well as Mrs. Gladstone, the wife of the Governor-General [3].
* He extended his contacts in the community among the poorer sections and devoted much time to public work for Indians [5].
* Gandhi was a strong advocate for non-violent resistance, which he termed "satyagraha" [1].
* He helped unite two rival organisations as “The Cape British Indian Union” [6].

The provided sources do not contain detailed information about Gandhi's life outside of his connections with the Gool family in South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained Coloured teachers under the apartheid-era segregated education system. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement [1]. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1]. The NEF was established in 1937 "to discuss everything under the sun" [3].

The New Era Fellowship was involved in various activities, such as:

* Organising lectures on political and social issues
* Hosting debates on philosophical topics
* Serving as a gathering place for progressive intellectuals and activists
* Issuing clarion calls to counter the predicted assault on the rights of the Coloured community [3]
* Participating in the formation of the Society of Young Africa (SOYA) [10]

The NEF had its own programme of events, which was overseen by an Organising Secretary [12]. One notable member was Hassan Bavasah, who served as Organising Secretary and was known for his debating skills [12].

In 1953, the New Era Fellowship viewed the establishment of the Cape Peninsula Students Union with grave misgiving, as they believed it might undermine their own influence among students [17].

The New Era Fellowship played an important role in the Coloured community, striking a chord that resonated with many and leading to a response from various organisations, including some previously moribund or defunct ones [19].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization founded in 1943 that united African, Coloured, and Indian people against apartheid and colonialism. Its program was based on a Ten Point plan for equal rights (cited from [5] & [10]). The NEUM adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions (cited from [1]).

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool (also known as Joe), was actively involved in cricket through the Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in the Cape [1]. At one point, he even captained a team with notable players such as Hassen Mall, Hamid Khan, Abrahams brothers (Jameel and Omar), Abou Desai, and Tiny Abed [2].

The author had to wait four years for his honour of being selected for a biennial tournament. His innings at Rosmead were not seen by selectors, which contributed to this delay [3]. The Western Province Indian Cricket Union side was chosen, with several players from the Kismets, but unfortunately, no selector had seen the author's innings, leading to him not being selected initially [9].

In his cricketing days, he developed a close friendship with Hassen Mall and became known as an "absolute rabbit" (51 in the parlance of the Gujarati community), meaning someone who scored very low runs. He began mastering the elements of the forward defensive stroke and built the foundation of batting. His goal was to score just 10 runs, which he would gladly have sold his soul for [14]. 

The author also played a part in organizing cricket teams at High School and College level but was thwarted by the entrenched bureaucracy of the Schools' Sports Union [11].

Despite this, he showed great potential as a cricketer and was considered one of the top players. Edross, the permanent secretary of the Barnato Cricket Board, even said that if he were a selector for the Indian cricket union, the author would have been his first choice due to his forward defensive stroke [17].

### q21 — Who was the author's mother?

The provided sources do not explicitly state the author's mother's name. However, they mention that the author had a sister named Yasmin (also called Jessie), and another sister who is referred to as "the girls" in [19]. The author also mentions their aunt, Auntie Dijah (Dija), who was mentioned in several excerpts.

### q22 — Who was the author's father?

The author's (Joe Rassool) father is Peter Alexander Rassool, who later became known as Peerbhai or Peru.

### q23 — Who were the author's siblings?

The provided sources state that the author had the following siblings:

1. Rasheda Rassool [1]
2. Abdul Rassool [1]
3. Berina Rassool [1]
4. Nasim Rassool [2], [5]
5. Zain Rassool [1]
6. Fazil Rassool [1], [19]
7. Yasmin Rassool (also known as Jessie) [18], [19]

Note that the sources do not provide a comprehensive list of all the author's siblings, and it is possible that there were other siblings mentioned elsewhere in the text who are not listed here.

### q24 — Who were the children of J.M.H. Gool?

According to [1], the children of J.M.H. Gool include:

* Abdul Hamid Gool
* Jane Gool-Tabata
* Mariam Margie Gool
* Minnie Gool
* Zobeida Gool
* Goolam Gool
* Fatima Gool
* Mariam Gool (note: this appears to be a duplication of Mariam Margie Gool)
* Mohamed Saaid Gool
* Ayesha Rassool
* Mohammed Hanief Gool
* Fatima Timmie Gool
* Zohra Abdurahman

Note that the list also mentions two wives, Bibi Gool and Wahida Gool, but it does not provide a comprehensive list of their children. However, [9] states that Cissie Gool (Zainunnissa) was the daughter-in-law of J.M.H. Gool through her marriage to Abdul Hamid Gool.

Also, according to [2], Yousuf (Joe) Rassool mentions his siblings, including:
* Nasim Rassool
* Berina Rassool
* Rasheda Rassool
* Zain Rassool
* Fazil Rassool
* Abdul Rassool

### q25 — Who was I.B. Tabata?

Based on the provided sources [1], [3], [9], [11], and [13], I.B. Tabata is described as:

* A prominent political activist and leader in the Non-European Unity Movement (Unity Movement) [1].
* Married to Jane Gool-Tabata, daughter of J.M.H. Gool [1].
* Accused by a group called The Forum Club of failing to have a class analysis in his historical tract, "The Awakening" [4, 19].

In terms of biographical details, sources do not provide:

* His exact origin or place of birth.
* His ancestry or family background.

Specific events and activities associated with I.B. Tabata include:

* Being involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].
* Writing an indictment against Ben Kies, which was evident because he rose to attack Kies at a meeting [2].
* Being accused of being a Black Nationalist by The Forum Club [4].

Overall, I.B. Tabata appears to have been a key figure in the anti-apartheid movement and the Unity Movement in South Africa during the 1950s.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the sources:

- He was a Cape Town doctor and city councillor [1].
- He was a leader of the African Political (later People's) Organisation (APO) [1].
- He was the first Coloured person elected to the Cape Town City Council [1].
- His father, also named Abdurahman, was brought to South Africa as a slave, bought his freedom, and became prominent in the community [4].
- Dr. Abdurahman's mother, Gadija Dollie, was said to be one of the great beauties of Cape Town and a member of the Dollie clan [4].
- He had several connections with notable figures, including Gandhi, who stayed at his residence in Loop Street during his visit to Cape Town [3], [16].

He is associated with several organizations and events, including:

* The Non-European Unity Movement
* Haji Joosub Maulvi Hamid Gool
* J.M.H. Gool family

Biographical facts about Dr. Abdurahman include:

* Born: 1872-09-08 [1]
* Died: 1940-02-20 [1]

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources state several connections between Gandhi and J.M.H. Gool:

1. Personal visits: Gandhi visited the Gool mansion at 7 Buitencingle Street in Cape Town on two occasions. He stayed with the family in February 1914, during which time he helped to furbish my uncle’s surgery by puttying and staining the floor [7].
2. Correspondence: There was a correspondence between Gandhi and J.M.H.'s son Abdul Hamid Gool while he was a medical student at Guy’s Hospital in London [3] & [19].
3. Shared interests: Gandhi extended his contacts in the community, particularly among the poorer sections, and devoted much of his time to public work for Indians [9]. This suggests that J.M.H. Gool's work with Indian communities may have been a point of connection between them.
4. Family connections: One of Gandhi's sons fell in love with one of Goulam Gool's sisters – a Moslem, indicating a family relationship or at least acquaintanceship [3].
5. Political collaboration: Gandhi helped to bring together two rival Indian organizations in Cape Town as "The Cape British Indian Union", of which J.M.H. Gool was made joint secretary and later president in 1911 [7].

### q28 — Which organisations was the author involved in?

The author, Joe Rassool, was involved in the following organisations:

* New Era Fellowship (NEF)
* Non-European Unity Movement (NEUM) [1]
* Teachers League of South Africa (TLSA) [1]
* All African Convention (AAC) [1]
* Cape Peninsula Students Union [19]

Note: The provided sources do not contain a comprehensive list of the author's organisational involvement. These are organisations mentioned as being relevant to his activities or experiences.

Additionally, there is a mention that the Coloured branch of Smuts’s United Party was also involved in the New Era Fellowship and other democratic movements, but it is unclear if Joe Rassool himself was directly involved with this organisation.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM) [1] and shared its boycott and non-collaboration programme [1]. The TLSA had a close relationship with the NEUM, with many of its members being involved in the movement. For example, Aunt Jane, Dr. Goolam's sister, was a key figure in the NEUM [10].

The TLSA also participated in the Unity Movement Conference, where delegates from Natal and Transvaal attended, including Auntie Minnie's home as the focal point of the meeting [12]. The TLSA's participation in the conference suggests that they were closely aligned with the NEUM.

In addition, Vic, a trade unionist, declared himself an adherent of the Unity Movement when speaking to the author, indicating that he and others saw the TLSA as part of the broader NEUM movement [16].

Overall, the relationship between the TLSA and the Non-European Unity Movement was one of close affiliation and shared goals.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain information on when J.M.H. Gool arrived in Cape Town or the origin of his arrival (i.e., from where). They mention various events related to him, such as his testimony being recorded in Supreme Court transcripts in 1892 and threats to his life in Natal in December 1896, but do not specifically state his arrival date in Cape Town.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Quwatul Islam Mosque or Loop Street Mosque, was founded in 1898 and is one of the oldest mosques in the Cape Muslim community. It is located on Loop Street in Cape Town (Source: [1]). The mosque was central to the history of the Gool family and the Cape Muslim community in District Six (Source: [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [9] and [10], Cissie Gool was the wife of Abdul Hamid Gool, who was one of the two sons of J.M.H. Gool. Therefore, Cissie Gool was the daughter-in-law of J.M.H. Gool.

Additionally, according to [13], Dr. Abdurahman (Cissie's husband) was related to J.M.H. Gool through the Dollie family. This suggests that Cissie Gool may have had a familial connection to J.M.H. Gool through her husband's side of the family.

However, it is worth noting that [1] and other sources do not mention any direct familial relationship between Cissie Gool and J.M.H. Gool.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the sources [1]–[20], the following are some of the notable historical figures that J.M.H. Gool knew personally:

1. Mahatma Gandhi: Gool received notable visitors including Gandhi during his 1914 farewell visit to South Africa before returning to India (Source [1]).
2. Cecil John Rhodes: Rhodes visited Gool, along with George Bernard Shaw and Dr. Abdulla Abdurahman (Source [1]).
3. Dr. Abdulla Abdurahman: Abdurahman was a friend of Gool's, and his residence was in Loop Street, close to Buitencingle where Gool lived (Source [18]).
4. Adam H.G. Mahomed: Mahomed was a senior, financier, and possibly mentor of Gool's, but not related to him (Sources [5] and [20]).

Additionally, Gool's family connections and involvement in the community suggest that he may have known other notable figures, such as:

1. Cissie Gool (Zainunnissa): Cissie was Dr. Abdulla Abdurahman's daughter and Abdul Hamid Gool's wife (Source [18]).
2. Solly Edross: Edross was a speaker at the meetings held in Cape Town during the 1930s, which Gool attended with his family (Source [8]).

However, it is worth noting that these connections are not explicitly stated as personal relationships but rather as part of Gool's community involvement and family ties.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [3] was a law that legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life. It dispersed the people of District Six across the barren sandhills of the Cape Flats, decimating the once vibrant community and way of life [2]. The Act was especially impactful on the South African Indians, who were among those most affected by its provisions.

As a result of the Group Areas Act, many residents of District Six lost their homes and were forced to move to other areas. Some of these new areas included Bonteheuwel, Hanover Park, Mannenberg, Mitchell's Plain, Vanguard Estate, and Bishop Lavis [4]. These areas were often characterized as sub-economic schemes, which meant that the inhabitants lost the municipal franchise because they were living in sub-economic houses. This effectively disfranchised the residents [5].

The Group Areas Act also had a significant impact on the economic and social fabric of District Six. The community's concerns about their future were centered around the marketing of fruit and vegetables, as well as the sustainability of an "Indian" area [7]. However, the Indians would hardly ever hire a Non-White lawyer to represent them in such cases, believing firmly that their case would be lost before it was presented if they did [7].

The Group Areas Act had far-reaching consequences for the people of District Six, leading to the displacement and marginalization of its residents.

### q35 — Who was Hassen Mall?

Hassen Mall was a close friend and fellow cricket player from District Six, according to [1]. He was described as having "something very striking" about him in [6], with characteristics including being not tall but radiating magnetism, cultivating a clipped brush moustache, and smoking a pipe. He was a brilliant cricketer who spun a wristy leg break that zipped like a circular saw, and later switched to Law after discovering the sciences were not his forte ([6]). Hassen Mall qualified with the degree BA LLb [44] before departing for Durban in 1951, leaving behind an indelible imprint of his personality.

Hassen's cricketing career included being captain of the team at least once, as seen in [16]-[17]. He was also part of a team that included Hassen Mall that had to face Hamid Khan when he played for them, and was described as having "a clipped brush moustache, which suited him well" ([6]). Hassen's cricketing abilities were so renowned that the author wrote that Tiny Abed would have gone on to represent South Africa if he had the required race-classification [10].

In non-cricketing contexts, Hassen Mall was also mentioned as a friend who introduced card games to the group, particularly whist and rummy ([8]). He was described as being "keen to foster the friendship" between the author and his family members, including paying visits and encouraging the author to visit him in Durban [4].

In addition to these facts, Hassen Mall's association with other notable individuals is mentioned. For instance, he was part of The Indian Congress along with Dawood, his cousin Fatima's husband ([20]).

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to [1], the main political organisations active in the Cape Coloured community during the mid-twentieth century were:

* The Non-European Unity Movement (NEUM)
* The Teachers League of South Africa (TLSA)
* The New Era Fellowship (NEF)
* The All African Convention (AAC)

These organisations shared a boycott and non-collaboration policy against apartheid institutions, with NEUM serving as the umbrella body.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer to work on a legal case. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance) as a tool of political struggle [1].

He was connected to the Gool family in Cape Town, staying at their home at 7 Buitencingle Street during his 1914 farewell visit [2]. Gandhi had come to South Africa on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria. He was sent for because the accounts were kept in Gujarati, which was Gandhi's mother tongue [6].

In fact, he first came to South Africa in 1893 to work on a legal case and lived there for over 20 years, developing his philosophy of satyagraha (non-violent resistance) as a tool of political struggle.

### q38 — Who was Cissie Gool's father?

Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to source [1], District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

Source [3] describes it as follows:

"It is hard to explain why the school grounds were undeveloped. It is possible that it had been dug up after the school was closed to prevent it being used as a playground by the children in the neighbourhood."

This suggests that before the forced removals, District Six was a lively and thriving community with active residents.

Source [4] notes:

"A community of freed slaves from America and the West Indies thrived in District Six in Victorian times. Some of them would achieve great position and wealth. Their expulsion at the turn of the century foreshadowed the Apartheid era evictions of the same neighbourhood decades later."

This indicates that District Six had a diverse history, with different groups contributing to its cultural richness.

Source [7] mentions:

"The Rassools were on the move again...Our family moved to 95 Caledon Street a few months before the outbreak of World War Two, which, for South Africa, began on the third of September 1939."

This implies that the community was dynamic and constantly evolving, with people moving in and out.

Source [16] describes District Six as follows:

"It seemed as if the madness of war had come to the streets of District Six. Meanwhile in Europe the Nazi army carried everything before them. By 1941 most of the countries allied to Britain had been over-run: Belgium, Holland, Denmark, France, and Norway."

This suggests that the community was affected by external events, but still managed to thrive despite the challenges.

Source [18] provides a vivid description:

"To me those bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention. To have been part of that pulsing life was an unforgettable education and privilege..."

This implies that District Six was a vibrant and lively community with a strong sense of life and culture.

Overall, based on the sources provided, it can be said that District Six was a thriving multicultural community before the forced removals, characterized by its rich urban culture, diversity, and dynamism.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy involved refusing to participate in any apartheid government institutions or activities. This included boycotting dummy elections, such as the "Native Representatives" election mentioned in [7] and [8], where the Communist Party intended to put up candidates despite the general desire among the Blacks to boycott these "dummy" elections.

The Movement also adopted a policy of non-collaboration, meaning that its members would not cooperate with the apartheid government or its institutions. This was stated in the 10 Point Programme, which was mentioned in [4] and [13].

In practice, this meant that the Unity Movement encouraged its supporters to boycott services provided by the apartheid government, such as public transportation, and to refuse to participate in activities that legitimized the apartheid regime.

For example, in [11], it is stated that the Anti-CAD movement used the tactic of ostracizing an agent of a large landowner who had accepted service on the C.A.C. (Coloured Advisory Council), which was seen as a pro-apartheid institution.

In [12], it is mentioned that the school where the author worked was gripped by tension due to the boycott of the Coloured Affairs Council, with even the headmaster being subjected to the boycott in its most venomous form.

The Unity Movement's boycott policy was not always successful, and there were instances where members of the movement failed to adhere to it. For example, [14] mentions that some members of the "Movement" simply ignored the injunction to boycott the Group Areas Act, while others appeared before the board or sent their legal representatives to register their protest against the decision to have their areas reclassified.

Overall, the Unity Movement's boycott policy was a key aspect of its strategy for resisting apartheid and promoting non-racialism in South Africa.


  ✅ Overall: 87.2% recall (token-overlap)  (182.2/209)  avg 26626ms
