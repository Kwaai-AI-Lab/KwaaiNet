# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 56.4% (127/225) |
| Avg latency | 29358ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 67923ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 33103ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 27361ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 26471ms |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: Uncle Aity (Mohamed Saaid Gool)] | 29444ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf | 30665ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 28060ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 26219ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 20500ms |
| q10 | Tell me about Kloof Nek. | 4/7 (57%) | LEST WE FORGET -rev25.pdf | 29848ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 30659ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Uncle Aity (Mohamed Saaid Gool)], LEST WE FORGET -rev25.pdf | 30253ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 28457ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29693ms |
| q15 | What were the forced removals from District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 30291ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 33201ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 23751ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 28053ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 31919ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 32292ms |
| q21 | Who was the author's mother? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 24945ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 27455ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 25926ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | LEST WE FORGET -rev25.pdf, [Graph: J. M. H. Gool & Co.] | 32099ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 24934ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 24872ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 29185ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 30309ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 27711ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 29823ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 32772ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Uncle Aity (Mohamed Saaid Gool)], LEST WE FORGET -rev25.pdf | 24879ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Uncle Aity (Mohamed Saaid Gool)] | 24449ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 32325ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 27467ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | LEST WE FORGET -rev25.pdf | 30363ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 29680ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 28834ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29703ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 18452ms |

## Answers

### q01 — Who is the author?

The author is Joe Rassool [9].

### q02 — Who are the author's children?

The author's children are: Feyruz, Reza, and Zarina [3] also, for his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [3].

### q03 — Who are the author's grandchildren?

According to excerpt [20], the author's grandchildren are:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

### q04 — To whom is the book dedicated?

The book, "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool (1984), is dedicated to the author's children: Feyruz, Reza, and Zarina; and his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [4].

### q05 — Who was J.M.H. Gool?

[8], [9] The author's grandfather, Joosub Maulvi Hamid Gool (also known as JMH Gool), is described as a notable figure and the second non-European from South Africa to qualify as a physician. He studied at Guys Hospital in London and married Cissie, daughter of Dr. Abdulla Abdurahman. J.M.H. Gool's properties were lost due to personal calamities, and his actions with regards to social exclusion are notable (See [10]).

### q06 — Tell me about Buitencingle.

Buitencingle is a name given to a street in Cape Town, South Africa. According to [15], the word "Buitencingle" has Dutch derivation and was pronounced as "Baytencingle", with "Ay" sound made rounder by pursing the lips ([20]). The word "cingle" literally means a belt or girdle, and Buitencingle referred to the outermost belt of Cape Town at that time ([15], [20]).

In the memoir "LEST WE FORGET -rev25.pdf", Joe Rassool vividly describes the street where he grew up in Buitencingle. He mentions seeing dignitaries such as Dr. Abdurahman's residence being nearby, which facilitated co-operation between Grandpa and Dr. Abdurahman ([18]).

Buitencingle is depicted as a prestigious neighborhood with wealthy merchants residing there, including Joe Rassool's grandfather Hamid Goollie Gooli ([14]). The street had a thriving trading community, as seen in [16], which featured photographs of Buitencingle and mentioned the significant role played by the residents.

As for its social dynamics, Buitencingle seemed to be quite cosmopolitan during Joe Rassool's childhood. The story describes influential people dropping by when Joe was just a young boy ([7]). In fact, Mohandas Gandhi would even visit there on some occasions, as indicated in [8] and further expanded upon in the context of Gandhi’s son falling in love with one of Goulam Gool's sisters, which is mentioned in [17].

Fazil Rassool mentions an Indian peer (later known as 5th Baron Headly) staying at their Buitencialg house; he left behind a navy-overcoat, and they later used it as a blanket when cold ([7]).

### q07 — Who is the author's wife?

The sources do not contain enough information to provide her name, just that she is an academic researcher. [13] and [19] mention the author thanking her, but only in relation to their roles as spouse and prompting him to incorporate research into his writing, rather than stating specific names or details about the wife herself beyond what can be inferred from a passage at [20], which shows she might have been named Nazima and possibly involved in reviewing drafts.

### q08 — Tell me more about the author's wife.

The provided sources do not contain sufficient information to provide details about the author's wife beyond mentioning her name as Nazima (in [19]) and stating that she is an academic researcher who urged the author to Ground his writing in background investigation ([16] and [17]).

### q09 — Who was the author's grandfather?

The author's grandfather's name is J.M.H. Gool [9].

### q10 — Tell me about Kloof Nek.

Kloof Nek [3, 4, 9 & 18] is described as a place of haunting beauty, connected by a saddle-shaped piece of land joining Table Mountain to Lion's Head. It "sprawled protectively on the eastern flank of the city with its head gazing inscrutably down on Table Bay." [18]

This area was also a popular destination for people from different walks of life, including lovers, mountaineers, nature lovers, and ramblers, as well as those seeking refuge in its cavelets, culverts, and bushes.

### q11 — What was the Teachers League of South Africa (TLSA)?

The sources do not provide a clear definition of what the Teachers League of South Africa (TLSA) was or did. However, they suggest that it was an organisation for teachers in South Africa that played a political role in promoting democracy and fighting against apartheid.

[5] states: "The TLSA knew that what the government did to the African today, they would do to the Coloureds and Indians tomorrow."

This implies that the TLSA was concerned with issues of racial justice and equality. [6] mentions that members of the TLSA were in the forefront of the struggle for full democratic rights.

[16] lists some of the key individuals associated with the TLSA, including Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, and others who are described as teachers and leaders.

[18] mentions that the Education Journal of the Teachers’ League of South Africa was a platform for training and publishing political/educational compositions by its members.

Overall, while the exact nature and goals of the TLSA are not clearly stated in the sources, they suggest that it was a significant organisation for teachers in South Africa during the apartheid era.

### q12 — Who was Cissie Gool?

The provided sources do not contain a comprehensive biography of Cissie Gool. However, excerpts [1]–[20] provide various details about her:

* She is mentioned as being involved in various activities and events related to politics, equality, and social justice.
* Excerpt [9] describes her addressing a meeting at the Banqueting Hall, where she spoke about issues including "influx control" and the street gangs (referred to as the “skolly” problem).
* In excerpt [2], Zain Gool is mentioned in an image of characters. It appears that Cissie Gool is part of this group.
* Excerpt [21] mentions her involvement with Uncle Doctor, which implies she may have been married to him (although not directly confirmed by any information in the provided excerpts).
* Her family connections and relationships are mentioned throughout the sources.

It's also worth noting that one can research more about Cissie Gool outside this document.

### q13 — What was the All Africa Convention?

The sources do not provide a detailed description of what the All African Convention (AAC) was. However, we can gather some information:

* Source [1] mentions that the author, along with Vic Wessels and Les Jacobs, attended the conference of the All African Convention in Bloemfontein in 1954. They stayed overnight at Dr. Murison's home.
* Source [17] implies that the AAC was part of a broader "movement" aimed at addressing issues related to education, democracy, and human rights in South Africa.

The actual definition and purpose of the All African Convention can be found outside these sources or is perhaps not available in the provided excerpts.

### q14 — Where was District Six and what kind of place was it?

According to the excerpts, District Six is located in Cape Town, South Africa. It's described as a place with "bare bulldozed acres" that had "more life, more zest, more fire, more warmth and care than many other places one could mention." (Excerpt [3])

The author remembers it as a vibrant community where people would gather, play, and socialize together. There were shops, a fish market, and a cinema in the area, which suggests that District Six was once a bustling commercial center.

It's also mentioned that the place was "bulldozed away" and eventually became "White-only". (Excerpt [7]). This suggests that District Six was forcibly cleared out as part of the apartheid regime's policy to separate racial groups in South Africa.

The author notes that the area was "undeveloped" and even had muddy roads, but it also had spacious schools, which were considered "location palaces". (Excerpts [11] and [13])

Overall, District Six seems to have been a dynamic, culturally rich community with its own unique character.

### q15 — What were the forced removals from District Six?

The provided sources do not contain explicit information about the forced removals from District Six. However, [9] states that District Six "began to die" as shops in Hanover Street started closing and the community was scattered due to Apartheid laws.

[10] mentions that District Six would soon be bulldozed out of existence, implying that it was being demolished or cleared for new development. 

However, [11] describes the original situation before any removals occurred: "the houses were all without ceilings or internal doors; ovens in summer and ice chests in winter."

[9] does discuss how Coloured owners of properties realized Group Areas Board's valuations of their properties were below market value, leading them to accept paltry offers from prospective white purchasers, which contributed to "the haemorrhaging" of District Six.

The events and context described suggest that the area was ultimately demolished as part of enforced racial segregation policies under Apartheid.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the excerpts [2] and [4], Mohandas Karamchand Gandhi, commonly known as Mahatma Gandhi, was a prominent figure who extended his contacts in the community, particularly among the poorer sections. He is mentioned as having visited the Gool family residence at 7 Buitencingle Street on several occasions.

Excerpt [2] mentions that Gandhi's correspondence with Joseph M.H. Gool (J.M.H. Gool) was published in Indian Opinion (October 6, 1906). It also states that J.M.H. Gool arranged for their meals at his place while they were at the station.

Gandhi's connection to the Gool family is further demonstrated by excerpts [4] and [10]. Excerpt [4] notes that Dr. Abdul Hamid Gool, Gandhi's uncle, corresponded with him whilst he was a medical student at Guy's Hospital in London (Indian Opinion, October 6, 1906). It also highlights the hospitality provided by the Gool family to Gandhis in February 1914.

In excerpt [7], it is mentioned that Grandpa, likely Joosub Maulvi Hamid, was proud and respected within the community. The text implies a strong bond between Grandpa and Gandhi.

Excerpt [14] provides additional details about the link between the Gools and Gandhis. It states that when Gokhale visited in 1912, he was a guest at Noor Bagh, and there is indication that Adam Gool married an English nurse, Laura Heffer, and they left the Cape for Australia.

Excerpt [20] adds to this understanding by mentioning Gandhi's appreciation for services rendered by Dr. Gool during his wife's illness and his own stay in Cape Town.

Overall, these excerpts demonstrate Gandhi's connections with the Gool family through personal relationships, correspondence, visits, and mutual respect.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college primarily for Coloured students, located opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds [19]. It was previously a whites-only Primary school, but was closed due to concerns about proximity to the prison and District Six. The college was used as the site of the first Post Matriculation Teacher Training facility for Coloureds in the country, and it opened its doors in 1944 with considerable fanfare [20].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural organisation that had been established in 1937 "to discuss everything under the sun" [6]. According to the sources, the NEF issued a clarion call to all Non-European organisations to meet to counter the predicted assault on the remaining rights of the “Coloured” community. It was associated with the Unity Movement and had its programme of lectures overseen by Hassan Bavasah as Organising Secretary [8 & 11]. The author of the memoir was involved with NEF and held a position within it, as he mentions himself chairing meetings at the Stakesby-Lewis Hostel [2] and later becoming chairman of the fellowship.

### q19 — What was the Non-European Unity Movement?

The provided sources do not directly explain what the Non-European Unity Movement (NEUM) is, but they mention it as an organization that brought the principles of non-collaboration, the refutation of "race", and the ideal of a non-racial society to the struggle for liberation in South Africa [6]. The NEUM's founding document mentioned that the majority of the oppressed were organized in non-political structures, such as sports bodies, churches, charities, mothers' unions, rate-payers, and civil societies, and needed only to be politicized and brought into the movement [8].

In [18], it is stated that the idea of non-European unity arose in 1943 when the government foisted The Coloured Affairs on the so-called Coloured people. This implies that the NEUM emerged as a response to the existing power structures and policies regarding "Coloured" individuals, aiming to unite those categorized as "non-white" under the apartheid regime.

The sources also mention individual leaders of the ALL African Convention (AAC), which is closely related to or possibly identical with the NEUM. Members mentioned include Tsotsi, Honono, Sihlali [9] and Enver Hassim, Errol Vawda, and Kader Essack who came from Natal [9].

### q20 — Describe the author's involvement in cricket.

The provided sources do not contain a comprehensive account of the author's entire involvement in cricket, but they do offer glimpses into various aspects of his association with the sport. Here are some key points:

* The author recalls playing cricket seriously thanks to his friendship with Hassen Mall (Source [9]). At that time, he was described as an "absolute rabbit" or a kebab (Source [9]), indicating a relatively novice level of skill.
* He describes mastering the forward defensive stroke and starting to build his batting skills (Source [9]).
* The author mentions his experience playing in various teams, particularly the Kismets Cricket Club, which he helped establish. This club was notable for its non-racial policy, which attracted players from different racial groups and transformed the Indian Cricket Union (Source [18]).
* He participated in a cricket tournament or match against other teams, where they were able to poach top cricketers and create new talents by having a non-racial team policy. An example of this is their acquisition of James Africa and "Woody" Woodman from Heather Cricket Club (Source [18]).
* The author's friend Muddy, with whom he became close friends through cricket, played for the Kismets as well.

However, what the provided sources do not offer a comprehensive or detailed account of his entire cricketing journey or significant events in which the author participated are:

* Specific matches, tournaments, or competitions that the author was part of.
* Any notable achievements or milestones he reached in cricket.
* How long he actively played cricket and whether his involvement extended beyond playing to coaching, umpiring, or leadership roles.

More information would be needed from other sources to gain a fully accurate picture of the author's involvement with cricket.

### q21 — Who was the author's mother?

The author's mother was Ayesha, also known as Lallie [13]. She had given birth to her after nine months on December 10th, 1900.

### q22 — Who was the author's father?

The provided sources do not contain specific information about the author's biological father. However, in [9], it is mentioned that "Malick Rassool’s marriage certificate courtesy Ciraj Rassool ", which implies a connection to the Rassool family.

### q23 — Who were the author's siblings?

The provided sources do not contain detailed information about the author's siblings. However, [5] mentions Peter Alexander Rassool (Peerbhai/Peru) and implies that he may be the author himself, as it refers to "I" and discusses the author's family relationships.

In [7], there is a mention of the author being born later than some of his cousins, implying that they had a sibling older than him. The name mentioned in this context is Nurjehan's twins: Nurjehan and Zarina.

### q24 — Who were the children of J.M.H. Gool?

The information about J.M.H. Gool's children comes from [12], [16], and [19]. 

J.M.H. Gool had multiple wives, including Bibi (sometimes referred to as Wahida) from India, with whom he had at least three sons: Kassim, Barrabhai, and Shaheen 

Shaheen is known to have been born in Cape Town and later took his own life in 1946 [13].

Barrabhai and his wife Nurjehan lived and likely settled in The Strand, a fishing village about thirty miles from Cape Town [16]. He fell in love with the place after they briefly stopped there.

Kassim is mentioned as having been born when Wahida arrived in Cape town, along with her children, around 1911 or later [15].

Additionally, there may have been other children of J.M.H. Gool who are not specifically named and described in these excerpts. 

It can be noted that the text indicates that Abdul was eight years old when the Gool residence was boarded up [18] .

### q25 — Who was I.B. Tabata?

According to [1], I.B. Tabata: 

I.B. Tabata: I.B. Tabata — occupation: chairman; affiliation: Author of The Awakening of a People

This is the only information provided about I.B. Tabata in the given sources.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century [1]. He was a medical doctor, father of Cissie Gool (Zainunnissa Gool), and was known personally to J.M.H. Gool, visiting the Gool family mansion at Buitencingle Street [2].

According to various sources, Dr. Abdurahman's grandfather was brought to South Africa as a slave, bought his freedom and became prominent in the community [4]. This closeness in kinship and geography with J.M.H. Grandpa facilitated co-operation between them and their organisations [4][6].

Dr. Abdurahman was also closely associated with Cissie Gool (his daughter) and J.M.H. Gool, suggesting a strong political connection within the community.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to the sources, particularly [7] and [11], Gandhi and J.M.H. Gool had a close relationship that went beyond friendship or acquaintanceship. Gandhi invested hope in Gool for community service and devoting public work for Indians, as mentioned in [12] & [13] that Gandhi's editorials reported Dr. Gool's career progress indicating Gandhi 's great potential in serving the Indian community.

[7] mentions that Gandi helped to "furbish my uncle’s surgery by puttying and staining the floor" indicating a close familial tie between them. J.M.H. Gool was also referred to as "my half-uncle" in [10]. 

Additionally, Gandhi saw Dr. Gool at his estate 7 Buitengelge in Cape Town multiple times from 1911 onwards, and the former helped to putty and stain the floors of his surgery, suggesting a strong personal connection between them.

While specific details about their relationship are scarce in these excerpts, they suggest that their bond went beyond mere acquaintanceship or professional ties.

### q28 — Which organisations was the author involved in?

The author was involved in various organisations. Based on the excerpts provided, here are some of the organisations mentioned:

1. New Era Fellowship (NEF) [14]: The author was actively involved with the NEF and attended conferences, such as the All African Convention in Bloemfontein.
2. Anti-CAD [12] or Anti-TLSA: The author advocated for affiliation with this organisation, which aimed to politicise various groups and bring them into the movement.
3. Trafalgar High School cultural society: As a student at Trafalgar High School, the author was part of the cultural society, where they debated, discussed, and developed their political analysis skills.
4. The Coloured branch of Smuts's United Party [14]: Although this organisation was not necessarily aligned with the author's ideological stance, they mentioned their involvement in discussions within the New Era Fellowship.

Additionally, the author mentioned being part of other groups or movements, but these were likely related to or intertwined with these organisations:

* The Movement: A broad term referring to the struggle for human rights and democracy in South Africa, which the NEF was a part of.
* The Cape Peninisula Students Union [19]: Although this organisation was established independently of the NEF, it shared similar goals and ideologies.

Please note that some of these organisations' relationships or direct affiliations with the author might be inferred based on the text; further verification may be required for absolute certainty.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to [2], the Teachers' League of South Africa (TLSA) "broke with its previous stance of non-political professionalism, and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society."

This indicates that the TLSA and the Non-European Unity Movement shared common goals and values, particularly in the area of education and human rights. The TLSA, recognizing its own role as educators and advocates for equality, aligned itself with the broader movement advocating for unity among non-whites against white oppression.

It was not specified what form this relationship took or if there were other aspects to their collaboration beyond mere shared goals, but [7] implies a certain level of commitment when it says that upon joining the TLSA, the author "could play an active part in the Liberator Movement".

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [5], the historical record is sparse for this period of J.M.H. Gool but it notes that his first son was born in 1886 and his testimony is recorded in Supreme Court transcripts in 1892.

However, a more specific answer can be found in [18] which mentions that when my aunt Jane (Wahida's daughter) died, I discovered that her date of birth had been the 19th March 1902. It then deduces that Grandpa had to be back in Cape Town at least nine months prior to this event, meaning he departed from Bombay in April or early May 1901 [18]. 

So J.M.H. Gool arrived in December 1900 with his party according to [20] (a cousin's interview of Aunt Jane), but came back to Cape Town for the second time in approximately April-May 1901.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The sources mention the Hanaffi Quwatul Islam Mosque in the following excerpts:

[2] mentions it once with reference number 3.

[8: 5 is actually a duplicate of previous pages, so I am skipping those], However [19] shows that Gool was trustee of this mosque but decided to abandon it along with other charitable donations due to financial difficulties after receiving advice from his accountants. It lost money, however, continued in the family through his second son's property ownership (Loop Street) taken over in 1911

### q32 — How was Cissie Gool related to J.M.H. Gool?

The sources do not state that Cissie Gool was directly related to J.M.H. Gool. However, they suggest a complex family connection through the Abdurahman and Dollie families.

[14] mentions that Dr. J.M.H. Gool's wife was Zainunnisa or "Cissie", who is the daughter of Dr. Abdulla Abdurahman.

[19] implies that Cissie Gool (Abdurahman) has a connection to the Gool family through her husband's involvement with Adam H.G. Mahomed, whom [20] describes as Gool's senior and possible mentor.

While it is not explicitly mentioned how Cissie Gool (Abdurahman) and J.M.H. Gool are related, their families appear to have intermarried or had connections through business and social interactions within the South African community in Cape Town.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the provided sources [3], [11], [12], and [17], it can be stated:

The notable historical figure that Joel mentions knowing personally is M.K. Gandhi, with whom he exchanged letters [13].

J.M.H.'s grandfather, also J.M.H. Gool is related to Abdul Wahid Bux, but the connection with him has gone astray [20]. There is a mention of Gool's association with Dr. Abdurahman in source [12], although it does not provide evidence that he knew Dr. Abdurahman personally.

In addition, there is no solid claim about any notable historical figure that Joel mentions knowing personally. He discusses other individuals' activities, like discussions between Gandhi and J.M.H., his grandfather's connections with figures, but the provided sources do not establish direct personal knowledge of a notable historical figure by Gool himself in this memoir.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act was legislation introduced in South Africa in 1950 that aimed to separate people of different racial groups into designated areas. The law required people to live in specific areas based on their race, and prohibited them from owning property or living elsewhere.

According to the sources [7] and [15], the Group Areas Act was a key piece of legislation in the Apartheid era, designed to Balkanise the country into separate ethnic components. It provided for the classification of people according to their racial group (Black, Coloured, Indian, and White) and forced them to live in designated areas.

In District Six, the Group Areas Act had a devastating impact. As mentioned in [1] and [20], the community of District Six was a thriving cosmopolitan area before the law was introduced. However, with the introduction of the Group Areas Act, the community began to disintegrate as people were forcibly removed from their homes and relocated to other areas.

As noted in [3] and [18], the Group Areas Board's valuation of properties was generally below market value, forcing Coloured owners to accept low offers from prospective White buyers. Furthermore, when someone sold their property at a profit, half of it had to go to the Group Area Boards (source [16]).

As for what happened to the residents of District Six, according to source [14], the government built new townships on the outskirts of Cape Town, such as Bonteheuwel and Hanover Park, in an attempt to entice people away from District Six. However, these estates were considered sub-economic and resulted in the inhabitants losing their municipal franchise.

The Group Areas Act effectively disfranchised the residents of these areas, leaving them without a say in local government (source [14]).

### q35 — Who was Hassen Mall?

Hassen Mall was a student from Durban who came to Cape Town to study medicine. He became a close friend of the author and played cricket with him. According to [5], he introduced and taught table tennis, specifically the game "cards", to the Allie family's children. Hassen was described in [17] as being short but radiating magnetism, and having a brilliant cricket career, including an indelible imprint of his personality when he departed for Durban in 1951. 

He also became the captain of the cricket team that included the author when they competed in the biennial tournament at Curry's Fountain in early January, according to [18] and [18].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources do not contain an exhaustive list of all political organisations that were active in the Cape Coloured community. However, some mentioned include:

* The New Era Fellowship (established in 1937)
* The African People’s Organisation (A.P.O.) ([11] mentions James Curtis and Rev R.A. Jackson as founders members)
* The Moravian Church
* The Communist Party of South Africa (membership included Cassim Amra, a perennial student at the University of Cape Town) [4]
* The Liberation League
* The Workers’ Party 
 The Coloured branch of Smuts’s United Party (believablely not mentioned before)

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi is referred to in several excerpts [1, 2, 3, 4, 9, 10, 13, 14, 15, 16, 17, 19]. 

He was a guest at the author's home, Buitencingle Street (excerpts [2] and [7]), where he stayed in rooms occupied by Mrs. Gandhi and her two sons, Manilal and Ramdas Gandhi (not mentioned here), as per an Indian Opinion article that had reprinted a travelogue of Ramsey MacDonald (excerpt [4]). He was impressed by the Gurukula at Hardwar, visited several times by Dr. Adam Gool (excerpts [2] and [3]) who repaid him with attentive medical care and wrote articles supporting his cause in Indian Opinion and other scholarly journals. Gandhi is also described as having a great deal of admiration for the late Mahatma Gandhi (not to be confused with himself) from Dr. Adam Gool's friend, Mr. Kies.

Gandhi was initially on assignment in South Africa in 1893 on behalf of Dada Abdullah, assisting in a civil suit against a merchant in Pretoria (excerpt [16]). The accounts were kept in Gujarati, Gandhi’s native tongue (excerpts [16] and also not mentioned in the provided source material), which is where this initial involvement began according to excerpt [17]. However, he ultimately played an important role in the Indian settlers’ struggle against discriminatory laws in South Africa.

### q38 — Who was Cissie Gool's father?

The provided sources do not specifically mention Cissie Gool's father. However, according to [4], her husband, Dr. Joosub (Gool), met his wife Bibi Gool in India when she arrived in Cape Town with two children; Cissie was the beautiful and clever daughter of Dr. Abdulla Abdurahman [2].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources, District Six was a cosmopolitan area in the heart of Cape Town that was thriving and vibrant. It was described as having "more life, more zest, more fire, more warmth and care than many other places one could mention" ([1], [18]). The neighborhood was home to people from different ethnic backgrounds, including Indians, Malays, and Africans.

The area was bustling with street life, and sources mention the fish market with its stalls, marble slabs, wet fish, fishy smells, and fish talk ([10]), as well as the Star Cinema or bioscope at the junction of Clifton and Hanover Streets ([10]). Street performances were common, including tap-dancing by The Cheshire Sunshine Boys ([11]).

The neighborhood had a history of political organizing and activism, with various groups aiming to achieve their goals through understanding and collective action ([12]). However, the community also faced challenges, such as poverty and oppression under Apartheid rule.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy was that if the "Coloured" voters insisted on voting they were asked to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip. [2]

