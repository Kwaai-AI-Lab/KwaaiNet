# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 90.0% (188.0/209) |
| Avg latency | 5370ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 7773ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 3494ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 3575ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 3356ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 9899ms |
| q06 | Tell me about Buitencingle. | 8/8 (100%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 6477ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 3445ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 4541ms |
| q09 | Who was the author's grandfather? | 6/9 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 3521ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 5730ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 5473ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 6441ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: All African Convention] | 4013ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 3393ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | [Graph: District Six], sequence_diagram:District Six, LEST WE FORGET -rev25.pdf | 16742ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 6476ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 3799ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 5048ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4913ms |
| q20 | Describe the author's involvement in cricket. | 4/5 (80%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 5658ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 4384ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 3154ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 3968ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 4713ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 5511ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 4953ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 6402ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 4156ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4749ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 4/6 (67%) | sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 3673ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 4817ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 5986ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 7612ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | sequence_diagram:District Six, [Graph: District Six], LEST WE FORGET -rev25.pdf | 6889ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 6759ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 4534ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 4394ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 3468ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 4517ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 6410ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1, 10].

### q02 — Who are the author's children?

According to excerpt [4], the author's children are:

1. Feyruz
2. Reza
3. Zarina

Additionally, according to excerpt [12] and again in excerpt [13], they have the following grandchildren:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

### q03 — Who are the author's grandchildren?

The provided sources do not list all of the author's grandchildren. However, we can enumerate those mentioned:

1. Joseph Rassool [16]
2. Samuel Rassool [19]
3. Alexander Rassool [19]
4. Leonardo Rassool [19]
5. Dylan Rassool [19]
6. Lewis Rassool [19]

### q04 — To whom is the book dedicated?

The book is dedicated to the author's children: Feyruz, Reza, and Zarina, as well as his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [3]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool, also known as Joosub Maulvi Hamid Gool, was a community leader and spice merchant who arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India [1]. He was married to two women: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra) and Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool) [1]. He was the grandfather of the memoir's author, Yousuf (Joe) Rassool. J.M.H. Gool lived at 7 Buitencingle Street in District Six and was a highly regarded citizen [5].

As a community leader, J.M.H. Gool was involved in various activities, including being a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six [1]. He received notable visitors such as Mahatma Gandhi, who stayed at his residence in February 1914 [8]. Gandhi saw great potential in J.M.H. Gool for community service and invested hope in him, building up his image in the community through editorials reporting his career progress [10].

J.M.H. Gool was also a medical doctor and wrote articles for the Indian Opinion, including two medical articles on "Modern Footwear" and "Tuberculosis" [17]. He continued to submit articles to other journals.

As a public figure, J.M.H. Gool attended meetings of various organizations, including the Coloured People's Vigilant Committee in 1906 [14] and the British Indian League, where he declared his support for the league and collected donations for it [14]. He also interacted with prominent figures such as Dr. Abdurahman, who was related to Wahida Gool (J.M.H.'s second wife) through marriage [3].

Specific facts about J.M.H. Gool from the provided sources include:

* Arrived in Cape Town: 1884
* Origin: Mauritius via Swat, Gujarat, India
* Occupation: Spice merchant and medical doctor
* Community leader
* Founding trustee of Hanaffi Quwatul Islam Mosque in District Six
* Received notable visitors such as Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman
* Had two wives: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra) and Wahida Gool (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool)
* Grandfather of the memoir's author, Yousuf (Joe) Rassool

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-roomed mansion located at No. 7 Buitencingle Street in District Six, Cape Town. It belonged to J.M.H. Gool, the grandfather of the author. The house had a wide and cobbled street with traffic islands sporting stunted palm trees opposite it stood the German Lutheran Church.

The author's family lived in one part of the house, while some rooms were left vacant, and others were occupied by relatives or possibly tramps. The house was spacious enough that the author had to climb fruit trees to reach the pears, and he made up fanciful stories about them for his siblings.

Buitencingle was a place where the author's family would often hold gatherings and parties, and it seemed to have a positive influence on the author's life. However, the house eventually fell into disrepair, and some rooms were used as makeshift beds by individuals, including possibly a relative or a tramp.

The author also mentions that his grandfather received an offer from Gardener Williams, who wanted to extend his premises next door to the Gool residence. 

Buitencingle was situated near Kloof Street, which led up to Kloof Nek, a connecting saddle of land joining Table Mountain to Lion's Head. Beyond it lay the foothills of Table Mountain.

The name Buitencingle is Dutch in origin, and "cingle" means belt. At that time, Buitencingle was the outermost belt of Cape Town. 

[1], [2], [3], [5], [9], [10] and [19] provide various details about Buitencingle.

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1]. She is an academic researcher who played a central role in shaping the memoir by urging the author, Yousuf Rassool, to ground his personal memories in historical research [1] and [6].

### q08 — Tell me more about the author's wife.

Based on the provided sources, here are all the specific facts mentioned about the author's wife:

1. Her name is Nazima Rassool [1].
2. She is an academic researcher [1] and a Professor [1].
3. The author owes a great deal to her for urging him to ground his writing in background investigation and making it historically documented [4, 13].
4. The author's wife insisted that he should not rely solely on his own memories but do some research to lend authenticity to the story [19].

Note: While there is limited information about Nazima Rassool in these sources, we can infer from the context that she was a significant influence on the author's writing process and helped him shape the memoir into its current form.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Haji Joosub Maulvi Hamid Gool. He arrived in Cape Town in 1884 from Mauritius, and his family origins were in Swat via Gujarat, India. [1]

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It is located near the Glen, a place of haunting beauty with views of both Table Mountain and Lion's Head [4] [17].

The area was known as Buitencingle Street in District Six [2], which referred to an outer belt of Cape Town. Beyond it lay the foothills of Table Mountain reached via Kloof Street [2]. The bus terminus for a single-decker trackless tram that ran from Adderley Street outside Fletchers & Cartwrights department store was near Kloof Nek [2].

Kloof Nek was used by lovers, mountaineers, nature lovers, ramblers, and down-and-outs known as "Bergies" for refuge in the area's cavelets, culverts, and bushes [5]. The Table Mountain cable car began its run a few hundred feet up the mountain path near Kloof Nek [5].

Kloof Street climbs from the city bowl up to Kloof Nek, beyond which lies the Glen and the Atlantic seaboard. Kloof Nek is also accessible via Buitencingle Street in District Six.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement, and its members adopted a boycott and non-collaboration policy to challenge racial discrimination in education and fight for equal opportunities for Coloured teachers and students [1].

The TLSA held conferences that were important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense. These events were also social occasions where teachers from other areas could come to the city to spend their winter vacation listening to eloquence about the nobility of the teaching profession and sometimes even to catch the eye of the president [2].

The TLSA was in the forefront of the struggle for democracy, with its members being involved in various activities, including establishing a Students' Union at Hewat [6]. The organisation also had an Education Journal where good writers were trained in political/educational composition and their articles published [8].

However, there is no information available on what happened to the TLSA or whether it continued to exist after 1952.

### q12 — Who was Cissie Gool?

Cissie Gool (full name Zainunnissa Gool) was a Cape Town city councillor, lawyer, and political activist closely associated with the Non-European Unity Movement and the anti-apartheid struggle. She was the daughter of Dr. Abdulla Abdurahman, a leading figure in the Coloured community.

Cissie's father, Dr. Abdurahman, was the first Coloured person to be elected to the Cape Town City Council [1]. Cissie herself was a prominent figure in her own right, known for her passionate oratory skills and her commitment to fighting against social, economic, and political injustice [13].

As a member of the Non-European Unity Movement, Cissie Gool played an important role in mobilizing the community against apartheid policies. She organized mass demonstrations and rallies, including a massive demonstration on the Grand Parade in 1937 to protest against residential segregation [4].

Cissie Gool's personal life is also documented in these sources. She was married to Abdul Hamid Gool, with whom she had a son named Shaheen Gool [1]. Her relationship with her brother-in-law or adopted sister Toetie is also mentioned in several excerpts [2].

The provided sources do not contain information on Cissie Gool's date of birth.

Sources:
[1] Cissie Gool [Person]
[4] 
[13]

### q13 — What was the All Africa Convention?

According to [1], The All African Convention (AAC) was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organisations seeking unity against racial oppression. It later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

Note: The exact founders of the All African Convention are not specified in the provided sources.

### q14 — Where was District Six and what kind of place was it?

District Six was a neighborhood located near the center of Cape Town, situated below Devil's Peak [1]. It was a vibrant, multicultural community that housed Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information about the forced removals from District Six. However, based on general knowledge, I can provide some context: The forced removals from District Six occurred during the apartheid era in South Africa. In 1960, the government of Prime Minister Hendrik Verwoerd decided to remove thousands of residents from the predominantly coloured community of District Six, located in Cape Town, to make way for a new "white" area. The removals were part of the Group Areas Act, which aimed to segregate communities along racial lines.

However, the exact dates and details of the forced removals are not mentioned in the provided sources.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi, also known as Mahatma Gandhi, was a significant figure in South African history. The sources describe him as a pioneer of satyagraha (non-violent resistance) who spent over 20 years in South Africa before returning to India.

The connection between Gandhi and the Gool family is one of friendship and hospitality. According to the sources [1] and [2], the Gool family, particularly Dr. J.M.H. Gool, hosted Gandhi at their residence at 7 Buitencingle Street in Cape Town on multiple occasions. Gandhi would visit the Gools' home, where he was welcomed by the family.

The correspondence between Gandhi and the Gool family is documented [4], with letters exchanged between them, especially during Gandhi's time as a medical student in London. The sources also mention that Gandhi had great potential for community service and that Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion and other scholarly journals.

The relationship between the two families is described as one of mutual respect and trust, with Gandhi having built up Dr. Gool's image in the community through editorials reporting his career progress [11].

It is also mentioned that Gandhi had visited the Gools' residence at Noor Bagh (8 Kloof Street) to receive G.H. Gokhale, a member of the Indian Viceroy's Legislative Assembly, in 1912 [9]. 

The connection between the two families was crucial in understanding the broader context of South African history and the role of the non-white struggle for equality in the land.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town, South Africa, specifically designed to train Coloured students as teachers. It operated under the apartheid-era segregated education system. The author, Yousuf Rassool, and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat.

During its operation, Hewat College trained teachers for the upper years of the primary division, which was considered more prestigious than training students for lower-class positions.

### q18 — What was the New Era Fellowship?

According to [1] and [4], the New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, serving as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community.

[2] mentions that Mr. I.B. Tabata became chairman of the NEF after completing his book "The Awakening of a People", which was to be an NEF publication.

[4] lists various organisations that responded to the call of the New Era Fellowship, including The African People’s Organisation (A.P.O.), sports’ clubs, The Moravian church, the Communist Party of South Africa, and others.

Throughout the sources, it is clear that the New Era Fellowship played an important role in promoting unity, democratic rights, and progressive ideas among the Coloured community in Cape Town.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a movement that emerged in 1943 with the goal of uniting African, Coloured, and Indian people against apartheid and colonialism. Its programme was based on the Ten Point plan for equal rights.

According to the sources [1], [7], and [10], the NEUM adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions. The movement revolutionized the political scene with its doctrine of non-collaboration and the boycott weapon, galvanizing the non-white disenfranchised to a realization of the need for a programme-based struggle.

Source [1] specifically states that the NEUM was founded in 1943 by the "young Turks", who included Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, as moving spirits.

### q20 — Describe the author's involvement in cricket.

According to the sources [1], [3], [9], and [20], the author, Yousuf (Joe) Rassool, was actively involved in cricket through the Kismets Cricket Club and non-European cricket more broadly. He mentions playing and administering club cricket in the Cape.

Source [2] mentions that Parker of the Indian Congress became General secretary of their Union which was a major strengthening to the Union.

Source [5] mentions Edross’s life revolved around the Barnato Cricket Board, but it is implied through this passage and other passages mentioning Kismets CC as his primary involvement rather than just a passing mention. The author recalls playing cricket seriously with Hassen Mall's help, learning to master the forward defensive stroke, and building the foundation of batting.

Source [10] mentions that he did not make the final team for the biennial tournament but notes that there were many good players in their non-racial Sunday league.

The author also participated in a meeting at Hewat where they tried to organize a cricket team. (source [7]) The author also proposed an idea of having a loose union, which could arrange regular fixtures.

Source [8] discusses the debate within the Western Province Indian Cricket Union regarding whether or not to play against the West Indians.

### q21 — Who was the author's mother?

The author's mother is Ayesha Rassool, also known as Lallie (or Lallie Gool). [1] 

Further details about her:
- Daughter of Bibi Gool and Haji Joosub Maulvi Hamid Gool.
- Half-sibling of Abdul Hamid Gool, Jane Gool-Tabata, Goolam Gool, Mohammed Hanief Gool.
- Mother of Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf (Joe) Rassool, Zain Rassool, Fazil Rassool, Yasmin Rassool.
- Married to Peter Alexander Rassool.

### q22 — Who was the author's father?

The author, Yousuf (Joe) Rassool, states that Peter Alexander Rassool was his father [1].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool (also referred to as "Fazil was then all of seven" in [19])
7. Yasmin Rassool 

These are the siblings mentioned in source [1].

### q24 — Who were the children of J.M.H. Gool?

According to sources [1]–[20], the children of J.M.H. Gool are as follows:

From his first wife, Bibi:
1. Abdul Hamid Gool (married to Cissie Gool)
2. Jane Gool-Tabata
3. Mariam Margie Gool
4. Minnie Gool
5. Zobeida Gool
6. Goolam Gool
7. Fatima Gool

From his second wife, Wahida:
1. Ayesha Rassool
2. Mohamed Saaid Gool
3. Mohammed Hanief Gool
4. Fatima Timmie Gool
5. Zohra Abdurahman

### q25 — Who was I.B. Tabata?

I.B. Tabata (also referred to as Mr. Tabata or Dr. Tabata) was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He was married to Jane Gool, daughter of J.M.H. Gool. He was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy.

Specific facts from the sources:

* Member of: All African Convention, Non-European Unity Movement.
* Married to: Jane Gool-Tabata
* Occupation: author [1]
* Had a book titled "The Awakening of a People" which was published by the New Era Fellowship and was an occasion of considerable excitement [3] [16]
* Was involved in the formation of SOYA (South African Organisation for Youth Abroad) as a training ground for young Africans [15]
* Had a class analysis accused of being undialectical code language that was un-Marxist by Tabata himself [17]

Note: I.B. Tabata is mentioned throughout the sources, but this list highlights some specific facts about him from the provided excerpts.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the provided sources:

* He was a Cape Town doctor and physician [1].
* He was the first Coloured person elected to the Cape Town City Council [1].
* He qualified as a doctor from Glasgow University in 1893 [6].
* He was associated with Haji Joosub Maulvi Hamid Gool and the Non-European Unity Movement [1].
* His mother, Gadija Dollie, was said to be a beautiful woman [6].
* Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [15].

Note: There is also another mention of a person named Abdullah (or Abdulla) Abdurahman in source [19], but it appears to be referring to a different individual or possibly an error.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The connection between Gandhi and J.M.H. Gool can be seen in several excerpts:

[2] mentions that Ralph Bunche's travel notes of his stay with the Gools in 1937 do not mention J.M.H. Gool, suggesting that by this time, he had possibly retired from public life.

[3] states that Gandhi corresponded with Abdul Hamid Gool (J.M.H.'s son) while he was a medical student at Guy's Hospital in London.

[8] mentions that Gandhi drafted a letter on behalf of five South African students, including A., to Lord Elgin, Colonial Secretary, supporting the case of Transvaal Indians.

[10] reports on meetings of the Coloured People’s Vigilant Committee, where J.M.H. Gool and others addressed the audience.

[13] mentions Gandhi's visit to Buitencingle in 1911 and his friendship with the Gools.

[15] states that Gandhi was impressed by Abdul Hamid Gool's community service potential and promoted him through editorials in Indian Opinion.

[16] reports on a function held for Gandhi before his departure from Cape Town, where he thanked J.M.H. Gool for his services to Mrs. Gandhi during her illness.

[17] mentions the close relationship between Gandhi and Dr. Gool, who provided medical care to Gandhi's family and wrote articles for Indian Opinion.

However, it is not explicitly stated what the nature of their connection was or how they first met.

### q28 — Which organisations was the author involved in?

The author, Joe Rassool, was involved in the following organisations:

1. The New Era Fellowship (NEF) [16]
2. The Teachers League of South Africa (TLSA) [6]
3. The All African Convention (AAC) [1]
4. The Non-European Unity Movement (NEUM) [1]

He also mentioned being involved with other organisations, such as:

1. The Anti-CAD (Anti-Coloured Affairs Department)
2. The Cape Peninsula Students Union [19]
3. The Hewat Amateur Theatrical Society
4. The Trafalgar Players

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement. They shared a boycott and non-collaboration programme [1]. 

This affiliation indicates that the TLSA supported the goals and policies of the Non-European Unity Movement, which aimed to unite African, Coloured, and Indian people against apartheid and colonialism [1].

Further evidence can be seen in [3] where it is mentioned that after the League took "the new road" by joining the Non-European Unity Movement, teachers were able to make meaningful contact with the people for the first time.

Additionally, [9] states that the author joined the Teachers' League of South Africa (TLSA) after starting his teaching career and began to work closely with the Non-European Unity Movement.

These sources indicate a strong connection between the TLSA and the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], Haji Joosub Maulvi Hamid Gool arrived in The Cape (Cape Town) from India via Mauritius in 1884.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Quwatul Islam Mosque or Loop Street Mosque, was founded in 1898 and is one of the oldest mosques in the Cape Muslim community. (Source: [1]) 

It is located on Loop Street, Cape Town. The mosque was central to the history of the Gool family and the Cape Muslim community in District Six. (Source: [1])

The author's grandfather was a founder and life trustee of the mosque. (Source: [20])

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not explicitly state how Cissie Gool was related to J.M.H. Gool. However, [1] mentions that Abdul Hamid Gool (one of the children of J.M.H. Gool and Bibi Gool) married Cissie Gool, who is also mentioned as his daughter-in-law ([1]). This implies that Cissie Gool was married to Abdul Hamid Gool, making her a daughter-in-law of J.M.H. Gool.

However, [8] states that Uncle Doctor and Cissie Gool had a son named Shaheen, which suggests that Cissie Gool may have been married to Uncle Doctor (whose first name is not mentioned), not Abdul Hamid Gool.

The correct relationship between Cissie Gool and J.M.H. Gool needs further clarification from the provided sources.

However, it seems from source [1] that  Cissie Gool was married to Abdul Hamid Gool and therefore was a daughter-in-law of Haji Joosub Maulvi Hamid Gool (J.M.H. Gool).

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources mention the following notable historical figures who interacted with or knew J.M.H. Gool personally:

1. Mahatma Gandhi [13]: Gandhi stayed at J.M.H. Gool's residence in 1914, and there is a picture of him hosted by the Gools [3].
2. Dr. Abdulla Abdurahman [17]: The Abdurahman family was related to the Dollie clan, which also connected them to J.M.H. Gool's second wife Wahida Ta'Al.
3. Cecil John Rhodes [1]: Although it is not explicitly stated that they knew each other, Gandhi and Rhodes were mentioned in the same context as visitors to J.M.H. Gool's residence.
4. George Bernard Shaw: Like Rhodes, Shaw was a visitor to J.M.H. Gool's residence, but there is no indication of a personal relationship [1].
5. Dr. Abdulla Abdurahman [17]: As mentioned earlier, the Abdurahman family had close ties with the Dollie clan and J.M.H. Gool.
6. Joseph Chamberlain: Although it is not explicitly stated that they knew each other, the context suggests that Chamberlain was a notable figure in South Africa during the time period.

It's also worth noting that J.M.H. Gool may have met or interacted with other notable figures mentioned in the text, such as Emily Hobhouse and Mrs. Gladstone (the wife of the Governor-General), but these interactions are not described as personal relationships.

The following individuals were board members of a school where J.M.H. Gool's son Abdul Hamid worked:

1. Joseph Chamberlain [11]
2. Dr. A Abdurahman [11]
3. J. M. Wilson [11]
4. Advocate Henry Sylvester Williams [11]

### q34 — What was the Group Areas Act and how did it affect District Six?

According to [1]–[21], the Group Areas Act was a law that transformed South Africa into a patchwork of Black "Coloured", Indian, and White areas. The act was passed in 1950 (source: [11]) and was considered the crowning piece of the Government legislation against Apartheid.

The Act had a significant impact on District Six. As stated in [2], the area began to die in the early fifties as shops in Hanover Street started closing, and the haemorrhaging was gradual but definite for years (source: [2]). The Group Areas Board's valuation of properties was generally below market value, making it difficult for Coloured owners to sell their properties or obtain fair compensation when they were forced to relocate (sources: [2], [6]).

The Act also led to the destruction of District Six as a vibrant community. According to [1] and [8], the period of recapturing subjugated cultural histories of Cape Town ends in 1956, marking the end of an era for the community. The Group Areas Act effectively disfranchised the residents by making them lose their municipal franchise due to living in sub-economic houses (source: [5]).

In addition, the Act forced many people out of District Six, leading to its eventual demolition and relocation of residents to other areas such as Bonteheuwel, Hanover Park, Mannenberg, Mitchell's Plain, Vanguard Estate, and Bishop Lavis (source: [5]). The Group Areas Act was a potent source of misery for the community, causing irreparable harm and loss.

### q35 — Who was Hassen Mall?

Based on the provided sources:

Hassen Mall was a close friend and fellow cricket player from District Six, Cape Town. He was also the captain of a team that included the author, Yousuf Rassool.

Specific facts about Hassen Mall include:

* He had come to Cape Town to do medicine (source [2])
* He was known for being brilliant at cricket and spinning a wristy leg break (source [14])
* He batted at number one, two, or four, depending on the situation (sources [16] and [17])
* He qualified with a degree BA LLB 44 from Durban (source [14])
* He had a clipped brush moustache and smoked a pipe in later years (source [14])
* His departure for Durban in 1951 left an indelible imprint of his personality on the author (source [14])
* He was known to have good relationships with other people, including Mrs. Allie, who provided him with table tennis and encouraged her sons to visit the author (sources [9] and [19])

Hassen Mall's role in Yousuf Rassool's life included:

* Introducing the author to cricket and helping him develop his skills (source [18])
* Being a source of inspiration and friendship for the author
* Leading a team that competed in the biennial cricket tournament at Durban's Curry's Fountain in early January, with Yousuf Rassool selected as part of the side (source [19])

Overall, Hassen Mall was an important figure in Yousuf Rassool's life and played a significant role in his development as a person.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were: 

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA)
3. The New Era Fellowship (NEF)
4. The All African Convention (AAC) [1]

These organisations shared a common policy of boycott and non-collaboration against apartheid institutions.

The Non-European Unity Movement was the umbrella body, while the Teachers League organised Coloured teachers; the New Era Fellowship was a discussion and lecture forum; and the All African Convention was a broader African political body that joined forces with the Unity Movement.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer on a one-year assignment for the merchant Dada Abdullah [1]. He stayed on for over 20 years developing satyagraha (non-violent resistance) after Natal moved to disenfranchise Indians in 1894 [1].

The author of this memoir does not provide information about Gandhi's background before his arrival in South Africa. However, according to excerpt [1], Gandhi was born Mohandas Karamchand Gandhi on October 2, 1869 [1].

### q38 — Who was Cissie Gool's father?

According to [1] and [20], Cissie Gool's (Zainunnissa Gool) father was Dr. Abdulla Abdurahman.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources [1] and [4], District Six was a vibrant, multicultural community near the centre of Cape Town. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life. Source [1] states that it was "a neighborhood" and describes its history as having "rich history of political struggle." 

In source [4], the author recalls walking through District Six with his ten-year-old eyes: "Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way."

The sources suggest that District Six was a bustling and diverse neighborhood before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

According to [1], the Non-European Unity Movement (NEUM) was founded in 1943 with a programme based on the Ten Point plan for equal rights. However, it does not explicitly state their boycott policy.

However, according to [5] and [7], the NEUM's alliance with the Communist Party disintegrated over the issue of boycotting "dummy" elections, specifically the election of Native Representatives to Parliament. The Communist Party wanted to participate in these elections, while the NEUM wanted to boycott them.

From [17], it is mentioned that during the boycott campaign of the Unity Movement, one of the board members took revenge on the group by informing them that they would no longer be able to meet at their previous location.

From [3], we know that when the Communist Party decided to put up its candidates despite the general desire among Blacks to boycott these "dummy" elections, they responded with convoluted thinking, which suggests that they had a different approach to boycotting than the NEUM.

However, from [7] and [10], it is clear that the Unity Movement's policy was to boycott the Group Areas Act. According to [10], the government worked on an age-old theory that people tended to anticipate the wishes of the authorities, but the NEUM's boycott did not work as intended because some members simply ignored the injunction to boycott.

From [1] and [4], it is mentioned that if the "Coloured" voters insisted on voting, they were asked to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip.

