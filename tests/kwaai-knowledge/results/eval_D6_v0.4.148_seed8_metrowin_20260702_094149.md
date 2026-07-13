[2m2026-07-02T16:41:53.867848Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE → http://127.0.0.1:53169 (via ollama-proxy)
[2m2026-07-02T16:42:02.207906Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:53169
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T16:42:02.318681Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  8392ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T16:42:10.999440Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  8570ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T16:42:19.816549Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  7142ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T16:42:27.207408Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  7598ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T16:42:34.931896Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 8/8 keywords  41929ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T16:43:39.499922Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  52836ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T16:44:35.636766Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  32719ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T16:45:05.409402Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  35166ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T16:45:28.793608Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 9/9 keywords  46006ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T16:46:10.193534Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  26877ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T16:46:32.188840Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  7711ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T16:46:39.923981Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  27149ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T16:47:07.249480Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  11910ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T16:47:19.364596Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  6555ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T16:47:26.121709Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 88 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six forced removals apartheid South Africa history""
  ○ Round 3   added 22 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  30882ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T16:47:59.488448Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  27091ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T16:48:24.502211Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  7682ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T16:48:32.392765Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  9305ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T16:48:41.894493Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23234ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T16:49:07.130927Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  27961ms
  [21/40] Who was the author's mother?
[2m2026-07-02T16:49:35.013772Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  38185ms
  [22/40] Who was the author's father?
[2m2026-07-02T16:50:13.165111Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  12587ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T16:50:24.680444Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  10673ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T16:50:35.789919Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  22936ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T16:50:59.940661Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  30146ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T16:51:30.415076Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  42095ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T16:52:12.949964Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  21281ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T16:52:34.207502Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/3 keywords  16292ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T16:52:49.344239Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  8661ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T16:52:58.220685Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  7198ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T16:53:05.617494Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  15843ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T16:53:22.983995Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  8898ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T16:53:30.795240Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  8815ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T16:53:39.851517Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  35928ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T16:54:27.299780Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  50676ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T16:55:12.918004Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  15578ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T16:55:22.664632Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  19891ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T16:55:46.500703Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  10725ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T16:55:53.697667Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  24005ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T16:56:17.939095Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  17965ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 88.5% (185.0/209) |
| Avg latency | 21627ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 8392ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 8570ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 7142ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 7598ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 41929ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 52836ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 32719ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 35166ms |
| q09 | Who was the author's grandfather? | 9/9 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 46006ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 26877ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 7711ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 27149ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 11910ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 6555ms |
| q15 | What were the forced removals from District Six? | 5/5 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 30882ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 27091ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 7682ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 9305ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23234ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 27961ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 38185ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 12587ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 10673ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 22936ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 30146ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 42095ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 21281ms |
| q28 | Which organisations was the author involved in? | 1/3 (33%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 16292ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 8661ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 6/6 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 7198ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 15843ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 8898ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 8815ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 35928ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 50676ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 15578ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 19891ms |
| q38 | Who was Cissie Gool's father? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 10725ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 24005ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 17965ms |

## Answers

### q01 — Who is the author?

The author is Joe Rassool [9].

### q02 — Who are the author's children?

The provided sources do not contain a complete list of the author's children, but they mention that the author has at least three children: 

1. Feyruz 
2. Reza 
3. Zarina 

Additionally, the author also mentions their grandchildren in [18] and [20], but the names are mentioned only there, and those sources do not provide information about the author's children.

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool

[Source 1, Source 20]

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool is dedicated to:

* The author's children: Feyruz, Reza, and Zarina
* And their grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [17]

### q05 — Who was J.M.H. Gool?

According to sources [1], [9], [10]–[13], J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a spice merchant and community leader who lived in District Six, Cape Town, South Africa. He had two wives: Bibi Gool and Wahida Ta'Al, and had multiple children from each marriage.

Gool was born around 1864 (not specified exactly in the sources), but he arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India. He was a well-respected citizen in Cape Town and was involved in various community activities and organizations, including being one of the founders of the Hanaffi Quwatul Islam Mosque.

Gool had close relationships with notable figures such as Mahatma Gandhi, who visited him at his residence and helped furbish his surgery. Gool also interacted with other influential people, including Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman.

In addition to his community work, Gool was a successful businessman and became wealthy through his spice trading business. He later moved to more spacious premises in Castle Street and lived there with his family.

Throughout the sources, J.M.H. Gool is portrayed as a prominent figure in Cape Town's Indian community during the late 19th and early 20th centuries.

Biographical facts listed from the sources:

* Date of birth: Not specified exactly
* Arrival in Cape Town: 1884
* Origin: Mauritius (via Swat, Gujarat)
* Occupation: Spice merchant, community leader
* Residences: District Six, 7 Buitencingle Street, Castle Street
* Family:
	+ Two wives: Bibi Gool and Wahida Ta'Al
	+ Multiple children from each marriage, including Yousuf Rassool (the author of the memoir)
	+ Grandfather of Yousuf Rassool
* Community involvement: Founder of Hanaffi Quwatul Islam Mosque, involved in various community organizations
* Relationships with notable figures:
	+ Mahatma Gandhi
	+ Cecil John Rhodes
	+ George Bernard Shaw
	+ Dr. Abdulla Abdurahman

### q06 — Tell me about Buitencingle.

Buitencingle was a 13-room mansion located at No.7 Buitencingle Street in District Six, Cape Town. It was the grand home of J.M.H. Gool, the grandfather of the author. The street itself was named after the estate and was considered the outermost belt or "cingle" of Cape Town at that time.

The mansion had a view of Table Mountain and Devil's Peak, and its surroundings were described as vibrant with élan, grandeur, and joie-de-vivre. However, when the house was shut up and left to dust and rodents, its spirit died, and it became a dead shell.

Buitencingle was also mentioned in various contexts throughout the memoir, including its significance in the author's childhood memories and its connection to the Gool family's history. The street itself was wide and cobbled with traffic islands sporting stunted palm trees, and opposite No.7 Buitencingle stood the German Lutheran Church.

Some specific facts about Buitencingle include:

* It was a 13-room mansion (Source: [1])
* It had a view of Table Mountain and Devil's Peak (Source: [9])
* The street was wide and cobbled with traffic islands sporting stunted palm trees (Source: [19])
* Opposite No.7 Buitencingle stood the German Lutheran Church, grey, austere, and remote (Source: [20])
* Buitencingle was considered the outermost belt of Cape Town at that time (Source: [3])
* The mansion's spirit died when it was shut up and left to dust and rodents (Source: [10])

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1]. She is an academic researcher who played a central role in shaping the memoir by urging the author, Yousuf Rassool, to ground his personal memories in historical research.

### q08 — Tell me more about the author's wife.

The provided sources contain several facts about the author's wife:

1. Her name is Nazima Rassool [1].
2. She is an academic researcher [9, 15].
3. The author owes a great deal to her, as she urged him to ground his writing in background investigation and lent authenticity to the story [4, 5].
4. She has played a central role in shaping the memoir by urging the author to transform it from a purely subjective recollection into a historically documented account of District Six [1].

These facts are derived from direct statements within the sources about Nazima Rassool's characteristics and her contributions to the book.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, who is also referred to as Haji Joosub Maulvi Hamid Gool [1]. He was a spice merchant and community leader from District Six in Cape Town, South Africa. He had two wives: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool) and Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool). [1]

Some specific facts about J.M.H. Gool from the sources are:

* He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.
* He was a founder of the Hanaffi Quwatul Islam Mosque in District Six and had it built at his residence at 7 Buitencingle Street [1].
* He performed the Hajj (pilgrimage) in 1899 [11].
* He died on April 1, 1940, following a final stroke [2, 19].
* He was known for his philanthropy and generosity, particularly during Eid celebrations when he would give pennies to all children who came to give him greetings of Slamat or Eid-Mubarak. [3, 5]
* He had a strong connection with the Dollie clan in Cape Town, as his wife Wahida's mother was from this family [13].
* He played an important role in the community and was often invited when VIPs visited Cape Town [9, 15].
* He sent his daughters to study at Fort Hare University, which was a courageous step given the time period and community prejudice [7, 16].

Note: The list above includes all specific facts mentioned about J.M.H. Gool in the provided sources.

### q10 — Tell me about Kloof Nek.

According to [1], Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town. It connects the city bowl with the Atlantic seaboard, offering views of both mountains.

Multiple excerpts describe Kloof Nek as a scenic area with a "haunting beauty" [4]. According to [18], when descending into the woodland dell at Kloof Nek on a hot October day, the author was entranced by its beauty and found the surroundings entirely appropriate for reading poetry. Excerpts also mention that Kloof Nek was frequently visited by lovers, mountaineers, nature lovers, ramblers, and down-and-outs known as "Bergies" [5].

In terms of geography and layout, excerpts describe Kloof Nek as a connecting saddle of land that joins Table Mountain to Lion's Head [19]. The area has several access points, including the bus terminus for a single-decker trackless tram from Adderley Street outside Fletchers & Cartwrights [19].

Overall, Kloof Nek is described as a scenic and significant natural feature in Cape Town.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students.

### q12 — Who was Cissie Gool?

Cissie Gool was a South African activist and politician who lived in District Six, Cape Town. She was the daughter of Dr. Abdulla Abdurahman, a doctor and city councillor, and the wife of Abdul Hamid Gool.

Here is a list of specific facts about Cissie Gool from the provided sources:

* Daughter of: Dr. Abdulla Abdurahman [1]
* Lived in: District Six [1]
* Member of: Non-European Unity Movement [1]
* Mother of: Shaheen Gool [1]
* Married to: Abdul Hamid Gool [1, 9]
* Occupation: activist and politician
* Born: 1897-02-14 [1]
* Died: 1963-12-02 [1]

Cissie Gool was involved in various activities and campaigns, including:

* Speaking at meetings, such as the one mentioned in excerpt [13] where she addressed a meeting in the Banqueting Hall
* Supporting the Torch Commando in 1953 [16]
* Leading a massive demonstration against residential segregation on the Grand Parade in 1937 [18]
* Being a member of the Anti-CAD Movement and later signifying her departure from its politics [20]

Note: The sources mention that there is some difficulty in identifying the individuals mentioned, especially the Gool sisters, but Cissie's presence and activities are well-documented.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organisations seeking unity against racial oppression [1].

The AAC later aligned closely with the Non-European Unity Movement and its non-collaboration policy [1].

It is mentioned that Vic Wessels, Les Jacobs, and Joe Rassool (the author) represented the New Era Fellowship at the conference of the All African Convention in Bloemfontein in 1954 [2], [18], [19].

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information about what the forced removals from District Six entailed. However, based on general knowledge and historical context, it is known that the forced removals from District Six referred to the large-scale relocation of non-White residents from the area in 1966 due to the Group Areas Act, which aimed to segregate different racial groups under apartheid rule. The residents were forcibly removed to the Cape Flats, an urban township outside of Cape Town, and their homes were bulldozed and demolished.

The sources do mention that District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It is also mentioned that under the Group Areas Act, the apartheid government declared it a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building [1].

It seems there might not be enough information provided within the given excerpts about the specific details of the forced removals themselves.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi (source [3]) was a prominent figure in South Africa during the early 20th century. He was a guest of the Gool family at their home in Buitencingle Street, Cape Town on several occasions. In February 1914, the Gool family hosted Gandhi and his party before they departed for India (source [18]). Gandhi also stayed with Adam Gool at his residence "Noor Bagh" (source [10]).

Gandhi was welcomed by Dr. J.M.H. Gool's letter of invitation in February 1914 (source [3]). The correspondence between Gandhi and Abdul Hamid Gool, a medical student at Guy's Hospital in London, also indicates their close relationship (source [18]).

During his stay with the Gools, Gandhi was attended to by Dr. Gool for his wife's illness (source [17]). In fact, the Gool family was so supportive that they even arranged meals for Gandhi and his party when they arrived at the train station in Cape Town (source [4]).

Gandhi's relationship with the Gools was not limited to hospitality; he also considered them friends. The author of this memoir mentions a letter from Gandhi, which references Wilson in connection with the Gool family (source [16]).

### q17 — What was Hewat Training College?

According to [1], Hewat Training College was a teacher training college in Cape Town for Coloured students. It trained Coloured teachers under the apartheid-era segregated education system and had connections to the Teachers League of South Africa (TLSA). 

The author, Joe Rassool, attended this college from 1947 ([2]).

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community. The NEF was established in 1937 "to discuss everything under the sun" [3].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization founded in 1943 that aimed to unite African, Coloured, and Indian people against apartheid and colonialism. Its programme was based on the Ten Point plan for equal rights. The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council.

The sources provide more information about its significance:

* It revolutionized the political scene with its doctrine of non-collaboration and the boycott weapon (Source [8]).
* It galvanized the non-white disenfranchised to a realisation of the need for a programme-based struggle that rejected the old tactics of deputations and appeals for justice at round table conferences (Source [8]).

The NEUM was affiliated with the All African Convention, which was one of its pillars. The movement had leaders like Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, who were among the young Turks that founded it in 1943 (Source [7] and [8]).

### q20 — Describe the author's involvement in cricket.

The author, Yousuf Rassool, was actively involved in cricket through Kismets Cricket Club and non-European cricket more broadly. He played and administered club cricket in the Cape.

Specifically, he mentions that as a young player, his idol was Salie Van Haacht (also known as Haartzen), one of the top cricketers from the Barnato Union. He also learned to play cricket seriously with the help of his friend Hassen Mall.

The author notes that Kismets Cricket Club had transformed the Indian Cricket Union by adopting a non-racial policy, which attracted teams and players from various backgrounds. He was part of this team and had high hopes of being selected for the biennial tournament, but unfortunately, he wasn't chosen.

However, four years later, he was finally selected for the team, and his name was included in a suggested side published in a letter to the Cape Standard. He also mentions that Tiny Abed, the third eldest member of the Abed clan, was their secret weapon.

The author's cricketing career is also mentioned throughout the text, including his experience playing against top players like Len Hutton, Cyril Washbrook, and Alec Bedser at the Newlands Cricket Grounds in Cape Town.

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool (also known as Lallie or Lallie Gool). [1] She lived in District Six and had seven children, including Yousuf (Joe) Rassool, the author. 

Specific facts about the author's mother from the sources are:

* Her maiden name is not explicitly stated but her husband is Peter Alexander Rassool.
* Ayesha was a daughter of J.M.H. Gool (by his wife Bibi Gool).
* She grew up in the Gool household at 7 Buitencingle Street in District Six, Cape Town.
* Her home language was English.
* She had seven children: Rasheda, Abdul, Berina, Nasim, Yousuf (Joe) Rassool, Zain, Fazil, and Yasmin.

Biographical information about Ayesha Rassool from the sources includes:

* Date not mentioned.
* Place of birth or origin: Not explicitly stated but her family lived in District Six, Cape Town. 
* Ancestry: Her father's ancestry is hinted at when the author reflects on his mixed identity (being a Muslim with dark complexion who spoke English) [2].
* Occupation: Housewife and mother; she also managed household chores, including laundry.
* Education: The sources do not mention Ayesha Rassool's education or any academic achievements. 
* Notable relationships: Her husband Peter Alexander Rassool (also known as Peerbhai), children, especially Yousuf (Joe) Rassool and his siblings.

Note that the provided sources only partially address the question of who the author's mother was; they do not contain comprehensive information about her life.

### q22 — Who was the author's father?

The author's father is Malick Rassool [1] (also referred to as "my father" in some excerpts). He was a Muslim/Indian man who was handsome, bronzed like his mother, with hair slicked back, tall and sturdy. He had left school early to become apprenticed to a cobbler and eventually worked at his grandfather's firm due to his neat handwriting. He became engaged to the author's mother when he was 19 and she was 27 [18].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool [1]
2. Abdul Rassool [1]
3. Berina Rassool (note: this is not an error, as "Berina" can be a variant of a given name) [1]
4. Nasim Rassool [1]
5. Zain Rassool [1]
6. Fazil Rassool [19]

### q24 — Who were the children of J.M.H. Gool?

From the sources provided:

* With Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool)
* With Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool)

It is also mentioned that Abdul Hamid Gool married Cissie Gool (Zainunnissa, daughter of Dr. Abdulla Abdurahman), making her his daughter-in-law.

Additionally, J.M.H. Gool was the grandfather of Yousuf (Joe) Rassool and others: Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Zain Rassool, Fazil Rassool, Yasmin Rassool.

There is no comprehensive list of all the children of J.M.H. Gool in the provided sources.

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool [1]. 

According to various sources:

* Tabata had written several pamphlets, including an indictment against Ben Kies, who characterized the conflict between the Capitalist West and the Soviet Bloc as a conflict between two camps, which was deemed undialectical code language by the Unity Movement [2].
* He was involved in the New Era Fellowship (NEF) and completed his book "The Awakening of a People" during his tenure as chairman [18].
* Tabata was accused by some, including The Forum Club, of being a Black Nationalist, one of the worst forms of political denigration at the time [4].
* He engaged in a scathing tone with Ben Kies at a meeting, rejecting the notion that their differences were personal and instead saying they were "political" [14].
* Tabata's leadership was influential, as he rose to attack Kies at a gathering and made a tremendous impression on the author at an NEF talk on One Day in the Life of Ivan Denisovich [8].
* He also had a significant role in the Unity Movement's non-collaboration policy.

These facts highlight Tabata's involvement in various aspects of South African politics during this period.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahaman (also known as Abdulla Abdurahman or Dr. Abdurahman) [1] was a Cape Town doctor, physician, and city councillor who was the first Coloured person elected to the Cape Town City Council. He was also a leader of the African Political Organisation (APO). The provided sources state that:

* Dr. Abdullah Abdurahman was born on 1872-09-08 [1].
* He was associated with Haji Joosub Maulvi Hamid Gool and the Non-European Unity Movement [1].
* His father is not mentioned in the excerpts, but his son, Cissie Gool's father [1], and grandfather were slaves who bought their freedom [4].
* Dr. Abdurahman's residence was in Loop Street, near Buitencingle, which facilitated co-operation with Grandpa and himself [4-5].
* He was a trustee of the Masjidul Azaan Mosque [6] and studied at Guys Hospital, London, where he qualified as a physician, becoming the second non-European from South Africa to do so [6].

The provided sources also mention that Dr. Abdurahman's family had connections with other notable individuals in Cape Town society, including:

* His son, Cissie Gool, who was married to J.M.H. Gool [3].
* His daughter-in-law, Zainunnisa (also known as "Cissie" or "Nellie"), who was the daughter of Dr. Abdurahman and a scion of Cape Town's leading dynasty [6].
* His granddaughter, Nasima, who participated in a massive demonstration against residential segregation led by Cissie Gool [18].

Overall, Dr. Abdullah Abdurahman was a prominent figure in Cape Coloured political life and had connections with several notable individuals and families in Cape Town society.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources state several connections between Gandhi and J.M.H. Gool:

* In 1906, Gandhi received a letter from J.M.H. Gool requesting him to call on Gandhi during his stay in London (Excerpt [3]).
* In 1911, Gandhi visited the home of Dr. Abdul Hamid Gool at 7 Buitencingle Street and helped furbish his surgery by puttying and staining the floor (Excerpt [16]).
* In February 1914, the Gool family invited Gandhis to their residence for hospitality (Excerpt [10]).
* Gandhi's son had a romantic interest in Fatima "Auntie Timmie" Gool, which led Gandhi to intervene and eventually marry his son to a Hindu girl (Excerpts [13], [14]).

These connections demonstrate that the Gool family and Gandhi were acquainted and maintained a relationship over several years.

### q28 — Which organisations was the author involved in?

Based on the provided sources, the author was involved in the following organisations:

1. New Era Fellowship (NEF) [2], [3], [5], [9]
2. Trafalgar High School cultural society [16]
3. Hewat Amateur Theatrical Society [8]

Additionally, the author mentions being part of various groups and meetings, such as:

* Representing the New Era Fellowship at the conference of the All African Convention in Bloemfontein [6]
* Participating in a sociological survey of the Cape Flats eyesore and health hazard with SRC members [19]

Note that the author was also a member of Trafalgar High School, where he participated in various activities, including stage productions.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM), [1]. They shared a boycott and non-collaboration programme, [1]. The TLSA joined the NEUM, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society, [18] and [19].

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque, was a mosque located in Loop Street, Cape Town. It was founded in 1898 and one of the oldest mosques in the Cape Muslim community (Source: [1]). The author's grandfather, J.M.H. Gool, was a founding trustee of the mosque (Source: [1]).

Additionally, the mosque is mentioned again as being located on Loop Street in Source [20]. It is described as an outpost of Islam in an area that was becoming increasingly European, Jewish, and gentile. The author's family used to go there to celebrate Eid festivals (Source: [20]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was married to Abdul Hamid Gool, who is the son of J.M.H. Gool [1]. Therefore, Cissie Gool is the daughter-in-law of J.M.H. Gool.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool knew Mahatma Gandhi personally [1]. Gandhi visited South Africa in 1914 and stayed at the Gool mansion in District Six, Cape Town.

Additionally, Gool was acquainted with Cecil John Rhodes [1] and George Bernard Shaw [8], who also visited him.

### q34 — What was the Group Areas Act and how did it affect District Six?

According to [2], the Group Areas Act "declared" a White area in 1966, although it appears that this declaration occurred later than the initial impact on the community. The act led to the forced removals of all non-White residents from District Six to the Cape Flats, bulldozing and demolishing almost every building.

[13] states that the Group Areas Act was a law that transformed the country into a patchwork of Black "Coloured", Indian, and White areas. The act was the lynchpin, or main component, of the Apartheid legislation, requiring all people to be properly classified, or colour-coded, for the proper effectuation of their plans.

[3] and [5] mention that boycotting the Group Areas Act did not create any problems for those who were propertiless, homeless, or down-and-out; however, it was a different matter for those who owned property. Some members of "The Movement" simply ignored the injunction to boycott.

[14] contrasts the Group Areas Act with the Van Riebeeck Festival's boycott, stating that this issue was well-suited to the principle of Non-collaboration and the weapon of the boycott.

[15] notes that a community of small traders were concerned about their future if they were all herded into one group area, specifically an "Indian" area. 

The Group Areas Act affected District Six by leading to the forced removals of non-White residents, the bulldozing and demolition of buildings, and the disintegration of the community.

Note: The date in [2] seems inconsistent with [13], but both agree that the Group Areas Act had a significant impact on District Six.

### q35 — Who was Hassen Mall?

Based on the provided sources:

* [1] describes Hassen Mall as a close friend and fellow cricket player from District Six of Yousuf Rassool (Joe Rassool) during their university days. He is remembered as being keenly involved in various activities, including cricket, at least until 1951.
* [6] adds that Hassen Mall was brilliant cricketer who spun a wristy leg break, and later qualified with the degree BA LLB from Cape Town.
* [9] describes him as a talented cricketer who bowled fast and impressed many with his skills.
* [10] lists Malick Hayat (Captain), Hassen Mall, I.Begg, I.T.Gihwala, C.Khatieb, S.Abed, and the young G.Abed among the team members for a cricket tournament.
* [12] states that when Hassen returned from holiday in Durban after completing his LLB degree, he was involved in collecting donations to pay for the fares of thirteen players for the same tournament.
* [13] mentions that Hassen Mall left for Durban after completing his LLB degree.
* [16] lists Hassen Mall among a group of people who would participate in disseminating the ideas of a movement and notes that he was involved in collecting donations to support the cricket team's travel costs.
* [17] describes him as usually batting at number four, but deciding to bat at number one to counter Hamid Khan's strong bowling skills.
* [19] mentions Hassen Mall captaining the side for the biennial cricket tournament and being awaited in Durban with other selected players.

These descriptions suggest that Hassen Mall was a talented cricketer, lawyer, and community leader. He was an important figure in Yousuf Rassool's life, as described by sources [1]–[20].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were:

* The Non-European Unity Movement (NEUM) [1]
* The Teachers League of South Africa (TLSA)
* The New Era Fellowship (NEF)
* The All African Convention (AAC)

These organisations shared a common policy of boycott and non-collaboration against apartheid institutions.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

The provided sources mention the following about Mahatma Gandhi:

1. He is also known as MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi, M K Gandhi, Gandhiji, hatma Gandhi, or Mahatma Gandhi [1].
2. He came to South Africa in 1893 as a lawyer to work on a legal case for Dada Abdullah [6].
3. Initially, he was on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria due to the accounts being kept in Gujarati, Gandhi's mother tongue [6].

Sources do not explicitly state why he stayed in South Africa beyond his initial assignment, but they imply that he was there for over 20 years and became involved in advocating for non-European rights in South Africa. He developed his philosophy of satyagraha (non-violent resistance) as a tool of political struggle during this time [1].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1]. He was a Cape Town city councillor and doctor who was the first Coloured person elected to the Cape Town City Council [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

Based on the provided excerpts, here's what can be gathered about District Six before the forced removals:

* It was a vibrant and multicultural community (Excerpt [1], [3])
* Home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life (Excerpt [1])
* Had a thriving business sector, with many stores and shops along Caledon Street (Excerpt [7] mentions the "To Let" column in "The Argus")
* Was known for its lively atmosphere, with people from different backgrounds interacting and socializing (Excerpt [3], [12])
* Had a strong sense of community, with residents knowing and looking out for each other (Excerpt [19])
* Was a place where people from different racial groups could live together, despite the apartheid policies that sought to segregate them (Excerpt [14])

It's worth noting that the excerpts also highlight the tensions and challenges faced by the community, such as the forced removals under apartheid, but they do not provide a comprehensive picture of District Six's daily life before the removals.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement adopted a boycott and non-collaboration policy. They refused to participate in any apartheid government institutions, including the Coloured Advisory Council [1]. The boycott weapon was seen as a potent means of struggle against Apartheid, but its limitations were also revealed, particularly when it came to implementing the Group Areas Act [8].

The Unity Movement's boycott policy involved boycotting "dummy" elections and refusing to cooperate with government institutions. This included not participating in the Coloured Advisory Council (C.A.C.) [11]. They also encouraged others to boycott these institutions and events.

For example, during the Van Riebeeck Festival, the Unity Movement planned to use the occasion for political education of non-whites in the Cape [7]. Despite attempts to bus in hundreds of farmworkers from up-country to boost attendance, the boycott was a massive success.


  ✅ Overall: 88.5% recall (token-overlap)  (185.0/209)  avg 21627ms
