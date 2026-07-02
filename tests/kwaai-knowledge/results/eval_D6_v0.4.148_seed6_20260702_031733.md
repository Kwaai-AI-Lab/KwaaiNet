[2m2026-07-02T09:58:21.400780Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T09:58:21.541750Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  45266ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T09:59:10.433731Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  24525ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T09:59:33.492042Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  22628ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T09:59:56.252755Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  22068ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T10:00:18.436268Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  33533ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T10:00:52.419559Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  29762ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T10:01:22.055385Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  21478ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T10:01:43.797580Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  22846ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T10:02:06.822426Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/9 keywords  26443ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T10:02:33.719983Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  32502ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T10:03:20.708932Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  43826ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T10:03:53.559004Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  35206ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T10:04:26.535840Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  22330ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T10:04:48.019194Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  19932ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T10:05:08.269330Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 88 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced relocation history South Africa apartheid era""
  ○ Round 3   added 19 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  30509ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T10:05:37.671990Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  29108ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T10:06:07.007252Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  18951ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T10:06:26.211345Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  26723ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T10:06:55.161147Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  22324ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T10:07:16.894513Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  25551ms
  [21/40] Who was the author's mother?
[2m2026-07-02T10:07:42.694437Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  21119ms
  [22/40] Who was the author's father?
[2m2026-07-02T10:08:03.852857Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  19616ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T10:08:23.752803Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  22856ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T10:08:46.736387Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  30426ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T10:09:18.511147Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  25678ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T10:09:43.452865Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  27403ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T10:10:11.169618Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  29607ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T10:10:40.646802Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  24036ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T10:11:04.981476Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  21025ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T10:11:26.074002Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1.8/6 keywords  28655ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T10:11:54.847419Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  28471ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T10:12:23.706863Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27267ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T10:12:51.173298Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  27112ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T10:13:18.453850Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  32505ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T10:13:51.470994Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  28312ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T10:14:19.795787Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23681ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T10:14:43.929947Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  24845ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T10:15:08.695872Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  22281ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T10:15:31.156282Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  27600ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T10:15:59.029318Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  25403ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 88.9% (185.8/209) |
| Avg latency | 26835ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 45266ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 24525ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 22628ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22068ms |
| q05 | Who was J.M.H. Gool? | 7/8 (88%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 33533ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 29762ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 21478ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 22846ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 26443ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 32502ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 43826ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 35206ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 22330ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 19932ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | [Graph: District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 30509ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/5 (80%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 29108ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 18951ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 26723ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 22324ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 25551ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 21119ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 19616ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 22856ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 30426ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 25678ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 27403ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29607ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 24036ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 21025ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1.8/6 (30%) | sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 28655ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 28471ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 27267ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 27112ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 3/6 (50%) | sequence_diagram:District Six, [Graph: District Six], LEST WE FORGET -rev25.pdf | 32505ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 28312ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 23681ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 24845ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 22281ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 27600ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 25403ms |

## Answers

### q01 — Who is the author?

The author of "District Six: Lest We Forget" is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The text does not provide a comprehensive list of the author's children, but it mentions the following:

1. Feyruz (daughter) [18] & [20]
2. Reza (son) [18] & [20]
3. Zarina (daughter) [18] & [20]

No further information is provided about their siblings or other family members in the given excerpts.

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool [15]

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina; and his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [4]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a person of significant importance in the history of District Six, Cape Town, South Africa.

From the sources provided:

* He was a spice merchant and community leader who arrived in Cape Town in 1884 from Mauritius.
* His family origins were in Swat (via Gujarat), India.
* He was married to two women: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra Abdurahman) and Wahida Ta'al (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool).
* He had a son named Abdul Hamid Gool who married Cissie Gool (Zainunnissa, daughter of Dr. Abdulla Abdurahman), making her his daughter-in-law.
* He performed the Hajj in 1899 and died in April 1940.
* He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He received notable visitors, including Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman.

J.M.H. Gool played an important role in the community as a leader, entrepreneur, and supporter of various organizations and causes, including the Indian Opinion (a newspaper) and the British Indian League. He was also involved in municipal politics and became the first black man to be elected to a public office in the UK.

He is notable for his involvement with Gandhi and other notable figures, as well as his contributions to the community through business, philanthropy, and leadership.

### q06 — Tell me about Buitencingle.

Based on the sources provided, here is a comprehensive account of Buitencingle:

**Origin**: The name "Buitencingle" is Dutch in origin, with "cingle" meaning "belt". At that time, it referred to the outermost belt of Cape Town.

**Location**: Buitencingle was situated near District Six, Cape Town. Opposite 7 Buitencingle Street stood the German Lutheran Church (Source [4]).

**House**: The Gool family's mansion at No. 7 Buitencingle Street had 13 rooms and a view of Table Mountain and Devil's Peak (Source [1]). It was described as vibrant, grand, and having a certain magic in the atmosphere, with perfumes from shrubs, herbs, flowers, and the smell of rich moist earth (Source [4]).

**History**: The Gool family, including Haji Joosub Maulvi Hamid Gooli's grandfather J.M.H. Gool, lived in Buitencingle. In 1911, Gandhi visited Buitencingle to help putty and stain the floor of Dr. Gool's surgery (Source [5]). The Gools were involved in politics, with Dr. Gool being a joint secretary of the Cape British Indian Union (Source [5]).

**Events**: According to Source [14], after Grandpa Gool had a stroke, he never ventured out again and spent most of his time at home, gazing into the burning coals of the fireplace.

**Notable visitors**: Emily Hobhouse visited Buitencingle (Source [11]), as did the Prime Minister Botha, his wife, Mrs. Gladstone, and other notable figures.

**Personality**: The house was described as having a "spirit" that dies when it is left empty (Source [10]).

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool, who is also known as Professor Nazima Rassool [1]. She is an academic researcher and played a central role in shaping the memoir by urging the author to ground his personal memories in historical research.

### q08 — Tell me more about the author's wife.

Based on the provided sources, here are some facts about the author's wife:

1. The author's wife is Professor Nazima Rassool [1].
2. She was married to Yousuf (Joe) Rassool [1], who is the author of "LEST WE FORGET -rev25.pdf".
3. Professor Rassool played a central role in shaping the memoir by urging her husband to ground his personal memories in historical research [1].
4. Her academic expertise as an academic researcher was instrumental in transforming the book from a purely subjective recollection into a historically documented account of District Six [1].
5. She is mentioned as having urged her husband to do background investigation and incorporate it into his writing, which led to some "staggering information" being uncovered [15], [19].

These are all the facts that can be gathered about Professor Nazima Rassool from the provided sources.

### q09 — Who was the author's grandfather?

The provided sources state that the author's grandfather was Haji Joosub Maulvi Hamid Gool (also referred to as J.M.H. Gool). The following facts can be gathered from the sources:

* He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.
* He was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He had two wives: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool) and Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool).
* He performed the Hajj in 1899.
* He died on April 1, 1940.

Sources:
[1], [3], [9]

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town, South Africa (Source: [1]). It was also referred to as Buitencingle, the outermost belt of Cape Town (Source: [2]).

The area around Kloof Nek was described as having "haunting beauty" (Source: [4] and [18]). It was a popular spot for lovers, mountaineers, nature lovers, and ramblers. The Glen at Kloof Nek was said to be a place of haunting beauty, with Signal Hill being a notable landmark in the area (Source: [18]).

There were also reports of "Bergies", down-and-outs who found refuge in the area's cavelets and bushes, drinking cheap wine and methylated spirits (Source: [16]). The Table Mountain cable car was nearby, which provided an alternative to walking up the mountain paths for those who found them too onerous (Source: [16]).

Kloof Nek had significant connections to other locations in Cape Town. It sat below Table Mountain, near Buitencingle Street in District Six, and beyond it lay the Glen and the Atlantic seaboard (Source: [1]). The area was also connected to Kloof Street, which climbed from the city bowl up to Kloof Nek (Source: [1]).

Kloof Nek played a role in the life of the author, who visited the area with friends. They read poetry together in the Glen, and the author reflected on the beauty of the location (Source: [5]). The area was also significant to Persotem Patel, who lived in Worcester but often visited Kloof Nek with the author (Source: [19]).

Overall, Kloof Nek was a unique and picturesque area that held significance for both its natural beauty and its connection to the lives of individuals living in Cape Town.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. [1] They were affiliated with the Non-European Unity Movement. [1] The TLSA adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students. [1]

The TLSA also provided training in the art of political/educational composition and published articles in their Education Journal. [4, 8] Those who were regarded as good writers were inducted onto the editorial board of the journal, where they received further training. [8]

According to [5], the old guard was "dropped from the van and the free men" when new leaders took control of the TLSA. The new leaders included Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, Frank Landsman, and others.

The TLSA was considered an important civic organisation, with its conferences being opened by the mayor with a civic banquet at the ratepayers' expense. [2] Members of the TLSA included Danny De Beer, who had done his degree through the University of South Africa, and Solly Edross, who was B.Sc. qualified. [12]

The TLSA played a significant role in the struggle for democratic rights for non-citizens in South Africa. [6] They were members of the Non-European Unity Movement, which acknowledged that the struggle for equal education was also bound up with the struggle for a democratic society. [13]

### q12 — Who was Cissie Gool?

Cissie Gool (also known as Zainunnisa Gool) was a Cape Town city councillor, lawyer, and political activist [1]. She was born on February 14, 1897. Her father was Dr. Abdulla Abdurahman, a pioneering Cape Town city councillor, physician, and political leader of the Cape Coloured community [2]. Cissie Gool was closely associated with the Non-European Unity Movement and the anti-apartheid struggle.

She was known for her strong views and actions: she spoke in support of measures to deal with "influx control" and street gangs (the "skolly" problem) [3], which shocked some people, including the author, who expected her to hold more radical views [4]. She also submitted a subscription to the Indian Opinion on behalf of Mr. Wilson [9].

Cissie Gool was married to Abdul Hamid Gool and had children: Shaheen Gool (son), Margie Gool, Timmie Gool, Fatima "Timmie" Gool, Zohra "Jolly" Gool, Mariam "Margie" Gool, Ayesha "Lallie" Gool, Dija "Dija" Noor [20]. She died on December 2, 1963.

She was associated with other notable individuals: Dr. Abdurahman (her father), Ben Kies, Solly Edross, Aunt Jane, and the author's uncle, Dr. [12]. She also knew people like Cissie Gool, Freda Lock, and Lilian Isaacson who were "women who loved" her husband Cheops [4].

The provided sources do not contain exhaustive information about Cissie Gool's early life or personal background beyond her birthdate and family connections.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was an organization that emerged in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights [1]. The AAC united a broad coalition of African political organizations seeking unity against racial oppression and later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

### q15 — What were the forced removals from District Six?

The provided sources do not directly address the forced removals from District Six. However, they mention that the area was being redeveloped (see [4]) and the family moved out of their house in 1956 ([5] doesn't explicitly state this but the previous source implies it). The author also describes the area as having become a run-down city centre by the time they left it in 1956 ([16]).

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi was Mahatma Gandhi, an Indian independence activist who played a key role in India's struggle for independence from British rule. According to the sources, Gandhi had close ties with the Gool family, particularly Dr. Amod Gool and J.M.H. Gool (also referred to as Dr. Gool). 

Gandhi visited 7 Buitencingle Street, which was the home of the Gool family, multiple times. In 1911, he helped to furbish Dr. Gool's surgery by putting down putty and staining the floor (source [15]). Gandhi also stayed with the Gools during his visits, including a notable stay in February 1914, which led to the Gools hosting Gandhis at their residence (sources [18] and [19]).

The sources indicate that Dr. Amod Gool was a medical doctor who attended to Mrs. Gandhi when she fell ill (source [4]). In fact, Indian Opinion reported that Dr. Gool's "unremitting" attendance on Mrs. Gandhi during her serious illness was greatly appreciated by Gandhi (source [4]).

Furthermore, the sources mention that Gandhi's family, including his wife and children, stayed with the Gools at 7 Buitencingle Street in February 1914 (sources [18] and [19]). This stay was significant, as it marked a crucial time in the satyagraha movement led by Gandhi.

Gandhi also had professional connections with the Gool family. In source [3], Gandhi is mentioned as being helped by Dr. Gool in his medical practice when he needed to attend to Mrs. Gandhi during her illness.

Overall, the sources suggest that Gandhi and the Gool family enjoyed a close personal and professional relationship, with the Gools providing hospitality and support to Gandhi during his visits to South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained Coloured teachers under the apartheid-era segregated education system [1]. The author and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1]. 

Note: The provided sources do not contain detailed information about Hewat Training College beyond this description.

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement [1]. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1]. The NEF saw to the programme of lectures of the New Era Fellowship [2] and had its own publication [6].

According to the provided sources, other notable facts about the New Era Fellowship include:

* It was established in 1937 [3].
* It issued a clarion call to all Non-European organisations to meet to counter the predicted assault on the remaining rights of the “Coloured” community [3].
* Mr. I.B. Tabata completed his book "The Awakening of a People" as an NEF publication, which recorded the coming into existence of the All African Convention [6].
* Hassan Bavasah was the Organising Secretary of the NEF, overseeing its programme of events [12] and [15].
* The NEF was involved in various activities, such as public lectures, debates, and discussions on political, social, and philosophical topics [1], [7], and [13].

Overall, the New Era Fellowship appears to have been an important platform for progressive ideas and activism within the Coloured community in Cape Town.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization founded in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism [1]. Its programme was based on the Ten Point plan for equal rights [9]. The NEUM adopted a boycott and non-collaboration policy - refusing to participate in any apartheid government institutions, including the Coloured Advisory Council [1].

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool (Joe), was actively involved in cricket through various roles and teams:

1. Played for Kismets Cricket Club (Source [1])
2. Associated with the Western Province Indian Cricket Union (Source [1])
3. Involved in administering club cricket in the Cape (Source [1])
4. Participated in a proposed visit of a West Indian Cricket team to South Africa (Source [10] and [18], mentioning the "Three W's"; Worrel, Weekes, and Walcott)
5. Attempted to organize a union for playing cricket at High School and College level, but was thwarted by bureaucracy (Source [11])
6. Learned to play cricket seriously due to his friendship with Hassen Mall (Source [14])
7. Called a meeting of interested students to form a cricket team at Hewat school (Source [14])
8. Scored 12 runs in an innings, which earned him recognition (Source [9])
9. Was mentioned as being on a list of potential players for the biennial tournament by a letter to the Cape Standard (Source [18])

These excerpts demonstrate that Joe Rassool was not only a player but also involved in organizing and administering cricket activities at various levels.

### q21 — Who was the author's mother?

According to [1], the author's mother is Ayesha Rassool (also known as Lallie or Ayesha Gool), who was born on December 10, 1900.

Additionally, [10] mentions that Bibi was the name given to the child, which implies that Bibi and Ayesha are interchangeable names for the mother.

### q22 — Who was the author's father?

The author's father is Peter Alexander Rassool [1].

### q23 — Who were the author's siblings?

According to source [1], the author's siblings are:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool (also mentioned in source [19])
7. Yasmin Rassool (also mentioned in sources [1] and [11])

Note that source [19] also mentions a brother named "Baby pear", but this is likely a nickname for one of the siblings, possibly Nasim or another sibling.

### q24 — Who were the children of J.M.H. Gool?

The sources list multiple children for J.M.H. Gool:

[1] Haji Joosub Maulvi Hamid Gool [Person]
...
Father of: Abdul Hamid Gool, Jane Gool-Tabata, Mariam Margie Gool, Minnie Gool, Zobeida Gool, Goolam Gool, Fatima Gool, Mariam Gool, Mohamed Saaid Gool, Ayesha Rassool, Mohammed Hanief Gool, Fatima Timmie Gool, Zohra Abdurahman.

[14] ...
Gandhi was at 7 Buitencingle, Cape Town in 1911 because he mentions in his diary that he helped to furbish my uncle’s surgery by puttying and staining the floor.
...

[15] ... George had treated the penniless poet from Johannesburg to a meal at a cafe in Castle Bridge, and put him in touch with my uncle, Goolam Gool.

Note: The second set of children (Ayesha Rassool, Mohammed Hanief Gool, Fatima Timmie Gool, Zohra Abdurahman) may be part of the family through J.M.H. Gool's two wives, Bibi and Wahida.

### q25 — Who was I.B. Tabata?

According to sources [1] and [12], I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool, and was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy.

From source [12], we can list the following facts about I.B. Tabata:

* Married to: Jane Gool (daughter of J.M.H. Gool)
* Member of: All African Convention, Non-European Unity Movement
* Involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy
* Author of the book "The Awakening of a People", which recorded the coming into existence of the All African Convention

Note that source [4] mentions Tabata being accused of being a Black Nationalist, but this is not supported by other sources.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman [1-4, 9] was a Cape Town doctor and leader of the African Political (later People's) Organisation (APO). He was the first Coloured person elected to the Cape Town City Council. His father brought him to South Africa as a slave but bought his freedom and became prominent in the community [4]. Dr. Abdurahman studied at Guys Hospital, London [6]. 

His wife, Cissie Gool's mother, was a member of the Dollie clan [4]. He had a close relationship with J.M.H. Gool's grandfather, which likely facilitated co-operation between their organisations [5].

Dr. Abdurahman's residence was in Loop Street, near Buitencingle, where Gandhi stayed during his visit to Cape Town [16]. Dr. Abdurahman also spoke at the farewell address to Gandhi on behalf of Port Elizabeth Indians [17].

He met and married Helen “Nellie” Potter James in Glasgow University in 1893, where he was the first Coloured person to qualify as a doctor [18]. The Abdurahman clan and the Ta'Als were linked through the Dollie family.

Dr. Abdullah Abdurahman died on February 20, 1940 [1].

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The connection between Gandhi and J.M.H. Gool is described in several excerpts:

* Excerpt [1] mentions that Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) lived at 7 Buitencingle Street, where notable visitors including Mahatma Gandhi would come to stay.
* Excerpt [4] describes how Gandhi's son fell in love with one of Goolam Gool's sisters, a Muslim girl, and how Gandhi intervened and refused the marriage because it was not socially acceptable in India at that time. However, two weeks later he recalled his son and married him to a Hindu girl.
* Excerpt [9] mentions that Gandhi extended his contacts in the community, particularly among the poorer sections, and devoted much of his time to public work for Indians. In 1896, Gandhi went to India to fetch his family, during which time he tried to publicize the grievances of the Natal Indians.
* Excerpt [10] describes how the Gool family hosted Gandhis in February 1914 at their residence at 7 Buitencingle Street, and that this led to a friendship with Joe Rassool (the author).
* Excerpt [11] mentions that J.M.H. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion.

Overall, the connection between Gandhi and J.M.H. Gool was one of mutual respect and support. Gandhi stayed at Gool's residence on several occasions, and Gool provided him with medical care and assistance. In return, Gandhi extended his contacts in the community to include Gool and his family, and Gool supported Gandhi's public work for Indians.

### q28 — Which organisations was the author involved in?

Based on the provided sources, the following organisations were mentioned as being associated with the author:

1. The New Era Fellowship (NEF) [3], [6], [17]
2. The Non-European Unity Movement (NEUM) [1] (mentioned as an umbrella body of which the NEF was a part)
3. The Teachers League of South Africa (TLSA) [11] (mentioned as an organisation of which the author and others were members)
4. The All African Convention (AAC) [17]
5. The Anti-CAD (Anti-Coloured Affairs Department) [13]

Additionally, the author mentions being involved in other organisations indirectly through their associations with individuals who were part of these groups, such as:

1. The Coloured branch of Smuts's United Party [17]
2. The Communist Party of South Africa [17]
3. The Fourth International of South Africa [17]
4. The Liberation League [17]
5. The Workers' Party [17]

The author also mentions being part of the student organisation, the New Era Fellowship, which was involved in the Cape Peninsula Students Union (CPSU) [19].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM). They shared a boycott and non-collaboration programme. [1]

In 1956, the TLSA approached the Congress movement to suggest formulating a unified strategy to counter the anti-educational plans of the regime. However, the Congress leadership declined the invitation to cooperate. [10]

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information about when or from where J.M.H. Gool arrived in Cape Town, except that his first son was born in 1886 [1]. However, it is mentioned in [3] that a party arrived in Cape Town in December and included Nasima's father who was born in Mauritius in September, but this does not provide direct information about J.M.H. Gool's arrival.

However, if we look at the source [2], it mentions that Abdurahman's grandfather was brought to South Africa as a slave, implying that some people arrived in South Africa through slavery.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as Quwatul Islam Mosque or Loop Street Mosque, was one of the oldest mosques in the Cape Muslim community. It was founded in 1898 and was central to the history of the Gool family and the Cape Muslim community in District Six (Source [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], J.M.H. Gool had two wives: Bibi Gool and Wahida Ta'Al (also known as Wahida Gool). 

Cissie Gool is mentioned in [2] as the daughter-in-law of Dr. Abdurahman, but also as married to Abdul Hamid Gool, who is a child of J.M.H. Gool from his first wife Bibi Gool.

This establishes that Cissie Gool was the daughter-in-law of J.M.H. Gool through her marriage to his son Abdul Hamid Gool, making her related to him as a member of his family but not by blood.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to [1], Haji Joosub Maulvi Hamid Gool lived at 7 Buitencingle Street, District Six, where he received notable visitors including:

* Mahatma Gandhi
* Cecil John Rhodes
* George Bernard Shaw
* Dr. Abdulla Abdurahman

Additionally, [18] mentions that J.M.H. Gool is not mentioned in the Ralph Bunche’s travel notes of his stay with the Gools in 1937, but this does not imply a personal relationship between them.

It's worth noting that [2] and [5] do not mention notable historical figures interacting with J.M.H. Gool.

This list only includes individuals from [1] and [18], but other sources may provide additional information about the relationships of Joosub Maulvi Hamid Gool.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [3] was legislation that played havoc with the lives of ordinary people in Cape Town, dividing most of South Africa into separate areas. The government worked on an age-old theory that people tended to anticipate the wishes of the authorities [4]. According to this theory, if they were given advance notice of what would happen if they didn't comply, they would be more likely to do so.

The Group Areas Act resulted in the dispersal of communities and ways of life. In Cape Town, a thriving cosmopolitan area called District Six was particularly affected. The community's needs were immediate, and they employed lawyers to delay the inevitable, which often exasperated the authorities [6].

In District Six, people were concerned about their future if they were herded into one group area. They would lose municipal franchise because they were living in sub-economic houses, effectively disfranchising them [4]. Some members of the "Movement" ignored the injunction to boycott, appearing before the board or sending legal representatives to register their protest against the decision to reclassify their areas [3].

The community's concerns and efforts to resist the Group Areas Act are detailed throughout the provided sources. The dispersal of communities and ways of life due to the Group Areas Act is a significant part of the story told in this memoir.

The exact details regarding how the Group Areas Act was implemented, its full effects on District Six, or any follow-up actions taken by the community against it are not fully explained within these excerpts. However, they do highlight the impact of the legislation on the people living in Cape Town and their efforts to resist it.

### q35 — Who was Hassen Mall?

Based on the sources, here are the specific facts that can be gathered about Hassen Mall:

1. He is a South African cricketer (Source [1])
2. He was born in Durban and stayed with his relatives in Cape Town while pursuing his studies at the University of Cape Town (Sources [1], [3], [4])
3. He was a brilliant cricketer who played for the Western Province Indian Cricket Union team, which included notable players such as Hamid Khan and the Abrahams brothers (Sources [9], [10], [13], [19])
4. He captained the team that went to Durban for the biennial cricket tournament in 1951 (Source [16])
5. He was a close friend of the author, Yousuf Rassool (Sources [1], [3], [6], [7], [8], [9], [10], [13])
6. He was a student at the University of Cape Town and later qualified with a degree in Law, BA LLB (Source [6])
7. He had a clipped brush moustache and smoked a pipe (Source [6])
8. He was known for his wristy leg break and was considered a fearsome fast bowler (Source [6])

Overall, Hassen Mall appears to be a talented cricketer and a close friend of the author's who played an important role in the author's life during their university days.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were: [1] 

* The Non-European Unity Movement (NEUM) 
* The Teachers League of South Africa (TLSA) 
* The New Era Fellowship (NEF)
* The All African Convention (AAC)

These organisations were united by the boycott and non-collaboration policy against apartheid institutions.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi (1869-1948) was a lawyer, philosopher, and politician from India. According to [1], he came to South Africa in 1893 as a lawyer to work on a legal case.

He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance or passive resistance) as a tool of political struggle ([1]). He also founded the newspaper Indian Opinion to advocate for non-European rights in South Africa ([1]).

Gandhi was deeply involved in the struggles of Indians and Coloured people in South Africa. He had connections with prominent individuals, including Dr. Abdulla Abdurahman and Haji Joosub Maulvi Hamid Gool ([1]), and stayed at the Gool mansion at 7 Buitencingle Street during his visits to Cape Town ([5]).

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources [1] and [3], District Six was a vibrant, multicultural community near the center of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

In source [2], it is mentioned that the neighborhood had a diverse range of people living in sub-economic houses, but despite these circumstances, the residents were effectively disfranchised due to their living arrangements.

Source [3] describes District Six as a place with "more life, more zest, more fire, more warmth and care than many other places one could mention." The author reminisces about his childhood memories of walking through the streets, playing games, and experiencing the joy of community life. He notes that the neighborhood was home to people from different backgrounds, including Coloured, African, Indian, and White residents.

Source [4] highlights the impact of apartheid on District Six, stating that it legitimized poverty and oppression for the majority of its peoples, uprooting communities and ways of life. However, it also acknowledges that District Six was a thriving cosmopolitan area before the Group Areas Act (1950) dispersed its people across the Cape Flats.

Overall, the sources portray District Six as a vibrant, diverse community with a strong sense of culture and identity before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

According to [11], the Unity Movement held firmly to the belief that a morally superior idea must overcome an inferior one. The boycott now became the weapon of choice and it perfectly fitted the demands of the situation in The Cape in 1943.

[1] states that the Non-European Unity Movement (NEUM) adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

Excerpt [8] mentions that boycotting the Group Areas Act was difficult for those who owned property but easy for the propertiless. Some members of the "Movement" ignored the injunction to boycott and either appeared before the board themselves or sent their legal representatives to register their protest against the decision to have their areas reclassified.

[11] states that All those who accepted service on the C.A.C. (Coloured Advisory Council) were boycotted in every aspect of their lives, socially, economically, and otherwise. The boycott was trumpeted throughout the length and breadth of the country, wherever the Coloureds lived.

The Unity Movement's stance is also described in [18] where it says that the Movement's stand was to boycott the election, but if the “Coloured” voters insisted on voting they were asked to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip.


  ✅ Overall: 88.9% recall (token-overlap)  (185.8/209)  avg 26835ms
