[2m2026-07-09T22:46:57.232222Z[0m [32m INFO[0m [2mkwaainet::identity[0m[2m:[0m Loaded identity from /Users/rezarassool/.kwaainet/identity.key: 12D3KooWAourfFoxBjBiXWHdZjxPAuenuyYLFjxHa1C9zknjYA7A
[2m2026-07-09T22:46:58.210423Z[0m [32m INFO[0m [2mkwaainet::shard_cmd[0m[2m:[0m p2p://auto → 12D3KooWGseeb1Btv4Sn33bwxi2iR9DSERWiqrULw2U91ZrmBvip (christophe-linux-x86_64/v0.5.0, 10.0 tok/s)
  ● p2p://auto → p2p://12D3KooWGseeb1Btv4Sn33bwxi2iR9DSERWiqrULw2U91ZrmBvip
[2m2026-07-09T22:46:59.626779Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWGseeb1Btv4Sn33bwxi2iR9DSERWiqrULw2U91ZrmBvip → http://127.0.0.1:59710 (via ollama-proxy)
[2m2026-07-09T22:47:00.322510Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│                RAG Eval  (20 questions, kb=Meetings)                │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:59710
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] What drawing tool did Reza introduce to the interns in the May 26 stan
[2m2026-07-09T22:47:00.844654Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/7 query terms found  (57%)
  ○ Round 2   gap-filling for [tool, introduce, standup]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [tool, introduce, standup]
  ○ Round 3   → "What drawing tool did Reza introduce to the interns in the May 26 stan"
  ○ Round 3   added 2 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=2.1/3  gen=0/3  1537ms
  [ 2/20] According to Reza's analogy in the May 26 standup, what is "the missin
[2m2026-07-09T22:47:02.617803Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  7/11 query terms found  (64%)
  ○ Round 2   gap-filling for [according, standup, missing, compared]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [according, standup, missing]
  ○ Round 3   → "According to Reza's analogy in the May 26 standup, what is "the missin"
  ○ Round 3   added 1 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=2/2  gen=0/2  1199ms
  [ 3/20] Who is Mitch Travers, and what phrase did Reza attribute to him regard
[2m2026-07-09T22:47:04.024889Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/7 query terms found  (29%)
  ○ Round 2   gap-filling for [mitch, travers, phrase, attribute, regarding]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [mitch, travers, phrase]
  ○ Round 3   → "Who is Mitch Travers, and what phrase did Reza attribute to him regard"
  ○ Round 3   added 2 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=2/3  gen=0/3  1183ms
  [ 4/20] What personal event did Reza mention happened on his birthday, and whe
[2m2026-07-09T22:47:05.572558Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/6 query terms found  (67%)
  ○ Round 2   gap-filling for [happened, birthday]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [happened, birthday]
  ○ Round 3   → "What personal event did Reza mention happened on his birthday, and whe"
  ○ Round 3   added 0 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=2/2  gen=0/2  1406ms
  [ 5/20] Who is Solomon Satari, and what was Reza doing with him over the prior
[2m2026-07-09T22:47:07.134857Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  4/9 query terms found  (44%)
  ○ Round 2   gap-filling for [solomon, satari, prior, according, standup]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [solomon, satari, prior]
  ○ Round 3   → "Who is Solomon Satari, and what was Reza doing with him over the prior"
  ○ Round 3   added 4 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=2/3  gen=0/3  1393ms
  [ 6/20] Who is "Doc Searles" as referenced in the May 26 standup, and how does
[2m2026-07-09T22:47:08.871315Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/5 query terms found  (40%)
  ○ Round 2   gap-filling for [searles, referenced, standup]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [searles, referenced, standup]
  ○ Round 3   → "Who is "Doc Searles" as referenced in the May 26 standup, and how does"
  ○ Round 3   added 0 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=1/3  gen=0/3  1421ms
  [ 7/20] Who is Professor Ruth Rasul, what is her academic background, and what
[2m2026-07-09T22:47:10.480977Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → ret=3/4  gen=0/4  1139ms
  [ 8/20] What was the title and approximate length of the book Ruth Rasul gave 
[2m2026-07-09T22:47:11.898294Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m54 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  6/10 query terms found  (60%)
  ○ Round 2   gap-filling for [approximate, gave, according, standup]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [approximate, gave, according]
  ○ Round 3   → "What was the title and approximate length of the book Ruth Rasul gave "
  ○ Round 3   added 1 chunks from reformulated query
  ○ Final     20 chunks from 1 documents — passing to LLM

