#!/usr/bin/env python3
"""
Entity extraction accuracy experiment.
Tests different input text strategies against a gold standard.

Experiments:
  1. Window size (adjacent chunks)
  2. Sentence alignment
  3. Context framing (doc title, section heading)
  4. Entity type scope (15-type vs 3-type)
  5. Relationship extraction with varying context

Metric: Entity Coverage Rate = found / gold_entities (recall)
        Field Completeness    = filled_fields / expected_fields (for Person/Place/Org)
        Precision proxy       = entities_with_fields / total_extracted
"""

import json, re, time, sys, textwrap
from typing import Optional
import urllib.request, urllib.error

OLLAMA_URL = "http://localhost:11434"
MODEL = "llama3.1:8b"

# ─── Test passages ────────────────────────────────────────────────────────────
# Transcribed from the D6 memoir PDF (pages 2-8).
# Three passages of different character and entity density.

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

# ─── Gold standard entity lists (manually curated from passages) ───────────────
# These are the named entities a perfect extractor should find.

GOLD_A = {
    "Person": [
        "M.K. Gandhi", "Dr. Abdul Hamid Gool", "Bernard Herzberg", "Eva Sachs",
        "Fred Bodmer", "Gregoire Boonzaaier", "Terence Macaw", "Ben Kies",
        "Paul Kostin", "Wolf Kiebel", "J.G. Taylor", "Dora Taylor", "Jane Gool",
        "I. B. Tabata", "Eddie Roux", "Goolam Gool", "Eli Weinberg",
        "Waradeya Abdurahman", "Dr. Abdulla Abdurahman", "Leah Solomon",
        "Zaid Gamiet", "Hans Friederichs", "Minnie Gool", "Cissie Gool",
        "Freda Lock", "Lilian Isaacson",
    ],
    "Place": ["Searle Street", "Trafalgar Park", "City Hall", "Germany", "South Africa"],
    "Organization": ["German Working Class"],
}

GOLD_B = {
    "Person": [
        "J.M.H. Gool", "Bibi", "Wahida Ta'al", "Joosub Maulvi Hamid",
        "Abbas", "Yusuf", "Aunt Jane", "Barney Barnato", "Solly Joel",
        "Cecil Rhodes", "Gandhi", "George Bernard Shaw", "Sarojini Naidu",
        "Dr. Abdullah Abdurahman", "Elizabeth Molteno", "Alice Greene",
        "Grahame Greene", "Lord Headley",
    ],
    "Place": [
        "Broach", "Gujarat", "India", "Mauritius", "Cape Town",
        "Swat", "Rander", "Buitencingle",
    ],
    "Organization": ["All India Football team", "British Muslims"],
}

GOLD_C = {
    "Person": ["Jan Van Riebeeck"],
    "Place": [
        "Buitencingle Street", "Cape Town", "Table Mountain", "Kloof Street",
        "Kloof Nek", "Adderley Street", "Lion's Head", "Loop Street",
        "Long Street", "Rogge Bay", "Orange Street", "Waterkant Street",
    ],
    "Organization": [
        "Fletchers and Cartwrights", "German Lutheran Church",
        "Hanaffi Qwatul Islam Mosque", "Mount Nelson Hotel",
        "Botanical Gardens", "St. George's Grammar",
        "The Houses of Parliament", "The South African Public Library",
        "The Little Theatre",
    ],
}

GOLD = {"A": GOLD_A, "B": GOLD_B, "C": GOLD_C}
PASSAGES = {"A": PASSAGE_A, "B": PASSAGE_B, "C": PASSAGE_C}

# ─── Entity types ──────────────────────────────────────────────────────────────
ENTITY_TYPES_15 = [
    "Person", "Organization", "Location", "Event", "Concept", "Method",
    "Claim", "Quantity", "Date", "Document", "Product", "Technology",
    "Role", "Topic", "Unknown",
]
ENTITY_TYPES_3 = ["Person", "Place", "Organization"]

EXPECTED_FIELDS = {
    "Person": ["birthDate", "birthPlace", "deathDate", "nationality",
               "occupation", "affiliation", "spouse", "parent", "sibling", "child"],
    "Place": ["addressLocality", "addressRegion", "addressCountry",
              "locationType", "historicalNote"],
    "Organization": ["foundingDate", "dissolutionDate", "location", "founder", "orgType"],
}

# ─── Prompts ───────────────────────────────────────────────────────────────────

def prompt_15type(text: str, section: Optional[str] = None) -> str:
    ctx = f'DOCUMENT CONTEXT: {section}\n\n' if section else ""
    etype_list = ", ".join(ENTITY_TYPES_15)
    return f"""{ctx}You are a precise knowledge extraction engine.
Extract named entities and relationships from the text below.
Return ONLY valid JSON matching this schema (no markdown, no explanation):
{{"entities":[{{"name":"...","type":"...","description":"1-2 sentences"}},...], "relations":[{{"from":"entity name","to":"entity name","relation":"relation_type"}},...] }}

Entity types: {etype_list}

IMPORTANT RULES:
- Never use pronouns or generic roles (I, me, he, narrator, author) as entity names.
- If "I" or "the author" refers to a named person, use their actual name.
- Only extract entities with real proper names.

Text:
{text}"""

def prompt_3type_structured(text: str, section: Optional[str] = None) -> str:
    ctx = f'DOCUMENT CONTEXT: {section}\n\n' if section else ""
    return f"""{ctx}You are a precise knowledge extraction engine.
Extract named entities from the text below.
Return ONLY valid JSON (no markdown, no explanation):
{{"entities":[{{"name":"...","type":"...","fields":{{...}}}},...] }}

Entity types: Person, Place, Organization

Field keys by entity type — include only keys whose values appear in the text:
  Person:       birthDate, birthPlace, deathDate, nationality, occupation, affiliation, spouse, parent, sibling, child
  Place:        addressLocality, addressRegion, addressCountry, locationType, historicalNote
  Organization: foundingDate, dissolutionDate, location, founder, orgType

IMPORTANT RULES:
- Never use pronouns or generic roles as entity names.
- Only extract entities with real proper names.
- Omit any field whose value is not clearly stated in the text.

Text:
{text}"""

def prompt_3type_nofields(text: str, section: Optional[str] = None) -> str:
    ctx = f'DOCUMENT CONTEXT: {section}\n\n' if section else ""
    return f"""{ctx}You are a precise knowledge extraction engine.
Extract named entities from the text below.
Return ONLY valid JSON (no markdown, no explanation):
{{"entities":[{{"name":"...","type":"..."}},...] }}

Entity types: Person, Place, Organization

IMPORTANT RULES:
- Never use pronouns or generic roles as entity names.
- Only extract entities with real proper names.

Text:
{text}"""

# ─── Utilities ─────────────────────────────────────────────────────────────────

def words(text: str) -> list[str]:
    return text.split()

def chunk_words(text: str, size: int, overlap: int = 0) -> list[str]:
    """Split text into word-count chunks with optional overlap."""
    w = words(text)
    chunks = []
    i = 0
    while i < len(w):
        chunk = " ".join(w[i:i+size])
        chunks.append(chunk)
        i += max(1, size - overlap)
    return chunks

def sentence_align(text: str, target_words: int) -> list[str]:
    """Split text into chunks aligned to sentence boundaries near target_words."""
    sentences = re.split(r'(?<=[.!?])\s+', text.strip())
    chunks = []
    current = []
    current_count = 0
    for sent in sentences:
        sw = len(sent.split())
        if current_count + sw > target_words and current:
            chunks.append(" ".join(current))
            current = [sent]
            current_count = sw
        else:
            current.append(sent)
            current_count += sw
    if current:
        chunks.append(" ".join(current))
    return chunks

def paragraph_chunks(text: str) -> list[str]:
    """Split at paragraph boundaries."""
    paras = [p.strip() for p in text.split("\n\n") if p.strip()]
    return paras

def repair_json(content: str) -> str:
    """Best-effort JSON repair: strip fences, trailing commas, control chars."""
    content = re.sub(r'^```(?:json)?\s*', '', content.strip())
    content = re.sub(r'\s*```$', '', content)
    # Remove trailing commas before ] or }
    content = re.sub(r',\s*([}\]])', r'\1', content)
    # Replace bare None/null
    content = content.replace(': None', ': null').replace(':None', ':null')
    return content

def call_ollama(prompt: str, temperature: float = 0.1) -> Optional[dict]:
    """Call Ollama chat API and parse JSON response."""
    body = json.dumps({
        "model": MODEL,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": temperature,
        "max_tokens": 2048,
        "stream": False,
    }).encode()
    req = urllib.request.Request(
        f"{OLLAMA_URL}/api/chat",
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=120) as resp:
            raw = json.loads(resp.read())
            content = raw["message"]["content"].strip()
            content = repair_json(content)
            return json.loads(content)
    except json.JSONDecodeError as e:
        # Try extracting first {...} block
        m = re.search(r'\{.*\}', content if 'content' in dir() else '', re.DOTALL)
        if m:
            try:
                return json.loads(repair_json(m.group()))
            except Exception:
                pass
        print(f"    [JSON ERROR] {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"    [ERROR] {e}", file=sys.stderr)
        return None

def extract_names(result: dict, type_filter: Optional[list] = None) -> set[str]:
    """Return set of lowercase entity names from an extraction result."""
    if not result or "entities" not in result:
        return set()
    names = set()
    for e in result["entities"]:
        t = e.get("type", "")
        if type_filter and t not in type_filter:
            continue
        name = e.get("name", "").strip()
        if name:
            names.add(name.lower())
    return names

def normalize(name: str) -> str:
    """Normalize for fuzzy matching: lowercase, strip titles."""
    n = name.lower()
    for prefix in ["dr. ", "dr ", "mr. ", "mr ", "mrs. ", "mrs ", "ms. ", "ms ",
                   "prof. ", "prof ", "lord ", "sir ", "the "]:
        n = n.removeprefix(prefix)
    return n.strip()

def recall(extracted: set[str], gold: list[str]) -> float:
    """Fraction of gold entities found (fuzzy by normalize)."""
    if not gold:
        return 1.0
    norm_extracted = {normalize(e) for e in extracted}
    found = sum(1 for g in gold if normalize(g) in norm_extracted)
    return found / len(gold)

def precision_proxy(result: dict, entity_types: list[str]) -> float:
    """Fraction of extracted entities that have at least one filled field (quality signal)."""
    if not result or "entities" not in result:
        return 0.0
    entities = [e for e in result["entities"] if e.get("type") in entity_types]
    if not entities:
        return 0.0
    with_fields = sum(
        1 for e in entities
        if e.get("fields") and any(v for v in e["fields"].values())
           or e.get("description", "").strip()
    )
    return with_fields / len(entities)

def field_completeness(result: dict) -> float:
    """Average fraction of expected fields filled, for Person/Place/Org entities."""
    if not result or "entities" not in result:
        return 0.0
    scores = []
    for e in result["entities"]:
        t = e.get("type", "")
        if t not in EXPECTED_FIELDS:
            continue
        expected = EXPECTED_FIELDS[t]
        fields = e.get("fields", {})
        if not fields:
            if e.get("description"):
                scores.append(0.1)  # has description but no structured fields
            continue
        filled = sum(1 for k in expected if str(fields.get(k) or "").strip())
        scores.append(filled / len(expected))
    return sum(scores) / len(scores) if scores else 0.0

def entity_count(result: dict) -> int:
    if not result or "entities" not in result:
        return 0
    return len(result["entities"])

def relation_count(result: dict) -> int:
    if not result or "relations" not in result:
        return 0
    return len(result["relations"])

# ─── Windowed extraction ───────────────────────────────────────────────────────

def extract_windowed(chunks: list[str], window: int, prompt_fn, section: str = None) -> dict:
    """
    For each chunk, call prompt_fn with a window of chunks centered on it.
    Aggregate all extracted entities (deduplicated by name).
    Returns merged result dict.
    """
    all_entities = {}  # name_lower -> entity dict (last write wins for fields)
    all_relations = []
    for i, _ in enumerate(chunks):
        start = max(0, i - window // 2)
        end = min(len(chunks), i + window // 2 + 1)
        context_text = "\n\n[...]\n\n".join(chunks[start:end])
        result = call_ollama(prompt_fn(context_text, section))
        if result:
            for e in result.get("entities", []):
                k = e.get("name", "").strip().lower()
                if k:
                    all_entities[k] = e
            all_relations.extend(result.get("relations", []))
    return {"entities": list(all_entities.values()), "relations": all_relations}

# ─── Run single experiment ─────────────────────────────────────────────────────

def run_experiment(label: str, passage_key: str, chunks: list[str],
                   prompt_fn, window: int = 0, section: str = None,
                   entity_types: list[str] = None) -> dict:
    """Run extraction and return metrics dict."""
    print(f"  Running {label} ({len(chunks)} chunk(s), window={window})...", end=" ", flush=True)
    t0 = time.time()

    if window == 0 and len(chunks) == 1:
        # Full passage — single call
        result = call_ollama(prompt_fn(chunks[0], section))
    elif window == 0:
        # No windowing — each chunk processed independently, then merged
        result = extract_windowed(chunks, 0, prompt_fn, section)
    else:
        result = extract_windowed(chunks, window, prompt_fn, section)

    elapsed = time.time() - t0
    gold = GOLD[passage_key]

    if entity_types is None:
        entity_types = ENTITY_TYPES_3

    # Compute metrics
    extracted_names = extract_names(result, entity_types)
    all_gold = [e for t in gold.values() for e in t]
    gold_persons = gold.get("Person", [])
    gold_places = gold.get("Place", [])
    gold_orgs = gold.get("Organization", [])

    metrics = {
        "label": label,
        "passage": passage_key,
        "n_chunks": len(chunks),
        "window": window,
        "n_entities_extracted": entity_count(result),
        "n_relations_extracted": relation_count(result),
        "recall_overall": round(recall(extracted_names, all_gold), 3),
        "recall_persons": round(recall(extract_names(result, ["Person"]), gold_persons), 3),
        "recall_places": round(recall(extract_names(result, ["Place", "Location"]), gold_places), 3),
        "recall_orgs": round(recall(extract_names(result, ["Organization"]), gold_orgs), 3),
        "field_completeness": round(field_completeness(result), 3),
        "precision_proxy": round(precision_proxy(result, entity_types), 3),
        "elapsed_s": round(elapsed, 1),
        "result": result,
    }
    print(f"recall={metrics['recall_overall']:.2f} persons={metrics['recall_persons']:.2f} "
          f"places={metrics['recall_places']:.2f} orgs={metrics['recall_orgs']:.2f} "
          f"fields={metrics['field_completeness']:.2f} t={elapsed:.0f}s")
    return metrics

# ─── Experiment suites ─────────────────────────────────────────────────────────

def exp1_window_size(results: list):
    """Experiment 1: How does context window size affect recall?"""
    print("\n═══ EXP 1: Window Size (Adjacent Chunks) ═══")
    chunk_size = 100  # words per chunk — simulates ~130 token chunks
    for pkey in ["A", "B", "C"]:
        passage = PASSAGES[pkey]
        chunks = chunk_words(passage, chunk_size, overlap=0)
        print(f"\n  Passage {pkey} ({len(words(passage))} words → {len(chunks)} chunks of ~{chunk_size}w):")

        # Full passage as single chunk (gold reference)
        r = run_experiment(f"P{pkey}_full", pkey, [passage], prompt_3type_nofields, window=0)
        results.append(r)

        # Independent chunks (baseline — what current system does)
        r = run_experiment(f"P{pkey}_w0_independent", pkey, chunks, prompt_3type_nofields, window=0)
        results.append(r)

        # +1 adjacent (window of 3)
        r = run_experiment(f"P{pkey}_w1_adjacent", pkey, chunks, prompt_3type_nofields, window=1)
        results.append(r)

        # +2 adjacent (window of 5)
        r = run_experiment(f"P{pkey}_w2_adjacent", pkey, chunks, prompt_3type_nofields, window=2)
        results.append(r)


def exp2_sentence_alignment(results: list):
    """Experiment 2: Raw word cut vs sentence-aligned chunks."""
    print("\n═══ EXP 2: Sentence Alignment ═══")
    target_words = 100
    for pkey in ["A", "B"]:
        passage = PASSAGES[pkey]
        print(f"\n  Passage {pkey}:")

        # Raw cut (may split mid-sentence)
        raw_chunks = chunk_words(passage, target_words, overlap=0)
        r = run_experiment(f"P{pkey}_raw_cut", pkey, raw_chunks,
                           prompt_3type_nofields, window=0)
        results.append(r)

        # Sentence-aligned chunks
        sent_chunks = sentence_align(passage, target_words)
        r = run_experiment(f"P{pkey}_sent_aligned", pkey, sent_chunks,
                           prompt_3type_nofields, window=0)
        results.append(r)

        # Sentence-aligned + window=1
        r = run_experiment(f"P{pkey}_sent_w1", pkey, sent_chunks,
                           prompt_3type_nofields, window=1)
        results.append(r)


def exp3_context_framing(results: list):
    """Experiment 3: Effect of document/section context on quality."""
    print("\n═══ EXP 3: Context Framing ═══")
    chunk_size = 100
    doc_context = "District Six: Lest We Forget, by Yousuf (Joe) Rassool. " \
                  "A memoir of life in Cape Town, South Africa, 1897-1956."
    section_contexts = {
        "A": "Chapter One - The Setting in Time and Place: Buitencingle Street, Cape Town",
        "B": "Chapter Two - Two Tales: The Gool family history and migration to South Africa",
        "C": "Chapter One - The Setting in Time and Place: The neighbourhood of Buitencingle",
    }
    for pkey in ["A", "B"]:
        passage = PASSAGES[pkey]
        chunks = sentence_align(passage, chunk_size)
        print(f"\n  Passage {pkey}:")

        # No context
        r = run_experiment(f"P{pkey}_no_ctx", pkey, chunks, prompt_3type_nofields, window=1)
        results.append(r)

        # Doc-level context only
        r = run_experiment(f"P{pkey}_doc_ctx", pkey, chunks,
                           lambda t, s=None: prompt_3type_nofields(t, doc_context),
                           window=1)
        results.append(r)

        # Section context only
        sec = section_contexts[pkey]
        r = run_experiment(f"P{pkey}_sec_ctx", pkey, chunks,
                           lambda t, s=None, _sec=sec: prompt_3type_nofields(t, _sec),
                           window=1)
        results.append(r)

        # Both doc + section
        full_ctx = f"{doc_context} | {section_contexts[pkey]}"
        r = run_experiment(f"P{pkey}_full_ctx", pkey, chunks,
                           lambda t, s=None, _ctx=full_ctx: prompt_3type_nofields(t, _ctx),
                           window=1)
        results.append(r)


def exp4_type_scope(results: list):
    """Experiment 4: 15-type vs 3-type (no fields) vs 3-type (with fields)."""
    print("\n═══ EXP 4: Entity Type Scope ═══")
    chunk_size = 100
    for pkey in ["A", "B"]:
        passage = PASSAGES[pkey]
        chunks = sentence_align(passage, chunk_size)
        print(f"\n  Passage {pkey}:")

        # 15 types (current default) — window=1 with section context
        def p15(t, s=None):
            return prompt_15type(t, s)
        r = run_experiment(f"P{pkey}_15type", pkey, chunks, p15, window=1,
                           entity_types=["Person", "Organization", "Location"])
        r["entity_types"] = "15-type"
        results.append(r)

        # 3 types, no structured fields
        r = run_experiment(f"P{pkey}_3type_plain", pkey, chunks,
                           prompt_3type_nofields, window=1,
                           entity_types=ENTITY_TYPES_3)
        r["entity_types"] = "3-type-plain"
        results.append(r)

        # 3 types, with structured fields
        r = run_experiment(f"P{pkey}_3type_fields", pkey, chunks,
                           prompt_3type_structured, window=1,
                           entity_types=ENTITY_TYPES_3)
        r["entity_types"] = "3-type-fields"
        results.append(r)


def exp5_relations(results: list):
    """Experiment 5: Relationship extraction with varying context."""
    print("\n═══ EXP 5: Relationship Extraction Window ═══")
    chunk_size = 100
    pkey = "B"  # Most relational passage (family history)
    passage = PASSAGES[pkey]
    chunks = sentence_align(passage, chunk_size)

    # Independent chunks
    r = run_experiment(f"P{pkey}_rel_w0", pkey, chunks, prompt_15type, window=0)
    r["note"] = "relations w=0"
    results.append(r)

    # Window of 1
    r = run_experiment(f"P{pkey}_rel_w1", pkey, chunks, prompt_15type, window=1)
    r["note"] = "relations w=1"
    results.append(r)

    # Window of 2
    r = run_experiment(f"P{pkey}_rel_w2", pkey, chunks, prompt_15type, window=2)
    r["note"] = "relations w=2"
    results.append(r)

    # Full passage single call
    r = run_experiment(f"P{pkey}_rel_full", pkey, [passage], prompt_15type, window=0)
    r["note"] = "relations full passage"
    results.append(r)


# ─── Reporting ─────────────────────────────────────────────────────────────────

def print_table(results: list, title: str, keys: list[str]):
    print(f"\n{'─'*80}")
    print(f"  {title}")
    print(f"{'─'*80}")
    header = f"{'Label':<35} {'Recall':>7} {'Person':>7} {'Place':>7} {'Org':>7} {'Fields':>7} {'#Ent':>5} {'#Rel':>5}"
    print(header)
    print("─" * 80)
    for r in results:
        if any(r["label"].startswith(k) for k in keys):
            print(f"{r['label']:<35} "
                  f"{r['recall_overall']:>7.2f} "
                  f"{r['recall_persons']:>7.2f} "
                  f"{r['recall_places']:>7.2f} "
                  f"{r['recall_orgs']:>7.2f} "
                  f"{r['field_completeness']:>7.2f} "
                  f"{r['n_entities_extracted']:>5} "
                  f"{r['n_relations_extracted']:>5}")

def summarise(results: list):
    print("\n" + "═" * 80)
    print("  SUMMARY: Best strategy per experiment")
    print("═" * 80)

    def best(subset, metric="recall_overall"):
        if not subset:
            return None
        return max(subset, key=lambda r: r[metric])

    # Exp 1
    e1 = [r for r in results if r["label"].startswith("P") and
          any(x in r["label"] for x in ["_full", "_w0_", "_w1_", "_w2_"])]
    groups = {}
    for r in e1:
        p = r["passage"]
        groups.setdefault(p, []).append(r)
    print("\nExp 1 — Window Size:")
    for p, g in groups.items():
        b = best(g)
        print(f"  Passage {p}: best={b['label']} recall={b['recall_overall']:.2f}")

    # Exp 2
    e2 = [r for r in results if any(x in r["label"] for x in ["_raw_cut", "_sent_"])]
    print("\nExp 2 — Sentence Alignment:")
    for p in ["A", "B"]:
        g = [r for r in e2 if r["passage"] == p]
        b = best(g)
        if b:
            print(f"  Passage {p}: best={b['label']} recall={b['recall_overall']:.2f}")

    # Exp 3
    e3 = [r for r in results if any(x in r["label"] for x in ["_no_ctx", "_doc_ctx", "_sec_ctx", "_full_ctx"])]
    print("\nExp 3 — Context Framing:")
    for p in ["A", "B"]:
        g = [r for r in e3 if r["passage"] == p]
        b = best(g)
        if b:
            print(f"  Passage {p}: best={b['label']} recall={b['recall_overall']:.2f}")

    # Exp 4
    e4 = [r for r in results if any(x in r["label"] for x in ["_15type", "_3type_"])]
    print("\nExp 4 — Type Scope:")
    for p in ["A", "B"]:
        g = [r for r in e4 if r["passage"] == p]
        b = best(g)
        if b:
            print(f"  Passage {p}: best={b['label']} recall={b['recall_overall']:.2f} "
                  f"fields={b['field_completeness']:.2f}")

    # Exp 5
    e5 = [r for r in results if "_rel_" in r["label"]]
    print("\nExp 5 — Relation Extraction Window:")
    b = best(e5, "n_relations_extracted")
    if b:
        print(f"  Best relation yield: {b['label']} relations={b['n_relations_extracted']} "
              f"recall={b['recall_overall']:.2f}")
    b2 = best(e5, "recall_overall")
    if b2:
        print(f"  Best entity recall: {b2['label']} recall={b2['recall_overall']:.2f} "
              f"relations={b2['n_relations_extracted']}")


# ─── Main ──────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    # Quick sanity check
    try:
        req = urllib.request.Request(f"{OLLAMA_URL}/api/tags", method="GET")
        urllib.request.urlopen(req, timeout=5)
    except Exception as e:
        print(f"ERROR: Ollama not reachable at {OLLAMA_URL}: {e}")
        sys.exit(1)

    print(f"Model: {MODEL} | Ollama: {OLLAMA_URL}")
    print(f"Passages: A={len(words(PASSAGE_A))}w  B={len(words(PASSAGE_B))}w  C={len(words(PASSAGE_C))}w")

    results = []

    exp1_window_size(results)
    exp2_sentence_alignment(results)
    exp3_context_framing(results)
    exp4_type_scope(results)
    exp5_relations(results)

    # Print tables
    print_table(results, "EXP 1 — Window Size", ["P"])
    summarise(results)

    # Save full results
    out_path = "tests/entity_extraction_results.json"
    # Strip result dicts to keep file manageable
    for r in results:
        if "result" in r:
            del r["result"]
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\nResults saved to {out_path}")
