# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 51.6% (116/225) |
| Avg latency | 24444ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 69265ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 23985ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf | 18677ms |
| q04 | To whom is the book dedicated? | 3/4 (75%) | LEST WE FORGET -rev25.pdf | 21357ms |
| q05 | Who was J.M.H. Gool? | 6/8 (75%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 27525ms |
| q06 | Tell me about Buitencingle. | 1/8 (12%) | LEST WE FORGET -rev25.pdf | 20529ms |
| q07 | Who is the author's wife? | 1/3 (33%) | LEST WE FORGET -rev25.pdf | 20090ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 20747ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 20677ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | LEST WE FORGET -rev25.pdf | 25970ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | LEST WE FORGET -rev25.pdf | 26177ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 28318ms |
| q13 | What was the All Africa Convention? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 24398ms |
| q14 | Where was District Six and what kind of place was it? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 24112ms |
| q15 | What were the forced removals from District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 21752ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 26753ms |
| q17 | What was Hewat Training College? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 18758ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | LEST WE FORGET -rev25.pdf | 31909ms |
| q19 | What was the Non-European Unity Movement? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 26168ms |
| q20 | Describe the author's involvement in cricket. | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 22412ms |
| q21 | Who was the author's mother? | 0/5 (0%) | LEST WE FORGET -rev25.pdf | 17667ms |
| q22 | Who was the author's father? | 2/4 (50%) | LEST WE FORGET -rev25.pdf | 17998ms |
| q23 | Who were the author's siblings? | 0/5 (0%) | LEST WE FORGET -rev25.pdf | 22809ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 23936ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 20962ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 23989ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 4/5 (80%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 22965ms |
| q28 | Which organisations was the author involved in? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 20978ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 23758ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 1/6 (17%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 26455ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 27849ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 23475ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 26657ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf | 28207ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 21138ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 22446ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 3/7 (43%) | LEST WE FORGET -rev25.pdf | 20739ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 21485ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 2/6 (33%) | LEST WE FORGET -rev25.pdf | 22370ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | LEST WE FORGET -rev25.pdf | 22305ms |

## Answers

### q01 — Who is the author?

The author is Yousuf (Joe) Rassool [9].

### q02 — Who are the author's children?

The author's children are Feyruz, Reza, and Zarina [13].

### q03 — Who are the author's grandchildren?

The author's grandchildren are listed as: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [20].

### q04 — To whom is the book dedicated?

The book is dedicated to "my children: Feyruz, Reza, and Zarina" and also to "grandschildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis". [2]

### q05 — Who was J.M.H. Gool?

According to the sources provided [3,[6][8][12] J.M.H. Gool appears to be a significant figure in the community, but his identity is somewhat fragmented across multiple excerpts.

From Excerpt [6], we know that Gool's mother is referred to as Peari Beghum, and she may not have been her real name (Source 6 refers to "ciraj Rassool"). 

Excerpt [3] introduces J.M.H. Gool as a person whose grandfather had two wives. However, it does not directly mention what this individual's role or achievements were.

Further, we see in Excerpts [9],[13],[16], and [20] that his full name was Joosub Maulvi Hamid Gool, who achieved prominence in the community as a public worker and possible politician (Source [17] mentions involvement with the Cape British Indian Union). He may have become involved with organizations such as the Wooding’s Preparatory School (Source [10]).

However, we also notice from Excerpt [19] that his family played an important role in the community: J.M.H. Gool's son was born in 1886 and Gool himself testified in Supreme Court transcripts in 1892.

### q06 — Tell me about Buitencingle.

The provided sources do not contain detailed information about Buitencingle beyond it being a location where the author and his family lived at some point, described as having "umpteen rooms" [9], a garden with fruit trees [5], and being located on Buitencingle Street [7].

### q07 — Who is the author's wife?

The author's wife is Nazima [2].

### q08 — Tell me more about the author's wife.

The provided sources do not contain much information about the author's wife beyond listing her names. 

[8] mentions "my wife", who is described as an academic researcher.
 
[7] tells us that "I was immensely encouraged by the words of Brian Willan quoted above, which supported the project I had embarked upon." That statement doesn't mention his wife.

[17], [18], and [20] state that Nazima urged him to ground writing in background investigation.

### q09 — Who was the author's grandfather?

The author states [5] and [7] that J.M.H. Gool, also known as "Grandpa", was his grandfather. 

Note: This information comes from explicit statements in excerpts [5] and [7].

### q10 — Tell me about Kloof Nek.

Kloof Nek is a location mentioned in several excerpts. It appears to be a natural area with scenic beauty, situated above Cape Town and forming a connecting saddle of land between Table Mountain and Lion's Head. According to the sources, it was a popular spot for various groups of people.

The Glen at Kloof Nek was described as a place "of haunting beauty" (Excerpt [5], [19]). It is mentioned that some people would take buses to this area on weekends, possibly for recreational purposes or simply to enjoy its natural beauty. 

Additionally, the sources suggest that Kloof Nek played an important part in the social life of various communities, with different groups of people often gathering there (Excerpt [5], [19]).

### q11 — What was the Teachers League of South Africa (TLSA)?

The provided sources do not contain a detailed description of the Teachers' League of South Africa (TLSA). However, based on various references across the excerpts:

- [3] and [13] mention that Cassim Amra was a member of both the Communist Party and TLSA ([3]), which indicates that some members of TLSA were affiliated with the communist movement.
- [5] describes Danny De Beer and Ben Kies as "the first non-Whites to teach at Trafalgar High School" ([5]). Solly Edross (B. Sc) and Ben Kies (M.A. B. Ed) are also mentioned as members of TLSA in this context, which implies that the league focused on improving education and working conditions for non-white teachers.
- [9] mentions Danny De Beer's membership in TLSA, further supporting that the organization included educators as its members ([9]).
- [16] talks about the Education Journal being associated with TLSA. This suggests a publication aspect of the organization.

### q12 — Who was Cissie Gool?

Cissie Gool is mentioned in several excerpts as a key figure in her family and community. She was married to Dr. Abdulla Abdurahman, the grandfather of the author Joe Rassool [20]. 

She was involved in politics, notably calling a massive demonstration against residential segregation in 1937 [2] and speaking out on issues such as influx control and street gangs [13]. She was also a member of the Unity Movement [4] but later departed from its politics [14].

Aunt Cissie is often mentioned alongside her husband Dr. Abdurahman, indicating a close relationship with her spouse and possibly a significant female figure in their community.

She is remembered for speaking at the Banqueting Hall on the issue of influx control, which was a shock to the author given his aunt's previous involvement in anti-government politics [12].

Excerpts also show Aunt Cissie serving tea at their Mount Street residence alongside her political colleague Nellie Abdurahman [13] and being part of gatherings such as picnics where they would engage in heated debates against Ismail, a fierce adversary who was known to pillory women, including Aunt Cissie [16].

### q13 — What was the All Africa Convention?

This information is found in multiple sources, notably in [1], which mentions "the leaders of the All African Convention- Messrs. Tsotsi, Honono and Sihlali" being present at a conference.

From [6], an account of people's lives can be read with differing outcomes: one person's future might have been "all these benefits" had she not suffered from the color bar.

Furthermore, in [19] we understand this Convention recorded The Awakening of a People by Mr. I.B. Tabata to be its formation along with being an NEF publication.

Additionally in [20], Vic Wessels, Les Jacobs and Joe Rassool (the author) attended the conference of the All African Convention in Bloemfontein.

We cannot pinpoint what "All Africa" refers to as there's no source saying 'Africa' but rather it seems a common misspelling which is likely to be an alternate name for "All AFRICAN".

### q14 — Where was District Six and what kind of place was it?

The provided sources do not contain specific information about the exact location of District Six (its latitude and longitude). However, we can infer its location from the context. 

District Six was a "thriving cosmopolitan area in the heart of Cape Town" [15], suggesting that it was centrally located within the city.

It is described as a vibrant community with a rich history of politics and struggle against the Apartheid state, rather than being defined by silence or acquiescence [13]. The author's family lived there before the Group Areas Act (1950) dispersed its people across the Cape Flats, suggesting that it was a densely populated area.

Source [9] describes Caledon Street as running from Corporation Street to Clifton Street and notes that Clifton and Hanover Streets crossed at a hub or "fountainhead" of District Six. This implies that District Six had multiple streets leading into its central area.

The descriptions suggest that District Six was a lively, diverse community in the heart of Cape Town, with a strong sense of identity and struggle against Apartheid policies.

### q15 — What were the forced removals from District Six?

The provided sources do not directly address the specific issue of forced removals from District Six. However, sources [11] and [17] mention that District Six was a thriving cosmopolitan area in the heart of Cape Town before the Group Areas Act (1950) dispersed its people across the barren sandhills of the Cape Flats.

### q16 — Who was Gandhi and what was his connection to the Gool family?

The provided sources do not contain a direct biography of Gandhi. However, based on the available information:

Gandhi is mentioned as a figure who interacted with members of the Gool family in various contexts. He is referred to as "Mr." and later as "Mahatma" (Great Spirit), suggesting a high degree of respect.

In one instance [11], it is stated that Dr. Gool offered a farewell address to Gandhi on behalf of Port Elizabeth Indians, during which he hailed Gandhi as 'Mahatma'. This implies a close relationship between the two individuals, with Dr. Gool expressing admiration for Gandhi's work.

Additionally, the Indian Opinion article mentioned in [11] contains an excerpt from Ramsey MacDonald's travelogue that reprinted a travelogue about the Gurukula at Hardwar, where Gandhi is stated to have been referred to as Mahatma.

The connection between Gandhi and the Gool family appears to be rooted in their shared community and interests. The Gools are mentioned as having interacted with Gandhi on several occasions, providing him with hospitality and support during his visits to Cape Town. Dr. Gool's involvement with Indian politics and his own professional endeavors suggest that he was an active figure within the local Indian community.

It is also notable that Gandhi visited 7 Buitencingle, a Gool residence, in 1911, staying with family members (likely relatives of Dr. Gool) while there [9].

### q17 — What was Hewat Training College?

Hewat Training College was a teacher training college. Specifically, it trained teachers for upper years of the primary division (according to [10] and [20]). The college had previously been a Whites-only Primary school but switched to being a teacher training college in 1944 ([9], [15], [19], [20]).

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a political, cultural organization established in 1937 "to discuss everything under the sun" [3]. It played a crucial role in the community and saw to the program of lectures of the NEF [1], indicating that it featured regular gatherings for discussion and debate. The author was involved with the NEF, chairing meetings, and was glad to be in this position as the audiences grew steadily [7, 16]. Despite its cultural goals, the narrative suggests that it maintained strong political commitments, as the NEF sought to assert its position in relation to other organizations, such as the Unity Movement and its various factions (e.g., [17]). The book provides extensive details about the experiences of the author and others within this context but does not supply a clear definition or description of the New Era Fellowship.

However, we can infer several things:

1. **Goals**: Its goal, according to [3], was "to discuss everything under the sun," suggesting an intention to be inclusive, provide a forum for discussion on various topics, and encourage participation.
2. **Activities**: Meetings were organized, with lectures given by members or guests (e.g., Ben Kies's lecture on Bernard Shaw in [1]). This indicates the organization was community-oriented but also had an educational component.
3. **Influence and Controversy**: The author mentions a "split" within this movement around the formation of the Society of Young Africa, indicating that the NEF was influential enough to be involved in these kinds of controversies (see [8]). 
4. **Political Stance**: While not explicitly stated, the narrative gives hints at a progressive and challenging stance toward authority, as seen in the author's criticism of Councillor Ismail, reactions against nationalism ([15]), and controversy surrounding Ben Kies.

Therefore, the New Era Fellowship appears to be an organization that promoted critical thinking, discussion, and activism within its community. Despite not being described in detail, its presence played a significant role in shaping community dynamics during the period discussed.

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain a full description of the Non-European Unity Movement, but they mention that it aimed to achieve a programme-based struggle with non-white people in South Africa. The movement's principles included non-collaboration and the use of boycotts as a weapon, with the goal of achieving democratic equality and ending apartheid.

According to [14], the movement was formed in 1943 by the "young Turks," led by figures such as Grandpa's daughter, Aunt Jane, and her brother, Dr. Goolam. It drew many of its members from various non-white groups in South Africa and aimed to create a united front against apartheid.

Additionally, source [12] mentions that the movement was active throughout South Africa during this time, with local branches functioning in Natal and Transvaal. The movement's leaders claimed that their strategy would ultimately lead to the triumph of morally superior ideas over those that were inferior.

It is worth noting that there is little information on how this movement specifically operated or what its key policies were beyond these sources.

### q20 — Describe the author's involvement in cricket.

Based on the provided sources, it appears that the author, Joe Rassool [9], had a significant involvement in cricket during his youth. Some key points about the author's involvement in cricket include:

* The author learned to play cricket seriously due to his friendship with Hassen Mall [10, 15].
* He demonstrated skills as a batsman, achieving several eighties against the top cricketer of that time, Ben Cassiem [18].
* He even mastered the forward defensive stroke and had an ambition to score at least 10 runs, even considering offering his soul to the devil for this achievement [17].
* The author also tried to organize a cricket union in schools and colleges but was thwarted by the entrenched bureaucracy of the Schools' Sports Union [14].

However, it is worth noting that while the author expressed a passion for cricket, he also prioritized political life, mentioning that his true interest lay in this area rather than cricket or table tennis.

### q21 — Who was the author's mother?

The provided sources do not contain that information.

### q22 — Who was the author's father?

The author's father is Malick Rassool [19].

### q23 — Who were the author's siblings?

[17] mentions that the South African Indians were a multi-racial community, but does not mention specific sibling names.

However [16] provides some information. 

The twins would complete their studies and then choose what to do with their lives in due time. It therefore came as a shock to me when one day Auntie Annie came to Caledon Street and said that Zarina had received a proposal of marriage from a Dr...

But it is [15] which provides some detailed information:

Nurjehan's four children, who were living with the Hanief Ally’s foster family: Gadija (Auntie Dij),  whom I loved more than words can say; Hanief Ally, my mentor, who guided me through the quicksands of adolescence; Toetie (whose real name I can’t remember ever hearing);  and Hamid (Midi), a handsome and athletic Adonis who enjoyed showing me how many chin-ups he could do.

### q24 — Who were the children of J.M.H. Gool?

According to [1] and [20], J.M.H. Gool had several children:

* First son: born in 1886 (but the book does not mention his name).
* Wahida's two children (son and daughter): Bibi arrived in Cape Town with these children, but their names are not mentioned in the excerpts.
* Kassim Abdurahman was carried to Cape Town by Dr. Abdurahman's father who had married Gadija Dollie, and his mother Nurjehan.  This seems to relate more directly to Yousuf (Joe) Rassool's family than J.M.H. Gool's.
* The following children are mentioned as being present at home when the writer was a child:
	+ Nurjehan 
	+ Twins: Zainunissa and Zarina

### q25 — Who was I.B. Tabata?

According to excerpt [2], I.B. Tabata rose through "the ladder of the Movement" after qualifying as BA LLB, and gave a powerful talk at the NEF on "One Day in the Life of Ivan Denisovich", which made a big impression on the author (Joe Rassool). 

His quick ascendancy suggested that he was an important figure in the movement, likely due to his intellectual and oratory abilities.

### q26 — Who was Dr. Abdullah Abdurahman?

Dr. Abdullah Abdurahman is mentioned in excerpts [1], [4], [6], and [19]. According to excerpt [1], he was a long-serving Cape Town city councillor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century; he was a medical doctor, and father of Cissie Gool (Zainunnissa Gool), the well-known activist and city councillor. Excerpt [6] also mentions that his grandfather was brought to South Africa as a slave, bought his freedom, and became prominent in the community.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

J.M.H Gool was a supporter of Gandhi's activism against anti-Indian legislation in South Africa. Gool wrote to Gandhi in 1897, expressing his admiration for Gandhi's stand for civil rights among Indians (Source: [14]). They collaborated in their efforts and had a close relationship. Gandhi often visited the Gools at Buitencingle Street, and he stayed with them on several occasions. The connection between Gandhi and J.M.H. Gool went beyond just friendship; they worked together towards common goals, including campaigning for Indian rights and fighting against racist laws (Source: [14] and various other excerpts).

Gandhi visited J.M.H. often to discuss the issues affecting the Indian community in South Africa, such as anti-Indian legislation, which he campaigned to change through civil disobedience methods ("passive resistance"). Gandhi's support for Indians' civic rights led to the Gools giving him subscriptions to the Indian Opinion newspaper, on behalf of Wilson (Source: [17]).

Gandhi and J.M.H. also kept in touch through correspondence; e.g., there was a letter from Gandhi to Gool about marriage and carnal sex mentioned by Gandhi (Source: [18]).

### q28 — Which organisations was the author involved in?

The provided sources do not contain an exhaustive list of all organizations the author was involved with. However, based on the excerpts, we can identify some:

1. New Era Fellowship (NEF) [7], a political-cultural organization associated with the Unity Movement.
2. Trafalgar High School cultural society [4].
3. The Moravian Church [8].
4. Anti-CAD (Anti-Coloured Affairs Department) [8].
5. Coloured branch of Smuts's United Party [4].
6. The African People's Organisation (APO) [4].
7.Cape Peninisula Students Union [19], 
 

These are some organizations mentioned in the excerpts, but it is likely that the author was involved with others as well.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The TLSA (Teach ers' League of South Africa) joined the Non-European Unity Movement, acknowledging that the struggle for equal education was also bound up with the struggle for a democratic society [3].

This step was not achieved without a fierce struggle, but in the end the young Turks succeeded in ousting the backward leadership and taking control of the TLSA [19].

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to [2], Gandhi and Mr. Ally arrived by train in Cape Town on October 3, 1906, from Johannesburg. However, the text does not specifically state that J.M.H. Gool was present at this time.

[20] mentions "the death of my aunt Jane Gool-Tabata (Wahida's daughter), I discovered that her date of birth had been the 19th March 1902." It is not directly stated where Grandpa was during this time period, but it is mentioned in [19] and [20] that:

* In February 1900, Hajima, now known as Wahida, bid farewell to Jeddah after performing their pilgrimage.
* In April or early May 1901, J.M.H. Gool departed from Bombay (now Mumbai) and left Bibi in India with their son, Mohamed Saaid.

It can be inferred that J.M.H. Gool likely traveled to Cape Town for business purposes before leaving for Bombay, but the exact date and circumstances are not mentioned.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was an outpost of Islam in a predominantly European, Jewish, and gentile area. It was located in Loop Street, and its construction took place in 1898 with money raised by the Kanamia Muslims. The mosque was used for Eid festivals by the community members, including the author's family.

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not contain explicit information about how Cissie Gool was related to J.M.H. Gool [7], but a related passage in source 16 indicates that Dr. Gool (Abdul Hamid Gool) was the half-uncle of the narrator, and his relationship to the narrator's grandfather (J.M.H. Gool) involved the fact that "my grandfather had two wives" (Bibi Gool and Wahida Ta'al).

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The author's grandfather, J.M.H. Gool, knew personally a number of notable historical figures.  According to [1] and [8], one notable figure was Mahatma (Gandhi), another Indian independence activist and leader of the Indian National Congress. He was referred to as J.M.G. or Gul in some contexts.

[12] mentions that the then prominent Indian political leader and educator, Gopal Krishna Gokhale, visited Noor Bagh where my grandfather lived, implying a connection between the two.

According to [2], [9], and [10], Dr Adam Abdurahman and J.M.H. Gool maintained close ties for many years; they co-operated with each other politically (presumably given their mutual interest in gaining democratic rights for Coloureds) and shared a bond of familial closeness, as both families belonged to the community leaders at that time.

[7] does not directly refer to J.M.H. Gool's personal interactions but mentions his being present with Adam Abdurahman during discussions on how to win over the coloured vote.

Ralph Bunche’s travel notes [17] and another note from Ralph Bunche (in [18]) report that in 1937, there was an absence of J.M.H. Gool's name.

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act (1950) [3] was an apartheid legislation that legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life. It aimed to transform South Africa into a patchwork of Black “Coloured”, Indian, and White areas [6]. The Group Areas Act classified people into racial categories and forced them to relocate to specific areas according to their designated racial group.

In the case of District Six, a thriving cosmopolitan area in Cape Town, the Group Areas Act had a devastating impact. The legislation led to the removal of over 60,000 residents, including Muslim, Christian, Jewish, African, and Indian families, from their homes [5]. Many were forcibly relocated to peripheral townships, such as Bonteheuwel, Hanover Park, Mannenberg, Mitchell's Plain, Vanguard Estate, and Bishop Lavis.

The Group Areas Act also led to the demolition of District Six, a vibrant community with a rich history. Shops in Hanover Street, the main artery of the area, started closing, and the residents were effectively disfranchised as they lost their municipal franchise due to living in sub-economic houses [7].

The Act's impact was not only physical but also emotional and psychological. The removal of families from their homes caused irreparable harm and trauma. For example, when Zaheer died, his cousin Nasim was inconsolable [15]. The Group Areas Act also led to the loss of livelihoods, as many residents were forced to abandon their businesses and shops.

The attempt to balkanize South Africa into ethnic areas was aimed at preserving White rule for hundreds of years. However, this effort ultimately backfired, leading to a more polarized and divided society [8].

According to Senator Edgar Brookes, the Group Areas Act was an essential iniquity that needed to be challenged [17]. The protests against the Act were led by liberals like Brookes and other organizations, but their efforts were met with minimal success.

Overall, the Group Areas Act had a profound impact on District Six, leading to the forced removal of its residents, destruction of homes, and trauma that still affects the community today.

### q35 — Who was Hassen Mall?

Hassen Mall was a student from Durban who came to Cape Town to study Medicine. He became friends with the author, Joe Rassool, and had a profound effect on his life in the years that followed [18]. According to the source, he was "a brilliant cricketer" [19] who "radiated a kind of magnetism that could not be denied" [19]. He also qualified for a degree BA LLb in 1944 [20].

Hassen was involved in cricket and was a team captain [1], [13]. He helped to raise funds for the Western Province Indian Cricket Union by organizing collections in Hanover Street [17].

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

According to excerpts [3] and [4], some of the political organisations active in the Cape Coloured community during the author's lifetime include:

* The New Era Fellowship (NEUM)
* The Teachers' League of South Africa
* The Communist Party of South Africa/International (represented by Cassim Amra)
* The African People’s Organisation (A.P.O.)

Excerpt [4] also mentions other organisations that responded to the call for unity and democratic rights, including:

* The Liberation League
* The Workers’ Party

It's worth noting that this is not an exhaustive list, as excerpt [4] mentions "a host of others" responding to the call.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

Mahatma Gandhi is known to be referred to by [19]. He visited Buitencingle several times, including in 1914 ([18] and [17]). According to the sources, Gandhi had come to South Africa initially on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria ([6]). The accounts were kept in Gujarati, which was Gandhi's mother tongue.

### q38 — Who was Cissie Gool's father?

Dr. Abdulla Abdurahman [18].

### q39 — What was District Six like as a neighbourhood before the forced removals?

The provided sources do not contain detailed descriptions of what District Six was like as a neighborhood before the forced removals. There is only one brief mention describing District Six in 1947, stating that it was "a thriving cosmopolitan area" (source [17]). 

However, some information can be inferred from a few phrases scattered throughout the sources: District Six is described as having a rich history ("vibrant community") and a distinct identity. It also appears to have been a lively neighborhood with many residents moving around, as mentioned in source [8] when it was written that people would gatecrash house parties.

Sources [4], [5] of the book allude to the fact that it is a vibrant place but nothing more. There are specific anecdotes about certain incidents or experiences, however these are from individuals' perspectives - the general view for others in District Six remains unknown due to its absence within the sources.

### q40 — What was the Unity Movement's boycott policy?

The sources provide some information about the boycott policy, but not all details. [1] mentions that if "Coloured" voters insisted on voting during elections, they were asked to spoil their ballot by writing: “For Full Democratic Rights” across the ballot slip.

In [3], Jadwat and Seedat of the Communist Party are quoted as saying that it was better to have "our man" inside promoting "our point of view" than having the seat occupied by a government stooge. However, they do not specifically mention their stance on boycotting elections or spoiling ballots.

[5] says that the Unity Movement held the belief that non-collaboration and boycott policies would lead to change. [9] provides information about a successful boycott against the Coloured Affairs Council in 1943; people who accepted service on the C.A.C. were boycotted socially, economically, and otherwise.

According to [15], the boycott was effective when applied against some institutions or individuals but proved ineffective as a strategy against Train Apartheid Resistance.

In [19] it is mentioned that there was 'little evidence' of compliance with instructions by voters to spoil their votes in an election for the District 6 Ward.

