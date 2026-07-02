# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 52.0% (117/225) |
| Avg latency | 25381ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 48718ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 28490ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 22065ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 19651ms |
| q05 | Who was J.M.H. Gool? | 2/8 (25%) | [Graph: Goolam Gool], LEST WE FORGET -rev25.pdf | 28152ms |
| q06 | Tell me about Buitencingle. | 2/8 (25%) | LEST WE FORGET -rev25.pdf | 23408ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 20145ms |
| q08 | Tell me more about the author's wife. | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 22384ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | LEST WE FORGET -rev25.pdf | 20846ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 28737ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 22278ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdul Hamid Gool] | 26765ms |
| q13 | What was the All Africa Convention? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 27408ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 22802ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 22898ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/7 (43%) | LEST WE FORGET -rev25.pdf, [Graph: Goolam Gool] | 27042ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 21603ms |
| q18 | What was the New Era Fellowship? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 28468ms |
| q19 | What was the Non-European Unity Movement? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 25520ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 25032ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 22980ms |
| q22 | Who was the author's father? | 1/4 (25%) | LEST WE FORGET -rev25.pdf | 23115ms |
| q23 | Who were the author's siblings? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 23672ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | [Graph: Dr. Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 26045ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 24169ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 27475ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 26293ms |
| q28 | Which organisations was the author involved in? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 21765ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 24653ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | [Graph: Dr. Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 28977ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 27473ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | [Graph: Adam Gool], LEST WE FORGET -rev25.pdf | 24583ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 24361ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 28764ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 27012ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 27795ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 23166ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 25612ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 25357ms |
| q40 | What was the Unity Movement's boycott policy? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 19595ms |

## Answers

### q01 — Who is the author?

The author of "LEST WE FORGET -rev25.pdf" or this memoir is Joe Rassool [1], [16].

### q02 — Who are the author's children?

The author's children, as mentioned in the excerpts, are:

* Feyruz
* Reza
* Zarina 

Additionally, his grandchildren are listed (although only their first names appear in excerpt [18] and it mentions that some of them have the same name as Joe Rassool):

* Joseph
* Samuel
* Alexander
* Leonardo
* Dylan
* Lewis

### q03 — Who are the author's grandchildren?

The author's grandchildren are:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis 

These names are mentioned in the dedication of the book on pages [1] and [20].

### q04 — To whom is the book dedicated?

The book is dedicated to the author's children: Feyruz, Reza, and Zarina; and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [18].

### q05 — Who was J.M.H. Gool?

The provided sources do not contain a lengthy biography of J.M.H. Gool, but they offer some information about his life. 

According to [3], J.M.H. Gool was mentioned in Ralph Bunche's travel notes of his stay with the Gools in 1937.

[14] describes him as a wealthy Indian businessman who had residences at Buitencingle and Kloof Street, where he hosted notable guests. His son, Ah. Gool, is also mentioned in this source as being born in 1886.

[16] notes his association with socialites and politicians of the time, including Gandhi, Gregoire Boonzaaier, and Terence Macaw, among others.

J.M.H. Gool was considered a prominent figure in Cape Town's Indian community during his lifetime, as seen by their residences' grandeur and his hosting of dignitaries like Gandhi.

[19] highlights his financial status and social standing within the community by mentioning that he participated in British Justice and his standing in the Cape community. 

As for details about J.M.H. Gool's personal life, family dynamics, or specific achievements, the provided sources do not provide exhaustive information.

### q06 — Tell me about Buitencingle.

Buitencingle is described as a thriving trading community (ex.[6]). It had many shops, including those that sold "ships' chandlers" and other items (ex.[11]). The area was also known for its cosmopolitan atmosphere: it's mentioned that the houses at 7 Buitencingle had been occupied by different people of various ethnic groups, including Indians, who were the author's family (ex. [9] indicates their household included multiple wives and servants). 

The area is described as having cobbled streets, traffic islands with stunted palm trees in the middle, and was opposite a German Lutheran Church (ex.[15] ).

### q07 — Who is the author's wife?

The provided sources do not give a unified name for the author's wife; however, they mention two instances: 

In [9], she is referred to as "Sushila". In this context, it appears that Joe Rassool may be referring to his own connection with her having previously mentioned being friends.

### q08 — Tell me more about the author's wife.

The provided sources do not contain much information specifically about the author's wife, although [4] mentions that the author owes a great deal to her wife, Nazima, who is described as an academic researcher. She urged him to ground his writing in background investigation and it was due to this that he began to research extensively for his family history, which transformed "the story qualitatively from being an entirely subjective recollection of purely familial interest to one which revealed a rich historical layer."

### q09 — Who was the author's grandfather?

The author's grandfather was Haji Joosub Maulvi Hamid Gooli (also known as J.M.H. Gool) [19].

### q10 — Tell me about Kloof Nek.

Kloof Nek is described as a connecting saddle of land that joined Table Mountain to Lion's Head [17]. It sprawled protectively on the eastern flank of the city with its head gazing inscrutably down on Table Bay, and in this location was Signal Hill from which the noonday gun heralded midday with unfailing regularity and a puff of smoke [2], [6].

In addition to natural scenery, Kloof Nek had urban features such as department stores like Fletchers & Cartwrights [17]. It also featured natural features that allowed people to climb Table Mountain without having to climb all the way from Cape Town [3]-[5].

Kloof Nek was particularly relevant in this memoir because it held sentimental value for the author, who reminisced about visiting Kloof Nek with his friends and reading poetry there, getting lost in its beauty. The Glen in Kloof Nek seemed to be a place of solace and inspiration for him [2], [3].

As of 1944-1945, Kloof Nek also featured areas that provided refuge to down-and-outs or "Bergies", who would find shelter in the cavelets, culverts, and bushes in the area.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was an organization of teachers, according to [11]. [4] describes it as a "Liberatory Movement" that aimed to resist apartheid.

### q12 — Who was Cissie Gool?

Cissie Gool was a leader in the struggle against apartheid in South Africa, particularly among the Coloured community. According to the sources [3] and [14], she was married to Dr. Abdul Hamid Gool, a doctor who qualified from Glasgow University in 1893.

The source [4] mentions that Cissie Gool supported the Torch Commando, a movement of ex-servicemen who voted for the United Party, but also quotes an excerpt from "Indian Opinion", January 14, 1911 [13], where it is stated that Dr. Abdurahman's opinion was at odds with his wife Cissie Gool and brother Goolam Gool.

Source [9] describes her as fighting against social, economic, and political injustice among the Coloured community, stating that she "tells the story of brave and strong women such as Cissie Gool fighting against social, economic, and political injustice." 

The sources [6], [16], and [20] show Cissie Gool being involved in politics, specifically supporting measures to deal with "influx control" and participating in the Anti-CAD Movement.

### q13 — What was the All Africa Convention?

The All African Convention (AAC) is mentioned several times in the provided excerpts. According to [1], it was one of the organizations that met at Auntie Minnie's home, where the author saw some of its leaders, including Messrs. Tsotsi, Honono, and Sihlali.

In [15] it mentions "a revolt among the “Coloured” community led by the young Turks of the New Era Fellowship and the Teachers’ League of South Africa that the forerunner of the Coloured Affairs Department was boycotted out of existence". This indicates that the AAC may have been an attempt to bring different groups together against the racial oppression they faced.

However, in [19], it is stated that Mr. Tabata's book "The Awakening of a People" recorded the coming into existence of the All African Convention as one of the pillars of the Non-European Unity Movement (NEUM). This suggests that AAC was closely related to the NEUM and its goal of bringing together different communities.

It appears that the AAC was an effort by various racial groups to unify against apartheid, but we do not have more details about its structure, goals, and achievements due to the lack of information in the provided excerpts.

From [20], we can infer that one of the events related to the AAC took place in Bloemfontein in 1954.

### q14 — Where was District Six and what kind of place was it?

District Six was a neighborhood in Cape Town, South Africa [8]. According to the excerpt, Caledon Street ran from Corporation Street straight through to Clifton Street, with Hanover Street being the main street of the district. The Star Cinema (bioscope) and the fish market were located at the junction of these streets.

District Six was described as a thriving cosmopolitan area in the heart of Cape Town [6]. It had a rich tapestry of life and culture, with people from different backgrounds living together [3]. According to the author, it was a place that "had more life, more zest, more fire, more warmth and care than many other places one could mention" [20].

The neighborhood was known for its vibrant streets, markets, and entertainment venues. People like tap dancers were part of the community's unique culture [7]. The area was also a hub for political activism, with organizations like the Unity Movement fighting against apartheid [14].

Overall, District Six was a lively and diverse community that was eventually bulldozed as part of the apartheid government's forced removals policy.

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information about the forced removals from District Six. However, they briefly mention the context of "displacements" and "forced migrations" without detailing what District Six specifically refers to or its historical background. For detailed insights into the forced removals from District Six, you might find additional context in histories about Cape Town, South Africa's apartheid period, or specific accounts related to this area.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the provided sources, Mahatma Gandhi was a close friend and associate of the Gool family in Cape Town. He had a special relationship with Dr. A.H. Gool (also referred to as "my uncle" by the author), who was a prominent figure in the Indian community in South Africa.

The connections between Gandhi and the Gools can be seen through various sources:

* Source [2] mentions that Gandhi had arrived in Cape Town, where he was greeted by members of the Gool family, including Yusuf Hamid Gool, Amod Gool, Lachiram, and Abdul Kadar.
* Source [3] indicates that Gandhi had a function in honour of himself before leaving for Phoenix, Natal, where Dr. Gool (referring to A.H. Gool) thanked those present for their services to Mrs. Gandhi during her illness.
* Source [7] mentions that in August 1914, Dr. Gool offered a farewell address to Gandhi on behalf of Port Elizabeth Indians, hailing him as "Mahatma" – Great Spirit.

Gandhi's connection with the Gools went beyond professional acquaintanceships; they also played host to his family members and were known for their hospitality.

The author of the memoir notes that they have a special fondness for Gandhi due to the close friendship developed during those years, which is a result of the support extended by the Gool family to Gandhis in February 1914.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town, South Africa. It trained teachers for the upper years of primary division and was known as the first Post Matriculation Teacher Training facility for Coloureds in the country [3]. The college had a staff recruited from the United Kingdom with a conservative educational philosophy that aimed to prevent the formation of left-wing radical ideas among students, similar to those found at Livingston and Trafalgar High schools [11].

### q18 — What was the New Era Fellowship?

The provided sources do not contain a detailed description of what the New Era Fellowship (NEF) was. They rather suggest that it was an organisation, as described in various excerpts. However, some information can be gleaned from these descriptions:

1. The NEF appears to have been a political-cultural organisation associated with the Unity Movement.

2. It seems to have had a significant role in representing different organisations and people at conferences and rallies. For example: [7] - "1954 was the year that Vic Wessels, Les Jacobs and I went to represent the New Era Fellowship at the conference of the All African Convention in Bloemfontein."  
   And from [19]: "The New Era Fellowship, a cultural organisation..."

3. The NEF had a lecture series or programme.

4. Excerpts suggest that the NEF was an important voice for unity and democracy among the non-European and coloured people of South Africa during this time period.

5. There is also discussion within the New Era Fellowship about their views against certain ideologies like nationalism ([6]) and what they considered to be bureaucracy, which could be related to its operations.

It's clear from [9], it had positions like Organising Secretary; a demanding job that was done by Hassan Bavasha.

### q19 — What was the Non-European Unity Movement?

The sources do not contain an exhaustive description of the Non-European Unity Movement. However, according to [3] and [7], the Non-European Unity Movement (NEUM) broke with its previous stance of non-political professionalism and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society.

According to [2] and [3], the NEUM "revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon."

The movement had several key principles, including:

* The principle of non-Collaboration: avoiding cooperation with the oppressive regime (as mentioned in [11]).
* The refutation of "race": rejecting racial divisions and promoting a non-racial society (as mentioned in [12] and [13]).

Additionally, according to [7], the NEUM was founded by young Turks in 1943.

### q20 — Describe the author's involvement in cricket.

The author was heavily involved in cricket at various levels. He dreamed of being selected to play for a team, but before that could happen, he was appointed as one of the fifteen representatives of the Western Province for the Cricket Tournament of their Union that took place in Cape Town (Source: [5]). 

He even helped arrange regular fixtures through a loose union and attempted to organize clubs at High School and College level. He spent time practicing his forward defensive stroke with Edross, who was impressed by the author's batting skills (Source: [3]).

The author expressed joy when his cricket team captain Hassen Mall left for Durban and he got an opportunity to play in a biennial cricket tournament (Source: [8]). Unfortunately, no selector saw him play and therefore, did not choose him for the Western Province Indian Cricket Union's side (Source: [14]).

He played with several top cricketers of the Barnato Union including Salie Van Haacht, who was nicknamed 'Haartzen', Ben Malamba, and Stanley Abrahams.

### q21 — Who was the author's mother?

The author's mother is implied to be Wahida, mentioned in [13]. However, it's also possible that her name could be Bibi or another woman. The information from excerpt [4] mentions a "Bibi" as J.M.H. Gool's wife whom he married in 1879.

A clear mention of the author's biological mother is found in excerpt [12], where she is said to have been born nine months later on December 10, 1900, being the baby of Grandpa Gool and his wife, who is referred to as Bibi.

### q22 — Who was the author's father?

The author's father [19] is mentioned as "my grandfather, Haji Joosub Maulvi Hamid Gooli" also referred to as his "grandfather", being a merchant living in Buitencingle Street. However, since the question asks about "the author's father", a more accurate response would be: The provided sources do not directly mention the author's father; however, they imply that the author is referring to his own father when describing his life and experiences, particularly as mentioned in [13] where it says: "My dad went about the Cape Peninsula earning his reputation as one of the top travellers on the road."

### q23 — Who were the author's siblings?

The provided sources do not contain a comprehensive list of the author's siblings. However, they mention that the author had several aunts who had children:

* Auntie Annie (also known as Amina) and her twin daughters Nurjehan and Zarina.
* Auntie Dij (also mentioned in #4) who was expecting a child with Uncle Ismail. Her daughter Shawquet is also mentioned.
* Another aunt, Auntie Jochee, was expecting a child.

The only siblings of the author mentioned in the sources are Yasmin (also referred to as Jessie), and probably Fazil who is mentioned with his age in #1. However, it's worth noting that the narrative is written in the first person, suggesting that Fazil may be the narrator rather than a brother.

### q24 — Who were the children of J.M.H. Gool?

The text doesn't directly mention all of the children, but we can piece together information from different excerpts.

From [1], we know that J.M.H. Gool had two wives: Bibi (whom he married in 1879 or 1880) and Wahida Ta’al. The children mentioned are:

* Kassim (born in August of one year): mentioned by a footnote in [6] as Grandpa’s child, implying that he is the son from his first wife with Bibi.
* Other members of J.M.H. Gool's family include Dr. Abdul Hamid Gool and Auntie Minnie who send a weekly offering to his surgery; their relationship to Kassim is unclear but implies they are other children.

Additional information can be gathered from [16], [17] and [20]. From these:

### q25 — Who was I.B. Tabata?

According to the excerpts [1] and [4], I.B. Tabata was a key figure in the Non-European Unity Movement (NEUM), and he completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship. 

Excerpt [4] explains that an indictment was levelled at Kies by Tabata, and it is evident because he rose to attack Kies.

Tabata's leadership within NEUM led to disagreements with other groups, such as The Forum Club, which regarded themselves as Marxist Trotskyists and condemned "The Awakening" for lacking a class analysis [17].

The provided sources do not contain detailed personal or biographical information about I.B. Tabata beyond his involvement in the NEUM and conflicts within the movement.

### q26 — Who was Dr. Abdullah Abdurahman?

According to source [1], Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was a medical doctor, father of Cissie Gool (Zainunnissa Gool), and an activist. Source [9] states that Dr. Abdurahman's residence was in Loop Street, close to Buitencingle Street, where his daughter Cissie and her husband Dr. J.M.H. Gool lived. 

Source [11] mentions that Dr. Abdurahman was among the visionaries who worked towards claiming the rights of Indians as subjects under the British institution. He also founded the Natal Indian Congress and organized the "monster petition" against the proposed Disfranchisement Law.

In source [12], Abdurahman is mentioned again, this time in relation to a court case where he was tried for advocating violence but found not guilty. Source [19] lists his name among several important figures of the time.

Source [20] mentions that Dr. Abdurahman's wealth brought him into contact with many prominent individuals, including Indian Agents General and notable activists such as Gandhi.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources indicate that there were multiple connections between Mahatma Gandhi and J.M.H. Gool:

1. Friendship: They worked together in partnership (Source [3]).
2. Personal visits: Gandhi frequently visited the Gools' residence at Buitencingle Street, where they would discuss various projects and topics of mutual interest (Sources [5] & [6] mention his visits to the house).
3. Personal relationships: Gandhi's son fell in love with one of J.M.H. Gool's sisters, prompting Gandhi to intervene and later recall his son to marry a Hindu girl instead (Source [17] mentions this incident as recorded by Owen Mathurin).
4. Letters and exchanges: They exchanged letters, including correspondence between Gandhi and J.M.H. Gool's uncle, Abdul Hamid Gool (Sources [19] & [20] mention their written conversations).
5. Collaboration and mutual respect: The sources suggest that there was a close bond between the Gools' and Gandhi's family, with both parties collaborating on various projects, sharing meals together, and even receiving each other at train stations or sending gifts to one another (e.g., Source [20] describes J.M.H. Gool hosting Gandhi's traveling companions).

These connections demonstrate that Gandhi maintained a close relationship with the Gools' family during his time in South Africa and valued their friendship and input on various initiatives they shared.

### q28 — Which organisations was the author involved in?

According to Sources [2] and [17], the author was involved in the New Era Fellowship (NEF) and the Trafalgar Players. Source [2] mentions the author's involvement with the NEF "with grave misgiving", implying that they had concerns about the organisation but still participated.

Additionally, Excerpts not listed explicitly involve the author with other organizations such as:

* Chapels Street Primary school (Source [12]): The author sang a song and collected scrap iron as part of a prize-winning effort.
* High School students' union (Source [3])
* Hewat Amateur Theatrical Society (Source [16])

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not explicitly describe the nature of the relationship between the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement. However, Excerpt [12] mentions that the " Non-European Unity Movement revolutionised the political scene...", and TLSA was mentioned as a part of this movement in different excerpts like [3]. It is implied that there was some kind of connection between them but no details are provided.

It is worth noting that the source author was influenced by the ideas of the Non-European Unity Movement and actively participated in its activities, which is evident from Excerpts [9], [12] and other parts.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to the sources, we can piece together the following information [18]:

* December 1900: The party consisting of Bibi and her son Mohamed Saaid arrived in Cape Town (Source [18]).
* After discovering that my aunt Jane Gool-Tabata was born on March 19, 1902, it is deduced that Grandpa J.M.H. Gool had to be back in Cape Town at least nine months prior to this event.
* It means that he departed from Bombay around April or early May 1901 (Source [18]).

Since the party arrived in Cape Town in December 1900 and it consisted of Bibi and her son, it can be inferred that J.M.H. Gool had already arrived in Cape Town before this date, likely with another wife Bibi from India. However, it is mentioned [16] that Grandpa took them to the fishing village called The Strand, about thirty miles from Cape Town after a brief stay in Cape Town.

There is also an excerpt suggesting possible travel for J.M.H. Gool and Gandhi visiting Buitencingle 1912 (February-March), but there isn't any direct information about this being travel of Grandpa to or from India.

Therefore, based on the available information, we can deduce that Grandpa probably traveled in between Bombay and Cape Town at least twice: once arriving in Cape Town before December 1900, and again around April-May 1901.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

[1] states: "The Hanaffi Quwatul Islam Mosque, an outpost of Islam in an area becoming increasingly European, Jewish and gentile. My grandfather was a founder and life trustee of the mosque, which was completed in 1898, and we used to go there to celebrate Eid festivals."

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not explicitly state how Cissie Gool is related to J.M.H. Gool. However, it can be inferred from [11] that Cissie Gool is the wife of Dr. Adam Hoosen "Gool" (uncle Doctor), who is the brother of J.M.H. Gool. Therefore, Cissie Gool and J.M.H. Gool are uncle's wife and nephew, respectively.

[11]: "It was at Grandpa’s funeral that I met my cousin, Shaheen 27, the son of Uncle Doctor and Cissie  Gool, for the first time."

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources [1-20], some of the notable historical figures that J.M.H. Gool knew personally include:

* Gandhi: Gandhi visited Cape Town in 1914, and Dr. Gool gave a farewell address on his behalf.
* Mahomed Adam Hai: He was mentioned as Gool's senior, financier, and possibly mentor.

Additionally, [13] mentions some of Gool's contemporaries as belonging to the "pharaonic style" court with ties to him, including:

* Cissie Gool (presumably J.M.H. Gool's wife or a relative)
* Freda Lock
* Lilian Isaacson

It is also mentioned in [12] that Gool knew various figures from art, music, and politics.

However, it does not provide detailed information on others.

### q34 — What was the Group Areas Act and how did it affect District Six?

According to the sources [1], [2], and [7], the Group Areas Act (1950) was a piece of legislation in South Africa that aimed to separate different racial groups into specific geographic areas. This act was passed by the Nationalist government and was part of their policy of apartheid.

The Group Areas Act had a devastating impact on District Six, a vibrant multi-racial community in Cape Town. According to sources [1] and [6], District Six thrived as a cosmopolitan area with people from different racial groups living together. However, the Group Areas Act sought to separate these communities into distinct areas.

As a result, the residents of District Six were forcibly removed from their homes and relocated to other areas on the outskirts of Cape Town. This led to the destruction of the community's social fabric and resulted in the loss of livelihoods, homes, and cultural heritage for thousands of people.

Source [1] describes District Six as "a thriving cosmopolitan area" that was uprooted by the Group Areas Act, while source [2] notes that the act "played havoc with the lives of ordinary people." Source [6] emphasizes that the act aimed to create a patchwork of Black, Coloured, Indian, and White areas, further highlighting its oppressive nature.

Source [7] provides a vivid description of the effects of the Group Areas Act on District Six: "District Six - Lest We Forget" notes how shops began closing in Hanover Street, the main artery of the area, as residents were forced to move out. By the early 1950s, the community was beginning to die, and by the mid-1960s, it had been largely demolished.

Overall, the Group Areas Act led to the displacement of thousands of people, the destruction of homes, businesses, and cultural heritage, and the fragmentation of a once-thriving community.

### q35 — Who was Hassen Mall?

Hassen Mall is described in the excerpts as a close friend of the author. According to various sources:

[2] [6] Hassen came from Durban, where he had completed his LLB degree. He arrived at Cape Town.

[2] In addition, it's mentioned that his husband Dawood and he were members of The Indian Congress.

[4] It's stated that "Captaining the side was Hassen Mall, who had left for Durban after completing his LLB degree."

[6] According to excerpts [6], Hassen was a brilliant cricketer, a "magnetism" filled person with a strong personality, and not very tall. He changed careers from being in sciences to Law, where he discovered his talent.

[8] It's mentioned that the author would play cards at Mr. Allie’s place, which implies Hassen likely visited or stayed there as well.

In some excerpts [4], it is stated that after leaving for Durban, Hassen left "the indelible imprint of his personality" in Cape Town.

Throughout these sources, Hassen Mall is portrayed as a figure with exceptional strength and character. He was intelligent (an LLB graduate), confident, socially adept (part of The Indian Congress), and physically gifted (a brilliant cricketer). Overall, it appears that he played an influential role in the author's life during their time at Cape Town.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources do not contain a comprehensive list of all political organizations active in the Cape Coloured community during the author's lifetime. However, they mention several:

1. The New Era Fellowship [3]: A cultural organization established "to discuss everything under the sun" that issued a call to Non-European organizations to counter government assault on remaining rights of the Coloured community.
2. Teachers' League of South Africa [4]: Part of the resistance against Harry Lawrence's announcement of the Coloured Advisory Council.
3. The Cape Standard [2, 3] and The Guardian (the mouthpiece of the South African Communist Party) [3]: Newssources that provided information on current political events to the community.

There are mentions of other organizations, such as:

1. Coloured People's Vigilance Association [6-8,12].
2. South African Communist Party [3], with Cassim Amra being a member.
 The sources also refer to various other organizations, but they do not appear to be exclusively Coloured:

1. African Methodist Episcopal Church (The Reverend Gow was Bishop) [5].
2. Indian Opinion, with which the author had a connection through his grandfather and Dr. Gool [6-8].

The New Era Fellowship and the Teachers' League seem to have been active during this time, as well as several other organizations, but a complete list is not provided in these sources.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi was a famous Indian independence activist and leader (1889-1948). He arrived in South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria [7]. The suit was related to accounts kept in Gujarati, which was Gandhi's mother tongue. 

He came for work and initially had no intention of staying long-term. Gandhi played an active role in the struggles of Indian settlers in South Africa, advocating the rights of Indians in South Africa through various methods including non-violence [5].

### q38 — Who was Cissie Gool's father?

The provided sources do not mention who Cissie Gool's mother is, but her husband's name is mentioned as Dr. Abdurahman [4]. Additionally, it is stated that "Abdurahman was in fact the first of the community to qualify as a doctor from Glasgow University in 1893" and also mentioned in [20]: "...the bond between Dr. Aburrahan and my grandfather also brought Cissie and my uncle Dr." Hence, [4] states: There he met and married Helen “Nellie” Potter James 

The provided sources, however, do not mention who Helen or Nellie's father was.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to excerpts [2] and [3], District Six was a thriving cosmopolitan area with a rich history and culture. It was described as having "more life, more zest, more fire, more warmth and care" than many other places.

Excerpt [3] suggests that the author grew up in District Six and has fond memories of the place. He writes about taking a walk through the streets of Caledon Street, which he remembers as crowded and vibrant. The street ran from Corporation Street to Clifton Street, with Caledon Street getting "tattier" along the way.

Excerpt [2] mentions that before the forced removals, District Six was not just home to poor residents but also to some affluent families who lived in "cottage-style" houses. However, it also notes that these housing schemes were often poorly managed and resulted in the loss of municipal franchise for residents living in sub-economic houses.

Overall, while District Six was likely a challenging neighborhood with issues of poverty and overcrowding, excerpts [2] and [3] suggest that it had a strong sense of community and culture before the forced removals.

### q40 — What was the Unity Movement's boycott policy?

According to the sources [3], if "the 'Coloured' voters insisted on voting" in the elections, they were asked to "spoil their ballot by writing: "For Full Democratic Rights" across the ballot slip."

Source: [3] This was a directive of the Unity Movement during election times.

