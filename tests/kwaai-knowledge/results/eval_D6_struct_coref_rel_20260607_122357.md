# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 57.3% (129/225) |
| Avg latency | 26061ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | LEST WE FORGET -rev25.pdf | 21776ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 31964ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 19422ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 29158ms |
| q05 | Who was J.M.H. Gool? | 2/8 (25%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 33648ms |
| q06 | Tell me about Buitencingle. | 6/8 (75%) | LEST WE FORGET -rev25.pdf | 32905ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 28758ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 22412ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 23073ms |
| q10 | Tell me about Kloof Nek. | 7/7 (100%) | LEST WE FORGET -rev25.pdf | 30276ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 41566ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | [Graph: J. M. H. Gool], LEST WE FORGET -rev25.pdf | 39274ms |
| q13 | What was the All Africa Convention? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 26002ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 22242ms |
| q15 | What were the forced removals from District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 24032ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 23433ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 22561ms |
| q18 | What was the New Era Fellowship? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 22114ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 26667ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 31650ms |
| q21 | Who was the author's mother? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 19757ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 22511ms |
| q23 | Who were the author's siblings? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22867ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Mr. Gool], LEST WE FORGET -rev25.pdf | 25792ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 25191ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 26350ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 21848ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 21789ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 27491ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | LEST WE FORGET -rev25.pdf, [Graph: Goolam Gool] | 29017ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 29931ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Mr. Gool] | 30359ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Goolam Gool] | 22931ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 27649ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 22379ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 22872ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 20008ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22471ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 25352ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22969ms |

## Answers

### q01 — Who is the author?

The author is Joe Rassool [9].

### q02 — Who are the author's children?

The author's children are Feyruz, Reza, and Zarina [19].

### q03 — Who are the author's grandchildren?

According to [1] and [20], the author's grandchildren are:

* Joseph
* Samuel
* Alexander
* Leonardo
* Dylan
* Lewis

Note: The book's dedication section in [1] and [20] lists these individuals as the author's grandchildren.

### q04 — To whom is the book dedicated?

The book "LEST WE FORGET" by Yousuf (Joe) Rassool [17] is dedicated to his children: Feyruz, Reza, and Zarina; as well as to his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

### q05 — Who was J.M.H. Gool?

J.M.H. Gool is described in various excerpts as a prominent Indian trader and community leader in Cape Town during the late 19th and early 20th centuries. He had two wives, Bibi (married in 1879) and Wahida Ta’al (whom he married after her mother died). J.M.H. Gool was known for his involvement in politics, education, and philanthropy. He was a regular target of evictions notices from the government due to his large business ventures.

According to excerpts [1], his grandson remembered him as having attended gatherings with notable leaders such as Gandhi, Cissie Gool (likely one of J.M.H.’s relatives or friends), and other prominent Indian community figures in South Africa.

### q06 — Tell me about Buitencingle.

From the provided sources [1], [3], [8], [13], [14], [15], and [20], here's what can be gathered about Buitencingle:

* Buitencingle was a large house in Cape Town, South Africa, specifically located at No. 7 Buitencingle Street ([20]).
* It had multiple rooms, including umpteen upstairs rooms that were apparently unused ([1] and [11]) and downstairs rooms that were eventually apportioned among family members ([11]).
* The street was wide and cobbled with traffic islands in the middle, sporting a few stunted palm trees ([20]), and opposite it stood the German Lutheran Church, which could be heard because of its Sunday morning clamour of bells ([20]).
* Buitencingle had a garden with fruit trees (at least one pear tree), where the author would climb to and make up stories about the fruits ([3]).
* The house was part of an outer belt area of Cape Town, beyond which lay the foothills of Table Mountain reachable via Kloof Street and Kloof Nek ([14]).
* Aboeta, Grandpa's half-brother or stepbrother (the nomenclature isn't clear in all instances), had a penchant for getting into trouble and staying at Buitencingle on occasion ([13] and [18]), which sometimes led to tension.
* There was an instance where Aboeta might have been sleeping in one of the unused rooms, leaving evidence such as burnt-down candles and newspapers ([11]).
* Buitencingle's history is associated with the Gool family, a prominent and wealthy merchant family, with its patriarchs playing significant roles in politics and society of the time ([12] and [10]).

Buitencingle seems to have been a place with rich memories for the author and his family, reflecting a life that was part of Cape Town's unique history during the times of the apartheid regime.

### q07 — Who is the author's wife?

The author, Joe Rassool, mentions that he owes a great deal to his academic researcher wife in excerpt [15] and again in excerpt [15]: "I owe a great deal to my wife, who is an academic researcher, and who urged me to ground my writing in background investigation." 

Therefore, I can say: The author's wife is an unnamed individual who is described as being an academic researcher.

### q08 — Tell me more about the author's wife.

The provided sources do not contain detailed information about the author's wife beyond what is mentioned in excerpts [2] and [16]. 

In [2], it is stated that "I owe a great deal to my wife, Nazima, who insisted that I should not rely merely on my own memories, but should do some research in order to lend authenticity to the story."

In [16], it is mentioned that the author's wife is an "academic researcher" whom he credits with encouraging him to ground his writing in background investigation.

### q09 — Who was the author's grandfather?

The author's grandfather is J.M.H. Gool [3].

### q10 — Tell me about Kloof Nek.

From the extracted text [5], [12]–[15] and [18]–[19], here's what it says about Kloof Nek:

Kloof Nek is a connecting saddle of land that joins Table Mountain to Lion’s Head (Source: [1]). 
It was described as a place of haunting beauty, which had wooded areas with cavelets, culverts, and bushes where people found refuge.

The Glen or Kloof Nek area had great historical significance in Cape Town. It also seemed an ideal place for reading poetry, particularly those like Gray’s Elegy and Milton's Lycidas (Sources: [5], [18]–[19]).

Kloof Street connected Buitencingle (the outermost belt of the city at that time) to Kloof Nek. The region had a cable car for travel up Table Mountain (Source: [4] in some versions, possibly meant for clarification in relation to [17]).
 
A historical connection with the Glen and Kloof Nek appears through references to people who used these places for studying or escape, among which is Dullah Omar, Yusuf Da Costa, Achmat Da Costa, and Dullie Desai (Source: [12]).

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources do not contain detailed documentation about the specific functions or activities of the Teachers' League of South Africa (TLSA). However, it is mentioned that TLAA became political and challenged the status quo, which led to an untenable situation for the authorities. The TLSA was said to be in the forefront of the struggle for full democratic rights [12].

From the given excerpts, some notable information about the TLSA can be gathered:

1. It had branches across different regions, as mentioned in excerpt [19], where a branch at Chapel Street Primary was discussed.

2. There was a Vice President named Edgar Maurice from the TLSA, who delivered a scholarly address about the education of non-Whites in South Africa [11].

3. The TLSA was involved in various civic and social issues, such as boycotting (or being boycoated) of events or organizations that compromised their ideals.

4. Key individuals were members of TLAA; some notable mentions include Solly Edross and Ben Kies (who taught at Trafalgar High School), Danny De Beer (the PE teacher who had an M.A. B.Ed.), and Alie Fataar among others [4].

5. The league had its own publication, the Education Journal, which was involved in training members to write about political/educational issues.

6. Its motto was “Let Us Live for our Children” [18].

7. The TLSA participated in conferences that discussed important civic and social topics [1–3 & 20].

More specific details might be available if the provided excerpts included more extensive information, but these points give a general sense of what the TLSA was involved in.

The exact context of its formation or how it evolved is not mentioned throughout these sources. However, they do provide insight into some key issues and activities the TLSAA became embroiled with during this period.

### q12 — Who was Cissie Gool?

According to the document, Cissie Gool was a prominent figure in South African politics during the apartheid era. She was married to Dr. Abdul Aziz Hendricks Gool and had four daughters: Jane (known as Margie), Fatima (Timmie), Ayesha (Lallie), and Zohra Muna (Jolly). Excerpts [4], [7], [11], [13], [14], [15], and [19] describe her political activities and relationships.

Excerpt [4]: Cissie is mentioned as being involved in the politics of the Unity Movement, although her support for measures related to "influx control" and street gangs (the "skolly problem") came as a shock to some, including the author.

Excerpt [7]: The issues of influx control were on the minds of many in the community at the time.

Excerpt [11] includes a photograph of Cissie Gool serving tea at her Mount Street residence in the 1930s, which indicates that she was also a leader of social gatherings and events in her home.

Excerpts [13], [14], and [15] further illustrate Cissie's relationship with various politicians, including Dr. Du Plessis, who addressed the conference on the need to control the influx of "Natives" into Cape Town, and Cissie Gool supporting measures to deal with this situation.

Excerpt [17]: A list shows people she associated with at social gatherings and events in London.

Excerpts [11] and [17] show that Cissie had a reputation for fighting against injustice within her community as well.

According to Excerpts [4], [11], and especially the image provided, she served tea along with Nellie Abdurahman. Excerpt [7]: Meetings related to these issues were held around the country. 

Excerpt [19] specifically says Cissie was part of a platform but also that Cissie Gool spoke in support for a Whites-only election which was contrary to her family's views; this shocked some, including the author.

Overall, Cissie Gool had involvement in various sectors of society at that time: politics, social gatherings, and public gatherings, especially on issues related to influx control.

### q13 — What was the All Africa Convention?

The description provided in excerpts [1] and [4]-[6] indicate that the Unity Movement took place after the New Year festivities, at Auntie Minnie's home. Delegates included leaders of the All African Convention (such as Messrs Tsotsi, Honono, Sihlali) and members like Enver Hassim, Errol Vawda, and Kader Essack from Natal.

This suggests that the [All] African Convention is a political organization or movement in South Africa.

### q14 — Where was District Six and what kind of place was it?

[1] describes District Six as "a thriving cosmopolitan area in the heart of Cape Town" where there is "bare bulldozed acres [that] had more life, more zest, more fire, more warmth and care than many other places one could mention." [3] mentions that after the Indian owner of the house was too young to own it at 28 years old, a Hindu association bought it and named it the Hindu Hall which was well-suited for political meetings in District Six. [4] relates that Pfaff's contribution to Non-White theatre in the Cape cannot be underestimated, suggesting an artistic presence. 

The provided sources indicate that District Six was a vibrant, cosmopolitan area in Cape Town known for its diverse community and cultural activities, which included theatre performances and political gatherings.

### q15 — What were the forced removals from District Six?

[9] mentions that "I saw District Six begin to die in the early fifties as shops in Hanover Street, the main artery of the area, started closing." However, for a more detailed account, [10] references the bulldozing away of everything that gave District Six its special flavour: "All of this was bulldozed away a few decades later to make central Cape Town for Whites only" indicating that forced removals took place and District Six was demolished.

[9] specifically mentions "I saw District Six begin to die in..." and [10] states it was bulldozed, implying the community was forcibly removed.

(No information on when exactly the removal happened; The provided sources do not contain that information.)

### q16 — Who was Gandhi and what was his connection to the Gool family?

The provided sources do not contain that information although [1] gives some context about Mahatma Gandhi's person.

However, according to [3, 6, 7, 8, 12], Gandhi had a close connection to the Gool family, particularly Dr. JMH (Yusuf Hamid) Gool and Adam Haji Gool Mahomed. Dr. JMH Gool was a public figure of Indian origin who played an important role in the fight for Indian rights in South Africa.

Gandhi's correspondence with Dr. JMH Gool is highlighted, along with instances where Gandhi visited at 7 Buitencingle Street, which belonged to Dr. JMH Gool, and interacted with his family. Gandhi was known to have invested hope in the young doctor and publicly recognized his contributions through editorials.

Additionally, there are accounts of Gandhi's involvement with other members of the Gool family, such as Adam Haji Gool Mahomed receiving a notable visitor at "Noor Bagh," an ornate rose garden located behind 7 Buitencingle Street.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college for Coloured students in South Africa. It had been established in 1944 on the site of a former Whites-only primary school that was closed to prevent children from District Six, a nearby neighborhood, from playing there [3]. The college trained teachers for upper years of the primary division and was located opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds [20].

According to the sources, the authorities at Hewat Training College were determined that it would not be an incubator of left-wing radical ideas like some other schools, and so the staff was chosen accordingly. The college had a somewhat structured curriculum, with demonstration lessons by Mr. Hogwood on the History of Education and Method, using established methods such as Herbart, Pestalozzi, Freubel, and the Dalton plan [11].

When Joe Rassool (the author) attended Hewat College in 1947, he was aware that it was a new phase of his life, although he still felt "entering a pedestrian stratum" after studying at university level [2].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural organisation established in 1937 "to discuss everything under the sun" [3]. It played a key role in countering the predicted assault on the remaining rights of the "Coloured" community and issued a clarion call to all Non-European organisations to meet to counter this attack. The NEF organised programmes of events, including lectures and discussions, and had members who were actively involved in political activism.

### q19 — What was the Non-European Unity Movement?

The documents provided do not contain a comprehensive description of the Non-European Unity Movement (NEUM). However, based on the excerpts:

* [3] mentions that the Immigration Act led to a deputation of protest led by Dr. Jabavu and others, foisting "The Coloured Affairs on the so-called Coloured people" in 1943.
* [6] states that the NEUM was formed in 1943 as a result of this government initiative, with Grandpa's daughter Aunt Jane and her brother Dr. Goolam being among its key figures.
* [9] describes the NEUM as an organization that brought the principle of non-collaboration (refusal to work with or submit to the apartheid regime), the refutation of "race" (challenging racial categories and classifications), and the ideal of a non-racial society to the struggle for liberation.
* [7] notes that the 10-Point Programme of the NEUM included demands such as the full franchise, free and equal education to the age of sixteen, and an end to forced removals.

The provided sources imply that the Non-European Unity Movement was a significant force in South Africa during this period, advocating for non-racial equality, challenging apartheid policies, and pushing for more radical action. However, the details about its structure, leadership, and activities are not fully provided.

Based on these excerpts, it is clear that the NEUM was a key event and a driving force behind social change in South Africa during this time period; the name suggests cooperation among groups of people from different ethnicities or backgrounds who were opposed to discriminatory practices.

### q20 — Describe the author's involvement in cricket.

The provided sources show that the author, Joe Rassool [9], was involved in cricket at several levels. 

He dreamt of being selected to play for his team when he was a young boy [2]. He read about and admired various cricketers, including Len Hutton, Cyril Washbrook, Dennis Compton, Godfrey Evans, Alec Bedser, Cliff Gladwin, and Doug Wright [5].

The author was initially hesitant in accepting offers of cricket practice sessions with his older colleagues due to not knowing how to field and bat well [7]. However, through time and effort, he began to master the forward defensive stroke and build a foundation for batting. He even made 12 runs, earning him great fame among students at his school [14].

Rassool also participated in non-racial Sunday league cricket. Although some of the clubs were predominantly Indian or Malay players, their policy was open to all people, regardless of ethnicity [18]. 

When Rassool began studying medicine, he made a decision that a cricket team shouldn't be formed (at first). However, he later changed his mind and started organizing a team with various interested students. This is reflected in one of the entries where he said "when I discovered Hewat as yet did not have a cricket team, I decided to do something about it" [16].

Overall, Rassool had an active interest in and involvement in the game of cricket, from reading and dreaming of being selected for the teams at younger ages to becoming involved in forming new teams at later stages.

The provided sources describe the author's experience up until his departure for England as a medical student.

### q21 — Who was the author's mother?

The provided sources do not contain that information. However, [2] mentions "my mother, tall, with hair that cascaded down her back but otherwise, quite plain of appearance," and [19] mentions Ayesha becoming engaged to Peter Alexander Rassool (the author) when she was 27 years old, implying that the author's mother might be named Ayesha or at least referred to as "my mother".

### q22 — Who was the author's father?

The author's father is Yousuf (Joe) Rassool [13].

### q23 — Who were the author's siblings?

The provided sources do not contain comprehensive information about the author's entire family. However, they mention the following siblings of the author:

1. Abdul: 
   - Source [9]: Mentioned as someone who had to share a bedroom with the author.
   - Description from source [9] implies he was hard of hearing and dropped out of school after failing his first year.

2. Fazil:
   - Source [1] mentions him being about seven years old (since "The book recounts many memories of growing up in District Six") which would make the author approximately 13 or 14 at that point.
   - Description from source [1]: He was greatly attached to the author.

3. Cissie Gool was an aunt, not a sibling.

4. Other siblings such as Toetie (whose full name is never mentioned) and Nasim were also present in sources but no sibling-specific details besides those previously mentioned are found. 

The exact number of the author's siblings cannot be determined with available information.

### q24 — Who were the children of J.M.H. Gool?

The provided sources do not contain a comprehensive list of the children of J.M.H. Gool, but they mention that his first son was born in 1886 (Source [7], [3], and [19]), and there are references to some of his other children.

Specifically:

* Kassim, mentioned as the eldest son, was born in August 1900 (Source [17]).
* Rasheda and Berina were siblings who were present when their father performed a Hajj (Source [3] mentions Berina being one year old, and Source [1] mentions having two children at home).
* Abdul, Fazil, Zainunissa (also known as Zaironesa), Nasima (mentioned in the list of people, but does not indicate that she was his child), and possibly others are mentioned in the text as the author's siblings or relatives.

The full list of J.M.H. Gool's children is likely comprehensive on pages 7–13 of "Letter from J.M.H. Gool to M.K. Gandhi" (Source [22]), but this information is not provided in the available excerpts.

### q25 — Who was I.B. Tabata?

[3] mentions that Mr. I.B. Tabata, became chairman of the New Era Fellowship, completed his book The Awakening of a People.

 [15] refers to "the Tabata faction" indicating him as a key figure in the movement. 

We cannot glean more information about who he was beyond these mentions [not a full quote from (3)].

### q26 — Who was Dr. Abdullah Abdurahman?

According to [1], Dr. Abdullah Abdurahman was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was also a medical doctor and father of Cissie Gool, the well-known activist and city councillor.

[9] mentions that he was part of an organisation along with Messrs Oosterwyk, Keraan, and Abdurahman himself, and women of calibre like Mrs Ahmed and Adams. 

The significance of Dr. Abdullah Abdurahman in the text suggests that he played a crucial role in political matters, likely related to apartheid or coloured rights.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources suggest that Mahatma Gandhi had a close connection with J.M.H. Gool through his family, particularly Gool's uncle Abdul Hamid Gool, who was a good friend of Gandhi during his time as a medical student at Guy's Hospital in London [3]. Gandhi and J.M.H. Gool maintained correspondence [20].

Additionally, the sources mention that Gandhi:

* Visited J.M.H. Gools' residence at Buitencingle in Cape Town in 1911 [13] and helped to furbish his uncle's surgery by puttying and staining the floor.
* Was offered a farewell address by Dr. Gool on behalf of Port Elizabeth Indians in August 1914 [14].
* Developed a close fondness for J.M.H. Gool, who he saw great potential for community service, and built up his image in the community through editorials reporting on his career progress.

Gandhi also visited the Gools' residence frequently during this period, as evidenced by his correspondence with Abdul Hamid Gool [3] and his involvement in the satyagraha movement led by Gandhi in 1914 [16].

### q28 — Which organisations was the author involved in?

The provided sources mention that the author (Joe Rassool) was involved with several organisations. 

He mentions being part of the New Era Fellowship (NEF), a political-cultural organisation associated with the Unity Movement [9]. The NEF advocated for unity and full democratic rights among the oppressed.

Additionally, as a student, he was involved in establishing the Hewat Amateur Theatrical Society, where he met Isaac Pfaff. He also mentions being part of the Trafalgar High School cultural society, which served as an arena for political training.

Furthermore, at some point during or before 1952, Joe Rassool and his fellow students were involved in planning to establish a Students' Union at the Cape Peninisula Students Union, though this plan ultimately fell through [4].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The TLSA (Teachers' League of South Africa) broke with its previous stance of non-political professionalism and joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society. This change in approach was led by the young Turks, who succeeded in ousting the backward leadership of the Goldings, Van der Rosses, Quints, Sonns, and others (source [2]).

The TLSA suggested a joint strategy of resistance to counter the anti-educational plans of the regime. Although this invitation was declined by the Congress leadership (source [11]), the unity movement between the two parties is evident in the TLSA's support for the Non-European Unity Movement, with some members participating in and even leading conferences (source [12]).

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The sources don't provide a direct answer to this question, but we can deduce an approximate arrival time and location for J.M.H. Gool:

From [2], "According to my cousin Nasima, her father was born in Mauritius in September."
and 
" What we also know is that the party arrived in Cape Town in December, which comes from an interview my Aunt Jane gave to Sharon Parker."

The author's aunt Jane mentions an interview (where this information came from isn't mentioned), where it is stated that the "party arrived in Cape Town in December." 

However, [2] or the corresponding excerpt of Dec 1900 is the first one mentioning J.M.H. Gool back in Cape town and there is no direct quote with which month or what was he doing. 

We can establish that by March 1902, at least one of Bibi's children (Jane Gool-Tabata) had been born in Cape Town, as mentioned in [12] that the gravestone and cape town archives record her birth on the 10th December 1899 but a telegram in MK Gandhi to JMH Gool on Dec 6, 1900 indicates that maybe Bibi hadn't returned.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque [3], a mosque completed in 1898, per [1]. It was an important structure in the area and played a significant role in the community. The author's grandfather was a founder and life trustee of this mosque (per [1]). On Eid festivals, the author would go to the Hanaffi Quwatul Islam Mosque to celebrate with his family (per [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

The sources do not specifically state how Cissie Gool was related to J.M.H. Gool, but they imply that she was a family member through her husband Dr. Abdurahman, whose mother, Gadija Dollie, was a cousin of Dr. Gool [14].

Additionally, in [15], it is mentioned that Cissie Gool disagreed with her husband's views on not identifying the Indian community's interests with those of other colored communities. This suggests that she and Dr. Abdurahman were together.

In [13], it is mentioned that at Grandpa J.M.H. Gool's funeral, the author met their cousin Shaheen, the son of Uncle Doctor (Dr. Abdurahman) and Cissie Gool, for the first time. This confirms that Shaheen was the child of Cissie Gool and Dr. Abdurahman.

It is likely that Cissie Gool was a daughter or possibly a wife of J.M.H. Gool's son (perhaps one of his four children mentioned by some researchers, whose names are mentioned in this work)

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to [20], Mr. E.S. Reddy is a courtesy for several references to Gandhi mentioned throughout this document.

### q34 — What was the Group Areas Act and how did it affect District Six?

According to source [6], The Group Areas Act "was the lynch pin, the Big Bertha 61 of the Apartheid weaponry" [1]. The act legitimized poverty and oppression for the majority of its people by circumscribing where they could live – uprooting communities and ways of life [2].

The Group Areas Act divided South Africa into separate areas based on race, with Whites being segregated from Coloureds, Indians, and Africans. This led to the classification and segregation of different racial groups in Cape Town (source [6]).

For District Six, the Group Areas Act had devastating effects: it dispersed its people across the barren sandhills of the Cape Flats, decimating the once vibrant community and way of life (source [1]). Many were forced out of their homes and relocated to newly built townships on the outskirts of Cape Town, such as Bonteheuwel, Hanover Park, and Mitchell's Plain (sources [7] and [20]).

The Group Areas Act also affected property ownership: those who owned property in District Six were forced to sell or relocate. The Government set a valuation for properties that was often below market value, and sellers had to accept a minimal amount of compensation; however, prospective White purchasers could offer low prices as they knew the Government's valuation was beneath the real worth (source [17]).

The Group Areas Act effectively disfranchised the residents of District Six by forcing them into sub-economic housing schemes that resulted in losing municipal franchise rights (source [7]).

This legislation had a profound impact on the community and its people, resulting in the loss of livelihoods, culture, and identity for the area's inhabitants.

### q35 — Who was Hassen Mall?

Hassen Mall was introduced by Muddy to Joe Rassool as a "slim, handsome green-eyed young man" (source [1]), and he was described as someone with a personality that went beyond just physical appearance. He was a student from Durban who came to Cape Town to study medicine, but then switched to law, eventually qualifying with the degree BA LLb in 1951 (source [16]).

Hassen Mall had a significant impact on Joe Rassool's life, and he became his friend and mentor. Hassen was involved in various activities with Joe, such as participating in cricket tournaments and playing cards together.

As captain of a cricket team in Cape Town, he captained the side when it went to Durban for the biennial tournament (source [17]). He was described as a brilliant cricketer who "spun a wristy leg break that zipped like a circular saw" (source [16]).

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

Based on the sources, the following political organisations were active in the Cape Coloured community during the author's lifetime:

1. New Era Fellowship [4]
2. African People’s Organisation (A.P.O.) [4]
3. Communist Party of South Africa [4], [17]
4. Fourth International of South Africa [4]
5. Liberation League [4]
6. Workers’ Party [4]
7. Coloured branch of Smuts’s United Party [4]

Additionally, the sources mention the following organisations that were formed or involved in struggles relevant to the Cape Coloured community:

1. The Cape British Indian Union (formed by Gandhi) [9]
2. The Coloured People's Vigilance Association [8]
3. The Teachers' League of South Africa [17]

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to source [1], Gandhi is referred to as "Mahatma". The title "Mahatma" means "Great Spirit".

The document does not explicitly state why Gandhi was in South Africa. However, source [15] mentions that Gandhi came to South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria.

### q38 — Who was Cissie Gool's father?

The sources do not contain direct information about Cissie Gool's father. However, it is mentioned that Cissie Gool married Abdul Hamid and had him as a son [6]. It can also be inferred through other narratives in the text that Dr. Abdurahman, the brother of Cissie Gool's mother, was close to Cissie's family (as noted in [14] and [20]). However, there is no direct mention or inference about who her father was.

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the provided sources, District Six was a thriving cosmopolitan area in the heart of Cape Town, with a rich history and diversity of cultures. It was described as "inimitable" (Source [1]) and was characterized by its vibrant community and way of life.

The neighborhood had a mix of cultures and languages, including South Africans from various ethnic groups, freed slaves from America and the West Indies (Source [5]), and Indian immigrants such as Dr. Gool's family (Sources [12] and [13]). The area was known for its diversity, with multiple languages spoken and practiced.

The neighborhood was also commercial, with a bustling market (Source [9]) and small businesses selling fish and other goods. Many residents were involved in the informal economy, which provided employment opportunities and added to the community's vibrancy.

It is also described as having a "rich history" (Sources [3] and [17]) and "pulsing life" (Source [17]), indicating that there was a strong sense of community and culture in the area. Overall, District Six was a unique and dynamic neighborhood with a diverse population and cultural landscape.

However, it's worth noting that the provided sources only offer glimpses into what life was like in District Six before the forced removals and do not provide a comprehensive description of its characteristics.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy, as stated in excerpts [3], [15], and [16], involved refusing to cooperate or engage with institutions, individuals, or organizations aligned with the government or apartheid regime. They boycotted "dummy" institutions, such as the Coloured Affairs Council (CAC), where Coloureds held positions of power but were still beholden to white authorities.

In excerpt [15], we see that all those who accepted service on the CAC were boycotted in every aspect of their lives, socially, economically, and otherwise. The message was spread throughout the country, and meetings were held almost every night at halls all over Cape Town, with the issue pervading the minds of the community.

Excerpt [5] mentions that the Unity Movement's boycott policy was thoroughly effective in paralysing the Coloured Affairs Council but ineffective against train apartheid. The Party began pressing for the addition of two Liberals to the committee, and when that was defeated, they walked out. The TARC collapsed with mutual recriminations and accusations.

Note the following quote from excerpt [16]: “Our thoughts are ours; their ends none of our own." This suggests that the Unity Movement's boycott policy aimed not only to resist apartheid but also to shape public opinion and create pressure for change on individuals in positions of power.

