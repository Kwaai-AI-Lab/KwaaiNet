# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 52.6% (61/116) |
 | Avg judge score | 1.80/2.00 (20 questions scored) |
| Avg latency | 24283ms |

## Per-question results

| ID | Question | Hit rate | Judge | Sources | Latency |
|----|----------|----------|-------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | 2/2 | [Graph: Gray’s Elegy], LEST WE FORGET -rev25.pdf | 26025ms |
| q02 | Who are the author's children? | 3/3 (100%) | 2/2 | [Graph: Children], LEST WE FORGET -rev25.pdf | 28626ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 23223ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 22277ms |
| q05 | Who was J.M.H. Gool? | 1/8 (12%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: E.S. Reddy Former] | 27669ms |
| q06 | Tell me about Buitencingle. | 3/8 (38%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: No.7 Buitencingle Street] | 26088ms |
| q07 | Who is the author's wife? | 0/3 (0%) | 1/2 | LEST WE FORGET -rev25.pdf | 22939ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 23761ms |
| q09 | Who was the author's grandfather? | 1/9 (11%) | 1/2 | LEST WE FORGET -rev25.pdf | 22592ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | 2/2 | LEST WE FORGET -rev25.pdf | 25761ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Native Life in South Africa] | 23357ms |
| q12 | Who was Cissie Gool? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Gool & Co.] | 27017ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Warsaw Pact] | 22642ms |
| q14 | Where was District Six and what kind of place was it? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf | 22542ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: French Revolution] | 18880ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Gool clan] | 28038ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | 2/2 | LEST WE FORGET -rev25.pdf | 21555ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Warsaw Pact] | 25061ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | 2/2 | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 23264ms |
| q20 | Describe the author's involvement in cricket. | 1/5 (20%) | 2/2 | LEST WE FORGET -rev25.pdf | 24354ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [9] in [1], and he was also mentioned in [8]. Additionally, his father had a medical article published, so it's likely there are multiple mentions of him. Also, a note to the editor from E.S. Reddy mentions that the second edition of District Six: Lest we Forget is being launched, and notes the Gool family's contribution; however, this was for an anthology or collection titled as such and did not include the father Joe Rassool as an author.

### q02 — Who are the author's children?

The provided sources do not contain that information. However, based on the dedication from source [8] the author's children are:

1. Feyruz
2. Reza
3. Zarina 
4. Joseph
5. Samuel
6. Alexander
7. Leonardo
8. Dylan
9. Lewis

### q03 — Who are the author's grandchildren?

The provided sources do not contain a complete list of all the author's grandchildren. However, they mention six of them: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis (Reza 2007).

### q04 — To whom is the book dedicated?

The book is dedicated "For my children: Feyruz, Reza, and Zarina  and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [17]

### q05 — Who was J.M.H. Gool?

The provided sources [3], [8] (and a partial mention of him in excerpt [11]), do not offer in-depth information about the personality or life events of Dr. J.M.H. Gool.

However, from various references dispersed throughout the book:

1) The historical record is sparse for this period of J.M.H.
[3]

2) Chapter Five: Fortunes Decline lists his son and granddaughter along with other members of Grandpa's family residence in Cape Town being a prominent household. 

Chapter Five, however, focuses on a broader topic – the decline of fortunes for various notable families including the Woolas and Gool's.

3) In excerpt [11], Bernard Herzberg recalls that J.G., which could mean J.M.H. Gool, was part of an influential group in art, music, and politics.

### q06 — Tell me about Buitencingle.

The provided sources describe Buitencingle as a street and a location where the author's grandfather, Haji Joosub Maulvi Hamid Gooli, lived with his family. 

 According to [3], the house at No. 7 Buitencingle Street stood out for its grandeur and elegance, belonging to the wealthy merchants of that time, including the Gools. The house had umpteen rooms, implying a large and spacious living area. However, in the same passage, it is mentioned that this grand estate came with a responsibility to manage many staff members as servants.

As depicted in [11], the photographs from around 1905 show Buitencingle Street filled with life, suggesting an event happening during those times. It can be assumed that celebrations and festivities were common in this part of town due to the wealthy inhabitants.

In [17], Eid celebrations at 7 Buitencingle Street are described as significant events in the community. This further highlights that the area played a vital role in bringing together people from different backgrounds for communal activities.

Unfortunately, the sources do not contain information about how Buitencingle was affected by the events leading to forced removals in District Six during apartheid era South Africa.

The provided sources don't contain specific details regarding changes or historical periods of time within Buitencingle beyond the accounts given.

### q07 — Who is the author's wife?

The provided sources do not contain that information. Only [10] mentions "my aunt named her Nurjahan, after her late mother" but does not confirm any relation with the author; similarly in [18], it only says "My wife and four sons would often peer over my shoulder to understand what possessed me.  In so doing, they were able to catch glimpses through the undergrowth into the past and to learn, along with me, something of who we are and from where we came."

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about Joe Rassool's wife. However, it is mentioned in [9] that Joosub Maulvi Hamid Gool's cousin Nasima's father was born in Mauritius in September, and in [18] Joe Rassool mentions his friendship with his grandfather, suggesting he was a key figure in researching the manuscript.

In [2], a woman named Nazima is mentioned as "my wife" who urged the author to ground his writing in background investigation. Additionally, [20] quotes a passage where Bibi (Joosub's second wife) receives messages from her husband when she and Ayesha were supposed to come to Cape Town.

We can be certain of the following:

* Joe Rassool is married to an academic researcher.
* In at least one source, his wife is named Nazima.
* His wife played a crucial role in encouraging him to conduct thorough research for the manuscript.

### q09 — Who was the author's grandfather?

The provided sources do not contain explicit information about what the author's grandfather's name is. However, from [8] and [18], it can be inferred that "Grandpa" has a business partnership with M. Wilson and he is referred to as having daughters Gadija (from [15]) and Janab/Zobeida (from [1]). 

But in [10], "My grandfather" is the subject of other sentences; no personal name is supplied for him.

Therefore, we cannot definitively answer your question.

### q10 — Tell me about Kloof Nek.

According to [3], when the author was a child, he followed a schoolmate to Kloof Nek, which was described as "a place of haunting beauty". The author had initially been reluctant to go there but became entranced by its natural beauty.

From [16] and [17], it is mentioned that Kloof Nek was a popular spot for lovers, mountaineers, nature lovers, and ramblers. It also housed down-and-outs known as "Bergies", who would drink cheap wine and methylated spirits in the area's cavelets and bushes.

[17] describes Kloof Nek as having a connecting saddle of land that joined Table Mountain to Lion's Head, sprawled on the eastern flank of the city with its head gazing down on Table Bay.

### q11 — What was the Teachers League of South Africa (TLSA)?

[A combination of sources [3], [4] and [8] provide information about the Teachers’ League of South Africa (TLSA)]. The TLSA is described as an organisation in the forefront of the struggle for democracy, a teachers' union that became political which could not be tolerated by the authorities. Members were regarded as leaders in the community and were involved in preparing for the eventuality of apartheid being extended to coloureds and Indians [6]. They worked on strategies to collaborate with other groups to prepare pupils against the impending onslaught. The union was known to induct teachers who were skilled in political/educational composition, training them through their Education Journal [8] and allowing publication of their articles in the same journal [7].

### q12 — Who was Cissie Gool?

Cissie Gool is mentioned in several excerpts as a prominent figure in the community, particularly in Cape Town. According to excerpt [3], she spoke at a meeting against "influx control" and street gangs ("skolly" problem). In excerpt [7], it mentions that her funeral procession was remarkable, with a traffic policeman appearing on the scene, probably due to her influence as a city counsel member.

Excerpt [11] and [12] describe Cissie Gool's family background. Her husband is Dr. Abdulla Abdurahman, and she is from a prominent dynasty in Cape Town. As mentioned in excerpt [13], she organized a massive demonstration against the government's plan to introduce residential segregation, calling it "A plague on both your houses" (a reference to the Unity Movement's stand).

Excerpts [14] mentions her among those who loved Cheops and were part of his "pharaonic style" court.

Cissie Gool is also mentioned in excerpt [15], [16], and [17] as a notable figure in the community, with various roles including speaking at meetings, serving tea at her Mount Street residence, and being on the platform with other familiar faces.

Excerpt [18] mentions that her actions, such as insisting to swim in an all-white swimming pool, might have been because of her position as a city councillor.

### q13 — What was the All Africa Convention?

There is no clear information in the provided sources about what the "All Africa Convention" actually did or represented. However, [14] and [17] suggest that it was related to non-White cricketers from different places coming together and discussing ways they would be treated while their skills were being showcased at South Africa.

[18] indicates that there was a conference of the All African Convention where issues like the Rehabilitation Scheme that affected peasants, land ownership, and educational inequality for the majority Black population in South Africa were some of its discussions.

### q14 — Where was District Six and what kind of place was it?

[16] states: "These were some of the noteworthy eccentrics that inhabited my world, and gave District Six its special flavour." 

 District Six is described as a place with a unique flavor provided by its eccentric residents in [16]. Specifically, an example of the kind of people living there are tap-dancers who would dance on the sidewalks without much hope of ever becoming famous.

[17] describes Caledon Street running "getting tattier along the way" in District Six. 

Therefore, District Six was a mixed and somewhat run-down district with a lively atmosphere, home to a diverse community with various cultural attractions such as live performances and shops like fish markets.

### q15 — What were the forced removals from District Six?

The provided sources do not contain explicit information about the forced removals from District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Based on the provided sources [1]–[20], Mahatma Gandhi was a key figure who visited the Cape Colony, particularly District Six in Cape Town, during the early 20th century. The Gool family, specifically Dr. Abubakar Abdurahman, Adam Gool, and J.M.H. Gool (also known as Hamid), played an essential role in his visit.

Gandhi was a guest at the Gool mansion, Noor Bagh (also referred to as 7 Buitencingle), on several occasions, during which he developed close friendships with various members of the family and their associates. The correspondence between Gandhi and Dr. Abdul Hamid Gool, who attended medical school in London, is one example.

Gandhi also relied on the support of the Gools while fighting against anti-Indian legislation. One such instance relates to his daughter's illness; Indian Opinion stated that during his stay in Cape Town, Dr. Gool was "unremitting" in his care for Mrs. Gandhi (who remained ill and unable to shake off this illness).

This visit occurred in 1914, which allowed Gandhi to get support from the people of South Africa who shared his vision of a non-racial democracy within the British Empire.

Additionally, other connections between Gool family members are evident through sources [14], where we can see that the latter were associated with various organizations promoting civic rights for Indians, such as being board members at Woodings Preparatory school that was run by West Indian teachers.

### q17 — What was Hewat Training College?

According to [1] and [5], Hewat Training College was a training institution for teachers, specifically training teachers for the upper years of the primary division. It eventually opened its first intake of students in 1944 with considerable fanfare.

### q18 — What was the New Era Fellowship?

The provided sources do not contain an explicit description of what the New Era Fellowship (NEF) is or its purpose [1-20]. However, it can be inferred that the NEF was a political-cultural organisation associated with the Unity Movement and involved various activities such as lectures, publications, and community engagement.

According to [7], the NEF was established in 1937 "to discuss everything under the sun." By [9] and [18], the NEF also organised public talks and discussions on topics like the Indian Question. The NEF's members included Victor, Hymie, and the author, who were also involved in other organisations such as the Unity Movement.

The NEF played a significant role during times of confrontation with the government or the apartheid regime. It can be inferred that the NEF was an organisation focused on addressing social issues, promoting unity among communities, and resisting the apartheid system [7, 18].

### q19 — What was the Non-European Unity Movement?

[7] indicates that the Non European Unity Movement was founded in 1943. It was an organization that brought together various non-white groups to challenge the apartheid regime. [8] confirms this information, mentioning "Unity Movement" and its connection to other organizations.

From [14], it is mentioned the principle of Non-collaboration was part of the NEUM's methods.

### q20 — Describe the author's involvement in cricket.

According to this memoir, the author was heavily involved in cricket, particularly during his younger days. 

The author belonged to the Malay Cricket Union (Barnato Cricket Board) but later becomes interested in non-Apartheid cricket and advocates against playing with the Springboks team.

In excerpt [5], the author describes enjoying playing table tennis at Mr. B. Allie's residence after a game of cricket, implying that he was an active player.

Excerpt [10] suggests that the author was part of teams that had non-racial policy, which allowed players from different racial backgrounds to play together.

In excerpts [17], [19] and [20] discuss how the author dreamed about being picked for the team, while learning to develop his cricket skills through Hassen Mall's guidance. 

Excerpt [3] mentions a "plethora of good players" in the non-racial Sunday league that the author was a part of.

Although there is no direct statement indicating whether the author played at higher levels or professionally, these accounts suggest involvement on an amateur level and deep enthusiasm for the sport.

