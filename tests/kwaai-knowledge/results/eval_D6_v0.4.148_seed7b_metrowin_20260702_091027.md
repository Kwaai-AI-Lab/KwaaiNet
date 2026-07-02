[2m2026-07-02T16:10:30.280479Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE → http://127.0.0.1:51315 (via ollama-proxy)
[2m2026-07-02T16:10:34.289380Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:51315
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T16:10:34.407669Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  21574ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T16:10:56.215805Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  24561ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T16:11:20.992941Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  8775ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T16:11:29.970120Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  6584ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T16:11:36.761870Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/8 keywords  46295ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T16:12:23.248173Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  24979ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T16:12:48.448249Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  4431ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T16:12:53.068825Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  12236ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T16:13:05.515153Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/9 keywords  43823ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T16:13:49.544800Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/7 keywords  23701ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T16:14:13.449483Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  12181ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T16:14:25.835595Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  59895ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T16:15:26.057180Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  8609ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T16:15:34.765970Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  16039ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T16:15:51.010900Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 89 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → "forced removals from District Six South Africa history"
  ○ Round 3   added 16 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  18200ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T16:16:09.416756Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  26564ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T16:16:36.182345Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  7140ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T16:16:43.534247Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  25043ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T16:17:08.772933Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  23994ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T16:17:32.965739Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  27190ms
  [21/40] Who was the author's mother?
[2m2026-07-02T16:18:00.361345Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  8211ms
  [22/40] Who was the author's father?
[2m2026-07-02T16:18:08.777578Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/4 keywords  8594ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T16:18:17.582776Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  16805ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T16:18:34.583411Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/7 keywords  22034ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T16:18:56.821143Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  21359ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T16:19:18.396147Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  28448ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T16:19:47.032851Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  45902ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T16:20:33.143719Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/3 keywords  14015ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T16:20:47.357239Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  12726ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T16:21:00.283419Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  8972ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T16:21:09.459954Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  14397ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T16:21:24.065923Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/5 keywords  17285ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T16:21:41.554036Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  38918ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T16:22:20.675340Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  35040ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T16:22:55.913440Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  14049ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T16:23:10.203854Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/4 keywords  9526ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T16:23:19.892791Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 7/7 keywords  12131ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T16:23:32.226486Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  4918ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T16:23:37.356197Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/6 keywords  21714ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T16:23:59.267312Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/5 keywords  17972ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 90.0% (188.0/209) |
| Avg latency | 20370ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21574ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 24561ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 8775ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 6584ms |
| q05 | Who was J.M.H. Gool? | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 46295ms |
| q06 | Tell me about Buitencingle. | 6/8 (75%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 24979ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 4431ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 12236ms |
| q09 | Who was the author's grandfather? | 5/9 (56%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 43823ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 23701ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 12181ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 59895ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 8609ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 16039ms |
| q15 | What were the forced removals from District Six? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 18200ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Gool family] | 26564ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 7140ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 25043ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23994ms |
| q20 | Describe the author's involvement in cricket. | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 27190ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 8211ms |
| q22 | Who was the author's father? | 2/4 (50%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 8594ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 16805ms |
| q24 | Who were the children of J.M.H. Gool? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 22034ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 21359ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 28448ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 45902ms |
| q28 | Which organisations was the author involved in? | 2/3 (67%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 14015ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 12726ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 8972ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 14397ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 17285ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 38918ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 35040ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 14049ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 9526ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 12131ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 4918ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 21714ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 17972ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources do not contain that information.

However, from [14] and [18], we can see that Y.S. Rassool has children and grandchildren:

* From [14]: For my children: Feyruz, Reza, and Zarina  
* From [18]: Dedication ... For my children: Feyruz, Reza, and Zarina  
and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis.

It appears that Y.S. Rassool's full name is Joe Rassool, which matches the author of "LEST WE FORGET -rev25.pdf". 

Y.S. Rassool/Joe Rassool has children named Feyruz, Reza, Zarina, and grandchildren Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

Note: The provided sources do not contain any information about Yousuf (Joe) Rassool's (the author of "LEST WE FORGET -rev25.pdf") wife Lallie or her children Yasmin, nine; Abdul was eight, Fazil was six, Zainunissa (or Zaironesa as the Birth registrar recorded it) was four-ish, Rasheda was just about two, and Berina, the fretful baby, with large soulful eyes was only a year.

### q03 — Who are the author's grandchildren?

The provided sources do not contain a comprehensive list of all the author's grandchildren. However, based on the information given in [1] and the dedication page in [2], the following grandchildren are mentioned:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" is dedicated to the author's children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [4]

### q05 — Who was J.M.H. Gool?

Based on the provided sources, here is a comprehensive answer:

* Full name: Joosub Maulvi Hamid Gool
* Born: In Gujarat, India (specifically in Swat Province), but later moved to Mauritius and then Cape Town, South Africa.
* Origin: From Mauritius via Swat, Gujarat, India
* Father: A Pushto-speaking Maulvi (religious leader) who had two sons, Abbas and Yusuf
* Spoke Urdu at home, despite being in a region where Gujarati was the common language
* Married twice:
	+ First wife: Bibi Gool (married in 1879 or 1880)
	+ Second wife: Wahida Ta’al (mother of Hajima/Wahida, who later became his third wife)
* Had multiple children with both wives:
	+ With Bibi Gool:
		- Abdul Hamid Gool
		- Jane Gool-Tabata
		- Mariam Margie Gool
		- Minnie Gool
		- Zobeida Gool
		- Goolam Gool
		- Fatima Gool
		- Mohamed Saaid Gool
	+ With Wahida Ta’al:
		- Hajima/Wahida (later became his third wife)
* Moved to Cape Town, South Africa in 1884 and became a prominent figure in the Indian community.
* A spice merchant and community leader who was involved in various activities, including founding the Hanaffi Quwatul Islam Mosque in District Six.
* Had close relationships with notable figures such as Mahatma Gandhi, Dr. Abdulla Abdurahman, and Cecil John Rhodes.
* Was a founder trustee of the Wooding's Preparatory private school staffed by West Indian teachers.
* Contributed to various medical articles for the Indian Opinion.
* Participated in politics, serving as one of the joint secretaries of the Cape British Indian Union (later resigning) and helping to bring two rival organisations together.
* Was involved in efforts to improve the lives of Indians in South Africa and fought against discrimination.
* Died on April 1, 1940.

Sources: [1], [2], [3], [4], [5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15], [16], and [20].

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion located on 7 Buitencingle Street in District Six, Cape Town. The street was wide and cobbled with traffic islands sporting stunted palm trees [3]. It was at the time the outermost belt of Cape Town [19], named after its location as a belt around the city.

The mansion had a view of Table Mountain and Devil's Peak [1] and was a vibrant house, full of grandeur and élan, especially when the family that lived there were doing well [10]. However, it became a dead shell once the family stopped living there [10].

Buitencingle played an important role in the history of District Six and its inhabitants. The Gool family, who owned the mansion, was central to this history. They were known for their hospitality, particularly when they hosted Gandhi in 1914 at their home [6]. 

In addition to its historical significance, Buitencingle was also a place where the author spent time as a child. He remembered the garden with fruit trees and his siblings listening to his stories about "Mommy pear" and "Daddy pear" and "Baby pear" [5].

Eventually, Buitencingle fell into disrepair. The author's grandfather collapsed in Long Street due to a stroke [9] and never fully recovered from it. He eventually passed away.

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

The provided sources do not contain comprehensive information about the author's wife. However, we can extract some facts from the available excerpts:

1. The author's wife is Professor Nazima Rassool [1].
2. She is an academic researcher [15, 19].
3. She urged the author to ground his personal memories in historical research, which transformed the book from a purely subjective recollection into a historically documented account of District Six [1].
4. She has four sons [16].

We cannot determine her age, date of birth, place of origin, or any other biographical details beyond what is explicitly mentioned in the sources.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Haji Joosub Maulvi Hamid Gool. 

Specific facts mentioned in the sources:
- [1] - His name is listed as J.M.H. Gool, and other variations.
- [2] - He instituted a custom of giving pennies to children who came to give Eid greetings.
- [3] - He lived at 7 Buitencingle Street.
- [4] - He took his second wife, Wahida, and their four children on the Hajj pilgrimage in 1899.
- [5] - His son, Dr. Abdul Hamid Gool, was a doctor.
- [6] - He was often invited to visit VIPs who came to Cape Town.
- [7] - He sent his daughters, Janab and Zobeida, to study at Fort Hare.
- [8] - He was known for his broad-minded vision.
- [9] - During their absence on the Hajj pilgrimage, he was kept in touch with his affairs by Mr. Wilson, a young black American accountant.
- [10] - His ascent to prosperity was probably due to his industrious wife Wahida's support and the rapid economic growth following the discovery of diamonds and gold.
- [11] - He hosted Mahatma Gandhi at their home.
- [12] - He collected donations for the South African Indian Congress (League) and became a member.
- [13] - He was known as "Grandpa" to the family.
- [14] - A photograph of him is included, showing him with his second wife, Wahida, and their children.
- [15] - He sent his daughters, Janab and Zobeida, to study at Fort Hare, which was a courageous step in that time period.
- [16] - The Gool family played an important part in the non-white struggle for equality in South Africa.
- [17] - This fact is mentioned again, highlighting his vision of accepting the oneness of the human race.
- [18] - He died on April 1st, 1940, following a final stroke.
- [19] - Before moving to Cape Town, he lived in Mauritius and was accompanied by four companions when sailing to Cape Town.
- [20] - This source does not contain any new information about his grandfather.

### q10 — Tell me about Kloof Nek.

Based on the sources [1], [2], [4], [5], and [20], here is what can be gathered about Kloof Nek:

* It is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town.
* The area is a landmark of Cape Town's topography, offering views of both Table Mountain and Lion's Head.
* It has a connecting saddle of land that joins Table Mountain to Lion's Head (source [20]).
* Kloof Nek was the outermost belt of Cape Town at one time (source [8] and [20]).
* The area is known for its beauty, with a description of it as "a place of haunting beauty" (source [4] and [18]).
* It has been frequented by lovers, mountaineers, nature lovers, ramblers, and even down-and-outs (known as "Bergies") who found refuge in the area.
* There is a Table Mountain cable car that begins its run near Kloof Nek (source [5]).

It's worth noting that sources [17] and [18] describe a visit to Kloof Nek by the author, but these are likely describing the same location as mentioned above.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1].

It was described as a union that became political, which was considered an untenable situation by the authorities because teachers were regarded as leaders in the community [19]. This led to attempts to restore the status quo by creating another organisation, the Teachers' Educational and Professional Association (TEPA) [19].

### q12 — Who was Cissie Gool?

Based on the provided sources [1]–[20], here are the specific facts about Cissie Gool:

* Full name: Zainunnissa Cissie Gool
* Also known as: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool, Zainunnissa
* Born: 1897-02-14
* Lived in: District Six
* Member of: Non-European Unity Movement
* Mother of: Shaheen Gool
* Married to: Abdul Hamid Gool
* Nationality: South African
* Occupation: Loved (no further explanation provided)
* Died: 1963-12-02

Cissie Gool was the daughter of Dr. Abdulla Abdurahman, a physician and city councillor, and a leader of the African People's Organisation (APO). She was a Cape Town city councillor, lawyer, and prominent political activist closely associated with the Non-European Unity Movement and the anti-apartheid struggle.

Sources [2]–[5] provide additional information about Cissie Gool's life and activities:

* Source [2] mentions that she was confident in front of the camera lens.
* Source [3] states that she was a member of the Unity Movement, whose stand on Whites-only elections was "A plague on both your houses."
* Source [4] lists her among luminaries who met with Dr. Gool (presumably Abdul Hamid Gool), including Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, and others.
* Source [5] describes her as a "bitter enemy" of the speaker Piccadilly, who was known for his gravelly growl and asthmatic wheeze.

Sources [6], [7], [9]-[11], [14], [16], and [18] provide further details about Cissie Gool's life:

* Source [6] mentions that she served tea at her Mount Street residence in the 1930s, with Nellie Abdurahman.
* Source [7] quotes Dr. Gool as saying that Indians are right not to tie up their cause with that of other colored people, which contradicts Cissie Gool's views and possibly contributed to the break-up of her marriage.
* Source [9]-[11] mention that she was a central figure in the history of District Six and the Indians and Coloured people of South Africa, and that she submitted subscriptions to the Indian Opinion on behalf of Mr. Wilson.
* Source [14] describes her addressing a meeting against influx control and street gangs (skolly problem), which shocked the author of the text.
* Source [16] repeats this information and mentions that Cissie Gool's actions signaled her departure from the politics of the Unity Movement.
* Source [18] describes her as a passionate orator who rallied the vote-less masses against residential segregation.

Source [19] provides an account of Cissie Gool's speech in support of measures to deal with 'influx control' and street gangs, which shocked the author and signaled her departure from the Unity Movement.

### q13 — What was the All Africa Convention?

According to [1] All African Convention [Organization], the All-African Convention (AAC) was an organization founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organizations seeking unity against racial oppression and later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

It was known for its political struggle against apartheid and had a history of being at the forefront of the struggle for democracy [3]. The area was also described as having "more life, more zest, more fire, more warmth and care than many other places one could mention" [4], indicating a strong sense of community and cultural identity.

By the 1950s, District Six was a thriving cosmopolitan area in the heart of Cape Town, but it was eventually dispersed under the Group Areas Act (1950) with its people being forced to move to the Cape Flats [10].

### q15 — What were the forced removals from District Six?

The sources do not explicitly describe the forced removals themselves, but they mention that under the Group Areas Act, the apartheid government declared District Six a White area in 1966 and carried out the forced removals of all non-White residents to the Cape Flats [1]. They also state that these removals destroyed a thriving community [1] and caused misery beyond anything the SRC members knew [10].

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi, or Mahatma Gandhi, was a prominent Indian independence activist who spent over 20 years in South Africa pioneering the philosophy of satyagraha (non-violent resistance). The provided sources describe his connections with the Gool family as follows:

* Gandhi visited the Gool family's home at 7 Buitencingle Street in Cape Town on several occasions, including a farewell visit to India in February-March 1914 [3], and during this time he was treated by Dr. Abdul Hamid Gool (also known as Uncle Doctor) [2].
* Gandhi was a guest of the Prime Minister Botha at Groot Schuur, but also visited the Gool residence on several occasions, where he received medical care from Dr. Gool [3].
* The Gool family hosted various dignitaries and activists, including Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman [1], suggesting their home was a hub of activity for anti-colonial struggles.
* Gandhi extended his contacts in the community, particularly among the poorer sections and devoted much of his time to public work for Indians [5].
* The Gool family was connected to national and international anti-colonial struggles, with J.M.H. Gool leading deputations and lobbying support against proposed legislation [4].

The sources also mention Gandhi's close relationship with Dr. Abdul Hamid Gool (Uncle Doctor) during his stay in London for medical school, as evidenced by their correspondence [18].

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained Coloured teachers under the apartheid-era segregated education system. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement [1]. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1]. The NEF was also referred to as an organisation that had been established in 1937 "to discuss everything under the sun" [3].

The New Era Fellowship is mentioned throughout various excerpts as an active participant in political movements and discussions. It is associated with notable figures such as I.B. Tabata, Ben Kies, Hassan Bavasah, and others who played significant roles within the organisation.

Some specific details about the NEF include:

* The NEF held public lectures on a variety of topics [4].
* The organisation was involved in distributing Mr. Tabata's book "The Awakening of a People" [6].
* The NEF had its own programme of events, which Hassan Bavasah oversaw as Organising Secretary [12].
* The NEF hosted meetings at the Stakesby-Lewis Hostel and other locations.
* The organisation was associated with the Unity Movement and Non-European Unity Movement [1].

The New Era Fellowship seems to have played a significant role in promoting intellectual and political discussions within the Coloured community, as well as advocating for social justice and equality.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organisation that aimed to unite African, Coloured, and Indian people against apartheid and colonialism in South Africa [1]. It was founded in 1943 with a programme based on the Ten Point plan for equal rights [1].

According to Yousuf Rassool's account, the NEUM adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council [1]. The Teachers League of South Africa (TLSA) was affiliated with the NEUM and shared its boycott and non-collaboration programme [1].

The NEUM played a significant role in the struggle for liberation in South Africa. It galvanised the non-white disenfranchised to a realisation of the need for a programme-based struggle that rejected the old tactics of deputations and appeals for justice at round table conferences [8]. The movement was led by various individuals, including those from Natal and Transvaal [11].

The NEUM's ideology included the principle of non-collaboration and the refutation of "race" and the ideal of a non-racial society in the struggle for liberation. Although its ideals were not achieved as expected, the movement had a lasting impact on South African history [14].

### q20 — Describe the author's involvement in cricket.

The author was actively involved in cricket through Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in Cape Town. 

As a player, he was known for his forward defensive stroke, which Edross praised as one of the best he had seen [17]. However, he struggled to get selected for teams, being left out of the Western Province Indian Cricket Union side despite having made 12 runs in an innings that could have won the match [9].

He organized a meeting to start a cricket union at his school, Hewat, which would facilitate playing cricket at High School and College level [11]. He also tried to organize a team for the biennial tournament but was thwarted by the bureaucracy of the Schools' Sports Union.

The author was part of Kismets Cricket Club, where he played alongside other notable cricketers. The club had a non-racial policy that attracted teams from different ethnicities, including Seventh Day Adventists and Muslims [16]. 

He developed close friendships with his cricket colleagues, including Hassen Mall, Edross, Achmat Hayat (Achee), and Isak Esmail [17]. His friendship with Muddy Allie was particularly close, as they became friends through their shared passion for cricket.

The author's cricketing ambitions were also thwarted by selectors not seeing his innings at Rosmead. However, he did get selected four years later, captained by Hassen Mall, and played alongside other talented cricketers [18].

### q21 — Who was the author's mother?

The author's mother was Ayesha Gool (also known as Lallie or Ayesha Rassool), [1]. She had a sister named Dija and was married to Peter Alexander Rassool, who became known as Peerbhai or Peru. Her parents were Bibi Gool and Haji Joosub Maulvi Hamid Gool.

### q22 — Who was the author's father?

The author's father was Malick Rassool [4], [14]. 

Additional information: He had a Muslim/Indian background, but his children were reared and baptized in the Wesleyan Church where their parents were married [14].
He had six children with Magdalena Isabella [14]. 
He died after 1925 in a road accident [11], [14].

### q23 — Who were the author's siblings?

According to [1], the author's siblings are:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool (also known as Jessie)

Also, from [1], the author's father is Peter Alexander Rassool and his mother is Ayesha Rassool.

Additionally, from [16] and [20], some of the family members mentioned include:

* Lallie (the author's mother)
* Jolly
* Dija
* Toetie
* Cissie Gool (author's aunt or cousin, depending on the context)

However, it is unclear whether these individuals are part of the Rassool family or a separate family.

### q24 — Who were the children of J.M.H. Gool?

According to the sources, the children of J.M.H. Gool (Joosub Maulvi Hamid Gool) are listed in [1] as follows:

From his first wife Bibi Gool:
Ayesha Rassool
Mohamed Saaid Gool
Zobeida Gool
Goolam Gool
Fatima Gool
Mariam Margie Gool (also known as Mariam Gool)
Abdul Hamid Gool

From his second wife Wahida Gool:
Toetie Ally (also known as Hanief Ally or Toetie Aity)
Hanief Ally (son, not mentioned in [1] but mentioned in other sources)
Other children from the various lists and mentions are:

Rasheda Rassool
Abdul Rassool
Nasim Rassool
Yousuf Rassool (the author of this memoir)
Zain Rassool
Fazil Rassool
Yasmin Rassool

Note that some sources mention other children, but these are the ones explicitly mentioned in [1] and/or other reliable sources.

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool, and was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].

Tabata qualified as BA LLB and gave a talk at the NEF on One Day in the Life of Ivan Denisovich that made a tremendous impression on the author [8]. He completed his book "The Awakening of a People", which was an occasion of considerable excitement as it was to be an NEF publication [9] and [18].

Tabata's differences with Kies were not personal but political. The latter put forward a plea to put aside personal differences for the sake of the Movement, but Tabata rose from his seat and responded scathingly, saying "Personal differences! It is an insult to say I have personal differences" [14] and [20].

Tabata was also accused by the Forum Club, who regarded themselves as Marxist Trotskyists, of failing to have a class analysis in his historical tract [4] and [6].

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman (also known as Abdulla Abdurahman) was a Cape Town physician and city councillor [1]. He was the leader of the African Political (later People's) Organisation (APO) [1]. As a city councillor and APO leader, he was a pioneering figure in Cape Coloured political life [1].

He was the father of Cissie Gool [1] and was known for his work with the Non-European Unity Movement [1].

Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [4]. He met Dr. Abdurahman in Loop Street, where Abdurahman lived [5]. They had informal discussions and collaborated on various projects between their organisations [5].

Dr. Abdurahman was the second of non-Europeans from South Africa to qualify as a physician [6], studying at Guys Hospital in London [6]. In 1911, his eldest son took over the ownership of Buitencingle upon returning from medical school [6].

Sources list various notable acquaintances and organisations associated with Dr. Abdurahman, including:

* Associated with: Haji Joosub Maulvi Hamid Gool, Non-European Unity Movement
* Father of: Cissie Gool
* Born: 1872-09-08
* Died: 1940-02-20

He was a prominent figure in Cape Town and played a significant role in the political life of the city.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources do not describe a single specific connection between Gandhi and J.M.H. Gool [1], but rather multiple connections and interactions. Here are some examples:

* Gandhi visited Gool's home at 7 Buitencingle Street in Cape Town, where he helped putty and stain the floor of Dr. Gool's surgery in 1911 [8].
* Gool was a frequent guest at Gandhi's homes and events, and Gandhi often stayed with him as well.
* The two men had a close relationship, with Gandhi referring to Gool as "my friend" and Gool responding with similar affection.
* Gandhi also wrote about the importance of his relationships with people like Gool in his writings [9].
* There are multiple instances where Gandhi visited or interacted with members of the Gool family, including Dr. Gool's children and grandchildren.

Some specific examples of their interactions include:

* In 1911, Gandhi helped to furbish Dr. Gool's surgery by putting and staining the floor.
* In February 1914, the Gool family hosted a visit from Gandhis in their home at 7 Buitencingle Street [10].
* In 1906, Gandhi attended a meeting of the Coloured People's Vigilant Committee, where he met with Gool [14].

Overall, it appears that Gandhi and J.M.H. Gool had a close personal and professional relationship, based on multiple instances of interaction and mutual respect.

Additional facts about Gandhi and J.M.H. Gool can be gathered from the following sources:

* They shared an interest in social justice and community service.
* Gandhi often sought out Gool's advice and counsel on matters related to Indian affairs in South Africa.
* The two men collaborated on several projects, including the Cape British Indian Union.
* Gool was one of the few individuals who could match Gandhi's level of commitment to social activism.

Their relationship is described in various sources as follows:

[1] Haji Joosub Maulvi Hamid Gool [Person]
[8] perplate  script  recording 
deliveries of goods to the drifts across the Vaal River.
6 Surat was known for fabrics especially silk.
4 Indian Opinion, June 10, 1905 
   Courtesy National Gandhi Museum
[9] new force had been born into colonial life’.  
 
  
22 Letter from J. M. H. Gool to M. K. Gandhi  23rd January 1897 
     First of 7 pages. (see further pages in Appendix) Copy courtesy Mr. E.S. Reddy

### q28 — Which organisations was the author involved in?

The author, Joe Rassool, was involved in several organisations. These include:

* The New Era Fellowship (NEF) [2, 4, 6]
* The Teachers League of South Africa (TLSA) [11, 10]
* The Anti-CAD (Anti-Coloured Affairs Division) [8]
* The All African Convention (AAC) [16]

Additionally, the author mentions that his cousin, Nina, was involved with Neville Alexander and others in the Cape Peninsula Students Union, which suggests that the author may have been involved with this organisation as well.

The author also notes that he met Isaac Pfaff, who ran the Trafalgar Players, through his involvement with the Hewat Amateur Theatrical Society.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources state that:

* The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement [1].
* They shared a boycott and non-collaboration programme [1].
* The TLSA approached the Congress movement suggesting a unified strategy to counter the anti-educational plans of the regime, but the Congress leadership declined the invitation to cooperate [11].

This indicates that the TLSA had close ties with the Non-European Unity Movement and shared similar goals and strategies. However, it also suggests that there may have been some level of cooperation or interaction between the two organizations, particularly in their efforts to counter anti-educational plans by the regime.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1] Haji Joosub Maulvi Hamid Gool, he arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India. 

The origin chain for J.M.H. Gool is:
- Original homeland: Swat (via) Gujarat, India
- Prior location: Mauritius

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was a mosque located on Loop Street, Cape Town. It was founded in 1898 [1]. The mosque is central to the history of the Gool family and the Cape Muslim community in District Six.

A description of the mosque can be found in excerpt [20], which describes it as "an outpost of Islam in an area becoming increasingly European, Jewish, and gentile" bounded by Long Street, Buitencingle, and Loop Street. The mosque was a whitewashed structure with a domed minaret topped by a spire with a moon and star, from which the muezzin called the faithful to prayer [20].

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was the wife of Dr. Abdul Hamid Gool, who was a son of J.M.H. Gool [1]. Therefore, Cissie Gool was the daughter-in-law of J.M.H. Gool and the sister of Yousuf (Joe) Rassool, the author of this memoir [1].

Additionally, according to excerpt [4], Cissie Gool was also an elder sister of Ayesha Rassool's mother [4], making her Yousuf (Joe) Rassool's aunt by marriage.

From excerpt [10] we can confirm that Cissie Gool's husband Dr. Abdul Hamid Gool is a son of J.M.H. Gool, thus making Cissie his wife and the sister-in-law of J.M.H.'s other children [10].

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The following are the notable historical figures that J.M.H. Gool knew personally, based on the provided sources:

1. Mahatma Gandhi: According to [9], Gandhi helped to furbish J.M.H. Gool's surgery by puttying and staining the floor during his visit to Cape Town in 1911.
2. Dr. Abdurahman: As mentioned in [4], Dr. Abdurahman was a prominent figure in the community, and his grandfather was brought to South Africa as a slave. Dr. Abdurahman's family lived nearby J.M.H. Gool's residence in Buitencingle Street.
3. Cecil John Rhodes (implied): Although not directly stated, [9] mentions that Gandhi stayed at 7 Buitencingle Street during his visit to Cape Town, which was J.M.H. Gool's residence. Since Cecil John Rhodes was the Prime Minister of the Cape Colony from 1890 to 1896 and again from 1899 to 1902, it is likely that he would have been acquainted with Gandhi and possibly J.M.H. Gool.
4. George Bernard Shaw (implied): Similarly, since Shaw visited South Africa in 1931-1932 [not mentioned explicitly], it is possible that he would have met or interacted with J.M.H. Gool.
5. Cissie Gool: As mentioned in [3] and [8], Cissie Gool was a close relative of J.M.H. Gool's family, and her father-in-law was Abdul Hamid Gool, who was J.M.H. Gool's son.
6. Ben Kies, Solly Edross, Aunt Jane: These individuals are mentioned as being on the platform with Dr. Gool (J.M.H. Gool) during meetings in [8].
7. Advocate Christopher: Mentioned in [15] as someone whose name seems to have been omitted from history books.

Please note that the list of notable historical figures may not be exhaustive, as the sources do not provide a comprehensive account of J.M.H. Gool's acquaintances and interactions.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was a law passed in 1950 by the South African government, which aimed to segregate different racial groups into separate areas. Under this act, the apartheid government declared that certain areas would be designated for specific racial groups, and non-whites were forcibly removed from their homes in District Six, Cape Town.

According to [2], "A potent source of misery occurred when a Coloured Area was declared White: the house prices plummeted as the new buyers were aware that the Coloured owners, who realized that the Group Areas Board’s valuation of their properties was generally below the market value, were obliged to accept any paltry offer from prospective White purchasers."

[9] describes District Six: "The houses were all without ceilings or internal doors; ovens in summer and ice chests in winter. For years the roads were mere mud tracks with hardly any street lighting." 

This act devastated the lives of hundreds of thousands of people, including those living in District Six, as they were forcibly removed from their homes and relocated to other areas. The community was destroyed, and the once-thriving neighborhood was left empty and abandoned.

The Group Areas Act also had a significant impact on the economy and social structure of the area. According to [18], "Most were for people in the lower income group, but the awful thing about these schemes was that they resulted in the inhabitants losing the municipal franchise because they were living in sub-economic houses!"

[20] states: "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

The Group Areas Act was a key component of the apartheid regime's policy of segregation and dispossession. It had far-reaching consequences for the people of District Six, leading to the destruction of their homes, communities, and ways of life.

### q35 — Who was Hassen Mall?

According to the sources, Hassen Mall was:

* A close friend of the author Yousuf Rassool [1]
* A cricketer from District Six in Cape Town [1]
* Studied law at the University of Cape Town and completed his LLB degree [6]
* Played for the Western Province Indian Cricket Union [13, 16]
* Was the chairman of the Athlone Branch of the TLSA (Trafalgar League Students' Association) [12]
* A key figure in organizing a cricket tournament and collecting funds to finance it [15, 18]

He was also described as having a "striking" presence, being a brilliant cricketer, and having a kind personality [6].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were: 

- The Non-European Unity Movement (NEUM) [1]
- The Teachers League of South Africa (TLSA) [12]
- The New Era Fellowship (NEF) [15] 
- The All African Convention (AAC) [1]

These organisations were united by the boycott and non-collaboration policy against apartheid institutions.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to source [1], Mohandas Karamchand Gandhi (also known as Mahatma Gandhi) came to South Africa in 1893 as a lawyer to work on a legal case. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance, or passive resistance) as a tool of political struggle.

The author does not mention specific reasons for Gandhi's initial visit beyond the legal case, but sources [1] and [16] imply that Gandhi's long-term presence in South Africa was related to advocating for Indian rights.

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to [1], District Six was "a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak." It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

[2] mentions that Auntie Dij had a child named Shawquet, and the author would often help her with him. The tone suggests a close-knit community where children played together.

[3] describes how the author was a primary school teacher in District Six, teaching Standard fives (a class of students). He notes that he tried to instill ideals of liberty and equality in his students by teaching them about African wars instead of "Kaffir" wars, indicating an effort to promote understanding and respect for different cultures.

[4] mentions a community of freed slaves from America and the West Indies who thrived in District Six in Victorian times. This adds to the idea that District Six was a culturally diverse neighborhood with a rich history.

Overall, before the forced removals, District Six appears to have been a vibrant, multicultural neighborhood where people from different backgrounds lived together and created a thriving community culture.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement adopted a boycott and non-collaboration policy. They refused to participate in any apartheid government institutions, including the Coloured Advisory Council [1]. The Teachers League of South Africa (TLSA) was affiliated with the NEUM and shared its boycott and non-collaboration programme [1]. 

In the context of the Group Areas Act, the Unity Movement instructed members to boycott and refuse to cooperate with the Group Areas Boards without any guarantee that this would stop the implementation of the law. Some members simply ignored the injunction to boycott [8], while others appeared before the board or sent legal representatives to register their protest against the decision to have their areas reclassified [16].

Additionally, when it came to the Van Riebeeck Festival in 1952, the Unity Movement used this occasion for political education of non-whites and to "contradict the historical fabrications" presented in school textbooks. The boycott was a massive success for the NEUM [15].


  ✅ Overall: 90.0% recall (token-overlap)  (188.0/209)  avg 20370ms
