# Family Relation Extraction — 5% sample

**Qualifying chunks (≥2 entities + trigger):** 168  
**Sampled:** 9  
**Model:** llama3.1:8b  
**Commit:** dry-run

---

## Chunk 1 / 9  (id=-9073283356142808755)

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
  - Yusuf

**Triggers found:** mother,  cousin

**CC pass (raw):**
```json
{"quote": "my mother was pleased"}
```

**CC quote:** `my mother was pleased`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---

## Chunk 2 / 9  (id=-9022111028918856016)

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

**Triggers found:** wed , sister

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 3 / 9  (id=-8746970108171972112)

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

## Chunk 4 / 9  (id=-8621339613752899196)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #943

**Entities in chunk:**
  - Abbas Dinath
  - Dr. Patel  (also: Patel)
  - E.S Reddy Former  (also: E.S. Reddy Former)
  - Goolam Gool  (also: Goulam Gool, Dr Goolam Gool, Dr. Goolam, Goolam (GH))
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
Here is the extracted family relationship:

```
{
  "relations": [
    {
      "from": "Gandhi",
      "relation": "child_of",
      "to": "Goolam Gool"
    },
    {
      "from": "Gandhi",
      "relation": "sibling_in_law_of",
      "to": "Goolam Gool"
    }
  ]
}
```

Note: I used the canonical names from the list to extract the relationships. The quote directly states that one of Gandhi's sons fell in love with one of Goulam Gool's sisters, which implies a sibling-in-law relationship between Gandhi and Goolam Gool.
```

**Extracted relations:** none

---

## Chunk 5 / 9  (id=-8512653961804288406)

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

## Chunk 6 / 9  (id=-8167953885918041797)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #396

**Entities in chunk:**
  - Abba (Louis Messaris)  (also: Abba)
  - Churchill Smuts  (also: Churchill)
  - Cissie Gool  (also: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool)
  - Field Marshal Smuts
  - George
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

## Chunk 7 / 9  (id=-8054118982771437578)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #768

**Entities in chunk:**
  - Abba (Louis Messaris)  (also: Abba)
  - Hymie Beimel  (also: Hymie)
  - Jane Gool-Tabata  (also: Jane Gool, Janab)
  - Phillis Ntantala Jordan
  - Victor
  - Willem Van Schoor  (also: W.P. Van Schoor, W.P Van Schoor)
  - Yousuf Rassool  (also: Yusuf Rassool, Joe Rassool, Yousuf (Joe) Rassool, Y.S. Rassool)

**Triggers found:** wed ,  aunt 

**CC pass (raw):**
```json
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 8 / 9  (id=-7923858109226632030)

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

## Chunk 9 / 9  (id=-7894516247149505240)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #737

**Entities in chunk:**
  - Abou Desai  (also: Abou)
  - Cissie Gool  (also: Zainunnisa Cissie Gool, Zainunnisa Gool, Zainunnissa Cissie Gool, Zainunnissa Gool)
  - General Smuts
  - Ghulzar Khan  (also: Gulzar Khan)
  - Hoosain Parker  (also: Mr. Hoosain Parker)
  - Moorgas Naidoo
  - Tiny Abed

**Triggers found:** wed , parent, brother

**CC pass (raw):**
```json
{"quote": "the entire Abed family were originally from Durban and had apparently settled in the Cape ages ago without the required permission."}
```

**CC quote:** `the entire Abed family were originally from Durban and had apparently settled in the Cape ages ago without the required permission.`  ✅ anchored → EC

**EC pass (raw):**
```json
Since there is no mention of a specific family relationship between any two individuals, the output will be an empty array.

```
{"relations":[]}
```
```

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 9 |
| Relations extracted | 0 |
| Relations written to graph | 0 |
