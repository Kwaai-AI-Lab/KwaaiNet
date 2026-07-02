# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 52.9% (119/225) |
| Avg latency | 28697ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 30462ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 32385ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 28304ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 26095ms |
| q05 | Who was J.M.H. Gool? | 1/8 (12%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 22759ms |
| q06 | Tell me about Buitencingle. | 4/8 (50%) | LEST WE FORGET -rev25.pdf | 25608ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 27078ms |
| q08 | Tell me more about the author's wife. | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 25965ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 25990ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 32019ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | [Graph: League of Nations], LEST WE FORGET -rev25.pdf | 24817ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 37656ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 27379ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 30075ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 28796ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 30060ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 27388ms |
| q18 | What was the New Era Fellowship? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 32734ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 31185ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 27508ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 27248ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 27914ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 28629ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 25645ms |
| q25 | Who was I.B. Tabata? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 33695ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 29727ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 30531ms |
| q28 | Which organisations was the author involved in? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 26747ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 26866ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Cape Town-Woodstock P.T.A.], LEST WE FORGET -rev25.pdf | 28114ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 32761ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 25075ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 28307ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 31710ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 23922ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town-Woodstock P.T.A.] | 31124ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 29324ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 32178ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 31607ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22510ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [9].

### q02 — Who are the author's children?

The provided sources mention that Y.S. Rassool is dedicating his book to his children [14][20]: Feyruz, Reza, Zarina, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

### q03 — Who are the author's grandchildren?

According to the dedication in [1] and [20], the author's grandchildren are:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

Note that this information is only present on the dedication page and is not repeated throughout the rest of the excerpts.

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET - rev25.pdf" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis. [18]

### q05 — Who was J.M.H. Gool?

[14] describes him as "Grandpa" J. M. H. Gool. The provided sources do not contain any additional detailed information about his life and activities. However, it is stated [18] that he was a well-known public worker whose family's residence was one of the most prominent in Cape Town. It also mentions that he participated in British Justice and had standing in the Cape community [18].

### q06 — Tell me about Buitencingle.

Buitencingle is a place in Cape Town, South Africa. According to the sources [8] and [18], it referred to a belt or a boundary area, being one of the outermost belts of Cape Town at that time. The name "Buitencingle" should more correctly be pronounced "Baytencingle", with a rounded "Ay" sound when pursing the lips.

In the memoir "Lest We Forget - Rev25.pdf", Buitencingle is described as a thriving trading community, and the home of the author's grandfather, Haji Joosub Maulvi Hamid Gooli. The author mentions that their family had a thirteen-roomed mansion in Buitencingle Street, where he spent his childhood.

Buitencingle was also an important hub for political activism during the apartheid era. According to source [15], Dr. Abdurahman's residence was located nearby, and there was likely collaboration between Grandpa Gooli and Dr. Abdurahman in various ways, including informal discussions and formal cooperation between their organizations.

The author also recalls a visit from Gandhi in 1914 (February-March) to the family home in Buitencingle, as mentioned in source [13].

### q07 — Who is the author's wife?

The author's wife is Nazima [2]. Additionally, it is mentioned in excerpt [6] that his wife is also an academic researcher who urged him to ground his writing in background investigation.

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about the author's wife, apart from mentioning her name as Nazima and her role in urging the author to conduct background research to ground his writing.

### q09 — Who was the author's grandfather?

The author's grandfather was Dr. J.M.H. Gool [16]. The text also refers to him as "Grandpa" (e.g., [10]).

### q10 — Tell me about Kloof Nek.

According to the sources [3] and [18], Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion's Head. It sprawled protectively on the eastern flank of the city with its head gazing inscrutably down on Table Bay. The Glen, also referred to as Kloof Nek or the "Glen", was a place of haunting beauty located near the slope where Signal Hill was situated.

Additionally, [17] and [20] describe Kloof Nek as an area that was frequented by lovers, mountaineers, nature lovers, and ramblers. However, there were also down-and-outs known as "Bergies", who found refuge in the various cavelets, culverts, and bushes in the Glen to drink away their sorrows.

The Glen at Kloof Nek was a popular spot where friends, like the narrator and Persotem Patel, would go to study, enjoy nature, and engage in literary pursuits. The area's beauty was particularly appreciated by the narrator who described it as a "haunting" beauty.

### q11 — What was the Teachers League of South Africa (TLSA)?

The book does not explicitly describe what the Teachers' League of South Africa (TLA) or TLSA's full mission and goals were, but it is mentioned that "it was a teachers union which stood for democratic rights for its non-White members" [6]. It also says the TLAS was pro-democratic [5], in the forefront in the struggle for full democratic rights for non-citizens in their homeland [6].

### q12 — Who was Cissie Gool?

The provided sources contain various references to Cissie Gool. 

[2] mentions that Cissie Gool is confident in a picture taken, where she holds an angelic pose.

[3] quotes that a tremendous shock to Joe Rassool (the author) when he happened upon her addressing a meeting at the Banqueting Hall.

This suggests that Cissie Gool was likely involved in the community and may have held significant social or political influence. [10, 13] also indicate her involvement with various movements and associations (e.g., "unity") but doesn't explicitly describe what she did exactly within those contexts.

[4] implies that she associated with notable individuals such as politicians, artists, or musicians during certain periods.

It is not clear from the sources how influential she was, but it appears that Joe Rassool held significant admiration and respect for her.

Additionally, [6 & 18] suggest tension between Cissie Gool (or at least a C. Gool) with individuals such as Dr Abdurahman's brother-in-law  ("brought Cissie and my uncle Dr." ) or Joe Rassool seems to be shocked that she supported measures for "influx control" which could put the Coloured people against other sections of the community.

From [15], we learn that she, Cissie Gool, almost single-handedly rocked the government because she called a massive demonstration of the Liberation League on Grand Parade in 1937.

In [16 & 17] we see numerous mentions and references to her name. She's associated with events and individuals such as Nellie Abdurahman (one of her friends), but it is not explicitly stated what relationship (personal or professional) they shared.

Given these references, Cissie Gool seems to have had active roles within the movement for the rights of Coloured people in South Africa, particularly during the 1930s.

To recap:

Cissie Gool was a woman involved with various social and political movements, had associations that crossed ethnic boundaries (based on her involvement with different groups and notable figures), held possible admiration in at least one account and demonstrated involvement particularly with “influx control”, though this detail is controversial.

### q13 — What was the All Africa Convention?

The provided sources do not contain detailed information about what the All African Convention was. However, it can be mentioned that [2] "We all eagerly purchased our copies" of The Awakening of a People by I.B. Tabata, "and set about distributing them among our friends and acquaintances." The book records the coming into existence of the All African Convention, an important organisation in the Non-European Unity Movement.

According to [3], it was a major concern that not only the Nationalist regime attacked on the educational front, but also directed its offensive against the African peasantry, with the "Rehabilitation Scheme" being one such measure.

### q14 — Where was District Six and what kind of place was it?

The provided sources do not contain explicit information on its exact location. However, we can infer that District Six was situated in Cape Town, South Africa. It is mentioned as "the outermost belt of Cape Town" [18].

As for the nature of District Six, the sources portray it as a vibrant and lively community with a strong sense of life, zest, fire, warmth, and care [19]. The author recalls walking away from it like Lot without looking back, indicating that he remembers it fondly [10]. 

It was an area where people lived in close proximity, often sharing small spaces, such as houses or rooms. People came together for entertainment, with activities like tap dancing (The Cheshire Sunshine Boys) and begging for pennies.

District Six was also home to many markets and shops. The fish market, the Star Cinema or bioscope, Hanover Street, and Caledon Street are all mentioned in connection with the area.

Furthermore, District Six had a significant presence of Indian culture, as indicated by words like "Zoot suit" [13] and the mention of Gujarati community parlance ["kebab"].

The area was an integral part of the apartheid scheme, which intended to create separate racial zones. However, in the case of District Six, its fate was sealed: it would be bulldozed out of existence to make way for Whites-only residential areas [10].

### q15 — What were the forced removals from District Six?

The provided sources do not explicitly mention the forced removals from District Six. However, based on general knowledge and the context of the memoire "Lest We Forget", it can be inferred that the author, Yousuf (Joe) Rassool, is referring to the event during which he was a university student.

The forced removal of residents from the District Six area, including those of Indian background like Joe Rassool's family, began around 1967. However, there are no specific details about this occasion in the document "Lest We Forget - rev25.pdf" that I can directly cite [1-20].

### q16 — Who was Gandhi and what was his connection to the Gool family?

The sources indicate that Mahatma Gandhi [1] was a Hindu politician from India. His connections to the Gool family are described in various excerpts:

* Dr. A.H. Gool, Joe Rassool's uncle, was involved in Indian politics in Cape Town and became one of the joint secretaries of "The Cape British Indian Union" co-founded by Gandhi [12].
* Gandhi stayed at 7 Buitencingle Street, the Gool family's residence, during his visits to Cape Town. He mentions helping to furbish Dr. Gool's surgery in 1911 [12]. 
* The Gool family hosted Gandhi and other dignitaries, such as Gokhale [7].
* Dr. A.H. Gool repaid Gandhi with attentive medical care, wrote articles for Indian Opinion, and served the community but did not follow a path of social activism like Gandhi [14].

Gandhi was also a guest at the Gool residence multiple times:

* In 1909, he invited his uncle to collect signatures for a letter in support of Transvaal Indians while Gandhi was on deputation to London [13].
* Gandhi visited Buitencingle Street again in February-March 1914 and stayed with the Gools during this period.

### q17 — What was Hewat Training College?

According to the sources, Hewat Training College was a teacher training college that accepted students from a Coloured background [2]. It trained teachers for the upper years of the primary division [16] and was known for having a conservative educational philosophy [3]. The staff at Hewat had been recruited from the United Kingdom, chosen for their missionary outlook and conservative ideas [2], which meant that it was not an incubator of left-wing radical ideas [2].

It's also mentioned in source [17] that Hewat Training College was part of the non-Departmental recognition system, implying that it wasn't sanctioned by the Department (likely the Department of Education). This lack of recognition led to a decline in membership as teachers with further career ambitions joined other recognized bodies [16].

### q18 — What was the New Era Fellowship?

The sources do not provide an explicit definition of the New Era Fellowship. However, through various excerpts, it can be inferred that the New Era Fellowship (NEF) was a political-cultural organisation associated with the Unity Movement and active in Cape Town, South Africa (Source [3]). It appears to have been involved in various activities, such as holding meetings and lectures, distributing pamphlets, and discussing current events and policies (Sources [2], [6], [7], and [19]).

The NEF seems to have had a significant impact on the Coloured community in Cape Town, particularly with regards to issues of identity, unity, and democratic rights. As mentioned in Source [16], "The New Era Fellowship struck a chord that resonated in the Coloured community." The NEF also played a role in promoting the idea of equality and resistance against the apartheid regime.

One of the key features of the NEF was its programme of lectures and activities, which were overseen by an Organising Secretary (Source [10]). Members and leaders of the NEF, such as Hassan Bavasah and Ben Kies, were involved in various aspects of the organisation's work (Sources [8], [10], and [20]).

It is also suggested that the NEF was involved in debates within the Unity Movement regarding issues like nationalism, class analysis, and the role of youth in social and political change (Sources [4] and [11]). Overall, the New Era Fellowship appears to have been a significant organisation in the context of South African history, particularly in relation to anti-apartheid activism and cultural development.

### q19 — What was the Non-European Unity Movement?

The provided sources [1]–[20] collectively describe the Non-European Unity Movement (NEUM) as an organization that emerged in 1943. The NEUM brought together people from different racial backgrounds who were working towards a common goal of achieving equal rights and a democratic society.

As described in source [4], the idea of non-European unity arose in 1903 when the government introduced ordinances aimed at segregating Indians, which led to protests led by Dr. Albert Luthuli. However, it wasn't until 1943 that the NEUM was formally established.

Source [6] mentions that the founders of the Non European Unity Movement were the "young Turks," including Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam.

The movement aimed to achieve several key goals, including:

* The principle of non-Collaboration
* The refutation of "race"
* The ideal of a non-racial society

As stated in source [7], the Unity Movement was involved in various activities, including trying to address issues faced by peasants. However, it's noted that the movement struggled with internal conflicts and disagreements.

The NEUM had affiliations and alliances with other groups, such as the Anti-CAD (source [8]), which reflects its efforts to engage in politics and promote social change.

In summary, the Non-European Unity Movement was a key organization in South African history that played a significant role in promoting racial equality and fighting for democratic rights.

### q20 — Describe the author's involvement in cricket.

The provided sources do not contain detailed information about the author's initial interest or ability in cricket, however they do reveal that he was part of a cricket club known as the Kismets and started playing seriously due to his friendship with Hassan Mall [8]. As a member of this club, at Rosmead, he scored 12 runs out of 200, being hailed as some sort of a hero, although he did not make it into the final team that went on tour to Johannesburg [10].

The author also recounts calling a meeting to establish a cricket union for High School and College students but was thwarted by bureaucracy [11]. Additionally, he seems to have been involved in organizing fixtures in this potential union.

### q21 — Who was the author's mother?

The author does not mention their own birth in this passage. However, [14] states that the author's name is Peter Alexander Rassool, and he mentions his mother in several places, but her full name is not given. 

[3] introduces a man from the author's family (Malick Rassool) with a marriage certificate that shows him as the father of the person who would have been the author's mother. Her birth name is mentioned, [9] and [16] mention the mother indirectly.

We know that [3] mentions "My father was handsome, bronzed like his mother, with hair slicked back, tall and sturdy." Thus at least we can say that the author has a vivid recollection of her appearance.

### q22 — Who was the author's father?

The author's father is not explicitly stated in the provided excerpts. However, based on [10] and [20], it can be inferred that the author's family name is Rassool. In [9], Dr. Gool is referred to as the author's "half-unucle", indicating a connection by marriage or relationship with the author's biological father. In [10], it is stated that Peter Alexander Rassool, formerly known as Peerbhai, had all three of his wives circumcised, and was born in Mauritius.

### q23 — Who were the author's siblings?

The provided sources do not contain comprehensive information about the author's siblings. However, [3] mentions a brother, Kassiem, and [7] refers to "my sister, Yasmin" but with her name also mentioned as Jessie.

Additional context from [14]: The twins Nurjehan and Zarina are not siblings of Joe Rassool or his story being told; they appear in a different narrative line. Their uncle is M.S, likely Malick Seedat or someone similar, who appears to be their father.
 

Given these clues, we can infer that the author has at least two identifiable siblings: Kassiem (brother) and Yasmin/Jessie (sister).

### q24 — Who were the children of J.M.H. Gool?

The sources do not provide a complete list of the children of J.M.H. Gool, but it is mentioned in [19] that by the end of 1899, J.M.H. Gool had four children, and shortly after, his fifth child was born during Hajja, the Islamic pilgrimage.

Additionally, in [17], Rasheda and Yasmin are mentioned as his daughters. 

The other siblings mentioned in [20] is not one of J.M.H.'s known children; Adam H. G. Mahomed's relationship to J.M.H.Gool appears to be that of a senior, financier, or possible mentor.

In [15] Toetie and Hamid (Midi) are mentioned as family members but there is no confirmation they were J.M.H. 'S biological children.

In [16], it mentions several grandchildren: Hoosain Ally and Nurjehan's Uncle Aity is Sa-aid in relation to J MH Gool

### q25 — Who was I.B. Tabata?

The provided sources do not contain detailed information about who I.B. Tabata was, but they mention him frequently throughout the excerpts.

From the sources, we know:

[2] Shortly after becoming chairman of the New Era Fellowship, Mr. I.B. Tabata completed his book The Awakening of a People. [8] Aunt Jane's home was the focal point of the meeting of the Unity Movement, and Auntie Minnie's nephew, Tabata, is likely referring to I.B. Tabata.

[3]  The author finds himself cast beyond the pale by the Gool branch of the family as a traitor for siding with Kies instead of Tabata.

[9] The author mentions that Victor Wessels and Cosmo Pieterse were banned for 5 years under the Riotous Assemblies Act, but I.B. Tabata is not mentioned alongside them.

[12] The discussion centered on the Labour Party and the Parliamentary road to Socialism, with Mr. Tabata being a central figure in this discussion.

[19] A political study group called The Forum Club, who regarded themselves as Marxist Trotskyists, had condemned Mr. Tabata's book "The Awakening" for failing to have a class analysis.

From these mentions, we can infer that I.B. Tabata was likely a prominent figure within the New Era Fellowship and the Unity Movement, being in charge (likely chairman) of the New Era Fellowship. He also wrote a book called "The Awakening of a People". Additionally, a political study group, The Forum Club, criticized his book for lacking Marxist analysis.

However, to provide more information about I.B. Tabata's background, we would need additional sources beyond what is provided in these excerpts

### q26 — Who was Dr. Abdullah Abdurahman?

The provided sources say that: [1] Dr. Abdulla Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was a medical doctor and father of Cissie Gool (Zainunnissa Gool), the well-known activist and city councillor.

He is also mentioned as being brought up by his grandfather, who had been a slave but became influential in the community [13]. The closeness between his family's residence, Buitencingle, and Dr. Abdurahman's home, the Abdurahman residence in Loop Street, likely facilitated co-operation and collaboration between these individuals [18].

According to another source, he was also a trustee of a local mosque [14]. 

There is not much more information available about Dr. Abdullah Abdurahman in the provided sources.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources show that Gandhi and J.M.H. Gool (also referred to as J.M.H.) had a close connection, likely based on mutual respect and admiration. There is evidence of significant interaction, collaboration, and affection between them.

Gandhi is said to have helped with the refurbishment of Dr. J.M.H. Gool's surgery in 1911 ([13] & [12]) by puttying and staining the floor, which demonstrates Gandhi's willingness to help his friend.

Dr. Gool repaid this favor with attentive medical care for Gandhi and wrote articles on behalf of Indian Opinion ([13]). Their close relationship is further emphasized by their mutual respect and trust in each other's skills and intentions.

It appears that Dr. J.M.H. Gool was a key figure in the community, providing hospitality to prominent individuals such as Gandhi, who regularly stayed at the Gools' residence. The closeness of this friendship can be inferred from their collaborative efforts in promoting social activism and public service ([5] & [13]).

Gandhi viewed Dr. J.M.H. Gool as having great potential for community service and invested hope in him by building up his image through editorials reporting his career progress ([12] & [13]).

A crucial time of co-operation was during the satyagraha campaign, where Gandhi received visits from various officials at the residence of Dr. J.M.H. Gool ([15]).

### q28 — Which organisations was the author involved in?

The author, Joe Rassool [9], was involved in several organisations:

1. New Era Fellowship (NEF): A cultural organisation founded in 1937 to discuss various topics, which became increasingly politicised and became a key player in the democratic movement.
2. The Anti-CAD: An anti-apartheid organisation that the NEF affiliated with [2].
3. Cape Peninsula Students Union: Established in 1953, which was seen as a threat by the author and his group, the New Era Fellowship.

The author also mentions representing the New Era Fellowship at the conference of the All African Convention in Bloemfontein in 1954 [14].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to excerpt [2], the League (TLSA) took "the new road" and joined the Non-European Unity Movement (NEUM), acknowledging that the struggle for equal education is also bound up with the struggle for a democratic society. This was after they broke with their previous stance of non-political professionalism.

In excerpt [5], it is further stated that this step was not achieved without a fierce struggle, but in the end the young Turks succeeded in ousting the backward leadership and taking control of the TLSA. 

The relationship can be summarized as that of alignment between the two movements; however, this process was not smooth with resistance from within the TLSA before they agreed to join forces with the NEUM.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [9], J.M.H. Gool arrived in Cape Town but the excerpt doesn't specify when he came to Cape Town, only that a telegram was sent from Mahatma Gandhi to him on December 6, 1900.

However, [14] indicates that Adam H. G. Mahomed and his father, possibly J.M.H. Gool's senior ("financier and possibly mentor" is mentioned in excerpt [16]), were present at a meeting of “Cape Coloured men” held in April 1897 to protest against their inclusion under the Transvaal pass laws.

[8] tells us that there was already an established business by someone named J. Gool, as listed in The 1894 Argus Annual and South African Directory lists a “J. Gool, draper, corner of Waal and Loop Sts, CT” (CT is the abbreviation for Cape Town). However it does not indicate that this person was J.M.H. Gool specifically.

Also in [8], his letterhead in 1897 names basket manufacturing and fish curing among his trades which may indicate some time had passed between him being in South Africa already and when he moved to bigger premises, suggesting possible years of establishing himself in Cape Town before arriving at these points in the historical record.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was an Islamic mosque that stood on Loop Street in District Six. According to [1], it was the founder and life trustee of which was my grandfather, completed in 1898. The author would often go there with family members to celebrate Eid festivals. It had a whitewashed structure with a domed minaret topped by a spire with a moon and star from which the muezzin called the faithful to prayer (Source [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool is stated to be the wife of Uncle Doctor [14]. She then appears as the mother in several photos including one where she and her husband Abdul Hamid are in the company other family members, with daughters Lallie, Jolly, Dija, Timmie, and Zohra [13, 18].

Cissie Gool is stated to be also wife of Abdul Hamid Gool not Uncle Doctor. 

The book states that Dr. Abdurahman was my grandfather's close associate and friend (no blood relation between them but closely associated) [4].

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources, J.M.H. Gool knew Gandhi personally [3], which is likely what contributed to Gandhi's interest in bringing a bride for J.M.H.'s son (Haji Maulvi Hamid Gool) from India [14].

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not contain a comprehensive overview of the Group Areas Act. However, they do mention its implementation in Cape Town, making reference to the law that divided South Africa into separate areas for different communities.

From [11], we know that most of South Africa was already divided into separate areas before the Nationalists came to power in 1948 and introduced their Apartheid legislation, which included the Group Areas Act.

From [14] onwards, various mentions are made about the law's effect on Indian populations in Natal. For instance, in [15], it is stated that the South African Indians were "the Non-White group most aggrieved by the Group areas Act."

From [16] and [18], we find discussions about delaying inevitable displacement through hiring lawyers and employing loophole mentality, which ultimately proved futile.

[16] indicates that while people tried to employ these strategies, they eventually had to cope with the loss of their livelihoods and community due to the implementation of the Group Areas Act in Cape Town.

Thus, from these excerpts, we understand that the Group Areas Act was at work in several parts of South Africa during this period but was causing significant disruption specifically for Indian communities. In reference to District Six, there is a statement that its residents lost their municipal franchise and were effectively disfranchised due to living in sub-economic houses by way of other acts of legislation that accompanied the Group Areas Act.

In [8], it's mentioned that "the awful thing about these schemes was / that they resulted in the inhabitants losing the municipal franchise because they were living in sub-economic houses! The residents were effectively disfranchised!"

### q35 — Who was Hassen Mall?

[3] describes Hassen as "a brilliant cricketer and spun a wristy leg break that zipped like a circular saw." [7] notes that Hassen Mall is not tall, but he radiates magnetism and has a clipped brush moustache. He had an excellent cricket game.

Additionally, Hassen is a student from Durban who stayed in Cape Town to do medicine (as stated in [16] & [17]).

Hassen was also involved with the fight against apartheid: [8] mentions that he understood "the logic of my argument, but... the dilemma of the people".  It's said [6] that he fought for full democratic rights and not just fighting the Group Areas Act. 

In a cricket tournament [9], Hassen Mall is described as a "fearsome fast bowler" when his brother bowed with this speed, earning him many accolades.

Hassen was Chairman of the Athlone branch TLSA [10].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to the provided sources:

* The New Era Fellowship was a cultural organisation founded in 1937 that called for unity among Non-European organisations (Source [1 is missing, but it seems [20] is about this topic])
* The African People’s Organisation (A.P.O.)
* The Communist Party of South Africa
* The Fourth International of South Africa
* The Liberation League
* The Workers’ Party
* The Coloured branch of Smuts's United Party

Note that the author's lifetime is not explicitly stated in the sources, but it appears to be around the mid-20th century based on the context and events mentioned.

Additionally, some individuals associated with these organisations are mentioned:

* Cissie Gool was a prominent figure associated with the New Era Fellowship (Source [17])
* George Manuel, mentioned in [7], was likely involved with various political and cultural organisations
* Dr. Goolam Gool and his son, Gandhi's visit to their home (Source [15]) suggest that they were active in Indian politics and community
* Various other individuals, such as Abubaker Ismail, are also named but without clear organisational affiliations.

There is limited information about other specific organisations or details regarding their activities during the author's lifetime.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi is referred to in excerpts [3] and [11], but the most detailed information on him can be found in excerpt [5].

According to that excerpt, Gandhi arrived in South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria. The accounts were kept in Gujarati, which was Gandhi's mother tongue.

Gandhi is also mentioned as being involved in the fight for Indian rights in South Africa, and he played a key role in advocating for the abolition of certain laws that discriminated against Indians.

However, it seems that there is no comprehensive description of who Mahatma Gandhi was or why he came to South Africa. 

Thus,
Mahatma Gandhi [5] was a politician.
He arrived in South Africa (1893) on a one-year assignment from Dada Abdullah [5].
 
The provided sources do not contain that information.

### q38 — Who was Cissie Gool's father?

According to [3], Dr. Abdurahman, specifically said to be the first of the community to qualify as a doctor from Glasgow University in 1893, was married to Helen (or "Nellie" Potter James). The text then links the Abdurahman clan and the Ta'Als through the Dollie family, and mentions that Dr. Abdurahman's mother, Gadija Dollie, was said to be a beautiful woman.

Nowhere is it explicitly stated who Cissie Gool's father is. However, we do know from [6] that "a telegram was received at the Gools' residence announcing the imminent arrival...of Bibi Gool", and this makes Bibi Gools' identity as Dr. Abdurahman's wife probable, if not certain.

Cissie being Dr. Abdurahman's daughter would make him her father [the information of Cissie's mother is however absent].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to excerpt [3], District Six was "a 'black spot' in the Apartheid scheme of things" and yet, it "had more life, more zest, more fire, more warmth and care than many other places one could mention." It is described as a vibrant community that buzzed with joy of life.

Excerpt [4] further contextualizes this by stating that apartheid legitimized poverty and oppression for the majority of its peoples, uprooting communities and ways of life. This suggests that District Six was not an ordinary neighbourhood, but one that thrived in spite of systemic injustices.

A more detailed description of District Six as a thriving cosmopolitan area is provided in excerpt [1]. The neighbourhood is remembered fondly for having "more zest, more fire, more warmth and care" than many other places.

Additionally, excerpts [7] through [13] provide brief glimpses into various aspects of life in District Six before the forced removals. These include:

* The Indian Opinion commenting on Dr. Gool's attainments (excerpt [7]).
* A description of a chaotic scene where a car skids to a halt due to a child running out, and its aftermath (excerpt [13]).
* An account of a person trying to understand the complexities of life in Soviet Russia while walking back through District Six with his friend Hassan (excerpt [14]).

Overall, these excerpts paint a picture of District Six as a dynamic, multicultural neighbourhood that was home to people from different racial groups and backgrounds.

### q40 — What was the Unity Movement's boycott policy?

According to source [1], when the Unity Movement wished for a boycott, there was little evidence that many followed the directive to spoil their votes. The Movement directed people in the District 6 Ward to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip if they insisted on voting despite boycotting.

Source [10] confirms that the Unity Movement believed in the boycott as a means of struggle against Apartheid, but it acknowledged its limitations, citing the failure of the Train Apartheid Resistance campaign in drawing up affidavits and highlighting that boycotting the Group Areas Act did not create significant problems for propertiless or homeless people, but was more challenging for those who owned property.

Source [20] suggests that there were instances where the Unity Movement's boycott policy was challenged by some of its members, particularly Jadwat and Seedat from Natal, who advocated for participating in "dummy" institutions to promote their point of view rather than boycotting them altogether.

