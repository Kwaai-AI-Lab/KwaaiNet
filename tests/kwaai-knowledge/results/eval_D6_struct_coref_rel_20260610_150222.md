# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 53.3% (120/225) |
| Avg latency | 39088ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 60565ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 93112ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 83087ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 95220ms |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 78003ms |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 92134ms |
| q07 | Who is the author's wife? | 1/3 (33%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 38179ms |
| q08 | Tell me more about the author's wife. | 2/6 (33%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 31375ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 28898ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf | 41970ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 30618ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 33038ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 32407ms |
| q14 | Where was District Six and what kind of place was it? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 27610ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 26560ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/7 (71%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 36822ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 30182ms |
| q18 | What was the New Era Fellowship? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 35825ms |
| q19 | What was the Non-European Unity Movement? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 26783ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 33993ms |
| q21 | Who was the author's mother? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 22132ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 28997ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 22068ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 35170ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 37593ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 24930ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 27346ms |
| q28 | Which organisations was the author involved in? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 33720ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 32670ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town Dr Gool] | 31369ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 35500ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 32784ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Town Dr Gool] | 32964ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 33938ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22274ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 0/6 (0%) | [Graph: Cape Town Dr Gool], LEST WE FORGET -rev25.pdf | 28541ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 31578ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 30745ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 34344ms |
| q40 | What was the Unity Movement's boycott policy? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 28491ms |

## Answers

### q01 — Who is the author?

The author of "LEST WE FORGET" and the manuscript being queried is Yousuf (Joe) Rassool [9].

### q02 — Who are the author's children?

The author's children are Feyruz, Reza, Zarina, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [20].

### q03 — Who are the author's grandchildren?

The provided sources do not contain a comprehensive list of the author's grandchildren. However, according to the book's dedication in source [2], the author has eight grandchildren whose names are mentioned: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis, plus two other grandsons whose first names (Feyruz, Reza) match one or more of them.

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [16].

### q05 — Who was J.M.H. Gool?

J.M.H. Gool's family life is described in the sources [13] and [20]. He had two wives, Bibi, whose family lived in Broach, Gujarat, India, whom he married in 1879 or 1880 [20], and Wahida Ta'Al [13].

Source [19] mentions that J.M.H Gool is not mentioned in the Ralph Bunche’s travel notes of his stay with the Gools in 1937 [19]. 

Source [13] states that he had two wives. 

More about Dr. Gool: Source [10] says I feel a sense of humility and pride as a scion of the Gool-Rassool family tree, to bring this notable story to public attention (referring to Dr J.M.H. Gool’s story) and states that Dr. Gool was my half-uncle in source [20].

Source [18] describes an incident involving Hajima and her son Joosub Maulvi Hamid Gool: Whether Hajima, as Wahida became known, would have acquiesced in returning home, had he told her about this, is a moot point.

### q06 — Tell me about Buitencingle.

According to excerpt [5], BuitenCingle (also known as Buitencingle) was at that time the outermost belt of Cape Town, referring to a "belt" likely designating a geographic or territorial boundary. It lay beyond the foothills of Table Mountain along Kloof Street.

Additionally, as described in excerpts [10] and [20], BuitenCingle was where Grandpa Gooli's 13-room mansion stood, located at No. 7 BuitenCingle Street, according to excerpt [10]. This suggests that BuitenCingle was a relatively affluent neighborhood with prominent families residing there.

Excerpt [14] implies that Buitencingle lost its vibrancy and became a "dead shell" when the family had to shut it up, remove belongings (including Grandpa's rusty nails), and search for these items.

### q07 — Who is the author's wife?

The author's wife is Nazima [16][18].

### q08 — Tell me more about the author's wife.

According to source [20], the author, Joe Rassool, owes a great deal to his wife, Nazima, who urged him to do some research to add authenticity to their family story. 

However, I couldn't find any additional information about her in the provided excerpts. She is only mentioned as someone who insisted on doing research and lending authenticity to the story.

It's worth noting that earlier, there was a mention of Joe Rassool's wife in source [17], but it seems inconsistent or different from Nazima as it didn't provide her name. 

The correct name of the author's wife is Nazima.

### q09 — Who was the author's grandfather?

The author's grandfather was J. M.H. Gool [5].

### q10 — Tell me about Kloof Nek.

Kloof Nek was a beautiful area in Cape Town, South Africa [1, 5, 15]. It was located on the eastern flank of the city, with its head gazing down on Table Bay [15]. On its flank, like a troublesome wart, was Signal Hill from which the noonday gun heralded midday with unfailing regularity and a puff of smoke [4, 5].

Kloof Nek served as a kind of refuge for individuals looking to escape the chaos of city life. Many, including lovers and down-and-outs known as "Bergies," sought solace in its wooded area, culminating over time with the accumulation of people drinking away their sorrows [2].

In terms of geography, Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion's Head [1, 5]. There were also roads leading up to the area; one was through the gardens and up Kloof Street until reaching Kloof Nek itself [3, 4]. This made it easy for residents of Cape Town to access and enjoy the scenery.

People visited the Glen at Kloof Nek for various reasons. Some appreciated its beauty and tranquility, while others utilized it as a study space for reading poetry and literature [2, 16]. The area's peacefulness had a profound effect on individuals. Even the author notes that on descending into this woodland dell during hot October day, all they could think was how beautiful it was, and their weariness instantly dissipated [3].

It is essential to note, however, that Kloof Nek was also a site significant in South African history, given its connection to District Six, which played the role of cultural hub for the local Indian community during apartheid. The author mentions visiting this area along with his friend who wanted him to study poetry before their literature exams [14].

The natural scenery at Kloof Nek has inspired many artists; a watercolour painting by Thomas Bowler in 1837 captures its beauty from above, as seen on page one of District Six - Lest We Forget.

### q11 — What was the Teachers League of South Africa (TLSA)?

The TLSA is mentioned as a civic and social organization for teachers. It had conferences where they received training in political/educational composition and had their articles published in the Education Journal [3]. They collaborated with other organizations like the Cape African Teachers' Association to develop strategies in anticipation of a potential assault on education by the government [4].

 The TLSA was in the forefront of the struggle for democracy, and its members were also part of other political organizations. For example, Vic (presumably Victor Wessels) attended meetings of the TLSA with other groups like the Party of South Africa, the Fourth International of South Africa, the Liberation League etc [9].

 There is no indication in these excerpts that TLSA was an official teachers' union.

### q12 — Who was Cissie Gool?

Cissie Gool is mentioned in several excerpts [4], [10], and [19]. According to these sources, Cissie Gool was:

* A prominent figure in South African politics, particularly among the coloured community.
* The second wife of Haji Joosub Maulvi Hamid Gool (also known as Dr. Abdurahman) and mother of, among others, Abdul Hamid, Goolam, Jane, Minnie, Fatima, Mariam, Zobeida, and Mohammed Hanief.
* A stalwart of the Anti-CAD Movement who later deviated from the politics of the Unity Movement.
* An advocate for social, economic, and political justice, as mentioned in [7].
* A skilled organizer and mobilizer, particularly evident when she called a massive demonstration against residential segregation on the Grand Parade in 1937.
* A strong and passionate orator, able to rouse large crowds with her words.

### q13 — What was the All Africa Convention?

The All-African Convention is mentioned in several excerpts [1], [2], and possibly others as well. 

Unfortunately, the text does not provide the information we need directly but the following can be inferred from these excerpts: 
[1] describes its conference at Bloemfontein.
[20] refers to an event regarding a new publication called The Awakening of a People written by Mr. I.B. Tabata that was related to the formation, activities, or principles of the All African Convention.

We cannot say what the main goals, specific organization structure, or other key facts about the convention were with certainty from the given text.

### q14 — Where was District Six and what kind of place was it?

According to [2], District Six was a thriving cosmopolitan area in the heart of Cape Town.

### q15 — What were the forced removals from District Six?

The provided sources do not contain explicit information about the forced removals from District Six. However, it is known that the book "LEST WE FORGET" by Yousuf (Joe) Rassool discusses the history of District Six and the impact of apartheid on its residents.

But based on various other historical accounts and context about apartheid South Africa:

The forced removals were a consequence of the Apartheid government's policies to forcibly remove residents from urban areas like District Six, which was considered a "black spot" due to its mixed population. The planned development of this area led to the bulldozing of homes in 1974.

While the text contains general discussions about apartheid and its effects on District Six, specific details regarding the forced removals themselves can be found in other sources.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Based on the sources provided, Mahatma Gandhi (note: the nickname "Mahatma" is used later, but Gandhi himself was referred to as that much earlier than [14] claims, see source [3]) was a prominent figure in Indian independence movements and was known for his non-violent resistance. He had connections with the Gool family, particularly Dr. J.M.H. Gool (mentioned in various sources), who was a public worker and a respected member of the community.

The sources mention that Gandhi stayed at the Gool residence at 7 Buitencingle Street in Cape Town on multiple occasions. In fact, source [2] states that Gandhi's stay at the Gool mansion led to the friendship between Wahida "Wah" Gool (the wife of Dr. J.M.H. Gool) and Joe Rassool, who wrote this memoir.

In 1896, according to source [17], Gandhi attempted to publicize the grievances of Natal Indians while in India but was misrepresented by a Reuter cable as defaming the Natal whites, resulting in threats against his life upon their return to Natal.

Sources also mention Dr. Gool's contributions to Indian Opinion and other scholarly journals (source [28]), indicating Gandhi possibly had interactions within the community through publications like Indian Opinion. Furthermore, source [12] suggests a stronger connection: when Gandhi visited Cape Town again in August 1914, he was praised by my grandfather and Dr.

Abdurahman during his speech for receiving gifts and addresses; and an account written by E.S. Reddy indicates that Gandhi stayed at the house of Morris and Ruth Alexander (Reddy refers to himself as a director of United Nations Centre against Apartheid), on his visit which further supports an existing relationship or interactions with members from Gool and associated families during Gandhis visits.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher-training college where the author, Joe Rassool, received his training to become a school teacher. It trained teachers for the upper years of the primary division [6]. The facilities were located opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds [3]. After World War II, it was converted into the first Post Matriculation Teacher Training facility for Coloureds in the country [19].

### q18 — What was the New Era Fellowship?

The provided sources do not contain an explicit description of the New Era Fellowship. However, we can gather some information from different excerpts:

* It was described as a cultural organisation in [16] that issued a clarion call to all Non-European organisations to meet and counter the predicted assault on the remaining rights of the "Coloured" community.
* In [18], it was mentioned as an organisation where Mr. R. O. Dudley delivered an address, and members were referred to as teachers - by definition intellectuals who were part of the Non European Unity Movement.
* In various excerpts, individuals such as Harry Lawrence ([16]), Hassan Bavasah ([11] and [13]), and Hassen Rassool (the author) themselves were involved with or held positions in the New Era Fellowship.

Based on these references, we can infer that the New Era Fellowship was a significant cultural organisation connected to the Non-European Unity Movement, comprising intellectuals and educators, particularly from the Coloured community.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) is not explicitly defined in the provided sources. However, [13] refers to it as a movement that was formed in 1943 and was known as "the young Turks" among its founders. It had a doctrine of non-collaboration and the boycott weapon, which revolutionized the political scene by galvanizing the non-white disenfranchised to realize the need for a programme-based struggle.

A more specific description can be inferred from [14], where it is mentioned that the Movement had four pillars: 

* The All African Convention
* The ANC
* The South African Congress
* The Congress of Democrats (a White organization)

The goals and activities of the NEUM suggest that it was focused on promoting unity among non-European people in South Africa, advocating for their rights, and working towards a democratic society.

### q20 — Describe the author's involvement in cricket.

The provided sources do not contain any direct information about the author's involvement in cricket. There is a mention of "Orient Cricket Club" (source [7]) but it does not specify the author's role or participation in it.

However, there is a mention of Hamid instictively reaching up to catch a ball, which dislocated his finger, ending his participation in the sporting activity that was not named as cricket. 

But there is also a mention of Achee and Issy acquiring motorcycles, and the author going on rides with them which could imply some sort of social or leisure involvement with those who played cricket but no direct evidence supports it.

The provided sources do not contain specific information about the author's personal experience or participation in sports like cricket.

### q21 — Who was the author's mother?

The author's mother is not directly mentioned by name in the excerpts. However, according to [4], she gave birth to a daughter on December 10th and another child (the twins Nurjehan and Zarina) soon after Nasima was born, which indicates that she was not one of Grandpa Gool's wives or the mother of Auntie Fatima, but rather someone else.

### q22 — Who was the author's father?

According to [8], "Although their father was Muslim/Indian...  Their father was Malick Rassool." 

Also mentioned in excerpt [12] as "Magdalena (known as Auntie Goosie) thereafter married an upright Christian gentleman, Rev. Visser, by whom she bore five more children... My father was handsome, bronzed like his mother, with hair slicked back, tall and sturdy."

### q23 — Who were the author's siblings?

The provided sources do not explicitly state who the author's siblings were as individuals with names. However, [17] indicates that Gadija (Auntie Dij) and Hanief Ally were part of a foursome of children who lived with their foster parents, along with Toetie and Hamid (Midi).

### q24 — Who were the children of J.M.H. Gool?

The sources do not contain information about all the specific children of J.M.H. Gool. 

However, some specific children mentioned in the excerpt [11] from Chapter Five  Fortunes Decline are:

Back Row: Grandpa (JMH Gool), Sayed Noor (Chota) 
   Front Row: Nurjehan, Mom (Ayesha), Granny (Bibi),  
                        Uncle Aity (Sa-aid), Hoosain Ally

And in excerpt [17] from Chapter Five  Fortunes Decline: 

The home language of the Gool ménage, strange to say, was neither Urdu nor Afrikaans, but 
English.
Only with Bibi did Grandpa converse in Urdu.
To his children and Wahida, he spoke 
English. 

One specific child mentioned is Nurjehan (from excerpt [11]), another one is J.M.H.'s son Kassim (born in August), whose mother was Wahida Ta'Al (excerpt [17]).

### q25 — Who was I.B. Tabata?

The provided sources do not contain a comprehensive biography of I.B. Tabata, but they mention him as an important figure in the context of the Movement (likely referring to the Non-European Unity Movement or another organization mentioned) and his writing. According to [4], Mr. Tabata completed a book called "The Awakening of a People" which was to be an NEF publication.

[5] to [7] reference him as being involved in discussions and meetings with various individuals, including Ben Kies and Joyce Meissenheimer. However, there is no detailed information about his background, personal characteristics, or achievements beyond being the chairman of the New Era Fellowship and writing "The Awakening of a People", which suggests that he played a significant role within some organization or movement.

It's worth noting that external sources are needed to obtain more comprehensive information about I.B. Tabata, but based on these excerpts, it seems that Tabata was an influential figure in his time. 

If you would like additional context outside of the given extracts, then an extensive search involving further materials such as historical documents and other sources is advisable for a deeper comprehension of the role played by this individual within those movements and organizations he belonged to or was involved with in the country at that point in its history.

### q26 — Who was Dr. Abdullah Abdurahman?

[1] states that Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was also a medical doctor, and [16] notes that he qualified as a doctor from Glasgow University in 1893.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to [9] and [20], Dr. J.M.H. Gool was a close friend and associate of Mahatma Gandhi. They were involved in various joint ventures, such as social activism and community work.

There is evidence in [9] that Dr. Gool participated in the politics of the Indians in Cape Town and helped organize deputations to counter proposed legislation during this time.

Excerpt: "He became one of the joint secretaries of the new body (The Cape British Indian Union) and president in 1911 but resigned later that year."

[3] provides another connection between the two: Gandhi had visited J.M.H. Gool's surgery while on a trip to London, where his son was studying at Guy's Hospital. Gandhi helped puttying and staining the floor in Dr. Gool's surgery.

Additionally, [15], there is an excerpt indicating that Gandhi received a greeting from Yusuf Hamid Gool (presumably J.M.H.'s brother) upon his arrival in Cape Town with Mr. Ally to lead deputations against anti-Indian legislation

[19] also shows the close connection between Gandhi and Dr. Gool, where Gandhi mentions that Dr. Gool had arranged for meals at his place upon his visit.

It is clear that the relationship goes beyond family ties or a mere business partnership; they worked together on various community and social causes in South Africa.

### q28 — Which organisations was the author involved in?

The author, Joe Rassool [9], was involved in multiple organisations. The provided sources do not contain an exhaustive list of his involvement, but here is what they mention:

1. Teachers' League of South Africa (TLSA) [12]: The author joined this organisation and participated in its conferences.
2. New Era Fellowship (NEF) [5, 16]: The author was involved in the NEF, which aimed to politicise non-political organisations and promote unity among the oppressed groups.
3. Coloured branch of Smuts's United Party [5]: Although the source only reports that this organisation "believe it or not" joined with the others in supporting the NEF, it implies the author's involvement with this branch.

Additionally, the text mentions the author's attendance at meetings and events, such as:

* Representing the New Era Fellowship at a conference of the All African Convention in Bloemfontein in 1954 [6].
* Participating in a school production and an annual concert [9].

It is likely that Joe Rassool was involved in other organisations not mentioned here.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not explicitly state the exact nature of the relationship between the TLSA (Teachers' League of South Africa) and the Non-European Unity Movement. However, based on Excerpts [2] and [17], we can infer that:

* The TLSA "broke with its previous stance of non-political professionalism" and joined the Non-European Unity Movement ([17]).
* The TLSA acknowledged that the struggle for equal education was also bound up with the struggle for a democratic society, which is in line with the principles of the Non-European Unity Movement ([2]).

This suggests a close relationship between the two organizations, with the TLSA adopting the principles and ideology of the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1] cousin Nasima, her father was born in Mauritius in September, and the party arrived in Cape Town in December [18]. Additionally, an interview my Aunt Jane gave to Sharon Parker also indicates that "the party arrived in Cape Town in December" [2].

Since the first son of J.M.H. Gool was born in 1886, it is likely that he had left Cape Town earlier in his life.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was founded by Haji Joosub Maulvi Hamid Gool and completed in 1898 [1]. According to [20], it was a mosque located on Loop Street, Cape Town, South Africa, which was an outpost of Islam in an area becoming increasingly European, Jewish, and gentile. It was also where the author's grandfather was a founder and life trustee of the mosque, which held significance for the community's Eid festivals [20].

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not contain explicit information on Cissie Gool's relationship with J.M.H. Gool.

However, note that [7] mentions Dr. Gool (J.M.H. Gool) says Indians are right in not tying up their cause with that of the other colored, because the Indian must turn to India for his salvation. The Indian can maintain cordial relations with the colored, but must not identify his interests with that of the colored. 

This implies Dr. J.M.H. Gool was alive and active around 1911.

Later on in [14], it mentions Cissie Gool as a notable figure when describing a gathering where several prominent individuals attended. One can infer she is still active or well-known in her community.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

There are several notable historical figures mentioned in the excerpts that knew J.M.H. Gool:

From excerpt [9], it is mentioned that Gandhi extended his contacts particularly among the poorer sections and devoted much time to public work for Indians, indicating a close association.

In excerpt [11], Mahatma (M.K.) Gandhi  had contacts in South Africa with individuals such as Rev. C.F. Andrews who knew J.M.H Gool, but it does not specify that J.M.H Gool was personally known. 

Excerpt [16] mentions that Adam Gool and Abdul Hamid were known to him. Excerpt [20] further reinforces that Gool is associated with notable figures like  Mahomed.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act (1950) was a law that led to the separation of different racial groups into distinct areas in South Africa. To answer your question, we can partially deduce:

According to [1], Apartheid legitimized poverty and oppression for the majority of its peoples by circumscribing where they could live – uprooting communities and ways of life.

From [3], it is stated that By 1950, the Juggernaut of Apartheid was grinding forward... The crowning piece of the Government legislation was a law that would transform the country into a patchwork of Black “Coloured”, Indian and White areas. The Group Areas Act was the lynch pin, the Big Bertha.

From [4], it is clarified that This attempt to balkanise the country into its distinct ethnic components was done in the hope of preserving White rule for the next three hundred years.

The effect on District Six can be partially understood from various excerpts. The Group Areas Act led to the uprooting and dispersal of communities, including families living in District Six [1]. It is mentioned that Indians were forced to live in their own restricted areas outside the city [5], implying a negative impact on families like those in District Six where members had different ethnic classifications.

From these excerpts, we can conclude that the Group Areas Act was an instrument of apartheid policy designed to separate and isolate various racial groups, leading to the dispersal and suffering of communities like District Six.

### q35 — Who was Hassen Mall?

According to sources [3], [4], and [5], Hassen Mall was a student from Durban who came to Cape Town to study medicine. He had previously studied law but found it not to be his forte, at which point he switched to medicine (source [4]). He is described as having a "kind of magnetism" and being a brilliant cricketer who spun a wristy leg break that was very effective (source [4]). He also had strong character and demonstrated understanding and empathy for the struggles faced by others, such as in his involvement with the fight against the Group Areas Act (source [7]).

Hassen Mall's friendship with the author affected his life profoundly and is considered a turning point. He was known to introduce the game of table tennis to the author and taught them to play whist (source [19]).

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources mention the following political organisations that were active in the Cape Coloured community during the author's lifetime:

1. The African People’s Organisation (APO)
2. New Era Fellowship
3. Communist Party of South Africa
4. Fourth International of South Africa
5. Liberation League
6. Workers’ Party
7. Coloured branch of Smuts’s United Party

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to source [1], Mahatma Gandhi repaid Dr. Gool with attentive medical care and wrote articles for Indian Opinion and other scholarly journals, but he chose not to follow the path of social activism.

However, according to source [16], Gandhi had come to South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria because the accounts were kept in Gujarati, which was Gandhi's mother tongue [source [17]] .

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information about Cissie Gool's parents.

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources do not contain detailed descriptions of District Six's characteristics. They refer to it as a "thriving cosmopolitan area" ([3]) and mention its vibrant community, but do not elaborate on what contributed to this vibrancy.

However, they convey the idea that District Six was alive with activity [2]. The author recalls walking away from the area without regret, suggesting that he held mixed feelings about it, but does indicate that it had significant meaning for him [1].

In excerpts discussing life in District Six, stories focus more on personal relationships and family events rather than neighborhood characteristics.

### q40 — What was the Unity Movement's boycott policy?

According to [10], the boycotting of Group Areas Act meant refusing to cooperate with the boards without having any guarantee that it would stop the implementation of the law. 

[6] suggests that when implementing the boycott, Unity Movement members were expected to spoil their votes by writing "For Full Democratic Rights" across the ballot slip; not actively participating in or cooperating with a certain institution is an example of boycotting but no specifics about this case are mentioned.

This indicates the core idea of refusing co-operation or participation without any guarantee to stop the implementation of laws.

