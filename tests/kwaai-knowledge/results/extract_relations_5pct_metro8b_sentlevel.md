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
  - Naz
  - Nicky Springveld  (also: Springveld)
  - Rashid Zaheer Nurjahan
  - Ronnie Jooste  (also: Ronnie)
  - Shawquet
  - Tiddles
  - Yusuf

**Triggers found:** mother,  cousin

**Focused sentences (sent to LLM):**
```
aited the outcome at the end of December. With Geoff and my cousin, Naz, I went to where. the results would be displayed in the windows of The Cape Argus in Burg Street at the centre of
```

**LLM response (raw):**
```json
{"relations":[{"from":"Naz","relation":"cousin_of","to":"Geoff"}]}
```

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

**Focused sentences (sent to LLM):**
```
ly salie van haacht - 
haartzen for short- as well as ben malamba, one of the top african cricketers in the cape. in 
addition, i discovered that one of my sister’s friends, a very athletic young man named stanley 
abrahams was a brilliant leg spinner who could bat, bowl and field like a wizard. our
```

**LLM response (raw):**
```json
{
  "relations": [
    {"from":"Ben Malamba","relation":"sibling_of","to":"Stanley Abrahams"}
  ]
}
```

**Extracted relations:**
- `Ben Malamba` **sibling_of** `Stanley Abrahams`

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
  - My mother
  - Toetie Ally  (also: Toetie, Auntie Toetie)
  - Zohra Abdurahman  (also: Zohra (Auntie Jolly), Auntie Jolly, Aunt Jolly, Jolly (Zohra))

**Triggers found:** mother, brother

**Focused sentences (sent to LLM):**
```
brother Fazil contributing towards the. domestic kitty, we were able to afford the luxury of a refrigerator, a large humming tall white. Kelvinator. Strangely, my mother did not buy a washing machine. Perhaps she was unsure about. pinecone, and then haul them upstairs to the loft to dry. How did my mother manage this day in,. day out
```

**LLM response (raw):**
```json
{"relations":[]}
```

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

**Focused sentences (sent to LLM):**
```
xxi Ralph Bunche records:. …one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters – a Moslem. The
```

**LLM response (raw):**
```json
{
  "relations": [
    {
      "from": "Mohandas Karamchand Gandhi",
      "relation": "parent_of",
      "to": "Gandhi's son"
    },
    {
      "from": "Goolam Gool",
      "relation": "sibling_of",
      "to": "Goulam Gool's sister"
    }
  ]
}
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

**Focused sentences (sent to LLM):**
```
imposed  by  balthazzar john  vorster,  the  minister  of justice,  on 
joyce  meissenheimer,  chairperson  of  the  cape  peninsula  council  of  ptas,  and  alie  fataar, 
former  secretary  of  the  tlsa.  aunt  jane  was  also  served  with  banning  orders  and  so  were 
messrs r.o.dudley, e.
```

**LLM response (raw):**
```json
{"relations":[]}
```

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

**Focused sentences (sent to LLM):**
```
became a patron. When rallies were held in the City Hall, Uncle Hanief, Musto and I went to. hear speeches from Smuts, the Soviet Ambassador, Cissie Gool, Moses Kotane, Comrade Bill,. tempted  to join. Musto  was  particularly  keen  to  do  so  and, had  he followed through  with  his. intention, I
```

**LLM response (raw):**
```json
{
"relations": [
{"from":"Musto","relation":"sibling_of","to":"Abba (Louis Messaris)"},
{"from":"Smuts","relation":"spouse_of","to":"Ray Alexander"}
]
}
```

**Extracted relations:**
- `Musto` **sibling_of** `Abba (Louis Messaris)`
- `Smuts` **spouse_of** `Ray Alexander`

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

**Focused sentences (sent to LLM):**
```
n constantia 
road,  wynberg,  i  was  conducted  to  his  panelled  study.  i  remember  he  was  seated  in  a 

district six - lest we forget 

yousuf (joe) rassool   144   

commodious wine-red leather armchair, seemingly engrossed in reading the collected works of 
wordsworth,  impressively  mo
```

**LLM response (raw):**
```json
{"relations":[]}
```

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

**Focused sentences (sent to LLM):**
```
Ismail, much to Bibi and my mother’s annoyance. It irked them that Katie should decline to do
```

**LLM response (raw):**
```json
{"relations":[]}
```

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

**Focused sentences (sent to LLM):**
```
Those involved were Tiny. Abed, Gulzar Khan, Abou Desai, Moorgas Naidoo, Hoosain Parker, the Hayat brothers and I. Our secretary's attention to detail brought to light some interesting revelations: Menage Rassool
```

**LLM response (raw):**
```json
{"relations":[]}
```

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 9 |
| Relations extracted | 3 |
| Relations written to graph | 0 |
