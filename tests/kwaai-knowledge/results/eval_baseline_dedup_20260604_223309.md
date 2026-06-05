# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 53.3% (120/225) |
| Avg latency | 29993ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 30950ms |
| q02 | Who are the author's children? | 0/3 (0%) | LEST WE FORGET -rev25.pdf | 34973ms |
| q03 | Who are the author's grandchildren? | 0/6 (0%) | LEST WE FORGET -rev25.pdf | 33009ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 27790ms |
| q05 | Who was J.M.H. Gool? | 3/8 (38%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Gool AH Gool] | 38899ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf | 32818ms |
| q07 | Who is the author's wife? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 24395ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 24041ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 21415ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 27692ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 28805ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 33384ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 57826ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 39469ms |
| q15 | What were the forced removals from District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 40981ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/7 (43%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 36882ms |
| q17 | What was Hewat Training College? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 38242ms |
| q18 | What was the New Era Fellowship? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 35914ms |
| q19 | What was the Non-European Unity Movement? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 33926ms |
| q20 | Describe the author's involvement in cricket. | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 25058ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 25190ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 26869ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 22057ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Gool AH Gool] | 25956ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 25273ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 33035ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Gool AH Gool] | 30502ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 24876ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 26663ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3/6 (50%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 25399ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 30684ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 23441ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 27648ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 27083ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf, [Graph: H. Mall] | 25629ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 29532ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 4/7 (57%) | LEST WE FORGET -rev25.pdf | 28125ms |
| q38 | Who was Cissie Gool's father? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 23121ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29355ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22814ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [4].

### q02 — Who are the author's children?

The provided sources mention that the author, Yousuf (Joe) Rassool, has three children: 

1. A boy named Joseph 
2. A boy named Samuel 
3. Two boys named Leonardo and Alexander

### q03 — Who are the author's grandchildren?

The question is not clearly answered in the source excerpts. Specifically:

* We can only guess because we do not see their names written in the excerpts.
 

However, it can be inferred that the book "District Six: Lest We Forget" was not written by one of Joe Rassool's children (as the name of his child is Yousuf, [3] and he has 12 kids), but rather to remember the lives of their grandkids, as indicated in [1].

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [1]/[11]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool (also spelled Goel or Gol) appears in about half of the source excerpts.

From these references, we can infer that:

* He was an Indian national [1].
* His surname "Gool" means 'flower', specifically a rose, and originates from the Pathan region near Afghanistan [5], [ii].
* He had multiple names, including Joseph Gool, Yusuf Gool, J.H. Gool, and others [9], [10], [17].
* His second name is sometimes listed as Molve, Moulvi or Maulvi (an honorific title for a Muslim scholar who has spread the religion to other lands) rather than Mohamed [5].
* He was an 'Oriental trader' described in some early documents [3].
* In 1886, he had a son named A.H. Gool, born in [17]ii.
* He was involved in British Justice and had standing in the Cape community [3], [6].
* According to his testimony, recorded in Supreme Court transcripts in 1892, he was part of a case with Adam H.G. Mahomed [6].
* In 1897, he wrote to M.K. Gandhi regarding various issues affecting the Indian community, including their rights and treatment in South Africa [15], [22].

Gool's relationships with prominent figures are also mentioned:

* He hosted Gandhi at his home (7 Buitencingle) on two separate occasions in February 1914, when Gandhi was involved in the satyagraha movement; one visit included a gathering of European women and others who met Gandhi there [11].
* The relationship between J.M.H. Gool's family and Dr. Abdurahman's family suggests that they collaborated or had similar goals in their public lives.

The lack of information about parts of his life makes it difficult to know other specific details, but from the available data we can conclude:

* He was Indian and Muslim.
* His occupation involved business and he interacted with several notable figures of his time.
* Gool may have been involved in politics beyond just public speaking since he participated at a meeting where Dr. Abdurahman was present [20].
* According to Joe Rassool [18], J.M.H. Gool's world view was constructed around the vision of non-racial democracy under the British Empire rather than Islamic tenets; however, no direct evidence is provided in this question set to support or contradict that claim.
 
The sources also mention related figures in his life:

* Adam H.G. Mahomed acted as J.M.H. Gool’s financier and possibly mentor [6].
* Cissie Gool was Dr. Gool's wife; she showed opposition to the South African government when it implemented residential segregation policies through her massive demonstration of thousands at Grand Parade.
* His first son, A.H. Gool, wrote two medical articles for Indian Opinion [11].

### q06 — Tell me about Buitencingle.

The provided sources do not contain a comprehensive description of Buitencingle. However, the following information can be gleaned:

Buitencingle is mentioned as a street in Cape Town where some of the Rassool family lived (Source [1]). The name "Buitencingle" is described as being of Dutch derivation and meaning a belt or cordon (Source [15]).

Buitencingle was once the outermost belt of Cape Town, beyond which lay the foothills of Table Mountain (Source [15]).
The street's location is near Kloof Street, which led from the bottom of Buitencingle up to Kloof Nek, a bus terminus (Source [8]).

Some buildings associated with Buitencingly include:

* The Gool mansion at 7 Buitencingle, where Gandhi stayed in 1914 and was received by Prime Minister Botha and other dignitaries (Sources [3], [12]), [15] and [19]).
*The Hanaffi Quwatul-Islam Mosque, an outpost of Islam in an increasingly European area, which stood in Loop Street near Buitencingle (Source [8]).

Buitencingly was a significant location for the Rassool family's life, with many events described as taking place there.

### q07 — Who is the author's wife?

According to excerpt [1], the author's wife is Nazima Rassool. According to [17] and additional excerpts (not separately numbered here), one other source also confirms that her name is Nazima. According to [1], she is "an academic researcher" who urged the author to ground his memoir in historical research.

### q08 — Tell me more about the author's wife.

The author mentions his wife, Nazima Rassool, on multiple occasions in the excerpts [1], [5], [6], and [20].

According to excerpt [1], Nazima Rassool is an academic researcher who urged her husband to ground his memoirs in historical research, emphasizing the importance of scholarly rigor.

In excerpt [2], it is not mentioned again. However, 
the author mentions that he has the "ability  to  recount  events  going  back  to  the  very  earliest  days  of  my  life  with  ease  and considerable clarity".

Excerpt [6] indicates the author's gratitude for his wife's academic background when he undertook his project: "I owe a great deal to my wife, who is an academic researcher..."

Lastly, in excerpt [20], Nazima Rassool again urges her husband to do research rather than rely solely on his memories. 

The sources do not provide any other information about the author's wife beyond these instances.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool [1, 6, 9, 12].

### q10 — Tell me about Kloof Nek.

According to the excerpts [1] and [18], Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion's Head. It sprawled protectively on the eastern flank of Cape Town, with its head gazing inscrutably down on Table Bay. The area was situated beyond Buitencingle (also spelled as Byootencingle or Baytencingle), which was the outermost belt of Cape Town.

In excerpt [17], it is described that Kloof Nek was a place of haunting beauty, where the author and his friend would often visit during summer. The area was frequented by lovers, mountaineers, nature lovers, and ramblers, but also had its fair share of down-and-outs who found refuge in nearby cavelets and bushes.

Additionally, excerpts [1] and [19] mention that Kloof Nek was the location of a bus terminus for the trackless tram, and it was from this point that one could ascend to Table Mountain via the cable car.

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources do not contain a comprehensive description of the Teachers' League of South Africa (TLSA). However, from various excerpts, we can infer that the TLSA played an important role in promoting democracy and equal rights for non-Whites in South Africa.

Excerpts [1], [3], [5], [7], [9] mention the TLSA as an organization active during this time. Excerpt [8] states that Edgar Maurice, a Vice President of the TLSA, delivered a well-researched and scholarly address on education for non-Whites in South Africa.

Excerpts [14] contain details about students who were part of the TLSA branch in their school. They worked to achieve the organization's goals and participated in events like a Van Riebeeck festival meeting.

Excerpt [21] lists several characters, including Sol Plaatje and Solly Edross, as members of the Teachers’ League of South Africa. 

The TLSA seems to have been a significant organization during this time in promoting equal rights and democratic ideals for non-Whites in South Africa. However, details about its full scope, goals, and operations are not provided.

It is worth noting that Excerpt [3] lists "Unity Movement" next to the list of some TLSA members' names; it could indicate a close relationship or connection between these two organizations.

### q12 — Who was Cissie Gool?

Cissie Gool's identity is partially confirmed by multiple sources [7], [11], and [14]. According to [6] her name and the context confirm she participated in protests, political rallies, and addressed meetings.

A description of Cissie Gool appears in [1] as Aunt Cissie, in [5] but a possible relation is not made, in [8] but it's unclear if this is the same person [2], and her appearance in public rallies [6].

The provided sources do not contain any direct confirmations of Cissie Gool's full name.

### q13 — What was the All Africa Convention?

The provided sources do not explicitly define what the All Africa Convention is, but they mention it as a significant organization in several excerpts. 

[12] mentions: "A Meeting of the All African Convention was held at Auntie Minnie's home in District Six, where I saw some of the leaders of the All African Convention- Messrs. Tsotsi, Honono and Sihlali..."

[13] discusses a rehabilitation scheme being proposed, but notes that the regime's answer to land hunger among blacks is not to provide more land, "but to have the cattle and other livestock culled, contending that the reserves were overpopulated."

[19] explains: "The book recorded the coming into existence of the All African Convention, the organisation that was one of the pillars of the Non-European Unity Movement."

Other sources mention its importance indirectly by discussing related events and organizations such as the Anti-Coloured Affairs Department and the Teachers' League of South Africa. The specific goal and mission of the All Africa Convention are not provided in these excerpts.

Further research would be required to gain a deeper understanding of the All Africa Convention, including its founding, purpose, goals, and impact on society of that time.

### q14 — Where was District Six and what kind of place was it?

According to excerpts [14] and [13], District Six was a thriving cosmopolitan area in the heart of Cape Town, South Africa.

Excerpt [15] describes it as "a black spot" in the Apartheid scheme of things that would soon be bulldozed out of existence. Excerpt [14] notes that District Six had a rich history of political struggle against the iniquities of the Apartheid regime, and emphasizes that it was not a community defined by silence and acquiescence, but rather one with a strong sense of self-expression.

Excerpts [1]-[3] also provide vivid descriptions of life in District Six. For example, excerpt [2] talks about passing through Old Chapel Street, which had two churches, Methodist and Catholic, but mentions that the narrator was encouraged by friends and relations to avoid even going for a "hurried look", suggesting that his community emphasized Islamic observances. Excerpt [3] describes the house where Zeenith (Joe's cousin) lived with her seven siblings and their mother in a cramped one-bedroomed house in Reform Street, and mentions the various shops and businesses including Hindu Hall near their old home.

Excerpts [18] and [19] mention that District Six originally had more affluent housing estates built to entice people to move away from it, but later sub-economic schemes like Silvertown were built instead, leading to displacement and erasure of the community in the 1950s under the Group Areas Act.

The narrative collectively portrays District Six as a vibrant mixed-ethnicity neighborhood with strong social connections, communal spaces (like shops and markets), rich cultural diversity, but also oppressive circumstances.

### q15 — What were the forced removals from District Six?

The sources do not contain any explicit mention of the "forced removals from District Six" as a single event or policy. However, Excerpt [11] mentions that:

"...the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community of District Six..."

Excerpts [10, 12, and 13], while not directly stating "forced removals," describe the displacement of residents from District Six due to apartheid-era legislation. Excerpt [10] mentions:

"...the Group Areas Act (1950)... dispersed its people across the barren sandhills of the Cape Flats..."

Excerpts [11] says:
 
"Apartheid policy celebrated white racial supremacy... It legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life."

Excerpt [13] mentions that "District Six would soon be bulldozed out of existence."

### q16 — Who was Gandhi and what was his connection to the Gool family?

The provided sources do not contain extensive information on who Gandhi was. However, they describe him as Mahatma Gandhi or M.K. Gandhi [10] and [13], and mention that he had come to South Africa in 1893. A few details about his connections with the Gool family are provided, but these require synthesizing from various excerpts.

Sources indicate that Gandhi was a guest at the residence of the Gools on several occasions [5]. It is mentioned that Dr. Gool submitted subscription dues for the Indian Opinion, presumably on behalf of Gandhi or as requested by him [1]. The sources suggest that Gandhi interacted with multiple members of the Gool family but do not specify whether they were biological relatives [4].

As a result, some of the key points about Gandhi's connections to the Gools are:

- Gandhi received warm hospitality from the Gools during his visits.
- Dr. Gool read an address of welcome on behalf of the community when Gokhale visited South Africa and was a guest at Adam Gool's residence (Noor Bagh) [10].
- There is evidence that Abdul Hamid Gool's correspondence with Gandhi while he was a medical student in London implies a close relationship between them [16].
- Gandhi referred to Dr. A.H. Gool as 'Mahatma' - Great Spirit, as recorded in an article by an Indian Opinion and a speech delivered by him on behalf of Port Elizabeth Indians [13].
- The article suggests that Gandhi likely met some members of the Gool family when he interacted with Abdul Hamid Gool or Adam Gool.
- Further information about Gandhi's exact relationship to these members or his frequent visits is absent from the sources.

This indicates that Gandhi maintained connections and a close relationship, not necessarily through blood ties, but one where they supported each other in various ways, often due to their shared interests in community service and social justice.

### q17 — What was Hewat Training College?

The excerpt [1] mentions that Hewat Training College "had once been a Whites only Primary school, but the authorities considered it too close to the prison and, more to the point, too close to District Six. It is hard to explain why the school...". However, this statement seems incomplete and does not provide further information about what Hewat Training College later became.

Excerpt [1] also mentions that "It opened for the first intake of students in 1944 with considerable fanfare." This suggests that it was a training college, but we need more specific text from subsequent excerpts.

According to excerpt [2], Hewat Training College is mentioned as the location where Heneke goes to arrange an interview between the narrator and a Head of another school. It implies that it was a reputable educational institution worthy of considering for this meeting.

Excerpt [9] refers to Joe Rassool, who graduated from Hewat Training College in 1948/49 [note: my source only contains these excerpts; so I rely on the information given].

To clarify directly what Hewat Training College was, excerpt [3] states that "Hewat Training College stood opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds."

### q18 — What was the New Era Fellowship?

The sources describe the New Era Fellowship (NEF) as a cultural organisation established in 1937 "to discuss everything under the sun" [1]. It gained prominence when its leadership called for unity and full democratic rights among non-White groups, who had previously stood aside in solidarity with African communities [2], [6], [7]. A call to action by the NEF resonated within the Coloured community, attracting a wide range of organisations, including The African People’s Organisation (A.P.O.), sports' clubs, the Moravian church, and others, who joined forces with the NEF [1]. 

The NEF had a strong organisational structure under Hassan's leadership as "organising secretary," hosting several events and lectures around this time [2], [9], [10].

### q19 — What was the Non-European Unity Movement?

Based on the excerpts, here's what we can gather about the Non-European Unity Movement:

* The movement brought the principle of non-Collaboration, the refutation of "race", and the ideal of a non-racial society to the struggle for liberation [13].
* It accepted that the struggle for equal education was also bound up with the struggle for a democratic society [16].
* The movement had a 10 Point Programme which included demands such as full unfettered franchise, free and equal education to the age of sixteen, penal reforms, workers' rights, and the right to buy and sell land [6].
* Despite its ideals, the political struggle did not go according to the hopes of the Movement, and it failed to meet the challenge of rapidly changing times [13].

It's worth noting that Victor Walter Wesley Wessels' father, Rev. Dan Wessels, was one of the leaders of the Non-European Unity Movement [16], which suggests a strong connection between Reverend Wessels and this movement.

However, it is not explicitly stated what the exact goals or nature of the Non-European Unity Movement were beyond these points mentioned in the excerpts.

### q20 — Describe the author's involvement in cricket.

The provided sources do not contain any information about the author's involvement in cricket.

### q21 — Who was the author's mother?

The provided sources do not contain direct information about the author's mother. However, they mention various relatives of the author's mother and other members of their extended family.

[1] mentions "my mother" in several instances but does not provide details about her identity.


In [3], it is mentioned that "Auntie Cissie Gool, my cousin, a gutsy eleven-year-old at the time, and I only nine, had gone to the meeting", suggesting a close relationship.

### q22 — Who was the author's father?

The sources do not provide the name of the author's parents. The name "Yousuf Rassool" is mentioned in [1] as the author of the book, but his relation to the rest of the family described in the excerpts is not explicitly stated.

However, it can be inferred that Yousuf (Joe) Rassool, who wrote District Six - Lest We Forget was likely referring to himself using "my father" in [7] and [13], when describing his parents, but his name and that of his mother are mentioned in [6].

One of the excerpts, [19], mentions Peter Alexander Rassool (also known as Peerbhai) as Uncle Aity's son. In [7] it is the father of Yousuf (Joe) Rassool who became a Muslim/Peerbhai after marrying his mother, Ayesha.

So while there are some allusions to family relationships in these excerpts none explicitly describe who or how the author's parent was related to them which makes pinpointing an exact answer for this factual question from sources difficult, however Peter Alexander Rassool can be seen as the father of at least one Yousuf.

### q23 — Who were the author's siblings?

The provided sources do not contain specific information about the author's siblings, only mentioning aunts (e.g., Auntie Timmie, Aunt Dij) and various other family members, but no brothers.

### q24 — Who were the children of J.M.H. Gool?

According to excerpts [3] and [14], J.M.H. Gool was a polygynous man with two wives (Bibi and Wahida), which produced multiple children. The sources do not provide an exhaustive list, but they mention the following:

1. Adam H. G. Mahomed (Excerpt [4])
2. A. Cader (Excerpt [12])
3. Abdul Hamid (Excerpt [15])
4. Mohammed Hanief (Excerpt [15])

Additionally, Excerpt [7] mentions that when Wahida returned to the Cape with J.M.H. Gool's party, her two nieces were also part of their group. It is likely that these nieces are children of Wahida's brother, but no specific information is available about them.

The number and names of all J.M.H. Gool's children are not provided by the sources, so a complete list cannot be compiled from this information.

### q25 — Who was I.B. Tabata?

According to [1], I.B. Tabata was a politician who had completed a book called "The Awakening of a People", which recorded the coming into existence of the All African Convention, an organization that was one of the pillars of the Non-European Unity Movement.

Throughout the excerpts, it is clear that I.B. Tabata held some level of authority within the Unity Movement (e.g. [4], [16]), and his views on certain issues are discussed critically by others (e.g. [9]).

There may be more to Tabata's background and political views, but this information is not provided in any of the excerpts.

### q26 — Who was Dr. Abdullah Abdurahman?

According to [1], Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was also a medical doctor and father to Cissie Gool (Zainunnissa Gool), an activist and city councillor. 

Source [5] mentions that he asked for provision to be made for the higher education of coloured people and received no favourable response.

Source [14] confirms Dr. Abdurahman's position as a political figure in Cape Town and also states that "The bond between Dr. Abdurahman and my grandfather … was a key factor in the non-European unity movement." 

[1] provides more information on Dr. Abdurahman's background: his grandson notes that his grandfather’s family had made their fortune from trading.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to the sources, there were several connections between Gandhi and J.M.H. Gool:

1. Hospitality: The Gool family hosted Gandhi at their mansion in Buitencingle in February 1914 [12], which led to Joe Rassool's friendship with Yousuf Reddy [12].
2. Correspondence: Gandhi had a correspondence with J.M.H. Gool, as seen in the Letters exchanged between them, available courtesy of Mr. ES Reddy [5-7].
3. Satyagraha: Gandhi considered the time spent at the Gool mansion a crucial time in the satyagraha movement he led [14].
4. Recognition: Dr. A.H. Gool offered a farewell address to Gandhi on behalf of Port Elizabeth Indians, hailing him as 'Mahatma' - Great Spirit [13].

These connections suggest that J.M.H. Gool and his family were close associates of Gandhi during his stay in South Africa, and played a significant role in supporting and hosting him.

However, not all information about the connection between Gandhi and J.M.H. Gool is present in these sources. We cannot determine from the sources provided:

* The nature of their relationship before February 1914
* Whether other members of Gandhi's family or close associates also visited the Gools
* If there was any significant exchange, gift, or service offered between the two parties

The provided sources do not contain that information.

### q28 — Which organisations was the author involved in?

The author was involved in the following organisations:

1. New Era Fellowship (NEF)
2. Students' Union (established by his cousin Nina and others from the High Schools, with the author's family considering establishing one earlier)
3. Teachers' League
4. Coloured People's Vigilance Committee
5. Seventh Day Adventists

He was also involved in the following initiatives:

1. The "Movement" (a broader social movement focused on spreading democratic ideas and equality)
2. Anti-CAD alliance (Anti-Coloured Affairs Department)

Additionally, he was a member of the Student Representative Council (SRC) at his school and attended conferences such as the All African Convention and the NUSAS conference.

Note: The author's family members were also involved in various organisations and initiatives, including Dr. A.H. Gool who was president of the Cape British Indian Union and Dr. J.M.H. Gool who was a prominent figure in the Indian community.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not contain this information. 

However, we can gather some related information from other excerpts:

[1] mentions that the school authorities refused to offer the principalship to Ben Kies, an editor of the TLSA Journal, in favor of Jack Meltzer, who then became an antagonist of the Unity Movement.

[6] states that the Teachers' League of South Africa (TLSA) broke with its non-political stance and joined the Non-European Unity Movement, which led to a change in leadership within the TLSA.

From these excerpts, it seems likely that the TLSA was at least associated with or influenced by the Non-European Unity Movement, but further information is needed to determine the exact nature of their relationship.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to excerpt [10], Joosub Maulvi Hamid Gool (J.M.H. Gool) went to explore the situation in South Africa after hearing rumors of people picking up diamonds in the veldt. He fell in love with the climate of Cape Town and decided to settle there.

The exact date of his arrival is not specified in this excerpt, but it mentions that he had arrived by 1894, as evidenced by a listing in The Argus Annual and South African Directory for that year ([10]).



This seems correct because excerpt [2] says that the historical record is sparse for this period of J.M.H. Gool's life. However, this still does not mention where he came from, only that it was via Mauritius.



It is mentioned in Excerpt [7] that The British Newspaper Library at Colindale reports his arrival in 1884.

Therefore the correct answer is: He arrived in Cape Town in 1884 via Mauritius.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The provided sources do not contain detailed information on what the Hanaffi Quwatul Islam Mosque specifically referred to or functioned as. However, it appears in multiple excerpts [1], [3], and [19] along with other mentions of notable people and events associated with mosques and Islamic practice.

From excerpt [1], we know that it was an outpost of Islam in a predominantly European area of Cape Town, South Africa, standing on Loop Street but meandering down to Waterkant Street. It also notes the construction completion date of 1898.

The Mosque's significance comes from being mentioned alongside notable events and people within Yousuf (Joe) Rassool's narrative, indicating its importance in District Six community history.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to excerpt [17] ii, Dr. A.H. Gool writes in an article that his father was a half-uncle to him (Yousuf Rassool). The relationship is due to the fact that Yousuf's grandfather had two wives: Bibi and Wahida Ta'al, whose mother was part of the Dollie clan. This implies that Cissie Gool was likely J.M.H. Gool's daughter from his marriage with Wahida Ta'al, making her Dr. A.H. Gool's half-sister or step-cousin in family parlance (or even full sibling).

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources do not directly mention all the specific individuals J.M.H. Gool knew personally, but they do mention a few notable historical figures who interacted or associated with him:

* Adam H.G. Mahomed: Adam was described as a senior, financier, and possibly mentor to Gool. He was also the President of the British Indian League.
* Dr. Abdurahman (Ebrahim I. Abdurahman): The two appear to have had a close bond and would regularly discuss issues facing their community. They were also involved in various activities together.
* Cissie Gool: She was Adam's wife, but it can be inferred that she was likely friends or acquainted with J.M.H. Gool.
* Mr. R.O. Dudley: J.M.H. Gool and others are mentioned as attending a meeting at the New Era Fellowship where Mr. Dudley spoke about the lack of contributions from intellectuals in the Non-European Unity Movement to literature.
* Gandhi (Mahatma): There is evidence that J.M.H. Gool submitted a subscription to the Indian Opinion, which suggests he was involved with or supported Gandhi's work indirectly.

These interactions suggest that J.M.H. Gool had close relationships and friendships with key figures in his community and their networks but we cannot assume this is an exhaustive list of individuals without further evidence.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [7, 13] was a law in South Africa that enforced the apartheid system by separating different racial groups into specific areas. It aimed to maintain white dominance by creating separate townships for each group.

In relation to District Six, the Act affected the community severely [8]. The area was declared a "white" area due to its proximity to the city center and its already existing infrastructure. This led to the eviction of many residents, particularly Indians [16], to make way for new white inhabitants.

The Group Areas Act effectively dismantled the social fabric of District Six by breaking up families with mixed heritage [17]. The Indian community was worried about being herded into one group area and losing their businesses [13]. They made representations to the Group Areas Board, but their requests often fell on deaf ears as loopholes were quickly closed.

The Act created a "poverty trap" in District Six. New townships like Silvertown were built to entice people away from the area [15], but they were designed for lower-income groups with no internal doors and only one room to share among several family members. People often refused wage increases to avoid losing their already-overcrowded homes.

The Group Areas Act also made it difficult for residents to navigate bureaucratic processes due to complex racial classifications. In some instances, family members had different racial designations on their registration papers [16], leading to confusion and problems with access to resources.

The community's response to the act included seeking legal assistance through lawyers, a "loophole mentality" that failed to address the broader systemic issues of apartheid. Individuals like Hassen Mall understood the need for a more comprehensive approach, advocating for full democratic rights as a solution [14].

### q35 — Who was Hassen Mall?

Hassen Mall was a medical student at Cape Town who later studied law (LLB degree) [1,2]. He was described as "not tall, he radiated a kind of magnetism that could not be denied" [2], and had a "clipped brush moustache" [2] and habitually smoked a pipe. He was an accomplished cricketer who spun a wristy leg break [2].

Hassen Mall was also described as a man with great confidence and leadership skills, serving as the captain of a local cricket team [13]. He is mentioned multiple times throughout the sources as being instrumental in various activities, including organizing a cricket tournament, running for Durban's Curry's Fountain [16], and participating in various other interests such as playing cards and theatrical productions.

[2] describes Hassen Mall as being deeply involved in his community and aware of social issues. He appears to have been well-respected by others, with many admiring his charisma and natural leadership abilities [11].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpt [2], the Rev. A.B. Magagane was at a meeting of the British Indian League, however this does not directly answer the question.

Excerpt [5] mentions several organizations: The South African Christian Political Association (SACPA), which J. Wilson and James Curry joined in 1895; and The Workers' Party, described as a front organization of the Communist Party, which was activated at every election.

The New Era Fellowship (NEF) is mentioned in excerpt [14], which is associated with the Unity Movement. It also seems to be involved with the Anti-Coloured Affairs Council (Anti-CAC), later transformed into the Anti-Coloured Affairs Department.

It is stated in excerpt [12] that "The response was overwhelming" for a call to unity and democratic rights from The New Era Fellowship, which resonated in the Coloured community. This indicates that it had influenced, or possibly been influential within the larger Coloured community, potentially indicating participation by more than just individuals specifically mentioned.

Additionally, excerpt [16] lists organizations that responded to The New Era Fellowship's call for unity: "The African People’s Organisation (A.P.O.), sports’ clubs, The Moravian church, the New Era Fellowship, the Communist Party of South Africa, the Fourth International of South Africa, the Liberation League, the Workers’ Party and, believe it or not, the Coloured branch of Smuts’s United Party, plus a host of others."

The organisations in excerpt [16] include:

* A.P.O. (African People's Organisation)
* The Moravian church
* The New Era Fellowship
* Communist Party of South Africa
* Fourth International of South Africa
* Liberation League
* Workers’ Party
* Coloured branch of Smuts’s United Party.

These are some of the political organisations active in the Cape Coloured community during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

The provided sources do not contain information about the complete biography of Mahatma Gandhi. However, a summary can be formed from the provided excerpts.

Mahatma Gandhi is today known as Mahatma Gandhi [1]. The title 'Mahatma' means Great Spirit and was initially used to describe him in an article published by Indian Opinion [19], which reprinted a travelogue of Ramsey MacDonald, a British Labour Party MP. Historians still assert that Gandhi first acquired this title during the campaign for Indian independence years after leaving South Africa [1].

Gandhi's presence in South Africa is mentioned in various excerpts. He was a guest at 7 Buitencingle Street hosted by AH Gool in February-March 1914 and had two previous visits to Cape Town (October 1912 and January-February 1913, which are not explicitly stated but mentioned in other excerpts) [17, 18]. In 1914, he visited the Cape Town Docks, where he was presented with gifts and addresses. Gandhi's wife Kasturba was taken care of by the Gools when she became ill during his visit.

It is also mentioned that Gandhi had returned to London in some capacity [24] and an invitation to meet him was extended to AH Gool before Gandhi's deputation departed from London (November 28, 1906) [25].

Overall, Mahatma Gandhi was a key figure who was present in South Africa during the early years of his Indian independence movement.

### q38 — Who was Cissie Gool's father?

The provided sources do not explicitly state who Cissie Gool's father was. However, it is mentioned in [12] that Abdul Hamid was Cissie Gool's husband, and in [16] that Wahida (Cissie Gool) got married to J.M.H. Gool. It can be inferred from these sources that J.M.H. Gool was likely Cissie Gool's father or at least one of her parents.

### q39 — What was District Six like as a neighbourhood before the forced removals?

Based on the sources [1]–[19], here's what we can gather about District Six before the forced removals:

* It was a thriving cosmopolitan area in the heart of Cape Town, where people from different cultures and backgrounds lived together. (Source [2])
* The streets were described as "mean" (Source [15]), but also full of life, joy, and community spirit.
* District Six was known for its vibrant culture, with many characters contributing to the neighborhood's unique atmosphere. (Source [18])
* The neighborhood had a mix of cultures, including Indian, Muslim, Malay, and African communities living together. (Sources [10], [12])
* Housing in District Six was varied, with some areas being more affluent than others, but overall it was considered a sub-economic area. (Source [13])
* Residents were largely from the lower-income group and often had to sacrifice their municipal rights in order to live there. (Source [2])
* Despite its challenges, District Six was described as having "more life, more zest, more fire, more warmth and care" than many other places. (Source [14])

It's clear that District Six was a dynamic and vibrant neighborhood before the forced removals, with a unique cultural blend and strong sense of community.

Additional information:

* Sources [15] and [16] describe the physical layout of the area, including Caledon Street, Kloof Nek, and Signal Hill.
* Source [17] tells us about Joosub Maulvi Hamid's wedding ceremony to Bibi, which was arranged by her family.
* Source [18] mentions the story of Zaheer's death and how it affected Joe Rassool.

However, some questions are left unanswered in the sources. We don't get a clear picture of the economic situation before the forced removals, or what daily life was like for residents beyond their individual experiences.

### q40 — What was the Unity Movement's boycott policy?

The sources do not fully address what the Unity Movement's specific policies were in regards to the boycotts, other than quoting individuals within it discussing the idea of a "boycott" weapon and methods for its implementation (e.g. [10], [11]). However, they provide insight into how key members perceived the effectiveness and application of boycott as a means of struggle against Apartheid (see quotes from sources [1] through [20]).

The exact details of when and where these specific "boycott policies" were established or how decisions within Unity Movement on boycotts actually functioned are not entirely found here; it's necessary to look into other historical references to comprehend the entire scope.

