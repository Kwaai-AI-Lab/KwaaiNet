# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 68.4% (151.8/222) |
| Avg latency | 36090ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21826ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 19964ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 20572ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 24793ms |
| q05 | Who was J.M.H. Gool? | 8/8 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 39243ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf | 27951ms |
| q07 | Who is the author's wife? | 2/3 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 22107ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 22335ms |
| q09 | Who was the author's grandfather? | 7/9 (78%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 26586ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | LEST WE FORGET -rev25.pdf | 28277ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 22178ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 35193ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 28374ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 24125ms |
| q15 | What were the forced removals from District Six? | 2/5 (40%) | sequence_diagram:District Six, LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 34102ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Gool family] | 26358ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 22096ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 22167ms |
| q19 | What was the Non-European Unity Movement? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 25833ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 26124ms |
| q21 | Who was the author's mother? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Ayesha Rassool] | 22170ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 23485ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 22080ms |
| q24 | Who were the children of J.M.H. Gool? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 26999ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 34398ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 43537ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 45428ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 53340ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/6 (67%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 65846ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 2.8/6 (47%) | [Graph: Haji Joosub Maulvi Hamid Gool], sequence_diagram:Haji Joosub Maulvi Hamid Gool, LEST WE FORGET -rev25.pdf | 50743ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 56152ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 54945ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 68071ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | [Graph: Chapter Twenty-One Leaving District Six], sequence_diagram:District Six, LEST WE FORGET -rev25.pdf | 58393ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 45079ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | [Graph: Cape Times], LEST WE FORGET -rev25.pdf | 50768ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 55762ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 42201ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Chapter Twenty-One Leaving District Six] | 52968ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 51038ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The author's children, as mentioned in [9] and [18], are:

1. Feyruz Rassool
2. Reza Rassool
3. Zarina Rassool

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph Rassool
2. Samuel Rassool
3. Alexander Rassool
4. Leonardo Rassool
5. Dylan Rassool
6. Lewis Rassool [18]

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis" [18].

### q05 — Who was J.M.H. Gool?

Based on the provided sources:

1. Haji Joosub Maulvi Hamid Gool [Person]
Also known as: J.M.H. Gool, J. M. H. Gool, J M H Gool, JMH Gool, J. Gool, Joosub Gool, Joosub Maulvi Hamid Gool, Joosub Maulvi Hamid.
Foster parent of: Hamid Ally, Gadija Dija Noor, Toetie Ally, Gadija Noor, Hanief Ally.
Founded: Hanaffi Quwatul Islam Mosque.
Grandchildren: Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf Rassool, Zain Rassool, Fazil Rassool, Yasmin Rassool.
Lived in: District Six, 7 Buitencingle Street.
Children: Abdul Hamid Gool, Jane Gool-Tabata, Mariam Margie Gool, Minnie Gool, Zobeida Gool, Goolam Gool, Fatima Gool, Mariam Gool, Mohamed Saaid Gool, Ayesha Rassool, Mohammed Hanief Gool, Fatima Timmie Gool, Zohra Abdurahman.
Married to: Bibi Gool, Wahida Gool.
arrived_cape_town: 1884.
deathDate: 1940-04-01.
origin: Mauritius (via Swat, Gujarat).

Haji Joosub Maulvi Hamid Gool was a spice merchant and community leader. He arrived in Cape Town in 1884 from Mauritius with family origins in Swat via Gujarat, India. He was the grandfather of Yousuf (Joe) Rassool, the author of this memoir.

Sources also mention him as:

* A founding trustee of the Hanaffi Quwatul Islam Mosque in District Six.
* A person of striking good looks and a tall stature.
* A well-respected citizen in Cape Town.
* The founder of J. M. H. Gool & Co., which became suppliers to the British Army's Indian troops during Queen Victoria's reign.
* A prominent figure in Cape Muslim and Indian political circles, associated with Mahatma Gandhi.

He had two wives, Bibi Gool and Wahida Gool, and a total of 14 children. He also performed the Hajj in 1899 and died on April 1, 1940.

### q06 — Tell me about Buitencingle.

Buitencingle is described in multiple excerpts as being located at No. 7 Buitencingle Street, a wide and cobbled street with traffic islands and stunted palm trees [18]. According to excerpt [20], the name "Buitencingle" has been pronounced differently (Baytencingle) by pursing one's lips, and "cingle" referred to a belt. At that time, Buitencingle was considered the outermost belt of Cape Town.

In excerpt [5] is an image description of Buitencingle from c1905, showing its grandeur and thriving trading community, with an advert by JMH Gool in Indian Opinion on February 25, 1905. 

Buitencingle had "a certain magic in the atmosphere" due to the perfume from shrubs, herbs, and flowers mingling with the smell of the rich moist earth [18]. The street was opposite a grey, austere German Lutheran Church that bustled into activity on Sunday mornings.

The excerpt also mentions Buitencingle's umpteen rooms being unmanageable without servants, implying it was a large house with many staff members [14].

Buitencingle is said to have been a vibrant and uplifting place in the past. It had fruit trees and gardens in the time when the narrator's family lived there.

### q07 — Who is the author's wife?

The provided sources do not explicitly state the author's full name. However, in excerpts [1] and [2], Nazima Rassool is mentioned as being married to Yousuf (Joe) Rassool, who is the author of "LEST WE FORGET -rev25.pdf". 

Therefore, based on this information, we can conclude that:

The author's wife is Nazima Rassool.

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about the author's wife. However, I can list specific facts that are mentioned:

* The author mentions his wife as a person who is an academic researcher [3], [14].
* In [17], it is stated that the author owes "a great deal" to his wife, Nazima, who insisted on research to lend authenticity to the story.
* There is no mention of the author's wife's name or any other details about her in the sources provided.

### q09 — Who was the author's grandfather?

The provided sources do not contain that information. However, from general knowledge, I can say that the author of "LEST WE FORGET -rev25.pdf" is Yousuf (Joe) Rassool and his grandfather is J.M.H. Gool or Joosub Maulvi Hamid Gool.

Here are some specific facts about Yousuf's grandfather:

* He was also known as Haji Joosub Maulvi Hamid Gool [1].
* His origin is Mauritius (via Swat, Gujarat) [1].
* He arrived in Cape Town in 1884 [1].
* He was a spice merchant and community leader who became a founding trustee of the Hanaffi Quwatul Islam Mosque in District Six [1].
* He had two wives: Bibi Gool and Wahida Gool [2,13].
* He performed the Hajj in 1899 [10,20].

Note that the information about Yousuf's grandfather is not directly provided by the sources but can be inferred from them.

### q10 — Tell me about Kloof Nek.

According to the sources [1], [3], and [17], Kloof Nek is a beautiful place described as a "haunting" and "place of haunting beauty". It is located in Cape Town, South Africa, and is situated at the connecting saddle of land that joins Table Mountain to Lion's Head.

Excerpt [2] mentions that Persotem Patel came from Worcester, which is about 80 miles away across the Drakenstein Mountains, but notes that a new pass through the mountains was being constructed, making it easier to travel to Cape Town.

Excerpt [5] describes Kloof Nek as "the Glen", which seemed entirely appropriate for reading poetry, particularly Gray's Elegy and Milton's Lycidas. It is also mentioned in excerpt [17] that when the narrator visited Kloof Nek with a friend, they were initially reluctant to go but were eventually entranced by its beauty.

Excerpt [20] provides more information about the geography of the area, noting that beyond Buitencingle lies the foothills of Table Mountain reached via Kloof Street.

### q11 — What was the Teachers League of South Africa (TLSA)?

The sources do not provide a detailed description of the Teachers League of South Africa (TLSA). However, it is mentioned that the TLSA opposed Apartheid and collaborated with the Cape African Teachers' Association [1]. The TLSA had conferences that were considered important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense [2, 20].

### q12 — Who was Cissie Gool?

Cissie Gool is a person mentioned in various excerpts. Here are specific facts about her:

1. Associated with: Non-European Unity Movement [1].
2. Lived in: District Six [1].
3. Married to: Abdul Hamid (AH) Gool, a doctor and activist [7, 11].
4. Her husband's medical articles were published in the Indian Opinion [8].
5. Had disagreements with her brother-in-law Dr. Gool about politics and their cause [7].
6. Was part of the platform at a meeting where thousands of White ex-soldiers were present [3].
7. Addressed a meeting at the Banqueting Hall, supporting measures to deal with 'influx control' and street gangs [14, 16].
8. Led a massive demonstration against residential segregation in 1937 [18].
9. Was part of the family that was beginning to show signs of affluence around the time her politics were changing [19].

She had children: Margie, Timmie, and Shaheen (son) with Abdul Hamid Gool.

The provided sources do not contain detailed biographical information about Cissie Gool's early life, ancestry, or exact dates.

### q13 — What was the All Africa Convention?

The sources do not provide a comprehensive description of the All African Convention. However, we can gather some information from various excerpts.

[2] mentions that Vic Wessels, Les Jacobs, and the author attended a conference of the All African Convention in Bloemfontein in 1954.
[19] lists Enver Hassim, Errol Vawda, and Kader Essack as members from Natal who attended the All African Convention.
[20] also mentions Tsotsi, Honono, Sihlali, Enver Hassim, Errol Vawda, and Kader Essack as leaders or members of the All African Convention.

From [3], we know that Mr. I.B. Tabata completed his book "The Awakening of a People," which recorded the coming into existence of the All African Convention, an organization that was one of the pillars of the Non-European Unity Movement.

Overall, it appears that the All African Convention was an important organization involved in the Non-European Unity Movement, but more specific information about its goals, structure, and activities is not provided.

### q14 — Where was District Six and what kind of place was it?

District Six was a thriving cosmopolitan area located in the heart of Cape Town, South Africa. According to source [10], "a thriving cosmopolitan area in the heart of Cape Town" and source [16] describes it as a "vibrant community". It is also mentioned in source [20] that District Six was once a desirable place with more life, zest, fire, warmth, and care than many other places.

District Six was home to people from various racial groups, including Indians and Coloureds. The Gool family's mansion at 7 Buitencingle (source [7]) was located in District Six, suggesting that it was a relatively affluent area.

The community had a rich history of politics and activism, with notable figures such as Ahmed Ismail (Piccadilly) holding political meetings and gatherings there (source [17]). The area also had a strong cultural scene, with the Trafalgar Players theatre company performing in the area (source [11]).

However, District Six was eventually demolished by the government due to apartheid policies, which aimed to separate people based on their racial backgrounds.

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information about the forced removals from District Six. However, based on general knowledge and context, it is known that the forced removals from District Six occurred in 1968-1970, as part of the apartheid government's efforts to eradicate non-white communities from urban areas and create separate, racially homogeneous neighborhoods.

The sources do mention that Yousuf Rassool was born in Cape Town (source [9]) and that he grew up in District Six. They also mention his experiences with the Teachers League of South Africa (TLOS6) and the Non-European Unity Movement (NUM7), which were organizations involved in the fight against apartheid.

The forced removals from District Six are a significant event in South African history, and their impact is still felt today. The removals led to the displacement of over 60,000 people, mostly of Indian and coloured descent, who were forcibly relocated to other areas on the outskirts of Cape Town.

It's worth noting that the sources provided focus more on personal experiences and anecdotes rather than historical events or specific details about the forced removals.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi was a key figure in the fight against anti-Indian legislation in South Africa. He extended his contacts in the community, particularly among the poorer sections, and devoted much of his time to public work for Indians.

The Gool family was central to Gandhi's connections in Cape Town and the wider Indian community. Specifically:

* The Gool family provided medical care to Gandhi.
* Dr. J.M.H. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion.
* Gandhi stayed at 7 Buitencingle Street, which belonged to the Gool family, on multiple occasions (1911, February-March 1914).
* The Gool family attended functions in honor of Gandhi, such as a function before his departure from Cape Town in March 1914.
* There is evidence of correspondence between Gandhi and Dr. Abdul Hamid Gool while he was a medical student at Guy's Hospital in London.

Gandhi's relationship with the Gool family is also reflected in the following specific events:

* In 1896, Gandhi stayed with J.M.H. Gool at his residence (7 Buitencingle Street) and helped puttying and staining the floor of Dr. Gool's surgery.
* In February 1914, Gandhi visited Buitencingle yet again for medical care from Dr. Gool.

Source citations:

[2], [3], [5], [9], [12], [13], [17], [18]

### q17 — What was Hewat Training College?

Hewat Training College was a post-matriculation teacher training facility for Coloureds in South Africa. It was located opposite the Roeland Street Gaol in Cape Town and was considered too close to the prison and District Six by the authorities. The college initially had undeveloped grounds, which may have been due to being dug up after the school it replaced was closed to prevent children from using it as a playground.

Hewat Training College trained teachers for the upper years of the primary division, but the author, Joe Rassool, started teaching at Habibia Institute, where he was supposed to teach 3rd year primary school pupils instead. [15]

The college had a staff that was chosen to ensure that it would not be an incubator of left-wing radical ideas like Livingston and Trafalgar High schools. The authorities were determined that Hewat would not develop such a philosophy, which is evident from the fact that some staff members, like Hogwood and Borchers, taught in a way that emphasized conservative educational philosophy. [10]

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a political-cultural organization associated with the Unity Movement. It was mentioned in several excerpts as being involved in various activities such as lectures, meetings, and rallies.

Here are some specific facts about the NEF:

* It was established to discuss various issues under the sun (Excerpt [19]).

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization that brought together people from different racial groups to fight against apartheid and segregation in South Africa. It was founded in 1943 by a group of young activists, including Victor Wessels and I.B. Tabata.

According to the sources [1], [2], [6], [8], and [11], the NEUM aimed to create a non-racial society where people of all races could work together to achieve equality and freedom. It was influenced by the ideas of non-violence and non-collaboration, and it used the boycott as a weapon against apartheid.

The movement had various leaders and members, including Aunt Jane, Dr. Goolam, Victor Wessels, I.B. Tabata, Mr. Tsotsi, Honono, Sihlali, Enver Hassim, Errol Vawda, Kader Essack, and Cissie Gool [1], [2], [6], [8], [11].

The NEUM had a 10 Point Programme that included demands for full franchise (the right to elect and be elected), free and equal education, and non-racial society. The movement also recognized the importance of literature in the struggle against apartheid and encouraged writers like Richard Rive and Alex La Guma to contribute their work [13].

Although the NEUM faced challenges and setbacks, it played an important role in shaping the anti-apartheid movement in South Africa.

Note: The Non-European Unity Movement is mentioned throughout the provided sources.

### q20 — Describe the author's involvement in cricket.

The author had a significant involvement in cricket. Here are some specific facts about his cricketing career and interests:

* He was part of a non-racial Sunday league team called Kismets, which played against other teams in the Western Province Indian Cricket Union [2].
* The author mentions that he captained the side for the biennial cricket tournament, along with Hassen Mall [9].
* He had the chance to play against top players such as Len Hutton and Cyril Washbrook, and was excited to watch them at Newlands Cricket Grounds [15].
* The author expresses his joy when selected for the Western Province Indian Cricket Union side for the biennial tournament, but unfortunately did not make it to the final team that went on tour to Johannesburg [13].
* He credits Hassen Mall with teaching him how to play cricket seriously and mentions learning to master the forward defensive stroke under Edross's guidance [7].
* The author also participated in organizing a union to facilitate playing cricket at High School and College level, but was thwarted by bureaucracy [10].

These facts suggest that the author had a strong interest in cricket, played for several teams, and was involved in the administration of the sport.

### q21 — Who was the author's mother?

The provided sources do not contain a direct reference to the author's mother's name. However, [10] mentions "my mother" and [20] refers to the conversation between the author and their mother about atheism. 

To answer the question based on the available information:

- According to [9], the family had become foster parents to children after the death of their mother.
- The mention of a mother in [10] is a flashback, referring back to an earlier event.

There are no other specific details about the author's biological mother.

### q22 — Who was the author's father?

The provided sources do not contain information about the author's mother. However, according to [1], Peter Alexander Rassool (the author) lived in District Six and had several children: Rasheda Rassool, Abdul Rassool, Berina Rassool, Nasim Rassool, Yousuf Rassool, Zain Rassool, Fazil Rassool, Yasmin Rassool. His wife's name was Ayesha Rassool.

According to [4], the author's father was handsome and had a thorough Christian upbringing. He left school early, became apprenticed to a cobbler, and worked for his grandfather's firm due to his neat handwriting.

The surname "Rassool" appears in multiple sources, suggesting it might be related to the family of the author.

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin Rassool

Note that "Yasmin" and "Berina" are sisters, as mentioned in excerpt [4].

### q24 — Who were the children of J.M.H. Gool?

The provided sources do not give an exhaustive list of all the children of J.M.H. Gool, but they mention several:

* Abdul Hamid Gool [1], born 1886
* Jane Gool-Tabata [1]
* Mariam Margie Gool [1]
* Minnie Gool [1]
* Zobeida Gool [1]
* Goolam Gool [7]
* Fatima Gool [1, 10]
* Mohamed Saaid Gool [1]
* Ayesha Rassool (note: her father is listed as J.M.H. Gool in excerpt [1], but she may have been born out of wedlock or through one of his two marriages)

### q25 — Who was I.B. Tabata?

According to the sources:

* I.B. Tabata completed his book "The Awakening of a People" [3], [19].
* He was the chairman of the New Era Fellowship [4], [13], [15], and was involved in the formation of SOYA as a training ground for young Africans [13].
* He was known to have been qualified as BA LLB, which contributed to his swift promotion in the Movement [14].
* He delivered a talk at the NEF on "One Day in the Life of Ivan Denisovich" and became a member of the inner sanctum of the National Anti-CAD [14].
* He wrote an indictment against Kies for his characterization of the conflict between the Capitalist West and the Soviet Bloc as a conflict between two camps, which was seen as undialectical code language that was un-Marxist [1].
* Tabata was involved in a split in the Movement with Ben Kies [4], [14].
* He was accused by The Forum Club of failing to have a class analysis in his historical tract "The Awakening", and being a Black Nationalist, one of the worst forms of political denigration at the time [18].

Tabata's involvement in various organizations, events, and publications indicates that he was a prominent figure in the Unity Movement.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the excerpts, here are specific facts about Dr. Abdullah Abdurahman:

1. He qualified as a doctor from Glasgow University in 1893 [3].
2. His mother, Gadija Dollie, was said to be a most beautiful woman and belonged to the Dollie family [3].
3. The Abdurahman clan and the Ta'Als were linked through the Dollie family [3].
4. He met and married Helen "Nellie" Potter James in Glasgow University [3].
5. He was a leading figure in Cape Town's Indian community.
6. He was one of the first non-Europeans to qualify as a physician from South Africa [16].
7. He studied at Guys Hospital, London [16].
8. He married Cissie (Zainunnisa) Abdurahman, daughter of Dr. Abdulla Abdurahman and granddaughter of Gadija Dollie [16].
9. His grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community [18].

Dr. Abdullah Abdurahman's family connections and achievements indicate that he was an influential figure in Cape Town's Indian community during the late 19th and early 20th centuries.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

Gandhi and J.M.H. Gool had a close connection. They were friends and corresponded with each other frequently. 

[3] states that Gandhi's correspondence with uncle Abdul Hamid Gool, while he was a medical student in London, is recorded.

[9] mentions a letter from J.M.H. Gool to M.K. Gandhi on 23rd January 1897.

[10] refers to the hospitality provided by the Gool family to Gandhis in February 1914.

[11] notes that Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion and other scholarly journals.

Gandhi visited J.M.H.'s residence, 7 Buitencingle Street, on several occasions, including:

- In 1911, when he helped furbish the uncle's surgery by puttying and staining the floor. [7]
- In October 1912, when Gandhi was a guest at 7 Buitencingle Street to receive G.H. Gokhale. [11]

J.M.H. Gool also attended meetings with Gandhi, such as:

- A meeting of the British Indian League in June 1906. [16]

The connection between J.M.H. Gool and Gandhi went beyond friendship; Gandhi was a frequent guest at their home and considered them close friends.

### q28 — Which organisations was the author involved in?

The provided sources do not contain a comprehensive list of all the organizations the author was involved in. However, the following organizations are mentioned:

1. New Era Fellowship [2, 3, 11]
2. Non-European Unity Movement (implied by "the Movement" [4])
3. Teachers League of South Africa [4] 
4. Cape Peninsula Students Union [2]
5. All African Convention [6]
6. Coloured Advisory Council (implied by the Anti-CAD in [13])
7. Communist Party of South Africa [17]
8. Fourth International of South Africa [17]
9. Liberation League [17]
10. Workers’ Party [17]
11. Smuts’s United Party, Coloured branch [17]

Note that some organizations may be mentioned indirectly or implied through context.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources state that:

* The TLSA (Teachers' League of South Africa) approached the Congress movement suggesting a unified strategy to counter the anti-educational plans of the regime [11].
* The Congress leadership declined the invitation to cooperate, and did not even prepare to boycott the Eiselen Commission’s 'fact-finding' assignment.
* However, the TLSA was closely associated with the Non-European Unity Movement, as indicated by the fact that Ben Kies was editor of the TLSA Journal and a member of the Unity Movement [20].
* The Teachers' League of South Africa joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society [18].

It appears that the TLSA had close ties to the Non-European Unity Movement, but the exact nature of their relationship is not fully clear from these sources.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information on when or from where J.M.H. Gool arrived in Cape Town. However, they mention that Haji Joosub Maulvi Hamid Gool's party (which includes his wife Bibi Gool) arrived in Cape Town in December, and that his first son was born in 1886 [1]. 

We also know from source [3] that a cousin of the family, Nasima, was born in Mauritius in September, but this is not directly related to J.M.H. Gool's arrival.

The origins of Haji Joosub Maulvi Hamid Gool are mentioned as having been brought to South Africa as a slave by his grandfather [2].

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is an outpost of Islam in an area becoming increasingly European, Jewish, and gentile. [1] It was located in Loop Street, Cape Town, South Africa. [20] According to the source, it was a whitewashed structure with a domed minaret, topped by a spire with a moon and star, from which the muezzin called the faithful to prayer. The author's grandfather was a founder and life trustee of the mosque, which was completed in 1898. [20]

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was J.M.H. Gool's wife (source [1]). 

This memoir by Yousuf (Joe) Rassool mentions Cissie Gool as the mother of his cousin Shaheen, indicating that she was also J.M.H. Gool's second wife (source [13]).

Therefore, Cissie Gool was J.M.H. Gool's second wife and the mother of Abdul Hamid Gool (one of J.M.H.'s sons from a different wife) and Shaheen (J.M.H.'s son with Cissie).

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources do not contain a comprehensive list of all the notable historical figures that J.M.H. Gool knew personally. However, based on the information available:

* Joosub Maulvi Hamid Gool was acquainted with Mahatma Gandhi during Gandhi's 1914 visit [1].
* J.M.H. Gool is mentioned as being associated with early Cape Muslim and Indian political circles [1].

Additionally, some notable figures are mentioned in the same context or era:

* Dr. Abdurahman’s grandfather, a slave who bought his freedom and became prominent in the community [18], was a neighbor and possibly a friend of J.M.H. Gool.
* Adam H.G. Mahomed, a financier and possible mentor to J.M.H. Gool [16, 20].

Other individuals are mentioned as part of the context or era:

* Dr. Abdurahman, who is related to J.M.H. Gool through marriage [18].
* Cissie Gool (possibly his daughter), who is mentioned in a photograph with other family members [17].

### q34 — What was the Group Areas Act and how did it affect District Six?

According to excerpts [2]–[8], the Group Areas Act was a piece of legislation that divided South Africa into separate areas for different racial groups, with the goal of maintaining apartheid. The act was introduced in 1950 and had a devastating impact on District Six, a thriving cosmopolitan area in Cape Town.

The act led to the forced removals of people from District Six, which was a mixed community with residents from various racial backgrounds. The government sought to reclassify the area as a White-only zone, displacing its residents and destroying their homes and way of life.

Excerpt [2] states that "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Excerpt [3] notes that while some members of the "Movement" ignored the injunction to boycott the Group Areas Act, those who owned property were faced with a different situation. The act was seen as an "essential iniquity" by many, including Senator Edgar Brookes, who proposed a motion of opposition to the entire apartheid system.

Excerpt [6] states that liberals like Brookes made their protests against the Group Areas Act, but ultimately it died a "stillborn death." As a result, the act was implemented, leading to the devastation of hundreds of thousands of unfranchised people in South Africa.

The impact of the Group Areas Act on District Six is described as decimating the community and way of life. Excerpt [2] states that the area was dispersed across the Cape Flats, leaving behind a once-thriving community.

### q35 — Who was Hassen Mall?

Based on the provided sources:

- Hassen Mall was a student from Durban who came to Cape Town to study medicine [16] and [18].
- He was a brilliant cricketer who spun a wristy leg break that zipped like a circular saw [16].
- In due course, he qualified with a degree BA LLb 44. [16]
- He had a clipped brush moustache, smoked a pipe, and radiated a kind of magnetism that could not be denied [16].
- He was a close friend of the author and profoundly affected his life in the years that followed [18] and [19].
- He captained the side in the biennial cricket tournament at Durban's Curry's Fountain in early January. 
Hassen Mall had left for Durban after completing his LLB degree [5], [6], and [20].

He is also mentioned in other excerpts as Hassen, who was a member of The Indian Congress with Dawood [1].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to the provided sources, several political organisations were active in the Cape Coloured community during the author's lifetime. Some of these include:

* The New Era Fellowship (founded in 1937) [15]
* The African People’s Organisation (A.P.O.) [15]
* The Moravian church [15]
* The Communist Party of South Africa [15, 16]
* The Fourth International of South Africa [15]
* The Liberation League [15]
* The Workers’ Party [15]
* The Coloured branch of Smuts's United Party [15]

Additionally, the sources mention various other organisations that were mentioned as being active in the community or involved with the author and his family, but their primary focus is not explicitly stated. These include:

* The Coloured People’s Vigilance Committee (mentioned in relation to J.M.H. Wilson) [4]
* The Teachers’ League of South Africa [16]

Note that some organisations mentioned in earlier sources may no longer be active during the author's lifetime, while others may have undergone changes or rebranding.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was a prominent Indian independence activist who is widely regarded as one of the key figures in the struggle for India's independence from British rule. He is today known as Mahatma Gandhi, which means "Great Soul" in Hindi.

Gandhi arrived in South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria [6]. The accounts were kept in Gujarati, which was Gandhi's mother tongue. He was sent for because the accounts were kept in Gujarati, which was Gandhi’s mother tongue [6].

In 1914, Gandhi visited Buitencingle again (February-March), and his wife Kasturba had become very ill and was taken care of by the Gools [15].

### q38 — Who was Cissie Gool's father?

The sources do not provide information about Cissie Gool's father. However, [16] mentions Dr. Abdurahman's mother, Gadija Dollie, being a beautiful woman and a link to the Ta'Als through the Dollie family. It is implied that she is related to Dr. Abdurahman.

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources do not contain a comprehensive description of what District Six was like as a neighborhood. However, they mention that it was a "thriving cosmopolitan area" (source [17]) with a rich history and culture. They also describe it as having a sense of community and warmth, where people came together to socialize and live their lives.

Source [18] describes the neighborhood as having "more life, more zest, more fire, more warmth and care than many other places one could mention." It also mentions that the children who grew up there would have had an "unforgettable education and privilege" from being part of this community.

Additionally, source [19] notes that District Six was a popular area with a mix of different housing types, including some elegant cottage-style houses. However, it also mentions that after World War II, sub-economic housing schemes were built in the suburbs, which would eventually lead to the forced removals of residents from District Six.

It is worth noting that while these sources provide some insight into what District Six was like before the forced removals, they do not offer a comprehensive or detailed description of the neighborhood's layout, architecture, and daily life.

### q40 — What was the Unity Movement's boycott policy?

The sources do not provide a detailed description of the Unity Movement's boycott policy. However, [5] mentions that "the Party began pressing for the addition of two Liberals to the committee" and when that was defeated, they walked out. This suggests that there were disagreements within the Unity Movement regarding their policies.

[6] states that boycotting the Group Areas Act did not create any problems, but to those who owned property it was a different matter. Some members of the "Movement" simply ignored the injunction to boycott.

[8] mentions that the abject collapse of the Train Apartheid Resistance revealed the limitations of the boycott weapon as a means of struggle against Apartheid.

[11] describes how all those who accepted service on the C.A.C. were boycotted in every aspect of their lives, socially, economically, and otherwise.

[16] mentions that Cissie Gool, one time stalwart of the Anti-CAD Movement, had irrevocably signalled her departure from the politics of the Unity Movement after she spoke in support of measures to deal with 'influx control' and the street gangs.

From these excerpts, it appears that the boycott policy of the Unity Movement was not consistently enforced within the organization.

