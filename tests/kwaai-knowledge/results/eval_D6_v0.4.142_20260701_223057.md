# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 66.6% (147.8/222) |
| Avg latency | 31695ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21143ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 23672ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23560ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22138ms |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 35100ms |
| q06 | Tell me about Buitencingle. | 4/8 (50%) | LEST WE FORGET -rev25.pdf | 39231ms |
| q07 | Who is the author's wife? | 3/3 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 23305ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 23503ms |
| q09 | Who was the author's grandfather? | 8/9 (89%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28827ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 29576ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 25550ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31351ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 24203ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf | 30782ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 42066ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 33513ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 19981ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 27784ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 22453ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 48714ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 49928ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 43083ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 49496ms |
| q24 | Who were the children of J.M.H. Gool? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 51405ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 56610ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 49876ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 32706ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 25529ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 26338ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 2.8/6 (47%) | sequence_diagram:Haji Joosub Maulvi Hamid Gool, LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28880ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 28704ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29112ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 34939ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | sequence_diagram:District Six, LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 30253ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 28849ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | [Graph: Cape Times], LEST WE FORGET -rev25.pdf | 25051ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: South African Directory] | 27877ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 28072ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 23575ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 21082ms |

## Answers

### q01 — Who is the author?

The author's name is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The author, Yousuf (Joe) Rassool, has the following children:

1. Feyruz
2. Reza
3. Zarina

He also mentions having grandchildren, but does not list them all individually in every excerpt, however we can infer that they include:

4. Joseph
5. Samuel
6. Alexander
7. Leonardo
8. Dylan
9. Lewis

### q03 — Who are the author's grandchildren?

The provided sources do not contain a comprehensive list of all the author's grandchildren. However, we can identify the following grandchildren mentioned in the excerpts:

1. Joseph Rassool [3]
2. Samuel Rassool [19]
3. Alexander Rassool [3, 13]
4. Leonardo Rassool [3]
5. Dylan Rassool [3]
6. Lewis Rassool [3]

Please note that this list may not be exhaustive as the sources do not provide a complete list of all the author's grandchildren.

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina and their grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [18].

### q05 — Who was J.M.H. Gool?

J.M.H. Gool was a community leader, spice merchant, and founder of the Hanaffi Quwatul Islam Mosque in District Six. He arrived in Cape Town from Mauritius (via Swat, Gujarat) in 1884 [1]. He had two wives: Bibi Gool and Wahida Ta’al, whom he married in 1879 or 1880 and 1899 respectively [3]. He had many children with both wives, including Ayesha Rassool by Bibi and Abdul Hamid Gool by Wahida [1].

He was a founding trustee of the Hanaffi Quwatul Islam Mosque [1] and lived in District Six at 7 Buitencingle Street [1]. J.M.H. Gool was acquainted with Mahatma Gandhi during his visit to Cape Town in 1914, where he helped furbish Dr. A.H. Gool's surgery by putting up shelves and staining the floor [16].

Gool became involved in the politics of Indians in Cape Town and was one of the joint secretaries of the Cape British Indian Union in 1911 [16]. He later became president but resigned that year due to possibly seeing the futility of sectarian politics [16]. Gandhi stayed with Gool at his home in 1911, where he helped with Dr. A.H. Gool's surgery.

J.M.H. Gool was described as a well-regarded citizen and one of the most prominent people in Cape Town. He wrote two medical articles for the Indian Opinion: “Dr. A. on Modern Footwear” (Jan 13, 1912) and “Dr. Gool on Tuberculosis” (Apr 6, 1912). He was also a witness in a case in the Supreme Court transcripts in 1892 [20].

He died in April 1940 at the age of unknown but his grandson is Yousuf Rassool, who wrote this memoir.

### q06 — Tell me about Buitencingle.

Buitencingle is a location mentioned throughout the memoir "LEST WE FORGET" by Yousuf (Joe) Rassool.

From the sources, here are the facts I've gathered:

* Buitencingle was an outermost belt of Cape Town [2].
* It had a significant number of rooms, with umpteen rooms being mentioned [9].
* The area was vibrant and grand in the early 20th century, with a "joie-de-vivre" quality [12].
* However, by the time Yousuf (Joe) Rassool's grandfather Gool had passed away, Buitencingle's spirit had died, and it became a dead shell [12].
* The location is associated with the Gool family, who lived in a thirteen-roomed mansion at 7 Buitencingle Street [4].
* Gandhi visited Yousuf's uncle Dr. Gool at his surgery in Buitencingle in 1911 and helped to furbish it by puttying and staining the floor [13].
* The area was also mentioned as a place where Yousuf would take his siblings on walks, showing them fruit trees and teaching them about different types of pears [17].

These are the specific details I've been able to gather from the sources. If you'd like more general information about Buitencingle or its history, I don't have any additional knowledge to draw upon.

List of facts about Buitencingle with their source citations:

* Located at the outermost belt of Cape Town [2].
* Had umpteen rooms [9].
* Was vibrant and grand in the early 20th century [12].
* Became a dead shell after Gool's passing [12].
* Associated with the Gool family, who lived at 7 Buitencingle Street [4].
* Visited by Gandhi in 1911 [13].
* Had fruit trees [17].

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1]. She is also known as Professor Nazima Rassool or Prof. Nazima Rassool [1]. The author credits her with urging him to do research and ground his writing in background investigation, which led to the discovery of new information about their family history [18] [19].

### q08 — Tell me more about the author's wife.

The provided sources do not contain a comprehensive list of facts about the author's wife, Nazima Rassool [1]. However, we can gather some information from other excerpts:

* The author mentions that his wife is an academic researcher [14] and [19].
* She urged him to ground his writing in background investigation to lend authenticity to the story [14] and [19].
* He expresses gratitude towards her for suggesting this approach and credits her with producing "staggering information" through their research together [14] and [19].

We do not have access to more detailed information about Nazima Rassool's background, profession, family, or other biographical facts.

It is worth noting that the author himself does not provide extensive details about his wife in this text. If you need further information, it may be necessary to consult additional sources or interviews beyond these excerpts.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Joosub Maulvi Hamid Gool (Haji). He is described in excerpt [1] and throughout the other excerpts.

Specific facts about him include:

* He arrived in Cape Town in 1884 from Mauritius.
* His family origins are in Swat via Gujarat, India.
* He was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He had two wives: Bibi Gool and Wahida Gool. 
* He performed the Hajj (pilgrimage) to Mecca in 1899, accompanied by his wife Wahida and their children.
* He died on April 1, 1940.

Other information about him is scattered throughout the excerpts:

* Excerpt [2] mentions him giving pennies to children who came to give Eid greetings at his house.
* Excerpt [3] shows a photo of him (and others) with Mahatma Gandhi in Phoenix.
* Excerpt [4] lists him as an attendee at a meeting of the Coloured People's Vigilant Committee on May 3, 1906.
* Excerpt [9] mentions his "magnetic personality" and respect from the community.
* Excerpt [12] describes him performing the Hajj and mentions that Wahida was pregnant with Timmie during this time.

Overall, J.M.H. Gool seems to have been a respected figure in Cape Town's Indian community, known for his business acumen, community involvement, and strong personality.

### q10 — Tell me about Kloof Nek.

According to the sources, Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion's Head. It was a popular spot for lovers, mountaineers, nature lovers, and ramblers (Excerpt [17]). The area had cavelets, culverts, and bushes where people could take refuge, including down-and-outs known as "Bergies" who would drink cheap wine and methylated spirits there.

Kloof Nek was also the location of a famous Table Mountain cable car that began its run a few hundred feet up the mountain path (Excerpt [17]).

The Glen in Kloof Nek was described as a place of haunting beauty, where the author and his friend read Gray's Elegy and Milton's Lycidas (Excerpts [3], [4], and [17]). The Glen seemed entirely appropriate for the plangent music of the Elegy and the pastoral scenes of Lycidas.

In Excerpt [19], it is mentioned that Persotem and Khadija used to meet at the South African Public Library in the Gardens on some Saturdays, where they would go ostensibly to study. Afterwards, they would take a bus to Kloof Nek.

Overall, Kloof Nek appears to be a scenic area with natural beauty and historical significance, popular among locals and visitors alike.

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources describe the Teachers' League of South Africa (TLSA) as a professional association [1] and a teachers' union that opposed apartheid. It had conferences, an Education Journal, and its members received training in political/educational composition and had their articles published in the Journal [2, 8].

Some notable members mentioned are Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, and Frank Landsman [5]. The TLSA collaborated with the Cape African Teachers' Association (CATA) [17] and was part of the struggle for full democratic rights for non-citizens in South Africa [6].

The TLSA was also associated with the Non-European Unity Movement [13], which implies a connection to anti-apartheid activism. Its motto is given as "Let Us Live for our Children" [17].

### q12 — Who was Cissie Gool?

Cissie Gool was a prominent figure in South African politics. She was known for her activism and leadership roles within various organizations.

* Cissie Gool was the wife of Dr. Abdurahman, but later married to Dr. A.H. Gool (Source [7]).
* She was involved with the Anti-CAD Movement and had previously been a stalwart of the Unity Movement (Source [20]).
* In 1937, she organized a massive demonstration against residential segregation in Cape Town (Source [18]).
* According to Source [16], Cissie Gool addressed a meeting at the Banqueting Hall, supporting measures to deal with 'influx control' and street gangs.
* She had also supported the Torch Commando in 1953 and was associated with the banned Communist party, possibly due to her relationship with its members (Source [16]).
* Cissie Gool was known for her passionate speeches and rallies, which often stirred the crowds. For example, Source [18] describes a meeting she organized where she "roused the crowd with her passionate oration."
* In personal life, Cissie Gool had multiple children: Ayesha "Lallie" Gool, Fatima "Timmie" Gool, Zohra "Jolly" Gool, Mariam "Margie" Gool, and others (Source [20]).

Please note that the sources provide information about Cissie Gool in a fragmented manner. Some details are mentioned across multiple excerpts, while other facts might be missing.

### q13 — What was the All Africa Convention?

The All African Convention is mentioned in sources [2], [4], and [19]. According to these sources:

* In source [2], the author mentions attending a conference of the All African Convention with Vic Wessels, Les Jacobs, in 1954. They stayed overnight at the home of Dr. Murison.
* Source [4] lists the All African Convention under the category "Organisation".
* Source [19] provides more information about the convention: it was attended by leaders such as Tsotsi, Honono, and Sihlali from various regions, including Natal.

There is no further information provided in these sources about the specific goals, activities, or affiliations of the All African Convention.

### q14 — Where was District Six and what kind of place was it?

According to excerpts [1], [3], and [9], District Six was a thriving cosmopolitan area in the heart of Cape Town. It is described as having "more life, more zest, more fire, more warmth and care than many other places one could mention" ([3]). 

Excerpt [11] notes that it was an "inimitable" place, which would soon be bulldozed out of existence due to the Group Areas Act (1950) and Apartheid policies.

### q15 — What were the forced removals from District Six?

The provided sources do not contain direct information on what the forced removals from District Six were. However, it is stated in [13] that "It was around this time that it seemed our family was at last beginning to show signs of affluence" and in [16] that "It was around this time that it seemed our family was at last beginning to show signs of affluence." 

However, we know from general knowledge that the forced removals were part of a larger process of apartheid-era urban planning.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi was a prominent figure, but according to these sources [3], [4], [6], [7], [10], [11], [12], [15], [16], [17], and [19], here are some key facts about him:

* Gandhi had a close relationship with the Gool family, particularly Dr. A. Gool (also known as Abdul Hamid Gool) and his brother J.M.H. Gool.
* He visited their residence at 7 Buitencingle Street in Cape Town on multiple occasions, including in 1911 when he helped to furbish Dr. Gool's surgery [15].
* Gandhi corresponded with J.M.H. Gool, as seen in a letter from January 23rd, 1897 [17].
* The Gool family was instrumental in providing hospitality to Gandhis during their stay at the residence [19].
* Gandhi often stayed with the Gools when he was in Cape Town, and they were close friends and allies in his political endeavors.
* Gandhi's connection to the Gool family also extended to his relationships with other members of the community, including Dr. A. Gool who became one of the joint secretaries of the Cape British Indian Union [15].

Gandhi's occupation is not mentioned explicitly in these sources, but based on the context it appears that he was a public figure and leader involved in the Indian independence movement.

It's worth noting that Gandhi's connections to the Gool family are highlighted throughout the text as an important aspect of his life and work.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training facility for Coloureds in South Africa. It opened in 1944 with considerable fanfare, and the author mentions that it was the first Post Matriculation Teacher Training facility for Coloureds in the country [3]. The college trained teachers for the upper years of the primary division, but when the author attended, they were assigned to teach a class of 3rd year primary school pupils, which was below their expected level of training [15].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) is mentioned throughout the document as a political-cultural organization associated with the Unity Movement. According to [2], it "struck a chord that resonated in the Coloured community" and received an overwhelming response from various organizations, including:

* The African People's Organisation (A.P.O.)
* Sports' clubs
* The Moravian church
* The Communist Party of South Africa
* The Fourth International of South Africa
* The Liberation League
* The Workers' Party
* The Coloured branch of Smuts's United Party

[3] mentions that the author was asked to join the NEF by Victor and Beimel, who were already members. [4] notes that the author and his cousin Nina were involved in the establishment of a Students' Union, which was seen as a development by the NEF.

The NEF is described as an organization that viewed itself as "too advanced" for African youth and had a program of lectures (mentioned in [6]). It also seems to have been involved in various community activities, such as hosting meetings and events ([15]).

[20] mentions that Hassan Bavasah was the Organising Secretary of the NEF and that the author saw him almost on a daily basis.

### q19 — What was the Non-European Unity Movement?

The provided sources do not provide a comprehensive description of the Non-European Unity Movement (NEUM). However, they do offer some information:

* [1] Refers to the movement as affiliated with the All African Convention.
* [8] Lists Aunt Jane and her brother, Dr. Goolam, among the "young Turks" who were moving spirits in the NEUM, indicating it was formed in 1943.
* [11] Mentions that leaders of the All African Convention attended a conference of the Unity Movement, suggesting a connection between the two organizations.

From these excerpts, we can infer that the Non-European Unity Movement (NEUM) was an organization active around 1943, affiliated with or connected to the All African Convention.

### q20 — Describe the author's involvement in cricket.

Based on the provided sources [1]–[20], here is a comprehensive description of the author's involvement in cricket:

* The author was enthusiastic about cricket and considered it his passion (Source: [2]).
* He was selected to play for the Western Province Indian Cricket Union team, but unfortunately, he did not make the final team that went on tour to Johannesburg (Source: [12]).
* At one point, he scored 12 runs and was hailed as a hero, despite his team losing (Source: [13]).
* The author learned to play cricket seriously with the help of Hassen Mall, who became his close friend (Source: [7]).
* He attempted to organize a union to facilitate playing cricket at High School and College level but was thwarted by bureaucracy (Source: [10]).
* As a member of the Kismets Cricket Club, he played alongside other notable cricketers, including Muddy Allie, Hassen Mall, Ebrahim Moosa, S.Abed, and C. Khatieb (Source: [3] and [13]).
* The author's cricketing career was influenced by his interactions with various individuals, including Edross, who was the permanent secretary of the Barnato Cricket Board, and Achmat Hayat, a close friend (Sources: [4] and [5]).

In summary, the author was an avid cricket enthusiast who played for the Western Province Indian Cricket Union team, was part of the Kismets Cricket Club, and attempted to organize a union to promote cricket at High School and College level.

### q21 — Who was the author's mother?

The provided sources do not explicitly mention the name of the author's mother. However, they describe her as being dark-haired and sad-eyed, born on December 10, 1900 (source [11]). She is referred to as "my mother" throughout the excerpts.

To identify the author's mother, we need more information or context that directly links one of the women mentioned in the sources to the author. Since the author's name is Joe Rassool, and his wife is mentioned as Peter Alexander Rassool (source [18]), it can be inferred that Joe Rassool's parents are Malick Rassool and Ayesha 'Lallie' Gool (source [1]). Therefore, Lallie Gool or Ayesha 'Lallie' Gool is the author's mother.

### q22 — Who was the author's father?

The provided sources do not contain explicit information about the author's father. However, based on [9], it is mentioned that Peter Alexander Rassool (the author) became known as Peerbhai or Peru after his circumcision, and his parents are named Ayesha Rassool (mother) and Malick Rassool (father).

### q23 — Who were the author's siblings?

The provided sources list the following siblings for Yousuf (Joe) Rassool:

1. Abdul Rassool
2. Berina Rassool
3. Fazil Rassool
4. Nasim Rassool
5. Rasheda Rassool
6. Yasmin Rassool 
7. Zain Rassool

### q24 — Who were the children of J.M.H. Gool?

The following are the children listed in [1] as grandchildren, however, the text also mentions "children" which implies they may have had more than just the listed grandkids:

* Rasheda Rassool
* Abdul Rassool
* Berina Rassool
* Nasim Rassool
* Yousuf Rassool (the author)
* Zain Rassool
* Fazil Rassool
* Yasmin Rassool

Additionally, the following are listed in [10] as children of J.M.H. Gool:

* Abdul Hamid Gool
* Jane Gool-Tabata
* Mariam Margie Gool
* Minnie Gool
* Zobeida Gool
* Goolam Gool
* Fatima Gool
* Mariam Gool
* Mohamed Saaid Gool
* Ayesha Rassool
* Mohammed Hanief Gool
* Fatima Timmie Gool
* Zohra Abdurahman

### q25 — Who was I.B. Tabata?

I.B. Tabata [3] was chairman of the New Era Fellowship (NEF) when he completed his book "The Awakening of a People" [19]. The same source [3] also mentions that Mr. I.B. Tabata gave a talk at the NEF on One Day in the Life of Ivan Denisovich, which made a tremendous impression on the author.

I.B. Tabata was a key figure in the Non-European Unity Movement (NEUM) and is mentioned throughout the sources as being involved with the movement and its related activities [1], [2], [12], [13], [18]. He was also criticized by another political study group, The Forum Club, who accused him of failing to have a class analysis in his historical tract "The Awakening" [18].

Tabata's involvement with SOYA (a training ground for young Africans) is mentioned as one reason why the author, along with others from the NEF, was sent to a meeting regarding its formation [13]. 

It seems that I.B. Tabata played an important role in the political and social movements of his time, particularly within the context of South Africa's Non-European Unity Movement.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman (1880-1934) [3, 18] was a South African doctor and activist who played a significant role in the country's history.

According to the sources:

* He was the first Coloured person from Cape Town to qualify as a doctor from Glasgow University in 1893 [3].
* His mother, Gadija Dollie, was said to be a beautiful woman [3].
* He married Helen "Nellie" Potter James.
* He was linked to the Abdurahman clan and the Ta'Als through the Dollie family [3].
* He was involved in various organizations, including being a trustee of a mosque and serving on the board of Wooding's Preparatory private school [16].

Dr. Abdullah Abdurahman is mentioned as having connections with prominent figures such as Gandhi, Cissie Gool, J.M.H. Gool, and other notable individuals of his time.

Note: The provided sources do not contain a comprehensive biography of Dr. Abdullah Abdurahman, but they highlight his achievements and relationships that demonstrate his significance in South African history.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources describe the connection between Gandhi and J.M.H. Gool as a close one, with multiple interactions documented:

* In 1914, Gandhi stayed at the Gool family's residence at 7 Buitencingle Street (Excerpt [10], [13], [14], [15]).
* Gandhi was welcomed by Dr. Gool when he visited South Africa in 1911, and Dr. Gool read an address of welcome to the visiting dignitary (Excerpt [11]).
* Gandhi helped furbish Dr. Gool's surgery by puttying and staining the floor in 1911 (Excerpt [15]).
* The two were associated with various organizations together, including the British Indian League and the Cape British Indian Union (Excerpt [14], [15]).
* There is correspondence between Gandhi and Abdul Hamid Gool, Dr. Gool's son, while he was a medical student at Guy's Hospital in London (Excerpt [19]).
* The Gool family provided hospitality to Gandhis in February 1914, which led to the friendship of Joe Rassool with E.S. Reddy (Excerpt [5]).

Overall, it appears that Gandhi and J.M.H. Gool had a close personal and professional relationship, with multiple interactions over several years.

### q28 — Which organisations was the author involved in?

The provided sources do not contain a comprehensive list of all organisations the author was involved in. However, they mention the following:

1. New Era Fellowship [3], [7], [15], [17]
2. Non-European Unity Movement (refers to the call for unity and full democratic rights) [17]
3. Teachers League of South Africa [9]
4. Anti-CAD [13] 
5. The African People's Organisation (A.P.O.) [18]
6. Communist Party of South Africa [19]
7. Fourth International of South Africa [19]
8. Liberation League [19]
9. Workers' Party [19]

The author's involvement in other organisations is not explicitly stated, but they mention being a member of a school cultural society at Trafalgar High School [16].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources state that the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement (NEUM) were closely related. [12] describes the NEUM as "a movement to which I was drawn when I began my teaching career and where I threw my energies in the attempt to 'take a nation to school', an aphorism that aptly captured the role of the Movement during those years." [15] states that the TLSA gave them the opportunity to blend theory and practice, implying a close relationship between the two organizations.

[14] mentions that the author joined the TLSA as soon as they could after starting their teaching position at Habibia institute. This suggests that the TLSA was closely associated with the NEUM, which is also mentioned in [14].

[18] states that "This was the situation even after the League took 'the new road', that is, broke with its previous stance of non-political professionalism, and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society." This confirms that the TLSA did indeed join the NEUM.

Overall, it appears that the TLSA and the NEUM were closely aligned, with the TLSA being an affiliate or partner of the NEUM.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain information about when J.M.H. Gool arrived in Cape Town or where he came from, other than mentioning that his family was from Mauritius (Excerpt [3]) and that he had a wife Bibi in India (various excerpts). 

However, one of the earliest mentions of J.M.H. Gool is in Excerpt [1] which states "His first son was born in 1886".

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile [1]. It stood in Loop Street. The mosque was completed in 1898 and was used to celebrate Eid festivals [20]. My grandfather was a founder and life trustee of the mosque [20].

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was Dr. Gool's wife. [1] mentions that J.M.H. Gool had two wives: Bibi Gool and Wahida Gool (also known as Ta'Al). [4] states that Cissie Gool was the second wife, whom he married in 1880 or 1879 when she was 11 years old and he was about 16.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The following are the notable historical figures that J.M.H. Gool knew personally:

* Adam H.G. Mahomed: a financier and possibly mentor, who worked with Gool in British Justice (Source [1] & [20])
* Adam Gool (also known as Dr. Abdurahman's grandfather): was involved in the Indian community, possibly affiliated with J.M.H. Gool's clan (Source [16])
* Mahatma Gandhi: helped to furbish Gool's surgery by puttying and staining the floor in 1911 (Source [11] & [12])
* Ralph Bunche: stayed with the Gools in 1937, but J.M.H. Gool is not mentioned in his travel notes (Source [4], [17], and [18] do not confirm this interaction directly; however, it is implied that they knew each other)

The following individuals are mentioned as having connections to or interactions with the Gool family, but their relationships with J.M.H. Gool personally are less clear:

* Dr. Abdurahman: had a close residence in Loop Street and his mother was related to Wahida's (J.M.H. Gool's second wife) mother, suggesting some level of connection or familiarity between the two families
* Ben Kies, Solly Edross, Cissie Gool, Aunt Jane, Dr. Gool: appeared on a platform with other notable individuals in District Six during meetings and protests against government schemes (Source [8])

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [9] was a law that would transform South Africa into a patchwork of Black “Coloured”, Indian, and White areas [13]. It was enacted to give proper effect to the plans of the Apartheid government, which aimed to preserve White rule for the next 300 years by dividing the country into separate ethnic groups.

The Act led to the forced removals of people from their homes in District Six to other parts of Cape Town. The community of District Six was a thriving cosmopolitan area with a rich history and culture, but it was deemed "Coloured" under the Act, which meant that its residents were forcibly relocated to make way for White development.

The Group Areas Act also created a complex problem for families like the Rassools', where some members were classified as Indian and others as Malay. This led to difficulties in determining where each family member could live.

The Act had devastating effects on the lives of hundreds of thousands of people, particularly those who were unfranchised. It was a key piece of Apartheid legislation that aimed to preserve White dominance and create separate group areas for different communities.

### q35 — Who was Hassen Mall?

Hassen Mall was a significant figure in the life of the author. According to the sources:

* He was a brilliant cricketer who spun a wristy leg break that zipped like a circular saw [16].
* He had completed his LLB degree and left for Durban in 1951 [6, 20].
* He captained the cricket team in which the author played [5, 20].
* He was a student of Law at some point, where he discovered his true talent [16].
* He was known to be a kind of magnetism that could not be denied [16].
* He had a clipped brush moustache and smoked a pipe [16].

He also played an important role in the author's life outside of cricket:

* He introduced the author to table tennis, which he taught him how to play [7].
* He was instrumental in collecting funds for the cricket team by walking from one end of Hanover Street to the other with Ebrahim, collecting donations at each Indian shop [6].
* He helped the author to learn to play whist and rummy, although it turned out that the author was not very skilled at cards [14].

Hassen Mall's relationship with the author began when he stayed with the author's relatives in Cape Town while studying medicine. The author's sisters were also fond of Hassen Mall, as evidenced by his visit to their home and tea with the mother and Bibi [18].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to sources [7] and [15], some of the political organisations that were active in the Cape Coloured community during the author's lifetime included:

1. The New Era Fellowship (active from 1930s onwards)
2. The Teachers' League of South Africa
3. The African People’s Organisation (A.P.O.)
4. The Communist Party of South Africa
5. The Fourth International of South Africa
6. The Liberation League
7. The Workers’ Party

Additionally, source [18] mentions that the Coloured branch of Smuts's United Party was also active.

Source [16] mentions Gandhi's efforts to unite two rival organisations, which eventually became "The Cape British Indian Union" (though it is unclear if this organisation specifically targeted the Coloured community).

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi is today known as Mahatma Gandhi [2]. He came to South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria. The accounts were kept in Gujarati, which was Gandhi's mother tongue [6].

In South Africa, Gandhi added a demand for the abolition of the £3 poll tax in Natal for former indentured labourers and members of their families and recognition of marriages according to Indian religious rites [5]. He also became involved in various activities, including launching a scathing diatribe against the then Colonial Secretary, Joseph Chamberlain, showing an astute grasp of the Imperialist mentality [6].

Gandhi was known as "Mahatma" (Great Spirit) after being hailed as such by Dr. Gool during his farewell address to Gandhi on behalf of Port Elizabeth Indians in August 1914 [2]. This term was previously used a few months earlier in an Indian Opinion article that reprinted a travelogue of Ramsey MacDonald, the British Labour Party MP, who was impressed by the Gurukula at Hardwar [2].

Gandhi's time in South Africa had a significant impact on his life and work. He became involved with various social and political movements, including the Indian nationalist movement, and developed many of his ideas about nonviolent resistance and civil disobedience.

The exact nature of Gandhi's involvement with the Gool family is not fully clear from the provided sources. However, it is evident that he had a close relationship with Dr. J.M.H. Gool, who was a prominent member of the Indian community in South Africa [9].

### q38 — Who was Cissie Gool's father?

The provided sources do not contain explicit information about Cissie Gool's father. However, it is mentioned in [6] and [20] that Dr. Abdurahman married Helen "Nellie" Potter James, and that the Abdurahman clan was linked to the Ta'Als through the Dollie family.

It is also mentioned in [9] that Tabata, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdulla Abdurahman, Leah Solomon and Zaid Gamiet were luminaries who met Yousuf Rassool's uncle, Dr. (initially unidentified), but also mentions Dr. Abdulla Abdurahman as one of them.

However, [4] states that my uncle, Dr., l put his attainments? His father is a well-known public worker.

This source [20] does not mention Cissie Gool's name explicitly but it does state that Dr. Abdulla Abdurahman was married to "Cissie" or Zainunnisa (daughter of Dr. Abdulla Abdurahman, scion of Cape Town’s other leading dynasty).

It is implied in [20] that Dr. Abdulla Abdurahman's daughter Cissie Gool's father might be the same as her husband's.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to source [17], before the Group Areas Act (1950) dispersed its people across the Cape Flats, District Six was a "thriving cosmopolitan area in the heart of Cape Town" with a rich history and culture. The neighbourhood had a diverse population and a distinct character.

Source [18] describes it as having "more life, more zest, more fire, more warmth and care than many other places one could mention." It's described as a place that was buzzing with joy and life.

In source [19], the author reminisces about his childhood in District Six, describing Caledon Street as getting "tattier along the way", but still being a vibrant and lively area.

Source [18] also notes that the children of District Six would be tasked with protecting its memory from fading away.

### q40 — What was the Unity Movement's boycott policy?

According to the sources [4], [11], and [16], the Unity Movement's boycott policy included:

* Boycotting "dummy" institutions, such as seats on the Coloured Affairs Council (C.A.C.) [4]
* Boycotting services of members who accepted service on the C.A.C. [11]
* Refusing to cooperate with Group Areas Boards without guaranteeing that it would stop the implementation of the law [8]

The boycott was considered a weapon of choice, and its principle was based on non-collaboration [4].

