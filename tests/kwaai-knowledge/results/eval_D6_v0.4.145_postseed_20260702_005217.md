[2m2026-07-02T07:52:19.398308Z[0m [32m INFO[0m [2mkwaainet::identity[0m[2m:[0m Loaded identity from /Users/rezarassool/.kwaainet/identity.key: 12D3KooWAourfFoxBjBiXWHdZjxPAuenuyYLFjxHa1C9zknjYA7A
[2m2026-07-02T07:52:20.358167Z[0m [32m INFO[0m [2mkwaainet::shard_cmd[0m[2m:[0m p2p://auto → 12D3KooWG92DacfbMiSoCBssAt8tUcrrBw8Dn7z8m4j5ACM4KJc5 (redmond-win-amd64/v0.4.120, 32.8 tok/s)
  ● p2p://auto → p2p://12D3KooWG92DacfbMiSoCBssAt8tUcrrBw8Dn7z8m4j5ACM4KJc5
[2m2026-07-02T07:52:21.491698Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWG92DacfbMiSoCBssAt8tUcrrBw8Dn7z8m4j5ACM4KJc5 → http://127.0.0.1:51554 (via ollama-proxy)
[2m2026-07-02T07:52:25.185254Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:51554
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T07:52:25.319873Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/3 keywords  1049ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T07:52:26.625503Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/3 keywords  458ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T07:52:27.291407Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  381ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T07:52:27.887012Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/4 keywords  387ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T07:52:28.492125Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/8 keywords  388ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T07:52:29.087690Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 1/8 keywords  381ms
  [ 7/40] Who is the author's wife?
[2m2026-07-02T07:52:29.667887Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/3 keywords  365ms
  [ 8/40] Tell me more about the author's wife.
[2m2026-07-02T07:52:30.244599Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  393ms
  [ 9/40] Who was the author's grandfather?
[2m2026-07-02T07:52:30.844852Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/9 keywords  385ms
  [10/40] Tell me about Kloof Nek.
[2m2026-07-02T07:52:31.426987Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/7 keywords  465ms
  [11/40] What was the Teachers League of South Africa (TLSA)?
[2m2026-07-02T07:52:32.100009Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  382ms
  [12/40] Who was Cissie Gool?
[2m2026-07-02T07:52:32.684963Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  444ms
  [13/40] What was the All Africa Convention?
[2m2026-07-02T07:52:33.329039Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  382ms
  [14/40] Where was District Six and what kind of place was it?
[2m2026-07-02T07:52:33.915624Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  384ms
  [15/40] What were the forced removals from District Six?
[2m2026-07-02T07:52:34.494635Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/3 query terms found  (67%)
  ○ Round 2   gap-filling for [removals]
  ○ Round 2   added 87 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [removals]
  ○ Round 3   → "What were the forced removals from District Six?"
  ○ Round 3   added 8 chunks from reformulated query
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  689ms
  [16/40] Who was Gandhi and what was his connection to the Gool family?
[2m2026-07-02T07:52:35.387074Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  373ms
  [17/40] What was Hewat Training College?
[2m2026-07-02T07:52:35.971199Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  462ms
  [18/40] What was the New Era Fellowship?
[2m2026-07-02T07:52:36.640071Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  447ms
  [19/40] What was the Non-European Unity Movement?
[2m2026-07-02T07:52:37.288508Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  389ms
  [20/40] Describe the author's involvement in cricket.
[2m2026-07-02T07:52:37.880353Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/5 keywords  385ms
  [21/40] Who was the author's mother?
[2m2026-07-02T07:52:38.466500Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  388ms
  [22/40] Who was the author's father?
[2m2026-07-02T07:52:39.082061Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  394ms
  [23/40] Who were the author's siblings?
[2m2026-07-02T07:52:39.687693Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  417ms
  [24/40] Who were the children of J.M.H. Gool?
[2m2026-07-02T07:52:40.277095Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/7 keywords  441ms
  [25/40] Who was I.B. Tabata?
[2m2026-07-02T07:52:40.925578Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/5 keywords  383ms
  [26/40] Who was Dr. Abdullah Abdurahman?
[2m2026-07-02T07:52:41.506881Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  370ms
  [27/40] What was the connection between Gandhi and J.M.H. Gool?
[2m2026-07-02T07:52:42.081521Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  383ms
  [28/40] Which organisations was the author involved in?
[2m2026-07-02T07:52:42.666423Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/3 keywords  377ms
  [29/40] What was the relationship between the TLSA and the Non-European Unity 
[2m2026-07-02T07:52:43.246670Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/6 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  376ms
  [30/40] When did J.M.H. Gool arrive in Cape Town and from where?
[2m2026-07-02T07:52:43.827992Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/5 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  470ms
  [31/40] What was the Hanaffi Quwatul Islam Mosque?
[2m2026-07-02T07:52:44.486796Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  357ms
  [32/40] How was Cissie Gool related to J.M.H. Gool?
[2m2026-07-02T07:52:45.061847Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  383ms
  [33/40] Who were the notable historical figures that J.M.H. Gool knew personal
[2m2026-07-02T07:52:45.649817Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  393ms
  [34/40] What was the Group Areas Act and how did it affect District Six?
[2m2026-07-02T07:52:46.245572Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  449ms
  [35/40] Who was Hassen Mall?
[2m2026-07-02T07:52:46.886346Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 0/4 keywords  379ms
  [36/40] What political organisations were active in the Cape Coloured communit
[2m2026-07-02T07:52:47.468793Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/4 keywords  444ms
  [37/40] Who was Mahatma Gandhi and why was he in South Africa?
[2m2026-07-02T07:52:48.117435Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/7 keywords  371ms
  [38/40] Who was Cissie Gool's father?
[2m2026-07-02T07:52:48.704757Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  3/3 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  386ms
  [39/40] What was District Six like as a neighbourhood before the forced remova
[2m2026-07-02T07:52:49.290287Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  5/6 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  443ms
  [40/40] What was the Unity Movement's boycott policy?
[2m2026-07-02T07:52:49.938454Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/4 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/5 keywords  440ms

# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 0.5% (1.0/209) |
| Avg latency | 425ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 0/3 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 1049ms |
| q02 | Who are the author's children? | 0/3 (0%) | LEST WE FORGET -rev25.pdf | 458ms |
| q03 | Who are the author's grandchildren? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 381ms |
| q04 | To whom is the book dedicated? | 0/4 (0%) | LEST WE FORGET -rev25.pdf | 387ms |
| q05 | Who was J.M.H. Gool? | 0/8 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 388ms |
| q06 | Tell me about Buitencingle. | 1/8 (12%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 381ms |
| q07 | Who is the author's wife? | 0/3 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 365ms |
| q08 | Tell me more about the author's wife. | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 393ms |
| q09 | Who was the author's grandfather? | 0/9 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 385ms |
| q10 | Tell me about Kloof Nek. | 0/7 (0%) | LEST WE FORGET -rev25.pdf | 465ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 382ms |
| q12 | Who was Cissie Gool? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 444ms |
| q13 | What was the All Africa Convention? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 382ms |
| q14 | Where was District Six and what kind of place was it? | 0/6 (0%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 384ms |
| q15 | What were the forced removals from District Six? | 0/5 (0%) | [Graph: District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 689ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 0/5 (0%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 373ms |
| q17 | What was Hewat Training College? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 462ms |
| q18 | What was the New Era Fellowship? | 0/4 (0%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 447ms |
| q19 | What was the Non-European Unity Movement? | 0/4 (0%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 389ms |
| q20 | Describe the author's involvement in cricket. | 0/5 (0%) | LEST WE FORGET -rev25.pdf | 385ms |
| q21 | Who was the author's mother? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 388ms |
| q22 | Who was the author's father? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 394ms |
| q23 | Who were the author's siblings? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 417ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 441ms |
| q25 | Who was I.B. Tabata? | 0/5 (0%) | LEST WE FORGET -rev25.pdf | 383ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 370ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 383ms |
| q28 | Which organisations was the author involved in? | 0/3 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 377ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 0/4 (0%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 376ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool, LEST WE FORGET -rev25.pdf | 470ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 0/6 (0%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 357ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 383ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 393ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 0/6 (0%) | [Graph: District Six], sequence_diagram:District Six, LEST WE FORGET -rev25.pdf | 449ms |
| q35 | Who was Hassen Mall? | 0/4 (0%) | LEST WE FORGET -rev25.pdf | 379ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/4 (0%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 444ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 0/7 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 371ms |
| q38 | Who was Cissie Gool's father? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 386ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 443ms |
| q40 | What was the Unity Movement's boycott policy? | 0/5 (0%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 440ms |

## Answers

### q01 — Who is the author?

(no response)

### q02 — Who are the author's children?

(no response)

### q03 — Who are the author's grandchildren?

(no response)

### q04 — To whom is the book dedicated?

(no response)

### q05 — Who was J.M.H. Gool?

(no response)

### q06 — Tell me about Buitencingle.

(no response)

### q07 — Who is the author's wife?

(no response)

### q08 — Tell me more about the author's wife.

(no response)

### q09 — Who was the author's grandfather?

(no response)

### q10 — Tell me about Kloof Nek.

(no response)

### q11 — What was the Teachers League of South Africa (TLSA)?

(no response)

### q12 — Who was Cissie Gool?

(no response)

### q13 — What was the All Africa Convention?

(no response)

### q14 — Where was District Six and what kind of place was it?

(no response)

### q15 — What were the forced removals from District Six?

(no response)

### q16 — Who was Gandhi and what was his connection to the Gool family?

(no response)

### q17 — What was Hewat Training College?

(no response)

### q18 — What was the New Era Fellowship?

(no response)

### q19 — What was the Non-European Unity Movement?

(no response)

### q20 — Describe the author's involvement in cricket.

(no response)

### q21 — Who was the author's mother?

(no response)

### q22 — Who was the author's father?

(no response)

### q23 — Who were the author's siblings?

(no response)

### q24 — Who were the children of J.M.H. Gool?

(no response)

### q25 — Who was I.B. Tabata?

(no response)

### q26 — Who was Dr. Abdullah Abdurahman?

(no response)

### q27 — What was the connection between Gandhi and J.M.H. Gool?

(no response)

### q28 — Which organisations was the author involved in?

(no response)

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

(no response)

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

(no response)

### q31 — What was the Hanaffi Quwatul Islam Mosque?

(no response)

### q32 — How was Cissie Gool related to J.M.H. Gool?

(no response)

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

(no response)

### q34 — What was the Group Areas Act and how did it affect District Six?

(no response)

### q35 — Who was Hassen Mall?

(no response)

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

(no response)

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

(no response)

### q38 — Who was Cissie Gool's father?

(no response)

### q39 — What was District Six like as a neighbourhood before the forced removals?

(no response)

### q40 — What was the Unity Movement's boycott policy?

(no response)


  ✅ Overall: 0.5% recall (token-overlap)  (1.0/209)  avg 425ms
