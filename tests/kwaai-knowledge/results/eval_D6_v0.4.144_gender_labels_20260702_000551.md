# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 66.2% (147.0/222) |
| Avg latency | 32292ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 0/3 (0%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 23272ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 48103ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 28586ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 24438ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 33464ms |
| q06 | Tell me about Buitencingle. | 6/8 (75%) | LEST WE FORGET -rev25.pdf | 24332ms |
| q07 | Who is the author's wife? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 20103ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 20913ms |
| q09 | Who was the author's grandfather? | 7/9 (78%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 29352ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 26195ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 22592ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 38119ms |
| q13 | What was the All Africa Convention? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 24150ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf | 22408ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf, sequence_diagram:District Six | 31923ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 27191ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 19105ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 24700ms |
| q19 | What was the Non-European Unity Movement? | 5/6 (83%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 22406ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 36904ms |
| q21 | Who was the author's mother? | 2/5 (40%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 23361ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 32614ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 33081ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 72729ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 54938ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 44069ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 44750ms |
| q28 | Which organisations was the author involved in? | 4/5 (80%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 40563ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 5/6 (83%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 54478ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: Haji Joosub Maulvi Hamid Gool] | 52217ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 2/6 (33%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 41341ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 31730ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28595ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six], sequence_diagram:District Six | 33494ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 27828ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Anti-CAD] | 32043ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | [Graph: South African Directory], LEST WE FORGET -rev25.pdf | 22561ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 24757ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | [Graph: Chapter Twenty-One Leaving District Six], LEST WE FORGET -rev25.pdf | 25662ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 22619ms |

## Answers

### q01 — Who is the author?

The provided sources do not contain that information.

### q02 — Who are the author's children?

According to source [8], the author's children are:

1. Feyruz
2. Reza
3. Zarina
4. Joseph (grandchild)
5. Samuel (grandchild)
6. Alexander (grandchild)
7. Leonardo (grandchild)
8. Dylan (grandchild)
9. Lewis (grandchild)

### q03 — Who are the author's grandchildren?

According to the provided sources:

[1] The author's grandchildren are mentioned in the dedication as:
Joseph Rassool
Samuel Rassool
Alexander Rassool
Leonardo Rassool
Dylan Rassool
Lewis Rassool [1]

Additionally, [18] mentions that the Gool family and the author have a friendship. In this context, it also mentions the following grandchildren in relation to the Gools:
The centenary of Gandhi’s stay at the Gool mansion at 7 Buitencingle is mentioned.

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" is dedicated to the author's children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [18].

### q05 — Who was J.M.H. Gool?

J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a spice merchant and community leader who lived in Cape Town, South Africa. Here are the specific facts stated about him:

* He arrived in Cape Town in 1884 from Mauritius.
* His family originated from Swat via Gujarat, India.
* He was married to two women: Bibi Gool (whom he married at age 16) and Wahida Ta'al.
* He had multiple children with both wives, including Ayesha Rassool (by Bibi) and Abdul Hamid Gool (by Wahida).
* He was a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* He lived at 7 Buitencingle Street.
* He performed the Hajj in 1899.
* He died on April 1, 1940.
* He was acquainted with Mahatma Gandhi during Gandhi's 1914 visit to Cape Town.
* He was associated with early Cape Muslim and Indian political circles.
* His children included:
	+ Ayesha Rassool (by Bibi Gool)
	+ Abdul Hamid Gool (by Wahida Ta'al)
	+ Jane Gool-Tabata
	+ Mariam Margie Gool
	+ Minnie Gool
	+ Zobeida Gool
	+ Goolam Gool
	+ Fatima Gool
	+ Mohamed Saaid Gool
* He was the grandfather of Yousuf (Joe) Rassool, the author of the memoir "LEST WE FORGET".

Note: These facts are based on information from [1] and other sources.

### q06 — Tell me about Buitencingle.

From the provided sources:

* In [1], the author describes Buitencingle as being located beyond the outermost belt of Cape Town, with a wide and cobbled street (Buitencingle Street) lined with traffic islands and stunted palm trees.
* The German Lutheran Church was situated opposite No. 7 Buitencingle Street, where the author's grandfather lived in a thirteen-roomed mansion ([3]).
* Buitencingle was described as vibrant with élan, grandeur, and joie-de-vivre at the turn of the century ([9]).

There is limited information about the layout or physical characteristics of Buitencingle beyond these descriptions.

### q07 — Who is the author's wife?

The provided sources state that the author's wife is Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about the author's wife, Nazima Rassool [1]. Here are the specific facts mentioned:

* She is married to Yousuf (Joe) Rassool [1].
* She is an academic researcher [9] and [19].
* Her husband was encouraged by Brian Willan's words in support of his project [7].

However, I must emphasize that most biographical questions require a comprehensive answer.

### q09 — Who was the author's grandfather?

The provided sources do not contain that information. However, based on source [1], Haji Joosub Maulvi Hamid Gool is mentioned as the grandfather of Yousuf (Joe) Rassool.

Here are the facts about him from source [1]:

* Name: Haji Joosub Maulvi Hamid Gool
* Also known by other names (e.g., J.M.H. Gool, J. M. H. Gool)
* Foster parent of: Hamid Ally, Gadija Dija Noor, Toetie Ally, Gadija Noor, Hanief Ally
* Founded: Hanaffi Quwatul Islam Mosque
* Grandfather of: Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf Rassool (the author), Zain Rassool, Fazil Rassool, Yasmin Rassool
* Lived in: District Six, 7 Buitencingle Street
* Father of: Abdul Hamid Gool, Jane Gool-Tabata, Mariam Margie Gool, Minnie Gool, Zobeida Gool, Goolam Gool, Fatima Gool, Mariam Gool, Mohamed Saaid Gool, Ayesha Rassool, Mohammed Hanief Gool, Fatima Timmie Gool, Zohra Abdurahman
* Married to: Bibi Gool, Wahida Gool
* Died on: 1940-04-01 (April 1st, 1940)
* Arrived in Cape Town: 1884
* Origin: Mauritius (via Swat, Gujarat)

### q10 — Tell me about Kloof Nek.

According to the sources:

[3] describes Kloof Nek as "a place of haunting beauty" and states that it is located on the flank of a hill, with Signal Hill on its flank.

[17] mentions that Kloof Nek was frequented by lovers, mountaineers, nature lovers, and ramblers, but also notes the presence of down-and-outs known as "Bergies", who would drink away their sorrows in the area.

[18] explains that Kloof Nek is a saddle of land connecting Table Mountain to Lion's Head, and that it sprawled protectively on the eastern flank of the city.

[20] provides further details about the location of Kloof Nek, stating that it was beyond Buitencingle, which was the outermost belt of Cape Town at the time.

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources do not contain detailed information about what the TLSA was. They mention it as an organization that opposed apartheid and collaborated with the Cape African Teachers' Association, but they don't provide a clear definition or description of its goals, activities, or mission. 

However, [1] describes the TLSA as "a professional association" affiliated with the Non-European Unity Movement, located in South Africa.

### q12 — Who was Cissie Gool?

According to the sources:

* [1] Haji Joosub Maulvi Hamid Gool's son Abdul Hamid Gool had two wives: Bibi and Wahida. Cissie Gool is mentioned as one of their daughters, and also as the wife of Dr Abdurahman (source [9]).
* [2-5] Cissie Gool is a prominent figure in the community, associated with various events and meetings, and has relationships with other notable individuals.
* [6-7] She was the wife of Dr Gool and later married to Dr Abdurahman.
* [8] Cissie Gool is mentioned as one of the brave and strong women fighting against social, economic, and political injustice.
* [9] Cissie Gool's relationship with Dr Abdurahman and her involvement in politics are highlighted.
* [10-14] Cissie Gool addresses meetings and rallies, supporting various causes, including "influx control" and the street gangs, colloquially referred to as the “skolly” problem.
* [15-17] Cissie Gool's role in organizing events and her personal life are described.
* [18-20] Cissie Gool is portrayed as a powerful figure in the community who calls massive demonstrations against residential segregation.

From the above, we can infer that:

Cissie Gool was a strong-willed and influential woman who played a significant role in the community. She was married to Dr Abdurahman and had relationships with other notable individuals. Cissie Gool was involved in various events and causes, including fighting against social, economic, and political injustice.

Here is a comprehensive list of facts about Cissie Gool from the sources:

* Married to: 
	+ Dr Gool (source [7])
	+ Dr Abdurahman (sources [9], [18-20])
* Children:
	+ Shaheen (son with Dr Abdurahman, source [13])
	+ Other children (names not specified) (source [19])
* Involvement in events and causes:
	+ Supported "influx control" and the street gangs, colloquially referred to as the “skolly” problem (sources [14], [16], [20])
	+ Called massive demonstrations against residential segregation (source [18])
	+ Involved in organizing events, including her husband's funeral (sources [13], [15])
* Relationships:
	+ With Dr Abdurahman and his family (sources [9], [18-20])
	+ With other notable individuals, including Cissie Gool's uncle and aunt (source [12])

Note: There may be additional information about Cissie Gool that is not mentioned in the sources.

### q13 — What was the All Africa Convention?

The sources do not provide a detailed description of the All African Convention. However, [3] mentions that "The book recorded the coming into existence of the All African Convention, the organisation that was one of the pillars of the Non-European Unity Movement." 

We can infer from this statement that the All Africa Convention (AAC) is an organization related to the Non-European Unity Movement.

It's also mentioned in [2] and [19-20] that the author attended a conference of the All African Convention, where he met some leaders, including Messrs. Tsotsi, Honono, Sihlali, Enver Hassim, Errol Vawda, and Kader Essack.

A partial list of items related to the AAC from [4] is:

* Ahmed Abdurahman, 40 
* All African Convention, 137, 148, 156

### q14 — Where was District Six and what kind of place was it?

According to the sources [11] and [20], District Six was a thriving cosmopolitan area located in the heart of Cape Town. It was described as a "black spot" in the Apartheid scheme of things, implying that it was an integrated community with people from different racial backgrounds.

Source [3] describes District Six as a place with "more life, more zest, more fire, more warmth and care than many other places one could mention." It is also mentioned in source [20] that it was being bulldozed out of existence due to Apartheid policies.

Additionally, source [10] states that District Six was a community where people from different racial backgrounds lived together, and the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats, decimating the once vibrant community and way of life.

### q15 — What were the forced removals from District Six?

The provided sources do not contain information about the specific details of the forced removals from District Six. They mention "the area to which we were moving was not then the run-down city centre that it was to become by the time we departed from it in 1956" [16] and "we did not know in 1956 that we would fail to... m[ove]" [17], suggesting that there was a movement or relocation taking place, but do not specify what this movement entailed.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi was an Indian independence activist who played a significant role in India's struggle for independence from British rule. According to the sources, Gandhi had connections with the Gool family through various interactions and relationships.

The following facts are mentioned in the sources:

* Gandhi visited Buitencingle Street in 1911 and helped putty and stain the floor of Dr. A. Gool's surgery [8].
* In 1906, Gandhi wrote a letter to Chaganlal Gandhi mentioning that the slide in Gool's fortunes started early [7].
* Gandhi attended a function at the Gool residence in March 1914, where he was hosted by the family and thanked them for their hospitality [4].
* Gandhi also stayed with the Gools on several occasions, including in February-March 1914 when he visited Buitencingle Street again [4].
* Dr. A. Gool read an address of welcome to visiting dignitary Gokhale at Adam Gool's residence "Noor Bagh" in 1912 [11].

These interactions suggest that Gandhi had a close relationship with the Gool family, who were influential Indians in Cape Town during his time there. The sources also mention that the Gools provided hospitality and support to Gandhi on several occasions.

It is worth noting that Adam Gool was mentioned as one of the Kanamia Muslims and the President of the British Indian League [11], while Dr. A. Gool was involved in politics and became president of the Cape British Indian Union in 1911 [8].

### q17 — What was Hewat Training College?

Hewat Training College was a Post Matriculation Teacher Training facility for Coloureds in South Africa. It opened for its first intake of students in 1944, and it trained teachers for the upper years of the primary division [3]. 

The college had undeveloped grounds that were once part of a Whites-only Primary school [20], but they were deemed suitable for the new facility despite being near the Roeland Street Gaol and District Six [20].

### q18 — What was the New Era Fellowship?

The provided sources do not contain a comprehensive description of what the New Era Fellowship (NEF) was. However, based on the excerpts, here is what can be inferred:

* The NEF was a political and cultural organisation [2].
* It was associated with the Unity Movement [3].
* The NEF had a programme of lectures and events, overseen by the Organising Secretary [5][9].
* Members included individuals such as Hassan Bavasah, who became the Organising Secretary [5], and Yousuf Rassool (the author), who later became the chairman [15].
* The NEF played a role in the Coloured community's resistance to apartheid, and its leaders were involved in various protests and movements.

The exact nature and scope of the New Era Fellowship are not explicitly stated in the provided sources.

### q19 — What was the Non-European Unity Movement?

According to [1], the Non-European Unity Movement (NEUM) was "a movement that was founded by the South African Communist Party and had a founding member in the Unity Movement." It was also mentioned in [8] as being seized by its founders in 1943, with Grandpa's daughter Aunt Jane and her brother Dr. Goolam being moving spirits.

[2] mentions the idea of non-European unity movement being discussed in Cape Town, but it was not yet ready for implementation. 

The NEUM is described in [8] as "revolutionised the political scene" with its doctrine of non-collaboration and the boycott weapon, and galvanized the non-white disenfranchised to a realisation of the need for a programme-based struggle.

[9] mentions that the Unity Movement was the only organisation active in the Transkeian Territories, where peasants turned to the NEUM in their plight.

### q20 — Describe the author's involvement in cricket.

The author's involvement in cricket is described in several excerpts:

* [2] The author mentions that the rusty structures and pigs snuffling about in the oozy mire was an experience that helped to strengthen their intentions to do something positive, but it doesn't specifically mention cricket.
* [3] The author says they had to wait four more years for the honour of being selected for a team. They were interested in playing cricket and were on a list of suggested players (H. Mall, C. Khatieb, M. Allie, S. Abed, I.Hayat, A. Banoo, etc.) by the Non-European Weekly.
* [4] The author mentions Edross's life revolved around the Barnato Cricket Board, and that they had a forward defensive stroke that impressed Edross enough to say he would have chosen them for the Indian cricket union if he were a selector.
* [5] The author mentions Hassen Mall as their friend who was learning to play cricket seriously. They began to master the elements of the forward defensive stroke and built the foundation of the art of batting.
* [6] The author says they had an experience in participating in disseminating the ideas of the movement, but also being able to go out at night and enjoy other activities, like cricket.
* [7] The author mentions that Hassen Mall was captaining a team for the biennial tournament and that they were selected to represent the Western Province.
* [8] No new information about the author's involvement in cricket is mentioned in this excerpt.
* [9] The author describes their cricketing days, mentioning that Aggressive as a batsman but gentle as a person with a great sense of humour. They also mention making an oh so glorious 12 runs and being hailed as some sort of hero.
* [10] The author says they were thrilled when Hassen Mall was chosen to captain the side for the biennial tournament, taking place at Durban's Curry's Fountain in early January.
* [11] No new information about the author's involvement in cricket is mentioned in this excerpt.
* [12] The author mentions that their brother Fazil made the highest bid for a residence after Cassiem and his wife died. They also mention that they did not make the final team that went on tour to Johannesburg.
* [13] The author says they were left out of selection, despite making an oh so glorious 12 runs.
* [14] No new information about the author's involvement in cricket is mentioned in this excerpt.
* [15] This excerpt mentions a thrilling game of cricket but doesn't specifically mention the author's role or experience.
* [16] The author describes seeing Len Hutton and other sportsmen play at Newlands Cricket Grounds.
* [17] The Kismets team, including the author, poached players from other teams, including Salie Van Haacht, Ben Malamba, and Stanley Abrahams.
* [18] No new information about the author's involvement in cricket is mentioned in this excerpt.
* [19] This excerpt mentions several players who were selected for a tournament, but doesn't specifically mention the author's role or experience.
* [20] No new information about the author's involvement in cricket is mentioned in this excerpt.

From these excerpts, it can be inferred that the author was involved in cricket as a player, and was part of teams such as the Kismets. They also seem to have had friends who were involved in cricket, including Hassen Mall, and they attended matches and tournaments.

### q21 — Who was the author's mother?

The provided sources do not contain specific information about the author's father or family relationships outside of his parents. However, according to [1], the author is Yousuf (Joe) Rassool, and his wife is mentioned as Peter Alexander Rassool, who was known as Peerbhai or Peru.

To answer your question directly: The provided sources do not contain specific information about the author's mother.

### q22 — Who was the author's father?

The provided sources do not directly mention the name of Yousuf (Joe) Rassool's parents. However, [1] mentions Peter Alexander Rassool as a person who lived in District Six and had children, including Abdul Rassool and Yousuf Rassool. It is likely that Peter Alexander Rassool was Yousuf Rassool's father.

Further information about the author's family can be found in various excerpts:

* [4] mentions Malick Rassool as the father of Yousuf (Joe) Rassool.
* [5] does not provide direct information but mentions Nasima, whose father was born in Mauritius. This might be a sibling or relative of the author's family.

To determine the correct answer with certainty, we would need more specific information from other sources about Yousuf Rassool's parents.

### q23 — Who were the author's siblings?

The provided sources list the following individuals as the author Yousuf (Joe) Rassool's siblings:

1. Abdul Rassool
2. Berina Rassool
3. Fazil Rassool
4. Nasim Rassool
5. Rasheda Rassool
6. Yasmin Rassool
7. Zain Rassool

### q24 — Who were the children of J.M.H. Gool?

According to the provided sources:

[1] lists the grandsons and granddaughters of J.M.H. Gool, but does not explicitly mention their parentage:
- Rasheda Rassool
- Abdul Rassool
- Berina Rassool
- Nasim Rassool
- Yousuf (Joe) Rassool
- Zain Rassool
- Fazil Rassool
- Yasmin Rassool

[2] mentions the birth of Kassim, but does not state that he is a child of J.M.H. Gool.

[3] and [4] do not mention the children of J.M.H. Gool.

[5] refers to an interview with Aunt Jane, but does not provide information on the children of J.M.H. Gool.

[6] mentions the presence of Wahida's industrious support in the family, but does not list the children of J.M.H. Gool.

[7] provides a list of notable individuals from the non-white struggle for equality in South Africa, including Manilal, who hosted the Gools at Phoenix, and is implied to be an associate or acquaintance of J.M.H. Gool, but this does not specify that he is a child.

[8] mentions the relationship between J.M.H. Gool and his grandson Yousuf (Joe) Rassool, who is the author of this memoir, but lists the grandsons as mentioned in [1].

[9] lists various individuals associated with the Gool family, including Cissie Gool, Abdul Hamid Gool, Fatima "Timmie" Gool, Zohra "Jolly" Gool, Mariam "Margie" Gool, and Toetie Noor as children of J.M.H. Gool.

[10] provides a photograph of the Gool family, including J.M.H. Gool (Grandpa), Sayed Noor (Chota), Nurjehan, Ayesha (Mom), Bibi (Granny), Uncle Aity (Sa-aid), and Hoosain Ally as individuals associated with the family.

[11] mentions that the Gool residence was one of the most prominent in Cape Town and that J.M.H. Gool had four children.

However, [1] lists the following 15 children of J.M.H. Gool:
- Abdul Hamid Gool
- Jane Gool-Tabata
- Mariam Margie Gool
- Minnie Gool
- Zobeida Gool
- Goolam Gool
- Fatima Gool
- Mariam Gool
- Mohamed Saaid Gool
- Ayesha Rassool
- Mohammed Hanief Gool
- Fatima Timmie Gool
- Zohra Abdurahman

It appears that there may be some discrepancy between the lists in [1] and [9], but both sources agree that Cissie Gool is a child of J.M.H. Gool, as stated in [9].

However, given the extensive list provided by [1], it is likely that this memoir's author, Yousuf (Joe) Rassool, refers to himself and his siblings listed in [1] when mentioning the children of J.M.H. Gool.

The correct list of children should be as follows:
- Abdul Hamid Gool
- Jane Gool-Tabata
- Mariam Margie Gool
- Minnie Gool
- Zobeida Gool
- Goolam Gool
- Fatima Gool
- Mariam Gool
- Mohamed Saaid Gool
- Ayesha Rassool (listed as Wahida's child in [1])
- Mohammed Hanief Gool
- Fatima Timmie Gool
- Zohra Abdurahman

### q25 — Who was I.B. Tabata?

I.B. Tabata was a key figure in the South African anti-apartheid movement, particularly within the Non-European Unity Movement (NEUM) and the New Era Fellowship (NEF). The following facts can be gleaned from the excerpts:

* He was a leader of the NEUM and the NEF (Excerpt [3], where he is mentioned as completing his book "The Awakening of a People" after becoming chairman of the NEF).
* He was involved in writing or editing publications, including "The Awakening of a People", which recorded the founding of the All African Convention (Excerpts [19] and [20]).
* He was associated with various events and meetings, such as the 1952 meeting to form SOYA (Student Organization of Youth Africans) (Excerpt [13]), where he explained the need for SOYA.
* His book "The Awakening of a People" was widely read and discussed by his followers, who accepted his interpretation of the movement's origins (Excerpts [19] and [20]).
* He was opposed to what he saw as nationalism, condemning it whenever suspected (Excerpt [16]).
* He clashed with Ben Kies, who was accused of having "personal differences" rather than political ones. Tabata attacked Kies for characterizing the conflict between the Capitalist West and the Soviet Bloc as a conflict between two camps (Excerpts [7], [14], and [20]).
* The Forum Club, a Marxist Trotskyist group, condemned his book "The Awakening of a People" for lacking class analysis (Excerpts [18] and [19]).
* He was also associated with various individuals, such as Victor Wessels (Excerpt [4]).

However, there is limited information on I.B. Tabata's personal background in the provided excerpts.

### q26 — Who was Dr. Abdullah Abdurahman?

From the sources:

* He was the first of his community to qualify as a doctor from Glasgow University in 1893 [3].
* His mother, Gadija Dollie, was said to be a most beautiful woman [3].
* In 1937, it was Cissie Gool who rocked the government when she called a massive demonstration against residential segregation, and Dr. Abdurahman was present at this meeting [4].
* He and my grandfather were present to hear the Progressive Party's candidates present their policies on January 9th, 1904, at St Paul's Mission School [4].
* There is a bond between him and my grandfather, which brought Cissie and my uncle Dr. into contact with Dr. Abdurahman [5].
* In 1937, it was Cissie Gool who offered a farewell address to Gandhi on behalf of Port Elizabeth Indians, and Dr. Abdurahman was involved in this delegation [6].
* He presided at the gathering where people were called to leave quietly after Mr. “Sonny” Abdurahman's call [9].

From these sources, we can infer that Dr. Abdullah Abdurahman was a prominent figure in his community, especially during the period around 1900-1940s. He was likely involved in various social and political activities, including advocacy for rights and equality for Indians and Coloured people in South Africa.

However, the provided sources do not contain detailed information about Dr. Abdurahman's background, family, or personal life beyond what is mentioned above.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources state a close relationship and frequent interactions between Gandhi and J.M.H. Gool:

* Gandhi stayed at 7 Buitencingle Street, Gool's residence, in February 1914 (Source [10]).
* Gandhi helped furbish my uncle’s surgery by puttying and staining the floor when he visited Cape Town in 1911 (Source [16]).
* J.M.H. Gool corresponded with Gandhi, as evident from a letter written to Gandhi on January 23rd, 1897 (Source [9]).
* Gandhi's son fell in love with one of Goolam Gool's sisters - a Moslem - but Gandhi refused to approve the marriage and instead married his son off to a Hindu girl (Source [3] and [13]).
* J.M.H. Gool attended meetings and events where Gandhi was present, such as the meeting of the Coloured People’s Vigilant Committee on May 3, 1906 (Source [8]).
* Gandhi's family stayed with the Gools during his visits to South Africa.

The connection between Gandhi and J.M.H. Gool was one of friendship and mutual respect, with frequent interactions and a strong bond between their families.

### q28 — Which organisations was the author involved in?

The provided sources list several organisations that the author, Yousuf (Joe) Rassool, was involved in. Here are the ones explicitly mentioned:

1. Teachers League of South Africa (TLSA)
2. New Era Fellowship (NEF)
3. Non-European Unity Movement
4. Anti-CAD (Anti-Coloured Affairs Department)

Additionally, some organisations were mentioned as participating or affiliated with the above-mentioned groups:

1. African People's Organisation (APO)
2. Communist Party of South Africa
3. Fourth International of South Africa
4. Liberation League
5. Workers' Party
6. Coloured branch of Smuts's United Party
7. All African Convention

The author also mentions being involved in various educational and cultural activities, such as playing a role in the school production of "Die Case about die Furrow" by Langenhoven, participating in the Hewat Amateur Theatrical Society, and being part of the Trafalgar Players.

Note that some organisations were mentioned as having participated or been involved with these groups (e.g., the Coloured branch of Smuts's United Party), but it is not clear if the author was directly involved in those specific organisations.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The TLSA (Teachers' League of South Africa) joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society [18]. This indicates a close relationship or alignment between the two organizations.

Additionally, [11] mentions that the TLSA approached the Congress movement suggesting the formulation of a unified strategy to counter the anti-educational plans of the regime. The fact that they were in communication and collaboration with the Non-European Unity Movement suggests a strong bond between them.

It's worth noting that [10] states: "The N.E.U.M. revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon." This implies that the Non-European Unity Movement was influential in shaping the TLSA's policies and actions, further indicating their close relationship.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The source [3] states that the party arrived in Cape Town in December, but it does not specify the year of arrival for J.M.H. Gool. However, based on the information provided in [4], we can deduce that J.M.H. Gool departed from Bombay (now Mumbai) in April or early May 1901, leaving Bibi and their son Mohamed Saaid behind.

Therefore, it is likely that J.M.H. Gool arrived in Cape Town sometime between December 1900 and the end of 1901. However, since the specific year of arrival for J.M.H. Gool is not mentioned, we cannot provide an exact answer to this question.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is described in [1] as an outpost of Islam in an area becoming increasingly European, Jewish, and gentile.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was one of the wives of Abdul Hamid Gool (also known as Dr. A. Gool), who is the son of J.M.H. Gool [9]. Therefore, Cissie Gool is the wife and not a direct relative of J.M.H. Gool.

However, if we look at the family relationships mentioned in the sources:

* J.M.H. Gool had two wives: Bibi Gool (whose family lived in Broach, Gujarat, India) and Wahida Ta'Al (from the Dollie family of Cape Town).
* Abdul Hamid Gool is one of the children from Wahida Ta'Al's side.
* Cissie Gool was married to Dr. A. Gool (Abdul Hamid Gool), who is J.M.H. Gool's son.

So, Cissie Gool was the wife and sister-in-law of Abdul Hamid Gool, making her also the daughter-in-law and niece of J.M.H. Gool [4].

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the provided sources [1]–[20], the following are notable historical figures mentioned as being known or associated with J.M.H. Gool:

1. Mahatma Gandhi: Gool was acquainted with Gandhi during Gandhi's 1914 visit (Source [2])
2. Dr. Abdurahman: Gool and his second wife Wahida Ta’al were related to Abdurahman through the Dollie clan, and their residences in Cape Town were close to each other (Source [18]). Abdurahman was a prominent community leader.
3. Adam H.G. Mahomed: Gool worked with Mahomed as part of British Justice and was described as an "Oriental trader" in court records. Mahomed is mentioned as Gool's senior, financier, and possibly mentor (Source [1] and [20]).

These individuals are notable historical figures who interacted or had associations with J.M.H. Gool based on the provided sources.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act, also known as the Group Areas Proclamation of 1950 (Proclamation R.82 of 1950), was a law passed by the South African government that aimed to segregate different racial groups into specific areas based on their ethnicity or nationality.

According to sources [3] and [5], this act affected District Six, a thriving cosmopolitan area in Cape Town, where people from various racial backgrounds lived together. The legislation played havoc with the lives of ordinary people in Cape Town, uprooting communities and ways of life.

Some key effects of the Group Areas Act on District Six include:

* Dispersal of communities: The law forced people to move out of their homes and neighborhoods, dispersing the community and breaking down its social fabric.
* Property reclassification: Properties were reclassified based on the racial groups they belonged to, which led to many owners facing financial losses due to lower valuations and reduced compensation.

The Group Areas Act was a significant contributor to the demolition of District Six in 1960s-1970s, as many residents were forcibly removed from their homes and relocated to other areas.

Sources:

* [3] "There are many things to learn from this book: First, in telling the story of his family, Joe Rassool provides us with not only a vivid story of growing up within the bosom of his extended family he also provides insight into the rich tapestry of the lives of the people who made District Six such a distinctive part of the city."
* [5] "The abject collapse of the Train Apartheid Resistance revealed the limitations of the boycott weapon as a means of struggle against Apartheid. The boycott, which was so potent in paralysing the Coloured Affairs Council, was totally ineffective in this regard."

### q35 — Who was Hassen Mall?

From the sources [1]–[20], here is what we know about Hassen Mall:

* He was a brilliant cricketer who spun a wristy leg break that zipped like a circular saw [16].
* He was a student from Durban studying medicine at Cape Town [17, 18, 19].
* He had completed his LLB degree before leaving for Durban in 1951 [5, 6].
* He was the chairman of the Western Province Indian Cricket Union and captain of the cricket team that played in the biennial tournament [12, 13].
* He was a friend and mentor to the author (Joe Rassool), who was deeply influenced by him and considered him a hero [17, 19].
* He had a clipped brush moustache and smoked a pipe [16].
* He was married with sons, whom the author's mother and Bibi met during Hassen Mall's visit [18].
* He introduced table tennis to the Allie family and taught it to the children [7].

These sources do not provide detailed information about his ancestry, family connections, or historical associations.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

Based on the provided sources [3], [7], [16] and [18], the following political organisations were active in the Cape Coloured community during the author's lifetime:

1. The New Era Fellowship
2. The African People’s Organisation (A.P.O.)
3. The Communist Party of South Africa
4. The Fourth International of South Africa
5. The Liberation League
6. The Workers’ Party
7. The Coloured branch of Smuts’s United Party

Additionally, the sources mention that various other organisations responded to the call for unity and full democratic rights, including:

1. Sports' clubs
2. The Moravian church
3. The New Era Fellowship (mentioned earlier)
4. Other host organisations (no specific names mentioned)

The Coloured Advisory Council (CAC) is also mentioned as a notable organisation, although its role in the community seems to be controversial.

Note: The provided sources do not mention all the political organisations that were active during this time period, but these are some of the ones specifically mentioned.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to [2] and [6], Mahatma Gandhi first arrived in South Africa in 1893 on a one-year assignment from Dada Abdullah, his attorney, to assist with a civil suit against a merchant in Pretoria. The accounts were kept in Gujarati, which was Gandhi's mother tongue.

The provided sources do not contain more information about why Gandhi was specifically sent to Pretoria or what the purpose of the suit was.

### q38 — Who was Cissie Gool's father?

The sources do not explicitly state who Cissie Gool's father was. However, it is mentioned in [18] that Dr. Gool's father was a well-known public worker, and in [20] that Dr. Abdulla Abdurahman was the son of Cape Town's other leading dynasty and married to Dr. Gool's wife, Cissie (or Zainunnisa). There is no direct connection made between Dr. Abdulla Abdurahman and Cissie Gool as her father, but it can be inferred that they were connected through marriage.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to [3], "To me those bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention." This suggests that the author remembers District Six being a vibrant and lively neighborhood.

In [2] it is mentioned: "Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way." This implies that Caledon Street was a major thoroughfare in the area.

[4] describes District Six as a "thriving cosmopolitan area" and notes that "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

[3] again: "To have been part of that pulsing life was an unforgettable education and privilege, the memory of which I would dearly like to rekindle in those who remember District Six in its heyday, and to provide a realistic picture to their children in the events leading to its demolition."

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in several excerpts. According to [11], they won an important battle using a simple tactic: ostracizing those who accepted service on the Coloured Affairs Council (C.A.C.). All those who accepted C.A.C. service were boycotted in every aspect of their lives, socially, economically, and otherwise.

In [16] and [18], it is mentioned that some members of the Movement ignored the injunction to boycott, especially when registering their protest against the Group Areas Act reclassification or voting in elections. However, the general policy was to boycott, as seen in [11].

The policy also involved using the boycott to paralyse institutions such as the Coloured Affairs Council (C.A.C.) ([8]), and it was effective in many instances, as mentioned in [6].

