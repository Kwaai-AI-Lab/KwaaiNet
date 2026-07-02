# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 53.8% (121/225) |
| Avg latency | 41521ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 56101ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 66508ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 44560ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 91566ms |
| q05 | Who was J.M.H. Gool? | 2/8 (25%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 51705ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | LEST WE FORGET -rev25.pdf | 51414ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 60050ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 46373ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 70768ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 84612ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 84875ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 93990ms |
| q13 | What was the All Africa Convention? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 42630ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 30132ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 29642ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/7 (71%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 36964ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 28992ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 30636ms |
| q19 | What was the Non-European Unity Movement? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 29647ms |
| q20 | Describe the author's involvement in cricket. | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 27762ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 25832ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 25665ms |
| q23 | Who were the author's siblings? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 29532ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 35901ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | [Graph: I.B. Tabata], LEST WE FORGET -rev25.pdf | 28156ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 2/6 (33%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 30238ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 26604ms |
| q28 | Which organisations was the author involved in? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 28233ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 32024ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town-Woodstock P.T.A.] | 32759ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 33490ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 32534ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 0/5 (0%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 28153ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 30265ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 33265ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | [Graph: Cape Town-Woodstock P.T.A.], LEST WE FORGET -rev25.pdf | 34161ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 28861ms |
| q38 | Who was Cissie Gool's father? | 2/5 (40%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 28464ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 31820ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 25959ms |

## Answers

### q01 — Who is the author?

The author is Joe Rassool [9].

### q02 — Who are the author's children?

The provided sources do not contain detailed information about all of the author's children. However, according to [3], the author's children are:

* Feyruz
* Reza
* Zarina 

Additionally, the author mentions grandchildren in [3]. Their names are:
 
Joseph
Samuel
Alexander
Leonardo
Dylan
Lewis

### q03 — Who are the author's grandchildren?

The author's grandchildren are Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [20].

### q04 — To whom is the book dedicated?

The book is dedicated to "[20]'s children: Feyruz, Reza, and Zarina and grand[children], Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [Source: [13]] However, since it's not explicitly stated that this excerpt refers to the same person as the author of the memoir, a stronger inference is based on the ISBN and Copyright information being identical. Therefore, the book is dedicated to the children and grandchildren of Y.S Rassool.

### q05 — Who was J.M.H. Gool?

Based on the provided excerpts, we can gather some information about J.M.H. Gool.

* He was a significant figure in the Cape Town community, particularly among the Indian population (Excerpt [13]). It is mentioned that if he were not part of an out-group at the time, his achievements would have been more recognized and documented in history (Excerpt [8]).
* He made notable contributions to the Non-European Unity Movement, although this was met with some criticism from his family members who did not support the organization's goals or methods (Excerpts [14], [16]).
* He had two wives: Bibi (from Broach, Gujarat, India) and Wahida Ta'al Dollie. His relationships with both women, particularly Wahida, were significant to his life (Excerpt [19]).
* He played a key role in the British Justice system, as mentioned in the Supreme Court transcripts from 1892, and was described as an "Oriental trader" (Excerpt [18]).

### q06 — Tell me about Buitencingle.

Buitencingle was an outer suburb of Cape Town, South Africa [1-5]. The name "Buitencingle" was derived from the Dutch word "buiten", meaning "outer" or "beyond", and "cingle", a belt-like boundary area [1-2, 5]. It was the outermost belt of Cape Town at that time.

According to the author's recollections, Buitencingle had a lot of space, including a garden with fruit trees, which was particularly uplifting for the family [9]. The garden also had a huge pear tree and fig tree on either side, as well as other plants [4].

Notable figures, such as Dr. Abdullah Abdurahman, an Afro-Caribbean community leader, had close ties with the author's grandfather, Haji Joosub Maulvi Hamid Gooli, who was a prominent citizen of Buitencingle and lived in a 13-roomed mansion [7-8].

Buitencingle also figured in notable political movements. Dr. Jabavu, an advocate for non-European unity, had some discussions about his idea at the residence of Buitencingle's residents [15]. 

The author's grandfather was known to be proactive and participated in various civic activities as well. He took part in meetings where non-Europeans were encouraged to vote for the "Coloured" community [15-8th paragraph].

Eventually, the building that included 7 Buicengele (Note: Buitencingle on some sources) was later demolished during the forced removals of the apartheid era District Six neighborhood in Cape Town.

### q07 — Who is the author's wife?

The author's wife is Nazima [1], [3], [11], [14].

### q08 — Tell me more about the author's wife.

We don't know much about the author's wife from the provided sources. However, it is mentioned in several excerpts:

* Excerpt [3] mentions that the author's "wife, who is an academic researcher, and who urged me to ground my writing in background investigation."
* Excerpt [17] contains a similar mention: "I owe a great deal to my wife, who is an academic researcher, and who urged me to ground my writing in background investigation."
* Excerpt [4] mentions that the author's wife was Nazima, who insisted on researching the story to lend authenticity.

The sources give us some insight into her role as someone who encouraged the author to do more research to create a more credible narrative. However, we don't have any further information about who she is beyond those points.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool [Source: [1]].

### q10 — Tell me about Kloof Nek.

According to the provided sources [2], [4], and [18], Kloof Nek is a place of natural beauty, often referred to as a "sylvan glen" or a "place of haunting beauty". It is mentioned that the area was used for relaxation and enjoyment by the author and his friends, who would walk there to escape the heat of Cape Town on hot days.

From sources [2], we can understand that the Glen at Kloof Nek was especially suitable for reading poetry, as it had a tranquil atmosphere. The author recalls reading Gray's Elegy and Milton's Lycidas in this location.

Source [4] gives us insight into the geography of the area. It describes Kloof Nek as being situated on a connecting saddle of land between Table Mountain and Lion's Head, which was surrounded by water and had Signal Hill nearby, from which the noonday gun would be fired.

The author also mentions that Kloof Nek was popular with various groups of people, including lovers, mountaineers, nature lovers, and ramblers. However, it has become a place where "Bergies" (homeless people) find refuge due to its abundance of cavelets and bushes.

### q11 — What was the Teachers League of South Africa (TLSA)?

According to various sources [1-20], including Excerpt 9 and 16:

The Teachers League of South Africa (TLSA) was an organization of teachers that aimed to promote democracy, equal education, and oppose apartheid. Members of TLSA were involved in the struggle for full democratic rights, especially for "non-citizens" (most likely referring to people of color in a racist context).

Specifically, excerpt 4 [4] mentions a revolt among the "Coloured" community led by the young Turks of the New Era Fellowship and the Teachers' League of South Africa. The TLSA was also mentioned as having members who opposed the Coloured Affairs Council (CAC) established by the government.

Excerpt 15 [15] describes how the old guard of the TLSA, which included people like Quints, Sonns, et cetera., had control until they were ousted by new leaders. These new leaders formed a part of what is often called the "Non-European Unity Movement".

In Excerpt 18 [18], it says about the Education Journal of the Teachers' League of South Africa.

It's worth noting that these excerpts suggest that TLSA played an important role in the struggle against apartheid, and its members were involved in key events during this period.

### q12 — Who was Cissie Gool?

Cissie Gool was a leading figure in South African politics, particularly among the non-European community. She was a stalwart of the Anti-CAD Movement (Anti-Coloured Affairs Department) and later became a member of the Unity Movement.

She was known for her passionate advocacy and organizing skills, including rallying votes-less masses against government plans to introduce residential segregation. According to the sources, she held a massive meeting on Grand Parade in 1937, which rocked the Government [16].

Cissie Gool also addressed meetings, as reported by the author Joe Rassool: "I happened to wander into the Banqueting Hall one evening to discover Cissie Gool addressing a meeting...It was some kind of pre-election rally. She had supported the Torch Commando in 1953...She had spoken in support of measures to deal with 'influx control' and the street gangs" [12].

The author Joe Rassool was shocked by this as he did not expect Cissie Gool to take such a stance, which seemed to betray her previous commitment to anti-discrimination causes.

From these sources, it can be seen that Cissie Gool was an influential and active figure in South African politics during the mid-20th century.

### q13 — What was the All Africa Convention?

The sources do not explicitly describe what the All Africa Convention (A-C) is, but they mention that it was the organization that Joe Rassool's delegation aimed to represent at the conference in Bloemfontein. They also mention that the A-C was one of the pillars of the Non-European Unity Movement.

However, source [1] implies that the leaders and members present at Auntie Minnie's home were part of the All Africa Convention.

Source [1]: "There I saw some of the leaders of the All African Convention- Messrs. Tsotsi, Honono and Sihlali, as well as members from Natal, Enver Hassim, Errol Vawda, and Kader Essack."

Based on the context and other historical records not provided here, it is likely that the All African Convention (AAC) was a Pan-Africanist organization focused on promoting unity and solidarity among non-European populations in South Africa during the early 20th century.

### q14 — Where was District Six and what kind of place was it?

According to the source excerpts [1], [3], and [17], District Six was located in Cape Town, South Africa.

From excerpts [3] and [15], we can gather that it was a vibrant and lively area, often described as having "more life, more zest, more fire, more warmth and care" than many other places. It was an impoverished area with overcrowded homes (excerpt [6]), but it had a strong sense of community and culture, with its own unique character and way of life.

In excerpt [18], District Six is described as being one of the areas targeted by apartheid authorities to be bulldozed out of existence, making way for new suburbs and housing schemes.

### q15 — What were the forced removals from District Six?

The provided sources do not contain explicit information about the forced removals from District Six. However, context [3] mentions "the New Year of 1955 plans to bring Apartheid to the City's buses" and mentions Non-white Councillors protesting, implying a backdrop of resistance to or an increasing presence of apartheid policies.

It is possible that information on forced removals can be inferred in relation to broader events of this era. As of the current query limit, I do not have access to additional sources for direct inference within these text constraints, suggesting limited direct evidence regarding District Six's specific forced removal history.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Gandhi (also known as Mahatma Gandhi) was a prominent figure in the struggle for Indian independence. His connection to the Gool family, particularly Dr. Abdul Hamid Gool, is that they hosted him at their residence in Cape Town, South Africa, on several occasions. Dr. Gool and his father, Adam Gool, were known to be close friends of Gandhi's. Dr. Gool treated Gandhi when he was ill (in 1914) and also read the welcome address at the residence of Mr. Gandhi when visiting dignitary W.C. Gokhale arrived in Cape Town.

In fact, according to [13] & [14], Gandhi mentions helping putty the floor at Dr. Gool's surgery and Dr. Gool had a close fondness for Gandhi. Additionally, there is evidence that Dr. Abdul Hamid Gool and his family hosted Gandhi on several occasions, including when he was at the height of his influence during the satyagraha.

Furthermore, as stated in [18] & [20], the Gool family also showed significant affection and respect to Gandhi. According to an account shared by E.S. Reddy ([2]), the Indians in Cape Town had taken up a subscription for Mahatma, and Mr. Gool in particular was one of its most generous supporters.

There is more detail about this connection as E.S. Reddy wrote that the address given at Dr Gool’s property in Buitencingle to Mr Gokhale, indicates how highly Gandhi was respected by his Indian comrades in South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a school that trained teachers, specifically for the upper years of the primary division (Source [5]). It was located in Fairly spacious but undeveloped grounds opposite the Roeland Street Gaol and was considered to be too close to District Six. The college accepted trainees from Coloured communities and aimed to provide training in the teaching profession.

### q18 — What was the New Era Fellowship?

The text does not contain a direct definition of what the New Era Fellowship is. However, it can be inferred that the New Era Fellowship was a political-cultural organization based on context provided in various excerpts.

According to [3], the New Era Fellowship (NEF) is associated with the Unity Movement and has a program of lectures. It also hosted talks on Saturdays at the Stakesby-Lewis Hostel in Canterbury Street ([13]).

The NEF seems to have been an influential organization in its time, as its chairmanship appears to be a significant position, with one individual presiding over a special meeting to discuss the report-back of the Unity Movement Conference (excerpts [17] and [18]).

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain detailed information about the Non-European Unity Movement. [1] mentions it briefly as the main topic of dispute, but only in passing.

Source [10] quotes members (including "A political study group called The Forum Club") accusing founder Tabata's work "The Awakening" of lacking a class analysis for being Marxist Trotskyists and Black Nationalist. It seems to view them disfavourably, labeling the Forum as "out of touch with realities but unceasing in prating about a 'Socialist Millenium'." [11] is more clear: that organization brought  'non-Collaboration',the refutation of ‘race’, and “the ideal for a non-racial society” to the struggle for liberation. They believed history would ultimately vindicate their efforts.

### q20 — Describe the author's involvement in cricket.

[12] states that Hamid was playing in a tournament and received a vicious ball to his finger, which dislocated it and ended his participation in the tournament (Hamid).

Also, [12] mentions that Ismail "Blondie" Shaik made a century [not specified against whom]. Unfortunately, no specific information about the author's own cricket career or achievements is provided.

### q21 — Who was the author's mother?

The provided sources do not contain detailed information about the author's maternal lineage beyond his mother's name and a few mentions of her being "Bibi" or having "lily-white skin." However, it can be inferred that Bibi had multiple marriages with different individuals, including Mohamed Saaid Gool (Grandpa) and Wahida Ta'al.

### q22 — Who was the author's father?

The author's father is Malick Rassool [4].

### q23 — Who were the author's siblings?

The text does not mention the names of all the author's siblings. However, it does mention that Hamid (Midi) and Fazil were among the author's brothers [3]. The author also mentions his half-uncle Dr. Abdul Hamid Gool (also referred to as BG or "Chops") in [19].

The siblings mentioned by name are: 
- Fazil Ally, a brother
- Hamid (Midi) Ally, a brother

### q24 — Who were the children of J.M.H. Gool?

The documents don't provide a complete and clear list of J.M.H. Gool's children. However, [5] mentions that "Bibi" (Gool's first wife) had two young children when she arrived in Cape Town, which implies that they were also J.M.H.'s children.

[19] mentions that by 1900, Hajima (Wahida Ta'Al, Gool's second wife) has given up hope of salvation and is described as being "bitter thirty-one years old", implying that she has been married to Gool for a significant amount of time. However, [5] also mentions that Wahida had two children, Kassim born in August 1888 and a son who was at least two months younger.

[5] also lists the following young family members who are likely J.M.H.'s children: Barrabhai and his Nurjehan who were living with them in The Strand. It's not clear if these individuals are married or not, but they seem to be part of the extended Gool family.

It is worth noting that [5] specifically mentions that Abdul Hamid Gool, Minnie, Timmie, Lallie (mother), and Kassim (born in 1888) were present at the Gool residence when he was a child.

### q25 — Who was I.B. Tabata?

According to the provided sources:

[1] I.B. Tabata: occupation: chairman; affiliation: New Era Fellowship [1]

This is the only reference that mentions I.B. Tabata by name in the given excerpts.

From other references, we can infer some information about him:

He was a high-ranking member of the New Era Fellowship and the Unity Movement.
He wrote The Awakening of a People, which became an NEF publication [19].
He had disagreements with Ben Kies and Victor Wessels, particularly over issues of class analysis and dialectics [3, 13]

### q26 — Who was Dr. Abdullah Abdurahman?

[4] says that "Dr. Abdulla Abdurahman" is mentioned as being brought to South Africa as a slave, buying his freedom and becoming prominent in the community.

[11] mentions Dr. Abdurahman's wife, Cissie Gool, serving tea at their Mount Street residence. 

[20]'s list includes: "Dr. Abdullah Abdurahman, 7"

Additionally, [7] mentions Mr. “Sonny” Abdurahman (presumably a family member or relative of Dr. Abdullah Abdurahman) as being involved in certain events.

From these excerpts, limited information about Dr. Abdullah Abdurahman is available: he was prominent in the community, and his wife Cissie Gool seems to have been an influential figure alongside him. No further details about him appear in the other excerpts.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to sources [7], [8], [10], [11], [12], [13], and others, Gandhi and J.M.H. Gool had a close connection.

Gandhi often stayed at 7 Buitencingle Street, the Gool residence in Cape Town, where he was treated with attentive medical care by Dr. J.M.H. Gool (source [8] mentions that Dr. Gool attended Mrs. Gandhi during her serious illness).

The two families were close friends; Gandhi corresponded with various members of the Gool family, including Abdul Hamid Gool, who was a medical student in London and had exchanged letters with Gandhi (sources [2], [19], and [20] mention this correspondence).

Gandhi also frequently visited the Gools at their residence, where he would meet with his close friends and allies (source [11] mentions that upon Gandhi's arrival in Cape Town in 1906, he was greeted by members of the Gool family).

The connection between Gandhi and J.M.H. Gool appears to have been one of mutual respect and affection.

It's worth noting that while sources do not provide details on Gandhi's personal interactions with J.M.H. Gool (e.g., whether they were friends, or simply acquaintances), their families' close relationship is a common thread throughout the cited documents.

### q28 — Which organisations was the author involved in?

The author, Yousuf (Joe) Rassool [1], was involved in several organisations. 

Some of them include: 
- The New Era Fellowship (NEF) [2, 13] 
- The Non-European Unity Movement (NEUM), mentioned as the "Movement" in the text [2, 12]
- The Teachers League of South Africa (TLSA), where he was a teacher and community activist [1]
- He also mentions being involved with the Hewat Amateur Theatrical Society and a friend who ran the Trafalgar Players [8]

The involvement with the other organisations mentioned in the text like the Coloured branch of Smuts's United Party, African People’s Organisation (A.P.O.), sports' clubs, Moravian church, Communist Party of South Africa, Fourth International of South Africa, Liberation League, and Workers’ Party seems to be through his participation with the New Era Fellowship [13].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to [8], the Non European Unity Movement (NEUM) "revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon". The TLSA (Teachers' League of South Africa) appears to be closely associated with NEUM, as stated in [9]: "At last I would be able to play an active part in the Liberator[y] Movement, and as soon as I could I joined the Teachers' League of South Africa, the TLSA for short." 

Furthermore, in [17], it is mentioned that the TLSA broke with its previous stance of non-political professionalism and joined the Non-European Unity Movement. This indicates a close relationship between the two organizations, with the TLSA participating in or affiliated with NEUM's activities.

Moreover, [18] also discusses a dispute centered on the Non-European Unity Movement within the context of discussions and debates taking place among individuals associated with the TSLS/NEUS, indicating that both were deeply intertwined.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain information about when J.M.H. Gool arrived in Cape Town or where he came from directly. However, excerpt [6] states that "His first son was born in 1886 and Gool's testimony is recorded in Supreme Court transcripts in 1892." And [3] indicates that Dr. e (Gool?) had to be back in Cape Town at least nine months prior to the event of Jane Gool-Tabata's birth on March 19, 1902.

Excerpts [18], also, states that there is an interview where Aunt Jane mentions "what we also know is that the party arrived in Cape Town in December." 

From these partial addresses and lack of a clear statement, it appears difficult to determine exactly when J.M.H. Gool arrived in Cape Town based on the information given within this document context.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

[2] mentions the Hanaffi Quwatul Islam Mosque as an outpost of Islam in a predominantly European and gentile area. It is stated that this mosque's founder and life trustee (my grandfather) completed it in 1898, indicating it was a significant structure for Muslims living in District Six at the time.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to excerpt [7], the bond between Dr. Abdurahman and my grandfather also brought Cissie and my uncle Dr. (referring to Dr. Adam H.I. Gool, but not explicitly stating this relationship with J.M.H.). However, in excerpts [12] and [20], it can be inferred that Cissie Gool was the wife of Dr. Adam H.I. Gool. Since there is also a mention in excerpt [15] where Cissie's husband is referred to as "Adam Mohamed", it is not clear how Cissie Gool is related to J.M.H Gool from this information alone.

However, considering that excerpt [6] implies that J.M.H was Adam H.I. Gool's (Cissie's) half-brother, making him the uncle of all children including Dr., and excerpt [19] is actually mentioning the same event but J.M.H Gool's name is not present there it can be inferred from excerpt [6].

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

[16] mentions Mahomed as J.M.H. Gool's senior, financier, and possibly mentor. [20] states that in a Supreme Court transcript referring to J.M.H. Gools participation in British Justice, there was a case where he was mentioned along with Adam H. G. Mahomed (also referred to as Adam Gool).

### q34 — What was the Group Areas Act and how did it affect District Six?

The provided sources do not provide an in-depth explanation of the Group Areas Act, but they indicate that it was a law that intended to separate different racial groups into specific areas. This was part of the Apartheid system implemented by the South African government.

According to the sources [10] and [20], the Group Areas Act affected District Six, which was a thriving cosmopolitan area in Cape Town. The Act led to the dispersal of its people across the barren sandhills of the Cape Flats, effectively decimating the once-vibrant community and way of life.

In source [14], it is mentioned that the Juggernaut of Apartheid "grinding onwards" included the Group Areas Act as a key piece of legislation. The law transformed the country into a patchwork of Black, Coloured, Indian, and White areas, with a focus on segregating different racial groups.

Source [15] further explains that the South African Indians were one of the Non-White groups most aggrieved by the Group Areas Act, as it would have herded them into a single group area, potentially causing problems for small traders.

### q35 — Who was Hassen Mall?

The sources mention several facts about Hassen Mall, a student who came to Cape Town from Durban to do medicine. He is described as:

* Brilliant cricketer who spun a wristy leg break that "zipped like a circular saw" [8]
* Had strong magnetism and charisma
* Not tall but had an exceptional personality
* Was left-handed
* Studied Law at the University of Cape Town, where he excelled and qualified with a BA LLb degree in 1951 [8]
* A versatile individual who made a big impact on the author's life
* Left for Durban after completing his law degree to study medicine 
* Member of The Indian Congress, along with Fatima's husband Dawood [17]

In cricket, he:

* Was an excellent batsman and bowler
* Was the captain of a team that included the author in the biennial tournament in 1951 (source: [17] indicates that Hassen was already no longer part of the team mentioned in [13])
* Led the team during their championship campaign at Curry's Fountain in Durban, where he captained side, beating Eastern Province.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpts [3] and [13], several political organisations were active in the Cape Coloured community during the author's lifetime:

1. The Coloured People's Vigilance Committee (excerpt [4])
2. The African People's Organisation (A.P.O.) (excerpt [8])
3. New Era Fellowship (excerpts [6], [9], and [20]). However, it is not mentioned as specifically "Coloured" in excerpt [20].
4. Anti-CAD (Communist Party aligned) active in the community by 1943, as revealed in excerpts [10] and [18].

Additionally, there are mentions of various groups and movements, but these four are explicitly identified as organizations relevant to the Coloured community:

* The Communist Party South Africa ([17])
* Fourth International of South Africa ([8])
* Liberation League ([8])
* Workers' Party ([8])
* Smuts's United Party (mentioned separately for its Coloured branch in [8])
* Moravian church ([8]), 
The New Era Fellowship is mentioned as "regarded from their inception" as a radical force in the community, having sparked a revolt among the “Coloured” community.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi is widely known as the leader of India's independence movement against British rule. The documents do not specify how or when Gandhi became "widely known" but refer to him being reprinted in an article by Ramsey MacDonald, a British Labour Party MP. However, it is mentioned that Grandpa initially referred to him as Mahatma Gandhi before his global fame [2].

Gandhi first came to South Africa in 1893 on a one-year assignment from Dada Abdullah (not a specified person) to assist his attorney in a civil suit against a merchant in Pretoria. He was sent because the accounts were kept in Gujarati, which was his mother tongue [6].

This information is found in excerpts [2], [6].

### q38 — Who was Cissie Gool's father?

The provided sources do not explicitly mention who Cissie Gool's father was.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to various excerpts in the provided sources:

* District Six was a "cosmopolitan area" (excerpt [4]) with a rich tapestry of cultures represented by its inhabitants. 
* The author recalls its energy and vitality vividly, stating that those "bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention" (excerpts [1], [3]). 
* It was known for its vibrant culture, with children playing in the streets, schools being located there, and the area buzzing with life. "Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way" (excerpt [3]).
* In terms of housing, many residents rented small houses without ceilings or internal doors, which were "ovens in summer and ice chests in winter" (excerpt [11]). 
* The area also had its share of challenges, with crime and violence being present (excerpt [17]).
* Overall, the author reflects on District Six as a vibrant and dynamic neighbourhood that was eventually bulldozed out of existence due to Apartheid legislation.

It's worth noting that these reflections are based on personal experience, rather than an exhaustive or comprehensive description of the entire neighborhood.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is mentioned in several excerpts. According to [20], if people insisted on voting, they were asked to spoil their ballots by writing "For Full Democratic Rights" across it.

This policy is also implied in [3] and [19]: the Movement stood for boycotting the election, but told voters to write "For Full Democratic Rights" on the ballot slip if they voted anyway.

