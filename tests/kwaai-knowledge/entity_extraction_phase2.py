#!/usr/bin/env python3
"""
Phase 2 experiments: Go deeper on the most promising findings from Phase 1.

Key Phase 1 findings:
  1. Sentence alignment beats raw cut (+10pp) — confirmed, explore further
  2. Window=1 is the sweet spot (w2 suffers from large JSON → parse failures)
  3. JSON parse failures kill large-context strategies — test JSON robustness mitigations
  4. Context framing inconsistent — doc context helps some passages, hurts others
  5. 3-type plain has highest recall per-token; 3-type-fields adds field quality
  6. Relations: full passage gives most relations but entity recall varies

Phase 2 experiments:
  A. JSON robustness: test strategies to reduce parse failures
     - Reduced max_tokens + entity-count cap in prompt
     - Two-pass: entities first, then fields separately
     - Ask for entities as a flat list first, then enrich
  B. Optimal sentence-aligned window (w=1 confirmed, test larger sentences)
     - Chunk size: 80w vs 100w vs 150w sentences, window=1
  C. Context framing — targeted test with the RIGHT context type
     - Just book title vs full provenance sentence
  D. Relation accuracy deep-dive
     - How many extracted relations are actually correct?
     - LLM-as-judge on a sample
     - Window=1 vs full passage for relation precision/recall
"""

import json, re, time, sys, textwrap
from typing import Optional
import urllib.request

OLLAMA_URL = "http://localhost:11434"
MODEL = "llama3.1:8b"

# ─── Same test passages and gold from Phase 1 ─────────────────────────────────
PASSAGE_A = """
My uncle's residence now shifted to Searle Street, close to Trafalgar Park. In the early thirties
he must have found the sectarian politics not to his liking. His mentor, M.K. Gandhi, greatly
wished him to throw his boundless energies into the South African Indian struggle, but he
rejected it. Perhaps he was now a complete cosmopolitan and did not accept the Indian ethnic
tag as essential to the cause. For all that, he made his home in Searle Street, unique in South
Africa of that time for being a place where all the leading luminaries in the political and cultural
arena would foregather for him to enjoy their verbal squabbles while they enjoyed his hospitality.
There at Searle Street Cheops could indulge his taste for art and music with politics thrown in
for seasoning. It was there that the young Bernard Herzberg, an escapee from Hitler's Germany
in 1933, came with his much older girlfriend Eva Sachs. It was Eva, a music lover, who spotted
the lone non-white sitting in the segregated area of the City Hall on a Saturday night and
ostentatiously went to sit beside him. Thence followed the invitation to Searle Street. Young
Bernard confessed he could not afford the tram fare and walked all the way, but there he met
such luminaries in art, music, and politics as Fred Bodmer, Gregoire Boonzaaier, Terence
Macaw, Ben Kies, Paul Kostin, Wolf Kiebel, J.G. and Dora Taylor, Jane Gool, I. B. Tabata,
Eddie Roux, Goolam Gool, Eli Weinberg, Waradeya Abdurahman, Dr. Abdulla Abdurahman,
Leah Solomon and Zaid Gamiet. Those were the days when politicos did not boycott opponents
but argued their cases face to face. There the young Hans Friederichs a staunch Stalinist,
recently arrived from Germany, (sweeping Minnie Gool off her feet), was heard to say,
"Hitler's rise did not mean the defeat of the German Working Class". In pharaonic style Cheops
held sway ably assisted by the women who loved him: Cissie Gool, Freda Lock, and Lilian Isaacson.
""".strip()

PASSAGE_B = """
I mentioned that Dr. Gool was my half-uncle. This relationship was due to the fact that my
grandfather, J.M.H. Gool, had two wives. There was Bibi, whose family lived in Broach,
Gujarat, India, whom he married in 1879 or 1880. She was at the tender age of eleven and he
must have been about sixteen. The second wife was Wahida Ta'al, whose mother was a Dollie,
a family of considerable importance in the Cape.

Joosub Maulvi Hamid was a person of striking good looks. He was tall and carried himself
proudly. His father was a Pushto speaking Maulvi, or religious leader, who had moved from the
Province of Swat to Rander in Gujarat with his wife and two sons, Abbas and Yusuf. Gujarati
was generally the language spoken in that part of India but the Gool family spoke Urdu.

The elder son, Abbas, bade farewell to the family to seek his fortune in the sugar cane plantations
of Mauritius, and it was there that the young Yusuf Hamid went after marrying Bibi, to follow
in his brother's footsteps. According to Aunt Jane, he became a fluent French speaker and lived
the life of a French seigneur before his health demanded a change of climate, and the Cape was
the answer.

His wealth brought him into contact with many notables of that time, such as Barney Barnato,
Solly Joel, Cecil Rhodes, Gandhi, the Prince of Wales, George Bernard Shaw, Sarojini Naidu,
Dr. Abdullah Abdurahman, Miss Elizabeth Molteno, Alice Greene (the aunt of Grahame Greene,
the novelist) and the various Indian Agents General. I believe the Irish Peer, Lord Headley, Head
of the British Muslims, also stayed at Buitencingle when he visited South Africa as did the
entire All India Football team that toured South Africa in the early thirties.
""".strip()

PASSAGE_C = """
We pronounced the name of the street Byootencingle. It was of Dutch derivation and should
more correctly have been pronounced Baytencingle, with the "Ay" sound made rounder by
pursing the lips. The cingle referred to a belt. Buitencingle was at that time the outermost belt of
Cape Town. Beyond it lay the foothills of Table Mountain reached via Kloof Street, which led
from the bottom of Buitencingle up to Kloof Nek. This was the bus terminus for the single-
decker trackless tram that ran from Adderley Street outside Fletchers and Cartwrights, a
department store that sold everything from flan cakes to furniture. Kloof Nek was the connecting
saddle of land that joined Table Mountain to Lion's Head.

My grandfather's house stood at No.7 Buitencingle Street. The street was wide and cobbled with
traffic islands in the middle, which sported a few stunted palm trees. Opposite it, stood the
German Lutheran Church, grey, austere and remote, bustling into activity on Sunday mornings
with a clamour of bells. It was bounded by Long Street, Buitencingle and Loop Street, which
branched off at right angles from Buitencingle to meander down to Waterkant Street a short
distance from the waterfront of Rogge Bay.

In Loop Street there stood the Hanaffi Qwatul Islam Mosque, an outpost of Islam in an area
becoming increasingly European, Jewish and gentile. My grandfather was a founder and life
trustee of the mosque, which was completed in 1898, and we used to go there to celebrate Eid
festivals. The Botanical Gardens, which date back to Jan Van Riebeeck, 1652, had its exit a short
walk up Orange Street, and the famous Mount Nelson Hotel, where visiting celebrities of the time
used to stay, was opposite the Gardens. My short-cut home from school was the Avenue through
the Gardens past St. George's Grammar, The Houses of Parliament, The South African Public
Library, the aviary, the rose garden, the art gallery, the museum and The Little Theatre.
Buitencingle was no common residential area.
""".strip()

GOLD_A = {
    "Person": ["M.K. Gandhi", "Dr. Abdul Hamid Gool", "Bernard Herzberg", "Eva Sachs",
               "Fred Bodmer", "Gregoire Boonzaaier", "Terence Macaw", "Ben Kies",
               "Paul Kostin", "Wolf Kiebel", "J.G. Taylor", "Dora Taylor", "Jane Gool",
               "I. B. Tabata", "Eddie Roux", "Goolam Gool", "Eli Weinberg",
               "Waradeya Abdurahman", "Dr. Abdulla Abdurahman", "Leah Solomon",
               "Zaid Gamiet", "Hans Friederichs", "Minnie Gool", "Cissie Gool",
               "Freda Lock", "Lilian Isaacson"],
    "Place": ["Searle Street", "Trafalgar Park", "City Hall", "Germany", "South Africa"],
    "Organization": ["German Working Class"],
}
GOLD_B = {
    "Person": ["J.M.H. Gool", "Bibi", "Wahida Ta'al", "Joosub Maulvi Hamid",
               "Abbas", "Yusuf", "Aunt Jane", "Barney Barnato", "Solly Joel",
               "Cecil Rhodes", "Gandhi", "George Bernard Shaw", "Sarojini Naidu",
               "Dr. Abdullah Abdurahman", "Elizabeth Molteno", "Alice Greene",
               "Grahame Greene", "Lord Headley"],
    "Place": ["Broach", "Gujarat", "India", "Mauritius", "Cape Town",
              "Swat", "Rander", "Buitencingle"],
    "Organization": ["All India Football team", "British Muslims"],
}
GOLD_C = {
    "Person": ["Jan Van Riebeeck"],
    "Place": ["Buitencingle Street", "Cape Town", "Table Mountain", "Kloof Street",
              "Kloof Nek", "Adderley Street", "Lion's Head", "Loop Street",
              "Long Street", "Rogge Bay", "Orange Street", "Waterkant Street"],
    "Organization": ["Fletchers and Cartwrights", "German Lutheran Church",
                     "Hanaffi Qwatul Islam Mosque", "Mount Nelson Hotel",
                     "Botanical Gardens", "St. George's Grammar",
                     "The Houses of Parliament", "The South African Public Library",
                     "The Little Theatre"],
}
GOLD = {"A": GOLD_A, "B": GOLD_B, "C": GOLD_C}
PASSAGES = {"A": PASSAGE_A, "B": PASSAGE_B, "C": PASSAGE_C}
ENTITY_TYPES_3 = ["Person", "Place", "Organization"]
EXPECTED_FIELDS = {
    "Person": ["birthDate", "birthPlace", "deathDate", "nationality", "occupation",
               "affiliation", "spouse", "parent", "sibling", "child"],
    "Place": ["addressLocality", "addressRegion", "addressCountry", "locationType", "historicalNote"],
    "Organization": ["foundingDate", "dissolutionDate", "location", "founder", "orgType"],
}

# ─── Utilities (same as Phase 1) ───────────────────────────────────────────────

def words(text): return text.split()
def normalize(name):
    n = name.lower()
    for p in ["dr. ","dr ","mr. ","mr ","mrs. ","mrs ","ms. ","ms ","prof. ","prof ","lord ","sir ","the "]:
        n = n.removeprefix(p)
    return n.strip()

def recall(extracted, gold):
    if not gold: return 1.0
    norm_ext = {normalize(e) for e in extracted}
    return sum(1 for g in gold if normalize(g) in norm_ext) / len(gold)

def repair_json(content):
    content = re.sub(r'^```(?:json)?\s*', '', content.strip())
    content = re.sub(r'\s*```$', '', content)
    content = re.sub(r',\s*([}\]])', r'\1', content)
    return content.replace(': None', ': null').replace(':None', ':null')

def call_ollama(prompt, temperature=0.1):
    body = json.dumps({
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": temperature,
        "max_tokens": 2048,
        "stream": False,
    }).encode()
    req = urllib.request.Request(f"{OLLAMA_URL}/api/chat", data=body,
                                  headers={"Content-Type": "application/json"}, method="POST")
    content = ""
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            raw = json.loads(resp.read())
            content = raw["message"]["content"].strip()
            return json.loads(repair_json(content))
    except json.JSONDecodeError as e:
        m = re.search(r'\{.*\}', content, re.DOTALL)
        if m:
            try: return json.loads(repair_json(m.group()))
            except: pass
        print(f"    [JSON ERROR] {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"    [ERROR] {e}", file=sys.stderr)
        return None

def extract_names(result, type_filter=None):
    if not result or "entities" not in result: return set()
    return {e.get("name","").strip().lower() for e in result["entities"]
            if (not type_filter or e.get("type","") in type_filter) and e.get("name","").strip()}

def field_completeness(result):
    if not result or "entities" not in result: return 0.0
    scores = []
    for e in result["entities"]:
        t = e.get("type","")
        if t not in EXPECTED_FIELDS: continue
        exp = EXPECTED_FIELDS[t]
        fields = e.get("fields",{})
        if not fields:
            if e.get("description","").strip(): scores.append(0.1)
            continue
        filled = sum(1 for k in exp if str(fields.get(k) or "").strip())
        scores.append(filled / len(exp))
    return sum(scores)/len(scores) if scores else 0.0

def sentence_align(text, target_words):
    sentences = re.split(r'(?<=[.!?])\s+', text.strip())
    chunks, current, count = [], [], 0
    for sent in sentences:
        sw = len(sent.split())
        if count + sw > target_words and current:
            chunks.append(" ".join(current)); current = [sent]; count = sw
        else:
            current.append(sent); count += sw
    if current: chunks.append(" ".join(current))
    return chunks

def extract_windowed(chunks, window, prompt_fn):
    all_ents = {}; all_rels = []
    for i in range(len(chunks)):
        start = max(0, i - window//2); end = min(len(chunks), i + window//2 + 1)
        text = "\n\n[...]\n\n".join(chunks[start:end])
        r = call_ollama(prompt_fn(text))
        if r:
            for e in r.get("entities",[]):
                k = e.get("name","").strip().lower()
                if k: all_ents[k] = e
            all_rels.extend(r.get("relations",[]))
    return {"entities": list(all_ents.values()), "relations": all_rels}

def metrics(result, pkey, label, n_chunks, window, elapsed, etypes=None):
    if etypes is None: etypes = ENTITY_TYPES_3
    gold = GOLD[pkey]
    all_gold = [e for t in gold.values() for e in t]
    ext = extract_names(result, etypes)
    r = {
        "label": label, "passage": pkey, "n_chunks": n_chunks, "window": window,
        "n_entities": len(result.get("entities",[])) if result else 0,
        "n_relations": len(result.get("relations",[])) if result else 0,
        "recall": round(recall(ext, all_gold), 3),
        "recall_persons": round(recall(extract_names(result,["Person"]), gold.get("Person",[])), 3),
        "recall_places": round(recall(extract_names(result,["Place","Location"]), gold.get("Place",[])), 3),
        "recall_orgs": round(recall(extract_names(result,["Organization"]), gold.get("Organization",[])), 3),
        "fields": round(field_completeness(result), 3),
        "elapsed_s": round(elapsed, 1),
    }
    print(f"  {label:<40} recall={r['recall']:.2f} P={r['recall_persons']:.2f} "
          f"Pl={r['recall_places']:.2f} O={r['recall_orgs']:.2f} "
          f"fields={r['fields']:.2f} rels={r['n_relations']} t={elapsed:.0f}s")
    return r


# ════════════════════════════════════════════════════════════════════════════════
# EXPERIMENT A: JSON Robustness Strategies
# ════════════════════════════════════════════════════════════════════════════════

def prompt_cap20(text):
    """Cap at 20 entities — reduces output size → fewer parse failures."""
    return f"""You are a precise knowledge extraction engine.
Extract the most important named entities from the text. List AT MOST 20 entities.
Return ONLY valid JSON (no markdown, no explanation):
{{"entities":[{{"name":"...","type":"..."}},...] }}

Entity types: Person, Place, Organization
Rules: No pronouns, no generic roles. Real proper names only.

Text:
{text}"""

def prompt_names_only(text):
    """Even simpler: just a JSON array of name strings, then type separately."""
    return f"""List every named Person, Place, and Organization mentioned in this text.
Return ONLY a JSON object like: {{"Person":["name1","name2",...], "Place":["..."], "Organization":["..."]}}
No markdown. Real proper names only, no pronouns.

Text:
{text}"""

def prompt_two_pass_entities(text):
    """Step 1: extract entities. Step 2 (separate call): enrich with fields."""
    return f"""You are a precise knowledge extraction engine.
Extract named entities from the text below.
Return ONLY valid JSON (no markdown):
{{"entities":[{{"name":"...","type":"..."}},...] }}
Entity types: Person, Place, Organization
Rules: No pronouns. Real proper names only.

Text:
{text}"""

def prompt_enrich(entities_json, text):
    """Step 2: given an entity list, fill fields for each."""
    return f"""For each entity in the list below, extract any field values that appear in the text.
Return ONLY valid JSON (no markdown):
{{"entities":[{{"name":"...","type":"...","fields":{{...}}}},...] }}

Field keys:
  Person: birthDate, birthPlace, nationality, occupation, affiliation, spouse, parent, sibling, child
  Place: addressLocality, addressRegion, addressCountry, locationType
  Organization: foundingDate, location, founder, orgType

Entities: {entities_json}

Text:
{text}"""

def exp_a_json_robustness(results):
    print("\n═══ EXP A: JSON Robustness Strategies ═══")
    pkey = "A"  # Hardest passage (26 people)
    passage = PASSAGES[pkey]
    chunks = sentence_align(passage, 100)

    strategies = [
        ("cap20_w1", lambda: extract_windowed(chunks, 1, prompt_cap20)),
        ("names_only_w1", lambda: extract_windowed(chunks, 1, prompt_names_only)),
        ("baseline_3type_w1", lambda: extract_windowed(chunks, 1,
            lambda t: f"""You are a precise knowledge extraction engine.
Extract named entities. Return ONLY valid JSON: {{"entities":[{{"name":"...","type":"..."}},...] }}
Entity types: Person, Place, Organization. No pronouns. Real names only.\n\nText:\n{t}""")),
    ]

    for label, fn in strategies:
        t0 = time.time()
        result = fn()
        elapsed = time.time() - t0
        # For names_only, convert list format to entity format
        if result and "Person" in result and "entities" not in result:
            ents = []
            for t, names in result.items():
                if isinstance(names, list):
                    for n in names:
                        if n: ents.append({"name": n, "type": t})
            result = {"entities": ents, "relations": []}
        r = metrics(result or {"entities":[],"relations":[]}, pkey, f"PA_{label}",
                   len(chunks), 1, elapsed)
        results.append(r)

    # Two-pass approach
    print(f"  Running two-pass on Passage A...", end=" ", flush=True)
    t0 = time.time()
    # Pass 1: extract entity names
    all_ents = {}
    for i, chunk in enumerate(chunks):
        start = max(0, i-1); end = min(len(chunks), i+2)
        ctx = "\n\n[...]\n\n".join(chunks[start:end])
        r1 = call_ollama(prompt_two_pass_entities(ctx))
        if r1:
            for e in r1.get("entities",[]):
                k = e.get("name","").strip().lower()
                if k: all_ents[k] = e
    # Pass 2: enrich with fields (one call per chunk, with entity list)
    ent_list = list(all_ents.values())
    enriched = {}
    for i, chunk in enumerate(chunks):
        start = max(0, i-1); end = min(len(chunks), i+2)
        ctx = "\n\n[...]\n\n".join(chunks[start:end])
        relevant = [e for e in ent_list if e.get("name","").lower() in ctx.lower()]
        if not relevant: continue
        ej = json.dumps(relevant[:15])  # cap to avoid huge prompts
        r2 = call_ollama(prompt_enrich(ej, ctx))
        if r2:
            for e in r2.get("entities",[]):
                k = e.get("name","").strip().lower()
                if k: enriched[k] = e
    # Merge: take enriched fields, fall back to pass-1 name+type
    merged = {**all_ents, **enriched}
    result = {"entities": list(merged.values()), "relations": []}
    elapsed = time.time() - t0
    r = metrics(result, pkey, "PA_two_pass", len(chunks), 1, elapsed)
    r["note"] = "two-pass"
    results.append(r)


# ════════════════════════════════════════════════════════════════════════════════
# EXPERIMENT B: Optimal Chunk Size (sentence-aligned)
# ════════════════════════════════════════════════════════════════════════════════

def exp_b_chunk_size(results):
    print("\n═══ EXP B: Optimal Chunk Size (sentence-aligned, window=1) ═══")
    baseline_prompt = lambda t: f"""Extract named entities. Return ONLY valid JSON:
{{"entities":[{{"name":"...","type":"..."}},...] }}
Entity types: Person, Place, Organization. No pronouns. Real names only.\n\nText:\n{t}"""

    for pkey in ["A", "B", "C"]:
        passage = PASSAGES[pkey]
        print(f"\n  Passage {pkey} ({len(words(passage))}w):")
        for target in [60, 100, 150, 200]:
            chunks = sentence_align(passage, target)
            t0 = time.time()
            result = extract_windowed(chunks, 1, baseline_prompt)
            elapsed = time.time() - t0
            r = metrics(result, pkey, f"P{pkey}_sent{target}w_w1",
                       len(chunks), 1, elapsed)
            r["chunk_words"] = target
            results.append(r)


# ════════════════════════════════════════════════════════════════════════════════
# EXPERIMENT C: Context framing — targeted, sentence-aligned w1
# ════════════════════════════════════════════════════════════════════════════════

def exp_c_context(results):
    print("\n═══ EXP C: Context Framing (best window from Exp B) ═══")
    book_ctx = "District Six: Lest We Forget by Yousuf Rassool, Cape Town, South Africa, 1897-1956."
    section_ctxs = {
        "A": "Part One, Chapter One: 1930s social scene at Searle Street, Cape Town.",
        "B": "Chapter Two: Gool family history, migrations from India, 1879-1933.",
        "C": "Chapter One: The neighbourhood of Buitencingle Street, Cape Town, c.1920s.",
    }

    def make_prompt(ctx_text):
        def p(t):
            return f"""DOCUMENT CONTEXT: {ctx_text}

Extract named entities. Return ONLY valid JSON:
{{"entities":[{{"name":"...","type":"..."}},...] }}
Entity types: Person, Place, Organization. No pronouns. Real names only.

Text:
{t}"""
        return p

    def no_ctx(t):
        return f"""Extract named entities. Return ONLY valid JSON:
{{"entities":[{{"name":"...","type":"..."}},...] }}
Entity types: Person, Place, Organization. No pronouns. Real names only.\n\nText:\n{t}"""

    for pkey in ["A", "B", "C"]:
        passage = PASSAGES[pkey]
        chunks = sentence_align(passage, 100)
        print(f"\n  Passage {pkey}:")

        t0 = time.time()
        r1 = extract_windowed(chunks, 1, no_ctx)
        r = metrics(r1, pkey, f"P{pkey}_no_ctx", len(chunks), 1, time.time()-t0)
        results.append(r)

        t0 = time.time()
        r2 = extract_windowed(chunks, 1, make_prompt(book_ctx))
        r = metrics(r2, pkey, f"P{pkey}_book_ctx", len(chunks), 1, time.time()-t0)
        results.append(r)

        t0 = time.time()
        r3 = extract_windowed(chunks, 1, make_prompt(section_ctxs[pkey]))
        r = metrics(r3, pkey, f"P{pkey}_section_ctx", len(chunks), 1, time.time()-t0)
        results.append(r)


# ════════════════════════════════════════════════════════════════════════════════
# EXPERIMENT D: Relation extraction accuracy deep-dive
# ════════════════════════════════════════════════════════════════════════════════

KNOWN_RELATIONS_B = [
    ("J.M.H. Gool", "Bibi", "spouse_of"),
    ("J.M.H. Gool", "Wahida Ta'al", "spouse_of"),
    ("J.M.H. Gool", "Dr. Abdul Hamid Gool", "parent_of"),
    ("Joosub Maulvi Hamid", "Abbas", "parent_of"),  # they're brothers, father had Abbas+Yusuf
    ("Abbas", "Yusuf", "sibling_of"),
    ("Bibi", "Gujarat", "birthPlace"),  # not a relation type but notable
    ("J.M.H. Gool", "Mauritius", "located_in"),
]

def prompt_relations_explicit(text):
    """Ask for explicitly stated relations only, with a small relation type set."""
    return f"""Extract named entities and their relationships from the text.
Only include relationships that are EXPLICITLY stated (not implied).
Return ONLY valid JSON:
{{"entities":[{{"name":"...","type":"..."}},...],
  "relations":[{{"from":"...","to":"...","relation":"..."}},...] }}

Entity types: Person, Place, Organization
Relation types: parent_of, child_of, spouse_of, sibling_of, works_at, founded,
                located_in, part_of, associated_with

No pronouns. Real names only. Explicit relations only.

Text:
{text}"""

def prompt_relations_inferred(text):
    """Ask for both explicit and reasonably inferred relations."""
    return f"""Extract named entities and relationships from the text.
Include both explicit and strongly implied relationships.
Return ONLY valid JSON:
{{"entities":[{{"name":"...","type":"..."}},...],
  "relations":[{{"from":"...","to":"...","relation":"",...}},...] }}

Entity types: Person, Place, Organization
Relation types: parent_of, child_of, spouse_of, sibling_of, works_at, founded,
                located_in, part_of, associated_with

No pronouns. Real names only.

Text:
{text}"""

def judge_relation(relation, text):
    """Use LLM as judge: is this relation supported by the text?"""
    prompt = f"""Is this relationship explicitly or clearly supported by the following text?
Relationship: {relation["from"]} --[{relation["relation"]}]--> {relation["to"]}
Answer with ONLY: "yes" or "no"

Text:
{text}"""
    body = json.dumps({
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": 10,
        "stream": False,
    }).encode()
    req = urllib.request.Request(f"{OLLAMA_URL}/api/chat", data=body,
                                  headers={"Content-Type": "application/json"}, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = json.loads(resp.read())
            ans = raw["message"]["content"].strip().lower()
            return ans.startswith("yes")
    except:
        return None

def exp_d_relations(results):
    print("\n═══ EXP D: Relation Extraction Deep-Dive (Passage B) ═══")
    pkey = "B"
    passage = PASSAGES[pkey]
    chunks = sentence_align(passage, 100)

    configs = [
        ("PB_rel_explicit_w0", 0, prompt_relations_explicit),
        ("PB_rel_explicit_w1", 1, prompt_relations_explicit),
        ("PB_rel_inferred_w0", 0, prompt_relations_inferred),
        ("PB_rel_inferred_w1", 1, prompt_relations_inferred),
        ("PB_rel_inferred_full", -1, prompt_relations_inferred),  # -1 = full passage
    ]

    for label, window, pfn in configs:
        t0 = time.time()
        if window == -1:
            result = call_ollama(pfn(passage))
            result = result or {"entities": [], "relations": []}
        else:
            result = extract_windowed(chunks, window, pfn)
        elapsed = time.time() - t0

        n_rels = len(result.get("relations", []))
        r = metrics(result, pkey, label, len(chunks), window, elapsed)
        results.append(r)

        # LLM-as-judge on relation precision (sample up to 10 rels)
        rels = result.get("relations", [])[:10]
        if rels:
            print(f"    Judging {len(rels)} relations...", end=" ", flush=True)
            verified = 0
            for rel in rels:
                j = judge_relation(rel, passage)
                if j is True:
                    verified += 1
            precision = verified / len(rels) if rels else 0
            print(f"precision={precision:.2f} ({verified}/{len(rels)} verified)")
            r["relation_precision"] = round(precision, 2)
            r["relations_judged"] = len(rels)

    # Known-relation recall: which strategies find the key B relations?
    print(f"\n  Known-relation recall for Passage B:")
    for label, window, pfn in configs:
        t0 = time.time()
        if window == -1:
            result = call_ollama(pfn(passage))
            result = result or {"entities": [], "relations": []}
        else:
            result = extract_windowed(chunks, window, pfn)
        rels = result.get("relations", [])
        found = 0
        for kr in KNOWN_RELATIONS_B[:5]:  # family relations only
            for r in rels:
                f_norm = normalize(r.get("from",""))
                t_norm = normalize(r.get("to",""))
                if (normalize(kr[0]) in f_norm or normalize(kr[0]) in t_norm) and \
                   (normalize(kr[1]) in f_norm or normalize(kr[1]) in t_norm):
                    found += 1
                    break
        print(f"    {label:<40} found {found}/{min(5, len(KNOWN_RELATIONS_B))} known family relations")


# ─── Summary ───────────────────────────────────────────────────────────────────

def summarise(results):
    print("\n" + "═"*80)
    print("  PHASE 2 SUMMARY")
    print("═"*80)

    def best(subset, key="recall"):
        return max(subset, key=lambda r: r.get(key, 0)) if subset else None

    groups = {
        "Exp A (JSON robustness)": [r for r in results if r["label"].startswith("PA_") and
            any(x in r["label"] for x in ["cap20","names_only","two_pass","3type"])],
        "Exp B (chunk size)": [r for r in results if "_sent" in r["label"]],
        "Exp C (context)": [r for r in results if "_ctx" in r["label"]],
        "Exp D (relations)": [r for r in results if "_rel_" in r["label"]],
    }
    for title, subset in groups.items():
        b = best(subset)
        if b:
            print(f"\n{title}:")
            print(f"  Best recall: {b['label']} = {b['recall']:.2f} "
                  f"(P={b['recall_persons']:.2f} Pl={b['recall_places']:.2f} O={b['recall_orgs']:.2f})")
        # Also show field completeness winner
        fb = best(subset, "fields")
        if fb and fb["fields"] > 0:
            print(f"  Best fields: {fb['label']} = {fb['fields']:.2f}")

    # Optimal settings recommendation
    print("\n" + "─"*80)
    print("  RECOMMENDED SETTINGS (based on Phase 1 + Phase 2):")
    print("─"*80)
    all_recall = [r["recall"] for r in results]
    top10 = sorted(results, key=lambda r: r["recall"], reverse=True)[:5]
    for r in top10:
        print(f"  {r['label']:<45} recall={r['recall']:.2f} fields={r['fields']:.2f} t={r['elapsed_s']:.0f}s")


if __name__ == "__main__":
    try:
        urllib.request.urlopen(f"{OLLAMA_URL}/api/tags", timeout=5)
    except Exception as e:
        print(f"ERROR: Ollama not reachable: {e}")
        sys.exit(1)

    print(f"Model: {MODEL} | Phase 2 experiments")
    results = []

    exp_a_json_robustness(results)
    exp_b_chunk_size(results)
    exp_c_context(results)
    exp_d_relations(results)
    summarise(results)

    for r in results:
        r.pop("result", None)
    with open("tests/entity_extraction_phase2_results.json", "w") as f:
        json.dump(results, f, indent=2)
    print("\nResults saved to tests/entity_extraction_phase2_results.json")
