# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 136  
**Sampled:** 136  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 136  (id=-9026931979920257841)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #209

**Entities in chunk:**
  - Abrahams  (also: Mr. Abrahams)
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
Since the narrator is 'my' and the NARRATOR name is given as "Auntie Timmie" which is another name for Fatima Gool, we can use that name as 'from'. However, the quote does not provide a canonical name for the dad. Therefore, according to the rules, we return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 2 / 136  (id=-8565879665326676003)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #890

**Entities in chunk:**
  - Achee
  - Achmed
  - Dawood Seedat  (also: Dawood)
  - Eventually Fatima  (also: Eventually)
  - Head  (also: Head Mr)
  - Meneer
  - Mr. Pearce  (also: Pearce)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband,  cousin

**CC pass (raw):**
```json
{"quote": "My cousin"}
```

**CC quote:** `My cousin`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 3 / 136  (id=-8505583857113009851)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #373

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
  - Ronnie Jooste  (also: Ronnie)

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

## Chunk 4 / 136  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs  (also: Hans)
  - Joosub Maulvi Hamid Gool: Gool  (also: Joosub)
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Sayed Kader Sayyed Noor  (also: Sayed Kader Sayed Noor)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
{"quote": "her brother’s"}
```

**CC quote:** `her brother’s`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 5 / 136  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Ebrahim
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen)
  - Natasha
  - Nikita Khruschev
  - Norman Bethune
  - Nosipho Majeke
  - Nurjahan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
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
{"relations":[{"from":"Ebrahim","relation":"spouse_of","to":"Fatima Gool"}]}
```

**Extracted relations:**
- `Ebrahim` **spouse_of** `Fatima Gool`

---

## Chunk 6 / 136  (id=-8225958241890195217)

**Section:** Yousuf (Joe) Rassool   30  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #166

**Entities in chunk:**
  - Alexander
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Heneke  (also: Mr Heneke)
  - Hewat
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Pat Naidoo
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

## Chunk 7 / 136  (id=-8186139953935417234)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #919

**Entities in chunk:**
  - Dora Taylor
  - Hassan Bavasah  (also: Hassan)
  - Isaac Pfaff
  - Kies  (also: Mr Kies)
  - Nosipho Majeke
  - Organising Secretary

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 8 / 136  (id=-8041283814668973231)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #109

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassan Bavasah  (also: Hassan)
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

## Chunk 9 / 136  (id=-7980378207581507601)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #207

**Entities in chunk:**
  - Aunty Annie
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Fazil Rassool  (also: Fazil)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
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
Since the narrator is "my" and the NARRATOR name is given as Yousuf Rassool, we can use that name as 'from'. The quote states "my dad chose Ayesha", which implies a parent-child relationship.

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Ayesha Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Ayesha Rassool`

---

## Chunk 10 / 136  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Churchill
  - Generals Alexander
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

## Chunk 11 / 136  (id=-7549255937705633040)

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
  - Reza
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

## Chunk 12 / 136  (id=-7025477089957931364)

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
  - Slingers  (also: Mr Slingers)
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

## Chunk 13 / 136  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - Albertus  (also: Mr. Albertus)
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

## Chunk 14 / 136  (id=-6782705425542896048)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #113

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 15 / 136  (id=-6710587015263367697)

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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 16 / 136  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Albertus  (also: Mr. Albertus)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Mussolini
  - Parker  (also: Mr Parker)
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

## Chunk 17 / 136  (id=-6141146778954841009)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #506

**Entities in chunk:**
  - Abdurahman
  - Adams
  - Ebrahim
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
{"quote": "my father"}
```

**CC quote:** `my father`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 18 / 136  (id=-6067802038126529915)

**Section:** Yousuf (Joe) Rassool   6  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #64

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Abbas Dinath  (also: Abbas)
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dr Jabavu  (also: Jabavu)
  - Gava Kadoes  (also: Gava)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hiema
  - J. Lucheram  (also: Lucheram)
  - M. Enus  (also: Enus)
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
Since the narrator is "Yousuf Rassool" and the quote mentions "his brother's footsteps", we can infer that Yousuf Rassool has a brother. However, the brother's name is not explicitly mentioned in the quote.

But wait! There is another person named Yusuf Hamid, which could be an alias for Yousuf Rassool (note the similar names). If so, then "his" would refer to Yusuf Hamid, and we can infer that Yusuf Hamid has a brother. 

However, without more information or context, it's impossible to determine who this brother is.

So, according to the rules, since both endpoints are not identifiable as canonical names, I must return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 19 / 136  (id=-6012376786615465402)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1022

**Entities in chunk:**
  - Cassiem  (also: Mr. Cassiem)
  - Fazil Rassool  (also: Fazil)
  - M Wilson  (also: Wilson)
  - Mrs Wooding  (also: Wooding)
  - Tregger  (also: Mr Tregger)
  - Van der Ross
  - Vassan
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

## Chunk 20 / 136  (id=-5897797065487598428)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #70

**Entities in chunk:**
  - Graduated as a Teacher
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Majesty Queen Victoria
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Tufty Mann

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 21 / 136  (id=-5853235968246498787)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #199

**Entities in chunk:**
  - Achee
  - Amina Minnie  (also: Amina)
  - Daria
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs  (also: Hans)
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
Since the narrator is "my" and the NARRATOR name is given as Yousuf Rassool, we can use that name as 'from'. The quote states "my grandfather", which means Yousuf Rassool's grandfather. However, there is no canonical name for Yousuf Rassool's grandfather in the list of known persons.

But since Haji Joosub Maulvi Hamid Gool is mentioned as a possible grandfather (given the title and age), we can make an educated guess that he might be the one referred to. 

So, assuming Haji Joosub Maulvi Hamid Gool is indeed Yousuf Rassool's grandfather:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":"Haji Joosub Maulvi Hamid Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **child_of** `Haji Joosub Maulvi Hamid Gool`

---

## Chunk 22 / 136  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
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
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. However, the quote mentions "Aunt Fatima" which is not a canonical name in the list. 

According to the rules, if the quote uses 'aunt', 'uncle', 'nephew', 'niece', or 'cousin', return {"relations":[]}.

So, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 23 / 136  (id=-5786981904019721036)

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

## Chunk 24 / 136  (id=-5714165220014992650)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #255

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jack Meltzer
  - Kies  (also: Mr Kies)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Ronnie Jooste  (also: Ronnie)
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

## Chunk 25 / 136  (id=-5610384977170982774)

**Section:** Yousuf (Joe) Rassool   160  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1029

**Entities in chunk:**
  - Alec Bedser
  - Cliff Gladwin
  - Cyril Washbrook
  - Dennis Compton
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

## Chunk 26 / 136  (id=-5556520109474695452)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #135

**Entities in chunk:**
  - Barmania MA
  - Dr Jabavu  (also: Jabavu)
  - George Golding
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mr B Allie  (also: B. Allie)
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 27 / 136  (id=-5460737940864037782)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1137

**Entities in chunk:**
  - Abdurahman
  - Adonis
  - Chota
  - Gadija Dollie
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Mr Bellingham  (also: Bellingham)
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

## Chunk 28 / 136  (id=-5418155653937249668)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1145

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Emily Hobhouse
  - Goulam Gool
  - Governor-General
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Gladstone  (also: Mrs. Gladstone)
  - Prime Minister Botha
  - Ralph J Bunche  (also: Ralph Bunche)
  - Senator Edgar Brookes
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

## Chunk 29 / 136  (id=-5364503672193663248)

**Section:** Yousuf (Joe) Rassool   16  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #119

**Entities in chunk:**
  - Alexander
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dylan
  - Eldest
  - Feyruz
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Ally  (also: Hussain Ally)
  - J. M. H. Gool
  - John Schlesinger
  - Joseph
  - Leonardo
  - Lewis
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Reza
  - Samuel
  - Sayed Noor Chota
  - Zarina

**Triggers found:** mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "Front Row: Nurjehan, Mom (Ayesha), Granny (Bibi)"}

This clause identifies a direct family relationship between three people:

* Nurjehan is the child/grandchild of Bibi (Granny) and Ayesha (Mom).
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
}]}
```

**Extracted relations:**
- `Nurjehan Gool Mohamed` **child_of** `Ayesha Rassool`

---

## Chunk 30 / 136  (id=-5340629096267782490)

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
Since the narrator is "Yousuf Rassool" and the quote mentions "her elder sister", we can infer that Yousuf Rassool has a sibling. However, the quote does not mention the name of the sister.

But since the narrator is referring to someone as "her", it implies that the narrator (Yousuf Rassool) is talking about his own family member. Therefore, the sister being referred to must be a sibling of Yousuf Rassool.

Unfortunately, we do not have enough information to identify the name of the sister. However, based on the given options and rules, we can still provide a valid JSON response:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 31 / 136  (id=-5184936258167894087)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #714

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Ghulzar Khan  (also: Gulzar Khan)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - M Wilson  (also: Wilson)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 32 / 136  (id=-5154709854984411195)

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

## Chunk 33 / 136  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head Student
  - Hoosain Parker
  - Mister Efficiency
  - Paddy Thomas
  - Salie Van Haacht
  - Stanley Abrahams
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** husband

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 34 / 136  (id=-5009073907762257871)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #100

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bhokkie
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Genuine Truth
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 35 / 136  (id=-4897146471319258283)

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

## Chunk 36 / 136  (id=-4713556574132170500)

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

## Chunk 37 / 136  (id=-4527305166854275038)

**Section:** 129  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #815

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Delarey
  - George
  - Kies  (also: Mr Kies)
  - Noors
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother,  cousin

**CC pass (raw):**
```json
{"quote": "my mother's cousins the Noors"}
```

**CC quote:** `my mother's cousins the Noors`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "my" refers to Yousuf Rassool and the quote states that Noors is his mother's cousin, we can infer that Yousuf Rassool's mother is a sibling of Noors' parent. However, this relationship type is not in the schema.

According to the rules, if the quote uses 'cousin', return {"relations":[]} 

So, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 38 / 136  (id=-4263780790926522616)

**Section:** 37  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #187

**Entities in chunk:**
  - Abdurahman
  - Henry Sylvester Williams
  - J.Boyce
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nazima Rassool  (also: Nazima, Professor Nazima Rassool, Prof. Nazima Rassool)
  - Nehru
  - Nelson Eddy
  - Nelson Mandela
  - Nerine Desmond
  - Neville Alexander
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 39 / 136  (id=-3900190985977953352)

**Section:** 25  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #150

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Isvarlal
  - Itie
  - Manilal Gandhi  (also: Manilal)
  - Manny
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Roger Streets
  - Sakarlal
  - Tyne
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 40 / 136  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Emily Hobhouse
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

## Chunk 41 / 136  (id=-3882651649594979630)

**Section:** 119  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #735

**Entities in chunk:**
  - Achee
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Mohmet
  - Wormy
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "Muddy’s eldest brother"}
```

**CC quote:** `Muddy’s eldest brother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 42 / 136  (id=-3872024114827273512)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #78

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Budenny
  - C. Rassool
  - Chaim Beimel
  - Churchill
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Comrade Bill
  - Field Marshal Smuts
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 43 / 136  (id=-3844520640191004859)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #96

**Entities in chunk:**
  - Aminabhen
  - Armin Dawood Afghan Rifles Subardar
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - Ronnie Jooste  (also: Ronnie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , mother, sister, brother, in-law

**CC pass (raw):**
```json
{"quote": "her sister-in-law, Aminabhen"}
```

**CC quote:** `her sister-in-law, Aminabhen`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "her" refers to a person not mentioned in the quote and only one endpoint is identifiable as a canonical name, we cannot establish a relationship. However, based on the given rules, I will provide an answer that follows the format.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 44 / 136  (id=-3635611545960891135)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #505

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Louis Messaris
  - Messaris Junior  (also: Messaris)
  - Tykie Messaris  (also: Tykie)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "“Tykie’s” father, Louis Messaris"}
```

**CC quote:** `“Tykie’s” father, Louis Messaris`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Louis Messaris","relation":"parent_of","to":"Tykie Messaris"}]}
```

**Extracted relations:**
- `Louis Messaris` **parent_of** `Tykie Messaris`

---

## Chunk 45 / 136  (id=-3578174338829636170)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #90

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
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

## Chunk 46 / 136  (id=-3473692056619927671)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #227

**Entities in chunk:**
  - Dada Abdullah
  - Fazil Rassool  (also: Fazil)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Muslims
  - Pretoria Gandhi
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

## Chunk 47 / 136  (id=-3325270166547380284)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #406

**Entities in chunk:**
  - Heerie
  - Ismail
  - Mr B Allie  (also: B. Allie)
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

## Chunk 48 / 136  (id=-3319062113236165871)

**Section:** Yousuf (Joe) Rassool   44  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #215

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Berina Rassool  (also: Berina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cobley
  - Edward VII
  - Fazil Rassool  (also: Fazil)
  - Hoosain Ally  (also: Hussain Ally)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - M Wilson  (also: Wilson)
  - Queen
  - Rasheda Rassool  (also: Rasheda)
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

## Chunk 49 / 136  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - C.Khatieb  (also: Khatieb)
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

## Chunk 50 / 136  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - Dr Ramamurthi  (also: Ramamurthi)
  - William Wordsworth
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

## Chunk 51 / 136  (id=-2892056374499759668)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #381

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Fazil Rassool  (also: Fazil)
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

## Chunk 52 / 136  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Charles Simons
  - Charles Starret
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - J. M. H. Gool
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

## Chunk 53 / 136  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
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

## Chunk 54 / 136  (id=-2550570213336857687)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #52

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Deanna Durbin
  - Gardener Williams
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

## Chunk 55 / 136  (id=-2266997192548340950)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #95

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Armin Dawood
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother, in-law

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

## Chunk 56 / 136  (id=-2049559656837982089)

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
Here is the extracted family relationship in the required format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Rasheda Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Rasheda Rassool`

---

## Chunk 57 / 136  (id=-2047049700032512222)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #223

**Entities in chunk:**
  - Adonis
  - Dija
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Mr. Gool
  - Noors
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

## Chunk 58 / 136  (id=-2010807931329773554)

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

## Chunk 59 / 136  (id=-1961234090315831513)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #380

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed
  - Fazil Rassool  (also: Fazil)
  - Halima Gool
  - Hassen Mall  (also: Hassen)
  - Khan  (also: Mr. Khan)
  - Selim Gool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 60 / 136  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Gool
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gokhale
  - Laura Heffer
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - President
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

## Chunk 61 / 136  (id=-1566108535838797793)

**Section:** 81  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #474

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Comrade Bill
  - Dora Taylor
  - Harry Snitcher
  - Jaffe  (also: Mr. Jaffe)
  - Kotane
  - Mnguni
  - Musto
  - Nosipho Majeke
  - Ray Alexander
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

## Chunk 62 / 136  (id=-1507950876298588482)

**Section:** 183  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1146

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Afr Gandhi
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - First Lallie
  - Goulam Gool
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

## Chunk 63 / 136  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis
  - Dr DuPlessis  (also: DuPlessis)
  - Morris
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

## Chunk 64 / 136  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Asalaamualaikum
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Bourgeois
  - C.Khatieb  (also: Khatieb)
  - Ebrahim
  - G.Abed
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Joosub Maulvi Hamid Gool: Gool  (also: Joosub)
  - Malick Hayat Captain
  - Qudrat
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
{"relations":[{"from":"Muddy","relation":"sibling_of","to":"Ebrahim"}]}
```

**Extracted relations:** none

---

## Chunk 65 / 136  (id=-1045087825811632573)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #465

**Entities in chunk:**
  - George
  - James Shirley
  - Joyce
  - Mr Edross  (also: Edross)
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

## Chunk 66 / 136  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - M Wilson  (also: Wilson)
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

## Chunk 67 / 136  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam H. G. Mahomed
  - Charles Simons
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Parker
  - J.M.H  (also: J.M.H.)
  - Molteno  (also: Mr Molteno)
  - Mr. Schreiner  (also: Schreiner)
  - Parker  (also: Mr Parker)
  - Richard Parker
  - S.Abed  (also: Abed)
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 68 / 136  (id=-441481017488792687)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #862

**Entities in chunk:**
  - Gava Kadoes  (also: Gava)
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

## Chunk 69 / 136  (id=-272192624856094471)

**Section:** Yousuf (Joe) Rassool   50  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #256

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Aboeta
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

## Chunk 70 / 136  (id=-258351906423493019)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #172

**Entities in chunk:**
  - ES Reddy
  - Kasturba
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris
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

## Chunk 71 / 136  (id=-93354464964914605)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #72

**Entities in chunk:**
  - 5th Baron
  - Alice Greene
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - George Bernard Shaw
  - Johnny Carr
  - Lord Headley
  - Miss Elizabeth Molteno  (also: Elizabeth Molteno)
  - Prince of Wales
  - Rev. Wessels  (also: Wessels)
  - Sarojini Naidu
  - TLSA
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 72 / 136  (id=32633455946576169)

**Section:** Yousuf (Joe) Rassool   60  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #319

**Entities in chunk:**
  - A. Cader  (also: Cader)
  - Abba
  - Brian Willan
  - Bunny Kriekler
  - C.Khatieb  (also: Khatieb)
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

## Chunk 73 / 136  (id=519016872720201104)

**Section:** Yousuf (Joe) Rassool   98  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #589

**Entities in chunk:**
  - Hitler
  - Khadija
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
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 74 / 136  (id=523998670856959750)

**Section:** 3  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #46

**Entities in chunk:**
  - Ali Bey, The Terrible Turk
  - Irwin Combrinck  (also: Irwin Combrick)
  - Jim Londos

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 75 / 136  (id=628953725132054043)

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

## Chunk 76 / 136  (id=776867616384834388)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #603

**Entities in chunk:**
  - Esther Berelowitz
  - Inez Vera Du Preez
  - Mohamed Bagus Allie
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

## Chunk 77 / 136  (id=819376064568638174)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #99

**Entities in chunk:**
  - Albertus  (also: Mr. Albertus)
  - Amina Minnie  (also: Amina)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hamza
  - Hoosain
  - Parker  (also: Mr Parker)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 78 / 136  (id=1153175004995048754)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #572

**Entities in chunk:**
  - Bartnicke
  - Eddieas
  - Geoff Abrahams  (also: Geoff)
  - INDEX 166  (also: INDEX	 166)
  - Kader Essack  (also: Kader)
  - Kassiem
  - Poppie
  - Ronnie Jooste  (also: Ronnie)
  - Shariefa Khan
  - Slingers  (also: Mr Slingers)
  - Springveld
  - Standard Seven
  - Standard Six

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 79 / 136  (id=1226620700780836565)

**Section:** 99  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #600

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)

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

## Chunk 80 / 136  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Ben Malamba
  - Carlson
  - Colin Wynne
  - Curly
  - Cynthia Fisher
  - George
  - Haartzen
  - Hassen Mall  (also: Hassen)
  - Lennie Dixon
  - Rashid Gardee
  - Salie Van Haacht
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 81 / 136  (id=1340028930833342443)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #760

**Entities in chunk:**
  - Daniel Francois Malan
  - E.A.
  - Head Student
  - Hussein Parker
  - INDEX 166  (also: INDEX	 166)
  - South African Indians
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
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"E.A.","relation":"parent_of","to":"Hussein Parker"}]}
```

**Extracted relations:**
- `E.A.` **parent_of** `Hussein Parker`

---

## Chunk 82 / 136  (id=1515453391350648521)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #27

**Entities in chunk:**
  - George Golding
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hamid  (also: Maulvi Hamid)
  - Hassen Mall  (also: Hassen)
  - Hogwood  (also: Mr Hogwood)
  - Ismail Hayat
  - Jameel
  - M Wilson  (also: Wilson)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 83 / 136  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
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

## Chunk 84 / 136  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Dullah Omar
  - Dullie Desai
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Morris
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

## Chunk 85 / 136  (id=2083741976242093100)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #179

**Entities in chunk:**
  - Hymie Beimel  (also: Hymie)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Phillis Ntantala Jordan
  - Reddy
  - Victor
  - W.P.  (also: W.P)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 86 / 136  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Jaffeist
  - Kies  (also: Mr Kies)
  - Victor

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 87 / 136  (id=2157482743913839493)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #698

**Entities in chunk:**
  - Gava Kadoes  (also: Gava)
  - Hiema
  - Mohamed Ali Jinnah
  - Mohammed Essop
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 88 / 136  (id=2228032443713191255)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #55

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Dora Taylor
  - Eva Sachs
  - Fred Bodmer
  - General Smuts
  - Gregoire Boonzaaier
  - J.G.  (also: J.G)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Messaris Junior  (also: Messaris)
  - Paul Kostin
  - Terence Macaw
  - Thence
  - Tykie Messaris  (also: Tykie)
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

## Chunk 89 / 136  (id=2263436661745583446)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1024

**Entities in chunk:**
  - Hogwood  (also: Mr Hogwood)
  - Ronald Heinrichsen
  - Three W
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

## Chunk 90 / 136  (id=2310160307929513794)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #254

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Douglas Mitchell
  - Fazil Rassool  (also: Fazil)
  - Heneke  (also: Mr Heneke)
  - Mohmet
  - Mr Edross  (also: Edross)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "my brother Fazil"}
```

**CC quote:** `my brother Fazil`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the required format:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 91 / 136  (id=2454257743793335716)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1000

**Entities in chunk:**
  - Abel
  - Demetrious Capenatakis
  - Fazil Rassool  (also: Fazil)
  - Les Jacobs
  - Murison
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

## Chunk 92 / 136  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Ismail
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

## Chunk 93 / 136  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Golding Van der Ross  (also: Golding)
  - Harry Lawrence
  - James Currey  (also: James Curry)
  - M Wilson  (also: Wilson)
  - Menachem Begin
  - Regan
  - Reuben Pogrund
  - Reverend Gow  (also: Rev. Gow)
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

## Chunk 94 / 136  (id=3099837090720220907)

**Section:** Yousuf (Joe) Rassool   38  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #193

**Entities in chunk:**
  - Auchinlek
  - Churchill
  - Daniel Francois Malan
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
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
{"relations":[{"from":"Wahida Gool","relation":"sibling_of","to":"Goolam Gool"}]}
```

**Extracted relations:**
- `Wahida Gool` **sibling_of** `Goolam Gool`

---

## Chunk 95 / 136  (id=3217057869876847730)

**Section:** Chapter Three  Declaration of War  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #364

**Entities in chunk:**
  - Blondie Shaik
  - Damoo Bansda
  - Farouk Du Preez
  - George Munsook
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Goolie
  - Hamid Khan
  - Hoosain Parker
  - Jackie Gihwala
  - Moorgas Naidoo
  - Mother Nature
  - Nasim Rassool  (also: Nasim)
  - Rashid Gardee
  - Salie Van Haacht
  - Shakham Osmany
  - Stephens
  - Tiny Abed
  - Van Haacht
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "my baby brother"}
```

**CC quote:** `my baby brother`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 96 / 136  (id=3336594518095080251)

**Section:** Yousuf (Joe) Rassool   64  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #349

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Hoosain Ally  (also: Hussain Ally)
  - Ismail
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 97 / 136  (id=3445648479411821512)

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

## Chunk 98 / 136  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Ben Malamba
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
  - Salie Van Haacht
  - Stanley Abrahams
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

## Chunk 99 / 136  (id=3610799175460804304)

**Section:** 101  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #617

**Entities in chunk:**
  - Beat
  - Dr Kolia  (also: Kolia)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Manilal Gandhi  (also: Manilal)
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
  - Uma Dhupelia-Mesthrie  (also: Uma Dhupelia-Mesthri)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina

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

## Chunk 100 / 136  (id=3621060795652394497)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #619

**Entities in chunk:**
  - Abe Desmore
  - Azad
  - David Poole
  - Johaar Mosaval  (also: Johaar Mosavel)
  - Nerine Desmond
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Rehana Mohamed
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

## Chunk 101 / 136  (id=3683676736687558285)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #789

**Entities in chunk:**
  - Allies
  - Generals Montgomery
  - George Golding
  - Hassiem Edross
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Rev. Gow)
  - Salie Dollie
  - Solly Edross
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

## Chunk 102 / 136  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Benito Mussolini
  - Even
  - Germans
  - Heneke  (also: Mr Heneke)
  - James Shirley
  - Mr Edross  (also: Edross)
  - Nasim Rassool  (also: Nasim)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "my beautiful, cherubic little brother now stood there"}
```

**CC quote:** `my beautiful, cherubic little brother now stood there`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the exact format requested:

{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 103 / 136  (id=3909298216041003849)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #228

**Entities in chunk:**
  - Aminabhen
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Fazil Rassool  (also: Fazil)
  - George Munsook
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hoosain Parker
  - Hudson
  - Kies  (also: Mr Kies)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Nehru
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, sister

**CC pass (raw):**
```json
{"quote": "his sister’s"}
```

**CC quote:** `his sister’s`  ✅ anchored → EC

**EC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 104 / 136  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jadwat
  - M Wilson  (also: Wilson)
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris
  - Ruth Alexander
  - Seedat
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

## Chunk 105 / 136  (id=4201484364537076764)

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

## Chunk 106 / 136  (id=4713749386019632490)

**Section:** Chapter Seven    War Rages  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #485

**Entities in chunk:**
  - Abba
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hassen Mall  (also: Hassen)
  - Julius Caesar
  - Marcina Kagan
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

## Chunk 107 / 136  (id=5228882496868360423)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1060

**Entities in chunk:**
  - David Bloomberg
  - Mister Mogamat Regal
  - Rev. Wessels  (also: Wessels)
  - Reza Rassool
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

## Chunk 108 / 136  (id=5499390051200912173)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #104

**Entities in chunk:**
  - Emperor Hirohito
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Prince of Wales
  - Sayed Fethi
  - Sayed Hussain Ally
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, husband, married, father, mother, sister, niece, nephew

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 109 / 136  (id=5528078340237292279)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #245

**Entities in chunk:**
  - Hitler
  - Morris
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 110 / 136  (id=5541857800846579304)

**Section:** 163  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1049

**Entities in chunk:**
  - Alie Fataar
  - Balthazzar John Vorster
  - C.Khatieb  (also: Khatieb)
  - E.L.Maurice
  - F.Landman
  - G.Abed
  - GL Abrahams
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - Hayat Captain
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - R.O.Dudley  (also: Mr R O Dudley)
  - Rasheda Rassool  (also: Rasheda)
  - S.Abed  (also: Abed)
  - Salie
  - Sayed Kader Sayyed Noor  (also: Sayed Kader Sayed Noor)
  - Zain Rassool  (also: Zain)

**Triggers found:** son ,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 111 / 136  (id=5781509709031527554)

**Section:** Yousuf (Joe) Rassool   136  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #866

**Entities in chunk:**
  - Arthur Tracey
  - Deanna Durbin
  - George Manuel
  - Hutch
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 112 / 136  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - D. Neethling  (also: Neethling)
  - Fataar
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Jaffe  (also: Mr. Jaffe)
  - Kies  (also: Mr Kies)
  - Mrs Meissenheimer  (also: Meissenheimer)
  - Rev. Wessels  (also: Wessels)
  - Sayed Kader Sayyed Noor  (also: Sayed Kader Sayed Noor)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 113 / 136  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Harold Kruger
  - Hewat
  - James Felix
  - John Smith
  - Joseph Bredenkamp
  - M Wilson  (also: Wilson)
  - Mahatma Gandhi Vol4 Gool
  - Ronald Heinrichsen

**Triggers found:** father,  uncle 

**CC pass (raw):**
```json
{"quote": "The bond between Dr. Abdurahman and my grandfather"}
```

**CC quote:** `The bond between Dr. Abdurahman and my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since "my" is used in the quote and no narrator name is given, we can infer that the narrator's grandfather is related to Abdurahman. However, without a specific canonical name for the narrator or their grandfather, we cannot establish a valid relationship.

Therefore, the correct response is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 114 / 136  (id=6105164271277742615)

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

## Chunk 115 / 136  (id=6263823088584352511)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1019

**Entities in chunk:**
  - Gaunt
  - George
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 116 / 136  (id=6307174216090843246)

**Section:** 91  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #546

**Entities in chunk:**
  - Eddie Roux  (also: Eddie)
  - George
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Jan Van Riebeeck
  - Kassiem
  - Manuel
  - Peter Abrahams
  - Poppie
  - Shariefa Khan

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 117 / 136  (id=6546657237567362266)

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

## Chunk 118 / 136  (id=6614900686580700622)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #888

**Entities in chunk:**
  - Abrahams  (also: Mr. Abrahams)
  - Andrew Mackrill
  - Colin Wynne
  - David Poole
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - George Veldsman
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Johaar Mosaval  (also: Johaar Mosavel)
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

## Chunk 119 / 136  (id=6622459745407552529)

**Section:** Yousuf (Joe) Rassool   18  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #120

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies  (also: Ben Kies M.A)
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

## Chunk 120 / 136  (id=6941537010882242866)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #80

**Entities in chunk:**
  - Eddieas
  - Field Marshal Tito
  - Hamid  (also: Maulvi Hamid)
  - Kader Essack  (also: Kader)
  - M Wilson  (also: Wilson)
  - Nasima
  - Poppie
  - Shariefa Khan
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

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

## Chunk 121 / 136  (id=7168244047913479805)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1141

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Gool AH Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Michael Johns
  - Ralph J Bunche  (also: Ralph Bunche)
  - Robert R. Edgar  (also: Robert R Edgar)
  - Stalin
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

## Chunk 122 / 136  (id=7243012516835118461)

**Section:** 71  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #407

**Entities in chunk:**
  - Adam Haji Gool Mahomed  (also: Adam Gool Mahomed)
  - Bagus Allie  (also: Bagus-Allie)
  - Elmer Rice
  - Heerie
  - Moosa Cape Town
  - Moosa Driver
  - O. Henry  (also: O.Henry)
  - Senator Edgar Brookes
  - Sulloo
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

## Chunk 123 / 136  (id=7262414809749442984)

**Section:** Yousuf (Joe) Rassool   114  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #699

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Alan Melville
  - Athol Rowan
  - Bruce Mitchell
  - Fazil Rassool  (also: Fazil)
  - Ken Viljoen
  - Mohamed Ali Jinnah
  - Paul Winslow
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 124 / 136  (id=7266467364867027437)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #107

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool  (also: Annie)
  - Aunt Jane Gool-Tabata Wahida
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - John Phillips  (also: Rev John Phillips)
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Nasima
  - Raza
  - Reagon
  - Reddy
  - Regan
  - Reginald Lesch
  - Rev Gordon  (also: Gordon)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** daughter, father,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt Jane Gool-Tabata"}
```

**CC quote:** `my aunt Jane Gool-Tabata`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 125 / 136  (id=7360610042715665275)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #158

**Entities in chunk:**
  - Ally
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX	 166)
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 126 / 136  (id=7448875471219160758)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #480

**Entities in chunk:**
  - Albertus  (also: Mr. Albertus)
  - Cecil Wightman
  - Chaplin
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
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 127 / 136  (id=7574535754100824916)

**Section:** Yousuf (Joe) Rassool   132  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #838

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Fabians Kies
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

## Chunk 128 / 136  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cyril Schoenraad
  - Helen Abrahams
  - Nasima
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

## Chunk 129 / 136  (id=7727744974142389137)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #884

**Entities in chunk:**
  - Albertus  (also: Mr. Albertus)
  - Parker  (also: Mr Parker)
  - S.Abed  (also: Abed)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 130 / 136  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Bernadette
  - Cyril
  - Dawood Seedat  (also: Dawood)
  - Fatima Seedat
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

## Chunk 131 / 136  (id=7794589649810439109)

**Section:** 15  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #116

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
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

## Chunk 132 / 136  (id=8034240727763764335)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #79

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies  (also: Ben Kies M.A)
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

## Chunk 133 / 136  (id=8675141585945090217)

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
  - Tiny Abed

**Triggers found:** parent, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 134 / 136  (id=8719056895392496748)

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
  - Melville
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Paul Winslow
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
{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Abdul Hamid Gool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `Abdul Hamid Gool`

---

## Chunk 135 / 136  (id=8730565953127446314)

**Section:** Yousuf (Joe) Rassool   156  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #997

**Entities in chunk:**
  - Abdurahman
  - Adams
  - INDEX 166  (also: INDEX	 166)
  - Keraan
  - Mrs Ahmed  (also: Ahmed)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 136 / 136  (id=9200706703269745412)

**Section:** Yousuf (Joe) Rassool   40  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #202

**Entities in chunk:**
  - A.H.  (also: A.H)
  - Ahmed Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - G.H.
  - Harold Cressy
  - Noors
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
| Chunks processed | 136 |
| Relations extracted | 30 |
| Relations written to graph | 30 |
