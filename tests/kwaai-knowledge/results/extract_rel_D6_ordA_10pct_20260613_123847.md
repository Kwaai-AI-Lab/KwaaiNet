# Family Relation Extraction — 10% sample

**Qualifying chunks (≥2 entities + trigger):** 87  
**Sampled:** 9  
**Model:** llama3.1:8b  
**Commit:** yes

---

## Chunk 1 / 9  (id=-9102990948384786174)

**Section:** Chapter Nineteen  All Africa Convention  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #989

**Entities in chunk:**
  - Adam Gool
  - Africa
  - Allie B.A.  (also: Allie)
  - Australia
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cape Town
  - E.S. Reddy  (also: Mr. E.S Reddy)
  - Gandhi's Indian Opinion  (also: Gandhi)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX 166)
  - Indian Opinion
  - Laura Heffer
  - Marcina Kagan
  - Natal
  - Natasha
  - National Gandhi Museum
  - Noor Bagh
  - Phoenix
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
Here is the result:

{"quote": "the circumstances surrounding my uncle Goolam Gool."}
```

**CC quote:** `the circumstances surrounding my uncle Goolam Gool.`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 2 / 9  (id=-8505583857113009851)

**Section:** 67  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #373

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Europe
  - Fazil Rassool  (also: Fazil)
  - Primary  (also: Primary)
  - Ronnie Jooste  (also: Ronnie)
  - South Africa  (also: North Africa)
  - Trafalgar High School  (also: Trafalgar High School)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "my parents decided that Abdul, Fazil and I should be done."}
```

**CC quote:** `my parents decided that Abdul, Fazil and I should be done.`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship:

{"relations":[{"from":"Abdul Rassool","relation":"sibling_of","to":"Fazil Rassool"}]}
```

**Extracted relations:**
- `Abdul Rassool` **sibling_of** `Fazil Rassool`

---

## Chunk 3 / 9  (id=-8468092369900413172)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #208

**Entities in chunk:**
  - Aunty Annie
  - Benjamin Maximilian Kies  (also: Ben Kies, B.M. Kies, BM.Kies, BMKies)
  - Chapel Street School  (also: Chapel Street School)
  - Cyril Schoenraad  (also: Cyril)
  - Feyruz Rassool  (also: Feyruz)
  - INDEX 166  (also: INDEX 166)
  - Lord Wotton
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Nasima M.S  (also: Nasima)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - Omar Khayyam
  - Peerbhai
  - Peter Alexander Rassool
  - Rehana Mohamed Annie
  - Reza Rassool  (also: Reza)
  - Teachers League Steward
  - Whybow Cloete
  - Zarina Rassool  (also: Zarina)

**Triggers found:** married, wed , daughter, father, mother,  uncle , gave birth

**CC pass (raw):**
```json
Here is the qualifying clause:

{"quote": "Annie and Uncle Aity had been married about three years earlier."}
```

**CC quote:** `Annie and Uncle Aity had been married about three years earlier.`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship:

{"relations":[{"from":"Aunty Annie", "relation":"spouse_of", "to":"Uncle Aity Mohamed Saaid Gool"}]}
```

**Extracted relations:** none

---

## Chunk 4 / 9  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Alie Fataar
  - Amina Minnie
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - Drill Hall
  - Fatima Seedat  (also: Fatima)
  - Gool-Rassool  (also: Gool)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Guy's Hospital  (also: Guys Hospital)
  - Hans Friederichs
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX 166)
  - Joosub
  - London
  - Movement
  - PTAs
  - Sayed Kader Sayed Noor
  - South Africa  (also: North Africa)
  - Teachers League of South Africa  (also: TLSA, Teachers' League of South Africa)
  - Victor Wessels
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima)

**Triggers found:** wife, son , sister, brother

**CC pass (raw):**
```json
After scanning the passage for clauses that identify direct family relationships between named individuals, I found one qualifying clause:

{"quote": "Wahida’s eldest son (B.G) and Goolam went to Guys Hospital in London."}

This clause meets the rules because it contains a relationship word ("son") and at least one proper name from the persons list (Wahida and Goolam).
```

**CC quote:** `Wahida’s eldest son (B.G) and Goolam went to Guys Hospital in London.`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship:

{"relations":[{"from":"Wahida Gool","relation":"parent_of","to":"B.G"}]}
```

**Extracted relations:** none

---

## Chunk 5 / 9  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Cape Town
  - Ebrahim
  - Fatima Seedat  (also: Fatima)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX 166)
  - NUSAS
  - Natasha
  - Nikita Khruschev
  - Non-European Unity Movement  (also: Non European Unity Movement, NEUM)
  - Noor Bagh
  - Noors  (also: Noors)
  - Norman Bethune
  - Nosipho Majeke
  - Nurjahan
  - Nurjehan Gool Mohamed  (also: Nurjehan)
  - O.Henry
  - Ob-Yennesi
  - Oddfellows Hall
  - Oliver Caldecott
  - Omar Abrahams  (also: Mr Abrahams, Omar)
  - Western Province  (also: Eastern Province)

**Triggers found:** brother, in-law

**CC pass (raw):**
```json
Here's the qualifying clause:

{"quote": "Fatima, heavy with child, and Ebrahim, her brother-in-law, our newfound friend, were also there."}
```

**CC quote:** `Fatima, heavy with child, and Ebrahim, her brother-in-law, our newfound friend, were also there.`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship:

{"relations":[{"from":"Fatima Seedat","relation":"sibling_of","to":"Ebrahim"}]}
```

**Extracted relations:**
- `Fatima Seedat` **sibling_of** `Ebrahim`

---

## Chunk 6 / 9  (id=-8167953885918041797)

**Section:** Yousuf (Joe) Rassool   70  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #396

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed
  - Cape Town
  - Chapel Street School  (also: Chapel Street School)
  - Choir
  - Halima Gool Courtesy Selim Gool
  - Hamid Khan
  - Hoosain Ally  (also: Hussain Ally)
  - Ismail
  - Ismail Kagee
  - Union Fruit Supply
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zeenith Ally  (also: Zeenith)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
After scanning the passage, I found a clause that identifies a direct family relationship between two people where at least one person from the list is named:

{"quote": "my uncle Ismail."}

This clause meets the criteria because it contains a relationship word ("uncle") and a proper name from the persons list (Ismail).
```

**CC quote:** `my uncle Ismail.`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 7 / 9  (id=-8157488290903077572)

**Section:** 55  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #288

**Entities in chunk:**
  - Barnato Board
  - Broadway Dawn Darkies
  - Caledon Square
  - Coon Club
  - Dr Goolam Gool District Six
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX 166)
  - Meneer
  - Movement
  - Mowbray
  - New Era Fellowship  (also: NEF)
  - Prof James  (also: James)
  - SOYA
  - Salie Van Haacht
  - Trafalgar High School  (also: Trafalgar High School)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, sister

**CC pass (raw):**
```json
After scanning the passage, I found a qualifying clause that identifies a direct family relationship between two people with at least one person named:

{"quote": "my sisters, when they were at Trafalgar High School begged my father to speak to Boeta Bunny about this interference with their studies"}

This clause meets the criteria because it contains a relationship word ("father") and at least one proper name from the persons list (Boeta Bunny).
```

**CC quote:** `my sisters, when they were at Trafalgar High School begged my father to speak to Boeta Bunny about this interference with their studies`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship:

{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Barnato Board"}]} 

Note: The quote mentions "my father", which refers to Yousuf Rassool, and "Boeta Bunny" is not a canonical name in the list. However, it seems that Barnato Board is being referred to as one of Yousuf's children, based on the context of the sentence.
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `Barnato Board`

---

## Chunk 8 / 9  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Churchill
  - France
  - Generals Alexander
  - INDEX 166  (also: INDEX 166)
  - Montgomery
  - Renee
  - Rommel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zarina Rassool  (also: Zarina)

**Triggers found:** father,  cousin

**CC pass (raw):**
```json
After scanning the passage, I found a clause that identifies a direct family relationship between two people where at least one person from the list is named:

{"quote": "my cousin Zarina, whom we all called Renee"}

This quote meets the criteria because it contains a relationship word ("cousin") and at least one proper name from the persons list (Zarina Rassool).
```

**CC quote:** `my cousin Zarina, whom we all called Renee`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 9 / 9  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Anti-CAD
  - Boeta
  - Chops
  - Club
  - Gootie
  - Grootjie
  - INDEX 166  (also: INDEX 166)
  - James Africa
  - National Gandhi Museum
  - Slamat
  - Sunday League
  - Woody Woodman

**Triggers found:** wed , sibling,  uncle 

**CC pass (raw):**
```json
{"quote": "his eldest son, my half-uncle, Dr. Abdul Hamid Gool"}
```

**CC quote:** `his eldest son, my half-uncle, Dr. Abdul Hamid Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[CC quote has fewer than 2 identifiable endpoints — EC skipped]
```

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 9 |
| Relations extracted | 3 |
| Relations written to graph | 2 |
