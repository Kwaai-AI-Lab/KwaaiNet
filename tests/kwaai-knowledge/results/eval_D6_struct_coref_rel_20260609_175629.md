# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 59.6% (134/225) |
| Avg latency | 28290ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 46399ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 33650ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 28386ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 25953ms |
| q05 | Who was J.M.H. Gool? | 3/8 (38%) | LEST WE FORGET -rev25.pdf, [Graph: Dr Goolam Gool District Six] | 30510ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 30696ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 23318ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 25800ms |
| q09 | Who was the author's grandfather? | 0/9 (0%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23873ms |
| q10 | Tell me about Kloof Nek. | 3/7 (43%) | LEST WE FORGET -rev25.pdf | 31284ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 27289ms |
| q12 | Who was Cissie Gool? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 29780ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 24834ms |
| q14 | Where was District Six and what kind of place was it? | 2/6 (33%) | [Graph: Dr Goolam Gool District Six], LEST WE FORGET -rev25.pdf | 27483ms |
| q15 | What were the forced removals from District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr Goolam Gool District Six] | 28275ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | [Graph: Goolam Gool], LEST WE FORGET -rev25.pdf | 31803ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 25570ms |
| q18 | What was the New Era Fellowship? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 27423ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 27908ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 27016ms |
| q21 | Who was the author's mother? | 2/5 (40%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 26058ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 26189ms |
| q23 | Who were the author's siblings? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 25000ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | LEST WE FORGET -rev25.pdf, [Graph: Dr Goolam Gool District Six] | 27962ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 26717ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 30151ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 27854ms |
| q28 | Which organisations was the author involved in? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 25190ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/6 (67%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 27624ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town-Woodstock P.T.A] | 27903ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 28992ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Nurjehan Gool Mohamed] | 27226ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Nurjehan Gool Mohamed] | 35554ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | [Graph: Dr Goolam Gool District Six], LEST WE FORGET -rev25.pdf | 30907ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 25152ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town-Woodstock P.T.A] | 29010ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 26599ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 25261ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 4/6 (67%) | [Graph: Dr Goolam Gool District Six], LEST WE FORGET -rev25.pdf | 28066ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 26953ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The author's children are: 

1. Feyruz
2. Reza
3. Zarina

Additionally, the author also has grandchildren: 

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

### q03 — Who are the author's grandchildren?

The author's grandchildren are: 

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan and
6. Lewis

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina ... and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis" [5].

### q05 — Who was J.M.H. Gool?

The provided sources do not contain a full profile of who J.M.H. Gool was. We know that he was the grandfather of the author, Dr. Yousuf (Joe) Rassool's "cousin," Marcina Kagan (presumably referred to as Aunt Fatima or Timmie), married a Hindu son named Ramdas [8]. He was a prominent citizen in Cape Town [14] and had interactions with many notable figures, such as Gandhi [7] and Gokhale [15]. However, the sources are sparse on the actual biography of J.M.H. Gool.

From [18], we understand that J.M.H. Gool performed his pilgrimage (Hajj) in 1900 and returned to Cape Town without Bibi, his second wife, who had given up any hope of salvation. He was a trader mentioned as an 'Oriental trader.' [20] 

There are some clues suggesting his involvement in civic and political issues; for example, he attended meetings of the Coloured People's Vigilant Committee [11], and the British Indian League [12]. The historical record is otherwise sparse.

### q06 — Tell me about Buitencingle.

Based on the provided sources, here's what can be said about Buitencingle:

* Buitencingle was a grand 13-room mansion located at No.7 Buitencingle Street in District Six, Cape Town.
* The house had a garden with fruit trees and was surrounded by a cobbled street with traffic islands, opposite the German Lutheran Church (Source: [9])
* The name "Buitencingle" is said to have been named after its location as the outermost belt of Cape Town at that time (Source: [7])
* Buitencingle housed multiple generations of the Gool family, including Haji Joosub Maulvi Hamid Gooli and his descendants (Sources: [1], [5])
* The house had a large number of rooms, which made it difficult to manage without the help of servants (Source: [18])
* Buitencingle served as a place for gatherings and events, including meetings of Dr. Abdullah Abdurahman and visits by important figures such as Gandhi (Source: [1], [3])

It is worth noting that Buitencingle played an important role in the family's history and was likely a significant landmark in District Six at the time.

However, some important questions about Buitencingle remain unanswered, such as:

* The exact date of its construction or when it fell into disrepair
* Details about the interior design and layout of the house beyond what is mentioned in sources [1] and [9]
* What became of Buitencingle after the Gool family left

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool, also known as Professor Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

[1] states that the author's wife is Nazima Rassool, who is also an academic researcher. 

Nazima urged the author not to rely solely on his own memories but to include historical research in the memoir, which led to a transformation of the book into a more historically grounded account.

The rest of the sources do not mention Joe's wife after Nazima being specified as his wife.

I could find no additional specific information about Nazima Rassool, the author's wife.

### q09 — Who was the author's grandfather?

The provided sources do not contain that information.

### q10 — Tell me about Kloof Nek.

According to the sources, Kloof Nek is a location in Cape Town that was known for its beauty (source [3], [18], and [20]). It was described as a "place of haunting beauty" (source [19] and [20]).

Kloof Nek seems to be situated between Table Mountain and Lion's Head, and it has a special significance in the story of the author. The author mentions that he and his friend Persotem used to go there together (source [1] and [18]), and that it was a popular spot for lovers, mountaineers, nature lovers, and rambles (source [18]).

In source [20], it is mentioned that Kloof Nek sprawled protectively on the eastern flank of the city, with Signal Hill visible from its flank. The noonday gun would sound regularly to herald midday, accompanied by a puff of smoke.

The description in source [3] mentions that when the author visited Kloof Nek for the first time at 14 years old, he was led there by a schoolmate who claimed it was an ideal spot for studying poetry. Despite initial reluctance, the author came to appreciate its beauty and tranquility.

Overall, Kloof Nek seems to be a cherished location in Cape Town with scenic views and a special significance in the author's life.

### q11 — What was the Teachers League of South Africa (TLSA)?

According to [1] and [6], the Teachers' League of South Africa (TLSA) was a non-European teachers' professional body that took a strong political stance against apartheid education policies. It had an editorial board where writers, who were considered good, received training in political/educational composition and got articles published in the TLSA's Education Journal ([5] & [8]). 

The TLSA later had a change in leadership with "a new guard" of pro-apartheid members taking over from the old anti-apartheid guard ([6]). The TLAS was seen as an untenable institution by the government because it opposed apartheid and promoted social equality for non-White people, which was not acceptable to the ruling authorities.

The TLSA had its own branch in Athlone (e.g. [12]), where Yousuf Rassool worked, including as chairman of the branch ([10]). Despite the challenges faced by the organization, its members, like Danny De Beer and Yousuf Rassool himself, continued to contribute actively to the struggle for democracy and equality.

### q12 — Who was Cissie Gool?

Cissie Gool was a prominent South African feminist and anti-apartheid activist. She was a member of the Gool family, who were of Indian origin and had lived in Cape Town for several generations [1], [3], [4]. According to one excerpt [9], Cissie Gool was also married to Dr. Gool. However, another source mentions that she was married to Dr. Abdul Gool [18].

The sources describe Cissie Gool as a highly respected figure in Cape Town society and a fierce advocate for the rights of her community [9]. She addressed mass meetings, including one on the Grand Parade where thousands of White ex-soldiers supported the Nationalist party's "influx control" measures [17]; however, this is contradicted by other excerpts [4] which suggest that she was not in favor of segregation and fought against it.

Cissie Gool also had a relationship with Ismail, a notable figure whose real name the author couldn't understand due to his gravelly growl accent [7].

### q13 — What was the All Africa Convention?

The provided sources do not contain detailed information about the All African Convention beyond mentioning it in several excerpts [2], [3], and [4]. It is mentioned as an organization related to Mr. Tsotsi, Honono, Sihlali, Enver Hassim, Errol Vawda, Kader Essack, and Victor (likely referring to I.B. Tabata in other sources) that was concerned with the educational front and African peasantry.

### q14 — Where was District Six and what kind of place was it?

District Six was a thriving cosmopolitan area in the heart of Cape Town, South Africa. According to the excerpts [1] and [2], Dr Goolam Gool mentioned that District Six was located within District Six, while Chapter Two of the book describes it as "the mean streets" but also notes that "those bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention."

The descriptions suggest that District Six was a densely populated urban area with a mix of cultures, languages, and ethnicities. [6] mentions the Cheshire Sunshine Boys, African tap dancers who would entertain passersby on the sidewalks without expectation of fame or fortune. The district seemed to have a lively atmosphere with various street performers, traders, and services.

However, it was also described as "a black spot" in the Apartheid scheme of things [13], implying that District Six was not well-regarded by the apartheid authorities due to its cosmopolitan nature and diversity.

### q15 — What were the forced removals from District Six?

[1] Dr Goolam Gool District Six: Dr. Goolam Gool District Six — location: District Six [Note: The provided sources do not describe the forced removals themselves, but they are implied to be related to the demolition of District Six.]

[10] Perhaps he knew the family, or perhaps it had to do with the dux medal bursary... He said firmly I did not belong there. This suggests that due to new housing laws and restrictions (possibly Group Areas Act), Joe Rassool would have been forced out of living at 7C.

[11] those bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention. To me District Six was a thriving cosmopolitan area before the demolitions.

[13] The Group Areas Act (1950) dispersed its [District Six's] people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life.

The sources describe District Six as a thriving, vibrant community before it was demolished. This would have implied that they were forcibly removed from their homes to other areas in accordance with the Group Areas Act and the apartheid regime's policies.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to sources [2], [3], and [17], Mohandas Karamchand Gandhi (also known as Mahatma Gandhi) had a close connection with the Gool family. Specifically, [2] mentions that Mr. Yusuf Hamid Gool arranged for meals for Gandhi and his party at his place when they arrived by train in Cape Town on October 3, 1906.

Sources [4] and [7] also highlight Gandhi's interactions with members of the Gool family during his stay in Cape Town. For instance, [7] mentions that Dr. G.H. Gokhale was a guest at Mr. Adam Gool's residence, "Noor Bagh," in 1912.

Gandhi's correspondence with J.M.H. Gool is also referred to in source [8], where it is mentioned that Gandhi helped Dr. Gool's son, Abbas, in his medical studies and stayed at their residence, Buitencingle, on multiple occasions.

Source [18] notes that Abdurahman were central to the history of District Six and of the Indians and Coloured people of South Africa, and mentions that Gandhi had hospitality provided by the Gool family during his visit in February 1914.

Overall, these sources suggest a close and cordial relationship between Gandhi and various members of the Gool family, with whom he interacted during multiple visits to Cape Town.

### q17 — What was Hewat Training College?

According to excerpts [2] and [9], Hewat Training College was a teacher training college in Cape Town, South Africa. It trained teachers for upper years of primary division schools (excerpts [5] & [16]). The author of this memoir attended Hewat Training College from January 1947 (excerpt [1]). The college grounds were located near District Six and had once been a Whites-only Primary school (excerpt [3]).

### q18 — What was the New Era Fellowship?

The documents don't provide a detailed description of the New Era Fellowship. However, from [14] it is evident that "We all eagerly purchased our copies, and set about distributing them among our friends and acquaintances." This implies that the book The Awakening of a People was published by NEF, suggesting that NEF is an organisational entity, not just a cultural movement (mentioned in [19]).

### q19 — What was the Non-European Unity Movement?

The sources [1] state that the Non-European Unity Movement (NEUM) was a South African political movement founded in 1943. The NEUM united non-European political organisations opposed to apartheid and racial discrimination, advocating for non-collaboration with apartheid institutions.

Source: [1]

Additionally, source [12] provides more context on the NEUM's founding and its significance:

"...the young Turks as they were called among whom Grandpa’s daughter, Aunt Jane, and her brother, Dr. Goolam, were moving spirits."

It can be inferred that the movement was led by individuals of Indian and African descent who aimed to bring about unity among non-European communities in resistance against apartheid.

Source: [12]

### q20 — Describe the author's involvement in cricket.

The provided sources describe the author's involvement in cricket throughout his childhood and adolescence.

At a young age, the author played cricket and was a member of several cricket clubs. He remembers making an impressive 12 runs in a match (Source [4]), indicating that he was an enthusiastic player.

Later on, the author developed a serious interest in cricket due to the influence of Hassen Mall. He began to learn how to play properly and started practicing his forward defensive stroke (Sources [16] and [18]).

As he grew older, the author became involved in organizing cricket teams and matches within his community. For instance, when Hewat school did not have a cricket team, the author took charge and formed one by calling meetings with interested students (Source [16]).

Despite not initially being selected to play for the top league, the author remained optimistic and dreamed of being chosen for his club's team in tournaments such as the biennial cricket tournament held in Durban (Sources [17] and [18]).

Through these sources, it is clear that the author was an enthusiastic and dedicated cricketer who wanted to make a name for himself in the sport. However, some specific details about his performance or achievements are absent from the provided text.

### q21 — Who was the author's mother?

The author's mother was Bibi. However, the text also mentions the name Ayesha as being associated with the baby born nine months after Bibi (presumably this is the same child or another reference to her), making it seem likely that Bibi was either Ayesha or had a daughter named Ayesha [7].

### q22 — Who was the author's father?

The source states that the author's father's name was Peter Alexander Rassool [10]. It is also mentioned in other sources that he had been known as Peerbhai or Peru after converting to Islam [8] and was a Muslim/Indian [13, 14], as cited by different excerpts.

### q23 — Who were the author's siblings?

The text does not provide a comprehensive list of the author's siblings. However, it mentions that Fazil was the author's brother (Source [14] is likely referring to another person with a similar name) and that the twins Nurjehan and Zarina were also mentioned ( Source [10]).

In source [14], Hanief Ally and Gadija are mentioned as foster children, but their relationship to the author is not clear. It could be inferred that they might be some of his siblings or possibly first cousins given their similar age and close proximity in text.

### q24 — Who were the children of J.M.H. Gool?

According to excerpts [1] and [8], J.M.H. Gool had five children:

1. Bibi (also referred to as Wahida's children, who were born in India)
2. Kassim (born in 1900)
3. Barrabhai (whose mother was Bibi, but the exact birth order is not specified)
4. Nurjehan (who fell in love with someone named The Strand and settled there)
5. Several others are mentioned in [8], including Nasima, twins Nurjehan and Zarina, but their births are not precisely dated.
 

In excerpts [7] it states that "my first five children" were involved playing a game of Tarzan.

### q25 — Who was I.B. Tabata?

According to [2], Mr. I.B. Tabata became chairman of the New Era Fellowship and completed his book The Awakening of a People, which was an occasion of considerable excitement as it would be an NEF publication.

Additionally, excerpts [7] and [16] suggest that Mr. Tabata was influential in the movement and had strong opinions, but they do not provide more information about who he is beyond being a leader or chairman.

[12] provides some context for his possible presence at meetings, but it does not give any specific information about Mr. Tabata's background.

Further details cannot be found within the provided sources.

### q26 — Who was Dr. Abdullah Abdurahman?

According to [1] Dr. Abdulla Abdurahman: Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century.

He was also mentioned as visiting the Gool family mansion at Buitencingle Street, being friends with J.M.H. Gool, [3] Dr. Abdurahman were present to hear the Progressive Party appeal for the “Coloured” vote, and having a bond with my grandfather, which ultimately brought Cissie and his uncle Dr.

He was also mentioned in various other passages mentioning that:

* He was one of the first doctors from Glasgow University in 1893 [17] 
* His mother, Gadija Dollie, was said to be a most beautiful woman. [16]
* His grandfather was brought as a slave to South Africa and bought his freedom.
* Dr Abdurahman was given a farewell address by Dr Gool on behalf of Port Elizabeth Indians in August 1914.

And is also mentioned with the following prominent individuals:

* Prince of Wales, George Bernard Shaw, Sarojini Naidu [20]
* My grandfather being close to him in Loop Street
* Being given gifts and addresses when visiting Cape Town
* Being mentioned alongside other important figures in society.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources describe a close relationship between Gandhi and J.M.H. Gool, with various aspects of their connection mentioned:

* Gandhi often visited Gool's residence at 7 Buitencingle Street in Cape Town, where he would help with household tasks (e.g., puttying the floor [9]). 
* Gool was a witness in the Supreme Court in 1892. 
* In 1911, Gandhi helped to furbish Gool's surgery by staining and puttying the floor.
* The authors of this memoir mention that Gandhi invested hope in young Dr. J.M.H. Gool for community services and built up his image in the Indian Community through editorials in newspapers. 

Their relationship is also mentioned on several occasions, such as when Ralph Bunche records Gandhi's disapproval of one of his sons falling in love with a Muslim girl from a prominent Indian family, similar to the Gools in [18].

### q28 — Which organisations was the author involved in?

The author, Yousuf (Joe) Rassool [1], was involved in several organisations:

1. Teachers League of South Africa (TLSA)
2. Non-European Unity Movement (NEUM)
3. New Era Fellowship (NEF)

He was a teacher and community activist, deeply involved in the democratic movement [16]. He also mentioned his involvement with the Cape Peninsula Students Union [18] but it does not seem to have been directly affiliated with him.

The author was a member of the New Era Fellowship and played an active role in representing them at various events and meetings, such as the conference of the All African Convention in Bloemfontein [14].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The Teachers' League of South Africa (TLSA) ultimately joined the Non-European Unity Movement (NEUM), acknowledging that the struggle for equal education is bound up with the struggle for a democratic society [17]. This step was not achieved without conflict, as the TLSA had to deal with the old guard and eventually took control from them with the new leadership taking charge of the organisation [17].

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information about when J.M.H. Gool arrived in Cape Town or from which location he arrived. However, we know that he had two wives: Bibi, who lived in India (Gujarat), and Wahida Ta'Al, a Dollie from the Cape [3] [11].

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was a mosque in Cape Town, founded by Haji Joosub Maulvi Hamid Gool and completed in 1898 [1]. It served the Muslim community of District Six and the surrounding area.

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not explicitly state how Cissie Gool was related to J.M.H. Gool, but [18] indicates that Wahida (Hajima) Ta'Al and Dr. Abdurahman's mother Gadija Dollie's family is tied to the Cape area of importance and suggests a potential link or connection through family; however, there is no explicit information about Cissie Gool's specific relationship to J.M.H. Gool in these sources.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources mention the following historical figures that Yousuf (Joe) Rassool's grandfather, Joosub Maulvi Hamid "JMH" Gool, knew personally:

1. Mahatma Gandhi: There is a family connection mentioned in [13] and [14], where it appears that Gandhi's son fell in love with J.M.H. Gool's daughter, Fatima (also known as Auntie Timmie). This incident suggests a personal relationship between the two families.

2. Cissie Gool: A prominent figure in her own right, she was present on the platform during meetings held by Gool [14].

3. Abdurahman: This is likely referring to Dr. Abdullah Abdurahman, who was a community leader and one of the five Non-European representatives to be elected to Parliament in 1920 [17]. The text implies that Dr. Abdurahman's grandfather was brought as a slave from India but later became prominent in the community.

4. Manilal: Manilal is mentioned as hosting the Gools at Phoenix in Figure 37, although his first name is missing and it seems to be referring to an unrelated historical figure or a misprint.

5. Mahomed Adam Haji Gool (possibly): In [18], Mahomed features in the historical record as "Gool's senior, financier and possibly mentor." It is not explicitly stated that he knew J.M.H. Gool personally, but their connection seems significant given this mention.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [13] was a law passed in South Africa in 1950, as part of the Apartheid legislation. Its purpose was to divide the country into separate group areas for different racial groups, with the ultimate goal of confining non-whites to their designated areas and preserving white dominance.

The act affected District Six significantly [17]. District Six, a thriving cosmopolitan area in Cape Town's city center, became one of the few areas where different racial groups lived together. However, under the Group Areas Act, African, Coloured, Indian, and White communities were forced to move to designated areas outside the city.

For people living in District Six, the act meant that they had to be classified according to their race [20]. Non-whites were relocated to Coloured or Bantu (African) Locations outside the city, leading to displacement and loss of livelihoods. Those with limited financial means and no other options were forced to move into areas where housing was inadequate, leading to overcrowding and poverty.

The classification process and subsequent removal of residents from District Six ultimately aimed at eradicating its multicultural character and economic vitality [11]. The government hoped that by dividing communities along racial lines, they could break the existing sense of unity and cooperation among residents and undermine the community's overall prosperity.

A consequence of this legislation was increased housing shortages within designated coloured areas. People previously residing in District Six were left to either move into cramped conditions or live with extended family members while others resorted to "renting out" parts of their homes to accommodate family members without separate living conditions [20].

The effects on the community as a whole were devastating, leading to social disintegration and displacement [16] of many residents. Some protested against the iniquity of the Group Areas Act, but ultimately, the implementation had an overwhelming impact on District Six's community and social fabric.

### q35 — Who was Hassen Mall?

Hassen Mall is described as a brilliant cricketer who spun a wristy leg break (source [16]). He also had strong personality and great charm (source [20]). According to the sources, he completed his LLB degree (source [3]). In his later years, he developed a clipped brush moustache and smoked a pipe (source [16]). Hassen was left for Durban in 1951 after having completed his studies there (source [16]). As of that time, he had already found his true talent in the field of Law. He later became the president and chairman of different organizations (not described here).

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpts [5], [16], and [18]-[20], the following political organisations were active in the Cape Coloured community during the author's lifetime:

* The New Era Fellowship
* The African People’s Organisation (A.P.O.)
* The Communist Party of South Africa
* The Fourth International of South Africa (implied to be a significant organisation, but no further details)
* The Liberation League
* The Workers’ Party
* The Anti-CAD movement (Anti-Coloured Affairs Department)
* The Congress Movement (mentioned in [11])
* Smuts’s United Party (had a Coloured branch)
* Other organisations that responded to the call for unity and full democratic rights in the New Era Fellowship ([16])

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was a prominent Indian independence activist, and it appears that there were actually multiple visits by different individuals with the name "Gandhi" to South Africa. 

According to [1, 7] the British Colonial Secretary at the time, Joseph Chamberlain, my grandfather believed that England was the place where their succour lay in addressing the difficulties faced by Indians in South Africa.

It is mentioned that Gandhi had visited Buitencingle yet again in 1914 (February- March) and was a guest of my grandfather's [14].

Mahatma Gandhi visited South Africa twice, first arriving in 1893 for a one-year assignment to assist his attorney in a civil suit against a merchant in Pretoria.

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information about Cissie Gool's father. [2] mentions that Cissie's husband, Abdul Hamid Gool, is the son of Dr. Abdulla Abdurahman (Cape Town’s other leading dynasty), but it does not mention her father.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to sources [1], [5], and [18], District Six was a thriving cosmopolitan area in the heart of Cape Town, known for its vibrant community and way of life. It had a rich tapestry of people from diverse backgrounds, including freed slaves from America and the West Indies who had settled there in Victorian times ([17] provides more context on this).

The neighborhood was bustling with activity, and Excerpt [5] describes it as "pulsing" with life, care, warmth, and zest. The author remembers seeing children playing in the streets, and hearing vendors calling out to potential customers from their stalls.

Excerpt [18] states that before the forced removals, District Six was a place where people of different ethnicities, cultures, and backgrounds lived together, sharing ways of life and experiences. However, this diversity was seen as a threat by the apartheid government, which eventually led to the displacement of its residents.

Overall, District Six was a dynamic and multicultural neighborhood that played an important role in shaping the lives of those who lived there, including the author of this memoir.

### q40 — What was the Unity Movement's boycott policy?

According to excerpts [1], [3], and [14], the Unity Movement's boycott policy involved refusing to cooperate with, or boycott, institutions and laws that they saw as supporting or promoting apartheid. However, it is also mentioned in excerpts [10] and [16] that this boycott was not always effective, especially when property owners were concerned.

There are two specific instances of the boycott: Firstly, in excerpt [1], it is mentioned that the board members of s Hostel informed the Unity Movement that they would no longer be allowed to meet there in retaliation for their boycott campaign. The author attributes this to an incident with Dr. Motala, a Dr., and one of the board members, 

 Secondly, in excerpts [13] and  [16], it is mentioned that a boycott was also effective against the Coloured Affairs Council (CAC) - Barbers refused to cut their hair, tailors would not sew suits, shopkeepers wouldn't serve CAC-men.

