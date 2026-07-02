# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 195  
**Sampled:** 195  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 195  (id=-9123613270288747913)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #601

**Entities in chunk:**
  - Anti-Zionism
  - Democracy
  - Facts
  - General Smuts
  - Green Point Commons  (also: Green Point Common)
  - INDEX 166  (also: INDEX 166)
  - Liberty Cavalcade
  - Nazism
  - St George's Street
  - Waldorf Restaurant
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed ,  uncle 

**CC pass (raw):**
```json
{"quote": "My Uncle Hanief"}
```

**CC quote:** `My Uncle Hanief`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 2 / 195  (id=-9026931979920257841)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #209

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Cape Town
  - Dawood Seedat  (also: Dawood)
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Indian Congress Eventually Fatima  (also: Indian Congress Eventually Fatima)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** wed , daughter, mother, sister, brother,  cousin,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad and aunt were brother and sister"}
```

**CC quote:** `my dad and aunt were brother and sister`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and there is no specific name given for the narrator, we cannot determine the exact relationship. However, based on the quote "my dad and aunt were brother and sister", we can infer that the narrator's dad and aunt are siblings.

But since the schema does not allow for relationships like 'aunt', we return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 3 / 195  (id=-8601693670707718054)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #201

**Entities in chunk:**
  - Arthur Elliot Collection Courtesy Western Cape Archives
  - Bibi's children
  - Broach
  - Buitencingle Street  (also: Buitencingle)
  - Calcutta
  - District Six  (also: District Six)
  - Harold Cressy
  - Hewat College Motto
  - Hewat Teacher training College
  - Hewat Training College  (also: Hewat Training College)
  - Indian Opinion Feb 25, 1905
  - Indian women in South Africa
  - Lest We Forget
  - Mohammed Saaid (Uncle Aity)
  - National Gandhi Museum JMH Gool
  - Rander district of Surat
  - Roeland Street Gaol
  - St Martini German Lutheran Church
  - Surat
  - Trafalgar High  (also: Trafalgar High)
  - Whites only Primary school
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "Her son, Mohammed Saaid (Uncle Aity to us)"}
```

**CC quote:** `Her son, Mohammed Saaid (Uncle Aity to us)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 4 / 195  (id=-8565879665326676003)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #890

**Entities in chunk:**
  - Achee
  - Achmed
  - Avondale Road
  - Coon Club
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head  (also: Head Mr)
  - Indian Congress Eventually Fatima  (also: Indian Congress Eventually Fatima)
  - Meneer
  - Mr Pearce  (also: Pearce)
  - Natal
  - Rickshaw-man
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 5 / 195  (id=-8505583857113009851)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #373

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - Primary School  (also: Primary)
  - Ronnie Jooste  (also: Ronnie)
  - South America  (also: South Africa)
  - Trafalgar High School  (also: Trafalgar High School)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Abdul Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 6 / 195  (id=-8488686338445114035)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #333

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Times
  - Helen Abrahams
  - Israel
  - Menachem Begin
  - Muir Street Primary  (also: Muir Street Primary)
  - Students Representative Council Many Non-Whites
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 7 / 195  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - Drill Hall
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Gool  (also: Gool)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Guy's Hospital  (also: Guys Hospital)
  - Hans Friederichs
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Joosub Gool  (also: Joosub)
  - London
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Movement
  - PTAs
  - Sayed Kader Sayed Noor
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
{"quote": "Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines."}
```

**CC quote:** `Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Fatima Gool","relation":"sibling_of","to":"Jooseb Gool"}]}
```

**Extracted relations:** none

---

## Chunk 8 / 195  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Cape Town
  - Ebrahim
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Natasha
  - National Union of South African Students  (also: NUSAS)
  - Nikita Khruschev
  - Non-European Unity Movement  (also: Non European Unity Movement)
  - Non-European Weekly
  - Noor Bagh
  - Noors  (also: Noors)
  - Norman Bethune
  - Nosipho Majeke
  - Nurjahan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - O.Henry
  - Ob-Yennesi  (also: Ob-Yennesi)
  - Oddfellows Hall
  - Oliver Caldecott
  - Omar Abrahams
  - Omar Khayyam
  - Western Province  (also: Eastern Province)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "Fatima, heavy with child, and Ebrahim, her brother-in-law"}
```

**CC quote:** `Fatima, heavy with child, and Ebrahim, her brother-in-law`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ebrahim","relation":"spouse_of","to":"Fatima Seedat"}]}
```

**Extracted relations:**
- `Ebrahim` **spouse_of** `Fatima Seedat`

---

## Chunk 9 / 195  (id=-8186139953935417234)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #919

**Entities in chunk:**
  - Dora Taylor
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX 166)
  - Isaac Pfaff
  - Kies  (also: Mr Kies)
  - Langehoven  (also: Langehoven)
  - Movement
  - NEF'S
  - Nosipho Majeke
  - Organising Secretary
  - South America  (also: South Africa)
  - Trafalgar High  (also: Trafalgar High)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 10 / 195  (id=-8157488290903077572)

**Section:** 55  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #288

**Entities in chunk:**
  - Barnato Board
  - Broadway Dawn Darkies
  - Caledon Square
  - Coon Club
  - Dr Goolam Gool District Six
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Meneer
  - Movement
  - Mowbray
  - Professor James  (also: James)
  - SOYA
  - Salie Van Haacht
  - Trafalgar High School  (also: Trafalgar High School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
{"quote": "my sisters"}
```

**CC quote:** `my sisters`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "my sisters", we can infer that Yousuf Rassool has sisters, but their names are not mentioned. However, since the schema requires both endpoints to be identifiable as canonical names, and the sisters' names are not provided, we return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 11 / 195  (id=-8077784496787990642)

**Section:** 51  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #265

**Entities in chunk:**
  - Allied
  - Allies  (also: Allies)
  - Ardenne
  - Argus
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Generals Montgomery
  - George Golding  (also: Mr. George Golding)
  - Normandy
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Rev. Gow)
  - Salie Dollie
  - Von Kesselring
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , father, mother

**CC pass (raw):**
```json
{"quote": "my father", "my mother"} 

 Wait, no! According to the rules, I should only return one quote that exactly matches the format.

{"quote": "my father"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 12 / 195  (id=-7980378207581507601)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #207

**Entities in chunk:**
  - Advocate Christopher
  - Aunty Annie
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - British Government
  - Buitencingle Street  (also: Buitencingle)
  - Castle Street
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Gool  (also: Gool)
  - INDEX 166  (also: INDEX 166)
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Natal
  - Peerbhai
  - Peter Alexander Rassool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** married, father, mother,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad chose Ayesha from among all the spinsters of Castle Street and Buitencingle and they were married on the 1st of January 1928"}
```

**CC quote:** `my dad chose Ayesha from among all the spinsters of Castle Street and Buitencingle and they were married on the 1st of January 1928`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Ayesha Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Ayesha Rassool`

---

## Chunk 13 / 195  (id=-7752783211163375761)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1062

**Entities in chunk:**
  - Argus boys
  - Avenue
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Castle Bridge
  - Darling Street
  - Gardens
  - Jan Van Riebeeck
  - Kimberley
  - Little Theatre
  - Mount Nelson Hotel  (also: Mount Nelson Hotel)
  - Mount Streets  (also: Mount Street)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Orange Street
  - Parade
  - Parliament  (also: Parliament)
  - Slamat
  - South African Public Library
  - St. George's Grammar
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 14 / 195  (id=-7549255937705633040)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #947

**Entities in chunk:**
  - Alexander
  - Cape Town
  - Carlson
  - Colin Wynne
  - Cynthia Fisher
  - Dylan
  - Fazil Rassool  (also: Fazil)
  - Feyruz Rassool  (also: Feyruz)
  - George Veldsman
  - Habibia Institute  (also: Habibia Institute)
  - INDEX 166  (also: INDEX 166)
  - Joseph
  - Kelvinator
  - Lennie Dixon
  - Leonardo
  - Lewis
  - Reza Rassool  (also: Reza)
  - Samuel
  - South America  (also: South Africa)
  - Trafalgar Players
  - Zarina Rassool  (also: Zarina)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 15 / 195  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Boeta
  - Chops
  - Dr Abdul Hamid Gool  (also: Dr. Abdul Hamid Gool)
  - Gootie
  - Grootjie
  - INDEX 166  (also: INDEX 166)
  - James Africa
  - Woody Woodman  (also: Woody)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sibling,  uncle 

**CC pass (raw):**
```json
{"quote": "my half-uncle, Dr. Abdul Hamid Gool"}
```

**CC quote:** `my half-uncle, Dr. Abdul Hamid Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 16 / 195  (id=-7082280809534297158)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #345

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Education Department
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Ismail
  - Joan
  - Malaai
  - Sarlegh Doalie
  - Seven Steps
  - Suleiman Vallie

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 17 / 195  (id=-7025477089957931364)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #573

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bartnicke  (also: Bartnicke)
  - Cape Town
  - Charles Simons
  - Geoff Abrahams  (also: Geoff)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Messaris  (also: Mr. Messaris)
  - Natal
  - National Union of South African Students  (also: NUSAS)
  - New Era Fellowship
  - Pietermaritzburg
  - Richard Parker
  - Slingers  (also: Mr Slingers)
  - South African Students
  - Springveld
  - Students Representative Council  (also: SRC)
  - Trafalgar High  (also: Trafalgar High)
  - University of Natal
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 18 / 195  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - Agent General  (also: Agent General)
  - Coloured Advisory Council
  - E. Albertus  (also: Albertus)
  - General
  - Habibia  (also: Habibia)
  - Hewat  (also: Hewat)
  - Hymie Beimel  (also: Hymie)
  - Smuts's United Party  (also: Smuts)
  - Standard Two
  - Teachers League of South Africa  (also: TLSA)
  - Torch Commando  (also: Torch)
  - Trafalgar High  (also: Trafalgar High)
  - Trafalgar Players
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "my sisters"}
```

**CC quote:** `my sisters`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"my sisters"}]}
```

**Extracted relations:** none

---

## Chunk 19 / 195  (id=-6782705425542896048)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #113

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - England
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall
  - Khadija Ebrahim  (also: Khadija)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband, wed , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 20 / 195  (id=-6763001656966195273)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1067

**Entities in chunk:**
  - Chapel Street Staff
  - Ciskei
  - Danny De Beer  (also: Danny)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nelson Mandela
  - Peninsula Maternity Hospital  (also: Peninsula Maternity Hospital)
  - Transkei
  - Unity Movement  (also: Unity)
  - Van Riebeeck
  - Winston Layne
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 21 / 195  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Councillor Parker
  - E. Albertus  (also: Albertus)
  - Edross  (also: Mr Edross)
  - Fabians Kies  (also: Fabians)
  - Group Areas Boards  (also: Group Areas Board)
  - Head  (also: Head Mr)
  - Hoosain
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Labour Party
  - Mussolini
  - Parker  (also: Mr. Parker)
  - Parliamentary
  - Shaw's South African Christian Politcal Association (SACPA)  (also: Shaw)
  - Socialism
  - South America  (also: South Africa)
  - Soviet Union  (also: Soviet Union)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son ,  uncle 

**CC pass (raw):**
```json
{"quote": "his son Hoosain"}
```

**CC quote:** `his son Hoosain`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Hooosain"}]}
```

**Extracted relations:** none

---

## Chunk 22 / 195  (id=-6563863471018850689)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1136

**Entities in chunk:**
  - A. Roberts  (also: Roberts)
  - Cango Caves
  - Cape Town
  - Clanwilliam
  - Drakenstein Mountains
  - Du Toit's Kloof
  - Eastern Cape
  - Ebrahim
  - Gool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Isgak Adams
  - J.M.H.  (also: J.M.H)
  - James Currey  (also: James Curry)
  - Little Karoo
  - M Wilson  (also: Wilson)
  - Oudtshoorn
  - Persotem Patel  (also: Persotem)
  - Professor James  (also: James)
  - Rev R.A. Jackson
  - Reverend Gow  (also: Rev. Gow)
  - Solly Mohammed
  - Transvaal  (also: Transvaal)
  - Worcester
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 23 / 195  (id=-6141146778954841009)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #506

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town Woodstock
  - District 6
  - Durban's Curry's Fountain  (also: Durban)
  - Ebrahim
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Keraan
  - Louis Messaris
  - Messaris King Peanut Butter
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - Teachers League of South Africa  (also: TLSA)
  - Tykie Messaris  (also: Tykie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "my dad"}
```

**CC quote:** `my dad`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 24 / 195  (id=-6067802038126529915)

**Section:** Yousuf (Joe) Rassool   6  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #64

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape of Good Hope
  - City Council
  - Coloured Affairs
  - Dr. Jabavu  (also: Jabavu)
  - Gava Kadoes  (also: Gava)
  - Hanover Street
  - Hiema
  - Hindu Association
  - India
  - Indian Opinion
  - J. Lucheram  (also: J Lucheram)
  - M. Enus  (also: M Enus)
  - Mauritius
  - Province of Swat
  - Rander in Gujarat
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Hamid

**Triggers found:** wife, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "his brother’s footsteps"}
```

**CC quote:** `his brother’s footsteps`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 25 / 195  (id=-6012376786615465402)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1022

**Entities in chunk:**
  - Belgium
  - Britain's Empire  (also: Britain)
  - Cassiem  (also: Cassiem)
  - Denmark
  - District Six  (also: District Six)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - France  (also: France)
  - Holland
  - M Wilson  (also: Wilson)
  - Mr. Tregger  (also: Tregger)
  - Mrs. Wooding  (also: Wooding)
  - Norway
  - Queens Road
  - Rev Dan Wessels  (also: Dan Wessels)
  - Unity Movement  (also: Unity)
  - Van der Ross
  - Vassan Hindu C.C  (also: Vassan)
  - Veldsman
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Woodstock
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** wife, brother

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 26 / 195  (id=-5915939077846947542)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #738

**Entities in chunk:**
  - Britain's Empire  (also: Britain)
  - Cape Times
  - Charles Simons
  - Councillor Parker
  - England
  - Germany  (also: Germany)
  - INDEX 166  (also: INDEX 166)
  - Natal Daily Witness
  - Richard Parker
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 27 / 195  (id=-5853235968246498787)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #199

**Entities in chunk:**
  - Achee
  - Amina Minnie  (also: Amina)
  - Avondale Road
  - Cape Town
  - Daria
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fort Hare  (also: Fort Hare)
  - Hans Friederichs
  - Janab Jane
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** married, daughter, father, brother

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 28 / 195  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - British Labour Party MP
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gurukula at Hardwar  (also: Gurukula)
  - Hamid Khan
  - Hardwar
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - India
  - Indian Opinion
  - J. M. H. Gool
  - Marcina Kagan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natalian
  - Ramsey MacDonald
  - Rasheda Rassool  (also: Rasheda)
  - Rev. C. F. Andrews
  - South America  (also: South Africa)
  - Wagieda
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** son ,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "my Aunt Fatima"}
```

**CC quote:** `my Aunt Fatima`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 29 / 195  (id=-5732511328885138558)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #498

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Abe Desmore
  - Alawie
  - Cape Corps
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall
  - Limpopo River
  - Mogadies
  - Mr Regal
  - Nerine Desmond
  - Per Angusta Ad Augusta
  - Roland Colman
  - Smuts's United Party  (also: Smuts)
  - South America  (also: South Africa)
  - Trafalgar Junior School
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad’s younger stepbrother"}
```

**CC quote:** `my dad’s younger stepbrother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 30 / 195  (id=-5714165220014992650)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #255

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Anti-CAD  (also: Anti-CAD)
  - Fazil Rassool  (also: Fazil)
  - Glen
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jack Meltzer
  - Kies  (also: Mr Kies)
  - Kloof Nek
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Movement
  - Platteklip Gorge
  - Slangolie
  - Table Mountain
  - Woody Woodman  (also: Woody)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother, brother

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 31 / 195  (id=-5610384977170982774)

**Section:** Yousuf (Joe) Rassool   160  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1029

**Entities in chunk:**
  - Alec Bedser
  - Blauwberg Strand
  - Cape Peninsula
  - Cliff Gladwin
  - Cyril Washbrook
  - Dennis Compton
  - Divisional Council
  - Doug Wright
  - Godfrey Evans
  - Hamid Khan
  - Len Hutton
  - Mylie
  - Victor Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** married, son , sister, brother, in-law

**CC pass (raw):**
```json
{"quote": "my sister Zain married Hamid Khan"}
```

**CC quote:** `my sister Zain married Hamid Khan`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Zain Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Zain Rassool`

---

## Chunk 32 / 195  (id=-5556520109474695452)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #135

**Entities in chunk:**
  - Australia
  - Australians
  - Board
  - Buitencingle Street  (also: Buitencingle)
  - Canada
  - Cape Town
  - Dr. Jabavu  (also: Jabavu)
  - Dr. Jamieson
  - George Golding  (also: Mr. George Golding)
  - Mr. B. Allie  (also: B. Allie)
  - Mr. Barmania (MA)  (also: Barmania MA)
  - New Zealanders
  - Progressive Party
  - Salie Dollie
  - Tommie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "My grandfather"}
```

**CC quote:** `My grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 33 / 195  (id=-5533644833763956894)

**Section:** 55  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #287

**Entities in chunk:**
  - Agent General  (also: Agent General)
  - Deep South
  - INDEX 166  (also: INDEX 166)
  - India
  - Mr Gool
  - Mrs. Wo  (also: Mrs Wo)
  - Tagara
  - Trafalgar High School  (also: Trafalgar High School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
{"quote": "my sisters"}
```

**CC quote:** `my sisters`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"my sisters"}]}
```

**Extracted relations:** none

---

## Chunk 34 / 195  (id=-5460737940864037782)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1137

**Entities in chunk:**
  - Abdurahman
  - Adonis
  - Bellingham  (also: Mr Bellingham)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town
  - Chota
  - Gadija Dollie
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Kloof Street  (also: Loop Street)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Ralph J. Bunche  (also: Ralph Bunche)
  - Sayed Fethi
  - South America  (also: South Africa)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother

**CC pass (raw):**
```json
{"quote": "His mother, Gadija Dollie"}
```

**CC quote:** `His mother, Gadija Dollie`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Gadija Dollie"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Gadija Dollie`

---

## Chunk 35 / 195  (id=-5418155653937249668)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1145

**Entities in chunk:**
  - All African Convention
  - Bloemfontein
  - Buitencingle Street  (also: Buitencingle)
  - District Six  (also: District Six)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Emily Hobhouse
  - Even Buitencingle
  - Goulam Gool
  - Governor-General
  - Groot Schuur
  - Group Areas Boards  (also: Group Areas Board)
  - Les Jacobs
  - Liberals
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Gladstone  (also: Mrs. Gladstone)
  - New Era Fellowship
  - Prime Minister Botha
  - Ralph J. Bunche  (also: Ralph Bunche)
  - United Nations Centre
  - Vic Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, sister

**CC pass (raw):**
```json
{"quote": "one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters"}
```

**CC quote:** `one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}

This is because the quote mentions a relationship between Gandhi's son and Goulam Gool's sister, but it does not mention a direct family relationship between two individuals that can be identified as canonical names. The relationships mentioned are "son" and "sister", which are not in the schema.
```

**Extracted relations:** none

---

## Chunk 36 / 195  (id=-5364503672193663248)

**Section:** Yousuf (Joe) Rassool   16  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #119

**Entities in chunk:**
  - Alexander
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle Street  (also: Buitencingle)
  - Chapel Street School  (also: Chapel Street School)
  - District Six  (also: District Six)
  - Dylan
  - Feyruz Rassool  (also: Feyruz)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Ally  (also: Hussain Ally)
  - J. M. H. Gool
  - John Schlesinger
  - Joseph
  - Leonardo
  - Lewis
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Reza Rassool  (also: Reza)
  - Samuel
  - Sayed Noor Chota
  - South America  (also: South Africa)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "Mom (Ayesha)"}

This clause identifies a direct family relationship between two people, where at least one person from the list is named. The possessive word "Mom" establishes that Ayesha is the mother of the narrator.
```

**CC quote:** `Mom (Ayesha)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 37 / 195  (id=-5340629096267782490)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #75

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
{"quote": "her elder sister"}
```

**CC quote:** `her elder sister`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "her elder sister", we can infer that Yousuf Rassool has a sibling. However, the quote does not mention the name of the sister or any other person in relation to Yousuf Rassool.

But since the quote uses 'sister', which is a valid relation type (sibling_of), and one endpoint is identifiable as a canonical name ('Yousuf Rassool'), we can return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 38 / 195  (id=-5308475734057125055)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #348

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Athlone Branch  (also: Athlone Branch)
  - Chapel Street
  - Chapel Street School  (also: Chapel Street School)
  - Domingo  (also: Mrs Domingo)
  - Even Buitencingle
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Khalifa
  - Miss Brown
  - Mount Streets  (also: Mount Street)
  - Teachers League of South Africa  (also: TLSA)
  - Wilfrid Greeff
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 39 / 195  (id=-5184936258167894087)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #714

**Entities in chunk:**
  - America
  - Bishop Lavis
  - Bonteheuwel
  - Cape Town
  - District Six  (also: District Six)
  - Even Buitencingle
  - Ghulzar Khan  (also: Gulzar Khan)
  - Gool  (also: Gool)
  - Habibia Institute  (also: Habibia Institute)
  - Hanover Park
  - India
  - M Wilson  (also: Wilson)
  - Mannenberg
  - Mitchell's Plain
  - Professor Dadaker  (also: Professor Dadekar)
  - Silvertown
  - Vanguard Estate
  - West Indies
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 40 / 195  (id=-5154709854984411195)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #761

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Chapel Street School  (also: Chapel Street School)
  - Compton
  - Doug Wright
  - E.A. Parker
  - Edrich
  - Faried Rossier
  - Gladwin
  - Habibia Institute  (also: Habibia Institute)
  - Hassan Bavasah  (also: Hassan)
  - Hutton  (also: Hutton)
  - Jannat
  - John Arlott
  - Nasim Rassool  (also: Nasim)
  - Roly Jenkins
  - Unity Movement  (also: Unity)
  - Washbrook
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "son of E.A. Parker"}
```

**CC quote:** `son of E.A. Parker`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 41 / 195  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Cape
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Heathrows C.C  (also: Heathrows C.C.)
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - Indian Affairs Department
  - Mister Efficiency
  - Muslims C.C.  (also: Muslims)
  - Natal
  - Paddy Thomas
  - Salie Van Haacht
  - Seventh Day Adventists
  - Stanley Abrahams
  - Western Province Indian Cricket Union
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 42 / 195  (id=-5010050899247603262)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1138

**Entities in chunk:**
  - Abdurahman
  - Alan Gregor Cobley
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town
  - Cathedral Hall
  - Coloured People's Vigilance Committee  (also: Coloured People's Vigilant Committee)
  - Edgar Maurice
  - Even Buitencingle
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Indian Opinion
  - J.M.H. Wilson
  - James M.
  - John Brown
  - Journal of Southern African Studies
  - Kloof Street  (also: Loop Street)
  - Nasima
  - Oswald Pirow
  - South America  (also: South Africa)
  - Victoria Street

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 43 / 195  (id=-5009073907762257871)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #100

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bhokkie
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 44 / 195  (id=-4897146471319258283)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #117

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "his young bride Nurjehan"}
```

**CC quote:** `his young bride Nurjehan`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his young bride Nurjehan", we can infer that Yousuf Rassool is the husband of Nurjehan Gool Mohamed.

Here is the extracted family relationship in JSON format:

{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Nurjehan Gool Mohamed"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Nurjehan Gool Mohamed`

---

## Chunk 45 / 195  (id=-4713556574132170500)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #609

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape
  - Cassim Amra
  - Catherine Pienaar
  - Helen Abrahams
  - INDEX 166  (also: INDEX 166)
  - Joss
  - Spelling Bee
  - University of Witwatersrand
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 46 / 195  (id=-4527305166854275038)

**Section:** 129  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #815

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Delarey
  - George
  - Grand Parade
  - INDEX 166  (also: INDEX 166)
  - Kies  (also: Mr Kies)
  - Population Registration Act
  - Strand
  - Trafalgar
  - United Party
  - Unity Movement  (also: Unity)
  - White Areas
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother,  cousin

**CC pass (raw):**
```json
{"quote": "my mother's cousins the Noors"}
```

**CC quote:** `my mother's cousins the Noors`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "my" refers to Yousuf Rassool and the quote mentions "my mother's cousins", we can infer that the Noors are the cousins of Yousuf Rassool's mother. However, since "cousin" is not in the schema, according to the rules, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 47 / 195  (id=-4440340057225779401)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #314

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Andrew Mackrill
  - Cape Town University  (also: Cape Town University)
  - Cardiff
  - Chapel
  - Colin Wynne
  - Cynthia Fisher
  - Fruit and Vegetable Market
  - George Veldsman
  - Hewat Amateur Theatrical Society
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Isaac Pfaff
  - Ivan Agherdien
  - Leonard Dixon
  - Moslem Mission School
  - Muir Street
  - Norman Florence
  - Rasheda Rassool  (also: Rasheda)
  - Sam Wo's Laundry  (also: Sam Wo's Laundry)
  - Selkirk Streets
  - Zain Rassool  (also: Zain)

**Triggers found:** married

**CC pass (raw):**
```json
{"quote": "Sam Wo was married to a Cape Malay woman"}
```

**CC quote:** `Sam Wo was married to a Cape Malay woman`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 48 / 195  (id=-4279669986134027190)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #620

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Capetown
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Government Avenue
  - India
  - Islamic
  - Ismail
  - Katie
  - Mouille Point
  - Nasima
  - Non-European Unity Movement  (also: Non European Unity Movement)
  - Raza
  - Somerset Hospital
  - Streptomycin
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 49 / 195  (id=-4263780790926522616)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #187

**Entities in chunk:**
  - Abdurahman
  - Advocate Henry Sylvester Williams
  - J. M. H. Gool
  - J.Boyce
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Daily Witness
  - Natal Indian Congress
  - National Gandhi Museum  (also: National Gandhi Musem)
  - National Road
  - National Theatre  (also: National Theatre)
  - Nationalists Party  (also: Nationalist Party)
  - Native Life in South Africa
  - Native Representative Councils
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Nehru
  - Nelson Eddy
  - Nelson Mandela
  - New Era Fellowship
  - Nicky Springveld
  - Nikita Khruschev
  - Noor Bagh
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 50 / 195  (id=-4183482280000304421)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #31

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Buitencingle Street  (also: Buitencingle)
  - Cape
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Gool  (also: Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - London
  - M Wilson  (also: Wilson)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Oxford
  - Pan African Conference
  - Rhodes House Library  (also: Rhodes House Library)
  - Trinidad
  - Williams
  - Wooding's Preparatory School

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 51 / 195  (id=-3965621148867394083)

**Section:** The struggle unfolds  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #132

**Entities in chunk:**
  - Achmed
  - Beloved Country
  - Cape Town
  - Chapel Street
  - Cow & Gate  (also: Cow & Gate)
  - Cronin  (also: Cronin)
  - Cultural Society
  - Cynthia Carelse
  - Cynthia Fisher
  - District Six  (also: District Six)
  - English-speaking Europeans
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Head  (also: Head Mr)
  - Meneer
  - Nationalists Party  (also: Nationalist Party)
  - South African Party
  - United Party
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 52 / 195  (id=-3900190985977953352)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #150

**Entities in chunk:**
  - Boers  (also: Boers)
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Durban's Curry's Fountain  (also: Durban)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Isvarlal
  - Itie
  - Manilal
  - Manny
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Roger Streets
  - Sakarlal
  - Transvaal Asiatic Act
  - Tyne and Roger Streets  (also: Tyne)
  - Union of South Africa  (also: Union of South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 53 / 195  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Buitencingle Street  (also: Buitencingle)
  - Education Department
  - Emily Hobhouse
  - Grey Street
  - Groot Schuur
  - INDEX 166  (also: INDEX 166)
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Gladstone  (also: Mrs. Gladstone)
  - Prime Minister Botha
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 54 / 195  (id=-3882651649594979630)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #735

**Entities in chunk:**
  - Achee
  - Afrikaans Language Movement  (also: Afrikaans)
  - Argus Annual
  - Cape Town
  - Genadendal
  - Gool  (also: Gool)
  - INDEX 166  (also: INDEX 166)
  - Loop Sts
  - Malmesbury
  - Maritzburg
  - Mohmet
  - Muddy  (also: Muddy)
  - Natal
  - National Union of South African Students  (also: NUSAS)
  - Parade
  - Pietermaritzburg
  - Psychology
  - South African Directory
  - Waal
  - Wormy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "Muddy’s eldest brother"}
```

**CC quote:** `Muddy’s eldest brother`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 55 / 195  (id=-3872024114827273512)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #78

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Budenny
  - C. Rassool
  - Chaim Beimel
  - Churchill
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - City Hall
  - Comrade Bill
  - District Six  (also: District Six)
  - Field Marshal Smuts
  - Friends of the Soviet Union
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Harry Snitcher
  - Hollywood  (also: Hollywood)
  - INDEX 166  (also: INDEX 166)
  - Joosub Gool  (also: Joosub)
  - King Edward VII
  - Malinovsky
  - Moses Kotane
  - Musto
  - Ray Alexander
  - Rokossovsky
  - Russia
  - Russians
  - Smuts's United Party  (also: Smuts)
  - Song
  - Soviet Union  (also: Soviet Union)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 56 / 195  (id=-3850285751536457852)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #251

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Malays
  - City Council
  - Dr Du Plessis  (also: Du Plessis)
  - Dr. Jabavu  (also: Jabavu)
  - Edross  (also: Mr Edross)
  - Heneke  (also: Mr Heneke)
  - Hindu Association
  - I.D. Du Plessis
  - INDEX 166  (also: INDEX 166)
  - J. Lucheram  (also: J Lucheram)
  - M. Enus  (also: M Enus)
  - Mauritius
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 57 / 195  (id=-3844520640191004859)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #96

**Entities in chunk:**
  - Aminabhen
  - Armin Dawood Afghan Rifles Subardar
  - Belgium
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Denmark
  - District Six  (also: District Six)
  - Europe
  - France  (also: France)
  - Hamza
  - Holland
  - INDEX 166  (also: INDEX 166)
  - Norway
  - Ronnie Jooste  (also: Ronnie)
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother, sister, brother, in-law

**CC pass (raw):**
```json
{"quote": "his brothers-in-law Armin Dawood"}
```

**CC quote:** `his brothers-in-law Armin Dawood`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "his" refers to Yousuf Rassool and the quote states "his brothers-in-law Armin Dawood", we can infer that Armin Dawood is the brother of Yousuf's spouse. However, since the name of Yousuf's spouse is not mentioned in the quote, we cannot establish a direct relationship.

But, looking at the list of known persons, we see that Aminabhen and Bibi Gool (also: Bibi, Peari Beghum) are listed as separate individuals. If we assume that one of them might be Yousuf's spouse, then Armin Dawood would be their sibling, making him Yousuf's brother-in-law.

However, without a direct mention of the spouse's name in the quote, we cannot establish this relationship with certainty. Therefore, based on the given rules and the information provided in the quote, we must return an empty list:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 58 / 195  (id=-3635611545960891135)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #505

**Entities in chunk:**
  - District Six  (also: District Six)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Ironically Tykie
  - Louis Messaris
  - Messaris Junior
  - South America  (also: South Africa)
  - Tykie Messaris  (also: Tykie)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "Ironically, “Tykie’s” father, Louis Messaris"}
```

**CC quote:** `Ironically, “Tykie’s” father, Louis Messaris`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Louis Messaris","relation":"parent_of","to":"Tykie Messaris"}]}
```

**Extracted relations:**
- `Louis Messaris` **parent_of** `Tykie Messaris`

---

## Chunk 59 / 195  (id=-3578174338829636170)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #90

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX 166)
  - J. Spice Merchants  (also: Spice Merchants)
  - Sarleh Abdul
  - Ship Chandlers
  - Students Union
  - Teachers League of South Africa  (also: TLSA)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , in-law

**CC pass (raw):**
```json
{"quote": "Your loving son, Al Hajj Joosub Maulvi Hamid"}
```

**CC quote:** `Your loving son, Al Hajj Joosub Maulvi Hamid`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Al Hajj Joosub Maulvi Hamid","relation":"child_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Al Hajj Joosub Maulvi Hamid` **child_of** `Yousuf Rassool`

---

## Chunk 60 / 195  (id=-3500962997584184096)

**Section:** Yousuf (Joe) Rassool   96  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #580

**Entities in chunk:**
  - Cape Town
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - Even Buitencingle
  - Gool  (also: Gool)
  - Guys Hospital, London
  - INDEX 166  (also: INDEX 166)
  - Irwin Combrinck
  - Nazli Behardien
  - South African Communist Party
  - South America  (also: South Africa)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zainunnisa

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "his father"}
```

**CC quote:** `his father`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his father", we can infer that Yousuf Rassool's father is being referred to. However, the name of the father is not explicitly mentioned in the quote.

But since the narrator is 'my' (implied by "I", "me") and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. And one of the known persons is also named "Gool" which could be a surname, so it's possible that Gool is Yousuf Rassool's father.

So, based on this assumption:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Gool`

---

## Chunk 61 / 195  (id=-3473692056619927671)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #227

**Entities in chunk:**
  - Blauwberg Strand
  - British Government
  - Cape Argus
  - Cape Peninsula
  - Dada Abdullah
  - Disfranchisement Law
  - Divisional Council
  - Fazil Rassool  (also: Fazil)
  - India
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Government
  - Natal Indian Congress
  - Pretoria Gandhi  (also: Pretoria)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "My third eldest brother, Fazil"}
```

**CC quote:** `My third eldest brother, Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 62 / 195  (id=-3364667971095419225)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #463

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape
  - Cape Town
  - Fazil Rassool  (also: Fazil)
  - George Meissenheimer  (also: George Meisenheimer)
  - Mauritius
  - Searle Street
  - South African Apartheid
  - Wahida Ta Al8
  - Wale Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 63 / 195  (id=-3325270166547380284)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #406

**Entities in chunk:**
  - Cape Town Fire Brigade
  - District Six  (also: District Six)
  - Grand Parade
  - Heerie  (also: Heerie)
  - Ismail
  - Mr. B. Allie  (also: B. Allie)
  - Nasim Rassool  (also: Nasim)
  - Rutger Street
  - Union Fruit Supply
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer

**Triggers found:** brother,  cousin

**CC pass (raw):**
```json
{"quote": "Nasim, my baby brother"}
```

**CC quote:** `Nasim, my baby brother`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 64 / 195  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - Botanical Gardens  (also: Botanical)
  - C.Khatieb  (also: Khatieb)
  - Government Avenue
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Captain
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Next Hassen
  - Nurjahan
  - Rasheda Rassool  (also: Rasheda)
  - Rashid
  - Raza
  - S.Abed  (also: Abed)
  - Salie
  - Shawquet
  - South America  (also: South Africa)
  - Trafalgar
  - Western Province  (also: Eastern Province)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)
  - Zurayda

**Triggers found:** wife, daughter, mother, sister,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 65 / 195  (id=-3059365739205808916)

**Section:** Chapter Five  Characters of District Six  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #411

**Entities in chunk:**
  - Athlone
  - Auchinlek
  - Avalon Cinema  (also: Avalon Cinema)
  - Cairo
  - Churchill
  - Egyptian
  - Gava Kadoes  (also: Gava)
  - Green Point Commons  (also: Green Point Common)
  - Habibia  (also: Habibia)
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - India
  - Municipal
  - Mymoena Roomaney
  - Rabeyah Mukkadam
  - Rasheda Pansari
  - Ritchie
  - South African 2nd Division
  - Tobruk
  - Wavell
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 66 / 195  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - District Six  (also: District Six)
  - Dr Ramamurthi
  - INDEX 166  (also: INDEX 166)
  - Wynberg
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zurayda

**Triggers found:** mother, sister, brother

**CC pass (raw):**
```json
{"quote": "His baby sister, Zurayda"}
```

**CC quote:** `His baby sister, Zurayda`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Zurayda"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Zurayda`

---

## Chunk 67 / 195  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Athlone
  - Charles Simons
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gool  (also: Gool)
  - INDEX 166  (also: INDEX 166)
  - India
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Pat Holmes
  - Richard Parker
  - Space  (also: Space)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 68 / 195  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Gool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - India
  - J.M.H.  (also: J.M.H)
  - Jeddah
  - Supreme Court
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 69 / 195  (id=-2740820424072264356)

**Section:** 5  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #59

**Entities in chunk:**
  - Ben Cloete
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Furjah
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 70 / 195  (id=-2550570213336857687)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #52

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Deanna Durbin
  - Gardener Williams  (also: Gardiner Williams)
  - Leslie Hutchinson
  - Nelson Eddy
  - Searle Street
  - Trafalgar High  (also: Trafalgar High)
  - Trafalgar Park
  - Walmer Estate
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent, sister

**CC pass (raw):**
```json
{"quote": "my parents"}
```

**CC quote:** `my parents`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 71 / 195  (id=-2487606880044980631)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #978

**Entities in chunk:**
  - African National Congress
  - Area
  - Bellingham  (also: Mr Bellingham)
  - Cape Town
  - Freedom Charter
  - Gregori Malenkov  (also: Grigori Malenkov)
  - Kenya Jomo Kenyatta  (also: Kenya)
  - Kliptown
  - Point Programme
  - Scotches Kloof
  - Stalin
  - Unity Movement  (also: Unity)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 72 / 195  (id=-2266997192548340950)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #95

**Entities in chunk:**
  - Armin Dawood
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Good Samaritan
  - Hamza
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "his brothers-in-law Armin Dawood"}
```

**CC quote:** `his brothers-in-law Armin Dawood`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "his" refers to Yousuf Rassool and the quote states "his brothers-in-law Armin Dawood", we can infer that Armin Dawood is the brother of Yousuf's spouse. However, since the narrator is 'I' (Yousuf), we can conclude that Armin Dawood is the brother of Yousuf's wife.

Assuming Wahida Gool is Yousuf's wife (based on the provided names), we can infer the relationship as follows:

{"relations":[{"from":"Wahida Gool","relation":"sibling_of","to":"Armin Dawood"}]}

However, since the task requires us to use only the canonical names mentioned in the quote and "Wahida Gool" is not explicitly mentioned in the quote, we cannot make this inference.

Therefore, based on the provided rules and the exact quote, we can conclude that:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 73 / 195  (id=-2219204583155630177)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #151

**Entities in chunk:**
  - Abdul Kadar
  - Amod Gool
  - Cape Town
  - Durban's Curry's Fountain  (also: Durban)
  - England
  - Fountain Head
  - INDEX 166  (also: INDEX 166)
  - Johannesburg
  - Joseph Chamberlain
  - Lachiram
  - London
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Transvaal Indians
  - Westminster
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Hamid Gool

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 74 / 195  (id=-2147031409973546390)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #509

**Entities in chunk:**
  - Abba
  - African Methodist Episcopal Church
  - Borstol
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Cassiem  (also: Cassiem)
  - Coloured Advisory Council
  - George Golding  (also: Mr. George Golding)
  - Harry Lawrence
  - Marks's furniture shop  (also: Marks)
  - Mr. Tregger  (also: Tregger)
  - Olino
  - Ou Bhai
  - Salie Dollie
  - Smuts's United Party  (also: Smuts)
  - Stalingrad

**Triggers found:** parent,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Naz"}
```

**CC quote:** `my cousin Naz`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 75 / 195  (id=-2140983584723589461)

**Section:** Chapter Nineteen  All Africa Convention  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #995

**Entities in chunk:**
  - Bellingham  (also: Mr Bellingham)
  - Cape
  - General Nguyen Giap
  - Miss Thwaites
  - Mr Borchers  (also: Borchers)
  - Mrs McDonald
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - President Harry Truman
  - Pretoria Gandhi  (also: Pretoria)
  - Prince of Wales  (also: Prince of Wales)
  - Professor Dadaker  (also: Professor Dadekar)
  - Professor James  (also: James)
  - Progressive Party
  - Prop Diamond  (also: Prop Diamond)
  - Province of Swat
  - Public Slipper & Turkish Baths
  - Punjabi  (also: Punjabi)
  - Purcell
  - Queen Victoria
  - Queen Victoria Street
  - Queens Road
  - Quwatul Islam Mosque
  - R. Hoedemaker  (also: Hoedemaker)
  - R.O.
  - Rita Olivier
  - Walter Swanson

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 76 / 195  (id=-2049559656837982089)

**Section:** Yousuf (Joe) Rassool   144  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #924

**Entities in chunk:**
  - Anti-Zionism
  - England
  - Gorki
  - Hymie Beimel  (also: Hymie)
  - INDEX 166  (also: INDEX 166)
  - Movement
  - Rasheda Rassool  (also: Rasheda)
  - SOYA
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "my sister Rasheda"}
```

**CC quote:** `my sister Rasheda`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Rasheda Rassool`

---

## Chunk 77 / 195  (id=-2047049700032512222)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #223

**Entities in chunk:**
  - Biology Lab  (also: Biology)
  - Fletchers & Cartwrights  (also: Fletchers)
  - Glen
  - Hamid Midi
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek
  - Lion's Head
  - Muir Street Moslem School
  - Signal Hill
  - South America  (also: South Africa)
  - Table Bay
  - Table Mountain
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 78 / 195  (id=-2010807931329773554)

**Section:** Yousuf (Joe) Rassool   x  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #19

**Entities in chunk:**
  - British Newspaper Library
  - Chinese Nationalists
  - Colindale
  - Generalissimo Chiang Kai Shek
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - India
  - Kuomintang Party
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Non-European Unity Movement  (also: Non European Unity Movement)
  - Rasheda Rassool  (also: Rasheda)
  - South America  (also: South Africa)
  - Trafalgar
  - Zain Rassool  (also: Zain)

**Triggers found:** wife, daughter, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "my wife, Nazima"}
```

**CC quote:** `my wife, Nazima`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Zain Rassool","relation":"spouse_of","to":"Nazima Rassool"}]}
```

**Extracted relations:**
- `Zain Rassool` **spouse_of** `Nazima Rassool`

---

## Chunk 79 / 195  (id=-1961234090315831513)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #380

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed
  - Fazil Rassool  (also: Fazil)
  - Halima Gool
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Khan's cafe  (also: Khan)
  - Selim Gool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "my dad"}
```

**CC quote:** `my dad`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 80 / 195  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Gool
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - British Indian League
  - Buitencingle Street  (also: Buitencingle)
  - District Six  (also: District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gokhale
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Kloof Street  (also: Loop Street)
  - Lest We Forget
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Noor Bagh
  - Shaheen Gool  (also: Shaheen)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , daughter, mother,  cousin,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "my cousin, Shaheen"}
```

**CC quote:** `my cousin, Shaheen`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 81 / 195  (id=-1671105553751995199)

**Section:** Yousuf (Joe) Rassool   104  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #631

**Entities in chunk:**
  - Arabic Surah
  - Broach
  - Bunny Kriekler
  - Earnest Livingston MQotsi
  - Gerald Newman
  - Harold Wolpe
  - INDEX 166  (also: INDEX 166)
  - John Clayton
  - Khadija Ebrahim  (also: Khadija)
  - Laurence Olivier
  - Oliver Caldecott
  - P.V. Tobias
  - Rander
  - Surat
  - Sydney Brenner
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Wuthering Heights

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 82 / 195  (id=-1594342289169322262)

**Section:** 61  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #331

**Entities in chunk:**
  - Cape African Teachers Association
  - Dickman's Bakery
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Islamic
  - League of Nations  (also: League)
  - Let Us Live
  - Maron  (also: Mr. Maron)
  - Omar Khayyam
  - Shaheen Gool  (also: Shaheen)
  - Synagogue
  - Teachers League of South Africa  (also: TLSA)
  - Vandeleur Street
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Waynick  (also: Waynick)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 83 / 195  (id=-1566108535838797793)

**Section:** 81  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #474

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Comrade Bill
  - De Waals Drive  (also: De Waal Drive)
  - Dora Taylor
  - Harry Snitcher
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Kotane
  - Mnguni
  - Mr Jaffe  (also: Jaffe)
  - Muir Street
  - Musto
  - Nosipho Majeke
  - Rander
  - Ray Alexander
  - Soviet Union  (also: Soviet Union)
  - Y'Allah
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 84 / 195  (id=-1507950876298588482)

**Section:** 183  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1146

**Entities in chunk:**
  - Afr Gandhi
  - Ben Kies  (also: Ben Kies M.A)
  - Cape
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Coloured Advisory Council
  - Henry Sylvester Williams
  - INDEX 166  (also: INDEX 166)
  - London
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Owen Mathurin
  - Pan-African Movement
  - Ralph J. Bunche  (also: Ralph Bunche)
  - Solly Edross
  - Trinidad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, son , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 85 / 195  (id=-1429068271951167841)

**Section:** Chapter Nine  Senior Year  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #564

**Entities in chunk:**
  - Cape Town
  - Clanwilliam
  - Hell's Heights Pass
  - INDEX 166  (also: INDEX 166)
  - Messaris Junior
  - Muir Street
  - Persotem Patel  (also: Persotem)
  - Tykie Messaris  (also: Tykie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son ,  uncle 

**CC pass (raw):**
```json
{"quote": "my Uncle Hanief"}
```

**CC quote:** `my Uncle Hanief`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 86 / 195  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis
  - Dr. DuPlessis  (also: DuPlessis)
  - Edross  (also: Mr Edross)
  - Morris
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 87 / 195  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bourgeois  (also: Bourgeois)
  - C.Khatieb  (also: Khatieb)
  - Democratic Parliament
  - Ebrahim
  - G.Abed
  - Hanover Street
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX 166)
  - Jeddah
  - Joosub Gool  (also: Joosub)
  - Malick Hayat Captain
  - Movement
  - Muddy  (also: Muddy)
  - S.Abed  (also: Abed)
  - Tabata
  - Ten Point Programme
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Western Province  (also: Eastern Province)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "Muddy’s brother"}
```

**CC quote:** `Muddy’s brother`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "Muddy's brother" is stated in the quote and Muddy is a known person, we can extract the relationship. However, since the narrator is not explicitly mentioned as 'my' with a given name, we cannot determine the exact canonical name for the narrator.

But, considering the rules, if both endpoints are not identifiable as canonical names, return empty.

So, in this case:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 88 / 195  (id=-1046818788447995973)

**Section:** 47  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #238

**Entities in chunk:**
  - Baby pear
  - Caledon Streets  (also: Caledon Street)
  - Daddy pear  (also: Daddy)
  - Education Department
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX 166)
  - Mommy pear
  - Vandeleur Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sibling

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 89 / 195  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Education Department
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gool  (also: Gool)
  - Grey Street
  - INDEX 166  (also: INDEX 166)
  - M Wilson  (also: Wilson)
  - Manilal
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Ralph J. Bunche  (also: Ralph Bunche)
  - Sushila
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, sibling

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 90 / 195  (id=-693280389959675287)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #500

**Entities in chunk:**
  - African National Congress
  - Cape Corps
  - General Smuts
  - INDEX 166  (also: INDEX 166)
  - Kliptown
  - Laingsburg
  - South America  (also: South Africa)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my Uncle Ben"}
```

**CC quote:** `my Uncle Ben`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 91 / 195  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam
  - Adam H. G. Mahomed
  - Bombay University  (also: Bombay)
  - British Justice
  - Cape
  - Charles Simons
  - Gool  (also: Gool)
  - Hoosain Parker
  - J.M.H.  (also: J.M.H)
  - Mahomed
  - Molteno
  - Mr. Schreiner
  - National Union of South African Students  (also: NUSAS)
  - Parker  (also: Mr. Parker)
  - Richard Parker
  - S.Abed  (also: Abed)
  - Students Representative Council  (also: SRC)
  - Supreme Court
  - University of Natal
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "the Abed family"}
```

**CC quote:** `the Abed family`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "the Abed family" is mentioned in the quote and S.Abed (also: Abed) is a known person, we can infer that the narrator is referring to their own family. However, without more information about the specific relationships within the Abed family, we cannot determine any specific familial relationships.

Therefore, according to the rules, since both endpoints are not identifiable as canonical names with a clear relationship, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 92 / 195  (id=-509545831926091450)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #160

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Kadar
  - Amod Gool
  - Guy's Hospital  (also: Guys Hospital)
  - INDEX 166  (also: INDEX 166)
  - Lachiram
  - London
  - M Wilson  (also: Wilson)
  - Messrs.
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs. Wooding  (also: Wooding)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Rev Dan Wessels  (also: Dan Wessels)
  - Unity Movement  (also: Unity)
  - Van der Ross
  - Vassan Hindu C.C  (also: Vassan)
  - Veldsman
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Hamid Gool
  - Zaheer
  - Zeenith Ally  (also: Zeenith)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle Abdul Hamid Gool"}
```

**CC quote:** `my uncle Abdul Hamid Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 93 / 195  (id=-441481017488792687)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #862

**Entities in chunk:**
  - African Affairs
  - Anti-Apartheid
  - Gava Kadoes  (also: Gava)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Houses of Parliament
  - Purcell
  - Ralph J. Bunche  (also: Ralph Bunche)
  - South America  (also: South Africa)
  - Torch Commando  (also: Torch)
  - United Nations
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle Goolam Gool"}
```

**CC quote:** `my uncle Goolam Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 94 / 195  (id=-272192624856094471)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #256

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Caledon
  - City
  - Daddy pear  (also: Daddy)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Aboeta
  - Health Department
  - Main Road
  - Mount Streets  (also: Mount Street)
  - Rondebosch
  - Slum
  - Transkei
  - University of Cape Town  (also: University of Cape Town)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "Daddy’s boy;"}

This clause identifies a direct family relationship between two people: Yousuf Rassool (the narrator) and Daddy pear (also referred to as Daddy), where Yousuf is named after his father, Daddy pear.
```

**CC quote:** `Daddy’s boy;`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Daddy pear"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Daddy pear`

---

## Chunk 95 / 195  (id=-93354464964914605)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #72

**Entities in chunk:**
  - 5th Baron
  - Alice Greene
  - All India Football team
  - Buitencingle Street  (also: Buitencingle)
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - George Bernard Shaw
  - Grahame Greene
  - Indian Agents General
  - Kimberley
  - Lord Headley
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Moravian Church  (also: Moravian)
  - Oddfellows Hall
  - Prince of Wales  (also: Prince of Wales)
  - Rev. Wessels  (also: Wessels)
  - Sarojini Naidu
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  aunt 

**CC pass (raw):**
```json
{"quote": "the aunt of Grahame Greene"}
```

**CC quote:** `the aunt of Grahame Greene`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 96 / 195  (id=32633455946576169)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #319

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Abba
  - Brian Willan
  - Buitencingle Street  (also: Buitencingle)
  - Buitengracht Street
  - Bunny Kriekler
  - Burg Street
  - C.Khatieb  (also: Khatieb)
  - Caledon Square
  - Caledon Streets  (also: Caledon Street)
  - Cango Caves
  - Cannon Street
  - Canterbury Street
  - Cape Argus
  - Cape Colony
  - Cape Flats
  - Cape Peninisula Students Union
  - Cape Technical College
  - Cape Times
  - Cape Town
  - Cape Town City Orchestra
  - Cavalla
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Ou Bhai
  - Tagara
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle’s friend"}
```

**CC quote:** `my uncle’s friend`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 97 / 195  (id=256564604167810176)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #962

**Entities in chunk:**
  - Advocate Christopher
  - British Settlers  (also: British Settlers)
  - Chapel Street School  (also: Chapel Street School)
  - Huguenots  (also: Huguenots)
  - Indian Opinion
  - Indian Settlers
  - J. M. H. Gool
  - M Wilson  (also: Wilson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - New South Africa  (also: New South Africa)
  - Omar Khayyam
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 98 / 195  (id=501428138843038233)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #511

**Entities in chunk:**
  - Hassan Bavasah  (also: Hassan)
  - Morris
  - New Era Fellowship
  - Smuts's United Party  (also: Smuts)
  - Stalingrad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 99 / 195  (id=519016872720201104)

**Section:** Yousuf (Joe) Rassool   98  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #589

**Entities in chunk:**
  - Allies  (also: Allies)
  - Eastern Front
  - Germany  (also: Germany)
  - Hitler
  - INDEX 166  (also: INDEX 166)
  - Khadija Ebrahim  (also: Khadija)
  - Mecca
  - Myrtle  (also: Myrtle)
  - Therefore
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , brother

**CC pass (raw):**
```json
{"quote": "Khadija’s young brother"}
```

**CC quote:** `Khadija’s young brother`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Khadija Ebrahim","relation":"sibling_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Khadija Ebrahim` **sibling_of** `Yousuf Rassool`

---

## Chunk 100 / 195  (id=600429570805554003)

**Section:** Yousuf (Joe) Rassool   96  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #576

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - France  (also: France)
  - Kimberley Sanatorium
  - Messaris  (also: Mr. Messaris)
  - Shaheen Gool  (also: Shaheen)
  - Tuan Koebers
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 101 / 195  (id=615438439207399386)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #149

**Entities in chunk:**
  - Amra
  - Bertie Louw
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Communist Party
  - Hyde Park
  - Ismail
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Indian  (also: Natal Indian)
  - University of Cape Town  (also: University of Cape Town)
  - Workers' Party  (also: Workers Party)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 102 / 195  (id=628953725132054043)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #605

**Entities in chunk:**
  - Cape Technical College
  - Hanover Street
  - Messaris  (also: Mr. Messaris)
  - Trafalgar
  - Tubby
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** father,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Naz"}
```

**CC quote:** `my cousin Naz`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 103 / 195  (id=641330737843452673)

**Section:** Yousuf (Joe) Rassool   120  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #748

**Entities in chunk:**
  - Abe Desmore
  - David Poole
  - District Six  (also: District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Grand Parade
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Mrs. Wo  (also: Mrs Wo)
  - NEUM
  - Nerine Desmond
  - TARC
  - Trafalgar Junior School
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 104 / 195  (id=776867616384834388)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #603

**Entities in chunk:**
  - Adderley Street  (also: Adderley)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Esther Berelowitz
  - Inez Vera Du Preez
  - Mohamed Bagus Allie
  - Rag Magazine
  - Rag Queen
  - Sheila Sachs

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 105 / 195  (id=787736854281663344)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #140

**Entities in chunk:**
  - Asia
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Jack Meltzer
  - Law 3 of 1885
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mohammedan
  - Mr. Meltzer  (also: Meltzer)
  - South African Republic
  - Trafalgar High School  (also: Trafalgar High School)
  - Trotskyist
  - Turkish Empire
  - Unity Movement  (also: Unity)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 106 / 195  (id=819376064568638174)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #99

**Entities in chunk:**
  - Amina Minnie  (also: Amina)
  - Arriehai
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - College  (also: College)
  - E. Albertus  (also: Albertus)
  - Hamza
  - Head  (also: Head Mr)
  - Hoosain
  - National Union of South African Students  (also: NUSAS)
  - Parker  (also: Mr. Parker)
  - Principal
  - South African Students
  - Torch Commando  (also: Torch)
  - Windermere

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 107 / 195  (id=1153175004995048754)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #572

**Entities in chunk:**
  - Bartnicke  (also: Bartnicke)
  - Burg Street
  - Cape Argus
  - City
  - Eddieas
  - Geoff Abrahams  (also: Geoff)
  - INDEX 166  (also: INDEX 166)
  - Kader
  - Kassiem
  - Muir Street
  - Poppie
  - Ronnie Jooste  (also: Ronnie)
  - Shariefa Khan
  - Slingers  (also: Mr Slingers)
  - Springveld
  - Standard Seven  (also: Standard Seven)
  - Standard Six  (also: Standard Six)

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 108 / 195  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Barnato Union
  - Ben Malamba
  - Cape Town
  - Curry's Fountain
  - Even Buitencingle
  - Rashid Gardee
  - Salie Van Haacht
  - Woodstock Town Hall
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 109 / 195  (id=1340028930833342443)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #760

**Entities in chunk:**
  - Cape Argus
  - Daniel Francois Malan
  - E.A.
  - Habibia Institute  (also: Habibia Institute)
  - Head Student
  - Hussein Parker
  - INDEX 166  (also: INDEX 166)
  - Prime Minister
  - South America  (also: South Africa)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "Hussein Parker, son of E.A."}
```

**CC quote:** `Hussein Parker, son of E.A.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"E.A.","relation":"parent_of","to":"Hussein Parker"}]}
```

**Extracted relations:**
- `E.A.` **parent_of** `Hussein Parker`

---

## Chunk 110 / 195  (id=1457016325443525292)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #951

**Entities in chunk:**
  - Anti-CAD  (also: Anti-CAD)
  - Charles Starret
  - Club
  - Dale Evans
  - Even Buitencingle
  - INDEX 166  (also: INDEX 166)
  - Johnny Mack Brown
  - Merrem  (also: Merrem)
  - Movement
  - Prop Diamond  (also: Prop Diamond)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 111 / 195  (id=1515453391350648521)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #27

**Entities in chunk:**
  - Cape Colony
  - College  (also: College)
  - Coloured Advisory Council
  - England
  - George Golding  (also: Mr. George Golding)
  - Gool  (also: Gool)
  - Hamid
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hogwood  (also: Mr Hogwood)
  - Ismail Hayat
  - Jameel Abrahams  (also: Jameel)
  - M Wilson  (also: Wilson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Tuscany
  - Union of South African Students

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 112 / 195  (id=1582519559517716798)

**Section:** 59  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #311

**Entities in chunk:**
  - Banqueting Hall
  - Cape Town
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - City
  - Dr Du Plessis  (also: Du Plessis)
  - Even Buitencingle
  - Fish Market
  - Geoff Abrahams  (also: Geoff)
  - Globe Gang  (also: Globe)
  - Green Point Commons  (also: Green Point Common)
  - Institute of Race Relations
  - Nazism
  - South African Bureau of Racial Affairs
  - Standard Seven  (also: Standard Seven)
  - Star Bioscope  (also: Star Bioscope)
  - U.P.  (also: U.P)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 113 / 195  (id=1690720582947668658)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1139

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - African American community
  - Afro-Caribbean Community
  - Britain's Empire  (also: Britain)
  - Cape Town
  - Desmond Green
  - India
  - Israel
  - James M. Wilson
  - Journal of Southern African Studies
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Narutham Godse
  - Pakistan
  - South America  (also: South Africa)
  - Trinidad
  - USSR  (also: USSR)
  - United States  (also: United States)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "his wife"}
```

**CC quote:** `his wife`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his wife", we can infer that Yousuf Rassool is the husband of his wife. However, since the name of the wife is not mentioned in the provided text, we cannot extract a valid relation.

According to the rules, if both endpoints are not identifiable as canonical names, return empty.

Therefore, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 114 / 195  (id=1743770337024172354)

**Section:** Chapter Eighteen    Chapel Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #965

**Entities in chunk:**
  - Cape Education Department
  - Chapel Street School  (also: Chapel Street School)
  - Danny De Beer  (also: Danny)
  - Ethnic Pride
  - J.M.H.  (also: J.M.H)
  - James Currey  (also: James Curry)
  - M Wilson  (also: Wilson)
  - Maritzburg
  - Mohmet
  - National Union of South African Students  (also: NUSAS)
  - Racial Prejudice
  - Rev R.A. Jackson
  - Reverend Gow  (also: Rev. Gow)
  - Shaw's South African Christian Politcal Association (SACPA)  (also: Shaw)
  - South America  (also: South Africa)
  - Star Cinema  (also: Star Cinema)
  - Town Hall
  - Transvaal  (also: Transvaal)
  - United Party Member of Parliament
  - University of South Africa
  - Victorian Cape Town
  - Vivian Bickford-Smith
  - Wesleyan W.B.
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 115 / 195  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Abrahams brothers  (also: Abrahams)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Durban's Curry's Fountain  (also: Durban)
  - Group Areas Act
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Jameel Abrahams  (also: Jameel)
  - Mount Streets  (also: Mount Street)
  - Nellie Abdurahman
  - Omar Abrahams
  - Tiny Abed
  - Trafalgar Players
  - University of South Africa
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 116 / 195  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Argus Annual
  - Cape
  - Cape Town
  - Dullah Omar
  - Dullie Desai
  - Even Buitencingle
  - Good Hope
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - India
  - South African Directory
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Da Costa

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 117 / 195  (id=2083741976242093100)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #179

**Entities in chunk:**
  - B.M.  (also: B.M)
  - Bambatta  (also: Bambatta)
  - Constantia Road
  - Herrenvolk's Apartheid  (also: Herrenvolk)
  - Hymie Beimel  (also: Hymie)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Phillis Ntantala Jordan
  - Port Elizabeth
  - Pretoria Gandhi  (also: Pretoria)
  - Reddy
  - South America  (also: South Africa)
  - Tabata
  - Victor
  - W.P.  (also: W.P)
  - Wynberg
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 118 / 195  (id=2104188275539776644)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #206

**Entities in chunk:**
  - Cape Town
  - Cassiem  (also: Cassiem)
  - Ciraj Rassool
  - Fazil Rassool  (also: Fazil)
  - Johannesburg
  - MQotsi  (also: MQotsi)
  - Malick Rassool
  - Messaris  (also: Mr. Messaris)
  - Moses Kotane
  - Moslem C.C.
  - Motala  (also: Dr Motala)
  - Mouille Point
  - Mount Nelson Hotel  (also: Mount Nelson Hotel)
  - Mount Streets  (also: Mount Street)
  - Walcott
  - Weekes
  - Western Province Indian Cricket Union
  - Worrel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 119 / 195  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Communist Party
  - Group Areas  (also: Group Area)
  - Jaffeism
  - Kies  (also: Mr Kies)
  - Land Tenure Advisory Board
  - Liberals
  - Red China
  - Slingers  (also: Mr Slingers)
  - Tabata
  - Teachers League
  - United States  (also: United States)
  - Victor

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 120 / 195  (id=2157482743913839493)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #698

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Caledon Streets  (also: Caledon Street)
  - Congress
  - Gava Kadoes  (also: Gava)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hiema
  - Indian Congress
  - Mohamed Ali Jinnah
  - Mohammed Essop
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muir Street
  - Muslim League
  - Muslims C.C.  (also: Muslims)
  - Musto
  - Parade
  - Parow
  - Passive Resistance
  - Queens Road
  - Quit India
  - Viceroy
  - Woodstock
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 121 / 195  (id=2190457622431566710)

**Section:** Yousuf (Joe) Rassool   36  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #183

**Entities in chunk:**
  - George Meissenheimer  (also: George Meisenheimer)
  - INDEX 166  (also: INDEX 166)
  - Life
  - Majesty
  - Pretoria Gandhi  (also: Pretoria)
  - Trafalgar George
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "Gandhi's son Manilal"}
```

**CC quote:** `Gandhi's son Manilal`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Pretoria Gandhi","relation":"parent_of","to":"Manilal"}]}
```

**Extracted relations:** none

---

## Chunk 122 / 195  (id=2228032443713191255)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #55

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Town's most famous regiment
  - Chapel Street
  - City Hall
  - Dora Taylor
  - Eva Sachs
  - Fred Bodmer
  - General Smuts
  - Gregoire Boonzaaier
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Messaris  (also: Mr. Messaris)
  - Paul Kostin
  - Searle Street
  - South America  (also: South Africa)
  - Terence Macaw
  - Thence
  - Tobruk
  - Tykie Messaris  (also: Tykie)
  - Van Riebeeck Festival
  - Wolf Kiebel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 123 / 195  (id=2263436661745583446)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1024

**Entities in chunk:**
  - Grand Parade
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX 166)
  - Prop Diamond  (also: Prop Diamond)
  - Ronald Heinrichsen
  - South America  (also: South Africa)
  - Springboks
  - Stall number 12
  - Three W
  - Walcott
  - Weekes
  - West Indian Cricket
  - Worrel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 124 / 195  (id=2310160307929513794)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #254

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Douglas Mitchell
  - Edross  (also: Mr Edross)
  - Fazil Rassool  (also: Fazil)
  - Heneke  (also: Mr Heneke)
  - Maritzburg
  - Mohmet
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 125 / 195  (id=2736700062244384633)

**Section:** Yousuf (Joe) Rassool   54  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #277

**Entities in chunk:**
  - Abass
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Communist Party
  - Liberals
  - Magdalena Isabella
  - Malick Rassool
  - Muir Street
  - Noors  (also: Noors)
  - Peter Alexander
  - Vandeleur Street
  - Victor
  - Wesleyan Church
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 126 / 195  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - District Six  (also: District Six)
  - Doxology
  - Fazil Rassool  (also: Fazil)
  - Ismail
  - Kelvinator
  - Rasheda Rassool  (also: Rasheda)
  - Rev. Wessels  (also: Wessels)
  - Teachers League of South Africa  (also: TLSA)
  - Van Loon's The Arts of Mankind
  - Waxie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** husband

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 127 / 195  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Cape Argus
  - Ethnic Pride
  - Golding
  - Green Point Commons  (also: Green Point Common)
  - Harry Lawrence
  - Israel
  - Joh'burg
  - M Wilson  (also: Wilson)
  - Menachem Begin
  - New Era Fellowship
  - Pogrund  (also: Pogrund)
  - Progressive Party
  - Racial Prejudice
  - Rand Daily Mail
  - Regan
  - Reuben Pogrund
  - Reverend Gow  (also: Rev. Gow)
  - Van der Ross
  - Vivian Bickford-Smith
  - Wesleyan W.B.

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 128 / 195  (id=2975384613280766645)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #955

**Entities in chunk:**
  - Ben Malamba
  - Dr. Verwoerd
  - Eiselen
  - Heathrows C.C  (also: Heathrows C.C.)
  - INDEX 166  (also: INDEX 166)
  - Muslims C.C.  (also: Muslims)
  - Native Affairs
  - Paddy Thomas
  - Salie Van Haacht
  - Seventh Day Adventists
  - Stanley Abrahams
  - Teachers League of South Africa  (also: TLSA)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 129 / 195  (id=3029290522947892988)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #657

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle Street  (also: Buitencingle)
  - Cape
  - Cassim Amra
  - General Smuts
  - India
  - Johannesburg
  - Joss
  - League of Nations  (also: League)
  - Louis Botha
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - South African Party
  - United Party
  - University of Witwatersrand
  - Witwatersrand
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 130 / 195  (id=3036940243594659710)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #715

**Entities in chunk:**
  - Allie B.A.  (also: Allie BA)
  - Dollies
  - Habibia Institute  (also: Habibia Institute)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mecca
  - Michael Johns
  - Practical Science
  - Ta'Als  (also: Ta'Als)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 131 / 195  (id=3093788627293641191)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #339

**Entities in chunk:**
  - C.I.D
  - Caledon Streets  (also: Caledon Street)
  - Cannon Street
  - Cape Times
  - Cape Town
  - Coloured Advisory Council
  - District Six  (also: District Six)
  - George Golding  (also: Mr. George Golding)
  - Harold Wolpe
  - High School  (also: High School)
  - Hogwood  (also: Mr Hogwood)
  - Jackie Marks  (also: Jackie Marks)
  - Natal Daily Witness
  - Western Province Indian Cricket Union
  - Wits  (also: Wits)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 132 / 195  (id=3099837090720220907)

**Section:** Yousuf (Joe) Rassool   38  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #193

**Entities in chunk:**
  - Auchinlek
  - Cairo
  - Cape Argus
  - Churchill
  - Daniel Francois Malan
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX 166)
  - India
  - Local History Museum
  - Manilal
  - Phoenix
  - Ramdas
  - Ritchie
  - South America  (also: South Africa)
  - Tobruk
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Wavell

**Triggers found:** father, brother

**CC pass (raw):**
```json
{"quote": "Their brother Goolam stands to the right with Wahida in the background."}
```

**CC quote:** `Their brother Goolam stands to the right with Wahida in the background.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Goolam Gool","relation":"sibling_of","to":"Wahida Gool"}]}
```

**Extracted relations:**
- `Goolam Gool` **sibling_of** `Wahida Gool`

---

## Chunk 133 / 195  (id=3217057869876847730)

**Section:** Chapter Three  Declaration of War  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #364

**Entities in chunk:**
  - B.Sc
  - Blondie Shaik
  - Damoo Bansda
  - Farouk Du Preez
  - George Munsook
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Goolie
  - Hamid Khan
  - Hoosain Parker
  - Jackie Gihwala
  - Liesbeek River
  - Matric  (also: Matric)
  - Moorgas Naidoo
  - Mother Nature
  - Nasim Rassool  (also: Nasim)
  - Natal
  - Rashid Gardee
  - Salie Van Haacht
  - Shakham Osmany
  - Stephens
  - Tiny Abed
  - Van Haacht
  - Western Province  (also: Eastern Province)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "my baby brother"}
```

**CC quote:** `my baby brother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 134 / 195  (id=3336594518095080251)

**Section:** Yousuf (Joe) Rassool   64  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #349

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Choir
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
  - Ismail
  - Khalifa
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** daughter,  cousin

**CC pass (raw):**
```json
{"quote": "my pretty cousin, Zeenith, the green-eyed, curly-headed daughter of Hoosain Ally"}
```

**CC quote:** `my pretty cousin, Zeenith, the green-eyed, curly-headed daughter of Hoosain Ally`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 135 / 195  (id=3445648479411821512)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #628

**Entities in chunk:**
  - All AJ Abrahamse Lectures
  - BM.Kies  (also: BM Kies)
  - Babla Salloogee
  - Berlin
  - Cathedral Hall
  - Clara Petacci
  - District Six  (also: District Six)
  - Germans  (also: Germans)
  - Hitler
  - INDEX 166  (also: INDEX 166)
  - John Vorster Prison
  - Late
  - Luneberg Heath
  - Milan
  - Mussolini
  - Non Europeans
  - Pretoria Gandhi  (also: Pretoria)
  - Queen Victoria Street
  - Red Army
  - Socialist Soviet Republics
  - South America  (also: South Africa)
  - Union
  - Unity Movement  (also: Unity)
  - Victor
  - World Civilization
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Youth League

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 136 / 195  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Ben Malamba
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
  - Heathrow Cricket Club
  - INDEX 166  (also: INDEX 166)
  - Indian Cricket Union
  - Salie Van Haacht
  - Seventh Day Adventists
  - Stanley Abrahams
  - Students Representative Council  (also: SRC)
  - Sunday League  (also: Sunday)
  - Wagieda
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 137 / 195  (id=3610799175460804304)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #617

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Banqueting Hall
  - Beat
  - Bombay University  (also: Bombay)
  - Buitencingle Street  (also: Buitencingle)
  - District Six  (also: District Six)
  - Dr Kolia
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - G.H.
  - Gandhi's Son Manilal
  - INDEX 166  (also: INDEX 166)
  - Kloof Street  (also: Loop Street)
  - Mafeking
  - Manilal Gandhi
  - Manilal Gandhi's Prisoner
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Uma Dhupelia-Mesthrie  (also: Uma Dhupelia-Mesthri)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** married,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt"}
```

**CC quote:** `my aunt`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 138 / 195  (id=3654419630807047063)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1056

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Chapel Street
  - Ebrahim Ali
  - Eustace Humphrey Moses
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX 166)
  - Mauritius
  - Michael Johns
  - Mr. Meltzer  (also: Meltzer)
  - Natal
  - Trafalgar
  - Trafalgar High School  (also: Trafalgar High School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 139 / 195  (id=3667304071003893536)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #246

**Entities in chunk:**
  - Emperor Hirohito
  - Europe
  - INDEX 166  (also: INDEX 166)
  - Morris
  - Nationalist Senators
  - Nationalists Party  (also: Nationalist Party)
  - Parliament  (also: Parliament)
  - South America  (also: South Africa)
  - Stephen Spender
  - United Party
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 140 / 195  (id=3683676736687558285)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #789

**Entities in chunk:**
  - Allies  (also: Allies)
  - Generals Montgomery
  - George Golding  (also: Mr. George Golding)
  - Hassiem Edross
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Rev. Gow)
  - Salie Dollie
  - Solly Edross
  - Von Kesselring
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "He was related to Solly Edross"}
```

**CC quote:** `He was related to Solly Edross`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 141 / 195  (id=3694579935737916495)

**Section:** Chapter Three  Declaration of War  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #367

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape
  - Cape Town Woodstock
  - District Six  (also: District Six)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Leather
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - Somehow
  - Teachers League of South Africa  (also: TLSA)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 142 / 195  (id=3698504895570842680)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #257

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Beimel
  - Buitencingle Street  (also: Buitencingle)
  - Communist Party
  - Forum Club
  - Goebbels
  - INDEX 166  (also: INDEX 166)
  - Kenny Jordaan
  - Liberals
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Native Representatives
  - Parliament  (also: Parliament)
  - Red China
  - TARC
  - United States  (also: United States)
  - Unity Movement  (also: Unity)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, brother

**CC pass (raw):**
```json
{"quote": "his brothers"}
```

**CC quote:** `his brothers`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his brothers", we can infer that Yousuf Rassool has brothers, but we don't know their names. However, since the narrator refers to himself as "I", "me", or "my", we can use his name as the "from" endpoint.

But in this case, we cannot identify the "to" endpoint as a canonical name. Therefore, according to the rules, we should return an empty list of relations.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 143 / 195  (id=3707629011541248812)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #29

**Entities in chunk:**
  - Cape Town
  - Chapel Street Primary  (also: Chapel Street Primary)
  - District Six  (also: District Six)
  - England
  - Gandhi's Indian Opinion
  - Habibia Institute  (also: Habibia Institute)
  - Herrenvolk's Apartheid  (also: Herrenvolk)
  - Port Elizabeth
  - Teachers League Conference
  - Valley Awakes
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 144 / 195  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Benito Mussolini
  - Edross  (also: Mr Edross)
  - Even
  - Germans  (also: Germans)
  - Heneke  (also: Mr Heneke)
  - INDEX 166  (also: INDEX 166)
  - James Shirley
  - Moscow
  - Nasim Rassool  (also: Nasim)
  - Pearl Harbour
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "my beautiful, cherubic little brother now stood there"}
```

**CC quote:** `my beautiful, cherubic little brother now stood there`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 145 / 195  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town Docks
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jadwat
  - King-Emperor
  - M Wilson  (also: Wilson)
  - Manilal
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris
  - Party of South Africa  (also: Party)
  - Port Elizabeth Indians
  - Ruth Alexander
  - Seedat
  - Unity Movement  (also: Unity)
  - Wright
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 146 / 195  (id=4201484364537076764)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #188

**Entities in chunk:**
  - Cape Town
  - Edgar Maurice
  - Empire Day  (also: Empire)
  - Gool  (also: Gool)
  - Heinemann When
  - INDEX 166  (also: INDEX 166)
  - J.Boyce
  - Mikhail Bakhtin
  - Plaatje  (also: Plaatje)
  - SOL PLAATJE Brian Willan
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA)
  - Train Apartheid
  - West Indian
  - Williams
  - Wooding's Preparatory
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 147 / 195  (id=4713749386019632490)

**Section:** Chapter Seven    War Rages  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #485

**Entities in chunk:**
  - Abba
  - Caledon Streets  (also: Caledon Street)
  - Cassiem  (also: Cassiem)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Darling Street
  - Habibia  (also: Habibia)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Marks's furniture shop  (also: Marks)
  - Mount Streets  (also: Mount Street)
  - Nellie Abdurahman
  - Ou Bhai
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 148 / 195  (id=4812989679158280412)

**Section:** 59  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #310

**Entities in chunk:**
  - Banqueting Hall
  - Clifton Street  (also: Clifton)
  - Dale Evans
  - District Six  (also: District Six)
  - Dr Kolia
  - England
  - Hanover
  - Mr Pearce  (also: Pearce)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - South America  (also: South Africa)
  - Star Bioscope  (also: Star Bioscope)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 149 / 195  (id=4888467865479780033)

**Section:** Chapter Four  White Flight  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #393

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ben Kies  (also: Ben Kies M.A)
  - Bunny Kriekler
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - District Six  (also: District Six)
  - Earnest Livingston MQotsi
  - Fazil Rassool  (also: Fazil)
  - Fort Hare  (also: Fort Hare)
  - Gerald Newman
  - Grand Parade
  - Harold Wolpe
  - Helen Abrahams
  - INDEX 166  (also: INDEX 166)
  - John Clayton
  - Moscow
  - Oliver Caldecott
  - P.V. Tobias
  - Shawquet
  - Sydney Brenner
  - Walmer Estate
  - Yasmin Rassool  (also: Yasmin, Jessie, Jessie Rassool)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "my sister, Yasmin"}
```

**CC quote:** `my sister, Yasmin`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Yasmin Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Yasmin Rassool`

---

## Chunk 150 / 195  (id=5096918597656922986)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #604

**Entities in chunk:**
  - Cape Technical College
  - Chapel Street School  (also: Chapel Street School)
  - Chota
  - Darling Street
  - Gadija (Auntie Dij)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Habibia  (also: Habibia)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Mohamed Bagus Allie
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Rag Magazine
  - Sayed Fethi
  - Standard Six  (also: Standard Six)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Trafalgar
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed ,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 151 / 195  (id=5207007205339208384)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #957

**Entities in chunk:**
  - Arabic Surah
  - Broach
  - Cape Town-Woodstock P.T.A.
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Kelvinator
  - Kies  (also: Mr Kies)
  - League of Nations  (also: League)
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - Rasheda Rassool  (also: Rasheda)
  - Surat
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Zain Rassool  (also: Zain)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 152 / 195  (id=5228882496868360423)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1060

**Entities in chunk:**
  - Communist Party
  - David Bloomberg
  - Department of Education  (also: Department)
  - District 6 Ward
  - Government
  - INDEX 166  (also: INDEX 166)
  - Mister Mogamat Regal
  - Moravian Church  (also: Moravian)
  - Queens Rd
  - Rev. Wessels  (also: Wessels)
  - Reza Rassool  (also: Reza)
  - Roland Colman
  - Sam Wo's Laundry  (also: Sam Wo's Laundry)
  - United Party
  - Unity Movement  (also: Unity)
  - Victor Wessels

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 153 / 195  (id=5499390051200912173)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #104

**Entities in chunk:**
  - Andaman Islands
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Burma
  - Caledon Streets  (also: Caledon Street)
  - Clifton Street  (also: Clifton)
  - Corporation Street
  - District Six  (also: District Six)
  - Emperor Hirohito
  - Far East
  - Imperial Japanese
  - India
  - Java
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Phillipines
  - Prince of Wales  (also: Prince of Wales)
  - Rander
  - Sayed Fethi
  - Sayed Hussain Ally
  - Singapore
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, married, father, mother, sister, niece, nephew

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 154 / 195  (id=5528078340237292279)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #245

**Entities in chunk:**
  - Caledon
  - Europe
  - Health Department
  - Hitler
  - INDEX 166  (also: INDEX 166)
  - Morris
  - Mount Streets  (also: Mount Street)
  - South America  (also: South Africa)
  - Transkei
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 155 / 195  (id=5781509709031527554)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #866

**Entities in chunk:**
  - Arthur Tracey
  - Deanna Durbin
  - George Manuel
  - Hutch
  - INDEX 166  (also: INDEX 166)
  - Messaris  (also: Mr. Messaris)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nehru
  - South America  (also: South Africa)
  - University
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "My uncle"}
```

**CC quote:** `My uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 156 / 195  (id=5896192977959377292)

**Section:** 61  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #330

**Entities in chunk:**
  - INDEX 166  (also: INDEX 166)
  - Islamic
  - Ivan Denisovich
  - Mecca
  - National Anti-CAD
  - Sunday League  (also: Sunday)
  - Synagogue
  - Vandeleur Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 157 / 195  (id=6014117459201014898)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #514

**Entities in chunk:**
  - Boycott
  - Damoo Bansda
  - Farouk Du Preez
  - George Munsook
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Goolie
  - Hamid Khan
  - Hoosain Parker
  - Jackie Gihwala
  - Junior Certificate
  - Liesbeek River
  - Moorgas Naidoo
  - Natal
  - Rashid Gardee
  - Ronnie Jooste  (also: Ronnie)
  - Salie Van Haacht
  - Shakham Osmany
  - South America  (also: South Africa)
  - Stephens
  - Tiny Abed
  - Van Haacht
  - Western Province  (also: Eastern Province)

**Triggers found:** wife, daughter, sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 158 / 195  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - D. Neethling  (also: Neethling)
  - Durban's Curry's Fountain  (also: Durban)
  - Fataar
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Kies  (also: Mr Kies)
  - Meissenheimer  (also: Mrs Meissenheimer)
  - Mr Jaffe  (also: Jaffe)
  - Natal
  - Rev. Wessels  (also: Wessels)
  - Sayed Kader Sayed Noor There
  - Tabata
  - Transvaal  (also: Transvaal)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 159 / 195  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Collected Works  (also: Collected Work)
  - Harold Kruger
  - Hewat  (also: Hewat)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - James Felix
  - John Smith
  - Joseph Bredenkamp
  - M Wilson  (also: Wilson)
  - Mahatma Gandhi Vol4 Gool
  - New South Africa  (also: New South Africa)
  - Ronald Heinrichsen

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "the bond between Dr. Abdurahman and my grandfather"}
```

**CC quote:** `the bond between Dr. Abdurahman and my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 160 / 195  (id=6105164271277742615)

**Section:** 57  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #292

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cecil John Rhodes
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Christian National Movement
  - Habibia Institute  (also: Habibia Institute)
  - Hassen Abrahams
  - INDEX 166  (also: INDEX 166)
  - Nationalist government
  - Teachers League of South Africa  (also: TLSA)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 161 / 195  (id=6295959623867363577)

**Section:** Chapter Seventeen  New Era Fellowship  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #933

**Entities in chunk:**
  - B.Sc
  - Caledon Streets  (also: Caledon Street)
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Congress
  - Eiselen Commission
  - Hassen Abrahams
  - INDEX 166  (also: INDEX 166)
  - Le Grange
  - M.B.Ch.B
  - Main Road
  - Mehuyzen
  - Oosthuisen  (also: Oosthuisen)
  - Rev. Wessels  (also: Wessels)
  - Rondebosch
  - Rousseau
  - Teachers League of South Africa  (also: TLSA)
  - University of Cape Town  (also: University of Cape Town)
  - Van Der Merwe
  - Van der Westhuisen
  - Viljoen
  - Vollenhoven

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 162 / 195  (id=6317719585632729785)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #398

**Entities in chunk:**
  - Bagus Allie  (also: Bagus-Allie)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Davids
  - Durban's Curry's Fountain  (also: Durban)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Ismail
  - Khadija Ebrahim  (also: Khadija)
  - Malaai
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Persotem Patel  (also: Persotem)
  - Rashid
  - Sarlegh Doalie
  - Shawquet
  - Suleiman Vallie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** father,  aunt , in-law

**CC pass (raw):**
```json
{"quote": "her father-in-law, old Mr. Davids"}
```

**CC quote:** `her father-in-law, old Mr. Davids`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 163 / 195  (id=6339413612312511153)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #655

**Entities in chunk:**
  - Britain's Empire  (also: Britain)
  - Darling St
  - Durban's Curry's Fountain  (also: Durban)
  - France  (also: France)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Henry Newbolt Though
  - INDEX 166  (also: INDEX 166)
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Narutham Godse
  - Palestinian people  (also: Palestinian)
  - State of Israel  (also: State of Israel)
  - USSR  (also: USSR)
  - United States  (also: United States)
  - Vitaï Lampada
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed ,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 164 / 195  (id=6468864461399055326)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #625

**Entities in chunk:**
  - De Waals Drive  (also: De Waal Drive)
  - E. Albertus  (also: Albertus)
  - Government Avenue
  - Grand Parade
  - Green Point Commons  (also: Green Point Common)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hoosain
  - INDEX 166  (also: INDEX 166)
  - Jane
  - Main Road
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Parker  (also: Mr. Parker)
  - Rev Gordon  (also: Gordon)
  - Reverend Gow  (also: Rev. Gow)
  - Varsity

**Triggers found:** wed , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 165 / 195  (id=6546657237567362266)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #374

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baku
  - Batum
  - Cape to Cairo  (also: Cape to Cairo)
  - Caucasus
  - Cecil John Rhodes
  - Crimean
  - CtoC  (also: CtoC)
  - District Six  (also: District Six)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - Moscow
  - Sebastopol
  - Stalingrad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, parent,  uncle 

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 166 / 195  (id=6557428703332889295)

**Section:** Chapter Eight    Anti-CAD  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #520

**Entities in chunk:**
  - Amazon Kindle Direct Publishing
  - Britain's Empire  (also: Britain)
  - Cape Town
  - Ciraj Rassool
  - District Six  (also: District Six)
  - District Six Museum
  - Kwaai Oak
  - Marcina Kagan
  - Nasim Moosa
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Naz Gool-Ebrahim
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Rebecca Rassool
  - Reddy
  - Ruth Rassool
  - South America  (also: South Africa)
  - Uma Dhupelia-Mesthrie  (also: Uma Dhupelia-Mesthri)
  - Univeristy of Western Cape
  - Western Cape Archive and Records Service
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zenariah Barends

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 167 / 195  (id=6610559471948150059)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #950

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Dr Abdul Hamid Gool  (also: Dr. Abdul Hamid Gool)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Goofy Timol
  - INDEX 166  (also: INDEX 166)
  - James Africa
  - Keith Miller
  - Shorty Docrat
  - Slamat
  - Woody Woodman  (also: Woody)
  - Young Doctor Bhorat
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 168 / 195  (id=6614900686580700622)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #888

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Andrew Mackrill
  - Cape Town
  - Colin Wynne
  - David Poole
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - General
  - George Veldsman
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hewat Amateur Theatricals
  - INDEX 166  (also: INDEX 166)
  - Indian Congress
  - Johaar Mosaval  (also: Johaar Mosavel)
  - McDonald Dixon
  - Paul Roubaix
  - Pfaff  (also: Pfaff)
  - President Harry Truman
  - Sadlers Wells Ballet
  - Smuts's United Party  (also: Smuts)
  - Teachers League of South Africa  (also: TLSA)
  - Torch Commando  (also: Torch)
  - Woodstock Town Hall

**Triggers found:** wed , parent, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 169 / 195  (id=6617129456286239306)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #479

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Belgium
  - Britain's Empire  (also: Britain)
  - Cecil Wightman
  - Denmark
  - France  (also: France)
  - Hitler
  - Holland
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mussolini
  - Natal
  - Norway
  - Royal
  - Snoek Town Calling
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband,  uncle 

**CC pass (raw):**
```json
{"quote": "Uncle Mamoo, Auntie Jolly’s husband"}
```

**CC quote:** `Uncle Mamoo, Auntie Jolly’s husband`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "Uncle Mamoo", we can infer that Uncle Mamoo is Yousuf's uncle. However, since "uncle" is not a valid relation type in the schema, according to the critical rules, we should return an empty array.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 170 / 195  (id=6622459745407552529)

**Section:** Yousuf (Joe) Rassool   18  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #120

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies  (also: Ben Kies M.A)
  - Bethune  (also: Bethune)
  - Cape Town
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Cronin  (also: Cronin)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
  - King Lear
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Sayed Noor Chota
  - Solly Edross
  - South America  (also: South Africa)

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 171 / 195  (id=6941537010882242866)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #80

**Entities in chunk:**
  - Eddieas
  - Field Marshal Tito
  - Hamid
  - INDEX 166  (also: INDEX 166)
  - Kader
  - M Wilson  (also: Wilson)
  - Muir Street
  - Nasima
  - Poppie
  - Shariefa Khan
  - Standard Seven  (also: Standard Seven)
  - Standard Six  (also: Standard Six)
  - Strand
  - USSR  (also: USSR)
  - Unity Movement  (also: Unity)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yugoslavia

**Triggers found:** father,  cousin

**CC pass (raw):**
```json
{"quote": "my Auntie Fatima"}
```

**CC quote:** `my Auntie Fatima`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 172 / 195  (id=6966615310251134524)

**Section:** Yousuf (Joe) Rassool   28  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #162

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - England
  - Fazil Rassool  (also: Fazil)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - London
  - Lord Elgin
  - Meneer
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Movement
  - SOYA
  - Salie Van Haacht
  - Transvaal Indians
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle"}
```

**CC quote:** `my uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 173 / 195  (id=7168244047913479805)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1141

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Dr AH Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Michael Johns
  - Ralph J. Bunche  (also: Ralph Bunche)
  - Robert R. Edgar
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, brother

**CC pass (raw):**
```json
{"quote": "his wife Cissie Gool"}
```

**CC quote:** `his wife Cissie Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Cissie Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Cissie Gool`

---

## Chunk 174 / 195  (id=7262414809749442984)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #699

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Alan Melville
  - Athol Rowan
  - Bruce Mitchell
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX 166)
  - India
  - Indian Congress
  - Ken Viljoen
  - Mohamed Ali Jinnah
  - Muir Street
  - Muslim League
  - Paul Winslow
  - Searle Street
  - Springbok
  - Trafalgar High School  (also: Trafalgar High School)
  - Wale Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 175 / 195  (id=7266467364867027437)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #107

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Cape Town
  - District Six  (also: District Six)
  - John Phillips  (also: Rev John Phillips)
  - Mauritius
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Nasima
  - Raza
  - Reagon
  - Reddy
  - Reform Street
  - Regan
  - Reginald Lesch
  - Reuters
  - Rev Gordon  (also: Gordon)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** daughter, father,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt Jane Gool-Tabata (Wahida’s daughter)"}
```

**CC quote:** `my aunt Jane Gool-Tabata (Wahida’s daughter)`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. 

The quote states that Jane Gool-Tabata is Yousuf's aunt, but since 'aunt' is not in the schema, we return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 176 / 195  (id=7360610042715665275)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #158

**Entities in chunk:**
  - All African Convention
  - Ally
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bloemfontein
  - British Government
  - Cape Town
  - INDEX 166  (also: INDEX 166)
  - Johannesburg
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - New Era Fellowship
  - Urdu
  - Vic Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 177 / 195  (id=7448875471219160758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #480

**Entities in chunk:**
  - Cecil Wightman
  - Chaplin
  - Coloured Advisory Council
  - E. Albertus  (also: Albertus)
  - Habibia  (also: Habibia)
  - Hitler
  - INDEX 166  (also: INDEX 166)
  - Mussolini
  - Nasim Rassool  (also: Nasim)
  - Torch Commando  (also: Torch)
  - Trafalgar High  (also: Trafalgar High)
  - Yasmin Rassool  (also: Yasmin, Jessie, Jessie Rassool)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, sister, brother

**CC pass (raw):**
```json
{"quote": "my baby brother Nasim"}
```

**CC quote:** `my baby brother Nasim`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 178 / 195  (id=7561700546005880907)

**Section:** 47  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #234

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Town
  - Gihwala
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX 166)
  - Russian Whaler Ob  (also: Russian Whaler Ob)
  - Tyger Valley
  - Tyne and Roger Streets  (also: Tyne)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle"}
```

**CC quote:** `my uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 179 / 195  (id=7562115678963637038)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #184

**Entities in chunk:**
  - African National Congress
  - Axis Powers
  - Beveridge Plan
  - Britain's Empire  (also: Britain)
  - Caledon Streets  (also: Caledon Street)
  - Churchill
  - Civil Service
  - Clifton Street  (also: Clifton)
  - Corporation Street
  - Democracy
  - District Six  (also: District Six)
  - INDEX 166  (also: INDEX 166)
  - Majesty
  - Sol Plaatjes
  - South African Native National Congress
  - Welfare State
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 180 / 195  (id=7574535754100824916)

**Section:** Yousuf (Joe) Rassool   132  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #838

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Fabians Kies  (also: Fabians)
  - Howard Fast
  - Huguenots  (also: Huguenots)
  - Hussein Parker
  - Hutch
  - Hyde Park
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Hymie Beimel  (also: Hymie)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - I.Begg  (also: Begg)
  - Ignazio Silone
  - Immigration Act
  - India
  - Jeddah
  - Kies  (also: Mr Kies)
  - Qudrat
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 181 / 195  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Grand Parade
  - Helen Abrahams
  - Mauritius
  - Nasima
  - Oswald Pirow
  - Walmer Estate
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, brother

**CC pass (raw):**
```json
{"quote": "my grandpa was sixteen he went to join his brother, Abbas"}
```

**CC quote:** `my grandpa was sixteen he went to join his brother, Abbas`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Abbas Dinath"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Abbas Dinath`

---

## Chunk 182 / 195  (id=7727744974142389137)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #884

**Entities in chunk:**
  - Abed family
  - Burger
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town
  - Durban's Curry's Fountain  (also: Durban)
  - E. Albertus  (also: Albertus)
  - England
  - Head  (also: Head Mr)
  - Khadija Ebrahim  (also: Khadija)
  - Kloof Nek
  - Parker  (also: Mr. Parker)
  - Persotem Patel  (also: Persotem)
  - Tiny
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 183 / 195  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Chemistry Physics  (also: Chemistry)
  - Cyril
  - Dawood Seedat  (also: Dawood)
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Seedat
  - INDEX 166  (also: INDEX 166)
  - Professor James  (also: James)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Fatima Seedat"}
```

**CC quote:** `my cousin Fatima Seedat`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 184 / 195  (id=7794589649810439109)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #116

**Entities in chunk:**
  - Afrikaans Language Movement  (also: Afrikaans)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town Fire Brigade
  - Castle Street
  - Grand Parade
  - INDEX 166  (also: INDEX 166)
  - Merwe
  - Mr. B. Allie  (also: B. Allie)
  - Rutger Street
  - Wholesale Store
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** son , mother, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "my Aunt Jolly who was actually named Zohra"}
```

**CC quote:** `my Aunt Jolly who was actually named Zohra`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 185 / 195  (id=7818984028335812846)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #958

**Entities in chunk:**
  - Ben Cloete
  - Cape Town-Woodstock P.T.A.
  - Congress Movement
  - District Six  (also: District Six)
  - Heneke  (also: Mr Heneke)
  - Hewat Training College  (also: Hewat Training College)
  - INDEX 166  (also: INDEX 166)
  - Liberation League
  - Manuel
  - Trade Unions
  - Trafalgar Head
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 186 / 195  (id=7992762871094121264)

**Section:** Chapter Twelve    Cricket  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #661

**Entities in chunk:**
  - Axis Powers
  - Beveridge Plan
  - Britain's Empire  (also: Britain)
  - Churchill
  - Democracy
  - District Six  (also: District Six)
  - Franchise Act
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hewat Training College  (also: Hewat Training College)
  - INDEX 166  (also: INDEX 166)
  - Primary School  (also: Primary)
  - Welfare State
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 187 / 195  (id=8003152637512369339)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #570

**Entities in chunk:**
  - Bonny-Bridge Dover Stove
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Domingo  (also: Mrs Domingo)
  - Khadija Ebrahim  (also: Khadija)
  - Kloof Nek
  - Persotem Patel  (also: Persotem)
  - South African Public Library
  - Vandeleur Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 188 / 195  (id=8034240727763764335)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #79

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Egypt
  - Jack Meltzer
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mr. Meltzer  (also: Meltzer)
  - TLSA Journal
  - Trafalgar High School  (also: Trafalgar High School)
  - Unity Movement  (also: Unity)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Wahida's nieces
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, niece

**CC pass (raw):**
```json
{"quote": "his two sons, Abdul Hamid and Mohammed Hanief"}
```

**CC quote:** `his two sons, Abdul Hamid and Mohammed Hanief`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Hamid Gool","relation":"child_of","to":"Yousuf Rassool"},{"from":"Mohammed Hanief Gool","relation":"child_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Abdul Hamid Gool` **child_of** `Yousuf Rassool`
- `Mohammed Hanief Gool` **child_of** `Yousuf Rassool`

---

## Chunk 189 / 195  (id=8317664100809929487)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #911

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cape Town
  - District Six  (also: District Six)
  - Dr. DuPlessis  (also: DuPlessis)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hoosain
  - Indian Opinion Dec
  - National Gandhi Museum District
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Parker  (also: Mr. Parker)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "his son Hoosain"}
```

**CC quote:** `his son Hoosain`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Hooosain"}]}
```

**Extracted relations:** none

---

## Chunk 190 / 195  (id=8675141585945090217)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #883

**Entities in chunk:**
  - Abdulla Mohamed
  - Abdulla Parker
  - Abou Desai  (also: Abou)
  - Ahmed Salasa
  - Alex Penkin
  - Caledon Streets  (also: Caledon Street)
  - Castle Bridge
  - Department of Education  (also: Department)
  - District Six Tomorrow
  - Fresh Fields
  - Ghulzar Khan  (also: Gulzar Khan)
  - Hanover Street
  - Hayat brothers
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - Menage Rassool
  - Mohamed Giri
  - Moorgas Naidoo
  - Pastures New Lycidas
  - South America  (also: South Africa)
  - Tiny Abed

**Triggers found:** parent, brother

**CC pass (raw):**
```json
{"quote": "the Hayat brothers"}
```

**CC quote:** `the Hayat brothers`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Hayat brothers","relation":"siblings_of","to":"Hayat brothers"}]}
```

**Extracted relations:** none

---

## Chunk 191 / 195  (id=8719056895392496748)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #195

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Aligarh College  (also: Aligarh)
  - Athol Rowan
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bruce Mitchell
  - Chapel Street
  - Fort Hare  (also: Fort Hare)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ken Viljoen
  - Khadija Ebrahim  (also: Khadija)
  - Melville
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Mrs Ahmed  (also: Ahmed)
  - Paul Winslow
  - Persotem Patel  (also: Persotem)
  - South America  (also: South Africa)
  - Trafalgar High Persotem
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , son , daughter, father

**CC pass (raw):**
```json
{"quote": "his son BG"}
```

**CC quote:** `his son BG`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Hamid Gool","relation":"parent_of","to":"BG"}]}
```

**Extracted relations:** none

---

## Chunk 192 / 195  (id=8730565953127446314)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #997

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Britain's Empire  (also: Britain)
  - Empire Day  (also: Empire)
  - England
  - Germany  (also: Germany)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Movement
  - Mrs Ahmed  (also: Ahmed)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 193 / 195  (id=8910810193593885847)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #606

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - INDEX 166  (also: INDEX 166)
  - Khan's cafe  (also: Khan)
  - Lalla's Cafe  (also: Lalla's Cafe)
  - Millards
  - Professor James  (also: James)
  - Tubby
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 194 / 195  (id=9062310575080274351)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #225

**Entities in chunk:**
  - Agent General  (also: Agent General)
  - Athlone Branch of the TLSA
  - Buitencingle Street  (also: Buitencingle)
  - Durban's Curry's Fountain  (also: Durban)
  - Government of India Representatives
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - India
  - Indian Congress
  - James Shirley
  - Kimberley
  - League of Nations  (also: League)
  - Parvati Sammy
  - Port Elizabeth
  - Western Province Indian Cricket Union
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, married,  aunt 

**CC pass (raw):**
```json
{"quote": "the Agent-General already had another wife, a Kimberley lady, Parvati Sammy"}
```

**CC quote:** `the Agent-General already had another wife, a Kimberley lady, Parvati Sammy`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Agent General","relation":"spouse_of","to":"Parvati Sammy"}]}
```

**Extracted relations:**
- `Agent General` **spouse_of** `Parvati Sammy`

---

## Chunk 195 / 195  (id=9200706703269745412)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #202

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Ahmed Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bonny Bridge Dover
  - Buitengracht Street
  - Fort Hare  (also: Fort Hare)
  - G.H.
  - Harold Cressy
  - Muir Street
  - Muir Street Moslem School
  - Noors  (also: Noors)
  - St Paul's School
  - Wesley Training School
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** wed , father, mother,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 195 |
| Relations extracted | 31 |
| Relations written to graph | 31 |
