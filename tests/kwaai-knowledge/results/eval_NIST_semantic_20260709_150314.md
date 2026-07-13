[2m2026-07-09T22:04:44.365303Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 9be6c04c-0d49-4173-9287-92d9b6aa3a2a: routing: not found
[2m2026-07-09T22:04:44.365377Z[0m [32m INFO[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → http://127.0.0.1:52084 (via ollama-proxy)
[2m2026-07-09T22:04:53.035494Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

╭─────────────────────────────────────────────────────────────────────╮
│                  RAG Eval  (20 questions, kb=NIST)                  │
╰─────────────────────────────────────────────────────────────────────╯

  Model:     llama3.1:8b
  Inference: http://127.0.0.1:52084
  top_k=20  mode=iterative  graph_mode=inject  query_classify=rule  hyde=false  rerank=false  understand=false  llm_judge=false  summary_expansion=false  biographical_expansion=false
─────────────────────────────────────────────────────────────────────
  [ 1/20] According to the CSI AI Data Security guidance, which agencies co-auth
[2m2026-07-09T22:04:55.659627Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  13/14 query terms found  (93%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

[2m2026-07-09T22:05:29.144209Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 0bb2d240-8974-4528-9e4d-7d12d7750131: routing: not found
[2m2026-07-09T22:05:29.144241Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=3.2/4  gen=0/4  35177ms
  [ 2/20] What is the CSI AI Data Security document's Traffic Light Protocol (TL
[2m2026-07-09T22:05:30.518414Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  8/9 query terms found  (89%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

[2m2026-07-09T22:05:32.815052Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 6696b735-5a49-42ee-93ea-a8e2a12648b9: routing: not found
[2m2026-07-09T22:05:32.815075Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
         → ret=4/4  gen=0/4  3314ms
  [ 3/20] Which three significant areas of data security risk does the CSI AI Da
[2m2026-07-09T22:05:33.257134Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

[2m2026-07-09T22:05:35.703993Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 3866c74b-c801-4664-94f0-3d0ff832823e: routing: not found
[2m2026-07-09T22:05:35.704015Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T22:05:35.704054Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (3 consecutive connection failure(s)); will retry in 30s
         → ret=3/3  gen=0/3  2611ms
  [ 4/20] What organization published NIST AI 800-1 ("Managing Misuse Risk for D
[2m2026-07-09T22:05:36.385231Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  13/14 query terms found  (93%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

         → ret=4/4  gen=0/4  2724ms
  [ 5/20] What are the four core functions of the NIST AI Risk Management Framew
[2m2026-07-09T22:05:39.124103Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  9/9 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  2464ms
  [ 6/20] According to NIST AI 100-1, which trustworthiness characteristic is de
[2m2026-07-09T22:05:42.068659Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=2/2  gen=0/2  2956ms
  [ 7/20] What is the publication date and purpose of NIST.AI.600-1, and what do
[2m2026-07-09T22:05:44.990535Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 10 documents
  ○ Coverage  7/7 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 9 documents — passing to LLM

         → ret=3.0/4  gen=0/4  2496ms
  [ 8/20] In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how 
[2m2026-07-09T22:05:47.736962Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  8/8 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.0/4  gen=0/4  2590ms
  [ 9/20] According to the OWASP document, what specific research finding is not
[2m2026-07-09T22:05:50.702094Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  10/12 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=3/3  gen=0/3  2663ms
  [10/20] Name two of the six prevention/mitigation strategies OWASP lists for p
[2m2026-07-09T22:05:53.487718Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=2.1/4  gen=0/4  2630ms
  [11/20] What is MITRE ATLAS modeled after, and what version of ATLAS is repres
[2m2026-07-09T22:05:56.635477Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  7/8 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3/3  gen=0/3  3089ms
  [12/20] What is the first tactic listed in the MITRE ATLAS tactics section, an
[2m2026-07-09T22:05:59.693967Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 11 documents
  ○ Coverage  8/10 query terms found  (80%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 5 documents — passing to LLM

         → ret=3.2/4  gen=0/4  2575ms
  [13/20] Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine
[2m2026-07-09T22:06:02.710645Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  11/11 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=4/4  gen=0/4  2781ms
  [14/20] What does NIST AI 100-4 primarily address regarding synthetic content?
[2m2026-07-09T22:06:05.533156Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  7/7 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 3 documents — passing to LLM

[2m2026-07-09T22:06:08.367929Z[0m [32m INFO[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → HalfOpen (cooldown elapsed)
[2m2026-07-09T22:06:08.453624Z[0m [33m WARN[0m [2mkwaai_p2p_daemon::persistent[0m[2m:[0m Daemon error for call 02828278-d8af-4233-8b28-5640abb200fb: routing: not found
[2m2026-07-09T22:06:08.453641Z[0m [33m WARN[0m [2mkwaainet::ollama_proxy[0m[2m:[0m inference_proxy: P2P call to 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs via /kwaai/ollama-proxy/1.0.0: Protocol error: Daemon error: routing: not found
[2m2026-07-09T22:06:08.453671Z[0m [33m WARN[0m [2mkwaainet::circuit_breaker[0m[2m:[0m circuit breaker: 12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs → Open (4 consecutive connection failure(s)); will retry in 30s
         → ret=1/1  gen=0/1  3084ms
  [15/20] What is the purpose of NIST SP 800-218A, and how does it relate to the
[2m2026-07-09T22:06:08.998987Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  10/10 query terms found  (100%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 2 documents — passing to LLM

         → ret=4/4  gen=0/4  2632ms
  [16/20] Cross-document: How does the CSI AI Data Security document's guidance 
[2m2026-07-09T22:06:11.910639Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  14/16 query terms found  (88%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 4 documents — passing to LLM

         → ret=2.1/4  gen=0/4  2515ms
  [17/20] Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 dif
[2m2026-07-09T22:06:14.460532Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 14 documents
  ○ Coverage  10/12 query terms found  (83%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=3.0/4  gen=0/4  2174ms
  [18/20] Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RM
[2m2026-07-09T22:06:16.871121Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  14/17 query terms found  (82%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=4/4  gen=0/4  2214ms
  [19/20] Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (1
[2m2026-07-09T22:06:19.369202Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 13 documents
  ○ Coverage  11/14 query terms found  (79%)
  ○ Coverage  satisfied — skipping gap-fill rounds
  ○ Final     20 chunks from 6 documents — passing to LLM

         → ret=3.2/4  gen=0/4  2212ms
  [20/20] Edge case: MITRE ATLAS is explicitly modeled after another well-known 
[2m2026-07-09T22:06:21.767109Z[0m [32m INFO[0m [2mkwaai_rag::graph[0m[2m:[0m graph store loaded [3mentities[0m[2m=[0m680 [3mrelations[0m[2m=[0m0

  ○ Round 1   vector+graph fusion → 80 chunks from 12 documents
  ○ Coverage  10/15 query terms found  (67%)
  ○ Round 2   gap-filling for [explicitly, another, conceptual, tactics/techniques, borrows]
  ○ Round 2   added 3 chunks via graph gap-fill
  ○ Round 3   LLM reformulation for [explicitly, conceptual, tactics/techniques]
  ○ Round 3   reformulation failed (error decoding response body: expected value at line 1 column 1)
  ○ Final     20 chunks from 7 documents — passing to LLM

         → ret=4/4  gen=0/4  2434ms

# RAG Eval Report

**KB:** `NIST`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 87.2% (62.8/72) |
| Generation recall (token-overlap + semantic) | 0.0% (0.0/72) |
| Scoring mode | token-overlap + semantic embedding (low=0.55, high=0.85) |
| Avg latency | 4266ms |

## Per-question results

| ID | Question | Retrieval | Generation | Sources | Latency |
|----|----------|-----------|------------|---------|--------|
| q01 | According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it? | 3.2/4 (79%) | 0/4 (0%) | [Graph: AI-DSPM Data Security Posture Management], CSI_AI_DATA_SECURITY.pdf, NIST.AI.600-1.pdf, nist.ai.100-1.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 35177ms |
| q02 | What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution? | 4/4 (100%) | 0/4 (0%) | CSI_AI_DATA_SECURITY.pdf, NIST.AI.600-1.pdf, csi-deploying-ai-systems-securely.pdf, [Graph: AI-DSPM Data Security Posture Management], OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf | 3314ms |
| q03 | Which three significant areas of data security risk does the CSI AI Data Security document examine in depth? | 3/3 (100%) | 0/3 (0%) | OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, NIST.AI.600-1.pdf, CSI_AI_DATA_SECURITY.pdf, [Graph: AI-DSPM Data Security Posture Management] | 2611ms |
| q04 | What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus? | 4/4 (100%) | 0/4 (0%) | NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST AI 800-1 2pd] | 2724ms |
| q05 | What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1? | 4/4 (100%) | 0/4 (0%) | [Graph: NIST AI 800-1 2pd], NIST.AI.600-1.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, nist.ai.100-1.pdf, CSI_AI_DATA_SECURITY.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf | 2464ms |
| q06 | According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others? | 2/2 (100%) | 0/2 (0%) | NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, [Graph: NIST AI 800-1 2pd], nist.ai.100-1.pdf | 2956ms |
| q07 | What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to? | 3.0/4 (76%) | 0/4 (0%) | NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST.AI.600-1.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, CSI_AI_DATA_SECURITY.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 2496ms |
| q08 | In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking? | 3.0/4 (76%) | 0/4 (0%) | owasp_llm_2025_notebooklm.txt, [Graph: LLM Prompt Injection], Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, NIST.AI.600-1.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf | 2590ms |
| q09 | According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection? | 3/3 (100%) | 0/3 (0%) | owasp_llm_2025_notebooklm.txt, atlas_notebooklm.txt, [Graph: INDIRECT PROMPT INJECTION], OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf | 2663ms |
| q10 | Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities. | 2.1/4 (53%) | 0/4 (0%) | owasp_llm_2025_notebooklm.txt, [Graph: INDIRECT PROMPT INJECTION], OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, atlas_full.json | 2630ms |
| q11 | What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export? | 3/3 (100%) | 0/3 (0%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, [Graph: MITRE AI Red Team], owasp_llm_2025_notebooklm.txt, atlas_notebooklm.txt, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, atlas_full.json, csi-deploying-ai-systems-securely.pdf | 3089ms |
| q12 | What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal? | 3.2/4 (79%) | 0/4 (0%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, atlas_full.json, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, atlas_notebooklm.txt, [Graph: MITRE AI Red Team] | 2575ms |
| q13 | Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")? | 4/4 (100%) | 0/4 (0%) | CSI_AI_DATA_SECURITY.pdf, NIST.AI.600-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, [Graph: NIST AI 800-1 2pd], NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 2781ms |
| q14 | What does NIST AI 100-4 primarily address regarding synthetic content? | 1/1 (100%) | 0/1 (0%) | NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST AI 800-1 2pd], NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 3084ms |
| q15 | What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)? | 4/4 (100%) | 0/4 (0%) | NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, [Graph: NIST SP 800-218A] | 2632ms |
| q16 | Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)? | 2.1/4 (53%) | 0/4 (0%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, [Graph: AI-DSPM Data Security Posture Management], OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, CSI_AI_DATA_SECURITY.pdf | 2515ms |
| q17 | Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)? | 3.0/4 (75%) | 0/4 (0%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, owasp_llm_2025_notebooklm.txt, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST AI 800-1 2pd], csi-deploying-ai-systems-securely.pdf | 2174ms |
| q18 | Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1? | 4/4 (100%) | 0/4 (0%) | nist.ai.100-1.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, [Graph: NIST AI 800-1 2pd], NIST.AI.600-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 2214ms |
| q19 | Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series? | 3.2/4 (79%) | 0/4 (0%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST.AI.600-1.pdf, [Graph: NIST AI 800-1 2pd], nist.ai.100-1.pdf, NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf | 2212ms |
| q20 | Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework. | 4/4 (100%) | 0/4 (0%) | owasp_llm_2025_notebooklm.txt, atlas_full.json, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, atlas_notebooklm.txt, csi-deploying-ai-systems-securely.pdf, [Graph: MITRE AI Red Team], OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf | 2434ms |

## Answers

### q01 — According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it?

(no response)

### q02 — What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution?

(no response)

### q03 — Which three significant areas of data security risk does the CSI AI Data Security document examine in depth?

(no response)

### q04 — What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus?

(no response)

### q05 — What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1?

(no response)

### q06 — According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others?

(no response)

### q07 — What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to?

(no response)

### q08 — In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking?

(no response)

### q09 — According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection?

(no response)

### q10 — Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities.

(no response)

### q11 — What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export?

(no response)

### q12 — What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal?

(no response)

### q13 — Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")?

(no response)

### q14 — What does NIST AI 100-4 primarily address regarding synthetic content?

(no response)

### q15 — What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)?

(no response)

### q16 — Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)?

(no response)

### q17 — Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)?

(no response)

### q18 — Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1?

(no response)

### q19 — Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series?

(no response)

### q20 — Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework.

(no response)


  ✅ Retrieval: 87.2%  Generation: 0.0%  (0.0/72)  avg 4266ms
