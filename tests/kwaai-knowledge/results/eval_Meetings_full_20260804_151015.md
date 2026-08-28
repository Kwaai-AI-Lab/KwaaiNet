# RAG Eval Report

**KB:** `Meetings`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 77.9% (46.0/59) |
| Generation recall (token-overlap + semantic) | 68.5% (40.4/59) |
| Scoring mode | token-overlap + semantic embedding (low=0.30, high=0.85) |
 | Avg judge score | 1.20/2.00 (20 questions scored) |
| Avg latency | 11198ms |

## Per-question results

| ID | Question | Retrieval | Generation | Judge | Sources | Latency |
|----|----------|-----------|------------|-------|---------|--------|
| q01 | What drawing tool did Reza introduce to the interns in the May 26 standup, and what was it used for? | 3/3 (100%) | 3/3 (100%) | 2/2 | Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt | 8805ms |
| q02 | According to Reza's analogy in the May 26 standup, what is "the missing piece" in current AI systems compared to the human brain? | 2/2 (100%) | 1.3/2 (63%) | 1/2 | Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt | 9127ms |
| q03 | Who is Mitch Travers, and what phrase did Reza attribute to him regarding Kwai? | 3/3 (100%) | 3/3 (100%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt | 8791ms |
| q04 | What personal event did Reza mention happened on his birthday, and where did he go? | 2/2 (100%) | 1.3/2 (64%) | 0/2 | Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt | 8806ms |
| q05 | Who is Solomon Satari, and what was Reza doing with him over the prior weekend according to the May 26 standup? | 3/3 (100%) | 3/3 (100%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt | 33816ms |
| q06 | Who is "Doc Searles" as referenced in the May 26 standup, and how does Reza describe his role? | 1.8/3 (60%) | 2.0/3 (68%) | 1/2 | Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt | 11680ms |
| q07 | Who is Professor Ruth Rasul, what is her academic background, and what topic did she cover as a guest lecturer on June 2? | 3.6/4 (90%) | 3.5/4 (88%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt | 10032ms |
| q08 | What was the title and approximate length of the book Ruth Rasul gave to Reza, according to the June 2 standup? | 3/3 (100%) | 1.5/3 (50%) | 2/2 | Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt | 7981ms |
| q09 | In the June 2 standup, what programming/language background did Chris Mayfield report having? | 1.3/2 (67%) | 2/2 (100%) | 2/2 | Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt | 8907ms |
| q10 | According to the June 17 standup, what is Chris Mayfield's academic and professional background? | 2.2/4 (54%) | 0.7/4 (17%) | 0/2 | Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt | 11746ms |
| q11 | According to the June 17 standup, what is Aman Avinash studying and at what university? | 1.5/4 (38%) | 2.4/4 (61%) | 1/2 | Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt | 11519ms |
| q12 | In the June 17 standup, what methodological question did Annika raise to Aman about varying document lengths in their test set, and what concern motivated it? | 2.4/3 (79%) | 1.4/3 (48%) | 2/2 | Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt | 11409ms |
| q13 | What personal document did Chris Mayfield use as his test set/corpus for his code, according to the June 15 standup? | 1.4/2 (72%) | 1.1/2 (57%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, [Graph: Cloud Code], Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt | 8959ms |
| q14 | What soccer/football tournament topic came up in casual conversation during the June 15 standup, and what specific rule enforcement did Reza comment on? | 1.7/3 (58%) | 1.2/3 (38%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt | 11741ms |
| q15 | What was Reza's explanation in the May 26 standup for why current human memory recall is described as "compressed"? | 2/2 (100%) | 2/2 (100%) | 2/2 | Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt | 12671ms |
| q16 | According to the May 26 standup, what was the first vanilla RAG tool the interns had been using before moving into the experimentation phase, and what week of the program was this? | 2.4/3 (81%) | 1.3/3 (44%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt | 9188ms |
| q17 | In the May 26 standup, what specific query example did Reza use to illustrate how a user query flows through a RAG system, and who did he attribute it to? | 3.5/4 (88%) | 3.2/4 (80%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-09.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-10.vtt | 8882ms |
| q18 | What did Annika ask Reza regarding installation requirements for Lucid in the May 26 standup, and what was Reza's answer? | 2.9/4 (73%) | 3.1/4 (77%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-05-28.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-22.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt | 11937ms |
| q19 | According to the June 17 standup, why does Chris Mayfield say many of his undergraduate courses didn't transfer toward his master's degree? | 1.3/2 (65%) | 1.2/2 (58%) | 1/2 | Intern Standup - Audio Transcript - 2026-06-04.vtt, Intern Standup - Audio Transcript - 2026-05-27.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt, Intern Standup - Audio Transcript - 2026-06-02.vtt | 8957ms |
| q20 | In the June 17 standup, what statistical/measurement theory did Chris Mayfield bring up in conversation, and what example did he use to explain the difference between interval and ratio data? | 1.8/3 (60%) | 2.3/3 (75%) | 2/2 | Intern Standup - Audio Transcript - 2026-06-11.vtt, Intern Standup - Audio Transcript - 2026-06-03.vtt, Intern Standup - Audio Transcript - 2026-06-01.vtt, Intern Standup - Audio Transcript - 2026-06-15.vtt, Intern Standup - Audio Transcript - 2026-05-26.vtt, Intern Standup - Audio Transcript - 2026-06-08.vtt, Intern Standup - Audio Transcript - 2026-06-16.vtt, Intern Standup - Audio Transcript - 2026-06-17.vtt | 9025ms |

## Answers

### q01 — What drawing tool did Reza introduce to the interns in the May 26 standup, and what was it used for?

According to excerpt [19], Reza introduced Lucid, a drawing package, to the interns. He explained that Lucid is great for drawing diagrams of all sorts. In excerpt [1], Reza mentioned that he likes using this drawing tool, which he identified as Lucid, and has used it a lot. 

The provided sources do not contain information about what specific task or project the interns were working on with Lucid in the May 26 standup, but they do indicate its general purpose and use for creating diagrams of various types.

### q02 — According to Reza's analogy in the May 26 standup, what is "the missing piece" in current AI systems compared to the human brain?

The provided sources do not contain a clear definition of "the missing piece." However, according to [1], it could be related to dreaming or the ability for AI to dream like humans do. 

In [3], Reza mentions that they haven't found the analogy between current AI systems and human brain structures.

In [5], Reza mentions an analogy but doesn't specify what it is.

The term "the missing piece" appears in [1] as a standalone statement, but no further information is provided to describe what this piece is.

### q03 — Who is Mitch Travers, and what phrase did Reza attribute to him regarding Kwai?

Mitch Travers was a speaker at a conference where Reza attended (though he had some audio issues with the AV equipment). Reza thought that Kwaite came from the conference floor. 

In another instance, it is mentioned that Mitch Travers is supposed to be a guest lecturer on Thursday, but Reza needs to confirm this with him first.

As for what phrase Reza attributed to Mitch Travers regarding Kwai, I couldn't find any specific quote or statement made by Mitch Travers in the provided transcripts.

### q04 — What personal event did Reza mention happened on his birthday, and where did he go?

According to [1], Reza mentioned that on his birthday, he attended a conference in Cape Town or Dutch East Indies (which is now Indonesia). 

In excerpt [7] it's specified:

"Friday, and for my birthday, I attended a conference down in San Diego..."

However the original mention of this event was in [1] where Reza mentioned both locations: "Dutch East Indies, or Cape Town".

The sources don't provide much detail about the specific personal event that happened on his birthday.

### q05 — Who is Solomon Satari, and what was Reza doing with him over the prior weekend according to the May 26 standup?

Unfortunately, I don't have any information on who Solomon Satari is. However, based on the transcript of the May 26 stand-up meeting provided, it appears that Solomon Satari may be involved in some capacity with Reza Rassool and possibly others, as Reza mentions his name in a group context related to a paper and an intern team. However, I don't have any further details on what specific activities or projects they were working on together.

### q06 — Who is "Doc Searles" as referenced in the May 26 standup, and how does Reza describe his role?

There is no mention of "Doc Searles" in any of the provided transcriptions. The name "Doc Searles" may be from a different transcript or conversation that was not included here.

### q07 — Who is Professor Ruth Rasul, what is her academic background, and what topic did she cover as a guest lecturer on June 2?

Based on the provided sources:

1. Professor Ruth Rasul is mentioned in various excerpts as a professor of English who will be giving a guest lecture.
2. In [3], it is stated that "Ruth Rasul... come at this… the stuff that you're doing, I come at it slightly differently."
3. In [4], Reza Rassool introduces Ruth Rasul as his wife and notes that she is an English professor who will give an English 101 lesson on how to read with comprehension.
4. In [20], it is mentioned that Professor Ruth Rasul is the guest lecturer, and in [19] (but not explicitly relevant), Maira Khwaja asks whether it's the Google Drive or info page where notes should be added.

There is no explicit mention of her academic background or specific topic covered on June 2. However, as a professor of English, she might cover topics related to reading comprehension, literature, and critical thinking.

### q08 — What was the title and approximate length of the book Ruth Rasul gave to Reza, according to the June 2 standup?

According to the transcript from June 2 (specifically from [20]), the title of the book is "How Do You Sleep?" The approximate length of the book is not mentioned.

### q09 — In the June 2 standup, what programming/language background did Chris Mayfield report having?

Based on the transcript, in the June 2 standup, Chris Mayfield reported having a "sort of, like, hand-wavy" understanding of algorithms and their time complexity. However, he later mentioned that he had experience with R and statistics classes, but his work was mostly based on simple one-line scripts.

In another part of the transcript (Standup #16), Chris Mayfield mentioned working on a project related to Kaggle competitions, which implies familiarity with competitive programming and possibly languages such as Python or R.

### q10 — According to the June 17 standup, what is Chris Mayfield's academic and professional background?

The provided sources do not contain information about Chris Mayfield's academic and professional background.

However, [13] mentions that Jaymes Davis is a Chief Technical Evangelist for Chasm Technologies, which was created out of the Department of Defense on Project Maven. 

[12] also states that Jaymes Davis' name is James Davis.

### q11 — According to the June 17 standup, what is Aman Avinash studying and at what university?

There is no information in the provided transcript about Aman Avinash's studies or the university he attends. The transcript mentions Arman and his tasks, but it does not mention Aman Avinash explicitly.

### q12 — In the June 17 standup, what methodological question did Annika raise to Aman about varying document lengths in their test set, and what concern motivated it?

The provided sources do not contain any information about a methodological question related to varying document lengths in the June 17 standup.

### q13 — What personal document did Chris Mayfield use as his test set/corpus for his code, according to the June 15 standup?

According to the transcript, Reza Rassool mentioned that his "code or my test set was this, biography of my, or memoir my dad wrote" at timestamp 50:10.12-22.23. However, Chris Mayfield's personal document used as a test set/corpus is not explicitly mentioned in the provided transcript.

However, based on the subsequent conversation between Reza Rassool and Chris Mayfield (timestamps 51:00-51:05), it appears that Chris Mayfield mentioned using his dad's memoir or biography as his test set.

### q14 — What soccer/football tournament topic came up in casual conversation during the June 15 standup, and what specific rule enforcement did Reza comment on?

I couldn't find any information about a soccer or football tournament being discussed in the provided excerpts.

### q15 — What was Reza's explanation in the May 26 standup for why current human memory recall is described as "compressed"?

The provided sources do not contain information about a specific "May 26 standup" or Reza Rassool explaining why current human memory recall is described as "compressed". However, in [8] and [9], it is mentioned that when humans recall memories, they don't recall them at the same resolution that they came in. This is compared to a compressed form, suggesting that human memory recall is indeed described as being "compressed" due to this process of recalling memories in a shorter and more condensed version.

### q16 — According to the May 26 standup, what was the first vanilla RAG tool the interns had been using before moving into the experimentation phase, and what week of the program was this?

According to [2] (00:09:43.130 --> 00:09:54.930), the first vanilla RAG tool that the interns had been using was Open Web UI, which they used for three weeks before moving into the experimentation phase. This occurred during week of…, although we don't know which specific week it was.

According to [20] (23: 00:03:25.080 --> 00:03:33.549), at this point in the program, the interns were already in week four and had been using vanilla RAG through Open Web UI for three weeks before moving into experimentation.

### q17 — In the May 26 standup, what specific query example did Reza use to illustrate how a user query flows through a RAG system, and who did he attribute it to?

The specific query example that Reza used in the May 26 standup to illustrate how a user query flows through a RAG system was not provided. However, according to the transcript, on May 30, Reza gave an example of a user querying their own memory and the process that goes on.

On May 29, it is mentioned that Reza had previously illustrated this with an example involving someone querying what a "one-shot prompt" was.

### q18 — What did Annika ask Reza regarding installation requirements for Lucid in the May 26 standup, and what was Reza's answer?

I cannot verify what Annika asked Reza regarding the installation requirements of Lucid on May 26. Is there anything else I can help you with?

### q19 — According to the June 17 standup, why does Chris Mayfield say many of his undergraduate courses didn't transfer toward his master's degree?

According to the transcript, Chris Mayfield says that many of his undergraduate courses didn't transfer toward his master's degree because he spent a semester abroad in Switzerland and later in China. He mentions that the transfer process was "kind of similar" to a situation where you have to know whether you want a specific seat or not, as described by Annika earlier. This implies that there might be some issue with getting credits transferred between institutions or countries.

### q20 — In the June 17 standup, what statistical/measurement theory did Chris Mayfield bring up in conversation, and what example did he use to explain the difference between interval and ratio data?

According to the transcript of the June 17 standup, Chris Mayfield brought up Item Response Theory (IRT) as a statistical/measurement theory. He used the example of pain measurement on a scale from 1 to 5 to explain the difference between interval and ratio data. 

In this context, he stated that while the numbers represent different levels of pain, they are not necessarily equivalent in magnitude. This is because people respond differently depending on their perception of pain, illustrating how IRT can help bridge the gap in consistent psychometric measurement.

