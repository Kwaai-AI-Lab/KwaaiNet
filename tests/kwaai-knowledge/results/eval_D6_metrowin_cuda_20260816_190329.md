# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 88.5% (185.0/209) |
| Avg latency | 6097ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 25142ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 4952ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 4669ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 3666ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 6302ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 5285ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 3238ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 4618ms |
| q09 | Who was the author's grandfather? | 6/9 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 3651ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Kloof Nek] | 5192ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 7805ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 8762ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 3827ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 4677ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, sequence_diagram:District Six, [Graph: District Six] | 16984ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 7744ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 4833ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 5788ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 6570ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 6115ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 3579ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 3002ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 4251ms |
| q24 | Who were the children of J.M.H. Gool? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5889ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 6046ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 5393ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 6881ms |
| q28 | Which organisations was the author involved in? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 4997ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 4894ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3/6 (50%) | sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 6945ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 4460ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 4857ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5521ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | sequence_diagram:District Six, LEST WE FORGET -rev25.pdf, [Graph: District Six] | 6712ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 7372ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 4298ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 4371ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 3479ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 7385ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 3738ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources do not contain a comprehensive list of all the author's children. However, they mention three children specifically:

1. Feyruz [4]
2. Reza [4]
3. Zarina [4]

Additionally, there is a mention of the grandchildren in [4], but it only lists six by their first names and does not provide any information about the parents (the author's children).

A separate source mentions that Uncle Aity (Mohammed Saaid) declined to further his academic studies, expressing a wish to go into business on his own, and my grandfather consented. However, this is a detail about one of the grandchildren rather than listing all the author's children.

The answer provided above is not comprehensive because it does not list every specific fact mentioned in the sources regarding the author's children or family connections.

### q03 — Who are the author's grandchildren?

The provided sources do not contain a complete list of the author's grandchildren. However, the following are mentioned as being named in the dedication:

1. Joseph Rassool [11]
2. Samuel Rassool [11]
3. Alexander Rassool [11]
4. Leonardo Rassool [11]
5. Dylan Rassool [11]
6. Lewis Rassool [11]

It is likely that there are other grandchildren not mentioned in the sources provided, but this information is currently unavailable.

Also mentioned as being named in the dedication:

1. Feyruz Rassool (child) [2] 
2. Reza Rassool (child) [2]
3. Zarina Rassool (child) [2]

### q04 — To whom is the book dedicated?

The book is dedicated to:

1. The author's children: Feyruz, Reza, and Zarina
2. The author's grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [3]

### q05 — Who was J.M.H. Gool?

According to the sources, J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a spice merchant and community leader who lived in District Six, Cape Town. He was born in Mauritius around 1884 and had family origins in Swat via Gujarat, India.

Some specific facts about him include:

* He was the grandfather of the memoir's author, Yousuf (Joe) Rassool.
* He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He lived at 7 Buitencingle Street, where he received notable visitors including Mahatma Gandhi and other dignitaries.
* He had two wives: Bibi Gool (with children Ayesha Rassool, Mohamed Saaid Gool, Zohra) and Wahida Gool (with children Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool).
* He performed the Hajj in 1899.
* He died in April 1940.

He was also involved in various community activities and was a prominent figure in the Indian community of Cape Town during his time.

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion located at 7 Buitenkingle Street in District Six, Cape Town, South Africa. It belonged to the Gool family, and specifically, it was the residence of J.M.H. Gool, the grandfather of the memoir's author, Yousuf (Joe) Rassool.

According to the sources [1] and [20], Buitencingle was a wide and cobbled street with traffic islands sporting stunted palm trees. The mansion had a view of Table Mountain and Devil's Peak.

Buitencingle was named after its location as an outer belt or perimeter of Cape Town at that time. The name is Dutch in origin, with "cingle" referring to a belt (Source [20]).

The mansion was large enough to accommodate the Gool family, including multiple generations, and had a garden with fruit trees, which the author enjoyed playing in as a child (Source [15]).

### q07 — Who is the author's wife?

According to [1] and [16], the author's wife is Professor Nazima Rassool.

### q08 — Tell me more about the author's wife.

Based on the provided sources, here are the facts about the author's wife:

* Her name is Nazima Rassool [1].
* She is a Professor and an academic researcher [1].
* She played a significant role in shaping the memoir by urging the author to ground his personal memories in historical research [1] and [3].
* She convinced the author to do background investigation, which led to some "staggering information" being discovered [4] and [14].
* The author owes a great deal to her for lending authenticity to the story and encouraging him to do further research [19].

There is no additional information about Nazima Rassool's life, family, or personal background in the provided sources.

### q09 — Who was the author's grandfather?

The author's grandfather was Haji Joosub Maulvi Hamid Gool [1]. He arrived in Cape Town from Mauritius in 1884, and had family origins in Swat via Gujarat, India.

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town [1]. It is situated on the eastern flank of the city, joining Table Mountain to Lion's Head [17, 20]. The area is known for its beauty and was often frequented by lovers, mountaineers, nature lovers, and ramblers [5].

There are several ways to access Kloof Nek. One route begins at Fletchers & Cartwrights, a department store that sold everything from flan cakes to furniture, where a single-decker trackless tram ran from Adderley Street [19]. Another route leads up through the gardens and along Kloof Street until reaching Kloof Nek [4, 20].

The Table Mountain cable car begins its run a few hundred feet up the mountain path at that time [5].

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organisation for Coloured teachers [1]. It was affiliated with the Non-European Unity Movement [1] and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students [1].

The TLSA had conferences that were important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense [2]. These events were social occasions where teachers could meet and discuss issues related to teaching and education.

At one point, the TLSA was taken over by new leadership, including Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, Frank Landsman and others [5]. This change in leadership was seen as a significant development by the authorities, who viewed teachers as leaders in the community and were concerned about the TLSA's growing politicisation.

The TLSA also had an Education Journal where members received training in political/educational composition and had their articles published [8,18].

[1] Teachers League of South Africa [Organization]

[2] The  conferences  of  the  Teachers’  League  of  South  Africa  had  always  been  important  civic 
occasions, opened by the mayor with a civic banquet at the ratepayers’ expense.

[5] osses,  Quints,  Sonns,  and others and taking  control  of  the  Teachers’ 
League of South Africa.

[8] Those who were regarded as good writers were inducted onto the editorial board of the Education 
Journal  of  the  Teachers’  League  of  South  Africa  where  they  received  training  in  the  art  of 
political/educational composition and had their articles published in the Journal.

[18] However, to 
enter  this inner  sanctum one  needed  the  nod of the then  editor.

### q12 — Who was Cissie Gool?

According to the sources:

* Cissie Gool's full name is Zainunnissa Gool [1].
* She is also known by various other names: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, and Zainunnissa Gool [1].
* Her father was Dr. Abdulla Abdurahman, a Cape Town doctor and city councillor [1].
* She was a daughter of District Six [1] and lived there.
* She was a member of the Non-European Unity Movement [1].
* Her mother's name is not mentioned in the sources.
* Cissie Gool married Abdul Hamid Gool, and had one child named Shaheen Gool [1].
* She became a lawyer and city councillor, and worked closely with the Non-European Unity Movement and the anti-apartheid struggle [1].
* Cissie Gool died in 1963 on December 2nd [1].

Cissie Gool was also described as:
* A Cape Town city councillor, lawyer, and political activist [1]
* A speaker for the Anti-CAD Movement and a member of the Non-European Unity Movement [1]
* Closely associated with the anti-apartheid struggle [1]

The provided sources do not contain detailed information about her early life or her education.

A list of all specific facts mentioned in the sources:

1. Her full name is Zainunnissa Gool.
2. She has various other names: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, and Zainunnissa Gool.
3. Her father's name is Dr. Abdulla Abdurahman.
4. He was a Cape Town doctor and city councillor.
5. She lived in District Six.
6. She was a member of the Non-European Unity Movement.
7. Her husband's name is Abdul Hamid Gool.
8. They had one child named Shaheen Gool.
9. Cissie Gool became a lawyer and city councillor.
10. She worked closely with the Non-European Unity Movement and the anti-apartheid struggle.
11. She died in 1963 on December 2nd.

Note: The provided sources do not mention any information about her ancestry, origins, or family connections beyond what is mentioned above.

### q13 — What was the All Africa Convention?

According to source [1], The All African Convention (AAC) was a broad coalition of African political organisations seeking unity against racial oppression, which was founded in 1935 in response to the Hertzog Bills. It later aligned closely with the Non-European Unity Movement and its non-collaboration policy.

### q14 — Where was District Six and what kind of place was it?

According to [1], District Six was a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

District Six is also described in various excerpts as "a thriving cosmopolitan area" [10], "inimitable District Six" [11], and "a 'black spot' in the Apartheid scheme of things" [11]. It was a place where people from different backgrounds came together to live, work, and socialize.

The area had a rich history of political struggle against the apartheid regime, as mentioned in excerpts [8] and [10]. Despite its cultural vibrancy and community spirit, District Six was ultimately bulldozed out of existence by the apartheid government.

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information about the forced removals from District Six. However, they do mention that the period of recapturing subjugated cultural histories of Cape Town begins in 1897 and ends in 1956 (Source [1] and [5]).

This suggests that during this time, there were significant events happening in District Six, but it does not specifically address the forced removals. To answer your question accurately, I would need additional information or sources that directly mention the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi (1869-1948) was a prominent Indian independence activist who played a significant role in India's struggle for freedom from British rule. According to the sources, Gandhi had connections with the Gool family of District Six in Cape Town.

Gandhi visited the Gool family at their home at 7 Buitencingle Street in February-March 1914, before his departure from South Africa (Excerpt [3]). The Gools were known for their hospitality, and they welcomed various notable guests, including Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman.

Gandhi was also a close friend of the Gool family, particularly J.M.H. Gool's father, Yusuf Hamid Gool (Excerpt [5]). In 1906, Gandhi arrived in Cape Town with Ally from Johannesburg to lobby for Indian rights and was greeted by Yusuf Hamid Gool and other members of the Gool family.

The correspondence between Mahatma Gandhi and J.M.H. Gool (Excerpt [18]) reveals that J.M.H. Gool was a significant figure in Gandhi's life, as he was a medical student at Guy's Hospital in London and maintained contact with Gandhi while there.

Additionally, Gandhi stayed at the home of Morris and Ruth Alexander when he visited Cape Town before his departure from South Africa in 1914 (Excerpt [12]). His wife, Kasturba, had become ill, and she was cared for by the Gools.

Ralph Bunche's records also mention that one of Gandhi's sons fell in love with a Moslem sister of Dr. Goolam Gool's, further illustrating the connection between the two families (Excerpt [11]).

Overall, the sources suggest that Gandhi had strong connections with the Gool family, particularly J.M.H. Gool and his father Yusuf Hamid Gool, during his time in South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained teachers under the apartheid-era segregated education system. The author Yousuf Rassool and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

It stood opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds [4]. According to [19], it was originally a Whites only Primary school, but the authorities considered it too close to the prison and District Six. It is hard to explain why the school grounds were undeveloped [4][19].

Hewat College trained teachers for the upper years of the primary division [6][15]. The college was established in 1944 as the first Post Matriculation Teacher Training facility for Coloureds in South Africa, with considerable fanfare [20].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

The NEF also had a programme of events that was overseen by its Organising Secretary [13]. Its membership included individuals such as Hassan Bavasah, who progressed to become one of the best Organising Secretaries the NEF ever had [12].

Some notable activities and initiatives associated with the NEF include:

* Representing the Non-European Unity Movement at conferences
* Issuing a clarion call to all Non-European organisations to meet in response to an expected assault on the remaining rights of the "Coloured" community (established in 1937)
* Hosting lectures and debates on political, social, and philosophical topics
* Participating in events such as the Unity Movement rally on the Grand Parade [19]
* Being involved with other organisations, including the ANC and the Communist Party [19]

Overall, the New Era Fellowship was a hub for progressive ideas, discussion, and activism among the Coloured community in Cape Town.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a movement that was founded in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism [1]. Its programme was based on the Ten Point plan for equal rights. The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council [1].

The movement brought the principle of non-Collaboration, the refutation of “race”, and the ideal of a non-racial society to the struggle for liberation [7]. The NEUM revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon. It galvanised the non-white disenfranchised to a realisation of the need for a programme-based struggle that rejected the old tactics of deputations and appeals for justice at round table conferences [14].

The NEUM was associated with various individuals, including Victor Walter Wesley Wessels (son of Rev. Dan Wessels), who was an articulate and critical member of the class, known for his sharp brain and withering invective [5]. The movement also had connections to other organisations, such as the Teachers League of South Africa (TLSA), which was affiliated with the NEUM and shared its boycott and non-collaboration programme [1].

The NEUM's ideas and methods of struggle were believed by its members to be morally superior and would eventually triumph over apartheid. However, it faced challenges and controversies within and outside the movement, including disagreements on tactics and strategies [10].

### q20 — Describe the author's involvement in cricket.

Based on the provided sources, here is an account of the author's involvement in cricket:

* The author, Yousuf Rassool (Joe), was actively involved in cricket through the Kismets Cricket Club and non-European cricket more broadly [1].
* He played and administered club cricket in the Cape.
* In [3], it is mentioned that he called a meeting to establish a cricket team at Hewat school, which suggests his initiative and leadership skills in promoting cricket.
* The author was a batsman who scored several eighties against Isak Esmail's name in their scorebooks, indicating his participation as a player [6].
* He developed a close friendship with Edross, whose life revolved around the Barnato Cricket Board, which may imply that he was also involved with this cricket union.
* In [12], it is mentioned that Hassen Mall, who captained the side in a biennial cricket tournament, was an old friend and classmate of the author's.
* The author's cricketing skills improved over time, particularly under the guidance of his friends and teammates. He learned to master the elements of the forward defensive stroke and even scored 12 runs in one game [9].
* He also organized a union to facilitate playing cricket at High School and College levels, although it was thwarted by bureaucracy [11].

Overall, the author's involvement in cricket spanned multiple aspects, including playing, administering, leading teams, and promoting the sport.

### q21 — Who was the author's mother?

The author's mother was Yousuf (Joe) Rassool's biological mother is not mentioned by name in the provided excerpts. However, his adoptive mother is Ayesha Rassool, also known as Lallie or Lallie Gool, who is mentioned in [1].

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool [1].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool
8. Reza Rassool
9. Feyruz Rassool
10. Zarina Rassool

Additionally, the author's sister Gadija is mentioned in [12] and their brother Hanief Ally is also mentioned in [12].

### q24 — Who were the children of J.M.H. Gool?

The children of J.M.H. Gool mentioned in the sources are:

From his marriage to Bibi Gool:
Ayesha Rassool [1, 9]
Mohamed Saaid Gool [1]
Zohra Abdurahman [1, 3]

From his marriage to Wahida Gool:
Abdul Hamid Gool [1, 8]
Mariam Margie Gool [1]
Minnie Gool [1]
Goolam Gool [1]
Fatima Gool [1]
Dr. Abdul Rassool mentioned in [9] is Yousuf (Joe) Rassool's grandfather and thus a child of J.M.H. Gool through his marriage to Wahida Gool, although his name is not listed among the children in [1]. 

Note: The text also mentions that Cissie Gool was Abdul Hamid's wife/daughter-in-law [1], but this does not imply that she was a child of J.M.H. Gool.

Additionally, it is mentioned in [11] and [12] that there were children whose names are italicized in the picture, but their identities are unclear.

### q25 — Who was I.B. Tabata?

I.B. Tabata (also known as B.M. Tabata) was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool, and was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].

He was accused by The Forum Club, who regarded themselves as Marxist Trotskyists, of failing to have a class analysis in his historical tract "The Awakening" [2] and being a Black Nationalist, one of the worst forms of political denigration at the time [3]. 

Tabata became chairman of the New Era Fellowship and completed his book "The Awakening of a People", which was an NEF publication [6]. The book recorded the coming into existence of the All African Convention, which was one of the pillars of the Non-European Unity Movement [6].

He played a role in a number of events and meetings, including a meeting where Ben Kies presented on Bernard Shaw and his anti-war play "Arms and the Man", which had a great impact on those present [15]. 

The sources also mention that Tabata's differences with Ben Kies were not personal but political, and this led to a split in the Movement [12].

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman (also referred to as Abdulla Abdurahman) was a significant figure in Cape Town's history. The sources provide the following information about him:

* He was born on September 8, 1872.
* He died on February 20, 1940.
* He was a doctor and physician from Glasgow University (1893).
* He married Helen "Nellie" Potter James in Glasgow.
* He was a leader of the African Political (later People's) Organisation (APO), a pioneering figure in Cape Coloured political life.
* He was associated with Haji Joosub Maulvi Hamid Gool and the Non-European Unity Movement.
* He had children, including Cissie Gool.
* He was involved in various community activities, including collaborating with Grandpa (author's grandfather) on different projects.

Source references:

[1] - [5], [8], [16]

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources describe a close connection between Gandhi and J.M.H. Gool. They mention that:

* Gandhi stayed at J.M.H. Gool's residence at 7 Buitencingle Street in Cape Town on multiple occasions, including during his farewell visit to South Africa in 1914 (excerpts [1], [6], [12]).
* Gandhi was impressed by J.M.H. Gool and saw great potential for community service in him (excerpt [13]).
* Gandhi invested hope in the young doctor and built up his image in the community through editorials reporting his career progress (excerpt [13]).
* J.M.H. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion, but chose not to follow the path of social activism (excerpt [11]).
* Gandhi helped bring together two rival organisations as "The Cape British Indian Union" and J.M.H. Gool became one of the joint secretaries of the new body (excerpt [16]).

Additionally, the sources mention that there was a close relationship between Gandhi's family and the Gool family. For example:

* One of Gandhi's sons fell in love with one of Goulam Gool's sisters, a Muslim girl (excerpt [18]).
* Gandhi's wife, Kasturba, became ill while staying at the Gools' residence, and J.M.H. Gool took care of her (excerpt [15]).

Overall, it seems that Gandhi and J.M.H. Gool had a close personal and professional relationship, with Gandhi seeing great potential in J.M.H. Gool and valuing his contributions to the community.

### q28 — Which organisations was the author involved in?

Based on the provided sources, the author (Yousuf "Joe" Rassool) was involved in the following organisations:

1. New Era Fellowship (NEF) [3, 5, 6, 8]
2. Cape Peninsula Students Union [3]
3. Teachers League of South Africa (TLSA) [16]
4. Anti-CAD (Anti-Communist Action?) [13]

Additionally, the author mentions his involvement in various events and initiatives that were related to other organisations, such as:

1. The African People's Organisation (APO) [5]
2. The Liberation League [5]
3. The Communist Party of South Africa [5]
4. The Fourth International of South Africa [5]

Note that the author was also involved in his school's cultural society and the Hewat Amateur Theatrical Society, but these are not explicitly stated as organisations he joined or led outside of his school context.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM), sharing its boycott and non-collaboration programme. [1] They worked together to spread the Parent Teachers' Association movement throughout the Cape, which attracted parents like Messrs Oosterwyk, Keraan, Abdurahman, Mrs Ahmed, and Adams. [10]

The TLSA also participated in the Unity Movement's 10 Point Programme, including the demand for free and equal education to the age of sixteen. [7] The movement's principles, such as non-collaboration and a non-racial society, were also adopted by the TLSA when it broke with its stance of non-political professionalism and joined the NEUM. [19]

In general, the TLSA was aligned with and actively participated in the activities of the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], J.M.H. Gool arrived in The Cape from India via Mauritius in 1884.

However, this is a one-time mention of his arrival without specifying the exact month or day. But it's mentioned again in other excerpts that give more context:

* In [10], it says "He had come from India to The Cape in 1884 via Mauritius leaving behind in India his eleven-year-old bride."
* And in [15] (which is similar to [1]), it says: "His first son was born in 1886 and Gool’s testimony is recorded in Supreme Court transcripts in 1892. wn, and he for Bombay. The historical record is sparse for this period of J.M.H. His first son was born in 1886..."

So while we know he arrived in 1884 via Mauritius from India, the exact month or day isn't specified in these sources.

If you're interested in knowing when J.M.H. Gool's first wife (his eleven-year-old bride) left India, it is mentioned that "According to my cousin Nasima, her father was born in Mauritius in September" [4], but there's no direct mention of his own arrival date in this excerpt.

It's worth noting that the sequence diagram provided in source [2] mentions J.M.H. Gool arriving in Cape Town in December 1900 (but I'm assuming this is a different event mentioned elsewhere, as it doesn't seem to match the context of this question).

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque, was a mosque located on Loop Street in Cape Town. It was founded in 1898 and was one of the oldest mosques in the Cape Muslim community. [1] The mosque was central to the history of the Gool family and the Cape Muslim community in District Six.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was married to Abdul Hamid Gool, who was a son of J.M.H. Gool [9]. This makes Cissie Gool the daughter-in-law of J.M.H. Gool. 

Additionally, [12] mentions that Shaheen is the "son of Uncle Doctor and Cissie Gool", implying that Cissie Gool has married Uncle Doctor (likely Dr. Abdulla Abdurahman), but since Shaheen is described as her son, it can be inferred that Abdul Hamid was indeed the husband.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool knew Mahatma Gandhi personally. Gandhi stayed at the Gools' home in Cape Town in February 1914 and was a guest of the family.

Source: [9] mentions that E.S. Reddy provided correspondence between Gandhi and JMH Gool, indicating their personal connection.

Additionally, source [2] states that Mahatma Gandhi visited the Gools at their residence at 7 Buitencingle Street in Cape Town during his farewell visit to South Africa before returning to India in 1914.

Source: [1] mentions that Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman also visited the Gools at their home.

The provided sources do not contain a comprehensive list of all historical figures J.M.H. Gool knew personally, but these specific individuals are mentioned as having interacted with him.

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not contain a comprehensive description of the Group Areas Act. However, [11] states that the Act was "the lynch pin, the Big Bertha of the Apartheid weaponry" that aimed to transform South Africa into a patchwork of Black, Coloured, Indian, and White areas.

According to [2], the effect of the Group Areas Act on District Six was gradual but definite, with shops in Hanover Street starting to close in the early 1950s. The area's community scattered and disintegrated due to Apartheid policies, including the Group Areas Act. The Act led to a decrease in property values as prospective White buyers offered paltry sums to Coloured owners, who were forced to accept them.

[10] mentions that the Group Areas Act was an "obscene social experiment" that devastated the lives of hundreds of thousands of unfranchised people in South Africa. However, it does not specifically address how the Act affected District Six.

The Indian population in Natal, mentioned in [18], was largely concentrated there and had achieved considerable wealth despite the odds against them. However, this information is unrelated to the impact of the Group Areas Act on District Six.

In summary, while the sources do not provide a comprehensive description of the Group Areas Act, they suggest that it had a significant impact on District Six, leading to the decline and eventual destruction of the community due to Apartheid policies.

### q35 — Who was Hassen Mall?

Based on the provided excerpts [1], [2], [3], [5], [8], [9], [10], [11], [13], [14], [15], and [20], here are the facts about Hassen Mall:

* He is a close friend of the author Yousuf Rassool.
* He was from Durban, staying with his friends in Cape Town (excerpts [2] and [9]).
* He came to study medicine at Cape Town.
* He had completed his LLB degree before captaining the cricket team for an event in January (excerpt [5]).
* He is a brilliant cricketer who spun a wristy leg break that zipped like a circular saw (excerpt [8]).
* He was a good student and switched from sciences to Law, where he discovered his true talent.
* He qualified with a degree BA LLB in 1951 (excerpt [8]).
* He captained the cricket team for an event in January and later became a close friend of the author (excerpt [5] and [15]).
* He was part of the Indian Congress, as mentioned in excerpt [15].
* He was involved in helping to raise funds for the cricket tournament by collecting donations from Indian shops (excerpt [13]).
* He played cricket with the author and others in a team that notched its first victory by beating Eastern Province (excerpts [10] and [13]).
* He was bowled out first ball in an event described in excerpt [14].
* He was known for his nonchalance on the field, which hid his lack of preparation for a particular situation.
* The author considered him to be one of the top cricketers from Cape Town.

Overall, Hassen Mall was a close friend and talented cricketer who became an important part of the author's life.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century, according to [1], were:

1. The Non-European Unity Movement (NEUM)
2. The Teachers League of South Africa (TLSA)
3. The New Era Fellowship (NEF)
4. The All African Convention (AAC)

These organisations were united by a boycott and non-collaboration policy against apartheid institutions, with the NEUM serving as an umbrella body.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer on a one-year assignment for the merchant Dada Abdullah, assisting his attorney in a civil suit against a merchant in Pretoria. He stayed on for over 20 years developing satyagraha (non-violent resistance) after Natal moved to disenfranchise Indians in 1894 [1].

Gandhi was sent for because the accounts were kept in Gujarati, which was Gandhi’s mother tongue [7].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources, District Six was a vibrant and multicultural community near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

The neighbourhood had a history of political struggle and resistance against apartheid [1]. Before the forced removals, District Six was a thriving cosmopolitan area with a rich tapestry of lives, including families like the Rassools [17].

In its heyday, Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way, and had a lively atmosphere [18]. The community had a strong sense of identity and culture, with residents enjoying music, jazz, and community life [1].

The neighbourhood was also known for its complexity in terms of family relationships and ethnicity. Many families, like the Rassools, had members classified as Indian, Malay, or Coloured, which created problems under apartheid's racial classification system [9].

Overall, District Six was a unique and vibrant community that played an important role in the history of Cape Town and South Africa.

Here are some specific facts about District Six before the forced removals:

* It was a multicultural community with Coloured, African, Indian, and White residents [1].
* The neighbourhood had a rich urban culture of music, jazz, and community life [1].
* It was a thriving cosmopolitan area in the heart of Cape Town [17].
* Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way [18].
* The community had a strong sense of identity and culture [1].
* Many families, like the Rassools, had members classified as Indian, Malay, or Coloured, which created problems under apartheid's racial classification system [9].

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy involved refusing to participate in any apartheid government institutions. According to [1], "The NEUM adopted a boycott and non-collaboration policy — refusing to participate in any apartheid government institutions, including the Coloured Advisory Council." 

Additionally, when asked not to vote, people were told to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip.

