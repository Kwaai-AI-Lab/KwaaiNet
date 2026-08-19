# RAG Eval Report

**KB:** `D6_native`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 0.5% (1.0/209) |
| Avg latency | 109ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 0/3 (0%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 88ms |
| q02 | Who are the author's children? | 0/3 (0%) | LEST WE FORGET -rev25.pdf | 109ms |
| q03 | Who are the author's grandchildren? | 0/6 (0%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 112ms |
| q04 | To whom is the book dedicated? | 0/4 (0%) | LEST WE FORGET -rev25.pdf | 108ms |
| q05 | Who was J.M.H. Gool? | 0/8 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 109ms |
| q06 | Tell me about Buitencingle. | 1/8 (12%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 108ms |
| q07 | Who is the author's wife? | 0/3 (0%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 109ms |
| q08 | Tell me more about the author's wife. | 0/6 (0%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 104ms |
| q09 | Who was the author's grandfather? | 0/9 (0%) | [Graph: Grandpa's right hand man], LEST WE FORGET -rev25.pdf | 109ms |
| q10 | Tell me about Kloof Nek. | 0/7 (0%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 110ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 0/4 (0%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 109ms |
| q12 | Who was Cissie Gool? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 111ms |
| q13 | What was the All Africa Convention? | 0/5 (0%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 99ms |
| q14 | Where was District Six and what kind of place was it? | 0/6 (0%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 111ms |
| q15 | What were the forced removals from District Six? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 154ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Gool family] | 103ms |
| q17 | What was Hewat Training College? | 0/5 (0%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 111ms |
| q18 | What was the New Era Fellowship? | 0/4 (0%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 110ms |
| q19 | What was the Non-European Unity Movement? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 109ms |
| q20 | Describe the author's involvement in cricket. | 0/5 (0%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 110ms |
| q21 | Who was the author's mother? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 113ms |
| q22 | Who was the author's father? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 113ms |
| q23 | Who were the author's siblings? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 99ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: J.M.H. Gool & Co.], LEST WE FORGET -rev25.pdf | 109ms |
| q25 | Who was I.B. Tabata? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 108ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 0/6 (0%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 109ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 112ms |
| q28 | Which organisations was the author involved in? | 0/3 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 99ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 111ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: J.M.H. Gool & Co.] | 114ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 113ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 109ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: J.M.H. Gool & Co.] | 113ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, sequence_diagram:District Six, [Graph: District Six] | 114ms |
| q35 | Who was Hassen Mall? | 0/4 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 108ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/4 (0%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 116ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 0/7 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 119ms |
| q38 | Who was Cissie Gool's father? | 0/5 (0%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 112ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 0/6 (0%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 109ms |
| q40 | What was the Unity Movement's boycott policy? | 0/5 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 103ms |

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

