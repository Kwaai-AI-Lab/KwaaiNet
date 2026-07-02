# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 63.1% (142/225) |
| Avg latency | 28339ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | Who is the author? | 2/3 (67%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 48210ms |
| q02 | Who are the author's children? | 3/3 (100%) | LEST WE FORGET -rev25.pdf | 28337ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 23375ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 24098ms |
| q05 | Who was J.M.H. Gool? | 3/8 (38%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 32936ms |
| q06 | Tell me about Buitencingle. | 3/8 (38%) | [Graph: 7 Buitencingle Street], LEST WE FORGET -rev25.pdf | 29437ms |
| q07 | Who is the author's wife? | 2/3 (67%) | LEST WE FORGET -rev25.pdf, [Graph: Nazima Rassool] | 21409ms |
| q08 | Tell me more about the author's wife. | 4/6 (67%) | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 23442ms |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21457ms |
| q10 | Tell me about Kloof Nek. | 6/7 (86%) | LEST WE FORGET -rev25.pdf | 29128ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 4/6 (67%) | [Graph: Teachers League of South Africa], LEST WE FORGET -rev25.pdf | 24046ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: Bibi Gool] | 26598ms |
| q13 | What was the All Africa Convention? | 6/6 (100%) | [Graph: All African Convention], LEST WE FORGET -rev25.pdf | 23618ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 24311ms |
| q15 | What were the forced removals from District Six? | 3/6 (50%) | [Graph: District Six], LEST WE FORGET -rev25.pdf | 24441ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 3/7 (43%) | [Graph: Dr. Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 28248ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | LEST WE FORGET -rev25.pdf | 27787ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | [Graph: New Era Fellowship], LEST WE FORGET -rev25.pdf | 24053ms |
| q19 | What was the Non-European Unity Movement? | 5/6 (83%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 26312ms |
| q20 | Describe the author's involvement in cricket. | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 23758ms |
| q21 | Who was the author's mother? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 27235ms |
| q22 | Who was the author's father? | 4/4 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Yousuf Rassool] | 21096ms |
| q23 | Who were the author's siblings? | 2/5 (40%) | LEST WE FORGET -rev25.pdf | 25255ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 26654ms |
| q25 | Who was I.B. Tabata? | 1/5 (20%) | LEST WE FORGET -rev25.pdf, [Graph: Mr I.B. Tabata] | 21600ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 30207ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 5/5 (100%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 22966ms |
| q28 | Which organisations was the author involved in? | 5/5 (100%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 26842ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 6/6 (100%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 34151ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 0/6 (0%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 49428ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | [Graph: Hanaffi Quwatul Islam Mosque], LEST WE FORGET -rev25.pdf | 55295ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdul Hamid Gool] | 50381ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 2/5 (40%) | [Graph: Dr. Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 35848ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 4/6 (67%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 25298ms |
| q35 | Who was Hassen Mall? | 4/4 (100%) | LEST WE FORGET -rev25.pdf | 23058ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 1/6 (17%) | LEST WE FORGET -rev25.pdf | 26539ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 4/7 (57%) | LEST WE FORGET -rev25.pdf, [Graph: Teachers League of South Africa] | 20240ms |
| q38 | Who was Cissie Gool's father? | 2/5 (40%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 25745ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 3/6 (50%) | LEST WE FORGET -rev25.pdf, [Graph: District Six] | 25352ms |
| q40 | What was the Unity Movement's boycott policy? | 4/5 (80%) | [Graph: Non-European Unity Movement], LEST WE FORGET -rev25.pdf | 25399ms |

## Answers

### q01 — Who is the author?

The author is Y.S. Rassool (Joe Rassool) [1].

### q02 — Who are the author's children?

The author's children are Feyruz, Reza, and Zarina [4]. The author also has multiple grandchildren including Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [4].

### q03 — Who are the author's grandchildren?

The provided sources do not fully answer that question but lists a selection of them: Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis.

### q04 — To whom is the book dedicated?

The book is dedicated to the author's children: Feyruz, Reza, and Zarina; and his grandchildren: Joseph, Samuel, Alexander, Leonardo, Dylan, Lewis [4].

### q05 — Who was J.M.H. Gool?

The provided sources do not contain a comprehensive biography of J.M.H. Gool, but they provide various details about his life. He appears to have been an Indian merchant and leader in Cape Town, South Africa during the late 19th and early 20th centuries.

According to [4], J.M.H. Gool is listed among prominent individuals associated with "District Six" as a location in Cape Town and a term that encompasses related historical concepts.

In [6] and [14], his address is mentioned, indicating he had a residence in Buitencingle at some point and was referred to by his first name, Joseph. 

According to various sources ([4] again, and possibly others indirectly through the mention of this work), J.M.H. Gool was known for being an influential figure, who participated in British justice as described as "Oriental traders" or similar categories. [13] mentions that Adam Mahomed features in historical records as a financier and possible mentor to him.

In other sources ([4] again), it is mentioned that he was born in 1886 with this detail possibly being incomplete regarding birth timing further on for our purpose in the response.) 

In the book we are working from ([9]), which is directly about "LEST WE FORGET" by Joe Rassool (1984), there is a mention of his involvement with Gandhi. From [17], J.M.H. Gool’s involvement with various political activities is noted including serving on board members, in particular Wooding’s Preparatory private school, and other related references which provide more insight into J M H Gools background, social standing among Cape Town Residents.

### q06 — Tell me about Buitencingle.

According to the provided sources, Buitencingle was a 13-room mansion in District Six, Cape Town, South Africa, built by Haji Joosub Maulvi Hamid Gool. It was a grand and sprawling house with umpteen rooms, a garden with fruit trees, and was the family home of the Gool family.

In its heyday, Buitencingle was a center of social activity, hosting prominent visitors such as Gandhi and Dr. Abdullah Abdurahman (Source [1]). The house had a large staff to manage it, and my grandmother (Bibi) tried to learn Afrikaans to communicate with them and interpret their insults (Sources [3] and [13]).

The author of the memoir, Joe Rassool, spent time at Buitencingle and fondly recalls the experience. He remembers having more space than they could cope with, playing in the garden, climbing trees, and making up stories about a pear tree (Sources [6] and [7]).

Buitencingle was also a place where the family received distinguished guests, including Lord Headley, the 5th Baron, who left behind a navy-blue overcoat that provided warmth as a substitute blanket in later years (Source [12]).

However, the mood at Buitencingle changed with the family's fortunes. In later years, the house fell into disrepair, and my grandfather moved to 129 Castle Street above his Wholesale Store (Source [13]). Eventually, the family left Buitencingle altogether.

It is worth noting that the memories of Buitencingle were bittersweet for Joe Rassool, as he also struggled with the pressures of growing up in a strict household where issues of sexuality and morality were not openly discussed.

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1]. She is also an academic researcher who urged the author to ground his writing in historical background investigation [8], [9] and [11].

### q08 — Tell me more about the author's wife.

The provided sources do not contain extensive information about the author's wife. However, based on excerpts [1] and [18], it can be inferred that:

* The author's wife is an academic researcher ([1]).
* She urged the author to ground his writing in background investigation, which resulted in "staggering information" being produced ([1] and [18]).
* The author credits her with helping him transform the book from a purely subjective family recollection into a work revealing a rich historical layer previously unrecorded by any historian.

The author's wife is mentioned by name only once in excerpt [17], where she is not explicitly identified as his wife.

### q09 — Who was the author's grandfather?

The author's grandfather is identified as J.M.H. Gool [1,7].

### q10 — Tell me about Kloof Nek.

Kloof Nek is a natural valley or glen located in Cape Town, South Africa. According to the sources [4], it was originally part of the outermost belt (buitemandle) of Cape Town called Buitencingle.

The area connects Table Mountain to Lion's Head via a saddle of land and provides access to both mountains. As described by various sources, including excerpts [19] and [20], Kloof Nek features a beautiful scenery with scenic views, natural caves, and bushes which provided refuge for poor people ("Bergies") who would drink cheap wine and methylated spirits.

It is also noted that Kloof Nek was popular with nature lovers, mountaineers, and ramblers. The area featured the Table Mountain cable car at some point in time, giving visitors another option to ascend Table Mountain without hiking.

### q11 — What was the Teachers League of South Africa (TLSA)?

[1] The Teachers League of South Africa (TLSA) was a non-European teachers' professional body that took a strong political stance against apartheid education policies. [14] Yousuf Rassool was an active member and involved in its work. 

The TLSA had conferences which were important civic occasions, opened by the mayor with a civic banquet at the ratepayers' expense ([2], [19]). The League's motto was "Let Us Live for our Children" ([18]), and its members tried to prepare students against the impending onslaught of apartheid ([18]). 

The League collaborated with other organizations, such as the Cape African Teachers' Association, to develop strategies in preparation for a possible future in which the government would do to Coloureds and Indians what it was doing to Africans ([18]).

### q12 — Who was Cissie Gool?

Cissie Gool (Zainunnisa Abass) was a prominent figure in the struggle against apartheid. She was the wife of Dr Abdul Gaartie, but later married to Abdul Hamid Gool [16]. According to Source [18], she held influential positions and spoke at meetings in various locations throughout Cape Town, fighting for the rights of Coloured people under apartheid.

Sources indicate that Cissie Gool:

* Was known for her passionate oratory skills, particularly in calling for unity among the oppressed (Source [12]).
* Addressed a pre-election rally where she spoke about measures to deal with 'influx control' and street gangs (Source [9]).
* Supported various campaigns against apartheid policies, such as residential segregation (Source [17]).

Cissie Gool's life story is part of Joe Rassool's memoir "LEST WE FORGET", which documents the history of District Six in Cape Town during apartheid.

### q13 — What was the All Africa Convention?

According to source [1], the All African Convention (AAC) was a South African political body formed in 1935 and affiliated with the Non-European Unity Movement. It represented African political organisations opposed to the Hertzog Bills and continued to oppose apartheid alongside the NEUM.

Note: The author does not provide additional information about the convention other than this.

### q14 — Where was District Six and what kind of place was it?

District Six was a vibrant, multi-cultural inner-city neighborhood in Cape Town, South Africa [1]. It was home to the Rassool and Gool families and thousands of other residents before the apartheid government forcibly removed its population and demolished its homes under the Group Areas Act in the 1970s and 1980s.

In its heyday, District Six was a "vibrant, multi-cultural" place with a rich history of political struggle against the iniquities of the Apartheid state [12]. It had a unique character, with "note-worthy eccentrics" such as Abdul's tap-dancing friends who would earn pennies on the sidewalks [9]. District Six was a place where people from different backgrounds lived together and interacted, but it was ultimately bulldozed away to make central Cape Town for Whites only.

### q15 — What were the forced removals from District Six?

The provided sources do not contain specific information on the details, dates, or extent of the forced removals from District Six. However, they mention that the area was a vibrant, multi-cultural inner-city neighborhood of Cape Town, and that it was home to the Rassool and Gool families, among others.

Source [1] mentions that the apartheid government forcibly removed its population under the Group Areas Act in the 1970s and 1980s, implying that this relocation impacted the area significantly.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to the sources [3], [9], and [19]-[20], Gandhi (also known as Mahatma Gandhi) visited Cape Town in 1912 and again in February 1914, during which time he stayed at the residence of Dr. Abdul Hamid Gool (Uncle Adam) at Noor Bagh, 8 Kloof Street. Dr. Gool read an address of welcome to visiting dignitaries on behalf of Gandhi.

In [9], it is stated that Dr. Gool repaid Gandhi with attentive medical care and wrote articles for Indian Opinion and other scholarly journals. Gandhi also stayed at the home of Morris and Ruth Alexander during one visit.

Sources [3] and [19]-[20] indicate a close connection between Gandhi and the Gool family, particularly Dr. Abdul Hamid Gool. Gandhi would often stay with them during his visits to Cape Town, and they provided him with support, including arranging meals for visiting dignitaries.

In [12], it is mentioned that Gandhi referred to himself as "Mahatma" (Great Spirit), which was first used in an Indian Opinion article printed by the British Labour Party MP.

### q17 — What was Hewat Training College?

Hewat Training College was a training facility for teachers of Coloureds in South Africa. According to [5], it trained teachers for the upper years of the primary division. The college opened in 1944 with its first intake of students, as mentioned in [1]. 

The author entered the grounds of Hewat Training College in January 1947 ([2]-[3]). The college's staff was recruited from the United Kingdom, chosen partly because of their conservative educational philosophy and missionary outlook (see [3]).

Hewat Training College is also referenced in [16] as an organisation that had Departmental recognition withdrawn, leading to a decline in membership. 

In [18], it is mentioned that the author's mother suggested he do a teacher-training course at Hewat Training College after he had other plans which were rejected by her.

Finally, [20] describes the college's location: "The Hewat Teacher training College stood opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds."

### q18 — What was the New Era Fellowship?

The New Era Fellowship (NEF) was a Cape Town educational and cultural organisation that discussed various social and political issues, according to [1]. Its activities included public lectures and discussions on national and international events. As mentioned in [5], it also organized events like chairing meetings at the Stakesby-Lewis Hostel.

The New Era Fellowship can be seen as part of a broader movement against apartheid, working closely with various groups including the Non-European Unity Movement (NEUM) and the Teachers' League of South Africa. In particular, members such as Hassan Bavasa took on leadership roles within NEF.

Throughout excerpts [2]–[20], it is evident that the New Era Fellowship was a significant organization within the anti-apartheid movement in Cape Town during the mid-20th century.

### q19 — What was the Non-European Unity Movement?

The Non-European Unity Movement (NEUM) was a South African political movement founded in 1943, which united non-European political organisations opposed to apartheid and racial discrimination. It advocated non-collaboration with apartheid institutions (Source [1] and [14]).

During its existence, the NEUM played a significant role in the struggle for liberation, bringing the principle of non-Collaboration, the refutation of “race”, and the ideal of a non-racial society to the struggle for liberation. However, the political struggle did not go according to their hopes, and they failed to achieve some of their ideals (Source [11]).

The NEUM was active in the Transkeian Territories and led a conference where peasants addressed the Unity Movement, seeking help from the educated people in Cape Town (Source [12]). They also revolutionised the political scene with their doctrine of non-collaboration and the boycott weapon (Source [14]).

The concept of non-European unity arose in 1943 when the government foisted The Coloured Affairs on the so-called Coloured people, yet nobody breathed a word about similar attempts to create a non-European unity movement led by Dr. Jabavu earlier (Sources [18] and [19]).

### q20 — Describe the author's involvement in cricket.

The author was involved in cricket as a player and a participant in the Western Province Indian Cricket Union (WPICU). 

The author participated in several teams, including the Kismets, which introduced non-racial policies to the Indian Cricket Union, and played against other clubs in their Sunday league. The author is described as learning to play cricket seriously under the guidance of Hassen Mall and mastered the elements of batting.

[7, 8, 14]

Although the author did not make it into the final team to tour Johannesburg, they were among the fifteen selected to represent the Western Province for a different match, which included Salie Van Haacht as captain. The author watched in admiration under the trees of the Liesbeek River as Van Haacht played against Natal's famous fast bowler.

[10]

The author held various positions within the WPICU and was part of several committees focused on cricket and community development.

[14]

### q21 — Who was the author's mother?

The provided sources do not contain explicit information about the author's mother's name. However, we can infer some details from excerpt [13] and [14]:

[13] mentions Bibi (my mother) and that she gave birth to a baby named Ayesha on December 10, 1900.

Although it does not reveal the name of Bibi's husband, another mention in excerpt [6] might give us a hint: "My father was handsome, bronzed like his mother..." which implies the author's mother was beautiful and had bronzed skin, similar to her father. However, this is only a hint.

In excerpt [12], Magdalena Isabella (née?) is described as a notable figure in the local Church and is mentioned as having a relationship with Malick Rassool who later became the author's uncle via marriage, making her presumably a mother-in-law of some sort to Baboo Gool. 

So we can be fairly confident that Magdalena Isabella (most definitely Bibi’s mother in law) would have been the wife of Baboo A.C. Gool.

However, based on excerpt [13], it appears that Bibi was not married when she had Ayesha, implying that her parents may have disapproved or there were other factors at play which led to her remaining single after giving birth.

Therefore the answer remains a complex family web with Bibi being Baboo's partner and eventually Gool’s grandmother inlaw

### q22 — Who was the author's father?

The author's father was Peter Alexander Rassool, who was also known as Peerbhai or Peru [3], [10].

### q23 — Who were the author's siblings?

According to [20], one of the author's siblings is Fazil. It does not mention how many other children the author had before this sibling.

From various excerpts where it mentions the number of people in a room or family: 

- In [12] we are told the editorial board, so presumably at least four (the author does not say that we are only talking about their siblings, but at the very minimum we know two adults are part of a group and two more were mentioned). However, no siblings were specifically named here.
- 
In [6], the author talks to "my brother" Kassiem after playing together as children with Morris.

### q24 — Who were the children of J.M.H. Gool?

The source excerpt does not provide a comprehensive list of all the children of J.M.H. Gool, but it mentions six of them by name:

1. Kassim (born in August 1900) [17]
2. Nasima [6] (her father was born in Mauritius in September)
3. Zainunissa ("Zaironesa" as the Birth registrar recorded it) [4]
4. Rasheda [4]
5. Berina [4]

Additionally, Yasmin is mentioned as a sister of these children, stating that she and their mother were pregnant when this part was described in the text [4].

### q25 — Who was I.B. Tabata?

I.B. Tabata is described in [2] as an author and affiliated with the New Era Fellowship (NEF), who published The Awakening, which was accused by other Marxist Trotskyists of failing to have a class analysis. He was later portrayed [16] and [18] as being involved in the NEF publication and founding figure of the All African Convention.

### q26 — Who was Dr. Abdullah Abdurahman?

According to source [1], Dr. Abdullah Abdurahman was a medical doctor and one of the most prominent political figures in the Cape Coloured community in the early twentieth century. He was a long-serving Cape Town city councillor, and father of Cissie Gool (Zainunnissa Gool), who was also an activist and city councillor.

Sources [1] and [7] confirm that Dr. Abdurahman was the first person from his community to qualify as a doctor from Glasgow University in 1893, and he met and married Helen "Nellie" Potter James there. His grandfather was brought to South Africa as a slave and he played an active role in the Cape Coloured community.

Sources [4], [13], and [15] mention that Dr. Abdurahman was connected to J.M.H. Gool (most likely the author's grandfather) and was linked through the Dollie family, with his mother being a member of the Dollie clan.

Source [3] states that he and Dr. n (presumably n is an abbreviation for another first name or the author's grandfather) were present at a meeting in 1904 where they heard the Progressive Party candidates' policies, but does not provide more information.

Sources [11], [18], and [20] confirm his involvement with prominent people of that time, such as Gandhi and Barney Barnato, and mentions his association with Buitencingle.

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The provided sources do not contain explicit information about the nature of the connection between Gandhi and J.M.H. Gool. However, it can be inferred that they were related to each other through the Gool family.

[9] mentions that Gandhi stayed at the Gool mansion at 7 Buitencingle Street, suggesting a close relationship or hospitality provided by the Gools.

[11] quotes an Indian Opinion article from March 1914 where Gandhi thanked the Gools and their services during his stay in Cape Town.

Other sources suggest that J.M.H. Gool was part of the community that Gandhi interfaced with, and that they might have been allies or associates in various social and political activities.

The connection between the two is not explicitly stated in the provided sources.

### q28 — Which organisations was the author involved in?

The author, Joe Rassool [1], was involved in several organisations. At least the following are mentioned:

* The New Era Fellowship (NEF) [4][13][17]: A cultural organisation established in 1937 to discuss various issues.
* The Anti-CAD Club: The author helped it affiliate with the Movement, and his friend Victor Wessels even sponsored the motion to affiliate [5].
* The Teachers League of South Africa (TLSA): As an educator and community activist, the author was deeply involved in this organisation, as mentioned in [1] and [14].
* The Non-European Unity Movement (NEUM): Although not explicitly stated, it is implied in [1] that the author participated in the NEUM.
* The All African Convention: In 1954, the author, along with Vic Wessels and Les Jacobs, went to represent the New Era Fellowship at this convention, held in Bloemfontein [14].
* The Liberation League: This organisation was involved in the struggle for human rights, similar to the author's other mentioned affiliations.

These affiliations suggest that the author, Joe Rassool, was deeply involved in various political and cultural organisations aimed at promoting social justice and fighting for human rights during the apartheid era in South Africa.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

According to [12] and [10], the Non-European Unity Movement (NEUM) was a movement that was drawn into by the author when they began their teaching career. The NEUM revolutionized the political scene with its doctrine of non-collaboration and the boycott weapon, galvanizing the non-white disenfranchised to realize the need for a programme-based struggle [12].

Additionally, according to [17], at some point after joining the TLSA (Teachers' League of South Africa), which was another organization involved in promoting African rights, it "broke with its previous stance of non-political professionalism, and joined the Non-European Unity Movement".

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

The historical record is sparse for this period of J.M.H. [2, 3]

However, we know that he arrived in Cape Town in December, which comes from an interview my Aunt Jane gave to Sharon Parker [4]. 

Also, it is mentioned that he departed from Bombay in April or early May 1901 leaving Bibi in the third or fourth month with her son [2]. Therefore, we can deduce that he must have arrived at least nine months prior to December, which would be around March 1901, but a more specific date is not mentioned.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

The Hanaffi Quwatul Islam Mosque was in Loop Street, Cape Town. It was founded by Haji Joosub Maulvi Hamid Gool and completed in 1898 (Source [1]).

### q32 — How was Cissie Gool related to J.M.H. Gool?

The provided sources do not specify how Cissie Gool was specifically related to J.M.H. Gool, but they do mention that she was Dr. Abdurahman's wife (see [15]). In another part of the text ([16]), it is stated that Cissie Gool and her brother Goolam Gool held an opposing view on how the Indians should approach their cause compared to Dr. A. Gool, which implies a familial relationship between them and Dr. A. Gool. Additionally, in [17], it is mentioned that Wahida Ta'Al was one of J.M.H. Gool's wives, while Cissie Gool (married to Abdurahman) was related to the Abdurahman dynasty ("scion" implies a son or daughter). This would make her and Dr. A. Gool in-laws through their respective spouses' relationships to Wahida.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

The provided sources do not contain a complete list of all the historical figures that J.M.H. Gool knew personally. However, they do mention the following:

1. Mahatma Gandhi [12]: Gool is mentioned as submitting a subscription to the Indian Opinion on behalf of Mr. Wilson.
2. Joseph Chamberlain [13]: Not specifically mentioned as a personal acquaintance, but he is included in a list of notable historical figures and events.
3. Abdurahman: A prominent figure in the community who was closely related to Gool through marriage and geography [17].
4. Adam H.G. Mahomed [18-19]: Features prominently in the historical record as Gool's senior, financier, and possibly mentor.

Other notable historical figures mentioned in passing include:

1. Mohandas Karamchand Gandhi (mentioned alongside J.M.H. Gool)
2. Louis Botha
3. General Smuts

### q34 — What was the Group Areas Act and how did it affect District Six?

The Group Areas Act [4, 3, 7] was a piece of legislation introduced in South Africa by the apartheid government to enforce racial segregation by allocating certain areas for specific racial groups. The act aimed to transform the country into a patchwork of Black, White, Indian, and Coloured areas.

District Six [1], a vibrant and multi-ethnic neighborhood in Cape Town, was severely affected by this legislation [8]. As a result of the Group Areas Act, the population of District Six was forcibly removed and relocated to other areas on the outskirts of the city. The area was then demolished, leaving many families without homes or belongings [4].

The residents of District Six had been a diverse group since Victorian times, including communities such as freed slaves from America and the West Indies, who had thrived in the area before being expelled at the turn of the century [13].

### q35 — Who was Hassen Mall?

The provided sources do not contain detailed information on who Hassen Mall is beyond describing him as a student from Durban doing medicine at Cape Town [18] and that he was staying with some relatives named Muddy [19]. However, the documents mention his involvement in several activities such as cricket, being the team captain of an unspecified tournament or match, and his interactions with the author. Given the context provided in the other excerpts, it can be inferred that Hassen Mall played a significant role in the author's life as a friend and possibly a mentor during their university days at Cape Town.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources list the following political organisations that were active in the Cape Coloured community during the author's lifetime:

1. The New Era Fellowship (existed since 1937) [1]
2. The Teachers' League of South Africa [4] and [12]
3. The Anti-CAD (Anti-Coloured Advisory Council) [12]

Additionally, some sources mention organisations that were relevant to or had an impact on the Cape Coloured community:

1. The Communist Party of South Africa [3]
2. The Fourth International of South Africa [5]
3. The Liberation League [5]
4. The Workers' Party [5]
5. Smuts's United Party (the Coloured branch) [5]

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

The provided sources do not contain a comprehensive biography of Mahatma Gandhi, but [16] clarifies that Gandhi had come to South Africa in 1893 on a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria because the accounts were kept in Gujarati, which was Gandhi's mother tongue.

### q38 — Who was Cissie Gool's father?

The provided sources do not specifically mention who Cissie Gool's father was. They do mention Hajima Gool arriving in Cape Town with two children and their mother, Bibi Gool, revealing that Bibi Gool is likely to be related to the Gool family, but they do not identify her as Cissie Gool's mother or clarify her relationship to Cissie Gool.

### q39 — What was District Six like as a neighbourhood before the forced removals?

Based on excerpts [1] and [17] to [20], here is what the sources say:

Before the forced removals, District Six was a vibrant, multi-cultural inner-city neighborhood of Cape Town, South Africa. [17] It had a rich history and a cosmopolitan population. [16] The neighborhood was home to various ethnic groups, including Indians, Malays, and Coloreds. [19]

District Six was known for its thriving community and pulsing life. [17] According to the author, it was "an unforgettable education and privilege" to be part of that life. [18] The neighborhood had many streets with people living in close proximity to each other, like Caledon Street, which ran from Corporation Street to Clifton Street. [18]

As a young boy, the author, who lived in District Six, remembered the neighborhood being filled with joy and life. [17] He described Caledon Street as getting "tattier" along the way but still had a lot of life. [18]

However, it is worth noting that the forced removals under the Group Areas Act (1950) were imminent at the time the author left District Six. [19][20]

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in several excerpts.

[2] states that the Unity Movement revolutionised the political scene with its doctrine of non-collaboration and the boycott weapon. [6] adds that the boycott now became the weapon of choice for the movement, perfectly fitting the situation in Cape Town in 1943.

According to [5], when Dr. Tabata was a member of the Communist Party, he agreed to put up candidates, which went against the Movement's desire to have people boycott the elections. The Party responded by using "some convoluted thinking" instead of simply accepting the Movement's stance on non-collaboration.

[10] mentions that barbers refused to cut their hair, tailors refused to sew their suits, and shopkeepers refused to serve those who accepted service on the CAC (Coloured Advisory Council). This further reinforces the point that the boycott was a policy adopted by various sectors of society in line with the Unity Movement's stance.

[14] states that during a certain occasion, the regime cullled cattle and other livestock, but did not provide more land to satisfy the demands of the peasantry. It then mentions the "simple" peasants trusting educated people in Cape Town would help them.

