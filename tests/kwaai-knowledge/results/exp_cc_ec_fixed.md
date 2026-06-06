# Family Relation Extraction — 10% sample

**Qualifying chunks (≥2 entities + trigger):** 211  
**Sampled:** 22  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** dry-run

---

## Chunk 1 / 22  (id=-9102990948384786174)

**Section:** Chapter Nineteen  All Africa Convention  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #989

**Entities in chunk:**
  - Goolam Gool  (also: Goulam Gool, Dr Goolam Gool, Dr. Goolam, Goolam (GH))
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Hamid  (also: Maulvi Hamid)
  - Hamid Khan
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hans Friederichs  (also: Hans)
  - Hansie Kroneberg
  - Harold Cressy
  - Harold Kruger
  - Harold Wolpe
  - Harry Lawrence
  - Hartog
  - Hassan Bavasah
  - Hassen Abrahams
  - Helen Abrahams
  - Herbert
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Zobeida Gool  (also: Billy, Zobeida (Billy), Zobeida Billy, Zobeida)

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

## Chunk 2 / 22  (id=-9073283356142808755)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #477

**Entities in chunk:**
  - Bartnicke
  - Davids  (also: Mr Davids)
  - Geoff
  - Mr. Slingers  (also: Slingers)
  - Nicky Springveld  (also: Springveld)
  - Rashid Zaheer Nurjahan
  - Ronnie Jooste  (also: Ronnie)
  - Shawquet
  - Tiddles
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)
  - Yusuf

**Triggers found:** mother,  cousin

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 3 / 22  (id=-9022111028918856016)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #790

**Entities in chunk:**
  - Ben Malamba
  - Dora Taylor
  - James Africa
  - Nosipho Majeke
  - Rubaiyat Politically
  - Salie Van Haacht
  - Stanley Abrahams
  - Trade Unionists
  - Woody Woodman
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 4 / 22  (id=-8746970108171972112)

**Section:** Chapter Twenty    Split in the Movement  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1018

**Entities in chunk:**
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Cissie Gool  (also: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool)
  - Dija
  - Fazil Rassool  (also: Fazil)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Morris
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** mother, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 5 / 22  (id=-8694007978331675509)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #127

**Entities in chunk:**
  - Abdul Kadar
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Lachiram
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mr. Yusuf Hamid Gool  (also: Yusuf Hamid Gool)
  - Victor

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 6 / 22  (id=-8621339613752899196)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #943

**Entities in chunk:**
  - Abbas Dinath
  - Dr. Patel  (also: Patel)
  - E.S Reddy Former  (also: E.S. Reddy Former)
  - Goolam Gool  (also: Goulam Gool, Dr Goolam Gool, Dr. Goolam, Goolam (GH))
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - MQotsi
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mr. Messaris  (also: Messaris)
  - Ralph J Bunche  (also: Ralph Bunche)

**Triggers found:** son , sister

**CC pass (raw):**
```json
{"quote": "one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters"}
```

**CC quote:** `one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Mohandas Karamchand Gandhi","relation":"parent_of","to":"Goolam Gool’s sister’s spouse"}]}
```

**Extracted relations:** none

---

## Chunk 7 / 22  (id=-8512653961804288406)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #872

**Entities in chunk:**
  - Alie Fataar
  - Balthazzar John Vorster
  - Cosmo Pieterse
  - E.L.Maurice
  - F.Landman
  - Geoffrey Abrahams  (also: Geoff Abrahams)
  - Irwin Combrinck  (also: Irwin Combrick, Irwin)
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Victor Wessels

**Triggers found:** son ,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 8 / 22  (id=-8435946538044972915)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #87

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Lord Roberts

**Triggers found:** wife, husband, married, father, mother, sister, niece, nephew

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is 'my' and the quote mentions "my grandfather", we can infer that the narrator's name is not explicitly given, but it refers to one of the known persons. However, since the quote does not specify which person's grandfather is being referred to, we cannot establish a valid relationship.

Therefore, according to the critical rules, we return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 9 / 22  (id=-8167953885918041797)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #396

**Entities in chunk:**
  - Abba (Louis Messaris)  (also: Abba)
  - Churchill Smuts  (also: Churchill)
  - Cissie Gool  (also: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool)
  - Field Marshal Smuts
  - George
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Harry Snitcher
  - Moses Kotane
  - Musto
  - Ray Alexander  (also: Alexander)
  - Smuts

**Triggers found:** wed ,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 10 / 22  (id=-8054118982771437578)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #768

**Entities in chunk:**
  - Abba (Louis Messaris)  (also: Abba)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Hymie Beimel  (also: Hymie)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Phillis Ntantala Jordan
  - Victor
  - Willem Van Schoor  (also: W.P. Van Schoor, W.P Van Schoor)
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed ,  aunt 

**CC pass (raw):**
```json
{"quote": "my aunt Jane Gool"}
```

**CC quote:** `my aunt Jane Gool`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 11 / 22  (id=-8052727503191672567)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #23

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Field Marshal Tito
  - General Smuts
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Hassen Mall  (also: Hassen)
  - Rebecca Rassool
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "My father, Yousuf Rassool"}
```

**CC quote:** `My father, Yousuf Rassool`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Rebecca Rassool","relation":"child_of","to":"Yousuf Rassool"}]}
```

**Extracted relations:**
- `Rebecca Rassool` **child_of** `Yousuf Rassool`

---

## Chunk 12 / 22  (id=-7978177967823103861)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #128

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 13 / 22  (id=-7970562435578500859)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #103

**Entities in chunk:**
  - Appollis Slingers  (also: Appolis Slingers)
  - Ben Kies  (also: Ben)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Chaganlal Gandhi
  - Ebrahim
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Hassen Mall  (also: Hassen)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Solly Mohammed
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 14 / 22  (id=-7923858109226632030)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #212

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Ismail
  - Katie
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 15 / 22  (id=-7894516247149505240)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #737

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Cissie Gool  (also: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool)
  - General Smuts
  - Ghulzar Khan  (also: Gulzar Khan)
  - Hoosain Parker  (also: Mr. Hoosain Parker)
  - Moorgas Naidoo
  - Tiny Abed
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed , parent, brother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 16 / 22  (id=-7842497683424219526)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #421

**Entities in chunk:**
  - D. D. T. Jabavu
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - Dr. Jamieson  (also: Jamieson)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Louis Messaris
  - Messaris Junior  (also: Mr. Messaris Junior)
  - Tykie Messaris  (also: Tykie)
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

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

## Chunk 17 / 22  (id=-7806481715678176623)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #414

**Entities in chunk:**
  - Appollis Slingers  (also: Appolis Slingers)
  - Field Marshal Jan C
  - Heyl  (also: Mr Heyl)
  - Mr. Slingers  (also: Slingers)
  - W.A.
  - William Alexander
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 18 / 22  (id=-7733792739382952186)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #387

**Entities in chunk:**
  - George
  - Rebecca Rassool
  - W.P.
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** son , mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 19 / 22  (id=-7701648031245183584)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #555

**Entities in chunk:**
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - Miss Thwaites  (also: Thwaites)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mr. Bellingham  (also: Bellingham)
  - Mr. Borchers  (also: Borchers, Mrs. Borchers, Mrs Borchers)
  - Mrs McDonald
  - Plaatje
  - Rita Olivier
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 20 / 22  (id=-7610407554188220939)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #777

**Entities in chunk:**
  - George
  - Hassen Abrahams
  - Head

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 21 / 22  (id=-7584328222459114241)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #734

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Achmat Clarke
  - Farouk Du Preez
  - George
  - Ghulzar Khan  (also: Gulzar Khan)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Ismail Hayat
  - Ismail Sh
  - Jameel
  - Joyce Meissenheimer  (also: Joyce Meisenheimer)
  - Moorgas Naidoo
  - Omar
  - Omar Abrahams  (also: Mr. Abrahams)
  - Tiny Abed
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** brother

**CC pass (raw):**
```json
{"quote": "the Abrahams brothers, Jameel - a wicket-keeper/ batsman - and Omar"}
```

**CC quote:** `the Abrahams brothers, Jameel - a wicket-keeper/ batsman - and Omar`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Omar Abrahams","relation":"sibling_of","to":"Jameel"}]}
```

**Extracted relations:**
- `Omar Abrahams` **sibling_of** `Jameel`

---

## Chunk 22 / 22  (id=-7487248401202293606)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #111

**Entities in chunk:**
  - Beria
  - Dr. Malan
  - European
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Sir Thomas Muir
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed , father

**CC pass (raw):**
```json
{"quote": "my grandfather"}
```

**CC quote:** `my grandfather`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "my" and the NARRATOR name is given as Yousuf Rassool, we can use that name as 'from'. The quote states "my grandfather", which implies a parent-of relationship.

However, since the schema only allows for child_of, parent_of, sibling_of, half_sibling_of, and spouse_of relationships, and the quote does not mention any of these relationships directly, but rather an ancestor-descendant relationship (grandfather), we cannot extract a valid relation using the given schema.
```

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 22 |
| Relations extracted | 3 |
| Relations written to graph | 0 |
