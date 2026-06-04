# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 54.3% (63/116) |
 | Avg judge score | 1.60/2.00 (20 questions scored) |
| Avg latency | 53507ms |

## Per-question results

| ID | Question | Hit rate | Judge | Sources | Latency |
|----|----------|----------|-------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 81861ms |
| q02 | Who are the author's children? | 3/3 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 19507ms |
| q03 | Who are the author's grandchildren? | 3/6 (50%) | 1/2 | LEST WE FORGET -rev25.pdf | 22780ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 22241ms |
| q05 | Who was J.M.H. Gool? | 2/8 (25%) | 1/2 | LEST WE FORGET -rev25.pdf | 67567ms |
| q06 | Tell me about Buitencingle. | 3/8 (38%) | 1/2 | LEST WE FORGET -rev25.pdf | 69064ms |
| q07 | Who is the author's wife? | 0/3 (0%) | 1/2 | LEST WE FORGET -rev25.pdf | 11206ms |
| q08 | Tell me more about the author's wife. | 3/6 (50%) | 1/2 | LEST WE FORGET -rev25.pdf | 48047ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | 2/2 | LEST WE FORGET -rev25.pdf | 21427ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | 1/2 | LEST WE FORGET -rev25.pdf | 62214ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 68893ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf | 68858ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | 1/2 | LEST WE FORGET -rev25.pdf | 54881ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 64896ms |
| q15 | What were the forced removals from District Six? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 53351ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | 2/2 | LEST WE FORGET -rev25.pdf | 80641ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 31677ms |
| q18 | What was the New Era Fellowship? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 44445ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | 1/2 | LEST WE FORGET -rev25.pdf | 69230ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf | 107354ms |

## Answers

### q01 — Who is the author?

The sources do not explicitly state who the author is, but based on excerpts [3] and [19], it appears to be Yousuf (Joe) Rassool, citing himself as "I" numerous times. 

Excerpt [3]: "...my wife... urged me to ground my writing in background investigation." Yousuf Rassool

Excerpt [19]: "My father, Yousuf Rassool..." Yousuf Rassool again mentioned his own name.

Therefore, the author of the sources is likely Yousuf (Joe) Rassool.

### q02 — Who are the author's children?

According to excerpt [1], the dedicatees of the book are the author's children: Feyruz, Reza, and Zarina; and also their grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

### q03 — Who are the author's grandchildren?

The author's children (grandparents from this generation) are mentioned as Feyruz, Reza, and Zarina. However, I can see three more names for their grandchildren mentioned later on:

Their grandchildren include Joseph, Samuel, and Alexander.

Note: There is no single source where all of the grandchildren's names together appear.

### q04 — To whom is the book dedicated?

The book "Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as their grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [15]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool is identified as Joe Rassool's grandfather [1], and was known variously in the press under different spellings or names: Joseph Gool, Yusuf Gool, Mr. Gul, J. H. Gool, J. H. M Gool [1] i.

He had a son, A.H. Gool, who was born in 1886 [2] ii.

In the press of that time, he is mentioned to have been involved in various meetings and activities, such as meeting Dr. Abdurahman at St Paul’s Mission School on January 9th, 1904, along with Adam Mohamed [4], and attending a meeting where Dr. Abdurahman asked what would be done about providing higher education for coloured people [4].

According to the sources, J.M.H. Gool was a respected citizen in Cape Town during his time: his house, Buitencingle, was one of the most prominent houses in the area, and he owned about 100 properties, some of which were located on prestigious streets like Adderley Street [17], [18].

### q06 — Tell me about Buitencingle.

Buitencingle is mentioned in several excerpts as the residence of Yousuf (Joe) Rassool's grandfather, JMH Gool, a prominent figure in Cape Town society. The house was located in what is now District 6 and had 13 rooms. 

[16] mentions that Buitencingle has a comprehensive list of page numbers where it is featured, including ix, 2, 3, 4, 6, 7, 15, 16, 19, 21, 22, 24, 30, 31, 33, 37, 39, 40, 41, 43, 44, 47, 49, 50, 51, 53, 95.

The excerpt [17] tells a story about the two wives of the grandfather. His first wife, Bibi, from India and the one he married in Cape Town, had "seething antipathy" for each other but were trained to instantly change their behavior when he came home, to avoid his infamous temper.

[18] does not add new information about Buitencingle itself, only a long list of locations including Buitencingle.

### q07 — Who is the author's wife?

The provided sources do not contain that information.

### q08 — Tell me more about the author's wife.

The provided sources do not contain specific information about the author's wife, other than mentioning in Source [19] that "I owe a great deal to my wife, who is an academic researcher, and who urged me to ground my writing in background investigation."

In source [1], it mentions "his wife, Kasturba", but this seems to refer to Gandhi's wife. 

A separate mention of a wife appears in Source [6], where the Gool family has a second wife named Wahida Ta’al after Grandpa Hajima marries her, leaving behind an eleven-year-old first wife Bibi Gool in India.

There is also a mention of Dr. Cissie Gool being assisted by Freda Lock and Lilian Isaacson as they fought against apartheid and Cassam the butcher at [15].

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool [1, 7, 9]. According to [16], his full name is not explicitly mentioned in the provided sources, but he is referred to as "Grandpa" throughout the text [17].

### q10 — Tell me about Kloof Nek.

According to the excerpts [1], [2], [4], [5], and [17], here's what can be said about Kloof Nek:

* It is a place of natural beauty, with a "haunting" or "sylvan" glen that seems ideal for study or contemplation.
* It is located near the Drakenstein Mountains and is connected to Table Mountain and Lion's Head by land.
* The area was used as a bus terminus, and people could take buses from Adderley Street outside Fletchers & Cartwrights department store.
* Kloof Nek was frequented by lovers, musicians, nature lovers, and ramblers, but also by down-and-outs or "Bergies" who took refuge in the area's cavelets and bushes to drink cheap wine and spirits.
* There is a Table Mountain cable car that began its run near Kloof Nek, carrying people up the mountain for breathtaking views.

It appears that Kloof Nek was a significant location mentioned multiple times throughout the provided excerpts.

### q11 — What was the Teachers League of South Africa (TLSA)?

The TLSA (Teachers' League of South Africa) is mentioned in sources [1], [2], [4], [5], [6], and [16]. It appears to be a teachers' union or organization that aimed to promote education and democracy for non-White students in South Africa, particularly during the time of Apartheid.

Some key points about the TLSA from the provided sources:

* The TLSA had a motto: "Let Us Live for our Children."
* The organization collaborated with other groups, such as the Cape African Teachers' Association.
* One of its strategies was to create Parent-Teacher Associations (PTAs) throughout the country to build alliances with parents and promote democratic education.
* The TLSA took on a more active role in politics after breaking away from its previous stance of non-political professionalism.

It is worth noting that there are some references to rival organizations, such as TEPA (Teachers' Educational and Professional Association), which seems to have been formed by the provincial authorities to counteract the political activities of the TLSA. However, more information about the TLSA's specific goals, leadership, and activities can be gleaned from other sources not provided in this excerpt.

### q12 — Who was Cissie Gool?

Based on the provided sources, here is what is known about Cissie Gool:

* [1] mentions that she "had supported the Torch Commando in 1953", indicating that she was involved with this movement which aimed to win votes among white electorate using General Smuts's war-time record.
* [18] implies that she was a surprising or shocking speaker who addressed an anti-influx rally, contradicting one's expectations based on her past involvement with the Anti-CAD Movement and the Unity Movement. She is quoted speaking in support of measures to deal with "influx control" and "the street gangs, colloquially referred to as the 'skolly' problem".
* [8] describes a photograph from the 1920s featuring Cissie Gool and mentions that she was a member of her family; however, it does not provide further details about her.
* [13] briefly describes another confrontation between Cissie Gool and Ismail (a politician) in an election constituency.

Cissie Gool's past involvement with the Anti-CAD Movement and Unity Movement suggests that she may have been a figure associated with the anti-apartheid movement, however more detail about her role is not provided.

### q13 — What was the All Africa Convention?

The provided sources do not contain a detailed description of what the All African Convention was. However, we can infer some information based on multiple excerpts.

[2] mentions that Vic Wessels, Les Jacobs, and the narrator went to represent the New Era Fellowship at the conference of the All African Convention in Bloemfontein in 1954. This suggests that the convention was a significant event for the anti-apartheid movement.

[18] states that it was one of the "pillars" of the Non-European Unity Movement, implying that it played a major role in this movement.

From these excerpts, we can conclude that the All African Convention was likely an anti-apartheid organization or gathering that aimed to unite various groups against racial segregation and discrimination. However, the specific details about its goals, structure, and activities are not provided.

It's worth noting that a quick search outside of the given sources might provide more information on the topic.

### q14 — Where was District Six and what kind of place was it?

According to the excerpts, District Six was a neighborhood or "group area" located in Cape Town, South Africa. It is described as a vibrant, multicultural community where people from different backgrounds lived together.

Excerpt [4] says: "I think it all began when I saw 95 Caledon Street standing empty..." (referring to an event that occurred when the narrator visited District Six).

Later, excerpt [6] describes visiting the home of his grandfather on Buitencingle, where there would be commiseration if a child was born with dark skin and joy if it was fair-skinned. This indicates a society where beauty standards are tied to physical appearance.

In excerpt [10], District Six is referred to as "our community" where residents were experiencing the effects of apartheid.

Excerpt [17] describes how, as a result of apartheid policies, District Six began to decline in the early 1950s: "A potent source of misery occurred when a Coloured Area was declared White..." (and house prices plummeted).

Overall, based on these excerpts, it appears that District Six was a complex place with different social and cultural dynamics.

### q15 — What were the forced removals from District Six?

The forced removals from District Six refer to the mass eviction of residents, primarily of African, Coloured, and Asian descent, from the area known as District Six in Cape Town, South Africa. This occurred as a result of the Group Areas Act (1950), which aimed to separate different racial groups into designated areas.

According to the source [19], "Apartheid policy celebrated white racial supremacy. It oppressed black women, men and children by subjugating their ‘being’, their selves. Apartheid legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life."

This led to the dispersal of the people across the barren sandhills of the Cape Flats, effectively decimating a once-vibrant community and way of life.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi (1869-1948) was an Indian independence activist who played a key role in India's struggle for freedom from British rule.

According to the sources, Gandhi had a close connection with the Gool family, particularly with Yusuf Hamid Gool and his son Abdul Hamid Gool. The sources mention that:

* In 1914, Gandhi visited Buitencingle, the Gool family's mansion in Cape Town, on three separate occasions: February-March for Kasturba Gandhi's illness, July 18th for his own departure from South Africa ([12])
* During one of these visits, Gandhi's son Manilal was sent home, and Gandhi was hailed as 'Mahatma' (Great Spirit) by Dr. A.H. Gool on behalf of Port Elizabeth Indians in August 1914 ([13])
* Gandhis were treated at the Gool residence during Gandhis wife Kasturba's serious illness in March 1914 ([6] and [16])
* Gandhi attended functions hosted by the Gool family, where he thanked them for their services to him and his wife Mrs. Gandhi ([6])

The connection between Gandhi and the Gool family appeared to be a long-standing one, with the two families sharing similar values of non-racial democracy and mutual support.

(Note: The provided sources describe various instances of interactions between Gandhi and members of the Gool family across multiple visits, highlighting their close relationship.)

### q17 — What was Hewat Training College?

Hewat Training College was an institution that provided teacher training for Coloureds. It was the first Post Matriculation Teacher Training facility for Coloureds in the country and opened for its first intake of students in 1944 (Source [2]). According to Source [12], it had previously been a Whites only Primary school, but was deemed good enough, despite being undeveloped, to be converted into a training college for Coloured teachers due to its location.

### q18 — What was the New Era Fellowship?

The sources do not contain a direct description of what the New Era Fellowship (NEF) was, but it is mentioned in multiple excerpts as:

* A "cultural organisation associated with the Unity Movement" [13]
* A political and cultural organization that issued a clarion call to all Non-European organisations to meet "to counter the predicted assault on the remaining rights of the ‘Coloured’ community." [8]
* Where Hassan Bavasah served as an organizing secretary, overseeing its program of events. [10]
* Also considered by the author to be viewed with "grave misgiving" because it competed with other organizations, such as the Cape Peninsula Students Union that was believed to be led by Mr. Tabata. [16]

### q19 — What was the Non-European Unity Movement?

The sources do not contain extensive information about the specific objectives or activities of the Non-European Unity Movement. However, [6] states that "the idea of non-European unity arose in 1943" and mentions Dr. Abdurahman and D.D.T. Jabavu's attempts to create a non-European unity movement in the early part of the 20th century.

But [19] further clarifies the origins and concept of the Non-European Unity Movement, stating that "the idea of non-European unity arose in 1943 when the government foisted The Coloured Affairs on the so-called Coloured people"... "[T]hese attempts floundered because the historical circumstances were not yet ready for it," but "the idea was there." This suggests that the Non-European Unity Movement was a proposed or conceptual movement aimed at unifying non-European communities in South Africa, possibly in response to segregationist policies and discriminatory laws enacted by the government.

[2] provides more context on how the idea of unity emerged and relates with another organisation (possibly connected to the NEUM), discussing attempts before 1943 and noting that people were aware of deep discussions regarding these unification ideas at some location.

### q20 — Describe the author's involvement in cricket.

According to the sources, the author was involved in cricket in various ways:

* He became friends with a fellow cricket player, referred to as "Muddy" [1-3]. Through their shared interest in cricket, they developed a close friendship and political alignment.
* The author joined a non-racial cricket league known as the Sunday League, which allowed teams from different ethnic groups to play together. He mentions that his club, Kismets C.C., was successful in recruiting new players from various groups [1-3].
* As an emerging cricketer, the author was trying to master the forward defensive stroke and build his batting skills. Although he didn't achieve great success initially, he gained experience and confidence over time [13-14].
* In 1951, the author's team played cricket against Eastern Province, but he missed the match against Natal [3]. He later became interested in organizing a union to enable playing of cricket at High School and College level, although this initiative was thwarted by existing bureaucracy [12].
* By 1952, the author's team was strengthened with the recruitment of talented players like Rashid Gardee and Salie Van Haacht from other unions. He also mentions his admiration for Mylie Begg, a seasoned cricketer who had the ability to frustrate any fast bowler [6-18].
* Overall, cricket played an important role in the author's life, providing opportunities for friendships, skill-building, and team involvement.

Some specific sources that are particularly relevant to describing the author's involvement in cricket are:

[1] "[...] became close friends through cricket [...]" [1]
[6] "[...] he was at his peak [...] our scorebooks had several eighties against his name." (referring to Mylie Begg) [6]
[13] "I began to master the elements of the forward defensive stroke and build the foundation of the art of batting." [13]
[18] "[...] cricket played an important part in my life as it gave me opportunities for skill-building [...]"

