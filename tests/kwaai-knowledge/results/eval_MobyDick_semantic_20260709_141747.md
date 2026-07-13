[2m2026-07-09T21:23:26.963905Z[0m [32m INFO[0m [2mkwaainet::identity[0m[2m:[0m Loaded identity from /Users/rezarassool/.kwaainet/identity.key: 12D3KooWAourfFoxBjBiXWHdZjxPAuenuyYLFjxHa1C9zknjYA7A
[2m2026-07-09T21:23:27.943696Z[0m [32m INFO[0m [2mkwaainet::shard_cmd[0m[2m:[0m p2p://auto → 12D3KooWSAa71FVsDEe9qEq1v9SFPM45JRFscKqMtdrEESQH5Zbd (christophe-linux-x86_64/v0.5.0, 10.0 tok/s)
  ● p2p://auto → p2p://12D3KooWSAa71FVsDEe9qEq1v9SFPM45JRFscKqMtdrEESQH5Zbd
[2m2026-07-09T21:23:28.176099Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWSAa71FVsDEe9qEq1v9SFPM45JRFscKqMtdrEESQH5Zbd → http://127.0.0.1:54215 (via ollama-proxy)
[2m2026-07-09T21:24:40.470104Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│                RAG Eval  (20 questions, kb=MobyDick)                │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:54215
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] What is the central plot and thematic obsession of Melville's Moby-Dic
[2m2026-07-09T21:25:34.420345Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  4/6 query terms found  (67%)
  ○ Round 2   gap-filling for [plot, obsession]
  ○ Round 2   added 8 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [plot, obsession]
  ○ Round 3   reformulation failed (error sending request for url (http://127.0.0.1:54215/api/chat): operation timed out)
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=0/4  295980ms
  [ 2/20] What is Bartleby, the Scrivener about, and how does its tone and scale
[2m2026-07-09T21:30:52.133142Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

[2m2026-07-09T21:31:08.771985Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWSAa71FVsDEe9qEq1v9SFPM45JRFscKqMtdrEESQH5Zbd via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: stream reset
         → ret=3.4/4  gen=0/4  91351ms
  [ 3/20] What is Billy Budd about, and what moral/legal dilemma does it center 
[2m2026-07-09T21:32:15.159518Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  2/5 query terms found  (40%)
  ○ Round 2   gap-filling for [moral/legal, dilemma, center]
  ○ Round 2   added 24 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [moral/legal, dilemma, center]
  ○ Round 3   reformulation failed (error sending request for url (http://127.0.0.1:54215/api/chat): operation timed out)
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=1.2/4  gen=0/4  252655ms
  [ 4/20] What is Typee, and how does it relate to Melville's own biographical e
[2m2026-07-09T21:36:23.504774Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=0/4  187926ms
  [ 5/20] What historical event does Melville's Battle-Pieces and Aspects of the
[2m2026-07-09T21:39:33.714771Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  6/9 query terms found  (67%)
  ○ Round 2   gap-filling for [historical, subject-wise, narratives]
  ○ Round 2   added 15 chunks via graph gap-fill
  ○ Final     20 chunks from 9 documents — passing to LLM

         → ret=4/4  gen=0/4  189917ms
  [ 6/20] What real 1820 historical event does the whaling-industry literature i
[2m2026-07-09T21:44:06.293483Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  12/13 query terms found  (92%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=4/4  gen=0/4  272253ms
  [ 7/20] What does the Book of Job (KJV) concern, and what thematic parallels d
[2m2026-07-09T21:50:37.271666Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  5/9 query terms found  (56%)
  ○ Round 2   gap-filling for [thematic, parallels, confrontation, unknowable]
  ○ Round 2   added 13 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [thematic, parallels, confrontation]
  ○ Round 3   → ""Book of Job (King James Version) themes comparison to Moby Dick confr"
  ○ Round 3   added 30 chunks from reformulated query
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=2.1/4  gen=0/4  422213ms
  [ 8/20] According to Weaver's "Herman Melville: Mariner and Mystic," what dual
[2m2026-07-09T21:57:40.660178Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3/3  gen=0/3  391603ms
  [ 9/20] What does D.H. Lawrence's Studies in Classic American Literature argue
[2m2026-07-09T22:02:20.879403Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  8/12 query terms found  (67%)
  ○ Round 2   gap-filling for [lawrence, argue, inclusion, cluster]
  ○ Round 2   added 6 chunks via graph gap-fill
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  279809ms
  [10/20] What does Browne's Etchings of a Whaling Cruise document, and how does
[2m2026-07-09T22:06:13.615637Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  8/12 query terms found  (67%)
  ○ Round 2   gap-filling for [document, firsthand, fictionalized, depiction]
  ○ Round 2   added 7 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [document, firsthand, fictionalized]
  ○ Round 3   reformulation failed (error sending request for url (http://127.0.0.1:54215/api/chat): operation timed out)
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=0/4  292451ms
  [11/20] What does Minnigerode's work compile regarding Melville, and what two 
[2m2026-07-09T22:10:47.260174Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  7/9 query terms found  (78%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=0/4  212977ms
  [12/20] To whom did Melville dedicate Moby-Dick, and what work by that author 
[2m2026-07-09T22:14:10.253326Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=0/4  202788ms
  [13/20] Cross-document: How does Hawthorne's The Scarlet Letter compare stylis
[2m2026-07-09T22:17:22.313605Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  7/12 query terms found  (58%)
  ○ Round 2   gap-filling for [cross-document, compare, stylistically, thematically, authors]
  ○ Round 2   added 68 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, stylistically, thematically]
  ○ Round 3   → ""Comparison of Melville's Moby Dick and Hawthorne's Scarlet Letter in "
  ○ Round 3   added 19 chunks from reformulated query
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=3.2/4  gen=0/4  224273ms
  [14/20] Cross-document: How does Starbuck's History of the American Whale Fish
[2m2026-07-09T22:21:10.564980Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  10/15 query terms found  (67%)
  ○ Round 2   gap-filling for [cross-document, factual, industrial, context, whaling-industry]
  ○ Round 2   added 28 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, factual, industrial]
  ○ Round 3   reformulation failed (error sending request for url (http://127.0.0.1:54215/api/chat): operation timed out)
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.4/4  gen=0/4  255513ms
  [15/20] Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, bu
[2m2026-07-09T22:25:28.794179Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  9/15 query terms found  (60%)
  ○ Round 2   gap-filling for [near-miss, narratives, largely, fictionalized, distinction]
  ○ Round 2   added 21 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, narratives, largely]
  ○ Round 3   reformulation failed (error sending request for url (http://127.0.0.1:54215/api/chat): operation timed out)
  ○ Final     20 chunks from 8 documents — passing to LLM

         → ret=1.2/4  gen=0/4  258460ms
  [16/20] Near-miss: Both Bartleby and Billy Budd deal with authority and refusa
[2m2026-07-09T22:29:50.289580Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  11/18 query terms found  (61%)
  ○ Round 2   gap-filling for [near-miss, deal, refusal, versus, respectively]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, deal, refusal]
  ○ Round 3   → ""Comparative analysis of authority dynamics in Bartleby and Billy Budd"
  ○ Round 3   added 20 chunks from reformulated query
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-09T22:31:16.066752Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWSAa71FVsDEe9qEq1v9SFPM45JRFscKqMtdrEESQH5Zbd via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: stream reset
         → ret=3.3/4  gen=0/4  159788ms
  [17/20] Cross-document: How might Weaver's critical biography and Minnigerode'
[2m2026-07-09T22:32:41.268484Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  9/15 query terms found  (60%)
  ○ Round 2   gap-filling for [cross-document, letters/bibliography, compilation, different, scholarly]
  ○ Round 2   added 12 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, letters/bibliography, compilation]
  ○ Round 3   → ""purposes of comparative biography vs literary bibliography in Melvill"
  ○ Round 3   added 27 chunks from reformulated query
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=0/4  242310ms
  [18/20] Cross-document: What thematic throughline connects the Book of Job's c
[2m2026-07-09T22:36:49.328272Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  6/12 query terms found  (50%)
  ○ Round 2   gap-filling for [cross-document, thematic, throughline, connects, confrontation]
  ○ Round 2   added 7 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, thematic, throughline]
  ○ Round 3   → ""Common themes in literary works about man vs nature in Book of Job an"
  ○ Round 3   added 34 chunks from reformulated query
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3/4  gen=0/4  244018ms
  [19/20] Edge case: Which single document in this cluster would be hardest to d
[2m2026-07-09T22:41:10.319203Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  14/19 query terms found  (74%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  233873ms
  [20/20] What does the presence of both fiction (Melville's other novels) and n
[2m2026-07-09T22:46:11.024011Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m8528 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  13/18 query terms found  (72%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

