# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 209  
**Sampled:** 209  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 209  (id=-9123613270288747913)

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

## Chunk 2 / 209  (id=-9102931590403333452)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #42

**Entities in chunk:**
  - German Lutheran Church  (also: German Lutheran Church)
  - INDEX 166  (also: INDEX 166)
  - Islamic
  - Nasima
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

## Chunk 3 / 209  (id=-9026931979920257841)

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
  - Indian Congress
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
Since the narrator is 'my' and refers to their dad and aunt, we can infer that the narrator's name is not explicitly mentioned. However, based on the quote, we can extract a family relationship.

The quote states: "my dad and aunt were brother and sister"

This implies that the narrator's dad is the sibling of the narrator's aunt. Since the narrator's name is not given, we cannot use it as 'from'. Instead, we will use the canonical names mentioned in the quote.

However, since the quote uses 'aunt', which is not a valid relation type according to the critical rules, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 4 / 209  (id=-8601693670707718054)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #201

**Entities in chunk:**
  - Bibi's children
  - Buitencingle Street  (also: Buitencingle)
  - Calcutta
  - District Six  (also: District Six)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Harold Cressy
  - Hewat College Motto
  - Hewat Training College  (also: Hewat Training College)
  - Mohammed Saaid  (also: Mohamed Saaid)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Primary School  (also: Primary)
  - Roeland Street Gaol
  - St Martini German Lutheran Church
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 5 / 209  (id=-8565879665326676003)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #890

**Entities in chunk:**
  - Achee
  - Achmed
  - Avondale Road
  - Coon Club
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head  (also: Head)
  - Indian Congress Eventually Fatima
  - Meneer
  - Mr. Pearce  (also: Mr Pearce)
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

## Chunk 6 / 209  (id=-8505583857113009851)

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

## Chunk 7 / 209  (id=-8488686338445114035)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #333

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Cape Times
  - Helen Abrahams
  - Israel
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

## Chunk 8 / 209  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - Drill Hall
  - E.L.Maurice
  - F.Landman
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - GL Abrahams
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Guy's Hospital  (also: Guys Hospital)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs
  - INDEX 166  (also: INDEX 166)
  - Joosub Gool  (also: Joosub)
  - London
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Messrs R.O.Dudley
  - Movement
  - PTAs
  - Riotous Assemblies Act
  - Sayed Kader Sayed Noor
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
{"quote": "Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines."}
```

**CC quote:** `Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Fatima Gool","relation":"sibling_of","to":"Haji Joosub Maulvi Hamid Gool"}]}
```

**Extracted relations:**
- `Fatima Gool` **sibling_of** `Haji Joosub Maulvi Hamid Gool`

---

## Chunk 9 / 209  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Cape Town
  - Ebrahim  (also: Ebrahim)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Natasha
  - National Union of South African Students  (also: NUSAS)
  - Nikita Khruschev
  - Non-European Unity Movement  (also: Non European Unity Movement)
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

## Chunk 10 / 209  (id=-8225958241890195217)

**Section:** Yousuf (Joe) Rassool   30  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #166

**Entities in chunk:**
  - Buitencingle Street  (also: Buitencingle)
  - Cape Peninisula Students Union
  - District Six  (also: District Six)
  - Dr. Gool
  - Heneke  (also: Mr Heneke)
  - Hewat Training College  (also: Hewat Training College)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Neville Alexander
  - New Era Fellowship
  - Pat Naidoo
  - Tabata
  - Trafalgar
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

## Chunk 11 / 209  (id=-8186139953935417234)

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

## Chunk 12 / 209  (id=-8157488290903077572)

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
  - Meneer
  - Movement
  - Mowbray
  - Professor James  (also: Prof James)
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

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"[unnamed sister(s)]"}]}

However, since the sister's name is not provided, we cannot specify the exact "to" endpoint. According to the rules, if both endpoints are not identifiable as canonical names, we should return an empty list.

So, the corrected output is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 13 / 209  (id=-8077784496787990642)

**Section:** 51  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #265

**Entities in chunk:**
  - Allied
  - Allies  (also: Allies)
  - Ardenne
  - Argus Annual  (also: Argus)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Generals Montgomery  (also: Generals Montgomery)
  - George Golding  (also: Mr. George Golding)
  - Kesselring
  - Montgomery
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

## Chunk 14 / 209  (id=-8041283814668973231)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #109

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Communist Party
  - District Six  (also: District Six)
  - Franchise Action Council
  - Grand Parade
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassan Bavasah  (also: Hassan)
  - India Ghulzar Khan  (also: India)
  - NEUM
  - USSR  (also: USSR)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, parent

**CC pass (raw):**
```json
{"quote": "her husband"}
```

**CC quote:** `her husband`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 15 / 209  (id=-7980378207581507601)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #207

**Entities in chunk:**
  - Advocate Christopher
  - Aunty Annie
  - Ayesha Lallie
  - British Government
  - Buitencingle Street  (also: Buitencingle)
  - Castle Street
  - Fazil Rassool  (also: Fazil)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Kelvinator
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Natal
  - Peerbhai
  - Peter Alexander Rassool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** married, father, mother,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 16 / 209  (id=-7752783211163375761)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1062

**Entities in chunk:**
  - Argus Annual  (also: Argus)
  - Avenue
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Castle Bridge
  - Darling Street
  - Gardens
  - Jan Van Riebeeck
  - Kimberley Hospital  (also: Kimberley)
  - Little Theatre
  - Mount Nelson Hotel  (also: Mount Nelson Hotel)
  - Mount Streets  (also: Mount Street)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Orange Street
  - Parade
  - Parliament  (also: Parliament)
  - Slamat
  - South African Public Library  (also: South African Public Library)
  - St. George's Grammar
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 17 / 209  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Churchill
  - France
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

## Chunk 18 / 209  (id=-7549255937705633040)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #947

**Entities in chunk:**
  - Alexander
  - Candy's dog
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

## Chunk 19 / 209  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Boeta
  - Chops
  - Dr. Abdul Hamid Gool  (also: Dr Abdul Hamid Gool)
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

## Chunk 20 / 209  (id=-7082280809534297158)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #345

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Education Department
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
  - Joan
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

## Chunk 21 / 209  (id=-7025477089957931364)

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
  - Messaris  (also: Mr Messaris)
  - National Union of South African Students  (also: NUSAS)
  - New Era Fellowship
  - Pietermaritzburg
  - Richard Parker
  - Slingers  (also: Mr Slingers)
  - Springveld
  - Students Representative Council  (also: SRC)
  - Trafalgar High  (also: Trafalgar High)
  - University of Natal  (also: University of Natal)
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

## Chunk 22 / 209  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - Coloured Advisory Council
  - E. Albertus  (also: Albertus)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - General  (also: General)
  - Habibia  (also: Habibia)
  - Hewat  (also: Hewat)
  - Hymie Beimel  (also: Hymie)
  - INDEX 166  (also: INDEX 166)
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 23 / 209  (id=-6782705425542896048)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #113

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - England
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Jamieson Hall
  - Khadija
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband, wed , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 24 / 209  (id=-6763001656966195273)

**Section:** 165  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1067

**Entities in chunk:**
  - Algerian Liberatory
  - Chapel Street Staff
  - Ciskei  (also: Ciskei)
  - Danny De Beer  (also: Danny)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nelson Mandela
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

## Chunk 25 / 209  (id=-6710587015263367697)

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
Since the narrator is "my" and the NARRATOR name is given as Yousuf Rassool, we can use that name as 'from'. The quote states "my grandfather's daughters", which implies that the daughters are the aunts of the narrator. However, according to the CRITICAL RULES, if the quote uses 'aunt', return {"relations":[]}.

But wait! We can infer the relationship between Yousuf Rassool and his grandfather's daughters' parent (i.e., his grandfather). Since the daughters are referred to as "my grandfather's daughters", it implies that Yousuf Rassool is the child_of the person whose daughters they are. 

However, we cannot identify the canonical name of the grandfather from the given information.

So, according to the CRITICAL RULES, if both endpoints are not identifiable as canonical names, return empty.

Therefore, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 26 / 209  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - E. Albertus  (also: Albertus)
  - Group Areas Board  (also: Group Area Boards)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Labour Party
  - Mr Edross  (also: Edross)
  - Mussolini
  - Parker  (also: Mr. Parker)
  - Parliamentary
  - Shaw
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
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Hoonsain"}]}
```

**Extracted relations:** none

---

## Chunk 27 / 209  (id=-6563863471018850689)

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
  - Ebrahim  (also: Ebrahim)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Isgak Adams
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Little Karoo
  - Nash  (also: Nash)
  - Oudtshoorn
  - Persotem Patel  (also: Persotem)
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

## Chunk 28 / 209  (id=-6390972800691852389)

**Section:** 51  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #264

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - British Army
  - Cape Town National Archive
  - Editor
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Lord Roberts
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Whites-only
  - Yasmin Rassool  (also: Yasmin, Jessie, Jessie Rassool)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , daughter, mother, gave birth

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 29 / 209  (id=-6141146778954841009)

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
  - Ebrahim  (also: Ebrahim)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Louis Messaris
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Thus
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

## Chunk 30 / 209  (id=-6067802038126529915)

**Section:** Yousuf (Joe) Rassool   6  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #64

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Abbas Dinath  (also: Abbas)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Cape of Good Hope
  - City Council
  - Coloured Affairs
  - Gava Kadoes  (also: Gava)
  - Gujarat
  - Hanover Street
  - Hiema
  - Hindu Association
  - Immigration Act
  - India Ghulzar Khan  (also: India)
  - Indian Opinion
  - J. Lucheram  (also: Lucheram)
  - Jabavu  (also: Dr. Jabavu)
  - M. Enus  (also: Enus)
  - Mauritius
  - President of the Hindu Association
  - Rander district  (also: Rander)
  - South America  (also: South Africa)
  - Swat
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "his brother Abbas"}
```

**CC quote:** `his brother Abbas`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Abbas Dinath"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Abbas Dinath`

---

## Chunk 31 / 209  (id=-6012376786615465402)

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
  - France
  - Holland
  - J. Wilson  (also: Wilson)
  - Mr Tregger  (also: Tregger)
  - Mrs Wooding  (also: Wooding)
  - Norway
  - Queens Road
  - Rev Dan Wessels  (also: Rev. Dan Wessels)
  - Unity Movement  (also: Unity)
  - Van der Ross
  - Vassan Hindu C.C  (also: Vassan)
  - Veldsman
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 32 / 209  (id=-5915939077846947542)

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

## Chunk 33 / 209  (id=-5897797065487598428)

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

## Chunk 34 / 209  (id=-5853235968246498787)

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
Since the narrator is "Yousuf Rassool" and the quote mentions "my grandfather", we can infer that Yousuf Rassool's grandfather is Haji Joosub Maulvi Hamid Gool (also known as J.M.H. Gool, etc.).

However, since the schema only allows for parent_of, child_of, sibling_of, half_sibling_of relationships, and the quote directly states "grandfather", which implies a more distant relationship, we cannot extract any valid JSON.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 35 / 209  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Gurukula at Hardwar  (also: Gurukula)
  - Hamid Khan
  - Hardwar
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Marcina Kagan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natalian
  - Ramsey MacDonald
  - Rasheda Rassool  (also: Rasheda)
  - Rev. C. F. Andrews
  - Wagieda
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zain Rassool  (also: Zain)

**Triggers found:** son ,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "From my cousin, Marcina Kagan, I also heard that Gandhi’s son had fallen in love with my Aunt Fatima"}
```

**CC quote:** `From my cousin, Marcina Kagan, I also heard that Gandhi’s son had fallen in love with my Aunt Fatima`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. However, the quote mentions "Aunt Fatima" which is not a canonical name in the list. According to the rules, if the quote uses 'aunt', 'uncle', 'nephew', 'niece', or 'cousin', return {"relations":[]}.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 36 / 209  (id=-5786981904019721036)

**Section:** Yousuf (Joe) Rassool   viii  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #7

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 37 / 209  (id=-5767629269527031038)

**Section:** Chapter One  Caledon Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #274

**Entities in chunk:**
  - Abdulla Mohamed
  - Abdulla Parker
  - Ahmed Salasa
  - Caledon Streets  (also: Caledon Street)
  - Cosmo Pieterse
  - Dickmans
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

## Chunk 38 / 209  (id=-5732511328885138558)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #498

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Abe Desmore
  - Alawie
  - Cape Corps
  - District Six  (also: District Six)
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

## Chunk 39 / 209  (id=-5635069970394508836)

**Section:** 45  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #220

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Glen
  - Gray's Elegy
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - J. Wilson  (also: Wilson)
  - Kloof Nek
  - Kloof Street  (also: Loop Street)
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

## Chunk 40 / 209  (id=-5610384977170982774)

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

## Chunk 41 / 209  (id=-5556520109474695452)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #135

**Entities in chunk:**
  - Australia
  - Australians
  - Barmania MA  (also: Barmania)
  - Board
  - Buitencingle Street  (also: Buitencingle)
  - Canada
  - Cape Town
  - Dr. Jamieson  (also: Dr Jamieson)
  - George Golding  (also: Mr. George Golding)
  - Group Areas  (also: Group Area)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Jabavu  (also: Dr. Jabavu)
  - Mr. B. Allie  (also: B. Allie)
  - New Zealanders
  - Progressive Party
  - Salie Dollie
  - St Paul's Mission School
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
Since the narrator is "Yousuf Rassool" and the quote mentions "My grandfather", we can infer that Yousuf Rassool's grandfather is being referred to. However, since the name of the grandfather is not mentioned, we cannot establish a valid relation.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 42 / 209  (id=-5460737940864037782)

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
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 43 / 209  (id=-5418155653937249668)

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
  - Groote Schuur  (also: Groot Schuur)
  - Group Areas Board  (also: Group Area Boards)
  - Les Jacobs
  - Liberals
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs. Gladstone  (also: Mrs Gladstone)
  - New Era Fellowship
  - Prime Minister Botha
  - Ralph J Bunche  (also: Ralph Bunche)
  - Senator Edgar Brookes
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

This is because the quote mentions a relationship between Gandhi's son and Goulam Gool's sister, but it does not mention a direct family relationship between two individuals that can be identified as canonical names. The quote only mentions a romantic relationship between two unnamed individuals who are related to Gandhi and Goulam Gool, respectively.
```

**Extracted relations:** none

---

## Chunk 44 / 209  (id=-5364503672193663248)

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
  - Eldest
  - Feyruz Rassool  (also: Feyruz)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hussain Ally  (also: Hoosain Ally)
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

This quote identifies a direct family relationship between the narrator and Ayesha Rassool (also known as Lallie, Ayesha Gool). The possessive word "my" is implied in the phrase "Mom", establishing that Ayesha is the mother of the narrator.
```

**CC quote:** `Mom (Ayesha)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 45 / 209  (id=-5340629096267782490)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #75

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
{"quote": "her elder sister"}
```

**CC quote:** `her elder sister`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 46 / 209  (id=-5308475734057125055)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #348

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Athlone Branch  (also: Athlone Branch)
  - Catholic
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

## Chunk 47 / 209  (id=-5184936258167894087)

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
  - Habibia Institute  (also: Habibia Institute)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanover Park
  - Higher Education
  - India Ghulzar Khan  (also: India)
  - J. Wilson  (also: Wilson)
  - Mannenberg
  - Mitchell's Plain
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

## Chunk 48 / 209  (id=-5154709854984411195)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #761

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Chapel Street School  (also: Chapel Street School)
  - Compton  (also: Compton)
  - Doug Wright
  - E.A. Parker  (also: Mr. E.A. Parker)
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

## Chunk 49 / 209  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Cape
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head Student
  - Heathrows C.C
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - Indian Affairs Department
  - Mister Efficiency
  - Muslims C.C  (also: Muslims)
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

## Chunk 50 / 209  (id=-5010050899247603262)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1138

**Entities in chunk:**
  - Abdurahman
  - Alan Gregor Cobley
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town
  - Cathedral Hall
  - Coloured People's Vigilance Committee  (also: Coloured People's Vigilant Committee)
  - Edgar Maurice  (also: Edgar)
  - Even Buitencingle
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Indian Opinion
  - J.M.H. Wilson
  - James M.
  - John Brown  (also: John Brown)
  - Journal of Southern African Studies
  - Kloof Street  (also: Loop Street)
  - Nasima
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

## Chunk 51 / 209  (id=-5009073907762257871)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #100

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bhokkie
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Caledon Streets  (also: Caledon Street)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 52 / 209  (id=-4897146471319258283)

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

Here is the extracted relationship in JSON format:

{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Nurjehan Gool Mohamed"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Nurjehan Gool Mohamed`

---

## Chunk 53 / 209  (id=-4713556574132170500)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #609

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 54 / 209  (id=-4527305166854275038)

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
  - Trafalgar
  - United Party
  - Unity Movement  (also: Unity)
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

## Chunk 55 / 209  (id=-4519318944871171242)

**Section:** Yousuf (Joe) Rassool   54  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #275

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Cleansing Department
  - Coloured People's Vigilance Association
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Indian Opinion
  - Jalbhoy
  - Natal
  - Parsee Rustomjee
  - Salie
  - Sorabjee
  - St Paul's Hall
  - Vernon Terrace
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 56 / 209  (id=-4493652338336870038)

**Section:** Yousuf (Joe) Rassool   134  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #850

**Entities in chunk:**
  - African Methodist Episcopal Church
  - Coloured Advisory Council
  - George Golding  (also: Mr. George Golding)
  - Hamid
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - Nationalists Party  (also: Nationalist Party)
  - Reverend Gow  (also: Rev. Gow)
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

## Chunk 57 / 209  (id=-4440340057225779401)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #314

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Andrew Mackrill
  - Arthur Rank Films
  - Cape Town University
  - Cardiff
  - Chapel
  - Colin Wynne
  - Cynthia Fisher
  - Fruit and Vegetable Market
  - George Veldsman
  - Hewat Amateur Theatrical Society
  - Ivan Agherdien
  - Leonard Dixon
  - Muir Street
  - Norman Florence
  - Rasheda Rassool  (also: Rasheda)
  - Sam Wo's Laundry
  - Selkirk Streets
  - South America  (also: South Africa)
  - Trafalgar Players
  - Zain Rassool  (also: Zain)

**Triggers found:** married

**CC pass (raw):**
```json
{"quote": "Sam Wo was married to a Cape Malay woman"}
```

**CC quote:** `Sam Wo was married to a Cape Malay woman`  ✅ anchored → EC

**EC pass (raw):**
```json
Since Sam Wo is mentioned in the quote and has a corresponding canonical name "Sam Wo's Laundry", we can extract the relationship. However, there is no direct mention of a specific person that Sam Wo was married to, only that it was a Cape Malay woman.

But since "Cape Town University" is not a person, and "a Cape Malay woman" does not directly correspond to any canonical name, we cannot establish a valid relation.

Therefore, the correct answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 58 / 209  (id=-4303196698623211308)

**Section:** The struggle unfolds  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #131

**Entities in chunk:**
  - Cape
  - INDEX 166  (also: INDEX 166)
  - Mike Allie
  - Nationalists Party  (also: Nationalist Party)
  - Sailor Malan
  - South America  (also: South Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
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

## Chunk 59 / 209  (id=-4279669986134027190)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #620

**Entities in chunk:**
  - Aligarh College  (also: Aligarh)
  - Capetown
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Government Avenue
  - India Ghulzar Khan  (also: India)
  - Islamic
  - Ismail  (also: Ismail)
  - Katie
  - Mauritius
  - Mouille Point
  - Nasima
  - Non-European Unity Movement  (also: Non European Unity Movement)
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

## Chunk 60 / 209  (id=-4263780790926522616)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #187

**Entities in chunk:**
  - Advocate Henry Sylvester Williams
  - Dr. A Abdurahman
  - J. M. H. Gool
  - J. M. Wilson
  - J.Boyce
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Daily Witness
  - Natal Indian Congress
  - National Gandhi Museum  (also: National Gandhi Musem)
  - National Road
  - National Theatre  (also: National Theatre)
  - Native Life in South Africa
  - Native Representative Councils
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Nehru
  - Nelson Eddy
  - Nelson Mandela
  - New Era Fellowship
  - Nicky Springveld
  - Nikita Khruschev
  - Non-European Unity Movement  (also: Non European Unity Movement)
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

## Chunk 61 / 209  (id=-4250574601481422414)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #424

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Anti-Zionism
  - Cape Argus
  - Desmond Green
  - Golding Van  (also: Golding)
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - New Era Fellowship
  - Regan
  - Reverend Gow  (also: Rev. Gow)
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

## Chunk 62 / 209  (id=-4183482280000304421)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #31

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Buitencingle Street  (also: Buitencingle)
  - Cape
  - Church Street Capetown S.A
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - J. Wilson  (also: Wilson)
  - London
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Oxford
  - Pan African Conference
  - Rhodes House Library  (also: Rhodes House Library)
  - Ship Chandlers
  - Williams
  - Wooding's Preparatory School  (also: Wooding's Preparatory School)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 63 / 209  (id=-3965621148867394083)

**Section:** The struggle unfolds  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #132

**Entities in chunk:**
  - Achmed
  - Beloved Country
  - Cape Town
  - Cow & Gate  (also: Cow&Gate)
  - Cronin  (also: Cronin)
  - Cultural Society
  - Cynthia Carelse
  - Cynthia Fisher
  - District Six  (also: District Six)
  - English-speaking Europeans
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Head  (also: Head)
  - INDEX 166  (also: INDEX 166)
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

## Chunk 64 / 209  (id=-3900190985977953352)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #150

**Entities in chunk:**
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - District Six Adonis  (also: District Six Adonis)
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
  - South America  (also: South Africa)
  - Tyne and Roger Streets  (also: Tyne)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 65 / 209  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Cape African Teachers' Association  (also: Cape African Teachers Association)
  - Education Department
  - Emily Hobhouse
  - Grey Street  (also: Bree Street)
  - Groote Schuur  (also: Groot Schuur)
  - INDEX 166  (also: INDEX 166)
  - League of Nations  (also: League)
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs. Gladstone  (also: Mrs Gladstone)
  - Prime Minister Botha
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 66 / 209  (id=-3882651649594979630)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #735

**Entities in chunk:**
  - Achee
  - Afrikaans Language Movement
  - Argus Annual  (also: Argus)
  - Cape Town
  - Genadendal
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 67 / 209  (id=-3872024114827273512)

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
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 68 / 209  (id=-3850285751536457852)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #251

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Malays
  - City Council
  - Coloured Affairs
  - Dr Du Plessis  (also: Du Plessis)
  - Heneke  (also: Mr Heneke)
  - Hindu Association
  - I.D. Du Plessis
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - J. Lucheram  (also: Lucheram)
  - Jabavu  (also: Dr. Jabavu)
  - M. Enus  (also: Enus)
  - Mauritius
  - Mr Edross  (also: Edross)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 69 / 209  (id=-3844520640191004859)

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
  - France
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 70 / 209  (id=-3668983573170314510)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #626

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - Guy's Hospital  (also: Guys Hospital)
  - Kimberley Hospital  (also: Kimberley)
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
{"quote": "my father"}
```

**CC quote:** `my father`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 71 / 209  (id=-3635611545960891135)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #505

**Entities in chunk:**
  - District Six  (also: District Six)
  - Dukes
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Louis Messaris
  - Messaris Junior  (also: Mr. Messaris Junior)
  - South America  (also: South Africa)
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
{"relations":[{"from":"Tykie","relation":"child_of","to":"Louis Messaris"}]}
```

**Extracted relations:** none

---

## Chunk 72 / 209  (id=-3473692056619927671)

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
  - India Ghulzar Khan  (also: India)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Government
  - Natal Indian Congress
  - Pretoria Gandhi  (also: Pretoria)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt", "my third eldest brother, Fazil"} 

 Wait, no! I made a mistake.

There are two qualifying clauses:

1. "my aunt" (role: aunt + name: none) -> does not qualify because it lacks a proper name from the persons list.
2. "My third eldest brother, Fazil" (role: brother + name: Fazil)

So, I should return only one valid JSON object with the qualifying clause:

{"quote": "My third eldest brother, Fazil"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 73 / 209  (id=-3364667971095419225)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #463

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape
  - Cape Town
  - Fazil Rassool  (also: Fazil)
  - George
  - Mauritius
  - South African Apartheid
  - Wahida Ta Al8
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 74 / 209  (id=-3325270166547380284)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #406

**Entities in chunk:**
  - Cape Town Fire Brigade
  - District Six  (also: District Six)
  - Grand Parade
  - Heerie
  - Ismail  (also: Ismail)
  - Mr. B. Allie  (also: B. Allie)
  - Nasim Rassool  (also: Nasim)
  - Rutger Street
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

## Chunk 75 / 209  (id=-3319062113236165871)

**Section:** Yousuf (Joe) Rassool   44  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #215

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Arthur Elliott Collection
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Berina Rassool  (also: Berina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape
  - Cobley
  - District Six  (also: District Six)
  - Edward VII
  - Fazil Rassool  (also: Fazil)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hussain Ally  (also: Hoosain Ally)
  - J. Wilson  (also: Wilson)
  - Queen
  - Rasheda Rassool  (also: Rasheda)
  - Western Cape Archives
  - Yasmin Rassool  (also: Yasmin, Jessie, Jessie Rassool)
  - Zainunissa

**Triggers found:** mother, sister

**CC pass (raw):**
```json
{"quote": "my mother Lallie"}
```

**CC quote:** `my mother Lallie`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ayesha Rassool","relation":"parent_of","to":"the narrator"}]}
```

**Extracted relations:** none

---

## Chunk 76 / 209  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - Botanical gardens  (also: Botanical)
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

## Chunk 77 / 209  (id=-3059365739205808916)

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
  - India Ghulzar Khan  (also: India)
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

## Chunk 78 / 209  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - I'll
  - INDEX 166  (also: INDEX 166)
  - Ramamurthi
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

## Chunk 79 / 209  (id=-2892056374499759668)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #381

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town
  - Fazil Rassool  (also: Fazil)
  - Groote Schuur  (also: Groot Schuur)
  - Khadija
  - Slamat
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 80 / 209  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Athlone
  - Charles Simons
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - INDEX 166  (also: INDEX 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Natal Indians  (also: Natal Indian)
  - Pat Holmes
  - Reuter
  - Richard Parker
  - Space
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "whose father was Pat Holmes"}
```

**CC quote:** `whose father was Pat Holmes`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Pat Holmes"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Pat Holmes`

---

## Chunk 81 / 209  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Cape Town
  - Diamond woman  (also: Diamond)
  - District Six  (also: District Six)
  - Five
  - Gool-Rassool  (also: Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - J.M.H  (also: J.M.H.)
  - Jeddah
  - Supreme Court
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zoot

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 82 / 209  (id=-2740820424072264356)

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

## Chunk 83 / 209  (id=-2659876011736734958)

**Section:** 85  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #496

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cobley
  - District Six - Lest We Forget  (also: District Six: Lest We Forget)
  - Dr. Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Granny's niece  (also: Granny)
  - Hussain Ally  (also: Hoosain Ally)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Indian Opinion
  - J. Wilson  (also: Wilson)
  - Jane Gool-Tabata Wahida's daughter
  - Natal
  - South America  (also: South Africa)
  - Trinidad
  - Waxy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 84 / 209  (id=-2550570213336857687)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #52

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Deanna Durbin
  - Gardener Williams  (also: Gardiner Williams)
  - INDEX 166  (also: INDEX 166)
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

## Chunk 85 / 209  (id=-2487606880044980631)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #978

**Entities in chunk:**
  - African National Congress
  - Area
  - Bellingham  (also: Mr Bellingham)
  - Cape Town
  - Freedom Charter
  - Gregori Malenkov  (also: Grigori Malenkov)
  - Kenya Jomo Kenyatta
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

## Chunk 86 / 209  (id=-2266997192548340950)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #95

**Entities in chunk:**
  - Armin Dawood
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - INDEX 166  (also: INDEX 166)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "his brothers-in-law Armin Dawood"}
```

**CC quote:** `his brothers-in-law Armin Dawood`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "his" refers to Yousuf Rassool and the quote states "his brothers-in-law Armin Dawood", we can infer that Armin Dawood is the brother of Yousuf Rassool's spouse. However, since the narrator is 'I' (Yousuf Rassool), we can conclude that Armin Dawood is the sibling_of Wahida Gool (who is likely Yousuf Rassool's spouse).

Here is the extracted relationship in JSON format:

{"relations":[{"from":"Wahida Gool","relation":"sibling_of","to":"Armin Dawood"}]}
```

**Extracted relations:**
- `Wahida Gool` **sibling_of** `Armin Dawood`

---

## Chunk 87 / 209  (id=-2219204583155630177)

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
  - Transvaal Indians  (also: Transvaal Indians)
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

## Chunk 88 / 209  (id=-2140983584723589461)

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
  - Prince of Wales
  - Professor Dadaker  (also: Professor Dadekar)
  - Professor James  (also: Prof James)
  - Progressive Party
  - Prop Diamond  (also: Prop Diamond)
  - Province of Swat
  - Public Slipper & Turkish Baths
  - Punjabi  (also: Punjabi)
  - Purcell  (also: Purcell)
  - Queen Victoria
  - Queen Victoria Street
  - Queens Road
  - Quints  (also: Quints)
  - Quwatul Islam Mosque
  - R. Hoedemaker  (also: Hoedemaker)
  - R.O.  (also: R.O.)
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

## Chunk 89 / 209  (id=-2049559656837982089)

**Section:** Yousuf (Joe) Rassool   144  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #924

**Entities in chunk:**
  - Anti-Zionism
  - England
  - Facts
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

## Chunk 90 / 209  (id=-2047049700032512222)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #223

**Entities in chunk:**
  - Dija
  - Fletchers & Cartwrights  (also: Fletchers)
  - Glen
  - Hamid Ally  (also: Midi)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek
  - Lion's Head
  - Mr Edross  (also: Edross)
  - Muir Street Moslem School
  - Noors  (also: Noors)
  - Signal Hill
  - South America  (also: South Africa)
  - Table Bay
  - Table Mountain
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 91 / 209  (id=-2010807931329773554)

**Section:** Yousuf (Joe) Rassool   x  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #19

**Entities in chunk:**
  - Chinese Nationalists
  - Colindale
  - Generalissimo Chiang Kai Shek
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
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

## Chunk 92 / 209  (id=-1961234090315831513)

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

## Chunk 93 / 209  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Buitencingle Street  (also: Buitencingle)
  - District Six  (also: District Six)
  - Gokhale
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hiroshima
  - INDEX 166  (also: INDEX 166)
  - Kloof Street  (also: Loop Street)
  - Laura Heffer
  - Milan
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nagasaki
  - Noor Bagh
  - President
  - Shaheen Gool  (also: Shaheen)
  - Union of Socialist Soviet Republics  (also: Union of Socialist Soviet Republics)
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

## Chunk 94 / 209  (id=-1671105553751995199)

**Section:** Yousuf (Joe) Rassool   104  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #631

**Entities in chunk:**
  - Arabic Surah
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Broach
  - Bunny Kriekler
  - Earnest Livingston MQotsi
  - Fort Hare  (also: Fort Hare)
  - Gerald Newman
  - Harold Wolpe
  - INDEX 166  (also: INDEX 166)
  - John Clayton
  - Khadija
  - Laurence Olivier
  - Moscow
  - Oliver Caldecott
  - P.V. Tobias
  - Rander district  (also: Rander)
  - Shakespeare Bibi
  - Surat
  - Sydney Brenner
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - World Conference Of Democratic Youth
  - Wuthering Heights  (also: Wuthering Heights)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 95 / 209  (id=-1594342289169322262)

**Section:** 61  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #331

**Entities in chunk:**
  - Cape African Teachers' Association  (also: Cape African Teachers Association)
  - Dickman's Bakery
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanover Street
  - Islamic
  - League of Nations  (also: League)
  - Let Us Live
  - Mr. Maron  (also: Maron)
  - Omar Khayyam
  - Shaheen Gool  (also: Shaheen)
  - Synagogue
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Vandeleur Street
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Waynick  (also: Waynick)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "my mother"}
```

**CC quote:** `my mother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 96 / 209  (id=-1566108535838797793)

**Section:** 81  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #474

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Comrade Bill
  - De Waal Drive
  - Dora Taylor
  - Harry Snitcher
  - Hyman Lieberman Institute  (also: Hyman Liberman Institute)
  - Jaffe  (also: Mr Jaffe)
  - Kotane
  - Mnguni
  - Muir Street
  - Musto
  - Nosipho Majeke
  - Rander district  (also: Rander)
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

## Chunk 97 / 209  (id=-1507950876298588482)

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

**Triggers found:** married, son , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 98 / 209  (id=-1429068271951167841)

**Section:** Chapter Nine  Senior Year  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #564

**Entities in chunk:**
  - Cape Town
  - Clanwilliam
  - Hell's Heights Pass
  - INDEX 166  (also: INDEX 166)
  - Messaris Junior  (also: Mr. Messaris Junior)
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

## Chunk 99 / 209  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis
  - Dr DuPlessis  (also: DuPlessis)
  - Morris Alexander  (also: Morris)
  - Mr Edross  (also: Edross)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 100 / 209  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bourgeois  (also: Bourgeois)
  - C.Khatieb  (also: Khatieb)
  - Democratic Parliament
  - Ebrahim  (also: Ebrahim)
  - G.Abed
  - Gool-Rassool  (also: Gool)
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
  - Ten Point Programme  (also: Ten-Point programme)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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
Since "Muddy's brother" is stated in the quote and Muddy is a known person, we can extract the relationship. However, to determine the exact relationship, we need to find out who Muddy's brother is.

After examining the list of known persons, I couldn't find any direct mention of Muddy's brother being one of them. Therefore, according to the rules, if both endpoints are not identifiable as canonical names, return empty.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 101 / 209  (id=-1290623638020229635)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #194

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - British Bioscope  (also: British Bioscope)
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Fort Hare  (also: Fort Hare)
  - J.Arthur Rank  (also: Arthur Rank)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
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

## Chunk 102 / 209  (id=-1046818788447995973)

**Section:** 47  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #238

**Entities in chunk:**
  - Baby pear
  - Caledon Streets  (also: Caledon Street)
  - Daddy pear
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
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 103 / 209  (id=-1045087825811632573)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #465

**Entities in chunk:**
  - Biology Lab  (also: Biology)
  - Chapter Fifteen Apartheid
  - George
  - James Shirley
  - Joyce
  - Kings Sceptre  (also: Kings)
  - Mr Edross  (also: Edross)
  - Trafalgar
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "his wife Joyce"}
```

**CC quote:** `his wife Joyce`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Joyce"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Joyce`

---

## Chunk 104 / 209  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Education Department
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Grey Street  (also: Bree Street)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - J. Wilson  (also: Wilson)
  - Manilal
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Ralph J Bunche  (also: Ralph Bunche)
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

## Chunk 105 / 209  (id=-693280389959675287)

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

## Chunk 106 / 209  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam H. G. Mahomed
  - Bombay University  (also: Bombay)
  - Charles Simons
  - Gool-Rassool  (also: Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Parker
  - INDEX 166  (also: INDEX 166)
  - J.M.H  (also: J.M.H.)
  - Mahomed
  - Molteno
  - Mr. Schreiner
  - National Union of South African Students  (also: NUSAS)
  - Parker  (also: Mr. Parker)
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
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 107 / 209  (id=-588616802558725797)

**Section:** Yousuf (Joe) Rassool   10  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #85

**Entities in chunk:**
  - Arthur Elliot Collection Courtesy Western Cape Archives
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Calcutta
  - District Six  (also: District Six)
  - Harold Cressy
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Mohammed Saaid Uncle Aity
  - National Gandhi Museum JMH Gool
  - Rt Rev Bishop J A Johnson
  - Solomon Tshekisho Plaatje
  - St Martini German Lutheran Church
  - Trafalgar High  (also: Trafalgar High)
  - Union  (also: Union)

**Triggers found:** in-law

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 108 / 209  (id=-509545831926091450)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #160

**Entities in chunk:**
  - Abdul Kadar
  - Amod Gool
  - Guy's Hospital  (also: Guys Hospital)
  - INDEX 166  (also: INDEX 166)
  - Lachiram
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Hamid Gool

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

## Chunk 109 / 209  (id=-441481017488792687)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #862

**Entities in chunk:**
  - African Affairs
  - Anti-Apartheid
  - Gava Kadoes  (also: Gava)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Houses of Parliament
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

## Chunk 110 / 209  (id=-395839892916216241)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #611

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - District 6
  - Durban's Curry's Fountain  (also: Durban)
  - Ebrahim  (also: Ebrahim)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Louis Messaris
  - Messaris King Peanut Butter
  - Thus
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "Ebrahim, (his elder brother)"}
```

**CC quote:** `Ebrahim, (his elder brother)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ebrahim","relation":"sibling_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Ebrahim` **sibling_of** `Yousuf Rassool`

---

## Chunk 111 / 209  (id=-272192624856094471)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #256

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Caledon
  - Cape Town
  - Gool-Rassool  (also: Gool)
  - Hanief Aboeta
  - Health Department
  - INDEX 166  (also: INDEX 166)
  - Main Road
  - Mount Streets  (also: Mount Street)
  - Rondebosch
  - Transkei
  - University of Cape Town  (also: University of Cape Town)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "as I had been named after his father"}
```

**CC quote:** `as I had been named after his father`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Gool-Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Gool-Rassool`

---

## Chunk 112 / 209  (id=-93354464964914605)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #72

**Entities in chunk:**
  - 5th Baron
  - Alice Greene
  - All India Football team
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town train station
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - George Bernard Shaw
  - Grahame Greene
  - Kimberley Hospital  (also: Kimberley)
  - Lord Headley
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Moravian church
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

## Chunk 113 / 209  (id=32633455946576169)

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

## Chunk 114 / 209  (id=256564604167810176)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #962

**Entities in chunk:**
  - Advocate Christopher
  - British Settlers
  - Chapel Street School  (also: Chapel Street School)
  - Huguenots  (also: Huguenots)
  - Indian Opinion
  - Indian Settlers
  - J. M. H. Gool
  - J. Wilson  (also: Wilson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - New South Africa  (also: New South Africa)
  - Omar Khayyam
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt"}
```

**CC quote:** `my aunt`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 115 / 209  (id=354692539872206818)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #614

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - City
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX 166)
  - Khadija
  - Musto
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Parow
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 116 / 209  (id=501428138843038233)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #511

**Entities in chunk:**
  - Hassan Bavasah  (also: Hassan)
  - Morris Alexander  (also: Morris)
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

## Chunk 117 / 209  (id=519016872720201104)

**Section:** Yousuf (Joe) Rassool   98  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #589

**Entities in chunk:**
  - Allies  (also: Allies)
  - Eastern Front
  - Germany  (also: Germany)
  - Hitler
  - INDEX 166  (also: INDEX 166)
  - Khadija
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
{"relations":[{"from":"Khadija","relation":"sibling_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Khadija` **sibling_of** `Yousuf Rassool`

---

## Chunk 118 / 209  (id=523998670856959750)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #46

**Entities in chunk:**
  - African People's Organisation (A.P.O.)
  - Coloured Affairs Council
  - Communist Party of South Africa
  - Europe
  - Fourth International of South Africa
  - INDEX 166  (also: INDEX 166)
  - Irwin
  - Irwin Combrinck  (also: Irwin Combrick)
  - Jim Londos
  - Liberation League
  - Moravian church
  - New Era Fellowship
  - Non-European Unity Movement  (also: Non European Unity Movement)
  - Smuts's United Party  (also: Smuts)
  - Ten-Point
  - Workers' Party

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 119 / 209  (id=615438439207399386)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #149

**Entities in chunk:**
  - Amra
  - Bertie Louw
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Cassim Amra
  - Communist Party
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hyde Park
  - Ismail  (also: Ismail)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - University of Cape Town  (also: University of Cape Town)
  - Workers' Party
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 120 / 209  (id=628953725132054043)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #605

**Entities in chunk:**
  - Cape Technical College
  - Dreamy
  - Hanover Street
  - INDEX 166  (also: INDEX 166)
  - Junior Certificate  (also: Senior Certificate)
  - Latin
  - Messaris  (also: Mr Messaris)
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

## Chunk 121 / 209  (id=641330737843452673)

**Section:** Yousuf (Joe) Rassool   120  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #748

**Entities in chunk:**
  - Abe Desmore
  - David Poole
  - District Six  (also: District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Grand Parade
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Mrs. Wo
  - NEUM
  - Nerine Desmond
  - TARC
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

## Chunk 122 / 209  (id=776867616384834388)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #603

**Entities in chunk:**
  - Adderley Street  (also: Adderley)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Esther Berelowitz
  - INDEX 166  (also: INDEX 166)
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

## Chunk 123 / 209  (id=787736854281663344)

**Section:** 23  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #140

**Entities in chunk:**
  - Asia
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Jack Meltzer
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

## Chunk 124 / 209  (id=819376064568638174)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #99

**Entities in chunk:**
  - Amina Minnie  (also: Amina)
  - Arriehai
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - College  (also: College)
  - E. Albertus  (also: Albertus)
  - Hamza
  - Head  (also: Head)
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

## Chunk 125 / 209  (id=1153175004995048754)

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

## Chunk 126 / 209  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Barnato Union
  - Ben Malamba
  - Candy's wife
  - Cape Town train station
  - Carlson
  - Colin Wynne
  - Curly
  - Curry's Fountain
  - Cynthia Fisher
  - Even Buitencingle
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Le Grange
  - Lennie Dixon
  - Mehuyzen
  - National Union of South African Students  (also: NUSAS)
  - Oosthuisen  (also: Oosthuisen)
  - Rashid Gardee
  - Rev. Wessels  (also: Wessels)
  - Rousseau
  - Salie Van Haacht
  - Van der Merwe
  - Van der Westhuisen
  - Viljoen
  - Vollenhoven
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

## Chunk 127 / 209  (id=1457016325443525292)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #951

**Entities in chunk:**
  - Anti-CAD  (also: Anti CAD)
  - Charles Starret
  - Club
  - Crown  (also: Crown)
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

## Chunk 128 / 209  (id=1515453391350648521)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #27

**Entities in chunk:**
  - Cape Colony
  - Coloured Advisory Council
  - England
  - George Golding  (also: Mr. George Golding)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hogwood  (also: Mr Hogwood)
  - Ismail Hayat
  - J. Wilson  (also: Wilson)
  - Jameel
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Reading
  - Transvaalers  (also: Transvaalers)
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

## Chunk 129 / 209  (id=1582519559517716798)

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

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 130 / 209  (id=1690720582947668658)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1139

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - African American  (also: African American)
  - Afro-Caribbean Community
  - Britain's Empire  (also: Britain)
  - Cape Town
  - Desmond Green
  - Greenpoint
  - India Ghulzar Khan  (also: India)
  - Israel
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

## Chunk 131 / 209  (id=1743770337024172354)

**Section:** Chapter Eighteen    Chapel Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #965

**Entities in chunk:**
  - A. Roberts  (also: Roberts)
  - Cape Education Department
  - Chapel Street School  (also: Chapel Street School)
  - Danny De Beer  (also: Danny)
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Maritzburg
  - Mohmet
  - National Union of South African Students  (also: NUSAS)
  - Rev R.A. Jackson
  - Star Cinema  (also: Star Cinema)
  - Town Hall
  - United Party Member of Parliament
  - University of South Africa
  - Vivian Bickford-Smith
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 132 / 209  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Abrahams brothers  (also: Abrahams)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Group Areas Act
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Jameel
  - Mount Streets  (also: Mount Street)
  - Nellie Abdurahman
  - Omar
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

## Chunk 133 / 209  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Argus Annual  (also: Argus)
  - Caledon Streets  (also: Caledon Street)
  - Cape
  - Cape Town
  - Dullah Omar
  - Dullie Desai
  - Even Buitencingle
  - Good Hope
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - International
  - Morris Alexander  (also: Morris)
  - Movement
  - Salt River
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

## Chunk 134 / 209  (id=2012893071791774830)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #200

**Entities in chunk:**
  - Biology Lab  (also: Biology)
  - Calcutta
  - Eddie Roux  (also: Eddie)
  - Geoff Abrahams  (also: Geoff)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Jane Abrahams
  - Mowbray
  - Poppie
  - Rander district  (also: Rander)
  - Religious Instruction
  - Shaheen Gool  (also: Shaheen)
  - South America  (also: South Africa)
  - Trafalgar
  - Tuan Koebers
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "my grandfather’s"}
```

**CC quote:** `my grandfather’s`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. The quote states "my grandfather’s", which implies a parent-of relationship.

However, since the schema only allows for child_of, parent_of, sibling_of, half_sibling_of relationships, and the quote does not directly state the name of the person who is Yousuf Rassool's grandfather, we cannot extract a valid relation.
```

**Extracted relations:** none

---

## Chunk 135 / 209  (id=2083741976242093100)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #179

**Entities in chunk:**
  - B.M.  (also: B.M)
  - Bambatta
  - Collected Works  (also: Collected)
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

## Chunk 136 / 209  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Communist Party
  - Group Areas  (also: Group Area)
  - Jaffeist
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

## Chunk 137 / 209  (id=2157482743913839493)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #698

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Caledon Streets  (also: Caledon Street)
  - Congress
  - Gava Kadoes  (also: Gava)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hanover Street
  - Hiema
  - Indian Congress
  - Mohamed Ali Jinnah
  - Mohammed Essop
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muslims C.C  (also: Muslims)
  - Musto
  - Parade
  - Parow
  - Passive Resistance
  - Queens Road
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

## Chunk 138 / 209  (id=2190457622431566710)

**Section:** Yousuf (Joe) Rassool   36  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #183

**Entities in chunk:**
  - Caledon Streets  (also: Caledon Street)
  - District Six  (also: District Six)
  - George Meissenheimer  (also: George Meisenheimer)
  - INDEX 166  (also: INDEX 166)
  - Liesbeek River
  - Majesty
  - Natal
  - Pretoria Gandhi  (also: Pretoria)
  - Trafalgar George
  - Western Province  (also: Eastern Province)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 139 / 209  (id=2228032443713191255)

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
  - Messaris  (also: Mr Messaris)
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

## Chunk 140 / 209  (id=2263436661745583446)

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
  - Three W  (also: Three W)
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

## Chunk 141 / 209  (id=2310160307929513794)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #254

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Douglas Mitchell
  - Fazil Rassool  (also: Fazil)
  - Heneke  (also: Mr Heneke)
  - Maritzburg
  - Mohmet
  - Mr Edross  (also: Edross)
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

## Chunk 142 / 209  (id=2736700062244384633)

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

## Chunk 143 / 209  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - District Six  (also: District Six)
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

## Chunk 144 / 209  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Cape Argus
  - Ethnic Pride
  - Golding Van  (also: Golding)
  - Green Point Commons  (also: Green Point Common)
  - Harry Lawrence
  - INDEX 166  (also: INDEX 166)
  - Israel
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Menachem Begin
  - Minister
  - New Era Fellowship
  - Racial Prejudice
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

## Chunk 145 / 209  (id=2975384613280766645)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #955

**Entities in chunk:**
  - Ben Malamba
  - Dr Verwoerd
  - Eiselen
  - Heathrows C.C
  - INDEX 166  (also: INDEX 166)
  - Muslims C.C  (also: Muslims)
  - Native Affairs
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

## Chunk 146 / 209  (id=3036940243594659710)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #715

**Entities in chunk:**
  - Allie BA
  - Dollies
  - Habibia Institute  (also: Habibia Institute)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mecca
  - Michael Johns
  - Practical Science
  - Ta'Als  (also: Ta'Als)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 147 / 209  (id=3093788627293641191)

**Section:** Yousuf (Joe) Rassool   62  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #339

**Entities in chunk:**
  - C.I.D
  - Caledon Streets  (also: Caledon Street)
  - Cannon Street
  - Cape Times
  - Cape Town
  - Coloured Advisory Council
  - District Six - Lest We Forget  (also: District Six: Lest We Forget)
  - George Golding  (also: Mr. George Golding)
  - Harold Wolpe
  - High School
  - Hogwood  (also: Mr Hogwood)
  - Jackie Marks
  - Natal Daily Witness
  - Western Province Indian Cricket Union
  - Wits  (also: Wits)
  - World Conference
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 148 / 209  (id=3099837090720220907)

**Section:** Yousuf (Joe) Rassool   38  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #193

**Entities in chunk:**
  - Cape Argus
  - Daniel Francois Malan
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Group
  - Local History Museum
  - Manilal
  - Phoenix
  - Prime Minister  (also: Prime Minister)
  - Ramdas
  - South America  (also: South Africa)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)

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

## Chunk 149 / 209  (id=3336594518095080251)

**Section:** Yousuf (Joe) Rassool   64  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #349

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Choir
  - Hussain Ally  (also: Hoosain Ally)
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
  - Khalifa
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:** daughter,  cousin

**CC pass (raw):**
```json
{"quote": "the green-eyed, curly-headed daughter of Hoosain Ally"}
```

**CC quote:** `the green-eyed, curly-headed daughter of Hoosain Ally`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "the green-eyed, curly-headed daughter of Hoosain Ally", we can infer that Yousuf Rassool is referring to a family relationship.

The valid JSON output would be:

{"relations":[{"from":"Hussain Ally","relation":"parent_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Hussain Ally` **parent_of** `Yousuf Rassool`

---

## Chunk 150 / 209  (id=3445648479411821512)

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
  - South America  (also: South Africa)
  - Union of Socialist Soviet Republics  (also: Union of Socialist Soviet Republics)
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

## Chunk 151 / 209  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Ben Malamba
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
  - Heathrow Cricket Club
  - Indian Cricket Union
  - James Africa
  - Salie Van Haacht
  - Seventh Day Adventists
  - Sonny Abdurahman
  - Stanley Abrahams
  - Students Representative Council  (also: SRC)
  - Sunday League  (also: Sunday)
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

## Chunk 152 / 209  (id=3610799175460804304)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #617

**Entities in chunk:**
  - Banqueting Hall
  - Beat
  - Bombay University  (also: Bombay)
  - Buitencingle Street  (also: Buitencingle)
  - District Six  (also: District Six)
  - Dr Kolia
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
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
{"quote": "my cousin"}
```

**CC quote:** `my cousin`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 153 / 209  (id=3621060795652394497)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #619

**Entities in chunk:**
  - Abe Desmore
  - Azad
  - David Poole
  - District Six  (also: District Six)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Mrs. Wo
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

## Chunk 154 / 209  (id=3654419630807047063)

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

## Chunk 155 / 209  (id=3683676736687558285)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #789

**Entities in chunk:**
  - Allies  (also: Allies)
  - Ardenne
  - Generals Montgomery  (also: Generals Montgomery)
  - George Golding  (also: Mr. George Golding)
  - Habibia  (also: Habibia)
  - Hassiem Edross
  - Normandy
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Rev. Gow)
  - Salie Dollie
  - Solly Edross
  - Trafalgar
  - Von Kesselring
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 156 / 209  (id=3694579935737916495)

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
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 157 / 209  (id=3698504895570842680)

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
  - Slingers  (also: Mr Slingers)
  - TARC
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
Since the narrator is "Yousuf Rassool" and the quote mentions "his brothers", we can infer that Yousuf Rassool has brothers, but we don't know their names. However, since the narrator is referring to himself in the third person ("his"), we can assume that the narrator is one of the brothers.

But, according to the rules, if both endpoints are not identifiable as canonical names, return empty.

So, the correct answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 158 / 209  (id=3707629011541248812)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #29

**Entities in chunk:**
  - Cape Town
  - Chapel Street Primary  (also: Chapel Street Primary)
  - District Six  (also: District Six)
  - Habibia Institute  (also: Habibia Institute)
  - Herrenvolk
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 159 / 209  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Benito Mussolini
  - Even
  - Germans  (also: Germans)
  - Heneke  (also: Mr Heneke)
  - INDEX 166  (also: INDEX 166)
  - Moscow
  - Mr Edross  (also: Edross)
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

## Chunk 160 / 209  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town Docks
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - J. Wilson  (also: Wilson)
  - Jadwat
  - Manilal Gandhi
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
  - Party of South Africa  (also: Party)
  - Port Elizabeth Indians
  - Ruth Alexander
  - Seedat  (also: Seedat)
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

## Chunk 161 / 209  (id=4201484364537076764)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #188

**Entities in chunk:**
  - Cape Town
  - Edgar Maurice  (also: Edgar)
  - Empire Day  (also: Empire)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 162 / 209  (id=4713749386019632490)

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
  - High School
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

## Chunk 163 / 209  (id=4812989679158280412)

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
  - Mr. Pearce  (also: Mr Pearce)
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

## Chunk 164 / 209  (id=4888467865479780033)

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

## Chunk 165 / 209  (id=5096918597656922986)

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
{"quote": "my cousin Naz"}
```

**CC quote:** `my cousin Naz`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 166 / 209  (id=5207007205339208384)

**Section:** Yousuf (Joe) Rassool   150  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #957

**Entities in chunk:**
  - Anti-CAD Movement
  - Arabic Surah
  - Broach
  - Cape Town-Woodstock P.T.A.
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Kelvinator
  - Kies  (also: Mr Kies)
  - League of Nations  (also: League)
  - Parent Teachers Association  (also: Parent-Teacher Associations)
  - Rander district  (also: Rander)
  - Rasheda Rassool  (also: Rasheda)
  - Surat
  - Unity Movement  (also: Unity)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Zain Rassool  (also: Zain)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 167 / 209  (id=5228882496868360423)

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
  - Moravian church
  - Queens Rd
  - Rev. Wessels  (also: Wessels)
  - Reza Rassool  (also: Reza)
  - Roland Colman
  - Sam Wo's Laundry
  - United Party
  - Unity Movement  (also: Unity)
  - Victor Wessels

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "The Moravian Church of the Rev Wessels"}
```

**CC quote:** `The Moravian Church of the Rev Wessels`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 168 / 209  (id=5499390051200912173)

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
  - India Ghulzar Khan  (also: India)
  - Java
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Phillipines
  - Prince of Wales
  - Rander district  (also: Rander)
  - Sayed Fethi
  - Sayed Hussain Ally
  - Singapore
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, married, father, mother, sister, niece, nephew

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 169 / 209  (id=5528078340237292279)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #245

**Entities in chunk:**
  - Caledon
  - Europe
  - Health Department
  - Hitler
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

## Chunk 170 / 209  (id=5541857800846579304)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1049

**Entities in chunk:**
  - Alie Fataar
  - Appellate Division
  - Balthazzar John Vorster
  - C.Khatieb  (also: Khatieb)
  - G.Abed
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Hayat Captain
  - Herrenvolk
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX 166)
  - Joyce Meissenheimer
  - S.Abed  (also: Abed)
  - Salie
  - Sayed Kader Sayed Noor
  - Sohnge Training School
  - South America  (also: South Africa)
  - Suppression of Communism Act
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Trafalgar
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

## Chunk 171 / 209  (id=5781509709031527554)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #866

**Entities in chunk:**
  - Arthur Tracey
  - Cape
  - Deanna Durbin
  - George Manuel
  - Hutch
  - INDEX 166  (also: INDEX 166)
  - Messaris  (also: Mr Messaris)
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

## Chunk 172 / 209  (id=5896192977959377292)

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

## Chunk 173 / 209  (id=5967827831367585362)

**Section:** 133  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #843

**Entities in chunk:**
  - Hamid Khan
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Isaac Pfaff
  - Langehoven  (also: Langehoven)
  - Mylie Begg  (also: Mylie)
  - Paul Roubaix
  - Smuts's United Party  (also: Smuts)
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
Since Hamid Khan is Mylie Begg's brother-in-law, that means Hamid Khan is the sibling of Mylie Begg's spouse. However, the quote does not mention Mylie Begg's spouse. Therefore, we cannot establish a direct family relationship between Hamid Khan and any other person mentioned in the known persons list.

According to the rules, if both endpoints are not identifiable as canonical names, return empty.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 174 / 209  (id=6014117459201014898)

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

## Chunk 175 / 209  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - D. Neethling  (also: Neethling)
  - Durban's Curry's Fountain  (also: Durban)
  - Fataar
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Jaffe  (also: Mr Jaffe)
  - Kies  (also: Mr Kies)
  - Meissenheimer  (also: Mrs Meissenheimer)
  - Natal
  - Rev. Wessels  (also: Wessels)
  - Sayed Kader Sayed Noor
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

## Chunk 176 / 209  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Bibi Gool  (also: Bibi, Peari Beghum)
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
Since "my" is used in the quote and no narrator name is given, we can infer that the narrator's grandfather is related to Abdurahman. However, without knowing the narrator's name, we cannot establish a direct relationship between two known persons.

But since the quote mentions "my grandfather", it implies that the narrator is the child or grandchild of someone, and Abdurahman has a bond with this person. Given the information available, we can only infer a relationship between Abdurahman and an unknown person (the narrator's grandfather).

However, based on the rules provided, since both endpoints are not identifiable as canonical names, the correct response would be:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 177 / 209  (id=6105164271277742615)

**Section:** 57  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #292

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cecil John Rhodes
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Differential Calculus
  - Economic History
  - Habibia Institute  (also: Habibia Institute)
  - Hassen Abrahams
  - Integral Calculus
  - Liquid Geometry
  - Mathematics
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

## Chunk 178 / 209  (id=6295959623867363577)

**Section:** Chapter Seventeen  New Era Fellowship  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #933

**Entities in chunk:**
  - B.Sc
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
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - University of Cape Town  (also: University of Cape Town)
  - Van der Merwe
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

## Chunk 179 / 209  (id=6307174216090843246)

**Section:** 91  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #546

**Entities in chunk:**
  - Avenue
  - Buitencingle Street  (also: Buitencingle)
  - Eddie Roux  (also: Eddie)
  - Gardens
  - George
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Indian Opinion Aug 20, 1904
  - Jan Van Riebeeck
  - Kassiem
  - Little Theatre
  - Manuel
  - Mount Nelson Hotel  (also: Mount Nelson Hotel)
  - National Gandhi Museum  (also: National Gandhi Musem)
  - Parliament  (also: Parliament)
  - Peter Abrahams
  - Poppie
  - Shariefa Khan
  - South African Public Library  (also: South African Public Library)
  - St. George's Grammar
  - Tell Freedom

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 180 / 209  (id=6317719585632729785)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #398

**Entities in chunk:**
  - Bagus Allie  (also: Bagus-Allie)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Davids
  - Durban's Curry's Fountain  (also: Durban)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - Ismail  (also: Ismail)
  - Khadija
  - Malaai  (also: Malaai)
  - Parow
  - Persotem Patel  (also: Persotem)
  - Rashid
  - Sarlegh Doalie
  - Shawquet
  - Strand
  - Suleiman Vallie
  - Tiddles
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

## Chunk 181 / 209  (id=6339413612312511153)

**Section:** 107  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #655

**Entities in chunk:**
  - Bellville
  - Britain's Empire  (also: Britain)
  - Darling St  (also: Darling St.)
  - Durban's Curry's Fountain  (also: Durban)
  - France
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - Henry Newbolt Though
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
  - Middle East
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Narutham Godse
  - Pakistan
  - State of Israel  (also: State)
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

## Chunk 182 / 209  (id=6524690631373597935)

**Section:** Yousuf (Joe) Rassool   28  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #163

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Advocate Christopher
  - Ben Cloete
  - Cape Town
  - Cecil Road
  - Chapel Street
  - Chapel Street Primary  (also: Chapel Street Primary)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Government
  - London
  - Natal
  - Primary School  (also: Primary)
  - Unity Movement  (also: Unity)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle"}
```

**CC quote:** `my uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 183 / 209  (id=6536075468746945958)

**Section:** 19  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #124

**Entities in chunk:**
  - Buitencingle Street  (also: Buitencingle)
  - Cape
  - Chaganlal Gandhi
  - Colin Wynne
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX 166)
  - Isaac Pfaff
  - Melanie J. House
  - Paul Roubaix
  - Peninsula Dramatic Society
  - Quwatul Islam Mosque
  - South African Stage
  - Trafalgar Players
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "his eldest son"}
```

**CC quote:** `his eldest son`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his eldest son", we can infer that Yousuf Rassool has a child. However, the name of the child is not mentioned.

But since the quote uses "his" instead of "my", it implies that the narrator (Yousuf Rassool) is referring to someone else's child. Therefore, no family relationship can be extracted from this quote with respect to Yousuf Rassool.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 184 / 209  (id=6546657237567362266)

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

## Chunk 185 / 209  (id=6557428703332889295)

**Section:** Chapter Eight    Anti-CAD  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #520

**Entities in chunk:**
  - Amazon Kindle Direct Publishing
  - Britain's Empire  (also: Britain)
  - British Newspaper Library
  - Ciraj Rassool
  - District Six Museum
  - E. Reddy  (also: Reddy)
  - Kwaai Oak
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

## Chunk 186 / 209  (id=6710772805342046917)

**Section:** 65  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #359

**Entities in chunk:**
  - Aesop
  - Ben Cloete
  - Cape
  - Cape Times Law Reports
  - Chapel Street School  (also: Chapel Street School)
  - E.A. Parker  (also: Mr. E.A. Parker)
  - Faried Rossier
  - Germany  (also: Germany)
  - Gladwin  (also: Gladwin)
  - Habibia Institute  (also: Habibia Institute)
  - Hussein Parker
  - Indian Opinion
  - J. Wilson  (also: Wilson)
  - Jannat
  - John Arlott
  - Kingsmead stadium  (also: Kingsmead)
  - Nasim Rassool  (also: Nasim)
  - Natal
  - Ralph J Bunche  (also: Ralph Bunche)
  - Robert R. Edgar
  - South America  (also: South Africa)
  - Supreme Court Proceedings
  - Trinidad

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "the birth of my baby brother, Nasim"}
```

**CC quote:** `the birth of my baby brother, Nasim`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Nasim Rassool","relation":"child_of","to":"Hussein Parker"}]}
```

**Extracted relations:**
- `Nasim Rassool` **child_of** `Hussein Parker`

---

## Chunk 187 / 209  (id=6941537010882242866)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #80

**Entities in chunk:**
  - Cape
  - Eddieas
  - Egypt
  - Field Marshal Tito
  - Hamid
  - INDEX 166  (also: INDEX 166)
  - J. Wilson  (also: Wilson)
  - Kader Essack  (also: Kader)
  - Muir Street
  - Nasima
  - Poppie
  - Shariefa Khan
  - Standard Seven  (also: Standard Seven)
  - Standard Six  (also: Standard Six)
  - Strand
  - USSR  (also: USSR)
  - Unity Movement  (also: Unity)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 188 / 209  (id=6966615310251134524)

**Section:** Yousuf (Joe) Rassool   28  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #162

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Barnato Board
  - District Six  (also: District Six)
  - England
  - Fazil Rassool  (also: Fazil)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen, Hassen Mall)
  - INDEX 166  (also: INDEX 166)
  - London
  - Lord Elgin
  - Meneer
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Movement
  - SOYA
  - Salie Van Haacht
  - Transvaal Indians  (also: Transvaal Indians)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "my uncle"}
```

**CC quote:** `my uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 189 / 209  (id=7168244047913479805)

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

## Chunk 190 / 209  (id=7243012516835118461)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #407

**Entities in chunk:**
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Bagus Allie  (also: Bagus-Allie)
  - Caledon Streets  (also: Caledon Street)
  - Durban's Curry's Fountain  (also: Durban)
  - Group Areas Board  (also: Group Area Boards)
  - Heerie
  - INDEX 166  (also: INDEX 166)
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

## Chunk 191 / 209  (id=7262414809749442984)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #699

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Alan Melville
  - Athol Rowan
  - Bruce Mitchell
  - Cape Town
  - Chapel Street Primary School  (also: Chapel Street Primary School)
  - England
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX 166)
  - India Ghulzar Khan  (also: India)
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

## Chunk 192 / 209  (id=7266467364867027437)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #107

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Aminabhen
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool  (also: Annie)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bombay University  (also: Bombay)
  - Cape Town
  - District Six  (also: District Six)
  - E. Reddy  (also: Reddy)
  - John Phillips  (also: Rev John Phillips)
  - Mauritius
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Nasima
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 193 / 209  (id=7360610042715665275)

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
  - Urdu  (also: Urdu)
  - Vic Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 194 / 209  (id=7448875471219160758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #480

**Entities in chunk:**
  - Cecil Wightman
  - Chaplin
  - E. Albertus  (also: Albertus)
  - Habibia  (also: Habibia)
  - Hitler
  - INDEX 166  (also: INDEX 166)
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

## Chunk 195 / 209  (id=7574535754100824916)

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
  - I.Begg  (also: Begg)
  - Ignazio Silone
  - India Ghulzar Khan  (also: India)
  - Jeddah
  - Kies  (also: Mr Kies)
  - Labour Party
  - Qudrat
  - Socialism
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

## Chunk 196 / 209  (id=7584768665477924623)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #81

**Entities in chunk:**
  - Alie Fataar
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - Edna
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - George Meissenheimer  (also: George Meisenheimer)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Gwen Wilcox
  - Halima Dudley
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - India Ghulzar Khan  (also: India)
  - Joyce Dixon
  - Joyce Meissenheimer
  - Kenny Jordaan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Veronica Pienaar
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "his aged mother in India"}
```

**CC quote:** `his aged mother in India`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 197 / 209  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Aunty Minnie
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Grand Parade
  - Helen Abrahams
  - INDEX 166  (also: INDEX 166)
  - John Brown  (also: John Brown)
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

## Chunk 198 / 209  (id=7727744974142389137)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #884

**Entities in chunk:**
  - Burger
  - Caledon Streets  (also: Caledon Street)
  - Cape Town
  - Durban's Curry's Fountain  (also: Durban)
  - E. Albertus  (also: Albertus)
  - England
  - INDEX 166  (also: INDEX 166)
  - Khadija
  - Kloof Nek
  - Parker  (also: Mr. Parker)
  - Persotem Patel  (also: Persotem)
  - S.Abed  (also: Abed)
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

## Chunk 199 / 209  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Chemistry Physics
  - Cyril
  - Dawood Seedat  (also: Dawood)
  - Durban's Curry's Fountain  (also: Durban)
  - Fatima Seedat
  - INDEX 166  (also: INDEX 166)
  - Professor James  (also: Prof James)
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

## Chunk 200 / 209  (id=7794589649810439109)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #116

**Entities in chunk:**
  - Allies  (also: Allies)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Buitencingle Street  (also: Buitencingle)
  - Cape Town Fire Brigade
  - Grand Parade
  - INDEX 166  (also: INDEX 166)
  - Merwe
  - Mr. B. Allie  (also: B. Allie)
  - Rutger Street
  - Union Fruit Supply
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

## Chunk 201 / 209  (id=7992762871094121264)

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

## Chunk 202 / 209  (id=8003152637512369339)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #570

**Entities in chunk:**
  - Bonny-Bridge Dover Stove
  - Buitencingle Street  (also: Buitencingle)
  - Caledon Streets  (also: Caledon Street)
  - INDEX 166  (also: INDEX 166)
  - Kloof Nek
  - Mrs Domingo  (also: Domingo)
  - South African Public Library  (also: South African Public Library)
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

## Chunk 203 / 209  (id=8317664100809929487)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #911

**Entities in chunk:**
  - Cape Town
  - Coloured Affairs
  - Darling St  (also: Darling St.)
  - District Six  (also: District Six)
  - Dr DuPlessis  (also: DuPlessis)
  - Dr. Gool
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hoosain
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion Dec
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

## Chunk 204 / 209  (id=8675141585945090217)

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
  - District Six Tomorrow
  - Fresh Fields
  - Ghulzar Khan  (also: Gulzar Khan)
  - Hanover Street
  - Hayat brothers
  - Hoosain Parker
  - Indian Affairs Department
  - Menage Rassool
  - Mohamed Giri
  - Moorgas Naidoo
  - Pastures New Lycidas
  - South America  (also: South Africa)
  - Tiny Abed
  - Tiny Abed family

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

## Chunk 205 / 209  (id=8719056895392496748)

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
  - India Ghulzar Khan  (also: India)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ken Viljoen
  - Khadija
  - Melville
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Mrs Ahmed  (also: Ahmed)
  - Paul Winslow
  - South America  (also: South Africa)
  - Trafalgar High  (also: Trafalgar High)
  - Trafalgar High Persotem
  - Urdu  (also: Urdu)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , son , daughter, father

**CC pass (raw):**
```json
{"quote": "his son BG"}
```

**CC quote:** `his son BG`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Abdul Hamid Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `Abdul Hamid Gool`

---

## Chunk 206 / 209  (id=8730565953127446314)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #997

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Germany  (also: Germany)
  - INDEX 166  (also: INDEX 166)
  - Keraan
  - Messrs Oosterwyk  (also: Messrs Oosterwyk)
  - Mrs Ahmed  (also: Ahmed)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 207 / 209  (id=8807579643631700797)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #241

**Entities in chunk:**
  - Cape
  - Chemistry Physics
  - Dija
  - Gool-Rassool  (also: Gool)
  - Jesus Christ
  - Jews  (also: Jews)
  - Morris Alexander  (also: Morris)
  - Professor James  (also: Prof James)
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 208 / 209  (id=9062310575080274351)

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
  - Indian Congress
  - James Shirley
  - Kimberley Hospital  (also: Kimberley)
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

## Chunk 209 / 209  (id=9200706703269745412)

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
| Chunks processed | 209 |
| Relations extracted | 33 |
| Relations written to graph | 33 |
