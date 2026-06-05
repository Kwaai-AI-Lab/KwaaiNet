# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 53.3% (120/225) |
| Avg latency | 31575ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 40120ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 20026ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 21993ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 21807ms |
| q05 | Who was J.M.H. Gool? | 3/8 (38%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 31195ms |
| q06 | Tell me about Buitencingle. | 6/8 (75%) | LEST WE FORGET -rev25.pdf | 26664ms |
| q07 | Who is the author's wife? | 2/3 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 20388ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 21754ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 24162ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf | 54864ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 51224ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 38970ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29113ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 39509ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 29982ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Gool AH Gool] | 30609ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 41900ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 66468ms |
| q19 | What was the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 40745ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 33635ms |
| q21 | Who was the author's mother? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 27867ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 29664ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 30794ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 35092ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 27875ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 35366ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 41675ms |
| q28 | Which organisations was the author involved in? | 0/5 (0%) | LEST WE FORGET -rev25.pdf | 24857ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 25554ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 4/6 (67%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 23548ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29967ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 23108ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 3/5 (60%) | [Graph: Dr. Gool AH Gool], LEST WE FORGET -rev25.pdf | 31899ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 26361ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: H. Mall] | 25949ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 24699ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 33579ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 24600ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 24213ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 31217ms |

## Answers

### q01 — Who is the author?

The answer is absent from the provided excerpts, but it can be inferred that the author is possibly a member of the Rassool or Gool family, given the content and context of the text. However, the most direct clue comes from [9]: "My father, Yousuf Rassool, sent me text to the second edition in 2002..."

### q02 — Who are the author's children?

The author mentions his children in the dedication as follows: "For my children: Feyruz, Reza, and Zarina".

### q03 — Who are the author's grandchildren?

According to excerpt [1], the author's children are Feyruz, Reza, and Zarina. Additionally, his grandchildren are Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis (from a taped interview in 1993/4 - [13]).

### q04 — To whom is the book dedicated?

The book "District Six - Lest we Forget" is dedicated to the children and grandchildren of the author, Joe Rassool. They are:

1. Feyruz
2. Reza
3. Zarina (the author's children)
4. Joseph
5. Samuel
6. Alexander
7. Leonardo
8. Dylan
9. Lewis (the author's grandchildren)

### q05 — Who was J.M.H. Gool?

J.M.H. Gool was an Indian doctor, described in [1] as nationality: Indian; occupation: Doctor; affiliation: Indian Opinion.

In [17], there is a list of references to him with numbers 15 and i "What is in a name? A rose by any other name is as sweet…" J.M.H.), indicating that his surname was mentioned several times, but not consistently named throughout the texts. Various names for J.M.H. Gool can be found. 

In [17] ii A.H., it only mentions his father's name as part of a description related to his own honourific title Maulvi.

To identify more about him:

* He was born in 1886, according to [6].
* His testimony is recorded in Supreme Court transcripts in 1892.
* In an early document referring to Gool, it shows his participation in British Justice and his standing in the Cape community. 
* In another document he is described as an 'Oriental trader.' (Source [6])
* He participated in various activities for South African Indians. 
    - According to [10] xiv of the sources: 
        - A petition was presented in 1903 against an attempt by the City Council to segregate Indians.
        - In it, J.M.H. Gool among others signed as members of the protest, including Adam Gool Mahomed.
* He had business activities like being a witness with Adam H.G. Mahomed in a case mentioned in [6].
* [1] (Nationality: Indian), and according to his first son being born in 1886 [2]ii A.H., that is the closest we get to information about J.M. H’s ethnicity.

### q06 — Tell me about Buitencingle.

According to the sources, Buitencingle was a street in Cape Town, South Africa. Here's what can be gathered about it:

* It was one of the outermost belts of Cape Town [2].
* The name "Buitencingle" is of Dutch derivation and should more correctly have been pronounced as "Baytencingle", with the "Ay" sound made rounder by pursing the lips [8], [9].
* Buitencingle was surrounded by other notable geographical features, including Table Mountain, which lay beyond it to the northeast [2]; Kloof Street, which led from the bottom of Buitencingle up to Kloof Nek [2]; and Loop Street, which branched off from Buitencingle to meander down to Waterkant Street and the Rogge Bay waterfront [13].
* The street was part of a cosmopolitan precinct with a mix of European, Jewish, and gentile residents, although it became increasingly Europeanized over time [14].
* By the time the narrator's family moved to Buitencingle, the area had already begun to decline, marked by the departure of wealthy whites and the influx of Black and Coloured people [13].

It is also worth noting that 7 Buitencingle Street was the location of the Gool mansion, a thirteen-roomed mansion belonging to the narrator's grandfather, Haji Joosub Maulvi Hamid Gooli [17].

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1]. 

Additional information: According to [9] and other sources, her name might be spelled "Bibi" as well. She appears to be a central figure in the family history and has been involved in various aspects of their lives, including providing historical context to the story.

### q08 — Tell me more about the author's wife.

The provided sources describe the author's wife in a limited capacity. Here is what can be inferred:

* Her name is Nazima Rassool [1,9].
* She is an academic researcher and an expert in her field [1,5].
* She urged the author to ground his writing in background investigation to lend authenticity to the story [1,5].
* The author owes a great deal to her for encouraging him to do historical research [5,19].

That's the extent of what can be determined about the author's wife from the given sources.

### q09 — Who was the author's grandfather?

The provided sources do not contain the full name of the author's grandfather, only that he is referred to as "Grandpa" or "J.M.H. Gool". Source [4] mentions him as J.M.H. Gool but in other excerpts, his first name is mentioned only in some versions and as J., for example: Version 1 refers to him with the initials but not the full names - [4].

### q10 — Tell me about Kloof Nek.

Kloof Nek appears to be a scenic location in Cape Town, South Africa.

According to the sources :

- Kloof Nek was a place of haunting beauty (Excerpt [18]).
- It was located at the bus terminus for a single-decker trackless tram that ran from Adderley Street outside Fletchers & Cartwrights department store (Excerpts [7] and [18]).
- The tram stopped near a connecting saddle of land that joined Table Mountain to Lion’s Head (Excerpt [18]).
- Kloof Nek was frequented by lovers, mountaineers, nature lovers, ramblers, and down-and-outs known as "Bergies" who found refuge in the area (Excerpts [4] and [5], although [5] primarily discusses eviction).
- There were cavelets, culverts, and bushes where people could take shelter (Excerpt [4]).
- The location was accessible via Kloof Street, which reached up to Buitencingle, Cape Town's outskirt at the time (Excerpts [7], [18], and [6] in another text not specifically mentioned in this answer which I've taken as a different account).

There is no information regarding its current condition or if it still exists.

### q11 — What was the Teachers League of South Africa (TLSA)?

The sources do not provide a complete description of the Teachers' League of South Africa. However, based on excerpts [1], [5]-[6], and [14]-[15]:

* It was an organization that opposed apartheid education [5].
* Its motto was "Let Us Live for our Children" [5].
* The TLSA believed the government's plan to create a supposedly "educated" man called the Eiselen man, who would accept his place in society without question or challenge, and this aligned with their goals of creating individuals loyal to apartheid principles.

The organization had some connection to the Cape African Teachers' Association and sought to support teachers against the implementation of apartheid education. They aimed to establish alliances between parents (Teachers) and pupils.

At least three figures had connections to the TLSA: 

1. Solly Edross, B. Sc., who was a member  [18].

2. Edgar Maurice, the Vice President of the TLSA - he delivered a speech at an event on the topic "Segregation in Education" which is mentioned multiple times ([14]-[15]).

3. The other connection to the TLSA that can be observed is from various lists found but there does not seem enough information to confirm its nature, these people included: Tabata [Tabata  ( 14, 132, 137, 143, 144, 148, 149, 152, 154, 160, 163 )], Thwaites and Tregger

### q12 — Who was Cissie Gool?

According to the provided sources, particularly [6], [7], and especially [8] and [16], we can learn more about Cissie Gool:

- Cissie was a member of a notable family. Her father was Dr Abdurahman, who was the first "native" from his community to qualify as a doctor from Glasgow University in 1893.
- She attended school and played an active role in her community.
- Source [7] indicates that Cissie had a significant relationship with the author of one of these accounts. However, it also says she addressed a meeting (in a venue called Banqueting Hall) at which the author was shocked to see her for reasons not explained.
- Cissie is highlighted in multiple mentions for her political work and activism against residential segregation, showing that she took a public stand for the rights of Indians and Coloured people during challenging times in South Africa's history.

The provided information suggests that Cissie Gool was involved in key historical events and had significant family background.

### q13 — What was the All Africa Convention?

The All African Convention is mentioned in [12] as being a meeting point for several leaders and members. In [17], it appears as one of the pillars of the Non-European Unity Movement and mentions that Mr. I.B. Tabata recorded its coming into existence in his book "The Awakening of a People" ([18]).

### q14 — Where was District Six and what kind of place was it?

The sources do not provide a specific location, but instead describe it as an area that "buzzed with the joy of life" [9], suggesting it was a vibrant and lively neighborhood. It is mentioned multiple times that people in District Six were forced to move out due to policies such as Group Areas Act (1950) [10] and the effects of apartheid, which suggests that it was likely located in an area designated for non-white residents, possibly Cape Town, South Africa.

District Six is described as a "thriving cosmopolitan area" [10] with a rich cultural heritage. It was home to people from various ethnic groups, including Indians, Africans, and Coloureds, who coexisted and interacted with each other [16]. The sources also mention specific locations within District Six, such as Hanover Street [12], Caledon Street [14][15], Clifton Streeet [9], Corporation Street [9], Chapel Street [1][18], and Vandeleur Street [17].

District Six was a neighborhood where people from different ethnic groups lived together, shopped at local businesses, attended community events, and went to school. It was a place where the family mentioned in the text had moved to [14] and experienced both the joys and hardships of living in this vibrant community.

The sources describe District Six as having its own unique culture, with people participating in activities such as koe'sisters sales [17], attending schools like Trafalgar High [16], and being part of a tight-knit community where everyone knew each other. The neighborhood had a strong sense of identity and was an important part of the residents' lives.

However, the sources also highlight the devastating effects of apartheid policies on District Six, stating that the area was subjected to forced removals, and its people were dispersed across the city [6]. The Group Areas Act (1950) led to the displacement of many families from their homes in District Six to other designated areas, often in poor living conditions with inadequate infrastructure [2][12].

Overall, based on the sources provided, it appears that District Six was a vibrant and culturally rich neighborhood that was devastated by apartheid policies.

### q15 — What were the forced removals from District Six?

The provided sources do not directly address the question of what the "forced removals" from District Six refer to. However, based on context [3], it can be implied that the removals might have been some form of eviction or displacement that occurred in the area.

Source [16] briefly mentions motions related to forced renaming of surnames at National Party conferences (which suggests efforts by authorities to suppress certain cultural identities), but not explicitly what these were connected to.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi was a prominent Indian independence activist who played a significant role in India's struggle for independence from British rule.

From the provided sources, it is clear that Gandhi had a close connection with the Gool family, particularly Dr. A.H. Gool and his uncle Abdul Hamid Gool.

Source [4] mentions that Gandhi was assured of the support of people of stature and influence in the Gool mansion at 7 Buitencingle when he came to Cape Town. Source [3] notes that one of the grandfather's letters of support to Gandhi was what prompted a public outpouring of respect from the Indian community.

Source [6] states that Gandhi had close correspondence with Abdul Hamid Gool while he was a medical student at Guy’s Hospital in London, showing their close relationship.

Multiple sources ([1], [3], and [4]) highlight the hospitality provided by the Gool family to Gandhis during February 1914 at their residence. The friendship that developed from this visit is celebrated, as it led to my connection with Mr. Joe Rassool." (my cousin) - Source [18]

Source [9] records Gandhi's initial letter in 1897 where he commended G Hool for his fearlessness in taking up the cause of Indian rights.

Another interesting aspect mentioned in source [19] is that Wilson, an African American called Mr., was considered J M'H Gool’s business partner.

### q17 — What was Hewat Training College?

According to source [1], Hewat Training College "had once been a Whites only Primary school" that was considered too close to the prison (Roeland Street Gaol) and District Six, so it was deemed suitable as the site of the first Post Matriculation Teacher Training facility for Coloureds in the country. It opened in 1944 with considerable fanfare [1].

### q18 — What was the New Era Fellowship?

The provided sources do not directly state a detailed description or explanation of what the New Era Fellowship (NEF) is. However, based on the contexts in which it is mentioned, it appears to be a cultural and political organization associated with the Unity Movement, aiming to promote unity and democracy among non-European communities in South Africa.

Here are some points gathered from various excerpts:

- The NEF seems to have been involved in organizing lectures and discussions (Source [13]: "[The NEF] saw to the programme of lectures...").
- Members of the NEF, like Hassan Bavasah (Source [8], [10]), were deeply concerned about the bureaucratic behavior within the leadership, particularly focusing on Ben Kies (at Source [8]).
- The organization appeared to have branches in schools, given that Hassan was serving as organizing secretary and worked with students like Yousuf Rassool regarding program events.
- It seems that there were instances of disagreement with other movements or organizations on ideology or strategy within both the Unity Movement and the New Era Fellowship (Sources [11], [17] and also discussed in Chapter Three).
- The NEF was involved in activities related to education and unity, as seen in Source [2].
 
More information about the specifics can be derived from context.

### q19 — What was the Non-European Unity Movement?

According to Excerpt [2], The idea of non-European unity arose in 1943... yet nobody breathed a word about the three occasions when Dr. Abdurahman and D. D. T. Jabavu strove to create a non-European unity movement...

In Excerpt [9], it is stated that... A further act of simulation by the Congress Alliance was to copy the 3-pillar structure of the Unity Movement, except that they had a White fourth pillar, the Congress of Democrats.

Similarly, in Excerpt [11], it is mentioned that The ideals of the 10 Point Programme would not be achieved, as we expected, in one fell swoop, which refers to the goals and aspirations of the Non-European Unity Movement.

### q20 — Describe the author's involvement in cricket.

The author mentions their involvement in cricket throughout various excerpts.

Initially (Excerpt [1]), they note that "cricket and table tennis" were among their favorite activities, but it is implied that cricket received more importance.

Excerpt [7] also discusses teaching a forward defensive stroke to the cricketer Hewat. However, no information on whether this improved any particular skills is provided.

Their first exposure to competitive team involvement was probably in Excerpt [13], "Still with cricket on the brain, I tried to organize a union to facilitate the playing of cricket at High School and College level." 

However, Excerpt [15] reveals their most significant performance on the field: making 12 runs for the Kismets. They also report that this incident helped them get chosen in the Western Province Cricket tournament.

In Excerpts [16], it is mentioned that they "impressed" Mr. Kies with their bowling, as he had expressed dissatisfaction earlier about Nehru's leadership abilities.

Lastly, from these sources, there isn't enough information to assess the extent of the author's overall achievements or any sort of lasting legacy within cricket.

### q21 — Who was the author's mother?

The provided sources do not explicitly mention the author's full name; however, based on information in [19], the author's father married a woman named Ayesha.

### q22 — Who was the author's father?

The provided sources do not mention a specific name for the author's father. Yousuf (Joe) Rassool is identified as "I" or "me", but his father's name is not mentioned. However, it does mention that in [9] J. H. M Gool, Yousuf's paternal grandfather was known as Maulvi.

### q23 — Who were the author's siblings?

The provided sources do not contain specific information about all of the author's siblings. However, Excerpt [4] mentions that some uncles came to visit while the author was at home and would discuss politics with the family. This suggests that the author likely had several uncle figures in their life.

From Excerpt [5], we know that the author's Aunt Dij (Gadija), Uncle Mamoo, and possibly other siblings were present when Ralph Bunche stayed with the Gool house around 1937.

### q24 — Who were the children of J.M.H. Gool?

The provided sources do not contain a complete list of all the children of J.M.H. Gool. However, according to [1], Dr. Gool AH Gool was mentioned as J.M.H. Gool's son.

From [12]'s District Six - Lest We Forget, we can determine some of his other children:

- Hanief Ally (also mentioned, however there is another source referring to him)
 
According to [16] it mentions A.H. Gool as being born in 1886 which is J.M.H. Gools first son
 

The rest are mentioned in the excerpts [12]:

- Nurjehan (considered his daughter) had four children, whose names are:
  - Zainunissa (or Zaironesa)
  - Rasheda
  - Berina 
    and lastly her sister Yasmin, though Lallie (Yousuf’s mother) was pregnant with Yasmin (meaning she hadn’t given birth)

Other excerpts mention additional children:  

In [16] it is stated that J.M.H. Gool has an eldest brother 

- Nurjehan’s husband did not rear their four kids themselves, they went to different members of their family which can lead us to assume these are the children we are talking about:
  - Toetie (Yusuf does not remember Toetie’s real name)
  - Gadija (Aunt Dij) 
    who later becomes a teacher at her uncle’s school
  - Hamid (Midi)

This list might be incomplete.

### q25 — Who was I.B. Tabata?

I.B. Tabata is mentioned in [1], as one who had written a historical tract ("The Awakening") on which some political study group called The Forum Club had condemned him for failing to have a class analysis (as of source [3]). Some groups considered him the founder of the Non-European Unity Movement, but others thought his book might be "an alien intrusion into the body politic".

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century [1]. He was a medical doctor, and his mother, Gadija Dollie, was said to be a beautiful woman, a member of the Dollie clan [15]. Dr. Abdurahman was known personally to J.M.H. Gool and visited the Gool family mansion at Buitencingle Street [1]. He met and married Helen "Nellie" Potter James while studying at Glasgow University in 1893 [15], becoming the first doctor from his community to qualify as a physician.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources do not explicitly state what the primary connection between Gandhi and J.M.H. Gool was, but they suggest a close and significant relationship. The sources indicate that there was:

* A mutual respect and admiration for each other (Gandhi hailed as 'Mahatma' - "Great Spirit" by Dr. A. H. Gool in his farewell address to Gandhi on behalf of Port Elizabeth Indians [7]) 
* Frequent interactions between them, including visits from Gandhi to the Gools' residence at 7 Buitencingle during February 1914 [10] and October or November 1906 [5]
* Correspondence between J.M.H. Gool and Gandhi, which was provided by E.S. Reddy in his letter [9]
* The Gool family providing hospitality to Gandhis in February 1914 at their residence in Buitencingle
* Joint satyagraha efforts led by Gandhi, where the interactions with the Gools facilitated the process of gathering signatures for the removal of Asiatic laws and a reduction in taxation on the Indian community [11]

Gandhi's son was also reported to have been treated well by J.M.H. Uncle Hanief at Buitencingle and may have even fallen in love with an Abdurahman sister [10]. The close social relationship between Gandhi, Gools family members and Dr. Abdurahman is evident.

However, the nature of their connection remains somewhat unclear without specific information on what initially brought them together or how significant these interactions were compared to other relationships Gandhi had during that period.

Therefore, it can be said that J.M.H. and his family, particularly the Gool residence in Buitencingle, played an influential social role with regards to the support of Gandhi's efforts.

### q28 — Which organisations was the author involved in?

The provided sources do not provide a comprehensive list of the author's involvement, but some organisations mentioned are:

1. The New Era Fellowship
2. SRC (Student Representative Council)
3. NUSAS (National Union of South African Students) 
4. Moravian Church (presumably Rev Wessels was involved in its activities)
5. Kismet Cricket Club 

The author also mentions a school production and being part of the community care service based on several projects that were expected to be delivered at a conference, which may indicate involvement in certain community or social work initiatives.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

There is no specific mention of a direct relationship between the TLSA (Trafalgar Literary and Scientific Association) journal and the Non-European Unity Movement in the provided excerpts. 

However, it is mentioned that the editorial team of "The Torch" included Messrs. Kies and Edross (Source [10]), who are both associated with the Unity Movement. This suggests a connection between the Unity Movement and publications such as "The Torch." Further investigation would be required to determine if there was a direct relationship between the TLSA journal and the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to excerpt [16] (and corroborated by other excerpts), J. M. H. Gool arrived in Cape Town in 1884 via Mauritius, leaving behind his eleven-year-old bride in India.

However, a more detailed account is found in excerpt [10], which states that Joosub Maulvi Hamid had heard of the supposed prosperity and easy access to diamonds at the southern tip of South Africa (specifically, Cape Town) from India before deciding to settle there. It also mentions his exploration trip. 

Despite this additional context, the specific date in [16] remains consistent: 1884.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The sources do not provide detailed information about the Hanaffi Quwatul Islam Mosque. However, according to [2]:

It stood in Loop Street and was an outpost of Islam in an area becoming increasingly European, Jewish, and gentile.

Also, in [5], there is a mention that Public Slipper & Turkish Baths are adjacent to it, which suggests its location, but no further details about the mosque's significance or history.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool and J.M.H. Gool were married partners [6, 14]. Additionally, her brother Dr. A.H. Gool was also married to the daughter of J.M.H. Gools first wife [15], who was Bibi, implying that Cissie Gool's uncle was a common ancestor.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the sources, J.M.H. Gool knew or interacted with several notable historical figures. These include:

1. Mahatma Gandhi: As evident from Letter 12 [12], J.M.H. Gool corresponded with Mahatma Gandhi.
2. Dr. Abdurahman: According to Source [16], his grandfather, Haji Joosub Maulvi Hamid Gool's interactions with Dr. Abdurahman were close and collaborative. They frequently discussed national issues on grandpa's verandah at Buitencingle Street and likely met at various gatherings.
   (Specific dates or direct quotes mentioning these interactions are not provided in the given excerpts, we can infer their relationship based on source 16 which discusses their "close and cooperative" relationship)
3. Adam H.G. Mahomed: Source [1] reveals that Adam was President of the British Indian League.
4. Mr. Schreiner: A judge who participated in a case where Gool testified and, according to some reports, may have even had his wife's family associated with Dr. Abdurahman’s residence ([17], but it is uncertain if they knew each other prior to that), [3].
5. DDT Jabavu (David Diamond Thabo): According to source 16, he strove to create a non-European unity movement with Dr. Abdurahman.

While not personally known, J.M.H. Gool may have also interacted indirectly with:

1. J. M. Wilson: According to Source [17], it mentions that JMH had interaction with him at two different meetings in 1904 with Indian Opinion (28th January), and another record The Indian Opinion, May 19, 1906.
2. Dr. (Professir) Jamieson: Though the text in [8] describes Dr. Gool as accompanying Dr. Abdurahman attending a meeting attended by Dr. Jamieson at St Paul’s Mission School.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was a law passed by the South African government in 1950 (Source [14] says "the Group areas Act, was the lynch pin, the Big Bertha of the Apartheid weaponry." and Source[9] notes that by 1950, "...the Juggernaut of Aparthood was confidently grinding onwards") , which aimed to segregate different racial groups in South Africa and create separate living areas for Whites, Coloureds, Indians, and Blacks. The Act led to the forced removal and relocation of people from their homes and communities to designated Group Areas.

In District Six, a thriving cosmopolitan area in Cape Town, the Group Areas Act had a devastating impact (Source [1]). The government decided that the area was too "mature" for Coloureds and must be converted into a White area. As a result, people living in District Six were forcibly removed from their homes and relocated to slums on the outskirts of Cape Town. This caused immense suffering and disruption to the lives of those who lived there (Source [4]).

[Source list: (1), (2), (3) - mentioning Group Areas Act; (4) mentioned how sub-economic schemes affected District 6 residents]

### q35 — Who was Hassen Mall?

Hassen Mall is a notable figure in the given sources, described as an Indian South African student from Durban who stayed with Muddy (whose mother was keen to foster his friendship) and the subject of a deep regard for the narrator. He left behind an indelible personality imprint before departing for Durban in 1951 [5]. Throughout various excerpts, Hassen is portrayed as a brilliant cricketer, whose dynamism transformed many lives, including that of Tiny Abed, his team captain, at Cape Town's Curry's Fountain.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to the sources [5] and [8], several political organisations were active in the Cape Coloured community during the author's lifetime:

1. African People’s Organisation (APO)
2. New Era Fellowship (NEF), a political cultural organisation associated with the Unity Movement
3. Communist Party of South Africa
4. Fourth International of South Africa
5. Liberation League
6. Workers’ Party
7. Anti-Coloured Affairs Council (Anti-CAC) and its later incarnation, the Anti-Coloured Affairs Department (Anti-CAD)

Source [5] xiii notes that some organisations "functioning, moribund, and even defunct responded to the call" of the New Era Fellowship.

These organisations were involved in various causes, including anti-coloured affairs, opposition to the Coloured Council, and advocacy for civil rights.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Based on the sources provided, here is what can be gathered about Mahatma Gandhi's trip to South Africa:

Mahatma Gandhi [16] arrived in South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria [15]. He had been sent for because the accounts were kept in Gujarati, which was Gandhi's mother tongue. When it was time for him to return to India, he was persuaded to stay on to help the Indian community as, in 1894, the Natal Government introduced a bill to disenfranchise Indians [16].

Gandhi stayed on in South Africa and worked with the Indian community to address the issues they faced, particularly regarding their rights and segregation. He lived in a "white" area in the style of a barrister and resided next door to the house of Gardener Williams, who offered him an empty property to let at no cost while he extended his premises [16]. Gandhi was later invited back to Buitencingle Street, where he stayed with the Gool family on multiple occasions [17].

It appears that Gandhi had a significant impact on the Indian community in South Africa and was well-respected by its leaders, including Dr. A. H. Gool, who hailed him as 'Mahatma' – Great Spirit [18].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdurahman [8].

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources do not contain extensive descriptions of what District Six was like as a neighborhood before its forced removals. However, we can infer from various excerpts that it was a vibrant, cosmopolitan area with a rich history and diverse cultural influences.

Excerpt [2] mentions the second World War was "brewing" when they first moved to District Six and notes how their family had experiences related to war, Holocaust, and Apartheid during this time. Excerpts [3] and [7] describe the living conditions in District Six as having dirty streets, grimy pavements, cracked tiles, and inefficient house maintenance by the authorities.

Excerpt [16] emphasizes that "there was life, more zest, more fire, more warmth, and care" in District Six. It likens the neighborhood to a pulsing organism and notes its importance in shaping the author's education and worldview.

From these sources, it appears that District Six had complex dynamics of poverty, cultural influences, and racial tensions before the forced removals. However, without direct descriptions from a broader range of perspectives or detailed histories within District Six, we can't get an entirely accurate picture of what the neighborhood was like in its initial stages.

### q40 — What was the Unity Movement's boycott policy?

The sources are somewhat ambiguous on this point, with claims and counter-claims about the Unity Movement's views on boycotting.

In Extract [7], it is stated that "they had no intention to fight ideas with ideas. They demanded to choose their own chairman after which, of course, they intended to pass a vote of no confidence in the Unity Movement."

In Extract [9], there is no direct mention of the movement's boycott policy but mentions “Boycott the Election 1943”

In Extract [11] it states "The idea of non-European unity arose… Yes, these attempts floundered because the historical circumstances were not yet ready for it" relating to boycotting.

Again on [11] it is mentioned that "boycott and protest led by Dr. A. Abdurahman"

Boycotting has been a tactic widely used during 1943, as described in Extracts [14] - [17].

In Extract [18], there are references to boycotting Group Areas Act.

