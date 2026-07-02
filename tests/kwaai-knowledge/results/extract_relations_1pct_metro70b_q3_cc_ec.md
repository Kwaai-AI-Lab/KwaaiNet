# Family Relation Extraction — 1% sample

**Qualifying chunks (≥2 entities + trigger):** 168  
**Sampled:** 2  
**Model:** llama3.1:70b-instruct-q3_K_M  
**Commit:** dry-run

---

## Chunk 1 / 2  (id=-9073283356142808755)

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
{"quote": "none"}
```

**CC quote:** none — EC pass skipped

**Extracted relations:** none

---

## Chunk 2 / 2  (id=-9022111028918856016)

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
{"quote": "one of my sister’s friends"}
```

**CC quote:** `one of my sister’s friends`  ⚠️ no entity name in quote → EC skipped

**Extracted relations:** none

---


## Summary

| Metric | Value |
|--------|-------|
| Chunks processed | 2 |
| Relations extracted | 0 |
| Relations written to graph | 0 |
