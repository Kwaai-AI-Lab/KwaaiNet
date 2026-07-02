# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 72.2% (151.0/209) |
| Avg latency | 30688ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 18341ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 23465ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21815ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22346ms |
| q05 | Who was J.M.H. Gool? | 7/8 (88%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 32082ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf | 26450ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 22723ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 22304ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 25917ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | LEST WE FORGET -rev25.pdf | 28437ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/4 (75%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 27230ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 29386ms |
| q13 | What was the All Africa Convention? | 2/5 (40%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 20975ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf | 18729ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six], sequence_diagram:District Six | 33188ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/5 (80%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 29945ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 21213ms |
| q18 | What was the New Era Fellowship? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 22919ms |
| q19 | What was the Non-European Unity Movement? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23707ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 25669ms |
| q21 | Who was the author's mother? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 29987ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 31362ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 37028ms |
| q24 | Who were the children of J.M.H. Gool? | 6/7 (86%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 52815ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 48710ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 39027ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 49978ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 52806ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 50241ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool, LEST WE FORGET -rev25.pdf | 61851ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 35333ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 26048ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 27730ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 31441ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 30687ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/4 (50%) | [Graph: Cape Anti-CAD], LEST WE FORGET -rev25.pdf | 24671ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 4/7 (57%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 27020ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 22904ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 27555ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 23490ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources do not contain a comprehensive list of the author's children. However, [12] mentions that the dedication is for "For my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." 

[17] is identical to [12], so it does not provide any additional information.

The author's children are:
1. Feyruz
2. Reza
3. Zarina

Their grandchildren include six individuals whose first names are listed, but their last name (Rassool) is mentioned in the context of the book's author.

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool [11], [19]

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [18]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool, also known as Haji Joosub Maulvi Hamid Gool, was a prominent figure in Cape Town, South Africa during the late 19th and early 20th centuries. Here are some specific facts about him:

* He was a spice merchant and community leader (Source [1]).
* He arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India (Sources [1], [18]).
* He had two wives: Bibi Gool and Wahida Ta'al. With Bibi, he had Ayesha Rassool (mother of the memoir's author) and possibly other children. With Wahida, he had Abdul Hamid Gool and possibly other children (Source [3] and implied in Sources [9], [14]).
* He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six (Source [1]).
* He met with Mahatma Gandhi during his 1914 visit to Cape Town, and Gandhi even helped furbish Gool's surgery by puttying and staining the floor (Sources [8], [9] and implied in Source [14]).
* He was involved in various community activities and was respected as a public figure (Sources [1], [5], [7], [11], [13]).

Note that some of these facts are mentioned multiple times across different sources, but I have only listed each fact once above.

### q06 — Tell me about Buitencingle.

Buitencingle was an affluent neighborhood in Cape Town, South Africa, where the author's grandfather lived in a 13-roomed mansion at No. 7 Buitencingle Street [4]. The house was considered one of the grandest homes in the area and had umpteen rooms [14].

According to the author, Buitencingle was a vibrant and lively place with a sense of grandeur and joie-de-vivre [9]. It was a neighborhood where people would often socialize, and the author's grandfather was known for his impressive parties and gatherings.

However, as time went by, the neighborhood began to decline. The author mentions that when they arrived at Buitencingle, it looked like a "dead shell" with many rooms empty and unused [11]. The once-grand house was now abandoned, and the author's family had to search for the grandfather's treasured items, such as his hammer and tins of nails [11].

Buitencingle Street itself was described as wide and cobbled with traffic islands in the middle, which sported a few stunted palm trees. The German Lutheran Church stood opposite it, and on Sunday mornings, the street would come alive with the sound of bells [20].

### q07 — Who is the author's wife?

The provided sources do not directly state the name of Yousuf (Joe) Rassool's wife. However, in excerpt [1], Nazima Rassool is mentioned as being married to Yousuf Rassool. In excerpts [2] and [18], it is stated that his wife, an academic researcher, urged him to ground his writing in background investigation.

Therefore, we can infer that the author's wife's name is likely Nazima Rassool, who is also mentioned as Professor Nazima Rassool or Prof. Nazima Rassool.

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about the author's wife. However, we can gather some facts from the following excerpts:

* [1] Nazima Rassool is mentioned as the author's wife in the excerpt, but no further details are given.
* [9] The author mentions that his wife is an academic researcher who urged him to ground his writing in background investigation.
* [19] The author again mentions his wife and notes that she had a significant influence on the project, encouraging him to engage in thorough research.

Unfortunately, we do not have more specific information about Nazima Rassool's background, family, or personal characteristics.

### q09 — Who was the author's grandfather?

The author's grandfather was Haji Joosub Maulvi Hamid Gool, also known as J.M.H. Gool [1]. He was born in Mauritius (via Swat, Gujarat) and arrived in Cape Town in 1884 [1]. 

Specific facts about the author's grandfather from the sources:

* Founder of the Hanaffi Quwatul Islam Mosque in District Six [1]
* Spice merchant and community leader
* Lived at 7 Buitencingle Street in District Six
* Married to Bibi Gool and Wahida Gool (had multiple wives)
* Had children including:
	+ Abdul Hamid Gool with Wahida Gool
	+ Ayesha Rassool (with Bibi) 
	+ Other children listed, but no information on their relation to Yousuf Rassool [1]
* Grandfather of the author, Yousuf (Joe) Rassool [1]
* Died in April 1940 [1]

### q10 — Tell me about Kloof Nek.

Kloof Nek is mentioned in multiple excerpts [2], [3], [13], and [16]–[18]. Here's a list of specific facts I found:

* Located near Cape Town, South Africa (excerpts [2] and [17])
* Has a natural beauty that is considered "haunting" or "plangent" ([3] and [17])
* A popular spot for lovers, mountaineers, nature lovers, ramblers, and "Bergies" (down-and-outs) who would drink cheap wine and methylated spirits in its cavelets, culverts, and bushes ([17])
* Has a connecting saddle of land that joins Table Mountain to Lion's Head ([18])
* Home to the Glen, described as a place of haunting beauty ([3] and [18])
* Located near Buitencingle (a neighborhood or street) and Kloof Street ([16] and [20])
* Had a famous cable car that ran from there up Table Mountain ([17])

Please note that these are the only facts I found about Kloof Nek in the provided sources.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional association for teachers [1]. It opposed apartheid and collaborated with the Cape African Teachers' Association [1]. The TLSA had branches, including the Athlone Branch, which Joe Rassool chaired and was secretary of the Western Province Indian Cricket Union [11].

According to source [6], members of the TLSA included Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, and Frank Landsman.

The TLSA's motto was "Let Us Live for Our Children" [4]. The organization planned to develop strategies with the Cape African Teachers' Association to prepare for the eventuality of apartheid being imposed on Coloureds and Indians [4].

The TLSA went through changes in leadership. According to sources [5] and [13], there was a power struggle, where "the young Turks" (as they were called) took control from the old guard. This change led to new initiatives, such as the Education Journal of the Teachers' League of South Africa [8].

The TLSA also had connections with other organizations, including the Non-European Unity Movement and the Unity Movement [16].

### q12 — Who was Cissie Gool?

Cissie Gool (née Dollie) was a South African activist and politician. She was known as "a stalwart of the Anti-CAD Movement" [19]. According to sources [4] and [17], she was married to Dr. Abdurahman's son, Uncle Doctor. Sources [13] and [15] mention that Cissie Gool addressed a meeting at the Banqueting Hall in support of "influx control" and measures against street gangs, which shocked her nephew Joe Rassool [16] and friend Ben Kies [19]. She was also known for organizing a massive demonstration against residential segregation on the Grand Parade in 1937 [18].

Cissie Gool had a complex and possibly contradictory political stance. On one hand, she was involved with the banned Communist party [16], and her nephew Joe Rassool describes her as a "stalwart of the Anti-CAD Movement" [19]. On the other hand, she supported measures against "influx control" and street gangs at a meeting in 1953 [16].

Sources also mention that Cissie Gool was a member of the Unity Movement. In this context, it appears that she may have betrayed her own political ideology when she addressed a meeting supporting the government's policy on influx control.

Cissie Gool had six sisters: Lallie, Jolly, Dija, Toetie, Zohra, and Margie [20].

### q13 — What was the All Africa Convention?

[2] The All African Convention is mentioned in this source excerpt: "1954 was the year that Vic Wessels, Les Jacobs and I went to represent the New Era Fellowship at the conference of the All African Convention in Bloemfontein." 

However, there are no further details about the All Africa Convention in this or any other source excerpt.

### q14 — Where was District Six and what kind of place was it?

According to excerpt [11], District Six was a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats.

### q15 — What were the forced removals from District Six?

The provided sources do not contain explicit information about the forced removals from District Six. However, it can be inferred that Yousuf Rassool was a resident of District Six and his family's experiences are mentioned in the text.

Source [2] mentions that "Barrabhai" (Haji Joosub Maulvi Hamid Gool) settled with his Nurjehan in The Strand, about thirty miles from Cape Town, but does not mention forced removals. Source [14] implies that Yousuf Rassool and his family were aware of the area's decline before leaving it in 1956.

Source [17] mentions that by the time they departed District Six in 1956, it had become a run-down city centre. However, this source does not directly mention forced removals but suggests that the area was undergoing significant changes.

It is likely that the forced removals from District Six are mentioned in other parts of the text or in external sources related to the topic.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to [3] and [4], Mahatma Gandhi arrived in Cape Town from Johannesburg on October 3, 1906. He had come to lead deputations and lobby support to counter proposed legislation.

Gandhi's connection to the Gool family is evident throughout the sources. [3] mentions that Messrs. Yusuf Hamid Gool, Amod Gool, Lachiram, and Abdul Kadar came to greet Gandhi at the station on Wednesday, and Mr. Yusuf Hamid Gool had arranged for their meals at his place.

[5] shows a letter from J.M.H. Gool to M.K. Gandhi in 1897, which indicates that Gandhi was in contact with the Gool family before this time. [6] mentions Dr. A. Gool's medical articles published in Indian Opinion and other scholarly journals.

Gandhi's relationship with the Gool family is further highlighted by his interactions with various members of the family. For instance, [12] notes that Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion, but chose not to follow the path of social activism.

Gandhi visited Buitencingle Street, where the Gools resided, on several occasions. In 1911, he helped to furbish my uncle's surgery by puttying and staining the floor ([15]). In 1912, Gandhi was a guest at Adam Gool's residence "Noor Bagh" ([11]).

In March 1914, there was a function in honour of Mr. Gandhi before his departure from Cape Town for Phoenix, Natal. According to the Indian Opinion (March 1914), Dr. Gool was unremitting in his attendance on Mrs. Gandhi during her serious illness ([17]).

Gandhi's stay at the Gool mansion at 7 Buitencingle is mentioned in [19], which notes that it was a crucial time in the satyagraha led by Gandhi, and Elizabeth Molteno and other European women visited Gandhi at the Gool residence.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college for Coloureds in South Africa. It is described as having been established on the site of a former Whites-only Primary school, which had been closed down and its grounds left undeveloped (Excerpt [2]). The college opened in 1944 with considerable fanfare (Excerpt [3]).

The Hewat Training College trained teachers for the upper years of the primary division (Excerpts [6] & [15]), and it is mentioned that its staff was chosen to ensure that the college would not be an incubator of left-wing radical ideas, as were certain other schools like Livingston and Trafalgar High schools (Excerpt [10]).

Hewat Training College also had a cricket team, which was established by the author when he discovered that there was no existing team (Excerpt [8]).

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a political-cultural organization associated with the Unity Movement. It played an important role in the lives of individuals mentioned in the document, including Yousuf (Joe) Rassool, Hassan Bavasah, and Hassen. The NEF hosted lectures and meetings at its headquarters in Stakesby-Lewis Hostel in Canterbury Street, where discussions on various topics took place, including Bernard Shaw's "Arms and the Man" [3].

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain a detailed description of the Non-European Unity Movement (NEUM). However, they mention its significance and influence on the anti-Apartheid struggle in South Africa.

According to [1], the NEUM is "a movement that was founded by the South African Communist Party and had a founding member in the Unity Movement." It was also mentioned as being affiliated with the All African Convention ([2]).

In [8], it is stated that the founders of the NEUM, known as the "young Turks," included Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam. The movement's principles and methods of struggle are described in [9] as including non-collaboration and the boycott.

The movement's 10 Point Programme is mentioned in [10], which aimed to bring about a non-racial society and equal rights for all. Its influence on other movements, such as the African National Congress (ANC), is noted in [11].

However, it appears that the NEUM was not well-documented or widely recognized, with some individuals expressing dissatisfaction or controversy surrounding its activities ([13], [14]).

### q20 — Describe the author's involvement in cricket.

The provided sources describe the author's involvement in cricket as follows:

* The author was a keen cricketer and played for the Kismets Cricket Club [3].
* He developed close friendships with other cricketers, including Hassen Mall and Achmat Hayat (Achee) [4].
* The author mentions several times his disappointment at not being selected for the Western Province Indian Cricket Union team, despite his promising batting skills [13].
* He was thrilled when he was eventually chosen to represent the Western Province in a biennial cricket tournament [14].
* The author mentions his friendship with Hassen Mall, who had left for Durban after completing his LLB degree and was captaining the side [9] [20].
* He describes his own batting skills as "an absolute rabbit" but notes that he was able to master the elements of the forward defensive stroke and build a strong foundation in batting [15].
* The author also mentions his efforts to establish a cricket team at Hewat College, where he attended school [16].

It's worth noting that the author's involvement in cricket is not limited to playing. He also seems to have been involved in organizing cricket matches and advocating for non-racial policies within the sport. For example, he notes that the Kismets Cricket Club had transformed the Indian Cricket Union by adopting a non-racial policy [17].

### q21 — Who was the author's mother?

The provided sources do not contain explicit information about the author's mother, including her name or any other identifying details. However, [1] mentions Ayesha Rassool (also known as Lallie Gool) as the daughter of Bibi Gool and Haji Joosub Maulvi Hamid Gool. It is likely that Ayesha Rassool is the mother of the author Yousuf Rassool, but this information is not directly stated in any of the excerpts.

If we follow the family connections mentioned in [1], the author's mother would be Lallie Gool (Ayesha Rassool), and she would have been married to Peter Alexander Rassool. However, since this information is not explicitly confirmed within the provided sources, it should be treated as an inference rather than a direct fact.

To clarify, the available information is:

* Ayesha Rassool (Lallie Gool) is mentioned in [1] as the daughter of Bibi Gool and Haji Joosub Maulvi Hamid Gool.
* Peter Alexander Rassool is mentioned in [4] as having been circumcised and becoming known as Peerbhai or Peru.

Without direct confirmation, it remains uncertain whether Ayesha Rassool (Lallie Gool) was indeed the mother of Yousuf Rassool.

### q22 — Who was the author's father?

According to [1] Peter Alexander Rassool lived in District Six. He was the father of: Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf Rassool (the author), Zain Rassool, Fazil Rassool, Yasmin Rassool. His wife was Ayesha Rassool.

We also find additional information about his father in the excerpt [9] where it is mentioned that Peter Alexander Rassool thenceforth became known as Peerbhai or Peru (as my mother called him).

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool (also known as the "fretful baby" and described as having "large soulful eyes")
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool (mentioned in [19] as being around 7 years old)
7. Yasmin Rassool (also known as Jessie)
8. Lallie is mentioned as the mother of the family, but there is no specific information on her relationship to the author or whether she was a sibling.

Note: The sources do not provide an exhaustive list of all the author's siblings, and it appears that some information may be missing.

### q24 — Who were the children of J.M.H. Gool?

The sources state that J.M.H. Gool had two wives: Bibi Gool and Wahida Gool. His children with these two women are listed as follows:

With Bibi Gool:
- Ayesha Rassool (note: the author, Yousuf (Joe) Rassool, is this person's child)

With Wahida Gool:
- Abdul Hamid Gool
- Mariam Margie Gool
- Minnie Gool
- Zobeida Gool
- Goolam Gool
- Fatima Gool
- Mohamed Saaid Gool

The sources also mention that Dr. Gool had a second family, as mentioned in [3] and [8]. The children of this second family are listed in [7] as follows:

- Cissie Gool (who became the wife of Abdul Hamid Gool)
- Dija
- Lallie
- Toetie
- Zohra

Note that there might be some discrepancies or missing information regarding the total number of children and their relationships with each other.

### q25 — Who was I.B. Tabata?

I.B. Tabata is mentioned in several excerpts as a key figure in the Unity Movement. He was:

* A member of the New Era Fellowship (NEF) and later its chairman [3, 19]
* A writer who completed his book "The Awakening of a People" which was an occasion of considerable excitement due to it being an NEF publication [3, 19]
* A speaker who gave talks at the NEF on topics such as One Day in the Life of Ivan Denisovich, making a tremendous impression on the author [14]
* Someone accused by the Forum Club (a Marxist Trotskyist group) of failing to have a class analysis in his historical tract "The Awakening" and being a Black Nationalist [18]
* A figure who was involved in the Unity Movement and had a significant impact on its development
* Mentioned as having been part of the gathering that included meeting with the author, which was attended by various notable figures including Ben Kies and Victor Wessels

Specifically, Tabata's background and personal details are not extensively mentioned in these excerpts. However, based on his involvement with the Unity Movement and his writings, we can infer that he was a prominent figure in the movement, likely involved in its leadership and ideological development.

In terms of specific facts about I.B. Tabata, from these excerpts:

* He qualified as BA LLB (see [14])
* He wrote "The Awakening of a People" which was an NEF publication
* He gave talks at the NEF on topics such as One Day in the Life of Ivan Denisovich
* He was accused by the Forum Club of failing to have a class analysis in his historical tract and being a Black Nationalist

No further information is provided about Tabata's personal background, family, or earlier life.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman (1872-1947) [18] was a South African physician, politician, and activist who played a significant role in the fight against apartheid. 

He qualified as a doctor from Glasgow University in 1893 [3]. Dr. Abdurahman's mother, Gadija Dollie, was said to be a most beautiful woman [3].

In 1937, it was Cissie Gool who, almost single-handedly, rocked the Government when she called a massive demonstration of the Liberation League on the Grand Parade to rally the vote-less masses against the government’s plan to introduce residential segregation. Dr. Abdurahman's bond with my grandfather also brought them into contact with other notable individuals [4].

Dr. Abdurahman was present at a meeting at St Paul’s Mission School in January 1904, where the Progressive Party appealed for the “Coloured” vote [4]. He asked what provision would be made for the higher education of coloured people, but the mealie-mouthed reply given by the government was not reported in The Indian Opinion [4].

Mr. "Sonny" Abdurahman presided at a gathering where he called on people to “Go home now” and “use the whole train, but do it quietly” [9]. Many people moved off to the station to do exactly what he said.

Dr. Abdurahman was married to Helen “Nellie” Potter James. The Abdurahman clan and the Ta’Als were linked through the Dollie family [3].

He was brought up in a household where he was encouraged to speak out against injustice, with his parents both being strong advocates of justice and human rights. Dr. Abdurahman's father had been a prominent figure in the community, having bought his own freedom from slavery before becoming one of the leading figures in Cape Town [18].

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to the sources, Gandhi and J.M.H. Gool had a close connection through various interactions and relationships.

* [1] states that Gandhi visited the Gools' residence at 7 Buitencingle Street in Cape Town.
* [3] mentions a correspondence between Gandhi and Abdul Hamid Gool, who was J.M.H. Gool's son, while he was a medical student at Guy’s Hospital in London.
* [5] indicates that Gandhi helped bring the two rival organisations together as "The Cape British Indian Union", with J.M.H. Gool becoming one of the joint secretaries of the new body.
* [6] states that Gandhi became involved in the politics of the Indians in Cape Town, and when he helped unite the rival organisations, J.M.H. Gool became one of the joint secretaries of the new body.
* [7] mentions a meeting where J.M.H. Gool declared his support for the British Indian League and handed in his first subscription as a member.
* [10] indicates that Gandhi stayed at the Gools' residence during his visits to Cape Town, including one visit with his family in February 1914.
* [11] states that J.M.H. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion, but chose not to follow the path of social activism.

The sources suggest that J.M.H. Gool was a significant figure in Gandhi's life in South Africa, providing hospitality, support, and even medical care when needed.

### q28 — Which organisations was the author involved in?

The provided sources list the following organisations that the author, Yousuf (Joe) Rassool, was involved in:

1. Teachers League of South Africa [1]
2. Non-European Unity Movement [5]
3. New Era Fellowship [9], [16]
4. Cape Peninsula Students Union [19]

Note: The sources do not provide an exhaustive list of the author's organisational affiliations; only the ones explicitly mentioned are included above.

Additionally, the sources mention other organisations in relation to the author or his activities:

* African People’s Organisation (A.P.O.) - a response to the New Era Fellowship's call for unity [5]
* Communist Party of South Africa - a response to the New Era Fellowship's call for unity [5]
* Fourth International of South Africa - a response to the New Era Fellowship's call for unity [5]
* Liberation League - a response to the New Era Fellowship's call for unity [5]
* Workers’ Party - a response to the New Era Fellowship's call for unity [5]
* Smuts’s United Party (Coloured branch) - a response to the New Era Fellowship's call for unity [5]

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources indicate that the Teachers' League of South Africa (TLSA) eventually aligned with the Non-European Unity Movement. 

According to [12], after the TLSA split, they worked together with dedication to spread the Parent-Teachers’ Association movement throughout the Cape.

[18] mentions that the TLSA joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain a direct answer to this question. However, based on [4], we know that Grandpa had to be back in Cape Town at least nine months prior to December 1900 (the date of his arrival) since his daughter Jane was born in March 1902 and he would have had to be there before she was conceived.

From [3] we also know that the party arrived in Cape Town in December, but it does not specify who exactly arrived. However, considering that a telegram announced Bibi Gool's imminent arrival at Cape Town Docks, it can be inferred that J.M.H. Gool (Bibi's husband) was already present in Cape Town before her arrival.

Considering the information from [3] and [4], we know that:

- The party arrived in Cape Town in December.
- Bibi Gool, accompanied by two children, arrived in Cape Town with a telegram announcing her imminent arrival.
- J.M.H. Gool was already present in Cape Town before Bibi's arrival.

However, the exact date of J.M.H. Gool's arrival in Cape Town and from where is not specified directly in the provided sources.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

According to [1], the Hanaffi Quwatul Islam Mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile. Additionally, from [20], it is described as a whitewashed structure with a domed minaret, topped by a spire with a moon and star, located in Loop Street, Cape Town. The mosque was completed in 1898 and was founded by the author's grandfather, who was also a life trustee of the mosque.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was the second wife of J.M.H. Gool [4]. She was married to him, whereas Bibi Gool was his first wife.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the sources provided, the following are some of the notable historical figures that J.M.H. Gool knew personally:

1. Mahatma Gandhi (Source: [9] mentions Gool's acquaintance with Gandhi during his 1914 visit to Cape Town)
2. R. O. Dudley (Source: [2] mentions an address by Dudley at the New Era Fellowship in the early 1950s, but it does not explicitly state that Gool knew him personally)
3. Adam H.G. Mahomed (Source: [1] lists Mahomed as a financier and possibly mentor to J.M.H. Gool, and source [5] mentions their association with each other in the context of British Justice)
4. Abdurahman was a prominent figure who had connections with Gool's family, particularly his mother, Wahida Ta'Al (Source: [18])
5. Manilal (Source: Figure 37 in [6] shows Manilal hosting the Gools at Phoenix)

Note that these are not exhaustive and there may be other notable historical figures mentioned in the sources that I have missed.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was a piece of legislation that was introduced by the South African government in 1950. According to sources [2]–[4], this act aimed to divide the country into separate areas based on racial groups, with Whites living in certain areas and Non-Whites living in others.

As stated in source [3], when it came to Cape Town, the Group Areas Act played havoc with the lives of ordinary people. The government's theory was that people tend to anticipate the wishes of the authorities, which led them to believe that most South Africans would comply with the legislation without any issues.

However, as mentioned in sources [2] and [3], the act had a significant impact on District Six, a thriving cosmopolitan area in Cape Town. The Group Areas Act resulted in the dispersal of people from District Six across the barren sandhills of the Cape Flats, decimating the community and way of life.

Source [5] explains that to entice people away from District Six, new townships were erected, including places like Bonteheuwel, Hanover Park, Mannenberg, Mitchell's Plain, Vanguard Estate, and Bishop Lavis. These townships were designed for lower-income groups but had a negative consequence: the residents lost their municipal franchise because they were living in sub-economic houses.

The Group Areas Act also led to the forced removals of people from District Six, which had a profound impact on the community's identity and culture.

### q35 — Who was Hassen Mall?

Based on the provided sources, here is a comprehensive list of specific facts that describe who Hassen Mall was:

1. He had completed his LLB degree [2] and went on to study medicine in Durban [16].
2. He was a brilliant cricketer [16] and could spin a wristy leg break [16].
3. He played for the Western Province Indian Cricket Union [8].
4. He was the chairman of the team that participated in the biennial cricket tournament at Curry's Fountain in Durban [5].
5. He was known as a "velvet smooth demon" on the cricket field [13].
6. He was described as having magnetism and being very striking, although not tall [16].
7. He had a clipped brush moustache [16] and smoked a pipe.
8. He qualified with a degree of BA LLb in 1951 [16].

Hassen Mall's personal relationships mentioned in the sources include:

* His friendship with the author (Joe Rassool), which began when they met through mutual acquaintances and developed into a close relationship [19].
* His association with Muddy, Ebrahim, and their family, where he stayed while studying medicine in Cape Town [18] and [20].
* His connection to Hamid Khan, who was also involved in cricket [13].
* His involvement with the Indian Congress through his husband Dawood [1].

Hassen Mall's professional and extracurricular activities mentioned include:

* His role as chairman of the Western Province Indian Cricket Union [8] and the team that participated in the biennial cricket tournament at Curry's Fountain in Durban [5].
* His work with students, including organizing a union to facilitate the playing of cricket at High School and College level [20].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources list several political organisations that were active in the Cape Coloured community during the author's lifetime:

1. The New Era Fellowship [15]
2. The African People’s Organisation (A.P.O.) [15]
3. The Communist Party of South Africa [16]
4. The Fourth International of South Africa [15]
5. The Liberation League [15]
6. The Workers’ Party [15]
7. The Coloured branch of Smuts's United Party [15]

These organisations, along with others mentioned in the sources (e.g., The Teachers' League of South Africa), played a significant role in the community's fight against apartheid and its struggles for unity and democratic rights.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

The sources do not explicitly state who Mahatma Gandhi was. However, excerpt [1] mentions that Gandhi is today known as Mahatma Gandhi.

Excerpts [2] and [17] suggest that Gandhi acquired the title "Mahatma" (Great Spirit) in 1914 when a British Labour Party MP named Ramsey MacDonald used this term to describe him in an article published in Indian Opinion. However, excerpt [18] mentions that historians still assert that Gandhi first acquired this title during the campaign for Indian independence years after leaving South Africa.

Excerpt [14] states that Gandhi was on deputation in London in 1906 when a letter was sent by five South African students, including Gool, supporting the case of Transvaal Indians to Lord Elgin, Colonial Secretary. This suggests that Gandhi was in London at least until 1906, but it does not specify why he was there.

Excerpt [15] mentions that Gandhi visited Buitencingle in February-March 1914 and that his wife Kasturba had become very ill and was taken care of by the Gools. However, it does not provide information about why Gandhi was in South Africa.

In summary, while the sources do not explicitly state who Mahatma Gandhi was or why he was in South Africa, they suggest that he acquired the title "Mahatma" during his stay in India and that he had connections with the Gool family in South Africa.

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information about Cissie Gool's father. However, they mention Dr. Abdurahman as "Cissie and my uncle Dr." [2] and also state that Dr. Abdurahman married Helen “Nellie” Potter James in 1893 [15].

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources do not contain a comprehensive description of what District Six was like as a neighborhood before the forced removals. However, we can gather some information from various excerpts:

* In [3], it is stated that "To me those bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention." This suggests that District Six was a vibrant and lively neighborhood.
* In [16], it is mentioned that the author's Auntie Dij had a child named Shawquet, who was about as old as his sister Yasmin (later referred to as Jessie). This indicates that there were families living in District Six.
* In [17], it is stated that "It was hard to explain why the school grounds were undeveloped." However, this excerpt does not provide any information about what District Six was like before the forced removals.
* In [18], it is mentioned that a community of freed slaves from America and the West Indies thrived in District Six in Victorian times. This suggests that District Six had a diverse population.

It is also worth noting that the author, Joe Rassool, mentions in the introduction that he wants to provide a realistic picture of what life was like in District Six before its demolition. However, this information is not explicitly stated in any of the provided excerpts.

A more detailed description of what District Six was like as a neighborhood can be inferred from [4], which states that "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town..."

### q40 — What was the Unity Movement's boycott policy?

According to excerpts [2], [4], [5], [8], and [11] the Unity Movement's boycott policy involved refusing to cooperate with certain institutions or laws, such as:

* The Group Areas Act: Members of the "Movement" were instructed not to register their protest against the decision to reclassify areas, but some members ignored this injunction. ([8])
* Dummy institutions: Jadwat and Seedat, leaders of a faction within the Unity Movement, believed it was better to have their own person inside these institutions to promote their point of view rather than having a government stooge. ([2])
* The C.A.C.: Members who accepted service on the Coloured Affairs Council were boycotted in every aspect of their lives, socially, economically, and otherwise. ([11])

However, the effectiveness of this boycott policy is questioned in excerpt [8], where it is stated that "The boycott, which was so potent in paralysing the Coloured Affairs Council, was totally ineffective in this regard."

