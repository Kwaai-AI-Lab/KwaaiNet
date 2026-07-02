[2m2026-06-24T19:10:46.042450Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:57585 (via ollama-proxy)
[2m2026-06-24T19:10:46.591873Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:57585
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-06-24T19:10:46.697552Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/3 keywords  9596ms
  [ 2/40] Who are the author's children?
[2m2026-06-24T19:10:56.534206Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  2396ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-06-24T19:10:59.147850Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 6/6 keywords  3016ms
  [ 4/40] To whom is the book dedicated?
[2m2026-06-24T19:11:02.382488Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  2623ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-06-24T19:11:05.208078Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/8 keywords  6994ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-06-24T19:11:12.424970Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/8 keywords  4615ms
  [ 7/40] Who is the author's wife?
[2m2026-06-24T19:11:17.243698Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/3 keywords  2238ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-06-24T19:11:19.683520Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/6 keywords  3674ms
  [ 9/40] Who was the author's grandfather?
[2m2026-06-24T19:11:23.565591Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/9 keywords  2343ms
  [10/40] Tell me about Kloof Nek.
[2m2026-06-24T19:11:26.106236Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 6/7 keywords  5355ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-06-24T19:11:31.663457Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  5287ms
  [12/40] Who was Cissie Gool?
[2m2026-06-24T19:11:37.155762Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/6 keywords  5116ms
  [13/40] What was the All Africa Convention?
[2m2026-06-24T19:11:42.473281Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  2847ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-06-24T19:11:45.523798Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/6 keywords  4553ms
  [15/40] What were the forced removals from District Six?
[2m2026-06-24T19:11:50.279230Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 1 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → ""District Six apartheid forced evictions South Africa history""
  ○ Round 3   added 18 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/6 keywords  13442ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-06-24T19:12:03.927103Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/7 keywords  4732ms
  [17/40] What was Hewat Training College?
[2m2026-06-24T19:12:08.862720Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/5 keywords  3043ms
  [18/40] What was the New Era Fellowship?
[2m2026-06-24T19:12:12.103287Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/6 keywords  4039ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-06-24T19:12:16.347007Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  5133ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-06-24T19:12:21.682671Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  5721ms
  [21/40] Who was the author's mother?
[2m2026-06-24T19:12:27.606443Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  2217ms
  [22/40] Who was the author's father?
[2m2026-06-24T19:12:30.025321Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/4 keywords  2098ms
  [23/40] Who were the author's siblings?
[2m2026-06-24T19:12:32.327261Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 5/5 keywords  2913ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-06-24T19:12:35.444440Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  4216ms
  [25/40] Who was I.B. Tabata?
[2m2026-06-24T19:12:39.862481Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/5 keywords  4356ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-06-24T19:12:44.421455Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/6 keywords  4612ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-06-24T19:12:49.239863Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  4714ms
  [28/40] Which organisations was the author involved in?
[2m2026-06-24T19:12:54.155273Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 2/5 keywords  3143ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-06-24T19:12:57.500540Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/6 keywords  3430ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-06-24T19:13:01.133757Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  2221ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-06-24T19:13:03.559282Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 5/6 keywords  4415ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-06-24T19:13:08.177452Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  3562ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-06-24T19:13:11.939978Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 2/5 keywords  4354ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-06-24T19:13:16.493596Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/6 keywords  5966ms
  [35/40] Who was Hassen Mall?
[2m2026-06-24T19:13:22.666045Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/4 keywords  4057ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-06-24T19:13:26.923475Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 1/6 keywords  4013ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-06-24T19:13:31.143711Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 5/7 keywords  3232ms
  [38/40] Who was Cissie Gool's father?
[2m2026-06-24T19:13:34.578529Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/5 keywords  2494ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-06-24T19:13:37.276805Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/6 keywords  5892ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-06-24T19:13:43.369074Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m436 [3mrelations[0m[2m=[0m209

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/5 keywords  3406ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 56.9% (128.0/225) |
| Avg latency | 4301ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 9596ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 2396ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 3016ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 2623ms |
| q05 | Who was J.M.H. Gool? | 3/8 (38%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 6994ms |
| q06 | Tell me about Buitencingle. | 4/8 (50%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 4615ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 2238ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 3674ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 2343ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf | 5355ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 5287ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 5116ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 2847ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 4553ms |
| q15 | What were the forced removals from District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 13442ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | [Graph: Gool], LEST WE FORGET -rev25.pdf | 4732ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 3043ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 4039ms |
| q19 | What was the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 5133ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 5721ms |
| q21 | Who was the author's mother? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 2217ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 2098ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 2913ms |
| q24 | Who were the children of J.M.H. Gool? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 4216ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 4356ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 4612ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 4714ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 3143ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 3430ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 2221ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 4415ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 3562ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 4354ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Population Registration Act] | 5966ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 4057ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 4013ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 3232ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 2494ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 5892ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 3406ms |

## Answers

### q01 — Who is the author?

The author is Joe Rassool [1].

### q02 — Who are the author's children?

The author, Yousuf (Joe) Rassool, mentions his children in the Dedication section of the book: Feyruz, Reza, and Zarina.

### q03 — Who are the author's grandchildren?

The provided sources do not contain a comprehensive list of the author's grandchildren. However, they mention that the author, Yousuf (Joe) Rassool, dedicates his book "District Six: Lest We Forget" to his children and grandchildren.

From the excerpts, we can identify some of the author's grandchildren as:

* Joseph
* Samuel
* Alexander
* Leonardo
* Dylan
* Lewis

### q04 — To whom is the book dedicated?

The book "District Six - Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [14]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool is mentioned throughout the sources as a prominent figure in South African history, particularly in Cape Town's District Six community.

From various sources [1-20], we can gather the following information about J.M.H. Gool:

* He was born around 1860 and had multiple wives: Bibi (whom he married in 1879 or 1880) and Wahida Ta'Al, whose mother was a Dollie.
* He was an Oriental trader who participated in British Justice and held a standing in the Cape community. [20]
* He was a witness in a case alongside Adam H.G. Mahomed in 1892. [20]
* In 1895, he joined the Wesleyan W.B. Shaw's South African Christian Political Association (SACPA) with J. Wilson and James Curry. [18]
* He attended meetings related to the "Coloured" vote and was present at a meeting chaired by Adam Mohamed, President of The British Indian League. [17]
* His family was associated with Dr. Abdurahman, who would later become a prominent figure in the community.
* J.M.H. Gool submitted articles to journals, including medical articles on "Modern Footwear" and "Tuberculosis". [14]
* He had business partners, one of whom might have been an African American called Mr. Wilson (although this is not confirmed). [9] and [12]
* His world outlook was constructed less around the tenets of his religion and more around the vision of a non-racial democracy under the umbrella of the British Empire. [9]

It's worth noting that J.M.H. Gool is also mentioned in various other contexts, such as being associated with Hanaffi Quwatul Islam Mosque, having grandchildren who were involved in the District Six community, and being an important figure in the history of Cape Town's District Six.

However, there is limited information available about his personal life, family, or specific achievements.

### q06 — Tell me about Buitencingle.

Buitencingle is a street located in Cape Town, South Africa. According to the sources, it was a wide and cobbled street with traffic islands in the middle, sporting a few stunted palm trees [9]. The street was bounded by Long Street, Loop Street, and Waterkant Street.

The sources also mention that Buitencingle was an area becoming increasingly European, Jewish, and gentile. It had a mix of cultures and religions, with the Hanaffi Quwatul Islam Mosque being located on Loop Street [20].

At the turn of the century, Buitencingle must have been vibrant with élan, grandeur, and joie-de-vivre [1]. However, by the time the author arrived with his family, the house at No. 7 Buitencingle Street had been shut up for a while, and its spirit seemed to be dying [20].

The area was also described as having a "sylvan glen" or a wooded valley nearby, which was a place of haunting beauty where one could wander down and enjoy the sights and sounds of nature [8].

### q07 — Who is the author's wife?

According to Excerpt [20], the author's wife is Nazima.

### q08 — Tell me more about the author's wife.

According to Excerpt [2], the author's wife is an academic researcher, but her name is not mentioned. She urged the author to ground his writing in background investigation, which led to some "staggering information" that altered the story qualitatively.

In Excerpt [16], it is mentioned that the author's grandfather had a second wife named Wahida, and the author's mother, Lallie, was pregnant again. However, there is no mention of the author's current or previous wives in this context.

However, in Excerpt [17], it is mentioned that the author "owes a great deal to my wife, Nazima", who insisted that he do some research to lend authenticity to the story.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool [1], also known as "Grandpa" [16].

### q10 — Tell me about Kloof Nek.

According to excerpts [1], [2], and [18], Kloof Nek is a place that joins Table Mountain to Lion's Head, located on the eastern flank of Cape Town with its head gazing down on Table Bay. It has a bus terminus for a single-decker trackless tram that runs from Adderley Street outside Fletchers & Cartwrights.

Excerpt [3] describes Kloof Nek as "a woodland dell" and mentions that it was a place of haunting beauty, where the author and his friend read poetry together. It also states that Kloof Nek was much frequented by lovers, mountaineers, nature lovers, and ramblers.

Excerpts [1], [2], and [19] describe the area around Kloof Nek, mentioning Signal Hill from which the noonday gun heralded midday with regularity and a puff of smoke. The Glen at Kloof Nek was mentioned as a place where people would go to study poetry before exams.

Excerpt [15] mentions that Persotem Patel and Khadija used to meet at the South African Public Library in the Gardens, then take a bus to Kloof Nek about which the author was "so lyrical earlier on in the story of Buitencingle".

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources do not contain a comprehensive description of the Teachers League of South Africa (TLSA). However, based on the information presented:

* [1] mentions that the TLSA is an organization that hosted a conference in East London, which focused on ethnicity and unique cultures. It also mentions that the TLSA was affiliated with the Non-European Unity Movement.
* [4] states that the Education Journal of the Teachers' League of South Africa provided training to writers in political/educational composition and published their articles.
* [8] implies that the TLSA played a role in the struggle for full democratic rights, mentioning its involvement alongside the New Era Fellowship.
* [10] mentions Danny De Beer, a PE teacher who was a member of the TLSA, as well as Edgar Maurice, the Vice President of the TLSA, delivering an essay on the education of non-Whites in South Africa.
* [16] states that the TLSA broke with its previous stance of non-political professionalism and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was bound up with the struggle for a democratic society.

From these excerpts, it can be inferred that the Teachers League of South Africa (TLSA) was an organization that advocated for equality in education and democracy. However, the full scope and purpose of the TLSA are not explicitly stated.

### q12 — Who was Cissie Gool?

Cissie Gool is identified in multiple sources as a member of the Non-European Unity Movement. She is also described as Dr. Abdulla Abdurahman's child, indicating that her father was a prominent figure [1]. 

She lived in District Six and had relationships with various organizations and events throughout her life. For instance, she was involved with the Torch Commando campaign led by Commandant Dolf Delarey [3], but her involvement with this particular movement seems to have been short-lived, as she is later described as a stalwart of the Anti-CAD Movement [16]. 

Cissie Gool's relationship with Dr. A. H. Gool is also mentioned; they were married and Dr. A. H. Gool was her husband [11], [17].

Cissie Gool also participated in the street protests against segregation in 1937, leading a massive demonstration of the Liberation League on the Grand Parade to rally the vote-less masses [2]. 

In addition, Cissie Gool gave speeches supporting measures to deal with "influx control" and the street gangs, colloquially referred to as the “skolly” problem [15], which was seen as a significant departure from her previous involvement in the Anti-CAD Movement.

### q13 — What was the All Africa Convention?

The sources do not provide a detailed description of the All African Convention. However, they mention it as an organization that was part of the Non-European Unity Movement (NEUM) and hosted a conference in 1954 where Vic Wessels, Les Jacobs, and Joe Rassool (the author) represented the New Era Fellowship [19].

### q14 — Where was District Six and what kind of place was it?

According to [1] and [17], District Six was a township or area in Cape Town, South Africa. 

From the various descriptions provided by Yousuf (Joe) Rassool in [16]-[19], District Six appears to have been a thriving cosmopolitan area with people of different ethnicities living together. It was described as having "everything" and being "home" to him, suggesting it was a vibrant and lively community.

In particular, the following are mentioned: 

- A Hindu Hall (located at what used to be no. 22), which hosted social gatherings and political meetings.
- Different shops along Chapel Street, including Dickman’s Bakery, where they bought bread; and opposite this bakery was a grocery shop where they purchased their weekly provisions.
- People of various ethnicities living in the area, such as Hindus, Muslims, and Jews.
- A mix of cultures and influences present, with references to Islamic faith (e.g., koe'sisters) and Jewish Sabbath services.

These details paint a picture of District Six as a diverse community with a rich cultural tapestry.

### q15 — What were the forced removals from District Six?

The provided sources do not contain a direct answer to this question, but they provide context and information about the displacement of people from District Six.

Source [2] mentions that "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Source [13] states that "A potent source of misery occurred when a Coloured Area was declared White: the house prices plummeted as the new buyers were aware that the Coloured owners, who realized that the Group Areas Board's valuation of their properties was generally low, had to vacate the premises."

Source [19] mentions that "Soon other townships were erected to entice the people away from District Six: places like Bonteheuwel, Hanover Park, Mannenberg, Mitchell’s Plain, Vanguard Estate, and Bishop Lavis. Most were for people in the lower income group, but the awful thing about these schemes was that they resulted in the inhabitants losing the municipal franchise because they were living in sub-economic houses!"

Source [20] mentions "against their wishes, they were dragooned to take part in the Van Riebeeck Festival" and "It must have been an acutely embarrassing time for them".

These excerpts suggest that people from District Six were forcibly removed due to the Group Areas Act (1950), which declared Coloured areas as White. They also mention the construction of new townships, which led to the displacement of inhabitants and resulted in the loss of their municipal franchise.

However, the sources do not provide a specific answer to what specifically constituted the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the sources [2], [9], [10], [17], and [19], Mahatma Gandhi was a leader in the Indian independence movement who visited South Africa on several occasions. His connection to the Gool family is that he often stayed at their residence, 7 Buitencingle Street, in Cape Town when he was in the area. The Gool family provided him with hospitality and medical care during his illnesses [9]. Gandhi also valued Dr. A.H. Gool's medical skills and considered him a friend [13].

In particular, Gandhi's correspondence with JMH Gool is mentioned in several sources [8], [17], and [19]. This suggests that there was a close personal connection between Gandhi and the Gool family, particularly with JMH Gool.

Additionally, the Gool family played an important role in supporting Gandhi during his visits to South Africa. They helped him lobby for the rights of Indians in the country, as mentioned in [17].

In summary, Gandhi's connection to the Gool family was one of friendship and mutual respect, with the Gools providing him with hospitality and support during his time in South Africa.

### q17 — What was Hewat Training College?

The provided sources do not contain detailed information on what Hewat Training College specifically was. However, they mention that it was a Post Matriculation Teacher Training facility for Coloureds [1] and that the author (Yousuf Rassool) attended it as a student.

[1]: ool grounds were undeveloped...However, it was deemed good enough to be the site of the first Post Matriculation Teacher Training facility for Coloureds in the country.

### q18 — What was the New Era Fellowship?

The provided sources describe the New Era Fellowship (NEF) as a political cultural organization associated with the Unity Movement. It was involved in various activities, including organizing lectures, events, and rallies, and had members who were active in the community.

According to [1] and [8], the NEF started a successful Ramblers' Club and a not-so-successful Film Club under the inspiration of its leader. The organization's chairman is mentioned as Mr. I.B. Tabata, and its organizing secretary was Hassan Bavasah (also known as Hassen).

The NEF appears to have had connections with other organizations, such as the Unity Movement, the All African Convention, and the Communist Party of South Africa.

It also seems that the NEF was involved in various social and political issues, including criticizing nationalism ([8]) and supporting democratic rights for all people, regardless of their race or background.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) is mentioned in several excerpts as a significant organization that played a crucial role in the struggle for liberation and equality during the Apartheid era in South Africa.

According to excerpt [9], the NEUM brought "the principle of non-Collaboration, the refutation of ‘race’, and the ideal of a non-racial society" to the struggle for liberation. It is also mentioned as one of the main themes in Yousuf (Joe) Rassool's life story, which is told in excerpts [11] and [15].

Excerpt [1] mentions that the Non-European Unity Movement was an organization whose members were intellectuals, but had not made any contribution to the field of literature. This suggests that despite their intellectual background, the movement's focus was on politics and social change rather than literary achievements.

Excerpt [10] describes a meeting where the NEUM faced opposition from Congress Youth Leaguers, who attempted to disrupt the event and pass a vote of no confidence in the NEUM.

Overall, it appears that the Non-European Unity Movement was a key player in the anti-Apartheid movement, advocating for non-collaboration with the oppressive government, challenging the concept of "race", and promoting a non-racial society.

### q20 — Describe the author's involvement in cricket.

The author was heavily involved in cricket and valued it greatly. He belonged to a non-racial cricket club called the Kismets Cricket Club [1], where he played table tennis with his friends, including Purcell (whose nickname is not mentioned) [2]. The club had a policy of being non-racial, which meant that players from different racial groups could play together.

The author mentions his admiration for Mylie Begg, one of the great opening batsmen in their union at that time. He also talks about Salie Van Haacht-Haartzen, Ben Malamba, and Stanley Abrahams being recruited to their club [15]. Through cricket, the author became close friends with Muddy (whose full name is not mentioned) [16].

The author was also involved in organizing a union for cricket at High School and College level but was thwarted by bureaucracy [11]. He mentions his disappointment at not making the final team that went on tour to Johannesburg and the presence of a plethora of good players from other teams in their non-racial Sunday league.

The author also talks about the proposed visit of a West Indian Cricket team to South Africa, which included players like Worrel, Weekes, and Walcott. He mentions that the matter was raised at a union meeting under the chairmanship of Councillor Parker [17]. The author's ability to convince club members not to support Apartheid cricket in favor of playing against the West Indians was crucial.

Throughout the excerpts, it is clear that cricket played a significant role in the author's life, and he had strong feelings about its politics.

### q21 — Who was the author's mother?

The author's mother is Ayesha (also referred to as Lallie) [1].

### q22 — Who was the author's father?

The author's father is Malick Rassool [9].

### q23 — Who were the author's siblings?

According to [4], the author's siblings were:

1. Abdul (8 years old)
2. Fazil (6 years old)
3. Zainunissa (or Zaironesa) was about 4 years old
4. Rasheda was just about 2 years old
5. Berina, the baby, was only a year old

### q24 — Who were the children of J.M.H. Gool?

The sources mention the following children of J.M.H. Gool:

1. Abdul Hamid Gool [1]
2. Jane Gool-Tabata [1]
3. Minnie Gool [1]
4. Zobeida Gool [1]
5. Goolam Gool [1]
6. Fatima Gool [1]
7. Mariam Gool (also known as Margie) [4]
8. Mohamed Saaid Gool [1]
9. Ayesha Rassool (note: her father is listed as J.M.H. Gool, but it's unclear if this means he was her biological father or adoptive father) [1]

Please note that the sources do not mention the children of J.M.H. Gool in a single list, so I have compiled them from various mentions throughout the excerpts.

### q25 — Who was I.B. Tabata?

The provided sources do not directly mention who I.B. Tabata is. However, it can be inferred from [9] that he was a prominent figure associated with the New Era Fellowship and a writer of the book "The Awakening of a People." Additionally, [2] mentions that Mr. I.B. Tabata completed his book "The Awakening of a People" shortly after becoming chairman of the New Era Fellowship.

A further hint can be found in [18], which states: "When Ben Kies entered the hall accompanied by Rev. Wessels there was prolonged applause, which was intensely moving... If the Herrenvolk had intended to cow us by these savage acts, they were mistaken." This suggests that I.B. Tabata may have been at odds with someone named Ben Kies.

Unfortunately, without more direct information or context about I.B. Tabata's life and work outside of these excerpts, it is difficult to provide a definitive answer about who he was.

### q26 — Who was Dr. Abdullah Abdurahman?

According to [4], Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community. His mother, Gadija Dollie, was said to be a most beautiful woman. 

[7] states that when Dr. Abdurahman asked what provision would be made for the higher education of coloured people at a meeting on January 9th, 1904, he received a "mealie-mouthed" reply.

[17] reports that Dr. Abdurahman was the first of the community to qualify as a doctor from Glasgow University in 1893. 

There is also mention of his residence being in Loop Street, a mere walk away from Buitencingle [4].

It seems that Dr. Abdullah Abdurahman was a prominent figure in the community, and had connections with J.M.H. Gool and other influential individuals of the time.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to the sources [1], [6], [8], [12], and [20], Gandhi and J.M.H. Gool had a close relationship that involved:

* Hospitality: The Gool family hosted Gandhi at their residence, 7 Buitencingle Street, in February 1914 ([8]).
* Friendship: E.S. Reddy mentions that his friendship with Joe Rassool began after he provided the author with correspondence between Gandhi and J.M.H. Gool, which suggests a connection between the two families ([12]).
* Mutual respect: Dr. A.H. Gool, J.M.H.'s son, was a close friend of Gandhi's, and Gandhi had invested hope in the young doctor for community service ([13]).
* Correspondence: There were letters exchanged between Gandhi and J.M.H. Gool, as well as between Gandhi and Dr. A.H. Gool ([20]).

Additionally, there are mentions of Gandhi visiting the Gool residence on several occasions ([10], [12]), including a visit in 1911 when he helped to furbish Dr. A.H. Gool's surgery ([10]).

### q28 — Which organisations was the author involved in?

The author was involved in the following organisations:

1. The New Era Fellowship (NEF)
2. The Anti-Coloured Affairs Department (Anti-CAD)

He also mentions being part of the Coloured branch of Smuts's United Party, and that he worked with others from various High Schools to establish a Students' Union, specifically the Cape Peninsula Students Union.

Additionally, the author was a member of the Teachers' League, as evidenced by his mention of the "Education Journal" in [10].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The TLSA (Teachers' League of South Africa) joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society [5]. This decision was not achieved without a fierce struggle, but in the end the young Turks succeeded in ousting the backward leadership and taking control of the TLSA [7].

Note: The exact nature of this relationship (e.g. whether it was a merger or an affiliation) is not specified in the sources.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain that information.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was an outpost of Islam located in Loop Street, Cape Town, South Africa. It was founded and completed in 1898 by my grandfather's community. The mosque was a place of worship for Muslims in the area, which was becoming increasingly European, Jewish, and gentile.

The mosque is mentioned in the text as a significant cultural and spiritual landmark for the Muslim community in Cape Town during that time period. It played an important role in the lives of my grandfather and his family, who would often go there to celebrate Eid festivals and other special occasions.

Unfortunately, no further information about the Hanaffi Quwatul Islam Mosque is provided in the text, such as its current status or whether it still exists today.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], Haji Joosub Maulvi Hamid Gool's children include Abdul Hamid Gool, and his wife is not mentioned here as being Cissie. However, the excerpt does mention that Abdul Hamid was married to Cissie Gool.

In [6] and [7], it is stated that Abdul Hamid was the husband of Cissie Gool.

Therefore, based on these excerpts, Cissie Gool was the wife of Abdul Hamid Gool (who was a son of J.M.H. Gool).

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool had personal connections with several notable historical figures:

1. Mahatma Gandhi [1]: Gool's fortunes declined according to a letter from Mahatma Gandhi.
2. Adam H.G. Mahomed [19]: Gool was a witness in a case along with Adam H.G. Mahomed, and Mahomed was the President of the British Indian League.
3. Dr. Abdullah Abdurahman [13 & 16]: Gool had close relationships with Dr. Abdurahman, including informal discussions and formal collaborations between their organizations.
4. J.M. Wilson [8 & 17]: Gool attended a political meeting together with Mr. M. Wilson, who was a leading 'coloured' businessman in Cape Town at the turn of the century.

These connections suggest that J.M.H. Gool moved in influential circles and had relationships with key figures in South African history.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources describe the Group Areas Act as a law that divided South Africa into separate areas for different racial groups, aiming to preserve White rule. This law affected District Six in several ways:

According to [1], the Group Areas Act was anticipated by the authorities and led to the division of people into separate areas.

Source [3] describes how the community of District Six began to die in the early 1950s as shops started closing, leading to a gradual but definite hemorrhaging of the community. The Group Areas Board's valuation of properties was generally below market value, causing Coloured owners to accept paltry offers from White purchasers.

Source [7] states that the Indian population of District Six was relatively small and most were owners of small corner shops or engaged in marketing fruit and vegetables.

Source [17] mentions that traders in the community were concerned about their future if they were herded into one group area, as it would likely lead to a significant decrease in business opportunities.

Sources [18] and [19] describe how District Six was affected by the Group Areas Act: residents lost the municipal franchise, were effectively disfranchised, and faced a poverty trap where they had no choice but to vacate if their income rose above the sub-economic level. The community's needs were not addressed, and the struggle for democratic rights was in its infancy.

Source [20] highlights that Apartheid policy celebrated white racial supremacy and oppressed black people by subjugating their 'being', legitimizing poverty and oppression for the majority of South Africans, including those living in District Six.

In summary, the Group Areas Act had a devastating impact on District Six, leading to the destruction of its community and way of life.

### q35 — Who was Hassen Mall?

The provided sources do not contain extensive biographical information about Hassen Mall. However, they suggest that he was a student from Durban who came to Cape Town to study medicine [1]. He is mentioned as the team captain of the cricket team in which the narrator played [10], and it appears that he had a significant influence on the narrator's life.

In excerpt [4], Hassen Mall introduced table tennis to the narrator, teaching him how to play whist. In excerpt [17], he is mentioned as having understood the logic of the narrator's argument about pursuing full democratic rights, but his role in the struggle for those rights is not specified.

The sources suggest that Hassen Mall was a talented cricketer and a respected figure, but they do not provide more extensive information about his background or personal life.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources do not contain an exhaustive list of all political organisations active in the Cape Coloured community during the author's lifetime. However, they mention the following:

* The Anti-Coloured Affairs Council (Anti-CAC) / Anti-Colour Affairs Department (Anti-CAD), which was formed to oppose the Coloured Advisory Council (CAC)
* The African People’s Organisation (APO)
* The Communist Party of South Africa
* The Fourth International of South Africa
* The Liberation League
* The Workers’ Party
* The New Era Fellowship
* The Teachers’ League of South Africa

These organisations were involved in various political activities, including protests and campaigns against the CAC/CAD, unity efforts, and advocating for democratic rights.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was a key figure in the Indian independence movement. According to the sources, Gandhi first came to South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria [18].

There is no direct information about why he stayed on in South Africa after completing his initial assignment, but it can be inferred that he chose to stay and help the Indian community as they faced disenfranchisement in Natal in 1894.

### q38 — Who was Cissie Gool's father?

Cissie Gool's father is Dr. Abdurahman, a doctor from Glasgow University who met and married Helen "Nellie" Potter James [4].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources, District Six was a thriving cosmopolitan area in the heart of Cape Town, with a rich tapestry of cultures and ways of life. It was described as "inimitable" [2] and had a unique buzz all its own [10]. The neighbourhood was home to people from diverse backgrounds, including Coloureds, Indians, Whites, and others.

Before the Group Areas Act (1950), District Six was a vibrant community with a mix of tenements, shops, and homes. However, as time went on, the area began to decline due to the influx of new residents, the departure of some well-to-do Whites, and the rise of gangs [12].

The sources describe the neighbourhood as having "pavements, dirty gutters, filthy streets and the cracked tiles" [1] but also mention that it was a place where people from different backgrounds lived together in a lively and dynamic community.

One author notes that District Six was not just a community defined by silence and acquiescence, but had a rich history of political struggle against the injustices of Apartheid [4]. The area was home to many activists and anti-Apartheid fighters, including Joe Rassool's family members, who played a significant role in shaping the neighbourhood's politics.

In summary, District Six was a vibrant and diverse community before the forced removals, with a mix of cultures, ways of life, and social classes. However, the area also faced challenges such as poverty, crime, and Apartheid-era policies that sought to erase its identity.

The provided sources do not contain detailed information about the exact demographics or population statistics of District Six before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy included:

* Boycotting elections: They decided to boycott the election, but if the "Coloured" voters insisted on voting, they were asked to spoil their ballot by writing: "For Full Democratic Rights" across the ballot slip [20].
* Boycotting the Coloured Affairs Council (CAC): Members of the CAC who accepted service were boycotted in every aspect of their lives, socially, economically, and otherwise [9].

They also believed that the boycott was a powerful weapon against Apartheid. However, they struggled to implement it effectively in certain situations, such as the Group Areas Act [17].


  ✅ Overall: 56.9% recall (token-overlap)  (128.0/225)  avg 4301ms
