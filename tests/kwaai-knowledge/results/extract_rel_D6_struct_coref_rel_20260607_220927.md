# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 139  
**Sampled:** 139  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 139  (id=-9026931979920257841)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #209

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Zarina

**Triggers found:** wed , daughter, mother, sister, brother,  cousin,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "my dad and aunt were brother and sister"}
```

**CC quote:** `my dad and aunt were brother and sister`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the NARRATOR name is given as "Fatima Gool" (also known as "Timmie", "Auntie Timmie"), we can use that name as 'from'. The quote states that "my dad and aunt were brother and sister". Since "aunt" is used, according to the rules, this relation type is not in the schema, so:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 2 / 139  (id=-8565879665326676003)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #890

**Entities in chunk:**
  - Achee
  - Achmed
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head
  - Meneer
  - Mr Pearce  (also: Pearce)
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

## Chunk 3 / 139  (id=-8505583857113009851)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #373

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
  - Ronnie Jooste  (also: Ronnie)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "my parents"}
```

**CC quote:** `my parents`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 4 / 139  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Ben Kies
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - E.L.Maurice
  - F.Landman
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - GL Abrahams
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Joosub
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Sayed Kader Sayed Noor
  - Victor Wessels

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
{"quote": "Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines."}
```

**CC quote:** `Fatima (Timmie) devoted herself to religious studies after matric, but also helped in her brother’s surgery as a receptionist and dispenser of medicines.`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Fatima Gool","relation":"sibling_of","to":"brother's name not mentioned"}]}
```

**Extracted relations:** none

---

## Chunk 5 / 139  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Ebrahim Ali  (also: Ebrahim)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Natasha
  - Nikita Khruschev
  - Noors
  - Norman Bethune
  - Nosipho Majeke
  - Nurjahan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - O.Henry  (also: O. Henry)
  - Ob-Yennesi
  - Oliver Caldecott
  - Omar Abrahams
  - Omar Khayyam

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "Fatima, heavy with child, and Ebrahim, her brother-in-law"}
```

**CC quote:** `Fatima, heavy with child, and Ebrahim, her brother-in-law`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ebrahim Ali","relation":"spouse_of","to":"Fatima Gool"}]}
```

**Extracted relations:**
- `Ebrahim Ali` **spouse_of** `Fatima Gool`

---

## Chunk 6 / 139  (id=-8041283814668973231)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #109

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 7 / 139  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Alexander
  - Boy Uncle Mamoo
  - Churchill
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Montgomery
  - Renee
  - Rommel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina

**Triggers found:** father,  cousin

**CC pass (raw):**
```json
{"quote": "my cousin Zarina, whom we all called Renee"}
```

**CC quote:** `my cousin Zarina, whom we all called Renee`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 8 / 139  (id=-7549255937705633040)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #947

**Entities in chunk:**
  - Alexander
  - Carlson
  - Colin Wynne
  - Cynthia Fisher
  - Dylan
  - Fazil Rassool  (also: Fazil)
  - Feyruz
  - George Veldsman
  - Joseph
  - Lennie Dixon
  - Leonardo
  - Lewis
  - Reza Rassool  (also: Reza)
  - Samuel
  - Zarina

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 9 / 139  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Chops
  - Gootie
  - Grootjie
  - INDEX 166  (also: INDEX	 166)
  - James Africa
  - Woody Woodman
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

## Chunk 10 / 139  (id=-7025477089957931364)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #573

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bartnicke
  - Charles Simons
  - Geoff Abrahams  (also: Geoff)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Richard Parker
  - Slingers  (also: Mr. Slingers)
  - Springveld
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

## Chunk 11 / 139  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - Agent General  (also: Agent-General)
  - E. Albertus  (also: Albertus)
  - Hymie Beimel  (also: Hymie)
  - Smuts
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 12 / 139  (id=-6782705425542896048)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #113

**Entities in chunk:**
  - Bhokkie
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Holy Prophet
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

## Chunk 13 / 139  (id=-6710587015263367697)

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

But wait! Since the quote mentions "grandfather's daughters", we can infer that the grandfather is the parent_of Yousuf Rassool's parent. And since the daughters are mentioned as "my grandfather's daughters", it implies that they are the siblings of Yousuf Rassool's parent.

However, this information is not directly stated in the quote and requires some inference. According to the CRITICAL RULES, we should only extract what the quote directly states.

Since the quote does not directly state any relationship between identifiable canonical names, we return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 14 / 139  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Councillor Parker
  - E. Albertus  (also: Albertus)
  - G.Hoosain  (also: Hoosain)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX	 166)
  - Mussolini
  - Parker  (also: Mr. Parker)
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
Here is the extracted family relationship in the exact format:

{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"G.Hoosain"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `G.Hoosain`

---

## Chunk 15 / 139  (id=-6563863471018850689)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1136

**Entities in chunk:**
  - Athletic
  - Ebrahim Ali  (also: Ebrahim)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Isgak Adams
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Persotem Patel  (also: Persotem)
  - Professor James  (also: Prof. James)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Rev R.A. Jackson
  - Reverend Gow  (also: Gow)
  - Roberts
  - Solly Mohammed
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 16 / 139  (id=-6141146778954841009)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #506

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Ebrahim Ali  (also: Ebrahim)
  - Hassen Mall  (also: Hassen)
  - Keraan
  - Louis Messaris
  - Messrs Oosterwyk
  - Mrs Ahmed  (also: Ahmed)
  - Tykie Messaris  (also: Tykie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 17 / 139  (id=-6067802038126529915)

**Section:** Yousuf (Joe) Rassool   6  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #64

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dr Jabavu  (also: Jabavu)
  - Gava-Kadoes  (also: Gava)
  - Hiema
  - J Lucheram  (also: Lucheram)
  - M Enus  (also: Enus)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf

**Triggers found:** wife, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "his brother’s"}
```

**CC quote:** `his brother’s`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote mentions "his brother's", we can infer that Yousuf Rassool is referring to his own brother. However, the quote does not mention the name of the brother.

But since the narrator is "Yousuf Rassool" and it says "his brother's", we know that the brother must be a sibling of Yousuf Rassool. 

Looking at the list of known persons, we can see that there are multiple individuals with the surname "Rassool". However, only one other person has this surname: Ayesha Rassool.

Therefore, based on the information provided and the rules specified, we can infer that:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Ayesha Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Ayesha Rassool`

---

## Chunk 18 / 139  (id=-6012376786615465402)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1022

**Entities in chunk:**
  - Cassiem
  - Fazil Rassool  (also: Fazil)
  - J. Wilson  (also: Wilson)
  - Mrs Wooding  (also: Wooding)
  - Tregger
  - Van der Ross
  - Vassan (Hindu C.C.)  (also: Vassan)
  - Veldsman
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 19 / 139  (id=-5915939077846947542)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #738

**Entities in chunk:**
  - Charles Simons
  - Councillor Parker
  - INDEX 166  (also: INDEX	 166)
  - Mitchell
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

## Chunk 20 / 139  (id=-5897797065487598428)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #70

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 21 / 139  (id=-5853235968246498787)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #199

**Entities in chunk:**
  - Achee
  - Amina Minnie  (also: Amina)
  - Daria
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
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
Since the narrator is "Yousuf Rassool" and the quote mentions "my grandfather", we can infer that Yousuf Rassool's grandfather is Haji Joosub Maulvi Hamid Gool.

Here is the extracted relationship in JSON format:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Haji Joosub Maulvi Hamid Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Haji Joosub Maulvi Hamid Gool`

---

## Chunk 22 / 139  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E. S. Reddy)
  - Hamid Khan
  - J. M. H. Gool
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
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. However, the quote mentions an aunt (Fatima) which is not in the schema. Therefore, according to the rules:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 23 / 139  (id=-5786981904019721036)

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

## Chunk 24 / 139  (id=-5714165220014992650)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #255

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jack Meltzer
  - Kies  (also: Mr Kies)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Ronnie Jooste  (also: Ronnie)
  - Trotskyist
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

## Chunk 25 / 139  (id=-5556520109474695452)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #135

**Entities in chunk:**
  - Australians
  - Barmania MA
  - Dr Jabavu  (also: Jabavu)
  - Dr. Jamieson  (also: Jamieson)
  - George Golding
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX	 166)
  - Mr. B. Allie  (also: B. Allie)
  - New Zealanders
  - Salie Dollie
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
Since the narrator is "Yousuf Rassool" and the quote mentions "My grandfather", we can infer that Yousuf Rassool's parent is the child of the mentioned person. However, since the name of the grandfather is not provided in the list of known persons, we cannot establish a valid relation.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 26 / 139  (id=-5460737940864037782)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1137

**Entities in chunk:**
  - Abdurahman
  - Bellingham  (also: Mr Bellingham)
  - Chota
  - Gadija Dollie
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid Midi  (also: Hamid (Midi))
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Ralph J Bunche  (also: Ralph Bunche)
  - Sayed Fethi
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

## Chunk 27 / 139  (id=-5418155653937249668)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1145

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E. S. Reddy)
  - Emily Hobhouse
  - Gladstone
  - Goulam Gool
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs.
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
```

**Extracted relations:** none

---

## Chunk 28 / 139  (id=-5364503672193663248)

**Section:** Yousuf (Joe) Rassool   16  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #119

**Entities in chunk:**
  - Alexander
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dylan
  - Feyruz
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
  - Zarina

**Triggers found:** mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "Front Row: Nurjehan, Mom (Ayesha), Granny (Bibi)"}

This clause identifies a direct family relationship between three people:

* Nurjehan is the child of Ayesha (Mom)
* Bibi is the grandmother (Granny) of Nurjehan
```

**CC quote:** `Front Row: Nurjehan, Mom (Ayesha), Granny (Bibi)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{
"from":"Nurjehan Gool Mohamed",
"relation":"child_of",
"to":"Ayesha Rassool"
},{
"from":"Nurjehan Gool Mohamed",
"relation":"grandchild_of",
"to":"Bibi Gool"
}]
```

**Extracted relations:** none

---

## Chunk 29 / 139  (id=-5308475734057125055)

**Section:** 63  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #348

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Khalifa
  - Miss Brown  (also: Brown)
  - Mrs Domingo  (also: Domingo)
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

## Chunk 30 / 139  (id=-5184936258167894087)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #714

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Ghulzar Khan  (also: Gulzar Khan)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - J. Wilson  (also: Wilson)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 31 / 139  (id=-5154709854984411195)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #761

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Compton
  - Doug Wright
  - E.A. Parker
  - Edrich
  - Faried Rossier
  - Gladwin
  - Hassan Bavasah  (also: Hassan)
  - Hussein Parker
  - Hutton
  - John Arlott
  - Nasim Rassool  (also: Nasim)
  - Roly Jenkins
  - Washbrook
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "Hussein Parker, son of E.A. Parker"}
```

**CC quote:** `Hussein Parker, son of E.A. Parker`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"E.A. Parker","relation":"parent_of","to":"Hussein Parker"}]}
```

**Extracted relations:**
- `E.A. Parker` **parent_of** `Hussein Parker`

---

## Chunk 32 / 139  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hoosain Parker
  - INDEX 166  (also: INDEX	 166)
  - Mister Efficiency
  - Paddy Thomas
  - Salie Van Haacht
  - Stanley Abrahams
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband

**CC pass (raw):**
```json
{"quote": "Fatima and her husband"}
```

**CC quote:** `Fatima and her husband`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Fatima Gool","relation":"spouse_of","to":"Unknown"}]}
```

**Extracted relations:** none

---

## Chunk 33 / 139  (id=-5010050899247603262)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1138

**Entities in chunk:**
  - Abdurahman
  - Alan Gregor Cobley
  - Edgar Maurice
  - J.M.H. Wilson
  - James M.
  - John Brown
  - Nasima
  - Oswald Pirow

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 34 / 139  (id=-4713556574132170500)

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

## Chunk 35 / 139  (id=-4263780790926522616)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #187

**Entities in chunk:**
  - Abdurahman
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Henry Sylvester Williams
  - J. M. H. Gool
  - J.Boyce
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Nehru
  - Nelson Eddy
  - Nelson Mandela
  - Nerine Desmond
  - Neville Alexander
  - Nicky Springveld
  - Nikita Khruschev
  - Noor
  - Noors
  - Sayed Kader Sayed Noor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 36 / 139  (id=-4250574601481422414)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #424

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Desmond Green
  - Diamond woman  (also: Diamond)
  - Harry Lawrence
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 37 / 139  (id=-4183482280000304421)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #31

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - J. Wilson  (also: Wilson)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Williams

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 38 / 139  (id=-4128970452264885702)

**Section:** Yousuf (Joe) Rassool   134  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #846

**Entities in chunk:**
  - Baboo Abed
  - Halima Gool  (also: Halima)
  - Hamid Khan
  - Ismail
  - Selim Gool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 39 / 139  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Emily Hobhouse
  - INDEX 166  (also: INDEX	 166)
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 40 / 139  (id=-3872024114827273512)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #78

**Entities in chunk:**
  - Budenny
  - C. Rassool
  - Churchill
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Comrade Bill
  - Field Marshal Smuts
  - Harry Snitcher
  - King Edward VII
  - Malinovsky
  - Moses Kotane
  - Musto
  - Ray Alexander
  - Rokossovsky
  - Russians
  - Smuts
  - Victor Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 41 / 139  (id=-3844520640191004859)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #96

**Entities in chunk:**
  - Aminabhen
  - Armin Dawood
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - INDEX 166  (also: INDEX	 166)
  - Ronnie Jooste  (also: Ronnie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother, sister, brother, in-law

**CC pass (raw):**
```json
{"quote": "his brothers-in-law Armin Dawood"}
```

**CC quote:** `his brothers-in-law Armin Dawood`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "his" refers to Yousuf Rassool and the quote states "his brothers-in-law Armin Dawood", we can infer that Armin Dawood is the brother of Yousuf's spouse. However, since the narrator is not explicitly stated as 'my' with a given name, we cannot directly use Yousuf Rassool as the 'from' endpoint.

But, considering the rules and the quote, we can deduce that Yousuf Rassool is the person being referred to by "his". Therefore, Armin Dawood is the sibling_of the spouse of Yousuf Rassool. However, this relation type is not directly supported by the schema.

Since the direct relation cannot be extracted using the given schema, we return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 42 / 139  (id=-3578174338829636170)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #90

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Sarleh Abdul
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , in-law

**CC pass (raw):**
```json
{"quote": "Your loving son, Al Hajj Joosub Maulvi Hamid"}
```

**CC quote:** `Your loving son, Al Hajj Joosub Maulvi Hamid`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Al Hajj Joosub Maulvi Hamid","relation":"child_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Al Hajj Joosub Maulvi Hamid` **child_of** `Yousuf Rassool`

---

## Chunk 43 / 139  (id=-3473692056619927671)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #227

**Entities in chunk:**
  - Dada Abdullah
  - Fazil Rassool  (also: Fazil)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt", "my third eldest brother, Fazil"} 

 Wait, no! I made a mistake!

According to the rules, I should only return one quote that exactly matches the format.

Here is the corrected answer:

 {"quote": "my third eldest brother, Fazil"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 44 / 139  (id=-3325270166547380284)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #406

**Entities in chunk:**
  - Heerie
  - Ismail
  - Mr. B. Allie  (also: B. Allie)
  - Nasim Rassool  (also: Nasim)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer

**Triggers found:** brother,  cousin

**CC pass (raw):**
```json
{"quote": "my baby brother"}
```

**CC quote:** `my baby brother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 45 / 139  (id=-3319062113236165871)

**Section:** Yousuf (Joe) Rassool   44  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #215

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Berina Rassool  (also: Berina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bibi's baby
  - Edward VII  (also: Edward)
  - Fazil Rassool  (also: Fazil)
  - Granny's niece  (also: Granny)
  - Hoosain Ally  (also: Hussain Ally)
  - J. Wilson  (also: Wilson)
  - Jane Gool-Tabata Wahida's daughter
  - Queen
  - Rasheda Rassool  (also: Rasheda)
  - Wright
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
{"relations":[{"from":"Ayesha Rassool","relation":"parent_of","to":"Abdul Hamid Gool"}]}
```

**Extracted relations:**
- `Ayesha Rassool` **parent_of** `Abdul Hamid Gool`

---

## Chunk 46 / 139  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - C.Khatieb  (also: Khatieb)
  - Hayat Captain
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX	 166)
  - Jochee
  - Next Hassen
  - Nurjahan
  - Rasheda Rassool  (also: Rasheda)
  - Rashid Gardee  (also: Rashid)
  - Raza
  - S.Abed  (also: Abed)
  - Salie
  - Shawquet
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

## Chunk 47 / 139  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - Little Raza
  - Manuel
  - Ramamurthi
  - William Wordsworth
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaatjie
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

## Chunk 48 / 139  (id=-2892056374499759668)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #381

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
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

## Chunk 49 / 139  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Charles Simons
  - Charles Starret
  - E.S. Reddy  (also: Mr. E. S. Reddy)
  - INDEX 166  (also: INDEX	 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Pat Holmes
  - Richard Parker
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

## Chunk 50 / 139  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - J.M.H  (also: J.M.H.)
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 51 / 139  (id=-2742088165969134957)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #716

**Entities in chunk:**
  - Greek God
  - Isgak Adams
  - Reza Rassool  (also: Reza)
  - Thomas Bowler
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zaheer

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 52 / 139  (id=-2550570213336857687)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #52

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Deanna Durbin
  - Gardiner Williams  (also: Gardener Williams)
  - Leslie Hutchinson
  - Nelson Eddy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent, sister

**CC pass (raw):**
```json
{"quote": "my parents"}
```

**CC quote:** `my parents`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 53 / 139  (id=-2266997192548340950)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #95

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Armin Dawood
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - Once
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 54 / 139  (id=-2049559656837982089)

**Section:** Yousuf (Joe) Rassool   144  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #924

**Entities in chunk:**
  - Hymie Beimel  (also: Hymie)
  - Rasheda Rassool  (also: Rasheda)
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

## Chunk 55 / 139  (id=-2010807931329773554)

**Section:** Yousuf (Joe) Rassool   x  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #19

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Generalissimo Chiang Kai Shek
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Rasheda Rassool  (also: Rasheda)
  - Zain Rassool  (also: Zain)

**Triggers found:** wife, daughter, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "her brother, Dr. Goolam"}
```

**CC quote:** `her brother, Dr. Goolam`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Nazima Rassool","relation":"sibling_of","to":"Goolam Gool"}]}
```

**Extracted relations:**
- `Nazima Rassool` **sibling_of** `Goolam Gool`

---

## Chunk 56 / 139  (id=-1961234090315831513)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #380

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed
  - Fazil Rassool  (also: Fazil)
  - Halima Gool  (also: Halima)
  - Hassen Mall  (also: Hassen)
  - Khan  (also: Mr. Khan)
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

## Chunk 57 / 139  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gokhale
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Laura Heffer
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
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

## Chunk 58 / 139  (id=-1566108535838797793)

**Section:** 81  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #474

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Comrade Bill
  - Dora Taylor
  - Harry Snitcher
  - Kotane
  - Mnguni
  - Mr. Jaffe  (also: Jaffe)
  - Musto
  - Nosipho Majeke
  - Ray Alexander
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 59 / 139  (id=-1507950876298588482)

**Section:** 183  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1146

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Heneke  (also: Mr Heneke)
  - Henry Sylvester Williams
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Owen Mathurin
  - Ralph J Bunche  (also: Ralph Bunche)
  - Solly Edross
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, son , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 60 / 139  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Abel
  - Ben Kies
  - Demetrious Capenatakis
  - Dr. DuPlessis  (also: DuPlessis)
  - Edross
  - Morris Alexander  (also: Morris)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 61 / 139  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bourgeois
  - C.Khatieb  (also: Khatieb)
  - Ebrahim Ali  (also: Ebrahim)
  - G.Abed
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Joosub
  - Malick Hayat Captain
  - S.Abed  (also: Abed)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "Muddy’s brother Ebrahim"}
```

**CC quote:** `Muddy’s brother Ebrahim`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Muddy","relation":"sibling_of","to":"Ebrahim Ali"}]}
```

**Extracted relations:** none

---

## Chunk 62 / 139  (id=-1290623638020229635)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #194

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Arthur Rank
  - Heyl  (also: Mr. Heyl)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Robert Newton
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

## Chunk 63 / 139  (id=-1045087825811632573)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #465

**Entities in chunk:**
  - Edross
  - George
  - James Shirley
  - Joyce
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

## Chunk 64 / 139  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - J. Wilson  (also: Wilson)
  - Manilal Gandhi  (also: Manilal)
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

## Chunk 65 / 139  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam H. G. Mahomed
  - Charles Simons
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Parker
  - J.M.H  (also: J.M.H.)
  - Molteno
  - Parker  (also: Mr. Parker)
  - Richard Parker
  - S.Abed  (also: Abed)
  - Schreiner
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

## Chunk 66 / 139  (id=-509545831926091450)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #160

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Kadar
  - Amod Gool
  - INDEX 166  (also: INDEX	 166)
  - J. Wilson  (also: Wilson)
  - Lachiram
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Wooding  (also: Wooding)
  - Rev Dan Wessels  (also: Dan Wessels)
  - Tregger
  - Van der Ross
  - Vassan (Hindu C.C.)  (also: Vassan)
  - Veldsman
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 67 / 139  (id=-441481017488792687)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #862

**Entities in chunk:**
  - Gava-Kadoes  (also: Gava)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Purcell
  - Ralph J Bunche  (also: Ralph Bunche)
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

## Chunk 68 / 139  (id=-272192624856094471)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #256

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 69 / 139  (id=-258351906423493019)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #172

**Entities in chunk:**
  - Abeds
  - ES Reddy
  - INDEX 166  (also: INDEX	 166)
  - Kasturba
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
  - Ruth Alexander
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, father

**CC pass (raw):**
```json
{"quote": "His wife, Kasturba"}
```

**CC quote:** `His wife, Kasturba`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Mohandas Karamchand Gandhi","relation":"spouse_of","to":"Kasturba"}]}
```

**Extracted relations:**
- `Mohandas Karamchand Gandhi` **spouse_of** `Kasturba`

---

## Chunk 70 / 139  (id=-93354464964914605)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #72

**Entities in chunk:**
  - 5th Baron
  - Alice Greene
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - George Bernard Shaw
  - INDEX 166  (also: INDEX	 166)
  - Johnny Carr
  - Lord Headley
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Prince of Wales
  - Rev. Wessels  (also: Wessels)
  - Sarojini Naidu
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 71 / 139  (id=32633455946576169)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #319

**Entities in chunk:**
  - Abba
  - Brian Willan
  - Buitencingle
  - Bunny Kriekler
  - C.Khatieb  (also: Khatieb)
  - Cader
  - Cavalla
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 72 / 139  (id=305041361240139524)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #375

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
  - Nurjahan
  - Rashid Gardee  (also: Rashid)
  - Shawquet
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf
  - Zaheer

**Triggers found:** father, mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 73 / 139  (id=519016872720201104)

**Section:** Yousuf (Joe) Rassool   98  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #589

**Entities in chunk:**
  - Hitler
  - Khadija Ebrahim  (also: Khadija)
  - Myrtle
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , brother

**CC pass (raw):**
```json
{"quote": "Khadija’s young brother"}
```

**CC quote:** `Khadija’s young brother`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the required format:

{"relations":[{"from":"Khadija Ebrahim","relation":"sibling_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Khadija Ebrahim` **sibling_of** `Yousuf Rassool`

---

## Chunk 74 / 139  (id=523998670856959750)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #46

**Entities in chunk:**
  - Ali Bey, The Terrible Turk
  - Irwin Combrinck  (also: Irwin Combrick)
  - Jim Londos
  - World Champion

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 75 / 139  (id=628953725132054043)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #605

**Entities in chunk:**
  - Dreamy
  - Messaris Junior  (also: Messaris)
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

## Chunk 76 / 139  (id=776867616384834388)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #603

**Entities in chunk:**
  - Esther Berelowitz
  - Inez Vera Du Preez
  - Mohamed Bagus Allie
  - Sheila Sachs

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 77 / 139  (id=1146286345936890093)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #462

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool
  - Appollis Slingers  (also: Appolis Slingers)
  - Arendze
  - Armin Dawood
  - Arthur Hugh Clough
  - Arthur Tracey
  - Athol Rowan
  - Babla Salloogee
  - Baboo Abed
  - Balthazzar John Vorster
  - Banoo
  - George
  - Ramamurthi
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 78 / 139  (id=1153175004995048754)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #572

**Entities in chunk:**
  - Bartnicke
  - Eddieas
  - Geoff Abrahams  (also: Geoff)
  - Kader Essack  (also: Kader)
  - Kassiem
  - Poppie
  - Ronnie Jooste  (also: Ronnie)
  - Shariefa Khan
  - Slingers  (also: Mr. Slingers)
  - Springveld

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 79 / 139  (id=1226620700780836565)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #600

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX	 166)
  - Noors

**Triggers found:** wed ,  aunt ,  uncle 

**CC pass (raw):**
```json
{"quote": "My aunt Cissie Gool"}
```

**CC quote:** `My aunt Cissie Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 80 / 139  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Ben Malamba
  - Carlson
  - Colin Wynne
  - Curly's wife
  - Cynthia Fisher
  - George
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Le Grange
  - Lennie Dixon
  - Mehuyzen
  - Oosthuisen
  - Rashid Gardee  (also: Rashid)
  - Rev. Wessels  (also: Wessels)
  - Rousseau
  - Salie Van Haacht
  - Van Der Merwe
  - Van der Westhuisen
  - Viljoen
  - Vollenhoven
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 81 / 139  (id=1340028930833342443)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #760

**Entities in chunk:**
  - Daniel Francois Malan
  - Hussein Parker
  - INDEX 166  (also: INDEX	 166)
  - Victor
  - Wage
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "Hussein Parker, son of E.A."}
```

**CC quote:** `Hussein Parker, son of E.A.`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Hussein Parker","relation":"child_of","to":"E.A."}]}
```

**Extracted relations:** none

---

## Chunk 82 / 139  (id=1515453391350648521)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #27

**Entities in chunk:**
  - George Golding
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid
  - Hassen Mall  (also: Hassen)
  - Hogwood  (also: Mr Hogwood)
  - Ismail Hayat
  - J. Wilson  (also: Wilson)
  - Jameel
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 83 / 139  (id=1743770337024172354)

**Section:** Chapter Eighteen    Chapel Street  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #965

**Entities in chunk:**
  - Danny De Beer
  - Douglas Mitchell
  - J. Wilson  (also: Wilson)
  - J.M.H  (also: J.M.H.)
  - James Currey  (also: James Curry)
  - Mohmet
  - Rev R.A. Jackson
  - Van Riebeeck
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

## Chunk 84 / 139  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Abrahams brothers  (also: Abrahams)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Jameel
  - Marcina Kagan
  - Nellie Abdurahman
  - Omar
  - Tiny Abed
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

## Chunk 85 / 139  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Dullah Omar
  - Dullie Desai
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Kies  (also: Mr Kies)
  - Morris Alexander  (also: Morris)
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

## Chunk 86 / 139  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX	 166)
  - Kies  (also: Mr Kies)
  - Slingers  (also: Mr. Slingers)
  - Tonight
  - Victor

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 87 / 139  (id=2157482743913839493)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #698

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Gava-Kadoes  (also: Gava)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hiema
  - Mohamed Ali Jinnah
  - Mohammed Essop
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Musto
  - Viceroy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 88 / 139  (id=2190457622431566710)

**Section:** Yousuf (Joe) Rassool   36  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #183

**Entities in chunk:**
  - Dutchmen
  - George
  - George Meissenheimer  (also: George Meisenheimer)
  - Hamid Khan
  - INDEX 166  (also: INDEX	 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Stephens
  - Thumbulingam
  - Van Haacht
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 89 / 139  (id=2228032443713191255)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #55

**Entities in chunk:**
  - Ben Kies
  - Dora Taylor
  - Eva Sachs
  - Fred Bodmer
  - General Smuts
  - Gregoire Boonzaaier
  - J.G.
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Paul Kostin
  - Terence Macaw
  - Tykie Messaris  (also: Tykie)
  - Wolf Kiebel
  - Young Bernard Herzberg
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 90 / 139  (id=2263436661745583446)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1024

**Entities in chunk:**
  - Hogwood  (also: Mr Hogwood)
  - INDEX 166  (also: INDEX	 166)
  - Ronald Heinrichsen
  - South African Students
  - Walcott
  - Weekes
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

## Chunk 91 / 139  (id=2310160307929513794)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #254

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Douglas Mitchell
  - Edross
  - Fazil Rassool  (also: Fazil)
  - Heneke  (also: Mr Heneke)
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

## Chunk 92 / 139  (id=2454257743793335716)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1000

**Entities in chunk:**
  - Abel
  - Demetrious Capenatakis
  - Dr. Murison
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX	 166)
  - Les Jacobs
  - Persotem Patel  (also: Persotem)
  - Vic Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 93 / 139  (id=2736700062244384633)

**Section:** Yousuf (Joe) Rassool   54  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #277

**Entities in chunk:**
  - Abass
  - Magdalena Isabella
  - Malick Rassool
  - Peter Alexander
  - Tagara
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 94 / 139  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Ismail
  - Kies  (also: Mr Kies)
  - Rasheda Rassool  (also: Rasheda)
  - Rev. Wessels  (also: Wessels)
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

## Chunk 95 / 139  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Golding
  - Harry Lawrence
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Menachem Begin
  - Regan
  - Reuben Pogrund
  - Reverend Gow  (also: Gow)
  - Van der Ross
  - Vivian Bickford-Smith

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 96 / 139  (id=3099837090720220907)

**Section:** Yousuf (Joe) Rassool   38  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #193

**Entities in chunk:**
  - Auchinlek
  - Churchill
  - Daniel Francois Malan
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX	 166)
  - Manilal Gandhi  (also: Manilal)
  - Ramdas
  - Ritchie
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
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

## Chunk 97 / 139  (id=3336594518095080251)

**Section:** Yousuf (Joe) Rassool   64  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #349

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Hoosain Ally  (also: Hussain Ally)
  - Ismail
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 98 / 139  (id=3445648479411821512)

**Section:** 103  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #628

**Entities in chunk:**
  - BM.Kies  (also: BM Kies)
  - Babla Salloogee
  - Clara Petacci
  - Germans
  - Hitler
  - Mussolini
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

## Chunk 99 / 139  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Ben Malamba
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
  - James Africa
  - Salie Van Haacht
  - Sonny Abdurahman
  - Stanley Abrahams
  - Wagieda
  - Woody Woodman
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 100 / 139  (id=3621060795652394497)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #619

**Entities in chunk:**
  - Abe Desmore
  - Azad
  - David Poole
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Johaar Mosavel  (also: Johaar Mosaval)
  - Mrs. Wo  (also: Mrs Wo)
  - Nerine Desmond
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Rehana Mohamed
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 101 / 139  (id=3654419630807047063)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1056

**Entities in chunk:**
  - Ben Kies
  - Ebrahim Ali  (also: Ebrahim)
  - Eustace Humphrey Moses
  - Hassan Bavasah  (also: Hassan)
  - INDEX 166  (also: INDEX	 166)
  - Michael Johns
  - Mr. Meltzer  (also: Meltzer)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 102 / 139  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Benito Mussolini
  - Even
  - INDEX 166  (also: INDEX	 166)
  - Nasim Rassool  (also: Nasim)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "my beautiful, cherubic little brother"}
```

**CC quote:** `my beautiful, cherubic little brother`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 103 / 139  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - J. Wilson  (also: Wilson)
  - Jadwat
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
  - Ruth Alexander
  - Seedat
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 104 / 139  (id=4201484364537076764)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #188

**Entities in chunk:**
  - Edgar Maurice
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - J.Boyce
  - Mikhail Bakhtin
  - SOL PLAATJE Brian Willan
  - Williams
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 105 / 139  (id=4713749386019632490)

**Section:** Chapter Seven    War Rages  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #485

**Entities in chunk:**
  - Abba
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Julius Caesar
  - Marcina Kagan
  - Nellie Abdurahman
  - Ou Bhai
  - Tregger
  - Trigger
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 106 / 139  (id=4888467865479780033)

**Section:** Chapter Four  White Flight  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #393

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ben Kies
  - Bunny Kriekler
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Earnest Livingston MQotsi
  - Fazil Rassool  (also: Fazil)
  - Gerald Newman
  - Harold Wolpe
  - Helen Abrahams
  - John Clayton
  - Oliver Caldecott
  - P.V. Tobias
  - Shawquet
  - Sydney Brenner
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
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Yasmin Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Yasmin Rassool`

---

## Chunk 107 / 139  (id=5228882496868360423)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1060

**Entities in chunk:**
  - Beebawbaw
  - Bloomberg
  - David Bloomberg
  - Mister Mogamat Regal
  - Regal
  - Reza Rassool  (also: Reza)
  - Roland Colman
  - Victor Wessels

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 108 / 139  (id=5499390051200912173)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #104

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Emperor Hirohito
  - INDEX 166  (also: INDEX	 166)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Prince of Wales
  - Rander Grandpa
  - Sayed Fethi
  - Sayed Hussain Ally
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

## Chunk 109 / 139  (id=5528078340237292279)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #245

**Entities in chunk:**
  - Hitler
  - Morris Alexander  (also: Morris)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 110 / 139  (id=5781509709031527554)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #866

**Entities in chunk:**
  - Arthur Tracey
  - Deanna Durbin
  - George Manuel
  - Hutch
  - Kies  (also: Mr Kies)
  - Messaris Junior  (also: Messaris)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nehru
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "My uncle"}
```

**CC quote:** `My uncle`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 111 / 139  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - D. Neethling  (also: Neethling)
  - Fataar
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Kies  (also: Mr Kies)
  - Meissenheimer  (also: Mrs Meissenheimer)
  - Mr. Jaffe  (also: Jaffe)
  - Rev. Wessels  (also: Wessels)
  - Sayed Kader Sayed Noor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 112 / 139  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Harold Kruger
  - Hewat
  - J. Wilson  (also: Wilson)
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

## Chunk 113 / 139  (id=6105164271277742615)

**Section:** 57  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #292

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Cecil John Rhodes
  - Hassen Abrahams
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 114 / 139  (id=6263823088584352511)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1019

**Entities in chunk:**
  - Gaunt
  - George
  - INDEX 166  (also: INDEX	 166)
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 115 / 139  (id=6307174216090843246)

**Section:** 91  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #546

**Entities in chunk:**
  - Eddie Roux  (also: Eddie)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Jan Van Riebeeck
  - Kassiem
  - Manuel
  - Peter Abrahams
  - Shariefa Khan

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 116 / 139  (id=6317719585632729785)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #398

**Entities in chunk:**
  - Bagus Allie  (also: Bagus-Allie)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Davids
  - Hassen Mall  (also: Hassen)
  - Ismail
  - Joan of Arc
  - Khadija Ebrahim  (also: Khadija)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Persotem Patel  (also: Persotem)
  - Rashid Gardee  (also: Rashid)
  - Shawquet
  - Suleiman Vallie
  - Tiddles
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina

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

## Chunk 117 / 139  (id=6488849859123662151)

**Section:** Yousuf (Joe) Rassool   24  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #146

**Entities in chunk:**
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
  - Kassiem
  - Kasturba
  - Katie  (also: Katie.)
  - Katzeff
  - Keith Miller
  - Ken Viljoen
  - Louis Botha
  - Victor Wessels

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and no specific name is given for the narrator, we can't determine the exact relationship. However, based on the quote "my grandfather", we know that the speaker's parent is not mentioned, but their grandparent is.

Assuming the narrator's name is one of the listed names (although it's not explicitly stated), and considering the quote mentions a grandfather, if we knew the narrator's name, we could establish a relationship. But since we don't have this information, we can only say that the speaker has a grandfather, without specifying who that grandfather is in relation to any other person.

However, according to the rules, if both endpoints are not identifiable as canonical names, return empty.

So, the answer would be:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 118 / 139  (id=6546657237567362266)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #374

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Cecil John Rhodes
  - Fazil Rassool  (also: Fazil)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, parent,  uncle 

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

## Chunk 119 / 139  (id=6614900686580700622)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #888

**Entities in chunk:**
  - Andrew Mackrill
  - Colin Wynne
  - David Poole
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - General
  - George Veldsman
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Johaar Mosavel  (also: Johaar Mosaval)
  - McDonald Dixon
  - Paul Roubaix
  - Pfaff
  - President Harry Truman
  - Smuts

**Triggers found:** wed , parent, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 120 / 139  (id=6622459745407552529)

**Section:** Yousuf (Joe) Rassool   18  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #120

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hoosain Ally  (also: Hussain Ally)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Sayed Noor Chota
  - Solly Edross

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 121 / 139  (id=7168244047913479805)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1141

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Gool AH Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 122 / 139  (id=7243012516835118461)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #407

**Entities in chunk:**
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Bagus Allie  (also: Bagus-Allie)
  - Elmer Rice
  - Heerie
  - Moosa Cape Town
  - Moosa Driver
  - O.Henry  (also: O. Henry)
  - Senator Edgar Brookes
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

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

## Chunk 123 / 139  (id=7262414809749442984)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #699

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Alan Melville
  - Athol Rowan
  - Bruce Mitchell
  - Fazil Rassool  (also: Fazil)
  - INDEX 166  (also: INDEX	 166)
  - Ken Viljoen
  - Mohamed Ali Jinnah
  - Paul Winslow
  - Viceroy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 124 / 139  (id=7266467364867027437)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #107

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Aminabhen
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - John Phillips  (also: Rev. John Phillips)
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Nasima
  - Regan
  - Reginald Lesch
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

## Chunk 125 / 139  (id=7360610042715665275)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #158

**Entities in chunk:**
  - Ally
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX	 166)
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muslims
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

## Chunk 126 / 139  (id=7448875471219160758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #480

**Entities in chunk:**
  - Cecil Wightman
  - Chaplin
  - E. Albertus  (also: Albertus)
  - Hitler
  - Mussolini
  - Nasim Rassool  (also: Nasim)
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
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 127 / 139  (id=7574535754100824916)

**Section:** Yousuf (Joe) Rassool   132  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #838

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Howard Fast
  - Hussein Parker
  - Hutch
  - Hymie Beimel  (also: Hymie)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Ignazio Silone
  - Joyce Dixon
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

## Chunk 128 / 139  (id=7584768665477924623)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #81

**Entities in chunk:**
  - Alie Fataar
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dudley Nourse  (also: Dudley)
  - Edna
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - George Meissenheimer  (also: George Meisenheimer)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Gwen Wilcox
  - Halima Gool  (also: Halima)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Joyce Dixon
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

## Chunk 129 / 139  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Ben Kies
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Great Man
  - Helen Abrahams
  - INDEX 166  (also: INDEX	 166)
  - Oswald Pirow
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

## Chunk 130 / 139  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies
  - Cyril
  - Dawood Seedat  (also: Dawood)
  - Fatima Seedat
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 131 / 139  (id=7794589649810439109)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #116

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX	 166)
  - Merwe
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** son , mother, brother,  aunt 

**CC pass (raw):**
```json
{"quote": "my Aunt Jolly"}
```

**CC quote:** `my Aunt Jolly`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 132 / 139  (id=8034240727763764335)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #79

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Jack Meltzer
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Mr. Meltzer  (also: Meltzer)
  - Victor
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, niece

**CC pass (raw):**
```json
{"quote": "his two sons, Abdul Hamid and Mohammed Hanief"}
```

**CC quote:** `his two sons, Abdul Hamid and Mohammed Hanief`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Abdul Hamid Gool","relation":"child_of","to":"Yousuf Rassool"},{"from":"Mohammed Hanief Gool","relation":"child_of","to":"Yousuf Rassool"},{"from":"Abdul Hamid Gool","relation":"sibling_of","to":"Mohammed Hanief Gool"}]}
```

**Extracted relations:**
- `Abdul Hamid Gool` **child_of** `Yousuf Rassool`
- `Mohammed Hanief Gool` **child_of** `Yousuf Rassool`
- `Abdul Hamid Gool` **sibling_of** `Mohammed Hanief Gool`

---

## Chunk 133 / 139  (id=8317664100809929487)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #911

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Dr. DuPlessis  (also: DuPlessis)
  - G.Hoosain  (also: Hoosain)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
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

## Chunk 134 / 139  (id=8675141585945090217)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #883

**Entities in chunk:**
  - Abdulla Mohamed
  - Abdulla Parker
  - Abou Desai  (also: Abou)
  - Ahmed Salasa
  - Alex Penkin
  - Ghulzar Khan  (also: Gulzar Khan)
  - Hoosain Parker
  - Menage Rassool
  - Mohamed Giri
  - Moorgas Naidoo
  - Rent
  - Tiny Abed

**Triggers found:** parent, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 135 / 139  (id=8719056895392496748)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #195

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Athol Rowan
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bruce Mitchell
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ken Viljoen
  - Khadija Ebrahim  (also: Khadija)
  - Melville
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Mrs Ahmed  (also: Ahmed)
  - Paul Winslow
  - Persotem Patel  (also: Persotem)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zobeida Gool  (also: Zobeida, Billy, Zobeida (Billy), Zobeida Billy)

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

## Chunk 136 / 139  (id=8730565953127446314)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #997

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Keraan
  - Messrs Oosterwyk
  - Mrs Ahmed  (also: Ahmed)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 137 / 139  (id=8807579643631700797)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #241

**Entities in chunk:**
  - Dija
  - Jesus Christ
  - Morris Alexander  (also: Morris)
  - Professor James  (also: Prof. James)
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

## Chunk 138 / 139  (id=8910810193593885847)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #606

**Entities in chunk:**
  - Dreamy
  - INDEX 166  (also: INDEX	 166)
  - Khan  (also: Mr. Khan)
  - Lalla
  - Professor James  (also: Prof. James)
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

## Chunk 139 / 139  (id=9200706703269745412)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #202

**Entities in chunk:**
  - A.H.
  - Ahmed Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - G.H.
  - Harold Cressy
  - INDEX 166  (also: INDEX	 166)
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
| Chunks processed | 139 |
| Relations extracted | 29 |
| Relations written to graph | 29 |
