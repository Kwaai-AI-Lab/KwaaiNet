⚠  Legacy redb store detected for 278e8c29-5a45-4bad-acc2-a30f72976602. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:02.411639Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 9e63ac0b-9a69-4334-adf5-4100ad5da578: routing: not found
[2m2026-07-13T17:25:02.411723Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:54038 (via ollama-proxy)
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:02.807803Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│                RAG Eval  (20 questions, kb=MobyDick)                │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:54038
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] What is the central plot and thematic obsession of Melville's Moby-Dic
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:03.162863Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  4/6 query terms found  (67%)
  ○ Round 2   gap-filling for [plot, obsession]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [plot, obsession]
[2m2026-07-13T17:25:35.028385Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 9471ad20-dd89-4fc2-b2af-dd3eef411bc2: routing: not found
[2m2026-07-13T17:25:35.028406Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 4 documents — passing to LLM

[2m2026-07-13T17:25:35.534247Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 02cd1c02-f0af-4e61-8a7f-9e75cae153a1: routing: not found
[2m2026-07-13T17:25:35.534265Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=4/4  gen=1.5/4  32378ms
  [ 2/20] What is Bartleby, the Scrivener about, and how does its tone and scale
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:35.811345Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  5/7 query terms found  (71%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

[2m2026-07-13T17:25:38.021070Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call d22a97dc-ae9e-4a41-9995-11106528b5d0: routing: not found
[2m2026-07-13T17:25:38.021096Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-13T17:25:38.021129Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
         → ret=3.6/4  gen=1.2/4  2225ms
  [ 3/20] What is Billy Budd about, and what moral/legal dilemma does it center 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:38.273598Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  3/5 query terms found  (60%)
  ○ Round 2   gap-filling for [moral/legal, center]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [moral/legal, center]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3.6/4  gen=1.4/4  1000ms
  [ 4/20] What is Typee, and how does it relate to Melville's own biographical e
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:39.497501Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=1.2/4  985ms
  [ 5/20] What historical event does Melville's Battle-Pieces and Aspects of the
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:40.703259Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  7/9 query terms found  (78%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=1.2/4  966ms
  [ 6/20] What real 1820 historical event does the whaling-industry literature i
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:41.890889Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  12/13 query terms found  (92%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=1.4/4  982ms
  [ 7/20] What does the Book of Job (KJV) concern, and what thematic parallels d
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:43.094531Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 6 documents
  ○ Coverage  5/9 query terms found  (56%)
  ○ Round 2   gap-filling for [thematic, parallels, confrontation, unknowable]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [thematic, parallels, confrontation]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3.5/4  gen=1.2/4  1019ms
  [ 8/20] According to Weaver's "Herman Melville: Mariner and Mystic," what dual
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:44.336545Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=3/3  gen=0.9/3  953ms
  [ 9/20] What does D.H. Lawrence's Studies in Classic American Literature argue
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:45.510367Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  8/12 query terms found  (67%)
  ○ Round 2   gap-filling for [lawrence, argue, inclusion, cluster]
  ○ Round 2   added 5 chunks via graph gap-fill
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=1.3/4  966ms
  [10/20] What does Browne's Etchings of a Whaling Cruise document, and how does
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:46.742523Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 7 documents
  ○ Coverage  9/12 query terms found  (75%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=1.1/4  997ms
  [11/20] What does Minnigerode's work compile regarding Melville, and what two 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:47.983889Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  8/9 query terms found  (89%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=4/4  gen=1.3/4  962ms
  [12/20] To whom did Melville dedicate Moby-Dick, and what work by that author 
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:49.168935Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → ret=4/4  gen=1.4/4  958ms
  [13/20] Cross-document: How does Hawthorne's The Scarlet Letter compare stylis
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:50.349108Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 5 documents
  ○ Coverage  7/12 query terms found  (58%)
  ○ Round 2   gap-filling for [cross-document, compare, stylistically, thematically, authors]
  ○ Round 2   added 5 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, compare, stylistically]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=1.4/4  1024ms
  [14/20] Cross-document: How does Starbuck's History of the American Whale Fish
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:51.595955Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  10/15 query terms found  (67%)
  ○ Round 2   gap-filling for [cross-document, factual, industrial, context, whaling-industry]
  ○ Round 2   added 0 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, factual, industrial]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 2 documents — passing to LLM

         → ret=4/4  gen=1.0/4  1010ms
  [15/20] Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, bu
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:52.829955Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  10/15 query terms found  (67%)
  ○ Round 2   gap-filling for [near-miss, largely, fictionalized, distinction, development]
  ○ Round 2   added 2 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, largely, fictionalized]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=4/4  gen=1.5/4  972ms
  [16/20] Near-miss: Both Bartleby and Billy Budd deal with authority and refusa
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:54.022039Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 9 documents
  ○ Coverage  11/18 query terms found  (61%)
  ○ Round 2   gap-filling for [near-miss, deal, refusal, versus, respectively]
  ○ Round 2   added 4 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [near-miss, refusal, versus]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3.6/4  gen=1.3/4  1123ms
  [17/20] Cross-document: How might Weaver's critical biography and Minnigerode'
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:55.367572Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  10/15 query terms found  (67%)
  ○ Round 2   gap-filling for [cross-document, letters/bibliography, compilation, scholarly, studying]
  ○ Round 2   added 3 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, letters/bibliography, compilation]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=1.3/4  1007ms
  [18/20] Cross-document: What thematic throughline connects the Book of Job's c
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:56.598115Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 6 documents
  ○ Coverage  6/12 query terms found  (50%)
  ○ Round 2   gap-filling for [cross-document, thematic, throughline, connects, confrontation]
  ○ Round 2   added 6 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cross-document, thematic, throughline]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.4/4  gen=1.3/4  985ms
  [19/20] Edge case: Which single document in this cluster would be hardest to d
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:57.805967Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  13/19 query terms found  (68%)
  ○ Round 2   gap-filling for [cluster, hardest, topical, similarity, textual]
  ○ Round 2   added 3 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [cluster, hardest, topical]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=3.4/4  gen=1.6/4  1623ms
  [20/20] What does the presence of both fiction (Melville's other novels) and n
⚠  Legacy redb graph store detected. Run `kwaainet rag rebuild` to migrate.
[2m2026-07-13T17:25:59.656116Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m282 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 8 documents
  ○ Coverage  13/18 query terms found  (72%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=2.5/3  gen=0.9/3  1069ms

# RAG Eval Report

**KB:** `MobyDick`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 95.7% (74.6/78) |
| Generation recall (token-overlap + semantic) | 32.5% (25.4/78) |
| Scoring mode | token-overlap + semantic embedding (low=0.30, high=0.85) |
| Avg latency | 2660ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | What is the central plot and thematic obsession of Melville's Moby-Dick? | 4/4 (100%) | 1.5/4 (37%) | Hawthorne - The Scarlet Letter.pdf, Starbuck - History of the American Whale Fishery.pdf, gutenberg.org-NARRATIVE OF THE MOST EXTRAORDINARY AND DISTRESSING SHIPWRECK OF THE WHALE-SHIP ESSEX OF NANTUCKET W.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 32378ms |
| q02 | What is Bartleby, the Scrivener about, and how does its tone and scale differ dramatically from Moby-Dick's? | 3.6/4 (91%) | 1.2/4 (30%) | Melville - Bartleby the Scrivener.pdf | 2225ms |
| q03 | What is Billy Budd about, and what moral/legal dilemma does it center on? | 3.6/4 (89%) | 1.4/4 (34%) | Melville - Bartleby the Scrivener.pdf, Melville - Billy Budd.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Typee.pdf, Browne - Etchings of a Whaling Cruise.pdf | 1000ms |
| q04 | What is Typee, and how does it relate to Melville's own biographical experience, per Weaver's critical biography? | 4/4 (100%) | 1.2/4 (30%) | Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Melville - Typee.pdf | 985ms |
| q05 | What historical event does Melville's Battle-Pieces and Aspects of the War address, and how does this differ subject-wise from his sea narratives? | 4/4 (100%) | 1.2/4 (31%) | Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, [Graph: South Sea], Browne - Etchings of a Whaling Cruise.pdf, Melville - Battle-Pieces and Aspects of the War.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 966ms |
| q06 | What real 1820 historical event does the whaling-industry literature in this cluster provide context for, that directly inspired Moby-Dick's plot? | 4/4 (100%) | 1.4/4 (36%) | Melville - Typee.pdf, gutenberg.org-NARRATIVE OF THE MOST EXTRAORDINARY AND DISTRESSING SHIPWRECK OF THE WHALE-SHIP ESSEX OF NANTUCKET W.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf, Melville - Billy Budd.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 982ms |
| q07 | What does the Book of Job (KJV) concern, and what thematic parallels does it have with Moby-Dick, such as confrontation with an unknowable natural force? | 3.5/4 (89%) | 1.2/4 (30%) | Melville - Moby-Dick or The Whale.pdf, King James Version - The Book of Job.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Starbuck - History of the American Whale Fishery.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 1019ms |
| q08 | According to Weaver's "Herman Melville: Mariner and Mystic," what dual character does the biography's title suggest about Melville? | 3/3 (100%) | 0.9/3 (29%) | Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 953ms |
| q09 | What does D.H. Lawrence's Studies in Classic American Literature argue about Moby-Dick, based on its inclusion as literary criticism in this cluster? | 4/4 (100%) | 1.3/4 (32%) | Weaver - Herman Melville Mariner and Mystic.pdf, Starbuck - History of the American Whale Fishery.pdf, Melville - Typee.pdf, Melville - Moby-Dick or The Whale.pdf, Browne - Etchings of a Whaling Cruise.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 966ms |
| q10 | What does Browne's Etchings of a Whaling Cruise document, and how does its firsthand account compare to Melville's fictionalized depiction in Moby-Dick? | 4/4 (100%) | 1.1/4 (28%) | Browne - Etchings of a Whaling Cruise.pdf, Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 997ms |
| q11 | What does Minnigerode's work compile regarding Melville, and what two types of material does its title indicate? | 4/4 (100%) | 1.3/4 (32%) | Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf, Melville - Typee.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 962ms |
| q12 | To whom did Melville dedicate Moby-Dick, and what work by that author is included in this cluster as contemporary context? | 4/4 (100%) | 1.4/4 (34%) | Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 958ms |
| q13 | Cross-document: How does Hawthorne's The Scarlet Letter compare stylistically and thematically to Moby-Dick, given the two authors were contemporaries and friends? | 4/4 (100%) | 1.4/4 (35%) | Melville - Typee.pdf, Starbuck - History of the American Whale Fishery.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Hawthorne - The Scarlet Letter.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Melville - Moby-Dick or The Whale.pdf | 1024ms |
| q14 | Cross-document: How does Starbuck's History of the American Whale Fishery provide factual and industrial context that Melville draws on for Moby-Dick's whaling-industry digressions? | 4/4 (100%) | 1.0/4 (26%) | Weaver - Herman Melville Mariner and Mystic.pdf, Starbuck - History of the American Whale Fishery.pdf | 1010ms |
| q15 | Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, but Typee is largely autobiographical while Moby-Dick is fully fictionalized — what does this distinction suggest about Melville's development as a writer? | 4/4 (100%) | 1.5/4 (36%) | Weaver - Herman Melville Mariner and Mystic.pdf, Starbuck - History of the American Whale Fishery.pdf, [Graph: South Sea], Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Melville - Moby-Dick or The Whale.pdf | 972ms |
| q16 | Near-miss: Both Bartleby and Billy Budd deal with authority and refusal, but in an office setting versus a naval setting respectively — how does each setting shape its exploration of institutional power? | 3.6/4 (90%) | 1.3/4 (32%) | Starbuck - History of the American Whale Fishery.pdf, gutenberg.org-NARRATIVE OF THE MOST EXTRAORDINARY AND DISTRESSING SHIPWRECK OF THE WHALE-SHIP ESSEX OF NANTUCKET W.pdf, Melville - Billy Budd.pdf, Melville - Typee.pdf | 1123ms |
| q17 | Cross-document: How might Weaver's critical biography and Minnigerode's letters/bibliography compilation serve different scholarly purposes for someone studying Melville? | 4/4 (100%) | 1.3/4 (31%) | Starbuck - History of the American Whale Fishery.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 1007ms |
| q18 | Cross-document: What thematic throughline connects the Book of Job's confrontation with an overwhelming natural force to Ahab's confrontation with the white whale in Moby-Dick? | 3.4/4 (85%) | 1.3/4 (33%) | Starbuck - History of the American Whale Fishery.pdf, Browne - Etchings of a Whaling Cruise.pdf, Melville - Moby-Dick or The Whale.pdf, Melville - Typee.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, gutenberg.org-NARRATIVE OF THE MOST EXTRAORDINARY AND DISTRESSING SHIPWRECK OF THE WHALE-SHIP ESSEX OF NANTUCKET W.pdf | 985ms |
| q19 | Edge case: Which single document in this cluster would be hardest to distinguish from Moby-Dick using only thematic or topical similarity, rather than direct textual overlap? | 3.4/4 (85%) | 1.6/4 (40%) | Browne - Etchings of a Whaling Cruise.pdf, Melville - Bartleby the Scrivener.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf, Melville - Typee.pdf, Starbuck - History of the American Whale Fishery.pdf | 1623ms |
| q20 | What does the presence of both fiction (Melville's other novels) and non-fiction (Starbuck's history, Browne's memoir) in this cluster allow a retrieval system to be tested on, that a fiction-only cluster could not? | 2.5/3 (82%) | 0.9/3 (31%) | Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Melville - Moby-Dick or The Whale.pdf, Browne - Etchings of a Whaling Cruise.pdf | 1069ms |

## Answers

### q01 — What is the central plot and thematic obsession of Melville's Moby-Dick?

(no response)

### q02 — What is Bartleby, the Scrivener about, and how does its tone and scale differ dramatically from Moby-Dick's?

(no response)

### q03 — What is Billy Budd about, and what moral/legal dilemma does it center on?

(no response)

### q04 — What is Typee, and how does it relate to Melville's own biographical experience, per Weaver's critical biography?

(no response)

### q05 — What historical event does Melville's Battle-Pieces and Aspects of the War address, and how does this differ subject-wise from his sea narratives?

(no response)

### q06 — What real 1820 historical event does the whaling-industry literature in this cluster provide context for, that directly inspired Moby-Dick's plot?

(no response)

### q07 — What does the Book of Job (KJV) concern, and what thematic parallels does it have with Moby-Dick, such as confrontation with an unknowable natural force?

(no response)

### q08 — According to Weaver's "Herman Melville: Mariner and Mystic," what dual character does the biography's title suggest about Melville?

(no response)

### q09 — What does D.H. Lawrence's Studies in Classic American Literature argue about Moby-Dick, based on its inclusion as literary criticism in this cluster?

(no response)

### q10 — What does Browne's Etchings of a Whaling Cruise document, and how does its firsthand account compare to Melville's fictionalized depiction in Moby-Dick?

(no response)

### q11 — What does Minnigerode's work compile regarding Melville, and what two types of material does its title indicate?

(no response)

### q12 — To whom did Melville dedicate Moby-Dick, and what work by that author is included in this cluster as contemporary context?

(no response)

### q13 — Cross-document: How does Hawthorne's The Scarlet Letter compare stylistically and thematically to Moby-Dick, given the two authors were contemporaries and friends?

(no response)

### q14 — Cross-document: How does Starbuck's History of the American Whale Fishery provide factual and industrial context that Melville draws on for Moby-Dick's whaling-industry digressions?

(no response)

### q15 — Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, but Typee is largely autobiographical while Moby-Dick is fully fictionalized — what does this distinction suggest about Melville's development as a writer?

(no response)

### q16 — Near-miss: Both Bartleby and Billy Budd deal with authority and refusal, but in an office setting versus a naval setting respectively — how does each setting shape its exploration of institutional power?

(no response)

### q17 — Cross-document: How might Weaver's critical biography and Minnigerode's letters/bibliography compilation serve different scholarly purposes for someone studying Melville?

(no response)

### q18 — Cross-document: What thematic throughline connects the Book of Job's confrontation with an overwhelming natural force to Ahab's confrontation with the white whale in Moby-Dick?

(no response)

### q19 — Edge case: Which single document in this cluster would be hardest to distinguish from Moby-Dick using only thematic or topical similarity, rather than direct textual overlap?

(no response)

### q20 — What does the presence of both fiction (Melville's other novels) and non-fiction (Starbuck's history, Browne's memoir) in this cluster allow a retrieval system to be tested on, that a fiction-only cluster could not?

(no response)


  ✅ Retrieval: 95.7%  Generation: 32.5%  (25.4/78)  avg 2660ms
