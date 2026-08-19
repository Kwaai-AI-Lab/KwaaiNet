# RAG Eval Report

**KB:** `D6_native`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 83.5% (174.6/209) |
| Avg latency | 5309ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 7054ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 3564ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 3328ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 3248ms |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 7267ms |
| q06 | Tell me about Buitencingle. | 6/8 (75%) | LEST WE FORGET -rev25.pdf, [Graph: 7 Buitencingle Street] | 6190ms |
| q07 | Who is the author's wife? | 3/3 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 3253ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 4426ms |
| q09 | Who was the author's grandfather? | 5/9 (56%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 3580ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | [Graph: Kloof Nek], LEST WE FORGET -rev25.pdf | 6081ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 6223ms |
| q12 | Who was Cissie Gool? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Cissie Gool] | 6599ms |
| q13 | What was the All Africa Convention? | 5/5 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 4164ms |
| q14 | Where was District Six and what kind of place was it? | 5/6 (83%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 4502ms |
| q15 | What were the forced removals from District Six? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: District Six], sequence_diagram:District Six | 16023ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/5 (80%) | [Graph: Gool family], LEST WE FORGET -rev25.pdf | 7519ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | [Graph: Hewat Training College], LEST WE FORGET -rev25.pdf | 3648ms |
| q18 | What was the New Era Fellowship? | 4/4 (100%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 6571ms |
| q19 | What was the Non-European Unity Movement? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 5187ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | [Graph: Kismets Cricket Club], LEST WE FORGET -rev25.pdf | 4568ms |
| q21 | Who was the author's mother? | 5/5 (100%) | [Graph: Ayesha Rassool], LEST WE FORGET -rev25.pdf | 3557ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 3172ms |
| q23 | Who were the author's siblings? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 3668ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | LEST WE FORGET -rev25.pdf, [Graph: J.M.H. Gool & Co.] | 5456ms |
| q25 | Who was I.B. Tabata? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: I.B. Tabata] | 5324ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 5755ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 3/5 (60%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5578ms |
| q28 | Which organisations was the author involved in? | 3/3 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 4705ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/4 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 5237ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3.6/6 (60%) | sequence_diagram:Haji Joosub Maulvi Hamid Gool, [Graph: J.M.H. Gool & Co.], LEST WE FORGET -rev25.pdf | 4274ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hanaffi Quwatul Islam Mosque] | 4705ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 5508ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | LEST WE FORGET -rev25.pdf, [Graph: J.M.H. Gool & Co.] | 7944ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | sequence_diagram:District Six, [Graph: District Six], LEST WE FORGET -rev25.pdf | 5998ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Hassen Mall] | 5603ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 4/4 (100%) | [Graph: Cape Coloured political organisations], LEST WE FORGET -rev25.pdf | 4034ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 7/7 (100%) | [Graph: Mohandas Karamchand Gandhi], LEST WE FORGET -rev25.pdf | 4454ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | [Graph: Cissie Gool], LEST WE FORGET -rev25.pdf | 3252ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 7494ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Non-European Unity Movement] | 3657ms |

## Answers

### q01 — Who is the author?

Yousuf (Joe) Rassool [1].

### q02 — Who are the author's children?

The provided sources list the following as the author's children:

1. Feyruz [17]
2. Reza [17]
3. Zarina [17]

It is also mentioned that they have grandchildren, but their names are not provided in the text excerpts: 
1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis [17]

### q03 — Who are the author's grandchildren?

The author's grandchildren are listed in the Dedication section of [19] as:

1. Joseph
2. Samuel
3. Alexander
4. Leonardo
5. Dylan
6. Lewis

These names appear along with his children, Feyruz, Reza, and Zarina.

### q04 — To whom is the book dedicated?

The book is dedicated to the author's children: Feyruz, Reza, and Zarina [17], as well as his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [17].

### q05 — Who was J.M.H. Gool?

J.M.H. Gool (also known as Joosub Maulvi Hamid Gool) was a community leader, trader, and spice merchant who lived in District Six, Cape Town. He was born in 1874 or 1880 (the exact date is not specified), and his family originated from Mauritius via Swat, Gujarat. 

According to the sources [1]–[20], he had two wives: Bibi Gool and Wahida Gool. From Bibi, he had children including Ayesha Rassool, Mohamed Saaid Gool, Zohra Abdurahman, and others. From Wahida, he had children including Abdul Hamid Gool, Mariam Margie Gool, Minnie Gool, Fatima Timmie Gool, and others.

Gool was a highly respected citizen in the community, known for his achievements and contributions to society. He was described as an 'Oriental trader' in the Supreme Court transcripts of 1892. In addition to being a merchant, he was also involved in British Justice, demonstrating his standing within the Cape community.

He lived at 7 Buitencingle Street in District Six and hosted notable visitors such as Mahatma Gandhi, Cecil John Rhodes, George Bernard Shaw, and Dr. Abdulla Abdurahman. He performed the Hajj pilgrimage in 1899.

Gool's family was close to other prominent families in Cape Town, including the Dollie clan and the Abdurahmans. His son, Abdul Hamid Gool, married Cissie Gool (Zainunnissa), making her his daughter-in-law. He died on April 1, 1940.

He had a notable impact on the community, as evident from the numerous events he hosted at his residence and the many visits he received from influential figures of his time.

### q06 — Tell me about Buitencingle.

Buitencingle was a grand 13-room mansion located in District Six, Cape Town. It was the home of J.M.H. Gool, the grandfather of the memoir's author (Joe Rassool). According to source [1], the street was wide and cobbled with traffic islands sporting stunted palm trees.

Buitencingle was situated on 7 Buitenkingle Street (also known as No. 7 Buitencingle), which had a view of Table Mountain and Devil's Peak. The house had many rooms, making it necessary for the family to employ servants to manage them. Source [12] states that my grandmother tried to learn Afrikaans not only to interpret the insults of her rival but also to communicate with the servants.

Source [9] shows an old photograph of Buitencingle (circa 1905) from the bell tower of St Martini German Lutheran Church, which gives a glimpse into what the mansion looked like during that time. 

The author reminisces about his childhood at Buitencingle in various chapters. He recalls having more space than they could cope with and a garden with fruit trees (Source [2]). However, there were also signs of neglect and abandonment, such as burnt-down candles and newspapers piled on the floor as a makeshift bed.

Buitencingle had a significant influence on the author's life and growth. In source [6], it is mentioned that Buitencingle was where he stayed and experienced the sovereign effect it had on his development.

### q07 — Who is the author's wife?

The author's wife is Professor Nazima Rassool [1]. She is an academic researcher who urged the author to ground his writing in background investigation [2, 14, 16].

### q08 — Tell me more about the author's wife.

The provided sources contain several references to the author's wife, Nazima Rassool [1, 9, 20]. Here are all the specific facts that can be gathered from these excerpts:

* Her title is Professor [1].
* She is an academic researcher [1, 8, 9, 20].
* She urged her husband to ground his writing in background investigation and research, which transformed the book into a historically documented account of District Six [1, 8, 9].
* She played a central role in shaping the memoir by encouraging her husband to undertake historical research [1].

Note that there is no information available about Nazima's age, origin, ancestry, family connections, occupation, achievements, or historical associations beyond what is mentioned above.

### q09 — Who was the author's grandfather?

According to [1], the author's grandfather was Haji Joosub Maulvi Hamid Gool (J.M.H. Gool). He was a spice merchant, community leader, and founder of the Hanaffi Quwatul Islam Mosque in District Six.

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural saddle or mountain pass between Table Mountain and Lion's Head in Cape Town. It is a landmark of Cape Town's topography, offering views of both Table Mountain and Lion's Head [1]. The area is also known for its scenic beauty, with the Glen situated on its eastern flank being described as a "place of haunting beauty" [4], [17], [18].

Kloof Nek was a popular spot for lovers, mountaineers, nature lovers, and ramblers, but it also had a downside, with many people finding refuge in the area's cavelets, culverts, and bushes to drink away their sorrows [5]. There were also instances of "Bergies", mountain dwellers who lived in poverty and drank cheap wine and methylated spirits.

In addition to its natural beauty, Kloof Nek was also a hub for social and cultural activities. The area had a tram terminus, which connected the city bowl with Table Mountain [3]. It was also a place where people would gather to enjoy art, music, and politics, as described by the author in regards to his friend Cheops' house in Searle Street [9].

Overall, Kloof Nek is a significant location in Cape Town's history and geography, serving as a connection between Table Mountain and Lion's Head and offering stunning views of the city.

### q11 — What was the Teachers League of South Africa (TLSA)?

The Teachers' League of South Africa (TLSA) was a professional and political organization for Coloured teachers. [1] It was affiliated with the Non-European Unity Movement and adopted the boycott and non-collaboration policy, challenging racial discrimination in education and fighting for equal opportunities for Coloured teachers and students. [1] The TLSA had an Education Journal where members received training in political/educational composition and had their articles published. [6]

Its conferences were important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense. [2, 19] The organization was also social, as teachers from upcountry came to the city for its conferences during their winter vacation. [19]

The TLSA had members who were part of the struggle for democracy, such as Alie Fataar, Edgar Maurice, Jane Gool, Ben Kies, Rev Gordon, Walter Parry, R.O.Dudley, Solly Edross, E.C Roberts, Abe Desmore, Rev Dan Wessels, and Frank Landsman. [15] They were seen as leaders in the community, but this was an untenable situation according to the authorities because it meant a teachers' union becoming political. [16, 20]

Eventually, the TLSA was forced to change its leadership and strategies due to government pressures, leading to the development of new organizations such as the Teachers' Educational and Professional Association (TEPA). [18]

### q12 — Who was Cissie Gool?

Cissie Gool (Zainunnissa Gool) was a Cape Town city councillor, lawyer, and political activist closely associated with the Non-European Unity Movement and the anti-apartheid struggle [1].

She was born on 1897-02-14 [1] and died on 1963-12-02 [1]. Her father was Dr. Abdulla Abdurahman, a Cape Town doctor and city councillor who was the first Coloured person elected to the Cape Town City Council [1].

Cissie Gool was married to Abdul Hamid Gool and had at least one child, Shaheen Gool [1]. She was also related to other notable individuals, including her brother Goolam Gool and her cousin Nellie Abdurahman [7, 10].

She was a member of the Unity Movement and an anti-apartheid activist. Cissie Gool was known for her strong personality and her ability to inspire loyalty in others. She was described as having "the aura of a leader" [4]. 

Cissie Gool was involved in various public meetings and events, including speaking at a meeting in support of measures to deal with 'influx control' and the street gangs [5, 13, 14].

She was also known for her role in organizing demonstrations against government policies. In 1937, she led a massive demonstration of the Liberation League on the Grand Parade to rally the vote-less masses against the government's plan to introduce residential segregation [15].

### q13 — What was the All Africa Convention?

The All African Convention (AAC) was a political organization founded in 1935. According to source [1], it was established "in response to the Hertzog Bills, which stripped Africans of their limited voting rights." The AAC united various African political organizations that sought unity against racial oppression.

In source [3], the author mentions attending an All African Convention meeting and seeing some of its leaders, including Tsotsi, Honono, and Sihlali.

### q14 — Where was District Six and what kind of place was it?

According to [1], District Six was a district in Cape Town, South Africa.

According to [2] and repeated in several other excerpts (e.g., [8], [11], [20]), it was a thriving, multicultural community that was home to people of various ethnic groups, including Coloured, African, Indian, and White residents. The community was known for its rich urban culture of music, jazz, and community life.

It's also mentioned in several excerpts (e.g., [5], [11]) that under the Group Areas Act, the apartheid government declared District Six a White area in 1966 and carried out forced removals of all non-White residents to the Cape Flats, bulldozing and demolishing almost every building.

### q15 — What were the forced removals from District Six?

The sources do not provide specific details about the forced removals from District Six. However, they mention that the community was dispersed under the Group Areas Act (1950) and the haemorrhaging of shops in Hanover Street began in the early 1950s [10]. It is stated that all the vibrant community scattered and disintegrated "irrevocably pulverised by Apartheid" [10].

### q16 — Who was Gandhi and what was his connection to the Gool family?

Mahatma Gandhi is mentioned throughout the sources as a prominent figure in Indian independence movement and South African politics. He spent over 20 years in South Africa, where he developed his philosophy of non-violent resistance.

According to [1] and [2], Gandhi was hosted by the Gool family at their home, 7 Buitencingle Street, District Six, Cape Town, in February 1914. This connection led to a friendship between Joe Rassool (the author) and the Gools, as stated in [2].

Gandhi's relationship with the Gools is further highlighted in [3], where Gandhi extended his contacts among the poorer sections of the community, particularly through his association with the Gool family. In [4], a letter from J.M.H. Gool to M.K. Gandhi (1897) demonstrates their correspondence and close connection.

Additionally, Gandhi was welcomed by the Prime Minister Botha at Groot Schuur in [2]. He also visited the residence of the Prime Minister and his wife, as well as Mrs. Gladstone, the wife of the Governor-General [2].

Gandhi's personal connections with the Gool family are evident through various letters and interactions mentioned throughout the sources:

* Gandhi's son fell in love with one of Goulam Gool's sisters, a Muslim girl, which led to Gandhi refusing their union due to concerns about religious barriers [6].
* Gandhi helped putty and stain his uncle Dr. A. Gool's surgery floor [11].
* The Gools welcomed Gandhi at Buitencingle Street multiple times, including in 1911 and 1914 [9] and [18].

Gandhi also recognized the potential for community service in Dr. A. Gool, as mentioned in [15]. However, Dr. Gool chose not to pursue social activism despite his connections with Gandhi.

Overall, Mahatma Gandhi had a close relationship with the Gool family, both professionally and personally, during his time in South Africa.

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college in Cape Town for Coloured students, established under the apartheid-era segregated education system [1]. It trained Coloured teachers and was associated with the Teachers League of South Africa (TLSA) [1]. The author, Yousuf Rassool, had connections to Hewat, having planned to attend it as a schoolteacher [4, 17].

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a cultural and political discussion society in Cape Town, associated with the Non-European Unity Movement. It held public lectures and debates on political, social, and philosophical topics, and served as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community [1].

The NEF was established in 1937 "to discuss everything under the sun" [15]. It had its own programme of events and lectures, which were overseen by the Organising Secretary [9]. The NEF played a significant role in uniting various organisations within the Non-European Unity Movement, including the Coloured branch of Smuts's United Party, sports clubs, churches, and trade unions [6].

The New Era Fellowship was involved in activities such as boycotting the Van Riebeeck Festival and campaigning for democratic rights [11]. It also published a book titled "The Awakening of a People" by Mr. I.B. Tabata, which recorded the coming into existence of the All African Convention [13].

Some notable individuals associated with the New Era Fellowship include Hassan Bavasah, who was an Organising Secretary and had a devastating debating style; Ben Kies, whose analysis of the international situation was criticized by SOYA (the Society of Young Africa); and Cassim Amra, a Natal Indian who was a member of the Communist Party [17].

The New Era Fellowship was seen as a gathering place for progressive intellectuals, teachers, and activists in the Coloured community. It held lectures on various topics, including the Indian Question, and played a significant role in uniting different organisations within the Non-European Unity Movement [10].

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a movement that emerged in 1943 to unite African, Coloured, and Indian people against apartheid and colonialism [1]. Its programme was based on the Ten Point plan for equal rights, which included demands such as the full unfettered franchise and free and equal education to the age of sixteen [5].

The NEUM adopted a boycott and non-collaboration policy, refusing to participate in any apartheid government institutions, including the Coloured Advisory Council [1]. The movement was influenced by various individuals and groups, including Dr. Jabavu, who attempted to create a non-European unity movement on three occasions before the NEUM's establishment [3], [20].

The NEUM revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon, galvanising the non-white disenfranchised to a realisation of the need for a programme-based struggle that rejected the old tactics of deputations and appeals for justice at round table conferences [7].

### q20 — Describe the author's involvement in cricket.

The provided sources describe the author's involvement in cricket as follows:

* The author was actively involved in cricket through Kismets Cricket Club and non-European cricket (Source [1]).
* He played and administered club cricket in the Cape (Source [1]).
* The author mentioned learning to play cricket seriously due to his friendship with Hassen Mall (Source [8]).
* He began mastering the elements of the forward defensive stroke and built the foundation of the art of batting (Source [8]).
* In a biennial cricket tournament, the author was not selected for the team despite having played an impressive innings at Rosmead (Source [12]).

Overall, the author's involvement in cricket involved playing, administering, and learning about the game through his membership with Kismets Cricket Club and interactions with other cricketers.

### q21 — Who was the author's mother?

According to [1] Ayesha Rassool (also known as Lallie, Ayesha Gool, or Ayesha Lallie Gool) was the author's mother. She was a daughter of Bibi Gool and Haji Joosub Maulvi Hamid Gool.

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool. He was also known as Peerbhai or Peru (as the mother called him) [1].

### q23 — Who were the author's siblings?

The provided sources list the following siblings of Yousuf (Joe) Rassool:

1. Fazil Rassool
2. Abdul Rassool
3. Berina Rassool
4. Nasim Rassool
5. Zain Rassool
6. Rasheda Rassool
7. Yasmin Rassool

### q24 — Who were the children of J.M.H. Gool?

According to source [20], J.M.H. Gool had a first son born in 1886, but the child's name is not specified. The text does mention that he had two wives: Bibi (from Gujarat, India) and Wahida Ta'al (a Dollie from the Cape). 

In source [16], there is no specific information about J.M.H.'s children.

However, in source [17] and [20], the names of some of his grandchildren are mentioned. In [17], the children mentioned as being at the Gool residence during an event are:
- Abdul (age 9)
- Fazil (age 6)
- Zainunissa (or Zaironesa, age 4-ish)
- Rasheda (age 2)
- Berina (baby, with "large soulful eyes" and only a year old)
- Lallie (Yasmin's mother) was pregnant with Yasmin at the time.

In source [20], Kassim is mentioned as one of Bibi's children.

### q25 — Who was I.B. Tabata?

I.B. Tabata was a prominent political activist and leader in the Non-European Unity Movement (Unity Movement). He married Jane Gool, daughter of J.M.H. Gool [1]. 

Tabata was deeply involved in the anti-apartheid struggle and the Unity Movement's non-collaboration policy [1].

He completed his book "The Awakening of a People" as chairman of the New Era Fellowship [3][16].

 Tabata himself had written an indictment against Kies, which was evident because he rose to attack him in a meeting [4].

In 1952, we believed that his approach to political organisation was the way forward, as delineated by B.M. Tabata [14].

His talk at the NEF on "One Day in the Life of Ivan Denisovich" made a tremendous impression on me at the time [17]. 

Soon he appeared on the platform at public meetings and became a member of the inner sanctum of the National Anti-CAD [17].

### q26 — Who was Dr. Abdullah Abdurahman?

The provided sources state the following facts about Dr. Abdullah Abdurahman:

1. Associated with: Haji Joosub Maulvi Hamid Gool, Non-European Unity Movement [1]
2. Father of: Cissie Gool [1]
3. Nationality: South Africa [1]
4. Born: 1872-09-08 [1]
5. Died: 1940-02-20 [1]

Dr. Abdullah Abdurahman was a Cape Town doctor, physician, and city councillor who was the first Coloured person elected to the Cape Town City Council. He was also a leader of the African Political (later People's) Organisation (APO), a pioneering figure in Cape Coloured political life.

In addition, the sources mention that:

6. His mother, Gadija Dollie, was said to be a most beautiful woman [17]
7. His grandfather was brought to South Africa as a slave and became prominent in the community [17]

Dr. Abdurahman's involvement with Gandhi is also mentioned: 

8. He helped Gandhis when they needed assistance in reading Gujarati accounts [4]

### q27 — What was the connection between Gandhi and J.M.H. Gool?

According to [2], Gandhi had a close relationship with the Gool family, particularly J.M.H. Gool, who received notable visitors including Mahatma Gandhi during his 1914 farewell visit to South Africa before returning to India.

[3] mentions that Gandhi extended his contacts in the community, particularly among the poorer sections and devoted much time to public work for Indians.

[7] states that Gandhi had a close relationship with Dr. J.M.H. Gool, who was a medical student at Guy's Hospital in London, as evident from their correspondence, which was published in Indian Opinion on October 6, 1906 ([18]).

Excerpt [8] mentions that Gandhi saw great potential for community service in Dr. Gool and built up his image in the community through editorials reporting his career progress.

[12] states that Gandhi helped to furbish my uncle’s surgery by puttying and staining the floor, indicating a close personal relationship between them.

Overall, there was a strong connection between Gandhi and J.M.H. Gool, characterized by mutual respect, trust, and cooperation in promoting social activism and community service.

### q28 — Which organisations was the author involved in?

The author, Joe Rassool, was involved in the following organisations:

1. The New Era Fellowship (NEF) [7], [17]
2. The Non-European Unity Movement (NEUM) [1] 
3. The Teachers League of South Africa (TLSA) [4]
4. The Trafalgar High school cultural society [5]
5. The Hewat Amateur Theatrical Society [11], [19]
6. The Trafalgar Players [3]
7. The All African Convention (AAC) [2] and later in life the Cape British Indian Union, of which he was joint secretary then president in 1911.
 
The author also mentions involvement with other organisations such as the Liberation League, the Workers’ Party, and the Communist Party of South Africa.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The sources do not explicitly state a formal organizational relationship between the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement (NEUM). However, they are associated with each other.

[1] mentions that the NEUM was affiliated with the All African Convention. [8] indicates that Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam, were "moving spirits" of the young Turks within the NEUM. [11] states that the TLSA worked with dedication to spread the Parent Teachers' Association movement throughout the Cape after a split in the movement was put behind them.

[18] explicitly states that the Teachers' League of South Africa (TLSA) joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society. This suggests that the TLSA and NEUM shared similar goals and values.

The TLSA's involvement with the NEUM is not detailed in the sources. However, it appears to be a collaborative effort rather than a formal organizational relationship.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The provided sources do not contain specific information about when J.M.H. Gool arrived in Cape Town or from where he came. However, it is mentioned that Bibi Gool's family lived in Broach, Gujarat, India, and she was married to Haji Joosub Maulvi Hamid Gool (also known as J.M.H. Gool) in 1879 or 1880 [1].

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was an Islamic place of worship, specifically an mosque, located in Loop Street, Cape Town. It was founded in 1898 and was central to the history of the Gool family and the Cape Muslim community in District Six [1]. The author's grandfather, Dr. Abdullah Abdurahman (but text states J.M.H. Gool), was a founding trustee of the mosque [1].

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to [1], Cissie Gool's husband Abdul Hamid Gool was the son of J.M.H. Gool. Therefore, Cissie Gool was the daughter-in-law of J.M.H. Gool. 

However, according to [3] and [9], there is a more complex family relationship: J.M.H. Gool had two wives, Bibi (who left him) and Wahida Ta'Al. It is implied in these excerpts that Cissie Gool was the daughter of Dr. Abdurahman, making her J.M.H. Gool's step-daughter or possibly a foster child, given the close relationship between Cissie and J.M.H.'s family.

The exact nature of their relationship is not specified in all sources, but it is clear that Abdul Hamid Gool was the son of J.M.H. Gool and Cissie Gool was his wife.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

Based on the provided sources:

From source [3], J.M.H. Gool was a member of the Non-European Unity Movement (NEUM), and among other individuals, Robben Island's founder member Roberts presided over an event where Wilson, F Gow, James Currey, and Rev R.A. Jackson were also present.

Source [8] mentions that J.M.H. Gool met luminaries such as Fred Bodmer, Gregoire Boonzaaier, Terence Macaw, Ben Kies, Paul Kostin, Wolf Kiebel, J.G., Dora Taylor, Jane Gool, I.B. Tabata, Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdullah Abdurahman, Leah Solomon, and Zaid Gamiet.

Source [12] mentions that Gandhi stayed at J.M.H. Gool's residence in 1914.

From source [18], J.M.H. Gool is not mentioned in Ralph Bunche’s travel notes of his stay with the Gools in 1937.

Therefore, some notable historical figures who were acquainted or associated with J.M.H. Gool include:

* Gandhi
* Dr. Abdulla Abdurahman
* Eddie Roux
* I.B. Tabata
* Eli Weinberg
* Waradeya Abdurahman

Additionally, source [5] mentions that Mahomed was not related to J.M.H. Gool but may have shared a clan affiliation and was his senior, financier, and possibly mentor.

The following individuals are mentioned as being on the same platform or present at events associated with J.M.H. Gool:

* Wilson
* F Gow
* James Currey
* Rev R.A. Jackson
* Roberts (founder member of Robben Island)
* Dr. A Abdurahman

These individuals were likely acquaintances, associates, or fellow activists of J.M.H. Gool, but their exact relationships are not specified in the provided sources.

### q34 — What was the Group Areas Act and how did it affect District Six?

According to source [1], the period of recapturing subjugated cultural histories of Cape Town begins in 1897. By 1950, the Group Areas Act had been established (source [1] and [2]). The Group Areas Act was legislation that aimed to separate different racial groups into specific areas, with the goal of preserving White dominance.

Source [2] states: "This is the story of the life and times of a family living in District Six, a thriving cosmopolitan area in the heart of Cape Town, before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Source [10] describes the effect of the Group Areas Act: "This obscene social experiment, that would devastate the lives of hundreds of thousands of the unfranchised of South Africa, occurred. To add insult to injury the Prime Minister, Daniel Francois Malan, declared sanctimoniously in his New Year's message to the nation..."

Source [14] describes the Group Areas Act as "The crowning piece of the Government legislation... The Group Areas Act, was the lynch pin, the Big Bertha of the Apartheid weaponry."

### q35 — Who was Hassen Mall?

From sources [1]–[20], we can gather information about Hassen Mall as follows:

* He was a close friend and fellow cricketer of the author Yousuf Rassool from District Six, Cape Town.
* He is described as "dark, lean and athletic" with a "magnetism" that could not be denied. (Source [4])
* He played cricket for various teams and was known for his skills in bowling and batting. In particular, he was the captain of a team participating in the biennial tournament at Curry's Fountain in Durban in 1951.
* After completing his LLB degree, Hassen Mall left for Durban to study medicine (Source [4]). However, it is not clear whether he actually completed his medical studies or continued practicing law instead.
* He was a member of The Indian Congress and had connections with other notable figures such as Dawood and Fatima (Sources [15] and [16]).
* Hassen Mall was described by the author as "my hero" who profoundly affected the author's life in the years that followed their meeting.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The main political organisations active in the Cape Coloured community during the mid-twentieth century were: [1]

* The Non-European Unity Movement (NEUM)
* The Teachers League of South Africa (TLSA)
* The New Era Fellowship (NEF)
* The All African Convention (AAC)

These organisations were united by the boycott and non-collaboration policy against apartheid institutions.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to [1], Mohandas Karamchand Gandhi, known as Mahatma Gandhi (Mahatma meaning "Great Soul"), came to South Africa in 1893 as a lawyer on a one-year assignment for the merchant Dada Abdullah. He stayed on for over 20 years developing satyagraha (non-violent resistance) after Natal moved to disenfranchise Indians in 1894.

He was in South Africa to assist his attorney in a civil suit against a merchant in Pretoria, and he had been sent for because the accounts were kept in Gujarati, which was Gandhi's mother tongue [7].

### q38 — Who was Cissie Gool's father?

Cissie Gool's father was Dr. Abdulla Abdurahman [1].

### q39 — What was District Six like as a neighbourhood before the forced removals?

According to the sources, District Six was a vibrant and multicultural community near the centre of Cape Town. It was situated below Devil's Peak and was home to Coloured, African, Indian, and White residents who built a rich urban culture of music, jazz, and community life (Source [1]). The neighbourhood had a "pulsing life" with "more life, more zest, more fire, more warmth and care than many other places one could mention" (Source [2]).

The author remembers the neighbourhood as having a strong sense of community, where people knew each other and looked out for one another. In Source [3], it is mentioned that the author's school grounds were undeveloped, but it was deemed good enough to be the site of the first Post Matriculation Teacher Training facility for Coloureds in the country.

Source [17] mentions that a community of freed slaves from America and the West Indies thrived in District Six in Victorian times. Some of them would achieve great position and wealth. Their expulsion at the turn of the century foreshadowed the Apartheid era evictions of the same neighbourhood decades later.

The author also describes the area as having a rich history, with many families living together in close proximity (Source [18]). The book tells the story of the life and times of a family living in District Six, which was a thriving cosmopolitan area in the heart of Cape Town before the Group Areas Act dispersed its people across the barren sandhills of the Cape Flats.

It is worth noting that while the author's description of District Six is generally positive, some sources suggest that the neighbourhood also had its challenges. For example, Source [5] mentions that the area was plagued by crime and violence during World War II, with gangs gatecrashing house parties and terrorizing guests.

Overall, based on the available sources, it appears that District Six was a vibrant and multicultural community before the forced removals, but also had its challenges and complexities.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement adopted a boycott and non-collaboration policy - refusing to participate in any apartheid government institutions. [1] Some members of the "Movement" simply ignored the injunction to boycott, but those who did boycott were mainly those who accepted service on the Coloured Affairs Council (CAC) and were boycotted in every aspect of their lives, socially, economically, and otherwise. [10]

