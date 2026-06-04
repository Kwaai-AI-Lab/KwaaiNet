# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 56.9% (128/225) |
 | Avg judge score | 1.73/2.00 (40 questions scored) |
| Avg latency | 53684ms |

## Per-question results

| ID | Question | Hit rate | Judge | Sources | Latency |
|----|----------|----------|-------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 30082ms |
| q02 | Who are the author's children? | 3/3 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 29831ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 19788ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 22230ms |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 108980ms |
| q06 | Tell me about Buitencingle. | 4/8 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 96354ms |
| q07 | Who is the author's wife? | 2/3 (67%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 11845ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | 2/2 | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 29938ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | 2/2 | LEST WE FORGET -rev25.pdf | 22655ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | 2/2 | LEST WE FORGET -rev25.pdf | 65503ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 74505ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | 1/2 | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 75458ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf | 60915ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 82560ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | 1/2 | LEST WE FORGET -rev25.pdf | 50085ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 88853ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 43761ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | 2/2 | LEST WE FORGET -rev25.pdf | 86973ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | 2/2 | LEST WE FORGET -rev25.pdf | 97118ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf | 87950ms |
| q21 | Who was the author's mother? | 1/5 (20%) | 1/2 | LEST WE FORGET -rev25.pdf | 20698ms |
| q22 | Who was the author's father? | 2/4 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 26055ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | 1/2 | LEST WE FORGET -rev25.pdf | 16432ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | 1/2 | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 44290ms |
| q25 | Who was I.B. Tabata? | 3/5 (60%) | 1/2 | LEST WE FORGET -rev25.pdf | 29252ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | 2/2 | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 26889ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 78422ms |
| q28 | Which organisations was the author involved in? | 1/5 (20%) | 2/2 | LEST WE FORGET -rev25.pdf | 44557ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/6 (33%) | 1/2 | LEST WE FORGET -rev25.pdf | 34690ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 77380ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 35531ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 82650ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | 2/2 | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 43475ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | 1/2 | LEST WE FORGET -rev25.pdf | 65841ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | 2/2 | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 63558ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 39354ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 40705ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | 1/2 | LEST WE FORGET -rev25.pdf | 44785ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 97045ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf | 50400ms |

## Answers

### q01 — Who is the author?

The provided sources do not clearly state the name of the author throughout all excerpts. However, in [3] "Editor’s Note" it mentions "My father, Yousuf Rassool sent me text to the second edition in 2002..."

Later on in [4], it refers to "my great grandfather, Joosub Maulvi Hamid Gool" through a narrative voice believed to belong to "the author".

### q02 — Who are the author's children?

The author's children, as mentioned in [15] and [20], are:

1. Feyruz Rassool
2. Reza Rassool
3. Zarina Rassool 

And also their grandchildren (listed only in [15]) are:  
 
Joseph Rassool
Samuel Rassool
Alexander Rassool
Leonardo Rassool 
Dylan Rassool 
Lewis Rassool

### q03 — Who are the author's grandchildren?

The author's grandchildren are mentioned in excerpt [1]. Their names are:

- Joseph
- Samuel
- Alexander
- Leonardo
- Dylan
- Lewis 

Note:  These names are directly stated in the sources provided.

### q04 — To whom is the book dedicated?

The book "District Six - Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, and to his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [6].

### q05 — Who was J.M.H. Gool?

The provided sources do not contain a comprehensive biography of J.M.H. Gool, but rather fragmented information and mentions throughout the chapters.

From the sources:

[1] states that Bibi Gool (Peari Beghum) was the first wife of Haji Joosub Maulvi Hamid Gool, making her possibly his mother, given the mention in [20] that his father's name is missing.

[3] mentions J.M.H. Gool as a historical figure whose life's events are sparse and refers to him as "Grandpa" in later chapters.

[5] introduces JMH Gool with 11 "Grandpa" J. M. H. Gool, where he is described as being ten years old at the "ripe old age of ten," showing that the information about his life's events prior to his tenth birth date is limited or absent in these sources.

Later chapters mention him as a key figure involved in various organizations and social events in Cape Town during key periods:

[14] Gool was described as Grandpa, who had properties in South Africa, such as many in the Ma
[6].
[4]'s 
description of J. M. H. Gool 
from an early document as "an ‘Oriental trader’ and his participation
in a British Justice event 
points to 
to him being some kind of businessperson whose trade was involved with the Indian population.
he 

Other information about J.M.H Gool can be extracted from multiple chapters: He lived in Buitencingle and attended various political meetings as described in [4] and also belonged to organisations mentioned by other excerpts.  

He married Cissie (Zainunnisa) Abdurahman, the daughter of Dr Abdulla Abdurahman; his wife Bibi's name is missing from the later mentioned details about J.M.H and Cissie Abdurrahman Gool's involvement however.

These extracts collectively reveal that Gool was a  well-respected member of the community and seems to have been active in multiple capacities.

### q06 — Tell me about Buitencingle.

According to the excerpts [1] and [8], Buitencingle was a significant place in the life of Yousuf (Joe) Rassool's family, particularly his grandfather J. M. H. Gool. 

Excerpt [1] mentions that Yousuf's experiences at the age of ten in Buitencingle taught him about "the realities of the South African way of life", which he recounts later in his story.

Excerpt [8] does not specifically describe what Buitencingle looked like or its geographical characteristics. However, it is depicted as a place with several rooms where staff lived and worked, especially since Yousuf's grandmother tried to learn Afrikaans so she could communicate better with the servants.

Buitencingle appears to have been the home of J. M. H. Gool, who received notable guests such as Mohandas Gandhi during his visit in 1914 [2]. The area had a mix of cultures residing together until District Six declined due to overcrowding and social unrest [16].

In excerpt [3], Gandhi was invited to stay at the home of Morris and Ruth Alexander, but there is no further mention of any connection between them and Buitencingle. 

Excerpts [1] and [8] indicate the central role that J. M. H. Gool played in Buitencingle, both as a prominent citizen and property owner with more than 100 properties in the Cape Town area [2].

No information about what happened to the original Gool mansion in Buitencingle is provided. However, excerpt [17] suggests that one of Gandhi's sons fell in love with J. M. H. Gool's sister, a Muslim girl, which led Gandhi to initially disapprove of such relationships between people from different religions.

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

The author does not mention his wife by name beyond "Nazima" [1, 17]. She is referred to as a person with whom he had close collaboration and shared his research and writing process. According to excerpts [9], she was an academic researcher who urged him to do background investigation rather than relying solely on personal memories for his memoir. This insistence ultimately led to the discovery of new information about their family's history that transformed the book.

### q09 — Who was the author's grandfather?

The provided sources do not directly mention the author's name, but based on other connections between Yousuf (Joe) Rassool and J.M.H. Gool, it appears that Joe Rassool is the author, and his grandfather was J.M.H. Gool [15].

### q10 — Tell me about Kloof Nek.

Kloof Nek is mentioned in several excerpts as a beautiful and significant location in Cape Town. According to [1] by Yousuf (Joe) Rassool, it was a "place of haunting beauty" located on the eastern flank of the city, joining Table Mountain to Lion's Head. The area was notable for its natural scenery and was frequented by lovers, mountaineers, nature lovers, and rambler.

According to [5] also by Yousuf (Joe) Rassool, Kloof Nek was a popular destination among young people, including the author himself, who visited it with a schoolmate when he was fourteen years old. They walked through the gardens along Kloof Street until they reached Kloof Nek, where they spent time reading poetry together.

In [16], Reza Rassool notes that even though Kloof Nek was a beautiful place, it was also notable for being one of the areas affected by the Apartheid era evictions decades later.

Overall, it appears that Kloof Nek was a significant and picturesque location in Cape Town, with cultural and historical importance.

### q11 — What was the Teachers League of South Africa (TLSA)?

The TLSA is mentioned throughout the excerpts as an organization that supported equal education for non-Whites in South Africa during the apartheid era. It appears to have been a key player in the struggle against apartheid, with its members including teachers who were involved in the Teachers' League of South Africa.

Some specific information about the TLSA can be gleaned from the excerpts:

* The TLSA was mentioned as being at odds with the government's education policies, particularly in relation to the founding of the Cape Colony (Excerpt [10]).
* A speaker from the TLSA is described giving a well-researched and scholarly address on the education of non-Whites in South Africa (Excerpt [8]).
* The TLSA was mentioned as being part of the Non-European Unity Movement, which suggests that it aligned itself with other anti-apartheid movements (Excerpt [15]).
* Some notable members of the TLSA included Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O. Dudley, Solly Edross, E.C Roberts, and Abe Desmore.

Overall, it appears that the TLSA was a prominent organization in the fight against apartheid education policies and advocated for equal rights and opportunities for non-White South Africans.

The name is not an abbreviation.

### q12 — Who was Cissie Gool?

The provided sources reveal that Cissie Gool was a prominent figure in South African politics. According to [1], she was the second wife of Haji Joosub Maulvi Hamid Gool and the mother of several children, including Abdul Hamid, Goolam, Jane, Minnie, Fatima, Mariam, Zobeida, and Mohammed Hanief.

[2] notes that Cissie Gool addressed a pre-election rally in the Banqueting Hall, unexpectedly speaking in support of measures to deal with "influx control" and the street gangs. This was reportedly a shock to the narrator, as they had previously associated her with progressive politics.

In [4], it is mentioned that Cissie Gool supported the Torch Commando in 1953, and she became a part of the platform for an event, which seemed out of character given her association with the Unity Movement.

According to [5], Cissie Gool played a significant role in rallying the vote-less masses against residential segregation in 1937.

[15] describes a photograph of the Gool family, including Cissie Gool, with four daughters who are identified as Lallie, Jolly, Dija, and Toetie.

Overall, Cissie Gool appears to have been an active participant in South African politics during the mid-20th century.

### q13 — What was the All Africa Convention?

According to Excerpts [2] and [18], the All African Convention was an organization that existed. However, it is not explicitly stated what its goals or values were.

Excerpt [1] mentions a meeting of the All Africa Convention but only to describe how the New Era Fellowship members' plans to attend were thwarted due to authorities preventing the meeting from taking place in Bloemfontein.

It is mentioned again in Excerpts [2], [3], and [18] that Vic Wessels, Les Jacobs, and the narrator attended a conference of the All Africa Convention in Bloemfontein.

### q14 — Where was District Six and what kind of place was it?

District Six is a neighborhood in Cape Town, South Africa [3]. It was described by Joe Rassool as "a pulsing life... an unforgettable education and privilege" and he says that "to have been part of that pulsing life was to recapture Subjugated Cultural Histories of Cape Town (1897 - 1956)" [4].

District Six has a diverse population, with people from different ethnic and cultural backgrounds living together. The area is known for its vibrant community and social gatherings, including weddings and other celebrations [1]. According to Joe Rassool, the neighborhood was home to many Indian and Malay communities, and there were also synagogues and mosques in the area [17-18].

The place had a mix of commercial and residential areas, with shops like Dickman's Bakery, grocery stores, and even a synagogue [18]. The author mentions that Hanover Street, which ran through District Six, was lined with shops where people would shop for their weekly provisions.

In the 1950s, however, District Six began to die as the government implemented Apartheid policies, including declared Coloured Areas being turned White, and residents had to vacate or sell their properties at low prices [17].

Overall, based on Joe Rassool's accounts, District Six appears to have been a vibrant, multicultural neighborhood filled with diversity, music, and activity.

### q15 — What were the forced removals from District Six?

The provided sources do not contain information about the forced removals from District Six. The sources mention District Six in several excerpts, but only in connection with personal narratives and experiences (e.g., [1] mentions the Unity Movement and its critique of the Forum Club being "out of touch" with realities, [3] does not discuss forced removals), historical events in South Africa such as Gandhi's visit to Cape Town in 1914 ([7]), and the author's family history but do not discuss urban development, ethnic or nationality-based displacement, or forced relocation policies that have become notorious in the case of District Six.

### q16 — Who was Gandhi and what was his connection to the Gool family?

The sources do not contain explicit information about Gandhi's personal life prior to being associated with the Gool family. He is referred to as a significant historical figure who led deputations and lobbied in support of Transvaal Indians.

However, Gandhi's connection to the Gool family can be inferred from several points:

* Excerpt [3] mentions that E.S. Reddy contributed to the book by providing correspondence between Gandhi and J.M.H. Gool.
* Excerpt [5] notes that Yousuf (Joe) Rassool recalls visits made by Gandhi to Buitencingle, the Gools' residence in Cape Town.
* In Excerpt [13], it is noted that E.S. Reddy sent a cutting with a photograph of Gandhi taken on the roof of his grandfather's house during one of these visits.
* There are several instances where Gandhi interacts closely with members of the Gool family, such as his wife Kasturba being taken care of by them during her illness (Excerpt [12]).
* The Collected Works of Mahatma Gandhi Vol 5 and other sources referenced in Excerpts [11] and [16], indicate a close relationship between Gandhi and members of the Gool family.
* The statement from E.S. Reddy, in Excerpt [3], that Gandhi was received at the residence of the Prime Minister Botha by Emily Hobhouse, later by the Prime Minister and his wife, and Mrs. Gladstone "these meetings helped to bring about a settlement."

### q17 — What was Hewat Training College?

Hewat Training College is described as the first Post Matriculation Teacher Training facility for Coloureds in the country, and it opened with considerable fanfare for its first intake of students in 1944 [2]. It had previously been a Whites only Primary school. The college trained teachers for the upper years of the primary division, but the narrator starts teaching at a lower level than he expected at The Habibia Institute after attending Hewat College [5].

### q18 — What was the New Era Fellowship?

According to the excerpts, the New Era Fellowship (NEF) was a "cultural organisation" established in 1937 [13] that aimed to discuss various issues under its umbrella. It was associated with the Unity Movement and sought to unite non-European peoples against their common oppressors [3]. The NEF also held lectures on various topics, including politics, history, and culture, and it appears to have been a hub for intellectual and activist activities among the Coloured community in Cape Town [1].

The NEF played an important role in organizing opposition to the Coloured Advisory Council (CAC) and worked closely with other organizations such as the Teachers' League of South Africa to counter the CAC's policies [4]. Its members, including Hassan Bavasah, were active in promoting unity and resistance against Apartheid policies.

The NEF also organized a wide range of programs, including lecture series, conferences, and workshops, which aimed to educate and empower its members and supporters. The organization was characterized by its willingness to engage with complex theoretical issues, such as the analysis of the International Situation [20].

However, it is worth noting that there appears to be some tension within the NEF, with certain individuals criticizing its leadership for being too bureaucratic or out of touch with the grassroots movement [6]. Nevertheless, the NEF remained an important force in the struggle against Apartheid and a hub for intellectual and activist activities among the Coloured community.

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain a direct definition of the Non-European Unity Movement. However, based on various excerpts, it appears to be an organization working towards unity among non-European groups in South Africa during the mid-20th century.

According to [6], the idea of non-European unity arose in 1943 when the government introduced The Coloured Affairs on the so-called Coloured people, yet nobody breathed a word about the attempts by Dr. Abdurahman and D.D.T. Jabavu to create a non-European unity movement on three previous occasions.

In [11], it is stated that the Unity Movement was involved in various initiatives, such as the Ten-Point Programme and The Freedom Charter, which aimed for equality, education, and justice for all people in South Africa. However, the Nationalist Party and the Communists were not in full agreement with the Unity Movement's policies.

Source [7] mentions that the Progressive Party of Dr. Jamieson appealed for the "Coloured" vote at a meeting at St Paul’s Mission School on January 9th, 1904, where Dr. Abdurahman and Adam Mohamed were present.

Source [19] provides further information about the attempts by Dr. Abdurahman to create a non-European unity movement in 1903, when he led a deputation of protest against segregation and immigration acts alongside other notable figures like J.M.H. Gool and M. Enus.

In summary, the Non-European Unity Movement was likely an organization that sought to unite and empower various non-European groups in South Africa during the mid-20th century, advocating for equality, education, and justice through initiatives such as The Ten-Point Programme and The Freedom Charter.

### q20 — Describe the author's involvement in cricket.

According to the sources [10] and [16], the author played in a cricket match against Eastern Province and batted well, implying that he was part of a team representing his region or school at some point.

However, it seems that the author did not think he was among the top players like Tiny Abed, Hamid Khan, the Abrahams brothers, or Hassen Mall [16]. Despite this, he was selected to play in a cricket tournament and took part in one match, where his team played against Natal. 

It is also mentioned that Victor Wessels was part of a different team called Barnato Union [11], but I couldn't find information on whether the author or Victor personally participated in the games at this point.

We do know that the author's cricket career and experiences were influenced by events such as meeting Hassen Mall, his team captain. The text shows some emotional investment in sports - particularly table tennis and cricket.

The provided sources indicate that the author played cricket from a relatively young age, having just begun his 'cricketing days' at the point when cricket master Mylie Begg was already 'at his peak'. 

Lastly, my research indicates [14] that the author's team did change over time. The author notes some changes brought by Kismets Club to other leagues as a club example of breaking barriers.

I couldn't find information about any other details regarding the author's cricket game or what position he played in the specific tournament or match, as those are not specified anywhere in the provided excerpts.

### q21 — Who was the author's mother?

The provided sources do not contain explicit information on the author's mother's name, though they mention many female relatives including "Bibi" [5] and "Amina" (whom Aunty Annie gave birth to) [11].

### q22 — Who was the author's father?

The provided sources do not directly state who the author's (Yousuf Rassool) mother is, but they provide various information about the author's relationships and ancestors.

From [13], we can determine that Yousuf Rassool's father was Peerbhai or Peru, which is what his father became known as after being circumcised.

### q23 — Who were the author's siblings?

The provided sources do not contain specific information about the author's siblings (father was mentioned in some excerpts but the focus is on his sister).

### q24 — Who were the children of J.M.H. Gool?

According to sources [1] and [19], the children of J.M.H. Gool mentioned are:

* Ayesha (Lallie) (daughter)
* Mohamed Saaid, 
* Zohra (daughter), 
* Hanief 
* Goolam 
* Mohammed Saaid 
* Dija (Gadija) 
* Nurjehan's husband Sayed Fethi or Chota had four children: 
  * Gadija (Auntie Dij)
  * Hanief Ally
  * Toetie (real name not mentioned)
  * Hamid (Midi)

Note that the children are listed as mentioned in sources [1] and [19].

### q25 — Who was I.B. Tabata?

There is limited information in the provided sources about who I.B. Tabata was other than mentioning him as "Chairman of the New Era Fellowship" and the fact that he wrote a book called "The Awakening of a People." However, it can be inferred from other sections in the excerpt [7] where people like Jane Gool speak about his influence in shaping their thoughts on social politics.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdulla Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century [1]. He was a medical doctor, father of Cissie Gool (Zainunnissa Gool), the well-known activist and city councillor.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources mention the following connections between Gandhi and J.M.H. Gool:

[1] Gandhi lived in South Africa from 1893 to 1914, during which he organized the Indian community's resistance to discriminatory legislation. He knew J.M.H. Gool personally and was associated with him through his work in the Coloured People's Vigilance Committee.
[15] Gandhi stayed at Gandhi's home in Cape Town before his departure from South Africa in 1914.
[19] Indian Opinion records that Gandhi thanked those present at a function, including Mr. and Mrs. Gool, for their services to Mrs. Gandhi during her illness.

The sources also mention several instances where the two men interacted or were involved in joint activities:

* [15] In 1914, Gandhi visited Buitencingle yet again and was photographed on the roof of J.M.H. Gool's house.
* [19] Indian Opinion stated that Dr. Gool was unremitting in his attendance on Mrs. Gandhi during her serious illness.
* [9] The collected works of Mahatma Gandhi mention a letter from J. M. H. Gool to M. K. Gandhi dated 23rd January 1897.

These connections suggest a close relationship and frequent interactions between the two men, with Gandhi likely seeking advice or support from J.M.H. Gool in his mission to challenge discriminatory legislation in South Africa.

### q28 — Which organisations was the author involved in?

The author was an active member of several organisations, including:

1. The New Era Fellowship (NEF) - a cultural organisation aimed at discussing various issues and advancing the rights of the Coloured community.
2. The Anti-CAD (Colonial Advisory Board?) movement - which sought to challenge the authorities' control over the community's affairs.

Although not explicitly mentioned in all excerpts, it appears that the author was also involved with:

1. The Students’ Union at Hewat school
2. The Moravian Church of Rev Wessels

Additionally, the author mentions connections with various groups and individuals through their grandfather Grandpa, such as the Coloured People's Vigilance Committee and the Coloured People's Association [19, xv].

The text does not provide an exhaustive list of all organisations the author was involved in.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not explicitly state the relationship between the TLSA (Trafalgar High School Archives) and the Non-European Unity Movement. However, they do mention that Ben Kies was editor of the TLSA Journal [1] and Mr. Meltzer's involvement with Jack Meltzer being an advocate for the Trotskyist movement which opposed the Unity Movement.

We know from [7] that a political study group called "The Forum Club", who regarded themselves as Marxist Trotskyists, had a differing view to the Non-European Unity Movement [4].

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The sources do not contain detailed information about when J.M.H. Gool arrived in Cape Town, but they do contain details that can be pieced together to form an approximate arrival date and place.

From [20], it is mentioned that "It means that he departed from Bombay in April or in early May 1901". This implies that he left Bombay around this time, rather than arriving. 

However, we have no information on his departure point until [13] mentioning him having arrived in India from South Africa, and then later marrying Bibi.

In [2], it is said Gandhi "and Mr. H.O. Ally arrived by train in Cape Town from Johannesburg" but there is no mention of J.M.H.Gool.

However,  [8]'s statement says Williams attended the presentation for A.H.yPapers, Rhodes House Library, Oxford  with a note that Gool lived at Church Street, Capetown, S.A. There is also an indication JMH Gool went back to London in 1906.

Since it is mentioned in excerpt [20] that Grandpa had to be back in Cape Town at least nine months prior to the event (attending a political meeting) and if we assume the earliest possible arrival from London would have been around May 1905 (after his return), we can deduce 

 J.M.H. Gool arrived in Cape Town: No specific date is provided, but it was after he left Bombay in April or early May of 1901.

From where: Bombay

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque is mentioned in excerpts [3], [10], and [19] as being located on Loop Street. In excerpt [2], it is described as an outpost of Islam in an area becoming increasingly European, Jewish, and gentile, where the narrator's grandfather was a founder and life trustee.

In excerpt [28], it does not appear.

There is no information about what the Quwatul Islam Mosque signified or what made it so significant.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool was married to Abdul Hamid, who is mentioned in [6] as having been likely dropped off at Al Azhar University in Cairo during his trip with J.M.H. Gool.

According to [15], Cissie Gool was a spouse of one of the siblings (not specified which) of J.M.-H. Gool. She sported an angelic pose and identical head-scarves as the elder sisters in the picture in [15].

However, according to both [1] and [16], Abdul Hamid's mother, whose name is not written, had a close relationship with one of the residents at Buitencingle (i.e., J.M.H. Gool), Dr. Abdurahman’s grandfather and his clan were connected by the Dollie family.

Although there isn't clear, explicit information in the source excerpts about Cissie Gool's being specifically married to a child/children (or sibling) of Gool or even which Gool sibling she was married, it can be inferred that through marriage, her husband would have made him a family member of J.M.H. Gool’s (as children/siblings).

The provided sources do not contain the information as to who exactly Cissie Gool's spouse was to J.M.-H. Gool’s sibling.

However, [20] seems to indicate that Cissie and her husband lived at Buitencingle and had close relationships with other family members connected to Dr Abdurahman (i.e., the Dollies), so it is clear they were in proximity of J. M.H.'s household with Dr Abbuderman

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources mention the following notable historical figures that J.M.H. Gool knew personally:

1. Dr. Abdullah Abdurahman [1, 13-14] 
2. Adam H. G. Mahomed - President of the British Indian League and possibly a senior, financier, or mentor to Gool [3, 10]
3. J.M. Wilson (an Indian leader) - with whom Gool attended a meeting at least once [8, 18]
4. Gandhi was mentioned as someone who faced similar treatment [13, 19] 

There may be other notable historical figures mentioned in the source excerpts that personally knew J.M.H. Gool; however, those provided only mention these four individuals.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act is described in excerpt [3] as legislation that allowed White purchasers to buy properties from Non-Whites at below market value, with half of any profit going to the Group Area Boards. The Board had a scale of compensation for cases where sellers received less than their valuation (Excerpt [3]). 

The Act mainly affected District Six as it would have dispersed its people and destroyed their way of life by relocating them outside of Cape Town to barren areas of the Cape Flats (Excerpt [1]).

It also caused unrest and conflict among different ethnic groups, particularly between "Indians" on one hand, who were seen as an advantage in wealth and businesses, and Whites, who felt they had an unfair advantage (Excerpts [4] and [17]). The Group Areas Act was part of the larger system of Apartheid policies instituted by South Africa that aimed to create separate areas and opportunities for different racial groups.

The Non-White group most affected by this act were the Indians. While there is no information on how District Six specifically was affected, Excerpt [4] indicates that about half a million Indians lived in Natal at the time and 90% of them were working class or poor peasants, with only some having achieved considerable wealth despite the odds (Excerpts [14] and [17]).

### q35 — Who was Hassen Mall?

According to the sources [1]–[20], Hassen Mall was:

* An Indian cricketer (source: [1])
* A student from Durban who came to Cape Town to do medicine at the University of Cape Town. He later qualified with a degree in BA LLB without any hitches (source: [18])
* The team captain of the cricket team that went to the biennial tournament in Durban's Curry's Fountain in early January (sources: [19] and [20])
* A member of the Indian Congress, whose connections led him to know Fatima, his cousin, who welcomed the team when they arrived in Durban (source: [20]) 
* A bright and charismatic person with a strong personality that impressed many people he met. He was also described as being handsome, sporting a clipped brush moustache, and smoking a pipe (source: [17])
* A talented cricket player who was able to spin a wristy leg break that zipped like a circular saw (source: [17])

He seems to have had a significant impact on the lives of those who met him, including the narrator, with whom he formed a lasting friendship (sources: [3], among others).

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to the sources, several political organisations were active in the Cape Coloured community during the author's lifetime. Some mentioned include:

1. South African People's Organisation (SAPR)
2. New Era Fellowship (NEF) - a cultural organisation that issued a clarion call for all Non-European organisations to meet to counter the predicted assault on their rights
3. The Workers' Party - a front organisation of the Communist Party, activated at every election.
4. Unity Movement - a movement associated with the NEF and which had close ties with the South African Communist Party.

These organizations were mentioned in excerpts [12], [16], [19] and [20].

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi (not a name at birth but later acquired as a title) [12] lived in South Africa from 1893 to 1914 [1]. He organised the Indian community's resistance to discriminatory legislation, developing his method of non-violent resistance (satyagraha). Initially, he came to South Africa on a one-year assignment from Dada Abdullah in 1893 [17] to assist in a civil suit against a merchant in Pretoria because the accounts were kept in Gujarati, Gandhi's mother tongue.

Gandhi developed a close fondness for A. H. Gool and saw great potential for community service in him, building up his image through editorials of his career progress [19].

### q38 — Who was Cissie Gool's father?

The provided sources do not contain sufficient information to confirm who Cissie Gool's father is with certainty.

However, based on [2], Dr. Abdurahman's mother, Gadija Dollie, was related to the Ta'Als and had a connection through the Dollie family to the Abd urah man clan. The text does not mention anything about Dr. Abdurahman being Cissie's father.

According to [3], Abdul Hamid Joosub could be one of Cissie Gool's parent’s but this is not confirmed.


The most certain statement on Cissie Gool is found in [7], that she was an aunt and later [6] confirms Abd ul  Hamid’s presence in the picture, suggesting this could indeed be Cisse 's father.

### q39 — What was District Six like as a neighbourhood before the forced removals?

Before the forced removals, District Six was a thriving cosmopolitan area in Cape Town, South Africa. According to Joe Rassool's account, it was a place where people from different racial and cultural backgrounds lived together, with a rich history of political struggle against the injustices of Apartheid.

As described in Excerpt [14], the neighborhood had a distinct character: "It seemed as if everything was in place, yet something drastic had changed." The streets were grimy, dirty, and filled with the smells of stale newspapers, sour horse manure, and shouting vendors. However, to Rassool, it was home.

Excerpt [18] describes the community's rich cultural heritage: "Apartheid policy celebrated white racial supremacy... This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town..." The neighborhood had a mix of cultures, including freed slaves from America and the West Indies.

Excerpt [1] mentions that District Six was referred to as "inimitable" and that it would be "bulldozed out of existence." However, Excerpt [6] portrays it as a vibrant place: "To have been part of that pulsing life was an unforgettable education and privilege..."

From these excerpts, we can infer that before the forced removals, District Six was a densely populated and lively neighborhood with its own unique culture and character. It was a place where people from different backgrounds lived together, had businesses, and engaged in political activities.

However, it is also worth noting that Excerpt [13] describes some neighborhoods as being run down. "The Terrace used to be a stylish retreat for poor retired genteel Whites", but later "the drab shabbiness of the terraced houses" became apparent.

Unfortunately, the sources do not provide a comprehensive description of District Six before the forced removals. Further investigation into other accounts or historical records would be necessary to gain a more complete understanding of the neighborhood's daily life and social dynamics.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy [10, 16] was to refuse to cooperate with institutions or laws that they deemed unjust or undemocratic, such as the Group Areas Act and the Coloured Affairs Council. According to Yousuf (Joe) Rassool's account [10], some members of the movement ignored this policy and either appeared before the board themselves or sent their legal representatives to register their protest against decisions related to the reclassification of areas.

The boycott was described as a "negative weapon" by Jadwat and Seedat, who believed that having someone on the inside promoting the Movement's point of view is better than having nothing [16]. They also advocated for sloganizing and whipping up the crowd at public meetings, which Rassool criticized as showing a disregard for intelligence.

