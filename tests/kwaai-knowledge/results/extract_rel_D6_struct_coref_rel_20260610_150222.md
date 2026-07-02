# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 286  
**Sampled:** 286  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 286  (id=-9123613270288747913)

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

## Chunk 2 / 286  (id=-9102931590403333452)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #42

**Entities in chunk:**
  - German Lutheran Church  (also: German Lutheran Church)
  - INDEX 166  (also: INDEX 166)
  - Islamic Synagogue  (also: Islamic)
  - Nasima M.S  (also: Nasima)
  - No.7 Buitencingle Street
  - Sonny Abdurahman
  - Students Representative Council  (also: SRC)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "My grandfather’s house"}
```

**CC quote:** `My grandfather’s house`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 3 / 286  (id=-9084456336763380431)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #112

**Entities in chunk:**
  - 7 Buitencingle Street  (also: Buitencingle Street, the Buitencingle mansion, 7 Buitenkingle Street)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Beimel
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Canterbury Street
  - Cape Town University  (also: Cape Town University)
  - New Era Fellowship (NEF)  (also: New Era Fellowship NEF)
  - Stakesby-Lewis Hostel  (also: Stakesby-Lewis Hostel)
  - Unity Movement  (also: Unity)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:** wed , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 4 / 286  (id=-9054503800870241600)

**Section:** Chapter Twenty    Split in the Movement  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1017

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - District Six  (also: District 6, the District, District Six)
  - Fazil Rassool  (also: Fazil)
  - Ismail  (also: Ismail)
  - Kelvinator
  - Kies  (also: Mr Kies)
  - League of Nations  (also: League)
  - Parent-Teacher Associations  (also: Parent Teachers Association)
  - Rasheda Rassool  (also: Rasheda)
  - Van Loon
  - Waxie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** mother, sister, brother

**CC pass (raw):**
```json
{"quote": "my younger brother Fazil"}
```

**CC quote:** `my younger brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 5 / 286  (id=-9026931979920257841)

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
  - Indian Congress
  - M. Allie  (also: Allie)
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
Since the narrator is 'my' and refers to their dad and aunt, we can infer that the narrator's name is not explicitly mentioned. However, based on the context of the quote, we can assume that the narrator is referring to themselves as a child of their dad.

Given this information, we cannot directly extract any family relationships between two canonical names from the provided list. The quote only mentions the relationship between the narrator and their dad/aunt, but does not provide enough information to link them to any specific canonical names.

Therefore, according to the rules, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 6 / 286  (id=-8654820806022910221)

**Section:** Yousuf (Joe) Rassool   28  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #161

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Caledon Streets  (also: Caledon Street)
  - Guy's Hospital  (also: Guys Hospital)
  - INDEX 166  (also: INDEX 166)
  - Kimberley Hospital  (also: Kimberley Hospital)
  - London
  - Vicks
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

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

## Chunk 7 / 286  (id=-8601693670707718054)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #201

**Entities in chunk:**
  - Broach
  - Buitencingle  (also: Buitencingle)
  - Calcutta
  - District Six  (also: District 6, the District, District Six)
  - Harold Cressy
  - Hewat College Motto
  - Hewat Training College  (also: Hewat Training College)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Primary School  (also: Primary)
  - Rander
  - Roeland Street Gaol
  - South America  (also: South Africa)
  - St Martini German Lutheran Church  (also: St Martini German Lutheran Church)
  - Surat
  - Trafalgar High  (also: Trafalgar High)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "Her son, Mohammed Saaid (Uncle Aity to us)"}
```

**CC quote:** `Her son, Mohammed Saaid (Uncle Aity to us)`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote uses 'Uncle', we need to follow the rules.

According to the rules, if the quote uses 'aunt', 'uncle', 'nephew', 'niece', or 'cousin', return {"relations":[]} — these relation types are not in the schema

So, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 8 / 286  (id=-8565879665326676003)

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
  - Natal
  - Pearce  (also: Mr Pearce)
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

## Chunk 9 / 286  (id=-8544226203384833051)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #912

**Entities in chunk:**
  - Abel
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis How
  - Dr. DuPlessis  (also: DuPlessis)
  - Edross  (also: Mr Edross)
  - INDEX 166  (also: INDEX 166)
  - Morris Alexander  (also: Morris)
  - Purcell  (also: Purcell)
  - Torch Commando  (also: Torch)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my Uncle Goolam"}
```

**CC quote:** `my Uncle Goolam`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 10 / 286  (id=-8505583857113009851)

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

## Chunk 11 / 286  (id=-8488686338445114035)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #333

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Times
  - Helen Abrahams
  - Israel  (also: Israel)
  - Menachem Begin
  - Muir Street Primary
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

## Chunk 12 / 286  (id=-8468092369900413172)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #208

**Entities in chunk:**
  - Aunty Annie
  - Ben Kies  (also: Ben Kies M.A)
  - Chapel Street School  (also: Chapel Street School)
  - Cyril
  - INDEX 166  (also: INDEX 166)
  - Lord Wotton
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Nasima M.S  (also: Nasima)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Omar Khayyam
  - Peerbhai
  - Peter Alexander Rassool
  - Rehana Mohamed Annie  (also: Rehana Mohamed Annie)
  - Reza Rassool  (also: Reza)
  - Teachers League Steward
  - Whybow Cloete
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** married, wed , daughter, father, mother,  uncle , gave birth

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 13 / 286  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Bibi  (also: Bibi)
  - Cosmo Pieterse
  - Drill Hall
  - Fatima Seedat
  - Gool  (also: Gool)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Guy's Hospital  (also: Guys Hospital)
  - Hans Friederichs
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Joosub Gool  (also: Joosub)
  - London
  - Movement
  - PTAs
  - Sayed Kader Sayed Noor
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
{"quote": "her brother’s"}
```

**CC quote:** `her brother’s`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 14 / 286  (id=-8243925636978039329)

**Section:** Yousuf (Joe) Rassool   x  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #20

**Entities in chunk:**
  - Cape Town
  - Churchill
  - England  (also: England)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX 166)
  - Los Angeles
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Stalingrad
  - United States  (also: United States)
  - William Wordsworth

**Triggers found:** daughter, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "her brother, Dr. Goolam"}
```

**CC quote:** `her brother, Dr. Goolam`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Nurjehan Gool Mohamed","relation":"sibling_of","to":"Goolam Gool"}]}
```

**Extracted relations:**
- `Nurjehan Gool Mohamed` **sibling_of** `Goolam Gool`

---

## Chunk 15 / 286  (id=-8225958241890195217)

**Section:** Yousuf (Joe) Rassool   30  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #166

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Buitencingle  (also: Buitencingle)
  - Cape Peninisula Students Union
  - District Six  (also: District 6, the District, District Six)
  - Heneke  (also: Mr Heneke)
  - Hewat  (also: Hewat)
  - Hewat Training College  (also: Hewat Training College)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Neville Alexander
  - New Era Fellowship  (also: NEF)
  - Pat Naidoo
  - Tabata
  - Trafalgar  (also: Trafalgar)
  - Ursula Wolhuter
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "my uncle’s surgery"}
```

**CC quote:** `my uncle’s surgery`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 16 / 286  (id=-8186139953935417234)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #919

**Entities in chunk:**
  - Dora Taylor
  - Hassan Bavasah  (also: Hassan)
  - Isaac Pfaff
  - Kies  (also: Mr Kies)
  - Movement
  - NEF'S
  - Nosipho Majeke
  - Organising Secretary
  - Rasheda Rassool  (also: Rasheda)
  - South America  (also: South Africa)
  - Zain Rassool  (also: Zain)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 17 / 286  (id=-8167953885918041797)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #396

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed  (also: Baboo)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Choir
  - Halima Gool Courtesy Selim Gool
  - Hamid Khan
  - Hoosain Ally  (also: Hussain Ally)
  - Ismail  (also: Ismail)
  - Ismail Kagee
  - Union Fruit Supply
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle Ismail"}
```

**CC quote:** `my uncle Ismail`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 18 / 286  (id=-8157488290903077572)

**Section:** 55  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #288

**Entities in chunk:**
  - Barnato Board
  - Broadway Dawn Darkies
  - Caledon Square
  - Coon Club
  - Dr Goolam Gool District Six  (also: Goolam Gool District Six)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Lest We Forget
  - Meneer
  - Movement
  - New Era Fellowship  (also: NEF)
  - Professor James  (also: Prof. James)
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
Since the narrator is "my" and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. The quote directly states "my sisters", which implies a sibling relationship.

Here is the extracted family relationship in JSON format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":[]}]}

However, since the quote does not specify the names of the sisters, we cannot provide a valid 'to' endpoint. According to the rules, if both endpoints are not identifiable as canonical names, we should return an empty list.

So, the corrected output is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 19 / 286  (id=-8077784496787990642)

**Section:** 51  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #265

**Entities in chunk:**
  - Allied
  - Ardenne
  - Argus
  - B. Allies  (also: Allies)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Generals Montgomery  (also: Generals Montgomery)
  - George Golding
  - Normandy
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Gow)
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

## Chunk 20 / 286  (id=-8041283814668973231)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #109

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Cape Town
  - Communist Party
  - District Six  (also: District 6, the District, District Six)
  - Franchise Action Council
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - India Ghulzar Khan  (also: India)
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - USSR  (also: USSR)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 21 / 286  (id=-7980378207581507601)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #207

**Entities in chunk:**
  - Aunty Annie
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - British Government
  - Buitencingle  (also: Buitencingle)
  - Castle Street
  - Fazil Rassool  (also: Fazil)
  - Gadija Aunty Dija
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Natal
  - Peter Alexander Rassool
  - Rehana Mohamed
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** married, father, mother,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad chose Ayesha"}
```

**CC quote:** `my dad chose Ayesha`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Fazil Rassool`

---

## Chunk 22 / 286  (id=-7970562435578500859)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #103

**Entities in chunk:**
  - Appollis Slingers  (also: Appolis Slingers)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Cape
  - Cape Town
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Natal
  - Rander
  - Transvaal  (also: Transvaal)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zoology Block

**Triggers found:** wife, father, mother, sister

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "my grandfather", we can infer that Yousuf Rassool's grandfather is being referred to. However, the name of the grandfather is not explicitly mentioned in the quote.

But since Haji Joosub Maulvi Hamid Gool is listed as a known person with multiple aliases including "J.M.H. Gool" and "Gool", which could be a surname match for Yousuf Rassool, we can make an educated guess that Haji Joosub Maulvi Hamid Gool might be the grandfather being referred to.

If this is correct, then the extracted family relationship would be:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Haji Joosub Maulvi Hamid Gool"}]}

However, without more explicit information in the quote, we cannot confirm this with certainty.
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Haji Joosub Maulvi Hamid Gool`

---

## Chunk 23 / 286  (id=-7752783211163375761)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1062

**Entities in chunk:**
  - Argus boys
  - Avenue
  - Buitencingle  (also: Buitencingle)
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

## Chunk 24 / 286  (id=-7603322562148184082)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #659

**Entities in chunk:**
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - Cape Town-Woodstock P.T.A.  (also: Cape Town-Woodstock P.T.A)
  - Congress Movement
  - District Six  (also: District 6, the District, District Six)
  - Gool  (also: Gool)
  - Heneke  (also: Mr Heneke)
  - Hewat Training College  (also: Hewat Training College)
  - Liberation League
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Scotches Kloof  (also: Scotches-Kloof)
  - Trade Unionists
  - Trafalgar  (also: Trafalgar)
  - Woodstock

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 25 / 286  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Churchill
  - France  (also: France)
  - Generals Alexander  (also: Generals Alexander)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Montgomery
  - Rommel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** father,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Zarina"}
```

**CC quote:** `my cousin Zarina`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 26 / 286  (id=-7549255937705633040)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #947

**Entities in chunk:**
  - Alexander
  - Candy
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
  - Steinbeck
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

## Chunk 27 / 286  (id=-7342763633505714765)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #332

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Ethnic Pride
  - Green Point Commons  (also: Green Point Common)
  - Habibia  (also: Habibia)
  - Israel  (also: Israel)
  - James Currey  (also: James Curry)
  - Joh'burg
  - M. Allie  (also: Allie)
  - M. Wilson  (also: Wilson)
  - Menachem Begin
  - Mr. Maron  (also: Maron)
  - Racial Prejudice
  - Reuben Pogrund
  - Victorian Cape Town
  - Vivian Bickford-Smith
  - Wesleyan W.B.

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "my brother Abdul"}
```

**CC quote:** `my brother Abdul`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Rassool","relation":"sibling_of","to":"Unknown"}]}
```

**Extracted relations:** none

---

## Chunk 28 / 286  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Anti-CAD  (also: Anti CAD)
  - Boeta
  - Chops
  - Club
  - Gootie
  - Grootjie
  - INDEX 166  (also: INDEX 166)
  - James Africa
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Slamat
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

## Chunk 29 / 286  (id=-7299298035852886932)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #569

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Aligarh College  (also: Aligarh)
  - Bibi's girls
  - Canadian High Commissioner  (also: Canadian)
  - Chapel Street
  - Corporation Street
  - Fort Hare  (also: Fort Hare)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - India Ghulzar Khan  (also: India)
  - Indian Congress
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Khadija Ebrahim  (also: Khadija)
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Mrs Ahmed  (also: Ahmed)
  - Persotem Patel  (also: Persotem)
  - Trafalgar High  (also: Trafalgar High)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** parent, sister, brother

**CC pass (raw):**
```json
{"quote": "her elder sisters and young overweight brother, Ahmed"}
```

**CC quote:** `her elder sisters and young overweight brother, Ahmed`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 30 / 286  (id=-7244458603064688503)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #43

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Buitencingle  (also: Buitencingle)
  - Corporation Street
  - German Lutheran Church  (also: German Lutheran Church)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanaffi Quwatul Islam Mosque  (also: Quwatul Islam Mosque, Loop Street Mosque, Hanafi Mosque, Hanaffi Mosque)
  - INDEX 166  (also: INDEX 166)
  - Jan Christiaan Smuts
  - Kloof Street  (also: Loop Street)
  - Malan-Havenga  (also: Malan)
  - Re-United National Party
  - Rogge Bay
  - South America  (also: South Africa)
  - United Nations
  - Waterkant Street

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "My grandfather was a founder and life trustee of the mosque"}
```

**CC quote:** `My grandfather was a founder and life trustee of the mosque`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Haji Joosub Maulvi Hamid Gool","relation":"child_of","to":"Abdul Rassool"}]}
```

**Extracted relations:**
- `Haji Joosub Maulvi Hamid Gool` **child_of** `Abdul Rassool`

---

## Chunk 31 / 286  (id=-7082280809534297158)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #345

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Education Department
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
  - Joan of Arc  (also: Joan)
  - Malaai  (also: Malaai)
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

## Chunk 32 / 286  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - Coloured Advisory Council
  - E. Albertus  (also: Albertus)
  - General
  - Habibia  (also: Habibia)
  - Hewat  (also: Hewat)
  - Hymie Beimel  (also: Hymie)
  - M. Allie  (also: Allie)
  - Smuts's United Party  (also: Smuts)
  - Standard Two
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"[unnamed sister]"}]}
```

**Extracted relations:** none

---

## Chunk 33 / 286  (id=-6869632677302387809)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #504

**Entities in chunk:**
  - Cape Town
  - Clanwilliam
  - Dukes
  - Europe
  - Hell's Heights Pass
  - INDEX 166  (also: INDEX 166)
  - Italian POWs
  - Messaris Junior
  - Persotem Patel  (also: Persotem)
  - Table Bay
  - Tykie Messaris
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "his father’s store"}
```

**CC quote:** `his father’s store`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his father's store", we can infer that Yousuf Rassool is referring to his own father. Therefore, the family relationship stated in the quote is:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"[father's name not specified]"}]}
```

**Extracted relations:** none

---

## Chunk 34 / 286  (id=-6782705425542896048)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #113

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - England  (also: England)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall  (also: Jamieson Hall)
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

## Chunk 35 / 286  (id=-6763001656966195273)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1067

**Entities in chunk:**
  - Chapel Street Staff
  - Ciskei  (also: Ciskei)
  - Danny De Beer  (also: Danny)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nelson Mandela
  - New Era Fellowship  (also: NEF)
  - Peninsula Maternity Hospital  (also: Peninsula Maternity Hospital)
  - Transkei
  - Unity Movement  (also: Unity)
  - Van Riebeeck  (also: Van Riebeeck)
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

## Chunk 36 / 286  (id=-6710587015263367697)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #141

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, daughter, father

**CC pass (raw):**
```json
{"quote": "my grandfather’s daughters"}
```

**CC quote:** `my grandfather’s daughters`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote uses "my", we can infer that Yousuf Rassool is referring to his own family relationships.

The quote states "my grandfather's daughters". This implies a relationship between Yousuf Rassool and his grandfather's daughters, but it does not directly state a familial relationship between two identifiable individuals. However, since the narrator is Yousuf Rassool, we can infer that his grandfather's daughters are his aunts.

But according to the CRITICAL RULES, if the quote uses 'aunt', return {"relations":[]} — these relation types are not in the schema.

So, the correct answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 37 / 286  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - E. Albertus  (also: Albertus)
  - G.Hoosain  (also: Hoosain)
  - Group Areas Boards  (also: Group Areas Board)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Labour Party
  - Parker  (also: Mr. Parker)
  - Parliamentary
  - South America  (also: South Africa)
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
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"G.Hoosain"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `G.Hoosain`

---

## Chunk 38 / 286  (id=-6658517349595717207)

**Section:** 97  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #587

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Church Street
  - Churchill
  - Ciraj Rassool
  - Ciskei  (also: Ciskei)
  - City Hall  (also: City Hall)
  - Clara Petacci
  - Cliff Gladwin
  - Clifford Odets
  - Deanna Durbin
  - Gardener Williams  (also: Gardiner Williams)
  - Great Man
  - INDEX 166  (also: INDEX 166)
  - Leslie Hutchinson
  - Nelson Eddy
  - Searle Street
  - Street Singer
  - Trafalgar Park
  - Walmer Estate
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 39 / 286  (id=-6575318364478995628)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #89

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Dr Tabata
  - Fabians Kies  (also: Fabians)
  - India Ghulzar Khan  (also: India)
  - Jeddah
  - Joyce Dixon
  - Kies  (also: Mr Kies)
  - Labour Party
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Qudrat of Allah  (also: Qudrat)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "Your loving son, Al Hajj Joosub Maulvi Hamid"}
```

**CC quote:** `Your loving son, Al Hajj Joosub Maulvi Hamid`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 40 / 286  (id=-6563863471018850689)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1136

**Entities in chunk:**
  - A. Roberts  (also: Roberts)
  - Cape Town
  - Clanwilliam
  - Drakenstein Mountains
  - Du Toit's Kloof
  - Eastern Cape
  - Ebrahim  (also: Ebrahim)
  - Gool  (also: Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hell's Heights Pass
  - Isgak Adams
  - J.M.H.  (also: J.M.H)
  - James Currey  (also: James Curry)
  - Little Karoo
  - M. Wilson  (also: Wilson)
  - Nash  (also: Nash)
  - Oudtshoorn
  - Persotem Patel  (also: Persotem)
  - Rev R.A. Jackson
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

## Chunk 41 / 286  (id=-6141146778954841009)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #506

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town Woodstock
  - District Six  (also: District 6, the District, District Six)
  - Durban's Curry's Fountain  (also: Durban)
  - Ebrahim  (also: Ebrahim)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Keraan
  - Louis Messaris
  - Messaris King Peanut Butter
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Parent-Teacher Associations  (also: Parent Teachers Association)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Tykie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "my dad"}
```

**CC quote:** `my dad`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 42 / 286  (id=-6012376786615465402)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1022

**Entities in chunk:**
  - Belgium  (also: Belgium)
  - Britain's Empire  (also: Britain)
  - Cassiem  (also: Cassiem)
  - Denmark  (also: Denmark)
  - District Six  (also: District 6, the District, District Six)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - France  (also: France)
  - Holland  (also: Holland)
  - M. Wilson  (also: Wilson)
  - Mrs Wooding  (also: Wooding)
  - Norway  (also: Norway)
  - Queens Road
  - Tregger  (also: Mr. Tregger)
  - Unity Movement  (also: Unity)
  - Van der Ross
  - Vassan Hindu C.C.  (also: Vassan)
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

## Chunk 43 / 286  (id=-5915939077846947542)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #738

**Entities in chunk:**
  - Britain's Empire  (also: Britain)
  - Cape Times
  - Charles Simons
  - Councillor Parker
  - England  (also: England)
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

## Chunk 44 / 286  (id=-5897797065487598428)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #70

**Entities in chunk:**
  - Anti-CAD  (also: Anti CAD)
  - Anti-Coloured Affairs Department
  - British Army
  - Gladwin  (also: Gladwin)
  - Graduated as a Teacher
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Indian Opinion
  - J. M. H. Gool & Co.
  - Majesty Queen Victoria
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Surat
  - Tufty Mann
  - Vaal River

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 45 / 286  (id=-5853235968246498787)

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
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs
  - Janab Jane
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** married, daughter, father, brother

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. The quote states "my grandfather", which implies a parent-of relationship.

However, since the schema only allows for spouse_of, parent_of, child_of, sibling_of, half_sibling_of relationships, and the quote does not directly state any of these relationships, but rather an indirect one (grandfather), we cannot extract a valid relation.
```

**Extracted relations:** none

---

## Chunk 46 / 286  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - British Labour Party MP
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gurukula at Hardwar  (also: Gurukula)
  - Hamid Khan
  - Hardwar
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - India Ghulzar Khan  (also: India)
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

## Chunk 47 / 286  (id=-5810165315343532045)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #574

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Communists
  - Deanna Durbin
  - Durban's Curry's Fountain  (also: Durban)
  - Formosa
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Green Point Commons  (also: Green Point Common)
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hutch
  - Moslem C.C.
  - Mylie Begg  (also: Mylie)
  - New Era Fellowship  (also: NEF)
  - Red China  (also: Red China)
  - Sastri College
  - United States  (also: United States)
  - University of Cape Town  (also: University of Cape Town)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 48 / 286  (id=-5786981904019721036)

**Section:** Yousuf (Joe) Rassool   viii  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #7

**Entities in chunk:**
  - Ben Malamba
  - Bibi  (also: Bibi)
  - Cape
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gool Gul
  - Gurukula at Hardwar  (also: Gurukula)
  - Haartzen
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hardwar
  - India Ghulzar Khan  (also: India)
  - J. M. H. Gool
  - James Africa
  - Marcina Kagan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Ramsey MacDonald
  - Rev. C. F. Andrews
  - Salie Van Haacht
  - South America  (also: South Africa)
  - Stanley Abrahams
  - Wagieda
  - Woody Woodman  (also: Woody)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 49 / 286  (id=-5767629269527031038)

**Section:** Chapter One  Caledon Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #274

**Entities in chunk:**
  - Abdulla Mohamed
  - Abdulla Parker
  - Ahmed Salasa
  - Buitencingle  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Cosmo Pieterse
  - Dickmans
  - M. Allie  (also: Allie)
  - Mohamed Giri
  - Natasha
  - Pogrunds
  - Reznikovs  (also: Reznikovs)
  - Shapiros

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 50 / 286  (id=-5732511328885138558)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #498

**Entities in chunk:**
  - Abe Desmore
  - Cape Corps
  - District Six  (also: District 6, the District, District Six)
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall  (also: Jamieson Hall)
  - Limpopo River
  - Nerine Desmond
  - Per Angusta Ad Augusta
  - Roland Colman
  - Smuts's United Party  (also: Smuts)
  - South America  (also: South Africa)
  - Trafalgar Junior School  (also: Trafalgar Junior School)
  - Trafalgar School Motto
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad’s younger stepbrother"}
```

**CC quote:** `my dad’s younger stepbrother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 51 / 286  (id=-5714165220014992650)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #255

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Anti-CAD  (also: Anti CAD)
  - Fazil Rassool  (also: Fazil)
  - Glen
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Jack Meltzer
  - Kies  (also: Mr Kies)
  - Kloof Nek  (also: Kloof Nek)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Platteklip Gorge
  - Ronnie Jooste  (also: Ronnie)
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

## Chunk 52 / 286  (id=-5635069970394508836)

**Section:** 45  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #220

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Glen
  - Gool  (also: Gool)
  - Gray's Elegy
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek  (also: Kloof Nek)
  - Kloof Street  (also: Loop Street)
  - M. Wilson  (also: Wilson)
  - Milton's Lycidas
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)

**Triggers found:** brother, half-brother

**CC pass (raw):**
```json
{"quote": "Lallie muttered that it had to be Aboeta, her half-brother"}
```

**CC quote:** `Lallie muttered that it had to be Aboeta, her half-brother`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ayesha Rassool","relation":"half_sibling_of","to":"Mohammed Hanief Gool"}]}
```

**Extracted relations:**
- `Ayesha Rassool` **half_sibling_of** `Mohammed Hanief Gool`

---

## Chunk 53 / 286  (id=-5610384977170982774)

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
  - INDEX 166  (also: INDEX 166)
  - Len Hutton
  - Mylie Begg  (also: Mylie)
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

## Chunk 54 / 286  (id=-5556520109474695452)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #135

**Entities in chunk:**
  - Australia
  - Australians
  - Barmania MA  (also: Barmania)
  - Board of the Stakesby-Lewis Hostel  (also: Board)
  - Buitencingle  (also: Buitencingle)
  - Canada
  - Cape Town
  - Dr. Jamieson  (also: Jamieson)
  - George Golding
  - Group Areas  (also: Group Area)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Jabavu  (also: Dr. Jabavu)
  - Mr. B. Allie  (also: B. Allie)
  - New Zealanders
  - Progressive Party
  - Salie Dollie
  - St Paul's Mission School  (also: St Paul's Mission School)
  - Tommie
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "My grandfather"}
```

**CC quote:** `My grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "My grandfather", we can infer that Yousuf Rassool's grandfather is being referred to. However, the name of the grandfather is not mentioned.

But since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. 

The quote directly states a family relationship between Yousuf Rassool and his grandfather, but the grandfather's name is not provided. Therefore, we cannot establish a valid relation with a canonical name.

According to the rules, if both endpoints are not identifiable as canonical names, return empty.

So, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 55 / 286  (id=-5542761636394941458)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #139

**Entities in chunk:**
  - Anti-Indian Laws
  - Caledon Square
  - Dr. Ramamurthi  (also: Ramamurthi)
  - George
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Indian South Africa
  - South African Apartheid
  - South African Republic
  - Wynberg
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zurayda

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 56 / 286  (id=-5460737940864037782)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1137

**Entities in chunk:**
  - Abdurahman
  - Adonis
  - Bellingham  (also: Mr Bellingham)
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - Chota
  - Gadija Dollie
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Kloof Street  (also: Loop Street)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Ralph J Bunche  (also: Ralph Bunche)
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

## Chunk 57 / 286  (id=-5418155653937249668)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1145

**Entities in chunk:**
  - All African Convention  (also: AAC, All-African Convention)
  - Buitencingle  (also: Buitencingle)
  - District Six  (also: District 6, the District, District Six)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Emily Hobhouse
  - Even Buitencingle
  - Gool  (also: Gool)
  - Goulam Gool
  - Group Areas Boards  (also: Group Areas Board)
  - Les Jacobs
  - Liberals
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Gladstone  (also: Mrs. Gladstone)
  - New Era Fellowship  (also: NEF)
  - Prime Minister Botha
  - Ralph J Bunche  (also: Ralph Bunche)
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

This is because the quote mentions a family relationship between Gandhi's son and Goulam Gool's sister, but it does not provide enough information to establish a direct relationship between two canonical names. The endpoints are not identifiable as canonical names in this case.
```

**Extracted relations:** none

---

## Chunk 58 / 286  (id=-5385222803517127501)

**Section:** 125  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #782

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Eastern Bloc  (also: Western Bloc)
  - Habibia  (also: Habibia)
  - Israel  (also: Israel)
  - Joh'burg
  - M. Allie  (also: Allie)
  - Menachem Begin
  - Mr. Maron  (also: Maron)
  - Pogrund  (also: Pogrund)
  - Rand Daily Mail
  - Reuben Pogrund
  - Socialist Camp
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 59 / 286  (id=-5364503672193663248)

**Section:** Yousuf (Joe) Rassool   16  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #119

**Entities in chunk:**
  - Alexander
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - District Six  (also: District 6, the District, District Six)
  - Dylan
  - Eldest
  - Feyruz Rassool  (also: Feyruz)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
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

This clause identifies a direct family relationship between two people: the narrator and their mother, Ayesha.
```

**CC quote:** `Mom (Ayesha)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 60 / 286  (id=-5340629096267782490)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #75

**Entities in chunk:**
  - Cape
  - Cape Town
  - George Meissenheimer  (also: George Meisenheimer)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Mauritius
  - South African Apartheid
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Wahida Ta Al8
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
{"quote": "her elder sister"}
```

**CC quote:** `her elder sister`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "her elder sister", we can infer that Yousuf Rassool is referring to his own sibling. However, the quote does not mention Yousuf's name explicitly, but rather uses "her" which implies a female sibling.

Unfortunately, there is no canonical name mentioned in the quote for the female sibling. Therefore, according to the critical rules, we must return an empty JSON:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 61 / 286  (id=-5308475734057125055)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #348

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Athlone Branch  (also: Athlone Branch)
  - Catholic Church  (also: Catholic)
  - Chapel Street
  - Chapel Street School  (also: Chapel Street School)
  - Even Buitencingle
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Khalifa
  - Methodist
  - Miss Brown
  - Mount Streets  (also: Mount Street)
  - Mrs Domingo  (also: Domingo)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 62 / 286  (id=-5272705089768951628)

**Section:** Yousuf (Joe) Rassool   134  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #849

**Entities in chunk:**
  - British Labour Party MP
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gurukula at Hardwar  (also: Gurukula)
  - Hamid Khan
  - Hardwar
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Indian Opinion
  - J. M. H. Gool
  - Marcina Kagan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Ramsey MacDonald
  - Rasheda Rassool  (also: Rasheda)
  - Rev. C. F. Andrews
  - Reverend Doke
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "my sisters Zain and Rasheda"}
```

**CC quote:** `my sisters Zain and Rasheda`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Zain Rassool"},{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Zain Rassool`
- `Yousuf Rassool` **sibling_of** `Rasheda Rassool`

---

## Chunk 63 / 286  (id=-5268117870089455287)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #97

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Aminabhen
  - Bibi  (also: Bibi)
  - Desmond Green
  - Fazil Rassool  (also: Fazil)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hudson  (also: Hudson)
  - INDEX 166  (also: INDEX 166)

**Triggers found:** wed , mother, brother

**CC pass (raw):**
```json
{"quote": "her brother"}
```

**CC quote:** `her brother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 64 / 286  (id=-5184936258167894087)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #714

**Entities in chunk:**
  - America
  - Bishop Lavis
  - Bonteheuwel
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - Even Buitencingle
  - Ghulzar Khan  (also: Gulzar Khan)
  - Gool  (also: Gool)
  - Habibia Institute  (also: Habibia Institute)
  - Hanover Park
  - India Ghulzar Khan  (also: India)
  - M. Wilson  (also: Wilson)
  - Mannenberg
  - Mitchell's Plain
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

## Chunk 65 / 286  (id=-5154709854984411195)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #761

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Bibi  (also: Bibi)
  - Chapel Street School  (also: Chapel Street School)
  - Compton  (also: Compton)
  - Doug Wright
  - E.A. Parker
  - Edrich
  - Faried Rossier
  - Germany  (also: Germany)
  - Gladwin  (also: Gladwin)
  - Habibia Institute  (also: Habibia Institute)
  - Hutton  (also: Hutton)
  - Jannat
  - John Arlott
  - NEF Hassan
  - Nasim Rassool  (also: Nasim)
  - Roly Jenkins
  - South America  (also: South Africa)
  - Star Bioscope  (also: Star Bioscope)
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
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"E.A. Parker"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `E.A. Parker`

---

## Chunk 66 / 286  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head Student
  - Heathrows C.C.  (also: Heathrows C.C)
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - Mister Efficiency
  - Muslims C.C.  (also: Muslims)
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

## Chunk 67 / 286  (id=-5010050899247603262)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1138

**Entities in chunk:**
  - Abdurahman
  - Alan Gregor Cobley
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - Cathedral Hall
  - Coloured People's Vigilance Committee  (also: Coloured People's Vigilant Committee)
  - Edgar Maurice
  - Even Buitencingle
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Indian Opinion
  - J.M.H. Wilson
  - James M.
  - John Brown  (also: John Brown)
  - Journal of Southern African Studies
  - Kloof Street  (also: Loop Street)
  - Nasima M.S  (also: Nasima)
  - Oswald Pirow
  - Segregation in Education
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Victoria Street

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 68 / 286  (id=-5009073907762257871)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #100

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bhokkie
  - Bibi  (also: Bibi)
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

## Chunk 69 / 286  (id=-4897146471319258283)

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

## Chunk 70 / 286  (id=-4726387397704307970)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #337

**Entities in chunk:**
  - Arthur Rank
  - British Bioscope
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Fort Hare  (also: Fort Hare)
  - Hatter's Castle  (also: Hatter's Castle)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - National Theatre  (also: National Theatre)
  - Oswald Pirow
  - Paramount
  - Robert Newton
  - Star Bioscope  (also: Star Bioscope)
  - Warner Brothers
  - William Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** wed , daughter, brother,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 71 / 286  (id=-4713556574132170500)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #609

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Cape
  - Cassim Amra
  - Catherine Pienaar
  - Helen Abrahams
  - INDEX 166  (also: INDEX 166)
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

## Chunk 72 / 286  (id=-4527305166854275038)

**Section:** 129  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #815

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Delarey  (also: Delarey)
  - George
  - Grand Parade
  - INDEX 166  (also: INDEX 166)
  - Kies  (also: Mr Kies)
  - Noors  (also: Noors)
  - Population Registration Act
  - Strand
  - Trafalgar  (also: Trafalgar)
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 73 / 286  (id=-4493652338336870038)

**Section:** Yousuf (Joe) Rassool   134  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #850

**Entities in chunk:**
  - African Methodist Episcopal Church
  - Coloured Advisory Council
  - George Golding
  - Hamid
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - Nationalists Party  (also: Nationalist Party)
  - Reverend Gow  (also: Gow)
  - Salie Dollie
  - Smuts's United Party  (also: Smuts)
  - Stalingrad
  - Stephen Spender
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 74 / 286  (id=-4440340057225779401)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #314

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Andrew Mackrill
  - Cape Town University  (also: Cape Town University)
  - Cardiff
  - Colin Wynne
  - Cynthia Fisher
  - Fish Market
  - George Veldsman
  - Hewat Amateur Theatrical Society
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Isaac Pfaff
  - Ivan Agherdien
  - Leonard Dixon
  - Moslem Mission School  (also: Moslem Mission School)
  - Muir Street
  - Norman Florence
  - Rasheda Rassool  (also: Rasheda)
  - Sam Wo's Laundry
  - Zain Rassool  (also: Zain)

**Triggers found:** married

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 75 / 286  (id=-4303196698623211308)

**Section:** The struggle unfolds  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #131

**Entities in chunk:**
  - Cape
  - INDEX 166  (also: INDEX 166)
  - Mike Allie
  - Sailor Malan
  - South America  (also: South Africa)
  - Union of South Africa  (also: Union of South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 76 / 286  (id=-4279669986134027190)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #620

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Capetown
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Government Avenue
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Islamic Synagogue  (also: Islamic)
  - Ismail  (also: Ismail)
  - Katie
  - Mouille Point
  - Nasima M.S  (also: Nasima)
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Raza
  - Somerset Hospital  (also: Somerset Hospital)
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

## Chunk 77 / 286  (id=-4263780790926522616)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #187

**Entities in chunk:**
  - Abdurahman
  - J.Boyce
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Daily Witness
  - Natal Indian Congress
  - National Gandhi Museum  (also: National Gandhi Musem)
  - National Road
  - National Theatre  (also: National Theatre)
  - Nationalists Party  (also: Nationalist Party)
  - Native Representative Councils
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Nehru
  - Nelson Eddy
  - Nelson Mandela
  - Nerine Desmond
  - Neville Alexander
  - New Era Fellowship  (also: NEF)
  - Nicky Springveld
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

## Chunk 78 / 286  (id=-4250574601481422414)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #424

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Anti-Zionism
  - Cape Argus
  - Desmond Green
  - Golding Van  (also: Golding)
  - Green Point Commons  (also: Green Point Common)
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - New Era Fellowship  (also: NEF)
  - Regan
  - Reverend Gow  (also: Gow)
  - Van der Ross
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my Uncle Aity"}
```

**CC quote:** `my Uncle Aity`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 79 / 286  (id=-4183482280000304421)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #31

**Entities in chunk:**
  - A.H.  (also: A.H)
  - AntiSlavery Papers
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Buitencingle  (also: Buitencingle)
  - C/91
  - Cape
  - Church Street Capetown S.A
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Gool  (also: Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - London
  - M. Wilson  (also: Wilson)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Oxford
  - Pan African Conference
  - Rhodes House Library  (also: Rhodes House Library)
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

## Chunk 80 / 286  (id=-4158673842638911363)

**Section:** 65  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #358

**Entities in chunk:**
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Bagus Allie  (also: Bagus-Allie)
  - Caledon Streets  (also: Caledon Street)
  - Chapel Street School  (also: Chapel Street School)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Durban's Curry's Fountain  (also: Durban)
  - Elmer Rice
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Heerie  (also: Heerie)
  - INDEX 166  (also: INDEX 166)
  - Jannat
  - Kanamia Muslims
  - Moosa Cape Town
  - Moosa Driver
  - Muir Street
  - O.Henry  (also: Henry)
  - Sulloo
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zinatul Islam Mosque

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 81 / 286  (id=-4048907962032009589)

**Section:** Chapter Nine  Senior Year  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #562

**Entities in chunk:**
  - Bethune  (also: Bethune)
  - Cape Argus
  - Castle Division
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Communist Party
  - Cronin  (also: Cronin)
  - David Bloomberg
  - District 6 Ward
  - Divisional Council
  - European residential area
  - General Smuts
  - George Sava
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hamid BG
  - Harry Snitcher
  - INDEX 166  (also: INDEX 166)
  - Persotem Patel  (also: Persotem)
  - United Party
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 82 / 286  (id=-3900190985977953352)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #150

**Entities in chunk:**
  - Boers  (also: Boers)
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - City
  - Durban's Curry's Fountain  (also: Durban)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Isvarlal
  - Itie
  - Manilal Gandhi  (also: Manilal)
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

## Chunk 83 / 286  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Cape African Teachers Association  (also: Cape African Teachers' Association)
  - Education Department
  - Emily Hobhouse
  - Grey Street  (also: Bree Street)
  - Groote Schuur  (also: Groot Schuur)
  - INDEX 166  (also: INDEX 166)
  - Let Us Live
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

## Chunk 84 / 286  (id=-3882651649594979630)

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
  - Muddy
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

## Chunk 85 / 286  (id=-3872024114827273512)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #78

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Budenny
  - Chaim Beimel
  - Churchill
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - City Hall  (also: City Hall)
  - Comrade Bill
  - Counter Attack
  - District Six  (also: District 6, the District, District Six)
  - Friends of the Soviet Union
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Harry Snitcher
  - Hollywood  (also: Hollywood)
  - INDEX 166  (also: INDEX 166)
  - Joosub Gool  (also: Joosub)
  - King Edward VII
  - Malinovsky
  - Moses Kotane
  - Rassool
  - Ray Alexander
  - Rokossovsky
  - Smuts's United Party  (also: Smuts)
  - Song of Russia  (also: Song)
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

## Chunk 86 / 286  (id=-3850285751536457852)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #251

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Buitencingle  (also: Buitencingle)
  - Cape Malays
  - City Council
  - Coloured Affairs
  - Dr Du Plessis  (also: Du Plessis)
  - Edross  (also: Mr Edross)
  - Heneke  (also: Mr Heneke)
  - Hindu Association
  - I.D. Du Plessis
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Jabavu  (also: Dr. Jabavu)
  - Lucheram
  - M Enus
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

## Chunk 87 / 286  (id=-3844520640191004859)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #96

**Entities in chunk:**
  - Aminabhen
  - Armin Dawood Afghan Rifles Subardar
  - Belgium  (also: Belgium)
  - Bibi  (also: Bibi)
  - Denmark  (also: Denmark)
  - District Six  (also: District 6, the District, District Six)
  - Europe
  - France  (also: France)
  - Hamza
  - Holland  (also: Holland)
  - INDEX 166  (also: INDEX 166)
  - Norway  (also: Norway)
  - Ronnie Jooste  (also: Ronnie)
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother, sister, brother, in-law

**CC pass (raw):**
```json
{"quote": "her sister-in-law, Aminabhen"}
```

**CC quote:** `her sister-in-law, Aminabhen`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 88 / 286  (id=-3762557488640433155)

**Section:** 79  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #452

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Fort Hare  (also: Fort Hare)
  - Golding Van  (also: Golding)
  - Heyl  (also: Mr. Heyl)
  - INDEX 166  (also: INDEX 166)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ossewa Brandwag
  - Regan
  - Reverend Gow  (also: Gow)
  - Van der Ross
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 89 / 286  (id=-3704547662899929915)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1140

**Entities in chunk:**
  - Aesop
  - Ben Cloete
  - Cape Times Law Reports
  - Chapel Street School  (also: Chapel Street School)
  - Faried Rossier
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - J. M. Wilson
  - Jannat
  - Merwe
  - Nasim Rassool  (also: Nasim)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Robert R. Edgar
  - South America  (also: South Africa)
  - Supreme Court Proceedings
  - Trinidad

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 90 / 286  (id=-3668983573170314510)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #626

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Buitencingle  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Guy's Hospital  (also: Guys Hospital)
  - Kimberley Hospital  (also: Kimberley Hospital)
  - London
  - London SE
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Peckham Road
  - Vicks
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 91 / 286  (id=-3635611545960891135)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #505

**Entities in chunk:**
  - District Six  (also: District 6, the District, District Six)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Louis Messaris
  - Messaris Junior
  - South America  (also: South Africa)
  - Tykie's father
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "“Tykie’s” father, Louis Messaris"}
```

**CC quote:** `“Tykie’s” father, Louis Messaris`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Louis Messaris","relation":"parent_of","to":"Tykie's father"}]}
```

**Extracted relations:**
- `Louis Messaris` **parent_of** `Tykie's father`

---

## Chunk 92 / 286  (id=-3473692056619927671)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #227

**Entities in chunk:**
  - Blauwberg Strand
  - British Government
  - Cape Peninsula
  - Dada Abdullah
  - Disfranchisement Law
  - Divisional Council
  - Fazil Rassool  (also: Fazil)
  - India Ghulzar Khan  (also: India)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muslims C.C.  (also: Muslims)
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

## Chunk 93 / 286  (id=-3445743877757991421)

**Section:** 33 Collected Works of Mahatma Gandhi Vol 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #170

**Entities in chunk:**
  - Adam Gool
  - Aligarh College  (also: Aligarh)
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Indian Opinion
  - Laura Heffer
  - Marcina Kagan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Noor Bagh
  - Phoenix
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** married,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 94 / 286  (id=-3364667971095419225)

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

## Chunk 95 / 286  (id=-3325270166547380284)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #406

**Entities in chunk:**
  - Cannon Street
  - Cape Town Fire Brigade
  - District Six  (also: District 6, the District, District Six)
  - Grand Parade
  - Heerie  (also: Heerie)
  - Ismail  (also: Ismail)
  - M. Allie  (also: Allie)
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

## Chunk 96 / 286  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - Botanical gardens
  - C.Khatieb  (also: Khatieb)
  - Government Avenue
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Captain
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX 166)
  - Next Hassen
  - Nurjahan
  - Rasheda Rassool  (also: Rasheda)
  - Rashid
  - S.Abed  (also: Abed)
  - Salie
  - Shawquet
  - South America  (also: South Africa)
  - Trafalgar  (also: Trafalgar)
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

## Chunk 97 / 286  (id=-3241597184977141743)

**Section:** 89  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #531

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - BM.Kies  (also: BM Kies)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Cape Town
  - Cathedral Hall
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Cosmopolitan Hall
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
  - J. M. H. Gool
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Queen Victoria Street
  - Sayed Noor Chota
  - Solly Edross
  - W.P. Van Schoor  (also: W.P Van Schoor)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:**  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 98 / 286  (id=-3118466577801575510)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #697

**Entities in chunk:**
  - Congress
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Katzeff
  - Malan-Havenga  (also: Malan)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Parow
  - Re-United National Party
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 99 / 286  (id=-3059365739205808916)

**Section:** Chapter Five  Characters of District Six  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #411

**Entities in chunk:**
  - Athlone
  - Auchinlek
  - Avalon Cinema
  - Cairo
  - Churchill
  - Egyptian
  - Gava-Kadoes  (also: Gava)
  - Green Point Commons  (also: Green Point Common)
  - Habibia  (also: Habibia)
  - Hanover Street
  - India Ghulzar Khan  (also: India)
  - Municipal
  - Mymoena Roomaney
  - Rabeyah Mukkadam
  - Rasheda Pansari
  - Ritchie
  - South African 2nd Division
  - South America  (also: South Africa)
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

## Chunk 100 / 286  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - Dr. Ramamurthi  (also: Ramamurthi)
  - I'll
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

## Chunk 101 / 286  (id=-2892056374499759668)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #381

**Entities in chunk:**
  - 7 Buitencingle Street  (also: Buitencingle Street, the Buitencingle mansion, 7 Buitenkingle Street)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Fazil Rassool  (also: Fazil)
  - Groote Schuur  (also: Groot Schuur)
  - INDEX 166  (also: INDEX 166)
  - Khadija Ebrahim  (also: Khadija)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 102 / 286  (id=-2865106326701141136)

**Section:** Chapter Eighteen    Chapel Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #969

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Asia
  - Ben Kies  (also: Ben Kies M.A)
  - Egypt
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mr. Meltzer  (also: Meltzer)
  - South African Republic
  - TLSA's Journal  (also: TLSA Journal)
  - Trafalgar High School  (also: Trafalgar High School)
  - Unity Movement  (also: Unity)
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

## Chunk 103 / 286  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Athlone
  - Charles Simons
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Natal Indians  (also: Natal Indian)
  - Pat Holmes
  - Reuter
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

## Chunk 104 / 286  (id=-2802818586001457035)

**Section:** Yousuf (Joe) Rassool   30  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #165

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - African National Congress
  - Cape Argus
  - Cape Town
  - Corresponding Secretary
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - South African Native National Congress
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "My uncle, Dr. Gool"}
```

**CC quote:** `My uncle, Dr. Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 105 / 286  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Cape Town
  - City
  - Diamond
  - District Six  (also: District 6, the District, District Six)
  - Gool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - J.M.H.  (also: J.M.H)
  - Jeddah
  - M. Allie  (also: Allie)
  - Supreme Court
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 106 / 286  (id=-2740820424072264356)

**Section:** 5  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #59

**Entities in chunk:**
  - Ben Cloete
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Furjah
  - Greeff  (also: Mr. Greeff)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 107 / 286  (id=-2659876011736734958)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #496

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - District Six  (also: District 6, the District, District Six)
  - Far East
  - Gates of India
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Indian Opinion
  - Lest We Forget
  - M. Wilson  (also: Wilson)
  - Medical Service of Natives in South Africa
  - Natal
  - South America  (also: South Africa)
  - Trinidad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 108 / 286  (id=-2550570213336857687)

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
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** parent, sister

**CC pass (raw):**
```json
{"quote": "my parents"}
```

**CC quote:** `my parents`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 109 / 286  (id=-2487606880044980631)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #978

**Entities in chunk:**
  - African National Congress
  - Area
  - Bellingham  (also: Mr Bellingham)
  - Cape Town
  - Freedom Charter
  - Gregori Malenkov
  - Kenya Jomo Kenyatta  (also: Kenya)
  - Kliptown
  - Point Programme
  - Scotches Kloof  (also: Scotches-Kloof)
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

## Chunk 110 / 286  (id=-2460317370086768921)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #739

**Entities in chunk:**
  - Aap Visser
  - Caledon Streets  (also: Caledon Street)
  - Cannon Street
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - George Golding
  - Harold Wolpe
  - Hogwood  (also: Mr Hogwood)
  - Jackie Marks
  - Wits  (also: Wits)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 111 / 286  (id=-2394831796565810193)

**Section:** Yousuf (Joe) Rassool   154  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #987

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - All African Convention Messrs  (also: All African Convention Messrs.)
  - Buitencingle  (also: Buitencingle)
  - Cape Peninisula Students Union
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Neville Alexander
  - New Era Fellowship  (also: NEF)
  - Pat Naidoo
  - Transvaal  (also: Transvaal)
  - Unity Movement  (also: Unity)
  - Ursula Wolhuter
  - Victor

**Triggers found:** wed ,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Nina"}
```

**CC quote:** `my cousin Nina`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 112 / 286  (id=-2278420777688822064)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #182

**Entities in chunk:**
  - Bombay University  (also: Bombay)
  - District Six  (also: District 6, the District, District Six)
  - Dr Kolia
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Mafeking
  - Manilal Gandhi  (also: Manilal)
  - Mariam Gool  (also: Margie, Mariam (Margie))
  - Uma Dhupelia-Mesthrie  (also: Uma Dhupelia-Mesthri)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** son , daughter, sister

**CC pass (raw):**
```json
{"quote": "Uma Dhupelia-Mesthri, granddaughter of Manilal Gandhi"}
```

**CC quote:** `Uma Dhupelia-Mesthri, granddaughter of Manilal Gandhi`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Uma Dhupelia-Mesthrie","relation":"child_of","to":"Manilal Gandhi"}]}
```

**Extracted relations:**
- `Uma Dhupelia-Mesthrie` **child_of** `Manilal Gandhi`

---

## Chunk 113 / 286  (id=-2266997192548340950)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #95

**Entities in chunk:**
  - Armin Dawood
  - Bibi  (also: Bibi)
  - Hamza
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "his brothers-in-law Armin Dawood"}
```

**CC quote:** `his brothers-in-law Armin Dawood`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "his" refers to Yousuf Rassool (the narrator), and the quote states "his brothers-in-law Armin Dawood", we can infer that Armin Dawood is the brother of Yousuf's spouse.

However, since the name of Yousuf's spouse is not mentioned in the quote, we cannot directly state the relationship between Yousuf and Armin. But we can infer that Zobeida Gool (the only female name in the list) might be Yousuf's spouse, making Armin Dawood her brother.

Assuming this, the correct JSON would be:

{"relations":[{"from":"Zobeida Gool","relation":"sibling_of","to":"Armin Dawood"}]}

But since the quote does not directly state Zobeida as Yousuf's spouse, we cannot confirm this relationship. Therefore, according to the rules, we should return an empty JSON.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 114 / 286  (id=-2219204583155630177)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #151

**Entities in chunk:**
  - Abdul Kadar
  - Amod Gool
  - Cape Town
  - Colonial Secretary
  - Durban's Curry's Fountain  (also: Durban)
  - England  (also: England)
  - Fountain Head  (also: Fountain Head)
  - INDEX 166  (also: INDEX 166)
  - Imperialist
  - Johannesburg
  - Joseph Chamberlain
  - Lachiram
  - London
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - South African Indians  (also: South African Indians)
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

## Chunk 115 / 286  (id=-2217941754263052096)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #86

**Entities in chunk:**
  - Broach
  - Cape Town-Woodstock P.T.A.  (also: Cape Town-Woodstock P.T.A)
  - Children
  - Khadija Ebrahim  (also: Khadija)
  - Laurence Olivier
  - League of Nations  (also: League)
  - Let Us Live
  - Parent-Teacher Associations  (also: Parent Teachers Association)
  - Rander
  - Surat
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** in-law

**CC pass (raw):**
```json
{"quote": "his wife Wahida"}
```

**CC quote:** `his wife Wahida`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Wahida Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Wahida Gool`

---

## Chunk 116 / 286  (id=-2088893138668240033)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #608

**Entities in chunk:**
  - Baytencingle
  - Buitencingle  (also: Buitencingle)
  - Byootencingle
  - Cape Town
  - Cassim Amra
  - Chilliwingi  (also: Chilliwingi)
  - District Six  (also: District 6, the District, District Six)
  - Geoff Abrahams  (also: Geoff)
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall  (also: Jamieson Hall)
  - Kloof Nek Road
  - Natallian  (also: Natallian)
  - Reza Rassool Dec
  - Sir Walter Raleigh
  - Standard Seven  (also: Standard Seven)
  - Table Mountain
  - Thomas Bowler
  - Van Riebeeck  (also: Van Riebeeck)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 117 / 286  (id=-2049559656837982089)

**Section:** Yousuf (Joe) Rassool   144  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #924

**Entities in chunk:**
  - Anti-Zionism
  - England  (also: England)
  - Facts
  - Glen
  - Gorki
  - INDEX 166  (also: INDEX 166)
  - Movement
  - New Era Fellowship  (also: NEF)
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

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda"}]}
```

**Extracted relations:** none

---

## Chunk 118 / 286  (id=-2047049700032512222)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #223

**Entities in chunk:**
  - Fletchers & Cartwrights  (also: Fletchers)
  - Glen
  - Hamid Midi
  - INDEX 166  (also: INDEX 166)
  - Lion's Head
  - Muir Street Moslem School
  - Signal Hill
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

## Chunk 119 / 286  (id=-2011986925656463381)

**Section:** Chapter Six    High School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #433

**Entities in chunk:**
  - Bhorat
  - Dickman's Bakery
  - Dollie Salie  (also: Dollie)
  - Donges
  - Dora Taylor
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - Hanover Street
  - Islamic Synagogue  (also: Islamic)
  - Mowbray Cemetry  (also: Mowbray)
  - Mr. Maron  (also: Maron)
  - Mrs Domingo  (also: Domingo)
  - Shaheen Gool  (also: Shaheen)
  - Synagogue  (also: Synagogue)
  - Vandeleur Street
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Waynick  (also: Waynick)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 120 / 286  (id=-2010807931329773554)

**Section:** Yousuf (Joe) Rassool   x  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #19

**Entities in chunk:**
  - British Newspaper Library
  - Chinese Nationalists
  - Colindale
  - Generalissimo Chiang Kai Shek
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - India Ghulzar Khan  (also: India)
  - Kuomintang Party
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Rasheda Rassool  (also: Rasheda)
  - South America  (also: South Africa)
  - Trafalgar  (also: Trafalgar)
  - Zain Rassool  (also: Zain)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

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

## Chunk 121 / 286  (id=-1961234090315831513)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #380

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed  (also: Baboo)
  - Fazil Rassool  (also: Fazil)
  - Halima Gool
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Mr. Khan  (also: Khan)
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

## Chunk 122 / 286  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - British Indian League
  - Buitencingle  (also: Buitencingle)
  - District Six  (also: District 6, the District, District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gokhale
  - Grandpa's tins of rusty nails  (also: Grandpa)
  - Hiroshima
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Japan
  - Kanamia Muslims
  - Kloof Street  (also: Loop Street)
  - Mariam Gool  (also: Margie, Mariam (Margie))
  - Milan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nagasaki
  - Noor Bagh
  - President
  - Shaheen Gool  (also: Shaheen)
  - Union of Socialist Soviet Republics  (also: Union of Socialist Soviet Republics)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

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

## Chunk 123 / 286  (id=-1647678421331218738)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #889

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Cape Town
  - David Poole
  - Dawood Seedat  (also: Dawood)
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Indian Congress
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - President Harry Truman
  - Sadlers Wells Ballet
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** husband, parent, brother,  cousin

**CC pass (raw):**
```json
{"quote": "Fatima, my cousin"}
```

**CC quote:** `Fatima, my cousin`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 124 / 286  (id=-1646081076253426755)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #252

**Entities in chunk:**
  - Buitencingle  (also: Buitencingle)
  - Dale Evans
  - Head  (also: Head Mr)
  - Hewat College  (also: Hewat College)
  - M. Allie  (also: Allie)
  - Mrs Domingo  (also: Domingo)
  - Peninsula Maternity Hospital  (also: Peninsula Maternity Hospital)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 125 / 286  (id=-1594342289169322262)

**Section:** 61  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #331

**Entities in chunk:**
  - Cape African Teachers Association  (also: Cape African Teachers' Association)
  - Dickman's Bakery
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanover Street
  - Islamic Synagogue  (also: Islamic)
  - League of Nations  (also: League)
  - Let Us Live
  - Mr. Maron  (also: Maron)
  - Omar Khayyam
  - Shaheen Gool  (also: Shaheen)
  - Synagogue  (also: Synagogue)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Vandeleur Street
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Waynick  (also: Waynick)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 126 / 286  (id=-1555606746070203337)

**Section:** Chapter Thirteen    Hewat  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #706

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Andrew Mackrill
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - Cardiff
  - Chapel
  - Colin Wynne
  - Cynthia Fisher
  - Dirk Bogarde
  - Fruit and Vegetable Market  (also: Fruit and Vegetable Market)
  - George Veldsman
  - Grandpa's tins of rusty nails  (also: Grandpa)
  - Hewat Amateur Theatrical Society
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Isaac Pfaff
  - Ivan Agherdien
  - J. Arthur Rank Films
  - Leonard Dixon
  - Moslem Mission School  (also: Moslem Mission School)
  - Muir Street
  - Norman Florence
  - One Acts
  - Rasheda Rassool  (also: Rasheda)
  - Sam Wo's Laundry
  - Selkirk Streets
  - Trafalgar Players
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "my sisters Rasheda and Zain"}
```

**CC quote:** `my sisters Rasheda and Zain`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda Rassool"},{"from":"Yousuf Rassool","relation":"sibling_of","to":"Zain Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Rasheda Rassool`
- `Yousuf Rassool` **sibling_of** `Zain Rassool`

---

## Chunk 127 / 286  (id=-1507950876298588482)

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
  - Ralph J Bunche  (also: Ralph Bunche)
  - Solly Edross
  - Trinidad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** married, son , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 128 / 286  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis
  - Dr. DuPlessis  (also: DuPlessis)
  - Edross  (also: Mr Edross)
  - Morris Alexander  (also: Morris)
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 129 / 286  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Bourgeois
  - C.Khatieb  (also: Khatieb)
  - Cannon Street
  - Democratic Parliament
  - Ebrahim  (also: Ebrahim)
  - G.Abed
  - Gool family
  - Hanover Street
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Indian shop
  - Jeddah
  - Joosub Gool  (also: Joosub)
  - Malick Hayat Captain
  - Muddy's brother
  - Qudrat of Allah  (also: Qudrat)
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 130 / 286  (id=-1290623638020229635)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #194

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Arthur Rank
  - British Bioscope
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Fort Hare  (also: Fort Hare)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Moslem school
  - National Theatre  (also: National Theatre)
  - Ossewa Brandwag
  - Paramount
  - Robert Newton
  - Warner Brothers
  - William Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** daughter, father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 131 / 286  (id=-1190264125139023098)

**Section:** Yousuf (Joe) Rassool   96  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #581

**Entities in chunk:**
  - Ali Bey, The Terrible Turk
  - Coloured Affairs Council
  - Europe
  - Guardian
  - Health Department
  - INDEX 166  (also: INDEX 166)
  - Irwin Combrinck  (also: Irwin Combrick)
  - Isaac Millar
  - Jim Londos
  - Kloof Street  (also: Loop Street)
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Orange Street
  - Public Slipper & Turkish Baths (Europeans Only)  (also: Public Slipper & Turkish Baths (Europeans Only))
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "his father"}
```

**CC quote:** `his father`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his father", we can infer that Yousuf Rassool's father is being referred to. However, since the father's name is not mentioned in the quote or the list of known persons, we cannot establish a valid relation.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 132 / 286  (id=-1157812265841671542)

**Section:** 131  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #833

**Entities in chunk:**
  - District Six  (also: District 6, the District, District Six)
  - INDEX 166  (also: INDEX 166)
  - Manuel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 133 / 286  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Dr. Gool
  - Gool  (also: Gool)
  - Grey Street  (also: Bree Street)
  - INDEX 166  (also: INDEX 166)
  - M. Wilson  (also: Wilson)
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Sushila
  - Timmie's sister Gadija  (also: Timmie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** wife, sibling

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 134 / 286  (id=-901522788973038565)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #483

**Entities in chunk:**
  - Cape Technical College  (also: Cape Technical College)
  - D. Neethling  (also: Neethling)
  - Dr. Goolam Gool
  - Dreamy
  - Hanover Street
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Junior Certificate  (also: Senior Certificate)
  - Latin  (also: Latin)
  - Messaris  (also: Mr. Messaris)
  - New Era Fellowship  (also: NEF)
  - Phillis Ntantala Jordan
  - Ralph Taylor
  - S. A. Jayiya  (also: S.A. Jayiya)
  - SOYA
  - Trafalgar  (also: Trafalgar)
  - Tubby/Dreamy  (also: Tubby)
  - W.P. Van Schoor  (also: W.P Van Schoor)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** son ,  cousin

**CC pass (raw):**
```json
{"quote": "a distant cousin of mine, Zeenith Ally"}
```

**CC quote:** `a distant cousin of mine, Zeenith Ally`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 135 / 286  (id=-693280389959675287)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #500

**Entities in chunk:**
  - African National Congress
  - Cape
  - Cape Corps
  - General Smuts
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

## Chunk 136 / 286  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam H. G. Mahomed
  - Bombay University  (also: Bombay)
  - British Justice
  - Cape
  - Charles Simons
  - Gool  (also: Gool)
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - J.M.H.  (also: J.M.H)
  - Molteno  (also: Mr. Molteno)
  - Mr. Schreiner  (also: Schreiner)
  - NUSAS Annual
  - National Union of South African Students  (also: NUSAS)
  - Oriental
  - Parker  (also: Mr. Parker)
  - Pietermaritzburg
  - Richard Parker
  - S.Abed  (also: Abed)
  - Students Representative Council  (also: SRC)
  - Supreme Court
  - University of Natal  (also: University of Natal)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "the entire Abed family"}
```

**CC quote:** `the entire Abed family`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "the entire Abed family", we can infer that Yousuf Rassool is a member of the Abed family. However, the exact relationship between Yousuf Rassool and other members of the Abed family is not specified.

But since S.Abed is mentioned in the list of known persons, we can assume that S.Abed is a member of the Abed family. Therefore, we can infer that Yousuf Rassool is a sibling of S.Abed.

Here is the extracted relationship in JSON format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"S.Abed"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `S.Abed`

---

## Chunk 137 / 286  (id=-588616802558725797)

**Section:** Yousuf (Joe) Rassool   10  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #85

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - Calcutta
  - District Six  (also: District 6, the District, District Six)
  - Harold Cressy
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion Feb
  - Mohammed Saaid Uncle Aity
  - National Gandhi Museum JMH Gool
  - Solomon Tshekisho Plaatje
  - St Martini German Lutheran Church  (also: St Martini German Lutheran Church)
  - Trafalgar High  (also: Trafalgar High)

**Triggers found:** in-law

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 138 / 286  (id=-509545831926091450)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #160

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Kadar
  - Amod Gool
  - Gandhi's family
  - Guy's Hospital  (also: Guys Hospital)
  - INDEX 166  (also: INDEX 166)
  - Lachiram
  - London
  - M. Wilson  (also: Wilson)
  - Messrs Yusuf Hamid Gool
  - Mr Yusuf Hamid Gool
  - Mrs Wooding  (also: Wooding)
  - Rev Dan Wessels  (also: Rev. Dan Wessels)
  - Tregger  (also: Mr. Tregger)
  - Unity Movement  (also: Unity)
  - Van der Ross
  - Vassan Hindu C.C.  (also: Vassan)
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

## Chunk 139 / 286  (id=-441481017488792687)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #862

**Entities in chunk:**
  - African Affairs
  - Anti-Apartheid
  - Gava-Kadoes  (also: Gava)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Purcell  (also: Purcell)
  - Ralph J Bunche  (also: Ralph Bunche)
  - South America  (also: South Africa)
  - Torch Commando  (also: Torch)
  - Train Apartheid Resistance
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

## Chunk 140 / 286  (id=-311470557972189499)

**Section:** Yousuf (Joe) Rassool   106  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #647

**Entities in chunk:**
  - B. Allies  (also: Allies)
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - Cannon Street
  - Cape Town
  - Cape Town Fire Brigade
  - District Six  (also: District 6, the District, District Six)
  - Grand Parade
  - Heerie  (also: Heerie)
  - INDEX 166  (also: INDEX 166)
  - M. Allie  (also: Allie)
  - Mr. B. Allie  (also: B. Allie)
  - Nasim Rassool  (also: Nasim)
  - Rutger Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** wife, mother

**CC pass (raw):**
```json
{"quote": "Mr. B. Allie and his sons"}
```

**CC quote:** `Mr. B. Allie and his sons`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Mr. B. Allie","relation":"parent_of","to":"B. Allies"}]}
```

**Extracted relations:**
- `Mr. B. Allie` **parent_of** `B. Allies`

---

## Chunk 141 / 286  (id=-283660553274171266)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #76

**Entities in chunk:**
  - Derek Prentice
  - Habibia Institute  (also: Habibia Institute)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX 166)
  - Inevitably
  - Mecca
  - Silvertown
  - South African Broadcasting Corporation
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** father, parent, sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 142 / 286  (id=-272192624856094471)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #256

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Caledon
  - Hanief Aboeta
  - Health Department
  - Main Road
  - Mount Streets  (also: Mount Street)
  - Rondebosch
  - Transkei
  - University of Cape Town  (also: University of Cape Town)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "as I had been named after his father"}
```

**CC quote:** `as I had been named after his father`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 143 / 286  (id=-155738895990866423)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #710

**Entities in chunk:**
  - Assistant Secretary-General
  - Buitencingle  (also: Buitencingle)
  - Dover
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - INDEX 166  (also: INDEX 166)
  - Israel  (also: Israel)
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Movement
  - Palestine
  - State of Israel  (also: State)
  - United Nations
  - Zionism
  - Zionist State  (also: Zionist State)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 144 / 286  (id=-136984792424007029)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1058

**Entities in chunk:**
  - Anti-Coloured Affairs Council (Anti-CAC)
  - Anti-Coloured Affairs Department (Anti-CAD)
  - Castle Division
  - Coloured branch of Smuts's United Party
  - Communist Party
  - David Bloomberg
  - District 6 Ward
  - Fazil Rassool  (also: Fazil)
  - Fourth International of South Africa
  - General Smuts's United Party
  - Harry Snitcher
  - Liberation League
  - Party of South Africa  (also: Party)
  - Practical Science
  - Queens Road, Woodstock
  - Salt River Primary school
  - Smuts's United Party  (also: Smuts)
  - Unity Movement  (also: Unity)
  - Workers' Party  (also: Workers Party)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** sister, brother

**CC pass (raw):**
```json
{"quote": "my sister, Zain"} and {"quote": "my brother Fazil"} 

However, according to your rules, only one quote should be returned.

{"quote": "my sister, Zain"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 145 / 286  (id=-93354464964914605)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #72

**Entities in chunk:**
  - 5th Baron
  - Alice Greene
  - All India Football team
  - Buitencingle  (also: Buitencingle)
  - Cape Town train station
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - George Bernard Shaw
  - Grahame Greene
  - INDEX 166  (also: INDEX 166)
  - Indian Agents General
  - Kimberley
  - Lord Headley
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Moravian church  (also: Moravian)
  - Oddfellows Hall
  - Prince of Wales
  - Rev. Wessels  (also: Wessels)
  - Sarojini Naidu
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 146 / 286  (id=-85130217379609273)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1059

**Entities in chunk:**
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - Fazil Rassool  (also: Fazil)
  - General Smuts
  - Louis Botha
  - MacAnthony
  - Prince of Wales
  - Queens Road
  - Tiny Remarkably
  - Victor Wessels
  - Woodstock
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, brother

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

## Chunk 147 / 286  (id=31492473950651402)

**Section:** 183  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1150

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Cape Flats
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Dija
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Students Representative Council  (also: SRC)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - United Nations
  - Windermere
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "Abdul Hamid, a decade into his marriage with Cissie"}
```

**CC quote:** `Abdul Hamid, a decade into his marriage with Cissie`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Hamid Gool","relation":"spouse_of","to":"Cissie Gool"}]}
```

**Extracted relations:**
- `Abdul Hamid Gool` **spouse_of** `Cissie Gool`

---

## Chunk 148 / 286  (id=32633455946576169)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #319

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Abba
  - Brian Willan
  - Buitencingle  (also: Buitencingle)
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
  - Cape Technical College  (also: Cape Technical College)
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

## Chunk 149 / 286  (id=256564604167810176)

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
  - M. Wilson  (also: Wilson)
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

## Chunk 150 / 286  (id=354692539872206818)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #614

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - City
  - House of Assembly
  - INDEX 166  (also: INDEX 166)
  - Khadija Ebrahim  (also: Khadija)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Parow
  - Peace
  - Strand
  - Trojan
  - Zarina Rassool  (also: Zarina)

**Triggers found:** sister,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "my cousins in the Strand, and those in Parow: Naz, and her twin sisters Zarina and Nurjehan"}
```

**CC quote:** `my cousins in the Strand, and those in Parow: Naz, and her twin sisters Zarina and Nurjehan`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}

This is because the quote mentions "my cousins", which is a relation type that is not in the schema. According to the critical rules, if the quote uses 'cousin', return {"relations":[]}.
```

**Extracted relations:** none

---

## Chunk 151 / 286  (id=501428138843038233)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #511

**Entities in chunk:**
  - INDEX 166  (also: INDEX 166)
  - Morris Alexander  (also: Morris)
  - New Era Fellowship  (also: NEF)
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

## Chunk 152 / 286  (id=519016872720201104)

**Section:** Yousuf (Joe) Rassool   98  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #589

**Entities in chunk:**
  - B. Allies  (also: Allies)
  - Germany  (also: Germany)
  - Hitler  (also: Hitler)
  - Khadija Ebrahim  (also: Khadija)
  - Mecca
  - Myrtle  (also: Myrtle)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , brother

**CC pass (raw):**
```json
{"quote": "Khadija’s young brother"}
```

**CC quote:** `Khadija’s young brother`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Khadija Ebrahim"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Khadija Ebrahim`

---

## Chunk 153 / 286  (id=600429570805554003)

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

## Chunk 154 / 286  (id=609470002235389128)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #961

**Entities in chunk:**
  - BM.Kies  (also: BM Kies)
  - Berlin
  - Cathedral Hall
  - Clara Petacci
  - District Six  (also: District 6, the District, District Six)
  - Hamid
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Luneberg Heath
  - Milan
  - Mussolini
  - Plein Street
  - Queen Victoria Street
  - South America  (also: South Africa)
  - Transvaal  (also: Transvaal)
  - Union of Socialist Soviet Republics  (also: Union of Socialist Soviet Republics)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "my Uncle Mamoo"}
```

**CC quote:** `my Uncle Mamoo`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 155 / 286  (id=615438439207399386)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #149

**Entities in chunk:**
  - Amra
  - Bertie Louw
  - Cape Town
  - Cassim Amra
  - Communist Party
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hyde Park
  - Ismail  (also: Ismail)
  - M. Allie  (also: Allie)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 156 / 286  (id=628953725132054043)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #605

**Entities in chunk:**
  - Cape Technical College  (also: Cape Technical College)
  - Cape Town
  - Dreamy
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Junior Certificate  (also: Senior Certificate)
  - Latin  (also: Latin)
  - Messaris  (also: Mr. Messaris)
  - Trafalgar  (also: Trafalgar)
  - Tubby/Dreamy  (also: Tubby)
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

## Chunk 157 / 286  (id=641330737843452673)

**Section:** Yousuf (Joe) Rassool   120  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #748

**Entities in chunk:**
  - Abe Desmore
  - David Poole
  - District Six  (also: District 6, the District, District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Grand Parade
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Liberals
  - Mrs. Wo  (also: Mrs Wo)
  - Nerine Desmond
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Regals
  - TARC  (also: TARC)
  - Trafalgar Junior School  (also: Trafalgar Junior School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 158 / 286  (id=776867616384834388)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #603

**Entities in chunk:**
  - Adderley Street
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

## Chunk 159 / 286  (id=787736854281663344)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #140

**Entities in chunk:**
  - Asia
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Jack Meltzer
  - Law 3 of 1885
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mohammedan
  - Mr. Meltzer  (also: Meltzer)
  - South African Republic
  - Trafalgar High School  (also: Trafalgar High School)
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

## Chunk 160 / 286  (id=861412338268840709)

**Section:** 51  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #263

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Ahmed Abdurahman
  - Appollis Slingers  (also: Appolis Slingers)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Bonny Bridge Dover
  - Buitengracht Street
  - Fort Hare  (also: Fort Hare)
  - G.H.
  - Harold Cressy
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Muir Street Moslem School
  - St Paul's School
  - Wesley Training School
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** daughter, mother, gave birth

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 161 / 286  (id=1092111527653010327)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #342

**Entities in chunk:**
  - Achmat Moortjie
  - Achmed
  - District Nurses
  - Even Buitencingle
  - INDEX 166  (also: INDEX 166)
  - Khan's cafe
  - Lalla's Cafe  (also: Lalla)
  - Millards  (also: Millards)
  - Professor James  (also: Prof. James)
  - Tubby/Dreamy  (also: Tubby)

**Triggers found:** father, mother

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 162 / 286  (id=1146286345936890093)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #462

**Entities in chunk:**
  - A. Arendze  (also: Arendze)
  - A. Banoo  (also: Banoo)
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Aminabhen
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool
  - Anti-CAD  (also: Anti CAD)
  - Anti-Coloured Affairs Council
  - Anti-Coloured Affairs Department
  - Anti-Indian Laws
  - Anti-TLSA
  - Anti-Zionism
  - Apartheid and Indian South Africa
  - Appollis Slingers  (also: Appolis Slingers)
  - Argus Annual
  - Armin Dawood
  - Arms and the Man
  - Arthur Hugh Clough
  - Arthur Tracey
  - Athol Rowan
  - Avalon Cinema
  - Avondale Road
  - Babla Salloogee  (also: Babla Salloogee)
  - Baboo Abed  (also: Baboo)
  - Caledon Square
  - Dr. Ramamurthi  (also: Ramamurthi)
  - George
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 163 / 286  (id=1153175004995048754)

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
  - Kader Essack  (also: Kader)
  - Kassiem
  - Muir Street
  - Poppie
  - Ronnie Jooste  (also: Ronnie)
  - Shariefa Khan
  - Slingers  (also: Mr. Slingers)
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

## Chunk 164 / 286  (id=1226620700780836565)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #600

**Entities in chunk:**
  - Cape Town
  - City Council
  - INDEX 166  (also: INDEX 166)
  - Noors  (also: Noors)
  - Sea Point
  - Strand
  - White Areas

**Triggers found:** wed ,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "My aunt Cissie Gool"}
```

**CC quote:** `My aunt Cissie Gool`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 165 / 286  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Barnato Union
  - Ben Malamba
  - Cape Town
  - Curry's Fountain
  - Even Buitencingle
  - INDEX 166  (also: INDEX 166)
  - National Union of South African Students  (also: NUSAS)
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

## Chunk 166 / 286  (id=1340028930833342443)

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
  - South African Indians  (also: South African Indians)
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

## Chunk 167 / 286  (id=1457016325443525292)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #951

**Entities in chunk:**
  - Anti-CAD  (also: Anti CAD)
  - Building and Basis of Unity
  - Charles Starret
  - Club
  - Dale Evans
  - Even Buitencingle
  - INDEX 166  (also: INDEX 166)
  - Johnny Mack Brown
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

## Chunk 168 / 286  (id=1515453391350648521)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #27

**Entities in chunk:**
  - Cape Colony
  - Coloured Advisory Council
  - George Golding
  - Gool  (also: Gool)
  - Hamid
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX 166)
  - Ismail Hayat
  - Jameel
  - M. Wilson  (also: Wilson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Transvaalers  (also: Transvaalers)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 169 / 286  (id=1582519559517716798)

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

## Chunk 170 / 286  (id=1653401825939138750)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #261

**Entities in chunk:**
  - Afr Gandhi
  - Bibi  (also: Bibi)
  - First Lallie
  - Henry Sylvester Williams
  - INDEX 166  (also: INDEX 166)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Owen Mathurin
  - Peninsula Maternity Hospital  (also: Peninsula Maternity Hospital)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "his frailty made it a farce is not a qualifying clause; however, the passage contains another clause that identifies a direct family relationship: "} 
{"quote": "this misbegotten son of his"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 171 / 286  (id=1654270249304850758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #478

**Entities in chunk:**
  - Aminabhen
  - Armin Dawood
  - Belgium  (also: Belgium)
  - Bibi  (also: Bibi)
  - Cassiem  (also: Cassiem)
  - Denmark  (also: Denmark)
  - District Six  (also: District 6, the District, District Six)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - France  (also: France)
  - Hamza
  - Holland  (also: Holland)
  - INDEX 166  (also: INDEX 166)
  - Norway  (also: Norway)
  - Queens Road
  - Woodstock
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband,  uncle 

**CC pass (raw):**
```json
{"quote": "Auntie Jolly’s husband"}
```

**CC quote:** `Auntie Jolly’s husband`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "Auntie Jolly's husband" is mentioned and the narrator is Yousuf Rassool, we can infer that Auntie Jolly is a relative of Yousuf. However, the quote does not provide any information about the relationship between Yousuf and Auntie Jolly's husband.

But since Auntie Jolly is likely to be an aunt of Yousuf (given the use of "Auntie"), we can infer that her husband would be Yousuf's uncle by marriage. However, according to the rules, 'uncle' is not a valid relation type in this schema.

Therefore, the correct answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 172 / 286  (id=1690720582947668658)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1139

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Afro-Caribbean Community
  - Britain's Empire  (also: Britain)
  - Cape Town
  - Desmond Green
  - Greenpoint
  - India Ghulzar Khan  (also: India)
  - Israel  (also: Israel)
  - James M. Wilson
  - Journal of Southern African Studies
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Narutham Godse
  - Pakistan
  - Rosneath Villa
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
Since the narrator is "Yousuf Rassool" and the quote mentions "his wife", we can infer that Yousuf Rassool is the husband of his wife. However, since the name of the wife is not mentioned in the quote, we cannot establish a valid relation.

According to the rules, if both endpoints are not identifiable as canonical names, return empty.

Therefore, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 173 / 286  (id=1731727716997051785)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #153

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - British Government
  - Dada Abdullah
  - Disfranchisement Law
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX 166)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Natal Government
  - Natal Indian Congress
  - Pretoria Gandhi  (also: Pretoria)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 174 / 286  (id=1742620772122585209)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #204

**Entities in chunk:**
  - Abass (Tagara)  (also: Abass)
  - Athlone
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - E.A. Parker
  - Habibia Institute  (also: Habibia Institute)
  - Magdalena Isabella
  - Malick Rassool
  - Muir Street
  - Neville Alexander
  - Noors  (also: Noors)
  - Peter Alexander
  - Vandeleur Street
  - Wesleyan Church
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, mother, parent, brother

**CC pass (raw):**
```json
{"quote": "her brother, Peter Alexander"}
```

**CC quote:** `her brother, Peter Alexander`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Peter Alexander","relation":"sibling_of","to":"Magdalena Isabella"}]}
```

**Extracted relations:**
- `Peter Alexander` **sibling_of** `Magdalena Isabella`

---

## Chunk 175 / 286  (id=1772720298534929784)

**Section:** 5  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #63

**Entities in chunk:**
  - Aunty Minnie
  - District Six  (also: District 6, the District, District Six)
  - Edross  (also: Mr Edross)
  - Galieyah
  - Gujarat
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Jimmy La Guma
  - Liberty Cavalcade
  - Natal
  - Rander
  - Roux Senior
  - Sharon Parker  (also: Ms Sharon Parker)
  - Swat Province  (also: Swat)
  - Transvaal  (also: Transvaal)
  - Van Riebeeck  (also: Van Riebeeck)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, son , father

**CC pass (raw):**
```json
{"quote": "His father was a Pushto speaking Maulvi, or religious leader, who had moved from the Province of Swat to Rander in Gujarat with his wife and two sons, Abbas and Yusuf."}
```

**CC quote:** `His father was a Pushto speaking Maulvi, or religious leader, who had moved from the Province of Swat to Rander in Gujarat with his wife and two sons, Abbas and Yusuf.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{
"from":"Yousuf Rassool",
"relation":"child_of",
"to":"Haji Joosub Maulvi Hamid Gool"
},{
"from":"Abbas",
"relation":"child_of",
"to":"Haji Joosub Maulvi Hamid Gool"
},{
"from":"Yousuf Rassool",
"relation":"sibling_of",
"to":"Abbas"
}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Haji Joosub Maulvi Hamid Gool`

---

## Chunk 176 / 286  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Durban's Curry's Fountain  (also: Durban)
  - Group Areas Act
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Jameel
  - Mount Streets  (also: Mount Street)
  - Nellie Abdurahman
  - Omar
  - South America  (also: South Africa)
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

## Chunk 177 / 286  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Argus Annual
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town
  - Dullah Omar
  - Dullie Desai
  - Even Buitencingle
  - Good Hope
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Kies  (also: Mr Kies)
  - Morris Alexander  (also: Morris)
  - Movement
  - New Era Fellowship  (also: NEF)
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

## Chunk 178 / 286  (id=2012893071791774830)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #200

**Entities in chunk:**
  - Biology Lab
  - Calcutta
  - Eddie Roux  (also: Eddie)
  - Geoff Abrahams  (also: Geoff)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Jane Abrahams
  - Mowbray Cemetry  (also: Mowbray)
  - Poppie
  - Rander
  - Religious Instruction
  - Shaheen Gool  (also: Shaheen)
  - South America  (also: South Africa)
  - Trafalgar  (also: Trafalgar)
  - Tuan Koebers
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "my grandfather’s"}
```

**CC quote:** `my grandfather’s`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote uses 'my', we can infer that the narrator is referring to themselves. The quote states "my grandfather's", which implies a family relationship.

However, since the schema only allows for specific relation types (spouse_of, parent_of, child_of, sibling_of, half_sibling_of), and "grandfather" does not fit into any of these categories, we cannot extract a valid relation.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 179 / 286  (id=2083741976242093100)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #179

**Entities in chunk:**
  - Bambatta  (also: Bambatta)
  - Collected  (also: Collected)
  - Constantia Road
  - E. Reddy  (also: Reddy)
  - Herrenvolk
  - Hymie Beimel  (also: Hymie)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Movement
  - Phillis Ntantala Jordan
  - Port Elizabeth
  - Pretoria Gandhi  (also: Pretoria)
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

## Chunk 180 / 286  (id=2093660615865932548)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #397

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Aligarh College  (also: Aligarh)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle  (also: Buitencingle)
  - Cape Town
  - Choir
  - Fazil Rassool  (also: Fazil)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Nurjahan
  - Rashid
  - Shawquet
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf
  - Zaheer

**Triggers found:**  aunt , gave birth

**CC pass (raw):**
```json
{"quote": "my aunt gave birth to all the rest of her children at 95 Caledon Street: Rashid, Zaheer, Nurjahan and Yusuf (named after me in the hope it would bring him luck)"}

This clause identifies a direct family relationship between Yousuf Rassool (the narrator) and Yusuf, where Yusuf is named after the narrator.
```

**CC quote:** `my aunt gave birth to all the rest of her children at 95 Caledon Street: Rashid, Zaheer, Nurjahan and Yusuf (named after me in the hope it would bring him luck)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 181 / 286  (id=2104188275539776644)

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
  - Motala  (also: Motala)
  - Mount Nelson Hotel  (also: Mount Nelson Hotel)
  - Mount Streets  (also: Mount Street)
  - South America  (also: South Africa)
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

## Chunk 182 / 286  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - Bibi  (also: Bibi)
  - Cape Town
  - Communist Party
  - Group Areas  (also: Group Area)
  - Jaffeism
  - Kies  (also: Mr Kies)
  - Land Tenure Advisory Board
  - Liberals
  - Red China  (also: Red China)
  - Slingers  (also: Mr. Slingers)
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

## Chunk 183 / 286  (id=2151609175130799335)

**Section:** Yousuf (Joe) Rassool   118  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #728

**Entities in chunk:**
  - Bloomberg
  - David Bloomberg
  - Department of Education  (also: Department)
  - Far East
  - Gates of India
  - Government
  - Jan Christiaan Smuts
  - Japan
  - Moravian church  (also: Moravian)
  - Natal
  - New Era Fellowship  (also: NEF)
  - Rev. Wessels  (also: Wessels)
  - Reza Rassool  (also: Reza)
  - Smuts's United Party  (also: Smuts)
  - Unity Movement  (also: Unity)
  - Victor Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 184 / 286  (id=2157482743913839493)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #698

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Caledon Streets  (also: Caledon Street)
  - Congress
  - Gava-Kadoes  (also: Gava)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hiema
  - India Ghulzar Khan  (also: India)
  - Indian Congress
  - Mohamed Ali Jinnah
  - Mohammed Essop
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muslims C.C.  (also: Muslims)
  - Musto
  - Parade
  - Parow
  - Passive Resistance
  - Queens Road
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

## Chunk 185 / 286  (id=2190457622431566710)

**Section:** Yousuf (Joe) Rassool   36  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #183

**Entities in chunk:**
  - Barolong
  - Caledon Streets  (also: Caledon Street)
  - District Six  (also: District 6, the District, District Six)
  - Dutchmen
  - Gandhi's Son Manilal
  - George Meissenheimer  (also: George Meisenheimer)
  - Hamid Khan
  - His Majesty
  - Liesbeek River
  - Life
  - Mahatma Gandhi Vol 35
  - Natal
  - National Gandhi Museum Comaroff
  - Pretoria Gandhi  (also: Pretoria)
  - Stephens
  - Thumbulingam
  - Trafalgar  (also: Trafalgar)
  - Trafalgar George
  - Van Haacht
  - Western Province  (also: Eastern Province)
  - Willan
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 186 / 286  (id=2228032443713191255)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #55

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Town's most famous regiment
  - Chapel Street
  - City Hall  (also: City Hall)
  - Dora Taylor
  - Eva Sachs
  - Fred Bodmer
  - General Smuts
  - Gregoire Boonzaaier
  - J.G.  (also: J.G)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Messaris  (also: Mr. Messaris)
  - Paul Kostin
  - Searle Street
  - Terence Macaw
  - Thence
  - Tobruk
  - Tykie Messaris
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

## Chunk 187 / 286  (id=2263436661745583446)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1024

**Entities in chunk:**
  - Grand Parade
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX 166)
  - Ronald Heinrichsen
  - South America  (also: South Africa)
  - Stall number 12
  - Three W  (also: Three W)
  - Walcott
  - Weekes
  - West Indian Cricket
  - Worrel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 188 / 286  (id=2310160307929513794)

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
  - National Union of South African Students  (also: NUSAS)
  - Town Hall
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother, brother,  uncle 

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

## Chunk 189 / 286  (id=2442521115231206495)

**Section:** Yousuf (Joe) Rassool   118  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #729

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Broederbond
  - Byootencingle
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - Dr. Malan  (also: Dr. Malan)
  - France  (also: France)
  - Herrenvolk
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek Road
  - Prison House
  - Reza Rassool Dec
  - South America  (also: South Africa)
  - Table Mountain
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Thomas Bowler
  - World
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 190 / 286  (id=2558316411945156110)

**Section:** 109  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #670

**Entities in chunk:**
  - All African Convention  (also: AAC, All-African Convention)
  - Cape Town
  - Genadendal
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Malmesbury
  - New Era Fellowship  (also: NEF)
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Rev Dan Wessels  (also: Rev. Dan Wessels)
  - Victor Walter Wesley Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , son 

**CC pass (raw):**
```json
{"quote": "the son of Rev. Dan Wessels"}
```

**CC quote:** `the son of Rev. Dan Wessels`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Victor Walter Wesley Wessels","relation":"child_of","to":"Rev Dan Wessels"}]}
```

**Extracted relations:**
- `Victor Walter Wesley Wessels` **child_of** `Rev Dan Wessels`

---

## Chunk 191 / 286  (id=2736700062244384633)

**Section:** Yousuf (Joe) Rassool   54  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #277

**Entities in chunk:**
  - Abass (Tagara)  (also: Abass)
  - Cape Town
  - Magdalena Isabella
  - Malick Rassool
  - Noors  (also: Noors)
  - Peter Alexander Although
  - Queen Victoria Street
  - Tagara
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

## Chunk 192 / 286  (id=2774912283328464312)

**Section:** 47  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #235

**Entities in chunk:**
  - Bobby Breen
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - District Six Adonis
  - Isvarlal
  - Itie
  - Manilal Gandhi  (also: Manilal)
  - Manny
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Rainbow on The River  (also: Rainbow)
  - Sakarlal
  - Tyne and Roger Streets  (also: Tyne)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 193 / 286  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - District Six  (also: District 6, the District, District Six)
  - Doxology
  - Fazil Rassool  (also: Fazil)
  - Ismail  (also: Ismail)
  - Kelvinator
  - Rasheda Rassool  (also: Rasheda)
  - Rev. Wessels  (also: Wessels)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 194 / 286  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Cape Argus
  - Ethnic Pride
  - Golding Van  (also: Golding)
  - Green Point Commons  (also: Green Point Common)
  - Group
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - Israel  (also: Israel)
  - M. Wilson  (also: Wilson)
  - Menachem Begin
  - Minister
  - New Era Fellowship  (also: NEF)
  - Pogrund  (also: Pogrund)
  - Racial Prejudice
  - Regan
  - Reuben Pogrund
  - Reverend Gow  (also: Gow)
  - Van der Ross
  - Victorian Cape Town
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

## Chunk 195 / 286  (id=2945565054281625486)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #108

**Entities in chunk:**
  - All African Convention Messrs  (also: All African Convention Messrs.)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Enver Hassim
  - Errol Vawda
  - Glories  (also: Glories)
  - INDEX 166  (also: INDEX 166)
  - James Shirley
  - Kader Essack  (also: Kader)
  - Kings Sceptre  (also: Kings)
  - Mauritius
  - Natal
  - Sharon Parker  (also: Ms Sharon Parker)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:** wife, husband, father, parent,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 196 / 286  (id=2975384613280766645)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #955

**Entities in chunk:**
  - Ben Malamba
  - Dr. Verwoerd  (also: Dr Verwoerd)
  - Eiselen man
  - Head Student
  - Heathrows C.C.  (also: Heathrows C.C)
  - INDEX 166  (also: INDEX 166)
  - Minister of Native Affairs
  - Muslims C.C.  (also: Muslims)
  - Paddy Thomas
  - Salie Van Haacht
  - Seventh Day Adventists
  - Stanley Abrahams
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 197 / 286  (id=3029290522947892988)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #657

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - Cape
  - Cassim Amra
  - General Smuts
  - Johannesburg
  - Joss
  - League of Nations  (also: League)
  - Louis Botha
  - South African Party
  - United Party
  - University of Witwatersrand
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 198 / 286  (id=3036940243594659710)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #715

**Entities in chunk:**
  - Allie BA
  - Dollies
  - Habibia Institute  (also: Habibia Institute)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX 166)
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

## Chunk 199 / 286  (id=3081245385979381949)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #249

**Entities in chunk:**
  - Achee
  - Bibi  (also: Bibi)
  - Hayat Achee
  - Head  (also: Head Mr)
  - Hewat College  (also: Hewat College)
  - INDEX 166  (also: INDEX 166)
  - Isak Esmail
  - Isak Issy
  - Ismail  (also: Ismail)
  - Katie
  - M. Allie  (also: Allie)
  - Mauritius
  - Miss Brown
  - Mrs Domingo  (also: Domingo)
  - Worcester

**Triggers found:** husband, mother,  uncle 

**CC pass (raw):**
```json
{"quote": "She was the girl who came to look after Auntie Jolly’s baby son, Ismail, when she and Uncle Mamoo, her husband, went to Mauritius for a holiday."}
```

**CC quote:** `She was the girl who came to look after Auntie Jolly’s baby son, Ismail, when she and Uncle Mamoo, her husband, went to Mauritius for a holiday.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 200 / 286  (id=3093788627293641191)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #339

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - Cannon Street
  - Cape Times
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - George Golding
  - Harold Wolpe
  - High School  (also: High School)
  - Hogwood  (also: Mr Hogwood)
  - Jackie Marks
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

## Chunk 201 / 286  (id=3099837090720220907)

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
  - India Ghulzar Khan  (also: India)
  - Local History Museum
  - Manilal Gandhi  (also: Manilal)
  - Phoenix
  - Ramdas
  - Ritchie
  - South African 2nd Division
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

## Chunk 202 / 286  (id=3280112095965555810)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #224

**Entities in chunk:**
  - Agent General for India
  - Bibi  (also: Bibi)
  - Dija
  - Honourable Sir Sayed Raza Ali
  - INDEX 166  (also: INDEX 166)
  - Mother Nature
  - Nasim Rassool  (also: Nasim)
  - School
  - Trafalgar High School  (also: Trafalgar High School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 203 / 286  (id=3336594518095080251)

**Section:** Yousuf (Joe) Rassool   64  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #349

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Choir
  - Hoosain Ally  (also: Hussain Ally)
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
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

## Chunk 204 / 286  (id=3445648479411821512)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #628

**Entities in chunk:**
  - All AJ Abrahamse Lectures
  - BM.Kies  (also: BM Kies)
  - Babla Salloogee  (also: Babla Salloogee)
  - Berlin
  - Cathedral Hall
  - Clara Petacci
  - District Six  (also: District 6, the District, District Six)
  - Germans  (also: Germans)
  - Hitler  (also: Hitler)
  - INDEX 166  (also: INDEX 166)
  - John Vorster Prison
  - Luneberg Heath
  - Milan
  - Mussolini
  - Pretoria Gandhi  (also: Pretoria)
  - Queen Victoria Street
  - Red Army
  - South America  (also: South Africa)
  - Union of Socialist Soviet Republics  (also: Union of Socialist Soviet Republics)
  - Unity Movement  (also: Unity)
  - Victor
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

## Chunk 205 / 286  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Barnato Union
  - Ben Malamba
  - Bibi  (also: Bibi)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
  - Heathrow Cricket Club
  - INDEX 166  (also: INDEX 166)
  - Indian Cricket Union
  - Kismets Cricket Club  (also: Kismet Cricket Club)
  - Salie Van Haacht
  - Seventh Day Adventists
  - Sonny Abdurahman
  - Stanley Abrahams
  - Students Representative Council  (also: SRC)
  - Wagieda
  - Woody Woodman  (also: Woody)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 206 / 286  (id=3621060795652394497)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #619

**Entities in chunk:**
  - Abe Desmore
  - Azad
  - David Poole
  - District Six  (also: District 6, the District, District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Mrs. Wo  (also: Mrs Wo)
  - Nerine Desmond
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Regals
  - Rehana Mohamed
  - Trafalgar Junior School  (also: Trafalgar Junior School)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "my cousin"}
```

**CC quote:** `my cousin`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 207 / 286  (id=3651484030627311178)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #248

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Capetown
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Ismail  (also: Ismail)
  - Mauritius
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - Non-European Unity Movement  (also: NEUM, Non European Unity Movement)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** husband, mother, parent,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "Auntie Jolly’s baby son, Ismail"}
```

**CC quote:** `Auntie Jolly’s baby son, Ismail`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "Auntie Jolly" is mentioned but not in the list of known persons, and "Ismail" is a known person, we can only extract one side of the relationship. However, according to the rules, if both endpoints are not identifiable as canonical names, return empty.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 208 / 286  (id=3654419630807047063)

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
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Trafalgar  (also: Trafalgar)
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

## Chunk 209 / 286  (id=3683676736687558285)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #789

**Entities in chunk:**
  - B. Allies  (also: Allies)
  - Generals Montgomery  (also: Generals Montgomery)
  - George Golding
  - Habibia  (also: Habibia)
  - Hassiem Edross
  - Normandy
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Gow)
  - Salie Dollie
  - Solly Edross
  - Trafalgar  (also: Trafalgar)
  - Von Kesselring
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "He was related to Solly Edross, my Maths teacher at Trafalgar."}
```

**CC quote:** `He was related to Solly Edross, my Maths teacher at Trafalgar.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 210 / 286  (id=3694579935737916495)

**Section:** Chapter Three  Declaration of War  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #367

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Bibi  (also: Bibi)
  - Cape Town Woodstock
  - District Six  (also: District 6, the District, District Six)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Leather
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Parent-Teacher Associations  (also: Parent Teachers Association)
  - Somehow  (also: Somehow)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 211 / 286  (id=3698504895570842680)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #257

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Beimel
  - Buitencingle  (also: Buitencingle)
  - Communist Party
  - Forum Club
  - Goebbels
  - INDEX 166  (also: INDEX 166)
  - Kenny Jordaan
  - Liberals
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Native Representatives  (also: Native Representative)
  - Parliament  (also: Parliament)
  - Red China  (also: Red China)
  - TARC  (also: TARC)
  - Teachers League
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

## Chunk 212 / 286  (id=3707629011541248812)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #29

**Entities in chunk:**
  - Cape Town
  - Chapel Street Primary  (also: Chapel Street Primary)
  - District Six  (also: District 6, the District, District Six)
  - England  (also: England)
  - Habibia Institute  (also: Habibia Institute)
  - Herrenvolk
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Indian Opinion
  - Port Elizabeth
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

## Chunk 213 / 286  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Afrikaans Language Movement  (also: Afrikaans)
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

## Chunk 214 / 286  (id=3728742021027485580)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #974

**Entities in chunk:**
  - Clan
  - Gool  (also: Gool)
  - INDEX 166  (also: INDEX 166)
  - Ivan Denisovich
  - Kies  (also: Mr Kies)
  - Mecca
  - National Anti-CAD
  - New Era Fellowship  (also: NEF)
  - Sunday  (also: Sunday)
  - Synagogue  (also: Synagogue)
  - Tabata
  - Vandeleur Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 215 / 286  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi  (also: Bibi)
  - Cape Town Docks
  - Gool  (also: Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jadwat
  - M. Wilson  (also: Wilson)
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
  - Party of South Africa  (also: Party)
  - Port Elizabeth Indians
  - Ruth Alexander
  - Seedat
  - USSR  (also: USSR)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 216 / 286  (id=4167963450933261137)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1055

**Entities in chunk:**
  - Baruch Hirson
  - Chapel Street
  - European Left
  - Le Grange
  - Mehuyzen
  - Mr Tabata
  - National Party
  - Oosthuisen  (also: Oosthuisen)
  - Rev. Wessels  (also: Wessels)
  - Rosmead Sportsground  (also: Rosmead)
  - Rousseau
  - Sidney Viret
  - South America  (also: South Africa)
  - Tykes  (also: Tykes)
  - Unity Movement  (also: Unity)
  - Van Der Merwe
  - Van der Westhuisen
  - Viljoen
  - Vollenhoven
  - Wally Hendricks
  - Wynberg

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 217 / 286  (id=4179969105113206089)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #867

**Entities in chunk:**
  - Fazil Rassool  (also: Fazil)
  - George Munsook
  - Habibia Primary School  (also: Habibia Primary School)
  - Hoosain Parker
  - Hudson  (also: Hudson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nehru
  - New Era Fellowship  (also: NEF)
  - Stakesby-Lewis Hostel  (also: Stakesby-Lewis Hostel)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 218 / 286  (id=4197470860421711371)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #93

**Entities in chunk:**
  - Abdurahman
  - Alexander
  - Aminabhen
  - Bibi  (also: Bibi)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Dylan
  - Feyruz Rassool  (also: Feyruz)
  - INDEX 166  (also: INDEX 166)
  - Joseph
  - Leonardo
  - Lewis
  - M. Wilson  (also: Wilson)
  - Rassool
  - Reza Rassool  (also: Reza)
  - Samuel
  - Y.S.
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 219 / 286  (id=4201484364537076764)

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
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 220 / 286  (id=4328984300427233384)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #798

**Entities in chunk:**
  - Alie Fataar
  - Appellate Division  (also: Appellate Division)
  - Balthazzar John Vorster
  - C.Khatieb  (also: Khatieb)
  - Cape Peninsula
  - G.Abed
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Captain
  - Herrenvolk
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Nurjahan
  - Rasheda Rassool  (also: Rasheda)
  - S.Abed  (also: Abed)
  - Salie
  - Shawquet
  - Sohnge Training School  (also: Sohnge Training School)
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Trafalgar High School  (also: Trafalgar High School)
  - Western Province  (also: Eastern Province)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** sister, brother

**CC pass (raw):**
```json
{"quote": "my sisters Zain and Rasheda"}
```

**CC quote:** `my sisters Zain and Rasheda`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Zain Rassool"},{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Zain Rassool`
- `Yousuf Rassool` **sibling_of** `Rasheda Rassool`

---

## Chunk 221 / 286  (id=4713749386019632490)

**Section:** Chapter Seven    War Rages  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #485

**Entities in chunk:**
  - Abba
  - Caledon Streets  (also: Caledon Street)
  - Cassiem  (also: Cassiem)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Darling Street
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - High School  (also: High School)
  - INDEX 166  (also: INDEX 166)
  - Julius Caesar  (also: Julius Caesar)
  - Marcina Kagan
  - Marks's furniture shop  (also: Marks)
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

## Chunk 222 / 286  (id=4769841949596225622)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #142

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Cleansing Department
  - Coloured People's Vigilance Association
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Gool  (also: Gool)
  - Grey Street  (also: Bree Street)
  - Jalbhoy
  - Natal
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Parsee Rustomjee
  - Sorabjee
  - St Paul's Hall
  - Vernon Terrace
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** daughter, father

**CC pass (raw):**
```json
{"quote": "one of my grandfather’s daughters, Gadija"}
```

**CC quote:** `one of my grandfather’s daughters, Gadija`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 223 / 286  (id=4812989679158280412)

**Section:** 59  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #310

**Entities in chunk:**
  - Banqueting Hall
  - Clifton Street  (also: Clifton)
  - Dale Evans
  - District Six  (also: District 6, the District, District Six)
  - Dr Kolia
  - England  (also: England)
  - Hanover
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Pearce  (also: Mr Pearce)
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

## Chunk 224 / 286  (id=4861950306266849132)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #226

**Entities in chunk:**
  - Canadian High Commissioner  (also: Canadian)
  - Chapel Street
  - INDEX 166  (also: INDEX 166)
  - Indian Congress
  - Khadija Ebrahim  (also: Khadija)
  - Mrs Ahmed  (also: Ahmed)
  - Nationalists Party  (also: Nationalist Party)
  - Persotem Patel  (also: Persotem)
  - Stephen Spender
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt"}
```

**CC quote:** `my aunt`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 225 / 286  (id=4888467865479780033)

**Section:** Chapter Four  White Flight  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #393

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ben Kies  (also: Ben Kies M.A)
  - Bunny Kriekler
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - District Six  (also: District 6, the District, District Six)
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
  - Sydney Brenner
  - Walmer Estate
  - World Conference Of Democratic Youth
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 226 / 286  (id=5207007205339208384)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #957

**Entities in chunk:**
  - Anti-CAD Movement
  - Arabic Surah
  - Broach
  - Cape Town-Woodstock P.T.A.  (also: Cape Town-Woodstock P.T.A)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Kelvinator
  - Kies  (also: Mr Kies)
  - League of Nations  (also: League)
  - Parent-Teacher Associations  (also: Parent Teachers Association)
  - Rander district
  - Rasheda Rassool  (also: Rasheda)
  - Surat
  - Unity Movement  (also: Unity)
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

## Chunk 227 / 286  (id=5228882496868360423)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1060

**Entities in chunk:**
  - David Bloomberg
  - Department of Education  (also: Department)
  - Government
  - INDEX 166  (also: INDEX 166)
  - Mister Mogamat Regal
  - Moravian church  (also: Moravian)
  - New Era Fellowship  (also: NEF)
  - Rev. Wessels  (also: Wessels)
  - Reza Rassool  (also: Reza)
  - Roland Colman
  - Sam Wo's Laundry
  - Victor Wessels

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 228 / 286  (id=5499390051200912173)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #104

**Entities in chunk:**
  - Andaman Islands
  - Bibi  (also: Bibi)
  - Burma
  - Caledon Streets  (also: Caledon Street)
  - Clifton Street  (also: Clifton)
  - Corporation Street
  - District Six  (also: District 6, the District, District Six)
  - Emperor Hirohito
  - Far East
  - Imperial Japanese army
  - India Ghulzar Khan  (also: India)
  - Java
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Phillipines
  - Prince of Wales
  - Rander
  - Sayed Fethi
  - Sayed Hussain Ally
  - Singapore
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, married, father, mother, sister, niece, nephew

**CC pass (raw):**
```json
{"quote": "my grandfather departed from Rander leaving behind his mother, sisters and wife all weeping their hearts out."}
```

**CC quote:** `my grandfather departed from Rander leaving behind his mother, sisters and wife all weeping their hearts out.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}

The quote does not mention any family relationships between individuals with known names. The narrator mentions their grandfather's mother, sisters, and wife, but these individuals are not identified by name in the provided list of known persons.
```

**Extracted relations:** none

---

## Chunk 229 / 286  (id=5528078340237292279)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #245

**Entities in chunk:**
  - Caledon
  - Europe
  - Health Department
  - Hitler  (also: Hitler)
  - INDEX 166  (also: INDEX 166)
  - Morris Alexander  (also: Morris)
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

## Chunk 230 / 286  (id=5541857800846579304)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1049

**Entities in chunk:**
  - Alie Fataar
  - Appellate Division  (also: Appellate Division)
  - Balthazzar John Vorster
  - C.Khatieb  (also: Khatieb)
  - G.Abed
  - Geoff Abrahams  (also: Geoff)
  - Gool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Captain
  - Herrenvolk
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX 166)
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - S.Abed  (also: Abed)
  - Salie
  - Sayed Kader Sayed Noor
  - Sohnge Training School  (also: Sohnge Training School)
  - South America  (also: South Africa)
  - Suppression of Communism Act
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Trafalgar High School  (also: Trafalgar High School)
  - Western Province  (also: Eastern Province)
  - Zain Rassool  (also: Zain)

**Triggers found:** son ,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 231 / 286  (id=5664244000205016249)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #613

**Entities in chunk:**
  - Bagus Allie  (also: Bagus-Allie)
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - Durban's Curry's Fountain  (also: Durban)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Khadija Ebrahim  (also: Khadija)
  - Mr. Davids  (also: Davids)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Persotem Patel  (also: Persotem)
  - Rashid
  - Shawquet
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** sister,  cousin

**CC pass (raw):**
```json
{"quote": "my cousins ... Zarina"}
```

**CC quote:** `my cousins ... Zarina`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 232 / 286  (id=5781509709031527554)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #866

**Entities in chunk:**
  - Arthur Tracey
  - Cape
  - Deanna Durbin
  - George Manuel
  - Hutch
  - INDEX 166  (also: INDEX 166)
  - Kies  (also: Mr Kies)
  - Messaris  (also: Mr. Messaris)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nehru
  - South America  (also: South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "My uncle"}
```

**CC quote:** `My uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 233 / 286  (id=5896192977959377292)

**Section:** 61  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #330

**Entities in chunk:**
  - INDEX 166  (also: INDEX 166)
  - Ivan Denisovich
  - Kies  (also: Mr Kies)
  - Mecca
  - National Anti-CAD
  - New Era Fellowship  (also: NEF)
  - Sunday  (also: Sunday)
  - Synagogue  (also: Synagogue)
  - Tabata
  - Vandeleur Street

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 234 / 286  (id=5967827831367585362)

**Section:** 133  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #843

**Entities in chunk:**
  - Green Point Commons  (also: Green Point Common)
  - Habibia  (also: Habibia)
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hewat Amateur Theatricals
  - Isaac Pfaff
  - Langehoven  (also: Langehoven)
  - Mylie Begg  (also: Mylie)
  - Paul Roubaix
  - Smuts's United Party  (also: Smuts)
  - Trafalgar  (also: Trafalgar)
  - Trafalgar Players
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , brother, in-law

**CC pass (raw):**
```json
{"quote": "He was Mylie Begg’s brother in-law, Hamid Khan."}
```

**CC quote:** `He was Mylie Begg’s brother in-law, Hamid Khan.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 235 / 286  (id=6014117459201014898)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #514

**Entities in chunk:**
  - Boycott
  - Damoo Bansda
  - Farouk Du Preez
  - George Munsook
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hamid Khan
  - Hoosain Parker
  - Jackie Gihwala
  - Junior Certificate  (also: Senior Certificate)
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
  - Western Province  (also: Eastern Province)

**Triggers found:** wife, daughter, sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 236 / 286  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - Cape Town
  - D. Neethling  (also: Neethling)
  - Durban's Curry's Fountain  (also: Durban)
  - Fataar
  - Gool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Jaffe  (also: Mr. Jaffe)
  - Kies  (also: Mr Kies)
  - Meissenheimer  (also: Mrs Meissenheimer)
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

## Chunk 237 / 286  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Bibi  (also: Bibi)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Harold Kruger
  - Hewat  (also: Hewat)
  - INDEX 166  (also: INDEX 166)
  - James Felix
  - John Smith
  - Joseph Bredenkamp
  - Ronald Heinrichsen

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "The bond between Dr. Abdurahman and my grandfather"}
```

**CC quote:** `The bond between Dr. Abdurahman and my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 238 / 286  (id=6105164271277742615)

**Section:** 57  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #292

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cecil John Rhodes
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Christian National Movement
  - Differential Calculus
  - Economic History
  - Habibia Institute  (also: Habibia Institute)
  - Hanover Street
  - Hassen Abrahams
  - Integral Calculus
  - Liquid Geometry
  - Mathematics
  - Nationalist government
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Trigonometry
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 239 / 286  (id=6118680768839939168)

**Section:** Yousuf (Joe) Rassool   24  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #144

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - British Indian League
  - Coloured People's Vigilance Committee  (also: Coloured People's Vigilant Committee)
  - Gool  (also: Gool)
  - M. Wilson  (also: Wilson)
  - Miss Sacks
  - Nasim Rassool  (also: Nasim)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Solly Edross
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 240 / 286  (id=6263823088584352511)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1019

**Entities in chunk:**
  - Gaunt
  - George
  - INDEX 166  (also: INDEX 166)
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Natal
  - National Union of South African Students  (also: NUSAS)
  - Pietermaritzburg
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Windermere

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 241 / 286  (id=6278718417584748989)

**Section:** 117  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #725

**Entities in chunk:**
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Achee
  - Isaac Millar
  - Isak Esmail
  - Isak Issy
  - Miss Brown
  - Mrs Domingo  (also: Domingo)
  - Swellendam
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Worcester

**Triggers found:** husband

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 242 / 286  (id=6295959623867363577)

**Section:** Chapter Seventeen  New Era Fellowship  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #933

**Entities in chunk:**
  - B.Sc
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Congress
  - Eiselen Commission
  - Geoff Abrahams  (also: Geoff)
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
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 243 / 286  (id=6312798123425973911)

**Section:** Chapter Eighteen    Chapel Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #968

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Ebrahim Ali
  - Eustace Humphrey Moses
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX 166)
  - Mauritius
  - Michael Johns
  - Mr. Meltzer  (also: Meltzer)
  - Trafalgar  (also: Trafalgar)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  uncle 

**CC pass (raw):**
```json
{"quote": "Hassan’s family was distantly connected to ours through my late Uncle Mamoo who, like Hassan’s grandmother, came from Mauritius."}
```

**CC quote:** `Hassan’s family was distantly connected to ours through my late Uncle Mamoo who, like Hassan’s grandmother, came from Mauritius.`  ✅ anchored → EC

**EC pass (raw):**
```json
{
  "relations": [
    {
      "from": "Yousuf Rassool",
      "relation": "child_of",
      "to": "Mamoo"
    },
    {
      "from": "Hassan Bavasah",
      "relation": "child_of",
      "to": "grandmother's sibling"
    }
  ]
}
```

**Extracted relations:** none

---

## Chunk 244 / 286  (id=6317719585632729785)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #398

**Entities in chunk:**
  - Bagus Allie  (also: Bagus-Allie)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Durban's Curry's Fountain  (also: Durban)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
  - Joan of Arc  (also: Joan)
  - Khadija Ebrahim  (also: Khadija)
  - Malaai  (also: Malaai)
  - Mr. Davids  (also: Davids)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Persotem Patel  (also: Persotem)
  - Rashid
  - Sarlegh Doalie
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 245 / 286  (id=6339413612312511153)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #655

**Entities in chunk:**
  - Britain's Empire  (also: Britain)
  - Darling St  (also: Darling St.)
  - Durban's Curry's Fountain  (also: Durban)
  - France  (also: France)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Henry Newbolt Though
  - Israel  (also: Israel)
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Narutham Godse
  - Palestinian
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

## Chunk 246 / 286  (id=6367374472954772384)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #102

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi  (also: Bibi)
  - Cape Town Docks
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Colosseum Theatre  (also: Colosseum)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - King-Emperor VII
  - M. Wilson  (also: Wilson)
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
  - Port Elizabeth Indians
  - Ruth Alexander
  - Tatarata
  - Wright brothers  (also: Wright)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother, brother, in-law

**CC pass (raw):**
```json
{"quote": "her mother-in-law"}
```

**CC quote:** `her mother-in-law`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 247 / 286  (id=6468864461399055326)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #625

**Entities in chunk:**
  - Burger
  - Chemistry Physics  (also: Chemistry)
  - De Waals Drive  (also: De Waal Drive)
  - E. Albertus  (also: Albertus)
  - G.Hoosain  (also: Hoosain)
  - Government Avenue
  - Grand Parade
  - Greeff  (also: Mr. Greeff)
  - Green Point Commons  (also: Green Point Common)
  - Gregoire Boonzaaier
  - Gregori Malenkov
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Head  (also: Head Mr)
  - INDEX 166  (also: INDEX 166)
  - Jane
  - M. Allie  (also: Allie)
  - Main Road
  - Mariam Gool  (also: Margie, Mariam (Margie))
  - Parker  (also: Mr. Parker)
  - Rev Gordon  (also: Gordon)
  - Reverend Gow  (also: Gow)
  - Torch Commando  (also: Torch)
  - Varsity  (also: Varsity)

**Triggers found:** wed , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 248 / 286  (id=6488849859123662151)

**Section:** Yousuf (Joe) Rassool   24  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #146

**Entities in chunk:**
  - District Six  (also: District 6, the District, District Six)
  - Fazil Rassool  (also: Fazil)
  - General Smuts
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - I.Kamaldien  (also: Kamaldien)
  - Jomo Kenyatta
  - Jon Dos Passos
  - Joseph Bredenkamp
  - Joseph Chamberlain
  - Joyce Dixon
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Kader Essack  (also: Kader)
  - Kalk Bay
  - Keith Miller
  - Ken Viljoen
  - Louis Botha
  - Queens Road
  - Victor Wessels

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the name of the narrator is not given, we can infer that the quote refers to a family relationship with an unnamed person. However, since there are no canonical names mentioned in the quote, we cannot establish a valid relation.

According to the rules, if both endpoints are not identifiable as canonical names, return empty:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 249 / 286  (id=6546657237567362266)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #374

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baku
  - Batum
  - Cape to Cairo
  - Caucasus
  - Cecil John Rhodes
  - Crimean
  - CtoC  (also: CtoC)
  - District Six  (also: District 6, the District, District Six)
  - Eastern Front  (also: Western Front)
  - Europe
  - Far-East fronts
  - Fazil Rassool  (also: Fazil)
  - Moscow
  - North African theatres of war
  - Sebastopol
  - Stalingrad
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, parent,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 250 / 286  (id=6557428703332889295)

**Section:** Chapter Eight    Anti-CAD  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #520

**Entities in chunk:**
  - Amazon Kindle Direct Publishing
  - Britain's Empire  (also: Britain)
  - British Newspaper Library
  - Cape Town
  - Ciraj Rassool
  - District Six  (also: District 6, the District, District Six)
  - District Six Museum
  - E. Reddy  (also: Reddy)
  - INDEX 166  (also: INDEX 166)
  - Kwaai Oak
  - M. Allie  (also: Allie)
  - Marcina Kagan
  - Nasim Moosa
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Naz Gool-Ebrahim
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Rebecca Rassool
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

## Chunk 251 / 286  (id=6610559471948150059)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #950

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Chops
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

## Chunk 252 / 286  (id=6614900686580700622)

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
  - George Veldsman
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hewat Amateur Theatricals
  - INDEX 166  (also: INDEX 166)
  - Indian Congress Eventually Fatima  (also: Indian Congress Eventually Fatima)
  - Johaar Mosaval  (also: Johaar Mosavel)
  - McDonald Dixon
  - Paul Roubaix
  - Pfaff  (also: Pfaff)
  - President Harry Truman
  - Sadlers Wells Ballet
  - Smuts's United Party  (also: Smuts)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 253 / 286  (id=6617129456286239306)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #479

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Belgium  (also: Belgium)
  - Britain's Empire  (also: Britain)
  - Cecil Wightman
  - Denmark  (also: Denmark)
  - France  (also: France)
  - Hitler  (also: Hitler)
  - Holland  (also: Holland)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mussolini
  - Natal
  - Norway  (also: Norway)
  - Royal  (also: Royal)
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
Since the narrator is "Yousuf Rassool" and the quote mentions "Uncle Mamoo", we can infer that "Mamoo" is an uncle of Yousuf. However, since "uncle" is not a valid relation type in the schema, according to the rules, we should return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 254 / 286  (id=6619818761336328416)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #384

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Cape Town
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - England  (also: England)
  - INDEX 166  (also: INDEX 166)
  - Irwin Combrinck  (also: Irwin Combrick)
  - Laurence Olivier's Hamlet
  - Los Angeles
  - Nazli Behardien
  - Reading, England  (also: Reading)
  - South African Communist Party
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** niece

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 255 / 286  (id=6622459745407552529)

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

## Chunk 256 / 286  (id=6710772805342046917)

**Section:** 65  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #359

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Cape
  - Cape Times Law Reports
  - Chapel Street School  (also: Chapel Street School)
  - Compton  (also: Compton)
  - Doug Wright
  - E.A. Parker
  - Edrich
  - Faried Rossier
  - Gladwin  (also: Gladwin)
  - Habibia Institute  (also: Habibia Institute)
  - Hussein Parker
  - Hutton  (also: Hutton)
  - Indian Opinion
  - Jannat
  - John Arlott
  - M. Wilson  (also: Wilson)
  - Nasim Rassool  (also: Nasim)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Robert R. Edgar
  - Supreme Court Proceedings
  - Trinidad
  - Washbrook

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "the birth of my baby brother, Nasim"}
```

**CC quote:** `the birth of my baby brother, Nasim`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Nasim Rassool","relation":"child_of","to":""}]}
```

**Extracted relations:** none

---

## Chunk 257 / 286  (id=6941537010882242866)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #80

**Entities in chunk:**
  - Eddieas
  - Field Marshal Tito
  - Hamid
  - INDEX 166  (also: INDEX 166)
  - Kader Essack  (also: Kader)
  - M. Wilson  (also: Wilson)
  - Muir Street
  - Nasima M.S  (also: Nasima)
  - New Era Fellowship  (also: NEF)
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
{"quote": "my cousin Nasima"}
```

**CC quote:** `my cousin Nasima`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 258 / 286  (id=6943291249971136246)

**Section:** Yousuf (Joe) Rassool   2  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #39

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Bergies
  - Fazil Rassool  (also: Fazil)
  - Gool  (also: Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Kloof Nek  (also: Kloof Nek)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Nationalists Party  (also: Nationalist Party)
  - Platteklip Gorge
  - Ronnie Jooste  (also: Ronnie)
  - Slangolie
  - Southern Africa
  - Table Mountain
  - Woody Woodman  (also: Woody)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 259 / 286  (id=6966615310251134524)

**Section:** Yousuf (Joe) Rassool   28  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #162

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Barnato Board
  - Fazil Rassool  (also: Fazil)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - London
  - Lord Elgin
  - Meneer
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mowbray Cemetry  (also: Mowbray)
  - Salie Van Haacht
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle"}
```

**CC quote:** `my uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 260 / 286  (id=7168244047913479805)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1141

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Dr AH Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Michael Johns
  - Ralph J Bunche  (also: Ralph Bunche)
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

## Chunk 261 / 286  (id=7243012516835118461)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #407

**Entities in chunk:**
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Bagus Allie  (also: Bagus-Allie)
  - Caledon Streets  (also: Caledon Street)
  - Chapel Street School  (also: Chapel Street School)
  - Durban's Curry's Fountain  (also: Durban)
  - Group Areas Boards  (also: Group Areas Board)
  - Heerie  (also: Heerie)
  - Jannat
  - Kanamia Muslims
  - Moosa Cape Town
  - Moosa Driver
  - Muir Street
  - Senator Edgar Brookes
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zinatul Islam Mosque

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "his adorable cousin Heerie"}
```

**CC quote:** `his adorable cousin Heerie`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 262 / 286  (id=7262414809749442984)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #699

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX 166)
  - Indian Congress
  - Mohamed Ali Jinnah
  - Muir Street
  - Muslim League
  - Searle Street
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

## Chunk 263 / 286  (id=7266467364867027437)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #107

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Cape Town
  - District Six  (also: District 6, the District, District Six)
  - E. Reddy  (also: Reddy)
  - John Phillips  (also: Rev. John Phillips)
  - Mauritius
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Nasima M.S  (also: Nasima)
  - Raza
  - Reagon
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

The quote states that Jane Gool-Tabata (Wahida’s daughter) is Yousuf's aunt. However, since 'aunt' is not in the schema, according to the rules, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 264 / 286  (id=7360610042715665275)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #158

**Entities in chunk:**
  - All African Convention  (also: AAC, All-African Convention)
  - Ally
  - Bibi  (also: Bibi)
  - Bloemfontein
  - British Government
  - Cape Town
  - INDEX 166  (also: INDEX 166)
  - Johannesburg
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal
  - New Era Fellowship  (also: NEF)
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

## Chunk 265 / 286  (id=7371595065091017673)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #903

**Entities in chunk:**
  - Cape Colony
  - Gool  (also: Gool)
  - Hamid
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Ismail Hayat
  - Jameel
  - M. Wilson  (also: Wilson)
  - Metropolitan Handicap
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mr Gool
  - Transvaalers  (also: Transvaalers)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 266 / 286  (id=7448875471219160758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #480

**Entities in chunk:**
  - Cecil Wightman
  - Chaplin
  - E. Albertus  (also: Albertus)
  - Habibia  (also: Habibia)
  - Hitler  (also: Hitler)
  - INDEX 166  (also: INDEX 166)
  - M. Allie  (also: Allie)
  - Mussolini
  - Nasim Rassool  (also: Nasim)
  - Torch Commando  (also: Torch)
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

## Chunk 267 / 286  (id=7450355554419660594)

**Section:** 91  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #545

**Entities in chunk:**
  - Bartnicke  (also: Bartnicke)
  - Burg Street
  - Cape Argus
  - City
  - Eddieas
  - Geoff Abrahams  (also: Geoff)
  - INDEX 166  (also: INDEX 166)
  - Kader Essack  (also: Kader)
  - M. Wilson  (also: Wilson)
  - Muir Street
  - Nasima M.S  (also: Nasima)
  - Poppie
  - Ronnie Jooste  (also: Ronnie)
  - Shariefa Khan
  - Slingers  (also: Mr. Slingers)
  - Springveld
  - Standard Seven  (also: Standard Seven)
  - Standard Six  (also: Standard Six)
  - Strand
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  cousin

**CC pass (raw):**
```json
{"quote": "his elder brother’s"}
```

**CC quote:** `his elder brother’s`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his elder brother's", we can infer that Yousuf Rassool has an elder brother. However, the name of the elder brother is not mentioned in the quote.

But since the narrator is 'my' (I) and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. 

So, the JSON output would be:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":null}]}

However, since the schema requires both endpoints to be identifiable as canonical names and the elder brother's name is not mentioned, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 268 / 286  (id=7520648796358329052)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #50

**Entities in chunk:**
  - Al Azhar College
  - Bernard Herzberg  (also: Bernard)
  - Chops
  - Egypt
  - Glen
  - Gray's Elegy
  - Guys  (also: Guys)
  - Heneke  (also: Mr Heneke)
  - Kloof Nek  (also: Kloof Nek)
  - Kloof Street  (also: Loop Street)
  - Lily Herzberg
  - London
  - Milton's Lycidas
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , son ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 269 / 286  (id=7561700546005880907)

**Section:** 47  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #234

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Town
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX 166)
  - Russian Whaler Ob  (also: Russian Whaler Ob)
  - Tyger Valley
  - Tyne and Roger Streets  (also: Tyne)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 270 / 286  (id=7562115678963637038)

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
  - District Six  (also: District 6, the District, District Six)
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

## Chunk 271 / 286  (id=7574535754100824916)

**Section:** Yousuf (Joe) Rassool   132  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #838

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi  (also: Bibi)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Fabians Kies  (also: Fabians)
  - Howard Fast
  - Hussein Parker
  - Hyde Park
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Hymie Beimel  (also: Hymie)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Ignazio Silone
  - Immigration Act
  - India Ghulzar Khan  (also: India)
  - Indian Congress
  - Indian Cricket Union
  - Indian Opinion
  - Inez Vera Du Preez
  - Institute of Race Relations
  - Jeddah
  - Kies  (also: Mr Kies)
  - Qudrat of Allah  (also: Qudrat)
  - Tabata
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 272 / 286  (id=7584768665477924623)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #81

**Entities in chunk:**
  - Alie Fataar
  - Bibi  (also: Bibi)
  - Cape Town
  - Darling St  (also: Darling St.)
  - Edna
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - George Meissenheimer  (also: George Meisenheimer)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Gwen Wilcox
  - Halima Dudley
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - India Ghulzar Khan  (also: India)
  - Joyce Dixon
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Kenny Jordaan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Veronica Pienaar
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "his aged mother in India"}
```

**CC quote:** `his aged mother in India`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"India Ghulzar Khan"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `India Ghulzar Khan`

---

## Chunk 273 / 286  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi  (also: Bibi)
  - Cape
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Grand Parade
  - Helen Abrahams
  - Mauritius
  - Nasima M.S  (also: Nasima)
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

## Chunk 274 / 286  (id=7727744974142389137)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #884

**Entities in chunk:**
  - Abed family  (also: Abed family)
  - Burger
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town
  - Durban's Curry's Fountain  (also: Durban)
  - E. Albertus  (also: Albertus)
  - England  (also: England)
  - Head  (also: Head Mr)
  - INDEX 166  (also: INDEX 166)
  - Khadija Ebrahim  (also: Khadija)
  - Kloof Nek  (also: Kloof Nek)
  - M. Allie  (also: Allie)
  - Parker  (also: Mr. Parker)
  - Persotem Patel  (also: Persotem)
  - Tiny  (also: Tiny)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 275 / 286  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Bernadette
  - Chemistry Physics  (also: Chemistry)
  - Cyril
  - Dawood Seedat  (also: Dawood)
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Seedat
  - INDEX 166  (also: INDEX 166)
  - Professor James  (also: Prof. James)
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

## Chunk 276 / 286  (id=7794589649810439109)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #116

**Entities in chunk:**
  - Afrikaans Language Movement  (also: Afrikaans)
  - Bibi  (also: Bibi)
  - Buitencingle  (also: Buitencingle)
  - Cape Town Fire Brigade
  - Castle Street
  - Grand Parade
  - INDEX 166  (also: INDEX 166)
  - M. Allie  (also: Allie)
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

## Chunk 277 / 286  (id=7818984028335812846)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #958

**Entities in chunk:**
  - Ben Cloete
  - Cape Town
  - Cape Town-Woodstock P.T.A.  (also: Cape Town-Woodstock P.T.A)
  - Congress Movement
  - Head of Trafalgar
  - Heneke  (also: Mr Heneke)
  - Hewat Training College  (also: Hewat Training College)
  - Liberation League
  - Manuel
  - Scotches Kloof  (also: Scotches-Kloof)
  - Trafalgar  (also: Trafalgar)
  - Woodstock
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 278 / 286  (id=7992762871094121264)

**Section:** Chapter Twelve    Cricket  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #661

**Entities in chunk:**
  - Axis Powers
  - Beveridge Plan
  - Britain's Empire  (also: Britain)
  - Churchill
  - Democracy
  - District Six  (also: District 6, the District, District Six)
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

## Chunk 279 / 286  (id=8003152637512369339)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #570

**Entities in chunk:**
  - Bonny-Bridge Dover Stove
  - Buitencingle  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek  (also: Kloof Nek)
  - Mrs Domingo  (also: Domingo)
  - South African Public Library
  - South America  (also: South Africa)
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

## Chunk 280 / 286  (id=8317664100809929487)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #911

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cape Town
  - Darling St  (also: Darling St.)
  - District Six  (also: District 6, the District, District Six)
  - Dr. DuPlessis  (also: DuPlessis)
  - G.Hoosain  (also: Hoosain)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Indian Opinion Dec 29, 1906 p986
  - National Gandhi Museum District Six
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
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"G.Hoosain"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `G.Hoosain`

---

## Chunk 281 / 286  (id=8501744309371801229)

**Section:** 79  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #455

**Entities in chunk:**
  - Even
  - Heyl  (also: Mr. Heyl)
  - INDEX 166  (also: INDEX 166)
  - Islamic Synagogue  (also: Islamic)
  - Mecca
  - Natal Whites  (also: Natal Whites)
  - South African Indians  (also: South African Indians)
  - Sunday  (also: Sunday)
  - Synagogue  (also: Synagogue)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Vandeleur Street
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 282 / 286  (id=8719056895392496748)

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
  - Hewat  (also: Hewat)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ken Viljoen
  - Melville Bruce Mitchell Athol Rowan Ken Viljoen  (also: Melville)
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Paul Winslow
  - South America  (also: South Africa)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
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

## Chunk 283 / 286  (id=8730565953127446314)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #997

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Britain's Empire  (also: Britain)
  - England  (also: England)
  - Germany  (also: Germany)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 284 / 286  (id=8778866406539819344)

**Section:** Yousuf (Joe) Rassool   104  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #636

**Entities in chunk:**
  - Bibi  (also: Bibi)
  - England  (also: England)
  - Holy Prophet
  - Jamieson Hall  (also: Jamieson Hall)
  - Khadija Ebrahim  (also: Khadija)
  - Nationalist Senators
  - Parliament  (also: Parliament)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
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

## Chunk 285 / 286  (id=9062310575080274351)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #225

**Entities in chunk:**
  - Agent General
  - Athlone Branch of the TLSA
  - Buitencingle  (also: Buitencingle)
  - Durban's Curry's Fountain  (also: Durban)
  - Government of India Representatives
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
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
{"quote": "My aunt"}
```

**CC quote:** `My aunt`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 286 / 286  (id=9086159078617739270)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #959

**Entities in chunk:**
  - Cannon Street
  - Congress Movement
  - Department of Education  (also: Department)
  - Health Department
  - Isaac Millar
  - League of Nations  (also: League)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Trade Unionists
  - W.P.  (also: W.P)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 286 |
| Relations extracted | 44 |
| Relations written to graph | 44 |
