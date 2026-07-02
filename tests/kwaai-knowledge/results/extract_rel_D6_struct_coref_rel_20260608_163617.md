# Family Relation Extraction — 100% sample

**Qualifying chunks (≥2 entities + trigger):** 124  
**Sampled:** 124  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** yes

---

## Chunk 1 / 124  (id=-9026931979920257841)

**Section:** Yousuf (Joe) Rassool   42  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #209

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
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
{"quote":"none","relations":[]}
```

**Extracted relations:** none

---

## Chunk 2 / 124  (id=-8565879665326676003)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #890

**Entities in chunk:**
  - Achee
  - Achmed
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head
  - Indian Congress Eventually Fatima
  - Meneer
  - Mr. Pearce  (also: Pearce)
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

## Chunk 3 / 124  (id=-8274761722709443063)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #198

**Entities in chunk:**
  - Alie Fataar
  - Amina Minnie  (also: Amina)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cosmo Pieterse
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hans Friederichs
  - INDEX 166  (also: INDEX	 166)
  - Joosub
  - Mariam Gool  (also: Margie, Mariam (Margie), Mariam Margie)
  - Sayed Kader Sayed Noor
  - Victor Wessels
  - Wahida's B.G

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

## Chunk 4 / 124  (id=-8243826696501819600)

**Section:** Yousuf (Joe) Rassool   142  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #905

**Entities in chunk:**
  - Achee
  - Ebrahim
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Fatima Seedat
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Natasha
  - Nikita Khruschev
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
{"relations":[{"from":"Ebrahim","relation":"spouse_of","to":"Fatima Gool"}]}
```

**Extracted relations:**
- `Ebrahim` **spouse_of** `Fatima Gool`

---

## Chunk 5 / 124  (id=-8186139953935417234)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #919

**Entities in chunk:**
  - Dora Taylor
  - Hassan Bavasah  (also: Hassan)
  - Isaac Pfaff
  - Kies  (also: Mr Kies)
  - Nosipho Majeke
  - Organising Secretary
  - Rasheda Rassool  (also: Rasheda)
  - Zain Rassool  (also: Zain)

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 6 / 124  (id=-8041283814668973231)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #109

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
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

## Chunk 7 / 124  (id=-7980378207581507601)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #207

**Entities in chunk:**
  - Advocate Christopher
  - Aunty Annie
  - Ayesha Lallie
  - Fazil Rassool  (also: Fazil)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - J. M. H. Gool
  - Mohamed Saaid Gool  (also: Uncle Aity Mohamed Saaid Gool, Aity (Mohamed Saaid Gool), Uncle Aity, Aity)
  - Peerbhai
  - Peter Alexander Rassool
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** married, father, mother,  uncle 

**CC pass (raw):**
```json
{"quote": "Uncle Aity (Mohamed Saaid Gool)"}

This quote identifies a direct family relationship between two people: Mohamed Saaid Gool is the uncle of the narrator, Yousuf Rassool.
```

**CC quote:** `Uncle Aity (Mohamed Saaid Gool)`  ✅ anchored → EC

**EC pass (raw):**
```json
[non-schema relation — EC skipped]
```

**Extracted relations:** none

---

## Chunk 8 / 124  (id=-7568891642123966893)

**Section:** Yousuf (Joe) Rassool   74  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #426

**Entities in chunk:**
  - Boy Uncle Mamoo
  - Churchill
  - Generals Alexander
  - INDEX 166  (also: INDEX	 166)
  - Montgomery
  - Renee
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

## Chunk 9 / 124  (id=-7339891663975412181)

**Section:** Yousuf (Joe) Rassool   4  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #49

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Boeta
  - Chops
  - Dr. Abdul Hamid Gool  (also: Dr Abdul Hamid Gool)
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

## Chunk 10 / 124  (id=-7025477089957931364)

**Section:** 95  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #573

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Charles Simons
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Messaris Junior  (also: Messaris)
  - Richard Parker
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

## Chunk 11 / 124  (id=-6911800068195269706)

**Section:** 135  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #858

**Entities in chunk:**
  - E. Albertus  (also: Albertus)
  - Hymie Beimel  (also: Hymie)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 12 / 124  (id=-6666185729482373539)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #913

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Councillor Parker
  - E. Albertus  (also: Albertus)
  - Hoosain
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX	 166)
  - Mr Edross  (also: Dr. Edross)
  - Mr. Parker  (also: Parker)
  - Mussolini
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
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Yousuf Rassool","relation":"parent_of","to":"Hoosein"}]}
```

**Extracted relations:** none

---

## Chunk 13 / 124  (id=-6141146778954841009)

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
{"quote": "my dad"}
```

**CC quote:** `my dad`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 14 / 124  (id=-6067802038126529915)

**Section:** Yousuf (Joe) Rassool   6  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #64

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dr Jabavu
  - Gava-Kadoes  (also: Gava)
  - Hiema
  - J. Lucheram  (also: J Lucheram)
  - M. Enus  (also: M Enus)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)
  - Yusuf Hamid

**Triggers found:** wife, brother,  aunt 

**CC pass (raw):**
```json
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 15 / 124  (id=-6012376786615465402)

**Section:** 159  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1022

**Entities in chunk:**
  - Cassiem  (also: Mr. Cassiem)
  - Fazil Rassool  (also: Fazil)
  - J. Wilson  (also: Wilson)
  - Mrs Wooding  (also: Wooding)
  - Tregger
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

## Chunk 16 / 124  (id=-5897797065487598428)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #70

**Entities in chunk:**
  - Graduated as a Teacher Life
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

## Chunk 17 / 124  (id=-5841396144210939468)

**Section:** Yousuf (Joe) Rassool   34  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #175

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
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
Since the narrator is 'my' and the NARRATOR name is given as "Yousuf Rassool", we can use that name as 'from'. However, the quote mentions an aunt, which is not in the schema. Therefore:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 18 / 124  (id=-5714165220014992650)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #255

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Anti-CAD  (also: Anti CAD)
  - Fazil Rassool  (also: Fazil)
  - Gray
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jack Meltzer
  - Kies  (also: Mr Kies)
  - Milton
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

## Chunk 19 / 124  (id=-5610384977170982774)

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

## Chunk 20 / 124  (id=-5460737940864037782)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1137

**Entities in chunk:**
  - Abdurahman
  - Adonis
  - Bellingham  (also: Mr Bellingham)
  - Chota
  - Gadija Dollie
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 21 / 124  (id=-5418155653937249668)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1145

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Emily Hobhouse
  - Goulam Gool
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Mrs Gladstone  (also: Mrs. Gladstone)
  - Prime Minister Botha
  - Ralph J Bunche  (also: Ralph Bunche)
  - Vic Wessels
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife, sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 22 / 124  (id=-5364503672193663248)

**Section:** Yousuf (Joe) Rassool   16  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #119

**Entities in chunk:**
  - Alexander
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Dylan
  - Eldest
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
  - Zarina Rassool  (also: Zarina)

**Triggers found:** mother, brother,  uncle 

**CC pass (raw):**
```json
{"quote": "Mom (Ayesha)"}

This quote identifies a direct family relationship between two people, specifically that Ayesha is the mother of the narrator.
```

**CC quote:** `Mom (Ayesha)`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Ayesha Rassool","relation":"parent_of","to":"Unknown"}]}
```

**Extracted relations:** none

---

## Chunk 23 / 124  (id=-5340629096267782490)

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

## Chunk 24 / 124  (id=-5184936258167894087)

**Section:** Yousuf (Joe) Rassool   116  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #714

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Ghulzar Khan  (also: Gulzar Khan)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - India Ghulzar Khan
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

## Chunk 25 / 124  (id=-5154709854984411195)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #761

**Entities in chunk:**
  - Aesop
  - Bedser
  - Ben Cloete
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Compton
  - Doug Wright
  - E.A. Parker  (also: Mr. E.A. Parker)
  - Edrich
  - Faried Rossier
  - Gladwin
  - Hassan Bavasah  (also: Hassan)
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

**Extracted relations:** none

---

## Chunk 26 / 124  (id=-5012652545998377803)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #882

**Entities in chunk:**
  - Ben Malamba
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - Head Student
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
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 27 / 124  (id=-5010050899247603262)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1138

**Entities in chunk:**
  - Abdurahman
  - Alan Gregor Cobley
  - Edgar Maurice
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
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

## Chunk 28 / 124  (id=-5009073907762257871)

**Section:** 13  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #100

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 29 / 124  (id=-4897146471319258283)

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

## Chunk 30 / 124  (id=-4713556574132170500)

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

## Chunk 31 / 124  (id=-4527305166854275038)

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
Since the narrator is "my" and the NARRATOR name is given as Yousuf Rassool, we can use that name as 'from'. The quote states that Noors are the mother's cousins of Yousuf Rassool. However, since the relation type "cousin" is not in the schema, according to the rules, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 32 / 124  (id=-4183482280000304421)

**Section:** xi  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #31

**Entities in chunk:**
  - A.H.
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
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

## Chunk 33 / 124  (id=-3900190985977953352)

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

## Chunk 34 / 124  (id=-3887986038321915414)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1144

**Entities in chunk:**
  - Emily Hobhouse
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 35 / 124  (id=-3882651649594979630)

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

## Chunk 36 / 124  (id=-3872024114827273512)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #78

**Entities in chunk:**
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

## Chunk 37 / 124  (id=-3635611545960891135)

**Section:** Yousuf (Joe) Rassool   86  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #505

**Entities in chunk:**
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Ironically Tykie
  - Louis Messaris
  - Messaris Junior  (also: Messaris)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father, mother

**CC pass (raw):**
```json
{"quote": "Ironically, “Tykie’s” father, Louis Messaris"}
```

**CC quote:** `Ironically, “Tykie’s” father, Louis Messaris`  ✅ anchored → EC

**EC pass (raw):**
```json
Here is the extracted family relationship in the requested format:

{"relations":[{"from":"Ironically Tykie","relation":"child_of","to":"Louis Messaris"}]}
```

**Extracted relations:**
- `Ironically Tykie` **child_of** `Louis Messaris`

---

## Chunk 38 / 124  (id=-3578174338829636170)

**Section:** 11  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #90

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 39 / 124  (id=-3473692056619927671)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #227

**Entities in chunk:**
  - Dada Abdullah
  - Fazil Rassool  (also: Fazil)
  - Ismail Davids
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
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

## Chunk 40 / 124  (id=-3325270166547380284)

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

## Chunk 41 / 124  (id=-3319062113236165871)

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
  - Hussain Ally Finally
  - J. Wilson  (also: Wilson)
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
{"relations":[{"from":"Ayesha Rassool","relation":"parent_of","to":"Abdul Hamid Gool"}]}
```

**Extracted relations:**
- `Ayesha Rassool` **parent_of** `Abdul Hamid Gool`

---

## Chunk 42 / 124  (id=-3272765215290529944)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #515

**Entities in chunk:**
  - C.Khatieb  (also: Khatieb)
  - Hayat Captain
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - INDEX 166  (also: INDEX	 166)
  - Nurjahan
  - Rasheda Rassool  (also: Rasheda)
  - Rashid Gardee  (also: Rashid)
  - S.Abed
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

## Chunk 43 / 124  (id=-2921256515978048530)

**Section:** 87  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #517

**Entities in chunk:**
  - Ben Cloete
  - Dr. Ramamurthi  (also: Ramamurthi)
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

## Chunk 44 / 124  (id=-2892056374499759668)

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

## Chunk 45 / 124  (id=-2848012846626486631)

**Section:** 111  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #685

**Entities in chunk:**
  - Charles Simons
  - Charles Starret
  - E.S. Reddy  (also: Mr. E.S. Reddy)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - INDEX 166  (also: INDEX	 166)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Pat Holmes
  - Richard Parker
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , father

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 46 / 124  (id=-2766571805421454851)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #82

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Diamond
  - Five
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hassen Mall  (also: Hassen)
  - J.M.H.  (also: J.M.H)
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

## Chunk 47 / 124  (id=-2550570213336857687)

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

## Chunk 48 / 124  (id=-2049559656837982089)

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

## Chunk 49 / 124  (id=-2047049700032512222)

**Section:** Yousuf (Joe) Rassool   46  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #223

**Entities in chunk:**
  - Adonis
  - Dija
  - Hamid Midi
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Mr. Gool
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

## Chunk 50 / 124  (id=-2010807931329773554)

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

## Chunk 51 / 124  (id=-1961234090315831513)

**Section:** Yousuf (Joe) Rassool   68  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #380

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Baboo Abed  (also: Baboo)
  - Fazil Rassool  (also: Fazil)
  - Halima Gool
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 52 / 124  (id=-1804180665392634971)

**Section:** 75  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #430

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Adam Gool
  - Adam Gool Mahomed  (also: Adam Haji Gool Mahomed)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
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

## Chunk 53 / 124  (id=-1566108535838797793)

**Section:** 81  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #474

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Comrade Bill
  - Dora Taylor
  - Harry Snitcher
  - Jaffe  (also: Mr Jaffe)
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

## Chunk 54 / 124  (id=-1507950876298588482)

**Section:** 183  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1146

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Afr Gandhi
  - Ben Kies  (also: Ben Kies M.A)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Henry Sylvester Williams
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

## Chunk 55 / 124  (id=-1397014855803302427)

**Section:** Chapter Nine    A World Divided  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #240

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
  - Demetrious Capenatakis
  - Dr DuPlessis  (also: DuPlessis)
  - Morris Alexander  (also: Morris)
  - Mr Edross  (also: Dr. Edross)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 56 / 124  (id=-1298385936741052025)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #797

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - C.Khatieb  (also: Khatieb)
  - Dear Mother
  - Ebrahim
  - G.Abed  (also: Abed)
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - I.Begg  (also: Begg)
  - I.T.Gihwala
  - Joosub
  - Malick Hayat Captain
  - S.Abed
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

## Chunk 57 / 124  (id=-1045087825811632573)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #465

**Entities in chunk:**
  - George
  - James Shirley
  - Joyce
  - Mr Edross  (also: Dr. Edross)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wife

**CC pass (raw):**
```json
{"quote": "his wife Joyce"}
```

**CC quote:** `his wife Joyce`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[{"from":"Yousuf Rassool","relation":"spouse_of","to":"Joyce"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **spouse_of** `Joyce`

---

## Chunk 58 / 124  (id=-1027624999817858838)

**Section:** 35  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #177

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
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

## Chunk 59 / 124  (id=-600672021279123300)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #885

**Entities in chunk:**
  - Adam H. G. Mahomed
  - Charles Simons
  - G.Abed  (also: Abed)
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Hoosain Parker
  - J.M.H.  (also: J.M.H)
  - Molteno
  - Mr. Parker  (also: Parker)
  - Mr. Schreiner  (also: Schreiner)
  - Richard Parker
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

## Chunk 60 / 124  (id=-441481017488792687)

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

## Chunk 61 / 124  (id=-272192624856094471)

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

## Chunk 62 / 124  (id=-258351906423493019)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #172

**Entities in chunk:**
  - ES Reddy
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

## Chunk 63 / 124  (id=-93354464964914605)

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
  - Rev Wessels  (also: Wessels)
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

## Chunk 64 / 124  (id=519016872720201104)

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

## Chunk 65 / 124  (id=776867616384834388)

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

## Chunk 66 / 124  (id=1146286345936890093)

**Section:** Yousuf (Joe) Rassool   80  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #462

**Entities in chunk:**
  - A. Arendze  (also: Arendze)
  - Amien Basier  (also: Amien)
  - Amina Minnie  (also: Amina)
  - Aminabhen
  - Amod Gool
  - Andrew Mackrill
  - Annalise
  - Annie Rassool  (also: Annie)
  - Appollis Slingers  (also: Appolis Slingers)
  - Armin Dawood
  - Arthur Hugh Clough
  - Arthur Tracey
  - Athol Rowan
  - Babla Salloogee
  - Baboo Abed  (also: Baboo)
  - Banoo Capt  (also: Banoo)
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

## Chunk 67 / 124  (id=1153175004995048754)

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

## Chunk 68 / 124  (id=1316112705452633850)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #948

**Entities in chunk:**
  - Ben Malamba
  - Carlson
  - Colin Wynne
  - Curly
  - Cynthia Fisher
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
  - Lennie Dixon
  - Rashid Gardee  (also: Rashid)
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

## Chunk 69 / 124  (id=1340028930833342443)

**Section:** Yousuf (Joe) Rassool   122  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #760

**Entities in chunk:**
  - Daniel Francois Malan
  - Head Student
  - Hussein Parker
  - INDEX 166  (also: INDEX	 166)
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

{"relations":[{"from":"Hussein Parker","relation":"child_of","to":"E.A."}]}
```

**Extracted relations:** none

---

## Chunk 70 / 124  (id=1515453391350648521)

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
  - Transvaalers

**Triggers found:** son 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 71 / 124  (id=1821211182596669987)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #879

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Abrahams brothers  (also: Abrahams)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - INDEX 166  (also: INDEX	 166)
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

## Chunk 72 / 124  (id=1918318182968961049)

**Section:** Yousuf (Joe) Rassool   152  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #972

**Entities in chunk:**
  - Achmat Da Costa
  - Dullah Omar
  - Dullie Desai
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

## Chunk 73 / 124  (id=2083741976242093100)

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

## Chunk 74 / 124  (id=2104188275539776644)

**Section:** 41  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #206

**Entities in chunk:**
  - Cassiem  (also: Mr. Cassiem)
  - Ciraj Rassool
  - Ebrahim
  - Fazil Rassool  (also: Fazil)
  - MQotsi
  - Malick Rassool
  - Messaris Junior  (also: Messaris)
  - Morris Alexander  (also: Morris)
  - Moses Kotane
  - Motala
  - Walcott
  - Weekes
  - Worrel
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** married, father, mother

**CC pass (raw):**
```json
{"quote": "My father was handsome, bronzed like his mother"}
```

**CC quote:** `My father was handsome, bronzed like his mother`  ✅ anchored → EC

**EC pass (raw):**
```json
Since the narrator is "Yousuf Rassool" and the quote uses 'my', we can infer that Yousuf Rassool is referring to himself. The quote states "My father was handsome...". Therefore, we can extract the family relationship as follows:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":null}]}

However, since the parent's name is not explicitly mentioned in the quote, we cannot determine the exact canonical name of the parent. According to the rules, if both endpoints are not identifiable as canonical names, we should return an empty list.

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 75 / 124  (id=2132705299650216504)

**Section:** Yousuf (Joe) Rassool   12  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #98

**Entities in chunk:**
  - Arriehai
  - Beimel
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - Kies  (also: Mr Kies)
  - Slingers  (also: Mr Slingers)
  - Victor

**Triggers found:** wed 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 76 / 124  (id=2157482743913839493)

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
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:**  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 77 / 124  (id=2228032443713191255)

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

## Chunk 78 / 124  (id=2263436661745583446)

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

## Chunk 79 / 124  (id=2310160307929513794)

**Section:** 49  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #254

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Douglas Mitchell
  - Fazil Rassool  (also: Fazil)
  - Heneke  (also: Mr Heneke)
  - Mohmet
  - Mr Edross  (also: Dr. Edross)
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

## Chunk 80 / 124  (id=2736700062244384633)

**Section:** Yousuf (Joe) Rassool   54  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #277

**Entities in chunk:**
  - Abass
  - Magdalena Isabella
  - Malick Rassool
  - Noors
  - Peter Alexander Although
  - Tagara
  - Victor
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** mother,  cousin,  uncle 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 81 / 124  (id=2804243781247779264)

**Section:** 127  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #806

**Entities in chunk:**
  - Bill Cody
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Fazil Rassool  (also: Fazil)
  - Ismail
  - Rasheda Rassool  (also: Rasheda)
  - Rev Wessels  (also: Wessels)
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

## Chunk 82 / 124  (id=2848871180607819699)

**Section:** 181  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1134

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Benjamin Pogrund
  - Golding Van  (also: Golding)
  - Harry Lawrence
  - J. Wilson  (also: Wilson)
  - James Currey  (also: James Curry)
  - Menachem Begin
  - Mr. Maron  (also: Maron)
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

## Chunk 83 / 124  (id=2945565054281625486)

**Section:** Yousuf (Joe) Rassool   14  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #108

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Enver Hassim
  - Errol Vawda
  - Glories
  - Honono
  - James Shirley
  - Kader Essack  (also: Kader)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Sihlali
  - Wahida Gool  (also: Wahida Ta'al, Wahida, Hajima, Haji Wahida)

**Triggers found:** wife, husband, father, parent,  cousin,  aunt 

**CC pass (raw):**
```json
{"quote": "her husband had all the time, unknown to her, a wife in India, who was now arriving in Cape Town with two children"}
```

**CC quote:** `her husband had all the time, unknown to her, a wife in India, who was now arriving in Cape Town with two children`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 84 / 124  (id=3099837090720220907)

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
{"quote":"none","relations":[]}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 85 / 124  (id=3217057869876847730)

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
  - Rashid Gardee  (also: Rashid)
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

## Chunk 86 / 124  (id=3336594518095080251)

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

## Chunk 87 / 124  (id=3603428895150736308)

**Section:** 149  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #949

**Entities in chunk:**
  - Ben Malamba
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Haartzen
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

## Chunk 88 / 124  (id=3621060795652394497)

**Section:** Yousuf (Joe) Rassool   102  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #619

**Entities in chunk:**
  - Abe Desmore
  - Azad
  - David Poole
  - Johhaar Mosaval  (also: Johaar Mosaval)
  - Mrs. Wo  (also: Mrs Wo)
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

## Chunk 89 / 124  (id=3683676736687558285)

**Section:** Yousuf (Joe) Rassool   126  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #789

**Entities in chunk:**
  - George Golding
  - Hassiem Edross
  - INDEX 166  (also: INDEX	 166)
  - Omar Bradley
  - Patten
  - Reverend Gow  (also: Gow)
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
Since the narrator is "Yousuf Rassool" and the quote states a relationship with Solly Edross, we can infer that Yousuf Rassool is related to Solly Edross. However, the type of relation (e.g., sibling_of, parent_of, etc.) is not specified in the quote.

Given the rules, since the exact nature of the relationship is not stated, and "related" does not fit into any of the allowed categories (spouse_of, parent_of, child_of, sibling_of, half_sibling_of), we cannot provide a specific relation type. Therefore:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 90 / 124  (id=3709193321337921603)

**Section:** Yousuf (Joe) Rassool   82  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #482

**Entities in chunk:**
  - Benito Mussolini
  - Even
  - Germans
  - Heneke  (also: Mr Heneke)
  - James Shirley
  - Mr Edross  (also: Dr. Edross)
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
{"relations":[{"from":"Yousuf Rassool","relation":"sibling_of","to":"Nasim Rassool"}]}
```

**Extracted relations:**
- `Yousuf Rassool` **sibling_of** `Nasim Rassool`

---

## Chunk 91 / 124  (id=3973039377211472545)

**Section:** 33  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #173

**Entities in chunk:**
  - Abdurahman
  - Ayesha Rassool  (also: Lallie, Ayesha, Lallie Gool, Ayesha Gool)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - J. Wilson  (also: Wilson)
  - Jadwat
  - Manilal Gandhi  (also: Manilal)
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Morris Alexander  (also: Morris)
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

## Chunk 92 / 124  (id=4201484364537076764)

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

## Chunk 93 / 124  (id=4713749386019632490)

**Section:** Chapter Seven    War Rages  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #485

**Entities in chunk:**
  - Abba
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Hassen Mall  (also: Hassen)
  - Julius Caesar
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

## Chunk 94 / 124  (id=4888467865479780033)

**Section:** Chapter Four  White Flight  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #393

**Entities in chunk:**
  - Abdul Rassool  (also: Abdul, Doolie, Doolie Rassool)
  - Ben Kies  (also: Ben Kies M.A)
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

## Chunk 95 / 124  (id=5228882496868360423)

**Section:** Yousuf (Joe) Rassool   164  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1060

**Entities in chunk:**
  - Beebawbaw
  - David Bloomberg
  - Mister Mogamat Regal
  - Rev Wessels  (also: Wessels)
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

## Chunk 96 / 124  (id=5499390051200912173)

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

## Chunk 97 / 124  (id=5528078340237292279)

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

## Chunk 98 / 124  (id=5781509709031527554)

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

## Chunk 99 / 124  (id=6037235218748930663)

**Section:** Yousuf (Joe) Rassool   100  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #610

**Entities in chunk:**
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - D. Neethling  (also: Neethling)
  - Fataar
  - Hassen Mall  (also: Hassen)
  - I.B. Tabata  (also: Tabata, I. Tabata, Mr. Tabata, Dr. Tabata)
  - INDEX 166  (also: INDEX	 166)
  - Jaffe  (also: Mr Jaffe)
  - Kies  (also: Mr Kies)
  - Meissenheimer  (also: Mrs Meissenheimer)
  - Rev Wessels  (also: Wessels)
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

## Chunk 100 / 124  (id=6059861131463635211)

**Section:** Yousuf (Joe) Rassool   22  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #137

**Entities in chunk:**
  - Abdurahman
  - Aminabhen
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Harold Kruger
  - Hewat
  - INDEX 166  (also: INDEX	 166)
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
Since "my" is used in the quote and no narrator name is given, we can infer that the narrator's grandfather is being referred to. However, there is no canonical name for the narrator or their grandfather among the known persons.

But since Abdurahman is mentioned as having a bond with the narrator's grandfather, we can establish a relationship between them.

{"relations":[{"from":"Abdurahman","relation":"child_of","to":"INDEX 166"}]}
```

**Extracted relations:**
- `Abdurahman` **child_of** `INDEX 166`

---

## Chunk 101 / 124  (id=6105164271277742615)

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

## Chunk 102 / 124  (id=6488849859123662151)

**Section:** Yousuf (Joe) Rassool   24  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #146

**Entities in chunk:**
  - Fazil Rassool  (also: Fazil)
  - General Smuts
  - Haji Joosub Maulvi Hamid Gool  (also: J.M.H. Gool, JMH Gool, J. Gool, Joosub Gool)
  - Jomo Kenyatta
  - Jon Dos Passos
  - Joseph Bredenkamp
  - Joseph Chamberlain
  - Joyce Dixon
  - Joyce Meissenheimer
  - Kader Essack  (also: Kader)
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
Since the narrator is 'my' and no specific name is given for the narrator, we can't determine the exact relationship. However, based on the quote "my grandfather", we know that the narrator's parent is not mentioned, but their grandparent is.

Assuming the narrator's name is one of the listed names (although it's not explicitly stated), and considering the quote mentions a grandparent, if we knew the narrator's name, we could potentially establish a relationship. However, since the narrator's name is not provided, we can't determine the exact relationship.

Given the rules, in this case, I would return:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 103 / 124  (id=6546657237567362266)

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

## Chunk 104 / 124  (id=6614900686580700622)

**Section:** Yousuf (Joe) Rassool   140  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #888

**Entities in chunk:**
  - Abrahams brothers  (also: Abrahams)
  - Andrew Mackrill
  - Colin Wynne
  - David Poole
  - Dawood Seedat  (also: Dawood)
  - Fatima Gool  (also: Fatima, Fatima (Timmie), Timmie, my Auntie Timmie)
  - George Veldsman
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Johhaar Mosaval  (also: Johaar Mosaval)
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

## Chunk 105 / 124  (id=6622459745407552529)

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

## Chunk 106 / 124  (id=6941537010882242866)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #80

**Entities in chunk:**
  - Eddieas
  - Field Marshal Tito
  - Hamid
  - J. Wilson  (also: Wilson)
  - Kader Essack  (also: Kader)
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

## Chunk 107 / 124  (id=7168244047913479805)

**Section:** Yousuf (Joe) Rassool   182  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1141

**Entities in chunk:**
  - Cissie Gool  (also: Cissie, Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool)
  - Dr AH Gool
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - INDEX 166  (also: INDEX	 166)
  - Michael Johns
  - Ralph J Bunche  (also: Ralph Bunche)
  - Robert R. Edgar  (also: Robert R Edgar)
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

## Chunk 108 / 124  (id=7243012516835118461)

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

## Chunk 109 / 124  (id=7262414809749442984)

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

## Chunk 110 / 124  (id=7266467364867027437)

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
  - Aunt Jane Gool-Tabata Wahida
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - John Phillips
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Ms Sharon Parker  (also: Sharon Parker)
  - Nasima
  - Raza
  - Reagon
  - Reddy
  - Regan
  - Reginald Lesch
  - Rev Gordon
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

The quote states: "my aunt Jane Gool-Tabata (Wahida’s daughter)"

However, according to the CRITICAL RULES, if the quote uses 'aunt', return {"relations":[]} — these relation types are not in the schema.

So, the answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 111 / 124  (id=7360610042715665275)

**Section:** 27  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #158

**Entities in chunk:**
  - Ally
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - INDEX 166  (also: INDEX	 166)
  - Les Jacobs
  - Mohandas Karamchand Gandhi  (also: MK Gandhi, M.K. Gandhi, M_ Gandhi, M. Gandhi)
  - Urdu
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

## Chunk 112 / 124  (id=7448875471219160758)

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

## Chunk 113 / 124  (id=7574535754100824916)

**Section:** Yousuf (Joe) Rassool   132  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #838

**Entities in chunk:**
  - Al Hajj Joosub Maulvi Hamid
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Fabians Kies
  - Howard Fast
  - Hussein Parker
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

## Chunk 114 / 124  (id=7584768665477924623)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #81

**Entities in chunk:**
  - Alie Fataar
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Edna
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - George Meissenheimer  (also: George Meisenheimer)
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Gwen Wilcox
  - Halima Dudley
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
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
Since the narrator is "Yousuf Rassool" and the quote mentions "his aged mother", we can infer that Yousuf Rassool's mother is being referred to. However, the name of the mother is not provided in the quote.

But since the quote does mention a family relationship, we can extract it as follows:

{"relations":[{"from":"Yousuf Rassool","relation":"child_of","to":null}]}

However, according to the rules, if both endpoints are not identifiable as canonical names, return empty. Therefore, the correct answer is:

{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 115 / 124  (id=7643675205690013378)

**Section:** 7  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #74

**Entities in chunk:**
  - Abbas Dinath  (also: Abbas)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Cyril Schoenraad  (also: Cyril Schoeraad)
  - Helen Abrahams
  - Nasima
  - Oswald Pirow
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** father, brother

**CC pass (raw):**
```json
{"quote": "my grandpa... went to join his brother, Abbas"}
```

**CC quote:** `my grandpa... went to join his brother, Abbas`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}

This is because the quote mentions "grandpa" and a brother named Abbas, but it does not directly state the relationship between Yousuf Rassool (the narrator) and Abbas. The relationship mentioned in the quote is between Yousuf's grandpa and Abbas, which is not within the scope of the allowed relationships.
```

**Extracted relations:** none

---

## Chunk 116 / 124  (id=7727744974142389137)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #884

**Entities in chunk:**
  - E. Albertus  (also: Albertus)
  - G.Abed  (also: Abed)
  - INDEX 166  (also: INDEX	 166)
  - Khadija
  - Mr. Parker  (also: Parker)
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** wed , parent

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 117 / 124  (id=7770199354322409691)

**Section:** Chapter Sixteen    Taking the Nation to School  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #881

**Entities in chunk:**
  - Ben Kies  (also: Ben Kies M.A)
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

## Chunk 118 / 124  (id=8034240727763764335)

**Section:** Yousuf (Joe) Rassool   8  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #79

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Ben Kies  (also: Ben Kies M.A)
  - Bibi Gool  (also: Bibi, Peari Beghum)
  - Jack Meltzer
  - Meltzer  (also: Mr. Meltzer)
  - Mohammed Hanief Gool  (also: Mohammed Hanief, Aboeta)
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

## Chunk 119 / 124  (id=8317664100809929487)

**Section:** 143  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #911

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Dr DuPlessis  (also: DuPlessis)
  - Gadija Noor  (also: Gadija, Auntie Dij)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Hoosain
  - Mr Edross  (also: Dr. Edross)
  - Mr. Parker  (also: Parker)
  - Nurjehan Gool Mohamed  (also: Nurjehan)
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

## Chunk 120 / 124  (id=8675141585945090217)

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

## Chunk 121 / 124  (id=8719056895392496748)

**Section:** Chapter Seven     Family Progress  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #195

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Athol Rowan
  - Bruce Mitchell
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hanief Ally  (also: Uncle Hanief, Sayed Hanief Ally, Hanief)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Ken Viljoen
  - Khadija
  - Melville
  - Mohammed Saaid  (also: Mohamed Saaid)
  - Mrs Ahmed  (also: Ahmed)
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
{"relations":[{"from":"Abdul Hamid Gool","relation":"child_of","to":"BG"}]}
```

**Extracted relations:** none

---

## Chunk 122 / 124  (id=8730565953127446314)

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

## Chunk 123 / 124  (id=9080060841814409203)

**Section:** 133  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #842

**Entities in chunk:**
  - Abdul Hamid Gool  (also: A.H. Gool, AH Gool, Abdul Hamid (BG), Abdul Hamid)
  - Arthur Tracey
  - Deanna Durbin
  - Goolam Gool  (also: Dr Goolam Gool, Dr. Goolam, Goolam (GH), Goolam)
  - Hamid Khan
  - Hassen Mall  (also: Hassen)
  - Hutch
  - Mylie Begg's brother-in-law
  - Yousuf Rassool  (also: Joe Rassool, Yousuf (Joe) Rassool, Yusuf Rassool, Y.S. Rassool)

**Triggers found:** son , brother, in-law

**CC pass (raw):**
```json
{"quote": "Mylie Begg’s brother in-law, Hamid Khan"}
```

**CC quote:** `Mylie Begg’s brother in-law, Hamid Khan`  ✅ anchored → EC

**EC pass (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---

## Chunk 124 / 124  (id=9200706703269745412)

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
| Chunks processed | 124 |
| Relations extracted | 25 |
| Relations written to graph | 25 |
