# RAG Eval Report

**KB:** `D6`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 40 |
| Overall recall (token-overlap) | 56.4% (127/225) |
 | Avg judge score | 1.65/2.00 (40 questions scored) |
| Avg latency | 59220ms |

## Per-question results

| ID | Question | Hit rate | Judge | Sources | Latency |
|----|----------|----------|-------|---------|--------|
| q01 | Who is the author? | 3/3 (100%) | 1/2 | LEST WE FORGET -rev25.pdf | 80695ms |
| q02 | Who are the author's children? | 0/3 (0%) | 2/2 | LEST WE FORGET -rev25.pdf | 11698ms |
| q03 | Who are the author's grandchildren? | 6/6 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 22645ms |
| q04 | To whom is the book dedicated? | 4/4 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 41793ms |
| q05 | Who was J.M.H. Gool? | 2/8 (25%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Goolam Gool] | 70307ms |
| q06 | Tell me about Buitencingle. | 3/8 (38%) | 2/2 | [Graph: Buitencingle Grandpa], LEST WE FORGET -rev25.pdf | 55946ms |
| q07 | Who is the author's wife? | 2/3 (67%) | 2/2 | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 12153ms |
| q08 | Tell me more about the author's wife. | 6/6 (100%) | 2/2 | [Graph: Nazima Rassool], LEST WE FORGET -rev25.pdf | 35724ms |
| q09 | Who was the author's grandfather? | 3/9 (33%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 64912ms |
| q10 | Tell me about Kloof Nek. | 5/7 (71%) | 2/2 | LEST WE FORGET -rev25.pdf | 75301ms |
| q11 | What was the Teachers League of South Africa (TLSA)? | 3/6 (50%) | 2/2 | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 25632ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | 2/2 | [Graph: Cissie], LEST WE FORGET -rev25.pdf | 78788ms |
| q13 | What was the All Africa Convention? | 4/6 (67%) | 1/2 | [Graph: James Africa], LEST WE FORGET -rev25.pdf | 39442ms |
| q14 | Where was District Six and what kind of place was it? | 3/6 (50%) | 2/2 | [Graph: Ben District Six], LEST WE FORGET -rev25.pdf | 88309ms |
| q15 | What were the forced removals from District Six? | 2/6 (33%) | 1/2 | [Graph: Ben District Six], LEST WE FORGET -rev25.pdf | 51869ms |
| q16 | Who was Gandhi and what was his connection to the Gool family? | 4/7 (57%) | 2/2 | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 110631ms |
| q17 | What was Hewat Training College? | 5/5 (100%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Hewat] | 27634ms |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | 1/2 | LEST WE FORGET -rev25.pdf | 57976ms |
| q19 | What was the Non-European Unity Movement? | 3/6 (50%) | 2/2 | LEST WE FORGET -rev25.pdf | 87415ms |
| q20 | Describe the author's involvement in cricket. | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf | 75114ms |
| q21 | Who was the author's mother? | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf | 69086ms |
| q22 | Who was the author's father? | 4/4 (100%) | 2/2 | [Graph: Peter Alexander Rassool], LEST WE FORGET -rev25.pdf | 81148ms |
| q23 | Who were the author's siblings? | 1/5 (20%) | 1/2 | LEST WE FORGET -rev25.pdf | 26419ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | 1/2 | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 85123ms |
| q25 | Who was I.B. Tabata? | 2/5 (40%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Goolam Gool] | 86417ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 3/6 (50%) | 1/2 | [Graph: Goolam Gool], LEST WE FORGET -rev25.pdf | 90372ms |
| q27 | What was the connection between Gandhi and J.M.H. Gool? | 2/5 (40%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 82353ms |
| q28 | Which organisations was the author involved in? | 0/5 (0%) | 1/2 | LEST WE FORGET -rev25.pdf | 48682ms |
| q29 | What was the relationship between the TLSA and the Non-European Unity Movement? | 3/6 (50%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 83703ms |
| q30 | When did J.M.H. Gool arrive in Cape Town and from where? | 3/6 (50%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Haji Joosub Maulvi Hamid Gool] | 28019ms |
| q31 | What was the Hanaffi Quwatul Islam Mosque? | 6/6 (100%) | 2/2 | LEST WE FORGET -rev25.pdf | 35025ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 5/5 (100%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 72949ms |
| q33 | Who were the notable historical figures that J.M.H. Gool knew personally? | 3/5 (60%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Abdul Hamid Gool] | 59129ms |
| q34 | What was the Group Areas Act and how did it affect District Six? | 5/6 (83%) | 2/2 | [Graph: Baruch Hirson], LEST WE FORGET -rev25.pdf | 87160ms |
| q35 | Who was Hassen Mall? | 3/4 (75%) | 2/2 | [Graph: Hassen Mall], LEST WE FORGET -rev25.pdf | 25339ms |
| q36 | What political organisations were active in the Cape Coloured community during the author's lifetime? | 2/6 (33%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Mohandas Karamchand Gandhi] | 61758ms |
| q37 | Who was Mahatma Gandhi and why was he in South Africa? | 5/7 (71%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Mahatma Gandhi] | 46558ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | 1/2 | LEST WE FORGET -rev25.pdf, [Graph: Peter Alexander Rassool] | 37711ms |
| q39 | What was District Six like as a neighbourhood before the forced removals? | 1/6 (17%) | 2/2 | [Graph: Ben District Six], LEST WE FORGET -rev25.pdf | 85873ms |
| q40 | What was the Unity Movement's boycott policy? | 3/5 (60%) | 2/2 | LEST WE FORGET -rev25.pdf, [Graph: Captain Boycott] | 62024ms |

## Answers

### q01 — Who is the author?

The provided sources do not explicitly state who the author of "Lest We Forget" or all its parts is. However, based on the context, it appears that Yousuf (Joe) Rassool [1] may be one of the contributors to the book, and possibly a key figure in compiling and writing certain chapters. 

However, I found a passage indicating ES Reddy was an editor who contributed notes to the book: “xx E.S. Reddy note to editor:”.

### q02 — Who are the author's children?

The provided sources do not contain that information.

### q03 — Who are the author's grandchildren?

The source does not specify all the names of the book's author's grandchildren, but mentions some:

1) Joseph [8] 
2) Samuel [8] 
3) Alexander [8] 
4) Leonardo [8] 
5) Dylan [8] and 
6) Lewis [8].

### q04 — To whom is the book dedicated?

The book "District Six - Lest We Forget" by Yousuf (Joe) Rassool is dedicated to his children: Feyruz, Reza, and Zarina, as well as his grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan, and Lewis [20].

Additionally, the dedication seems to be part of a larger publication, and there are other dedications present in earlier editions (e.g. "For my children: Feyruz, Reza, and Zarina / and grandchildren, Joseph, Samuel, Alexander, Leonardo, Dylan and Lewis."  appears as one, while "to my family" and "My loved ones" appear in the front matter of e-book versions).

### q05 — Who was J.M.H. Gool?

J.M.H. Gool, also referred to as Dr. Gool or Grandpa by his grandson Yousuf (Joe) Rassool, was a prominent citizen of Cape Town in the early 20th century. This is based on information from [3] and other sources.

According to [3], he was "highly regarded" and owned almost one hundred properties at the peak of his power, including some in prestigious streets like Adderley Street, Wale Street, and Long Street, as well as many in the Ma area. He was also a member of several organizations, such as the Wooding's Preparatory School board, which was staffed by West Indian teachers.

Additionally, [6] mentions that Adam H. G. Mahomed, also referred to as Adam Gool, may have been a senior, financier, and possibly mentor to J.M.H. Gool is not confirmed but appears likely given the shared affiliation with this organization.

However, the provided sources do not contain much information about J.M.H. Gool's life before he became prominent in Cape Town society or his later years.

The term "Dr." may imply that J.M.H. Gool held a medical degree although there is no confirmation of this from the given excerpts

### q06 — Tell me about Buitencingle.

Based on the provided sources, here's what they say about Buitencingle:

* [2] mentions that The Gool mansion in District Six is no more, but we should not forget the history made at 7 Buitencingle by the Gools and their friends.
* [9] (Yusuf Rassool's story) mentions his grandfather Haji Joosub Maulvi Hamid Gooli living in a thirteen-roomed mansion in Buitencingle Street, but it does not provide comprehensive information about Buitencingle.
* No other source ([1], [3]-[8], [10]-[15], [17]-[20]) mentions Buitencingle in the text provided.

It can be concluded that:

Buitencingle: The Gool family once had a mansion at 7 Buitencingle, but it is gone and only remembered for its historical significance. 

Further information about Buitencingle is not available based on these sources.

### q07 — Who is the author's wife?

The author's wife is Nazima Rassool [1].

### q08 — Tell me more about the author's wife.

Information about the author's wife, Nazima Rassool, can be found in [1]. It states that Professor Nazima Rassool is the author's wife and an academic researcher who urged him to ground his memoir in historical research. Her insistence on scholarly rigor transformed the book into a work revealing a rich historical layer previously unrecorded by any historian.

Note: Other mentions of E.S. Reddy provide additional context about Gandhi's interactions with the Gool family, but do not directly address the author's wife, Nazima Rassool.

### q09 — Who was the author's grandfather?

The author did not directly state their own grandfathers' names, but rather referred to him as "Grandpa" or "JMH Gool". However, he was a South African Indian and a highly regarded citizen of Cape Town. He owned nearly one hundred properties, was involved in the struggle for the rights of Indians in South Africa at the turn of the century, and wrote a letter in support of Mahatma Gandhi [9]. His granddaughter (the author's mother) had two daughters, both of whom qualified as Bachelors of Arts at Fort Hare. He also raised his step-mother-in-law, Wahida, who was Gadija Dollie's family member, and allowed her to live with him and help her take care of the household [8].

He married Bibi and then Wahida after marrying Bibi in 1879 or 1880 in Broach, Gujarat, India. Later, he sent one of his sons to visit Aligarh College in India for formal education [18], however, details about him were gathered through snippets of chit-chat collected from the author’s childhood memories with their family members [2].

### q10 — Tell me about Kloof Nek.

According to the sources [1], [5], and [15]–[17], Kloof Nek is a place of natural beauty, located between Table Mountain and Lion's Head in Cape Town. It is described as a "sylvan glen" and a "place of haunting beauty". The Glen at Kloof Nek was a popular spot for lovers, mountaineers, nature lovers, and ramblers to visit.

Source [15] mentions that it was also a place where people would meet secretly, citing the example of Persotem and Khadija meeting there on Saturdays. Source [17] also mentions the 1912 visit of Mahatma Gandhi to Adam Gool's property, "Noor Bagh", which is located near Kloof Nek.

Source [6] provides more details about the geography and location of Kloof Nek. It describes it as a connecting saddle of land between Table Mountain and Lion's Head, with Signal Hill nearby. The Glen at Kloof Nek was also mentioned in source [16].

However, there is no information available on exactly what "Kloof Nek" means or where the name originates from.

Source [1] implies that it is a popular spot for people to study and socialize, as Persotem and the narrator would meet friends there regularly.

### q11 — What was the Teachers League of South Africa (TLSA)?

The sources do not provide a detailed description of the Teachers' League of South Africa (TLSA). However, they suggest that it was an organization for teachers [9], which had various members including Alie Fataar was one such member [13]. The TLSA was involved in advocating for education rights and opposing apartheid policies.

### q12 — Who was Cissie Gool?

The sources do not provide a full biography of Cissie Gool. However, based on the information provided, Cissie Gool appears to be a woman who was:

* Part of a prominent social and political clan (extract [1])
* Married to Dr. Abdul Hamid Ishak (or "Abdul-Hamid", [2], [5])
* The daughter of Dr. Abdulla Abdurahman ([4], [5])
* An active participant in politics, including addressing meetings and rallies (extracts [3], [11], [16], [18] and [20] all mention her as a speaker)
* A member of the Anti-CAD Movement (but also appeared to support measures for "influx control" and dealing with street gangs, which was seen as a betrayal by some ([13], [17]))
* A leader figure in the community, being referred to as "Auntie Cissie" ([11])
* Possibly involved with members of the banned Communist party ([3])
* Had family connections to other prominent individuals, including her husband Abdul Hamid and her aunt Jane Gool (extract [14])

Overall, it appears that Cissie Gool was a significant figure in Cape Town politics during the mid-20th century, and was known for her oratory skills and activism. However, there are gaps in the information provided and it is not possible to provide a more comprehensive biography based on these sources alone.

### q13 — What was the All Africa Convention?

The sources do not provide a comprehensive definition of what the All Africa Convention (AAC) was. However, we can gather some information from the excerpts:

* In Chapter Nineteen [15], it is mentioned that Vic Wessels, Les Jacobs, and the narrator attended the conference of the All African Convention in Bloemfontein in 1954.
* According to Chapter Seventeen [16] and Chapter Eighteen [19], the AAC was an organization that played a significant role in the Non-European Unity Movement.

There is no further information about the purpose, goals, or activities of the All Africa Convention.

### q14 — Where was District Six and what kind of place was it?

District Six was a residential area in Cape Town, South Africa. According to Yousuf (Joe) Rassool's account, District Six was once a vibrant and bustling community with its own shops, markets, and street life.

Excerpt [10] describes it as: "Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along the way. ... it was a place that catered largely for Xhosa speaking Africans!" However, this statement might be misleading - excerpt [13] mentions Hanover Park and other townships were erected to entice people away from District Six.

Excerpt [10] actually describes Caledon Street in 1945: "Hanover Street was the main street of the district, with little plateaux where other streets intersected... The Star Cinema or bioscope as it was called occupied the one corner of the junction, and opposite it was the fish market with its stalls, marble slabs, wet fish, fishy smells and fish talk. The street corners were always lined with the hand-drawn carts of the fruit and vegetable hawkers dexterously slapping open their paper bags and bawling out to the passers-by “Lovely tomatoes, Merrem- only penny a pyown’.”

Based on these descriptions, District Six was an area with a mix of cultures, including Xhosa speakers. It was known for its crowded streets, markets, shops, and community life.

Yousuf (Joe) Rassool's book is a nostalgic account of his childhood in District Six, and he portrays it as a place where people lived, worked, played, and socialized together.

### q15 — What were the forced removals from District Six?

The provided sources do not contain direct information about the forced removals from District Six. However, it is widely known that the residents of District Six in Cape Town, South Africa, were forcibly removed by the apartheid government in the 1960s and early 1970s as part of a large-scale urban renewal project aimed at relocating "non-white" communities away from white areas.

The sources do mention the changes happening to District Six, such as the auctioning off of homes (Excerpt [10]) and the new residents moving in ([18] mentions being taken to Queens Road). However, these mentions are not explicitly connected to forced removals but rather part of a broader period of transformation in the area.

To gain more information  on this issue it's necessary to look into additional sources beyond those provided.

### q16 — Who was Gandhi and what was his connection to the Gool family?

According to Excerpt [3], Gandhi's closeness to the Gool family is evident from the correspondence between him and my uncle Abdul Hamid Gool whilst he was a medical student at Guy’s Hospital in London.

Excerpt [4] mentions that Es. S. Reddy contributed correspondence of Gandhi with JMH Gool for the book.

Excerpt [6] discusses Gandhi's visit to Cape Town, where he stayed with the Gool family, particularly 7 Buitencingle, and notes: "That was a crucial time in the satyagraha led by Gandhi. Elizabeth Molteno and other European women visited Gandhi at the Gool residence."

Excerpt [7] contains the full text of a letter from Gandhi to one of his sons mentioning the difficulties involved in marrying outside his religion.

Gandhi is known as "Mahatma" due to his exceptional qualities, established by Excerpt [11].

Other evidence connecting Gandhi with J. M. H. Gool and the Gool family can be found in various other excerpts (Excerpts 1, 5–8).

Overall, it appears that Gandhi had a close relationship with the Gool family, visiting them at their residence at 7 Buitencingle, and engaging with correspondence from Uncle Abdul Hamid Gool.

### q17 — What was Hewat Training College?

According to excerpt [18], Hewat Training College was a school where students were trained to become teachers for Coloured pupils. It was located opposite the Roeland Street Gaol in fairly spacious but undeveloped grounds. The college had opened for the first intake of students in 1944 with considerable fanfare, and it specifically offered Post Matriculation Teacher Training (for Coloureds).

### q18 — What was the New Era Fellowship?

The provided sources do not contain comprehensive information about the New Era Fellowship's organization or its primary goals. However, they suggest the following:

* They were a type of political cultural organization associated with the Unity Movement (excerpts [2][6][7]).

* They had a Chairman who presided over meetings and had a say in publications (excerpts [4][17]).
 
The details about their activities or purpose are absent from these specific excerpts, but general information suggests that they played a role in the Anti-CAD movement.  

Sources:
[2] "Hassan Bavasah, my friend, and organising secretary of the NEF, saw to the programme of lectures of the New Era Fellowship."
[6] "[Joe Rassool's] uncle Goolam had also joined the New Era Fellowship..."
[7] The Cape Peninsula Students Union was viewed with gravest misgiving by the New Era Fellowship.

The provided sources, however, lack comprehensive details regarding their structure or primary goals.

### q19 — What was the Non-European Unity Movement?

The provided sources do not contain a detailed description of the Non-European Unity Movement. However, we can gather some information from the excerpts.

From [5], it is mentioned that an idea of non-European unity arose in 1943 when the government foisted "The Coloured Affairs" on the so-called Coloured people. It suggests that the movement was a response to this attempt by the government to segregate and marginalize non-European populations.

[7] states that there were attempts before 1943 to create a non-European unity movement, in which Dr. Abdurahman and D.D.T. Jabavu played key roles. However, these attempts failed due to unfavorable historical circumstances.

From [17], we know that the Non-European Unity Movement was associated with several events, including a petition against segregation presented by Indian Opinion (June 18, 1903), which mentions Dr. Abdurahman as its leader.

[7] Also states "The Freedom Charter appeared to be in many respects an imitation of our 10 Point Programme." suggesting that the Non-European Unity Movement had some sort of political program or platform.

Overall, we can infer that the Non-European Unity Movement was a movement aimed at promoting unity and equality among non-European populations in South Africa during the early 20th century. It appears to have been active from around 1943 onwards and may have involved efforts to challenge segregation and discrimination.

It's worth noting that further research would be necessary to obtain a more comprehensive understanding of the Non-European Unity Movement, including its goals, structure, achievements, and impact.

### q20 — Describe the author's involvement in cricket.

The provided sources do not contain comprehensive information about the author's entire career or personal life, regarding their role in cricket. However, they offer glimpses into certain aspects of this interest.

• [1] The author narrates their experience playing cricket for Kismets Cricket Club and participating in a match against another team (name not specified). In that game, he made 12 runs and was hailed as a hero for his innings score (source [1]). He felt elated after scoring over ninety runs against the Barnato Board at Mowbray (source [8]).

• [2] and [3]: The author discusses their initial attempts to master cricket through playing, learning forward defensive strokes, with support from their club. 

• [5], [7], and [9] - [16]: These sources mention the West Indian tour of South Africa and its potential for improving local players' skills but raise ethical concerns about supporting Apartheid cricket.

• Various instances: He highlights friendships formed through his interest in cricket; with Achmat Hayat (Achee) at Habibia and Mylie Begg, both fellow cricketers mentioned.

Some interesting cricket-related anecdotes include:

- Being selected for a match but not making the final team for the tour to Johannesburg.
- Participating in a raffle to raise funds for the equipment needed by his club.
- Joining a non-racial Sunday league and playing within that setting.

### q21 — Who was the author's mother?

The provided sources do not contain information about the author's biological mother, but rather focus on his grandfather and grandmother, particularly M. S. Gool (the author's grandfather) and Bibi/ Pee-rah-beegum (his wife), as well as Wahida "Bhokkie" Ta'al and Haji Wahida (later known as Hajima after her pilgrimage to Mecca) who were his grandfather's subsequent wives.

However, from [6] we can gather that the author was close to George and Joyce Meisenheimer and the author himself mentions developing a deep regard for Muddy’s mother as a person. 

It is not until the end of excerpt [18] where it might be inferred  that the author had a conversation with his mother about Morris saying they were Coloured, but the identity of this mother is not confirmed.

However from [19], we learn that J.M.H. Gool’s first wife was named Bibi (Pee-rah-beegum) and later married Wahida Ta'al who was previously married to a Dollie. Their children included Fatima, Muddy, Nasimah, Nurjehan, Zarina, and the narrator himself.

But as for the author's biological mother, it is absent from all excerpts.

### q22 — Who was the author's father?

The author mentions two family names "Rassool" and "Gool", and states that he himself is Yousuf (Joe) Rassool, but does not explicitly state who his biological fathers are.

However, [1] tells us: "Peter Alexander Rassool: A family member of the narrator, married to Ayesha in 1928..." and it can be inferred that Peter Alexander Rassool is either the grandfather or father of the author. Later [17]: "...my dad... my grandfather found him a position in the firm, probably because of his neat copperplate handwriting." implies that the father of Yousuf Joe Rassool's mother, Ayesha, was the one with the position and he is referred to as Grandpa.

But then in [9] we see: "When a baby was born there would be commiseration if it was dark and joy if it was fair. Black at that time was definitely not beautiful...my dad." - which suggests the author is talking about his biological father, not grandfather this time.

Later [15]: "My father was handsome, bronzed like his mother, with hair slicked back, tall and sturdy... My grandfather found him a position in the firm,..."

[15] and other later passages also call Peter Alexander Rassool the grandfather of Yousuf Joe Rassool.

### q23 — Who were the author's siblings?

Based on source [2], the author notes that Fazil was a seven-year-old sibling who greatly attached to the author, and there is also mention of "my sisters" but no specific names mentioned.

### q24 — Who were the children of J.M.H. Gool?

According to [1], Abdul Hamid Gool's children with his second wife Wahida are:

* Cissie Gool (Zainunnissa), daughter, who married Abdul Hamid Gool.

As for J.M.H. Gool's children with his two wives (Bibi and Wahida), I found mentions of the following children in the excerpts, but not their mother or father:

From [3], Yousuf (Joe) Rassool mentions four quiet, well-mannered, and adorable children: 
Gadija, Hanief Ally, Toetie, and Hamid Gool. 

However, it seems to be referring to Nurjehan's children: 
* Gadija is also mentioned in [3] as “Auntie Dij” 
* Hamid Gool (also known as Midi) died at the age of 18.

And from [17], we see that J.M.H. Gool’s first son, A.H. Gool, was born in 1886.
It appears the sources do not provide information about all his children with Bibi and Wahida.

### q25 — Who was I.B. Tabata?

According to [3], Mr. Kies considered Shaw "a good man fallen among Fabians". This view was not contradicted, but on another matter, the characterisation of Mr. Tabata himself as the writer of a pamphlet criticising Ben Kies's analysis is evident because he rose to attack Kies in a meeting.

I.B. Tabata is mentioned numerous times throughout the sources, and it appears he was an important figure in the Non-European Unity Movement (NEUM) and the New Era Fellowship (NEF). He wrote a book called "The Awakening of a People" which became an NEF publication. The exact nature and extent of his role within these organisations is not provided in the excerpts.

No specific information about Tabata's personal history, birthdate, job title, or education can be found in the sources provided.

### q26 — Who was Dr. Abdullah Abdurahman?

According to the sources, [4] states that Dr. A. H. Gool married Cissie, who is referred to as "Zainunnisa", daughter of Dr. Abdulla Abdurahman in 1916 ([5]). Excerpt [10] mentions that Dr. Abdurahman was the first of the community "to qualify as a doctor from Glasgow University in 1893". It also states that he met and married Helen "Nellie" Potter James, suggesting that Nellie Abdurahman mentioned in Chapter Seven might be the wife.

Additionally, excerpt [11] mentions Dr. A.H. Gool writing two medical articles for the Indian Opinion; however, it does not directly address who Dr. Abdullah Abdurahman was. 

Excerpt [4]: (This is a partial summary of his life from multiple excerpts.)

Other Excerpts like [16] and [20] do not provide new information about Dr. Abdulla Abdurahman, but provide further context on the history of District Six as well as the relationship between him and Cissie Gool's grandfather J.M.H. Gool.

The provided sources suggest that there are two relevant pieces of information:

1. Dr. Abdulla Abdurahman was a doctor from Glasgow University who qualified in 1893.
2. He had a daughter named Nellie Abdurahman, and his daughter-in-law through marriage to Dr. A.H. Gool's wife Cissie (Zainunnisa).

### q27 — What was the connection between Gandhi and J.M.H. Gool?

The sources mention several connections between Gandhi and J.M.H. Gool:

* In [3], Owen Mathurin's book reports that one of Gandhi's sons fell in love with one of J. M. H. Gool's sisters - a Moslem girl, who wrote to Gandhi asking for his approval of their union.
* In [8], Eric Germain suggests that J.M.H. Gool was part of a pan-Islamic network between Liverpool, Capetown, and Perth, but the body of evidence from J. M. H. Gool's deeds and associations suggests that his world outlook was constructed less around the tenets of his religion and more around the vision of a non-racial democracy under the umbrella of the British Empire.
* In [19], it is stated that Gandhi's letter mentions Wilson in connection with Gool, suggesting some level of interaction or association between them.
* In [5], the author notes that Gandhi foresaw problems with a Hindu man marrying a Muslim girl, and intervened to prevent this union, indicating some familiarity and respect for J. M. H. Gool's family.
* In [6], Owen Mathurin's book reports that Gandhi wrote a letter in 1914 praising Dr. Abdurahman's and J.M.H. Gool's efforts as members of the Coloured People’s Vigilance Association.

Overall, it appears that Gandhi had a personal connection with the Gool family and was involved with their lives to some extent.

### q28 — Which organisations was the author involved in?

The sources do not explicitly state which organizations the author was directly involved with; however, they do mention several organisations that were relevant to his experience. A Students’ Union of some kind was being considered and eventually formed. The New Era Fellowship was a cultural organisation established by his family. Additionally, both the All African Convention and the Student Christian Movement's anti- discriminatory law were discussed in the passages provided.

The Anti-Coloured Affairs Department (Anti-CAD) movement is mentioned as an entity that he felt connected to through politicizing non-political bodies:

[2] e began to see eye 
to eye and when I suggested to him that the Club should affiliate to the Anti-CAD he agreed straightaway.
He even sponsored the motion to affiliate, which was carried unanimously.

This could indicate that the author supported and possibly participated in meetings with the movement.

### q29 — What was the relationship between the TLSA and the Non-European Unity Movement?

The provided sources do not contain information about a specific relationship between the Teachers' League of South Africa (TLSA) and the Non-European Unity Movement. However, [1] mentions that Haji Joosub Maulvi Hamid Gool's son Abdul Hamid "B.G. or Cheops" studied medicine in London, and married Cissie Gool (Zainunnissa), daughter of Dr. Abdullah Abdurahman. It implies a connection between the Gools and Dr. Abdurahman to these movements, but not explicitly stated as an active affiliation.

[7] mentions that “The idea of non-European unity arose in 1943 when the government foisted The Coloured Affairs on the so-called Coloured people... Yes, these attempts floundered because the historical circumstances were not yet ready for it”

Later [8] and [15] reference the role of Dr. Abdurahman and the “three occasions” he strove to create a non-European unity movement alongside D. D. T. Jabavu before “the idea” finally “floundered”.

[14] talks about the Teachers’ League of South Africa taking part in Non-European Unity Movement and that it is after they took ‘“the new road”, that is, broke with its previous stance of non-political professionalism.”

However, no clear details on a concrete relationship between the TLSA (Teachers' League) and the Non-European Unity Movement could be inferred.

### q30 — When did J.M.H. Gool arrive in Cape Town and from where?

According to excerpt [1], Haji Joosub Maulvi Hamid Gool, J.M.H. Gool's grandfather, arrived at Cape Town via Mauritius in 1884. 

However, the question specifically asks when J.M.H. Gool himself arrived there, not his grandfather. The source does not directly state this information about J.M.H. Gool.

### q31 — What was the Hanaffi Quwatul Islam Mosque?

According to [2] and [3], the Hanaffi Quwatul Islam Mosque was a mosque located in Loop Street, Cape Town. It was an outpost of Islam in an area becoming increasingly European, Jewish, and gentile. The mosque was completed in 1898 and was used by my grandfather's community (specifically his grandfather) for Eid festivals. [2] also mentions the Gools were founders and life trustees of the mosque.

### q32 — How was Cissie Gool related to J.M.H. Gool?

According to the sources, Cissie Gool (also known as Zainunnissa) was the daughter of Dr. Abdullah Abdurahman [1]). This makes her the sister-in-law of J.M.H. Gool, and also indicates that she was in close relation to him through this family connection. Later texts confirm: It was Cissie who, almost single-handedly, rocked the Government in 1937 when she called a massive demonstration (Letter JMH Gool to MK Gandhi Page 1 of 7 Courtesy Mr. ES Reddy as described by Yousuf (Joe) Rassool [177]).

The texts confirm that there is likely some uncertainty about some other family member identities, but the relationship between Cissie and J.M.H. is clearly documented here:  it was confirmed at multiple points in excerpt [4]:  
When we hypothesize the identities of the others (italicized), then the picture takes on a greater historical significance.
Cissie Gool, confidently returns the gaze of the camera lens and holds an angelic pose in the center of the frame.
Abdul Hamid, a decade into his marriage with Cissie, choses, instead, to cling protectively to his adopted sister, Toetie.

### q33 — Who were the notable historical figures that J.M.H. Gool knew personally?

According to Excerpt [7], Dr. Abdurahman's grandfather was a slave in South Africa brought by his master for freedom, and became prominent in the community. His mother, Gadija Dollie, a member of the Dollie clan, one of the great beauties of Cape Town, lived near the Gools at Buitencingle.

Excerpt [7] does not explicitly state that J.M.H. Gool personally knew Abdurahman's grandfather but mentions that the closeness in kinship and geography facilitated co-operation between Dr. Abdurahman and Grandpa and JMH Gool.

We know, however, from Excerpts [11] & also [7], that J.M.H. Gool was acquainted with prominent individuals including:
- Adam H.G Mahomed 
- Adam Gail Mahomed was a leader of the British Indian League.
The fact sheet provided by the author indicates he had discussions with notable people like Gandhi and  Rev Dan Wessels

### q34 — What was the Group Areas Act and how did it affect District Six?

According to excerpt [2], the Group Areas Act (1950) "legitimized poverty and oppression for the majority of its peoples, circumscribed where they could live – uprooting communities and ways of life." This act designated specific racial and ethnic groups to reside in particular areas, essentially creating a system of spatial segregation under Apartheid.

Excerpt [6] explains that when "a Coloured Area was declared White: the house prices plummeted as the new buyers were aware that the Coloured owners... would accept any paltry offer from prospective White purchasers." This meant that property values decreased due to racist assumptions by potential buyers.

As for District Six specifically, excerpt [2] states that it was "a thriving cosmopolitan area in the heart of Cape Town"... before the Group Areas Act dispersed its people across the "barren sandhills of the Cape Flats decimating the once vibrant community and way of life."

Additionally, excerpt [11] notes that as "the Indian population" under the Group Areas Act was being forcibly relocated, many were concerned about their future livelihoods, especially those who owned small shops. 

Excerpt [18] explains that "it came down to making representations to the Group Areas Board and seeking loopholes in the law; loopholes that were no sooner revealed than they were closed." This suggests that under this system, people sought ways navigate its restrictions, but such opportunities for navigating these regulations would eventually dry up.

Thus, while not a comprehensive definition of the Act itself, it is evident that the Group Areas Act was a tool to consolidate and legitimize racial separation which had a devastating effect on communities such as District Six.

### q35 — Who was Hassen Mall?

According to excerpt [1], Hassen Mall was a cricketer and an affiliate of Kismets C.C., with a nationality of South African. He is also mentioned in multiple other excerpts as a cricket captain, lawyer (he completed his LLB degree), and a person who was influential in the narrator's life.

### q36 — What political organisations were active in the Cape Coloured community during the author's lifetime?

The provided sources mention several political organisations that were active in the Cape Coloured community during the author's lifetime. However, I must report that not all specific information is available from the given excerpts.

According to [10], the following organisations were mentioned as being involved in the unity movement:

1. New Era Fellowship (NEF)
2. African People’s Organisation (A.P.O.)
3. Liberation League
4. Workers’ Party
5. Coloured branch of Smuts's United Party

Additionally, [11] mentions that these organisations joined together to establish the Anti-Car Colour (Anti-CAC) organisation, which aimed to counter the actions of the "Coloured Council".

In [14], the Communist Party is mentioned as another active player.

Other organisations such as The Cape Standard and The Guardian are listed in [17] and [18].

The sources do suggest a rich political landscape within the Cape Coloured community during this time period, with various groups striving to make their voices heard. 

To further identify all the active organisations at that time would need more information or other supplementary readings beyond what was provided.

### q37 — Who was Mahatma Gandhi and why was he in South Africa?

According to various excerpts [1], Mahatma Gandhi is widely known as "Indian leader and activist" who received a farewell address from Dr. A. H. Gool on behalf of Port Elizabeth Indians, where Dr. Gool hailed Gandhi as ‘Mahatma’ – Great Spirit (Excerpt [15]). 

It can be established that Gandhi first left South Africa in 1914 but had visited many times before [17]. He initially arrived in South Africa in 1893 for a one-year assignment from Dada Abdullah to assist his attorney in a civil suit against a merchant in Pretoria, due to the Gujarati language being the accounts-tongue (Excerpt [16]).

The provided sources do not contain Gandhi's full background before or after this period in South Africa.

### q38 — Who was Cissie Gool's father?

The provided sources do not contain information on who Cissie Gool's father was.

However, it is mentioned in [19] that "J. M. H. Gool and Dr. Abdurahman were all board members of the Wooding’s Preparatory private school", suggesting a possible relationship between J.M.H. Gool (Cissie Gool's husband) and Hajima Gool (mentioned in [20]). But it does not confirm any familial relationship between them.

There is no other source that explicitly mentions Cissie Gool's father or her parents.

### q39 — What was District Six like as a neighbourhood before the forced removals?

Based on Excerpts [1], [6], and [11], District Six was a thriving cosmopolitan neighborhood before the forced removals. It was described as "inimitable" and had a rich history of political struggle against apartheid. The author recalls walking away from it without regret, suggesting that they valued their experiences there.

Excerpt [4] describes Granny's memories of life in Rander before moving to District Six. She remembers the magnolia trees of her childhood and how pretty she was when seen by someone later on.

Before the removals, Excerpt [6] describes 95 Caledon Street as "home" where everything seemed "in place." The author recalls going there after a confusing situation with an auction, implying some disruption had occurred.

The area had various cultures and racial groups living together. As described in Excerpt [7], the neighborhood had a pre-Garveyite, pan-Africanist group of thinkers that counted Gool among their cohort.

Excerpt [5] states that Apartheid policy oppressed black women, men, and children by subjugating their being and legitimized poverty and oppression for the majority of its people. It also highlights how communities were uprooted, and ways of life were decimated when the Group Areas Act (1950) dispersed District Six's inhabitants across the Cape Flats.

Excerpt [11] provides a description of Caledon Street as being from Corporation Street to Clifton Street, "getting tattier along the way." The author notes that this was their home and describes how it seemed when viewed through their 10-year-old eyes each morning.

### q40 — What was the Unity Movement's boycott policy?

The Unity Movement's boycott policy is described in several sources [1], [3], [12], and [15].

Initially, the Movement used the boycott weapon to target individuals who were seen as agents of oppression or collaborators with the ruling authorities. For example, Captain Boycott, an agent of a large landowner, was targeted and ostracized by the peasants [1]. 

Later, in 1943, during the Group Areas Act campaign, the Unity Movement also employed boycotting tactics to counter the government's efforts to forcibly remove Coloured people from their homes. However, this boycott policy had its limitations. Some members of the Movement did not adhere to the boycott and still participated in registering their protests against reclassification [14], and [18].

It is worth noting that there were different interpretations within the Unity Movement about the most effective way to employ boycotted tactics, as illustrated by Jadwat's and Seedat’s differing views on how to handle dummy institutions that had seats to be elected into [2]. 

However, it appears that the overall strategy was to use boycotts as a means of non-collaboration and resistance against oppression.

