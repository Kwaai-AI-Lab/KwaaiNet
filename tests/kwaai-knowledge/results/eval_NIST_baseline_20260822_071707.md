# RAG Eval Report

**KB:** `NIST`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 76.4% (55.0/72) |
| Avg latency | 22950ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it? | 1/4 (25%) | [Graph: NIST AI Risk Management Framework (AI RMF)], nist.ai.100-1.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, CSI_AI_DATA_SECURITY.pdf | 20247ms |
| q02 | What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution? | 4/4 (100%) | csi-deploying-ai-systems-securely.pdf, [Graph: Principles for the security of machine learning], CSI_AI_DATA_SECURITY.pdf, NIST.AI.600-1.pdf, atlas_notebooklm.txt | 18185ms |
| q03 | Which three significant areas of data security risk does the CSI AI Data Security document examine in depth? | 3/3 (100%) | OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, NIST.AI.600-1.pdf, [Graph: Key Risks Overview: Data Security in LLMs], CSI_AI_DATA_SECURITY.pdf | 33853ms |
| q04 | What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus? | 4/4 (100%) | [Graph: NIST AI 800-1 2pd (Second Public Draft)], NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf | 19544ms |
| q05 | What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1? | 4/4 (100%) | nist.ai.100-1.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST.AI.600-1.pdf, [Graph: NIST AI Risk Management Framework (AI RMF)], NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, CSI_AI_DATA_SECURITY.pdf | 22308ms |
| q06 | According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others? | 2/2 (100%) | nist.ai.100-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST AI 600-1 RMF Generative AI Profile], OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 17541ms |
| q07 | What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to? | 2/4 (50%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, atlas_notebooklm.txt, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, NIST.AI.600-1.pdf, [Graph: GenAI Stages of Learning], OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf | 26960ms |
| q08 | In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking? | 2/4 (50%) | NIST.AI.600-1.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, owasp_llm_2025_notebooklm.txt, [Graph: LLM01:2025 — Prompt Injection], OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, atlas_notebooklm.txt, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 23549ms |
| q09 | According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection? | 2/3 (67%) | OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, owasp_llm_2025_notebooklm.txt, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, atlas_notebooklm.txt, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, [Graph: OWASP CycloneDX OWASP CycloneDX] | 16604ms |
| q10 | Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities. | 2/4 (50%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, [Graph: OWASP CycloneDX OWASP CycloneDX], owasp_llm_2025_notebooklm.txt, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, atlas_notebooklm.txt | 18540ms |
| q11 | What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export? | 3/3 (100%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, atlas_notebooklm.txt, owasp_llm_2025_notebooklm.txt, [Graph: MITRE ATLAS Adversarial Threat Landscape], Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, csi-deploying-ai-systems-securely.pdf | 19013ms |
| q12 | What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal? | 2/4 (50%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, [Graph: MITRE ATLAS Adversarial Threat Landscape], OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, atlas_notebooklm.txt | 16548ms |
| q13 | Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")? | 4/4 (100%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, [Graph: Principles for the security of machine learning], CSI_AI_DATA_SECURITY.pdf, NIST.AI.600-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf | 26151ms |
| q14 | What does NIST AI 100-4 primarily address regarding synthetic content? | 1/1 (100%) | [Graph: NIST AI 600-1 RMF Generative AI Profile], NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 16270ms |
| q15 | What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)? | 4/4 (100%) | NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST's Secure Software Development Framework (SSDF)], NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf | 24104ms |
| q16 | Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)? | 2/4 (50%) | [Graph: NIST AI Risk Management Framework (AI RMF)], OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, CSI_AI_DATA_SECURITY.pdf | 27692ms |
| q17 | Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)? | 3/4 (75%) | csi-deploying-ai-systems-securely.pdf, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, owasp_llm_2025_notebooklm.txt, NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: NIST AI 800-1 2pd (Second Public Draft)], NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf | 26688ms |
| q18 | Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1? | 3/4 (75%) | [Graph: NIST AI 600-1 RMF Generative AI Profile], nist.ai.100-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, NIST.AI.600-1.pdf | 28260ms |
| q19 | Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series? | 3/4 (75%) | nist.ai.100-1.pdf, NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, [Graph: NIST AI 600-1 RMF Generative AI Profile], Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST.AI.600-1.pdf | 26929ms |
| q20 | Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework. | 4/4 (100%) | OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, [Graph: MITRE ATLAS Adversarial Threat Landscape], OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, NIST.AI.600-1.pdf, atlas_notebooklm.txt, nist.ai.100-1.pdf | 30021ms |

## Answers

### q01 — According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it?

According to the sources [2] and [3], the document was co-authored by the National Security Agency (NSA) and the National Institute of Standards and Technology (NIST). 

The NIST AI RMF defines six major stages in the lifecycle of AI systems, starting from Plan & Design and progressing all the way to Operate & Monitor, as stated in the source [3].

### q02 — What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution?

The CSI AI Data Security document is marked TLP:CLEAR. This means that the information in this document can be shared without restriction, and recipients are not required to request permission or limit the distribution of the information.

According to the sources [1], [2], [5], and [20], TLP:CLEAR is a marking on the Traffic Light Protocol (TLP) that indicates the information can be shared freely without any restrictions.

### q03 — Which three significant areas of data security risk does the CSI AI Data Security document examine in depth?

The CSI AI Data Security document examines the following three significant areas of data security risk in AI systems:

1. Data supply chain
2. Maliciously modified ("poisoned") data
3. Data drift

### q04 — What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus?

According to source [11], the organization that published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models") is the U.S. Department of Commerce, specifically the National Institute of Standards and Technology (NIST).

According to source [11] again, the version in this corpus is the "Second Public Draft".

### q05 — What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1?

According to [12] and [11], the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1, are:

1. GOVERN
2. MAP
3. MEASURE
4. MANAGE

Additionally, [3] and [5] mention that these functions are further broken down into categories and subcategories.

It is also mentioned in [13] that the MANAGE function is further categorized into:

1. MANAGE 1: AI risks based on assessments and other analytical output from the MAP and MEASURE functions are prioritized, responded to, and managed.

MANAGE 1.1: A determination is made as to whether the AI system achieves its intended purposes and stated objectives and whether its development or deployment should proceed.

### q06 — According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others?

[3] states that "Valid & Reliable is a necessary condition of trustworthiness and is shown as the base for other trustworthiness characteristics."

### q07 — What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to?

The publication date of NIST.AI.600-1 is not explicitly mentioned in the sources, but it is available online in 2024, and a previous version (Initial Public Draft) was published in 2023 [274]. The publication "NIST Trustworthy and Responsible AI  NIST AI 600-1 Artificial Intelligence Risk Management Framework: Generative Artificial Intelligence Profile" is available for free from the website [275].

The "GenAI Profile" refers to a Generative Artificial Intelligence Profile, which is a companion resource to the AI Risk Management Framework (AI RMF 1.0) developed by the National Institute of Standards and Technology (NIST) [274]. The profile is a focused resource for generative AI, which is a specific type of artificial intelligence that is designed to generate synthetic content [275]. The profile provides guidance and best practices for developing and managing generative AI systems in a secure and trustworthy manner [275].

The "GenAI Profile" is mentioned in multiple sources, including [275] and [274], which provide more information about the AI Risk Management Framework and the role of the Generative AI Profile within it.

Source:
[274] National Institute of Standards and Technology. Artificial Intelligence Risk Management Framework (AI RMF 1.0). https://doi.org/10.6028/NIST.AI.100-1, 2023. Online.
[275] National Institute of Standards and Technology. Managing misuse risk for dual-use foundation models. https://doi.org/10.6028/NIST.AI.100-1, 2023. Online.

### q08 — In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking?

According to [20] OWASP Top 10 for Large Language Model Applications — 2025 Edition, LLM01 is "Prompt Injection" [LLM01:2025 — Prompt Injection].

OWASP distinguishes prompt injection from jailbreaking as follows:

* Prompt injection involves manipulating model responses through specific inputs to alter its behavior, which can include bypassing safety measures [6].
* Jailbreaking is a form of prompt injection where the attacker provides inputs that cause the model to disregard its safety protocols entirely [6].
* OWASP notes that prompt injection and jailbreaking are related concepts in LLM security, but are often used interchangeably. To mitigate prompt injection attacks, developers can build safeguards into system prompts and input handling, but effective prevention of jailbreaking requires ongoing updates to the model's training and safety mechanisms [14].
* Direct prompt injections occur when a user's prompt input directly alters the behavior of the model in unintended or unexpected ways, which can be either intentional or unintentional [14].
* Jailbreaking can be seen as a more severe form of prompt injection, where the model's safety protocols are completely disregarded.

### q09 — According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection?

According to [11] and [20], the research finding is that even sophisticated defenses like Retrieval Augmented Generation (RAG) and fine-tuning do not fully prevent these vulnerabilities.

### q10 — Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities.

The provided sources do not contain information about the specific prevention/mitigation strategies listed by OWASP for prompt injection vulnerabilities. However, OWASP Top 10 for LLM Applications 2025 does mention the following general measures to mitigate prompt injection vulnerabilities:

* Developers can build safeguards into system prompts and input handling to help mitigate prompt injection attacks.
* Effective prevention of jailbreaking requires ongoing updates to the model's training and safety mechanisms.

 OWASP Top 10 for LLM Applications 2025 does not list specific prevention/mitigation strategies.

### q11 — What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export?

MITRE ATLAS is modeled after the MITRE ATT&CK framework [17, 20]. The version of ATLAS represented in this corpus's export is 5.6.0 [20].

### q12 — What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal?

According to source [3], the first tactic listed in the MITRE ATLAS tactics section is:

AML.TA0002 — Reconnaissance

The stated adversary goal for this tactic is:

"The adversary is trying to gather information about the AI system they can use to plan future operations." [3]

### q13 — Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")?

The listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations") are:

1. Apostol Vassilev [7], [8], [14], [20]
2. Alina Oprea [7], [8], [14], [20]
3. Alie Fordyce [7], [8], [14], [20]
4. Hyrum Anderson [7], [8], [14], [20]
5. Xander Davies [7], [8], [14], [20]
6. Maia Hamin [7], [8], [14], [20]

Note that the authors contributed equally to this work, as mentioned in [3] and [8].

### q14 — What does NIST AI 100-4 primarily address regarding synthetic content?

NIST AI 100-4 primarily addresses reducing risks posed by synthetic content, particularly with regards to digital content transparency.

### q15 — What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)?

According to excerpt [2], NIST SP 800-218A is a Secure Software Development Practices for Generative AI and Dual-Use Foundation Models, which is an SSDF Community Profile. 

Excerpt [12] states that NIST SP 800-218A is "Secure Software Development Practices for Generative AI and Dual-Use Foundation Models: An SSDF Community Profile (2024)" and that it supplements the Secure Software Development Framework (SSDF) version 1.1. 

Excerpt [8] explains that the SSDF Community Profile is intended to be used in conjunction with NIST Special Publication (SP) 800-218, Secure Software Development Framework (SSDF) Version 1.1: Recommendations for Mitigating the Risk of Software Vulnerabilities [6] and should not be used without SP 800-218.

Therefore, the purpose of NIST SP 800-218A is to provide a community profile that supplements the base Secure Software Development Framework (SSDF) version 1.1 by adding practices, tasks, recommendations, considerations, notes, and informative references specific to AI model development throughout the software development life cycle.

### q16 — Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)?

The CSI AI Data Security document and the NIST AI 100-2e2025 document both discuss data supply chain risk and "poisoning" attacks. According to the CSI AI Data Security document [10], "data supply chain, maliciously modified data, and data drift" are three significant areas of data security risks in AI systems. 

In NIST AI 100-2e2025 [5], "Data Poisoning" (ID: NISTAML.013) is listed as an attack type, and it is described as an attack where the adversary manipulates the training data to affect the model's behavior. The document also mentions that "model poisoning attacks are also possible in supply-chain scenarios in which models or components of the model provided by suppliers are poisoned with malicious code" [5].

Both documents emphasize the importance of data supply chain security in preventing "poisoning" attacks. The CSI AI Data Security document provides a robust approach to securing AI data and addressing the risks associated with the data supply chain, malicious data, and data drift [18]. This guidance can be seen as a way to prevent the type of "poisoning" attacks described in NIST AI 100-2e2025.

However, the CSI AI Data Security document also acknowledges that traditional software supply chain risk management practices may not be sufficient to detect vulnerabilities in models such as those introduced through model poisoning attacks [19]. It proposes using methods from mechanistic interpretability to identify backdoor features and detecting and counteracting triggers when they are seen at inference time [19].

The relationship between the two documents is that the CSI AI Data Security document's guidance on data supply chain risk is meant to prevent the type of "poisoning" attacks described in NIST AI 100-2e2025, but it also recognizes that additional measures may be needed to detect and mitigate these attacks.

### q17 — Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)?

According to [2] NIST AI 800-1 2pd (Second Public Draft) Managing Misuse Risk for Dual-Use Foundation Models (January 2025), the document focuses on "dual-use foundation models", which are models that can be used for both beneficial and malicious purposes.

On the other hand, according to [11] OWASP Top 10 2025, the OWASP LLM Top 10 2025 focuses on "Large Language Model Applications", which are the applications that utilize LLMs.

Therefore, NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in that NIST AI 800-1 focuses on the foundation models themselves, while the OWASP LLM Top 10 2025 focuses on the applications that use these models.

It's worth noting that foundation models are a type of LLM, but not all LLM applications use foundation models. So, while there is some overlap between the two, they are not identical in scope.

Source: [2] NIST AI 800-1 2pd (Second Public Draft) Managing Misuse Risk for Dual-Use Foundation Models (January 2025) and [11] OWASP Top 10 2025

### q18 — Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1?

Based on the provided sources, it appears that the AI RMF (NIST AI 100-1) is a general framework for managing AI risks, while a "profile" document like NIST AI 600-1 (GenAI Profile) is a specialized document that elaborates on the risks unique to or exacerbated by a specific type of AI, in this case, generative AI.

The sources mention that the AI RMF is a living document that will be regularly updated, and that profiles like NIST AI 600-1 are intended to be part of this framework (Source [16]). They are described as "implementations of the AI RMF functions, categories, and subcategories for a specific setting or application" (Source [16]).

This suggests that the profiles are built on top of the base AI RMF, and provide more specific guidance on how to manage AI risks in a particular context or domain. The GenAI Profile, for example, is described as "elaborating risks unique to or exacerbated by generative AI and how to apply the AI RMF to manage them" (Source [15]).

In other words, the profiles are specialized documents that provide more detailed and specific guidance on how to manage AI risks in a particular context, while the base AI RMF provides a general framework for managing AI risks.

The relationship between the base AI RMF and the profiles can be thought of as follows:

* The base AI RMF (NIST AI 100-1) provides a general framework for managing AI risks, which includes the MAP, MEASURE, and MANAGE functions.
* The profiles, such as the GenAI Profile (NIST AI 600-1), are specialized documents that build on the base AI RMF and provide more specific guidance on how to manage AI risks in a particular context or domain.
* The profiles can be thought of as "implementations" of the base AI RMF, tailored to a specific setting or application.

This relationship is further supported by the fact that the base AI RMF and the profiles are both part of the NIST AI initiative, and are intended to be used together to manage AI risks.

### q19 — Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series?

Based on the sources, it appears that NIST uses the following terminology:

* "AI RMF core" documents have a numbering format of "NIST AI 100-1", where the "100" represents the document type (AI RMF core) and the "1" represents the version number. These documents seem to be fundamental or foundational documents in the AI RMF series.
* "Profiles" have a numbering format of "NIST AI 600-1", where the "600" represents the document type (profile) and the "1" represents the version number. These documents appear to be specialized or specific documents that build upon the core documents.

For example, [2] mentions the "NIST AI 100-1 AI Risk Management Framework (AI RMF) and NIST AI 600-1 RMF Generative AI Profile (AI RMF Generative AI Profile)". This suggests that NIST AI 100-1 is the core document, while NIST AI 600-1 is a profile document that focuses on Generative AI.

Similarly, [19] mentions "NIST AI 600-1 Artificial Intelligence Risk Management Framework: Generative Artificial Intelligence Profile", which further reinforces the idea that "600" represents a profile document.

It's worth noting that the exact distinction between "core" and "profile" documents is not explicitly stated in the sources, and further research may be needed to fully understand the nuances of NIST's documentation structure.

### q20 — Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework.

Based on the provided excerpts, it appears that MITRE ATLAS Adversarial Threat Landscape is modeled after the MITRE ATT&CK framework.

[1] states that the MITRE ATLAS is a creative work and publication that provides information on how to acquire and synthesize chemical biological agents from natural environments or laboratories, as well as design and testing of such agents. It also discusses the potential for high-impact dual-use applications in understanding CB agent behavior and developing countermeasures.

However, there is no direct mention of the conceptual structure borrowed from the ATT&CK framework. But, [4] states that "Technique ID: AML.T0024" is related to "Exfiltration via AI Inference API" which is a technique used in the ATT&CK framework to describe how an adversary can exfiltrate sensitive information from a victim's system.

[4] further states that the ATT&CK framework is used to describe how an adversary can use AI and machine learning techniques to exfiltrate sensitive information from a victim's system.

[12] also states that the ATT&CK framework is used to describe how an adversary can use AI and machine learning techniques to exfiltrate sensitive information from a victim's system.

Therefore, based on the provided excerpts, it appears that the conceptual structure of the MITRE ATLAS is borrowed from the MITRE ATT&CK framework, specifically the techniques used to exfiltrate sensitive information from a victim's system.

