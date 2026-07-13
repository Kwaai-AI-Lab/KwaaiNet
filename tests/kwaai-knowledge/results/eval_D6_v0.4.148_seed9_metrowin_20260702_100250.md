# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 89.5% (187.0/209) |
| Avg latency | 18163ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 8840ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 5386ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 10374ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 6893ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 39348ms |
| q06 | Tell me about Buitencingle. | 7/8 (88%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 35401ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 6200ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 10990ms |
| q09 | Who was the author's grandfather? | 6/9 (67%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 25412ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 28694ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 20884ms |
| q12 | Who was Cissie Gool? | 5/6 (83%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 34708ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 12461ms |
| q14 | Where was District Six and what kind of place was it? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 14401ms |
| q15 | What were the forced removals from District Six? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 19714ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 5/5 (100%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 21258ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hewat Training College] | 7889ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: New Era Fellowship] | 10346ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 26698ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Kismets Cricket Club] | 26844ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 18896ms |
| q22 | Who was the author's father? | 4/4 (100%) | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 6974ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 8636ms |
| q24 | Who were the children of J.M.H. Gool? | 7/7 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 28622ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 28099ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 33298ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 0/5 (0%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 12654ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 38143ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 4/4 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 15133ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 6/6 (100%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 9447ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 9289ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 9834ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Haji Joosub Maulvi Hamid Gool], LEST WE FORGET -rev25.pdf | 18855ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 26780ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 21679ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cape Coloured political organisations] | 10897ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 6/7 (86%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 12565ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 4931ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 20090ms |
| q40 | What was the Unity Movement's boycott policy? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 18986ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

According to Excerpt [4], the author's children are:

1. Feyruz
2. Reza
3. Zarina

### q03 — Who are the author's grandchildren?

According to the provided sources:

* The author's grandchildren are mentioned in the dedication as:
	+ Joseph Rassool [9]
	+ Samuel Rassool [1]
	+ Alexander Rassool [1]
	+ Leonardo Rassool [1]
	+ Dylan Rassool [1]
	+ Lewis Rassool [1]

They are listed by name in both the 2000 and 2014 editions of the dedication.

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis." [4]

### q05 — Who was J.M.H. Gool?

J.M.H. Gool was a person of notable position and influence in Cape Town, South Africa, during the late 19th and early 20th centuries. Here are some specific facts that can be gathered from the provided sources:

1. He was an Indian trader or merchant who settled in Cape Town.
2. He was married to two women: Bibi Gool (children: Ayesha Rassool, Mohamed Saaid Gool, Zohra, Ahmed, Goolam Gool, Jane Gool-Tabata, Minnie, Fatima Gool) and Wahida Ta’al (children: Abdul Hamid Gool, Mariam, Zobeida, Mohammed Hanief Gool).
3. He was the grandfather of the memoir's author, Yousuf (Joe) Rassool.
4. He arrived in Cape Town in 1884 from Mauritius.
5. His family originally hailed from Swat via Gujarat, India.
6. He became a wealthy man and moved to more spacious premises in Castle Street, with rooms upstairs.
7. He was a community leader who helped to found the Hanaffi Quwatul Islam Mosque in District Six.
8. He received notable visitors, including Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman.
9. He performed the Hajj in 1899.
10. He died in April 1940.

Some other interesting facts can be gathered from the sources:

* J.M.H. Gool was a supporter of various organizations, including the British Indian League and the Coloured People's Vigilant Committee.
* He attended meetings and spoke at events, such as a meeting of the British Indian League in 1906.
* He helped to found a school that his niece attended.
* He was described as an 'Oriental trader' in Supreme Court transcripts.

Overall, J.M.H. Gool appears to have been a prominent figure in Cape Town's Indian community and played an active role in various social and political organizations during the late 19th and early 20th centuries.

### q06 — Tell me about Buitencingle.

Buitencingle is a place mentioned throughout the memoir "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool.

Here are the specific facts about Buitencingle that can be gathered from the sources:

1. **Location**: Buitencingle was located in District Six, Cape Town.
2. **Dutch origin**: The name "Buitencingle" is Dutch in origin, with "cingle" meaning "belt".
3. **Architecture**: At the turn of the century (around 1905), Buitencingle must have been a vibrant and grand house, with many rooms and a garden.
4. **Family significance**: The Gool family lived at Buitencingle. Joe Rassool's grandfather, J.M.H. Gool, was one of its residents, and the house had a significant impact on his life and growth.
5. **Layout**: The street where Buitencingle was located was wide and cobbled, with traffic islands sporting stunted palm trees. Opposite it stood the German Lutheran Church.
6. **History**: In the early 20th century, there were signs of decline at Buitencingle. Rooms were unoccupied, and evidence suggested that someone had been sleeping in one of them.
7. **Name variations**: The name "Buitencingle" was sometimes pronounced as "Baytencingle", with the "Ay" sound made rounder by pursing the lips.
8. **Geographical significance**: Buitencingle was located at the outermost belt of Cape Town, beyond which lay the foothills of Table Mountain.

These facts provide a glimpse into the history and significance of Buitencingle in the memoir "LEST WE FORGET -rev25.pdf" by Yousuf (Joe) Rassool.

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1, 11].

### q08 — Tell me more about the author's wife.

The provided sources do not contain that information.

However, we can infer some details about the author's wife from source [1] and [9].

Source [1] mentions that Nazima Rassool is also known as Professor Nazima Rassool (title: Professor) and is married to Yousuf Rassool, which makes her the author's wife.

Source [9] states that the author's wife is an academic researcher who played a central role in shaping the memoir by urging the author to ground his personal memories in historical research.

### q09 — Who was the author's grandfather?

The author's grandfather was J.M.H. Gool, also known as Joosub Maulvi Hamid Gool. 

Specific facts about J.M.H. Gool:

* Born in Mauritius (via Swat, Gujarat) [1]
* Arrived in Cape Town in 1884 [1]
* Lived at 7 Buitencingle Street, District Six, Cape Town [1]
* Had two wives: Bibi Gool and Wahida Gool [1,8]
* Children from first wife:
	+ Abdul Hamid Gool (married to Cissie Gool) [1]
	+ Jane Gool-Tabata [1]
	+ Mariam Margie Gool [1]
	+ Minnie Gool [1]
	+ Zobeida Gool [1]
	+ Goolam Gool [1]
	+ Fatima Gool (also known as Timmie) [1]
* Children from second wife:
	+ Ayesha Rassool [1]
	+ Mohammed Hanief Gool [1]
	+ Daughter named Zohra Abdurahman (married to someone with the surname Abdurahman, but her first husband is not specified in this source) [8]
* Grandfather of Yousuf (Joe) Rassool, the author [2]

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town, South Africa (1). It is a landmark of Cape Town's topography, offering views of both Table Mountain and Lion's Head (1).

According to the sources, Kloof Nek was a connecting saddle of land that joined Table Mountain to Lion’s Head (2). Beyond it lay the foothills of Table Mountain reached via Kloof Street, which led from the bottom of Buitencingle up to Kloof Nek (2).

Kloof Nek is described as a place of haunting beauty (4) and was a popular spot for lovers, mountaineers, nature lovers, and ramblers (5). However, it also had its darker side, with some people using the area as a refuge from their troubles (5).

In addition to its natural beauty and cultural significance, Kloof Nek is also mentioned in various contexts throughout the sources. For example, it was a bus terminus for a single-decker trackless tram that ran from Adderley Street outside Fletchers & Cartwrights (2). It was also a location where people would take walks, including the author and his friend Persotem Patel, who took him to Kloof Nek on a hot October day (17).

Overall, Kloof Nek is portrayed as a significant and scenic area in Cape Town's geography and history.

### q11 — What was the Teachers League of South Africa (TLSA)?

According to [1], the Teachers League of South Africa (TLSA) was a professional and political organisation for Coloured teachers. It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students.

Additionally, [5] states that the old guard had been ousted from control of the TLSA, and new leaders like Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, Frank Landsman, and others had taken control.

[6] mentions that the TLSA was in the forefront of the struggle for full democratic rights for non-citizens in South Africa, and [13] states that the step to join the Non-European Unity Movement acknowledged that the struggle for equal education was also bound up with the struggle for a democratic society.

### q12 — Who was Cissie Gool?

Cissie Gool (Zainunnisa Cissie Gool) was a South African politician, lawyer, and anti-apartheid activist. According to [1], she was the daughter of Dr. Abdulla Abdurahman, a Cape Town doctor and city councillor, and had her own involvement in politics and activism.

From various sources:

* [2] states that Cissie Gool was part of the Unity Movement, which stood against "Whites only" elections.
* [4] lists her as one of the individuals met by the author, Joe Rassool (in art, music, and politics).
* [9] mentions that her husband, Abdul Hamid, wrote medical articles for Indian Opinion in 1912.
* [11] notes that she was central to the history of District Six and the Indians and Coloured people of South Africa.
* [12] describes her as addressing a meeting at the Banqueting Hall in support of measures against "influx control" and street gangs.
* [18] mentions that in 1937, Cissie Gool almost single-handedly rocked the Government with a massive demonstration on the Grand Parade to rally against residential segregation.
* [19] further states that her actions led to her departure from the Unity Movement's politics.

Biographical facts about Cissie Gool are:

* Born: 1897-02-14 (according to [1])
* Died: 1963-12-02 (according to [1])
* Nationality: South African
* Occupation: loved (according to [1]) and lawyer/politician (as mentioned in other sources)
* Spouse: Abdul Hamid Gool (married to)
* Children:
	+ Shaheen Gool (mother of)

### q13 — What was the All Africa Convention?

[1] states that the All African Convention (AAC) was founded in 1935 in response to the Hertzog Bills, which stripped Africans of their limited voting rights. It united a broad coalition of African political organisations seeking unity against racial oppression. 

Additionally, [3] mentions that The Awakening of a People by I.B. Tabata recorded "the coming into existence of the All African Convention", indicating it was an important event.

[19] also refers to Messrs Tsotsi, Honono and Sihlali as leaders of the All African Convention.

### q14 — Where was District Six and what kind of place was it?

District Six was a district in Cape Town, South Africa. It was known for being a vibrant, multicultural community near the centre of Cape Town, situated below Devil's Peak [1]. The community was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life [1].

In addition to this information, we know that:

- District Six was declared a White area by the apartheid government in 1966 under the Group Areas Act [1].
- The district was demolished and forced removals were carried out on its non-White residents to the Cape Flats, decimating the once vibrant community and way of life [10].

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific details about the forced removals from District Six. They do mention that under the Group Areas Act, the apartheid government declared it a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats (source [1]). However, they do not provide further information on the specifics of these removals.

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi was a prominent Indian independence activist who lived in South Africa for over 20 years. He is known for pioneering satyagraha, or non-violent resistance.

The Gool family hosted Gandhi at their home, 7 Buitencingle Street, District Six, Cape Town, in 1914 before his departure from South Africa to India. Gandhi was a close friend and frequent visitor to the Gool family. In fact, he helped to furbish Dr. A. Gool's surgery by puttying and staining the floor in 1911 (Source: [8]).

Gandhi also had correspondence with Abdul Hamid Gool while he was a medical student at Guy's Hospital in London (Source: [3]). Additionally, Gandhi attended a function in honor of Mr. Gandhi before their departure from Cape Town for Phoenix, Natal, where Mr. Gandhi thanked those present, including the Gool family, for their services to him and his wife during their illness (Source: [4]).

Gandhi's connection to the Gool family is evident through various sources, which highlight the close relationship between them.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students [1]. It trained teachers under the apartheid-era segregated education system. The author and his associates in the Teachers League of South Africa (TLSA) had connections to Hewat [1].

### q18 — What was the New Era Fellowship?

According to [1], the New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community.

Additionally, [2] states that the NEF saw to the programme of lectures of the New Era Fellowship, indicating that it was involved in organizing events and discussions.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was an organization founded in 1943 that aimed to unite African, Coloured, and Indian people against apartheid and colonialism. Its program was based on the Ten Point plan for equal rights, and it adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council. [1]

The NEUM played a significant role in revolutionizing the political scene with its doctrine of non-collaboration and the boycott weapon. It galvanized the non-white disenfranchised to realize the need for a programme-based struggle that rejected the old tactics of deputations and appeals for justice at round table conferences. [8]

The organization was affiliated with the All African Convention and had leaders such as Aunt Jane, Dr. Goolam, and others who were drawn from various communities. The NEUM had a significant impact on the anti-apartheid struggle in South Africa, influencing other movements, including the African National Congress's Freedom Charter. [8] [11]

The NEUM also had a strong focus on education and equality, with leaders such as Cissie Gool speaking out against measures to deal with 'influx control' and street gangs, which was seen as a departure from the Unity Movement's politics. [14]

Overall, the Non-European Unity Movement was an important organization in South Africa during the apartheid era, fighting for equal rights and unity among different communities.

### q20 — Describe the author's involvement in cricket.

The author, Yousuf (Joe) Rassool, was actively involved in cricket through the Kismets Cricket Club and non-European cricket more broadly [1]. He played and administered club cricket in the Cape, and the club attracted several non-Indian teams that preferred to play on Sundays [4].

Rassool mentions specific cricketers he admired or had interactions with, such as Wally Hendricks (a printer who took a crucial wicket), Hassen Mall (who captained the biennial cricket tournament team), and Tiny Abed (a six-foot-plus genial teenaged giant and secret weapon of the team) [9, 12, 19].

He also participated in organizing cricket at High School and College level, but was thwarted by the entrenched bureaucracy of the Schools' Sports Union [11]. Rassool's own cricketing career involved being part of a Sunday league team that included players from various ethnic backgrounds, and he mentions specific matches and tournaments, such as the biennial tournament in Durban.

Rassool began to take cricket seriously with the help of his friend Hassen Mall, whom he credits for teaching him the forward defensive stroke [14]. He even hoped to sell his soul to Mephistopheles (a mythological figure) for just ten runs, a feat that eluded him [14].

The author's involvement in cricket seems to be deeply intertwined with his life and relationships.

### q21 — Who was the author's mother?

The author's mother was Ayesha Gool (also referred to as Lallie or Bibi). [1] 

Here are specific facts stated about her:

- Born on December 10, 1900. [10]
- Daughter of J.M.H. Gool and his wife Bibi Gool.
- She had eight children: Rasheda, Abdul, Berina, Nasim, Yousuf (Joe) Rassool, Zain, Fazil, and Yasmin (also referred to as Jessie).
- Her name was chosen by her mother-in-law, who wanted a girl named after herself. [10]
- Ayesha Gool had the habit of doing ironing in the passage outside the kitchen, which is mentioned in multiple excerpts.
- The author's father found Bibi's cooking unimaginative and asked his wife (the author's mother) to prepare more palatable meals for him. [13]

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool [1]. He was a Muslim/Indian who later converted to Islam, and he became known as Peerbhai or Peru [9].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of the author:

1. Rasheda Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Fazil Rassool
7. Yasmin (also known as Jessie) Rassool

Source: [1]

### q24 — Who were the children of J.M.H. Gool?

The text mentions multiple sources that list children of J.M.H. Gool, but it's essential to be comprehensive and include every child mentioned across all excerpts:

From [1]:
- Ayesha Rassool
- Abdul Hamid Gool (married to Cissie Gool)
- Mohammed Saaid Gool
- Zohra Abdurahman (born to Bibi Gool, making her a half-sister to the other children listed above)
- Ahmed
- Goolam Gool
- Jane Gool-Tabata
- Minnie Gool
- Fatima Gool
- Mariam Margie Gool
- Mariam Gool (doubt is cast on whether this refers to one person or two, with another Mariam mentioned in [13])
- Zobeida Gool

From [14]:
- Ayesha “Lallie” Gool
- Fatima “Timmie” Gool
- Zohra “Jolly” Gool
- Mariam “Margie” Gool
- Gadija “Dija” Noor (born to Wahida, making her a half-sister)
- Toetie Noor (also born to Wahida, making her a half-sister)

From [20]:
- Kassim

Please note that some children are mentioned in more than one source with the same name or nicknames. However, for comprehensiveness, each child is listed individually based on the information provided across all sources.

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool-Tabata, daughter of J.M.H. Gool [1]. Tabata was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy.

Specific facts mentioned about I.B. Tabata:

* He wrote the pamphlet that characterised Ben Kies' analysis as undialectical code language, un-Marxist [2].
* He completed his book "The Awakening of a People" after becoming chairman of the New Era Fellowship [3] and [18].
* The book was an occasion of considerable excitement as it would be an NEF publication [3].
* Tabata's interpretation of the origins of the organisation (All African Convention) was accepted by some, but others condemned his work for failing to have a class analysis in his historical tract [4].
* He was accused of being a Black Nationalist, one of the worst forms of political denigration at the time [4].
* Tabata's differences with Kies were not personal but political [14].
* He was involved in the Unity Movement and its policies.
* Tabata had an intense conversation with Joyce Meissenheimer about Joyce Dixon, a supporter of the NEUM [13] and [20].

The provided sources do not contain detailed biographical information on I.B. Tabata's early life, birthplace, or ancestry.

### q26 — Who was Dr. Abdullah Abdurahman?

Based on the provided sources:

1. Dr. Abdulla Abdurahman [Person]
2. Dr. Abdullah Abdurahman (also Abdulla Abdurahman) was a Cape Town doctor, physician, and city councillor — the first Coloured person elected to the Cape Town City Council.
3. He was the father of Cissie Gool.
4. Dr. Abdurahman’s grandfather was brought to South Africa as a slave, bought his freedom and became prominent in the community.
5. His mother, Gadija Dollie, one of the great beauties of Cape Town, and Wahida’s mother were also members of the Dollie clan.
6. The Abdurahman residence was in Loop Street, a mere walk away from Buitencingle.
7. Dr. Abdurahman was associated with Haji Joosub Maulvi Hamid Gool and the Non-European Unity Movement.

Biographical details about Dr. Abdullah Abdurahman:

* Born: 1872-09-08
* Died: 1940-02-20

Notable facts about his life:

* He qualified as a doctor from Glasgow University in 1893, making him the first Coloured person to do so.
* He was a city councillor and the first Coloured person elected to the Cape Town City Council.
* He was associated with prominent figures such as Gandhi, Haji Joosub Maulvi Hamid Gool, and Cissie Gool (his daughter).
* His family had connections with other influential families in South Africa, including the Dollie clan.

Please note that these sources do not provide an exhaustive biography of Dr. Abdurahman's life, but they offer some key details about his background, achievements, and associations.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

(no response)

### q28 — Which organisations was the author involved in?

The provided sources mention the following organisations that the author was involved with:

1. The New Era Fellowship (NEF) [3, 13, 16]
2. The Non-European Unity Movement (NEUM) [16, 19]
3. The Teachers League of South Africa (TLSA) [5]
4. The All African Convention (AAC) [5]

Note that the author was also a member or involved with other organisations mentioned in the excerpts, but these four are specifically highlighted as being directly associated with the author's activities.

Additionally, the author mentions affiliating with the Anti-CAD (Anti-Colour bar Discrimination), and participating in events and discussions related to various other groups, such as the Coloured branch of Smuts's United Party [16], the Communist Party of South Africa, the Fourth International of South Africa, the Liberation League, the Workers' Party [16]. However, these are not explicitly stated as organisations that the author was an active member of.

It is also worth noting that the author was a student at Trafalgar High school and was involved with their cultural society and theatrical productions [12, 13].

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to [1], the Teachers League of South Africa (TLSA) was affiliated with the Non-European Unity Movement (NEUM). They shared a boycott and non-collaboration programme.

Additionally, [12] states that "The N.E.U.M. revolutionised the political scene with its doctrine of non- collaboration and the boycott weapon." 

However, [8] mentions that in 1956, the TLSA approached the Congress movement suggesting the formulation of a unified strategy to counter the anti-educational plans of the regime, but the Congress leadership declined the invitation to cooperate.

[11] states that the TLSA was part of the struggle for liberation and its ideals were aligned with the Non-European Unity Movement.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [1], Haji Joosub Maulvi Hamid Gool (J.M.H. Gool) arrived in Cape Town in 1884 from Mauritius, with family origins in Swat via Gujarat, India.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque, also known as the Loop Street Mosque, is a mosque located on Loop Street in Cape Town. It was founded in 1898 and one of the oldest mosques in the Cape Muslim community [1]. The mosque was central to the history of the Gool family and the Cape Muslim community in District Six.

### q32 — How was Cissie Gool related to J.M.H. Gool?

Cissie Gool (also known as Zainunnissa) was the daughter of Dr. Abdulla Abdurahman [5, 8, 12]. She was married to Abdul Hamid Gool, who was a son of J.M.H. Gool. Therefore, Cissie Gool was the sister-in-law of Yousuf (Joe) Rassool, the author's grandfather.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to the sources:

- Mahatma Gandhi visited J.M.H. Gool at his home in District Six, Cape Town (Excerpt [1] and [10]). 
- Dr Abdulla Abdurahman was a notable acquaintance of J.M.H. Gool's. His daughter-in-law, Cissie Gool, married J.M.H.'s son, Abdul Hamid Gool (Excerpt [1]). Additionally, Dr. Abdurahman's grandfather was brought to South Africa as a slave and became prominent in the community; he was close geographically and by kinship to J.M.H. Gool (Excerpt [18]).
- Adam H.G. Mahomed, who is described as J.M.H.'s senior, financier, and possibly mentor, visited or worked together with J.M.H. on a case (Excerpt [1], [5] & [20]).
 
These historical figures are mentioned in the sources.

### q34 — What was the Group Areas Act and how did it affect District Six?

According to excerpts [1]–[20], the Group Areas Act was a law that aimed to create separate areas for different racial groups in South Africa. The act declared certain areas as White, Black, Coloured, or Indian, effectively segregating communities based on their race.

In District Six's case, the Act led to its forced removals. The community, which was multicultural and vibrant, had shops, schools, and a rich urban culture of music and jazz. However, under the Group Areas Act, it was declared a White area in 1966, leading to the forced removals of non-White residents to the Cape Flats.

The act's impact on District Six can be summarized as follows:

* The community was destroyed, with almost every building being bulldozed and demolished.
* Non-White residents were forcibly removed from their homes and relocated to other areas.
* The Group Areas Board devalued properties in non-White areas, forcing owners to sell at low prices or accept minimal compensation.
* This led to economic hardship for many residents who lost their homes and livelihoods.

The Group Areas Act was part of the apartheid regime's legislation aimed at maintaining racial segregation and domination. It had far-reaching consequences for communities like District Six, causing immense suffering and displacement.

(Note: The exact date mentioned in excerpt [20] is 1950, but it seems to be a typo or misplacement of information as other excerpts mention that the Group Areas Act was declared in 1966.)

### q35 — Who was Hassen Mall?

Hassen Mall was a close friend and fellow cricket player of the author Yousuf Rassool from District Six in Cape Town. He is described as brilliant, charismatic, and an excellent cricketer who played leg breaks with great skill. After completing his LLB degree, he left for Durban and later returned to become team captain for a biennial cricket tournament.

Specific facts about Hassen Mall include:

* He was a student from Durban who stayed with the author's family in Cape Town [4]
* He had a clipped brush moustache and smoked a pipe [6]
* He switched from studying science to law, where he discovered his true talent [6]
* He qualified with a BA LLB degree [6]
* He was a keen cricketer who played leg breaks and was known for his wristy spin [6]
* He had a strong personality that left an indelible mark on the author's life [6, 16]
* He was instrumental in collecting funds to finance a cricket tournament by walking over a mile of Hanover Street, accompanied by Ebrahim [15, 18]

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to [1], the main political organisations active in the Cape Coloured community during the mid-twentieth century were:

* The Non-European Unity Movement (NEUM)
* The Teachers League of South Africa (TLSA), which organised Coloured teachers
* The New Era Fellowship (NEF), a discussion and lecture forum
* The All African Convention (AAC)

These organisations shared a boycott and non-collaboration policy against apartheid institutions.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi came to South Africa in 1893 as a lawyer to work on a legal case. He lived and worked in South Africa for over 20 years, developing his philosophy of satyagraha (non-violent resistance) as a tool of political struggle [1].

He was invited by Dada Abdullah, a merchant, to assist his attorney in a civil suit against a merchant in Pretoria because the accounts were kept in Gujarati, Gandhi's mother tongue [6]. 

Gandhi had come to South Africa on a one-year assignment from Dada Abdullah and stayed for over 20 years [2].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources:

[1] describes District Six as a "vibrant, multicultural community" near the centre of Cape Town, situated below Devil's Peak. It was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life.

[3] says that to the author, those "bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other places one could mention."

[4] describes District Six as a "thriving cosmopolitan area in the heart of Cape Town" before the Group Areas Act dispersed its people across the Cape Flats. It mentions that the community was not defined by a culture of silence and acquiescence, but rather had a rich history of politics.

[7] gives a description of Caledon Street, saying it "ran from Corporation Street straight through to Clifton Street, getting tattier along the way."

Overall, District Six was described as a vibrant and multicultural community with a rich urban culture and a strong sense of community.

### q40 — What was the Unity Movement's boycott policy?

According to [1] and [11], the Unity Movement's boycott policy was to refuse participation in any apartheid government institutions, including the Coloured Advisory Council. This was based on their 10 Point Programme, which aimed for equal rights.

In particular, as stated in [1], they adopted a "boycott and non-collaboration policy" and refused to participate in any apartheid government institutions.

Further, according to [11] and [18], the boycott policy also included boycotting specific laws or policies, such as the Group Areas Act. In the case of the Group Areas Act, some members of the Unity Movement simply ignored the injunction to boycott, while others appeared before the board themselves or sent their legal representatives to register their protest against the decision.

In [8], it is mentioned that the boycott was "totally ineffective" in paralysing the Coloured Affairs Council and dealing with the Group Areas Act. However, this does not change the fact that the Unity Movement's policy included boycotting these institutions and laws.

