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
    {"from":"Naz","relation":"sibling_of","to":"Geoff"},
    {"from":"Ronnie Jooste","relation":"spouse_of","to":"Mr. Slingers"}
  ]
}
```

**Extracted relations:**
- `Naz` **sibling_of** `Geoff`
- `Ronnie Jooste` **spouse_of** `Mr. Slingers`

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

**Chunk text:**
```
ly Salie Van Haacht - 
Haartzen for short- as well as Ben Malamba, one of the top African cricketers in the Cape. In 
addition, I discovered that one of my sister’s friends, a very athletic young man named Stanley 
Abrahams was a brilliant leg spinner who could bat, bowl and field like a wizard. Our club, the 
Kismets, by our non-racial policy, had transformed the Indian Cricket Union to such an extent 
that  a  team  of  Seventh  Day  Adventists,  the  Heathrow  Cricket  Club,  joined  into  the  Sunday 
League bringing some new talents in James Africa and “Woody” Woodman. Most other clubs 
followed our example. 
 
Through cricket, Muddy and I became close friends. He showed an interest in poetry and often 
whilst driving about would recite long extracts of The Rubaiyat. Politically we be
```

**LLM response (raw):**
```json
{"relations":[{"from":"Ben Malamba","relation":"spouse_of","to":"Salie Van Haacht"}]}
```

**Extracted relations:**
- `Ben Malamba` **spouse_of** `Salie Van Haacht`

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

**Chunk text:**
```
brother Fazil contributing towards the 
domestic kitty, we were able to afford the luxury of a refrigerator, a large humming tall white 
Kelvinator.
Strangely, my mother did not buy a washing machine.
Perhaps she was unsure about 
the  churning  metal  propellor  and  what  it  might  do  to  the  clothes.  Instead,  she  preferred  to 
continue to bend over the iron bath on washdays, and rip the shirts and linen to shreds with a
pinecone, and then haul them upstairs to the loft to dry.
How did my mother manage this day in, 
day out?
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

**Chunk text:**
```
elped to bring about a settlement. 
    The Gool mansion in District Six is no more, but we should not forget the history 
made at 7 Buitencingle by the Gools and their friends. 
    With best wishes, 
    E.S. Reddy 
   Former director of United Nations Centre against Apartheid 
 
xxi Ralph Bunche records: 
…one of Gandhi’s sons fell in love with one of Goulam Gool’s sisters – a Moslem. The 
girl wrote to Gandhi telling him of their love and asking him for his approval of this 

 

   183    

                                                                                                                                                  
union of a Hindu and Moslem as a fine demonstration of the breakdown of religious 
barriers between the group, which he had advocated. (The son was in S.
```

**LLM response (raw):**
```json
{
"relations": [
{"from":"Mohandas Karamchand Gandhi","relation":"parent_of","to":"Goolam Gool"},
{"from":"Goolam Gool","relation":"sibling_of","to":"MQotsi"}
]
}
```

**Extracted relations:**
- `Mohandas Karamchand Gandhi` **parent_of** `Goolam Gool`
- `Goolam Gool` **sibling_of** `MQotsi`

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

**Chunk text:**
```
imposed  by  Balthazzar John  Vorster,  the  Minister  of Justice,  on 
Joyce  Meissenheimer,  Chairperson  of  the  Cape  Peninsula  council  of  PTAs,  and  Alie  Fataar, 
former  secretary  of  the  TLSA.  Aunt  Jane  was  also  served  with  banning  orders  and  so  were 
Messrs R.O.Dudley, E.L.Maurice, GL Abrahams, and F.Landman. Victor Wessels and Cosmo 
Pieterse were also banned for 5 years under the Riotous Assemblies Act. 
 
I remember going to one of the most poignant mass meetings of protest held in the Drill Hall on 
Sunday  15
th  February  to  protest  against  the  dismissals.  The  split  in  the  Movement  was 
temporarily  set  aside.  The  mood  was  bitter,  angry  and  defiant.  There  were  tears,  even  from 
hardened campaigners like Alie Fataar, who had switched
```

**LLM response (raw):**
```json
{"relations":[{"from":"Alie Fataar","relation":"spouse_of","to":"Geoffrey Abrahams"}]}
```

**Extracted relations:**
- `Alie Fataar` **spouse_of** `Geoffrey Abrahams`

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

**Chunk text:**
```
red themselves 
bosom friends: Churchill, Smuts, and Hollywood. We had films like “Song of Russia”, “Counter 
Attack” and others showing the Russians as normal human beings. A new organisation called 
the  Friends  of  the  Soviet  Union  was  launched  and  Field  Marshal  Smuts,  the  Prime  Minister 
became a patron! When rallies were held in the City Hall, Uncle Hanief, Musto and I went to 
hear speeches from Smuts, the Soviet Ambassador, Cissie Gool, Moses Kotane, Comrade Bill, 
Ray Alexander and Harry Snitcher. It was all very stimulating and exciting. After the rally the 
members  of  the communist  party  handed out leaflets and application  forms. We  were  sorely 
tempted  to join.  Musto  was  particularly  keen  to  do  so  and, had  he followed through  with  his 
intention, I
```

**LLM response (raw):**
```json
{"relations":[]}
```

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

**Chunk text:**
```
n Constantia 
Road,  Wynberg,  I  was  conducted  to  his  panelled  study.  I  remember  he  was  seated  in  a 

District Six - Lest We Forget 

Yousuf (Joe) Rassool   144   

commodious wine-red leather armchair, seemingly engrossed in reading The Collected works of 
Wordsworth,  impressively  morrocan-bound.  Victor  and  Hymie  came  together  a  few  minutes 
later, and it seemed that they too were a bit overawed by it all. Then Mr. Tabata and my aunt 
Jane Gool arrived. We were now in the presence of three of the top people of the Movement, 
and we felt like downy-cheeked, wet-behind-the-ears novices. 
 
  
 
67  Rally  against  the  Van  Riebeeck  Festival  –  see  cross  on  upside  down  statue  of  Van 
Riebeeck. From left to right: Phillis Ntantala Jordan, W.P. van Schoor (Pres
```

**LLM response (raw):**
```json
{
"relations": [
{"from":"Yousuf Rassool","relation":"spouse_of","to":""},
{"from":"","relation":"parent_of","to":"Yousuf Rassool"},
{"from":"Jane Gool-Tabata","relation":"aunt_of","to":"Yousuf Rassool"}
]
}
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

**Chunk text:**
```
Ismail, much to Bibi and my mother’s annoyance. It irked them that Katie should decline to do 
any household chores on the grounds that it was not what she had been instructed to do by my 
aunt. Katie slept in our room, with little Ismail. At night she would read her bible and I would 
ask her about things that bothered me. I raised the matter of death with her and for the first time 
got  a  non-threatening  point  of  view.  To  Katie,  the  coffin  was  a  soft  silk-lined  box  ever  so 
comfortable.  Somehow,  she  managed  to  allay  my  dread  about  the  inevitability  of  death,  and 
quieted my uneasy heart again. 
 
Katie  carried  out  her  duties  to  the  letter  and  beyond.  Baby  Ismail  was  fed  on  Cow  &  Gate 
powdered milk. Cow&Gate was poured down his tiny throat w
```

**LLM response (raw):**
```json
{
"relations": [
{"from":"Yousuf Rassool","relation":"parent_of","to":"Ismail"},
{"from":"Bibi Gool","relation":"spouse_of","to":"Yousuf Rassool"}
]
}
```

**Extracted relations:**
- `Yousuf Rassool` **parent_of** `Ismail`
- `Bibi Gool` **spouse_of** `Yousuf Rassool`

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

**Chunk text:**
```
he “Indian” members had to go to the Indian Affairs 
Department and obtain a permit to travel from The Cape to Natal. Those involved were Tiny 
Abed, Gulzar Khan, Abou Desai, Moorgas Naidoo, Hoosain Parker, the Hayat brothers and I. 
Our secretary's attention to detail brought to light some interesting revelations: Menage Rassool 
was not on the Department's files, whereupon a file was duly created. When Tiny’s turn arrived, 
it was discovered that the entire Abed family were originally from Durban and had apparently 
settled in the Cape ages ago without the required permission. With a sadistic smirk, the officer 
declared that whereas Tiny needed no permit to go to Durban, his return to Cape Town would 
be illegal. I recall the unsettling panic that followed that announcement. Suddenly th
```

**LLM response (raw):**
```json
{
  "relations": [
    {"from":"Tiny Abed","relation":"child_of","to":"Abou Desai"},
    {"from":"Ghulzar Khan","relation":"spouse_of","to":"Cissie Gool"}
  ]
}
```

**Extracted relations:**
- `Tiny Abed` **child_of** `Abou Desai`
- `Ghulzar Khan` **spouse_of** `Cissie Gool`

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 9 |
| Relations extracted | 10 |
| Relations written to graph | 0 |
