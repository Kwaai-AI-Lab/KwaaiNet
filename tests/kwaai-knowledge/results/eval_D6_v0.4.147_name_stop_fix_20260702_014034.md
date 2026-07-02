[2m2026-07-02T08:40:55.142550Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

╭─────────────────────────────────────────────────────────────────────╮
│                   RAG Eval  (40 questions, kb=D6)                   │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://localhost:11434
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/40] Who is the author?
[2m2026-07-02T08:41:10.163891Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  0/0 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 3/3 keywords  68707ms
  [ 2/40] Who are the author's children?
[2m2026-07-02T08:42:11.985047Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 3/3 keywords  31506ms
  [ 3/40] Who are the author's grandchildren?
[2m2026-07-02T08:42:58.411010Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 0/6 keywords  42835ms
  [ 4/40] To whom is the book dedicated?
[2m2026-07-02T08:43:47.974666Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 1 documents — passing to LLM

         → 4/4 keywords  51574ms
  [ 5/40] Who was J.M.H. Gool?
[2m2026-07-02T08:44:39.160114Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  2/2 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 6/8 keywords  61428ms
  [ 6/40] Tell me about Buitencingle.
[2m2026-07-02T08:45:44.787749Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m928 [3mrelations[0m[2m=[0m221

  ○ Round 1   vector+graph fusion → 80 chunks from 1 documents
  ○ Coverage  1/1 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → 4/8 keywords  59823ms
  [ 7/40] Who is the author's wife?
Error: opening graph store

Caused by:
    0: opening graph store at /Users/rezarassool/.kwaainet/rag/D6/graph-788496c3-82e1-4fcb-89fa-9df7161a53f1.redb
    1: Database already open. Cannot acquire lock.
