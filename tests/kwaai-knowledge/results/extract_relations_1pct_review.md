# Family Relation Extraction — 1% sample

**Qualifying chunks (≥2 entities + trigger):** 326  
**Sampled:** 4  
**Model:** llama3.1:8b  
**Commit:** dry-run

---

## Chunk 1 / 4  (id=-9102990948384786174)

**Section:** Chapter Nineteen  All Africa Convention  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #989

**Entities in chunk:**
  - Goolam Gool  (also: Goulam Gool, Dr Goolam Gool, Dr. Goolam, Goolam (GH))
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

**Triggers found:** uncle

**Chunk text:**
```
Africa quoting from Homer. 
He was, of course, referring to the Herrenvolk rulers and the mania, which seemingly beset them 
during their striving to maintain and preserve their privileged existence.
The expression could 
also have aptly applied to the circumstances surrounding my uncle Goolam Gool.
```

**LLM response (raw):**
```json
{
  "relations": [
    {"from":"Goolam Gool","relation":"parent_of","to":"Hanief Ally"}
  ]
}
```

**Extracted relations:**
- `Goolam Gool` **parent_of** `Hanief Ally`

---

## Chunk 2 / 4  (id=-9084546144930260499)

**Section:** Yousuf (Joe) Rassool   166  
**Doc:** LEST WE FORGET -rev25.pdf  chunk #1071

**Entities in chunk:**
  - Amien Basier  (also: Amien)
  - Amod Gool
  - Andrew Mackrill
  - Annie Rassool
  - Dr. Abdulla Abdurahman  (also: Abdulla Abdurahman, Abdullah Abdurahman, Dr. Abdullah Abdurahman, Dr. Abdurahman)
  - Haji Joosub Maulvi Hamid Gooli  (also: Joosub Maulvi Hamid Gooli, Haji Joosub Maulvi Hamid Gool, J.M.H. Gool, JMH Gool)
  - Helen Nellie Potter James

**Triggers found:** aunt

**Chunk text:**
```
143, 152, 160 
Amien, 61, 139 
 Amien Basier, 139 
Amina, 5, 12, 39, 42 
Aminabhen, 12 
Amod Gool, 27 
ANC, 137, 155 
Andrew Mackrill, 115, 135 
Annalise, 40 
Annie Rassool, 40 
Anti-CAD, 89, 90, 144
 143, 152, 160 
Amien, 61, 139 
 Amien Basier, 139 
Amina, 5, 12, 39, 42 
Aminabhen, 12 
Amod Gool, 27 
ANC, 137, 155 
Andrew Mackrill, 115, 135 
Annalise, 40 
Annie Rassool, 40 
Anti-CAD, 89, 90, 144, 149, 152, 156, 158, 
159 
Anti-Coloured Affairs Council, 89 
Anti-Coloured Affairs Department, 89, 91 
Anti-Indian Laws, 23 
Anti-TLSA, 156 
Anti-Zionism, 74 
Apartheid and Indian South Africa, 23 
Appollis Slingers, 92 
Arendze, 38 
Argus Annual, 6, 8 
Armin Dawood, 12 
Arms and the Man, 132, 135 
Arthur Hugh Clough, 158 
Arthur Tracey, 96, 97 
Athol Rowan, 114 
Auntie Eileen, 96 
Auntie Tootie, 87, 128 
Avalon Cinema, 128, 137 
Avondale Road, 140 
Babla Salloogee, 156 
Baboo Abed, 133, 134 
Bagalia, 141 
Balalaika, 97 
Ballade, 79 
Balthazzar John Vorster, 163 
Banoo, 111 
Banyas, 6, 60 
B
```

**LLM response (raw):**
```json
{
"relations": [
{"from":"Helen Nellie Potter James","relation":"spouse_of","to":"Dr. Abdulla Abdurahman"},
{"from":"Amod Gool","relation":"parent_of","to":"Amien Basier"}
]
}
```

**Extracted relations:**
- `Helen Nellie Potter James` **spouse_of** `Dr. Abdulla Abdurahman`
- `Amod Gool` **parent_of** `Amien Basier`

---

## Chunk 3 / 4  (id=-9073283356142808755)

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

**Triggers found:** mother, cousin, aunt, uncle

**Chunk text:**
```
aited the outcome at the end of December. With Geoff and my cousin, Naz, I went to where 
the results would be displayed in the windows of The Cape Argus in Burg Street at the centre of 
the City.  
  “You’ve  got  an  exemption!”  shouted  Geoff,  and  my  hopes  rose  fractionally  for  a 
moment and then plunged. I had failed to achieve a first grade. I glanced through the list and 
there was Ronnie Jooste’s name, as well as Slingers, Bartnicke and Springveld from our class 
and a number of others from the A class. Naz and Geoff kept trying to boost my spirits, and I 
think I began to perk up by the time we reached home. I think my mother was pleased and so 
were my aunts and uncles. My dad was a bit disappointed about the first grade. He would dearly 
have liked to bruit about such a s
```

**LLM response (raw):**
```json
{
  "relations": [
    {"from":"Naz","relation":"cousin_of","to":"Bartnicke"},
    {"from":"Geoff","relation":"spouse_of","to":"Davids"}
  ]
}
```

**Extracted relations:**
- `Geoff` **spouse_of** `Davids`

---

## Chunk 4 / 4  (id=-9027039585919698742)

**Doc:** LEST WE FORGET -rev25.pdf  chunk #258

**Entities in chunk:**
  - Ben Kies  (also: Ben)
  - Jack Meltzer  (also: Mr. Jack Meltzer)
  - Mr. Meltzer  (also: Meltzer)
  - NEF Hassan  (also: Hassan)

**Triggers found:** children of

**Chunk text:**
```
can we the children of District Six protect its memory from fading away? To me those 
bare bulldozed acres had more life, more zest, more fire, more warmth and care than many other 
places one could mention. To have been part of that pulsing life was an unforgettable education 
and  privilege,  the  memory  of  which  I  would  dearly  like  to  rekindle  in  those  who  remember 
District Six in its heyday, and to provide a realistic picture to their children in the events leading 
to its demolition. So, let me take you on a walk-through streets that once buzzed with the joy of 
life, and look at it through my ten-year-old eyes as I headed for school each morning. 
 
Caledon Street ran from Corporation Street straight through to Clifton Street, getting tattier along 
the way. Clifton Stre
```

**LLM response (raw):**
```json
{"relations":[{"from":"Ben Kies","relation":"spouse_of","to":"Mr. Meltzer"}]}
```

**Extracted relations:**
- `Ben Kies` **spouse_of** `Mr. Meltzer`

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 4 |
| Relations extracted | 5 |
| Relations written to graph | 0 |
