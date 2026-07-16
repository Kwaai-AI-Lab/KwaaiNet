# RAG Eval Report

**KB:** `NIST`  **Model:** `llama3.1:8b`

**Flags:** top_k=5  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 63.9% (46.0/72) |
| Avg latency | 2964ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it? | 2/4 (50%) | nist.ai.100-1.pdf, CSI_AI_DATA_SECURITY.pdf, NIST - SP 800-53 Control Overlays for Securing AI Systems - Concept Paper.pdf | 6075ms |
| q02 | What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution? | 4/4 (100%) | csi-deploying-ai-systems-securely.pdf, CSI_AI_DATA_SECURITY.pdf | 2019ms |
| q03 | Which three significant areas of data security risk does the CSI AI Data Security document examine in depth? | 3/3 (100%) | CSI_AI_DATA_SECURITY.pdf | 2418ms |
| q04 | What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus? | 3/4 (75%) | NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf | 2345ms |
| q05 | What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1? | 2/4 (50%) | NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf, nist.ai.100-1.pdf | 2326ms |
| q06 | According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others? | 2/2 (100%) | nist.ai.100-1.pdf | 2279ms |
| q07 | What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to? | 2/4 (50%) | NIST.AI.600-1.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 5635ms |
| q08 | In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking? | 0/4 (0%) | owasp_llm_2025_notebooklm.txt | 2622ms |
| q09 | According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection? | 0/3 (0%) | OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, owasp_llm_2025_notebooklm.txt | 1303ms |
| q10 | Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities. | 2/4 (50%) | OWASP - GenAI Data Security Risks and Mitigations 2026 v1.0.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf, owasp_llm_2025_notebooklm.txt | 2771ms |
| q11 | What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export? | 3/3 (100%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, atlas_notebooklm.txt | 2140ms |
| q12 | What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal? | 2/4 (50%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, atlas_notebooklm.txt | 2865ms |
| q13 | Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")? | 4/4 (100%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 2828ms |
| q14 | What does NIST AI 100-4 primarily address regarding synthetic content? | 1/1 (100%) | Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf, NIST - Reducing Risks Posed by Synthetic Content - AI 100-4.pdf | 1559ms |
| q15 | What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)? | 4/4 (100%) | NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf | 3154ms |
| q16 | Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)? | 3/4 (75%) | CSI_AI_DATA_SECURITY.pdf, Vassilev et al. - Adversarial Machine Learning Taxonomy and Terminology of Attacks and Mitigations - NIST AI 100-2e2025.pdf | 3789ms |
| q17 | Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)? | 2/4 (50%) | OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf, NIST - Secure Software Development Practices for Generative AI and Dual-Use Foundation Models - SP 800-218A.pdf, OWASP - LLM and Gen AI Data Security Best Practices 2025.pdf | 4176ms |
| q18 | Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1? | 3/4 (75%) | NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, NIST.AI.600-1.pdf, nist.ai.100-1.pdf | 3813ms |
| q19 | Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series? | 0/4 (0%) | NIST.AI.600-1.pdf, NIST - Managing Misuse Risk for Dual-Use Foundation Models - AI 800-1 Initial Public Draft 2.pdf, nist.ai.100-1.pdf | 1424ms |
| q20 | Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework. | 4/4 (100%) | atlas_notebooklm.txt, OWASP - LLM Applications Cybersecurity and Governance Checklist v1.1 English.pdf | 3748ms |

## Answers

### q01 — According to the CSI AI Data Security guidance, which agencies co-authored the document, and how many major AI lifecycle stages does the NIST AI RMF define within it?

The provided sources do not contain that information.

However, we can infer some related facts from the other excerpts:

* The CSI AI Data Security guidance is mentioned in [3] as part of the Table 1.
* It is stated in [2] and [4] that various agencies and entities have collaborated with NIST on their efforts.
 

But, we cannot determine which specific document the CSI AI Data Security guidance is a part of.

As for the second question:

According to [3], the National Institute of Standards and Technology (NIST) defines six major stages in the lifecycle of AI systems.

### q02 — What is the CSI AI Data Security document's Traffic Light Protocol (TLP) marking, and what does that marking mean for distribution?

The CSI AI Data Security document's Traffic Light Protocol (TLP) marking is TLP:CLEAR. This means that the information in this document may be distributed without restriction [1], [2], [4], [5]. 

In the context of the Traffic Light Protocol, "CLEAR" signifies that the information can be shared widely and openly, without any restrictions on distribution or usage.

### q03 — Which three significant areas of data security risk does the CSI AI Data Security document examine in depth?

The CSI AI Data Security document examines the following three significant areas of data security risk in depth:

1. Data supply chain [1, 2, 3, 4, 5]
2. Maliciously modified (“poisoned”) data [1, 2, 3]
3. Data drift [1, 2]

Note that [3] and [5] are identical excerpts, but they both list the same three areas of data security risk.

### q04 — What organization published NIST AI 800-1 ("Managing Misuse Risk for Dual-Use Foundation Models"), and what draft stage was the version in this corpus?

According to [3], the publication is available from the National Institute of Standards and Technology (NIST), specifically under Charles H. Romine, Acting Under Secretary of Commerce for Standards and Technology and Acting NIST Director.

[1] states that the document is "Managing Misuse Risk for Dual-Use Foundation Models" in the U.S. version, with a publication date of January 2025.

The draft stage mentioned is the Second Public Draft, according to [3] and also present in other excerpts.

### q05 — What are the four core functions of the NIST AI Risk Management Framework (AI RMF 1.0), as defined in NIST AI 100-1?

According to source [4], the Core is composed of four functions:

1. GOVERN
2. MAP
3. MEASURE
4. MANAGE

Source [4] also states that these high-level functions are broken down into categories and subcategories, which are further subdivided into specific actions and outcomes.

Note: The AI RMF 1.0 is mentioned in source [1], but it only provides general information about the framework, without detailing its core functions.

### q06 — According to NIST AI 100-1, which trustworthiness characteristic is described as a necessary condition that forms the base for the others?

According to [4], Valid & Reliable is a necessary condition of trustworthiness and is shown as the base for other trustworthiness characteristics. 

Additionally, [2] states: "Valid & Reliable is a necessary condition of trustworthiness and is shown as the base for other trustworthi-ness characteristics."

[1] also mentions that Valid & Reliable is a necessary condition of trustworthiness.

All three excerpts agree on this point.

### q07 — What is the publication date and purpose of NIST.AI.600-1, and what does "GenAI Profile" refer to?

According to source [3], the publication date of NIST AI 100-2e2025 (not NIST.AI.600-1) is March 2025.

However, according to source [5] and other sources ([1] and [4]), there are multiple documents related to NIST AI 600-1:

* Source [5]: NIST SP 800-218A mentions "NIST AI 600-1" as a document that is available online.
* Source [1] and [3]: Both mention NIST AI 600-1 without mentioning its specific purpose or publication date.

Source [2] states: "[273]  NIST.  Artificial Intelligence Risk Management Framework:  Generative Artificial 
Intelligence Profile.  https://airc.nist.gov/docs/NIST.AI.600-1.GenAI-Profile.ipd.pdf, 
2024." which implies that a version of the document "NIST AI 600-1" was published in 2024.

As for the purpose of NIST AI 600-1 and what "GenAI Profile" refers to:

* The title itself mentions that it is an "Artificial Intelligence Risk Management Framework: Generative Artificial Intelligence Profile".
* Source [3] explains that GenAI (Generative Artificial Intelligence) is a branch of AI that develops models that can generate content, such as images, text, and other media.
* Source [5] mentions the same framework in the context of "Secure Software Development Practices for Generative AI and Dual-Use Foundation Models".

The exact purpose of NIST AI 600-1 (specifically) is not clearly stated in the sources. However, based on the context, it appears to be a document related to artificial intelligence risk management, specifically focusing on generative artificial intelligence.

To clarify further, I would like to say that there seems to be multiple documents associated with NIST AI 600-1 across different sources ([1], [3], and [5]).

### q08 — In the OWASP Top 10 for LLM Applications 2025, what is LLM01, and how does OWASP distinguish prompt injection from jailbreaking?

According to [1] and [2], LLM01:2025 refers to a Prompt Injection Vulnerability. According to [2], these inputs can affect the model even if they are imperceptible to humans.

Regarding the distinction between prompt injection and jailbreaking, [3] states that while prompt injection involves manipulating model responses through specific inputs to alter its behavior, bypassing safety measures, jailbreaking is a form of prompt injection where the attacker provides inputs that cause the model to disregard its safety protocols entirely.

### q09 — According to the OWASP document, what specific research finding is noted about RAG and fine-tuning's ability to mitigate prompt injection?

The provided sources do not contain that information.

### q10 — Name two of the six prevention/mitigation strategies OWASP lists for prompt injection vulnerabilities.

The sources do not directly list "prevention/mitigation strategies" but rather discuss ways to mitigate the impact of prompt injections or approaches that aim to make LLM outputs more relevant and accurate, which may also help prevent prompt injection vulnerabilities.

However, according to [2] and [4], two measures mentioned to limit the impact of prompt injections are:

1. Provide specific instructions about the model's role, capabilities, and limitations within the system prompt.
2. Enforce strict context adherence, limit responses to specific tasks or topics, and instruct the model to ignore attempts to modify core instructions.

Note: The sources discuss ways to mitigate the impact of prompt injection attacks rather than directly listing OWASP prevention strategies.

### q11 — What is MITRE ATLAS modeled after, and what version of ATLAS is represented in this corpus's export?

MITRE ATLAS is modeled after the MITRE ATT&CK framework [1-4]. 

The complete MITRE ATLAS (Adversarial Threat Landscape for Artificial-Intelligence Systems) knowledge base, including all tactics, techniques, sub-techniques, mitigations, and case studies in version 5.6.0, was exported on 2026-06-09 [1].

### q12 — What is the first tactic listed in the MITRE ATLAS tactics section, and what is its stated adversary goal?

The first tactic listed in the MITRE ATLAS tactics section is AML.TA0002 — Reconnaissance.

According to [1], the adversary's goal for this tactic is: "to gather information about the AI system they can use to plan future operations." 

This is also mentioned in [2] as: "The adversary is trying to gather information about the AI system they can use to plan future operations."

Additionally, [4] mentions that in the ATT&CK Matrix, this tactic begins the attack order and progresses towards exfiltration or impact.

### q13 — Who are the listed authors of NIST AI 100-2e2025 ("Adversarial Machine Learning: A Taxonomy and Terminology of Attacks and Mitigations")?

According to [3] Vassilev A, Oprea A, Fordyce A, Anderson H, Davies X, Hamin M (2025) Adversarial Machine Learning: A 
Taxonomy and Terminology of Attacks and Mitigations. (National Institute of Standards and Technology, 
Gaithersburg, MD)  NIST Trustworthy and Responsible AI, NIST AI 100-2e2025. 

The listed authors are:
1. Apostol Vassilev
2. Alina Oprea
3. Alie Fordyce
4. Hyrum Anderson
5. Xander Davies
6. Maia Hamin

### q14 — What does NIST AI 100-4 primarily address regarding synthetic content?

NIST AI 100-4 primarily addresses reducing risks posed by synthetic content through an overview of technical approaches to digital content transparency. [1][2]

### q15 — What is the purpose of NIST SP 800-218A, and how does it relate to the base Secure Software Development Framework (SSDF)?

According to [1], NIST SP 800-218A "Secure Software Development Practices for Generative AI and Dual-Use Foundation Models" augments the secure software development practices and tasks defined in Secure Software Development Framework (SSDF) version 1.1 by adding practices, tasks, recommendations, considerations, notes, and informative references that are specific to AI model development throughout the software development life cycle.

Additionally, [4] states that this Profile supplements what SSDF version 1.1 already includes and is intended to be used in conjunction with NIST Special Publication (SP) 800-218.

It seems that NIST SP 800-218A provides additional guidelines and recommendations specific to AI model development, which can be applied on top of the existing SSDF framework.

### q16 — Cross-document: How does the CSI AI Data Security document's guidance on data supply chain risk relate to the "poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.)?

According to [4], the guidance provided in the CSI AI Data Security document addresses the risks associated with the data supply chain. Similarly, in [3] and [5], "data supply chain" is mentioned as one of the significant areas of data security risks in AI systems.

In NIST AI 100-2e2025 (Vassilev et al.), which is referenced in [1] and [2], "Data Poisoning" (ID: NISTAML.013) is described as a type of adversarial attack on machine learning models. This suggests that the "poisoning" attack category is a specific manifestation of data supply chain risk, where maliciously modified or contaminated data is intentionally introduced into the AI system.

Therefore, it can be inferred that the CSI AI Data Security document's guidance on data supply chain risk relates to the "Data Poisoning" attack category described in NIST AI 100-2e2025 (Vassilev et al.). The guidelines aim to mitigate the risks associated with data supply chain, including maliciously modified or contaminated data, which is a key aspect of the "poisoning" attack.

### q17 — Cross-document: How do NIST AI 800-1 and the OWASP LLM Top 10 2025 differ in what population of AI systems they scope (foundation models vs. LLM applications)?

The provided sources do not explicitly state the difference between foundation models and large language model (LLM) applications as specified by NIST AI 800-218A and OWASP LLM Top 10 2025, respectively.

However, [4] mentions "Generative AI and Dual-Use Foundation Models" in the context of NIST SP 800-218A. It does not provide a direct comparison with OWASP LLM Top 10 2025.

OWASP LLM Top 10 2025 is mentioned as "LLM Top 10 for Large Language Model Applications." This suggests that it focuses on LLM applications rather than foundation models specifically.

Therefore, based on the available information, we can infer that OWASP LLM Top 10 2025 likely scopes large language model (LLM) applications, whereas NIST AI 800-218A seems to focus on generative AI and dual-use foundation models. However, a more explicit comparison or clarification of their scoping differences is not provided in the given sources.

The exact difference between these populations of AI systems (foundation models vs. LLM applications) as per NIST AI 800-218A and OWASP LLM Top 10 2025 cannot be directly stated based on the available information.

### q18 — Cross-document: NIST.AI.600-1 (GenAI Profile) and NIST AI 100-1 (AI RMF 1.0) come from the same overall framework initiative — what is the relationship between the base AI RMF and a "profile" document like AI 600-1?

According to [3], NIST.AI.600-1 (GenAI Profile) is a companion resource for the AI Risk Management Framework (AI RMF 1.0), specifically for Generative AI.

A "profile" document, such as AI 600-1, appears to be an extension or specialization of the base AI RMF, tailored to a specific area or application of AI, in this case, Generative AI. The profile is likely intended to provide additional guidance and considerations specific to the challenges and risks associated with generative AI.

The fact that NIST AI 600-1 uses the same versioning system as the AI RMF (e.g., "AI RMF 1.0") suggests a close relationship between the two documents, with the profile document building on or supplementing the base framework.

It's worth noting that [4] mentions that the AI RMF is intended to be a living document, and NIST will review its content regularly, which implies that companion profiles like AI 600-1 may also be subject to periodic updates to reflect changes in the field.

### q19 — Edge case: NIST AI 100-1 and NIST.AI.600-1 are numbered differently (100-1 vs 600-1) — what distinguishes a numbered "AI RMF core" document from a "profile" in NIST's AI publication series?

The provided sources do not contain that information.

### q20 — Edge case: MITRE ATLAS is explicitly modeled after another well-known MITRE framework — name it, and explain what conceptual structure (tactics/techniques) it borrows from that framework.

The well-known MITRE framework that MITRE ATLAS is modeled after is the MITRE ATT&CK framework.

MITRE ATLAS borrows its conceptual structure of tactics and techniques from the MITRE ATT&CK framework. According to [2], the MITRE ATT&CK matrix contains a collection of strategies used by adversaries to achieve a certain goal, which are classified as tactics. The objectives are outlined in attack order, beginning with reconnaissance and progressing to the eventual goal of exfiltration or impact.

Similarly, [4] states that ATLAS organizes techniques under the following tactics: AML.TA0002 — Reconnaissance. This indicates that both frameworks share a similar structure where tactics are high-level goals and techniques are specific actions taken to achieve those goals.

[3] also mentions that ATLAS is based on the MITRE ATT&CK architecture, and its tactics and procedures complement those found in ATT&CK, further supporting this conclusion.

Note: The provided sources do not provide a detailed explanation of the conceptual structure, but they clearly indicate that MITRE ATLAS borrows from the MITRE ATT&CK framework.

